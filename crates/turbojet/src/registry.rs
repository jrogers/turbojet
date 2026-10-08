//! Tracks live sessions and lets code outside the connection task send on them.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use chrono::{DateTime, TimeDelta, Utc};
use tokio::sync::{Notify, broadcast, mpsc, oneshot};
use tracing::{info, warn};

use crate::application::{Application, Disconnect};
use crate::cancel::{CancelTracker, CancelTrigger};
use crate::fields::ApplVerId;
use crate::message::Message;
use crate::peer::ConnectionInfo;
use crate::schedule::Clock;
use crate::store::{MemoryStorage, Opened, SessionId, SessionLog, SessionStorage};

/// A request to a session's connection task. `T` is what it sends: a FIX [`Message`], or for a
/// FIXP session an encoded SBE message.
#[derive(Debug)]
#[non_exhaustive]
pub enum Command<T = Message> {
    /// Send an application message, and say what became of it on the reply, if any; see
    /// [`SessionHandle::send`].
    Send(T, Option<ReceiptSender>),
    /// Log out, with this Text(58) if any; see [`SessionHandle::logout`].
    Logout(Option<String>),
    /// An operator change to sequence numbers, answered on the channel.
    Sequence(SequenceCommand, oneshot::Sender<Result<SequenceNumbers, SequenceError>>),
    /// FIXP: finish sending, ending the logical session; see
    /// [`FixpHandle::finish`](crate::fixp::FixpHandle). FIX sessions ignore it.
    Finish,
}

impl<T> Command<T> {
    /// Send `msg`, with no one waiting to hear what became of it.
    pub fn send(msg: T) -> Self {
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
#[non_exhaustive]
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
#[non_exhaustive]
pub enum SequenceCommand {
    /// Report them unchanged; see [`SessionHandle::sequence_numbers`].
    Get,
    /// See [`SessionHandle::set_next_incoming`].
    SetNextIncoming(u64),
    /// See [`SessionHandle::set_next_outgoing`].
    SetNextOutgoing(u64),
    /// See [`SessionHandle::reset_sequence_numbers`].
    Reset,
    /// See [`SessionHandle::request_resend`].
    RequestResend(u64),
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
#[non_exhaustive]
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
        // Only a logged-on session sends one, so a session that isn't connected can't.
        SequenceCommand::RequestResend(_) => {
            return Err(SequenceError::Invalid("a ResendRequest needs the session logged on".into()));
        }
        SequenceCommand::Reset => {
            log.reset()?;
            log.set_created_at(clock.now().into())?;
        }
    }
    let numbers = SequenceNumbers { next_incoming: log.next_incoming(), next_outgoing: log.next_outgoing() };
    // The log says what the command asked for: checked here, as the store recorded it.
    assert!(numbers.next_incoming >= 1, "sequence numbers start at 1");
    assert!(numbers.next_outgoing >= 1, "sequence numbers start at 1");
    match command {
        SequenceCommand::Get | SequenceCommand::RequestResend(_) => {}
        SequenceCommand::SetNextIncoming(seq) => assert_eq!(numbers.next_incoming, seq),
        SequenceCommand::SetNextOutgoing(seq) => assert_eq!(numbers.next_outgoing, seq),
        SequenceCommand::Reset => {
            assert_eq!(numbers.next_incoming, 1);
            assert_eq!(numbers.next_outgoing, 1);
        }
    }
    Ok(numbers)
}

/// Why a session couldn't be bound.
#[derive(Debug)]
pub(crate) enum AcquireError {
    // Boxed: a `SessionId` is large, and this is only built when acquiring fails.
    AlreadyConnected(Box<SessionId>),
    /// The counterparty already has this many sessions connected.
    TooManySessions(Box<SessionId>, usize),
    /// An operator paused the session: see [`SessionHandle::pause`].
    Paused(Box<SessionId>),
    Storage(Box<SessionId>, io::Error),
}

/// A session claimed in the registry for an operator change, released when dropped.
struct Claim<'a, T> {
    registry: &'a SessionRegistry<T>,
    id: &'a SessionId,
    commands: CommandSender<T>,
}

impl<T> Drop for Claim<'_, T> {
    fn drop(&mut self) {
        self.registry.release(self.id, &self.commands);
    }
}

impl fmt::Display for AcquireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyConnected(id) => write!(f, "{id} is already connected"),
            Self::TooManySessions(id, limit) => {
                write!(f, "{} already has {limit} sessions connected, the most it may", id.target_comp_id)
            }
            Self::Paused(id) => write!(f, "{id} is paused"),
            Self::Storage(id, e) => write!(f, "cannot open session store for {id}: {e}"),
        }
    }
}

/// Most logout and operator commands queued for a session at once. They have a queue of their own,
/// which the connection takes from first, so a queue full of sends never holds them up.
pub const CONTROL_QUEUE: usize = 64;
const _: () = assert!(CONTROL_QUEUE > 0);

/// A logout or operator command, as queued.
#[derive(Debug)]
enum Control {
    /// Log out, or finish sending, once the first `after` sends have been taken: those queued
    /// before it.
    Ending {
        ending: Ending,
        after: u64,
    },
    Sequence(SequenceCommand, oneshot::Sender<Result<SequenceNumbers, SequenceError>>),
}

/// A command that ends the session once the sends queued before it have gone.
#[derive(Debug)]
enum Ending {
    Logout(Option<String>),
    Finish,
}

impl Ending {
    fn command<T>(self) -> Command<T> {
        match self {
            Ending::Logout(text) => Command::Logout(text),
            Ending::Finish => Command::Finish,
        }
    }
}

/// The queues that carry [`SessionHandle`] commands to a session's connection task: application
/// sends, bounded by [`SessionConfig::send_queue`](crate::SessionConfig::send_queue), and
/// logout and operator commands, bounded by [`CONTROL_QUEUE`].
#[derive(Debug)]
pub struct CommandSender<T = Message> {
    sends: mpsc::Sender<(T, ReceiptSender)>,
    control: mpsc::Sender<Control>,
    /// Sends queued so far, for a Logout to wait behind.
    queued: Arc<AtomicU64>,
}

impl<T> Clone for CommandSender<T> {
    fn clone(&self) -> Self {
        Self { sends: self.sends.clone(), control: self.control.clone(), queued: self.queued.clone() }
    }
}

impl<T> CommandSender<T> {
    /// Whether `other` sends to the same connection.
    fn is_same(&self, other: &Self) -> bool {
        self.sends.same_channel(&other.sends)
    }
}

/// A connection task's end of a [`CommandSender`]. A driver takes [`control`](Self::try_control)
/// commands whenever it can, and [`sends`](Self::try_send) when the session can take them; a
/// Logout comes out of the control queue only once every send queued before it has been taken,
/// so a message sent before logging out still goes out first.
#[derive(Debug)]
pub struct CommandReceiver<T = Message> {
    sends: mpsc::Receiver<(T, ReceiptSender)>,
    control: mpsc::Receiver<Control>,
    /// Sends taken so far.
    taken: u64,
    /// A Logout (or Finish) waiting for the sends queued before it.
    held: Option<(Ending, u64)>,
    /// A send taken off its queue only to notice it (see [`Sends::Notice`]), first in line. It
    /// isn't counted in `taken` until it's handed out, so a Logout queued after it still waits.
    /// It has left the bounded queue, so while it's here one more send than `send_queue` waits.
    noticed: Option<(T, ReceiptSender)>,
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
pub(crate) enum Next<T = Message> {
    Command(Command<T>),
    /// A send is waiting, not taken; see [`Sends::Notice`].
    Noticed,
}

/// A session's command queues, sends holding up to `send_queue` messages.
pub fn command_queues<T>(send_queue: usize) -> (CommandSender<T>, CommandReceiver<T>) {
    assert!(send_queue > 0, "a send queue holds at least one message");
    let (sends, sends_rx) = mpsc::channel(send_queue);
    let (control, control_rx) = mpsc::channel(CONTROL_QUEUE);
    let sender = CommandSender { sends, control, queued: Arc::default() };
    (sender, CommandReceiver { sends: sends_rx, control: control_rx, taken: 0, held: None, noticed: None })
}

impl<T> CommandReceiver<T> {
    /// The next logout or operator command that's due, if any: operator commands at once, a
    /// Logout once the sends queued before it have been taken.
    pub fn try_control(&mut self) -> Option<Command<T>> {
        loop {
            if let Some((_, after)) = &self.held
                && self.taken >= *after
            {
                let (ending, _) = self.held.take().expect("checked");
                return Some(ending.command());
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
    pub async fn next(&mut self, sends: bool) -> Option<Command<T>> {
        match self.next_with(if sends { Sends::Take } else { Sends::Ignore }).await? {
            Next::Command(command) => Some(command),
            Next::Noticed => unreachable!("sends are only noticed when asked to be"),
        }
    }

    /// [`next`](Self::next), with application sends taken, noticed or ignored as `sends` says.
    /// Cancel-safe, as `next` is: a noticed send is kept, first in line.
    pub(crate) async fn next_with(&mut self, sends: Sends) -> Option<Next<T>> {
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
                        debug_assert!(self.noticed.is_none(), "one send is noticed at a time");
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
    fn take_noticed(&mut self) -> Option<Command<T>> {
        let (msg, receipt) = self.noticed.take()?;
        self.taken += 1;
        Some(Command::Send(msg, Some(receipt)))
    }

    /// A control command as it comes off its queue: a Logout not yet due is held (a second one
    /// while one is held adds nothing).
    fn take(&mut self, control: Control) -> Option<Command<T>> {
        match control {
            Control::Sequence(command, reply) => Some(Command::Sequence(command, reply)),
            Control::Ending { ending, after } if self.taken >= after && self.held.is_none() => Some(ending.command()),
            Control::Ending { ending, after } => {
                self.held.get_or_insert((ending, after));
                None
            }
        }
    }

    /// The next queued application message, if any.
    pub fn try_send(&mut self) -> Option<Command<T>> {
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
struct Entry<T> {
    commands: CommandSender<T>,
    /// How a Logon bound the session; `None` while an operator command holds a session that
    /// isn't connected.
    bound: Option<Binding>,
}

/// How a session's Logon bound it to its connection, as [`SessionStatus`] reports it.
#[derive(Debug, Clone)]
pub(crate) struct Binding {
    pub since: DateTime<Utc>,
    pub connection: ConnectionInfo,
    /// FIXT sessions: the application version in use on this connection.
    pub appl_ver_id: Option<ApplVerId>,
    /// FIX sessions: what the session updates as it runs.
    pub live: Option<Arc<LiveStatus>>,
}

/// What a FIX session updates as it runs, for [`SessionStatus::activity`]: plain atomic stores
/// on the paths that already change these, so reading it never waits on the session.
#[derive(Debug)]
pub(crate) struct LiveStatus {
    /// When the session bound, as an `Instant`, which the times below count from.
    origin: Instant,
    next_incoming: AtomicU64,
    next_outgoing: AtomicU64,
    /// A [`SessionState`] as its index.
    state: AtomicU8,
    resending: AtomicBool,
    /// Nanoseconds from `origin`.
    last_received: AtomicU64,
    last_sent: AtomicU64,
}

impl LiveStatus {
    pub(crate) fn new(origin: Instant) -> Self {
        Self {
            origin,
            next_incoming: AtomicU64::new(0),
            next_outgoing: AtomicU64::new(0),
            state: AtomicU8::new(SessionState::LoggingOn as u8),
            resending: AtomicBool::new(false),
            last_received: AtomicU64::new(0),
            last_sent: AtomicU64::new(0),
        }
    }

    pub(crate) fn set_next_incoming(&self, seq: u64) {
        self.next_incoming.store(seq, Ordering::Relaxed);
    }

    pub(crate) fn set_next_outgoing(&self, seq: u64) {
        self.next_outgoing.store(seq, Ordering::Relaxed);
    }

    pub(crate) fn set_state(&self, state: SessionState) {
        self.state.store(state as u8, Ordering::Relaxed);
    }

    /// Whether it changed.
    pub(crate) fn set_resending(&self, resending: bool) -> bool {
        self.resending.swap(resending, Ordering::Relaxed) != resending
    }

    pub(crate) fn received(&self, now: Instant) {
        self.last_received.store(self.nanos(now), Ordering::Relaxed);
    }

    pub(crate) fn sent(&self, now: Instant) {
        self.last_sent.store(self.nanos(now), Ordering::Relaxed);
    }

    fn nanos(&self, now: Instant) -> u64 {
        // Saturating: a session would have to run for centuries to pass u64 nanoseconds.
        u64::try_from(now.saturating_duration_since(self.origin).as_nanos()).unwrap_or(u64::MAX)
    }

    /// The values now, the times counted from `since`, the wall time at `origin`.
    fn activity(&self, since: DateTime<Utc>) -> Activity {
        let at = |nanos: &AtomicU64| {
            since + TimeDelta::nanoseconds(i64::try_from(nanos.load(Ordering::Relaxed)).unwrap_or(i64::MAX))
        };
        let state = match self.state.load(Ordering::Relaxed) {
            0 => SessionState::LoggingOn,
            1 => SessionState::LoggedOn,
            _ => SessionState::LoggingOut,
        };
        Activity {
            state,
            resending: self.resending.load(Ordering::Relaxed),
            next_incoming: self.next_incoming.load(Ordering::Relaxed),
            next_outgoing: self.next_outgoing.load(Ordering::Relaxed),
            last_received: at(&self.last_received),
            last_sent: at(&self.last_sent),
        }
    }
}

/// What happened to a session, for an operator's live view: see [`SessionRegistry::subscribe`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SessionEvent {
    /// The session it happened to.
    pub id: SessionId,
    /// What happened.
    pub kind: SessionEventKind,
}

/// What happened to a session: see [`SessionEvent`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionEventKind {
    /// A Logon bound it to a connection: it's in [`SessionRegistry::statuses`] from now on.
    Connected,
    /// FIX sessions: Logons exchanged.
    LoggedOn,
    /// FIX sessions: a Logout was sent or received.
    LoggingOut,
    /// FIX sessions: a gap in what it received is being recovered, with a ResendRequest.
    ResendStarted,
    /// FIX sessions: the gap is filled.
    ResendFinished,
    /// Its connection ended: it's out of [`SessionRegistry::statuses`].
    Disconnected,
    /// An operator paused it: see [`SessionHandle::pause`].
    Paused,
    /// An operator resumed it.
    Resumed,
    /// A Logon for it was refused, for this reason: paused, already connected, too many sessions
    /// for its counterparty, or its store failing to open. Refusals the application or the
    /// session's checks make before then are logged, not sent here.
    Refused(String),
}

/// Where a connected FIX session is in its life, as [`Activity`] reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionState {
    /// Bound by a Logon, the exchange of Logons not yet done.
    LoggingOn = 0,
    /// Logons exchanged: messages flow.
    LoggedOn = 1,
    /// A Logout has been sent or received, and the session is ending.
    LoggingOut = 2,
}

/// A connected FIX session's progress: see [`SessionStatus::activity`]. Read as the session
/// left it, each value on its own, so they may be a message apart.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Activity {
    /// Where it is in its life.
    pub state: SessionState,
    /// Whether it's recovering a gap in what it received: a ResendRequest of its own is open.
    pub resending: bool,
    /// The MsgSeqNum expected next from the counterparty.
    pub next_incoming: u64,
    /// The MsgSeqNum of the next message it sends.
    pub next_outgoing: u64,
    /// When it last received a message (or bound, if it hasn't since), by the session's clock.
    pub last_received: DateTime<Utc>,
    /// When it last sent one (or bound, if it hasn't since).
    pub last_sent: DateTime<Utc>,
}

/// A connected session, for an operator: see [`SessionRegistry::statuses`] and
/// [`SessionHandle::status`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SessionStatus {
    /// The session.
    pub id: SessionId,
    /// When its Logon bound it to the connection: an acceptor's when the counterparty's Logon
    /// arrived, an initiator's when its own went out. The session's clock.
    pub since: DateTime<Utc>,
    /// The connection: the remote address and the peer's verified certificates.
    pub connection: ConnectionInfo,
    /// FIXT sessions: the default application version on this connection.
    pub appl_ver_id: Option<ApplVerId>,
    /// Whether an operator has paused it, to be logged out (see [`SessionHandle::pause`]).
    pub paused: bool,
    /// FIX sessions: its state, sequence numbers and last messages. Not yet tracked for FIXP
    /// sessions, which have `None`.
    pub activity: Option<Activity>,
}

fn status<T>(id: &SessionId, entry: &Entry<T>, paused: bool) -> Option<SessionStatus> {
    let bound = entry.bound.as_ref()?;
    Some(SessionStatus {
        id: id.clone(),
        since: bound.since,
        connection: bound.connection.clone(),
        appl_ver_id: bound.appl_ver_id,
        paused,
        activity: bound.live.as_ref().map(|live| live.activity(bound.since)),
    })
}

/// Opens session logs from storage, ensures each session runs on at most one connection at a
/// time, routes [`SessionHandle`] commands to that connection, and keeps the
/// [cancel-on-disconnect](crate::SessionConfig::cancel_on_disconnect) countdowns of its sessions.
/// `T` is what its sessions send (see [`Command`]); cancel on disconnect is for FIX sessions.
pub struct SessionRegistry<T = Message> {
    storage: Arc<dyn SessionStorage>,
    /// Connected sessions only: an entry is removed when its connection releases it.
    sessions: Mutex<HashMap<SessionId, Entry<T>>>,
    /// For creation times recorded by operator resets of disconnected sessions.
    clock: Clock,
    /// Locked on its own. Nothing takes its lock with `sessions` held; a cancel callback may
    /// take `sessions` under it (sending through a `SessionHandle`), which is the only order.
    cancels: CancelTracker,
    /// Whether the task driving `cancels` is running: one per registry, however many acceptors
    /// and initiators share it, so the wake has one waiter. Shared with the task, which clears it
    /// as it goes (see `Driving`).
    cancel_task: Arc<AtomicBool>,
    /// Sessions an operator paused: their Logons are refused until resumed. Locked only with
    /// `sessions` held, so a Logon and a pause can't cross.
    paused: Mutex<HashSet<SessionId>>,
    /// Woken when a session is resumed, for initiators waiting to reconnect.
    resumed: Notify,
    /// What [`subscribe`](Self::subscribe) hands out.
    events: broadcast::Sender<SessionEvent>,
}

/// Most session events held for a subscriber that hasn't read them, past which it misses the
/// oldest (and is told how many): enough for every session of a large acceptor to connect at
/// once, a few events each, without a reader that's a moment behind losing any.
pub const EVENT_QUEUE: usize = 4096;

/// Takes no lock, so it's safe to format anywhere.
impl<T> fmt::Debug for SessionRegistry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionRegistry").finish_non_exhaustive()
    }
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new(Arc::new(MemoryStorage::new()))
    }
}

impl SessionRegistry {
    /// A registry of FIX sessions opening session logs from `storage`. The [`Default`] registry
    /// keeps them in memory.
    pub fn new(storage: Arc<dyn SessionStorage>) -> Self {
        Self::with_storage(storage)
    }
}

impl<T> SessionRegistry<T> {
    /// A registry of sessions sending `T`, opening session logs from `storage`: as
    /// [`SessionRegistry::new`], for any kind of session.
    pub fn with_storage(storage: Arc<dyn SessionStorage>) -> Self {
        Self {
            storage,
            sessions: Mutex::default(),
            clock: Clock::system(),
            cancels: CancelTracker::new(),
            cancel_task: Arc::default(),
            paused: Mutex::default(),
            resumed: Notify::new(),
            events: broadcast::channel(EVENT_QUEUE).0,
        }
    }

    /// Uses `clock` for creation times recorded by operator resets (normally the sessions'
    /// [`SessionConfig::clock`](crate::SessionConfig::clock)).
    #[must_use]
    pub fn with_clock(mut self, clock: Clock) -> Self {
        self.clock = clock;
        self
    }

    /// A handle for `id`. It can be created before the session connects and stays valid across
    /// reconnects; sends succeed while the session is connected.
    pub fn handle(self: &Arc<Self>, id: SessionId) -> SessionHandle<T> {
        SessionHandle { id, registry: self.clone() }
    }

    /// Sessions currently bound to a connection.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.lock().keys().cloned().collect()
    }

    /// What happens to the registry's sessions from now on, for a live view: connections and
    /// disconnections, logons and logouts, resends, pauses and refused Logons. Pair it with
    /// [`statuses`](Self::statuses), read after subscribing, for where each session stands.
    /// Events are published as they happen, on the session's task, and never wait for a
    /// subscriber: one more than [`EVENT_QUEUE`] events behind misses the oldest, and its next
    /// read says how many ([`broadcast::error::RecvError::Lagged`]), the cue to read `statuses`
    /// again.
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.events.subscribe()
    }

    /// Tells subscribers, if there are any.
    pub(crate) fn publish(&self, id: &SessionId, kind: SessionEventKind) {
        if self.events.receiver_count() > 0 {
            // Fails only with no subscribers, who'd have nothing to miss.
            let _ = self.events.send(SessionEvent { id: id.clone(), kind });
        }
    }

    /// The sessions an operator has paused (see [`SessionHandle::pause`]), connected or not.
    pub fn paused(&self) -> Vec<SessionId> {
        self.lock_paused().iter().cloned().collect()
    }

    /// The sessions connected now, each with when and how it connected: for an operator's view.
    /// Sessions an operator command holds while not connected aren't included.
    pub fn statuses(&self) -> Vec<SessionStatus> {
        let sessions = self.lock();
        let paused = self.lock_paused();
        sessions.iter().filter_map(|(id, entry)| status(id, entry, paused.contains(id))).collect()
    }

    /// Binds `id` to a connection and opens its log, at once or by a job (see
    /// [`SessionStorage::begin_open`]); if the job fails, the caller releases `id`. Fails if it is
    /// already bound elsewhere, if its counterparty (BeginString and CompIDs) already has
    /// `per_counterparty` sessions bound, or if storage cannot be opened.
    pub(crate) fn acquire(
        &self,
        id: &SessionId,
        commands: CommandSender<T>,
        bound: Option<Binding>,
        per_counterparty: usize,
    ) -> Result<Opened, AcquireError> {
        // A Logon's outcome, not an operator command's on a disconnected session.
        let logon = bound.is_some();
        let acquired = self.claim(id, commands, bound, per_counterparty);
        if logon {
            match &acquired {
                Ok(_) => self.publish(id, SessionEventKind::Connected),
                Err(e) => self.publish(id, SessionEventKind::Refused(e.to_string())),
            }
        }
        acquired
    }

    /// [`acquire`](Self::acquire), but for telling subscribers.
    fn claim(
        &self,
        id: &SessionId,
        commands: CommandSender<T>,
        bound: Option<Binding>,
        per_counterparty: usize,
    ) -> Result<Opened, AcquireError> {
        {
            let mut sessions = self.lock();
            if sessions.contains_key(id) {
                return Err(AcquireError::AlreadyConnected(Box::new(id.clone())));
            }
            // A Logon, not an operator command on a disconnected session.
            if bound.is_some() && self.lock_paused().contains(id) {
                return Err(AcquireError::Paused(Box::new(id.clone())));
            }
            // A scan, but only at logon, over sessions the acceptor's connection limit bounds.
            let same_counterparty = |other: &&SessionId| {
                other.target_comp_id == id.target_comp_id
                    && other.sender_comp_id == id.sender_comp_id
                    && other.begin_string == id.begin_string
            };
            if sessions.keys().filter(same_counterparty).count() >= per_counterparty {
                return Err(AcquireError::TooManySessions(Box::new(id.clone()), per_counterparty));
            }
            sessions.insert(id.clone(), Entry { commands: commands.clone(), bound });
        }
        self.storage.begin_open(id).map_err(|e| {
            self.release(id, &commands);
            AcquireError::Storage(Box::new(id.clone()), e)
        })
    }

    /// Applies an operator command to a session that isn't connected, holding its registration
    /// meanwhile so it can't log on half-way through.
    async fn apply_offline(&self, id: &SessionId, command: SequenceCommand) -> Result<SequenceNumbers, SequenceError> {
        // No receiver: a send through a handle during the change fails as not connected.
        let (placeholder, _) = command_queues(1);
        let opened = self.acquire(id, placeholder.clone(), None, usize::MAX).map_err(|e| match e {
            AcquireError::AlreadyConnected(_) => SequenceError::Connected,
            AcquireError::TooManySessions(..) => unreachable!("no limit is given"),
            AcquireError::Paused(_) => unreachable!("an operator command isn't refused for a pause"),
            AcquireError::Storage(_, e) => SequenceError::Storage(e),
        })?;
        // Declared before the log, so dropped after it: the log is closed (its lock released)
        // before the session can be claimed again, even if the caller gives up on this future.
        let _claim = Claim { registry: self, id, commands: placeholder };
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

    /// Unbinds `id` from the connection whose queues are `commands`. Only that connection's
    /// registration goes: one that released twice must not unbind the session from the next
    /// connection to claim it. Not `assert!`, though it's once per connection: the session
    /// releases from its `Drop`, where a panic while unwinding would abort.
    pub(crate) fn release(&self, id: &SessionId, commands: &CommandSender<T>) {
        let (ours, was_logon) = {
            let mut sessions = self.lock();
            let ours = sessions.get(id).is_some_and(|entry| entry.commands.is_same(commands));
            let removed = if ours { sessions.remove(id) } else { None };
            (ours, removed.is_some_and(|entry| entry.bound.is_some()))
        };
        // Outside the lock, which a panic would poison; and not while unwinding already.
        debug_assert!(ours || std::thread::panicking(), "{id} is released only by the connection that holds it");
        if was_logon {
            self.publish(id, SessionEventKind::Disconnected);
        }
    }

    fn sender(&self, id: &SessionId) -> Option<CommandSender<T>> {
        self.lock().get(id).map(|e| e.commands.clone())
    }

    fn appl_ver_id(&self, id: &SessionId) -> Option<ApplVerId> {
        self.lock().get(id).and_then(|e| e.bound.as_ref()?.appl_ver_id)
    }

    fn status(&self, id: &SessionId) -> Option<SessionStatus> {
        let sessions = self.lock();
        let paused = self.lock_paused().contains(id);
        sessions.get(id).and_then(|entry| status(id, entry, paused))
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<SessionId, Entry<T>>> {
        self.sessions.lock().expect("session registry lock poisoned")
    }

    /// Taken only with `sessions` held, or alone.
    fn lock_paused(&self) -> MutexGuard<'_, HashSet<SessionId>> {
        self.paused.lock().expect("paused sessions lock poisoned")
    }

    /// Pauses `id`, under the `sessions` lock so a Logon binding now sees it. Whether it was
    /// connected.
    fn pause(&self, id: &SessionId) -> bool {
        let connected = {
            let sessions = self.lock();
            self.lock_paused().insert(id.clone());
            sessions.contains_key(id)
        };
        self.publish(id, SessionEventKind::Paused);
        connected
    }

    fn resume(&self, id: &SessionId) {
        {
            let _sessions = self.lock();
            self.lock_paused().remove(id);
        }
        self.resumed.notify_waiters();
        self.publish(id, SessionEventKind::Resumed);
    }

    /// Waits until `id` isn't paused.
    pub(crate) async fn wait_resumed(&self, id: &SessionId) {
        loop {
            let resumed = self.resumed.notified();
            tokio::pin!(resumed);
            // Registered before checking, so a resume between the check and the wait is seen.
            resumed.as_mut().enable();
            if !self.is_paused(id) {
                return;
            }
            resumed.await;
        }
    }

    /// Whether an operator has paused `id`: see [`SessionHandle::pause`].
    pub fn is_paused(&self, id: &SessionId) -> bool {
        self.lock_paused().contains(id)
    }
}

impl SessionRegistry<Message> {
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
    /// order. While they run, logons and endings of every session in the registry wait. Calling it
    /// early is harmless: it acts only on what is due. Calling it from inside
    /// `on_cancel_on_disconnect` deadlocks; from other callbacks it's fine.
    pub fn run_due_cancels(self: &Arc<Self>, now: Instant) {
        self.cancels.run_due(|id| self.handle(id.clone()), now);
    }

    /// Fires every cancel-on-disconnect countdown under way, due or not, as a shutdown does.
    /// Calling it from inside `on_cancel_on_disconnect` deadlocks; from other callbacks it's fine.
    pub fn run_all_cancels(self: &Arc<Self>) {
        self.cancels.run_all(|id| self.handle(id.clone()));
    }

    /// Fires the countdowns under way of the sessions `which` picks, due or not: those of an
    /// acceptor or initiator shutting down.
    pub(crate) fn run_cancels_now(self: &Arc<Self>, which: impl Fn(&SessionId) -> bool) {
        self.cancels.run_now(|id| self.handle(id.clone()), which);
    }

    /// Spawns the task that fires the countdowns as they fall due, unless it's running already.
    /// The acceptor and initiator call it as each connection starts, so a countdown, which only a
    /// session ending can start, has the task to fire it while the runtime of the connection that
    /// spawned it runs. Needs a tokio runtime.
    ///
    /// The countdowns run on that runtime. If it shuts down, or the task ends some other way (a
    /// panic outside the guarded callbacks), the task is gone; the next connection, on whatever
    /// runtime, spawns another, which fires anything that fell due meanwhile. Until then nothing
    /// does.
    ///
    /// The task holds the registry only while it fires what's due, so it doesn't keep it alive.
    /// It ends once the registry has gone, woken by the registry's `Drop`. It runs application
    /// callbacks, which must not block, on the runtime like any other task, and reads tokio's
    /// clock, as the connections do, so that paused time in tests moves the deadlines too.
    pub(crate) fn spawn_cancel_task(self: &Arc<Self>) {
        // Relaxed: the flag only picks which caller spawns; it guards no data.
        if self.cancel_task.swap(true, Ordering::Relaxed) {
            return;
        }
        // Made before spawning and moved into the task: a runtime shutting down drops a future
        // spawned on it without polling it, and the flag must still be cleared then.
        let driving = Driving(self.cancel_task.clone());
        let registry = Arc::downgrade(self);
        let wake: Arc<Notify> = self.cancels.wake.clone();
        tokio::spawn(async move {
            let _driving = driving;
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
}

/// Marks the task driving a registry's countdowns as running, until it's dropped: when the task
/// ends, or its runtime drops it, so the next connection spawns another.
struct Driving(Arc<AtomicBool>);

impl Drop for Driving {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Relaxed);
    }
}

impl<T> Drop for SessionRegistry<T> {
    fn drop(&mut self) {
        // Each bound session holds the registry, so none is left once it goes: a registration
        // left here was never released.
        if !std::thread::panicking() {
            let sessions = self.sessions.get_mut().map(|sessions| sessions.len()).unwrap_or_default();
            debug_assert_eq!(sessions, 0, "sessions left bound as the registry goes");
        }
        // Wakes the task driving the countdowns, if any, to find the registry gone and end: a
        // permit if it's between waits, and waking it if it's waiting.
        self.cancels.wake.notify_one();
        self.cancels.wake.notify_waiters();
        // Countdowns still pending go with it: the acceptor or initiator that would fire them, and
        // every session that could log back on, have gone too.
        let lost = self.cancels.count();
        if lost > 0 {
            crate::telemetry::cancels_removed(lost);
            warn!(lost, "session registry dropped with cancel-on-disconnect countdowns pending; they won't fire");
        }
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
pub struct SessionHandle<T = Message> {
    id: SessionId,
    registry: Arc<SessionRegistry<T>>,
}

impl<T> Clone for SessionHandle<T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), registry: self.registry.clone() }
    }
}

/// The session's ID, as [`SessionId`] displays it.
impl<T> fmt::Display for SessionHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, f)
    }
}

impl<T> SessionHandle<T> {
    /// A handle to `id` on a registry of its own, which no connection ever claims: its sends
    /// report [`SendError::NotConnected`]. For unit-testing an [`Application`],
    /// whose callbacks are each given a handle.
    pub fn disconnected(id: SessionId) -> Self {
        Arc::new(SessionRegistry::with_storage(Arc::new(crate::MemoryStorage::new()))).handle(id)
    }

    /// The session this handle sends on.
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    /// Whether a connection has claimed the session (see [`SessionHandle`]) and not yet ended.
    /// Commands fail as not connected otherwise.
    pub fn is_connected(&self) -> bool {
        self.registry.sender(&self.id).is_some()
    }

    /// The session's connection, and when it logged on, if it's connected: see [`SessionStatus`].
    pub fn status(&self) -> Option<SessionStatus> {
        self.registry.status(&self.id)
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
    ///
    /// # Errors
    ///
    /// [`SendError::NotConnected`] if no connection has the session, or [`SendError::Full`] if its
    /// send queue is full; either way the message comes back in it.
    pub fn send(&self, msg: impl Into<T>) -> Result<Receipt, SendError<T>> {
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
    ///
    /// # Errors
    ///
    /// [`SendError::NotConnected`] if no connection has the session, or it ends while waiting; the
    /// message comes back in it.
    pub async fn send_when_ready(&self, msg: impl Into<T>) -> Result<Receipt, SendError<T>> {
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
    ///
    /// # Errors
    ///
    /// [`CommandError::NotConnected`] if no connection has the session, or [`CommandError::Full`]
    /// if its control queue is full.
    pub fn logout(&self, text: Option<&str>) -> Result<(), CommandError> {
        self.end(Ending::Logout(text.map(String::from)))
    }

    /// Pauses the session: logs it out if it's connected (with `text`, once the messages queued
    /// have gone), and refuses its Logons until [`resume`](Self::resume): an acceptor closes a
    /// counterparty's connection at its Logon, and an initiator waits to reconnect. Operator
    /// changes to its stored sequence numbers still work. Pauses are kept in memory, not stored:
    /// a restart forgets them.
    pub fn pause(&self, text: Option<&str>) {
        if self.registry.pause(&self.id) {
            // If the control queue is full, a logout is already queued.
            let _ = self.logout(text);
        }
        info!(session = %self.id, "paused by operator");
    }

    /// Lets a paused session log on again; an initiator reconnects at once.
    pub fn resume(&self) {
        self.registry.resume(&self.id);
        info!(session = %self.id, "resumed by operator");
    }

    /// Whether the session is paused: see [`pause`](Self::pause).
    pub fn is_paused(&self) -> bool {
        self.registry.is_paused(&self.id)
    }

    /// Queues `ending` behind the sends already queued.
    fn end(&self, ending: Ending) -> Result<(), CommandError> {
        let sender = self.registry.sender(&self.id).ok_or(CommandError::NotConnected)?;
        let after = sender.queued.load(Ordering::Acquire);
        sender.control.try_send(Control::Ending { ending, after }).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => CommandError::Full,
            mpsc::error::TrySendError::Closed(_) => CommandError::NotConnected,
        })
    }

    /// FIXP: finishes sending once the messages already queued have gone, ending the logical
    /// session; see [`FixpHandle::finish`](crate::fixp::FixpHandle).
    pub(crate) fn finish_sending(&self) -> Result<(), CommandError> {
        self.end(Ending::Finish)
    }

    // ---- Operator control of sequence numbers ----
    //
    // These work whether or not the session is connected. A connected session applies the change
    // on its connection task, keeping its state (and, where needed, the counterparty) in step; a
    // disconnected one has its stored state changed directly.

    /// The session's next incoming and outgoing sequence numbers.
    ///
    /// # Errors
    ///
    /// [`SequenceError::Storage`] if the store fails.
    pub async fn sequence_numbers(&self) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::Get).await
    }

    /// Sets the next MsgSeqNum expected from the counterparty. It can go down as well as up:
    /// lowering it means messages already processed may be accepted again.
    ///
    /// # Errors
    ///
    /// [`SequenceError::Invalid`] for 0, or [`SequenceError::Storage`] if the store fails.
    pub async fn set_next_incoming(&self, seq: u64) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::SetNextIncoming(seq)).await
    }

    /// Asks the counterparty to send again everything from MsgSeqNum `from` on: for an
    /// application that lost messages it had processed. The session expects `from` next and
    /// sends a ResendRequest, so the counterparty's resends (PossDupFlag(43)=Y, or gap fills for
    /// what it doesn't resend) are delivered as if they were new, and any new messages after them.
    /// The reply gives the numbers once it's sent.
    ///
    /// # Errors
    ///
    /// [`SequenceError::Invalid`] if the session isn't logged on, a resend is already under way,
    /// or `from` isn't below the next MsgSeqNum expected; [`SequenceError::Storage`] if the store
    /// fails. FIXP sessions don't take it.
    pub async fn request_resend(&self, from: u64) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::RequestResend(from)).await
    }

    /// Moves the next outgoing MsgSeqNum forward (it can't go back; see
    /// [`reset_sequence_numbers`](Self::reset_sequence_numbers)). If the session is logged on,
    /// the counterparty is told to expect `seq` next with a SequenceReset in gap-fill mode, which a
    /// counterparty still filling a gap applies only once it's filled, so nothing sent before is
    /// lost; otherwise it finds out from the next message, and asks for the skipped numbers.
    ///
    /// # Errors
    ///
    /// [`SequenceError::Invalid`] for 0 or a number below the next one, or
    /// [`SequenceError::Storage`] if the store fails.
    pub async fn set_next_outgoing(&self, seq: u64) -> Result<SequenceNumbers, SequenceError> {
        self.sequence(SequenceCommand::SetNextOutgoing(seq)).await
    }

    /// Resets both sequence numbers to 1 and clears the resend store. Only while the session is
    /// disconnected ([`SequenceError::Connected`] otherwise); for a reset with the counterparty,
    /// log on with ResetSeqNumFlag instead.
    ///
    /// # Errors
    ///
    /// [`SequenceError::Connected`] if a connection has the session, or [`SequenceError::Storage`]
    /// if the store fails.
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

impl<T> fmt::Debug for SessionHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionHandle").field("id", &self.id).finish()
    }
}

/// Why [`SessionHandle::send`] didn't queue a message, with the message, to retry or keep.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SendError<T = Message> {
    /// No connection has the session (see [`SessionHandle`]).
    NotConnected(T),
    /// The session's send queue is full: the counterparty is reading more slowly than the
    /// application sends.
    Full(T),
}

impl<T> SendError<T> {
    /// The message that wasn't queued.
    pub fn into_message(self) -> T {
        match self {
            Self::NotConnected(msg) | Self::Full(msg) => msg,
        }
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected(_) => f.write_str("session is not connected"),
            Self::Full(_) => f.write_str("session's send queue is full"),
        }
    }
}

impl<T: fmt::Debug> std::error::Error for SendError<T> {}

/// Why [`SessionHandle::logout`] didn't queue the logout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
        sender.control.try_send(Control::Ending { ending: Ending::Logout(None), after }).unwrap();
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
        let id = SessionId::new("FIX.4.4", "A", "B");
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
        assert_eq!(wake.strong_count(), 2, "the tracker's and one task's");
        drop(registry);
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        assert_eq!(wake.strong_count(), 0, "the task has ended");
    }

    /// Counts the cancels it's told of.
    #[derive(Default)]
    struct Counter(AtomicU64);

    impl Application for Counter {
        fn on_cancel_on_disconnect(&self, _session: &SessionHandle, _ended: Disconnect) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread().enable_time().start_paused(true).build().unwrap()
    }

    fn start(registry: &SessionRegistry, target: &str, app: &Arc<Counter>, grace: Duration) {
        let id = SessionId::new("FIX.4.2", "US", target);
        let deadline = tokio::time::Instant::now().into_std() + grace;
        registry.start_cancel(id, Disconnect::ConnectionLost, CancelTrigger::Disconnect, deadline, app.clone());
    }

    /// The runtime the task ran on shuts down with a countdown pending; the next connection, on
    /// another runtime, spawns another task, which fires it, and those started since, when due.
    #[test]
    fn the_cancel_task_is_spawned_again_after_its_runtime_goes() {
        let registry = Arc::new(SessionRegistry::default());
        let app = Arc::<Counter>::default();
        let first = runtime();
        first.block_on(async {
            registry.spawn_cancel_task();
            start(&registry, "A", &app, Duration::from_secs(5));
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        drop(first);
        assert_eq!(app.0.load(Ordering::Relaxed), 0);

        runtime().block_on(async {
            registry.spawn_cancel_task();
            start(&registry, "B", &app, Duration::from_secs(5));
            tokio::time::sleep(Duration::from_secs(10)).await;
        });
        assert_eq!(app.0.load(Ordering::Relaxed), 2, "both fired");
        assert_eq!(registry.next_cancel_deadline(), None);
    }
}
