//! One simulation: two nodes, the network between them, its faults and a workload, run from a
//! seed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use std::future::Future;
use std::pin::Pin;
use turbojet::codec::{Decoded, decode};
use turbojet::message::tags;
use turbojet::store::SessionStorage;

use turbojet::{
    CancelOnDisconnect, CancelTrigger, DiskStorage, HolidayCalendar, InboundLimit, InitiatorConfig, MemoryStorage,
    RateLimit, ReconnectPolicy, SequenceError, SequenceNumbers, SessionConfig, SessionHandle, SessionId,
    SessionRegistry, SessionSchedule,
};

use crate::Side;
use crate::app::{RecordingApp, Sent, order, report};
use crate::check::{Checker, Violation};
use crate::files::{DiskFiles, Tear};
use crate::hostile::Proxy;
use crate::net::{ConnId, Net, Params};
use crate::node::{Effects, Node, Role};
use crate::queue::Queue;
use crate::rng::Rng;
use crate::store::{Call, LedgerStorage, Trap};
use crate::time::{Clocks, SimTime, wall_start};

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
    /// A bug planted in the simulator for the checker to find (its self-tests).
    pub plant: Option<Plant>,
}

/// A bug planted in the simulator, standing for one in the engine, for the checker's self-tests:
/// each must be caught.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plant {
    /// The acceptor's application never sees its 10th delivery.
    DropDelivery,
    /// The acceptor's application sees its 10th delivery twice, unmarked.
    DuplicateDelivery,
    /// From its 10th, the acceptor's store keeps each message's number but not the message.
    ForgetMessages,
    /// Resent application messages reach the checker with their ids changed.
    AlterResends,
    /// The acceptor's store reports each commit done before it is: its changes reach the ledger
    /// only with the next one.
    EarlyCommit,
    /// The acceptor's application never hears of its first cancel on disconnect.
    SkipCancel,
    /// The acceptor's cancel-on-disconnect countdowns are run a millisecond after they end.
    LateCancel,
    /// The acceptor's application is told to cancel as its first session ends, whatever the
    /// trigger and grace.
    SpuriousCancel,
}

impl Options {
    /// The run each seed gets on every push.
    pub fn per_push(seed: u64) -> Self {
        Self { seed, busy: Duration::from_secs(30), quiet: Duration::from_secs(10), verbose: false, plant: None }
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
    /// Deliveries marked as possibly handled already (after a crash or store failure).
    pub redelivered: usize,
    /// Sends the applications had refused, their queue full.
    pub refused: u64,
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
    /// Resend this many sequence numbers per step, if set: small, so a resend after a lost
    /// connection takes several steps, with the driver's input paused between them.
    resend_batch: Option<u64>,
    /// Mean time between process crashes (of either node) while the workload runs, if any.
    crash_every: Option<Duration>,
    /// Longest a crashed node takes to restart.
    restart_max: Duration,
    /// Where the sessions keep their state.
    store: StoreKind,
    /// Mean time between traps set in a store call (of either node), if any, and the chance in a
    /// million that one crashes the process rather than just failing the call.
    trap_every: Option<Duration>,
    trap_crashes: u32,
    /// Disk stores with sync: the chance in a million that a trap that crashes is a power loss
    /// tearing the call's write, and whether a seqnums record can tear within a sector.
    tears: u32,
    sub_sector: bool,
    /// Disk stores without sync: mean time between power losses, if any.
    power_loss_every: Option<Duration>,
    /// Mean time between an application asking to log out, an operator skipping a side's
    /// outgoing numbers ahead, and an operator resetting both sides, if any.
    logout_every: Option<Duration>,
    skip_every: Option<Duration>,
    reset_both_every: Option<Duration>,
    /// Both sides run to a daily schedule whose first period ends in the busy phase.
    scheduled: bool,
    /// A hostile middlebox tampers with this many messages in a million during the busy phase.
    hostile: Option<u32>,
    /// Each side's send queue, small on some seeds so bursts fill it; and the mean time between
    /// bursts of orders, if any.
    send_queue: usize,
    burst_every: Option<Duration>,
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
            resend_batch: if rng.chance(330_000) { Some(rng.between(1, 8)) } else { None },
            crash_every: rng.pick(&[None, Some(secs(60)), Some(secs(15))]),
            restart_max: rng.pick(&[ms(100), secs(5), secs(30)]),
            store: rng.pick(&[
                StoreKind::Memory,
                StoreKind::Memory,
                StoreKind::Disk { sync: false },
                StoreKind::Disk { sync: true },
            ]),
            trap_every: rng.pick(&[None, Some(secs(30)), Some(secs(5))]),
            trap_crashes: rng.pick(&[0, 500_000, 1_000_000]),
            tears: rng.pick(&[0, 500_000]),
            sub_sector: rng.chance(300_000),
            power_loss_every: rng.pick(&[None, Some(secs(60)), Some(secs(20))]),
            logout_every: rng.pick(&[None, Some(secs(60)), Some(secs(15))]),
            skip_every: rng.pick(&[None, Some(secs(30)), Some(secs(10))]),
            reset_both_every: rng.pick(&[None, None, Some(secs(60))]),
            scheduled: rng.chance(250_000),
            hostile: rng.chance(200_000).then(|| rng.pick(&[10_000, 50_000])),
            send_queue: rng.pick(&[10_000, 10_000, 50, 5]),
            burst_every: rng.pick(&[None, Some(secs(10)), Some(secs(3))]),
        }
    }

    /// Longest the sessions can take to settle once faults stop: TCP giving up on a black hole,
    /// then the slowest reconnect and logon, with time over for resends.
    fn settle_limit(&self, reconnect: Duration, heartbeat: Duration) -> Duration {
        self.give_up
            + self.stall_max
            + self.connect_max
            + self.restart_max
            + reconnect
            + heartbeat * 10
            + Duration::from_secs(60)
    }
}

/// Where a seed's sessions keep their state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoreKind {
    /// `MemoryStorage`, standing for files that survive a process crash.
    Memory,
    /// `DiskStorage` in a directory of the seed's own. `sync` is what a power loss keeps: the
    /// store itself runs without fsync, since the simulator decides what a power loss loses.
    Disk { sync: bool },
}

#[derive(Debug)]
enum Event {
    /// The initiator tries to connect. Each attempt carries the initiator's process generation,
    /// so one begun before a crash is dropped after it.
    Connect(u64),
    /// Its connect succeeds, or fails (refused or timed out).
    Established(u64),
    ConnectFailed(u64),
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
    /// The store's commit under way finishes.
    Committed(Side, ConnId),
    /// Room in the send buffer: the driver writes more of its output.
    Writable(Side, ConnId),
    Timer(Side, ConnId, SimTime),
    /// The outbound window frees up for the sends it held, or the inbound window for the input
    /// it held.
    SendsFree(Side, ConnId, SimTime),
    InputFree(Side, ConnId, SimTime),
    /// A side's registry runs the cancel-on-disconnect countdowns that have ended, as the task
    /// `Acceptor` and `Initiator` spawn does.
    Cancels(Side, SimTime),
    SendOrder,
    SendReport,
    Reset,
    BlackHole,
    Stall,
    /// TCP gives up on a black-holed connection.
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
    /// The initiator's application sends a burst of orders at once.
    Burst,
    /// One side's application asks to log out.
    Logout,
    /// An operator moves one side's next outgoing number ahead.
    Skip,
    /// An operator resets both sides: logs them out, then resets both stores once neither is
    /// connected, checking again until then.
    ResetBoth,
    ResetBothStep,
}

/// What's scheduled for one connection's driver, so each kind of wake-up is queued once.
#[derive(Default)]
struct Pending {
    read: bool,
    commands: bool,
    resume: bool,
    commit: bool,
    writable: bool,
    timer: Option<SimTime>,
    sends_free: Option<SimTime>,
    input_free: Option<SimTime>,
}

/// Both ends stuck with output to write, each waiting for the other to read. The connection
/// driver used to write all its output before reading again, so once both send buffers filled,
/// neither read again; it now reads while output waits, and this would be a regression.
pub const WRITE_DEADLOCK: &str = "write deadlock";

/// How often a reset of both sides checks whether neither is connected yet, and how long it
/// waits for that before giving up.
const RESET_BOTH_CHECK: Duration = Duration::from_millis(100);
const RESET_BOTH_LIMIT: Duration = Duration::from_secs(60);

/// How long a slow store's commit takes, mostly (an fsync), and now and then (a busy device).
const COMMIT_TIME: Duration = Duration::from_millis(5);
const SLOW_COMMIT_TIME: Duration = Duration::from_millis(50);

/// How often the OS writes back a disk store without sync.
const WRITE_BACK_EVERY: Duration = Duration::from_secs(5);

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
    /// How long commits take, on seeds whose stores are slow to commit, apart from `rng` so that
    /// seeds keep the faults they had before slow commits were simulated.
    commit_rng: Option<Rng>,
    /// The initiator's reconnect jitter, apart from `rng` for the same reason.
    reconnect_rng: Rng,
    /// Connects in a row that haven't logged on, as Initiator::run counts them.
    reconnect_attempt: u32,
    queue: Queue<Event>,
    nodes: [Node; 2],
    storage: [Arc<LedgerStorage>; 2],
    /// The disk stores' directory, removed when the run ends.
    _dir: Option<tempfile::TempDir>,
    net: Net,
    checker: Checker,
    pending: BTreeMap<(Side, ConnId), Pending>,
    /// When each side's registry runs its cancel-on-disconnect countdowns next, if it has any.
    cancels_at: [Option<SimTime>; 2],
    /// Connections reset: whatever was on its way is lost.
    reset: BTreeSet<ConnId>,
    black_holed: BTreeSet<ConnId>,
    /// A connect is scheduled or under way.
    connecting: bool,
    /// Each node is down after a crash, until it restarts.
    down: [bool; 2],
    /// The initiator's process generation: one more after each crash.
    generation: u64,
    /// Each side's session.
    ids: [SessionId; 2],
    /// Operator requests under way, until they resolve.
    operators: Vec<Operator>,
    /// A reset of both sides under way, since when: the initiator doesn't connect meanwhile.
    resetting: Option<SimTime>,
    /// The daily schedule both sides run to, if any.
    schedule: Option<SessionSchedule>,
    /// When the workload and faults stop.
    busy_end: SimTime,
    /// The hostile middlebox, on a hostile seed.
    proxy: Option<Proxy>,
    /// Arrivals, closes and failures on their way.
    in_flight: usize,
    next_id: u64,
    digest: u64,
    events: u64,
    connections: u64,
    resent: u64,
    refused: u64,
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
            refused: world.refused,
            redelivered: world
                .nodes
                .iter()
                .map(|n| n.app.deliveries.lock().unwrap().iter().filter(|d| d.redelivered).count())
                .sum(),
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

/// On half the seeds, disk stores in segments of a few messages, keeping a few segments, so that
/// they rotate and evict all the time: the segment size and the budget. Drawn apart from the
/// world's random stream so that seeds keep the faults they had before.
fn segment_sizes(seed: u64) -> Option<(u64, u64)> {
    let mut rng = Rng::new(seed ^ 0x5e6_3e47);
    rng.chance(500_000).then(|| {
        let segment = rng.between(512, 4_096);
        (segment, segment * rng.between(2, 8))
    })
}

/// The rate limits a seed's sessions run with: on about a quarter of seeds each, the initiator's
/// outbound limit and the acceptor's inbound limit, which delays. Low enough that bursts of orders
/// (a few hundred at once) wait on them, and the steady flow (an order every 100 ms or so) now and
/// then, but high enough that a busy phase's orders get through in well under the time the
/// sessions have to settle. Drawn apart from the world's random stream so that seeds keep the
/// faults they had before.
fn limits(seed: u64) -> (Option<RateLimit>, Option<InboundLimit>) {
    let mut rng = Rng::new(seed ^ 0x7407_7e1e);
    let draw = |rng: &mut Rng| {
        let (messages, per) = rng.pick(&[(2, 50), (5, 100), (20, 250), (100, 1_000)]);
        RateLimit::new(messages, Duration::from_millis(per))
    };
    let outbound = rng.chance(250_000).then(|| draw(&mut rng));
    let inbound = rng.chance(250_000).then(|| InboundLimit::Delay(draw(&mut rng)));
    (outbound, inbound)
}

/// Cancel on disconnect for each side's session, on about two thirds of seeds each: either
/// trigger, with a grace period of up to 30 s, zero on some. Drawn apart from the world's random
/// stream so that seeds keep the faults they had before.
fn cancels(seed: u64) -> [Option<CancelOnDisconnect>; 2] {
    const _: () = assert!(MAX_GRACE.as_secs() <= turbojet::MAX_CANCEL_GRACE.as_secs());
    let mut rng = Rng::new(seed ^ 0xca2c_e100);
    [(); 2].map(|()| {
        let trigger = rng.pick(&[None, Some(CancelTrigger::Disconnect), Some(CancelTrigger::DisconnectOrLogout)])?;
        let most = u64::try_from(MAX_GRACE.as_millis()).expect("short");
        let grace = if rng.chance(200_000) { Duration::ZERO } else { Duration::from_millis(rng.between(1, most)) };
        Some(CancelOnDisconnect::new(trigger, grace))
    })
}

/// How many messages each side asks for per ResendRequest, chunked on about half the seeds each.
/// Drawn apart from the world's random stream, as for [`cancels`].
fn resend_chunks(seed: u64) -> [Option<u64>; 2] {
    let mut rng = Rng::new(seed ^ 0x2e5e_7d00);
    [(); 2].map(|()| rng.pick(&[None, None, None, Some(1), Some(2), Some(10)]))
}

/// The two nodes' roles, with the seed's rate limits, cancels on disconnect and resend chunks.
fn roles(mut initiator: InitiatorConfig, mut acceptor: SessionConfig, seed: u64) -> [Role; 2] {
    let (outbound, inbound) = limits(seed);
    initiator.session.outbound_limit = outbound;
    acceptor.inbound_limit = inbound;
    [initiator.session.cancel_on_disconnect, acceptor.cancel_on_disconnect] = cancels(seed);
    [initiator.session.resend_request_chunk, acceptor.resend_request_chunk] = resend_chunks(seed);
    [Role::Initiator(initiator), Role::Acceptor(acceptor)]
}

/// Each side's store, of `kind`, wrapped to keep a ledger, slow to commit if `slow`, the acceptor's
/// with the planted bug of committing early if `early`; disk stores in segments of `segments`
/// bytes, keeping `segments.1`, if given; and for disk stores, their directory.
fn stores(
    kind: StoreKind,
    slow: bool,
    early: bool,
    segments: Option<(u64, u64)>,
) -> (Option<tempfile::TempDir>, [Arc<LedgerStorage>; 2]) {
    let dir = matches!(kind, StoreKind::Disk { .. }).then(|| tempfile::tempdir().expect("a temp dir"));
    let store = |name: &str| -> Arc<dyn SessionStorage> {
        match &dir {
            None => Arc::new(MemoryStorage::new()),
            Some(dir) => {
                let disk = DiskStorage::new(dir.path().join(name), false).expect("a store directory");
                Arc::new(match segments {
                    Some((segment, max)) => disk.with_max_session_bytes(max).with_segment_bytes(segment),
                    None => disk,
                })
            }
        }
    };
    let ledger = |name: &str, early: bool| {
        let files = dir.as_ref().map(|dir| DiskFiles::new(dir.path().join(name)));
        let storage = LedgerStorage::new(store(name), files);
        Arc::new(if slow || early { storage.slow_commits(early) } else { storage })
    };
    let storage = [ledger("initiator", false), ledger("acceptor", early)];
    (dir, storage)
}

/// The two nodes, in `roles`, over `storage`.
fn nodes(
    roles: [Role; 2],
    storage: &[Arc<LedgerStorage>; 2],
    clocks: &Clocks,
    faults: &Faults,
    plant: Option<Plant>,
) -> [Node; 2] {
    let [initiator, acceptor] = roles;
    let node = |side: Side, role, app| {
        let store: Arc<dyn SessionStorage> = storage[side.index()].clone();
        let registry = Arc::new(SessionRegistry::new(store).with_clock(clocks.wall_clock()));
        let mut node = Node::new(side, role, registry, app, clocks.clone());
        node.resend_batch = faults.resend_batch;
        node
    };
    [
        node(Side::Initiator, initiator, RecordingApp::initiator(clocks.clone())),
        node(Side::Acceptor, acceptor, RecordingApp::acceptor(plant, clocks.clone())),
    ]
}

/// An operator request under way.
struct Operator {
    side: Side,
    /// The next outgoing number it moves to.
    to: u64,
    future: Pin<Box<dyn Future<Output = Result<SequenceNumbers, SequenceError>>>>,
}

/// Polls `future` once: its output if it's ready. Operator requests on a session that isn't
/// connected resolve at once; on one that is, once its driver has taken the command.
fn poll_once<F: Future + ?Sized>(future: Pin<&mut F>) -> Option<F::Output> {
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    match future.poll(&mut cx) {
        std::task::Poll::Ready(output) => Some(output),
        std::task::Poll::Pending => None,
    }
}

fn session_id(sender: &str, target: &str) -> SessionId {
    SessionId::new("FIX.4.4", sender, target)
}

/// For a scheduled seed, a daily schedule whose first period ends 20 s in and whose next starts
/// the next morning, or on half the seeds the morning after, the next day being a holiday; and
/// the busy phase running 20 s into that next period. Otherwise none, and the busy phase as the
/// options say. The holiday is drawn apart from the world's random stream so that seeds keep the
/// faults they had before.
fn schedule(faults: &Faults, options: &Options) -> (Option<SessionSchedule>, SimTime) {
    if !faults.scheduled {
        return (None, SimTime::from_duration(options.busy));
    }
    let time = |h, m, s| chrono::NaiveTime::from_hms_opt(h, m, s).expect("a time");
    let mut schedule = SessionSchedule::daily(time(8, 0, 0), time(9, 0, 20));
    if Rng::new(options.seed ^ 0x0401_1da7).chance(500_000) {
        let tomorrow = wall_start().date_naive() + chrono::Days::new(1);
        schedule = schedule.with_holidays(HolidayCalendar::new([tomorrow]));
    }
    let next = schedule.next_start(wall_start() + chrono::TimeDelta::seconds(21)).expect("a later period");
    let busy_end = SimTime::from_duration((next - wall_start()).to_std().expect("later")).after(options.busy);
    (Some(schedule), busy_end)
}

/// The initiator's reconnects, fixed or backing off from `reconnect`, chosen and jittered from
/// `rng`, apart from the world's random stream so that seeds keep the faults they had before
/// reconnects backed off.
fn reconnect_policy(reconnect: Duration, rng: &mut Rng) -> ReconnectPolicy {
    if rng.chance(500_000) {
        ReconnectPolicy::fixed(reconnect)
    } else {
        let max = reconnect * u32::try_from(rng.between(2, 20)).expect("small");
        let mut policy = ReconnectPolicy::exponential(reconnect, max);
        policy.jitter = rng.chance(500_000);
        policy
    }
}

impl World {
    fn new(options: Options) -> Self {
        let clocks = Clocks::new();
        let mut rng = Rng::new(options.seed);
        let heartbeat = Duration::from_secs(rng.pick(&[1, 2, 5, 10, 30]));
        let reconnect = Duration::from_millis(rng.between(100, 5_000));
        let faults = Faults::draw(&mut rng.fork(), heartbeat);
        let (schedule, busy_end) = schedule(&faults, &options);
        let config = |sender: &str| {
            let mut config = SessionConfig::new("FIX.4.4", sender);
            config.clock = clocks.wall_clock();
            config.schedule.clone_from(&schedule);
            config.send_queue = faults.send_queue;
            config
        };
        let mut initiator = InitiatorConfig::new(config("CLIENT"), "GATEWAY");
        initiator.heartbeat_interval = heartbeat;
        let mut reconnect_rng = Rng::new(options.seed ^ 0x7ec0_22ec);
        initiator.reconnect = reconnect_policy(reconnect, &mut reconnect_rng);
        let mut commit_rng = Rng::new(options.seed ^ 0x00c0_ff17);
        let early = options.plant == Some(Plant::EarlyCommit);
        let slow = commit_rng.chance(500_000) || early;
        let (dir, storage) = stores(faults.store, slow, early, segment_sizes(options.seed));
        if options.plant == Some(Plant::ForgetMessages) {
            storage[Side::Acceptor.index()].forget_messages_from(PLANTED_AT);
        }
        let roles = roles(initiator, config("GATEWAY"), options.seed);
        let nodes = nodes(roles, &storage, &clocks, &faults, options.plant);
        let net = Net::new(faults.net.clone(), rng.fork());
        let proxy = faults.hostile.map(|rate| Proxy::new(rate, rng.fork()));
        let mut world = Self {
            // Before `options` moves into the world.
            checker: Checker::new(cancels(options.seed)),
            options,
            faults,
            clocks,
            rng,
            commit_rng: slow.then_some(commit_rng),
            reconnect_rng,
            reconnect_attempt: 0,
            queue: Queue::new(),
            nodes,
            storage,
            _dir: dir,
            net,
            pending: BTreeMap::new(),
            cancels_at: [None; 2],
            reset: BTreeSet::new(),
            black_holed: BTreeSet::new(),
            connecting: true,
            down: [false; 2],
            generation: 0,
            ids: [session_id("CLIENT", "GATEWAY"), session_id("GATEWAY", "CLIENT")],
            operators: Vec::new(),
            resetting: None,
            schedule,
            busy_end,
            proxy,
            in_flight: 0,
            next_id: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            events: 0,
            connections: 0,
            resent: 0,
            refused: 0,
            trace: Vec::new(),
        };
        world.schedule_start();
        world
    }

    /// The first events: a trace header, the first connect, the workload and the faults.
    fn schedule_start(&mut self) {
        if self.faults.store == (StoreKind::Disk { sync: false }) {
            self.queue.push(SimTime(0), Event::WriteBack);
            if let Some(every) = self.faults.power_loss_every {
                let at = self.after_about(SimTime(0), every);
                self.queue.push(at, Event::PowerLoss);
            }
        }
        let mut header = format!(
            "seed {}: {:?}, {:?}",
            self.options.seed,
            self.nodes[0].initiator_config().map(|c| (c.heartbeat_interval, c.reconnect)),
            self.faults
        );
        // Only when there is one, so the headers (and digests) of other seeds stay as they were.
        if let Some(holiday) = self.schedule.as_ref().and_then(|s| s.holidays().last()) {
            header.push_str(&format!(", holiday {holiday}"));
        }
        // Likewise the limits.
        let (outbound, inbound) = limits(self.options.seed);
        if let Some(limit) = outbound {
            header.push_str(&format!(", initiator outbound limit {limit}"));
        }
        if let Some(InboundLimit::Delay(limit)) = inbound {
            header.push_str(&format!(", acceptor inbound limit {limit}, delayed"));
        }
        for node in &self.nodes {
            if let Some(CancelOnDisconnect { trigger, grace, .. }) = node.cancel_on_disconnect() {
                let side = node.side;
                header.push_str(&format!(", {side:?} cancels on {trigger:?} after {grace:?}"));
            }
        }
        self.record(&header);
        self.queue.push(SimTime(0), Event::Connect(0));
        self.queue.push(SimTime(0), Event::SendOrder);
        self.queue.push(SimTime(0), Event::SendReport);
        for (every, event) in [
            (self.faults.reset_every, Event::Reset),
            (self.faults.black_hole_every, Event::BlackHole),
            (self.faults.stall_every, Event::Stall),
            (self.faults.crash_every, Event::Crash),
            (self.faults.trap_every, Event::Trap),
            (self.faults.logout_every, Event::Logout),
            (self.faults.skip_every, Event::Skip),
            (self.faults.reset_both_every, Event::ResetBoth),
            (self.faults.burst_every, Event::Burst),
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

    fn reconnect_policy(&self) -> ReconnectPolicy {
        self.nodes[0].initiator_config().expect("the initiator").reconnect
    }

    /// The wait before the initiator connects again, as Initiator::run's backoff gives it:
    /// `logged_on` if the session that ended had logged on, which starts the count again.
    fn reconnect_delay(&mut self, logged_on: bool) -> Duration {
        if logged_on {
            self.reconnect_attempt = 0;
        }
        let delay = self.reconnect_policy().delay(self.reconnect_attempt, self.reconnect_rng.next_u64());
        self.reconnect_attempt = self.reconnect_attempt.saturating_add(1);
        delay
    }

    /// Somewhere between half and one and a half times `mean` after `now`.
    fn after_about(&mut self, now: SimTime, mean: Duration) -> SimTime {
        let mean = SimTime::from_duration(mean).0;
        SimTime(now.0 + self.rng.between(mean / 2, mean + mean / 2))
    }

    fn run(&mut self) -> Result<(), Violation> {
        let busy_end = self.busy_end;
        let end = busy_end.after(self.options.quiet);
        // With time for a countdown started as the sessions settle to end.
        let settle = self.faults.settle_limit(self.reconnect_policy().max, self.heartbeat());
        let limit = end.after(settle + MAX_GRACE);
        let mut same_time = (SimTime(0), 0u64);
        while let Some(at) = self.queue.peek_time() {
            same_time = if at == same_time.0 { (at, same_time.1 + 1) } else { (at, 1) };
            if same_time.1 > MAX_EVENTS_AT_ONCE || self.events > MAX_EVENTS {
                return Err(Violation {
                    rule: "runaway",
                    detail: format!("{} events, {} at {at}", self.events, same_time.1),
                });
            }
            // Countdowns under way run out first, so every one is checked.
            let counting_down = self.cancels_at.iter().any(Option::is_some);
            if at > end && !counting_down && (self.checker.is_lossy() || self.idle() && self.check_settled().is_ok()) {
                // A seed that lost data to a power loss isn't required to settle.
                return self.checker.no_countdowns();
            }
            // A seed that lost data needn't settle, and its sessions may keep ending and logging
            // back on within the grace period, a countdown always under way. Each was checked
            // after every event: none came due unfired.
            if at > limit && self.checker.is_lossy() {
                return Ok(());
            }
            if at > limit {
                let reason = self.check_settled().err().map_or_else(|| "still busy".into(), |v| v.detail);
                return Err(Violation { rule: "liveness", detail: format!("not settled by {limit}: {reason}") });
            }
            let (at, event) = self.queue.pop().expect("peeked");
            self.clocks.advance_to(at);
            self.events += 1;
            let described = describe(&event);
            self.record(&described);
            self.dispatch(event, at, busy_end)?;
            self.poll_operators()?;
            self.operator_traps(at)?;
            self.schedule_cancels(at);
            // A trap's crash kills the process when its call is made, not at some later step.
            assert!(
                self.storage.iter().all(|s| !s.crash_pending()),
                "seed {} at {at}: a trap's crash outlived {described}",
                self.options.seed
            );
            // Whatever path the event took, what it stored and delivered is checked before the next.
            self.check_all()?;
        }
        // Timers keep a running session's driver waking, so an empty queue means every driver is
        // stuck: blocked writing, with nothing to unblock it.
        self.check_settled().map_err(|v| {
            let all_blocked =
                self.nodes.iter().all(|n| n.conns().next().is_some() && n.conns().all(|c| !n.unwritten(c).is_empty()));
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
                let unwritten = node.unwritten(conn).len();
                out.push(format!("{:?} conn {conn} {state}, {unwritten} bytes unwritten", node.side));
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
            // A crashed initiator's attempt went with it; its restart starts again.
            Event::Connect(g) | Event::ConnectFailed(g) | Event::Established(g) if g != self.generation => {}
            // A crashed acceptor's port refuses connections.
            Event::Connect(g) | Event::Established(g) if self.down[Side::Acceptor.index()] => {
                self.queue.push(now.after(Duration::from_millis(1)), Event::ConnectFailed(g));
            }
            // While both sides are being reset, the initiator waits; the reset reconnects it.
            Event::Connect(_) if self.resetting.is_some() => self.connecting = false,
            // Outside the schedule, the initiator waits for the next period, as Initiator::run does.
            Event::Connect(g) if self.active_from(now) > now => {
                let at = self.active_from(now);
                self.queue.push(at, Event::Connect(g));
            }
            // Refused and slow connects are faults: they stop with the others, after the busy phase.
            Event::Connect(g) => {
                let timeout = self.nodes[0].initiator_config().expect("the initiator").connect_timeout;
                if busy && self.rng.chance(self.faults.refuse) {
                    self.queue.push(now.after(Duration::from_millis(1)), Event::ConnectFailed(g));
                } else {
                    let connect_max =
                        if busy { self.faults.connect_max } else { self.faults.connect_max.min(timeout / 2) };
                    let takes = Duration::from_nanos(self.rng.between(0, SimTime::from_duration(connect_max).0));
                    if takes >= timeout {
                        self.queue.push(now.after(timeout), Event::ConnectFailed(g));
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
                // As Initiator::run, one connection at a time.
                assert!(self.nodes[Side::Initiator.index()].conns().next().is_none(), "the initiator connects once");
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
                    let bytes = match &mut self.proxy {
                        // The middlebox reads what arrives at once, so the pipe drains as it does.
                        Some(proxy) => {
                            let passed = proxy.pass(conn, to, &bytes, busy);
                            self.net.read(conn, to, bytes.len());
                            self.wake_writer(to.other(), conn, now);
                            passed
                        }
                        None => bytes,
                    };
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
            Event::Resume(side, conn) => {
                self.pending_for(side, conn).resume = false;
                let effects = self.nodes[side.index()].resume(conn, now);
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
            Event::SendsFree(side, conn, at) => {
                if self.pending.get(&(side, conn)).is_some_and(|p| p.sends_free == Some(at)) {
                    self.pending_for(side, conn).sends_free = None;
                    let effects = self.nodes[side.index()].window_free(conn, now);
                    self.apply(side, conn, effects, now)?;
                }
            }
            Event::InputFree(side, conn, at) => {
                if self.pending.get(&(side, conn)).is_some_and(|p| p.input_free == Some(at)) {
                    self.pending_for(side, conn).input_free = None;
                    let effects = self.nodes[side.index()].window_free(conn, now);
                    self.apply(side, conn, effects, now)?;
                }
            }
            Event::Cancels(side, at) => {
                if self.cancels_at[side.index()] == Some(at) {
                    self.cancels_at[side.index()] = None;
                    self.nodes[side.index()].registry.run_due_cancels(self.clocks.instant(now));
                }
            }
            Event::SendOrder => {
                let id = self.fresh_id("o");
                self.nodes[Side::Initiator.index()].app.send(order(&id));
                self.after_all(Side::Initiator, now)?;
                if busy {
                    let next = now.after(Duration::from_millis(self.rng.between(1, 200)));
                    let next = self.active_from(next);
                    self.queue.push(next, Event::SendOrder);
                }
            }
            Event::SendReport => {
                let id = self.fresh_id("u");
                self.nodes[Side::Acceptor.index()].app.send(report(&id, None));
                self.after_all(Side::Acceptor, now)?;
                if busy {
                    let next = now.after(Duration::from_millis(self.rng.between(50, 1_000)));
                    let next = self.active_from(next);
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
            Event::Crash => {
                let side = if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor };
                if !self.down[side.index()] {
                    self.crash(side, now);
                }
                self.again(busy, now, self.faults.crash_every, Event::Crash);
            }
            Event::Trap => {
                let side = if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor };
                let crash = self.rng.chance(self.faults.trap_crashes);
                let tear = (crash
                    && self.faults.store == StoreKind::Disk { sync: true }
                    && self.rng.chance(self.faults.tears))
                .then(|| Tear {
                    cut: self.rng.between(0, 1000),
                    in_record: self.rng.chance(500_000).then(|| self.rng.between(0, 1000)),
                    sub_sector: self.faults.sub_sector,
                });
                // Only a commit writes a disk store's files, so only a commit tears; and only a disk
                // store's commit can fail without its changes taking effect.
                let calls: &[Call] = if matches!(self.faults.store, StoreKind::Disk { .. }) {
                    &[Call::RecordOutgoing, Call::SetNextIncoming, Call::SetInFlight, Call::Commit]
                } else {
                    &[Call::RecordOutgoing, Call::SetNextIncoming, Call::SetInFlight]
                };
                let call = self.rng.pick(calls);
                let trap = Trap {
                    call: if tear.is_some() { Call::Commit } else { call },
                    applies: self.rng.chance(500_000),
                    crash,
                    tear,
                };
                self.record(&format!("trap {side:?} {trap:?}"));
                self.storage[side.index()].arm(trap);
                self.again(busy, now, self.faults.trap_every, Event::Trap);
            }
            Event::WriteBack => {
                for storage in &self.storage {
                    storage.write_back();
                }
                if busy {
                    // Nothing changes outside the schedule, so neither does what's written back.
                    let next = self.active_from(now.after(WRITE_BACK_EVERY));
                    self.queue.push(next, Event::WriteBack);
                }
            }
            Event::PowerLoss => {
                let side = if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor };
                if !self.down[side.index()] {
                    self.crash(side, now);
                    let (kept, new_seqnums) = (self.rng.between(0, 1000), self.rng.chance(500_000));
                    self.record(&format!(
                        "power lost {side:?}: kept {kept}/1000 of the body since, new seqnums {new_seqnums}"
                    ));
                    self.storage[side.index()].lose_power(kept, new_seqnums);
                    self.checker.lose();
                }
                self.again(busy, now, self.faults.power_loss_every, Event::PowerLoss);
            }
            Event::Burst => {
                for _ in 0..self.rng.between(1, 500) {
                    let id = self.fresh_id("o");
                    if self.nodes[Side::Initiator.index()].app.send(order(&id)) == Sent::Full {
                        self.refused += 1;
                    }
                }
                self.after_all(Side::Initiator, now)?;
                self.again(busy, now, self.faults.burst_every, Event::Burst);
            }
            Event::Logout => {
                let side = self.pick_side();
                // Not connected: nothing to log out of.
                let _ = self.handle(side).logout(Some("asked to by the application"));
                self.after_all(side, now)?;
                self.again(busy, now, self.faults.logout_every, Event::Logout);
            }
            Event::Skip => {
                let side = self.pick_side();
                self.skip(side, now)?;
                self.again(busy, now, self.faults.skip_every, Event::Skip);
            }
            Event::ResetBoth => {
                if self.resetting.is_none() {
                    self.resetting = Some(now);
                    for side in [Side::Initiator, Side::Acceptor] {
                        let _ = self.handle(side).logout(Some("operator reset"));
                        self.after_all(side, now)?;
                    }
                    self.queue.push(now.after(RESET_BOTH_CHECK), Event::ResetBothStep);
                }
                self.again(busy, now, self.faults.reset_both_every, Event::ResetBoth);
            }
            Event::ResetBothStep => self.reset_both_step(now)?,
            Event::Restart(side) => {
                self.down[side.index()] = false;
                if side == Side::Initiator {
                    // A new process: its backoff starts again.
                    self.reconnect_attempt = 0;
                }
                let storage: Arc<dyn SessionStorage> = self.storage[side.index()].clone();
                let registry = Arc::new(SessionRegistry::new(storage).with_clock(self.clocks.wall_clock()));
                self.nodes[side.index()].restart(registry);
                if side == Side::Initiator {
                    self.connecting = true;
                    self.queue.push(now, Event::Connect(self.generation));
                }
            }
            Event::GiveUp(conn) => {
                self.net.reset(conn);
                for side in [Side::Initiator, Side::Acceptor] {
                    if self.nodes[side.index()].fail(conn, now) {
                        self.ended(side, conn, now);
                    }
                }
            }
        }
        Ok(())
    }

    /// `side`'s process dies: its sessions go without another write, the OS resets its
    /// connections (so the other end's fail), and it restarts after a while.
    fn crash(&mut self, side: Side, now: SimTime) {
        self.down[side.index()] = true;
        // The countdowns under way go with the process's registry.
        self.cancels_at[side.index()] = None;
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

    fn pick_side(&mut self) -> Side {
        if self.rng.chance(500_000) { Side::Initiator } else { Side::Acceptor }
    }

    /// An operator's handle on `side`'s session, from its registry.
    fn handle(&self, side: Side) -> SessionHandle {
        self.nodes[side.index()].registry.handle(self.ids[side.index()].clone())
    }

    /// `now`, or if the sessions run to a schedule and it's outside it, the next period's start.
    fn active_from(&self, now: SimTime) -> SimTime {
        let Some(schedule) = &self.schedule else { return now };
        let wall = wall_start() + chrono::TimeDelta::from_std(now.since(SimTime(0))).expect("short");
        if schedule.is_active(wall) {
            return now;
        }
        let next = schedule.next_start(wall).expect("a daily schedule starts again");
        SimTime::from_duration((next - wall_start()).to_std().expect("later"))
    }

    /// An operator moves `side`'s next outgoing number a little ahead; the checker is told first.
    fn skip(&mut self, side: Side, now: SimTime) -> Result<(), Violation> {
        self.sync_ledger(side)?;
        let to = self.checker.numbers(side).0 + self.rng.between(1, 50);
        self.record(&format!("operator: {side:?} next outgoing to {to}"));
        self.checker.expect_skip(side, to);
        let handle = self.handle(side);
        self.operators.push(Operator { side, to, future: Box::pin(async move { handle.set_next_outgoing(to).await }) });
        self.poll_operators()?;
        self.after_all(side, now)
    }

    /// Resets both stores once neither side is connected; checks again shortly if not, and gives
    /// up after a while.
    fn reset_both_step(&mut self, now: SimTime) -> Result<(), Violation> {
        let Some(since) = self.resetting else { return Ok(()) };
        let idle = self.nodes.iter().all(|n| n.conns().next().is_none());
        if idle {
            for side in [Side::Initiator, Side::Acceptor] {
                let handle = self.handle(side);
                let result = poll_once(std::pin::pin!(handle.reset_sequence_numbers()));
                self.record(&format!("operator: reset {side:?}: {result:?}"));
            }
        } else if now.since(since) < RESET_BOTH_LIMIT {
            self.queue.push(now.after(RESET_BOTH_CHECK), Event::ResetBothStep);
            return Ok(());
        }
        self.resetting = None;
        // An initiator still connected (a connect under way when the reset began, which it then
        // waited for in vain) reconnects when that connection ends, as Initiator::run does.
        let initiator = Side::Initiator.index();
        if !self.connecting && !self.down[initiator] && self.nodes[initiator].conns().next().is_none() {
            self.connecting = true;
            self.queue.push(now, Event::Connect(self.generation));
        }
        Ok(())
    }

    /// Crashes a trap set off outside a driver step, which handles its own: in an operator's call
    /// on a session that isn't connected, which runs in the process making it. That's the node, if
    /// it's running (a `SessionHandle` is in-process); if it's down, the operator's offline tool
    /// (as the gateway's `seqnums`), whose call has failed already.
    fn operator_traps(&mut self, now: SimTime) -> Result<(), Violation> {
        for side in [Side::Initiator, Side::Acceptor] {
            if self.storage[side.index()].sprung() != Some(true) {
                continue;
            }
            if self.down[side.index()] {
                self.record(&format!("crash of an offline tool in {side:?}'s store"));
            } else {
                self.record(&format!("crash {side:?} in an operator's store call"));
                self.sync_ledger(side)?;
                self.crash(side, now);
            }
        }
        Ok(())
    }

    /// Polls the operator requests under way; one that has resolved is done with, and a skip that
    /// didn't happen is withdrawn from the checker.
    fn poll_operators(&mut self) -> Result<(), Violation> {
        let mut i = 0;
        while i < self.operators.len() {
            let Some(result) = poll_once(self.operators[i].future.as_mut()) else {
                i += 1;
                continue;
            };
            let Operator { side, to, .. } = self.operators.swap_remove(i);
            self.record(&format!("operator: {side:?} answered {result:?}"));
            // What the skip recorded is checked before the checker stops expecting it. A store
            // failure may have taken effect all the same, and lands with the session's next
            // commit, so that skip stays expected.
            self.sync_ledger(side)?;
            if !matches!(result, Err(SequenceError::Storage(_))) {
                self.checker.skip_done(side, to);
            }
        }
        Ok(())
    }

    /// Schedules each running side's cancel-on-disconnect countdowns for when the next ends, as
    /// the task driving its registry sleeps until then: once per deadline, again if a countdown
    /// started, stopped or fired. On the planted bug, the acceptor's run late.
    fn schedule_cancels(&mut self, now: SimTime) {
        for side in [Side::Initiator, Side::Acceptor] {
            if self.down[side.index()] {
                continue;
            }
            let next = self.nodes[side.index()].registry.next_cancel_deadline().map(|d| self.clocks.sim_time(d));
            if next == self.cancels_at[side.index()] {
                continue;
            }
            self.cancels_at[side.index()] = next;
            if let Some(at) = next {
                let late = self.options.plant == Some(Plant::LateCancel) && side == Side::Acceptor;
                let run_at = if late { at.after(Duration::from_millis(1)) } else { at };
                self.queue.push(run_at.max(now), Event::Cancels(side, at));
            }
        }
    }

    /// Schedules the next fault of a kind, while the workload runs.
    fn again(&mut self, busy: bool, now: SimTime, every: Option<Duration>, event: Event) {
        if let (true, Some(every)) = (busy, every) {
            let at = self.after_about(now, every);
            let at = self.active_from(at);
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

    /// Checks what a driver step had the session send, writes what the send buffer takes,
    /// closes the connection if the driver returned or the session closed and its output has
    /// gone, and schedules what the driver waits for next.
    fn apply(&mut self, side: Side, conn: ConnId, effects: Effects, now: SimTime) -> Result<(), Violation> {
        // A trap that crashed the process: nothing this step produced is written.
        if self.storage[side.index()].sprung() == Some(true) {
            self.record(&format!("crash {side:?} in a store call"));
            self.sync_ledger(side)?;
            self.crash(side, now);
            return Ok(());
        }
        // Behind a middlebox, it has read already.
        if effects.read > 0 && self.proxy.is_none() {
            self.net.read(conn, side, effects.read);
            self.wake_writer(side.other(), conn, now);
        }
        if !effects.output.is_empty() {
            let output = match self.options.plant {
                Some(Plant::AlterResends) => alter_resends(&effects.output),
                _ => effects.output.clone(),
            };
            self.observe(side, &output, now)?;
        }
        if effects.ended {
            // The driver returned, dropping the stream: the connection closes from here.
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

    /// Checks what both stores have recorded, both applications received, and the receipts
    /// they've been answered, since the last call.
    fn check_all(&mut self) -> Result<(), Violation> {
        for s in [Side::Initiator, Side::Acceptor] {
            self.sync_ledger(s)?;
        }
        for s in [Side::Initiator, Side::Acceptor] {
            let deliveries = self.nodes[s.index()].app.deliveries.lock().unwrap();
            self.checker.delivered(s, &deliveries)?;
        }
        let now = self.clocks.now();
        for s in [Side::Initiator, Side::Acceptor] {
            let lifecycle = self.nodes[s.index()].app.lifecycle.lock().unwrap();
            self.checker.lifecycle(s, &lifecycle, now)?;
        }
        for s in [Side::Initiator, Side::Acceptor] {
            let mut receipts = self.nodes[s.index()].app.receipts.lock().unwrap();
            let mut i = 0;
            while i < receipts.len() {
                match receipts[i].1.try_outcome() {
                    Some(outcome) => {
                        self.checker.receipt(s, &receipts[i].0, &outcome)?;
                        receipts.swap_remove(i);
                    }
                    None => i += 1,
                }
            }
        }
        Ok(())
    }

    /// Checks what `side`'s store has recorded since the last call.
    fn sync_ledger(&mut self, side: Side) -> Result<(), Violation> {
        let ledger = self.storage[side.index()].ledger.lock().unwrap();
        self.checker.stored(side, &ledger)
    }

    /// Checks and counts what `side` wrote, after what its store recorded.
    fn observe(&mut self, side: Side, bytes: &[u8], now: SimTime) -> Result<(), Violation> {
        self.sync_ledger(side)?;
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

    /// `side`'s connection `conn` ended. A writer with output still to go on its other end fails,
    /// as a write into a closed socket does (unless it's a black hole, which tells no one); the initiator
    /// reconnects after its interval.
    fn ended(&mut self, side: Side, conn: ConnId, now: SimTime) {
        self.pending.remove(&(side, conn));
        if let Some(proxy) = &mut self.proxy
            && !self.nodes[side.other().index()].conns().any(|c| c == conn)
        {
            proxy.forget(conn);
        }
        if !self.black_holed.contains(&conn) && !self.nodes[side.other().index()].unwritten(conn).is_empty() {
            self.in_flight += 1;
            self.queue.push(now.after(Duration::from_micros(100)), Event::Fail { side: side.other(), conn });
        }
        let logged_on = side == Side::Initiator && self.nodes[side.index()].had_logged_on(conn);
        if side == Side::Initiator && !self.connecting && !self.down[side.index()] {
            self.connecting = true;
            let delay = self.reconnect_delay(logged_on);
            self.queue.push(now.after(delay), Event::Connect(self.generation));
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
        self.check_all()?;
        let Some(wants) = self.nodes[side.index()].wants(conn, now) else {
            self.pending.remove(&(side, conn));
            return Ok(());
        };
        let mut push = Vec::new();
        let pending = self.pending.entry((side, conn)).or_default();
        if wants.resume && !pending.resume {
            pending.resume = true;
            push.push((now, Event::Resume(side, conn)));
        }
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
        // Like the timer, each window's wake-up is in the future when wanted.
        if wants.sends_free != pending.sends_free {
            pending.sends_free = wants.sends_free;
            if let Some(at) = wants.sends_free {
                push.push((at, Event::SendsFree(side, conn, at)));
            }
        }
        if wants.input_free != pending.input_free {
            pending.input_free = wants.input_free;
            if let Some(at) = wants.input_free {
                push.push((at, Event::InputFree(side, conn, at)));
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
            && self.pending.values().all(|p| {
                !p.read
                    && !p.commands
                    && !p.resume
                    && !p.writable
                    && !p.commit
                    && p.sends_free.is_none()
                    && p.input_free.is_none()
            })
    }

    /// Liveness: one connection, both sides logged on over it, every application message sent
    /// with a MsgSeqNum delivered, and the sequence numbers agreed.
    fn check_settled(&self) -> Result<(), Violation> {
        let fail = |detail: String| Err(Violation { rule: "liveness", detail });
        // A power loss lost what the sessions needed to agree: settling isn't promised.
        if self.checker.is_lossy() {
            return Ok(());
        }
        // Every send has been answered: stored or dropped.
        for node in &self.nodes {
            if let Some((id, _)) = node.app.receipts.lock().unwrap().first() {
                return fail(format!("{:?}'s receipt for {id} never resolved", node.side));
            }
        }
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
            let undelivered = self.checker.committed(side).filter(|id| !received.contains(id));
            for missing in undelivered {
                let seq = self.checker.seq_of(side, missing).unwrap_or(0);
                if !self.checker.rejected_late(side, seq) {
                    return fail(format!("{side:?} sent {missing} as {seq}, never delivered"));
                }
            }
            // As each store last recorded them (a disk store can't be opened while its session is).
            let (ours, theirs) = (self.checker.numbers(side).0, self.checker.numbers(side.other()).1);
            if ours != theirs {
                return fail(format!("{side:?} sends {ours} next, but the other side expects {theirs}"));
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

/// The longest cancel-on-disconnect grace period a seed draws.
const MAX_GRACE: Duration = Duration::from_secs(30);

/// Where a planted bug strikes: the 10th delivery or stored message.
pub(crate) const PLANTED_AT: u64 = 10;

/// `bytes` with each resent application message's id changed (a planted bug).
fn alter_resends(mut bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    while let Decoded::Message(mut msg, len) = decode(bytes) {
        let tag = match msg.msg_type() {
            turbojet::MsgType::NewOrderSingle => Some(tags::CL_ORD_ID),
            turbojet::MsgType::ExecutionReport => Some(tags::EXEC_ID),
            _ => None,
        };
        match tag.filter(|_| msg.get(tags::POSS_DUP_FLAG) == Some("Y")) {
            Some(tag) => {
                msg.set(tag, "altered");
                out.extend(turbojet::codec::encode(&msg).expect("a resend encodes"));
            }
            None => out.extend_from_slice(&bytes[..len]),
        }
        bytes = &bytes[len..];
    }
    out.extend_from_slice(bytes);
    out
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
