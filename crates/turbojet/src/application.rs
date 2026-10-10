//! The interface between the engine and the code using it.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

use tracing::error;

use crate::fields::{BusinessRejectReason, SessionRejectReason};
use crate::message::{FieldError, FixMessage, Message};
use crate::peer::ConnectionInfo;
use crate::registry::{CommandSender, SessionHandle};
use crate::store::SessionId;

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
    /// # use turbojet_fix44::messages::{NewOrderSingle, NewOrderSingleRef};
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
pub struct Context<'a> {
    session: &'a SessionHandle,
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
    pub fn new(session: &'a SessionHandle) -> Self {
        Self::with_outbox(session, Outbox::default())
    }

    /// A context whose replies go into `outbox`, which the session keeps between calls to reuse
    /// its allocations.
    pub(crate) fn with_outbox(session: &'a SessionHandle, outbox: Outbox) -> Self {
        debug_assert!(outbox.sent.is_empty(), "the session sends every reply before the next call");
        Self { session, outbox, redelivered: false, holding: None }
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
    /// messages aren't. Stores that don't record messages in flight never mark one.
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
    /// acting on it again. Holds don't outlast the connection: one released after it ended does
    /// nothing, and the next connection starts from the store's marker, as after a crash.
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

    /// The session the message arrived on: its ID.
    pub fn session_id(&self) -> &SessionId {
        self.session.id()
    }

    /// Queues an application message; the engine adds the standard header. A message with SOH
    /// inside a value is dropped (and logged), since it would add fields on the wire. On FIXT.1.1
    /// sessions, ApplVerID(1128) may name any version the session supports (the default goes
    /// unstated), and a message naming another is dropped (and logged); CstmApplVerID(1129) and
    /// ApplExtID(1156) pass through. On FIX 4.x sessions, ApplVerID(1128) is dropped.
    ///
    /// A typed message is written into a message the session reuses, so in steady state sending
    /// one allocates nothing beyond what the typed message itself holds.
    pub fn send(&mut self, msg: impl Reply) {
        msg.send_to(self);
    }
}

/// A message kept in flight after `on_message` returned: see [`Context::hold`]. Released by
/// [`release`](Self::release), or by dropping it.
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
    /// Wakes the session's connection, unless it's been woken since it last looked.
    fn drop(&mut self) {
        if !self.shared.woken.swap(true, Ordering::SeqCst) {
            self.shared.commands.hold_released();
        }
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
        let shared = self
            .shared
            .get_or_insert_with(|| Arc::new(HoldShared { commands: commands.clone(), woken: AtomicBool::new(false) }));
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
            // Before looking, so a release after this is either seen or wakes the connection.
            shared.woken.store(false, Ordering::SeqCst);
        }
        while self.entries.front().is_some_and(|(_, entry)| entry.strong_count() == 0) {
            self.entries.pop_front();
        }
    }

    /// Forgets every hold: the numbers they're for have been reset. Their releases do nothing.
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }
}

/// What [`Context::send`] takes: a [`Message`], or any typed [`FixMessage`].
pub trait Reply: sealed::Sealed {}

impl Reply for Message {}
impl<T: FixMessage> Reply for T {}

mod sealed {
    use super::{Context, FixMessage, Message};

    pub trait Sealed {
        /// Adds the message to what the application sent through `ctx`.
        fn send_to(self, ctx: &mut Context<'_>);
    }

    impl Sealed for Message {
        fn send_to(self, ctx: &mut Context<'_>) {
            ctx.outbox.sent.push(self);
        }
    }

    impl<T: FixMessage> Sealed for T {
        fn send_to(self, ctx: &mut Context<'_>) {
            let mut msg = ctx.outbox.spare.pop().unwrap_or_default();
            self.write_into(&mut msg);
            ctx.outbox.sent.push(msg);
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageReject {
    /// The message is malformed: answered with a session-level Reject(3).
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
    Business {
        /// Sent as BusinessRejectReason(380).
        reason: BusinessRejectReason,
        /// Sent as Text(58).
        text: String,
    },
}

impl MessageReject {
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
            reason: BusinessRejectReason::ConditionallyRequiredFieldMissing,
            text: format!("Conditionally required tag {tag} missing"),
        }
    }

    /// The application doesn't handle this MsgType: a BusinessMessageReject(j) with reason
    /// UnsupportedMessageType.
    pub fn unsupported_message_type() -> Self {
        Self::Business { reason: BusinessRejectReason::UnsupportedMessageType, text: "Unsupported message type".into() }
    }
}

/// The reject it's answered with: `Reject(3) RequiredTagMissing, tag 55: Required tag 55 missing`.
impl std::fmt::Display for MessageReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session { ref_tag: Some(tag), reason, text } => write!(f, "Reject(3) {reason:?}, tag {tag}: {text}"),
            Self::Session { ref_tag: None, reason, text } => write!(f, "Reject(3) {reason:?}: {text}"),
            Self::Business { reason, text } => write!(f, "BusinessMessageReject(j) {reason:?}: {text}"),
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

    #[test]
    fn typed_replies_are_built_in_spare_messages() {
        let id = SessionHandle::disconnected(session_id());
        let mut outbox = Outbox::default();
        outbox.recycle(Message::new(MsgType::ExecutionReport).with(tags::TEXT, "old"));
        let mut ctx = Context::with_outbox(&id, outbox);
        ctx.send(Message::new(MsgType::OrderCancelReject));
        assert_eq!(ctx.outbox.spare.len(), 1);
        ctx.send(Heartbeat { test_req_id: Some("T".into()) });
        assert!(ctx.outbox.spare.is_empty(), "the typed reply took the spare");
        assert_eq!(
            ctx.replies(),
            [Message::new(MsgType::OrderCancelReject), Heartbeat { test_req_id: Some("T".into()) }.into()]
        );
        // With no spare left, a typed reply is built in a new message.
        ctx.send(Heartbeat { test_req_id: None });
        assert_eq!(ctx.replies().len(), 3);
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
