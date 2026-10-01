//! One side of the simulation: its sessions, one per connection, and the driver that feeds each
//! as `connection::run` in `turbojet/src/connection.rs` does. Keep the two in step: each method
//! here is one branch of that driver's loop.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use turbojet::codec::{DecodedInto, decode_into};
use turbojet::message::DataFields;
use turbojet::registry::CommandReceiver;
use turbojet::{InitiatorConfig, Message, Session, SessionConfig, SessionRegistry};

use crate::Side;
use crate::app::RecordingApp;
use crate::net::ConnId;
use crate::time::{Clocks, SimTime};

/// As `MAX_COMMANDS_PER_BATCH` in the connection driver.
const MAX_COMMANDS_PER_BATCH: usize = 256;
/// As `MAX_TIMER_SLEEP` in the connection driver.
const MAX_TIMER_SLEEP: Duration = Duration::from_secs(1);

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
}

/// The session on one connection, and its driver's state.
struct Running {
    session: Session,
    commands: CommandReceiver,
    /// Delivered but not yet read: the socket's receive buffer.
    unread: Vec<u8>,
    /// The other end's close has arrived, behind `unread`.
    fin: bool,
    /// Read but not yet decoded: a partial frame, as the driver's read buffer keeps.
    buf: Vec<u8>,
    scratch: Message,
    /// When the driver's timer fires next.
    timer: SimTime,
    /// A deadline `on_timer` left in the past, which waits for the ceiling rather than spin.
    stuck: Option<SimTime>,
    /// Output the send buffer couldn't take yet: the driver is blocked in `write_all`, doing
    /// nothing else until it has gone.
    blocked: Vec<u8>,
    /// The session closed: once its output has gone, the connection closes.
    closing: bool,
}

/// What a driver step leaves for the world: bytes to write to the connection, how many it read,
/// and whether the connection closes once they've gone.
#[derive(Default)]
pub struct Effects {
    pub written: Vec<u8>,
    pub read: usize,
    pub close: bool,
    /// The connection ended from this side's view (the other end closed it): no close to send.
    pub ended: bool,
    /// `written` is what's left of an earlier write, not new output from the session.
    pub resumed: bool,
}

/// What a connection's driver is waiting for, for the world to schedule.
pub struct Wants {
    pub resume: bool,
    pub read: bool,
    pub commands: bool,
    pub timer: Option<SimTime>,
}

impl Node {
    pub fn new(side: Side, role: Role, registry: Arc<SessionRegistry>, app: Arc<RecordingApp>, clocks: Clocks) -> Self {
        let data_fields = match &role {
            Role::Initiator(config) => config.session.data_fields.clone(),
            Role::Acceptor(config) => config.data_fields.clone(),
        };
        Self { side, role, registry, app, clocks, running: BTreeMap::new(), data_fields, resend_batch: None }
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

    pub fn has(&self, conn: ConnId) -> bool {
        self.running.contains_key(&conn)
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
            scratch: Message::default(),
            timer: now.after(MAX_TIMER_SLEEP),
            stuck: None,
            blocked: Vec::new(),
            closing: false,
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

    /// The read branch: everything in the receive buffer, decoded and fed in, one message at a
    /// time into the one reused message, then the close if it came. Not while resending.
    pub fn read(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let data_fields = &self.data_fields;
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| r.ready() && (!r.unread.is_empty() || r.fin)) else {
            return Effects::default();
        };
        let read = running.unread.len();
        running.buf.append(&mut running.unread);
        let mut consumed = 0;
        loop {
            match decode_into(&running.buf[consumed..], data_fields, &mut running.scratch) {
                DecodedInto::Message(len) => {
                    consumed += len;
                    running.session.on_message(&running.scratch, instant);
                }
                DecodedInto::Incomplete => break,
                DecodedInto::Garbled { skip, .. } => consumed += skip,
            }
        }
        running.buf.drain(..consumed);
        let mut effects = self.settle(conn);
        effects.read = read;
        // A read of 0 after the data: the driver returns, dropping the session. What it just
        // wrote goes first, as the loop writes before reading again.
        if let Some(running) = self.running.get(&conn)
            && running.fin
            && running.unread.is_empty()
            && running.blocked.is_empty()
            && !running.closing
        {
            self.running.remove(&conn);
            effects.ended = true;
        }
        effects
    }

    /// The command branch: what's queued, up to a batch. Not while resending.
    pub fn commands(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.get_mut(&conn).filter(|r| r.ready()) else {
            return Effects::default();
        };
        for _ in 0..MAX_COMMANDS_PER_BATCH {
            let Ok(command) = running.commands.try_recv() else { break };
            running.session.on_command(command, instant);
        }
        self.settle(conn)
    }

    /// The resume branch, while resending.
    pub fn resume(&mut self, conn: ConnId, now: SimTime) -> Effects {
        if !self.running.get(&conn).is_some_and(|r| r.blocked.is_empty() && r.session.is_resending()) {
            return Effects::default();
        }
        self.step(conn, now, Session::on_resume)
    }

    /// The timer branch, if the timer is due: at `now`, or earlier while the driver was blocked
    /// (a sleep whose deadline has passed fires at once).
    pub fn timer(&mut self, conn: ConnId, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let clocks = self.clocks.clone();
        let Some(running) = self.running.get_mut(&conn).filter(|r| r.blocked.is_empty() && r.timer <= now) else {
            return Effects::default();
        };
        running.timer = now.after(MAX_TIMER_SLEEP);
        running.session.on_timer(instant);
        running.stuck = running.session.next_deadline().map(|d| clocks.sim_time(d)).filter(|d| *d <= now);
        self.settle(conn)
    }

    /// The send buffer took only `accepted` of the bytes last written: the driver blocks on the
    /// rest.
    pub fn block(&mut self, conn: ConnId, rest: &[u8]) {
        if let Some(running) = self.running.get_mut(&conn) {
            debug_assert!(running.blocked.is_empty(), "a blocked driver writes nothing more");
            running.blocked.extend_from_slice(rest);
        }
    }

    pub fn is_blocked(&self, conn: ConnId) -> bool {
        self.running.get(&conn).is_some_and(|r| !r.blocked.is_empty())
    }

    /// Room in the send buffer: the blocked write carries on.
    pub fn unblock(&mut self, conn: ConnId) -> Effects {
        let Some(running) = self.running.get_mut(&conn).filter(|r| !r.blocked.is_empty()) else {
            return Effects::default();
        };
        Effects {
            written: std::mem::take(&mut running.blocked),
            close: running.closing,
            resumed: true,
            ..Effects::default()
        }
    }

    /// The connection closed from this end once its output went: the driver has returned.
    pub fn closed(&mut self, conn: ConnId) {
        self.running.remove(&conn);
    }

    fn step(&mut self, conn: ConnId, now: SimTime, f: impl FnOnce(&mut Session, std::time::Instant)) -> Effects {
        let instant = self.clocks.instant(now);
        if let Some(running) = self.running.get_mut(&conn) {
            f(&mut running.session, instant);
        }
        self.settle(conn)
    }

    /// The top of the driver's loop: write what the session sent, close if it closed, and bring
    /// the timer forward to its next deadline.
    fn settle(&mut self, conn: ConnId) -> Effects {
        let clocks = &self.clocks;
        let Some(running) = self.running.get_mut(&conn) else { return Effects::default() };
        let written = running.session.output().to_vec();
        running.session.clear_output();
        if running.session.is_closed() {
            running.closing = true;
            return Effects { written, close: true, ..Effects::default() };
        }
        if let Some(deadline) = running.session.next_deadline().map(|d| clocks.sim_time(d))
            && deadline < running.timer
            && running.stuck != Some(deadline)
        {
            running.timer = deadline;
        }
        Effects { written, ..Effects::default() }
    }

    /// What `conn`'s driver would do next, for the world to schedule.
    pub fn wants(&self, conn: ConnId) -> Option<Wants> {
        let running = self.running.get(&conn)?;
        if !running.blocked.is_empty() || running.closing {
            return Some(Wants { resume: false, read: false, commands: false, timer: None });
        }
        let resending = running.session.is_resending();
        Some(Wants {
            resume: resending,
            read: !resending && (!running.unread.is_empty() || running.fin),
            commands: !resending && !running.commands.is_empty(),
            timer: Some(running.timer),
        })
    }
}

impl Running {
    /// The driver's read and command branches are enabled: it isn't blocked writing, closing or
    /// resending.
    fn ready(&self) -> bool {
        self.blocked.is_empty() && !self.closing && !self.session.is_resending()
    }
}
