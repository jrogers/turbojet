//! The FIXP session: FIXP 1.0 point to point, as a sans-IO state machine, in either role.

use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::oneshot;
use tracing::{debug, info, warn};

use super::framing::{self, Framed};
use super::messages::{self as m, Decoded, FlowType};
use super::{
    ClientLogin, Ended, FixpApplication, FixpConfig, FixpContext, FixpHandle, FixpRegistry, Received, Role, SbeMessage,
};
use crate::connection::Driven;
use crate::fields::{Precision, UtcTimestamp};
use crate::peer::ConnectionInfo;
use crate::registry::{
    Command, CommandReceiver, CommandSender, Dropped, ReceiptSender, SequenceCommand, SequenceError, SequenceNumbers,
    apply_sequence_command, command_queues,
};
use crate::sbe::{Encode, SbeError};
use crate::session::DELIVERIES_PER_COMMIT;
use crate::store::{Commit, Fetched, Job, Opened, SentMessages, SessionId, SessionLog};
use crate::telemetry::{LatencyMetrics, SessionMetrics};

/// The BeginString part of a FIXP session's [`SessionId`], so its logs never share a name with a
/// FIX session's.
const BEGIN_STRING: &str = "FIXP";

/// A session ID: a UUID.
type Uuid = [u8; 16];

/// Most bytes of live messages held behind a gap on a recoverable flow, to deliver in order once
/// it's filled: as much as the connection driver holds unprocessed. Past it they're dropped, and
/// asked for once what's before them has arrived.
const MAX_QUEUED_BYTES: usize = 16 * 1024 * 1024;
/// Most messages held behind a gap: fewer than a window, so that the frame filling the gap and
/// all those it lets through are handed over in one committed window.
#[allow(clippy::cast_possible_truncation, reason = "255 fits any usize")]
const MAX_QUEUED: usize = (DELIVERIES_PER_COMMIT - 1) as usize;

/// Where the session is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Client: connected, opening its log or about to negotiate or establish.
    Starting,
    /// Server: waiting for a client's `Negotiate` or `Establish`.
    AwaitingClient,
    /// Client: sent `Negotiate`, waiting for the answer.
    Negotiating,
    /// Server: answered `Negotiate`, waiting for `Establish`.
    Negotiated,
    /// Client: sent `Establish`, waiting for the answer.
    Establishing,
    Established,
    /// We sent `Terminate`, and wait for the counterparty's.
    Terminating,
    Closed,
}

/// What the server does once the log a client named is open.
#[derive(Debug, Clone, Copy)]
enum Request {
    Negotiate { timestamp: u64 },
    Establish { timestamp: u64, keepalive: Duration, next_seq: Option<u64> },
}

/// A log the store is opening with a job.
struct Opening {
    id: SessionId,
    session_id: Uuid,
    then: Option<Request>,
    job: Option<Job<Box<dyn SessionLog>>>,
}

/// The session's log, once open, and which session it is.
struct Bound {
    id: SessionId,
    handle: FixpHandle,
    log: Box<dyn SessionLog>,
    session_id: Uuid,
}

/// A read of stored messages for a retransmission.
struct Fetch {
    job: Option<Job<SentMessages>>,
    from: u64,
    count: u64,
    timestamp: u64,
}

/// What we know of the counterparty's sequenced flow. The log's next incoming number is the
/// first not yet received in order: everything before it has been.
#[derive(Debug, Default)]
struct Inbound {
    /// The number the next live message takes. On a recoverable flow, past the log's next
    /// incoming number while there's a gap, which is asked for.
    live_next: u64,
    /// Recoverable: live messages past a gap, held to deliver in order once it's filled, ascending
    /// by number. Never delivered first: the log records only what's in order, so a message
    /// delivered past a gap would be asked for, and delivered, again after a reconnect.
    queued: VecDeque<(u64, Vec<u8>)>,
    /// The bytes in `queued`.
    queued_bytes: usize,
    /// A retransmission under way: the number the next replayed message takes, and how many are
    /// left.
    replay: Option<(u64, u64)>,
    /// The retransmission asked for and not yet answered: its first number and count.
    asked: Option<(u64, u64)>,
    /// The counterparty has finished sending, up to this number: we answer once we have it all.
    finishing: Option<u64>,
}

/// One FIXP session over one connection.
pub struct FixpSession {
    config: FixpConfig,
    registry: Arc<FixpRegistry>,
    app: Arc<dyn FixpApplication>,
    commands: CommandSender<SbeMessage>,
    state: State,
    bound: Option<Bound>,
    opening: Option<Opening>,
    fetch: Option<Fetch>,
    /// Framed messages for the driver to write; only `output[..committed]` may be.
    output: Vec<u8>,
    committed: usize,
    committing: bool,
    /// The log has changed since the last commit.
    dirty: bool,
    /// Receipts and operator replies, answered once what they report is committed.
    receipts: Vec<(ReceiptSender, u64)>,
    sequence_replies: Vec<(oneshot::Sender<Result<SequenceNumbers, SequenceError>>, SequenceNumbers)>,
    /// What the application replies from a callback (see [`FixpContext`]), kept to reuse.
    replies: Vec<u8>,
    ends: Vec<usize>,
    /// Scratch for encoding our own application messages (`NotApplied`).
    scratch: Vec<u8>,
    inbound: Inbound,
    /// We answered the counterparty's `FinishedSending`: the logical session ends with this
    /// connection.
    finalized: bool,
    peer_keepalive: Duration,
    last_sent: Instant,
    last_received: Instant,
    /// The time of the call being handled: what's sent now counts as sent then.
    now: Instant,
    /// When the handshake started, or our `Terminate` was sent.
    waiting_since: Instant,
    /// Whether the application was told the session was established, so is owed `on_ended`.
    established: bool,
    ended: Option<Ended>,
    /// Where the store's in-flight marker stood when the log opened: messages from it on (up to
    /// a window of them) may have been handed over before a crash or a lost connection, so their
    /// retransmissions are marked `maybe_redelivered`.
    recovered: Option<u64>,
    /// The end (exclusive) of the incoming numbers that may be handed over: the window the last
    /// commit recorded in flight, so that a crash while they're handled is noticed. Input that
    /// could hand over more waits for the next commit.
    window_end: Option<u64>,
    /// The start of the window the commit under way records.
    window_opening: Option<u64>,
    /// The store failed: nothing more is committed, so what it holds is what it last committed.
    store_failed: bool,
    /// We've sent `FinishedSending`: no more application messages go out, and the counterparty's
    /// `FinishedReceiving` finalizes the session.
    finishing: bool,
    /// The connection the session runs on, for [`FixpApplication::verify`].
    connection: ConnectionInfo,
}

impl fmt::Debug for FixpSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FixpSession")
            .field("id", &self.bound.as_ref().map(|b| &b.id))
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

/// Whether a flow numbers its messages.
fn sequenced(flow: FlowType) -> bool {
    matches!(flow, FlowType::Recoverable | FlowType::Idempotent)
}

impl FixpSession {
    /// A session for `config`'s role, for [`run`](super::run) or [`run_spinning`](super::run_spinning),
    /// or a driver of your own (see [`on_connect`](Self::on_connect)). The receiver carries
    /// [`FixpHandle`] commands.
    ///
    /// # Panics
    ///
    /// If `config` is invalid; see [`FixpConfig::check`].
    pub fn new(
        config: FixpConfig,
        registry: Arc<FixpRegistry>,
        app: Arc<dyn FixpApplication>,
        now: Instant,
    ) -> (Self, CommandReceiver<SbeMessage>) {
        if let Err(e) = config.check() {
            panic!("invalid FIXP configuration: {e}");
        }
        let (commands, receiver) = command_queues(config.send_queue);
        let state = if matches!(config.role, Role::Server(_)) { State::AwaitingClient } else { State::Starting };
        let session = Self {
            peer_keepalive: config.keepalive,
            config,
            registry,
            app,
            commands,
            state,
            bound: None,
            opening: None,
            fetch: None,
            output: Vec::new(),
            committed: 0,
            committing: false,
            dirty: false,
            receipts: Vec::new(),
            sequence_replies: Vec::new(),
            replies: Vec::new(),
            ends: Vec::new(),
            scratch: Vec::new(),
            inbound: Inbound::default(),
            finalized: false,
            last_sent: now,
            last_received: now,
            now,
            waiting_since: now,
            established: false,
            ended: None,
            recovered: None,
            window_end: None,
            window_opening: None,
            store_failed: false,
            finishing: false,
            connection: ConnectionInfo::default(),
        };
        (session, receiver)
    }

    /// Describes the connection the session runs on, for [`FixpApplication::verify`]: the
    /// client's address and, over TLS, its certificates. Without it, `verify` sees neither.
    #[must_use]
    pub fn with_connection(mut self, connection: ConnectionInfo) -> Self {
        self.connection = connection;
        self
    }

    /// Whether the session is established.
    #[must_use]
    pub fn is_established(&self) -> bool {
        self.state == State::Established
    }

    /// Whether the session has ended: write what's in the output, then close the connection.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.state == State::Closed
    }

    fn is_server(&self) -> bool {
        matches!(self.config.role, Role::Server(_))
    }

    /// Our flow, and the counterparty's.
    fn flows(&self) -> (FlowType, FlowType) {
        if self.is_server() {
            (self.config.server_flow, self.config.client_flow)
        } else {
            (self.config.client_flow, self.config.server_flow)
        }
    }

    /// Now on the wall clock, in nanoseconds since the Unix epoch, as FIXP timestamps are.
    fn timestamp(&self) -> u64 {
        let nanos = self.config.clock.now().timestamp_nanos_opt().unwrap_or_default();
        u64::try_from(nanos).unwrap_or_default()
    }

    /// Whether a counterparty's `timestamp` is within the window of our clock.
    fn timestamp_ok(&self, timestamp: u64) -> bool {
        let window = u64::try_from(self.config.timestamp_window.as_nanos()).unwrap_or(u64::MAX);
        self.timestamp().abs_diff(timestamp) <= window
    }

    fn log(&mut self) -> &mut dyn SessionLog {
        self.bound.as_mut().expect("bound before its log is used").log.as_mut()
    }

    fn session_id(&self) -> Uuid {
        self.bound.as_ref().map_or([0; 16], |b| b.session_id)
    }

    fn handle(&self) -> FixpHandle {
        self.bound.as_ref().expect("bound once established").handle.clone()
    }

    // ---- Connecting and binding ----

    fn connect(&mut self) {
        self.waiting_since = self.now;
        let Role::Client(client) = &self.config.role else { return };
        let id = SessionId::new(BEGIN_STRING, client.name.clone(), client.server.clone());
        self.bind(id, [0; 16], None);
    }

    /// Claims `id` and opens its log, then goes on with `then` (server) or starts (client).
    fn bind(&mut self, id: SessionId, session_id: Uuid, then: Option<Request>) {
        assert!(self.bound.is_none(), "a session binds to one log");
        match self.registry.acquire(&id, self.commands.clone(), None) {
            Ok(Opened::Ready(log)) => self.bound(id, session_id, log, then),
            Ok(Opened::Pending(job)) => self.opening = Some(Opening { id, session_id, then, job: Some(job) }),
            Err(reason) => {
                warn!("refusing the FIXP session: {reason}");
                self.close(Ended::Error);
            }
        }
    }

    fn bound(&mut self, id: SessionId, session_id: Uuid, log: Box<dyn SessionLog>, then: Option<Request>) {
        tracing::Span::current().record("id", tracing::field::display(&id));
        let handle = self.registry.handle(id.clone());
        self.recovered = log.in_flight();
        if let Some(start) = self.recovered {
            info!(start, "messages from this one on may have been handled before");
        }
        self.bound = Some(Bound { id, handle, log, session_id });
        match then {
            None => self.client_start(),
            Some(Request::Negotiate { timestamp }) => self.answer_negotiate(timestamp),
            Some(Request::Establish { timestamp, keepalive, next_seq }) => {
                self.answer_establish(timestamp, keepalive, next_seq);
            }
        }
    }

    // ---- The client's handshake ----

    /// A fresh log negotiates a new session; one already negotiated establishes it.
    fn client_start(&mut self) {
        let Role::Client(client) = &self.config.role else { unreachable!("only a client starts") };
        let name = client.name.clone();
        let created = self.log().created_at();
        let (created, negotiate) = match created {
            Some(at) => (at.into(), false),
            None => {
                let at = self.config.clock.now();
                // To the nanosecond, as the ID is derived from it: a store that writes it as text
                // (DiskStorage, SQL) writes what it's given, by default milliseconds.
                let stored = UtcTimestamp::new(at, Precision::Nanos);
                if let Err(e) = self.log().set_created_at(stored) {
                    return self.storage_failed(e);
                }
                self.dirty = true;
                (at, true)
            }
        };
        let session_id = session_uuid(&name, created);
        self.bound.as_mut().expect("bound").session_id = session_id;
        if negotiate { self.send_negotiate() } else { self.send_establish() }
    }

    fn send_negotiate(&mut self) {
        let Role::Client(client) = &self.config.role else { unreachable!("only a client negotiates") };
        let credentials = client.credentials.clone();
        let negotiate = m::Negotiate {
            session_id: self.session_id(),
            timestamp: self.timestamp(),
            client_flow: self.config.client_flow,
            credentials: &credentials,
        };
        info!(session_id = %uuid_text(&self.session_id()), "negotiating a new FIXP session");
        self.send(&negotiate);
        self.state = State::Negotiating;
    }

    fn send_establish(&mut self) {
        let Role::Client(client) = &self.config.role else { unreachable!("only a client establishes") };
        let credentials = client.credentials.clone();
        let next_seq_no = sequenced(self.config.client_flow).then(|| self.log().next_outgoing());
        let establish = m::Establish {
            session_id: self.session_id(),
            timestamp: self.timestamp(),
            keepalive_interval: millis(self.config.keepalive),
            next_seq_no,
            credentials: &credentials,
        };
        self.send(&establish);
        self.state = State::Establishing;
    }

    fn on_negotiation_response(&mut self, response: &m::NegotiationResponseRef<'_>) {
        if response.session_id() != self.session_id() {
            return self.protocol_error("the NegotiationResponse names another session");
        }
        if response.server_flow() != self.config.server_flow {
            return self.protocol_error("the server offers a flow other than the one configured");
        }
        self.send_establish();
    }

    fn on_negotiation_reject(&mut self, code: m::NegotiationRejectCode) {
        warn!(?code, "the server rejected Negotiate");
        // The ID is spent: the next connection negotiates another.
        self.reset_log();
        self.close(Ended::NegotiationRejected(code));
    }

    fn on_establishment_ack(&mut self, ack: &m::EstablishmentAckRef<'_>) {
        if ack.session_id() != self.session_id() {
            return self.protocol_error("the EstablishmentAck names another session");
        }
        self.peer_keepalive = Duration::from_millis(u64::from(ack.keepalive_interval()).max(1));
        self.establish();
        if let Some(next) = ack.next_seq_no() {
            self.peer_next(next);
        }
    }

    fn on_establishment_reject(&mut self, code: m::EstablishmentRejectCode) {
        warn!(?code, "the server rejected Establish");
        // The session is unknown or finalized there: negotiate a new one next time.
        if code == m::EstablishmentRejectCode::Unnegotiated {
            self.reset_log();
        }
        self.close(Ended::EstablishmentRejected(code));
    }

    // ---- The server's handshake ----

    fn on_negotiate(&mut self, negotiate: &m::NegotiateRef<'_>) {
        let (session_id, timestamp) = (negotiate.session_id(), negotiate.timestamp());
        let login = ClientLogin { session_id, credentials: negotiate.credentials(), connection: &self.connection };
        let reject = if self.state != State::AwaitingClient {
            Some((m::NegotiationRejectCode::Unspecified, "already negotiated on this connection"))
        } else if !self.timestamp_ok(timestamp) {
            Some((m::NegotiationRejectCode::Unspecified, "timestamp too far from the server's clock"))
        } else if negotiate.client_flow() != self.config.client_flow {
            Some((m::NegotiationRejectCode::FlowTypeNotSupported, "that client flow isn't supported"))
        } else if !self.app.verify(&login) {
            Some((m::NegotiationRejectCode::Credentials, "credentials refused"))
        } else {
            None
        };
        if let Some((code, reason)) = reject {
            return self.reject_negotiate(session_id, timestamp, code, reason);
        }
        let id = self.server_id(&session_id);
        self.bind(id, session_id, Some(Request::Negotiate { timestamp }));
    }

    fn server_id(&self, session_id: &Uuid) -> SessionId {
        let Role::Server(server) = &self.config.role else { unreachable!("only a server names logs by session") };
        SessionId::new(BEGIN_STRING, server.name.clone(), uuid_text(session_id))
    }

    fn answer_negotiate(&mut self, timestamp: u64) {
        let session_id = self.session_id();
        // A session ID is negotiated once, for all time: its log's creation time says it has been.
        if self.log().created_at().is_some() {
            let code = m::NegotiationRejectCode::DuplicateId;
            return self.reject_negotiate(session_id, timestamp, code, "session ID already used");
        }
        let at = self.config.clock.now().into();
        if let Err(e) = self.log().set_created_at(at) {
            return self.storage_failed(e);
        }
        self.dirty = true;
        let Role::Server(server) = &self.config.role else { unreachable!("only a server answers") };
        let credentials = server.credentials.clone();
        let response = m::NegotiationResponse {
            session_id,
            request_timestamp: timestamp,
            server_flow: self.config.server_flow,
            credentials: &credentials,
        };
        self.send(&response);
        self.state = State::Negotiated;
    }

    fn reject_negotiate(&mut self, session_id: Uuid, timestamp: u64, code: m::NegotiationRejectCode, reason: &str) {
        warn!(session_id = %uuid_text(&session_id), ?code, reason, "rejecting a FIXP Negotiate");
        let reject = m::NegotiationReject { session_id, request_timestamp: timestamp, code, reason: reason.as_bytes() };
        self.send(&reject);
        // No further messages: the connection closes.
        self.close(Ended::Error);
    }

    fn on_establish(&mut self, establish: &m::EstablishRef<'_>) {
        let (session_id, timestamp) = (establish.session_id(), establish.timestamp());
        let keepalive = Duration::from_millis(u64::from(establish.keepalive_interval()));
        let login = ClientLogin { session_id, credentials: establish.credentials(), connection: &self.connection };
        let reject = match self.state {
            State::Negotiated if session_id != self.session_id() => {
                Some((m::EstablishmentRejectCode::Unnegotiated, "another session was negotiated"))
            }
            State::Negotiated | State::AwaitingClient => None,
            _ => Some((m::EstablishmentRejectCode::AlreadyEstablished, "already established")),
        };
        let reject = reject.or_else(|| {
            if !self.timestamp_ok(timestamp) {
                Some((m::EstablishmentRejectCode::Unspecified, "timestamp too far from the server's clock"))
            } else if keepalive.is_zero() || keepalive > self.config.max_keepalive {
                Some((m::EstablishmentRejectCode::KeepaliveInterval, "keepalive interval out of range"))
            } else if !self.app.verify(&login) {
                Some((m::EstablishmentRejectCode::Credentials, "credentials refused"))
            } else {
                None
            }
        });
        if let Some((code, reason)) = reject {
            return self.reject_establish(session_id, timestamp, code, reason);
        }
        let next_seq = establish.next_seq_no();
        if self.state == State::Negotiated {
            self.answer_establish(timestamp, keepalive, next_seq);
        } else {
            let id = self.server_id(&session_id);
            self.bind(id, session_id, Some(Request::Establish { timestamp, keepalive, next_seq }));
        }
    }

    fn answer_establish(&mut self, timestamp: u64, keepalive: Duration, next_seq: Option<u64>) {
        let session_id = self.session_id();
        if self.log().created_at().is_none() {
            let code = m::EstablishmentRejectCode::Unnegotiated;
            return self.reject_establish(session_id, timestamp, code, "never negotiated, or finalized");
        }
        self.peer_keepalive = keepalive;
        let our_next = sequenced(self.flows().0).then(|| self.log().next_outgoing());
        let ack = m::EstablishmentAck {
            session_id,
            request_timestamp: timestamp,
            keepalive_interval: millis(self.config.keepalive),
            next_seq_no: our_next,
        };
        self.send(&ack);
        self.establish();
        if let Some(next) = next_seq {
            self.peer_next(next);
        }
    }

    fn reject_establish(&mut self, session_id: Uuid, timestamp: u64, code: m::EstablishmentRejectCode, reason: &str) {
        warn!(session_id = %uuid_text(&session_id), ?code, reason, "rejecting a FIXP Establish");
        let reject =
            m::EstablishmentReject { session_id, request_timestamp: timestamp, code, reason: reason.as_bytes() };
        self.send(&reject);
        self.close(Ended::Error);
    }

    /// Established: each sequenced flow starts with a `Sequence`.
    fn establish(&mut self) {
        self.state = State::Established;
        self.established = true;
        self.last_received = self.now;
        let next_incoming = self.log().next_incoming();
        self.inbound = Inbound { live_next: next_incoming, ..Inbound::default() };
        info!(session_id = %uuid_text(&self.session_id()), "FIXP session established");
        if sequenced(self.flows().0) {
            let next = self.log().next_outgoing();
            self.send(&m::Sequence { next_seq_no: next });
        }
        let handle = self.handle();
        self.app.on_established(&handle);
    }

    // ---- Messages ----

    fn on_frame(&mut self, bytes: &[u8]) {
        self.last_received = self.now;
        match m::decode(bytes) {
            // Applied and NotApplied are application messages: they take a sequence number.
            Ok((Decoded::Applied(_) | Decoded::NotApplied(_), _)) => self.on_application(bytes),
            Ok((message, _)) => self.on_session_message(message),
            // Not a session message: an application message, of this schema or another.
            Err(SbeError::UnknownTemplate(_) | SbeError::SchemaId(_)) => self.on_application(bytes),
            Err(e) => self.protocol_error(&format!("a malformed session message: {e}")),
        }
    }

    fn on_session_message(&mut self, message: Decoded<'_>) {
        debug!(target: "turbojet::messages", direction = "in", "{message:?}");
        let server = self.is_server();
        match (message, self.state) {
            (Decoded::Negotiate(n), _) if server => self.on_negotiate(&n),
            (Decoded::Establish(e), _) if server => self.on_establish(&e),
            (Decoded::NegotiationResponse(r), State::Negotiating) => self.on_negotiation_response(&r),
            (Decoded::NegotiationReject(r), State::Negotiating) => self.on_negotiation_reject(r.code()),
            (Decoded::EstablishmentAck(a), State::Establishing) => self.on_establishment_ack(&a),
            (Decoded::EstablishmentReject(r), State::Establishing) => self.on_establishment_reject(r.code()),
            (Decoded::Terminate(t), _) => self.on_terminate(t.code()),
            (message, State::Established) => self.on_established_message(message),
            // Our Terminate crossed the answer to our FinishedSending: the counterparty finalizes
            // on our Terminate, so we do too.
            (Decoded::FinishedReceiving(_), State::Terminating) if self.finishing => self.finalized = true,
            // After our Terminate, anything but the answer is ignored.
            (_, State::Terminating) => {}
            (message, state) => self.protocol_error(&format!("{message:?} isn't expected while {state:?}")),
        }
    }

    fn on_established_message(&mut self, message: Decoded<'_>) {
        let (ours, theirs) = self.flows();
        match message {
            Decoded::Sequence(s) if sequenced(theirs) => self.peer_next(s.next_seq_no()),
            Decoded::UnsequencedHeartbeat(_) if !sequenced(theirs) => {}
            Decoded::RetransmitRequest(r) if ours == FlowType::Recoverable => self.on_retransmit_request(&r),
            Decoded::Retransmission(r) if theirs == FlowType::Recoverable => {
                self.on_retransmission(r.next_seq_no(), u64::from(r.count()));
            }
            Decoded::RestransmitReject(r) if theirs == FlowType::Recoverable => self.on_retransmit_reject(r.code()),
            Decoded::FinishedSending(f) => self.on_finished_sending(f.last_seq_no()),
            Decoded::FinishedReceiving(_) if self.finishing => {
                info!("the counterparty has everything we sent: the session ends");
                self.finalized = true;
                self.terminate(m::TerminationCode::Finished);
            }
            Decoded::FinishedReceiving(_) => warn!("ignoring FinishedReceiving: we haven't finished sending"),
            message => self.protocol_error(&format!("{message:?} breaks the session's flows")),
        }
    }

    fn on_terminate(&mut self, code: m::TerminationCode) {
        // The answer to ours.
        if self.state == State::Terminating {
            let ours = self.ended.unwrap_or(Ended::TerminatedByUs(code));
            return self.close(if self.finalized { Ended::Finalized } else { ours });
        }
        info!(?code, "the counterparty terminated the FIXP connection");
        let terminate = m::Terminate { session_id: self.session_id(), code, reason: b"" };
        self.send(&terminate);
        self.close(if self.finalized { Ended::Finalized } else { Ended::TerminatedByPeer(code) });
    }

    /// An application message: numbered, if the counterparty's flow is sequenced, and handed to
    /// the application.
    fn on_application(&mut self, bytes: &[u8]) {
        // After our Terminate, ignored as session messages are: sent before it arrived. Unrecorded,
        // they're retransmitted or reported not applied on the next connection. But while we're
        // finishing, the counterparty may finalize on our Terminate, and there's no next
        // connection: they're handled.
        if self.state == State::Terminating && !self.finishing {
            return;
        }
        if !self.handles_input() {
            return self.protocol_error("an application message before the session is established");
        }
        match self.flows().1 {
            FlowType::None => return self.protocol_error("an application message on a None flow"),
            FlowType::Recoverable => self.on_recoverable(bytes),
            FlowType::Idempotent => {
                // A gap was reported not applied when it was found: always in order.
                let seq = self.inbound.live_next;
                self.inbound.live_next += 1;
                self.handle_application(bytes, Some(seq), false);
                self.set_next_incoming(seq + 1);
            }
            _ => self.handle_application(bytes, None, false),
        }
        self.check_finished();
    }

    /// An application message on a recoverable flow: replayed or live, taken if it's the next in
    /// order, held if it's live past a gap.
    fn on_recoverable(&mut self, bytes: &[u8]) {
        let contiguous = self.log().next_incoming();
        if let Some((next, left)) = self.inbound.replay {
            debug_assert!(left > 0, "a replay ends when nothing is left");
            self.inbound.replay = (left > 1).then(|| (next + 1, left - 1));
            // One already taken (held messages fill the gap from its end) is a repeat.
            if next == contiguous {
                self.take_in_order(bytes, next, true);
            }
            if self.inbound.replay.is_none() {
                self.replay_done();
            }
            return;
        }
        let seq = self.inbound.live_next;
        self.inbound.live_next += 1;
        if seq == contiguous {
            self.take_in_order(bytes, seq, false);
        } else if self.inbound.queued.len() < MAX_QUEUED && self.inbound.queued_bytes + bytes.len() <= MAX_QUEUED_BYTES
        {
            debug_assert!(seq > contiguous, "live numbers run ahead of the log's");
            self.inbound.queued_bytes += bytes.len();
            self.inbound.queued.push_back((seq, bytes.to_vec()));
        }
    }

    /// Takes message `seq`, the next in order, and then those held behind it that follow on.
    ///
    /// Each is recorded as received once it's been handed over: a crash in between leaves it in
    /// the window marked in flight, so it comes again marked `maybe_redelivered`, never lost.
    fn take_in_order(&mut self, bytes: &[u8], seq: u64, retransmitted: bool) {
        self.handle_application(bytes, Some(seq), retransmitted);
        self.set_next_incoming(seq + 1);
        self.take_queued();
    }

    /// Takes the held messages that are next in order.
    fn take_queued(&mut self) {
        while self.handles_input()
            && let Some(&(seq, _)) = self.inbound.queued.front()
            && seq == self.log().next_incoming()
        {
            let (seq, bytes) = self.inbound.queued.pop_front().expect("just looked");
            self.inbound.queued_bytes -= bytes.len();
            self.handle_application(&bytes, Some(seq), false);
            self.set_next_incoming(seq + 1);
        }
    }

    /// Hands a numbered (or unsequenced) application message on: the session's own Applied and
    /// NotApplied to itself, the rest to the application.
    fn handle_application(&mut self, bytes: &[u8], seq: Option<u64>, retransmitted: bool) {
        match m::decode(bytes) {
            Ok((Decoded::Applied(a), _)) => self.on_acknowledged(a.from_seq_no(), a.count(), true),
            Ok((Decoded::NotApplied(n), _)) => self.on_acknowledged(n.from_seq_no(), n.count(), false),
            _ => self.deliver(bytes, seq, retransmitted),
        }
    }

    /// Hands an application message to the application, then sends what it replied.
    fn deliver(&mut self, bytes: &[u8], seq: Option<u64>, retransmitted: bool) {
        debug_assert!(seq.is_none_or(|seq| self.covers(seq)), "{seq:?} is handed over in a committed window");
        // One in the window recovered from before may have been handled then if it's coming
        // again; a live one can't have been.
        let maybe_redelivered = retransmitted
            && seq.is_some_and(|seq| {
                self.recovered.is_some_and(|start| (start..start + DELIVERIES_PER_COMMIT).contains(&seq))
            });
        if maybe_redelivered {
            info!(?seq, "delivering a message that may have been handled before");
        }
        let (mut replies, mut ends) = (std::mem::take(&mut self.replies), std::mem::take(&mut self.ends));
        // Lent, not cloned: a clone copies the session ID's strings, for every message.
        let handle = &self.bound.as_ref().expect("bound once established").handle;
        let mut ctx = FixpContext { replies: &mut replies, ends: &mut ends, handle };
        self.app.on_message(&mut ctx, Received { bytes, seq, retransmitted, maybe_redelivered });
        let mut start = 0;
        for &end in &ends {
            self.send_application(&replies[start..end], None);
            start = end;
        }
        replies.clear();
        ends.clear();
        (self.replies, self.ends) = (replies, ends);
    }

    /// The counterparty's `Applied` or `NotApplied` of our idempotent flow.
    fn on_acknowledged(&mut self, from: u64, count: u32, applied: bool) {
        if self.flows().0 != FlowType::Idempotent {
            return self.protocol_error("Applied or NotApplied for a flow that isn't idempotent");
        }
        let handle = self.handle();
        if applied {
            self.app.on_applied(&handle, from, u64::from(count));
        } else {
            self.app.on_not_applied(&handle, from, u64::from(count));
        }
    }

    /// The counterparty says its next message is `next`: in a `Sequence`, `Establish` or
    /// `EstablishmentAck`. On a recoverable flow, what's skipped is asked for; on an idempotent
    /// one, it's reported not applied.
    fn peer_next(&mut self, next: u64) {
        let theirs = self.flows().1;
        if !sequenced(theirs) {
            return;
        }
        // A replay ends with the Sequence back to the live flow, whether or not it's complete.
        if self.inbound.replay.take().is_some() {
            self.replay_done();
        }
        let live = self.inbound.live_next;
        if next < live {
            return self.protocol_error("the counterparty's sequence numbers went backwards");
        }
        if next > live {
            self.inbound.live_next = next;
            if theirs == FlowType::Idempotent {
                self.send_not_applied(live, next - live);
                self.set_next_incoming(next);
            } else {
                warn!(from = live, to = next, "missed messages; asking for them");
            }
        }
        self.ask_for_gap();
        self.check_finished();
    }

    fn send_not_applied(&mut self, from: u64, count: u64) {
        let count = u32::try_from(count).unwrap_or(u32::MAX);
        if self.flows().0 == FlowType::None {
            return warn!(from, count, "messages not applied, but there's no flow to say so on");
        }
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.clear();
        let encoded = m::NotApplied { from_seq_no: from, count }.encode_into(&mut scratch);
        debug_assert!(encoded.is_ok(), "NotApplied is fixed-size");
        self.send_application(&scratch, None);
        self.scratch = scratch;
    }

    /// A retransmission we asked for has been answered: what's still missing is asked for.
    fn replay_done(&mut self) {
        self.inbound.asked = None;
        self.ask_for_gap();
    }

    /// Asks for the next part of a gap on a recoverable flow, if there's one and nothing's
    /// outstanding: from the log's next incoming number up to the first message held, or the live
    /// flow's next.
    fn ask_for_gap(&mut self) {
        if self.inbound.asked.is_some() || self.state != State::Established || self.flows().1 != FlowType::Recoverable {
            return;
        }
        let from = self.log().next_incoming();
        let end = self.inbound.queued.front().map_or(self.inbound.live_next, |&(seq, _)| seq);
        let count = end.saturating_sub(from).min(u64::from(self.config.max_retransmit));
        if count == 0 {
            return;
        }
        let request = m::RetransmitRequest {
            session_id: self.session_id(),
            timestamp: self.timestamp(),
            from_seq_no: from,
            count: u32::try_from(count).expect("at most max_retransmit"),
        };
        info!(from, count, "asking the counterparty to retransmit");
        self.send(&request);
        self.inbound.asked = Some((from, count));
    }

    /// A `Retransmission`: the next `count` application messages are replayed from `next`, which
    /// must lie within what we asked for.
    fn on_retransmission(&mut self, next: u64, count: u64) {
        let Some((from, asked)) = self.inbound.asked else {
            return self.protocol_error("a Retransmission no one asked for");
        };
        let within = count > 0 && next >= from && next.checked_add(count).is_some_and(|end| end <= from + asked);
        if !within || self.inbound.replay.is_some() {
            return self.protocol_error("a Retransmission outside what was asked for");
        }
        self.inbound.replay = Some((next, count));
    }

    fn on_retransmit_reject(&mut self, code: m::RetransmitRejectCode) {
        let Some((from, count)) = self.inbound.asked.take() else { return };
        warn!(?code, from, count, "the counterparty refused to retransmit; those messages are lost");
        // What was asked for is skipped. What's held past it is dropped and asked for again,
        // rather than handed over here, past the committed window.
        self.inbound.queued.clear();
        self.inbound.queued_bytes = 0;
        self.set_next_incoming(from + count);
        self.ask_for_gap();
        self.check_finished();
    }

    /// The counterparty has sent its last message, `last`: once we have everything up to it, we
    /// answer, and the logical session ends with this connection.
    fn on_finished_sending(&mut self, last: Option<u64>) {
        if self.finalized || self.inbound.finishing.is_some() {
            // Sent again as a heartbeat while it waits for our answer.
            return;
        }
        let theirs = self.flows().1;
        let live = self.inbound.live_next;
        let last = if sequenced(theirs) { last.unwrap_or(live.saturating_sub(1)) } else { 0 };
        self.inbound.finishing = Some(last);
        if sequenced(theirs) && last >= live {
            // We haven't seen its last messages live: the same as a Sequence past them.
            self.peer_next(last.saturating_add(1));
        }
        self.check_finished();
    }

    /// Answers the counterparty's `FinishedSending` once everything up to its last message has
    /// arrived in order.
    fn check_finished(&mut self) {
        let Some(last) = self.inbound.finishing else { return };
        if self.finalized || self.state != State::Established {
            return;
        }
        if sequenced(self.flows().1) && self.log().next_incoming() <= last {
            return;
        }
        self.send(&m::FinishedReceiving { session_id: self.session_id() });
        self.finalized = true;
        info!("the counterparty finished sending: the session ends with this connection");
    }

    fn on_retransmit_request(&mut self, request: &m::RetransmitRequestRef<'_>) {
        let (from, count, timestamp) = (request.from_seq_no(), u64::from(request.count()), request.timestamp());
        if self.fetch.is_some() {
            return self.terminate(m::TerminationCode::ReRequestInProgress);
        }
        // What the requester can't know (our session's ID, our limit) is refused; what it should
        // have known (what we've sent) ends the connection.
        if request.session_id() != self.session_id() {
            return self.reject_retransmit(timestamp, m::RetransmitRejectCode::InvalidSession);
        }
        if count > u64::from(self.config.max_retransmit) {
            return self.reject_retransmit(timestamp, m::RetransmitRejectCode::RequestLimitExceeded);
        }
        let next_outgoing = self.log().next_outgoing();
        if from == 0 || count == 0 || from.checked_add(count).is_none_or(|end| end > next_outgoing) {
            return self.terminate(m::TerminationCode::ReRequestOutOfBounds);
        }
        match self.log().fetch(from, from + count - 1) {
            Ok(Fetched::Ready(stored)) => self.retransmit(stored, from, count, timestamp),
            Ok(Fetched::Pending(job)) => self.fetch = Some(Fetch { job: Some(job), from, count, timestamp }),
            Err(e) => self.storage_failed(e),
        }
    }

    fn reject_retransmit(&mut self, timestamp: u64, code: m::RetransmitRejectCode) {
        let reject =
            m::RestransmitReject { session_id: self.session_id(), request_timestamp: timestamp, code, reason: b"" };
        self.send(&reject);
    }

    /// Sends a `Retransmission`, the stored messages, and a `Sequence` back to the live flow. A
    /// range with messages missing from the store (evicted) is refused instead.
    fn retransmit(&mut self, stored: SentMessages, from: u64, count: u64, timestamp: u64) {
        let complete = u64::try_from(stored.len()).is_ok_and(|n| n == count)
            && stored.iter().zip(from..).all(|((seq, _), expected)| *seq == expected);
        if !complete {
            return self.reject_retransmit(timestamp, m::RetransmitRejectCode::OutOfRange);
        }
        let retransmission = m::Retransmission {
            session_id: self.session_id(),
            request_timestamp: timestamp,
            next_seq_no: from,
            count: u32::try_from(count).expect("at most max_retransmit"),
        };
        self.send(&retransmission);
        // Stored framed, as sent.
        for (_, frame) in &stored {
            self.output.extend_from_slice(frame);
        }
        let next = self.log().next_outgoing();
        self.send(&m::Sequence { next_seq_no: next });
    }

    // ---- Sending ----

    /// Frames a session message into the output.
    fn send(&mut self, msg: &impl Encode) {
        // Closed (a store failure, say) partway through handling something: nothing more goes out.
        if self.state == State::Closed {
            return;
        }
        self.last_sent = self.now;
        if let Err(e) = framing::push(&mut self.output, msg) {
            // Session messages are fixed or bounded by the configuration: this is a bug.
            debug_assert!(false, "a session message didn't encode: {e}");
            warn!("a session message didn't encode: {e}");
        }
    }

    /// Sends an application message on our flow: numbered and recorded if it's sequenced (and
    /// kept, framed, if it's recoverable); the receipt is answered once committed.
    fn send_application(&mut self, sbe: &[u8], receipt: Option<ReceiptSender>) {
        let ours = self.flows().0;
        let refuse = |receipt: Option<ReceiptSender>, why: Dropped| {
            if let Some(receipt) = receipt {
                let _ = receipt.send(Err(why));
            }
        };
        if ours == FlowType::None {
            return refuse(receipt, Dropped::Rejected("our flow is None: no application messages".into()));
        }
        if self.state == State::Closed {
            return refuse(receipt, Dropped::Disconnected);
        }
        // A reply from a callback, say: we've said we've sent our last, or the session is ending.
        if self.done_sending() {
            return refuse(receipt, Dropped::LoggingOut);
        }
        let start = self.output.len();
        if let Err(e) = framing::push_bytes(&mut self.output, sbe) {
            return refuse(receipt, Dropped::Rejected(e.to_string()));
        }
        self.last_sent = self.now;
        let mut seq = 0;
        if sequenced(ours) {
            let bound = self.bound.as_mut().expect("bound before sending");
            seq = bound.log.next_outgoing();
            let stored = (ours == FlowType::Recoverable).then_some(&self.output[start..]);
            if let Err(e) = bound.log.record_outgoing(seq, stored) {
                self.output.truncate(start);
                refuse(receipt, Dropped::Storage);
                return self.storage_failed(e);
            }
            self.dirty = true;
        }
        if let Some(receipt) = receipt {
            self.receipts.push((receipt, seq));
        }
    }

    fn set_next_incoming(&mut self, seq: u64) {
        // Closed partway through (a store failure): what it would record never happened, so the
        // counterparty sends it again, or it's reported, on the next connection.
        if self.state == State::Closed {
            return;
        }
        // That clears the in-flight marker, so the open window's is recorded again: a store that
        // makes each change as it's made must still have it if the process stops mid-batch.
        let marker = self.window_end.map(|end| end - DELIVERIES_PER_COMMIT);
        let log = self.log();
        if let Err(e) =
            log.set_next_incoming(seq).and_then(|()| marker.map_or(Ok(()), |start| log.set_in_flight(start)))
        {
            return self.storage_failed(e);
        }
        self.dirty = true;
    }

    /// The session is finished with (spent, refused or finalized): the next connection starts a
    /// new one.
    fn reset_log(&mut self) {
        if let Err(e) = self.log().reset() {
            return self.storage_failed(e);
        }
        // Windows count in the old numbers.
        self.recovered = None;
        self.window_end = None;
        self.dirty = true;
    }

    // ---- Ending ----

    /// Sends `Terminate` and waits for the counterparty's.
    fn terminate(&mut self, code: m::TerminationCode) {
        self.send(&m::Terminate { session_id: self.session_id(), code, reason: b"" });
        self.state = State::Terminating;
        self.ended = Some(Ended::TerminatedByUs(code));
        self.waiting_since = self.now;
    }

    fn protocol_error(&mut self, reason: &str) {
        warn!("ending the FIXP connection: {reason}");
        let code = m::TerminationCode::UnspecifiedError;
        self.send(&m::Terminate { session_id: self.session_id(), code, reason: reason.as_bytes() });
        self.close(Ended::Error);
    }

    /// The store failed: nothing uncommitted goes out, and the session ends.
    fn storage_failed(&mut self, e: io::Error) {
        warn!("FIXP session store failed: {e}");
        self.store_failed = true;
        self.output.truncate(self.committed);
        for (receipt, _) in self.receipts.drain(..) {
            let _ = receipt.send(Err(Dropped::Storage));
        }
        self.close(Ended::Error);
    }

    fn close(&mut self, how: Ended) {
        if self.state == State::Closed {
            return;
        }
        self.state = State::Closed;
        self.ended = Some(how);
        if how == Ended::Finalized {
            self.reset_log();
        }
        if self.established {
            let handle = self.handle();
            self.app.on_ended(&handle, how);
        }
    }

    // ---- Commits and commands ----

    fn finish_commit(&mut self) {
        self.committed = self.output.len();
        self.window_end = self.window_opening.take().map(|start| start + DELIVERIES_PER_COMMIT);
        for (receipt, seq) in self.receipts.drain(..) {
            let _ = receipt.send(Ok(seq));
        }
        for (reply, numbers) in self.sequence_replies.drain(..) {
            let _ = reply.send(Ok(numbers));
        }
    }

    fn on_sequence_command(
        &mut self,
        command: SequenceCommand,
        reply: oneshot::Sender<Result<SequenceNumbers, SequenceError>>,
    ) {
        // Resetting a FIXP session means negotiating a new one, which needs it disconnected.
        if self.bound.is_none() || command == SequenceCommand::Reset {
            let _ = reply.send(Err(SequenceError::Connected));
            return;
        }
        let clock = self.config.clock.clone();
        match apply_sequence_command(self.log(), command, &clock) {
            Ok(numbers) => {
                self.dirty = true;
                self.sequence_replies.push((reply, numbers));
            }
            Err(e) => {
                let _ = reply.send(Err(e));
            }
        }
    }

    fn keepalive(&mut self) {
        let now = self.now;
        let silence = self.peer_keepalive.saturating_mul(self.config.keepalive_misses);
        if now >= self.last_received + silence {
            warn!(?silence, "the counterparty has been silent too long");
            let code = m::TerminationCode::UnspecifiedError;
            self.send(&m::Terminate { session_id: self.session_id(), code, reason: b"keepalive interval lapsed" });
            self.close(Ended::TerminatedByUs(code));
        } else if now >= self.last_sent + self.config.keepalive {
            if self.finishing {
                // Repeated until it's answered.
                self.send_finished_sending();
            } else if sequenced(self.flows().0) {
                let next = self.log().next_outgoing();
                self.send(&m::Sequence { next_seq_no: next });
            } else {
                self.send(&m::UnsequencedHeartbeat {});
            }
        }
    }
}

impl FixpSession {
    /// Finishes sending: `FinishedSending`, with the last number on a sequenced flow.
    fn finish(&mut self) {
        if self.finishing {
            return;
        }
        info!("finished sending: waiting for the counterparty to have everything");
        self.finishing = true;
        self.send_finished_sending();
    }

    /// Whether application messages may no longer go out: we've finished sending, or answered the
    /// counterparty's FinishedSending, so the session ends with this connection and a message
    /// sent now would be lost with it.
    fn done_sending(&self) -> bool {
        self.finishing || self.finalized
    }

    fn send_finished_sending(&mut self) {
        let last_seq_no = sequenced(self.flows().0).then(|| self.log().next_outgoing() - 1);
        self.send(&m::FinishedSending { session_id: self.session_id(), last_seq_no });
    }
}

/// The session's sans-IO interface: what a driver calls, as [`Session`](crate::Session)'s.
impl FixpSession {
    /// Whether the session waits for its store (a commit, the opening of its log, or a read for a
    /// retransmission): feed it nothing until that ends.
    #[must_use]
    pub fn is_waiting_on_store(&self) -> bool {
        self.committing || self.opening.is_some() || self.fetch.is_some()
    }

    /// Whether a commit from [`take_commit`](Self::take_commit) is under way.
    #[must_use]
    pub fn is_committing(&self) -> bool {
        self.committing
    }

    /// Framed messages to write to the counterparty, in order: only what the store has committed.
    /// Once [`is_closed`](Self::is_closed), write them, then close the connection.
    #[must_use]
    pub fn output(&self) -> &[u8] {
        &self.output[..self.committed]
    }

    /// Empties [`output`](Self::output) once it's been written.
    pub fn clear_output(&mut self) {
        self.output.drain(..self.committed);
        self.committed = 0;
    }

    /// When [`on_timer`](Self::on_timer) is next due.
    #[must_use]
    pub fn next_deadline(&self) -> Option<Instant> {
        match self.state {
            State::Closed => None,
            State::Established => {
                let silence = self.peer_keepalive.saturating_mul(self.config.keepalive_misses);
                Some((self.last_sent + self.config.keepalive).min(self.last_received + silence))
            }
            // Unanswered for a keepalive interval, the session counts as terminated anyway.
            State::Terminating => Some(self.waiting_since + self.peer_keepalive),
            _ => Some(self.waiting_since + self.config.handshake_timeout),
        }
    }

    /// The connection is up: a client negotiates or establishes, a server waits for a client.
    pub fn on_connect(&mut self, now: Instant) {
        self.now = now;
        self.connect();
    }

    /// Ends the session: terminates once established, otherwise closes.
    pub fn on_shutdown(&mut self, now: Instant) {
        self.now = now;
        match self.state {
            State::Established => self.terminate(m::TerminationCode::Finished),
            State::Terminating | State::Closed => {}
            _ => self.close(Ended::TerminatedByUs(m::TerminationCode::Finished)),
        }
    }

    /// The connection has gone.
    pub fn on_disconnect(&mut self, now: Instant) {
        self.now = now;
        let how = match self.ended {
            // The counterparty has finalized too, whether or not our Terminate reached it.
            _ if self.finalized => Ended::Finalized,
            Some(how @ Ended::TerminatedByUs(_)) if self.state == State::Terminating => how,
            _ => Ended::ConnectionLost,
        };
        self.close(how);
    }

    /// Sends a keepalive, or ends a session silent too long, or a handshake or termination that
    /// took too long.
    pub fn on_timer(&mut self, now: Instant) {
        self.now = now;
        match self.state {
            State::Closed => {}
            State::Established => self.keepalive(),
            State::Terminating if now >= self.waiting_since + self.peer_keepalive => {
                let how = if self.finalized { Ended::Finalized } else { self.ended.unwrap_or(Ended::Error) };
                self.close(how);
            }
            State::Terminating => {}
            _ if now >= self.waiting_since + self.config.handshake_timeout => {
                warn!(state = ?self.state, "the FIXP handshake timed out");
                self.close(Ended::Error);
            }
            _ => {}
        }
    }

    /// A command from a [`FixpHandle`]: a send, a logout (terminate) or an operator command.
    pub fn on_command(&mut self, command: Command<SbeMessage>, now: Instant) {
        self.now = now;
        match command {
            Command::Send(msg, receipt) if self.state == State::Established && !self.done_sending() => {
                self.send_application(msg.bytes(), receipt);
            }
            Command::Send(_, receipt) => {
                if let Some(receipt) = receipt {
                    let ending = self.state == State::Terminating || self.done_sending();
                    let dropped = if ending { Dropped::LoggingOut } else { Dropped::Disconnected };
                    let _ = receipt.send(Err(dropped));
                }
            }
            Command::Logout(_) if self.state == State::Established => self.terminate(m::TerminationCode::Finished),
            Command::Logout(_) => self.close(Ended::TerminatedByUs(m::TerminationCode::Finished)),
            Command::Sequence(command, reply) => self.on_sequence_command(command, reply),
            Command::Finish if self.state == State::Established => self.finish(),
            Command::Finish => debug!("ignoring a request to finish sending: not established"),
        }
    }

    /// The store's commit of what the session has done, to run before any of it is written: call
    /// it after every call into the session. `None` when nothing waits (the store committed at
    /// once).
    pub fn take_commit(&mut self) -> Option<Commit> {
        // After a failure, what the store may half hold (a change made before the call failed)
        // isn't committed: the next connection starts from the last commit.
        if self.committing || self.store_failed {
            return None;
        }
        if let Err(e) = self.mark_in_flight() {
            self.storage_failed(e);
            return None;
        }
        if self.dirty
            && let Some(bound) = self.bound.as_mut()
        {
            self.dirty = false;
            match bound.log.commit() {
                Ok(Some(commit)) => {
                    self.committing = true;
                    return Some(commit);
                }
                Ok(None) => {}
                Err(e) => {
                    self.storage_failed(e);
                    return None;
                }
            }
        }
        self.finish_commit();
        None
    }

    /// The store's read for a retransmission, if it returned a job: run it, then
    /// [`on_fetched`](Self::on_fetched).
    pub fn take_fetch(&mut self) -> Option<Job<SentMessages>> {
        self.fetch.as_mut()?.job.take()
    }

    /// The store's opening of the session's log, if it returned a job: run it, then
    /// [`on_opened`](Self::on_opened).
    pub fn take_open(&mut self) -> Option<Job<Box<dyn SessionLog>>> {
        self.opening.as_mut()?.job.take()
    }

    /// Records the window the next batch is handed over in as in flight, for the commit about to
    /// start: from the next incoming number on. A recovered window's messages still to come are
    /// in it too, so a second crash still notices them. Once the session has closed, with nothing
    /// left in flight and no recovered window's messages still to come, the marker is cleared.
    fn mark_in_flight(&mut self) -> io::Result<()> {
        let receiving = self.receiving();
        let closed = self.state == State::Closed;
        let recovered = self.recovered;
        let Some(bound) = self.bound.as_mut() else { return Ok(()) };
        let next = bound.log.next_incoming();
        // Messages from the window recovered from before that are still to come may have been
        // handled then: its marker stays until they've come.
        let recovering = recovered.is_some_and(|start| next < start + DELIVERIES_PER_COMMIT);
        if receiving {
            if bound.log.in_flight() != Some(next) {
                bound.log.set_in_flight(next)?;
                self.dirty = true;
            }
            self.window_opening = Some(next);
        } else if closed && !recovering && bound.log.in_flight().is_some() {
            // Everything handed over was recorded with it: nothing is in flight.
            bound.log.set_next_incoming(next)?;
            self.dirty = true;
        }
        Ok(())
    }

    /// Whether incoming messages are handed over: established, with a sequenced flow from the
    /// counterparty.
    fn receiving(&self) -> bool {
        self.handles_input() && sequenced(self.flows().1)
    }

    /// Whether application messages from the counterparty are handled: established, or
    /// terminating while finishing (see `on_application`).
    fn handles_input(&self) -> bool {
        self.state == State::Established || self.state == State::Terminating && self.finishing
    }

    /// Whether incoming `seq` may be handed to the application: it's in the committed window.
    fn covers(&self, seq: u64) -> bool {
        self.window_end.is_some_and(|end| seq < end)
    }

    /// Whether the window covers whatever the next frame could hand over: the next in order and
    /// every message held behind it.
    fn window_fits(&mut self) -> bool {
        let held = u64::try_from(self.inbound.queued.len()).expect("bounded by MAX_QUEUED");
        let last = self.log().next_incoming() + held;
        self.covers(last)
    }

    /// A commit from [`take_commit`](Self::take_commit) has ended.
    pub fn on_committed(&mut self, result: io::Result<()>, now: Instant) {
        self.now = now;
        debug_assert!(self.committing, "a commit was taken");
        self.committing = false;
        match result {
            Ok(()) => self.finish_commit(),
            Err(e) => self.storage_failed(e),
        }
    }

    /// A read from [`take_fetch`](Self::take_fetch) has ended.
    pub fn on_fetched(&mut self, result: io::Result<SentMessages>, now: Instant) {
        self.now = now;
        let Some(fetch) = self.fetch.take() else { return };
        match result {
            Ok(stored) if self.state == State::Established => {
                self.retransmit(stored, fetch.from, fetch.count, fetch.timestamp);
            }
            Ok(_) => {}
            Err(e) => self.storage_failed(e),
        }
    }

    /// An opening from [`take_open`](Self::take_open) has ended.
    pub fn on_opened(&mut self, result: io::Result<Box<dyn SessionLog>>, now: Instant) {
        self.now = now;
        let Some(Opening { id, session_id, then, .. }) = self.opening.take() else { return };
        match result {
            Ok(log) if self.state == State::Closed => {
                drop(log);
                self.registry.release(&id, &self.commands);
            }
            Ok(log) => self.bound(id, session_id, log, then),
            Err(e) => {
                warn!("refusing the FIXP session: cannot open its store: {e}");
                self.registry.release(&id, &self.commands);
                self.close(Ended::Error);
            }
        }
    }

    /// Hands the session the complete frames at the start of `buf`, removing them, until it must
    /// stop: it's closed, waits for its store, or waits for a commit to record the next messages
    /// it hands over as in flight. Returns whether input was left waiting for the store (call
    /// [`take_commit`](Self::take_commit), then feed it again), rather than for more to arrive.
    pub fn feed(&mut self, buf: &mut Vec<u8>, now: Instant) -> bool {
        self.now = now;
        let mut consumed = 0;
        let deferred = loop {
            if self.state == State::Closed {
                break false;
            }
            // Waiting for the store, or for a commit to record a window to hand messages over in.
            if self.is_waiting_on_store() || self.receiving() && !self.window_fits() {
                break consumed < buf.len();
            }
            match framing::frame(&buf[consumed..]) {
                Framed::Message(len) => {
                    debug_assert!(len > framing::HEADER);
                    let frame = consumed + framing::HEADER..consumed + len;
                    consumed += len;
                    self.on_frame(&buf[frame]);
                }
                Framed::Incomplete => break false,
                Framed::Invalid(reason) => {
                    consumed = buf.len();
                    self.protocol_error(reason);
                    break false;
                }
            }
        };
        buf.drain(..consumed);
        deferred
    }
}

impl Drop for FixpSession {
    fn drop(&mut self) {
        if self.state != State::Closed {
            self.close(Ended::ConnectionLost);
        }
        for (receipt, _) in self.receipts.drain(..) {
            let _ = receipt.send(Err(Dropped::Disconnected));
        }
        if let Some(opening) = self.opening.take() {
            self.registry.release(&opening.id, &self.commands);
        }
        if let Some(bound) = self.bound.take() {
            drop(bound.log);
            self.registry.release(&bound.id, &self.commands);
        }
    }
}

impl Driven for FixpSession {
    type Item = SbeMessage;
    type Scratch = ();

    fn has_logged_on(&self) -> bool {
        self.is_established()
    }
    fn is_closed(&self) -> bool {
        FixpSession::is_closed(self)
    }
    fn is_resending(&self) -> bool {
        false
    }
    fn is_waiting_on_store(&self) -> bool {
        FixpSession::is_waiting_on_store(self)
    }
    fn is_committing(&self) -> bool {
        FixpSession::is_committing(self)
    }
    fn output(&self) -> &[u8] {
        FixpSession::output(self)
    }
    fn clear_output(&mut self) {
        FixpSession::clear_output(self);
    }
    fn next_deadline(&self) -> Option<Instant> {
        FixpSession::next_deadline(self)
    }
    fn can_send(&self, _now: Instant) -> bool {
        true
    }
    fn send_free_at(&self) -> Option<Instant> {
        None
    }
    fn input_free_at(&self) -> Option<Instant> {
        None
    }
    fn metrics(&self) -> Option<&SessionMetrics> {
        None
    }
    fn latency_metrics(&self) -> Option<&LatencyMetrics> {
        None
    }
    fn times_latency(&self) -> bool {
        false
    }
    fn on_connect(&mut self, now: Instant) {
        FixpSession::on_connect(self, now);
    }
    fn on_shutdown(&mut self, _text: Option<&str>, now: Instant) {
        FixpSession::on_shutdown(self, now);
    }
    fn on_disconnect(&mut self, now: Instant) {
        FixpSession::on_disconnect(self, now);
    }
    fn on_timer(&mut self, now: Instant) {
        FixpSession::on_timer(self, now);
    }
    fn on_resume(&mut self, _now: Instant) {}
    fn on_command(&mut self, command: Command<SbeMessage>, now: Instant) {
        FixpSession::on_command(self, command, now);
    }
    fn on_sends_held(&mut self) {}
    fn take_commit(&mut self, _now: Instant) -> Option<Commit> {
        FixpSession::take_commit(self)
    }
    fn take_fetch(&mut self) -> Option<Job<SentMessages>> {
        FixpSession::take_fetch(self)
    }
    fn take_open(&mut self) -> Option<Job<Box<dyn SessionLog>>> {
        FixpSession::take_open(self)
    }
    fn on_committed(&mut self, result: io::Result<()>, now: Instant) {
        FixpSession::on_committed(self, result, now);
    }
    fn on_fetched(&mut self, result: io::Result<SentMessages>, now: Instant) {
        FixpSession::on_fetched(self, result, now);
    }
    fn on_opened(&mut self, result: io::Result<Box<dyn SessionLog>>, now: Instant) {
        FixpSession::on_opened(self, result, now);
    }
    fn feed(&mut self, buf: &mut Vec<u8>, _scratch: &mut (), now: Instant) -> bool {
        FixpSession::feed(self, buf, now)
    }
}

/// The session ID a client negotiates at `created`: a version 4 UUID, its random bits a hash of
/// the client's name and the time, so the client derives the same ID again after a restart, and
/// a new one for each new session. FNV-1a, which doesn't change between Rust versions as the
/// standard library's hashers may.
fn session_uuid(name: &str, created: chrono::DateTime<chrono::Utc>) -> Uuid {
    let nanos = created.timestamp_nanos_opt().unwrap_or_default().to_le_bytes();
    let mut uuid = [0; 16];
    for (half, seed) in uuid.chunks_mut(8).zip([0xcbf2_9ce4_8422_2325_u64, 0x6c62_272e_07bb_0142]) {
        let mut hash = seed;
        for &byte in name.as_bytes().iter().chain(&[0xFF]).chain(&nanos) {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
        }
        half.copy_from_slice(&hash.to_le_bytes());
    }
    uuid[6] = (uuid[6] & 0x0F) | 0x40;
    uuid[8] = (uuid[8] & 0x3F) | 0x80;
    uuid
}

/// A UUID as text: 32 hex digits.
fn uuid_text(uuid: &Uuid) -> String {
    uuid.iter().map(|b| format!("{b:02x}")).collect()
}

fn millis(duration: Duration) -> u32 {
    u32::try_from(duration.as_millis()).expect("FixpConfig::check bounds keepalives to u32 milliseconds")
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
