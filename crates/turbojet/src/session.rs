//! FIX session layer: logon, sequencing, heartbeats, resends and logout.
//!
//! [`Session`] is a sans-IO state machine that plays either the acceptor or the initiator role.
//! A driver (see [`crate::connection`]) feeds it inbound messages and handle commands, calls its
//! timer when its next deadline falls due, writes the encoded messages it leaves in
//! [`Session::output`], and closes the connection once [`Session::is_closed`], so the protocol
//! logic is deterministic and testable without sockets. Sequence numbers and sent messages are
//! persisted through a [`SessionLog`], and committed (see [`Session::take_commit`]) before the
//! corresponding message is handed to the driver.
//! Application messages are delivered to an [`Application`].

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use tracing::{debug, error, info, warn};

use crate::admin::{
    BusinessMessageReject, Heartbeat, Logon, Logout, Reject, ResendRequest, SequenceReset, TestRequest,
};
use crate::application::{Application, Context, Disconnect, MessageReject, Outbox, guarded};
use crate::cancel::{CancelOnDisconnect, MAX_CANCEL_GRACE};
use crate::codec::{Decoded, decode_stored, frame_stored, push_digits, push_trailer};
use crate::counterparty::{Counterparties, Counterparty};
use crate::fields::{
    ApplVerId, BusinessRejectReason, CompactString, EncryptMethod, MsgType, Precision, Secret, SessionRejectReason,
    ToFix, UtcTimestamp,
};
use crate::initiator::InitiatorConfig;
use crate::message::{DataFields, FieldError, Message, is_header_or_trailer, tags};
use crate::peer::ConnectionInfo;
use crate::registry::{
    Command, CommandReceiver, CommandSender, Dropped, ReceiptSender, SequenceCommand, SequenceError, SequenceNumbers,
    SessionHandle, SessionRegistry, apply_sequence_command, command_queues,
};
use crate::schedule::{Clock, Period, SessionSchedule};
use crate::store::{Commit, Fetched, Job, Opened, SentMessages, SessionId, SessionLog};
use crate::telemetry::{LatencyMetrics, SessionMetrics};
use crate::throttle::{Inbound, InboundLimit, Over, RateLimit, Window};

/// An application version a FIXT.1.1 session supports, with the dictionary its messages are
/// checked against, if any. Build it with [`SessionConfig::with_appl_ver_id`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ApplVersion {
    /// DefaultApplVerID(1137): the version, as sent and accepted at logon.
    pub id: ApplVerId,
    /// Checks this version's inbound application messages; see
    /// [`SessionConfig::with_dictionary`].
    #[cfg(feature = "validation")]
    pub validator: Option<Arc<crate::validation::Validator>>,
}

/// Why a configuration can't run a session: what [`SessionConfig::check`] and the other
/// configurations' `check`s find.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

impl From<String> for ConfigError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

/// Settings shared by both roles.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SessionConfig {
    /// BeginString(8), e.g. `FIX.4.4`, or `FIXT.1.1` with [`with_appl_ver_id`](Self::with_appl_ver_id).
    pub begin_string: String,
    /// Our CompID.
    pub sender_comp_id: String,
    /// How long to wait for Logon (acceptor) or the Logon reply (initiator).
    pub logon_timeout: Duration,
    /// How long to wait for the counterparty's Logout reply before disconnecting.
    pub logout_timeout: Duration,
    /// When the session may be logged on, and when its sequence numbers reset. `None` (the
    /// default) means always, with sequence numbers reset only on request.
    pub schedule: Option<SessionSchedule>,
    /// Wall-clock time for the schedule, for store creation times, for checking inbound
    /// SendingTime and for stamping outbound SendingTime. Read at most once per call into the
    /// session. Replace it in tests.
    pub clock: Clock,
    /// How far an inbound message's SendingTime(52) may be from `clock`, either way. A message
    /// further off is rejected (SessionRejectReason 10) and the session logs out; a Logon is
    /// refused. 120 seconds by default, as QuickFIX's MaxLatency; `None` turns the check off.
    pub max_latency: Option<Duration>,
    /// Require OrigSendingTime(122) on messages with PossDupFlag(43)=Y, no later than their
    /// SendingTime: a missing one is rejected (1), and a later one rejected (10) and the session
    /// logged out. Gap fills are exempt. On by default.
    pub check_orig_sending_time: bool,
    /// Reject a message with a header field after a body field (14). On by default.
    pub check_header_order: bool,
    /// How SendingTime(52) is written, on resends and gap fills too: milliseconds by default.
    /// A resend's OrigSendingTime(122) is copied as it was first sent.
    pub timestamp_precision: Precision,
    /// The data fields: each is as long as the Length field before it says, and may contain SOH
    /// and bytes that aren't UTF-8. The standard ones by default; add a venue's own with
    /// [`with_data_field`](Self::with_data_field).
    pub data_fields: DataFields,
    /// Most application messages queued through [`SessionHandle::send`](crate::SessionHandle::send)
    /// and not yet taken by the connection: past it, `send` hands the message back
    /// ([`SendError::Full`](crate::SendError::Full)) rather than queue it without limit. One more
    /// may wait while the [`outbound_limit`](Self::outbound_limit)'s window is full: the
    /// connection holds the first to arrive, to know that one waits. 10,000 by default.
    pub send_queue: usize,
    /// At most this many messages to ask for in one ResendRequest, for a counterparty that
    /// caps them: a gap is then asked for a chunk at a time, the next once the last has arrived,
    /// and the last chunk to the end (EndSeqNo 0). `None` (the default) asks for the whole gap at
    /// once.
    pub resend_request_chunk: Option<u64>,
    /// At most this many application messages sent per window (see [`RateLimit`]).
    /// [`SessionHandle::send`](crate::SessionHandle::send)s beyond it wait in the send queue
    /// until the window allows them, so a full queue hands them back as usual. Replies the
    /// application makes in [`Application::on_message`] can't wait: they go out at once but
    /// count, so they can take a window past the limit, and while they alone fill it, queued
    /// sends wait. Admin messages, the BusinessMessageRejects the session sends itself, and
    /// resends neither count nor wait; nor does a message the session refuses to send. A message
    /// counts when the session frames it, not when it's written, and each connection starts with
    /// an empty window. `None` (the default) means no limit.
    ///
    /// [`SessionHandle::logout`](crate::SessionHandle::logout) waits for the sends queued before
    /// it, which leave at the limit's rate. A shutdown, or a logout the session starts itself (a
    /// schedule's end, the counterparty's Logout), drops them instead.
    ///
    /// The first time sends wait on a connection, Turbojet's connection driver logs a warning;
    /// after that they wait quietly, and `turbojet_throttled_total` counts them (see
    /// [`telemetry`](crate::telemetry)).
    pub outbound_limit: Option<RateLimit>,
    /// At most this many application messages received per window, counted on each connection,
    /// and what happens to the rest: [`InboundLimit::Delay`] stops reading until the window
    /// allows more, and [`InboundLimit::Reject`] answers them with a BusinessMessageReject. Admin
    /// messages don't count. `None` (the default) means no limit.
    ///
    /// With `Delay`, every application message read counts as it arrives, resends and messages
    /// queued ahead of a gap included, as do messages the session itself refuses (a Reject(3),
    /// say). The message that fills the window is handled, then nothing more is processed, and
    /// the transport isn't read, until the window frees up a whole window after the oldest
    /// message in it. The counterparty's input waits, in order, unread, and TCP slows it down;
    /// nothing is sent to say so. So:
    ///
    /// - Admin messages wait too, behind the input before them. A Heartbeat or an answer to our
    ///   TestRequest is handled only once the hold ends, so the time held doesn't count as the
    ///   counterparty's silence, nor as a lack of progress in a resend we asked for. With a
    ///   window much longer than HeartBtInt, a counterparty that has died is noticed up to a
    ///   window late.
    /// - A Logout or ResendRequest from the counterparty behind held input waits up to a window,
    ///   so its own logout or resend timeout may fire first.
    /// - A counterparty that closes the connection meanwhile is noticed once the hold ends.
    ///
    /// With `Reject`, an application message that arrives while the window is full isn't handed
    /// to the application: it's answered with a BusinessMessageReject(j) whose RefSeqNum(45) is
    /// its MsgSeqNum, BusinessRejectReason(380) 0 (Other) and Text(58) "throttle limit
    /// exceeded". It still takes its sequence number, as when the application rejects a message,
    /// so the counterparty doesn't resend it. Rejected messages don't count, so a counterparty
    /// sending too fast can't keep the window full; nor do messages the session refuses before
    /// handing them over (failing validation, say). Recovery we asked for counts but is never
    /// rejected, so it isn't lost: any message that arrives while a ResendRequest of ours is
    /// outstanding, and those that arrived ahead of the gap it fills, handled once it's filled.
    /// New traffic after it waits for the window. PossDupFlag(43)=Y alone isn't recovery: it's
    /// only the sender's claim.
    ///
    /// Which to use: `Reject` protects against a fast but well-behaved counterparty. A hostile
    /// one controls its own gaps, so it can push one gap's worth of messages through as the
    /// "resend" (they count, so the window then turns new traffic away). `Delay` holds against
    /// a hostile counterparty too, since everything it sends is paced as it's read. Under a real
    /// flood `Delay` is also the cheaper: `Reject` stores and writes a BusinessMessageReject for
    /// each message over the limit.
    ///
    /// The first time the limit is reached on a connection, it logs a warning; after that,
    /// `turbojet_throttled_total` counts holds and rejects (see [`telemetry`](crate::telemetry)),
    /// and each reject is logged at DEBUG only, so a flood doesn't flood the log.
    pub inbound_limit: Option<InboundLimit>,
    /// Cancel on disconnect: when a session that logged on ends in a way the trigger counts, the
    /// application is told to cancel the session's orders, through
    /// [`on_cancel_on_disconnect`](Application::on_cancel_on_disconnect), unless the counterparty
    /// logs back on within the grace period, which stops the countdown. Our own shutdown and the
    /// schedule's end never count. `None` (the default) means off; an acceptor sets it per
    /// counterparty through [`Counterparties`].
    pub cancel_on_disconnect: Option<CancelOnDisconnect>,
    /// FIXT.1.1 only: the application versions (DefaultApplVerID(1137)) this session supports; see
    /// [`with_appl_ver_id`](Self::with_appl_ver_id).
    pub appl_versions: Vec<ApplVersion>,
    /// FIX 4.x sessions: checks inbound application messages against a data dictionary before
    /// delivering them (feature `validation`); see [`with_dictionary`](Self::with_dictionary).
    /// FIXT sessions have one per application version instead.
    #[cfg(feature = "validation")]
    pub validator: Option<Arc<crate::validation::Validator>>,
    /// Records the latency histograms (feature `metrics`): the time to handle each inbound message,
    /// to commit, and from reading input to its replies being ready to write; see
    /// [`telemetry`](crate::telemetry#latency-histograms). Off by default: they cost a clock read
    /// per inbound message, and a few per batch.
    #[cfg(feature = "metrics")]
    pub latency_metrics: bool,
}

impl SessionConfig {
    /// A configuration for BeginString `begin_string` with our CompID `sender_comp_id`: 10 seconds
    /// to log on and 5 to log out, no schedule, the system clock, SendingTime within 120 seconds,
    /// OrigSendingTime and header order checked, a send queue of 10,000, no rate limits and no cancel
    /// on disconnect.
    pub fn new(begin_string: impl Into<String>, sender_comp_id: impl Into<String>) -> Self {
        Self {
            begin_string: begin_string.into(),
            sender_comp_id: sender_comp_id.into(),
            logon_timeout: Duration::from_secs(10),
            logout_timeout: Duration::from_secs(5),
            schedule: None,
            clock: Clock::system(),
            max_latency: Some(Duration::from_secs(120)),
            check_orig_sending_time: true,
            check_header_order: true,
            timestamp_precision: Precision::Millis,
            data_fields: DataFields::standard(),
            send_queue: 10_000,
            resend_request_chunk: None,
            outbound_limit: None,
            inbound_limit: None,
            cancel_on_disconnect: None,
            appl_versions: Vec::new(),
            #[cfg(feature = "validation")]
            validator: None,
            #[cfg(feature = "metrics")]
            latency_metrics: false,
        }
    }

    /// Adds a venue's data field `data_tag`, whose length in bytes is given by Length field
    /// `length_tag` just before it. The standard data fields, such as RawData(96), are known
    /// already.
    ///
    /// ```
    /// # use turbojet::SessionConfig;
    /// let config = SessionConfig::new("FIX.4.4", "VENUE").with_data_field(5000, 5001);
    /// ```
    #[must_use]
    pub fn with_data_field(mut self, length_tag: u32, data_tag: u32) -> Self {
        self.data_fields = self.data_fields.with(length_tag, data_tag);
        self
    }

    /// FIXT.1.1 sessions: supports application version `id`. An initiator sends its first as
    /// DefaultApplVerID(1137); an acceptor accepts any version configured this way, refusing
    /// others, and echoes the counterparty's choice. Either side then accepts, and may send,
    /// messages that name any of them in ApplVerID(1128), each checked against its own version's
    /// dictionary.
    #[cfg_attr(
        feature = "validation",
        doc = "[`with_dictionary`](Self::with_dictionary) and [`with_validator`](Self::with_validator) after this",
        doc = "apply to this version's messages."
    )]
    /// The engine owns DefaultApplVerID(1137) in Logon, so don't change it in
    /// [`Application::to_admin`].
    ///
    /// ```
    /// # use turbojet::{SessionConfig, ApplVerId};
    /// let config = SessionConfig::new("FIXT.1.1", "VENUE")
    ///     .with_appl_ver_id(ApplVerId::Fix50Sp2)
    ///     .with_appl_ver_id(ApplVerId::Fix50Sp1);
    /// ```
    #[must_use]
    pub fn with_appl_ver_id(mut self, id: ApplVerId) -> Self {
        self.appl_versions.push(ApplVersion {
            id,
            #[cfg(feature = "validation")]
            validator: None,
        });
        self
    }

    /// Whether BeginString is FIXT (`FIXT.1.1`), whose sessions negotiate an application version.
    pub fn is_fixt(&self) -> bool {
        self.begin_string.starts_with("FIXT.")
    }

    /// Why this configuration can't run a session, if it can't.
    ///
    /// # Errors
    ///
    /// The first problem found: a BeginString that can't be sent (empty, or containing SOH or
    /// `=`; any other, a venue's own included, is fine), a rate limit out of bounds (see [`RateLimit`]), a cancel grace over
    /// [`MAX_CANCEL_GRACE`], a resend request chunk of 0, or a FIXT session without an
    /// application version, or with one twice, or a FIX 4.x session with one.
    pub fn check(&self) -> Result<(), ConfigError> {
        self.first_problem().map_err(ConfigError)
    }

    /// [`check`](Self::check), as text, for checks that build on it.
    pub(crate) fn first_problem(&self) -> Result<(), String> {
        if self.begin_string.is_empty() || self.begin_string.contains(['\x01', '=']) {
            return Err(format!(
                "begin_string {:?} can't be sent: it's empty, or has SOH or '=' in it",
                self.begin_string
            ));
        }
        if self.resend_request_chunk == Some(0) {
            return Err("resend_request_chunk must be at least 1".into());
        }
        if let Some(limit) = &self.outbound_limit {
            limit.check().map_err(|e| format!("outbound_limit: {e}"))?;
        }
        if let Some(InboundLimit::Delay(limit) | InboundLimit::Reject(limit)) = &self.inbound_limit {
            limit.check().map_err(|e| format!("inbound_limit: {e}"))?;
        }
        if let Some(CancelOnDisconnect { grace, .. }) = self.cancel_on_disconnect
            && grace > MAX_CANCEL_GRACE
        {
            return Err(format!("cancel_on_disconnect: grace {grace:?} is over {MAX_CANCEL_GRACE:?}"));
        }
        let versions = self.appl_versions.len();
        if !self.is_fixt() {
            return match versions {
                0 => Ok(()),
                _ => Err(format!("application versions need a FIXT BeginString, not '{}'", self.begin_string)),
            };
        }
        for (i, version) in self.appl_versions.iter().enumerate() {
            if self.appl_versions[..i].iter().any(|v| v.id == version.id) {
                return Err(format!("application version {:?} added twice", version.id));
            }
        }
        #[cfg(feature = "validation")]
        if self.validator.is_some() {
            return Err("with_dictionary or with_validator called before with_appl_ver_id on a FIXT session".into());
        }
        match versions {
            0 => Err("a FIXT session needs an application version: call with_appl_ver_id".into()),
            _ => Ok(()),
        }
    }

    /// Panics with [`check`](Self::check)'s message if this configuration is invalid.
    pub(crate) fn assert_valid(&self) {
        if let Err(e) = self.first_problem() {
            panic!("invalid session configuration: {e}");
        }
    }

    /// Checks inbound application messages against `dictionary`, with every check on: one that
    /// fails is answered with a session Reject and not delivered. After
    /// [`with_appl_ver_id`](Self::with_appl_ver_id), it applies to that version's messages. See
    /// [`crate::validation`]. The dictionary's data fields are added to
    /// [`data_fields`](Self::data_fields).
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn with_dictionary(self, dictionary: &turbojet_dictionary::Dictionary) -> Self {
        self.with_validator(crate::validation::Validator::new(dictionary))
    }

    /// Checks inbound application messages with `validator`, e.g. one with some checks off. After
    /// [`with_appl_ver_id`](Self::with_appl_ver_id), it applies to that version's messages. Its
    /// data fields are added to [`data_fields`](Self::data_fields).
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn with_validator(mut self, validator: crate::validation::Validator) -> Self {
        for &(length_tag, data_tag) in validator.data_fields() {
            self.data_fields = self.data_fields.with(length_tag, data_tag);
        }
        let validator = Some(Arc::new(validator));
        match self.appl_versions.last_mut() {
            Some(version) => version.validator = validator,
            None => self.validator = validator,
        }
        self
    }
}

#[derive(Debug, Clone)]
enum Role {
    /// Waits for the counterparty's Logon and learns its CompID from it.
    Acceptor,
    /// Sends Logon on connect to a known counterparty.
    Initiator {
        target_comp_id: String,
        heartbeat: Duration,
        reset_on_logon: bool,
        next_expected: bool,
        username: Option<String>,
        password: Option<Secret>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    AwaitingLogon,
    Active,
    LoggingOut { since: Instant },
    Closed,
}

struct Peer {
    id: SessionId,
    /// Given to the application's callbacks.
    handle: SessionHandle,
    log: Box<dyn SessionLog>,
    heartbeat: Duration,
    metrics: SessionMetrics,
}

/// What an acceptor needs from a well-formed Logon. Whether its HeartBtInt and
/// DefaultApplVerID are acceptable depends on the counterparty (see
/// [`Session::logon_terms`]).
struct LogonRequest {
    comp_id: String,
    heartbeat: Duration,
    seq_num: u64,
    their_next: Option<u64>,
    /// FIXT sessions: their DefaultApplVerID(1137).
    appl_ver_id: Option<ApplVerId>,
}

/// Most messages kept while waiting for a gap to be filled. Beyond it they're dropped, and come
/// back with the resend, which asks for everything from the gap on.
const MAX_QUEUED: usize = 10_000;

/// Sequence numbers resent per step of a replay (see [`Session::on_resume`]), so answering a
/// ResendRequest holds at most this many stored messages, decoded and framed, at a time: about
/// 77 KB of typical ExecutionReports.
///
/// Measured resending 100,000 messages to a client over localhost (Apple M3, 2026-10-01): 64.2 ms
/// in steps of 64, 57.7 ms of 256, 56.8 ms of 1,024 and 60.1 ms all at once. Steps start the
/// client reading sooner; below about 256, the write per step starts to cost. The session's own
/// work doesn't depend on the step: 51 ms from memory, 112 ms from disk, at every size.
const MAX_RESEND_BATCH: u64 = 256;

/// Incoming application messages handed over in one in-flight window. Opening a window costs a
/// commit before its first message is handed over, and after a crash every redelivered message in
/// it is marked as possibly handled: so the window bounds both how often a busy session waits for
/// the store and how many messages a crash marks needlessly. Matches the most commands a driver
/// takes in one batch.
pub(crate) const DELIVERIES_PER_COMMIT: u64 = 256;

/// An operator's reply, held until the change it reports is committed.
type HeldReply =
    (tokio::sync::oneshot::Sender<Result<SequenceNumbers, SequenceError>>, Result<SequenceNumbers, SequenceError>);

/// A resend in progress: what's left of the range asked for.
struct Replay {
    /// The first sequence number not yet resent or gap-filled: an open gap fill starts here.
    next: u64,
    /// The first sequence number not yet looked up in the store.
    scan: u64,
    /// The last sequence number to resend.
    end: u64,
    /// The store's read of this step, while it's under way: the step resends up to `to`.
    fetch: Option<Fetching>,
}

/// A log the store opens with a job, and what logon does once it's open.
struct OpeningLog {
    id: SessionId,
    heartbeat: Duration,
    then: AfterOpen,
    /// The job, until the driver takes it.
    job: Option<Job<Box<dyn SessionLog>>>,
}

/// The rest of logon, once the session's log is open.
#[derive(Debug, Clone, Copy)]
enum AfterOpen {
    /// Initiator: send our Logon, resetting first with `reset`.
    SendLogon { reset: bool },
    /// Acceptor: answer the counterparty's Logon, which asked to reset with `reset`, came with
    /// MsgSeqNum `seq_num`, and NextExpectedMsgSeqNum `their_next` if any.
    AnswerLogon { reset: bool, seq_num: u64, their_next: Option<u64>, heartbeat: Duration },
}

/// A read of stored messages for a step of a resend that the store returned as a job.
struct Fetching {
    /// The last sequence number the step covers.
    to: u64,
    /// The job, until the driver takes it.
    job: Option<Job<SentMessages>>,
}

/// A message that arrived ahead of a gap, kept until its turn.
struct Queued {
    msg: Message,
    /// A ResendRequest answered on arrival: its turn only takes its number.
    answered: bool,
}

/// An outstanding ResendRequest.
struct Resend {
    /// The MsgSeqNum that revealed the gap: the resend is complete once it's been received.
    target: u64,
    /// When `next_incoming` last advanced (or the request was sent).
    progress_at: Instant,
    /// `next_incoming` at `progress_at`.
    seen: u64,
    /// Whether it has been sent again after a timeout.
    retried: bool,
    /// The last MsgSeqNum asked for, if not to the end: once it's been received, the next chunk
    /// is (see [`SessionConfig::resend_request_chunk`]).
    asked_through: Option<u64>,
}

/// One FIX session over one connection.
pub struct Session {
    config: SessionConfig,
    role: Role,
    registry: Arc<SessionRegistry>,
    app: Arc<dyn Application>,
    commands: CommandSender,
    /// FIXT sessions: the application version in use, and its validator. The initiator's is
    /// configured; the acceptor's is chosen from the counterparty's Logon.
    appl_version: Option<ApplVersion>,
    /// The transport, as reported by the driver; shown to the application at logon.
    connection: ConnectionInfo,
    /// Acceptor: decides each counterparty's settings at Logon; see [`Counterparties`].
    counterparties: Option<Arc<dyn Counterparties>>,
    /// Acceptor: the HeartBtInt(108) a counterparty may ask for, once its settings are known.
    heartbeat_range: RangeInclusive<Duration>,
    status: Status,
    /// Set once the session is bound to its log: on accepting Logon, or on connect as initiator.
    peer: Option<Peer>,
    /// Whether the application has been told about the logon (and so is owed an `on_logout`).
    app_logged_on: bool,
    /// Why we started to log out, for `on_logout` once it ends; `None` until then.
    ending: Option<Disconnect>,
    /// Whether the session ever reached the logged-on state.
    ever_logged_on: bool,
    logon_deadline_from: Instant,
    last_sent: Instant,
    last_received: Instant,
    test_request_sent: Option<Instant>,
    /// The outstanding ResendRequest, while resending.
    resend: Option<Resend>,
    /// Messages that arrived ahead of a gap, by MsgSeqNum.
    queued: BTreeMap<u64, Queued>,
    /// The store failed: nothing more is saved, and the session is closing.
    store_failed: bool,
    /// The schedule period the session logged on in, if it has a schedule.
    period: Option<Period>,
    test_req_counter: u64,
    /// Encoded messages for the driver to write, in order; see [`output`](Self::output). The driver
    /// empties it after each wake-up, so it holds the replies to one read, one batch of commands,
    /// or one step of a resend.
    output: Vec<u8>,
    /// How much of `output` the store has committed: only that may be written.
    committed: usize,
    /// A commit the store returned is under way: nothing more is fed in until it ends.
    committing: bool,
    /// Incoming messages before this may be handed to the application: the end of the in-flight
    /// window the store last committed. Each commit while receiving records one, starting at the
    /// next incoming number, for the batch after it.
    window_end: Option<u64>,
    /// The start of the window the commit under way opens.
    opening: Option<u64>,
    /// The window the store had in flight when the session was bound: the messages in it may have
    /// been handled before a restart.
    recovered: Option<u64>,
    /// Messages that waited for a gap are in sequence, but wait for a window to be handled in.
    drain_waiting: bool,
    /// Operator replies, sent once the changes they report are committed. At most one per
    /// command, and commands are taken in batches between commits.
    replies: Vec<HeldReply>,
    /// Receipts for messages stored, answered once they're committed. At most one per command.
    receipts: Vec<(ReceiptSender, u64)>,
    /// The resend in progress, if any; see [`on_resume`](Self::on_resume).
    replay: Option<Replay>,
    /// The session's log, while the store opens it with a job: logon waits for it.
    opening_log: Option<OpeningLog>,
    /// New messages sent while a resend is in progress, framed and stored, to follow it. Drivers
    /// feed no input meanwhile, so it holds what the call that started the resend went on to
    /// send: replies to one message, to those queued behind a gap, or the commands held during
    /// logon.
    held: Vec<u8>,
    /// Sequence numbers resent per step: `MAX_RESEND_BATCH`, a field so tests can shrink it.
    resend_batch: u64,
    /// Handle commands received while logon is in progress, applied in order once it completes.
    /// Turbojet's drivers take sends only once logged on, operator commands are answered at once,
    /// and only the first Logout is kept, so with them it holds one command at most. A driver of
    /// its own that gives sends before logon bounds it by how many it gives.
    pending: Vec<Command>,
    /// The times of the last application messages sent, under `config.outbound_limit`; see
    /// [`can_send`](Self::can_send). A new connection starts with an empty window.
    outbound: Option<Window>,
    /// The times of the last application messages received, under `config.inbound_limit`, and
    /// what happens to one over it; see [`deliver`](Self::deliver). A new connection starts with
    /// an empty window.
    inbound: Option<Inbound>,
    /// Whether this connection has warned that sends wait for the outbound window. It warns once,
    /// not once per send, which a flood would turn into a flood of warnings.
    outbound_warned: bool,
    /// Whether this connection has warned that the inbound limit was reached, once, as for
    /// `outbound_warned`.
    inbound_warned: bool,
    /// Scratch space for [`frame_into`](Self::frame_into)'s header, kept to reuse its allocation.
    header: String,
    /// Scratch space for the stored copy of a message, when it differs from the one sent.
    stored: Vec<u8>,
    /// What the application sends from `on_message`, and spare messages to build its replies in,
    /// kept to reuse their allocations.
    outbox: Outbox,
    /// `config.clock`'s time, read at most once per call into the session (see
    /// [`wall_clock`](Self::wall_clock)).
    wall_clock: Cell<Option<DateTime<Utc>>>,
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("id", &self.peer.as_ref().map(|peer| &peer.id))
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl Session {
    /// A session that waits for a counterparty's Logon. The receiver carries
    /// [`SessionHandle`] commands and must be fed to [`Session::on_command`].
    ///
    /// # Panics
    ///
    /// If `config` is invalid; see [`SessionConfig::check`].
    pub fn acceptor(
        config: SessionConfig,
        registry: Arc<SessionRegistry>,
        app: Arc<dyn Application>,
        now: Instant,
    ) -> (Self, CommandReceiver) {
        Self::new(config, Role::Acceptor, registry, app, now)
    }

    /// A session that logs on to `config.target_comp_id` when [`Session::on_connect`] is called.
    ///
    /// # Panics
    ///
    /// If `config` is invalid; see [`InitiatorConfig::check`].
    pub fn initiator(
        config: &InitiatorConfig,
        registry: Arc<SessionRegistry>,
        app: Arc<dyn Application>,
        now: Instant,
    ) -> (Self, CommandReceiver) {
        config.assert_valid();
        let role = Role::Initiator {
            target_comp_id: config.target_comp_id.clone(),
            heartbeat: config.heartbeat_interval,
            reset_on_logon: config.reset_on_logon,
            next_expected: config.next_expected_msg_seq_num,
            username: config.username.clone(),
            password: config.password.clone(),
        };
        Self::new(config.session.clone(), role, registry, app, now)
    }

    fn new(
        config: SessionConfig,
        role: Role,
        registry: Arc<SessionRegistry>,
        app: Arc<dyn Application>,
        now: Instant,
    ) -> (Self, CommandReceiver) {
        let initiator = matches!(role, Role::Initiator { .. });
        config.assert_valid();
        let appl_version = if initiator { config.appl_versions.first().cloned() } else { None };
        let (commands, receiver) = command_queues(config.send_queue);
        let outbound = config.outbound_limit.map(Window::new);
        let inbound = config.inbound_limit.map(Inbound::new);
        let session = Self {
            config,
            role,
            registry,
            app,
            commands,
            appl_version,
            connection: ConnectionInfo::default(),
            counterparties: None,
            heartbeat_range: Counterparty::DEFAULT_HEARTBEAT,
            status: Status::AwaitingLogon,
            peer: None,
            app_logged_on: false,
            ending: None,
            ever_logged_on: false,
            logon_deadline_from: now,
            last_sent: now,
            last_received: now,
            test_request_sent: None,
            resend: None,
            queued: BTreeMap::new(),
            store_failed: false,
            period: None,
            test_req_counter: 0,
            output: Vec::new(),
            committed: 0,
            committing: false,
            window_end: None,
            opening: None,
            recovered: None,
            drain_waiting: false,
            replies: Vec::new(),
            receipts: Vec::new(),
            replay: None,
            opening_log: None,
            held: Vec::new(),
            resend_batch: MAX_RESEND_BATCH,
            pending: Vec::new(),
            outbound,
            inbound,
            outbound_warned: false,
            inbound_warned: false,
            header: String::new(),
            wall_clock: Cell::new(None),
            stored: Vec::new(),
            outbox: Outbox::default(),
        };
        (session, receiver)
    }

    /// Describes the transport (remote address, TLS peer certificates) for
    /// [`Application::verify_logon`]. Set it before feeding the session any messages.
    pub fn set_connection_info(&mut self, connection: ConnectionInfo) {
        self.connection = connection;
    }

    /// Acceptor: decides each counterparty's settings at Logon, in place of the configuration
    /// the session was made with; see [`Counterparties`]. Set it before feeding the session any
    /// messages. An initiator ignores it.
    pub fn set_counterparties(&mut self, counterparties: Arc<dyn Counterparties>) {
        self.counterparties = Some(counterparties);
    }

    /// The data fields to decode inbound messages with.
    pub(crate) fn data_fields(&self) -> &DataFields {
        &self.config.data_fields
    }

    /// The session's ID, once known.
    pub fn session_id(&self) -> Option<&SessionId> {
        self.peer.as_ref().map(|p| &p.id)
    }

    /// Whether the session is logged on: Logon exchanged, and logout not yet started.
    pub fn is_logged_on(&self) -> bool {
        self.status == Status::Active
    }

    /// Whether the session has ended: write what's in [`output`](Self::output), then close the
    /// connection. It ignores further events.
    pub fn is_closed(&self) -> bool {
        self.status == Status::Closed
    }

    /// Encoded messages to write to the counterparty, in order, since the last
    /// [`clear_output`](Self::clear_output): it grows until cleared. It holds only what the store
    /// has committed, so call [`take_commit`](Self::take_commit) after each call into the session.
    /// Once [`is_closed`](Self::is_closed), write them, then close the connection.
    pub fn output(&self) -> &[u8] {
        debug_assert!(self.committed <= self.output.len(), "only output that exists is committed");
        &self.output[..self.committed]
    }

    /// Empties [`output`](Self::output), keeping its capacity, once it's been written.
    pub fn clear_output(&mut self) {
        debug_assert!(self.committed <= self.output.len(), "only output that exists is committed");
        self.output.drain(..self.committed);
        self.committed = 0;
    }

    /// The store's commit of what the session has done since the last one, which must be durable
    /// before any of it goes on the wire: call it after every call into the session. `None` when
    /// there's nothing to wait for (the store committed at once, as `MemoryStorage` and
    /// `DiskStorage` without fsync do), and [`output`](Self::output) then holds everything sent.
    /// Otherwise run the commit where blocking is harmless, feed the session nothing meanwhile,
    /// and hand its result to [`on_committed`](Self::on_committed).
    pub fn take_commit(&mut self, now: Instant) -> Option<Commit> {
        self.wall_clock.set(None);
        while !self.committing {
            match self.begin_commit() {
                Ok(Some(commit)) => {
                    self.committing = true;
                    return Some(commit);
                }
                // Committed: go round again only if that let messages waiting for it be handled.
                Ok(None) if self.finish_commit(now) => {}
                Ok(None) => break,
                Err(e) => {
                    self.opening = None;
                    self.storage_failed(e, now);
                    break;
                }
            }
        }
        None
    }

    /// A commit from [`take_commit`](Self::take_commit) has ended: if it succeeded, what it covers
    /// joins [`output`](Self::output), and the session can take input again; if not, the store
    /// has failed, so the session drops what wasn't committed and closes. Call
    /// [`take_commit`](Self::take_commit) again after it.
    pub fn on_committed(&mut self, result: io::Result<()>, now: Instant) {
        self.wall_clock.set(None);
        debug_assert!(self.committing, "a commit was taken");
        self.committing = false;
        match result {
            Ok(()) => {
                self.finish_commit(now);
            }
            Err(e) => {
                self.opening = None;
                self.storage_failed(e, now);
            }
        }
    }

    /// The store's read of the messages for the next step of a resend, if it returned one as a
    /// job (see [`SessionLog::fetch`]): run it off the async runtime, feed the session nothing
    /// meanwhile, and hand its result to [`on_fetched`](Self::on_fetched). Stores that read at
    /// once never return one. Call it after [`take_commit`](Self::take_commit) returns `None`.
    pub fn take_fetch(&mut self) -> Option<Job<SentMessages>> {
        self.replay.as_mut()?.fetch.as_mut()?.job.take()
    }

    /// A read from [`take_fetch`](Self::take_fetch) has ended: the resend step it was for goes
    /// into [`output`](Self::output), or, if it failed, the store has failed and the session
    /// closes. A resend that ended meanwhile (the session closed) ignores it. Call
    /// [`take_commit`](Self::take_commit) after it, as after any call into the session.
    pub fn on_fetched(&mut self, result: io::Result<SentMessages>, now: Instant) {
        self.wall_clock.set(None);
        let Some(fetch) = self.replay.as_mut().and_then(|replay| replay.fetch.take()) else {
            return;
        };
        debug_assert!(fetch.job.is_none(), "the read was taken");
        match result {
            Ok(stored) => self.resend_stored(stored, fetch.to, now),
            Err(e) => self.storage_failed(e, now),
        }
    }

    /// The store's opening of the session's log, if it returned one as a job (see
    /// [`SessionStorage::begin_open`](crate::store::SessionStorage::begin_open)), once logon has
    /// told the session its ID: run it off the async runtime, feed the session nothing
    /// meanwhile, and hand its result to [`on_opened`](Self::on_opened). Stores that open at once
    /// never return one. Call it after [`take_commit`](Self::take_commit) returns `None`.
    pub fn take_open(&mut self) -> Option<Job<Box<dyn SessionLog>>> {
        self.opening_log.as_mut()?.job.take()
    }

    /// An opening from [`take_open`](Self::take_open) has ended: logon goes on, or, if it failed,
    /// the session refuses it and closes. A session that closed meanwhile (shutdown) closes the
    /// log at once. Call [`take_commit`](Self::take_commit) after it, as after any call into the
    /// session.
    pub fn on_opened(&mut self, result: io::Result<Box<dyn SessionLog>>, now: Instant) {
        self.wall_clock.set(None);
        let Some(OpeningLog { id, heartbeat, then, job }) = self.opening_log.take() else {
            debug_assert!(false, "an opening was taken");
            return;
        };
        debug_assert!(job.is_none(), "the opening was taken");
        match result {
            Ok(log) if self.status == Status::Closed => {
                // Closed before the store opened it: close the log before releasing the claim.
                drop(log);
                self.registry.release(&id, &self.commands);
            }
            Ok(log) => self.bound(id, heartbeat, log, then, now),
            Err(e) => {
                warn!("refusing logon: cannot open session store for {id}: {e}");
                self.registry.release(&id, &self.commands);
                self.close(Disconnect::Error, now);
            }
        }
    }

    /// Whether the session waits for the store: a commit, a read for a resend or the opening of
    /// its log, taken or due to be. Feed it nothing until that ends.
    pub fn is_waiting_on_store(&self) -> bool {
        self.committing
            || self.opening_log.is_some()
            || self.replay.as_ref().is_some_and(|replay| replay.fetch.is_some())
    }

    /// Whether a commit from [`take_commit`](Self::take_commit) is under way.
    pub fn is_committing(&self) -> bool {
        self.committing
    }

    /// Whether the session can take the next message from the counterparty now. If not, input
    /// waits for a commit, under way or due from [`take_commit`](Self::take_commit): one that
    /// records the messages about to be handed to the application as in flight, so that a crash
    /// while they're handled is noticed when they're resent. A message fed in anyway is handled
    /// once the session has committed that itself, on the calling thread. Input also waits for
    /// a resend's read or the opening of the session's log (see
    /// [`is_waiting_on_store`](Self::is_waiting_on_store)), and while
    /// [`input_free_at`](Self::input_free_at) holds it, which needs the time.
    pub fn ready_for_input(&mut self) -> bool {
        if self.is_waiting_on_store() {
            return false;
        }
        // Otherwise the next commit records a window that covers it.
        !self.receiving() || self.covers(self.peer().log.next_incoming())
    }

    /// Whether an application message may be sent at `now` under
    /// [`outbound_limit`](SessionConfig::outbound_limit): true unless the window is full. Check it
    /// before taking each application send from the queue, and leave the send there if not. Once
    /// the session is logging out or closed it's always true: sends are dropped then, so holding
    /// them would only tell the application a window later.
    pub fn can_send(&self, now: Instant) -> bool {
        self.send_free_at().is_none_or(|at| at <= now)
    }

    /// When the outbound window frees up, if it's full; `None` if it isn't, if there's no
    /// [`outbound_limit`](SessionConfig::outbound_limit), or if the session isn't logged on (it
    /// may be logging out or closed; see [`can_send`](Self::can_send)). It may have passed: a
    /// driver holding sends because [`can_send`](Self::can_send) said no waits until then. It
    /// isn't part of [`next_deadline`](Self::next_deadline): only the driver knows whether sends
    /// are waiting, and with none, waking for the window would cost a wake-up per message at a
    /// steady rate near the limit.
    pub fn send_free_at(&self) -> Option<Instant> {
        if self.status != Status::Active {
            return None;
        }
        self.outbound.as_ref().and_then(Window::free_at_or_none)
    }

    /// Tells the session its driver has started holding application sends because
    /// [`can_send`](Self::can_send) said no. The first time on a connection, it logs a warning:
    /// a flood of sends waits quietly after that. The driver counts the sends that waited.
    pub(crate) fn on_sends_held(&mut self) {
        if std::mem::replace(&mut self.outbound_warned, true) {
            return;
        }
        if let Some(window) = &self.outbound {
            warn!("outbound rate limit {} reached: sends wait", window.limit());
        }
    }

    /// When the inbound window frees up, if it's full under an [`InboundLimit::Delay`]; `None` if
    /// it isn't, if there's no such limit, or if the session isn't logged on (one logging out
    /// just takes its input). Until then, feed the session no more input and don't read the
    /// transport, so the counterparty's input waits, in order, and TCP slows it down. It may have
    /// passed: input goes on once it has. The message that fills the window is handled; only the
    /// next one waits. A message fed in anyway is handled, and counts.
    ///
    /// Like [`send_free_at`](Self::send_free_at), it isn't part of
    /// [`next_deadline`](Self::next_deadline): the driver holding input wakes for it itself.
    pub fn input_free_at(&self) -> Option<Instant> {
        if self.status != Status::Active {
            return None;
        }
        self.hold_end()
    }

    /// When input held by a Delay limit goes on, if the window has filled since it last had room,
    /// whether or not that has passed. Whatever the counterparty sends meanwhile, its Heartbeats
    /// and answers to our TestRequests included, waits unread, so
    /// [`silent_from`](Self::silent_from) doesn't count the hold as silence.
    fn hold_end(&self) -> Option<Instant> {
        let inbound = self.inbound.as_ref().filter(|inbound| inbound.over == Over::Delay)?;
        inbound.window.free_at_or_none()
    }

    /// When silence since `since` (the last message received, our TestRequest, or the last
    /// progress of our ResendRequest) counts from:
    /// the end of a hold on input, if that's later, since what the counterparty sent meanwhile
    /// waits unread. While the hold lasts that's in the future, so no TestRequest goes out and
    /// none goes unanswered; after it, silence counts from its end. The window keeps that end
    /// until a message is next recorded, which also moves `last_received` past it.
    fn silent_from(&self, since: Instant) -> Instant {
        self.hold_end().map_or(since, |hold_end| since.max(hold_end))
    }

    /// Runs the store's work on this thread until none is left, the commits, resend reads and
    /// opening it returns as jobs: for drivers that may block (tests, tools).
    pub fn commit_blocking(&mut self, now: Instant) {
        loop {
            if let Some(commit) = self.take_commit(now) {
                self.on_committed(commit.run(), now);
            } else if let Some(fetch) = self.take_fetch() {
                self.on_fetched(fetch.run(), now);
            } else if let Some(open) = self.take_open() {
                self.on_opened(open.run(), now);
            } else {
                break;
            }
        }
    }

    /// Whether a resend is in progress: more of it is due once [`output`](Self::output) has been
    /// written. Until it ends, call [`on_resume`](Self::on_resume) rather than feed the session
    /// messages or commands, which would wait behind the rest of the resend.
    /// [`on_timer`](Self::on_timer) and [`on_shutdown`](Self::on_shutdown) still apply.
    pub fn is_resending(&self) -> bool {
        self.replay.is_some()
    }

    /// Resends the next step of the resend in progress, if any, into [`output`](Self::output). A
    /// long range is resent in steps so that it never has to be held, read from the store and
    /// framed, all at once.
    pub fn on_resume(&mut self, now: Instant) {
        self.wall_clock.set(None);
        if self.replay.is_some() && self.status != Status::Closed {
            self.resend_step(now);
        }
    }

    /// Resends `batch` sequence numbers per step rather than the usual number, so that fuzzing
    /// reaches resends of several steps with short inputs. Not part of the API.
    #[doc(hidden)]
    pub fn set_resend_batch(&mut self, batch: u64) {
        assert!(batch > 0, "a step resends at least one sequence number");
        self.resend_batch = batch;
    }

    /// Whether the session has logged on at any point, even if it has since logged out.
    pub fn has_logged_on(&self) -> bool {
        self.ever_logged_on
    }

    /// The transport is connected. An initiator sends Logon; an acceptor waits.
    pub fn on_connect(&mut self, now: Instant) {
        self.wall_clock.set(None);
        if let Role::Initiator { target_comp_id, heartbeat, reset_on_logon, .. } = self.role.clone() {
            self.start_logon(target_comp_id, heartbeat, reset_on_logon, now);
        }
    }

    /// The transport ended (closed by the other end, reset or failed) while the session was open.
    /// Does nothing once the session has closed. A session logging out ends as its logout would
    /// have: the counterparty closing instead of answering is no different from not answering.
    /// `now` is when the transport ended, which a cancel-on-disconnect countdown starts from. A
    /// driver that never calls this gets the same `on_logout` when it drops the session, with the
    /// countdown starting then.
    pub fn on_disconnect(&mut self, now: Instant) {
        self.wall_clock.set(None);
        if self.status != Status::Closed {
            info!("connection lost");
            self.close(self.ending.unwrap_or(Disconnect::ConnectionLost), now);
        }
    }

    /// A message was decoded from the transport.
    pub fn on_message(&mut self, msg: &Message, now: Instant) {
        self.wall_clock.set(None);
        if self.opening_log.is_some() {
            // Logon can't go on until the log is open, and `ready_for_input` holds input meanwhile,
            // so this is a driver's mistake.
            debug_assert!(false, "a message was fed while the session's log was opening");
            warn!(msg_type = %msg.msg_type(), "dropping a message received while the session's log is opening");
            return;
        }
        self.last_received = now;
        self.test_request_sent = None;
        if self.receiving() && !self.committing && !self.covers(self.peer().log.next_incoming()) {
            self.open_window_now(now);
        }
        match self.status {
            Status::AwaitingLogon => self.on_logon(msg, now),
            Status::Active | Status::LoggingOut { .. } => {
                if self.inbound.is_some() && self.status == Status::Active && !msg.msg_type().is_admin() {
                    self.count_arrival(now);
                }
                self.on_session_message(msg, now, true);
                self.after_incoming(now);
            }
            Status::Closed => {}
        }
        if let Some(peer) = &self.peer {
            peer.metrics.message_received();
        }
    }

    /// Carries out a command from a [`SessionHandle`].
    ///
    /// Commands that arrive while logon is in progress are queued and applied, in order, as soon
    /// as it completes; a Logout after the first, which would be ignored then, isn't kept. Once
    /// logout has started, sends are dropped and logged. With an
    /// [`outbound_limit`](SessionConfig::outbound_limit), give it sends only once
    /// [`has_logged_on`](Self::has_logged_on), and while [`can_send`](Self::can_send): a send
    /// before logon would go out past the window, and panics in debug builds.
    pub fn on_command(&mut self, command: Command, now: Instant) {
        self.wall_clock.set(None);
        // Operator requests are answered straight away, even mid-logon: they must not wait in the
        // queue for a logon that may never complete.
        let command = match command {
            Command::Sequence(request, reply) => {
                // A change is reported once it's committed; a failure or a query at once.
                match self.apply_sequence(request, now) {
                    result @ Ok(_) if request != SequenceCommand::Get => self.replies.push((reply, result)),
                    result => {
                        let _ = reply.send(result);
                    }
                }
                return;
            }
            other => other,
        };
        if self.status == Status::AwaitingLogon {
            // Pending commands are applied all at once at logon, past the outbound window. The
            // drivers take sends only once logged on, and a session never returns to awaiting
            // logon, so none lands here while a limit is set.
            debug_assert!(
                self.outbound.is_none() || !matches!(command, Command::Send(..)),
                "with an outbound limit, sends are taken only once logged on"
            );
            // Only the first Logout would act once logged on, and FIX sessions ignore Finish, so
            // neither repeats here: with no sends before logon, pending holds one command at most.
            match command {
                Command::Finish => self.apply_command(command, now),
                Command::Logout(_) if self.pending.iter().any(|c| matches!(c, Command::Logout(_))) => {
                    debug!("ignoring logout request: one is already queued for logon");
                }
                command => {
                    debug!(queued = self.pending.len() + 1, "logon in progress; queueing handle command");
                    self.pending.push(command);
                }
            }
        } else {
            self.apply_command(command, now);
        }
    }

    /// The application is shutting down: a logged-on session logs out with `text` (disconnecting
    /// on the counterparty's reply, or after the logout timeout), and one that hasn't logged on
    /// yet disconnects. A session already logging out carries on.
    pub fn on_shutdown(&mut self, text: Option<&str>, now: Instant) {
        self.wall_clock.set(None);
        match self.status {
            Status::Active => self.logout(text, Disconnect::Shutdown, now),
            Status::AwaitingLogon => {
                info!("shutting down before logon; disconnecting");
                self.close(Disconnect::Shutdown, now);
            }
            Status::LoggingOut { .. } | Status::Closed => {}
        }
    }

    fn apply_command(&mut self, command: Command, now: Instant) {
        match command {
            Command::Send(mut msg, receipt) => {
                // The driver takes a send only while the outbound window allows it (see
                // `on_command`); replies past it come through `Context`, not here.
                debug_assert!(
                    self.status != Status::Active || self.can_send(now),
                    "a send is applied only while the outbound window allows it"
                );
                let outcome = if self.status == Status::Active {
                    self.send_app(&mut msg, now)
                } else {
                    warn!(msg_type = %msg.msg_type(), "dropping message: session is logging out");
                    Err(Dropped::LoggingOut)
                };
                // The application may not be waiting to hear. A message stored is reported once
                // it's committed.
                match (receipt, outcome) {
                    (Some(receipt), Ok(seq)) => self.receipts.push((receipt, seq)),
                    (Some(receipt), outcome) => {
                        let _ = receipt.send(outcome);
                    }
                    (None, _) => {}
                }
            }
            Command::Logout(text) if self.status == Status::Active => {
                self.logout(text.as_deref(), Disconnect::Logout, now)
            }
            Command::Logout(_) => debug!("ignoring logout request: session is already logging out"),
            Command::Finish => debug!("ignoring a request to finish sending: FIX sessions don't"),
            Command::Sequence(..) => unreachable!("handled in on_command"),
        }
    }

    /// An operator change to this (bound) session's sequence numbers.
    fn apply_sequence(&mut self, request: SequenceCommand, now: Instant) -> Result<SequenceNumbers, SequenceError> {
        if self.peer.is_none() {
            return Err(SequenceError::Invalid("the session has not been identified yet".into()));
        }
        if self.store_failed && request != SequenceCommand::Get {
            // Nothing more is committed, so the change would never be stored.
            return Err(SequenceError::Storage(io::Error::other("the session store has failed")));
        }
        match request {
            SequenceCommand::Reset => return Err(SequenceError::Connected),
            SequenceCommand::SetNextOutgoing(seq)
                if self.status == Status::Active && seq > self.peer().log.next_outgoing() =>
            {
                // Tell the counterparty to expect `seq` next, then skip to it. In gap-fill mode, under
                // its own MsgSeqNum: a counterparty still filling a gap applies it in turn, after
                // what we sent before it, where reset mode would make it abandon the gap.
                info!(new_seq_no = seq, "sending SequenceReset (gap fill)");
                self.send(SequenceReset { gap_fill_flag: Some(true), new_seq_no: seq }.into(), now);
            }
            _ => {}
        }
        let clock = self.config.clock.clone();
        let result = apply_sequence_command(self.peer_mut().log.as_mut(), request, &clock);
        if let Ok(numbers) = &result {
            if let (SequenceCommand::SetNextIncoming(seq), Some(resend)) = (request, &mut self.resend) {
                if seq > resend.target {
                    self.resend = None;
                } else {
                    // A new baseline: progress is measured from here.
                    resend.seen = seq;
                    resend.progress_at = now;
                    resend.retried = false;
                    self.ask_next_chunk(seq, now);
                }
            }
            if request != SequenceCommand::Get {
                info!(?request, ?numbers, "sequence numbers changed by operator");
            }
            self.update_sequence_gauges();
        }
        // Queued messages the new number reaches are processed, and any it passes dropped. The
        // reply gives the numbers after that, since processing them may send messages.
        if result.is_ok() && matches!(request, SequenceCommand::SetNextIncoming(_)) {
            self.after_incoming(now);
            if let Some(peer) = &self.peer {
                let (next_incoming, next_outgoing) = (peer.log.next_incoming(), peer.log.next_outgoing());
                return Ok(SequenceNumbers { next_incoming, next_outgoing });
            }
        }
        result
    }

    /// Drops commands queued during a logon that never completed.
    fn discard_pending(&mut self) {
        if !self.pending.is_empty() {
            warn!(count = self.pending.len(), "discarding handle commands queued before logon: session ended");
            self.pending.clear();
        }
    }

    /// Drives timeouts and heartbeats. Call it at [`next_deadline`](Self::next_deadline), and at
    /// least once a second so the end of a scheduled session is noticed. Calling it early is
    /// harmless: it acts only on what is due.
    pub fn on_timer(&mut self, now: Instant) {
        self.wall_clock.set(None);
        if self.period_ended() {
            match self.status {
                Status::Active => {
                    info!("session period ended; logging out");
                    self.logout(Some("End of session"), Disconnect::Shutdown, now);
                }
                Status::AwaitingLogon => {
                    warn!("session period ended before logon completed");
                    self.close(Disconnect::Shutdown, now);
                }
                Status::LoggingOut { .. } | Status::Closed => {}
            }
        }
        match self.status {
            Status::AwaitingLogon => {
                if now.duration_since(self.logon_deadline_from) >= self.config.logon_timeout {
                    warn!("no Logon received within {:?}", self.config.logon_timeout);
                    self.close(Disconnect::Error, now);
                }
            }
            Status::LoggingOut { since } => {
                if now.duration_since(since) >= self.config.logout_timeout {
                    warn!("no Logout reply received; disconnecting");
                    self.close(self.ending.expect("LoggingOut is entered only through logout, which sets ending"), now);
                }
            }
            // The counterparty's input waits while we resend, and we're sending anyway: Heartbeats,
            // TestRequests and our own ResendRequest's timeout wait for the end of the resend.
            Status::Active if self.replay.is_some() => {}
            Status::Active => {
                self.check_heartbeats(now);
                if self.status == Status::Active {
                    self.check_resend(now);
                }
            }
            Status::Closed => {}
        }
    }

    /// When [`on_timer`](Self::on_timer) next has something to do: a logon or logout timeout, a
    /// Heartbeat or TestRequest falling due, or an unanswered ResendRequest. `None` once closed,
    /// and while [`is_resending`](Self::is_resending). While [`input_free_at`](Self::input_free_at)
    /// holds input, the counterparty's silence counts from when it frees up.
    ///
    /// Schedule boundaries aren't included, since they are wall-clock times; call `on_timer` at
    /// least once a second as well. A timeout too long to represent as an `Instant` (such as
    /// `Duration::MAX`) has no deadline.
    ///
    /// Every call to [`on_message`](Self::on_message), [`on_command`](Self::on_command) or
    /// `on_timer` can change it, so read it again after each. `on_timer` can leave it in the past,
    /// for example when [`Application::to_admin`] makes a Heartbeat unsendable; don't call
    /// `on_timer` again straight away, but leave it to the once-a-second call.
    pub fn next_deadline(&self) -> Option<Instant> {
        match self.status {
            Status::AwaitingLogon => self.logon_deadline_from.checked_add(self.config.logon_timeout),
            Status::LoggingOut { since } => since.checked_add(self.config.logout_timeout),
            Status::Active if self.replay.is_some() => None,
            Status::Active => {
                let interval = self.peer().heartbeat;
                let heartbeat = self.last_sent.checked_add(interval);
                let test_request = match self.test_request_sent {
                    Some(sent) => self.silent_from(sent).checked_add(interval),
                    None => self.silent_from(self.last_received).checked_add(probe_after(interval)),
                };
                let resend = self
                    .resend
                    .as_ref()
                    .and_then(|r| self.silent_from(r.progress_at).checked_add(resend_timeout(interval)));
                [heartbeat, test_request, resend].into_iter().flatten().min()
            }
            Status::Closed => None,
        }
    }

    /// Keep in step with [`next_deadline`](Self::next_deadline).
    fn check_heartbeats(&mut self, now: Instant) {
        let interval = self.peer().heartbeat;
        // Input held by a Delay limit isn't silence: see `silent_from`.
        match self.test_request_sent {
            Some(sent) if now.duration_since(self.silent_from(sent)) >= interval => {
                warn!("counterparty did not answer TestRequest; disconnecting");
                self.close(Disconnect::HeartbeatTimeout, now);
                return;
            }
            Some(_) => {}
            None if now.duration_since(self.silent_from(self.last_received)) >= probe_after(interval) => {
                self.test_req_counter += 1;
                let id = crate::fields::format_compact!("TEST{}", self.test_req_counter);
                self.send(TestRequest { test_req_id: id }.into(), now);
                self.test_request_sent = Some(now);
            }
            None => {}
        }
        if now.duration_since(self.last_sent) >= interval {
            self.send(Heartbeat { test_req_id: None }.into(), now);
        }
    }

    /// Sends an outstanding ResendRequest again if it has made no progress for two heartbeat
    /// intervals, once; after two more, logs out rather than wait for ever. Keep in step with
    /// [`next_deadline`](Self::next_deadline).
    fn check_resend(&mut self, now: Instant) {
        let timeout = resend_timeout(self.peer().heartbeat);
        let next = self.peer().log.next_incoming();
        let Some(progress_at) = self.resend.as_ref().map(|resend| resend.progress_at) else { return };
        // The resend we asked for may be waiting behind input a Delay limit holds.
        if now.duration_since(self.silent_from(progress_at)) < timeout {
            return;
        }
        let resend = self.resend.as_mut().expect("guarded by the let-else above");
        if resend.retried {
            let text = format!("ResendRequest from {next} unanswered");
            warn!("{text}; logging out");
            return self.logout(Some(&text), Disconnect::Error, now);
        }
        resend.retried = true;
        resend.progress_at = now;
        warn!(expected = next, "ResendRequest unanswered; requesting again");
        let target = resend.target;
        let asked_through = self.ask_resend(next, target, now);
        if let Some(resend) = &mut self.resend {
            resend.asked_through = asked_through;
        }
    }

    // ---- Logon ----

    /// Initiator: bind to the session log and send Logon.
    fn start_logon(&mut self, target_comp_id: String, heartbeat: Duration, reset: bool, now: Instant) {
        let id = self.session_id_for(target_comp_id);
        if let Some(reason) = self.outside_schedule() {
            warn!(session = %id, "not logging on: {reason}");
            return self.close(Disconnect::Shutdown, now);
        }
        self.bind(id, heartbeat, AfterOpen::SendLogon { reset }, now);
    }

    /// Initiator, bound: send Logon.
    fn send_logon(&mut self, reset: bool, now: Instant) {
        let Role::Initiator { heartbeat, .. } = self.role else { unreachable!("only initiators send Logon first") };
        if reset && let Err(e) = self.reset_store() {
            return self.storage_failed(e, now);
        }
        self.logon_deadline_from = now;
        info!("sending Logon");
        let mut logon = logon_message(heartbeat, reset, None, self.appl_ver_id());
        if let Role::Initiator { next_expected, username, password, .. } = &self.role {
            logon.next_expected_msg_seq_num = next_expected.then(|| self.peer().log.next_incoming());
            logon.username = username.as_deref().map(CompactString::from);
            logon.password = password.clone();
        }
        self.send(logon.into(), now);
    }

    fn on_logon(&mut self, msg: &Message, now: Instant) {
        if msg.msg_type() != MsgType::Logon {
            warn!("expected Logon, got MsgType '{}'; disconnecting", msg.msg_type());
            return self.close(Disconnect::Error, now);
        }
        match self.role {
            Role::Acceptor => self.accept_logon(msg, now),
            Role::Initiator { .. } => self.complete_logon(msg, now),
        }
    }

    /// Acceptor: validate the counterparty's Logon and reply.
    fn accept_logon(&mut self, msg: &Message, now: Instant) {
        let LogonRequest { comp_id, heartbeat, seq_num, their_next, appl_ver_id } =
            match self.validate_logon_request(msg) {
                Ok(v) => v,
                Err(reason) => {
                    warn!("refusing logon: {reason}");
                    return self.close(Disconnect::Error, now);
                }
            };
        let id = self.session_id_for(comp_id);
        let terms = self.take_counterparty(&id, msg).and_then(|()| self.logon_terms(heartbeat, appl_ver_id));
        let appl_version = match terms {
            Ok(version) => version,
            Err(reason) => {
                warn!(session = %id, "refusing logon: {reason}");
                return self.close(Disconnect::Error, now);
            }
        };
        if let Some(reason) = self.outside_schedule() {
            warn!(session = %id, "refusing logon: {reason}");
            return self.close(Disconnect::Error, now);
        }
        let handle = self.registry.handle(id.clone());
        let verdict = guarded("verify_logon", || self.app.verify_logon(&handle, msg, &self.connection));
        if let Err(reason) = verdict.unwrap_or_else(|| Err("verify_logon panicked".into())) {
            warn!(session = %id, "application refused logon: {reason}");
            return self.close(Disconnect::Error, now);
        }
        self.appl_version = appl_version;
        let reset = msg.flag(tags::RESET_SEQ_NUM_FLAG);
        self.bind(id, heartbeat, AfterOpen::AnswerLogon { reset, seq_num, their_next, heartbeat }, now);
    }

    /// Acceptor, bound: reply to the counterparty's Logon.
    fn answer_logon(&mut self, reset: bool, seq_num: u64, their_next: Option<u64>, heartbeat: Duration, now: Instant) {
        assert!(self.peer.is_some(), "a Logon is answered once its session is bound");
        assert_eq!(self.status, Status::AwaitingLogon, "a Logon is answered once");
        if reset && let Err(e) = self.reset_store() {
            return self.storage_failed(e, now);
        }
        let expected = self.peer().log.next_incoming();
        if seq_num < expected {
            self.logout(
                Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")),
                Disconnect::Error,
                now,
            );
            return self.close(Disconnect::Error, now);
        }
        // Our reply's MsgSeqNum: the most their 789 can be.
        let reply_seq = self.peer().log.next_outgoing();
        if !self.check_next_expected(their_next, reply_seq, now) {
            return;
        }
        // Answer 789 with ours: after their Logon, or where their resend should start.
        let our_next = their_next.map(|_| if seq_num == expected { expected + 1 } else { expected });
        self.send(logon_message(heartbeat, reset, our_next, self.appl_ver_id()).into(), now);
        if let Some(from) = their_next
            && from < reply_seq
        {
            self.resend(from, reply_seq - 1, now);
        }
        self.logged_on(seq_num, expected, their_next.is_some(), now);
    }

    /// Whether the counterparty's NextExpectedMsgSeqNum(789), if it sent one, is possible:
    /// between 1 and `limit`, the next MsgSeqNum we'll send. Logs out and closes if not.
    fn check_next_expected(&mut self, their_next: Option<u64>, limit: u64, now: Instant) -> bool {
        match their_next {
            Some(next) if next == 0 || next > limit => {
                let text =
                    format!("NextExpectedMsgSeqNum(789) too high, expecting at most {limit} but received {next}");
                self.logout(Some(&text), Disconnect::Error, now);
                self.close(Disconnect::Error, now);
                false
            }
            _ => true,
        }
    }

    /// Initiator: validate the counterparty's Logon reply.
    fn complete_logon(&mut self, msg: &Message, now: Instant) {
        if let Err(mismatch) = self.check_header(msg) {
            warn!("refusing Logon reply: {mismatch}");
            return self.close(Disconnect::Error, now);
        }
        if let Some(text) = self.sending_time_problem(msg) {
            warn!("refusing Logon reply: {text}");
            return self.close(Disconnect::Error, now);
        }
        let identifying = [tags::SENDER_COMP_ID, tags::TARGET_COMP_ID, tags::SENDING_TIME];
        if let Some(tag) = identifying.into_iter().find(|tag| msg.get(*tag).is_none_or(str::is_empty)) {
            warn!("refusing Logon reply: tag {tag} missing or empty");
            return self.close(Disconnect::Error, now);
        }
        let Ok(seq_num) = msg.field::<u64>(tags::MSG_SEQ_NUM) else {
            warn!("refusing Logon reply: MsgSeqNum(34) missing or invalid");
            return self.close(Disconnect::Error, now);
        };
        if let Some(defect) = msg.defect() {
            warn!("refusing Logon reply: {}", defect.text);
            return self.close(Disconnect::Error, now);
        }
        let reply = match msg.parse::<Logon>() {
            Ok(reply) => reply,
            Err(e) => {
                warn!("refusing Logon reply: {e}");
                return self.close(Disconnect::Error, now);
            }
        };
        if let Some(tag) = empty_field(msg) {
            warn!("refusing Logon reply: tag {tag} specified without a value");
            return self.close(Disconnect::Error, now);
        }
        if self.config.is_fixt() && reply.default_appl_ver_id != self.appl_ver_id() {
            let expected = self.appl_ver_id().map_or("none", ApplVerId::code);
            let received = reply.default_appl_ver_id.map_or("none", ApplVerId::code);
            warn!("refusing Logon reply: DefaultApplVerID(1137) must be '{expected}', not '{received}'");
            return self.close(Disconnect::Error, now);
        }
        let verdict = guarded("verify_logon", || self.app.verify_logon(&self.peer().handle, msg, &self.connection));
        if let Err(reason) = verdict.unwrap_or_else(|| Err("verify_logon panicked".into())) {
            warn!("application refused Logon reply: {reason}");
            return self.close(Disconnect::Error, now);
        }
        let expected = self.peer().log.next_incoming();
        if seq_num < expected {
            self.logout(
                Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")),
                Disconnect::Error,
                now,
            );
            return self.close(Disconnect::Error, now);
        }
        let their_next = reply.next_expected_msg_seq_num;
        // They've had our Logon, the last message we sent.
        let next_outgoing = self.peer().log.next_outgoing();
        if !self.check_next_expected(their_next, next_outgoing, now) {
            return;
        }
        if let Some(from) = their_next
            && from + 1 < next_outgoing
        {
            self.resend(from, next_outgoing - 2, now);
        }
        let we_sent_ours = matches!(self.role, Role::Initiator { next_expected: true, .. });
        self.logged_on(seq_num, expected, we_sent_ours && their_next.is_some(), now);
    }

    /// Common tail of logon for both roles, once `seq_num >= expected`. With `await_resend` (both
    /// Logons carried NextExpectedMsgSeqNum), a gap is left for the counterparty to fill: it
    /// resends from our 789 unasked, and if it doesn't, the next message out of sequence asks.
    fn logged_on(&mut self, seq_num: u64, expected: u64, await_resend: bool, now: Instant) {
        if self.status == Status::Closed {
            return;
        }
        assert!(self.peer.is_some(), "a session logs on bound to its log");
        self.status = Status::Active;
        info!(heartbeat = ?self.peer().heartbeat, "logged on");
        if seq_num > expected {
            if !await_resend {
                self.request_resend(expected, seq_num, now);
            }
        } else {
            self.set_next_incoming(expected + 1, now);
        }
        if self.status == Status::Active {
            self.ever_logged_on = true;
            self.app_logged_on = true;
            self.peer().metrics.logged_on();
            // A completed logon stops the countdown, not the claim, so a reconnect refused
            // after claiming the session doesn't. A cancel running now finishes first.
            self.registry.stop_cancel(&self.peer().id);
            guarded("on_logon", || self.app.on_logon(&self.peer().handle));
            // Commands sent before logon completed go out first; anything the application sends
            // from on_logon arrives through the command channel afterwards.
            for command in std::mem::take(&mut self.pending) {
                self.apply_command(command, now);
            }
        }
    }

    fn validate_logon_request(&self, msg: &Message) -> Result<LogonRequest, String> {
        self.check_begin_string(msg)?;
        // Session test case 1d.
        if let Some(text) = self.sending_time_problem(msg) {
            return Err(text);
        }
        if let Some(defect) = msg.defect() {
            return Err(defect.text.clone());
        }
        if let Some(tag) = empty_field(msg) {
            return Err(format!("tag {tag} specified without a value"));
        }
        if msg.get(tags::TARGET_COMP_ID) != Some(self.config.sender_comp_id.as_str()) {
            return Err(format!("TargetCompID(56) must be '{}'", self.config.sender_comp_id));
        }
        let comp_id = msg.get(tags::SENDER_COMP_ID).ok_or("SenderCompID(49) missing")?;
        let logon: Logon = msg.parse().map_err(|e| e.to_string())?;
        if logon.encrypt_method != EncryptMethod::None {
            return Err("only EncryptMethod(98)=0 is supported".into());
        }
        let seq_num = msg.field::<u64>(tags::MSG_SEQ_NUM).map_err(|e| e.to_string())?;
        let appl_ver_id = match self.config.is_fixt() {
            true => Some(logon.default_appl_ver_id.ok_or("DefaultApplVerID(1137) missing")?),
            false => None,
        };
        Ok(LogonRequest {
            comp_id: comp_id.to_string(),
            heartbeat: Duration::from_secs(logon.heart_bt_int),
            seq_num,
            their_next: logon.next_expected_msg_seq_num,
            appl_ver_id,
        })
    }

    /// Acceptor: takes the settings the [`Counterparties`] resolver gives counterparty `id`, if
    /// there is one, in place of the session's own. Fails if it refuses, panics, or gives
    /// settings that can't apply.
    fn take_counterparty(&mut self, id: &SessionId, logon: &Message) -> Result<(), String> {
        let Some(counterparties) = self.counterparties.clone() else { return Ok(()) };
        let resolved = guarded("resolve", || counterparties.resolve(&self.config, id, logon, &self.connection))
            .unwrap_or_else(|| Err("the counterparty resolver panicked".into()))?;
        resolved.first_problem(&self.config)?;
        if resolved.require_client_certificate && self.connection.peer_certificate().is_none() {
            return Err("a client certificate is required".into());
        }
        let Counterparty { config, heartbeat, .. } = resolved;
        debug_assert!(config.sender_comp_id == self.config.sender_comp_id && config.clock.same_as(&self.config.clock));
        // Nothing has been counted yet: admin messages don't count.
        self.outbound = config.outbound_limit.map(Window::new);
        self.inbound = config.inbound_limit.map(Inbound::new);
        self.config = config;
        self.heartbeat_range = heartbeat;
        Ok(())
    }

    /// Acceptor: whether the counterparty's HeartBtInt and, on FIXT sessions, DefaultApplVerID
    /// are acceptable, and if so the application version they choose.
    fn logon_terms(&self, heartbeat: Duration, appl_ver_id: Option<ApplVerId>) -> Result<Option<ApplVersion>, String> {
        let range = &self.heartbeat_range;
        if !range.contains(&heartbeat) {
            let (low, high) = (range.start().as_secs(), range.end().as_secs());
            return Err(format!("HeartBtInt(108) must be between {low} and {high}"));
        }
        let appl_version = if let Some(id) = appl_ver_id {
            let version = self.config.appl_versions.iter().find(|v| v.id == id).ok_or_else(|| {
                let supported: Vec<_> =
                    self.config.appl_versions.iter().map(|v| format!("'{}'", v.id.code())).collect();
                let one_of = if supported.len() > 1 { "one of " } else { "" };
                format!("DefaultApplVerID(1137) must be {one_of}{}, not '{}'", supported.join(", "), id.code())
            })?;
            Some(version.clone())
        } else {
            None
        };
        Ok(appl_version)
    }

    fn session_id_for(&self, target_comp_id: String) -> SessionId {
        SessionId {
            begin_string: self.config.begin_string.clone(),
            sender_comp_id: self.config.sender_comp_id.clone(),
            target_comp_id,
        }
    }

    /// FIXT sessions: the application version in use.
    fn appl_ver_id(&self) -> Option<ApplVerId> {
        self.appl_version.as_ref().map(|v| v.id)
    }

    /// The validator for an inbound message: on FIXT sessions, that of the version it states in
    /// ApplVerID(1128), or else the negotiated default's.
    #[cfg(feature = "validation")]
    fn validator(&self, msg: &Message) -> Option<&crate::validation::Validator> {
        match &self.appl_version {
            Some(default) => {
                let stated = msg.get(tags::APPL_VER_ID).and_then(|v| self.supported_version(v));
                stated.unwrap_or(default).validator.as_deref()
            }
            None => self.config.validator.as_deref(),
        }
    }

    /// FIXT sessions: the configured version with ApplVerID code `code`, if supported.
    fn supported_version(&self, code: &str) -> Option<&ApplVersion> {
        self.config.appl_versions.iter().find(|v| v.id.code() == code)
    }

    /// Claims the session in the registry and opens its log, then does the rest of logon,
    /// `then`; if the store opens the log with a job, that waits for it (see
    /// [`take_open`](Self::take_open)). Closes on failure.
    fn bind(&mut self, id: SessionId, heartbeat: Duration, then: AfterOpen, now: Instant) {
        // A session binds to one log, once: a second Logon on the connection is refused first.
        assert!(self.peer.is_none(), "the session is bound once");
        assert!(self.opening_log.is_none(), "the session is bound once");
        match self.registry.acquire(&id, self.commands.clone(), self.appl_ver_id()) {
            Ok(Opened::Ready(log)) => self.bound(id, heartbeat, log, then, now),
            Ok(Opened::Pending(job)) => {
                debug!(session = %id, "waiting for the store to open the session's log");
                self.opening_log = Some(OpeningLog { id, heartbeat, then, job: Some(job) });
            }
            Err(reason) => {
                warn!("refusing logon: {reason}");
                self.close(Disconnect::Error, now);
            }
        }
    }

    /// The session's log is open: the session is bound, and logon goes on with `then`.
    fn bound(&mut self, id: SessionId, heartbeat: Duration, log: Box<dyn SessionLog>, then: AfterOpen, now: Instant) {
        // Label the driver's span (see `connection::run`), so every later log line, from the
        // engine or the application, carries the session ID.
        tracing::Span::current().record("id", tracing::field::display(&id));
        #[cfg(feature = "metrics")]
        let latency = self.config.latency_metrics;
        #[cfg(not(feature = "metrics"))]
        let latency = false;
        let metrics = SessionMetrics::new(&id, latency);
        assert!(self.peer.is_none(), "the session is bound once");
        let handle = self.registry.handle(id.clone());
        self.peer = Some(Peer { id, handle, log, heartbeat, metrics });
        if let Err(e) = self.start_period() {
            return self.storage_failed(e, now);
        }
        self.recovered = self.peer().log.in_flight();
        if let Some(start) = self.recovered {
            info!(start, "messages from this one on may have been handled before a restart");
        }
        self.update_sequence_gauges();
        match then {
            AfterOpen::SendLogon { reset } => self.send_logon(reset, now),
            AfterOpen::AnswerLogon { reset, seq_num, their_next, heartbeat } => {
                self.answer_logon(reset, seq_num, their_next, heartbeat, now);
            }
        }
    }

    /// Why the schedule forbids logging on now, if it does.
    fn outside_schedule(&self) -> Option<String> {
        self.config.schedule.as_ref()?.closed_reason(self.config.clock.now())
    }

    /// Ties a newly bound session to its schedule period. Stored state from an earlier period is
    /// reset; a store without a recorded creation time gets one (now), so it resets from the next
    /// period on rather than losing sequence numbers mid-period.
    fn start_period(&mut self) -> io::Result<()> {
        let now = self.config.clock.now();
        self.period = self.config.schedule.as_ref().and_then(|schedule| schedule.period_at(now));
        let period = self.period;
        let log = &mut self.peer_mut().log;
        match (log.created_at(), period) {
            (Some(created), Some(period)) if created.time() < period.start => {
                info!(%created, period_start = %period.start, "new session period: resetting sequence numbers");
                log.reset()?;
                log.set_created_at(now.into())?;
            }
            (Some(_), _) => {}
            (None, _) => {
                log.set_created_at(now.into())?;
                if period.is_some() && log.created_at().is_none() {
                    warn!("session store does not record creation times; sequence numbers will not reset on schedule");
                }
            }
        }
        Ok(())
    }

    /// Whether the schedule period this session logged on in is over (including when a new period
    /// has begun straight after it).
    fn period_ended(&self) -> bool {
        match (&self.config.schedule, &self.peer) {
            (Some(schedule), Some(_)) => schedule.period_at(self.config.clock.now()) != self.period,
            _ => false,
        }
    }

    /// Resets sequence numbers and the resend store, recording now as the store's creation time.
    fn reset_store(&mut self) -> io::Result<()> {
        let now = self.config.clock.now();
        let log = &mut self.peer_mut().log;
        log.reset()?;
        // Paired with the store's own reset: both numbers start again.
        assert_eq!(log.next_outgoing(), 1, "a reset store sends from 1");
        assert_eq!(log.next_incoming(), 1, "a reset store expects 1");
        log.set_created_at(now.into())?;
        // Windows count in the old numbers.
        self.recovered = None;
        self.window_end = None;
        self.update_sequence_gauges();
        Ok(())
    }

    // ---- Established session ----

    fn check_begin_string(&self, msg: &Message) -> Result<(), String> {
        if msg.get(tags::BEGIN_STRING) != Some(self.config.begin_string.as_str()) {
            return Err(format!("BeginString(8) must be '{}'", self.config.begin_string));
        }
        Ok(())
    }

    /// Checks the fields that identify the session. A CompID that is missing or empty is left to
    /// be rejected like any other field ([`missing_header_field`], [`empty_field`]).
    fn check_header(&self, msg: &Message) -> Result<(), HeaderMismatch> {
        self.check_begin_string(msg).map_err(HeaderMismatch::BeginString)?;
        let target = &self.peer().id.target_comp_id;
        let differs = |tag, expected: &str| msg.get(tag).is_some_and(|value| !value.is_empty() && value != expected);
        if differs(tags::SENDER_COMP_ID, target) {
            return Err(HeaderMismatch::CompId(format!("SenderCompID(49) must be '{target}'")));
        }
        if differs(tags::TARGET_COMP_ID, &self.config.sender_comp_id) {
            return Err(HeaderMismatch::CompId(format!("TargetCompID(56) must be '{}'", self.config.sender_comp_id)));
        }
        Ok(())
    }

    /// An inbound message on an established session. `arrived` is false for one taken from the
    /// queue, which was checked for SendingTime when it arrived.
    fn on_session_message(&mut self, msg: &Message, now: Instant, arrived: bool) {
        let Some(seq_num) = self.check_arrival(msg, now, arrived) else { return };
        let mtype = msg.msg_type();
        if self.handle_unsequenced(msg, &mtype, seq_num, now) {
            return;
        }
        let expected = self.peer().log.next_incoming();
        match seq_num.cmp(&expected) {
            Ordering::Greater => self.handle_ahead(msg, &mtype, seq_num, expected, now),
            Ordering::Less => self.handle_behind(msg, seq_num, expected, now),
            Ordering::Equal => self.handle_expected(msg, &mtype, seq_num, now, arrived),
        }
    }

    /// The checks on a message before its sequence number counts: its header's BeginString and
    /// CompIDs, its MsgSeqNum, and (if it has just `arrived`) its SendingTime. Returns its
    /// MsgSeqNum, or `None` once it has been answered with a Logout.
    fn check_arrival(&mut self, msg: &Message, now: Instant, arrived: bool) -> Option<u64> {
        match self.check_header(msg) {
            Ok(()) => {}
            // Session test case 2i: Logout, without a Reject.
            Err(HeaderMismatch::BeginString(text)) => {
                warn!("{text}; logging out");
                self.logout(Some(&text), Disconnect::Error, now);
                self.close(Disconnect::Error, now);
                return None;
            }
            // Session test case 2k: Reject, then Logout.
            Err(HeaderMismatch::CompId(text)) => {
                warn!("{text}; logging out");
                self.reject(msg, None, Some(SessionRejectReason::CompIDProblem), &text, now);
                self.logout(Some(&text), Disconnect::Error, now);
                self.close(Disconnect::Error, now);
                return None;
            }
        }
        let Ok(seq_num) = msg.field::<u64>(tags::MSG_SEQ_NUM) else {
            self.logout(Some("MsgSeqNum(34) missing or invalid"), Disconnect::Error, now);
            self.close(Disconnect::Error, now);
            return None;
        };
        // Session test case 2o: Reject, then Logout. The message still takes its number.
        if arrived && let Some(text) = self.sending_time_problem(msg) {
            warn!("{text}; logging out");
            let reason = Some(SessionRejectReason::SendingTimeAccuracyProblem);
            self.reject(msg, Some(tags::SENDING_TIME), reason, &text, now);
            if seq_num == self.peer().log.next_incoming() {
                self.set_next_incoming(seq_num + 1, now);
            }
            self.logout(Some(&text), Disconnect::Error, now);
            return None;
        }
        Some(seq_num)
    }

    /// Handles the messages whose MsgSeqNum doesn't follow the sequence: an intraday reset, and a
    /// SequenceReset in reset mode. Returns whether `msg` was one.
    fn handle_unsequenced(&mut self, msg: &Message, mtype: &MsgType, seq_num: u64, now: Instant) -> bool {
        // Intraday reset: a Logon with ResetSeqNumFlag=Y and MsgSeqNum 1 while logged on. Any
        // other Logon goes on to be refused as usual.
        if *mtype == MsgType::Logon
            && seq_num == 1
            && msg.flag(tags::RESET_SEQ_NUM_FLAG)
            && self.status == Status::Active
            && msg.defect().is_none()
        {
            self.intraday_reset(msg, now);
            return true;
        }
        // Reset mode ignores MsgSeqNum entirely.
        if *mtype == MsgType::SequenceReset && !msg.flag(tags::GAP_FILL_FLAG) {
            if !self.reject_defect(msg, now) {
                self.on_sequence_reset(msg, now);
            }
            return true;
        }
        false
    }

    /// A message ahead of the sequence: there's a gap before it. It waits in the queue for the
    /// gap to be filled, a resend asked for if one isn't under way.
    fn handle_ahead(&mut self, msg: &Message, mtype: &MsgType, seq_num: u64, expected: u64, now: Instant) {
        debug_assert!(seq_num > expected);
        // Session test case 1a: a counterparty that's leaving is unlikely to resend first, so its
        // Logout is answered now. The next logon finds the gap again.
        if *mtype == MsgType::Logout {
            return self.on_logout_message(msg, now);
        }
        // Answer their ResendRequest now so both sides can recover from a mutual gap.
        let answered = *mtype == MsgType::ResendRequest && msg.defect().is_none();
        if answered {
            self.on_resend_request(msg, now);
        }
        self.queue(seq_num, msg, answered);
        if self.resend.is_none() {
            self.request_resend(expected, seq_num, now);
        }
    }

    /// A message behind the sequence: a possible duplicate is checked and ignored; anything else
    /// means the counterparty lost messages it had sent, and the session logs out.
    fn handle_behind(&mut self, msg: &Message, seq_num: u64, expected: u64, now: Instant) {
        debug_assert!(seq_num < expected);
        if msg.flag(tags::POSS_DUP_FLAG) {
            // Session test cases 2f and 2g: a duplicate is still checked, but doesn't take a
            // number.
            if !self.reject_orig_sending_time(msg, now) {
                debug!(seq_num, "ignoring possible duplicate");
            }
            return;
        }
        self.logout(
            Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")),
            Disconnect::Error,
            now,
        );
        self.close(Disconnect::Error, now);
    }

    /// The message the sequence expects: handled, then its number saved.
    fn handle_expected(&mut self, msg: &Message, mtype: &MsgType, seq_num: u64, now: Instant, arrived: bool) {
        debug_assert_eq!(seq_num, self.peer().log.next_incoming());
        // A defective gap fill falls through: it uses its number and is rejected like any other.
        if *mtype == MsgType::SequenceReset && msg.defect().is_none() {
            return self.on_gap_fill(msg, seq_num, now);
        }
        self.handle_in_sequence(msg, seq_num, now, arrived);
        // Saved only now that the message is handled and anything sent in reply is stored: if
        // the process stops first, the counterparty resends it (at-least-once delivery). Not once
        // the store has failed, when an application message may not have been delivered.
        if !self.store_failed {
            self.set_next_incoming(seq_num + 1, now);
        }
    }

    /// Checks and acts on a message that arrived in sequence, before its number is saved.
    /// `arrived` is false for one taken from the queue.
    fn handle_in_sequence(&mut self, msg: &Message, seq_num: u64, now: Instant, arrived: bool) {
        let mtype = msg.msg_type();
        if self.reject_defect(msg, now) {
            return;
        }
        let (misplaced, empty) = misplaced_header_and_empty_field(msg, self.config.check_header_order);
        if let Some(tag) = misplaced {
            let reason = Some(SessionRejectReason::TagSpecifiedOutOfRequiredOrder);
            return self.reject(msg, Some(tag), reason, &format!("Tag {tag} specified out of required order"), now);
        }
        if let Some(tag) = missing_header_field(msg) {
            let reason = Some(SessionRejectReason::RequiredTagMissing);
            return self.reject(msg, Some(tag), reason, &format!("Required tag {tag} missing"), now);
        }
        if let Some(tag) = empty {
            let reason = Some(SessionRejectReason::TagSpecifiedWithoutValue);
            return self.reject(msg, Some(tag), reason, &format!("Tag {tag} specified without a value"), now);
        }
        if msg.flag(tags::POSS_DUP_FLAG) && self.reject_orig_sending_time(msg, now) {
            return;
        }
        if self.appl_version.is_some()
            && !mtype.is_admin()
            && let Some(stated) = msg.get(tags::APPL_VER_ID)
            && self.supported_version(stated).is_none()
        {
            let reason = Some(SessionRejectReason::InvalidUnsupportedApplicationVersion);
            let supported: Vec<&str> = self.config.appl_versions.iter().map(|v| v.id.code()).collect();
            let text = format!("ApplVerID(1128) '{stated}' not supported; one of {}", supported.join(", "));
            return self.reject(msg, Some(tags::APPL_VER_ID), reason, &text, now);
        }
        #[cfg(feature = "validation")]
        if let Some(validator) = self.validator(msg)
            && validator.applies_to(&mtype)
            && let Err(invalid) = validator.validate(msg)
        {
            return self.reject(msg, invalid.tag, invalid.reason, &invalid.text, now);
        }
        self.dispatch(msg, seq_num, now, arrived);
    }

    fn dispatch(&mut self, msg: &Message, seq_num: u64, now: Instant, arrived: bool) {
        match msg.msg_type() {
            MsgType::Heartbeat => self.notify_admin(msg),
            MsgType::TestRequest => {
                self.notify_admin(msg);
                match msg.parse::<TestRequest>() {
                    Ok(request) => self.send(Heartbeat { test_req_id: Some(request.test_req_id) }.into(), now),
                    Err(e) => self.reject_field(msg, e, now),
                }
            }
            MsgType::ResendRequest => self.on_resend_request(msg, now),
            MsgType::Reject => {
                warn!(
                    ref_seq_num = ?msg.get(tags::REF_SEQ_NUM),
                    text = ?msg.get(tags::TEXT),
                    "counterparty rejected a message"
                );
                self.notify_admin(msg);
            }
            MsgType::Logout => self.on_logout_message(msg, now),
            MsgType::Logon => self.reject(msg, None, None, "Session is already logged on", now),
            _ => self.deliver(msg, seq_num, now, arrived),
        }
    }

    /// Why a message's SendingTime is too far from our clock, if it is (see
    /// [`SessionConfig::max_latency`]). A missing or malformed one is left to other checks.
    fn sending_time_problem(&self, msg: &Message) -> Option<String> {
        let max = self.config.max_latency?;
        let sent: UtcTimestamp = msg.opt_field(tags::SENDING_TIME).ok()??;
        let now = self.wall_clock();
        let off = (now - sent.time()).abs().to_std().unwrap_or(Duration::MAX);
        (off > max).then(|| format!("SendingTime accuracy problem: {}s from our clock", off.as_secs()))
    }

    /// Rejects a PossDup message whose OrigSendingTime is missing, or later than its SendingTime
    /// (then logging out), if [`SessionConfig::check_orig_sending_time`]. Returns whether it did.
    fn reject_orig_sending_time(&mut self, msg: &Message, now: Instant) -> bool {
        if !self.config.check_orig_sending_time {
            return false;
        }
        let Ok(orig) = msg.opt_field::<UtcTimestamp>(tags::ORIG_SENDING_TIME) else { return false };
        let Some(orig) = orig else {
            let reason = Some(SessionRejectReason::RequiredTagMissing);
            let tag = tags::ORIG_SENDING_TIME;
            self.reject(msg, Some(tag), reason, &format!("Required tag {tag} missing"), now);
            return true;
        };
        match msg.opt_field::<UtcTimestamp>(tags::SENDING_TIME) {
            Ok(Some(sent)) if orig > sent => {
                let text = "OrigSendingTime(122) later than SendingTime(52)";
                warn!("{text}; logging out");
                let reason = Some(SessionRejectReason::SendingTimeAccuracyProblem);
                self.reject(msg, Some(tags::ORIG_SENDING_TIME), reason, text, now);
                self.logout(Some(text), Disconnect::Error, now);
                true
            }
            _ => false,
        }
    }

    /// The counterparty reset sequence numbers while logged on: both sides start again at 1.
    /// Their Logon is 1 and our reply goes out at 1, with ResetSeqNumFlag=Y. What was outstanding
    /// (a resend, messages queued behind a gap, stored messages) belonged to the old numbering and
    /// is dropped.
    fn intraday_reset(&mut self, msg: &Message, now: Instant) {
        info!("counterparty reset sequence numbers while logged on; resetting ours");
        // A resend in progress, and anything held behind it, belong to the old numbers.
        self.replay = None;
        self.held.clear();
        if let Err(e) = self.reset_store() {
            return self.storage_failed(e, now);
        }
        self.resend = None;
        self.queued.clear();
        self.set_next_incoming(2, now);
        let our_next = msg.get(tags::NEXT_EXPECTED_MSG_SEQ_NUM).is_some().then_some(2);
        let heartbeat = self.peer().heartbeat;
        self.send(logon_message(heartbeat, true, our_next, self.appl_ver_id()).into(), now);
    }

    /// The counterparty's Logout: a request to answer, or the reply to ours.
    fn on_logout_message(&mut self, msg: &Message, now: Instant) {
        self.notify_admin(msg);
        // Only an established session gets here: a Logout answering an initiator's Logon is refused
        // as a Logon reply.
        let reason = if self.status == Status::Active {
            info!(text = ?msg.get(tags::TEXT), "counterparty logged out");
            self.send(Logout { text: None }.into(), now);
            Disconnect::CounterpartyLogout
        } else {
            info!("logout confirmed");
            self.ending.expect("LoggingOut is entered only through logout, which sets ending")
        };
        self.close(reason, now);
    }

    /// Keeps a message that arrived ahead of a gap until its turn; the first one kept for each
    /// number wins, so an original isn't replaced by its resend.
    fn queue(&mut self, seq_num: u64, msg: &Message, answered: bool) {
        debug_assert!(seq_num > self.peer().log.next_incoming(), "only a message ahead of the gap waits");
        if self.queued.len() >= MAX_QUEUED && !self.queued.contains_key(&seq_num) {
            debug!(seq_num, "queue full; dropping a message ahead of the gap");
            return;
        }
        self.queued.entry(seq_num).or_insert_with(|| Queued { msg: msg.clone(), answered });
        debug_assert!(self.queued.len() <= MAX_QUEUED);
    }

    /// After the incoming sequence may have moved: processes queued messages now in sequence,
    /// notes the resend's progress, and asks again if a hole is left behind a completed resend.
    fn after_incoming(&mut self, now: Instant) {
        while matches!(self.status, Status::Active | Status::LoggingOut { .. }) {
            let next = self.peer().log.next_incoming();
            let covered = self.covers(next);
            let Some(entry) = self.queued.first_entry() else { break };
            if *entry.key() > next {
                break;
            }
            // One in sequence waits for a window to be handled in, as input does.
            if *entry.key() == next && !entry.get().answered && !covered {
                self.drain_waiting = true;
                break;
            }
            let (seq_num, queued) = entry.remove_entry();
            if seq_num < next {
                continue; // Covered since, by a gap fill or an operator.
            }
            if queued.answered {
                self.set_next_incoming(seq_num + 1, now);
            } else {
                // Its SendingTime was checked when it arrived.
                self.on_session_message(&queued.msg, now, false);
            }
        }
        let Some(peer) = &self.peer else { return };
        let next = peer.log.next_incoming();
        if matches!(self.status, Status::Active | Status::LoggingOut { .. }) && !self.drain_waiting {
            // Whatever is still queued is ahead of the gap.
            debug_assert!(self.queued.first_key_value().is_none_or(|(&seq_num, _)| seq_num > next));
        }
        if let Some(resend) = &mut self.resend {
            if next > resend.target {
                info!("resend complete");
                self.resend = None;
            } else if next > resend.seen {
                resend.seen = next;
                resend.progress_at = now;
                resend.retried = false;
            }
        }
        if matches!(self.status, Status::Active | Status::LoggingOut { .. }) {
            self.ask_next_chunk(next, now);
        }
        if self.resend.is_none()
            && !self.drain_waiting
            && matches!(self.status, Status::Active | Status::LoggingOut { .. })
            && let Some(&ahead) = self.queued.keys().next()
        {
            self.request_resend(next, ahead, now);
        }
    }

    /// Hands an application message to the application and sends its replies or reject.
    /// `arrived` is false for one taken from the queue, where it waited behind a gap.
    fn deliver(&mut self, msg: &Message, seq_num: u64, now: Instant, arrived: bool) {
        // The committed window marks it in flight: still marked after a crash, it tells the next
        // connection that the resend of this message may have been handled already.
        debug_assert!(self.covers(seq_num), "{seq_num} is handed over in a committed window");
        // A message in the window recovered from before a restart may have been handled then if
        // it's coming again: flagged as a resend, or answering our ResendRequest (in case the
        // counterparty doesn't flag its resends). New messages can't have been.
        let resent = msg.flag(tags::POSS_DUP_FLAG) || self.resend.is_some();
        debug_assert!(!msg.msg_type().is_admin(), "admin messages are the session's, never delivered");
        // Recovery we asked for counts but is never rejected: what arrives while our
        // ResendRequest is outstanding, and what waited behind the gap it fills, which may be
        // released after the request is done. Rejecting it would lose messages we asked for. It
        // still counts, since the counterparty controls its gaps: one that makes a gap can't
        // then send new traffic past the limit as its "resend". PossDupFlag alone is only the
        // sender's claim.
        let requested = self.resend.is_some() || !arrived;
        if self.over_inbound_limit(now, requested) {
            // Logged at debug: a flood over the limit would otherwise log a warning per message.
            // The limit's first warning on the connection says it's being reached.
            debug!(msg_type = %msg.msg_type(), seq_num, "rejecting a message over the inbound rate limit");
            let text = "throttle limit exceeded".into();
            return self.send_business_reject(msg, seq_num, BusinessRejectReason::Other, text, now);
        }
        let redelivered =
            resent && self.recovered.is_some_and(|start| (start..start + DELIVERIES_PER_COMMIT).contains(&seq_num));
        if redelivered {
            info!(seq_num, "delivering a message that may have been handled before a restart");
        }
        // Borrows the peer field alone (not `self.peer()`), so the application can be called
        // without cloning the handle for every message.
        let handle = &self.peer.as_ref().expect("session is not bound before logon").handle;
        let mut ctx = Context::with_outbox(handle, std::mem::take(&mut self.outbox));
        if redelivered {
            ctx = ctx.redelivery();
        }
        let result = guarded("on_message", || self.app.on_message(&mut ctx, msg));
        let mut outbox = ctx.into_outbox();
        let mut sent = std::mem::take(&mut outbox.sent);
        for mut reply in sent.drain(..) {
            // Replies queued before a panic may be half-done, so they're dropped, not sent.
            // Replies have no receipt: anything wrong with one is logged.
            if result.is_some() {
                let _ = self.send_app(&mut reply, now);
            }
            outbox.recycle(reply);
        }
        outbox.sent = sent;
        self.outbox = outbox;
        let Some(result) = result else {
            // The message counts as received, so say it wasn't processed.
            let reason = BusinessRejectReason::ApplicationNotAvailable;
            return self.business_reject(msg, seq_num, reason, "Application error".into(), now);
        };
        match result {
            Ok(()) => {}
            Err(MessageReject::Session { ref_tag, reason, text }) => {
                self.reject(msg, ref_tag, Some(reason), &text, now)
            }
            Err(MessageReject::Business { reason, text }) => self.business_reject(msg, seq_num, reason, text, now),
        }
    }

    /// Whether an application message handed over at `now` is over an inbound Reject limit, and
    /// so is to be rejected. One that isn't is recorded; one that is isn't, so rejections don't
    /// keep the window full. Recovery we asked for (`requested`) is recorded, past full if need
    /// be, but never rejected. Each message rejected counts as throttled. A Delay limit counts
    /// messages as they arrive instead (see [`count_arrival`](Self::count_arrival)).
    fn over_inbound_limit(&mut self, now: Instant, requested: bool) -> bool {
        let Some(inbound) = &mut self.inbound else { return false };
        if inbound.over != Over::Reject {
            return false;
        }
        if requested || inbound.window.free_at(now).is_none() {
            inbound.window.record(now);
            return false;
        }
        let limit = inbound.window.limit();
        self.inbound_throttled(limit, Over::Reject);
        true
    }

    /// Records an application message arriving at `now` under a Delay limit, whatever becomes of
    /// it: handed over, queued ahead of a gap, a resend, or refused by the session. Delay paces
    /// what's read, so counting on arrival leaves a counterparty nothing to gain from a gap it
    /// makes: what it queues ahead of the gap was paced as it was read, and isn't counted again
    /// when it's handed over. When the message fills the window, the driver holds input (see
    /// [`input_free_at`](Self::input_free_at)), and that hold counts as throttled: the input it
    /// keeps waiting is unread, so a hold is counted, not the messages it holds.
    fn count_arrival(&mut self, now: Instant) {
        let Some(inbound) = &mut self.inbound else { return };
        if inbound.over != Over::Delay {
            return;
        }
        inbound.window.record(now);
        if inbound.window.free_at_or_none().is_some() {
            let limit = inbound.window.limit();
            self.inbound_throttled(limit, Over::Delay);
        }
    }

    /// Counts a message rejected by the inbound limit, or a hold, and warns the first time on
    /// the connection.
    fn inbound_throttled(&mut self, limit: RateLimit, over: Over) {
        self.peer().metrics.throttled_inbound();
        if std::mem::replace(&mut self.inbound_warned, true) {
            return;
        }
        match over {
            Over::Delay => warn!("inbound rate limit {limit} reached: delaying input"),
            Over::Reject => warn!("inbound rate limit {limit} reached: rejecting messages over it"),
        }
    }

    fn business_reject(
        &mut self,
        msg: &Message,
        seq_num: u64,
        reason: BusinessRejectReason,
        text: String,
        now: Instant,
    ) {
        warn!(msg_type = %msg.msg_type(), %reason, %text, "business reject");
        self.send_business_reject(msg, seq_num, reason, text, now);
    }

    /// [`business_reject`](Self::business_reject), without the warning.
    fn send_business_reject(
        &mut self,
        msg: &Message,
        seq_num: u64,
        reason: BusinessRejectReason,
        text: String,
        now: Instant,
    ) {
        self.peer().metrics.business_reject();
        let reply = BusinessMessageReject {
            ref_seq_num: Some(seq_num),
            ref_msg_type: msg.msg_type(),
            business_reject_reason: reason,
            text: Some(text.into()),
        };
        let reply = self.with_ref_appl_version(Message::from(reply).with_reverse_route(msg), msg);
        self.send(reply, now);
    }

    fn on_sequence_reset(&mut self, msg: &Message, now: Instant) {
        self.notify_admin(msg);
        let reset: SequenceReset = match msg.parse() {
            Ok(reset) => reset,
            Err(e) => return self.reject_field(msg, e, now),
        };
        let expected = self.peer().log.next_incoming();
        if reset.new_seq_no >= expected {
            info!(from = expected, to = reset.new_seq_no, "sequence reset");
            self.set_next_incoming(reset.new_seq_no, now);
        } else {
            let text = format!("NewSeqNo {} is lower than expected {expected}", reset.new_seq_no);
            self.reject(msg, Some(tags::NEW_SEQ_NO), Some(SessionRejectReason::ValueIsIncorrect), &text, now);
        }
    }

    fn on_gap_fill(&mut self, msg: &Message, seq_num: u64, now: Instant) {
        self.notify_admin(msg);
        match msg.parse::<SequenceReset>() {
            Ok(fill) if fill.new_seq_no > seq_num => {
                debug!(from = seq_num, to = fill.new_seq_no, "gap fill");
                self.set_next_incoming(fill.new_seq_no, now);
            }
            parsed => {
                self.set_next_incoming(seq_num + 1, now);
                match parsed {
                    Err(e) => self.reject_field(msg, e, now),
                    Ok(_) => self.reject(
                        msg,
                        Some(tags::NEW_SEQ_NO),
                        Some(SessionRejectReason::ValueIsIncorrect),
                        "NewSeqNo(36) must be greater than MsgSeqNum(34)",
                        now,
                    ),
                }
            }
        }
    }

    /// Replays stored application messages with PossDupFlag=Y and gap-fills everything else.
    fn on_resend_request(&mut self, msg: &Message, now: Instant) {
        self.notify_admin(msg);
        let ResendRequest { begin_seq_no: begin, end_seq_no: end } = match msg.parse() {
            Ok(request) => request,
            Err(e) => return self.reject_field(msg, e, now),
        };
        self.peer().metrics.resend_request_received();
        let last_sent = self.peer().log.next_outgoing() - 1;
        let end = if end == 0 || end > last_sent { last_sent } else { end };
        if begin == 0 || begin > end {
            let reason = Some(SessionRejectReason::ValueIsIncorrect);
            return self.reject(msg, Some(tags::BEGIN_SEQ_NO), reason, "Invalid resend range", now);
        }
        if let Some(evicted_through) = self.peer().log.evicted_through().filter(|evicted| begin <= *evicted) {
            warn!(begin, end, evicted_through, "resend reaches messages the store has evicted; gap-filling them");
            self.peer().metrics.resend_request_evicted();
        }
        self.resend(begin, end, now);
    }

    /// Starts resending `begin..=end` as stored, gap-filling what wasn't stored (session
    /// messages), without using new sequence numbers. The first step goes out now, and the rest
    /// from [`on_resume`](Self::on_resume).
    fn resend(&mut self, begin: u64, end: u64, now: Instant) {
        // A closed session sends nothing further (the Logon before it may have failed to store).
        if self.status == Status::Closed {
            return;
        }
        info!(begin, end, "resending messages");
        assert!(begin >= 1, "sequence numbers start at 1");
        assert!(begin <= end, "a resend covers at least one number");
        // Never past what's been sent: those numbers would be used again by new messages.
        assert!(end < self.peer().log.next_outgoing(), "resend {begin}..={end} reaches numbers not yet sent");
        // Only a driver that feeds messages during a resend gets here with one in progress. The
        // new range replaces it; what was held is stored, so it's resent if the range covers it,
        // and otherwise the counterparty finds the gap at our next message.
        if self.replay.is_some() {
            self.held.clear();
        }
        self.replay = Some(Replay { next: begin, scan: begin, end, fetch: None });
        self.resend_step(now);
    }

    /// Resends the next `resend_batch` sequence numbers of the replay, and ends it after the last.
    /// If the store reads them with a job, the step waits for it: see
    /// [`take_fetch`](Self::take_fetch).
    fn resend_step(&mut self, now: Instant) {
        let Some(replay) = &self.replay else { return };
        if replay.fetch.is_some() {
            return;
        }
        let (scan, end) = (replay.scan, replay.end);
        debug_assert!(replay.next <= scan && scan <= end);
        let to = end.min(scan.saturating_add(self.resend_batch - 1));
        match self.peer_mut().log.fetch(scan, to) {
            Ok(Fetched::Ready(stored)) => self.resend_stored(stored, to, now),
            Ok(Fetched::Pending(job)) => {
                let replay = self.replay.as_mut().expect("checked above");
                replay.fetch = Some(Fetching { to, job: Some(job) });
            }
            Err(e) => self.storage_failed(e, now),
        }
    }

    /// Resends `stored`, the messages the store holds of the replay's next step, up to `to`, and
    /// gap-fills the rest of the step.
    fn resend_stored(&mut self, stored: SentMessages, to: u64, now: Instant) {
        let Some(Replay { mut next, scan, end, fetch }) = self.replay.take() else { return };
        debug_assert!(fetch.is_none(), "the step's read has ended");
        let originals = match self.parse_stored(stored, scan, to) {
            Ok(originals) => originals,
            Err(e) => return self.storage_failed(e, now),
        };
        let now_ts = UtcTimestamp::from(self.wall_clock()).with_precision(self.config.timestamp_precision).to_fix();
        for (seq, original) in originals {
            // Declined: the gap fill before the next message resent covers it.
            if !self.app_resends(&original) {
                continue;
            }
            if seq > next {
                self.send_gap_fill(next, seq, &now_ts);
            }
            let orig_time = original.get(tags::SENDING_TIME).unwrap_or(&now_ts).to_string();
            let mut output = std::mem::take(&mut self.output);
            let start = output.len();
            self.frame_into(&original, seq, &now_ts, Some(&orig_time), false, &mut output);
            self.output = output;
            self.emit(&self.output, start);
            next = seq + 1;
        }
        if to < end {
            // A step that ends in numbers not resent gap-fills them now rather than with the next
            // message resent: through a long run of them, the counterparty sees progress each
            // step, and doesn't take a slow resend for an unanswered one.
            if next <= to {
                self.send_gap_fill(next, to + 1, &now_ts);
                next = to + 1;
            }
            self.replay = Some(Replay { next, scan: to + 1, end, fetch: None });
        } else {
            if next <= end {
                self.send_gap_fill(next, end + 1, &now_ts);
            }
            self.finish_replay(now);
        }
        self.last_sent = now;
    }

    /// Whether the application has `msg` resent, as it does if it panics deciding.
    fn app_resends(&self, msg: &Message) -> bool {
        guarded("should_resend", || self.app.should_resend(&self.peer().handle, msg)).unwrap_or(true)
    }

    /// Stored messages `begin..=end`, parsed with the session's data fields: all of them, or an
    /// error if any is corrupt, so that none of a corrupt range is resent.
    fn parse_stored(&self, stored: SentMessages, begin: u64, end: u64) -> io::Result<Vec<(u64, Message)>> {
        // Stores keep the bytes as sent; they're parsed here.
        let mut originals = Vec::with_capacity(stored.len());
        let mut last = None;
        for (seq, bytes) in stored {
            assert!((begin..=end).contains(&seq), "the store read {seq}, outside {begin}..={end}");
            assert!(last < Some(seq), "the store read {seq} after {last:?}");
            last = Some(seq);
            match decode_stored(&bytes, &self.config.data_fields) {
                Decoded::Message(msg, len) if len == bytes.len() && msg.defect().is_none() => {
                    originals.push((seq, msg))
                }
                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("stored message {seq} is corrupt")));
                }
            }
        }
        Ok(originals)
    }

    /// The replay has ended: the counterparty's silence while it was being resent doesn't count.
    fn finish_replay(&mut self, now: Instant) {
        self.replay = None;
        self.output.extend_from_slice(&self.held);
        self.held.clear();
        self.last_received = self.last_received.max(now);
        if let Some(resend) = &mut self.resend {
            resend.progress_at = resend.progress_at.max(now);
        }
    }

    fn send_gap_fill(&mut self, seq: u64, new_seq_no: u64, now_ts: &str) {
        debug_assert!(seq < new_seq_no, "a gap fill moves the sequence forward");
        let body = SequenceReset { gap_fill_flag: Some(true), new_seq_no }.into();
        let mut output = std::mem::take(&mut self.output);
        let start = output.len();
        self.frame_into(&body, seq, now_ts, Some(now_ts), false, &mut output);
        self.output = output;
        self.emit(&self.output, start);
    }

    fn request_resend(&mut self, from: u64, received: u64, now: Instant) {
        debug_assert!(from < received, "a gap lies before the message that revealed it");
        warn!(expected = from, received, "sequence gap detected; requesting resend");
        self.peer().metrics.sequence_gap();
        let asked_through = self.ask_resend(from, received, now);
        self.resend = Some(Resend { target: received, progress_at: now, seen: from, retried: false, asked_through });
    }

    /// Sends a ResendRequest from `from` for a gap that `target` revealed: a chunk if the session
    /// asks for them and the gap is longer, otherwise everything to the end. Returns the last
    /// MsgSeqNum asked for, if not to the end.
    fn ask_resend(&mut self, from: u64, target: u64, now: Instant) -> Option<u64> {
        debug_assert!(from < target, "a gap lies before the message that revealed it");
        let through = self.config.resend_request_chunk.map(|chunk| from + chunk - 1).filter(|&last| last + 1 < target);
        self.send(ResendRequest { begin_seq_no: from, end_seq_no: through.unwrap_or(0) }.into(), now);
        through
    }

    /// Asks for the next chunk of the gap once everything asked for has been received, `next`
    /// being the MsgSeqNum now expected.
    fn ask_next_chunk(&mut self, next: u64, now: Instant) {
        let Some(resend) = &self.resend else { return };
        if resend.asked_through.is_none_or(|through| next <= through) || next >= resend.target {
            return;
        }
        let target = resend.target;
        let asked_through = self.ask_resend(next, target, now);
        if let Some(resend) = &mut self.resend {
            resend.asked_through = asked_through;
        }
    }

    fn reject(
        &mut self,
        msg: &Message,
        ref_tag: Option<u32>,
        reason: Option<SessionRejectReason>,
        text: &str,
        now: Instant,
    ) {
        let Ok(ref_seq_num) = msg.field::<u64>(tags::MSG_SEQ_NUM) else {
            warn!(text, "cannot reject a message without a valid MsgSeqNum(34)");
            return;
        };
        // FIX 4.2 defines reasons up to 11; for a later one, the Text says what's wrong.
        let reason =
            reason.filter(|r| self.config.begin_string != "FIX.4.2" || r.code().parse().is_ok_and(|n: u32| n <= 11));
        warn!(ref_seq_num, ?ref_tag, ?reason, text, "rejecting message");
        let reject = Reject {
            ref_seq_num,
            ref_tag_id: ref_tag,
            ref_msg_type: Some(msg.msg_type()),
            session_reject_reason: reason,
            text: Some(text.into()),
        };
        self.peer().metrics.session_reject();
        let reject = self.with_ref_appl_version(Message::from(reject).with_reverse_route(msg), msg);
        self.send(reject, now);
    }

    /// FIXT sessions: a reject naming the version fields of the message it answers, in
    /// RefApplVerID(1130), RefCstmApplVerID(1131) and RefApplExtID(1406).
    fn with_ref_appl_version(&self, mut reject: Message, answered: &Message) -> Message {
        if self.config.is_fixt() {
            for (from, to) in [
                (tags::APPL_VER_ID, tags::REF_APPL_VER_ID),
                (tags::CSTM_APPL_VER_ID, tags::REF_CSTM_APPL_VER_ID),
                (tags::APPL_EXT_ID, tags::REF_APPL_EXT_ID),
            ] {
                if let Some(value) = answered.get(from).filter(|v| !v.is_empty()) {
                    reject.set(to, value);
                }
            }
        }
        reject
    }

    /// Answers a message received with a malformed body field instead of acting on it. Returns
    /// whether it was rejected.
    fn reject_defect(&mut self, msg: &Message, now: Instant) -> bool {
        let Some(defect) = msg.defect() else {
            return false;
        };
        self.reject(msg, defect.tag, Some(defect.reason), &defect.text, now);
        true
    }

    fn reject_field(&mut self, msg: &Message, e: FieldError, now: Instant) {
        self.reject(msg, Some(e.tag), Some(e.reject_reason()), &e.to_string(), now);
    }

    /// Sends Logout and waits for the reply. `reason` is what `on_logout` is told when the session
    /// ends: the first logout's reason stands, unless the session is then closed for another
    /// reason, which wins.
    fn logout(&mut self, text: Option<&str>, reason: Disconnect, now: Instant) {
        info!(text, "logging out");
        self.ending.get_or_insert(reason);
        // Shutdown isn't held up by a long resend: the counterparty asks for the rest next time.
        if self.replay.is_some() {
            info!("logging out during a resend; the rest of it isn't sent");
            self.finish_replay(now);
        }
        self.send(Logout { text: text.map(CompactString::from) }.into(), now);
        // The timeout counts from the first Logout: a counterparty that keeps sending messages we
        // log out over (stale ones, after a long stall) doesn't put it off.
        if matches!(self.status, Status::Active | Status::AwaitingLogon) {
            self.status = Status::LoggingOut { since: now };
        }
    }

    /// Ends the session. The driver writes the output, then closes the connection, so nothing may
    /// add to the output once closed: `send` and `resend` check, and `emit` asserts it.
    ///
    /// `reason` reaches `on_logout` only once logged on; before that it is never reported.
    fn close(&mut self, reason: Disconnect, now: Instant) {
        if self.status != Status::Closed {
            self.status = Status::Closed;
            self.queued.clear();
            self.replay = None;
            self.held.clear();
            if let Some(peer) = &self.peer {
                peer.metrics.logged_off();
            }
            self.discard_pending();
            self.notify_logout(reason, now);
        }
    }

    /// Shows the application an inbound admin message the session is about to act on.
    fn notify_admin(&self, msg: &Message) {
        debug_assert!(msg.msg_type().is_admin() && msg.msg_type() != MsgType::Logon);
        guarded("on_admin_message", || self.app.on_admin_message(&self.peer().handle, msg));
    }

    /// Tells the application a logged-on session has ended, at `now`, and starts its
    /// cancel-on-disconnect countdown if the ending counts.
    fn notify_logout(&mut self, reason: Disconnect, now: Instant) {
        if !std::mem::take(&mut self.app_logged_on) {
            return;
        }
        guarded("on_logout", || self.app.on_logout(&self.peer().handle, reason));
        if let Some(CancelOnDisconnect { trigger, grace }) = self.config.cancel_on_disconnect
            && trigger.counts(reason)
        {
            let deadline = now.checked_add(grace).expect("a grace period of at most an hour fits in an Instant");
            info!(?reason, ?grace, "cancel on disconnect: counting down");
            self.registry.start_cancel(self.peer().id.clone(), reason, trigger, deadline, self.app.clone());
            if grace.is_zero() {
                self.registry.run_due_cancels(now);
            }
        }
    }

    // ---- Sending ----

    /// Sends an application message; session-level message types are refused. Returns its
    /// MsgSeqNum once stored, or why it was dropped.
    ///
    /// Sends from the queue and replies alike count against the outbound limit: only messages
    /// stored count, as only they go out. Resends don't come this way, and don't count.
    fn send_app(&mut self, msg: &mut Message, now: Instant) -> Result<u64, Dropped> {
        if msg.msg_type().is_admin() {
            warn!(msg_type = %msg.msg_type(), "applications cannot send session-level messages; dropping");
            return Err(Dropped::Rejected(format!("{} is a session-level message type", msg.msg_type())));
        }
        if self.appl_version.is_some()
            && let Some(stated) = msg.get(tags::APPL_VER_ID)
            && self.supported_version(stated).is_none()
        {
            error!(msg_type = %msg.msg_type(), stated, "dropping message: its ApplVerID(1128) isn't one this session supports");
            return Err(Dropped::Rejected(format!("ApplVerID(1128) {stated} isn't one this session supports")));
        }
        let outcome = self.send_outcome(msg, now);
        if outcome.is_ok()
            && let Some(window) = &mut self.outbound
        {
            window.record(now);
        }
        outcome
    }

    /// Assigns the next outgoing MsgSeqNum, adds the standard header, persists it and queues the
    /// message. Does nothing once the session is closed.
    fn send(&mut self, mut body: Message, now: Instant) {
        // Session messages and replies have no one waiting to hear; anything wrong is logged.
        let _ = self.send_outcome(&mut body, now);
    }

    /// [`send`](Self::send), returning the MsgSeqNum once stored, or why the message was dropped.
    fn send_outcome(&mut self, body: &mut Message, now: Instant) -> Result<u64, Dropped> {
        if self.status == Status::Closed {
            return Err(Dropped::Disconnected);
        }
        let admin = body.msg_type().is_admin();
        // A panic may have left the message half-modified: disconnect rather than send it.
        if admin && guarded("to_admin", || self.app.to_admin(&self.peer().handle, body)).is_none() {
            self.close(Disconnect::Error, now);
            return Err(Dropped::Disconnected);
        }
        // SOH ends a field on the wire, so a value containing one would add fields of its own,
        // unless it's a data field whose length is given just before it.
        if let Some(tag) = body.invalid_data_field(&self.config.data_fields) {
            warn!(
                msg_type = %body.msg_type(),
                tag,
                "dropping message: a value contains SOH, or a data field doesn't follow its length"
            );
            return Err(Dropped::Rejected(format!(
                "tag {tag} contains SOH, or is a data field that doesn't follow its length"
            )));
        }
        let seq = self.peer().log.next_outgoing();
        let sending_time = UtcTimestamp::from(self.wall_clock()).with_precision(self.config.timestamp_precision);
        // During a resend, new messages wait for the rest of it.
        let holding = self.replay.is_some();
        let mut output = std::mem::take(if holding { &mut self.held } else { &mut self.output });
        let start = output.len();
        self.frame_into(body, seq, sending_time, None, false, &mut output);
        // With more than one version, the default may differ on a later connection, so the stored
        // copy states its version for resends to keep.
        let mut stored = std::mem::take(&mut self.stored);
        let unstated = match (self.appl_ver_id(), body.get(tags::APPL_VER_ID)) {
            (Some(_), None) => true,
            (Some(default), Some(stated)) => stated == default.code(),
            (None, _) => false,
        };
        let copy = if admin {
            None
        } else if self.config.appl_versions.len() > 1 && unstated {
            stored.clear();
            self.frame_into(body, seq, sending_time, None, true, &mut stored);
            Some(stored.as_slice())
        } else {
            Some(&output[start..])
        };
        let recorded = self.peer_mut().log.record_outgoing(seq, copy);
        self.stored = stored;
        let failed = recorded.err();
        if failed.is_some() {
            // It isn't sent.
            output.truncate(start);
        }
        *(if holding { &mut self.held } else { &mut self.output }) = output;
        if let Some(e) = failed {
            self.storage_failed(e, now);
            return Err(Dropped::Storage);
        }
        debug_assert_eq!(self.peer().log.next_outgoing(), seq + 1, "recording a message uses its number");
        self.peer().metrics.next_outgoing(seq + 1);
        self.last_sent = now;
        self.emit(if holding { &self.held } else { &self.output }, start);
        Ok(seq)
    }

    /// Counts and logs the message `buf` (the output, or the messages held during a resend) holds
    /// from `start`, just framed, for the driver to write.
    fn emit(&self, buf: &[u8], start: usize) {
        // The driver writes the output, then closes: anything added once closed would still go out.
        debug_assert!(self.status != Status::Closed, "a closed session sends nothing");
        // Exactly one message, framed as the counterparty will frame it.
        debug_assert_eq!(frame_stored(&buf[start..]), Ok(buf.len() - start));
        self.peer().metrics.message_sent();
        debug!(target: "turbojet::messages", direction = "out", "{}", Outbound(&buf[start..], &self.config.data_fields));
    }

    /// Records the next incoming number. That clears the store's in-flight marker, so while one is
    /// needed it's recorded again: a store that makes each change as it's made (rather than at the
    /// commit) must still have it if the process stops before the batch ends.
    fn set_next_incoming(&mut self, seq: u64, now: Instant) {
        let marker = self.marker(seq);
        let log = &mut self.peer_mut().log;
        let recorded =
            log.set_next_incoming(seq).and_then(|()| marker.map_or(Ok(()), |start| log.set_in_flight(start)));
        match recorded {
            Ok(()) => self.peer().metrics.next_incoming(seq),
            Err(e) => self.storage_failed(e, now),
        }
    }

    fn update_sequence_gauges(&self) {
        let peer = self.peer();
        peer.metrics.next_incoming(peer.log.next_incoming());
        peer.metrics.next_outgoing(peer.log.next_outgoing());
    }

    /// Byte counts from the driver, once the session is bound.
    pub(crate) fn metrics(&self) -> Option<&SessionMetrics> {
        self.peer.as_ref().map(|p| &p.metrics)
    }

    /// The latency histograms, once the session is bound, if its configuration asks for them.
    pub(crate) fn latency_metrics(&self) -> Option<&LatencyMetrics> {
        self.metrics()?.latency()
    }

    /// Whether the configuration asks for the latency histograms: the driver times input from
    /// when it's read, before the session that records it is bound (an acceptor's Logon).
    pub(crate) fn times_latency(&self) -> bool {
        #[cfg(feature = "metrics")]
        return self.config.latency_metrics;
        #[cfg(not(feature = "metrics"))]
        false
    }

    /// Messages stored whose commit failed, or never finished: they may have been stored.
    fn fail_receipts(&mut self) {
        for (receipt, _) in self.receipts.drain(..) {
            let _ = receipt.send(Err(Dropped::Storage));
        }
    }

    /// The session cannot continue without durable state, so it disconnects.
    fn storage_failed(&mut self, e: io::Error, now: Instant) {
        error!("session store failed: {e}; disconnecting");
        self.store_failed = true;
        // What wasn't committed may not have been stored, so it isn't sent.
        self.output.truncate(self.committed);
        for (reply, _) in self.replies.drain(..) {
            let _ = reply.send(Err(SequenceError::Storage(io::Error::new(e.kind(), e.to_string()))));
        }
        self.fail_receipts();
        self.close(Disconnect::Error, now);
    }

    /// The reference for [`frame_into`](Self::frame_into): the framed message as a `Message`.
    #[cfg(test)]
    fn frame(&self, body: &Message, seq: u64, sending_time: impl ToFix, orig_sending_time: Option<&str>) -> Message {
        let target = self.peer().id.target_comp_id.as_str();
        let in_body = |tag| !is_header_or_trailer(tag);
        // Size the message up front: the header's variable parts, plus fixed room for the tags,
        // MsgType, MsgSeqNum (up to 20 digits), SendingTime and PossDupFlag, plus the body as-is.
        let header_bytes = 96
            + self.config.begin_string.len()
            + self.config.sender_comp_id.len()
            + target.len()
            + orig_sending_time.map_or(0, str::len);
        let mut msg = Message::default();
        msg.reserve(header_bytes + body.segments_len(in_body), 8 + body.field_count());
        msg.push(tags::BEGIN_STRING, self.config.begin_string.as_str());
        msg.push(tags::MSG_TYPE, body.msg_type());
        msg.push(tags::SENDER_COMP_ID, self.config.sender_comp_id.as_str());
        msg.push(tags::TARGET_COMP_ID, target);
        msg.push(tags::MSG_SEQ_NUM, seq);
        if orig_sending_time.is_some() {
            msg.push(tags::POSS_DUP_FLAG, "Y");
        }
        // PossResend is set by applications; keep it, in the header where it belongs.
        if let Some(poss_resend) = body.get(tags::POSS_RESEND) {
            msg.push(tags::POSS_RESEND, poss_resend);
        }
        msg.push(tags::SENDING_TIME, sending_time);
        if let Some(orig) = orig_sending_time {
            msg.push(tags::ORIG_SENDING_TIME, orig);
        }
        // ApplVerID(1128) on FIXT sessions, where it isn't the default (and so goes unstated).
        if let (Some(default), Some(stated)) = (self.appl_ver_id(), body.get(tags::APPL_VER_ID))
            && stated != default.code()
        {
            msg.push(tags::APPL_VER_ID, stated);
        }
        // Header fields the sender set, such as routing: kept, in the header.
        msg.extend_from(body, |tag| is_header_or_trailer(tag) && !set_by_session(tag));
        msg.extend_from(body, in_body);
        msg
    }

    /// Appends `body` framed as a message to send, encoded, to `out`, in one pass: BeginString and
    /// BodyLength; the standard header in order (MsgType, the CompIDs, MsgSeqNum, PossDupFlag and
    /// OrigSendingTime when `orig_sending_time` marks it a possible duplicate, PossResend if
    /// `body` has it, SendingTime); on FIXT sessions, ApplVerID(1128) only when `body` states one
    /// that isn't the default; header fields the sender set, such as routing, CstmApplVerID(1129)
    /// and ApplExtID(1156); then the body fields, and CheckSum. Header fields the session sets
    /// itself are taken from it, not `body`. With `state_version`, ApplVerID(1128) is written even
    /// when it's the session's default (for the stored copy of a multi-version session).
    fn frame_into(
        &mut self,
        body: &Message,
        seq: u64,
        sending_time: impl ToFix,
        orig_sending_time: Option<&str>,
        state_version: bool,
        out: &mut Vec<u8>,
    ) {
        fn field(header: &mut String, prefix: &str, value: impl ToFix) {
            header.push_str(prefix);
            value.write_fix(header);
            header.push('\x01');
        }
        // The header after BodyLength; reused so that it doesn't allocate.
        let mut header = std::mem::take(&mut self.header);
        header.clear();
        field(&mut header, "35=", body.msg_type());
        field(&mut header, "49=", self.config.sender_comp_id.as_str());
        field(&mut header, "56=", self.peer().id.target_comp_id.as_str());
        field(&mut header, "34=", seq);
        if orig_sending_time.is_some() {
            field(&mut header, "43=", "Y");
        }
        if let Some(poss_resend) = body.get(tags::POSS_RESEND) {
            field(&mut header, "97=", poss_resend);
        }
        field(&mut header, "52=", sending_time);
        if let Some(orig) = orig_sending_time {
            field(&mut header, "122=", orig);
        }
        let version = match (self.appl_ver_id(), body.get(tags::APPL_VER_ID)) {
            (Some(default), Some(stated)) if state_version || stated != default.code() => Some(stated),
            (Some(default), None) if state_version => Some(default.code()),
            _ => None,
        };
        if let Some(version) = version {
            field(&mut header, "1128=", version);
        }

        let kept_header = |tag| is_header_or_trailer(tag) && !set_by_session(tag);
        let in_body = |tag| !is_header_or_trailer(tag);
        // One pass for the length of both, noting whether the body has header fields of its own
        // (usually not), so their write pass can be skipped.
        let any_kept_header = Cell::new(false);
        let body_len = header.len()
            + body.segments_len(|tag| {
                let kept = kept_header(tag);
                any_kept_header.set(any_kept_header.get() | kept);
                kept || in_body(tag)
            });
        let begin_string = self.config.begin_string.as_bytes();
        let start = out.len();
        out.reserve(begin_string.len() + body_len + 24);
        out.extend_from_slice(b"8=");
        out.extend_from_slice(begin_string);
        out.extend_from_slice(b"\x019=");
        push_digits(out, body_len);
        out.push(b'\x01');
        out.extend_from_slice(header.as_bytes());
        if any_kept_header.get() {
            body.write_segments(out, kept_header);
        }
        body.write_segments(out, in_body);
        push_trailer(out, start);
        self.header = header;
    }

    /// The wall-clock time for this call into the session: `config.clock` read once, for both
    /// checking inbound SendingTime(52) and stamping outbound. A call is over in microseconds.
    fn wall_clock(&self) -> DateTime<Utc> {
        if let Some(now) = self.wall_clock.get() {
            return now;
        }
        let now = self.config.clock.now();
        self.wall_clock.set(Some(now));
        now
    }

    fn peer(&self) -> &Peer {
        self.peer.as_ref().expect("session is not bound before logon")
    }

    /// Whether messages from the counterparty are being processed: logged on, or logging out.
    fn receiving(&self) -> bool {
        matches!(self.status, Status::Active | Status::LoggingOut { .. })
    }

    /// Where the store's in-flight marker must start once the next incoming number is `next`: the
    /// window recovered from before a restart while messages in it may still come back, so a
    /// second crash finds it too; otherwise the open window's start, if one is open.
    fn marker(&self, next: u64) -> Option<u64> {
        self.live_recovered(next).or_else(|| self.window_end.map(|end| end - DELIVERIES_PER_COMMIT))
    }

    /// The window recovered from before a restart, while messages in it may still come back.
    fn live_recovered(&self, next: u64) -> Option<u64> {
        self.recovered.filter(|start| next < start + DELIVERIES_PER_COMMIT)
    }

    /// Whether incoming `seq` may be handed to the application: it's in the committed window.
    fn covers(&self, seq: u64) -> bool {
        self.window_end.is_some_and(|end| seq < end)
    }

    /// Asks the store to commit, recording a window of incoming messages in flight first if
    /// input waits for one.
    fn begin_commit(&mut self) -> io::Result<Option<Commit>> {
        debug_assert!(!self.committing && self.opening.is_none());
        let receiving = self.receiving();
        let Some(peer) = self.peer.as_mut().filter(|_| !self.store_failed) else {
            // Nothing is stored: the reply to a Logon that was refused, or a store that failed.
            return Ok(None);
        };
        let next = peer.log.next_incoming();
        // A recovered window still in use is kept: a new one starts where it does.
        let recovered = self.recovered.filter(|start| next < start + DELIVERIES_PER_COMMIT);
        if receiving {
            // Every commit records the window the next batch is handed over in, so that batch
            // needs no commit of its own first. Unchanged, it costs nothing.
            let start = recovered.unwrap_or(next);
            if peer.log.in_flight() != Some(start) {
                peer.log.set_in_flight(start)?;
            }
            self.opening = Some(start);
        } else if self.status == Status::Closed && recovered.is_none() && peer.log.in_flight().is_some() {
            // The session has ended cleanly, so nothing is in flight.
            peer.log.set_next_incoming(next)?;
        }
        peer.log.commit()
    }

    /// The commit is durable: what it covers may be written, and the window it opened (if any)
    /// replaces the last, which it closed. Messages that waited for a gap and then for the window
    /// are handled, and returns true: what they did is to be committed next. Otherwise operators
    /// hear of their changes.
    fn finish_commit(&mut self, now: Instant) -> bool {
        debug_assert!(!self.committing, "a commit has ended before it's finished");
        self.committed = self.output.len();
        self.window_end = self.opening.take().map(|start| start + DELIVERIES_PER_COMMIT);
        if self.store_failed {
            // Nothing was committed: the store failed, and the session is closing.
            self.fail_receipts();
        }
        for (receipt, seq) in self.receipts.drain(..) {
            let _ = receipt.send(Ok(seq));
        }
        if self.window_end.is_some() && std::mem::take(&mut self.drain_waiting) {
            self.after_incoming(now);
            return true;
        }
        self.send_replies();
        false
    }

    /// Operators hear of their changes, committed now, with the numbers as they stand (a new
    /// incoming number may have let queued messages be handled since).
    fn send_replies(&mut self) {
        if self.replies.is_empty() {
            return;
        }
        if self.store_failed {
            // The change, made after the store failed, can't have been committed.
            for (reply, _) in self.replies.drain(..) {
                let _ = reply.send(Err(SequenceError::Storage(io::Error::other("the session store has failed"))));
            }
            return;
        }
        let peer = self.peer.as_ref().expect("operator changes need a bound session");
        let now = SequenceNumbers { next_incoming: peer.log.next_incoming(), next_outgoing: peer.log.next_outgoing() };
        for (reply, result) in self.replies.drain(..) {
            let _ = reply.send(result.map(|_| now));
        }
    }

    /// For a driver that feeds a message without asking [`ready_for_input`](Self::ready_for_input):
    /// opens the window now, running the store's commit on this thread if it returns one.
    fn open_window_now(&mut self, now: Instant) {
        debug!("committing the in-flight window on the calling thread");
        match self.begin_commit() {
            Ok(None) => {
                self.finish_commit(now);
            }
            Ok(Some(commit)) => {
                self.committing = true;
                self.on_committed(commit.run(), now);
            }
            Err(e) => {
                self.opening = None;
                self.storage_failed(e, now);
            }
        }
    }

    fn peer_mut(&mut self) -> &mut Peer {
        self.peer.as_mut().expect("session is not bound before logon")
    }
}

/// An encoded message displayed for the log, decoded, with its passwords masked (see
/// [`Message::redacted`]).
struct Outbound<'a>(&'a [u8], &'a DataFields);

impl fmt::Display for Outbound<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match decode_stored(self.0, self.1) {
            Decoded::Message(msg, _) => write!(f, "{}", msg.redacted()),
            // Never the bytes themselves: redaction needs the fields.
            _ => write!(f, "<{} bytes that don't decode>", self.0.len()),
        }
    }
}

/// Silence after which the counterparty is sent a TestRequest: HeartBtInt plus 20% for
/// transmission delay.
fn probe_after(heartbeat: Duration) -> Duration {
    heartbeat.saturating_add(heartbeat / 5)
}

/// How long an outstanding ResendRequest may go without progress.
fn resend_timeout(heartbeat: Duration) -> Duration {
    heartbeat.saturating_mul(2)
}

fn logon_message(
    heartbeat: Duration,
    reset: bool,
    next_expected: Option<u64>,
    appl_ver_id: Option<ApplVerId>,
) -> Logon {
    Logon {
        encrypt_method: EncryptMethod::None,
        heart_bt_int: heartbeat.as_secs(),
        reset_seq_num_flag: reset.then_some(true),
        next_expected_msg_seq_num: next_expected,
        username: None,
        password: None,
        default_appl_ver_id: appl_ver_id,
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.fail_receipts();
        self.discard_pending();
        // Dropped mid-logout, the logout says how the session ended. The clock is read here, not
        // taken from the last call: a session is dropped without `on_disconnect` when its task is
        // aborted or its future dropped (an initiator's, or a shutdown giving up), and its last
        // call can be a heartbeat interval old, which would end a short grace period at once.
        // It's tokio's clock, which the task driving the countdowns sleeps on: under paused time,
        // a deadline from the system clock would look long overdue. Outside a runtime it's the
        // system clock.
        let now = tokio::time::Instant::now().into_std();
        self.notify_logout(self.ending.unwrap_or(Disconnect::ConnectionLost), now);
        // Dropped while the store opened its log (the connection failed): the log, if the job
        // still opens it, is closed when the job's result is dropped.
        if let Some(opening) = self.opening_log.take() {
            self.registry.release(&opening.id, &self.commands);
        }
        if let Some(peer) = self.peer.take() {
            peer.metrics.disconnected();
            // Close the log (releasing any file lock) before another connection can acquire it.
            drop(peer.log);
            self.registry.release(&peer.id, &self.commands);
        }
    }
}

/// The first tag in `msg` whose value is empty. FIX doesn't allow empty values.
/// Header fields the session writes itself when framing a message, so any in the body it was
/// given are dropped: the framing fields, identity, sequencing and timestamps, and PossResend and
/// ApplVerID(1128), which it places itself.
fn set_by_session(tag: u32) -> bool {
    use tags::*;
    matches!(
        tag,
        BEGIN_STRING
            | BODY_LENGTH
            | CHECK_SUM
            | MSG_TYPE
            | SENDER_COMP_ID
            | TARGET_COMP_ID
            | MSG_SEQ_NUM
            | POSS_DUP_FLAG
            | POSS_RESEND
            | SENDING_TIME
            | ORIG_SENDING_TIME
            | APPL_VER_ID
    )
}

/// Why a message can't belong to this session.
enum HeaderMismatch {
    BeginString(String),
    CompId(String),
}

impl fmt::Display for HeaderMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeginString(text) | Self::CompId(text) => f.write_str(text),
        }
    }
}

/// The first header field every message must have that this one lacks. BeginString, BodyLength,
/// MsgType and CheckSum are checked by the codec, and a missing MsgSeqNum ends the session.
fn missing_header_field(msg: &Message) -> Option<u32> {
    [tags::SENDER_COMP_ID, tags::TARGET_COMP_ID, tags::SENDING_TIME].into_iter().find(|tag| msg.get(*tag).is_none())
}

/// In one pass: the first header field that comes after a body field, if `check_order` (CheckSum
/// ends every message), and the first field without a value.
fn misplaced_header_and_empty_field(msg: &Message, check_order: bool) -> (Option<u32>, Option<u32>) {
    let (mut misplaced, mut empty) = (None, None);
    let mut in_body = false;
    for (tag, value) in msg.fields() {
        if empty.is_none() && value.is_empty() {
            empty = Some(tag);
        }
        if check_order && misplaced.is_none() && tag != tags::CHECK_SUM {
            match (is_header_or_trailer(tag), in_body) {
                (true, true) => misplaced = Some(tag),
                (false, _) => in_body = true,
                (true, false) => {}
            }
        }
        if empty.is_some() && (misplaced.is_some() || !check_order) {
            break;
        }
    }
    (misplaced, empty)
}

fn empty_field(msg: &Message) -> Option<u32> {
    msg.fields().find(|(_, value)| value.is_empty()).map(|(tag, _)| tag)
}

#[cfg(test)]
mod tests;
