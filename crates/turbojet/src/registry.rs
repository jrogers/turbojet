//! Tracks live sessions and lets code outside the connection task send on them.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::{mpsc, oneshot};
use tracing::info;

use crate::fields::ApplVerId;
use crate::message::Message;
use crate::schedule::Clock;
use crate::store::{MemoryStorage, SessionId, SessionLog, SessionStorage};

/// A request to a session's connection task.
#[derive(Debug)]
pub enum Command {
    /// Send an application message; see [`SessionHandle::send`].
    Send(Message),
    /// Log out, with this Text(58) if any; see [`SessionHandle::logout`].
    Logout(Option<String>),
    /// An operator change to sequence numbers, answered on the channel.
    Sequence(SequenceCommand, oneshot::Sender<Result<SequenceNumbers, SequenceError>>),
}

/// An operator request to inspect or change a session's sequence numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceCommand {
    /// Report them unchanged; see [`SessionHandle::sequence_numbers`].
    Get,
    /// See [`SessionHandle::set_next_incoming`].
    SetNextIncoming(u64),
    /// See [`SessionHandle::set_next_outgoing`].
    SetNextOutgoing(u64),
    /// See [`SessionHandle::reset_sequence_numbers`].
    Reset,
}

/// A session's next expected inbound and next outbound MsgSeqNum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceNumbers {
    /// The MsgSeqNum expected on the next message from the counterparty.
    pub next_incoming: u64,
    /// The MsgSeqNum of the next message we send.
    pub next_outgoing: u64,
}

/// Why an operator change to sequence numbers was refused.
#[derive(Debug)]
pub enum SequenceError {
    /// The value isn't allowed (zero, or moving outgoing numbers backwards).
    Invalid(String),
    /// The change needs the session to be disconnected (resetting to 1).
    Connected,
    /// The session store failed.
    Storage(io::Error),
}

impl fmt::Display for SequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(reason) => f.write_str(reason),
            Self::Connected => f.write_str("the session is connected; disconnect it first"),
            Self::Storage(e) => write!(f, "session store failed: {e}"),
        }
    }
}

impl std::error::Error for SequenceError {}

impl From<io::Error> for SequenceError {
    fn from(e: io::Error) -> Self {
        Self::Storage(e)
    }
}

/// Applies `command` to a session's stored state. The connection task uses this for a live
/// session (after any protocol side effects); the registry uses it for one that isn't connected.
pub(crate) fn apply_sequence_command(
    log: &mut dyn SessionLog,
    command: SequenceCommand,
    clock: &Clock,
) -> Result<SequenceNumbers, SequenceError> {
    match command {
        SequenceCommand::Get => {}
        SequenceCommand::SetNextIncoming(0) | SequenceCommand::SetNextOutgoing(0) => {
            return Err(SequenceError::Invalid("sequence numbers start at 1".into()));
        }
        SequenceCommand::SetNextIncoming(seq) => log.set_next_incoming(seq)?,
        SequenceCommand::SetNextOutgoing(seq) => {
            let current = log.next_outgoing();
            if seq < current {
                return Err(SequenceError::Invalid(format!(
                    "outgoing sequence numbers only move forward (next is {current}); reset to start again"
                )));
            }
            if seq > current {
                // Record the skipped numbers as used, without messages: resends gap-fill them.
                log.record_outgoing(seq - 1, None)?;
            }
        }
        SequenceCommand::Reset => {
            log.reset()?;
            log.set_created_at(clock.now())?;
        }
    }
    Ok(SequenceNumbers { next_incoming: log.next_incoming(), next_outgoing: log.next_outgoing() })
}

/// Why a session couldn't be bound.
#[derive(Debug)]
pub(crate) enum AcquireError {
    AlreadyConnected(SessionId),
    Storage(SessionId, io::Error),
}

impl fmt::Display for AcquireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyConnected(id) => write!(f, "{id} is already connected"),
            Self::Storage(id, e) => write!(f, "cannot open session store for {id}: {e}"),
        }
    }
}

/// Sends [`Command`]s to a session's connection task.
pub type CommandSender = mpsc::UnboundedSender<Command>;
/// A connection task's end of a [`CommandSender`].
pub type CommandReceiver = mpsc::UnboundedReceiver<Command>;

/// A connected session's registration.
struct Entry {
    commands: CommandSender,
    /// FIXT sessions: the application version in use on this connection.
    appl_ver_id: Option<ApplVerId>,
}

/// Opens session logs from storage, ensures each session runs on at most one connection at a
/// time, and routes [`SessionHandle`] commands to that connection.
pub struct SessionRegistry {
    storage: Arc<dyn SessionStorage>,
    sessions: Mutex<HashMap<SessionId, Entry>>,
    /// For creation times recorded by operator resets of disconnected sessions.
    clock: Clock,
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new(Arc::new(MemoryStorage::new()))
    }
}

impl SessionRegistry {
    /// A registry opening session logs from `storage`. The [`Default`] registry keeps them in
    /// memory.
    pub fn new(storage: Arc<dyn SessionStorage>) -> Self {
        Self { storage, sessions: Mutex::default(), clock: Clock::system() }
    }

    /// Uses `clock` for creation times recorded by operator resets (normally the sessions'
    /// [`SessionConfig::clock`](crate::SessionConfig::clock)).
    pub fn with_clock(mut self, clock: Clock) -> Self {
        self.clock = clock;
        self
    }

    /// A handle for `id`. It can be created before the session connects and stays valid across
    /// reconnects; sends succeed while the session is connected.
    pub fn handle(self: &Arc<Self>, id: SessionId) -> SessionHandle {
        SessionHandle { id, registry: self.clone() }
    }

    /// Sessions currently bound to a connection.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.lock().keys().cloned().collect()
    }

    /// Binds `id` to a connection and opens its log. Fails if it is already bound elsewhere or
    /// storage cannot be opened.
    pub(crate) fn acquire(
        &self,
        id: &SessionId,
        commands: CommandSender,
        appl_ver_id: Option<ApplVerId>,
    ) -> Result<Box<dyn SessionLog>, AcquireError> {
        {
            let mut sessions = self.lock();
            if sessions.contains_key(id) {
                return Err(AcquireError::AlreadyConnected(id.clone()));
            }
            sessions.insert(id.clone(), Entry { commands, appl_ver_id });
        }
        self.storage.open(id).map_err(|e| {
            self.release(id);
            AcquireError::Storage(id.clone(), e)
        })
    }

    /// Applies an operator command to a session that isn't connected, holding its registration
    /// meanwhile so it can't log on half-way through.
    fn apply_offline(&self, id: &SessionId, command: SequenceCommand) -> Result<SequenceNumbers, SequenceError> {
        // No receiver: a send through a handle during the change fails as not connected.
        let (placeholder, _) = mpsc::unbounded_channel();
        let mut log = self.acquire(id, placeholder, None).map_err(|e| match e {
            AcquireError::AlreadyConnected(_) => SequenceError::Connected,
            AcquireError::Storage(_, e) => SequenceError::Storage(e),
        })?;
        let result = apply_sequence_command(log.as_mut(), command, &self.clock);
        drop(log);
        self.release(id);
        if let (Ok(numbers), false) = (&result, command == SequenceCommand::Get) {
            info!(session = %id, ?command, ?numbers, "sequence numbers changed by operator (session disconnected)");
        }
        result
    }

    pub(crate) fn release(&self, id: &SessionId) {
        self.lock().remove(id);
    }

    fn sender(&self, id: &SessionId) -> Option<CommandSender> {
        self.lock().get(id).map(|e| e.commands.clone())
    }

    fn appl_ver_id(&self, id: &SessionId) -> Option<ApplVerId> {
        self.lock().get(id).and_then(|e| e.appl_ver_id)
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<SessionId, Entry>> {
        self.sessions.lock().expect("session registry lock poisoned")
    }
}

/// Sends on a session from anywhere: other tasks, threads, or application callbacks.
///
/// Messages are sequenced, persisted and written by the session's connection task in the order
/// they were sent. The handle counts as connected from the moment a connection claims the
/// session (for an initiator, when it sends Logon). Commands sent while logon is still in
/// progress are queued and go out, in order, as soon as it completes. A command is dropped (and
/// logged) only if the connection ends before logon completes, or if it arrives after logout has
/// started.
#[derive(Clone)]
pub struct SessionHandle {
    id: SessionId,
    registry: Arc<SessionRegistry>,
}

impl SessionHandle {
    /// The session this handle sends on.
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    /// Whether a connection has claimed the session (see [`SessionHandle`]) and not yet ended.
    /// Commands fail with [`NotConnected`] otherwise.
    pub fn is_connected(&self) -> bool {
        self.registry.sender(&self.id).is_some()
    }

    /// FIXT.1.1 sessions: the default application version (DefaultApplVerID(1137)) on the current
    /// connection: an initiator's first configured one from when it sends Logon (a reply naming
    /// another ends the connection), or the one an acceptor chose from the counterparty's Logon.
    /// Messages may name another the session supports (see [`send`](Self::send)). `None`
    /// when disconnected, and always for FIX 4.x sessions.
    pub fn appl_ver_id(&self) -> Option<ApplVerId> {
        self.registry.appl_ver_id(&self.id)
    }

    /// Queues an application message; the engine adds the standard header. A message with SOH
    /// inside a value is dropped (and logged), since it would add fields on the wire. On FIXT.1.1
    /// sessions, ApplVerID(1128) may name any version the session supports (the default goes
    /// unstated), and a message naming another is dropped (and logged); CstmApplVerID(1129) and
    /// ApplExtID(1156) pass through. On FIX 4.x sessions, ApplVerID(1128) is dropped.
    pub fn send(&self, msg: impl Into<Message>) -> Result<(), NotConnected> {
        self.command(Command::Send(msg.into()))
    }

    /// Starts an orderly logout.
    pub fn logout(&self, text: Option<&str>) -> Result<(), NotConnected> {
        self.command(Command::Logout(text.map(String::from)))
    }

    // ---- Operator control of sequence numbers ----
    //
    // These work whether or not the session is connected. A connected session applies the change
    // on its connection task, keeping its state (and, where needed, the counterparty) in step; a
    // disconnected one has its stored state changed directly.

    /// The session's next incoming and outgoing sequence numbers.
    pub async fn sequence_numbers(&self) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::Get).await
    }

    /// Sets the next MsgSeqNum expected from the counterparty. It can go down as well as up:
    /// lowering it means messages already processed may be accepted again.
    pub async fn set_next_incoming(&self, seq: u64) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::SetNextIncoming(seq)).await
    }

    /// Moves the next outgoing MsgSeqNum forward (it can't go back; see
    /// [`reset_sequence_numbers`](Self::reset_sequence_numbers)). If the session is logged on,
    /// the counterparty is told with a SequenceReset (reset mode) to expect `seq` next.
    pub async fn set_next_outgoing(&self, seq: u64) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::SetNextOutgoing(seq)).await
    }

    /// Resets both sequence numbers to 1 and clears the resend store. Only while the session is
    /// disconnected ([`SequenceError::Connected`] otherwise); for a reset with the counterparty,
    /// log on with ResetSeqNumFlag instead.
    pub async fn reset_sequence_numbers(&self) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::Reset).await
    }

    async fn sequence(&self, command: SequenceCommand) -> Result<SequenceNumbers, SequenceError> {
        if let Some(sender) = self.registry.sender(&self.id) {
            let (reply, answer) = oneshot::channel();
            if sender.send(Command::Sequence(command, reply)).is_ok()
                && let Ok(result) = answer.await
            {
                return result;
            }
            // The connection ended in the meantime: fall back to the stored state.
        }
        self.registry.apply_offline(&self.id, command)
    }

    fn command(&self, command: Command) -> Result<(), NotConnected> {
        let sender = self.registry.sender(&self.id).ok_or(NotConnected)?;
        sender.send(command).map_err(|_| NotConnected)
    }
}

impl fmt::Debug for SessionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionHandle").field("id", &self.id).finish()
    }
}

/// A command sent through a [`SessionHandle`] while its session isn't connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotConnected;

impl fmt::Display for NotConnected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("session is not connected")
    }
}

impl std::error::Error for NotConnected {}
