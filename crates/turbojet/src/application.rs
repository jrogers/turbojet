//! The interface between the engine and the code using it.

use tracing::error;

use crate::fields::{BusinessRejectReason, SessionRejectReason};
use crate::message::{FieldError, FixMessage, Message};
use crate::peer::ConnectionInfo;
use crate::registry::SessionHandle;
use crate::store::SessionId;

/// Callbacks from the engine into the application. The engine handles the session layer
/// (logon, sequencing, heartbeats, resends, logout); the application sees only what it needs.
///
/// Callbacks run synchronously on the session's connection task and must not block. To do slow
/// work, hand it off (e.g. to a spawned task) and send the result later with a [`SessionHandle`].
///
/// A panicking callback doesn't end the connection. A panic in `on_message` is answered with a
/// BusinessMessageReject (ApplicationNotAvailable) and any replies it queued are dropped; in
/// `verify_logon` it refuses the logon; in `to_admin` it disconnects rather than send a message it
/// may have half-modified; in `on_logon`, `on_logout` or `on_cancel_on_disconnect` it is only
/// logged. Each panic is logged and counted (`turbojet_application_panics_total`). With
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
    fn verify_logon(&self, _session: &SessionId, _logon: &Message, _connection: &ConnectionInfo) -> Result<(), String> {
        Ok(())
    }

    /// Called before an outbound session-level message is sent, e.g. to add credentials to
    /// Logon. Header fields are added afterwards and cannot be changed here. On FIXT.1.1 sessions,
    /// leave DefaultApplVerID(1137) to the engine and configure it with
    /// [`SessionConfig::with_appl_ver_id`](crate::SessionConfig::with_appl_ver_id).
    fn to_admin(&self, _session: &SessionId, _msg: &mut Message) {}

    /// The session is logged on. Keep the handle to send messages outside of callbacks.
    fn on_logon(&self, _session: SessionHandle) {}

    /// A logged-on session has ended: it logged out, or the connection was lost. `ended` says
    /// which, so the application can tell a Logout, which both sides agreed to, from a session
    /// that stopped without one. Called once per logon, and never for a session that didn't log
    /// on.
    fn on_logout(&self, _session: &SessionId, _ended: Disconnect) {}

    /// A session with [cancel on disconnect](crate::SessionConfig::cancel_on_disconnect) ended
    /// with `ended`, and the counterparty didn't log back on within the grace period: cancel its
    /// resting orders. Called once per ending, after `on_logout`, from the task that keeps the
    /// countdowns (or, with no grace period, the connection's). A logon of the same session waits
    /// for this to return, so it never comes after that logon's `on_logon`. Like every callback it
    /// must not block. Countdowns pending when the acceptor or initiator shuts down fire then; a
    /// process that stops without shutting down loses them, so after a restart, check orders you
    /// kept.
    fn on_cancel_on_disconnect(&self, _session: &SessionId, _ended: Disconnect) {}

    /// An application-level message arrived in sequence. Replies sent through `ctx` go out in
    /// order immediately after this returns. Returning `Err` sends the corresponding reject.
    ///
    /// Delivery is at least once: the message counts as received only after this returns and its
    /// replies are stored, so if the process stops first, the counterparty resends it. Such a
    /// redelivery has [`Context::maybe_redelivered`] set; check whether you handled it already
    /// (by ClOrdID, say) before acting on it again. Work handed off to another task counts as
    /// handled once this returns.
    ///
    /// Parse typed messages with [`Message::parse`]; a [`FieldError`] converts into the matching
    /// session-level reject. The borrowed form, `NameRef`, reads the message without copying its
    /// strings; [`into_owned`](crate::FixMessageRef::into_owned) gives the owned one to keep:
    ///
    /// ```
    /// # use turbojet::{Context, FixMessageRef, Message, MessageReject, MsgType};
    /// # use turbojet_fix44::messages::{NewOrderSingle, NewOrderSingleRef};
    /// # fn book(_: NewOrderSingle) {}
    /// fn on_message(ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
    ///     if msg.msg_type() != MsgType::NewOrderSingle {
    ///         return Err(MessageReject::unsupported_message_type());
    ///     }
    ///     let order: NewOrderSingleRef = msg.parse()?;
    ///     if order.symbol == Some("XYZ") {
    ///         return Err(MessageReject::value_incorrect(55, "not traded here"));
    ///     }
    ///     book(order.into_owned());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The default rejects every message as an unsupported type.
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
    session: &'a SessionId,
    outbox: Outbox,
    redelivered: bool,
}

impl<'a> Context<'a> {
    /// A context for `session`. The engine creates these; construct one yourself to unit-test an
    /// [`Application`], then inspect what it sent with [`Context::replies`].
    pub fn new(session: &'a SessionId) -> Self {
        Self::with_outbox(session, Outbox::default())
    }

    /// A context whose replies go into `outbox`, which the session keeps between calls to reuse
    /// its allocations.
    pub(crate) fn with_outbox(session: &'a SessionId, outbox: Outbox) -> Self {
        debug_assert!(outbox.sent.is_empty(), "the session sends every reply before the next call");
        Self { session, outbox, redelivered: false }
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

    /// Messages sent so far through this context, in order.
    pub fn replies(&self) -> &[Message] {
        &self.outbox.sent
    }

    /// The session the message arrived on.
    pub fn session_id(&self) -> &SessionId {
        self.session
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
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() }
    }

    /// A typed reply is built in a spare message, which `send` takes from the outbox; a message
    /// sent as it is leaves the spares alone.
    #[test]
    fn typed_replies_are_built_in_spare_messages() {
        let id = session_id();
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
