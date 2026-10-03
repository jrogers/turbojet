//! One side of the simulation: its sessions, one per connection, and the driver that feeds each
//! as `connection::run` in `turbojet/src/connection.rs` does. Keep the two in step: each method
//! here is one branch of that driver's loop.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use turbojet::codec::{DecodedInto, decode_into};
use turbojet::message::DataFields;
use turbojet::registry::CommandReceiver;
use turbojet::store::{Commit, Job, SentMessages, SessionLog};
use turbojet::{InitiatorConfig, Message, Session, SessionConfig, SessionRegistry};

use crate::Side;
use crate::app::RecordingApp;
use crate::net::ConnId;
use crate::time::{Clocks, SimTime};

/// As `MAX_COMMANDS_PER_BATCH` in the connection driver.
const MAX_COMMANDS_PER_BATCH: usize = 256;
/// As `MAX_TIMER_SLEEP` in the connection driver.
const MAX_TIMER_SLEEP: Duration = Duration::from_secs(1);
/// As `COMMANDS_PAUSE_AT` in the connection driver.
const COMMANDS_PAUSE_AT: usize = 256 * 1024;
/// As `MAX_UNWRITTEN` and `MAX_UNPROCESSED` in the connection driver.
const MAX_UNWRITTEN: usize = 16 * 1024 * 1024;
const MAX_UNPROCESSED: usize = 16 * 1024 * 1024;

pub enum Role {
    Initiator(InitiatorConfig),
    Acceptor(SessionConfig),
}

pub struct Node {
    pub side: Side,
    role: Role,
    pub registry: Arc<SessionRegistry>,
    pub app: Arc<RecordingApp>,
    clocks: Clocks,
    /// A session per connection: an initiator has at most one, an acceptor one per connection
    /// it has accepted and not yet seen end.
    running: BTreeMap<ConnId, Running>,
    data_fields: DataFields,
    /// Resend this many sequence numbers per step, if set.
    pub resend_batch: Option<u64>,
    /// Connections whose session has logged on, until the world asks after they end.
    logged_on: BTreeSet<ConnId>,
}

/// The session on one connection, and its driver's state.
struct Running {
    session: Session,
    commands: CommandReceiver,
    /// Delivered but not yet read: the socket's receive buffer.
    unread: Vec<u8>,
    /// The other end's close has arrived, behind `unread`.
    fin: bool,
    /// Read but not yet decoded: a partial frame, or input waiting for a resend to end.
    buf: Vec<u8>,
    /// `buf` holds input read during a resend.
    deferred: bool,
    scratch: Message,
    /// When the driver's timer fires next.
    timer: SimTime,
    /// A deadline `on_timer` left in the past, which waits for the ceiling rather than spin.
    stuck: Option<SimTime>,
    /// Output the send buffer hasn't taken yet.
    outbox: Vec<u8>,
    /// The store's commit, read or opening under way: the session waits for it.
    store: Option<StoreWork>,
}

/// As `StoreTask` in the connection driver: the store's work under way, for the world to finish.
enum StoreWork {
    Commit(Commit),
    Fetch(Job<SentMessages>),
    Open(Job<Box<dyn SessionLog>>),
}

/// What a driver step leaves for the world: new output from the session (to check; the node
/// keeps it until written), how many bytes it read, and whether the connection ended under it.
#[derive(Default)]
pub struct Effects {
    pub output: Vec<u8>,
    pub read: usize,
    /// The driver returned: the other end closed, or the counterparty stopped reading (a limit).
    /// The connection closes from this side unless the other end already did.
    pub ended: bool,
}

/// What a connection's driver is waiting for, for the world to schedule.
pub struct Wants {
    /// A commit is under way, for the world to finish.
    pub commit: bool,
    pub resume: bool,
    pub read: bool,
    pub commands: bool,
    pub timer: Option<SimTime>,
    /// When the outbound window frees up for sends waiting on it.
    pub sends_free: Option<SimTime>,
    /// When the inbound window frees up for input it holds.
    pub input_free: Option<SimTime>,
}

impl Node {
    pub fn new(side: Side, role: Role, registry: Arc<SessionRegistry>, app: Arc<RecordingApp>, clocks: Clocks) -> Self {
        let data_fields = match &role {
            Role::Initiator(config) => config.session.data_fields.clone(),
            Role::Acceptor(config) => config.data_fields.clone(),
        };
        let (running, logged_on) = (BTreeMap::new(), BTreeSet::new());
        Self { side, role, registry, app, clocks, running, data_fields, resend_batch: None, logged_on }
    }

    pub fn conns(&self) -> impl Iterator<Item = ConnId> + '_ {
        self.running.keys().copied()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &Session> {
        self.running.values().map(|r| &r.session)
    }

    pub fn sessions_by_conn(&self, conn: ConnId) -> Option<&Session> {
        self.running.get(&conn).map(|r| &r.session)
    }

    pub fn initiator_config(&self) -> Option<&InitiatorConfig> {
        match &self.role {
            Role::Initiator(config) => Some(config),
            Role::Acceptor(_) => None,
        }
    }

    /// A new connection: the driver starts with `on_connect`.
    pub fn start(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let (mut session, commands) = match &self.role {
            Role::Initiator(config) => Session::initiator(config, self.registry.clone(), self.app.clone(), instant),
            Role::Acceptor(config) => {
                Session::acceptor(config.clone(), self.registry.clone(), self.app.clone(), instant)
            }
        };
        if let Some(batch) = self.resend_batch {
            session.set_resend_batch(batch);
        }
        let running = Running {
            session,
            commands,
            unread: Vec::new(),
            fin: false,
            buf: Vec::new(),
            deferred: false,
            scratch: Message::default(),
            timer: now.after(MAX_TIMER_SLEEP),
            stuck: None,
            outbox: Vec::new(),
            store: None,
        };
        assert!(self.running.insert(conn, running).is_none(), "a connection starts once");
        self.step(conn, now, |session, instant| session.on_connect(instant))
    }

    /// Bytes arrive on `conn`: they wait in the receive buffer until the driver reads.
    pub fn receive(&mut self, conn: ConnId, bytes: &[u8]) {
        if let Some(running) = self.running.get_mut(&conn) {
            running.unread.extend_from_slice(bytes);
        }
    }

    /// The other end's close arrives on `conn`: the driver sees it once it has read what came
    /// before.
    pub fn fin(&mut self, conn: ConnId) {
        if let Some(running) = self.running.get_mut(&conn) {
            running.fin = true;
        }
    }

    /// `conn` failed (a reset, or TCP giving up): the driver's read or write returns an error and
    /// it ends, dropping the session without writing anything more. Unread bytes are lost.
    pub fn fail(&mut self, conn: ConnId) -> bool {
        self.running.remove(&conn).is_some()
    }

    /// The read branch, enabled unless the inbound window holds input: everything in the receive
    /// buffer. While resending it waits, read, for the resend to end; once the session has closed
    /// it's dropped; otherwise it's fed in, one message at a time into the one reused message, up
    /// to a message that starts a resend or one the inbound window holds. Then the close, if it
    /// came.
    pub fn read(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let data_fields = &self.data_fields;
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| r.reads(instant)) else {
            return Effects::default();
        };
        let read = running.unread.len();
        running.buf.append(&mut running.unread);
        if running.session.is_closed() {
            running.buf.clear();
        } else if running.session.is_resending() || running.deferred || running.store.is_some() {
            running.deferred = true;
        } else {
            running.deferred = feed(running, data_fields, instant);
        }
        let mut effects = self.settle(conn, now);
        effects.read = read;
        // A read of 0 after the data: the driver returns, dropping the session.
        if self.running.get(&conn).is_some_and(|r| r.fin && r.unread.is_empty()) {
            self.running.remove(&conn);
            effects.ended = true;
        }
        effects
    }

    /// The command branch: a logout or operator command that's due, whatever else is going on;
    /// otherwise, once logged on and not resending or backed up, sends, up to a batch or as many
    /// as the outbound window allows.
    pub fn commands(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| !r.session.is_closed() && r.store.is_none()) else {
            return Effects::default();
        };
        if let Some(command) = running.commands.try_control() {
            running.session.on_command(command, instant);
        } else if running.takes_sends() {
            for _ in 0..MAX_COMMANDS_PER_BATCH {
                if !running.session.can_send(instant) {
                    break;
                }
                let Some(command) = running.commands.try_send() else { break };
                running.session.on_command(command, instant);
            }
        }
        self.settle(conn, now)
    }

    /// The resume branch, while resending, once the last step has been written.
    pub fn resume(&mut self, conn: ConnId, now: SimTime) -> Effects {
        if !self.running.get(&conn).is_some_and(Running::resumes) {
            return Effects::default();
        }
        self.step(conn, now, Session::on_resume)
    }

    /// The timer branch, if the timer is due: at `now`, or earlier (a sleep whose deadline has
    /// passed fires at once).
    pub fn timer(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let clocks = self.clocks.clone();
        let Some(running) =
            self.running.get_mut(&conn).filter(|r| !r.session.is_closed() && r.store.is_none() && r.timer <= now)
        else {
            return Effects::default();
        };
        running.timer = now.after(MAX_TIMER_SLEEP);
        running.session.on_timer(instant);
        running.stuck = running.session.next_deadline().map(|d| clocks.sim_time(d)).filter(|d| *d <= now);
        self.settle(conn, now)
    }

    /// The store branch: the store's commit has finished, and what it covers can be written; or
    /// its read, and the resend step goes on; or its opening, and logon goes on.
    pub fn committed(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let Some(work) = self.running.get_mut(&conn).and_then(|r| r.store.take()) else {
            return Effects::default();
        };
        self.step(conn, now, |session, instant| match work {
            StoreWork::Commit(commit) => session.on_committed(commit.run(), instant),
            StoreWork::Fetch(fetch) => session.on_fetched(fetch.run(), instant),
            StoreWork::Open(open) => session.on_opened(open.run(), instant),
        })
    }

    /// The window branches: the outbound window has freed up for the sends it held, or the
    /// inbound window for the input it held. The driver goes round, feeding held input and then
    /// taking sends.
    pub fn window_free(&mut self, conn: ConnId, now: SimTime) -> Effects {
        self.settle(conn, now)
    }

    /// Output the send buffer hasn't taken yet.
    pub fn unwritten(&self, conn: ConnId) -> &[u8] {
        self.running.get(&conn).map_or(&[], |r| &r.outbox)
    }

    /// The send buffer took `n` bytes of the output.
    pub fn wrote(&mut self, conn: ConnId, n: usize) {
        if let Some(running) = self.running.get_mut(&conn) {
            running.outbox.drain(..n);
        }
    }

    /// The session has closed and its output has all gone: the driver closes the connection and
    /// returns.
    pub fn finished(&self, conn: ConnId) -> bool {
        self.running.get(&conn).is_some_and(|r| r.session.is_closed() && r.outbox.is_empty() && r.store.is_none())
    }

    /// Whether the session on `conn`, which has ended, had logged on; asked once.
    pub fn had_logged_on(&mut self, conn: ConnId) -> bool {
        self.logged_on.remove(&conn)
    }

    /// The process crashes: every session goes, without writing anything more, with the
    /// process's registry; the connections it had are returned for the OS to reset. A restart
    /// comes with a new registry.
    pub fn crash(&mut self) -> Vec<ConnId> {
        self.app.crash();
        let conns = self.running.keys().copied().collect();
        self.running.clear();
        self.logged_on.clear();
        conns
    }

    pub fn restart(&mut self, registry: Arc<SessionRegistry>) {
        assert!(self.running.is_empty(), "a crashed node has no sessions");
        self.registry = registry;
    }

    pub fn remove(&mut self, conn: ConnId) {
        self.running.remove(&conn);
    }

    fn step(&mut self, conn: ConnId, now: SimTime, f: impl FnOnce(&mut Session, Instant)) -> Effects {
        let instant = self.clocks.instant(now);
        if let Some(running) = self.running.get_mut(&conn) {
            f(&mut running.session, instant);
        }
        self.settle(conn, now)
    }

    /// The top of the driver's loop: input that waited for a resend once it has ended, then the
    /// session's output into the outbox, the limits, and the timer brought forward to its next
    /// deadline.
    fn settle(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let clocks = &self.clocks;
        let data_fields = &self.data_fields;
        let Some(running) = self.running.get_mut(&conn) else { return Effects::default() };
        if running.session.has_logged_on() {
            self.logged_on.insert(conn);
        }
        let instant = clocks.instant(now);
        loop {
            if running.deferred
                && !running.session.is_resending()
                && !running.session.is_closed()
                && !running.session.is_waiting_on_store()
                && running.input_held_until(instant).is_none()
            {
                running.deferred = feed(running, data_fields, instant);
            }
            // A Logout that was waiting for the sends queued before it, now they've been taken.
            while !running.session.is_closed()
                && !running.session.is_waiting_on_store()
                && let Some(command) = running.commands.try_control()
            {
                running.session.on_command(command, instant);
            }
            // Whatever the session did is committed before it's written; a resend step or a logon
            // the store reads or opens with a job waits for it.
            if running.store.is_none() {
                running.store = running
                    .session
                    .take_commit(instant)
                    .map(StoreWork::Commit)
                    .or_else(|| running.session.take_fetch().map(StoreWork::Fetch))
                    .or_else(|| running.session.take_open().map(StoreWork::Open));
            }
            // Input that stopped for a commit made at once goes on. Input the inbound window holds
            // waits for its wake-up.
            let stopped = running.deferred && running.store.is_none();
            if !(stopped
                && !running.session.is_resending()
                && !running.session.is_closed()
                && running.input_held_until(instant).is_none())
            {
                break;
            }
        }
        let output = running.session.output().to_vec();
        running.session.clear_output();
        running.outbox.extend_from_slice(&output);
        if running.outbox.len() > MAX_UNWRITTEN || running.buf.len() > MAX_UNPROCESSED {
            // The counterparty has stopped reading: the driver returns an error.
            self.running.remove(&conn);
            return Effects { output, ended: true, ..Effects::default() };
        }
        if let Some(deadline) = running.session.next_deadline().map(|d| clocks.sim_time(d))
            && deadline < running.timer
            && running.stuck != Some(deadline)
        {
            running.timer = deadline;
        }
        Effects { output, ..Effects::default() }
    }

    /// What `conn`'s driver would do next at `now`, for the world to schedule.
    pub fn wants(&self, conn: ConnId, now: SimTime) -> Option<Wants> {
        let running = self.running.get(&conn)?;
        let instant = self.clocks.instant(now);
        let closed = running.session.is_closed();
        let committing = running.store.is_some();
        let sends = running.takes_sends() && running.commands.has_sends();
        Some(Wants {
            commit: committing,
            resume: running.resumes(),
            read: running.reads(instant),
            commands: !closed
                && !committing
                && (running.commands.has_control() || sends && running.session.can_send(instant)),
            timer: (!closed && !committing).then_some(running.timer),
            // As `sends_this_time` in the connection driver: only while sends wait, so a steady
            // rate near the limit doesn't wake the driver for each message.
            sends_free: sends
                .then(|| running.session.send_free_at())
                .flatten()
                .filter(|at| *at > instant)
                .map(|at| self.clocks.sim_time(at)),
            input_free: running.input_held_until(instant).map(|at| self.clocks.sim_time(at)),
        })
    }
}

impl Running {
    /// The read branch is enabled: something has arrived, and the inbound window doesn't hold
    /// input, so a peer that closes meanwhile is noticed once the hold ends.
    fn reads(&self, instant: Instant) -> bool {
        (!self.unread.is_empty() || self.fin) && self.input_held_until(instant).is_none()
    }

    /// As `input_held_until` in the connection driver: when input held by the inbound window may
    /// go on, if it's held at `instant`.
    fn input_held_until(&self, instant: Instant) -> Option<Instant> {
        self.session.input_free_at().filter(|at| *at > instant)
    }

    /// The command branch takes sends.
    fn takes_sends(&self) -> bool {
        self.session.has_logged_on()
            && !self.session.is_resending()
            && !self.session.is_closed()
            && self.store.is_none()
            && self.outbox.len() < COMMANDS_PAUSE_AT
    }

    /// The resume branch is enabled.
    fn resumes(&self) -> bool {
        self.session.is_resending() && self.outbox.is_empty() && !self.session.is_closed() && self.store.is_none()
    }
}

/// As `feed` in the connection driver: the complete messages in `buf` fed in, up to one that
/// starts a resend, or one that waits for a commit or the inbound window. Returns true if input is
/// left waiting.
fn feed(running: &mut Running, data_fields: &DataFields, instant: Instant) -> bool {
    let mut consumed = 0;
    let deferred = loop {
        if running.session.is_resending() || running.session.is_closed() {
            break running.session.is_resending();
        }
        // The message that fills the inbound window has been handled; the next one waits.
        if consumed < running.buf.len()
            && (!running.session.ready_for_input() || running.session.input_free_at().is_some_and(|at| at > instant))
        {
            break true;
        }
        match decode_into(&running.buf[consumed..], data_fields, &mut running.scratch) {
            DecodedInto::Message(len) => {
                consumed += len;
                running.session.on_message(&running.scratch, instant);
            }
            DecodedInto::Incomplete => break false,
            DecodedInto::Garbled { skip, .. } => consumed += skip,
        }
    };
    running.buf.drain(..consumed);
    deferred
}
