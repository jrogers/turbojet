//! FIXP sessions: the FIX Performance Session Layer, version 1.0, over TCP, with SBE session
//! messages and Simple Open Framing Headers, in either role.
//!
//! FIXP is the session protocol under SBE-encoded order entry. A client *negotiates* a logical
//! session, identified by a UUID, and the type of flow in each direction; then *establishes* it
//! on each connection. Sequence numbers are implicit: a `Sequence` message sets the next one and
//! each application message after it takes the next. Each direction's flow is one of:
//!
//! - **Recoverable**: exactly once. The receiver asks for what it missed to be retransmitted.
//! - **Idempotent**: at most once. The receiver reports what it missed as not applied, and the
//!   sender decides what to do about it.
//! - **Unsequenced**: best effort, no numbers.
//! - **None**: no application messages that way.
//!
//! `Terminate` ends the connection, not the logical session, which the client establishes again
//! on the next. `FinishedSending` and `FinishedReceiving` end the logical session for good.
//!
//! [`FixpAcceptor`] serves clients over TCP, and [`FixpInitiator`] connects to a server and
//! reconnects. Underneath, a [`FixpSession`] is the protocol as a sans-IO state machine, run over
//! any byte stream by [`run`] or [`run_spinning`], as a FIX [`Session`](crate::Session) is. It keeps its state in a
//! [`SessionStorage`](crate::SessionStorage):
//!
//! - **The client** keeps one log per [`ClientConfig::name`], as `FIXP <name> → <server>`. The
//!   log's creation time is when the session was negotiated, and the session ID is derived from
//!   it and the name, so it survives restarts; a fresh log negotiates a new session, otherwise
//!   the client establishes the one it negotiated.
//! - **The server** keeps one log per session ID, as `FIXP <name> → <session ID>`.
//!
//! Both keep what they send on a recoverable flow, for retransmission. The application sees
//! application messages as their SBE bytes (header and body, framing removed), which it decodes
//! with a codec generated from its schema (see [`crate::sbe`]), and sends any generated message
//! through its [`FixpContext`] or [`FixpHandle`].
//!
//! [`FixpAcceptor::serve_tls`] and [`FixpInitiator::with_tls`] run sessions over TLS (feature
//! `tls`), and [`FixpApplication::verify`] sees the client certificate a server asked for.
//!
//! A [`FixpInitiator`] fails over to backup endpoints ([`FixpInitiator::with_failover`]), and
//! sessions record the per-session metrics (feature `metrics`; see
//! [`telemetry`](crate::telemetry)).
//!
//! Not supported: multiplexing sessions over one transport (`Context`), multicast (`Topic`),
//! in-band templates (`MessageTemplate`), and throttling.

mod endpoints;
pub(crate) mod framing;
// Generated from the FIX Trading Community's schema (scripts/codegen.sh); the session uses part.
#[allow(dead_code)]
mod messages;
mod session;

use std::fmt;
use std::io;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::runtime::Handle;

pub use endpoints::{FixpAcceptor, FixpInitiator};
pub use messages::{EstablishmentRejectCode, FlowType, NegotiationRejectCode, TerminationCode};
pub use session::FixpSession;

use crate::peer::ConnectionInfo;
use crate::registry::{CommandReceiver, SessionHandle, SessionRegistry};
use crate::sbe::{Encode, SbeError};
use crate::schedule::Clock;

/// Whether `bytes` is one whole FIXP frame, as FIXP sessions store the messages they send.
pub(crate) fn is_one_frame(bytes: &[u8]) -> bool {
    framing::frame(bytes) == framing::Framed::Message(bytes.len())
}

/// An encoded SBE message, header and body, as queued to send on a FIXP session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbeMessage(Vec<u8>);

impl SbeMessage {
    /// Encodes `msg`.
    ///
    /// # Errors
    ///
    /// As [`Encode::encode_into`].
    pub fn encode(msg: &impl Encode) -> Result<Self, SbeError> {
        let mut bytes = Vec::new();
        msg.encode_into(&mut bytes)?;
        Ok(Self(bytes))
    }

    /// The encoded message.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Sends on a FIXP session from anywhere; see [`SessionHandle`]. Its receipts resolve to the
/// message's sequence number, or 0 on an unsequenced flow.
pub type FixpHandle = SessionHandle<SbeMessage>;

impl SessionHandle<SbeMessage> {
    /// Finishes sending, once the messages already queued through
    /// [`send`](SessionHandle::send) have gone: the session sends `FinishedSending` with the
    /// last of them, refuses further sends, and once the counterparty answers that it has them
    /// all (`FinishedReceiving`), terminates, ending the logical session for good
    /// ([`Ended::Finalized`]); the next connection negotiates a new one. A connection lost first
    /// leaves the session unfinished: call it again once it's re-established.
    ///
    /// # Errors
    ///
    /// As [`logout`](SessionHandle::logout).
    pub fn finish(&self) -> Result<(), crate::registry::CommandError> {
        self.finish_sending()
    }
}

/// Tracks live FIXP sessions; see [`SessionRegistry`]. Build one with
/// [`SessionRegistry::with_storage`].
pub type FixpRegistry = SessionRegistry<SbeMessage>;

/// Which end of the session this is.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Role {
    /// The client, which negotiates and establishes.
    Client(ClientConfig),
    /// The server, which clients negotiate and establish with.
    Server(ServerConfig),
}

/// A client's identity and credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ClientConfig {
    /// Names this client's log, and goes into the session IDs it negotiates.
    pub name: String,
    /// Names the server in the log's ID.
    pub server: String,
    /// Sent in `Negotiate` and `Establish`, in whatever form the counterparties agree.
    pub credentials: Vec<u8>,
}

impl ClientConfig {
    /// A client called `name` of `server`, with no credentials.
    pub fn new(name: impl Into<String>, server: impl Into<String>) -> Self {
        Self { name: name.into(), server: server.into(), credentials: Vec::new() }
    }
}

/// A server's identity.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ServerConfig {
    /// Names the server's logs.
    pub name: String,
    /// Sent in `NegotiationResponse`, for clients that authenticate the server.
    pub credentials: Vec<u8>,
}

impl ServerConfig {
    /// A server called `name`, with no credentials.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), credentials: Vec::new() }
    }
}

/// A FIXP session's settings. Both ends configure both flows, as their rules of engagement fix
/// them: a server refuses a client asking for another flow, and a client a server offering one.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct FixpConfig {
    /// Client or server.
    pub role: Role,
    /// The flow from client to server.
    pub client_flow: FlowType,
    /// The flow from server to client.
    pub server_flow: FlowType,
    /// How long we may stay silent before sending a keepalive: sent in `Establish` (client) or
    /// `EstablishmentAck` (server).
    pub keepalive: Duration,
    /// The longest keepalive interval a server accepts from a client.
    pub max_keepalive: Duration,
    /// Silence from the counterparty for this many of its keepalive intervals ends the
    /// connection: some leniency, as the specification recommends, for timers that differ.
    pub keepalive_misses: u32,
    /// How far the timestamp of a `Negotiate` or `Establish` may be from our clock (server).
    pub timestamp_window: Duration,
    /// How long negotiating and establishing may take.
    pub handshake_timeout: Duration,
    /// The most messages one retransmission carries: a larger request is refused (sender), and
    /// a gap is asked for in parts no larger (receiver).
    pub max_retransmit: u32,
    /// Most application sends waiting for the connection; see
    /// [`SessionConfig::send_queue`](crate::SessionConfig::send_queue).
    pub send_queue: usize,
    /// The clock for timestamps and session IDs.
    pub clock: Clock,
    /// Records the latency histograms (feature `metrics`), as
    /// [`SessionConfig::latency_metrics`](crate::SessionConfig::latency_metrics) does for FIX.
    /// Off by default: they cost a clock read per inbound message, and a few per batch.
    #[cfg(feature = "metrics")]
    pub latency_metrics: bool,
}

impl FixpConfig {
    /// Settings for `role` with recoverable flows both ways: a keepalive of 1 s (at most 60 s from
    /// a client) and 3 misses, a 120 s timestamp window, 10 s to negotiate and establish,
    /// retransmissions of up to 1000 messages, and a send queue of 1024.
    pub fn new(role: Role) -> Self {
        Self {
            role,
            client_flow: FlowType::Recoverable,
            server_flow: FlowType::Recoverable,
            keepalive: Duration::from_secs(1),
            max_keepalive: Duration::from_secs(60),
            keepalive_misses: 3,
            timestamp_window: Duration::from_secs(120),
            handshake_timeout: Duration::from_secs(10),
            max_retransmit: 1000,
            send_queue: 1024,
            clock: Clock::system(),
            #[cfg(feature = "metrics")]
            latency_metrics: false,
        }
    }

    /// Checks the settings.
    ///
    /// # Errors
    ///
    /// Both flows `None` or an unknown flow, a keepalive of zero, over a `u32` of milliseconds or
    /// over `max_keepalive`, or a zero count, timeout or queue.
    pub fn check(&self) -> Result<(), crate::ConfigError> {
        let fail = |reason: &str| Err(crate::ConfigError::from(reason.to_string()));
        let known = |flow: FlowType| !matches!(flow, FlowType::Unknown(_));
        if !known(self.client_flow) || !known(self.server_flow) {
            return fail("client_flow and server_flow must be flow types FIXP defines");
        }
        if self.client_flow == FlowType::None && self.server_flow == FlowType::None {
            return fail("only one of client_flow and server_flow may be None");
        }
        let max_millis = Duration::from_millis(u64::from(u32::MAX));
        if self.keepalive.is_zero() || self.keepalive > self.max_keepalive || self.max_keepalive > max_millis {
            return fail("keepalive must be more than zero and at most max_keepalive, itself at most u32::MAX ms");
        }
        if self.keepalive_misses == 0 || self.handshake_timeout.is_zero() || self.max_retransmit == 0 {
            return fail("keepalive_misses, handshake_timeout and max_retransmit must be more than zero");
        }
        if self.send_queue == 0 {
            return fail("send_queue must be more than zero");
        }
        Ok(())
    }
}

/// A client asking to negotiate or establish, as the server's application sees it.
#[derive(Debug)]
#[non_exhaustive]
pub struct ClientLogin<'a> {
    /// The session ID it names.
    pub session_id: [u8; 16],
    /// Its credentials, as sent.
    pub credentials: &'a [u8],
    /// The connection it came on: the client's address and, over TLS with client certificates,
    /// the certificate chain it presented, already verified against the configured CAs.
    pub connection: &'a ConnectionInfo,
}

/// An application message received, as the application sees it. On a sequenced flow, messages
/// arrive in order, each once: on a recoverable one, live messages that arrive past a gap wait
/// for the retransmission that fills it.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Received<'a> {
    /// The SBE message, header and body: decode it with the matching generated codec.
    pub bytes: &'a [u8],
    /// Its implicit sequence number, on a sequenced flow.
    pub seq: Option<u64>,
    /// Whether it came in a retransmission we asked for, rather than live.
    pub retransmitted: bool,
    /// Whether it may have been handed over already: the connection was lost, or the process
    /// crashed, while messages from shortly before this one were being handled, before they were
    /// recorded as received, and this is its retransmission. Up to 256 messages from the first
    /// that may have been handled are marked, so some never handled may be too; live messages
    /// never are. Stores that don't record messages in flight never mark one.
    pub maybe_redelivered: bool,
}

/// How a FIXP connection ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Ended {
    /// We sent `Terminate` with this code (and the counterparty answered, or didn't in time).
    TerminatedByUs(TerminationCode),
    /// The counterparty sent `Terminate` with this code.
    TerminatedByPeer(TerminationCode),
    /// The logical session was finalized (`FinishedSending` answered), and the connection ended:
    /// the next connection negotiates a new session.
    Finalized,
    /// The server refused to negotiate.
    NegotiationRejected(NegotiationRejectCode),
    /// The server refused to establish.
    EstablishmentRejected(EstablishmentRejectCode),
    /// The connection was lost.
    ConnectionLost,
    /// The counterparty broke the protocol, or the store failed.
    Error,
}

/// What a FIXP session tells the application. Callbacks run on the connection's task and must
/// not block.
pub trait FixpApplication: Send + Sync {
    /// Server: whether a client may negotiate or establish. `false` refuses it as bad
    /// credentials. The default lets every client in.
    fn verify(&self, client: &ClientLogin<'_>) -> bool {
        let _ = client;
        true
    }

    /// The session is established on this connection: application messages can flow.
    fn on_established(&self, session: &FixpHandle) {
        let _ = session;
    }

    /// An application message from the counterparty.
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>);

    /// On an idempotent flow from us: the counterparty didn't apply `count` of our messages from
    /// `from` on. They won't be acted on, and won't be resent. Send them again, as new messages,
    /// or not.
    fn on_not_applied(&self, session: &FixpHandle, from: u64, count: u64) {
        let _ = (session, from, count);
    }

    /// On an idempotent flow from us: the counterparty applied `count` of our messages from
    /// `from` on (its `Applied`, where it has no application acknowledgement of its own).
    fn on_applied(&self, session: &FixpHandle, from: u64, count: u64) {
        let _ = (session, from, count);
    }

    /// The connection has ended, having been established, `how`.
    fn on_ended(&self, session: &FixpHandle, how: Ended) {
        let _ = (session, how);
    }
}

/// What a callback can do on the session it was called from: reply at once, ahead of anything
/// queued through a handle.
pub struct FixpContext<'a> {
    /// The replies, encoded one after another, each ending at the next of `ends`: buffers the
    /// session keeps, so replying allocates nothing once they've grown.
    pub(crate) replies: &'a mut Vec<u8>,
    pub(crate) ends: &'a mut Vec<usize>,
    pub(crate) handle: &'a FixpHandle,
}

impl fmt::Debug for FixpContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FixpContext").field("session", self.handle).finish_non_exhaustive()
    }
}

impl FixpContext<'_> {
    /// Sends `msg` after this callback, numbered next on a sequenced flow.
    ///
    /// # Errors
    ///
    /// As [`Encode::encode_into`]: nothing is sent.
    pub fn send(&mut self, msg: &impl Encode) -> Result<(), SbeError> {
        msg.encode_into(self.replies)?;
        self.ends.push(self.replies.len());
        Ok(())
    }

    /// The session's handle.
    #[must_use]
    pub fn session(&self) -> &FixpHandle {
        self.handle
    }
}

/// Runs a FIXP `session` over `stream` until either side disconnects: as [`crate::connection::run`].
///
/// # Errors
///
/// As [`crate::connection::run`].
pub async fn run<S>(stream: S, session: FixpSession, commands: CommandReceiver<SbeMessage>) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    crate::connection::run_tracked(stream, session, commands, &mut false, None).await
}

/// Runs a FIXP `session` on the calling thread, spinning: as [`crate::connection::run_spinning`].
///
/// # Errors
///
/// As [`crate::connection::run`].
pub fn run_spinning<S>(
    stream: S,
    session: FixpSession,
    commands: CommandReceiver<SbeMessage>,
    runtime: &Handle,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let _runtime = runtime.enter();
    crate::connection::run_spinning_tracked(stream, session, commands, &mut false, None)
}
