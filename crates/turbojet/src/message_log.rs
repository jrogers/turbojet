//! [`MessageLog`]: every message a session receives and sends, as its bytes on the wire.

use std::fmt;

use crate::SessionId;

/// Sees every message a FIX or FIXP session receives and sends, framed exactly as on the wire:
/// for an audit trail, or a store of what was received. Set it with
/// [`SessionConfig::message_log`](crate::SessionConfig::message_log), or a FIXP session or
/// endpoint's `with_message_log`: [`FixpSession`](crate::fixp::FixpSession::with_message_log),
/// [`FixpAcceptor`](crate::fixp::FixpAcceptor::with_message_log) or
/// [`FixpInitiator`](crate::fixp::FixpInitiator::with_message_log).
///
/// Turbojet's connection drivers call it: [`inbound`](Self::inbound) for each message read, just
/// before the session handles it, and [`outbound`](Self::outbound) for each message the session
/// sends (resends included), once its store has committed what the message covers. Within one
/// wake-up, the replies to a batch of input come after the whole batch, as they do on the wire.
/// Garbled input the codec skips isn't seen. A message is logged as it's queued to be written,
/// even if the connection fails before it is.
/// For FIXP, [`FixpSession::feed`](crate::fixp::FixpSession::feed) makes the inbound call itself,
/// so a driver of your own makes only the outbound one.
///
/// The calls are made on the session's task, between its handling of messages, so they must not
/// block: hand the bytes to a channel or a buffer that something else writes out. The bytes are
/// raw: Password(554), NewPassword(925) and FIXP credentials are in them.
///
/// `session` is `None` until the session is identified: an acceptor's Logon (a FIXP server's
/// first `Negotiate` or `Establish`) and anything before it, and anything sent on a connection
/// whose Logon is refused, or whose FIXP handshake is refused before the session's log is open.
/// The message itself names the counterparty.
///
/// ```
/// use std::sync::Mutex;
/// use std::sync::mpsc::{Sender, channel};
///
/// use turbojet::{MessageLog, SessionId};
///
/// /// Hands each message to a thread that writes the audit trail.
/// #[derive(Debug)]
/// struct Audit(Mutex<Sender<(bool, Option<String>, Vec<u8>)>>);
///
/// impl MessageLog for Audit {
///     fn inbound(&self, session: Option<&SessionId>, frame: &[u8]) {
///         let entry = (true, session.map(ToString::to_string), frame.to_vec());
///         let _ = self.0.lock().unwrap().send(entry);
///     }
///     fn outbound(&self, session: Option<&SessionId>, frame: &[u8]) {
///         let entry = (false, session.map(ToString::to_string), frame.to_vec());
///         let _ = self.0.lock().unwrap().send(entry);
///     }
/// }
///
/// let (sender, receiver) = channel();
/// let audit = Audit(Mutex::new(sender));
/// audit.inbound(None, b"8=FIX.4.4\x01...");
/// assert!(receiver.recv().unwrap().0);
/// ```
pub trait MessageLog: Send + Sync + fmt::Debug {
    /// A message read from the counterparty, before the session handles it: a FIX message from
    /// BeginString(8) to CheckSum(10), or a FIXP message with its framing header.
    fn inbound(&self, session: Option<&SessionId>, frame: &[u8]);

    /// A message the session sends, resends included, once its store has committed what it
    /// covers, as it's queued to be written.
    fn outbound(&self, session: Option<&SessionId>, frame: &[u8]);
}
