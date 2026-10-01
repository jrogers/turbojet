//! One simulation: two nodes, the network between them and a workload, run from a seed.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use turbojet::store::SessionStorage;
use turbojet::{InitiatorConfig, MemoryStorage, SessionConfig, SessionId, SessionRegistry};

use crate::Side;
use crate::app::{RecordingApp, order, report};
use crate::check::{Checker, Violation};
use crate::net::{ConnId, Net};
use crate::node::{Effects, Node, Role};
use crate::queue::Queue;
use crate::rng::Rng;
use crate::time::{Clocks, SimTime};

/// How a simulation runs.
#[derive(Debug, Clone)]
pub struct Options {
    pub seed: u64,
    /// How long the workload runs.
    pub busy: Duration,
    /// Then how long the sessions have to settle, with no new work.
    pub quiet: Duration,
    /// Keep a trace of every event.
    pub verbose: bool,
}

impl Options {
    /// The run each seed gets on every push.
    pub fn per_push(seed: u64) -> Self {
        Self { seed, busy: Duration::from_secs(20), quiet: Duration::from_secs(20), verbose: false }
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

#[derive(Debug)]
enum Event {
    /// The initiator connects to the acceptor.
    Connect,
    Arrive {
        to: Side,
        conn: ConnId,
        bytes: Vec<u8>,
    },
    Eof {
        to: Side,
        conn: ConnId,
    },
    Read(Side),
    Commands(Side),
    Resume(Side),
    Timer(Side, SimTime),
    SendOrder,
    SendReport,
}

/// What's scheduled for one node, so each kind of wake-up is queued once.
#[derive(Default)]
struct Pending {
    read: bool,
    commands: bool,
    resume: bool,
    timer: Option<SimTime>,
}

/// Longest the sessions may take to settle once the quiet phase is over.
const SETTLE_LIMIT: Duration = Duration::from_secs(120);

struct World {
    options: Options,
    clocks: Clocks,
    rng: Rng,
    queue: Queue<Event>,
    nodes: [Node; 2],
    storage: [Arc<MemoryStorage>; 2],
    ids: [SessionId; 2],
    net: Net,
    checker: Checker,
    pending: [Pending; 2],
    /// Arrivals and closes on their way.
    in_flight: usize,
    next_id: u64,
    digest: u64,
    events: u64,
    trace: Vec<String>,
}

pub fn run(options: &Options) -> Result<Report, Failure> {
    let mut world = World::new(options.clone());
    match world.run() {
        Ok(()) => Ok(Report {
            digest: world.digest,
            events: world.events,
            committed: [Side::Initiator, Side::Acceptor].map(|s| world.checker.committed(s).count()),
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

/// An event for the trace: messages as their type and MsgSeqNum (`D#4`, PossDup marked `D#4*`).
fn describe(event: &Event) -> String {
    match event {
        Event::Arrive { to, conn, bytes } => format!("arrive {to:?} conn {conn}: {}", summary(bytes)),
        other => format!("{other:?}"),
    }
}

/// The messages in `bytes`, briefly.
pub fn summary(mut bytes: &[u8]) -> String {
    let mut out = Vec::new();
    while let turbojet::codec::Decoded::Message(msg, len) = turbojet::codec::decode(bytes) {
        let seq = msg.get(turbojet::message::tags::MSG_SEQ_NUM).unwrap_or("?");
        let dup = if msg.get(turbojet::message::tags::POSS_DUP_FLAG) == Some("Y") { "*" } else { "" };
        out.push(format!("{}#{seq}{dup}", msg.msg_type()));
        bytes = &bytes[len..];
    }
    if !bytes.is_empty() {
        out.push(format!("<{} undecodable bytes>", bytes.len()));
    }
    out.join(" ")
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
        let storage = [Arc::new(MemoryStorage::new()), Arc::new(MemoryStorage::new())];
        let registry = |side: Side| {
            let storage: Arc<dyn SessionStorage> = storage[side.index()].clone();
            Arc::new(SessionRegistry::new(storage).with_clock(clocks.wall_clock()))
        };
        let nodes = [
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
        let mut world = Self {
            options,
            clocks,
            rng,
            queue: Queue::new(),
            nodes,
            storage,
            ids: [session_id("CLIENT", "GATEWAY"), session_id("GATEWAY", "CLIENT")],
            net: Net::new(),
            checker: Checker::new(),
            pending: Default::default(),
            in_flight: 0,
            next_id: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            events: 0,
            trace: Vec::new(),
        };
        world.queue.push(SimTime(0), Event::Connect);
        world.queue.push(SimTime(0), Event::SendOrder);
        world.queue.push(SimTime(0), Event::SendReport);
        world
    }

    fn run(&mut self) -> Result<(), Violation> {
        let busy_end = SimTime::from_duration(self.options.busy);
        let end = busy_end.after(self.options.quiet);
        while let Some(at) = self.queue.peek_time() {
            if at > end && self.idle() {
                break;
            }
            if at > end.after(SETTLE_LIMIT) {
                return Err(Violation { rule: "liveness", detail: "still busy long after the quiet phase".into() });
            }
            let (at, event) = self.queue.pop().expect("peeked");
            self.clocks.advance_to(at);
            self.events += 1;
            self.record(&describe(&event));
            self.dispatch(event, at, busy_end)?;
        }
        self.check_settled()
    }

    fn dispatch(&mut self, event: Event, now: SimTime, busy_end: SimTime) -> Result<(), Violation> {
        match event {
            Event::Connect => {
                assert!(
                    self.nodes[Side::Acceptor.index()].conn().is_none(),
                    "the acceptor saw the last connection end"
                );
                let conn = self.net.connect();
                let effects = self.nodes[Side::Acceptor.index()].start(conn, now);
                self.apply(Side::Acceptor, effects, now)?;
                let effects = self.nodes[Side::Initiator.index()].start(conn, now);
                self.apply(Side::Initiator, effects, now)?;
            }
            Event::Arrive { to, conn, bytes } => {
                self.in_flight -= 1;
                self.nodes[to.index()].receive(conn, &bytes);
                self.after(to, now)?;
            }
            Event::Eof { to, conn } => {
                self.in_flight -= 1;
                if self.nodes[to.index()].eof(conn) {
                    self.ended(to, now);
                }
                self.after(to, now)?;
            }
            Event::Read(side) => {
                self.pending[side.index()].read = false;
                let effects = self.nodes[side.index()].read(now);
                self.apply(side, effects, now)?;
            }
            Event::Commands(side) => {
                self.pending[side.index()].commands = false;
                let effects = self.nodes[side.index()].commands(now);
                self.apply(side, effects, now)?;
            }
            Event::Resume(side) => {
                self.pending[side.index()].resume = false;
                let effects = self.nodes[side.index()].resume(now);
                self.apply(side, effects, now)?;
            }
            Event::Timer(side, at) => {
                if self.pending[side.index()].timer == Some(at) {
                    self.pending[side.index()].timer = None;
                    let effects = self.nodes[side.index()].timer(now);
                    self.apply(side, effects, now)?;
                }
            }
            Event::SendOrder => {
                let id = self.fresh_id("o");
                self.nodes[Side::Initiator.index()].app.send(order(&id));
                self.after(Side::Initiator, now)?;
                if now < busy_end {
                    let next = now.after(Duration::from_millis(self.rng.between(1, 200)));
                    self.queue.push(next, Event::SendOrder);
                }
            }
            Event::SendReport => {
                let id = self.fresh_id("u");
                self.nodes[Side::Acceptor.index()].app.send(report(&id, None));
                self.after(Side::Acceptor, now)?;
                if now < busy_end {
                    let next = now.after(Duration::from_millis(self.rng.between(50, 1_000)));
                    self.queue.push(next, Event::SendReport);
                }
            }
        }
        Ok(())
    }

    fn fresh_id(&mut self, prefix: &str) -> String {
        self.next_id += 1;
        format!("{prefix}{}", self.next_id)
    }

    /// Sends what a driver step wrote, closes the connection if the session did, and schedules
    /// what the node waits for next.
    fn apply(&mut self, side: Side, effects: Effects, now: SimTime) -> Result<(), Violation> {
        if !effects.written.is_empty() {
            self.checker.written(side, &effects.written, now)?;
            self.hash(&effects.written);
            if let Some(at) = self.net.write(effects.conn, side, now) {
                self.in_flight += 1;
                self.queue.push(at, Event::Arrive { to: side.other(), conn: effects.conn, bytes: effects.written });
            }
        }
        if effects.closed {
            if let Some(at) = self.net.close(effects.conn, side, now) {
                self.in_flight += 1;
                self.queue.push(at, Event::Eof { to: side.other(), conn: effects.conn });
            }
            self.ended(side, now);
        }
        self.after(side, now)
    }

    /// `side`'s connection ended: the initiator reconnects after its interval.
    fn ended(&mut self, side: Side, now: SimTime) {
        self.pending[side.index()] = Pending::default();
        if let Some(interval) = self.nodes[side.index()].reconnect_interval() {
            self.queue.push(now.after(interval), Event::Connect);
        }
    }

    /// Checks deliveries, and schedules what `side`'s driver would do next.
    fn after(&mut self, side: Side, now: SimTime) -> Result<(), Violation> {
        for s in [Side::Initiator, Side::Acceptor] {
            let deliveries = self.nodes[s.index()].app.deliveries.lock().unwrap().clone();
            self.checker.delivered(s, &deliveries)?;
        }
        let Some(wants) = self.nodes[side.index()].wants() else {
            self.pending[side.index()] = Pending::default();
            return Ok(());
        };
        let pending = &mut self.pending[side.index()];
        if wants.resume && !pending.resume {
            pending.resume = true;
            self.queue.push(now, Event::Resume(side));
        }
        if wants.read && !pending.read {
            pending.read = true;
            self.queue.push(now, Event::Read(side));
        }
        if wants.commands && !pending.commands {
            pending.commands = true;
            self.queue.push(now, Event::Commands(side));
        }
        if wants.timer != pending.timer {
            pending.timer = wants.timer;
            if let Some(at) = wants.timer {
                self.queue.push(at.max(now), Event::Timer(side, at));
            }
        }
        Ok(())
    }

    /// Nothing on the network, and nothing for either driver to do but wait for its timer.
    fn idle(&self) -> bool {
        self.in_flight == 0 && self.pending.iter().all(|p| !p.read && !p.commands && !p.resume)
    }

    /// Liveness, once the quiet phase is over and the network idle: both sides logged on, every
    /// application message sent with a MsgSeqNum delivered, and the sequence numbers agreed.
    fn check_settled(&self) -> Result<(), Violation> {
        let fail = |detail: String| Err(Violation { rule: "liveness", detail });
        for node in &self.nodes {
            match node.session() {
                Some(s) if s.is_logged_on() && !s.is_resending() => {}
                Some(_) => return fail(format!("{:?} isn't logged on, or is still resending", node.side)),
                None => return fail(format!("{:?} isn't connected", node.side)),
            }
        }
        for side in [Side::Initiator, Side::Acceptor] {
            let received: std::collections::BTreeSet<&str> = self.checker.received_ids(side.other()).collect();
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
