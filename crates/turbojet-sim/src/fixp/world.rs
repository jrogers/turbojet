//! The FIXP world: a client and a server over the simulated network, the workload, the faults,
//! and the loop that runs them from a seed, checking after every event. It follows the FIX world
//! (`crate::world`), without what FIXP lacks: resend steps, windows, schedules, operators and
//! cancel on disconnect.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Duration;

use turbojet::fixp::{ClientConfig, FixpConfig, FixpRegistry, FlowType, Role, ServerConfig};
use turbojet::store::SessionStorage;
use turbojet::{DiskStorage, MemoryStorage, ReconnectPolicy};

use super::app::RecordingApp;
use super::check::Checker;
use super::node::{Effects, Node};
use crate::Side;
use crate::app::Sent;
use crate::check::Violation;
use crate::files::DiskFiles;
use crate::files::Tear;
use crate::net::{ConnId, Net, Params};
use crate::queue::Queue;
use crate::rng::Rng;
use crate::store::{Call, LedgerStorage, Trap};
use crate::time::{Clocks, SimTime};
use crate::world::{Failure, Options, PLANTED_AT, Plant, Report};

/// How long a slow store's commit takes, mostly, and now and then: as in the FIX world.
const COMMIT_TIME: Duration = Duration::from_millis(5);
const SLOW_COMMIT_TIME: Duration = Duration::from_millis(50);
/// How often the OS writes back a disk store without sync.
const WRITE_BACK_EVERY: Duration = Duration::from_secs(5);
/// As `FixpInitiator`'s.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Most events at one instant, and in a run, before the run counts as spinning.
const MAX_EVENTS_AT_ONCE: u64 = 100_000;
const MAX_EVENTS: u64 = 20_000_000;

/// One seed's settings and faults.
#[derive(Debug, Clone)]
struct Faults {
    client_flow: FlowType,
    server_flow: FlowType,
    keepalive: Duration,
    /// The most messages one retransmission carries, the same on both sides: a server refusing
    /// a request over its own limit is the client's mistake, which this simulation doesn't make.
    max_retransmit: u32,
    net: Params,
    reset_every: Option<Duration>,
    black_hole_every: Option<Duration>,
    stall_every: Option<Duration>,
    stall_max: Duration,
    refuse: u32,
    connect_max: Duration,
    give_up: Duration,
    disk: bool,
    terminate_every: Option<Duration>,
    send_queue: usize,
    burst_every: Option<Duration>,
    /// Mean time between process crashes (of either node), if any, and the longest a restart takes.
    crash_every: Option<Duration>,
    restart_max: Duration,
    /// A disk store with sync: a power loss tears the call in progress. Without, the files go
    /// back to what the OS had written back.
    sync: bool,
    /// Mean time between traps set in a store call, if any, and the chance in a million that one
    /// crashes the process rather than just failing the call.
    trap_every: Option<Duration>,
    trap_crashes: u32,
    /// With sync: the chance in a million that a trap that crashes is a power loss tearing the
    /// call's write, and whether a record can tear within a sector.
    tears: u32,
    sub_sector: bool,
    /// Without sync: mean time between power losses, if any.
    power_loss_every: Option<Duration>,
    /// Mean time between one side's application finishing sending, ending the logical session.
    finish_every: Option<Duration>,
}

impl Faults {
    fn draw(rng: &mut Rng) -> Self {
        let ms = Duration::from_millis;
        let secs = Duration::from_secs;
        let flow = |rng: &mut Rng| {
            rng.pick(&[FlowType::Recoverable, FlowType::Recoverable, FlowType::Idempotent, FlowType::Unsequenced])
        };
        let (mut client_flow, mut server_flow) = (flow(rng), flow(rng));
        if rng.chance(100_000) {
            if rng.chance(500_000) { client_flow = FlowType::None } else { server_flow = FlowType::None }
        }
        let keepalive = rng.pick(&[ms(100), ms(500), secs(1), secs(5)]);
        Self {
            client_flow,
            server_flow,
            keepalive,
            max_retransmit: rng.pick(&[1, 5, 100, 1000]),
            net: Params {
                latency: (Duration::from_micros(10), rng.pick(&[Duration::from_micros(100), ms(1), ms(20)])),
                capacity: rng.pick(&[4 * 1024, 64 * 1024, 1024 * 1024]),
                max_segments: rng.pick(&[1, 4, 16]),
            },
            reset_every: rng.pick(&[None, Some(secs(60)), Some(secs(20)), Some(secs(5))]),
            black_hole_every: rng.pick(&[None, None, Some(secs(60)), Some(secs(20))]),
            stall_every: rng.pick(&[None, Some(secs(30)), Some(secs(10))]),
            stall_max: keepalive * 3,
            refuse: rng.pick(&[0, 100_000, 500_000]),
            connect_max: rng.pick(&[Duration::ZERO, secs(2), secs(15)]),
            give_up: secs(rng.between(60, 300)),
            disk: rng.chance(500_000),
            terminate_every: rng.pick(&[None, Some(secs(60)), Some(secs(15))]),
            send_queue: rng.pick(&[10_000, 10_000, 50, 5]),
            burst_every: rng.pick(&[None, Some(secs(10)), Some(secs(3))]),
            crash_every: rng.pick(&[None, Some(secs(60)), Some(secs(15))]),
            restart_max: rng.pick(&[ms(100), secs(5), secs(30)]),
            sync: rng.chance(500_000),
            trap_every: rng.pick(&[None, Some(secs(30)), Some(secs(5))]),
            trap_crashes: rng.pick(&[0, 500_000, 1_000_000]),
            tears: rng.pick(&[0, 500_000]),
            sub_sector: rng.chance(300_000),
            power_loss_every: rng.pick(&[None, Some(secs(60)), Some(secs(20))]),
            finish_every: rng.pick(&[None, Some(secs(20)), Some(secs(5))]),
        }
    }

    /// Longest the sessions can take to settle once faults stop: TCP giving up on a black hole,
    /// the slowest reconnect and handshake, and time over for retransmissions.
    fn settle_limit(&self, reconnect: Duration) -> Duration {
        self.give_up
            + self.stall_max
            + self.connect_max
            + self.restart_max
            + reconnect
            + self.keepalive * 10
            + Duration::from_secs(60)
    }
}

#[derive(Debug)]
enum Event {
    /// The client tries to connect, and its connect succeeds or fails. Each attempt carries the
    /// client's process generation, so one begun before a crash is dropped after it.
    Connect(u64),
    Established(u64),
    ConnectFailed(u64),
    Arrive {
        to: Side,
        conn: ConnId,
        bytes: Vec<u8>,
    },
    Fin {
        to: Side,
        conn: ConnId,
    },
    Fail {
        side: Side,
        conn: ConnId,
    },
    Read(Side, ConnId),
    Commands(Side, ConnId),
    Committed(Side, ConnId),
    Writable(Side, ConnId),
    Timer(Side, ConnId, SimTime),
    SendOrder(Side),
    Burst,
    /// One side's application ends the connection with a Terminate.
    Terminate,
    /// One side's application finishes sending, ending the logical session.
    Finish,
    Reset,
    BlackHole,
    Stall,
    GiveUp(ConnId),
    /// A node's process crashes, and later restarts.
    Crash,
    Restart(Side),
    /// A trap is set in the next store call of a kind on one node.
    Trap,
    /// The OS writes back both disk stores (without sync).
    WriteBack,
    /// A node loses power (a disk store without sync).
    PowerLoss,
}

/// What's scheduled for one connection's driver, so each kind of wake-up is queued once.
#[derive(Default)]
struct Pending {
    read: bool,
    commands: bool,
    commit: bool,
    writable: bool,
    timer: Option<SimTime>,
}

struct World {
    options: Options,
    faults: Faults,
    clocks: Clocks,
    rng: Rng,
    /// How long commits take, on seeds whose stores are slow.
    commit_rng: Option<Rng>,
    reconnect: ReconnectPolicy,
    reconnect_rng: Rng,
    reconnect_attempt: u32,
    queue: Queue<Event>,
    nodes: [Node; 2],
    storage: [Arc<LedgerStorage>; 2],
    _dir: Option<tempfile::TempDir>,
    net: Net,
    checker: Checker,
    pending: BTreeMap<(Side, ConnId), Pending>,
    reset: BTreeSet<ConnId>,
    black_holed: BTreeSet<ConnId>,
    connecting: bool,
    /// Each node is down after a crash, until it restarts.
    down: [bool; 2],
    /// The client's process generation: one more after each crash.
    generation: u64,
    busy_end: SimTime,
    in_flight: usize,
    next_id: u64,
    digest: u64,
    events: u64,
    connections: u64,
    refused: u64,
    trace: Vec<String>,
}

/// Runs one seed of the FIXP simulation.
pub fn run(options: &Options) -> Result<Report, Failure> {
    let mut world = World::new(options.clone());
    match world.run() {
        Ok(()) => Ok(Report {
            digest: world.digest,
            events: world.events,
            committed: [Side::Initiator, Side::Acceptor].map(|s| world.checker.committed(s)),
            connections: world.connections,
            resent: world.checker.resent,
            redelivered: world
                .nodes
                .iter()
                .map(|n| n.app.deliveries.lock().unwrap().iter().filter(|d| d.redelivered).count())
                .sum(),
            refused: world.refused,
            trace: world.trace,
        }),
        Err(violation) => Err(Failure {
            seed: options.seed,
            at: world.clocks.now(),
            violation,
            trace: std::mem::take(&mut world.trace),
            fixp: true,
        }),
    }
}

/// Each side's store, memory or disk, wrapped to keep a ledger, and slow to commit if `slow`.
fn stores(disk: bool, slow: bool, early: bool) -> (Option<tempfile::TempDir>, [Arc<LedgerStorage>; 2]) {
    let dir = disk.then(|| tempfile::tempdir().expect("a temp dir"));
    let ledger = |name: &str, early: bool| {
        let (store, files): (Arc<dyn SessionStorage>, _) = match &dir {
            None => (Arc::new(MemoryStorage::new()), None),
            Some(dir) => {
                let path = dir.path().join(name);
                let store = DiskStorage::new(&path, false).expect("a store directory");
                (Arc::new(store), Some(DiskFiles::new(path)))
            }
        };
        let storage = LedgerStorage::new(store, files);
        Arc::new(if slow || early { storage.slow_commits(early) } else { storage })
    };
    let storage = [ledger("client", false), ledger("server", early)];
    (dir, storage)
}

/// The client and the server, configured as `faults` says, over `storage`.
fn nodes(faults: &Faults, storage: &[Arc<LedgerStorage>; 2], clocks: &Clocks, plant: Option<Plant>) -> [Node; 2] {
    let config = |role| {
        let mut config = FixpConfig::new(role);
        config.client_flow = faults.client_flow;
        config.server_flow = faults.server_flow;
        config.keepalive = faults.keepalive;
        config.max_retransmit = faults.max_retransmit;
        config.send_queue = faults.send_queue;
        config.clock = clocks.wall_clock();
        config
    };
    let node = |side: Side, config, app| {
        let store: Arc<dyn SessionStorage> = storage[side.index()].clone();
        let registry = Arc::new(FixpRegistry::with_storage(store).with_clock(clocks.wall_clock()));
        Node::new(side, config, registry, app, clocks.clone())
    };
    [
        node(Side::Initiator, config(Role::Client(ClientConfig::new("CLIENT", "SERVER"))), RecordingApp::client()),
        node(Side::Acceptor, config(Role::Server(ServerConfig::new("SERVER"))), RecordingApp::server(plant)),
    ]
}

impl World {
    fn new(options: Options) -> Self {
        let clocks = Clocks::new();
        let mut rng = Rng::new(options.seed);
        let faults = Faults::draw(&mut rng.fork());
        let mut reconnect_rng = Rng::new(options.seed ^ 0x7ec0_22ec);
        let reconnect = crate::world::reconnect_policy(
            Duration::from_millis(reconnect_rng.between(100, 5_000)),
            &mut reconnect_rng,
        );
        let mut commit_rng = Rng::new(options.seed ^ 0x00c0_ff17);
        let early = options.plant == Some(Plant::EarlyCommit);
        let slow = commit_rng.chance(500_000) || early;
        let (dir, storage) = stores(faults.disk, slow, early);
        if options.plant == Some(Plant::ForgetMessages) {
            storage[Side::Acceptor.index()].forget_messages_from(PLANTED_AT);
        }
        let nodes = nodes(&faults, &storage, &clocks, options.plant);
        let net = Net::new(faults.net.clone(), rng.fork());
        let mut world = Self {
            checker: Checker::new([faults.client_flow, faults.server_flow]),
            busy_end: SimTime::from_duration(options.busy),
            options,
            faults,
            clocks,
            rng,
            commit_rng: slow.then_some(commit_rng),
            reconnect,
            reconnect_rng,
            reconnect_attempt: 0,
            queue: Queue::new(),
            nodes,
            storage,
            _dir: dir,
            net,
            pending: BTreeMap::new(),
            reset: BTreeSet::new(),
            black_holed: BTreeSet::new(),
            connecting: true,
            down: [false; 2],
            generation: 0,
            in_flight: 0,
            next_id: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            events: 0,
            connections: 0,
            refused: 0,
            trace: Vec::new(),
        };
        world.schedule_start();
        world
    }

    /// The first events: a trace header, the first connect, the workload and the faults.
    fn schedule_start(&mut self) {
        let header = format!("seed {} (FIXP): {:?}, {:?}", self.options.seed, self.reconnect, self.faults);
        self.record(&header);
        self.queue.push(SimTime(0), Event::Connect(0));
        if self.faults.disk && !self.faults.sync {
            self.queue.push(SimTime(0), Event::WriteBack);
            if let Some(every) = self.faults.power_loss_every {
                let at = self.after_about(SimTime(0), every);
                self.queue.push(at, Event::PowerLoss);
            }
        }
        for side in [Side::Initiator, Side::Acceptor] {
            if self.flow(side) != FlowType::None {
                self.queue.push(SimTime(0), Event::SendOrder(side));
            }
        }
        for (every, event) in [
            (self.faults.reset_every, Event::Reset),
            (self.faults.black_hole_every, Event::BlackHole),
            (self.faults.stall_every, Event::Stall),
            (self.faults.terminate_every, Event::Terminate),
            (self.faults.finish_every, Event::Finish),
            (self.faults.crash_every, Event::Crash),
            (self.faults.trap_every, Event::Trap),
            (self.faults.burst_every.filter(|_| self.flow(Side::Initiator) != FlowType::None), Event::Burst),
        ] {
            if let Some(every) = every {
                let at = self.after_about(SimTime(0), every);
                self.queue.push(at, event);
            }
        }
    }

    fn flow(&self, side: Side) -> FlowType {
        match side {
            Side::Initiator => self.faults.client_flow,
            Side::Acceptor => self.faults.server_flow,
        }
    }

    /// The wait before the client connects again, as `FixpInitiator::run`'s backoff gives it.
    fn reconnect_delay(&mut self, established: bool) -> Duration {
        if established {
            self.reconnect_attempt = 0;
        }
        let delay = self.reconnect.delay(self.reconnect_attempt, self.reconnect_rng.next_u64());
        self.reconnect_attempt = self.reconnect_attempt.saturating_add(1);
        delay
    }

    /// Somewhere between half and one and a half times `mean` after `now`.
    fn after_about(&mut self, now: SimTime, mean: Duration) -> SimTime {
        let mean = SimTime::from_duration(mean).0;
        SimTime(now.0 + self.rng.between(mean / 2, mean + mean / 2))
    }

    fn run(&mut self) -> Result<(), Violation> {
        let end = self.busy_end.after(self.options.quiet);
        let limit = end.after(self.faults.settle_limit(self.reconnect.max));
        let mut same_time = (SimTime(0), 0u64);
        while let Some(at) = self.queue.peek_time() {
            same_time = if at == same_time.0 { (at, same_time.1 + 1) } else { (at, 1) };
            if same_time.1 > MAX_EVENTS_AT_ONCE || self.events > MAX_EVENTS {
                return Err(Violation {
                    rule: "runaway",
                    detail: format!("{} events, {} at {at}", self.events, same_time.1),
                });
            }
            // A seed that lost data to a power loss isn't required to settle.
            if at > end && (self.checker.is_lossy() || self.idle() && self.check_settled().is_ok()) {
                return Ok(());
            }
            if at > limit {
                let reason = self.check_settled().err().map_or_else(|| "still busy".into(), |v| v.detail);
                return Err(Violation { rule: "liveness", detail: format!("not settled by {limit}: {reason}") });
            }
            let (at, event) = self.queue.pop().expect("peeked");
            self.clocks.advance_to(at);
            self.events += 1;
            self.record(&describe(&event));
            self.dispatch(event, at)?;
            // A trap's crash kills the process when its call is made, not at some later step.
            assert!(
                self.storage.iter().all(|s| !s.crash_pending()),
                "seed {} at {at}: a trap's crash outlived the event",
                self.options.seed
            );
            self.check_all()?;
        }
        // Timers keep a running session's driver waking, so an empty queue means every driver is
        // stuck.
        self.check_settled()
            .map_err(|v| Violation { rule: "stuck", detail: format!("nothing left to happen ({})", v.detail) })
    }

    #[expect(clippy::too_many_lines, reason = "one arm per event, as the FIX world's dispatch")]
    fn dispatch(&mut self, event: Event, now: SimTime) -> Result<(), Violation> {
        let busy = now < self.busy_end;
        match event {
            // A crashed client's attempt went with it; its restart starts again.
            Event::Connect(g) | Event::ConnectFailed(g) | Event::Established(g) if g != self.generation => {}
            // A crashed server's port refuses connections.
            Event::Connect(g) | Event::Established(g) if self.down[Side::Acceptor.index()] => {
                self.queue.push(now.after(Duration::from_millis(1)), Event::ConnectFailed(g));
            }
            Event::Connect(g) => {
                if busy && self.rng.chance(self.faults.refuse) {
                    self.queue.push(now.after(Duration::from_millis(1)), Event::ConnectFailed(g));
                } else {
                    let connect_max =
                        if busy { self.faults.connect_max } else { self.faults.connect_max.min(CONNECT_TIMEOUT / 2) };
                    let takes = Duration::from_nanos(self.rng.between(0, SimTime::from_duration(connect_max).0));
                    if takes >= CONNECT_TIMEOUT {
                        self.queue.push(now.after(CONNECT_TIMEOUT), Event::ConnectFailed(g));
                    } else {
                        self.queue.push(now.after(takes), Event::Established(g));
                    }
                }
            }
            Event::ConnectFailed(g) => {
                let delay = self.reconnect_delay(false);
                self.queue.push(now.after(delay), Event::Connect(g));
            }
            Event::Established(_) => {
                assert!(self.nodes[Side::Initiator.index()].conns().next().is_none(), "the client connects once");
                self.connecting = false;
                self.connections += 1;
                let conn = self.net.connect();
                let effects = self.nodes[Side::Acceptor.index()].start(conn, now);
                self.apply(Side::Acceptor, conn, effects, now)?;
                let effects = self.nodes[Side::Initiator.index()].start(conn, now);
                self.apply(Side::Initiator, conn, effects, now)?;
            }
            Event::Arrive { to, conn, bytes } => {
                self.in_flight -= 1;
                if !self.reset.contains(&conn) {
                    self.nodes[to.index()].receive(conn, &bytes);
                }
                self.after(to, conn, now)?;
            }
            Event::Fin { to, conn } => {
                self.in_flight -= 1;
                if !self.reset.contains(&conn) {
                    self.nodes[to.index()].fin(conn);
                }
                self.after(to, conn, now)?;
            }
            Event::Fail { side, conn } => {
                self.in_flight -= 1;
                if self.nodes[side.index()].fail(conn, now) {
                    self.ended(side, conn, now);
                }
                self.after(side, conn, now)?;
            }
            Event::Read(side, conn) => {
                self.pending_for(side, conn).read = false;
                let effects = self.nodes[side.index()].read(conn, now);
                self.apply(side, conn, effects, now)?;
            }
            Event::Commands(side, conn) => {
                self.pending_for(side, conn).commands = false;
                let effects = self.nodes[side.index()].commands(conn, now);
                self.apply(side, conn, effects, now)?;
            }
            Event::Committed(side, conn) => {
                if self.pending.get(&(side, conn)).is_some_and(|p| p.commit) {
                    self.pending_for(side, conn).commit = false;
                    let effects = self.nodes[side.index()].committed(conn, now);
                    self.apply(side, conn, effects, now)?;
                }
            }
            Event::Writable(side, conn) => {
                self.pending_for(side, conn).writable = false;
                self.write_out(side, conn, now);
                self.after(side, conn, now)?;
            }
            Event::Timer(side, conn, at) => {
                if self.pending.get(&(side, conn)).is_some_and(|p| p.timer == Some(at)) {
                    self.pending_for(side, conn).timer = None;
                    let effects = self.nodes[side.index()].timer(conn, now);
                    self.apply(side, conn, effects, now)?;
                }
            }
            Event::SendOrder(side) => {
                let id = self.fresh_id();
                self.nodes[side.index()].app.send(id);
                self.after_all(side, now)?;
                if busy {
                    let (least, most) = if side == Side::Initiator { (1, 200) } else { (50, 1_000) };
                    let next = now.after(Duration::from_millis(self.rng.between(least, most)));
                    self.queue.push(next, Event::SendOrder(side));
                }
            }
            Event::Burst => {
                for _ in 0..self.rng.between(1, 500) {
                    let id = self.fresh_id();
                    if self.nodes[Side::Initiator.index()].app.send(id) == Sent::Full {
                        self.refused += 1;
                    }
                }
                self.after_all(Side::Initiator, now)?;
                self.again(busy, now, self.faults.burst_every, Event::Burst);
            }
            Event::Terminate => {
                let side = if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor };
                let handle = self.nodes[side.index()].app.handle.lock().unwrap().clone();
                if let Some(handle) = handle {
                    // Not connected: nothing to end.
                    let _ = handle.logout(None);
                }
                self.after_all(side, now)?;
                self.again(busy, now, self.faults.terminate_every, Event::Terminate);
            }
            Event::Finish => {
                let side = self.pick_side();
                let handle = self.nodes[side.index()].app.handle.lock().unwrap().clone();
                if let Some(handle) = handle {
                    // Not connected: nothing to finish.
                    let _ = handle.finish();
                }
                self.after_all(side, now)?;
                self.again(busy, now, self.faults.finish_every, Event::Finish);
            }
            Event::Reset => {
                if let Some(conn) = self.current_conn()
                    && self.net.reset(conn)
                {
                    self.reset.insert(conn);
                    for side in [Side::Initiator, Side::Acceptor] {
                        let at = now.after(Duration::from_micros(self.rng.between(0, 1_000)));
                        self.in_flight += 1;
                        self.queue.push(at, Event::Fail { side, conn });
                    }
                }
                self.again(busy, now, self.faults.reset_every, Event::Reset);
            }
            Event::BlackHole => {
                if let Some(conn) = self.current_conn().filter(|c| !self.black_holed.contains(c)) {
                    self.net.black_hole(conn);
                    self.black_holed.insert(conn);
                    self.queue.push(now.after(self.faults.give_up), Event::GiveUp(conn));
                }
                self.again(busy, now, self.faults.black_hole_every, Event::BlackHole);
            }
            Event::Stall => {
                if let Some(conn) = self.current_conn() {
                    let from = if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor };
                    let lasts = self.rng.between(0, SimTime::from_duration(self.faults.stall_max).0);
                    self.net.stall(conn, from, SimTime(now.0 + lasts));
                }
                self.again(busy, now, self.faults.stall_every, Event::Stall);
            }
            Event::GiveUp(conn) => {
                self.net.reset(conn);
                for side in [Side::Initiator, Side::Acceptor] {
                    if self.nodes[side.index()].fail(conn, now) {
                        self.ended(side, conn, now);
                    }
                }
            }
            Event::Crash | Event::Restart(_) | Event::Trap | Event::WriteBack | Event::PowerLoss => {
                self.process_fault(event, now, busy);
            }
        }
        Ok(())
    }

    /// The faults that strike a node's process or its store.
    fn process_fault(&mut self, event: Event, now: SimTime, busy: bool) {
        match event {
            Event::Crash => {
                let side = self.pick_side();
                if !self.down[side.index()] {
                    self.crash(side, now);
                }
                self.again(busy, now, self.faults.crash_every, Event::Crash);
            }
            Event::Restart(side) => {
                self.down[side.index()] = false;
                let storage: Arc<dyn SessionStorage> = self.storage[side.index()].clone();
                let registry = Arc::new(FixpRegistry::with_storage(storage).with_clock(self.clocks.wall_clock()));
                self.nodes[side.index()].restart(registry);
                if side == Side::Initiator {
                    // A new process: its backoff starts again.
                    self.reconnect_attempt = 0;
                    self.connecting = true;
                    self.queue.push(now, Event::Connect(self.generation));
                }
            }
            Event::Trap => {
                let side = self.pick_side();
                let trap = self.draw_trap();
                self.record(&format!("trap {side:?} {trap:?}"));
                self.storage[side.index()].arm(trap);
                self.again(busy, now, self.faults.trap_every, Event::Trap);
            }
            Event::WriteBack => {
                for storage in &self.storage {
                    storage.write_back();
                }
                if busy {
                    self.queue.push(now.after(WRITE_BACK_EVERY), Event::WriteBack);
                }
            }
            Event::PowerLoss => {
                let side = self.pick_side();
                if !self.down[side.index()] {
                    self.crash(side, now);
                    let (kept, new_seqnums) = (self.rng.between(0, 1000), self.rng.chance(500_000));
                    self.record(&format!("power lost {side:?}: kept {kept}/1000, new seqnums {new_seqnums}"));
                    self.storage[side.index()].lose_power(kept, new_seqnums);
                    self.checker.lose();
                }
                self.again(busy, now, self.faults.power_loss_every, Event::PowerLoss);
            }
            other => unreachable!("not a process fault: {other:?}"),
        }
    }

    /// A trap for one store call: it applies or not and fails, maybe crashing the process; on a
    /// disk store with sync, a crash may be a power loss tearing a commit's write.
    fn draw_trap(&mut self) -> Trap {
        let crash = self.rng.chance(self.faults.trap_crashes);
        let tear =
            (crash && self.faults.disk && self.faults.sync && self.rng.chance(self.faults.tears)).then(|| Tear {
                cut: self.rng.between(0, 1000),
                in_record: self.rng.chance(500_000).then(|| self.rng.between(0, 1000)),
                sub_sector: self.faults.sub_sector,
            });
        // Only a commit writes a disk store's files, so only a commit tears, and only a disk
        // store's commit can fail untaken.
        let calls: &[Call] = if self.faults.disk {
            &[Call::RecordOutgoing, Call::SetNextIncoming, Call::SetInFlight, Call::Commit]
        } else {
            &[Call::RecordOutgoing, Call::SetNextIncoming, Call::SetInFlight]
        };
        let call = self.rng.pick(calls);
        Trap { call: if tear.is_some() { Call::Commit } else { call }, applies: self.rng.chance(500_000), crash, tear }
    }

    fn pick_side(&mut self) -> Side {
        if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor }
    }

    /// `side`'s process dies: its sessions go without another write, the OS resets its
    /// connections (so the other end's fail), and it restarts after a while.
    fn crash(&mut self, side: Side, now: SimTime) {
        self.down[side.index()] = true;
        if side == Side::Initiator {
            self.connecting = false;
            self.generation += 1;
        }
        for conn in self.nodes[side.index()].crash() {
            self.pending.remove(&(side, conn));
            if self.net.reset(conn) {
                self.reset.insert(conn);
                let at = now.after(Duration::from_micros(self.rng.between(0, 1_000)));
                self.in_flight += 1;
                self.queue.push(at, Event::Fail { side: side.other(), conn });
            }
        }
        let after = Duration::from_nanos(self.rng.between(0, SimTime::from_duration(self.faults.restart_max).0));
        self.queue.push(now.after(after), Event::Restart(side));
    }

    fn again(&mut self, busy: bool, now: SimTime, every: Option<Duration>, event: Event) {
        if let Some(every) = every.filter(|_| busy) {
            let at = self.after_about(now, every);
            self.queue.push(at, event);
        }
    }

    /// The client's connection, if it has one.
    fn current_conn(&self) -> Option<ConnId> {
        self.nodes[Side::Initiator.index()].conns().next()
    }

    fn fresh_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    fn pending_for(&mut self, side: Side, conn: ConnId) -> &mut Pending {
        self.pending.entry((side, conn)).or_default()
    }

    /// What a driver step did: its output checked and written, or the connection closed.
    fn apply(&mut self, side: Side, conn: ConnId, effects: Effects, now: SimTime) -> Result<(), Violation> {
        match self.storage[side.index()].sprung() {
            // A trap that crashed the process: nothing this step produced is written.
            Some(true) => {
                self.record(&format!("crash {side:?} in a store call"));
                // The session went on to fail and say so, but the process died in the call.
                let seen = self.checker.ended_seen(side);
                self.nodes[side.index()].app.ended.lock().unwrap().truncate(seen);
                self.sync_ledger(side)?;
                self.crash(side, now);
                return Ok(());
            }
            Some(false) => self.checker.store_failed(side),
            None => {}
        }
        if effects.read > 0 {
            self.net.read(conn, side, effects.read);
            self.wake_writer(side.other(), conn, now);
        }
        let effects = match self.options.plant {
            Some(Plant::AlterResends) => Effects { output: alter_resends(&effects.output), ..effects },
            _ => effects,
        };
        if !effects.output.is_empty() {
            if self.options.verbose {
                let frames = super::wire::frames(&effects.output).unwrap_or_default();
                let decoded: Vec<_> = frames.iter().map(|f| super::wire::decode(f)).collect();
                self.trace.push(format!("{} {side:?} wrote on {conn:?}: {decoded:?}", self.clocks.now()));
            }
            self.sync_ledger(side)?;
            self.checker.written(side, conn, &effects.output)?;
            self.hash(&effects.output);
        }
        if effects.ended {
            self.close(side, conn, now);
        } else {
            self.write_out(side, conn, now);
        }
        self.after(side, conn, now)
    }

    /// Room in `writer`'s send buffer on `conn`: it writes more, if it has more.
    fn wake_writer(&mut self, writer: Side, conn: ConnId, now: SimTime) {
        if !self.nodes[writer.index()].unwritten(conn).is_empty() && !self.pending_for(writer, conn).writable {
            self.pending_for(writer, conn).writable = true;
            self.queue.push(now, Event::Writable(writer, conn));
        }
    }

    /// Writes as much of `side`'s output on `conn` as the send buffer takes, and closes the
    /// connection once a closed session's output has all gone.
    fn write_out(&mut self, side: Side, conn: ConnId, now: SimTime) {
        let unwritten = self.nodes[side.index()].unwritten(conn).to_vec();
        if !unwritten.is_empty() {
            let written = self.net.write(conn, side, &unwritten, now);
            for (at, bytes) in written.segments {
                self.in_flight += 1;
                self.queue.push(at, Event::Arrive { to: side.other(), conn, bytes });
            }
            self.nodes[side.index()].wrote(conn, written.accepted);
        }
        if self.nodes[side.index()].finished(conn) {
            self.nodes[side.index()].remove(conn);
            self.close(side, conn, now);
        }
    }

    /// `side`'s driver for `conn` has returned: its end of the connection closes.
    fn close(&mut self, side: Side, conn: ConnId, now: SimTime) {
        if let Some(at) = self.net.close(conn, side, now) {
            self.in_flight += 1;
            self.queue.push(at, Event::Fin { to: side.other(), conn });
        }
        self.ended(side, conn, now);
    }

    /// `side`'s connection `conn` ended. A writer with output still to go on its other end fails,
    /// as a write into a closed socket does; the client reconnects after its interval.
    fn ended(&mut self, side: Side, conn: ConnId, now: SimTime) {
        self.pending.remove(&(side, conn));
        if !self.black_holed.contains(&conn) && !self.nodes[side.other().index()].unwritten(conn).is_empty() {
            self.in_flight += 1;
            self.queue.push(now.after(Duration::from_micros(100)), Event::Fail { side: side.other(), conn });
        }
        if side == Side::Initiator {
            let established = self.nodes[side.index()].had_established(conn);
            if !self.connecting && !self.down[side.index()] {
                self.connecting = true;
                let delay = self.reconnect_delay(established);
                self.queue.push(now.after(delay), Event::Connect(self.generation));
            }
        }
    }

    /// Checks what both stores recorded, both applications received, how their connections
    /// ended, and the receipts they've been answered, since the last call.
    fn check_all(&mut self) -> Result<(), Violation> {
        for side in [Side::Initiator, Side::Acceptor] {
            self.sync_ledger(side)?;
        }
        for side in [Side::Initiator, Side::Acceptor] {
            let app = self.nodes[side.index()].app.clone();
            self.checker.delivered(side, &app.deliveries.lock().unwrap())?;
            self.checker.ended(side, &app.ended.lock().unwrap())?;
            self.checker.not_applied(side, &app.not_applied.lock().unwrap())?;
            let mut receipts = app.receipts.lock().unwrap();
            let mut i = 0;
            while i < receipts.len() {
                match receipts[i].1.try_outcome() {
                    Some(outcome) => {
                        self.checker.receipt(side, receipts[i].0, &outcome)?;
                        receipts.swap_remove(i);
                    }
                    None => i += 1,
                }
            }
        }
        Ok(())
    }

    fn sync_ledger(&mut self, side: Side) -> Result<(), Violation> {
        let ledger = self.storage[side.index()].ledger.lock().unwrap();
        self.checker.stored(side, &ledger)
    }

    fn after_all(&mut self, side: Side, now: SimTime) -> Result<(), Violation> {
        let conns: Vec<ConnId> = self.nodes[side.index()].conns().collect();
        for conn in conns {
            self.after(side, conn, now)?;
        }
        Ok(())
    }

    /// Checks, and schedules what `conn`'s driver on `side` would do next.
    fn after(&mut self, side: Side, conn: ConnId, now: SimTime) -> Result<(), Violation> {
        self.check_all()?;
        let Some(wants) = self.nodes[side.index()].wants(conn) else {
            self.pending.remove(&(side, conn));
            return Ok(());
        };
        let mut push = Vec::new();
        let pending = self.pending.entry((side, conn)).or_default();
        if wants.commit && !pending.commit {
            pending.commit = true;
            let rng = self.commit_rng.as_mut().expect("only slow stores leave commits under way");
            let most = if rng.chance(100_000) { SLOW_COMMIT_TIME } else { COMMIT_TIME };
            let took = Duration::from_micros(rng.between(0, most.as_micros().try_into().expect("short")));
            push.push((now.after(took), Event::Committed(side, conn)));
        }
        if wants.read && !pending.read {
            pending.read = true;
            push.push((now, Event::Read(side, conn)));
        }
        if wants.commands && !pending.commands {
            pending.commands = true;
            push.push((now, Event::Commands(side, conn)));
        }
        if wants.timer != pending.timer {
            pending.timer = wants.timer;
            if let Some(at) = wants.timer {
                push.push((at.max(now), Event::Timer(side, conn, at)));
            }
        }
        for (at, event) in push {
            self.queue.push(at, event);
        }
        Ok(())
    }

    /// Nothing on the network, no connect under way, and nothing for any driver to do but wait
    /// for its timer.
    fn idle(&self) -> bool {
        self.in_flight == 0
            && !self.connecting
            && !self.down.iter().any(|d| *d)
            && self.pending.values().all(|p| !p.read && !p.commands && !p.writable && !p.commit)
    }

    /// Liveness: one connection, established on both sides, every send answered, and everything
    /// each side sent delivered or accounted for, with the sequence numbers agreed.
    fn check_settled(&self) -> Result<(), Violation> {
        let fail = |detail: String| Err(Violation { rule: "liveness", detail });
        if self.checker.is_lossy() {
            return Ok(());
        }
        for node in &self.nodes {
            if let Some((id, _)) = node.app.receipts.lock().unwrap().first() {
                return fail(format!("{:?}'s receipt for {id} never resolved", node.side));
            }
            let sessions: Vec<_> = node.sessions().collect();
            match sessions.as_slice() {
                [s] if s.is_established() => {}
                [_] => return fail(format!("{:?} isn't established", node.side)),
                other => return fail(format!("{:?} has {} sessions", node.side, other.len())),
            }
        }
        for side in [Side::Initiator, Side::Acceptor] {
            let not_applied = self.nodes[side.index()].app.not_applied.lock().unwrap();
            if let Some(missing) = self.checker.missing(side, &not_applied) {
                return fail(missing);
            }
        }
        Ok(())
    }

    /// Folds `bytes` into the digest (FNV-1a).
    fn hash(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.digest = (self.digest ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01B3);
        }
    }

    fn record(&mut self, line: &str) {
        let line = format!("{} {line}", self.clocks.now());
        self.hash(line.as_bytes());
        if self.options.verbose {
            self.trace.push(line);
        }
    }
}

/// The planted bug of retransmitted orders that arrive altered: each one's id flipped.
fn alter_resends(output: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(output.len());
    let mut replaying = 0;
    for frame in super::wire::frames(output).unwrap_or_default() {
        let mut frame = frame.to_vec();
        match super::wire::decode(&frame) {
            Ok(super::wire::Frame::Retransmission { count, .. }) => replaying = count,
            Ok(super::wire::Frame::Order(id)) if replaying > 0 => {
                // The order's id follows the framing and SBE headers.
                frame[14..22].copy_from_slice(&(id ^ 0xFFFF).to_le_bytes());
                replaying -= 1;
            }
            Ok(f) if f.is_application() && replaying > 0 => replaying -= 1,
            _ => {}
        }
        out.extend_from_slice(&frame);
    }
    out
}

/// An event for the trace: arrivals by their length, not their bytes.
fn describe(event: &Event) -> String {
    match event {
        Event::Arrive { to, conn, bytes } => format!("Arrive {to:?} {conn:?} {} bytes", bytes.len()),
        other => format!("{other:?}"),
    }
}
