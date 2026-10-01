//! The interface between the engine and the code using it.

use crate::fields::{BusinessRejectReason, SessionRejectReason};
use crate::message::{FieldError, Message};
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
/// may have half-modified; in `on_logon` or `on_logout` it is only logged. Each panic is logged
/// and counted (`turbojet_application_panics_total`). With `panic = "abort"` the process stops.
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

    /// The session has logged out or the connection was lost.
    fn on_logout(&self, _session: &SessionId) {}

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

/// Passed to [`Application::on_message`] to reply on the same session.
pub struct Context<'a> {
    session: &'a SessionId,
    pub(crate) replies: Vec<Message>,
    redelivered: bool,
}

impl<'a> Context<'a> {
    /// A context for `session`. The engine creates these; construct one yourself to unit-test an
    /// [`Application`], then inspect what it sent with [`Context::replies`].
    pub fn new(session: &'a SessionId) -> Self {
        Self { session, replies: Vec::new(), redelivered: false }
    }

    /// The same context, for a message that [may have been handled](Context::maybe_redelivered)
    /// already; for unit-testing an [`Application`].
    #[must_use]
    pub fn redelivery(mut self) -> Self {
        self.redelivered = true;
        self
    }

    /// Whether this message may have been handled already: the process stopped while it was being
    /// handled, before it was recorded as received, and the counterparty has resent it. Only
    /// that message is marked; an ordinary resend (PossDupFlag) of one never delivered isn't.
    /// Stores that don't record the message in flight never mark one.
    pub fn maybe_redelivered(&self) -> bool {
        self.redelivered
    }

    /// Messages sent so far through this context, in order.
    pub fn replies(&self) -> &[Message] {
        &self.replies
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
    pub fn send(&mut self, msg: impl Into<Message>) {
        self.replies.push(msg.into());
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
