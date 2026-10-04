//! Tracks live sessions and lets code outside the connection task send on them.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use tokio::sync::{Notify, mpsc, oneshot};
use tracing::info;

use crate::application::{Application, Disconnect};
use crate::cancel::{CancelTracker, CancelTrigger};
use crate::fields::ApplVerId;
use crate::message::Message;
use crate::schedule::Clock;
use crate::store::{MemoryStorage, Opened, SessionId, SessionLog, SessionStorage};

/// A request to a session's connection task.
#[derive(Debug)]
pub enum Command {
    /// Send an application message, and say what became of it on the reply, if any; see
    /// [`SessionHandle::send`].
    Send(Message, Option<ReceiptSender>),
    /// Log out, with this Text(58) if any; see [`SessionHandle::logout`].
    Logout(Option<String>),
    /// An operator change to sequence numbers, answered on the channel.
    Sequence(SequenceCommand, oneshot::Sender<Result<SequenceNumbers, SequenceError>>),
}

impl Command {
    /// Send `msg`, with no one waiting to hear what became of it.
    pub fn send(msg: Message) -> Self {
        Self::Send(msg, None)
    }
}

/// The session's end of a [`Receipt`]: answered with the message's MsgSeqNum once it's stored
/// and committed, or why it was dropped. Dropped unanswered, the receipt reads [`Dropped::Disconnected`].
pub type ReceiptSender = oneshot::Sender<Result<u64, Dropped>>;

/// What became of a message queued with [`SessionHandle::send`]: a future that resolves to its
/// MsgSeqNum once the session has stored it and the store has committed it (from then on it's
/// resent if the counterparty misses it, across reconnects and restarts), or to why it was
/// dropped. Ignoring it is fine.
#[derive(Debug)]
pub struct Receipt(oneshot::Receiver<Result<u64, Dropped>>);

impl Receipt {
    /// The outcome, if there's one yet.
    pub fn try_outcome(&mut self) -> Option<Result<u64, Dropped>> {
        match self.0.try_recv() {
            Ok(outcome) => Some(outcome),
            Err(oneshot::error::TryRecvError::Empty) => None,
            Err(oneshot::error::TryRecvError::Closed) => Some(Err(Dropped::Disconnected)),
        }
    }
}

impl std::future::Future for Receipt {
    type Output = Result<u64, Dropped>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0).poll(cx).map(|answer| answer.unwrap_or(Err(Dropped::Disconnected)))
    }
}

/// Why a message queued with [`SessionHandle::send`] was never stored or sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dropped {
    /// The connection ended first: while the message was queued, or held during a logon that
    /// never completed.
    Disconnected,
    /// The session had started logging out.
    LoggingOut,
    /// The session wouldn't send it, for the reason given: a value containing SOH, an
    /// ApplVerID(1128) the session doesn't support, or a session-level message type.
    Rejected(String),
    /// The session store failed while recording or committing it, and the session disconnects;
    /// or the connection ended while its commit was under way. A store can fail after the write
    /// took effect (a failed fsync, say), so the message may have been stored after all: if so,
    /// it's resent when the counterparty asks for it, as any stored message is. Sending it again
    /// risks a duplicate.
    Storage,
}

impl fmt::Display for Dropped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected => f.write_str("the connection ended before the message was sent"),
            Self::LoggingOut => f.write_str("the session was logging out"),
            Self::Rejected(reason) => write!(f, "the session wouldn't send it: {reason}"),
            Self::Storage => f.write_str("the session store failed or stopped recording it (it may have been stored)"),
        }
    }
}

impl std::error::Error for Dropped {}

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
            log.set_created_at(clock.now().into())?;
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

/// A session claimed in the registry for an operator change, released when dropped.
struct Claim<'a> {
    registry: &'a SessionRegistry,
    id: &'a SessionId,
}

impl Drop for Claim<'_> {
    fn drop(&mut self) {
        self.registry.release(self.id);
    }
}

impl fmt::Display for AcquireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyConnected(id) => write!(f, "{id} is already connected"),
            Self::Storage(id, e) => write!(f, "cannot open session store for {id}: {e}"),
        }
    }
}

/// Most logout and operator commands queued for a session at once. They have a queue of their own,
/// which the connection takes from first, so a queue full of sends never holds them up.
pub const CONTROL_QUEUE: usize = 64;

/// A logout or operator command, as queued.
#[derive(Debug)]
enum Control {
    /// Log out once the first `after` sends have been taken: those queued before it.
    Logout {
        text: Option<String>,
        after: u64,
    },
    Sequence(SequenceCommand, oneshot::Sender<Result<SequenceNumbers, SequenceError>>),
}

/// The queues that carry [`SessionHandle`] commands to a session's connection task: application
/// sends, bounded by [`SessionConfig::send_queue`](crate::SessionConfig::send_queue), and
/// logout and operator commands, bounded by [`CONTROL_QUEUE`].
#[derive(Debug, Clone)]
pub struct CommandSender {
    sends: mpsc::Sender<(Message, ReceiptSender)>,
    control: mpsc::Sender<Control>,
    /// Sends queued so far, for a Logout to wait behind.
    queued: Arc<AtomicU64>,
}

/// A connection task's end of a [`CommandSender`]. A driver takes [`control`](Self::try_control)
/// commands whenever it can, and [`sends`](Self::try_send) when the session can take them; a
/// Logout comes out of the control queue only once every send queued before it has been taken,
/// so a message sent before logging out still goes out first.
#[derive(Debug)]
pub struct CommandReceiver {
    sends: mpsc::Receiver<(Message, ReceiptSender)>,
    control: mpsc::Receiver<Control>,
    /// Sends taken so far.
    taken: u64,
    /// A Logout waiting for the sends queued before it.
    held: Option<(Option<String>, u64)>,
    /// A send taken off its queue only to notice it (see [`Sends::Notice`]), first in line. It
    /// isn't counted in `taken` until it's handed out, so a Logout queued after it still waits.
    /// It has left the bounded queue, so while it's here one more send than `send_queue` waits.
    noticed: Option<(Message, ReceiptSender)>,
}

/// What [`CommandReceiver::next_with`] does with application sends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sends {
    /// Hands the next one out.
    Take,
    /// Returns [`Next::Noticed`] once one is waiting, keeping it first in line: for a driver that
    /// can't take sends yet (the outbound window is full) but must know when one waits.
    Notice,
    /// Leaves them queued.
    Ignore,
}

/// What [`CommandReceiver::next_with`] found.
#[derive(Debug)]
pub(crate) enum Next {
    Command(Command),
    /// A send is waiting, not taken; see [`Sends::Notice`].
    Noticed,
}

/// A session's command queues, sends holding up to `send_queue` messages.
pub fn command_queues(send_queue: usize) -> (CommandSender, CommandReceiver) {
    assert!(send_queue > 0, "a send queue holds at least one message");
    let (sends, sends_rx) = mpsc::channel(send_queue);
    let (control, control_rx) = mpsc::channel(CONTROL_QUEUE);
    let sender = CommandSender { sends, control, queued: Arc::default() };
    (sender, CommandReceiver { sends: sends_rx, control: control_rx, taken: 0, held: None, noticed: None })
}

impl CommandReceiver {
    /// The next logout or operator command that's due, if any: operator commands at once, a
    /// Logout once the sends queued before it have been taken.
    pub fn try_control(&mut self) -> Option<Command> {
        loop {
            if let Some((_, after)) = &self.held
                && self.taken >= *after
            {
                let (text, _) = self.held.take().expect("checked");
                return Some(Command::Logout(text));
            }
            let control = self.control.try_recv().ok()?;
            if let Some(command) = self.take(control) {
                return Some(command);
            }
        }
    }

    /// Waits for the next command: a logout or operator command that's due, before anything
    /// else, or, if `sends`, an application message. Cancel-safe: a Logout that has arrived but
    /// isn't due yet is kept for later.
    pub async fn next(&mut self, sends: bool) -> Option<Command> {
        match self.next_with(if sends { Sends::Take } else { Sends::Ignore }).await? {
            Next::Command(command) => Some(command),
            Next::Noticed => unreachable!("sends are only noticed when asked to be"),
        }
    }

    /// [`next`](Self::next), with application sends taken, noticed or ignored as `sends` says.
    /// Cancel-safe, as `next` is: a noticed send is kept, first in line.
    pub(crate) async fn next_with(&mut self, sends: Sends) -> Option<Next> {
        loop {
            if let Some(command) = self.try_control() {
                return Some(Next::Command(command));
            }
            if sends == Sends::Take
                && let Some(command) = self.take_noticed()
            {
                return Some(Next::Command(command));
            }
            // One noticed send is enough: then only control commands are waited for.
            let receiving = match sends {
                Sends::Take => true,
                Sends::Notice => self.noticed.is_none(),
                Sends::Ignore => false,
            };
            tokio::select! {
                biased;
                control = self.control.recv() => {
                    if let Some(command) = self.take(control?) {
                        return Some(Next::Command(command));
                    }
                }
                send = self.sends.recv(), if receiving => {
                    let (msg, receipt) = send?;
                    if sends == Sends::Notice {
                        self.noticed = Some((msg, receipt));
                        return Some(Next::Noticed);
                    }
                    self.taken += 1;
                    return Some(Next::Command(Command::Send(msg, Some(receipt))));
                }
            }
        }
    }

    /// The noticed send, if any, now taken.
    fn take_noticed(&mut self) -> Option<Command> {
        let (msg, receipt) = self.noticed.take()?;
        self.taken += 1;
        Some(Command::Send(msg, Some(receipt)))
    }

    /// A control command as it comes off its queue: a Logout not yet due is held (a second one
    /// while one is held adds nothing).
    fn take(&mut self, control: Control) -> Option<Command> {
        match control {
            Control::Sequence(command, reply) => Some(Command::Sequence(command, reply)),
            Control::Logout { text, after } if self.taken >= after && self.held.is_none() => {
                Some(Command::Logout(text))
            }
            Control::Logout { text, after } => {
                self.held.get_or_insert((text, after));
                None
            }
        }
    }

    /// The next queued application message, if any.
    pub fn try_send(&mut self) -> Option<Command> {
        if let Some(command) = self.take_noticed() {
            return Some(command);
        }
        let (msg, receipt) = self.sends.try_recv().ok()?;
        self.taken += 1;
        Some(Command::Send(msg, Some(receipt)))
    }

    /// Whether application messages are waiting.
    pub fn has_sends(&self) -> bool {
        self.noticed.is_some() || !self.sends.is_empty()
    }

    /// Whether a logout or operator command could be taken now: one queued, or a Logout whose
    /// sends have been taken. A Logout still waiting for sends doesn't count.
    pub fn has_control(&self) -> bool {
        !self.control.is_empty() || self.held.as_ref().is_some_and(|(_, after)| self.taken >= *after)
    }
}

/// A connected session's registration.
struct Entry {
    commands: CommandSender,
    /// FIXT sessions: the application version in use on this connection.
    appl_ver_id: Option<ApplVerId>,
}

/// Opens session logs from storage, ensures each session runs on at most one connection at a
/// time, routes [`SessionHandle`] commands to that connection, and keeps the
/// [cancel-on-disconnect](crate::SessionConfig::cancel_on_disconnect) countdowns of its sessions.
pub struct SessionRegistry {
    storage: Arc<dyn SessionStorage>,
    /// Connected sessions only: an entry is removed when its connection releases it.
    sessions: Mutex<HashMap<SessionId, Entry>>,
    /// For creation times recorded by operator resets of disconnected sessions.
    clock: Clock,
    /// Locked on its own. Nothing takes its lock with `sessions` held; a cancel callback may
    /// take `sessions` under it (sending through a `SessionHandle`), which is the only order.
    cancels: CancelTracker,
    /// Whether the task driving `cancels` has been spawned: one per registry, however many
    /// acceptors and initiators share it, so the wake has one waiter.
    cancel_task: AtomicBool,
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
        Self {
            storage,
            sessions: Mutex::default(),
            clock: Clock::system(),
            cancels: CancelTracker::new(),
            cancel_task: AtomicBool::new(false),
        }
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

    /// Binds `id` to a connection and opens its log, at once or by a job (see
    /// [`SessionStorage::begin_open`]); if the job fails, the caller releases `id`. Fails if it is
    /// already bound elsewhere or storage cannot be opened.
    pub(crate) fn acquire(
        &self,
        id: &SessionId,
        commands: CommandSender,
        appl_ver_id: Option<ApplVerId>,
    ) -> Result<Opened, AcquireError> {
        {
            let mut sessions = self.lock();
            if sessions.contains_key(id) {
                return Err(AcquireError::AlreadyConnected(id.clone()));
            }
            sessions.insert(id.clone(), Entry { commands, appl_ver_id });
        }
        self.storage.begin_open(id).map_err(|e| {
            self.release(id);
            AcquireError::Storage(id.clone(), e)
        })
    }

    /// Applies an operator command to a session that isn't connected, holding its registration
    /// meanwhile so it can't log on half-way through.
    async fn apply_offline(&self, id: &SessionId, command: SequenceCommand) -> Result<SequenceNumbers, SequenceError> {
        // No receiver: a send through a handle during the change fails as not connected.
        let (placeholder, _) = command_queues(1);
        let opened = self.acquire(id, placeholder, None).map_err(|e| match e {
            AcquireError::AlreadyConnected(_) => SequenceError::Connected,
            AcquireError::Storage(_, e) => SequenceError::Storage(e),
        })?;
        // Declared before the log, so dropped after it: the log is closed (its lock released)
        // before the session can be claimed again, even if the caller gives up on this future.
        let _claim = Claim { registry: self, id };
        let mut log = match opened {
            Opened::Ready(log) => log,
            Opened::Pending(job) => job.run_here().await.map_err(SequenceError::Storage)?,
        };
        let result = match apply_sequence_command(log.as_mut(), command, &self.clock) {
            Ok(numbers) => match log.commit() {
                Ok(None) => Ok(numbers),
                Ok(Some(commit)) => commit.run_here().await.map(|()| numbers).map_err(SequenceError::Storage),
                Err(e) => Err(SequenceError::Storage(e)),
            },
            Err(e) => Err(e),
        };
        if let (Ok(numbers), false) = (&result, command == SequenceCommand::Get) {
            info!(session = %id, ?command, ?numbers, "sequence numbers changed by operator (session disconnected)");
        }
        result
    }

    pub(crate) fn release(&self, id: &SessionId) {
        self.lock().remove(id);
    }

    // ---- Cancel on disconnect ----
    //
    // The registry keeps the countdowns, rather than each session, because a countdown outlives
    // the connection that started it, and the next connection for the session must stop it.
    // `Acceptor` and `Initiator` drive them on a task of their own (`spawn_cancel_task`), and
    // when one shuts down it fires its own sessions' pending ones: a registry can be shared, and
    // another acceptor or initiator's sessions carry on. A custom driver, or a simulation, drives
    // them with the three public methods below, as it drives a `Session`.

    /// Starts the cancel-on-disconnect countdown for `id`, which ended with `ended`, to fire at
    /// `deadline` on `app`. A countdown already under way for `id` keeps its deadline.
    pub(crate) fn start_cancel(
        &self,
        id: SessionId,
        ended: Disconnect,
        trigger: CancelTrigger,
        deadline: Instant,
        app: Arc<dyn Application>,
    ) {
        self.cancels.start(id, ended, trigger, deadline, app);
    }

    /// `id` has logged on: its countdown, if any, stops. Waits for its cancel if one is running.
    pub(crate) fn stop_cancel(&self, id: &SessionId) {
        self.cancels.logged_on(id);
    }

    /// When the next cancel-on-disconnect countdown ends, if one is under way: when to call
    /// [`run_due_cancels`](Self::run_due_cancels) next.
    pub fn next_cancel_deadline(&self) -> Option<Instant> {
        self.cancels.next_deadline()
    }

    /// Fires the cancel-on-disconnect countdowns that have ended by `now`, calling
    /// [`on_cancel_on_disconnect`](Application::on_cancel_on_disconnect) for each, in deadline
    /// order. A logon of one of these sessions waits until its callback has returned. Calling it
    /// early is harmless: it acts only on what is due. Calling it from inside
    /// `on_cancel_on_disconnect` deadlocks; from other callbacks it's fine.
    pub fn run_due_cancels(&self, now: Instant) {
        self.cancels.run_due(now);
    }

    /// Fires every cancel-on-disconnect countdown under way, due or not, as a shutdown does.
    /// Calling it from inside `on_cancel_on_disconnect` deadlocks; from other callbacks it's fine.
    pub fn run_all_cancels(&self) {
        self.cancels.run_all();
    }

    /// Fires the countdowns under way of the sessions `which` picks, due or not: those of an
    /// acceptor or initiator shutting down.
    pub(crate) fn run_cancels_now(&self, which: impl Fn(&SessionId) -> bool) {
        self.cancels.run_now(which);
    }

    /// Spawns the task that fires the countdowns as they fall due, unless it's running already.
    /// The acceptor and initiator call it as each connection starts, so a countdown, which only a
    /// session ending can start, always has the task to fire it. Needs a tokio runtime.
    ///
    /// The task holds the registry only while it fires what's due, so it doesn't keep it alive.
    /// It ends once the registry has gone, woken by the registry's `Drop`. It runs application
    /// callbacks, which must not block, on the runtime like any other task, and reads tokio's
    /// clock, as the connections do, so that paused time in tests moves the deadlines too.
    pub(crate) fn spawn_cancel_task(self: &Arc<Self>) {
        if self.cancel_task.swap(true, Ordering::AcqRel) {
            return;
        }
        let registry = Arc::downgrade(self);
        let wake: Arc<Notify> = self.cancels.wake.clone();
        tokio::spawn(async move {
            loop {
                // Not held across the wait, so the registry can go meanwhile.
                let deadline = match registry.upgrade() {
                    Some(registry) => {
                        registry.run_due_cancels(tokio::time::Instant::now().into_std());
                        registry.next_cancel_deadline()
                    }
                    None => return,
                };
                // A countdown started since the deadline was read has left a permit, so the wait
                // ends at once and the deadline is read again.
                match deadline {
                    Some(at) => tokio::select! {
                        () = tokio::time::sleep_until(at.into()) => {}
                        () = wake.notified() => {}
                    },
                    None => wake.notified().await,
                }
            }
        });
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

impl Drop for SessionRegistry {
    fn drop(&mut self) {
        // Wakes the task driving the countdowns, if any, to find the registry gone and end.
        // Countdowns still pending go with it: the acceptor or initiator that would fire them, and
        // every session that could log back on, have gone too.
        self.cancels.wake.notify_one();
    }
}

/// Sends on a session from anywhere: other tasks, threads, or application callbacks.
///
/// Messages are sequenced, persisted and written by the session's connection task in the order
/// they were sent. The handle counts as connected from the moment a connection claims the
/// session (for an initiator, when it sends Logon). Messages sent while logon is still in progress
/// wait in their bounded queue and go out, in order, as soon as it completes. A message is dropped
/// (and logged) if the connection ends while it's queued, or if logout has started; its
/// [`Receipt`] says so.
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
    /// Commands fail as not connected otherwise.
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
    ///
    /// The queue holds up to [`SessionConfig::send_queue`](crate::SessionConfig::send_queue)
    /// messages; when it's full, as when the counterparty reads more slowly than the application
    /// sends, the message comes back in [`SendError::Full`] rather than waiting.
    /// [`send_when_ready`](Self::send_when_ready) waits for room instead.
    ///
    /// Queued, the message comes with a [`Receipt`]: it resolves to the MsgSeqNum once the
    /// session has stored the message, or to why it was dropped, such as the connection ending
    /// first. Ignore it for fire-and-forget.
    pub fn send(&self, msg: impl Into<Message>) -> Result<Receipt, SendError> {
        let msg = msg.into();
        let Some(sender) = self.registry.sender(&self.id) else { return Err(SendError::NotConnected(msg)) };
        let (reply, receipt) = oneshot::channel();
        match sender.sends.try_send((msg, reply)) {
            Ok(()) => {
                sender.queued.fetch_add(1, Ordering::AcqRel);
                Ok(Receipt(receipt))
            }
            Err(mpsc::error::TrySendError::Full((msg, _))) => Err(SendError::Full(msg)),
            Err(mpsc::error::TrySendError::Closed((msg, _))) => Err(SendError::NotConnected(msg)),
        }
    }

    /// Queues an application message as [`send`](Self::send) does, waiting for room in the queue
    /// if it's full. Fails only if the session isn't connected, or the connection ends meanwhile.
    pub async fn send_when_ready(&self, msg: impl Into<Message>) -> Result<Receipt, SendError> {
        let msg = msg.into();
        let Some(sender) = self.registry.sender(&self.id) else { return Err(SendError::NotConnected(msg)) };
        let (reply, receipt) = oneshot::channel();
        match sender.sends.send((msg, reply)).await {
            Ok(()) => {
                sender.queued.fetch_add(1, Ordering::AcqRel);
                Ok(Receipt(receipt))
            }
            Err(mpsc::error::SendError((msg, _))) => Err(SendError::NotConnected(msg)),
        }
    }

    /// Starts an orderly logout, once the messages already queued through
    /// [`send`](Self::send) have gone. It has a queue of its own, so a full send queue doesn't
    /// hold it up.
    pub fn logout(&self, text: Option<&str>) -> Result<(), CommandError> {
        let sender = self.registry.sender(&self.id).ok_or(CommandError::NotConnected)?;
        let after = sender.queued.load(Ordering::Acquire);
        let logout = Control::Logout { text: text.map(String::from), after };
        sender.control.try_send(logout).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => CommandError::Full,
            mpsc::error::TrySendError::Closed(_) => CommandError::NotConnected,
        })
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
    /// the counterparty is told to expect `seq` next with a SequenceReset in gap-fill mode, which a
    /// counterparty still filling a gap applies only once it's filled, so nothing sent before is
    /// lost; otherwise it finds out from the next message, and asks for the skipped numbers.
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
            if sender.control.send(Control::Sequence(command, reply)).await.is_ok()
                && let Ok(result) = answer.await
            {
                return result;
            }
            // The connection ended in the meantime: fall back to the stored state.
        }
        self.registry.apply_offline(&self.id, command).await
    }
}

impl fmt::Debug for SessionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionHandle").field("id", &self.id).finish()
    }
}

/// Why [`SessionHandle::send`] didn't queue a message, with the message, to retry or keep.
#[derive(Debug, Clone)]
pub enum SendError {
    /// No connection has the session (see [`SessionHandle`]).
    NotConnected(Message),
    /// The session's send queue is full: the counterparty is reading more slowly than the
    /// application sends.
    Full(Message),
}

impl SendError {
    /// The message that wasn't queued.
    pub fn into_message(self) -> Message {
        match self {
            Self::NotConnected(msg) | Self::Full(msg) => msg,
        }
    }
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected(_) => f.write_str("session is not connected"),
            Self::Full(_) => f.write_str("session's send queue is full"),
        }
    }
}

impl std::error::Error for SendError {}

/// Why [`SessionHandle::logout`] didn't queue the logout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    /// No connection has the session (see [`SessionHandle`]).
    NotConnected,
    /// [`CONTROL_QUEUE`] commands are already waiting.
    Full,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected => f.write_str("session is not connected"),
            Self::Full => f.write_str("session's control queue is full"),
        }
    }
}

impl std::error::Error for CommandError {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::fields::MsgType;
    use crate::message::tags;

    /// Queues `id` as [`SessionHandle::send`] does.
    fn send(sender: &CommandSender, id: &str) -> Receipt {
        let (reply, receipt) = oneshot::channel();
        let msg = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id);
        sender.sends.try_send((msg, reply)).unwrap();
        sender.queued.fetch_add(1, Ordering::AcqRel);
        Receipt(receipt)
    }

    /// Queues a Logout as [`SessionHandle::logout`] does.
    fn logout(sender: &CommandSender) {
        let after = sender.queued.load(Ordering::Acquire);
        sender.control.try_send(Control::Logout { text: None, after }).unwrap();
    }

    fn sent_id(command: Option<Command>) -> String {
        match command {
            Some(Command::Send(msg, _)) => msg.get(tags::CL_ORD_ID).unwrap().to_string(),
            other => panic!("not a send: {other:?}"),
        }
    }

    fn taken(next: Option<Next>) -> Option<Command> {
        match next {
            Some(Next::Command(command)) => Some(command),
            other => panic!("not a command: {other:?}"),
        }
    }

    /// A noticed send leaves the queue but isn't taken, and still counts as waiting.
    #[tokio::test]
    async fn notice_keeps_a_send_without_taking_it() {
        let (sender, mut receiver) = command_queues(4);
        let _receipt = send(&sender, "A");
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        assert_eq!(receiver.taken, 0);
        assert!(receiver.sends.is_empty());
        assert!(receiver.has_sends(), "the noticed send is waiting");
    }

    /// With a send noticed, Notice waits for control commands only, leaving the queue alone.
    #[tokio::test(start_paused = true)]
    async fn notice_with_a_send_noticed_waits_only_for_control() {
        let (sender, mut receiver) = command_queues(4);
        let _receipts = [send(&sender, "A"), send(&sender, "B")];
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        let waited = tokio::time::timeout(Duration::from_secs(1), receiver.next_with(Sends::Notice)).await;
        assert!(waited.is_err(), "returned {waited:?}");
        assert_eq!(receiver.sends.len(), 1, "B is still queued");

        let (reply, _answer) = oneshot::channel();
        sender.control.try_send(Control::Sequence(SequenceCommand::Get, reply)).unwrap();
        let next = receiver.next_with(Sends::Notice).await;
        assert!(matches!(next, Some(Next::Command(Command::Sequence(SequenceCommand::Get, _)))), "{next:?}");
        assert_eq!(receiver.taken, 0);
    }

    /// The noticed send is handed out first, by Take or try_send, then the queue, in order.
    #[tokio::test]
    async fn the_noticed_send_comes_out_first() {
        let (sender, mut receiver) = command_queues(4);
        let _receipts = [send(&sender, "A"), send(&sender, "B"), send(&sender, "C"), send(&sender, "D")];
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        assert_eq!(sent_id(receiver.try_send()), "A");
        assert_eq!(sent_id(taken(receiver.next_with(Sends::Take).await)), "B");
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        assert_eq!(sent_id(taken(receiver.next_with(Sends::Take).await)), "C");
        assert_eq!(sent_id(receiver.try_send()), "D");
        assert_eq!(receiver.taken, 4);
        assert!(!receiver.has_sends());
    }

    /// A Logout queued after a noticed send waits until it has been taken.
    #[tokio::test]
    async fn a_logout_waits_for_the_noticed_send() {
        let (sender, mut receiver) = command_queues(4);
        let _receipt = send(&sender, "A");
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        logout(&sender);
        assert!(receiver.try_control().is_none());
        assert!(!receiver.has_control());
        assert_eq!(sent_id(receiver.try_send()), "A");
        assert!(receiver.has_control());
        assert!(matches!(receiver.try_control(), Some(Command::Logout(None))));
    }

    /// A noticed send dropped with the receiver, as when the connection ends, says so.
    #[tokio::test]
    async fn a_noticed_send_dropped_with_the_receiver_is_disconnected() {
        let (sender, mut receiver) = command_queues(4);
        let receipt = send(&sender, "A");
        assert!(matches!(receiver.next_with(Sends::Notice).await, Some(Next::Noticed)));
        drop(receiver);
        assert_eq!(receipt.await, Err(Dropped::Disconnected));
    }

    /// An operator's change to a disconnected session waits for the store to open the log and
    /// commit the change, and releases the session after.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_offline_change_waits_for_the_stores_jobs() {
        let storage = Arc::new(crate::store::deferring::DeferringStorage::deferring_all());
        let calls = storage.calls.clone();
        let registry = Arc::new(SessionRegistry::new(storage.clone()));
        let id = SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "A".into(), target_comp_id: "B".into() };
        let handle = registry.handle(id.clone());
        let numbers = handle.set_next_outgoing(9).await.unwrap();
        assert_eq!(numbers.next_outgoing, 9);
        assert_eq!(*calls.lock().unwrap(), [format!("open {id}"), "outgoing 8".into(), "commit".into()]);
        assert!(registry.sessions().is_empty(), "released");

        *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("connection refused")));
        assert!(matches!(handle.set_next_outgoing(10).await, Err(SequenceError::Storage(_))));
        assert!(registry.sessions().is_empty(), "released after a failed opening");
    }

    /// One task drives a registry's countdowns, however often it's asked for, and it ends once
    /// the registry has gone: it holds the wake while it runs.
    #[tokio::test]
    async fn the_cancel_task_is_spawned_once_and_ends_with_the_registry() {
        let registry = Arc::new(SessionRegistry::default());
        let wake = Arc::downgrade(&registry.cancels.wake);
        registry.spawn_cancel_task();
        registry.spawn_cancel_task();
        tokio::task::yield_now().await;
        assert_eq!(wake.strong_count(), 2, "the tracker's and one task's");
        drop(registry);
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        assert_eq!(wake.strong_count(), 0, "the task has ended");
    }
}
