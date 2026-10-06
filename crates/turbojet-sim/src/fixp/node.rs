//! One side of the FIXP simulation: its sessions, one per connection, and the driver that feeds
//! each as `connection::run_tracked` in `turbojet/src/connection.rs` does for a `FixpSession`.
//! It's the FIX node (`crate::node`) without what a FIXP session never asks for: resend steps,
//! the inbound and outbound windows, and decoding (a FIXP session frames its own input).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use turbojet::fixp::{FixpConfig, FixpRegistry, FixpSession, SbeMessage};
use turbojet::registry::CommandReceiver;
use turbojet::store::{Commit, Job, SentMessages, SessionLog};

use super::app::RecordingApp;
use crate::Side;
use crate::net::ConnId;
use crate::time::{Clocks, SimTime};

/// As `MAX_COMMANDS_PER_BATCH`, `MAX_TIMER_SLEEP`, `COMMANDS_PAUSE_AT`, `MAX_UNWRITTEN` and
/// `MAX_UNPROCESSED` in the connection driver.
const MAX_COMMANDS_PER_BATCH: usize = 256;
const MAX_TIMER_SLEEP: Duration = Duration::from_secs(1);
const COMMANDS_PAUSE_AT: usize = 256 * 1024;
const MAX_UNWRITTEN: usize = 16 * 1024 * 1024;
const MAX_UNPROCESSED: usize = 16 * 1024 * 1024;

pub struct Node {
    pub side: Side,
    pub config: FixpConfig,
    pub registry: Arc<FixpRegistry>,
    pub app: Arc<RecordingApp>,
    clocks: Clocks,
    /// A session per connection: the client has at most one, the server one per connection it
    /// has accepted and not yet seen end.
    running: BTreeMap<ConnId, Running>,
    /// Connections whose session has been established, until the world asks after they end.
    established: BTreeSet<ConnId>,
}

/// The session on one connection, and its driver's state.
struct Running {
    session: FixpSession,
    commands: CommandReceiver<SbeMessage>,
    /// Delivered but not yet read: the socket's receive buffer.
    unread: Vec<u8>,
    /// The other end's close has arrived, behind `unread`.
    fin: bool,
    /// Read but not yet fed: a partial frame, or input waiting for the store.
    buf: Vec<u8>,
    /// `buf` holds input that waits for the store.
    deferred: bool,
    /// When the driver's timer fires next.
    timer: SimTime,
    /// A deadline `on_timer` left in the past, which waits for the ceiling rather than spin.
    stuck: Option<SimTime>,
    /// Output the send buffer hasn't taken yet.
    outbox: Vec<u8>,
    /// The store's commit, read or opening under way: the session waits for it.
    store: Option<StoreWork>,
}

/// As `StoreTask` in the connection driver.
enum StoreWork {
    Commit(Commit),
    Fetch(Job<SentMessages>),
    Open(Job<Box<dyn SessionLog>>),
}

/// What a driver step leaves for the world, as the FIX node's `Effects`.
#[derive(Default)]
pub struct Effects {
    pub output: Vec<u8>,
    pub read: usize,
    pub ended: bool,
}

/// What a connection's driver is waiting for, for the world to schedule.
pub struct Wants {
    pub commit: bool,
    pub read: bool,
    pub commands: bool,
    pub timer: Option<SimTime>,
}

impl Node {
    pub fn new(
        side: Side,
        config: FixpConfig,
        registry: Arc<FixpRegistry>,
        app: Arc<RecordingApp>,
        clocks: Clocks,
    ) -> Self {
        Self { side, config, registry, app, clocks, running: BTreeMap::new(), established: BTreeSet::new() }
    }

    pub fn conns(&self) -> impl Iterator<Item = ConnId> + '_ {
        self.running.keys().copied()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &FixpSession> {
        self.running.values().map(|r| &r.session)
    }

    /// A new connection: the driver starts with `on_connect`.
    pub fn start(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let (session, commands) =
            FixpSession::new(self.config.clone(), self.registry.clone(), self.app.clone(), instant);
        let running = Running {
            session,
            commands,
            unread: Vec::new(),
            fin: false,
            buf: Vec::new(),
            deferred: false,
            timer: now.after(MAX_TIMER_SLEEP),
            stuck: None,
            outbox: Vec::new(),
            store: None,
        };
        assert!(self.running.insert(conn, running).is_none(), "a connection starts once");
        self.step(conn, now, FixpSession::on_connect)
    }

    /// Bytes arrive on `conn`: they wait in the receive buffer until the driver reads.
    pub fn receive(&mut self, conn: ConnId, bytes: &[u8]) {
        if let Some(running) = self.running.get_mut(&conn) {
            running.unread.extend_from_slice(bytes);
        }
    }

    /// The other end's close arrives on `conn`, behind what came before.
    pub fn fin(&mut self, conn: ConnId) {
        if let Some(running) = self.running.get_mut(&conn) {
            running.fin = true;
        }
    }

    /// `conn` failed (a reset, or TCP giving up): the driver ends, telling the session the
    /// connection is gone. Whether it was running.
    pub fn fail(&mut self, conn: ConnId, now: SimTime) -> bool {
        self.end(conn, now)
    }

    /// The read branch: everything in the receive buffer, dropped once the session has closed,
    /// left waiting while it waits for its store, otherwise fed in. Then the close, if it came.
    pub fn read(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| r.reads()) else {
            return Effects::default();
        };
        let read = running.unread.len();
        running.buf.append(&mut running.unread);
        if running.session.is_closed() {
            running.buf.clear();
        } else if running.session.is_waiting_on_store() || running.deferred || running.store.is_some() {
            running.deferred = true;
        } else {
            running.deferred = running.session.feed(&mut running.buf, instant);
        }
        let mut effects = self.settle(conn, now);
        effects.read = read;
        // A read of 0 after the data: the driver returns, dropping the session.
        if self.running.get(&conn).is_some_and(|r| r.fin && r.unread.is_empty()) {
            self.end(conn, now);
            effects.ended = true;
        }
        effects
    }

    /// The command branch: a control command first; otherwise, once established and not backed
    /// up, sends, up to a batch.
    pub fn commands(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| !r.session.is_closed() && r.store.is_none()) else {
            return Effects::default();
        };
        if let Some(command) = running.commands.try_control() {
            running.session.on_command(command, instant);
        } else if running.takes_sends() {
            for _ in 0..MAX_COMMANDS_PER_BATCH {
                let Some(command) = running.commands.try_send() else { break };
                running.session.on_command(command, instant);
            }
        }
        self.settle(conn, now)
    }

    /// The timer branch, if the timer is due.
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

    /// The store branch: its commit, read or opening has finished.
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

    /// The session has closed and its output has all gone: the driver closes the connection.
    pub fn finished(&self, conn: ConnId) -> bool {
        self.running.get(&conn).is_some_and(|r| r.session.is_closed() && r.outbox.is_empty() && r.store.is_none())
    }

    /// Whether the session on `conn`, which has ended, had been established; asked once.
    pub fn had_established(&mut self, conn: ConnId) -> bool {
        self.established.remove(&conn)
    }

    /// The session closed and its output has gone: the driver returns.
    pub fn remove(&mut self, conn: ConnId) {
        let removed = self.running.remove(&conn);
        assert!(removed.is_none_or(|r| r.session.is_closed()), "only a closed session is removed");
    }

    /// The driver returns as the transport ended under it, telling the session first.
    fn end(&mut self, conn: ConnId, now: SimTime) -> bool {
        let Some(mut running) = self.running.remove(&conn) else { return false };
        running.session.on_disconnect(self.clocks.instant(now));
        true
    }

    fn step(&mut self, conn: ConnId, now: SimTime, f: impl FnOnce(&mut FixpSession, Instant)) -> Effects {
        let instant = self.clocks.instant(now);
        if let Some(running) = self.running.get_mut(&conn) {
            f(&mut running.session, instant);
        }
        self.settle(conn, now)
    }

    /// The top of the driver's loop, as `process` and `stage_output`: input that waited for the
    /// store, control commands, the store's next job, then the session's output into the outbox,
    /// the limits, and the timer brought forward to its next deadline.
    fn settle(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let clocks = &self.clocks;
        let Some(running) = self.running.get_mut(&conn) else { return Effects::default() };
        if running.session.is_established() {
            self.established.insert(conn);
        }
        let instant = clocks.instant(now);
        loop {
            if running.deferred && running.can_feed() {
                running.deferred = running.session.feed(&mut running.buf, instant);
            }
            while !running.session.is_closed()
                && !running.session.is_waiting_on_store()
                && let Some(command) = running.commands.try_control()
            {
                running.session.on_command(command, instant);
            }
            if running.store.is_none() {
                running.store = running
                    .session
                    .take_commit()
                    .map(StoreWork::Commit)
                    .or_else(|| running.session.take_fetch().map(StoreWork::Fetch))
                    .or_else(|| running.session.take_open().map(StoreWork::Open));
            }
            if !(running.deferred && running.store.is_none() && running.can_feed()) {
                break;
            }
        }
        let output = running.session.output().to_vec();
        running.session.clear_output();
        running.outbox.extend_from_slice(&output);
        if running.outbox.len() > MAX_UNWRITTEN || running.buf.len() > MAX_UNPROCESSED {
            self.end(conn, now);
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

    /// What `conn`'s driver would do next, for the world to schedule.
    pub fn wants(&self, conn: ConnId) -> Option<Wants> {
        let running = self.running.get(&conn)?;
        let closed = running.session.is_closed();
        let committing = running.store.is_some();
        let sends = running.takes_sends() && running.commands.has_sends();
        Some(Wants {
            commit: committing,
            read: running.reads(),
            commands: !closed && !committing && (running.commands.has_control() || sends),
            timer: (!closed && !committing).then_some(running.timer),
        })
    }
}

impl Running {
    /// The read branch is enabled: something has arrived.
    fn reads(&self) -> bool {
        !self.unread.is_empty() || self.fin
    }

    /// As `can_feed` in the connection driver.
    fn can_feed(&self) -> bool {
        !self.session.is_closed() && !self.session.is_waiting_on_store()
    }

    /// The command branch takes sends.
    fn takes_sends(&self) -> bool {
        self.session.is_established()
            && !self.session.is_closed()
            && self.store.is_none()
            && self.outbox.len() < COMMANDS_PAUSE_AT
    }
}
