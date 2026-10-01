//! One simulation: two nodes, the network between them, its faults and a workload, run from a
//! seed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use turbojet::codec::{Decoded, decode};
use turbojet::message::tags;
use turbojet::store::SessionStorage;
use turbojet::{InitiatorConfig, MemoryStorage, SessionConfig, SessionId, SessionRegistry};

use crate::Side;
use crate::app::{RecordingApp, order, report};
use crate::check::{Checker, Violation};
use crate::net::{ConnId, Net, Params};
use crate::node::{Effects, Node, Role};
use crate::queue::Queue;
use crate::rng::Rng;
use crate::time::{Clocks, SimTime};

/// How a simulation runs.
#[derive(Debug, Clone)]
pub struct Options {
    pub seed: u64,
    /// How long the workload and the faults run.
    pub busy: Duration,
    /// Then how long the sessions have, with no new work or faults, before they must have
    /// settled; they may take longer if they're still busy, up to a limit.
    pub quiet: Duration,
    /// Keep a trace of every event.
    pub verbose: bool,
}

impl Options {
    /// The run each seed gets on every push.
    pub fn per_push(seed: u64) -> Self {
        Self { seed, busy: Duration::from_secs(30), quiet: Duration::from_secs(10), verbose: false }
    }
}

/// A seed that passed.
#[derive(Debug)]
pub struct Report {
    /// A hash of every event and every byte written, to show a replay was identical.
    pub digest: u64,
    pub events: u64,
    /// Application messages each side sent with a MsgSeqNum.
    pub committed: [usize; 2],
    /// Connections made, and resends (PossDup messages) sent.
    pub connections: u64,
    pub resent: u64,
    pub trace: Vec<String>,
}

/// A seed that broke an invariant.
#[derive(Debug)]
pub struct Failure {
    pub seed: u64,
    pub at: SimTime,
    pub violation: Violation,
    pub trace: Vec<String>,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seed {} failed at {}: {}\n  replay: scripts/sim.sh 0 {} -v",
            self.seed, self.at, self.violation, self.seed
        )
    }
}

impl std::error::Error for Failure {}

/// One seed's faults.
#[derive(Debug, Clone)]
struct Faults {
    net: Params,
    /// Mean time between faults of each kind during the busy phase, if they happen at all.
    reset_every: Option<Duration>,
    black_hole_every: Option<Duration>,
    stall_every: Option<Duration>,
    /// Longest a stall lasts.
    stall_max: Duration,
    /// Chance in a million that a connect is refused.
    refuse: u32,
    /// Longest a connect takes; past the initiator's connect timeout, it fails.
    connect_max: Duration,
    /// How long TCP keeps retransmitting into a black hole before failing the connection.
    give_up: Duration,
    /// Resend this many sequence numbers per step, if set.
    resend_batch: Option<u64>,
}

impl Faults {
    fn draw(rng: &mut Rng, heartbeat: Duration) -> Self {
        let ms = Duration::from_millis;
        let secs = Duration::from_secs;
        Self {
            net: Params {
                latency: (Duration::from_micros(10), rng.pick(&[Duration::from_micros(100), ms(1), ms(20)])),
                capacity: rng.pick(&[4 * 1024, 64 * 1024, 1024 * 1024]),
                max_segments: rng.pick(&[1, 4, 16]),
            },
            reset_every: rng.pick(&[None, Some(secs(60)), Some(secs(20)), Some(secs(5))]),
            black_hole_every: rng.pick(&[None, None, Some(secs(60)), Some(secs(20))]),
            stall_every: rng.pick(&[None, Some(secs(30)), Some(secs(10))]),
            stall_max: heartbeat * 3,
            refuse: rng.pick(&[0, 100_000, 500_000]),
            connect_max: rng.pick(&[Duration::ZERO, secs(2), secs(15)]),
            give_up: secs(rng.between(60, 300)),
            resend_batch: None,
        }
    }

    /// Longest the sessions can take to settle once faults stop: TCP giving up on a black hole,
    /// then the slowest reconnect and logon, with time over for resends.
    fn settle_limit(&self, reconnect: Duration, heartbeat: Duration) -> Duration {
        self.give_up + self.stall_max + self.connect_max + reconnect + heartbeat * 10 + Duration::from_secs(60)
    }
}

#[derive(Debug)]
enum Event {
    /// The initiator tries to connect.
    Connect,
    /// Its connect succeeds, or fails (refused or timed out).
    Established,
    ConnectFailed,
    Arrive {
        to: Side,
        conn: ConnId,
        bytes: Vec<u8>,
    },
    /// The other end's close arrives.
    Fin {
        to: Side,
        conn: ConnId,
    },
    /// The connection fails under this end: a reset, or TCP giving up.
    Fail {
        side: Side,
        conn: ConnId,
    },
    Read(Side, ConnId),
    Commands(Side, ConnId),
    Resume(Side, ConnId),
    Unblock(Side, ConnId),
    Timer(Side, ConnId, SimTime),
    SendOrder,
    SendReport,
    Reset,
    BlackHole,
    Stall,
    /// TCP gives up on a black-holed connection.
    GiveUp(ConnId),
}

/// What's scheduled for one connection's driver, so each kind of wake-up is queued once.
#[derive(Default)]
struct Pending {
    read: bool,
    commands: bool,
    resume: bool,
    unblock: bool,
    timer: Option<SimTime>,
}

/// Both ends blocked writing to each other, each waiting for the other to read: the connection
/// driver writes all its output before reading again, so once both send buffers fill, neither
/// reads, and its timers can't fire. A known Turbojet bug (ROADMAP "Full-duplex connection I/O").
pub const WRITE_DEADLOCK: &str = "write deadlock";

/// Most events at one instant before the run counts as spinning: the drivers and network do
/// a bounded amount at once, so far fewer than this is normal.
const MAX_EVENTS_AT_ONCE: u64 = 100_000;
/// Most events in a run.
const MAX_EVENTS: u64 = 20_000_000;

struct World {
    options: Options,
    faults: Faults,
    clocks: Clocks,
    rng: Rng,
    queue: Queue<Event>,
    nodes: [Node; 2],
    storage: [Arc<MemoryStorage>; 2],
    ids: [SessionId; 2],
    net: Net,
    checker: Checker,
    pending: BTreeMap<(Side, ConnId), Pending>,
    /// Connections reset: whatever was on its way is lost.
    reset: BTreeSet<ConnId>,
    black_holed: BTreeSet<ConnId>,
    /// A connect is scheduled or under way.
    connecting: bool,
    /// Arrivals, closes and failures on their way.
    in_flight: usize,
    next_id: u64,
    digest: u64,
    events: u64,
    connections: u64,
    resent: u64,
    trace: Vec<String>,
}

pub fn run(options: &Options) -> Result<Report, Failure> {
    let mut world = World::new(options.clone());
    match world.run() {
        Ok(()) => Ok(Report {
            digest: world.digest,
            events: world.events,
            committed: [Side::Initiator, Side::Acceptor].map(|s| world.checker.committed(s).count()),
            connections: world.connections,
            resent: world.resent,
            trace: world.trace,
        }),
        Err(violation) => Err(Failure {
            seed: options.seed,
            at: world.clocks.now(),
            violation,
            trace: std::mem::take(&mut world.trace),
        }),
    }
}

fn session_id(sender: &str, target: &str) -> SessionId {
    SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: sender.into(), target_comp_id: target.into() }
}

impl World {
    fn new(options: Options) -> Self {
        let clocks = Clocks::new();
        let mut rng = Rng::new(options.seed);
        let config = |sender: &str| {
            let mut config = SessionConfig::new("FIX.4.4", sender);
            config.clock = clocks.wall_clock();
            config
        };
        let mut initiator = InitiatorConfig::new(config("CLIENT"), "GATEWAY");
        initiator.heartbeat_interval = Duration::from_secs(rng.pick(&[1, 2, 5, 10, 30]));
        initiator.reconnect_interval = Duration::from_millis(rng.between(100, 5_000));
        let mut faults = Faults::draw(&mut rng.fork(), initiator.heartbeat_interval);
        faults.resend_batch = None;
        let storage = [Arc::new(MemoryStorage::new()), Arc::new(MemoryStorage::new())];
        let registry = |side: Side| {
            let storage: Arc<dyn SessionStorage> = storage[side.index()].clone();
            Arc::new(SessionRegistry::new(storage).with_clock(clocks.wall_clock()))
        };
        let mut nodes = [
            Node::new(
                Side::Initiator,
                Role::Initiator(initiator),
                registry(Side::Initiator),
                RecordingApp::initiator(),
                clocks.clone(),
            ),
            Node::new(
                Side::Acceptor,
                Role::Acceptor(config("GATEWAY")),
                registry(Side::Acceptor),
                RecordingApp::acceptor(),
                clocks.clone(),
            ),
        ];
        for node in &mut nodes {
            node.resend_batch = faults.resend_batch;
        }
        let net = Net::new(faults.net.clone(), rng.fork());
        let mut world = Self {
            options,
            faults,
            clocks,
            rng,
            queue: Queue::new(),
            nodes,
            storage,
            ids: [session_id("CLIENT", "GATEWAY"), session_id("GATEWAY", "CLIENT")],
            net,
            checker: Checker::new(),
            pending: BTreeMap::new(),
            reset: BTreeSet::new(),
            black_holed: BTreeSet::new(),
            connecting: true,
            in_flight: 0,
            next_id: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            events: 0,
            connections: 0,
            resent: 0,
            trace: Vec::new(),
        };
        world.schedule_start();
        world
    }

    /// The first events: a trace header, the first connect, the workload and the faults.
    fn schedule_start(&mut self) {
        let header = format!(
            "seed {}: {:?}, {:?}",
            self.options.seed,
            self.nodes[0].initiator_config().map(|c| (c.heartbeat_interval, c.reconnect_interval)),
            self.faults
        );
        self.record(&header);
        self.queue.push(SimTime(0), Event::Connect);
        self.queue.push(SimTime(0), Event::SendOrder);
        self.queue.push(SimTime(0), Event::SendReport);
        for (every, event) in [
            (self.faults.reset_every, Event::Reset),
            (self.faults.black_hole_every, Event::BlackHole),
            (self.faults.stall_every, Event::Stall),
        ] {
            if let Some(every) = every {
                let at = self.after_about(SimTime(0), every);
                self.queue.push(at, event);
            }
        }
    }

    fn heartbeat(&self) -> Duration {
        self.nodes[0].initiator_config().expect("the initiator").heartbeat_interval
    }

    fn reconnect_interval(&self) -> Duration {
        self.nodes[0].initiator_config().expect("the initiator").reconnect_interval
    }

    /// Somewhere between half and one and a half times `mean` after `now`.
    fn after_about(&mut self, now: SimTime, mean: Duration) -> SimTime {
        let mean = SimTime::from_duration(mean).0;
        SimTime(now.0 + self.rng.between(mean / 2, mean + mean / 2))
    }

    fn run(&mut self) -> Result<(), Violation> {
        let busy_end = SimTime::from_duration(self.options.busy);
        let end = busy_end.after(self.options.quiet);
        let limit = end.after(self.faults.settle_limit(self.reconnect_interval(), self.heartbeat()));
        let mut same_time = (SimTime(0), 0u64);
        while let Some(at) = self.queue.peek_time() {
            same_time = if at == same_time.0 { (at, same_time.1 + 1) } else { (at, 1) };
            if same_time.1 > MAX_EVENTS_AT_ONCE || self.events > MAX_EVENTS {
                return Err(Violation {
                    rule: "runaway",
                    detail: format!("{} events, {} at {at}", self.events, same_time.1),
                });
            }
            if at > end && self.idle() && self.check_settled().is_ok() {
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
            self.dispatch(event, at, busy_end)?;
        }
        // Timers keep a running session's driver waking, so an empty queue means every driver is
        // stuck: blocked writing, with nothing to unblock it.
        self.check_settled().map_err(|v| {
            let all_blocked =
                self.nodes.iter().all(|n| n.conns().next().is_some() && n.conns().all(|c| n.is_blocked(c)));
            let rule = if all_blocked { WRITE_DEADLOCK } else { "stuck" };
            Violation {
                rule,
                detail: format!("nothing left to happen ({}); connections: {}", v.detail, self.describe_conns()),
            }
        })
    }

    /// Each node's connections: blocked, resending or logged on.
    fn describe_conns(&self) -> String {
        let mut out = Vec::new();
        for node in &self.nodes {
            for conn in node.conns() {
                let state = node.sessions_by_conn(conn).map_or("gone", |s| {
                    if s.is_resending() {
                        "resending"
                    } else if s.is_logged_on() {
                        "logged on"
                    } else {
                        "not logged on"
                    }
                });
                let blocked = if node.is_blocked(conn) { ", blocked writing" } else { "" };
                out.push(format!("{:?} conn {conn} {state}{blocked}", node.side));
            }
        }
        if self.connecting {
            out.push("initiator connecting".into());
        }
        out.join("; ")
    }

    #[expect(clippy::too_many_lines, reason = "one arm per event, each short")]
    fn dispatch(&mut self, event: Event, now: SimTime, busy_end: SimTime) -> Result<(), Violation> {
        let busy = now < busy_end;
        match event {
            Event::Connect => {
                let timeout = self.nodes[0].initiator_config().expect("the initiator").connect_timeout;
                if self.rng.chance(self.faults.refuse) {
                    self.queue.push(now.after(Duration::from_millis(1)), Event::ConnectFailed);
                } else {
                    let takes =
                        Duration::from_nanos(self.rng.between(0, SimTime::from_duration(self.faults.connect_max).0));
                    if takes >= timeout {
                        self.queue.push(now.after(timeout), Event::ConnectFailed);
                    } else {
                        self.queue.push(now.after(takes), Event::Established);
                    }
                }
            }
            Event::ConnectFailed => self.queue.push(now.after(self.reconnect_interval()), Event::Connect),
            Event::Established => {
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
                if self.nodes[side.index()].fail(conn) {
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
            Event::Resume(side, conn) => {
                self.pending_for(side, conn).resume = false;
                let effects = self.nodes[side.index()].resume(conn, now);
                self.apply(side, conn, effects, now)?;
            }
            Event::Unblock(side, conn) => {
                self.pending_for(side, conn).unblock = false;
                let effects = self.nodes[side.index()].unblock(conn);
                self.apply(side, conn, effects, now)?;
            }
            Event::Timer(side, conn, at) => {
                if self.pending.get(&(side, conn)).is_some_and(|p| p.timer == Some(at)) {
                    self.pending_for(side, conn).timer = None;
                    let effects = self.nodes[side.index()].timer(conn, now);
                    self.apply(side, conn, effects, now)?;
                }
            }
            Event::SendOrder => {
                let id = self.fresh_id("o");
                self.nodes[Side::Initiator.index()].app.send(order(&id));
                self.after_all(Side::Initiator, now)?;
                if busy {
                    let next = now.after(Duration::from_millis(self.rng.between(1, 200)));
                    self.queue.push(next, Event::SendOrder);
                }
            }
            Event::SendReport => {
                let id = self.fresh_id("u");
                self.nodes[Side::Acceptor.index()].app.send(report(&id, None));
                self.after_all(Side::Acceptor, now)?;
                if busy {
                    let next = now.after(Duration::from_millis(self.rng.between(50, 1_000)));
                    self.queue.push(next, Event::SendReport);
                }
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
                    if self.nodes[side.index()].fail(conn) {
                        self.ended(side, conn, now);
                    }
                }
            }
        }
        Ok(())
    }

    /// Schedules the next fault of a kind, while the workload runs.
    fn again(&mut self, busy: bool, now: SimTime, every: Option<Duration>, event: Event) {
        if let (true, Some(every)) = (busy, every) {
            let at = self.after_about(now, every);
            self.queue.push(at, event);
        }
    }

    /// The initiator's connection, the one faults hit.
    fn current_conn(&self) -> Option<ConnId> {
        self.nodes[Side::Initiator.index()].conns().next().filter(|c| self.net.is_open(*c))
    }

    fn fresh_id(&mut self, prefix: &str) -> String {
        self.next_id += 1;
        format!("{prefix}{}", self.next_id)
    }

    fn pending_for(&mut self, side: Side, conn: ConnId) -> &mut Pending {
        self.pending.entry((side, conn)).or_default()
    }

    /// Sends what a driver step wrote, blocking the driver on what the send buffer can't take,
    /// closes the connection once the output has gone if the session closed, and schedules what
    /// the driver waits for next.
    fn apply(&mut self, side: Side, conn: ConnId, effects: Effects, now: SimTime) -> Result<(), Violation> {
        if effects.read > 0 {
            self.net.read(conn, side, effects.read);
            if self.nodes[side.other().index()].is_blocked(conn) && !self.pending_for(side.other(), conn).unblock {
                self.pending_for(side.other(), conn).unblock = true;
                self.queue.push(now, Event::Unblock(side.other(), conn));
            }
        }
        if !effects.written.is_empty() {
            if !effects.resumed {
                self.observe(side, &effects.written, now)?;
            }
            let written = self.net.write(conn, side, &effects.written, now);
            for (at, bytes) in written.segments {
                self.in_flight += 1;
                self.queue.push(at, Event::Arrive { to: side.other(), conn, bytes });
            }
            if written.accepted < effects.written.len() {
                self.nodes[side.index()].block(conn, &effects.written[written.accepted..]);
                return self.after(side, conn, now);
            }
        }
        if effects.close {
            self.nodes[side.index()].closed(conn);
            if let Some(at) = self.net.close(conn, side, now) {
                self.in_flight += 1;
                self.queue.push(at, Event::Fin { to: side.other(), conn });
            }
            self.ended(side, conn, now);
        } else if effects.ended {
            self.ended(side, conn, now);
        }
        self.after(side, conn, now)
    }

    /// Checks and counts what `side` wrote.
    fn observe(&mut self, side: Side, bytes: &[u8], now: SimTime) -> Result<(), Violation> {
        let mut rest = bytes;
        while let Decoded::Message(msg, len) = decode(rest) {
            if msg.get(tags::POSS_DUP_FLAG) == Some("Y") {
                self.resent += 1;
            }
            rest = &rest[len..];
        }
        self.checker.written(side, bytes, now)?;
        self.hash(bytes);
        Ok(())
    }

    /// `side`'s connection `conn` ended. A writer blocked on its other end fails, as a write into
    /// a closed socket does (unless it's a black hole, which tells no one); the initiator
    /// reconnects after its interval.
    fn ended(&mut self, side: Side, conn: ConnId, now: SimTime) {
        self.pending.remove(&(side, conn));
        if !self.black_holed.contains(&conn) && self.nodes[side.other().index()].is_blocked(conn) {
            self.in_flight += 1;
            self.queue.push(now.after(Duration::from_micros(100)), Event::Fail { side: side.other(), conn });
        }
        if side == Side::Initiator && !self.connecting {
            self.connecting = true;
            self.queue.push(now.after(self.reconnect_interval()), Event::Connect);
        }
    }

    /// Checks deliveries after the application on `side` sent something, and schedules what
    /// each of its connections' drivers would do next.
    fn after_all(&mut self, side: Side, now: SimTime) -> Result<(), Violation> {
        let conns: Vec<ConnId> = self.nodes[side.index()].conns().collect();
        for conn in conns {
            self.after(side, conn, now)?;
        }
        Ok(())
    }

    /// Checks deliveries, and schedules what `conn`'s driver on `side` would do next.
    fn after(&mut self, side: Side, conn: ConnId, now: SimTime) -> Result<(), Violation> {
        for s in [Side::Initiator, Side::Acceptor] {
            let deliveries = self.nodes[s.index()].app.deliveries.lock().unwrap().clone();
            self.checker.delivered(s, &deliveries)?;
        }
        let Some(wants) = self.nodes[side.index()].wants(conn) else {
            self.pending.remove(&(side, conn));
            return Ok(());
        };
        let mut push = Vec::new();
        let pending = self.pending.entry((side, conn)).or_default();
        if wants.resume && !pending.resume {
            pending.resume = true;
            push.push((now, Event::Resume(side, conn)));
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
            && self.pending.values().all(|p| !p.read && !p.commands && !p.resume && !p.unblock)
    }

    /// Liveness: one connection, both sides logged on over it, every application message sent
    /// with a MsgSeqNum delivered, and the sequence numbers agreed.
    fn check_settled(&self) -> Result<(), Violation> {
        let fail = |detail: String| Err(Violation { rule: "liveness", detail });
        for node in &self.nodes {
            let sessions: Vec<_> = node.sessions().collect();
            match sessions.as_slice() {
                [s] if s.is_logged_on() && !s.is_resending() => {}
                [_] => return fail(format!("{:?} isn't logged on, or is still resending", node.side)),
                other => return fail(format!("{:?} has {} sessions", node.side, other.len())),
            }
        }
        for side in [Side::Initiator, Side::Acceptor] {
            let received: BTreeSet<&str> = self.checker.received_ids(side.other()).collect();
            if let Some(missing) = self.checker.committed(side).find(|id| !received.contains(id)) {
                return fail(format!("{side:?} sent {missing}, never delivered"));
            }
            let ours = self.storage[side.index()].open(&self.ids[side.index()]).expect("a memory store opens");
            let theirs = self.storage[side.other().index()].open(&self.ids[side.other().index()]).expect("opens");
            if ours.next_outgoing() != theirs.next_incoming() {
                return fail(format!(
                    "{side:?} sends {} next, but the other side expects {}",
                    ours.next_outgoing(),
                    theirs.next_incoming()
                ));
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

/// An event for the trace: messages as their type and MsgSeqNum (`D#4`, PossDup marked `D#4*`).
fn describe(event: &Event) -> String {
    match event {
        Event::Arrive { to, conn, bytes } => format!("arrive {to:?} conn {conn}: {}", summary(bytes)),
        other => format!("{other:?}"),
    }
}

/// The messages in `bytes`, briefly; a piece of one shows as its length.
pub fn summary(mut bytes: &[u8]) -> String {
    let mut out = Vec::new();
    while let Decoded::Message(msg, len) = decode(bytes) {
        let seq = msg.get(tags::MSG_SEQ_NUM).unwrap_or("?");
        let dup = if msg.get(tags::POSS_DUP_FLAG) == Some("Y") { "*" } else { "" };
        out.push(format!("{}#{seq}{dup}", msg.msg_type()));
        bytes = &bytes[len..];
    }
    if !bytes.is_empty() {
        out.push(format!("<{} bytes>", bytes.len()));
    }
    out.join(" ")
}
