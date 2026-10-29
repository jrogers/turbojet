//! The interface between the engine and the code using it.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Weak};

use tracing::error;

use crate::fields::{BusinessRejectReason, CompactString, MsgType, SessionRejectReason};
use crate::message::{DataFields, FieldError, FixMessage, Message, tags};
use crate::peer::ConnectionInfo;
use crate::registry::{CommandSender, SessionHandle, SessionRegistry};
use crate::session::ApplVersion;
use crate::store::SessionId;
use crate::sync::{AtomicBool, Mutex, Ordering};

/// Callbacks from the engine into the application. The engine handles the session layer
/// (logon, sequencing, heartbeats, resends, logout); the application sees only what it needs.
///
/// Callbacks run synchronously on the session's connection task and must not block. To do slow
/// work, hand it off (e.g. to a spawned task) and send the result later with a [`SessionHandle`].
///
/// Every callback is given the session's [`SessionHandle`] (`on_message` through its
/// [`Context`]): its [`id`](SessionHandle::id), and a way to act on the session, to log it out
/// after a Reject, say. Clone it to keep it.
///
/// A panicking callback doesn't end the connection. A panic in `on_message` is answered with a
/// BusinessMessageReject (ApplicationNotAvailable) and any replies it queued are dropped; in
/// `verify_logon` it refuses the logon; in `to_admin` it disconnects rather than send a message it
/// may have half-modified; in `should_resend` the message is resent; in `on_admin_message`, `on_logon`, `on_logout` or `on_cancel_on_disconnect`
/// it is only logged. Each panic is logged and counted (`turbojet_application_panics_total`). With
/// `panic = "abort"` the process stops.
///
/// ```
/// use turbojet::{Application, ConnectionInfo, Context, Disconnect, Message, MessageReject, MsgType, SessionHandle};
///
/// struct Desk;
///
/// impl Application for Desk {
///     fn verify_logon(&self, session: &SessionHandle, _logon: &Message, _from: &ConnectionInfo) -> Result<(), String> {
///         let known = ["CLIENT1", "CLIENT2"].contains(&session.id().target_comp_id.as_str());
///         if known { Ok(()) } else { Err("unknown CompID".into()) }
///     }
///
///     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
///         match msg.msg_type() {
///             MsgType::NewOrderSingle => Ok(()), // parse it, act on it, reply with ctx.send
///             _ => Err(MessageReject::unsupported_message_type()),
///         }
///     }
///
///     fn on_logout(&self, session: &SessionHandle, ended: Disconnect) {
///         println!("{} ended: {ended:?}", session.id());
///     }
/// }
/// ```
pub trait Application: Send + Sync + 'static {
    /// Inspects an inbound Logon before it is accepted: the counterparty's request on an
    /// acceptor, or the reply on an initiator. `connection` describes the transport it arrived
    /// on, including the peer's TLS certificate if it presented one (on an acceptor, the client's;
    /// on an initiator, the server's). Return `Err` with a reason to refuse the logon; the
    /// connection is then dropped.
    ///
    /// Credentials are in the typed Logon, read here in its borrowed form:
    ///
    /// ```
    /// # use turbojet::{admin::LogonRef, Message};
    /// fn check(logon: &Message) -> Result<(), String> {
    ///     let logon: LogonRef = logon.parse().map_err(|e| e.to_string())?;
    ///     match (logon.username, logon.password.map(|p| p.expose())) {
    ///         (Some("trader"), Some("secret")) => Ok(()),
    ///         _ => Err("unknown user or wrong password".into()),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Return `Err` with the reason to refuse the logon; the connection is then dropped.
    fn verify_logon(
        &self,
        _session: &SessionHandle,
        _logon: &Message,
        _connection: &ConnectionInfo,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Called before an outbound session-level message is sent, e.g. to add credentials to
    /// Logon. Header fields are added afterwards and cannot be changed here. On FIXT.1.1 sessions,
    /// leave DefaultApplVerID(1137) to the engine and configure it with
    /// [`SessionConfig::with_appl_ver_id`](crate::SessionConfig::with_appl_ver_id).
    fn to_admin(&self, _session: &SessionHandle, _msg: &mut Message) {}

    /// An inbound session-level message other than a Logon (which goes to `verify_logon`): a
    /// Heartbeat, TestRequest, ResendRequest, Reject, SequenceReset or Logout. Called once for
    /// each that passes the session's checks on its header and sequence number, before the session
    /// acts on it; one whose body then turns out to be invalid is rejected as usual. The session
    /// handles every one of them itself; this is so the application can see, for instance, that
    /// the counterparty rejected one of its messages (a Reject's RefSeqNum(45) and Text(58)), or
    /// why it logged out.
    fn on_admin_message(&self, _session: &SessionHandle, _msg: &Message) {}

    /// Whether to resend `msg`, an application message we sent, which the counterparty has asked
    /// for again: return `false` to gap-fill it instead, for one that's stale by now (an order
    /// the market has moved past, say). `msg` is as stored, with its original header. Called for
    /// each stored message a ResendRequest covers, in order; a run of messages declined, or not
    /// stored, is covered by one SequenceReset within each step of the resend (256 sequence
    /// numbers). The default resends every one.
    fn should_resend(&self, _session: &SessionHandle, _msg: &Message) -> bool {
        true
    }

    /// The session is logged on. Clone the handle to send messages outside of callbacks.
    fn on_logon(&self, _session: &SessionHandle) {}

    /// A logged-on session has ended: it logged out, or the connection was lost. `ended` says
    /// which, so the application can tell a Logout, which both sides agreed to, from a session
    /// that stopped without one. Called once per logon, and never for a session that didn't log
    /// on.
    fn on_logout(&self, _session: &SessionHandle, _ended: Disconnect) {}

    /// A session with [cancel on disconnect](crate::SessionConfig::cancel_on_disconnect) ended
    /// with `ended`, and the counterparty didn't log back on within the grace period: cancel its
    /// resting orders. Called once each time the session is gone, with how it first ended: endings
    /// before it logs back on (failed reconnects, say) don't add calls. Called after `on_logout`,
    /// from the task that keeps the countdowns, or a connection's (with no grace period, a session
    /// ending fires every countdown due). A logon of the same session waits for this to return,
    /// so it never comes after that logon's `on_logon`; so do logons and endings of every other
    /// session in the registry, which is one more reason that, like every callback, it must not
    /// block. Countdowns pending when the acceptor or initiator shuts down fire then; a
    /// process that stops without shutting down loses them, so after a restart, check orders you
    /// kept.
    fn on_cancel_on_disconnect(&self, _session: &SessionHandle, _ended: Disconnect) {}

    /// An application-level message arrived in sequence. Replies sent through `ctx` go out in
    /// order immediately after this returns. Returning `Err` sends the corresponding reject.
    ///
    /// Delivery is at least once: the message counts as received only after this returns and its
    /// replies are stored, so if the process stops first, the counterparty resends it. Such a
    /// redelivery has [`Context::maybe_redelivered`] set; check whether you handled it already
    /// (by ClOrdID, say) before acting on it again. Work handed off to another task counts as
    /// handled once this returns, unless the message is held with [`Context::hold`] until that
    /// work is done.
    ///
    /// Parse typed messages with [`Message::parse`]; a [`FieldError`] converts into the matching
    /// session-level reject. The borrowed form, `NameRef`, reads the message without copying its
    /// strings; [`into_owned`](crate::FixMessageRef::into_owned) gives the owned one to keep:
    ///
    /// ```
    /// # use turbojet::{Context, FixMessageRef, Message, MessageReject, MsgType};
    /// # use turbojet::message::tags;
    /// # use turbojet_fix::fix44::messages::{NewOrderSingle, NewOrderSingleRef};
    /// # fn book(_: NewOrderSingle) {}
    /// fn on_message(ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
    ///     if msg.msg_type() != MsgType::NewOrderSingle {
    ///         return Err(MessageReject::unsupported_message_type());
    ///     }
    ///     let order: NewOrderSingleRef = msg.parse()?;
    ///     if order.symbol == Some("XYZ") {
    ///         return Err(MessageReject::value_incorrect(tags::SYMBOL, "not traded here"));
    ///     }
    ///     book(order.into_owned());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The default rejects every message as an unsupported type.
    ///
    /// # Errors
    ///
    /// Return `Err` to reject the message: a [`MessageReject::Session`] answers it with a
    /// Reject(3), a [`MessageReject::Business`] with a BusinessMessageReject(j). Replies sent
    /// through `ctx` before then are dropped.
    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        Err(MessageReject::unsupported_message_type())
    }
}

/// How a logged-on session ended, given to [`Application::on_logout`] and
/// [`Application::on_cancel_on_disconnect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Disconnect {
    /// We logged out (through [`SessionHandle::logout`](crate::SessionHandle::logout)), and the
    /// counterparty answered, didn't within the logout timeout, or closed the connection instead.
    Logout,
    /// The counterparty logged out.
    CounterpartyLogout,
    /// The connection ended without a Logout: closed or reset by the counterparty, dropped by us
    /// because the counterparty stopped reading, ended with the connection's task, or failed with
    /// any other I/O error.
    ConnectionLost,
    /// The counterparty didn't answer a TestRequest.
    HeartbeatTimeout,
    /// We ended the session over an error: a sequence or header problem, a storage failure, a
    /// panicking `to_admin`. A Logout may have been sent first.
    Error,
    /// Our side shut down, or the session's schedule ended.
    Shutdown,
}

/// Passed to [`Application::on_message`] to reply on the same session.
///
/// ```
/// # use turbojet::*;
/// struct Echo;
///
/// impl Application for Echo {
///     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
///         if ctx.maybe_redelivered() {
///             // Handled before a crash, perhaps: look it up before acting on it again.
///         }
///         ctx.send(msg.clone())?; // replies go out with this message's batch
///         Ok(())
///     }
/// }
///
/// // In a unit test: a context for a disconnected session, and what it sent.
/// let session = SessionHandle::disconnected(SessionId::new("FIX.4.4", "US", "THEM"));
/// let mut ctx = Context::new(&session);
/// Echo.on_message(&mut ctx, &"8=FIX.4.4|35=B|148=Hello".parse()?)?;
/// assert_eq!(ctx.replies().len(), 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Context<'a> {
    session: &'a SessionHandle,
    checks: SendChecks<'a>,
    outbox: Outbox,
    redelivered: bool,
    /// Where [`hold`](Self::hold) records a hold: `None` but in `on_message`, and in tests.
    holding: Option<Holding<'a>>,
}

/// The session's side of [`Context::hold`] for one message.
pub(crate) struct Holding<'a> {
    holds: &'a mut Holds,
    commands: &'a CommandSender,
    seq_num: u64,
}

impl std::fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("session", self.session.id())
            .field("redelivered", &self.redelivered)
            .finish_non_exhaustive()
    }
}

impl<'a> Context<'a> {
    /// A context for `session`. The engine creates these; construct one yourself to unit-test an
    /// [`Application`], with a [`SessionHandle::disconnected`], then inspect what it sent with
    /// [`Context::replies`].
    ///
    /// Its replies are checked as a FIX 4.x session's are, with the standard data fields.
    pub fn new(session: &'a SessionHandle) -> Self {
        static STANDARD: std::sync::LazyLock<DataFields> = std::sync::LazyLock::new(DataFields::standard);
        Self::with_outbox(session, SendChecks { data_fields: &STANDARD, appl_versions: None }, Outbox::default())
    }

    /// A context whose replies are checked against `checks` and go into `outbox`, which the
    /// session keeps between calls to reuse its allocations.
    pub(crate) fn with_outbox(session: &'a SessionHandle, checks: SendChecks<'a>, outbox: Outbox) -> Self {
        debug_assert!(outbox.sent.is_empty(), "the session sends every reply before the next call");
        Self { session, checks, outbox, redelivered: false, holding: None }
    }

    /// The same context, for message `seq_num`, which [`hold`](Self::hold) records in `holds`;
    /// a release is told to the connection through `commands`.
    pub(crate) fn holding(mut self, holds: &'a mut Holds, commands: &'a CommandSender, seq_num: u64) -> Self {
        self.holding = Some(Holding { holds, commands, seq_num });
        self
    }

    /// The outbox, holding what the application sent, for the session to send and keep.
    pub(crate) fn into_outbox(self) -> Outbox {
        self.outbox
    }

    /// The same context, for a message that [may have been handled](Context::maybe_redelivered)
    /// already; for unit-testing an [`Application`].
    #[must_use]
    pub fn redelivery(mut self) -> Self {
        self.redelivered = true;
        self
    }

    /// Whether this message may have been handled already: the session stopped (a crash, or a
    /// dropped connection) while messages from shortly before this one were being handled, before
    /// they were recorded as received, and the counterparty has sent it again: flagged as a
    /// resend (PossDupFlag), or answering our ResendRequest. Up to 256 messages from the first
    /// that may have been handled are marked, so resends of some never handled may be too; new
    /// messages aren't. Stores that don't record messages in flight never mark one. A session
    /// taken over from another instance that may have handled messages this one doesn't count as
    /// received ([`Marks::maybe_handled`](crate::store::Marks::maybe_handled), as a replicated
    /// standby's promotion sets) marks the resends of everything the counterparty sent before its
    /// Logon, however many.
    pub fn maybe_redelivered(&self) -> bool {
        self.redelivered
    }

    /// Keeps this message in flight after `on_message` returns, until the [`Hold`] is released or
    /// dropped: for an application that finishes it elsewhere, such as a gateway forwarding a
    /// client's order to a venue's session, whose task stores it later.
    ///
    /// Without a hold, the message counts as received once `on_message` returns and its replies
    /// are stored. With one, it's still recorded as received, and later messages are handed over
    /// as usual, but the store's in-flight marker stays at it. If the process stops before the hold
    /// is released, the next connection asks for this message and everything after it again, each
    /// marked [`maybe_redelivered`](Self::maybe_redelivered): check whether you finished it before
    /// acting on it again. A hold released after its connection ended moves the stored marker on
    /// (on a tokio task, as an operator's change to a disconnected session is made), so the next
    /// connection doesn't ask for it again; one released only once the session has connected
    /// again, or released with no tokio runtime current, is asked for again, as after a crash.
    ///
    /// A session hands over at most 256 messages past the oldest one held: then it reads nothing
    /// more from the counterparty until that hold is released, so TCP slows the counterparty, and
    /// the wait doesn't count as the counterparty's silence. A release costs one message to the
    /// session's task, which then records the new marker with its next commit.
    ///
    /// Calling it again for the same message returns another handle to the same hold, released
    /// once every handle is. On a context made with [`Context::new`] it returns a hold that does
    /// nothing. Not available on FIXP sessions.
    #[must_use = "dropping a hold releases it"]
    pub fn hold(&mut self) -> Hold {
        match &mut self.holding {
            Some(holding) => holding.holds.take(holding.seq_num, holding.commands),
            None => Hold { _entry: None },
        }
    }

    /// Messages sent so far through this context, in order.
    pub fn replies(&self) -> &[Message] {
        &self.outbox.sent
    }

    /// The session the message arrived on.
    pub fn session(&self) -> &SessionHandle {
        self.session
    }

    /// Queues an application message, sent once `on_message` returns, in this message's batch;
    /// the engine adds the standard header. On FIXT.1.1 sessions, ApplVerID(1128) may name any
    /// version the session supports (the default goes unstated); CstmApplVerID(1129) and
    /// ApplExtID(1156) pass through. On FIX 4.x sessions, ApplVerID(1128) is left out.
    ///
    /// A typed message is written into a message the session reuses, so in steady state sending
    /// one allocates nothing beyond what the typed message itself holds.
    ///
    /// # Errors
    ///
    /// If the session would never send the message, it isn't queued: see [`SendRejected`]. The
    /// other replies are sent as usual. A reply can still go unsent after `on_message` returns,
    /// if the session's store fails or the connection ends, and the session's end says so
    /// ([`Application::on_logout`]).
    pub fn send(&mut self, msg: impl Reply) -> Result<(), SendRejected> {
        msg.send_to(self)
    }
}

/// A message kept in flight after `on_message` returned: see [`Context::hold`]. Released by
/// [`release`](Self::release), or by dropping it.
///
/// ```
/// # use turbojet::*;
/// struct Forwarder {
///     venue: SessionHandle,
/// }
///
/// impl Application for Forwarder {
///     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
///         let hold = ctx.hold(); // still in flight until released
///         let (venue, order) = (self.venue.clone(), msg.clone());
///         tokio::spawn(async move {
///             if let Ok(receipt) = venue.send_when_ready(order).await {
///                 let _ = receipt.await; // stored on the venue's session
///             }
///             hold.release();
///         });
///         Ok(())
///     }
/// }
/// ```
#[derive(Debug)]
#[must_use = "dropping a hold releases it"]
pub struct Hold {
    /// Kept for its drop, which releases the message; `None` for a hold that does nothing.
    _entry: Option<Arc<HoldEntry>>,
}

impl Hold {
    /// Releases the message: the application has finished it.
    pub fn release(self) {}
}

/// One message held, shared by its [`Hold`]s; the session keeps a weak reference, so the message
/// is released once the last hold is dropped.
#[derive(Debug)]
pub(crate) struct HoldEntry {
    shared: Arc<HoldShared>,
}

impl Drop for HoldEntry {
    /// Wakes the session's connection, unless it's been woken since it last looked; once the
    /// connection has ended, moves the store's in-flight marker on instead.
    fn drop(&mut self) {
        if !self.shared.woken.swap(true, Ordering::SeqCst) {
            self.shared.commands.hold_released();
        }
        AfterClose::released(&self.shared);
    }
}

/// What a session's holds share.
#[derive(Debug)]
struct HoldShared {
    /// The session's own command queues, which a release wakes its connection through.
    commands: CommandSender,
    /// A release has woken the connection since the session last looked at its holds, so later
    /// ones needn't: at most one wake-up waits in the control queue.
    woken: AtomicBool,
    /// Set as the session ends with messages still held: see [`AfterClose`].
    after_close: Mutex<Option<AfterClose>>,
    /// Whether `after_close` has been set, so a release while connected takes no lock.
    ended: AtomicBool,
}

/// The holds of a session whose connection has ended, and the in-flight marker it left in the
/// store at the oldest of them. A release moves the marker on, so the next connection doesn't
/// ask for the message again, through the registry, as an operator's change to a disconnected
/// session is made: on a task of its own, one at a time. A store whose marker isn't the one
/// left (the session has connected again, which already asked from it, or the last commit
/// failed) is left alone, and so are the holds from then on.
struct AfterClose {
    registry: Arc<SessionRegistry>,
    id: SessionId,
    /// By MsgSeqNum, as the session held them.
    entries: VecDeque<(u64, Weak<HoldEntry>)>,
    /// The next incoming number as the session ended.
    next: u64,
    /// Messages recovered from before a restart may still come back, so with no message held
    /// the marker stays at `next` rather than going.
    keep_marker: bool,
    /// The marker the store has, as this last wrote it or the session left it; `None` once gone.
    stored: Option<u64>,
    /// Whether a task is moving it.
    moving: bool,
}

impl fmt::Debug for AfterClose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AfterClose").field("id", &self.id).field("stored", &self.stored).finish_non_exhaustive()
    }
}

impl AfterClose {
    /// Where the marker belongs now: at the oldest message still held, or else at `next` while
    /// recovered messages may come back, or gone.
    fn target(&self) -> Option<u64> {
        let oldest = self.entries.iter().find(|(_, entry)| entry.strong_count() > 0).map(|&(seq, _)| seq);
        oldest.or(self.keep_marker.then_some(self.next))
    }

    /// A hold of `shared`'s was released: if its connection has ended and the marker should
    /// move, starts the task that moves it, unless one runs. Without a tokio runtime to run it
    /// on, the marker stays, and the next connection asks for the message again.
    fn released(shared: &Arc<HoldShared>) {
        if !shared.ended.load(Ordering::Acquire) {
            return;
        }
        let mut after = shared.after_close.lock().expect("holds lock poisoned");
        let Some(state) = after.as_mut() else { return };
        if state.moving || state.target() == state.stored {
            return;
        }
        let Ok(runtime) = tokio::runtime::Handle::try_current() else { return };
        state.moving = true;
        runtime.spawn(Self::move_marker(shared.clone()));
    }

    /// Moves the stored marker until it's where the holds say, as they're released meanwhile.
    async fn move_marker(shared: Arc<HoldShared>) {
        loop {
            let (registry, id, next, from, to) = {
                let mut after = shared.after_close.lock().expect("holds lock poisoned");
                let Some(state) = after.as_mut() else { return };
                let to = state.target();
                if to == state.stored {
                    state.moving = false;
                    return;
                }
                let from = state.stored.expect("a marker that's gone stays gone: no hold comes back");
                (state.registry.clone(), state.id.clone(), state.next, from, to)
            };
            let moved = registry.move_in_flight_offline(&id, next, from, to).await;
            let mut after = shared.after_close.lock().expect("holds lock poisoned");
            match moved {
                Ok(true) => {
                    if let Some(state) = after.as_mut() {
                        state.stored = to;
                    }
                }
                // Connected again, or another marker: the next connection has it in hand.
                Ok(false) | Err(_) => {
                    *after = None;
                    return;
                }
            }
        }
    }
}

/// The messages the application holds in a session.
#[derive(Debug, Default)]
pub(crate) struct Holds {
    /// By MsgSeqNum, one per message held. A released one is forgotten when it reaches the front,
    /// so they're all within the window: at most `DELIVERIES_PER_COMMIT`.
    entries: VecDeque<(u64, Weak<HoldEntry>)>,
    /// Made with the first hold, so a session that never holds allocates nothing for them.
    shared: Option<Arc<HoldShared>>,
}

impl Holds {
    /// A hold on message `seq_num`: the one it already has, if any.
    fn take(&mut self, seq_num: u64, commands: &CommandSender) -> Hold {
        let at = self.entries.partition_point(|&(seq, _)| seq < seq_num);
        if let Some(entry) = self.entries.get(at).filter(|(seq, _)| *seq == seq_num).and_then(|(_, e)| e.upgrade()) {
            return Hold { _entry: Some(entry) };
        }
        let shared = self.shared.get_or_insert_with(|| {
            Arc::new(HoldShared {
                commands: commands.clone(),
                woken: AtomicBool::new(false),
                after_close: Mutex::default(),
                ended: AtomicBool::new(false),
            })
        });
        let entry = Arc::new(HoldEntry { shared: shared.clone() });
        self.entries.insert(at, (seq_num, Arc::downgrade(&entry)));
        Hold { _entry: Some(entry) }
    }

    /// The first message still held, if any.
    pub(crate) fn oldest(&self) -> Option<u64> {
        self.entries.iter().find(|(_, entry)| entry.strong_count() > 0).map(|&(seq, _)| seq)
    }

    /// Forgets the released holds at the front. Releases from now on wake the connection again.
    pub(crate) fn tidy(&mut self) {
        if let Some(shared) = &self.shared {
            // Before looking, so a release after this is either seen or wakes the connection. A
            // swap, not a store: the counts below are read with a relaxed load, which a store
            // doesn't keep after it (on ARM64 the store can wait in a buffer while the load runs),
            // so a release could see the flag still set while we see its hold still live, and the
            // wake-up would be lost until the next timer. A swap's acquire keeps the loads after.
            let _ = shared.woken.swap(false, Ordering::SeqCst);
        }
        while self.entries.front().is_some_and(|(_, entry)| entry.strong_count() == 0) {
            self.entries.pop_front();
        }
    }

    /// Forgets every hold: the numbers they're for have been reset. Their releases do nothing.
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    /// The session `id` has ended, leaving the store's in-flight marker at `stored`, with `next`
    /// the next incoming number: from now on, a release moves the marker on (see
    /// [`AfterClose`]). `keep_marker`: recovered messages may still come back.
    pub(crate) fn hand_over(
        &mut self,
        registry: Arc<SessionRegistry>,
        id: SessionId,
        next: u64,
        stored: u64,
        keep_marker: bool,
    ) {
        let Some(shared) = &self.shared else { return };
        let entries = std::mem::take(&mut self.entries);
        let state = AfterClose { registry, id, entries, next, keep_marker, stored: Some(stored), moving: false };
        *shared.after_close.lock().expect("holds lock poisoned") = Some(state);
        shared.ended.store(true, Ordering::Release);
        // A release just before the hand-over found nothing to do: catch it up.
        AfterClose::released(shared);
    }
}

/// What [`Context::send`] takes: a [`Message`], or any typed [`FixMessage`].
pub trait Reply: sealed::Sealed {}

impl Reply for Message {}
impl<T: FixMessage> Reply for T {}

mod sealed {
    use super::{Context, FixMessage, Message, SendRejected};

    pub trait Sealed {
        /// Adds the message to what the application sent through `ctx`, if the session would
        /// send it.
        fn send_to(self, ctx: &mut Context<'_>) -> Result<(), SendRejected>;
    }

    impl Sealed for Message {
        fn send_to(self, ctx: &mut Context<'_>) -> Result<(), SendRejected> {
            ctx.checks.check(&self)?;
            ctx.outbox.sent.push(self);
            Ok(())
        }
    }

    impl<T: FixMessage> Sealed for T {
        fn send_to(self, ctx: &mut Context<'_>) -> Result<(), SendRejected> {
            let mut msg = ctx.outbox.spare.pop().unwrap_or_default();
            self.write_into(&mut msg);
            if let Err(rejected) = ctx.checks.check(&msg) {
                ctx.outbox.recycle(msg);
                return Err(rejected);
            }
            ctx.outbox.sent.push(msg);
            Ok(())
        }
    }
}

/// Why the session would never send an application message, so [`Context::send`] refuses it at
/// once. A message sent through a [`SessionHandle`] is checked the same way, and dropped with
/// [`Dropped::Rejected`](crate::Dropped::Rejected).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SendRejected {
    /// A session-level message type, which only the session sends.
    SessionLevel(MsgType),
    /// ApplVerID(1128) names a version this FIXT.1.1 session doesn't support.
    UnsupportedApplVerID(CompactString),
    /// The field with this tag would be misread on the wire, adding fields of its own: its
    /// value contains SOH, or it's a data field that doesn't directly follow its length.
    MisreadField(u32),
}

impl fmt::Display for SendRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionLevel(msg_type) => write!(f, "{msg_type} is a session-level message type"),
            Self::UnsupportedApplVerID(stated) => write!(f, "ApplVerID(1128) {stated} isn't one this session supports"),
            Self::MisreadField(tag) => {
                write!(f, "tag {tag} contains SOH, or is a data field that doesn't follow its length")
            }
        }
    }
}

impl std::error::Error for SendRejected {}

/// So that `on_message` can send with `ctx.send(reply)?`: a reply the session would never send
/// rejects the message it answers as a BusinessMessageReject(j) with reason
/// ApplicationNotAvailable, as a panic in `on_message` does, saying why in its Text. Anything the
/// application did with the message before the reply was refused stays done.
impl From<SendRejected> for MessageReject {
    fn from(rejected: SendRejected) -> Self {
        error!(%rejected, "rejecting a message whose reply the session wouldn't send");
        Self::business(BusinessRejectReason::ApplicationNotAvailable, format!("Application error: {rejected}"))
    }
}

/// What a session checks each application message it sends against.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SendChecks<'a> {
    /// The data fields, which alone may hold SOH.
    pub(crate) data_fields: &'a DataFields,
    /// On a FIXT.1.1 session, the versions it supports; `None` on FIX 4.x.
    pub(crate) appl_versions: Option<&'a [ApplVersion]>,
}

impl SendChecks<'_> {
    pub(crate) fn check(&self, msg: &Message) -> Result<(), SendRejected> {
        let msg_type = msg.msg_type();
        if msg_type.is_admin() {
            return Err(SendRejected::SessionLevel(msg_type));
        }
        if let Some(versions) = self.appl_versions
            && let Some(stated) = msg.get(tags::APPL_VER_ID)
            && !versions.iter().any(|v| v.id.code() == stated)
        {
            return Err(SendRejected::UnsupportedApplVerID(stated.into()));
        }
        match msg.invalid_data_field(self.data_fields) {
            Some(tag) => Err(SendRejected::MisreadField(tag)),
            None => Ok(()),
        }
    }
}

/// Spare messages kept to build replies in. Replies to one message are usually one or two, so a
/// few cover a burst without holding much memory.
const MAX_SPARE_MESSAGES: usize = 8;

/// The most a spare message may hold allocated (64 KiB), so one unusually large reply doesn't stay
/// allocated for the life of the session. Typed messages reserve about 32 bytes per field they
/// define, a few KiB for the largest.
const MAX_SPARE_MESSAGE_BYTES: usize = 64 * 1024;

/// The messages an application sends from a callback, and spare ones to build typed replies in.
/// The session keeps one between calls, so neither list, nor the replies built in the spares,
/// allocate once they've grown.
#[derive(Default)]
pub(crate) struct Outbox {
    /// What the application sent in this call. Only the application decides how many, so it has
    /// no limit here.
    pub(crate) sent: Vec<Message>,
    /// Emptied messages, at most `MAX_SPARE_MESSAGES`.
    spare: Vec<Message>,
}

impl Outbox {
    /// Keeps `msg`, sent or abandoned, to build a later reply in, unless enough are kept already
    /// or it holds too much memory to keep.
    pub(crate) fn recycle(&mut self, mut msg: Message) {
        debug_assert!(self.spare.len() <= MAX_SPARE_MESSAGES);
        if self.spare.len() < MAX_SPARE_MESSAGES && msg.capacity_bytes() <= MAX_SPARE_MESSAGE_BYTES {
            // Emptied, so a spare holds no stale content, only its allocations.
            msg.clear();
            self.spare.push(msg);
        }
    }
}

/// Why an application message was not accepted.
///
/// ```
/// use turbojet::message::tags;
/// use turbojet::{Message, MessageReject};
///
/// fn check(order: &Message) -> Result<(), MessageReject> {
///     let Some(qty) = order.get(tags::ORDER_QTY) else {
///         return Err(MessageReject::required_tag_missing(tags::ORDER_QTY));
///     };
///     if qty == "0" {
///         return Err(MessageReject::value_incorrect(tags::ORDER_QTY, "quantity must be positive"));
///     }
///     Ok(())
/// }
///
/// let order: Message = "8=FIX.4.4|35=D|11=order-1".parse().unwrap();
/// assert!(check(&order).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageReject {
    /// The message is malformed: answered with a session-level Reject(3).
    #[non_exhaustive]
    Session {
        /// The offending field, sent as RefTagID(371).
        ref_tag: Option<u32>,
        /// Sent as SessionRejectReason(373).
        reason: SessionRejectReason,
        /// Sent as Text(58).
        text: String,
    },
    /// The message is well-formed but cannot be processed: answered with a
    /// BusinessMessageReject(j).
    #[non_exhaustive]
    Business {
        /// The referenced message's business ID, its ClOrdID say, sent as
        /// BusinessRejectRefID(379). FIX requires it whenever the message has one: set it with
        /// [`with_ref_id`](MessageReject::with_ref_id).
        ref_id: Option<CompactString>,
        /// Sent as BusinessRejectReason(380).
        reason: BusinessRejectReason,
        /// Sent as Text(58).
        text: String,
    },
}

impl MessageReject {
    /// A session-level Reject(3) with `reason` and `text`, naming `ref_tag` if given.
    pub fn session(ref_tag: Option<u32>, reason: SessionRejectReason, text: impl Into<String>) -> Self {
        Self::Session { ref_tag, reason, text: text.into() }
    }

    /// A BusinessMessageReject(j) with `reason` and `text`; name the message's business ID with
    /// [`with_ref_id`](Self::with_ref_id).
    pub fn business(reason: BusinessRejectReason, text: impl Into<String>) -> Self {
        Self::Business { ref_id: None, reason, text: text.into() }
    }

    /// A required field is missing: a Reject(3) with reason RequiredTagMissing.
    pub fn required_tag_missing(tag: u32) -> Self {
        Self::Session {
            ref_tag: Some(tag),
            reason: SessionRejectReason::RequiredTagMissing,
            text: format!("Required tag {tag} missing"),
        }
    }

    /// A field's value isn't permitted: a Reject(3) with reason ValueIsIncorrect and `text`.
    pub fn value_incorrect(tag: u32, text: impl Into<String>) -> Self {
        Self::Session { ref_tag: Some(tag), reason: SessionRejectReason::ValueIsIncorrect, text: text.into() }
    }

    /// A field required in this context (e.g. Price on a limit order) is missing.
    pub fn conditionally_required_field_missing(tag: u32) -> Self {
        Self::Business {
            ref_id: None,
            reason: BusinessRejectReason::ConditionallyRequiredFieldMissing,
            text: format!("Conditionally required tag {tag} missing"),
        }
    }

    /// The application doesn't handle this MsgType: a BusinessMessageReject(j) with reason
    /// UnsupportedMessageType.
    pub fn unsupported_message_type() -> Self {
        Self::Business {
            ref_id: None,
            reason: BusinessRejectReason::UnsupportedMessageType,
            text: "Unsupported message type".into(),
        }
    }

    /// A Business reject naming the rejected message's business ID, its ClOrdID say, as
    /// BusinessRejectRefID(379). A Session reject has no such field, and is returned unchanged.
    #[must_use]
    pub fn with_ref_id(mut self, id: impl Into<CompactString>) -> Self {
        if let Self::Business { ref_id, .. } = &mut self {
            *ref_id = Some(id.into());
        }
        self
    }
}

/// The reject it's answered with: `Reject(3) RequiredTagMissing, tag 55: Required tag 55 missing`.
impl std::fmt::Display for MessageReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session { ref_tag: Some(tag), reason, text } => write!(f, "Reject(3) {reason:?}, tag {tag}: {text}"),
            Self::Session { ref_tag: None, reason, text } => write!(f, "Reject(3) {reason:?}: {text}"),
            Self::Business { ref_id: Some(id), reason, text } => {
                write!(f, "BusinessMessageReject(j) {reason:?}, ID {id}: {text}")
            }
            Self::Business { ref_id: None, reason, text } => write!(f, "BusinessMessageReject(j) {reason:?}: {text}"),
        }
    }
}

impl std::error::Error for MessageReject {}

impl From<FieldError> for MessageReject {
    fn from(e: FieldError) -> Self {
        Self::Session { ref_tag: Some(e.tag), reason: e.reject_reason(), text: e.to_string() }
    }
}

/// Runs an application callback, catching a panic so that one failing callback can't end the
/// connection (and lose the message it was handling). Returns `None`, having logged and counted
/// the panic, if it panicked. With `panic = "abort"` the process stops instead.
pub(crate) fn guarded<R>(callback: &'static str, f: impl FnOnce() -> R) -> Option<R> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(value) => Some(value),
        Err(payload) => {
            let text = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("(no message)");
            error!(callback, panic = text, "application panicked");
            crate::telemetry::application_panic(callback);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MsgType;
    use crate::admin::Heartbeat;
    use crate::message::tags;

    fn session_id() -> SessionId {
        SessionId::new("FIX.4.4", "GATEWAY", "CLIENT")
    }

    /// A typed reply is built in a spare message, which `send` takes from the outbox; a message
    /// sent as it is leaves the spares alone.
    #[test]
    fn rejects_display_as_the_reject_they_send() {
        assert_eq!(
            MessageReject::required_tag_missing(55).to_string(),
            "Reject(3) RequiredTagMissing, tag 55: Required tag 55 missing"
        );
        assert_eq!(
            MessageReject::unsupported_message_type().to_string(),
            "BusinessMessageReject(j) UnsupportedMessageType: Unsupported message type"
        );
    }

    fix_message! {
        /// News(B), with a Text(58) for the tests, not its Headline.
        News / NewsRef = "B" {
            text: opt CompactString = tags::TEXT,
        }
    }

    fn context(session: &SessionHandle, outbox: Outbox) -> Context<'_> {
        static STANDARD: std::sync::LazyLock<DataFields> = std::sync::LazyLock::new(DataFields::standard);
        Context::with_outbox(session, SendChecks { data_fields: &STANDARD, appl_versions: None }, outbox)
    }

    #[test]
    fn typed_replies_are_built_in_spare_messages() {
        let id = SessionHandle::disconnected(session_id());
        let mut outbox = Outbox::default();
        outbox.recycle(Message::new(MsgType::ExecutionReport).with(tags::TEXT, "old"));
        let mut ctx = context(&id, outbox);
        ctx.send(Message::new(MsgType::OrderCancelReject)).unwrap();
        assert_eq!(ctx.outbox.spare.len(), 1);
        ctx.send(News { text: Some("H".into()) }).unwrap();
        assert!(ctx.outbox.spare.is_empty(), "the typed reply took the spare");
        assert_eq!(ctx.replies(), [Message::new(MsgType::OrderCancelReject), News { text: Some("H".into()) }.into()]);
        // With no spare left, a typed reply is built in a new message.
        ctx.send(News { text: None }).unwrap();
        assert_eq!(ctx.replies().len(), 3);
    }

    /// What the session would never send is refused at once, and isn't queued; a refused typed
    /// reply gives its spare back.
    #[test]
    fn replies_the_session_would_never_send_are_refused() {
        let id = SessionHandle::disconnected(session_id());
        let versions = [ApplVersion::new(crate::ApplVerID::Fix50Sp2)];
        let data_fields = DataFields::standard();
        let mut outbox = Outbox::default();
        outbox.recycle(Message::new(MsgType::ExecutionReport));
        let checks = SendChecks { data_fields: &data_fields, appl_versions: Some(&versions) };
        let mut ctx = Context::with_outbox(&id, checks, outbox);
        assert_eq!(ctx.send(Heartbeat { test_req_id: None }), Err(SendRejected::SessionLevel(MsgType::Heartbeat)));
        assert_eq!(ctx.outbox.spare.len(), 1, "the refused typed reply gave its spare back");
        assert_eq!(
            ctx.send(Message::new(MsgType::ExecutionReport).with(tags::APPL_VER_ID, "7")),
            Err(SendRejected::UnsupportedApplVerID("7".into()))
        );
        assert_eq!(ctx.send(News { text: Some("a\x01b".into()) }), Err(SendRejected::MisreadField(tags::TEXT)));
        assert!(ctx.replies().is_empty());
        ctx.send(Message::new(MsgType::ExecutionReport).with(tags::APPL_VER_ID, "9")).unwrap();
        assert_eq!(ctx.replies().len(), 1, "a supported ApplVerID is sent");
    }

    #[test]
    fn spares_are_emptied_and_bounded_in_number_and_size() {
        let mut outbox = Outbox::default();
        for _ in 0..MAX_SPARE_MESSAGES + 3 {
            outbox.recycle(Message::new(MsgType::OrderCancelReject).with(tags::TEXT, "headline"));
        }
        assert_eq!(outbox.spare.len(), MAX_SPARE_MESSAGES);
        assert!(outbox.spare.iter().all(|msg| msg.fields_bytes().next().is_none()), "spares hold no fields");

        let mut outbox = Outbox::default();
        let large = Message::new(MsgType::OrderCancelReject).with(tags::TEXT, "x".repeat(MAX_SPARE_MESSAGE_BYTES));
        outbox.recycle(large);
        assert!(outbox.spare.is_empty(), "a message holding more than the limit isn't kept");
    }
}

#[cfg(all(test, shuttle))]
mod shuttle_tests {
    use shuttle::scheduler::RandomScheduler;
    use shuttle::{Config, MaxSteps, Runner};

    use super::*;
    use crate::registry::{Command, command_queues};

    /// Runs `test` in many random interleavings of its threads: random rather than PCT, which at
    /// as many never found a lost wake-up planted in `Holds::tidy`, since it switches threads
    /// only at a yield or a few change points spread over the run. One that spins past the step
    /// limit is a lost wake-up: the thread waiting has nothing left to wake it.
    fn check(test: impl Fn() + Send + Sync + 'static) {
        let mut config = Config::new();
        config.max_steps = MaxSteps::FailAfter(20_000);
        Runner::new(RandomScheduler::new(5_000), config).run(test);
    }

    /// Releases on other threads, while the session tidies and looks at its holds, each either
    /// shows as released when it looks or wakes it to look again: it never waits with every hold
    /// released and no wake-up coming.
    #[test]
    fn a_release_is_seen_or_wakes_the_session() {
        check(|| {
            let (commands, mut queues) = command_queues::<Message>(1);
            let mut holds = Holds::default();
            let releases: Vec<_> = [2, 3]
                .map(|seq| holds.take(seq, &commands))
                .into_iter()
                .map(|hold| shuttle::thread::spawn(move || hold.release()))
                .collect();
            loop {
                holds.tidy();
                if holds.oldest().is_none() {
                    break;
                }
                // Input waits for the next command, as the connection's driver does.
                while !matches!(queues.try_control(), Some(Command::HoldReleased)) {
                    shuttle::thread::yield_now();
                }
            }
            releases.into_iter().for_each(|release| release.join().unwrap());
        });
    }
}
