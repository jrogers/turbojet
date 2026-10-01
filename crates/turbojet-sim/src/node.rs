//! One side of the simulation: a session, and the driver that feeds it as `connection::run` in
//! `turbojet/src/connection.rs` does. Keep the two in step: each method here is one branch of
//! that driver's loop.

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
    running: Option<Running>,
    data_fields: DataFields,
}

/// The session on the current connection, and its driver's state.
struct Running {
    session: Session,
    commands: CommandReceiver,
    conn: ConnId,
    /// Delivered but not yet read: the socket's receive buffer.
    unread: Vec<u8>,
    /// Read but not yet decoded: a partial frame, as the driver's read buffer keeps.
    buf: Vec<u8>,
    scratch: Message,
    /// When the driver's timer fires next.
    timer: SimTime,
    /// A deadline `on_timer` left in the past, which waits for the ceiling rather than spin.
    stuck: Option<SimTime>,
}

/// What a driver step leaves for the world: bytes written to the connection, and whether the
/// session closed it afterwards.
#[derive(Default)]
pub struct Effects {
    pub conn: ConnId,
    pub written: Vec<u8>,
    pub closed: bool,
}

/// What the node is waiting for after a step, for the world to schedule.
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
        Self { side, role, registry, app, clocks, running: None, data_fields }
    }

    pub fn session(&self) -> Option<&Session> {
        self.running.as_ref().map(|r| &r.session)
    }

    pub fn conn(&self) -> Option<ConnId> {
        self.running.as_ref().map(|r| r.conn)
    }

    pub fn reconnect_interval(&self) -> Option<Duration> {
        match &self.role {
            Role::Initiator(config) => Some(config.reconnect_interval),
            Role::Acceptor(_) => None,
        }
    }

    /// A new connection: the driver starts with `on_connect`.
    pub fn start(&mut self, conn: ConnId, now: SimTime) -> Effects {
        assert!(self.running.is_none(), "{:?} already has a connection", self.side);
        let instant = self.clocks.instant(now);
        let (session, commands) = match &self.role {
            Role::Initiator(config) => Session::initiator(config, self.registry.clone(), self.app.clone(), instant),
            Role::Acceptor(config) => {
                Session::acceptor(config.clone(), self.registry.clone(), self.app.clone(), instant)
            }
        };
        self.running = Some(Running {
            session,
            commands,
            conn,
            unread: Vec::new(),
            buf: Vec::new(),
            scratch: Message::default(),
            timer: now.after(MAX_TIMER_SLEEP),
            stuck: None,
        });
        self.step(now, |session, instant| session.on_connect(instant))
    }

    /// Bytes arrive on `conn`: they wait in the receive buffer until the driver reads.
    pub fn receive(&mut self, conn: ConnId, bytes: &[u8]) {
        if let Some(running) = self.running.as_mut().filter(|r| r.conn == conn) {
            running.unread.extend_from_slice(bytes);
        }
    }

    /// The other end closed `conn`: the driver's read returns 0 and it ends, dropping the session
    /// without writing anything more. Returns whether this node's connection ended.
    pub fn eof(&mut self, conn: ConnId) -> bool {
        if self.conn() == Some(conn) {
            // Unread bytes are lost with the connection.
            self.running = None;
            true
        } else {
            false
        }
    }

    /// The read branch: everything in the receive buffer, decoded and fed in, one message at a
    /// time into the one reused message. Not while resending.
    pub fn read(&mut self, now: SimTime) -> Effects {
        let data_fields = &self.data_fields;
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.as_mut().filter(|r| !r.session.is_resending() && !r.unread.is_empty()) else {
            return Effects::default();
        };
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
        self.settle(now)
    }

    /// The command branch: what's queued, up to a batch. Not while resending.
    pub fn commands(&mut self, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let Some(running) = self.running.as_mut().filter(|r| !r.session.is_resending()) else {
            return Effects::default();
        };
        for _ in 0..MAX_COMMANDS_PER_BATCH {
            let Ok(command) = running.commands.try_recv() else { break };
            running.session.on_command(command, instant);
        }
        self.settle(now)
    }

    /// The resume branch, while resending.
    pub fn resume(&mut self, now: SimTime) -> Effects {
        if !self.session().is_some_and(Session::is_resending) {
            return Effects::default();
        }
        self.step(now, Session::on_resume)
    }

    /// The timer branch, if the timer is due at `now`.
    pub fn timer(&mut self, now: SimTime) -> Effects {
        let instant = self.clocks.instant(now);
        let clocks = self.clocks.clone();
        let Some(running) = self.running.as_mut().filter(|r| r.timer == now) else {
            return Effects::default();
        };
        running.timer = now.after(MAX_TIMER_SLEEP);
        running.session.on_timer(instant);
        running.stuck = running.session.next_deadline().map(|d| clocks.sim_time(d)).filter(|d| *d <= now);
        self.settle(now)
    }

    fn step(&mut self, now: SimTime, f: impl FnOnce(&mut Session, std::time::Instant)) -> Effects {
        let instant = self.clocks.instant(now);
        if let Some(running) = self.running.as_mut() {
            f(&mut running.session, instant);
        }
        self.settle(now)
    }

    /// The top of the driver's loop: write what the session sent, end if it closed, and bring the
    /// timer forward to its next deadline.
    fn settle(&mut self, _now: SimTime) -> Effects {
        let Some(running) = self.running.as_mut() else { return Effects::default() };
        let mut effects = Effects { conn: running.conn, written: running.session.output().to_vec(), closed: false };
        running.session.clear_output();
        if running.session.is_closed() {
            effects.closed = true;
            self.running = None;
            return effects;
        }
        if let Some(deadline) = running.session.next_deadline().map(|d| self.clocks.sim_time(d))
            && deadline < running.timer
            && running.stuck != Some(deadline)
        {
            running.timer = deadline;
        }
        effects
    }

    /// What the driver would do next, for the world to schedule.
    pub fn wants(&self) -> Option<Wants> {
        let running = self.running.as_ref()?;
        let resending = running.session.is_resending();
        Some(Wants {
            resume: resending,
            read: !resending && !running.unread.is_empty(),
            commands: !resending && !running.commands.is_empty(),
            timer: Some(running.timer),
        })
    }
}
