//! FIX session layer: logon, sequencing, heartbeats, resends and logout.
//!
//! [`Session`] is a sans-IO state machine that plays either the acceptor or the initiator role.
//! A driver (see [`crate::connection`]) feeds it inbound messages and handle commands, calls its
//! timer when its next deadline falls due, writes the encoded messages it leaves in
//! [`Session::output`], and closes the connection once [`Session::is_closed`], so the protocol
//! logic is deterministic and testable without sockets. Sequence numbers and sent messages are
//! persisted through a [`SessionLog`] before the corresponding message is handed to the driver.
//! Application messages are delivered to an [`Application`].

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::admin::{
    BusinessMessageReject, Heartbeat, Logon, Logout, Reject, ResendRequest, SequenceReset, TestRequest,
};
use crate::application::{Application, Context, MessageReject};
use crate::codec::{Decoded, decode_stored, frame_stored, push_digits, push_trailer};
use crate::fields::{
    ApplVerId, BusinessRejectReason, EncryptMethod, MsgType, Precision, Secret, SessionRejectReason, ToFix,
    UtcTimestamp,
};
use crate::initiator::InitiatorConfig;
use crate::message::{DataFields, FieldError, Message, is_header_or_trailer, tags};
use crate::peer::ConnectionInfo;
use crate::registry::{
    Command, CommandReceiver, CommandSender, SequenceCommand, SequenceError, SequenceNumbers, SessionRegistry,
    apply_sequence_command,
};
use crate::schedule::{Clock, Period, SessionSchedule};
use crate::store::{SessionId, SessionLog};
use crate::telemetry::SessionMetrics;

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

/// Settings shared by both roles.
#[derive(Debug, Clone)]
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
    /// FIXT.1.1 only: the application versions (DefaultApplVerID(1137)) this session supports; see
    /// [`with_appl_ver_id`](Self::with_appl_ver_id).
    pub appl_versions: Vec<ApplVersion>,
    /// FIX 4.x sessions: checks inbound application messages against a data dictionary before
    /// delivering them (feature `validation`); see [`with_dictionary`](Self::with_dictionary).
    /// FIXT sessions have one per application version instead.
    #[cfg(feature = "validation")]
    pub validator: Option<Arc<crate::validation::Validator>>,
}

impl SessionConfig {
    /// A configuration for BeginString `begin_string` with our CompID `sender_comp_id`: 10 seconds
    /// to log on and 5 to log out, no schedule, the system clock, SendingTime within 120 seconds,
    /// and OrigSendingTime and header order checked.
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
            appl_versions: Vec::new(),
            #[cfg(feature = "validation")]
            validator: None,
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

    /// Why this configuration can't run a session, if it can't: a FIXT session needs an
    /// application version, each configured once, and a FIX 4.x session none.
    pub fn check(&self) -> Result<(), String> {
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
        if let Err(e) = self.check() {
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
    log: Box<dyn SessionLog>,
    heartbeat: Duration,
    metrics: SessionMetrics,
}

/// What an acceptor needs from a valid Logon.
struct LogonRequest {
    comp_id: String,
    heartbeat: Duration,
    seq_num: u64,
    their_next: Option<u64>,
    /// FIXT sessions: the configured version matching their DefaultApplVerID(1137).
    appl_version: Option<ApplVersion>,
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

/// A resend in progress: what's left of the range asked for.
struct Replay {
    /// The first sequence number not yet resent or gap-filled: an open gap fill starts here.
    next: u64,
    /// The first sequence number not yet looked up in the store.
    scan: u64,
    /// The last sequence number to resend.
    end: u64,
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
    status: Status,
    /// Set once the session is bound to its log: on accepting Logon, or on connect as initiator.
    peer: Option<Peer>,
    /// Whether the application has been told about the logon (and so is owed an `on_logout`).
    app_logged_on: bool,
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
    /// The resend in progress, if any; see [`on_resume`](Self::on_resume).
    replay: Option<Replay>,
    /// New messages sent while a resend is in progress, framed and stored, to follow it. Drivers
    /// feed no input meanwhile, so it holds what the call that started the resend went on to
    /// send: replies to one message, to those queued behind a gap, or the commands held during
    /// logon.
    held: Vec<u8>,
    /// Sequence numbers resent per step: `MAX_RESEND_BATCH`, a field so tests can shrink it.
    resend_batch: u64,
    /// Handle commands received while logon is in progress, applied in order once it completes.
    /// The logon timeout bounds how long it fills, but not how much: it's as unbounded as the
    /// handle's channel (ROADMAP "Bound the session command queue").
    pending: Vec<Command>,
    /// Scratch space for [`frame_into`](Self::frame_into)'s header, kept to reuse its allocation.
    header: String,
    /// Scratch space for the stored copy of a message, when it differs from the one sent.
    stored: Vec<u8>,
    /// `config.clock`'s time, read at most once per call into the session (see
    /// [`wall_clock`](Self::wall_clock)).
    wall_clock: Cell<Option<DateTime<Utc>>>,
}

impl Session {
    /// A session that waits for a counterparty's Logon. The receiver carries
    /// [`SessionHandle`](crate::SessionHandle) commands and must be fed to [`Session::on_command`].
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
        let (commands, receiver) = mpsc::unbounded_channel();
        let session = Self {
            config,
            role,
            registry,
            app,
            commands,
            appl_version,
            connection: ConnectionInfo::default(),
            status: Status::AwaitingLogon,
            peer: None,
            app_logged_on: false,
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
            replay: None,
            held: Vec::new(),
            resend_batch: MAX_RESEND_BATCH,
            pending: Vec::new(),
            header: String::new(),
            wall_clock: Cell::new(None),
            stored: Vec::new(),
        };
        (session, receiver)
    }

    /// Describes the transport (remote address, TLS peer certificates) for
    /// [`Application::verify_logon`]. Set it before feeding the session any messages.
    pub fn set_connection_info(&mut self, connection: ConnectionInfo) {
        self.connection = connection;
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
    /// [`clear_output`](Self::clear_output): it grows until cleared. Once
    /// [`is_closed`](Self::is_closed), write them, then close the connection.
    pub fn output(&self) -> &[u8] {
        &self.output
    }

    /// Empties [`output`](Self::output), keeping its capacity, once it's been written.
    pub fn clear_output(&mut self) {
        self.output.clear()
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

    /// A message was decoded from the transport.
    pub fn on_message(&mut self, msg: &Message, now: Instant) {
        self.wall_clock.set(None);
        self.last_received = now;
        self.test_request_sent = None;
        match self.status {
            Status::AwaitingLogon => self.on_logon(msg, now),
            Status::Active | Status::LoggingOut { .. } => {
                self.on_session_message(msg, now, true);
                self.after_incoming(now);
            }
            Status::Closed => {}
        }
        if let Some(peer) = &self.peer {
            peer.metrics.message_received();
        }
    }

    /// Carries out a command from a [`SessionHandle`](crate::SessionHandle).
    ///
    /// Commands that arrive while logon is in progress are queued and applied, in order, as soon
    /// as it completes. Once logout has started, sends are dropped and logged.
    pub fn on_command(&mut self, command: Command, now: Instant) {
        self.wall_clock.set(None);
        // Operator requests are answered straight away, even mid-logon: they must not wait in the
        // queue for a logon that may never complete.
        let command = match command {
            Command::Sequence(request, reply) => {
                let _ = reply.send(self.apply_sequence(request, now));
                return;
            }
            other => other,
        };
        if self.status == Status::AwaitingLogon {
            debug!(queued = self.pending.len() + 1, "logon in progress; queueing handle command");
            self.pending.push(command);
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
            Status::Active => self.logout(text, now),
            Status::AwaitingLogon => {
                info!("shutting down before logon; disconnecting");
                self.close();
            }
            Status::LoggingOut { .. } | Status::Closed => {}
        }
    }

    fn apply_command(&mut self, command: Command, now: Instant) {
        match command {
            Command::Send(msg) if self.status == Status::Active => self.send_app(msg, now),
            Command::Send(msg) => warn!(msg_type = %msg.msg_type(), "dropping message: session is logging out"),
            Command::Logout(text) if self.status == Status::Active => self.logout(text.as_deref(), now),
            Command::Logout(_) => debug!("ignoring logout request: session is already logging out"),
            Command::Sequence(..) => unreachable!("handled in on_command"),
        }
    }

    /// An operator change to this (bound) session's sequence numbers.
    fn apply_sequence(&mut self, request: SequenceCommand, now: Instant) -> Result<SequenceNumbers, SequenceError> {
        if self.peer.is_none() {
            return Err(SequenceError::Invalid("the session has not been identified yet".into()));
        }
        match request {
            SequenceCommand::Reset => return Err(SequenceError::Connected),
            SequenceCommand::SetNextOutgoing(seq)
                if self.status == Status::Active && seq > self.peer().log.next_outgoing() =>
            {
                // Tell the counterparty to expect `seq` next, then skip to it.
                info!(new_seq_no = seq, "sending SequenceReset (reset mode)");
                self.send(SequenceReset { gap_fill_flag: None, new_seq_no: seq }.into(), now);
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
                    self.logout(Some("End of session"), now);
                }
                Status::AwaitingLogon => {
                    warn!("session period ended before logon completed");
                    self.close();
                }
                Status::LoggingOut { .. } | Status::Closed => {}
            }
        }
        match self.status {
            Status::AwaitingLogon => {
                if now.duration_since(self.logon_deadline_from) >= self.config.logon_timeout {
                    warn!("no Logon received within {:?}", self.config.logon_timeout);
                    self.close();
                }
            }
            Status::LoggingOut { since } => {
                if now.duration_since(since) >= self.config.logout_timeout {
                    warn!("no Logout reply received; disconnecting");
                    self.close();
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
    /// and while [`is_resending`](Self::is_resending).
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
                    Some(sent) => sent.checked_add(interval),
                    None => self.last_received.checked_add(probe_after(interval)),
                };
                let resend = self.resend.as_ref().and_then(|r| r.progress_at.checked_add(resend_timeout(interval)));
                [heartbeat, test_request, resend].into_iter().flatten().min()
            }
            Status::Closed => None,
        }
    }

    /// Keep in step with [`next_deadline`](Self::next_deadline).
    fn check_heartbeats(&mut self, now: Instant) {
        let interval = self.peer().heartbeat;
        match self.test_request_sent {
            Some(sent) if now.duration_since(sent) >= interval => {
                warn!("counterparty did not answer TestRequest; disconnecting");
                self.close();
                return;
            }
            Some(_) => {}
            None if now.duration_since(self.last_received) >= probe_after(interval) => {
                self.test_req_counter += 1;
                let id = format!("TEST{}", self.test_req_counter);
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
        let Some(resend) = &mut self.resend else { return };
        if now.duration_since(resend.progress_at) < timeout {
            return;
        }
        if resend.retried {
            let text = format!("ResendRequest from {next} unanswered");
            warn!("{text}; logging out");
            return self.logout(Some(&text), now);
        }
        resend.retried = true;
        resend.progress_at = now;
        warn!(expected = next, "ResendRequest unanswered; requesting again");
        self.send(ResendRequest { begin_seq_no: next, end_seq_no: 0 }.into(), now);
    }

    // ---- Logon ----

    /// Initiator: bind to the session log and send Logon.
    fn start_logon(&mut self, target_comp_id: String, heartbeat: Duration, reset: bool, now: Instant) {
        let id = self.session_id_for(target_comp_id);
        if let Some(reason) = self.outside_schedule() {
            warn!(session = %id, "not logging on: {reason}");
            return self.close();
        }
        if !self.bind(id, heartbeat) {
            return;
        }
        if reset && let Err(e) = self.reset_store() {
            return self.storage_failed(e);
        }
        self.logon_deadline_from = now;
        info!("sending Logon");
        let mut logon = logon_message(heartbeat, reset, None, self.appl_ver_id());
        if let Role::Initiator { next_expected, username, password, .. } = &self.role {
            logon.next_expected_msg_seq_num = next_expected.then(|| self.peer().log.next_incoming());
            logon.username = username.clone();
            logon.password = password.clone();
        }
        self.send(logon.into(), now);
    }

    fn on_logon(&mut self, msg: &Message, now: Instant) {
        if msg.msg_type() != MsgType::Logon {
            warn!("expected Logon, got MsgType '{}'; disconnecting", msg.msg_type());
            return self.close();
        }
        match self.role {
            Role::Acceptor => self.accept_logon(msg, now),
            Role::Initiator { .. } => self.complete_logon(msg, now),
        }
    }

    /// Acceptor: validate the counterparty's Logon and reply.
    fn accept_logon(&mut self, msg: &Message, now: Instant) {
        let LogonRequest { comp_id, heartbeat, seq_num, their_next, appl_version } =
            match self.validate_logon_request(msg) {
                Ok(v) => v,
                Err(reason) => {
                    warn!("refusing logon: {reason}");
                    return self.close();
                }
            };
        let id = self.session_id_for(comp_id);
        if let Some(reason) = self.outside_schedule() {
            warn!(session = %id, "refusing logon: {reason}");
            return self.close();
        }
        let verdict = guarded("verify_logon", || self.app.verify_logon(&id, msg, &self.connection));
        if let Err(reason) = verdict.unwrap_or_else(|| Err("verify_logon panicked".into())) {
            warn!(session = %id, "application refused logon: {reason}");
            return self.close();
        }
        self.appl_version = appl_version;
        if !self.bind(id, heartbeat) {
            return;
        }
        let reset = msg.flag(tags::RESET_SEQ_NUM_FLAG);
        if reset && let Err(e) = self.reset_store() {
            return self.storage_failed(e);
        }
        let expected = self.peer().log.next_incoming();
        if seq_num < expected {
            self.logout(Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")), now);
            return self.close();
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
                self.logout(Some(&text), now);
                self.close();
                false
            }
            _ => true,
        }
    }

    /// Initiator: validate the counterparty's Logon reply.
    fn complete_logon(&mut self, msg: &Message, now: Instant) {
        if let Err(mismatch) = self.check_header(msg) {
            warn!("refusing Logon reply: {mismatch}");
            return self.close();
        }
        if let Some(text) = self.sending_time_problem(msg) {
            warn!("refusing Logon reply: {text}");
            return self.close();
        }
        let identifying = [tags::SENDER_COMP_ID, tags::TARGET_COMP_ID, tags::SENDING_TIME];
        if let Some(tag) = identifying.into_iter().find(|tag| msg.get(*tag).is_none_or(str::is_empty)) {
            warn!("refusing Logon reply: tag {tag} missing or empty");
            return self.close();
        }
        let Ok(seq_num) = msg.field::<u64>(tags::MSG_SEQ_NUM) else {
            warn!("refusing Logon reply: MsgSeqNum(34) missing or invalid");
            return self.close();
        };
        if let Some(defect) = msg.defect() {
            warn!("refusing Logon reply: {}", defect.text);
            return self.close();
        }
        let reply = match msg.parse::<Logon>() {
            Ok(reply) => reply,
            Err(e) => {
                warn!("refusing Logon reply: {e}");
                return self.close();
            }
        };
        if let Some(tag) = empty_field(msg) {
            warn!("refusing Logon reply: tag {tag} specified without a value");
            return self.close();
        }
        if self.config.is_fixt() && reply.default_appl_ver_id != self.appl_ver_id() {
            let expected = self.appl_ver_id().map_or("none", ApplVerId::code);
            let received = reply.default_appl_ver_id.map_or("none", ApplVerId::code);
            warn!("refusing Logon reply: DefaultApplVerID(1137) must be '{expected}', not '{received}'");
            return self.close();
        }
        let verdict = guarded("verify_logon", || self.app.verify_logon(&self.peer().id, msg, &self.connection));
        if let Err(reason) = verdict.unwrap_or_else(|| Err("verify_logon panicked".into())) {
            warn!("application refused Logon reply: {reason}");
            return self.close();
        }
        let expected = self.peer().log.next_incoming();
        if seq_num < expected {
            self.logout(Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")), now);
            return self.close();
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
        self.status = Status::Active;
        info!(heartbeat = ?self.peer().heartbeat, "logged on");
        if seq_num > expected {
            if !await_resend {
                self.request_resend(expected, seq_num, now);
            }
        } else {
            self.set_next_incoming(expected + 1);
        }
        if self.status == Status::Active {
            self.ever_logged_on = true;
            self.app_logged_on = true;
            self.peer().metrics.logged_on();
            let handle = self.registry.handle(self.peer().id.clone());
            guarded("on_logon", || self.app.on_logon(handle));
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
        if !(1..=3600).contains(&logon.heart_bt_int) {
            return Err("HeartBtInt(108) must be between 1 and 3600".into());
        }
        let seq_num = msg.field::<u64>(tags::MSG_SEQ_NUM).map_err(|e| e.to_string())?;
        let appl_version = if self.config.is_fixt() {
            let id = logon.default_appl_ver_id.ok_or("DefaultApplVerID(1137) missing")?;
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
        Ok(LogonRequest {
            comp_id: comp_id.to_string(),
            heartbeat: Duration::from_secs(logon.heart_bt_int),
            seq_num,
            their_next: logon.next_expected_msg_seq_num,
            appl_version,
        })
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

    /// Claims the session in the registry and opens its log. Closes on failure.
    fn bind(&mut self, id: SessionId, heartbeat: Duration) -> bool {
        match self.registry.acquire(&id, self.commands.clone(), self.appl_ver_id()) {
            Ok(log) => {
                // Label the driver's span (see `connection::run`), so every later log line, from
                // the engine or the application, carries the session ID.
                tracing::Span::current().record("id", tracing::field::display(&id));
                let metrics = SessionMetrics::new(&id);
                self.peer = Some(Peer { id, log, heartbeat, metrics });
                if let Err(e) = self.start_period() {
                    self.storage_failed(e);
                    return false;
                }
                self.update_sequence_gauges();
                true
            }
            Err(reason) => {
                warn!("refusing logon: {reason}");
                self.close();
                false
            }
        }
    }

    /// Why the schedule forbids logging on now, if it does.
    fn outside_schedule(&self) -> Option<String> {
        let schedule = self.config.schedule.as_ref()?;
        let now = self.config.clock.now();
        if schedule.is_active(now) {
            return None;
        }
        Some(match schedule.next_start(now) {
            Some(next) => format!("outside session time (schedule '{schedule}'); next session starts {next}"),
            None => format!("outside session time (schedule '{schedule}')"),
        })
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
        log.set_created_at(now.into())?;
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
    #[expect(clippy::too_many_lines, reason = "see ROADMAP: split long functions")]
    fn on_session_message(&mut self, msg: &Message, now: Instant, arrived: bool) {
        match self.check_header(msg) {
            Ok(()) => {}
            // Session test case 2i: Logout, without a Reject.
            Err(HeaderMismatch::BeginString(text)) => {
                warn!("{text}; logging out");
                self.logout(Some(&text), now);
                return self.close();
            }
            // Session test case 2k: Reject, then Logout.
            Err(HeaderMismatch::CompId(text)) => {
                warn!("{text}; logging out");
                self.reject(msg, None, Some(SessionRejectReason::CompIDProblem), &text, now);
                self.logout(Some(&text), now);
                return self.close();
            }
        }
        let Ok(seq_num) = msg.field::<u64>(tags::MSG_SEQ_NUM) else {
            self.logout(Some("MsgSeqNum(34) missing or invalid"), now);
            return self.close();
        };
        // Session test case 2o: Reject, then Logout. The message still takes its number.
        if arrived && let Some(text) = self.sending_time_problem(msg) {
            warn!("{text}; logging out");
            let reason = Some(SessionRejectReason::SendingTimeAccuracyProblem);
            self.reject(msg, Some(tags::SENDING_TIME), reason, &text, now);
            if seq_num == self.peer().log.next_incoming() {
                self.set_next_incoming(seq_num + 1);
            }
            return self.logout(Some(&text), now);
        }
        let mtype = msg.msg_type();
        // Intraday reset: a Logon with ResetSeqNumFlag=Y and MsgSeqNum 1 while logged on. Any
        // other Logon goes on to be refused as usual.
        if mtype == MsgType::Logon
            && seq_num == 1
            && msg.flag(tags::RESET_SEQ_NUM_FLAG)
            && self.status == Status::Active
            && msg.defect().is_none()
        {
            return self.intraday_reset(msg, now);
        }
        // Reset mode ignores MsgSeqNum entirely.
        if mtype == MsgType::SequenceReset && !msg.flag(tags::GAP_FILL_FLAG) {
            if self.reject_defect(msg, now) {
                return;
            }
            return self.on_sequence_reset(msg, now);
        }

        let expected = self.peer().log.next_incoming();
        if seq_num > expected {
            // Session test case 1a: a counterparty that's leaving is unlikely to resend first,
            // so its Logout is answered now. The next logon finds the gap again.
            if mtype == MsgType::Logout {
                return self.on_logout_message(msg, now);
            }
            // Answer their ResendRequest now so both sides can recover from a mutual gap.
            let answered = mtype == MsgType::ResendRequest && msg.defect().is_none();
            if answered {
                self.on_resend_request(msg, now);
            }
            self.queue(seq_num, msg, answered);
            if self.resend.is_none() {
                self.request_resend(expected, seq_num, now);
            }
            return;
        }
        if seq_num < expected {
            if msg.flag(tags::POSS_DUP_FLAG) {
                // Session test cases 2f and 2g: a duplicate is still checked, but doesn't take a
                // number.
                if self.reject_orig_sending_time(msg, now) {
                    return;
                }
                debug!(seq_num, "ignoring possible duplicate");
                return;
            }
            self.logout(Some(&format!("MsgSeqNum too low, expecting {expected} but received {seq_num}")), now);
            return self.close();
        }

        // A defective gap fill falls through: it uses its number and is rejected like any other.
        if mtype == MsgType::SequenceReset && msg.defect().is_none() {
            return self.on_gap_fill(msg, seq_num, now);
        }
        self.handle_in_sequence(msg, seq_num, now);
        // Saved only now that the message is handled and anything sent in reply is stored: if
        // the process stops first, the counterparty resends it (at-least-once delivery). Not once
        // the store has failed, when an application message may not have been delivered.
        if !self.store_failed {
            self.set_next_incoming(expected + 1);
        }
    }

    /// Checks and acts on a message that arrived in sequence, before its number is saved.
    fn handle_in_sequence(&mut self, msg: &Message, seq_num: u64, now: Instant) {
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
        self.dispatch(msg, seq_num, now);
    }

    fn dispatch(&mut self, msg: &Message, seq_num: u64, now: Instant) {
        match msg.msg_type() {
            MsgType::Heartbeat => {}
            MsgType::TestRequest => match msg.parse::<TestRequest>() {
                Ok(request) => self.send(Heartbeat { test_req_id: Some(request.test_req_id) }.into(), now),
                Err(e) => self.reject_field(msg, e, now),
            },
            MsgType::ResendRequest => self.on_resend_request(msg, now),
            MsgType::Reject => warn!(
                ref_seq_num = ?msg.get(tags::REF_SEQ_NUM),
                text = ?msg.get(tags::TEXT),
                "counterparty rejected a message"
            ),
            MsgType::Logout => self.on_logout_message(msg, now),
            MsgType::Logon => self.reject(msg, None, None, "Session is already logged on", now),
            _ => self.deliver(msg, seq_num, now),
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
                self.logout(Some(text), now);
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
            return self.storage_failed(e);
        }
        self.resend = None;
        self.queued.clear();
        self.set_next_incoming(2);
        let our_next = msg.get(tags::NEXT_EXPECTED_MSG_SEQ_NUM).is_some().then_some(2);
        let heartbeat = self.peer().heartbeat;
        self.send(logon_message(heartbeat, true, our_next, self.appl_ver_id()).into(), now);
    }

    /// The counterparty's Logout: a request to answer, or the reply to ours.
    fn on_logout_message(&mut self, msg: &Message, now: Instant) {
        if self.status == Status::Active {
            info!(text = ?msg.get(tags::TEXT), "counterparty logged out");
            self.send(Logout { text: None }.into(), now);
        } else {
            info!("logout confirmed");
        }
        self.close();
    }

    /// Keeps a message that arrived ahead of a gap until its turn; the first one kept for each
    /// number wins, so an original isn't replaced by its resend.
    fn queue(&mut self, seq_num: u64, msg: &Message, answered: bool) {
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
            let Some(entry) = self.queued.first_entry() else { break };
            if *entry.key() > next {
                break;
            }
            let (seq_num, queued) = entry.remove_entry();
            if seq_num < next {
                continue; // Covered since, by a gap fill or an operator.
            }
            if queued.answered {
                self.set_next_incoming(seq_num + 1);
            } else {
                // Its SendingTime was checked when it arrived.
                self.on_session_message(&queued.msg, now, false);
            }
        }
        let Some(peer) = &self.peer else { return };
        let next = peer.log.next_incoming();
        if matches!(self.status, Status::Active | Status::LoggingOut { .. }) {
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
        if self.resend.is_none()
            && matches!(self.status, Status::Active | Status::LoggingOut { .. })
            && let Some(&ahead) = self.queued.keys().next()
        {
            self.request_resend(next, ahead, now);
        }
    }

    /// Hands an application message to the application and sends its replies or reject.
    fn deliver(&mut self, msg: &Message, seq_num: u64, now: Instant) {
        // Marked in flight first: still marked after a crash, it tells the next connection that
        // the resend of this message may have been handled already.
        let redelivered = self.peer().log.in_flight() == Some(seq_num);
        if redelivered {
            info!(seq_num, "delivering a message that may have been handled before a restart");
        } else if let Err(e) = self.peer_mut().log.set_in_flight(seq_num) {
            return self.storage_failed(e);
        }
        // Borrows the peer field alone (not `self.peer()`), so the application can be called
        // without cloning the SessionId for every message.
        let id = &self.peer.as_ref().expect("session is not bound before logon").id;
        let mut ctx = Context::new(id);
        if redelivered {
            ctx = ctx.redelivery();
        }
        let Some(result) = guarded("on_message", || self.app.on_message(&mut ctx, msg)) else {
            // The message counts as received, so say it wasn't processed. Replies queued before
            // the panic may be half-done, so they're dropped.
            let reason = BusinessRejectReason::ApplicationNotAvailable;
            return self.business_reject(msg, seq_num, reason, "Application error".into(), now);
        };
        for reply in ctx.replies {
            self.send_app(reply, now);
        }
        match result {
            Ok(()) => {}
            Err(MessageReject::Session { ref_tag, reason, text }) => {
                self.reject(msg, ref_tag, Some(reason), &text, now)
            }
            Err(MessageReject::Business { reason, text }) => self.business_reject(msg, seq_num, reason, text, now),
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
        self.peer().metrics.business_reject();
        let reply = BusinessMessageReject {
            ref_seq_num: Some(seq_num),
            ref_msg_type: msg.msg_type(),
            business_reject_reason: reason,
            text: Some(text),
        };
        let reply = self.with_ref_appl_version(Message::from(reply).with_reverse_route(msg), msg);
        self.send(reply, now);
    }

    fn on_sequence_reset(&mut self, msg: &Message, now: Instant) {
        let reset: SequenceReset = match msg.parse() {
            Ok(reset) => reset,
            Err(e) => return self.reject_field(msg, e, now),
        };
        let expected = self.peer().log.next_incoming();
        if reset.new_seq_no >= expected {
            info!(from = expected, to = reset.new_seq_no, "sequence reset");
            self.set_next_incoming(reset.new_seq_no);
        } else {
            let text = format!("NewSeqNo {} is lower than expected {expected}", reset.new_seq_no);
            self.reject(msg, Some(tags::NEW_SEQ_NO), Some(SessionRejectReason::ValueIsIncorrect), &text, now);
        }
    }

    fn on_gap_fill(&mut self, msg: &Message, seq_num: u64, now: Instant) {
        match msg.parse::<SequenceReset>() {
            Ok(fill) if fill.new_seq_no > seq_num => {
                debug!(from = seq_num, to = fill.new_seq_no, "gap fill");
                self.set_next_incoming(fill.new_seq_no);
            }
            parsed => {
                self.set_next_incoming(seq_num + 1);
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
        debug_assert!(begin <= end);
        // Only a driver that feeds messages during a resend gets here with one in progress. The
        // new range replaces it; what was held is stored, so it's resent if the range covers it,
        // and otherwise the counterparty finds the gap at our next message.
        if self.replay.is_some() {
            self.held.clear();
        }
        self.replay = Some(Replay { next: begin, scan: begin, end });
        self.resend_step(now);
    }

    /// Resends the next `resend_batch` sequence numbers of the replay, and ends it after the last.
    fn resend_step(&mut self, now: Instant) {
        let Some(Replay { mut next, scan, end }) = self.replay.take() else { return };
        debug_assert!(next <= scan && scan <= end);
        let to = end.min(scan.saturating_add(self.resend_batch - 1));
        let originals = match self.stored_messages(scan, to) {
            Ok(originals) => originals,
            Err(e) => return self.storage_failed(e),
        };
        let now_ts = UtcTimestamp::from(self.wall_clock()).with_precision(self.config.timestamp_precision).to_fix();
        let sent_any = !originals.is_empty() || to == end;
        for (seq, original) in originals {
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
            self.replay = Some(Replay { next, scan: to + 1, end });
        } else {
            if next <= end {
                self.send_gap_fill(next, end + 1, &now_ts);
            }
            self.finish_replay(now);
        }
        if sent_any {
            self.last_sent = now;
        }
    }

    /// Stored messages `begin..=end`, parsed with the session's data fields: all of them, or an
    /// error if any is corrupt, so that none of a corrupt range is resent.
    fn stored_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Message)>> {
        // Stores keep the bytes as sent; they're parsed here.
        let stored = self.peer_mut().log.sent_messages(begin, end)?;
        let mut originals = Vec::with_capacity(stored.len());
        for (seq, bytes) in stored {
            debug_assert!((begin..=end).contains(&seq));
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
        let body = SequenceReset { gap_fill_flag: Some(true), new_seq_no }.into();
        let mut output = std::mem::take(&mut self.output);
        let start = output.len();
        self.frame_into(&body, seq, now_ts, Some(now_ts), false, &mut output);
        self.output = output;
        self.emit(&self.output, start);
    }

    fn request_resend(&mut self, from: u64, received: u64, now: Instant) {
        warn!(expected = from, received, "sequence gap detected; requesting resend");
        self.peer().metrics.sequence_gap();
        self.send(ResendRequest { begin_seq_no: from, end_seq_no: 0 }.into(), now);
        self.resend = Some(Resend { target: received, progress_at: now, seen: from, retried: false });
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
            text: Some(text.to_string()),
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

    fn logout(&mut self, text: Option<&str>, now: Instant) {
        info!(text, "logging out");
        // Shutdown isn't held up by a long resend: the counterparty asks for the rest next time.
        if self.replay.is_some() {
            info!("logging out during a resend; the rest of it isn't sent");
            self.finish_replay(now);
        }
        self.send(Logout { text: text.map(String::from) }.into(), now);
        if self.status != Status::Closed {
            self.status = Status::LoggingOut { since: now };
        }
    }

    /// Ends the session. The driver writes the output, then closes the connection, so nothing may
    /// add to the output once closed: `send` and `resend` check, and `emit` asserts it.
    fn close(&mut self) {
        if self.status != Status::Closed {
            self.status = Status::Closed;
            self.queued.clear();
            self.replay = None;
            self.held.clear();
            if let Some(peer) = &self.peer {
                peer.metrics.logged_off();
            }
            self.discard_pending();
            self.notify_logout();
        }
    }

    fn notify_logout(&mut self) {
        if std::mem::take(&mut self.app_logged_on) {
            guarded("on_logout", || self.app.on_logout(&self.peer().id));
        }
    }

    // ---- Sending ----

    /// Sends an application message; session-level message types are refused.
    fn send_app(&mut self, msg: Message, now: Instant) {
        if msg.msg_type().is_admin() {
            warn!(msg_type = %msg.msg_type(), "applications cannot send session-level messages; dropping");
            return;
        }
        if self.appl_version.is_some()
            && let Some(stated) = msg.get(tags::APPL_VER_ID)
            && self.supported_version(stated).is_none()
        {
            error!(msg_type = %msg.msg_type(), stated, "dropping message: its ApplVerID(1128) isn't one this session supports");
            return;
        }
        self.send(msg, now);
    }

    /// Assigns the next outgoing MsgSeqNum, adds the standard header, persists it and queues the
    /// message. Does nothing once the session is closed.
    fn send(&mut self, mut body: Message, now: Instant) {
        if self.status == Status::Closed {
            return;
        }
        let admin = body.msg_type().is_admin();
        // A panic may have left the message half-modified: disconnect rather than send it.
        if admin && guarded("to_admin", || self.app.to_admin(&self.peer().id, &mut body)).is_none() {
            return self.close();
        }
        // SOH ends a field on the wire, so a value containing one would add fields of its own,
        // unless it's a data field whose length is given just before it.
        if let Some(tag) = body.invalid_data_field(&self.config.data_fields) {
            warn!(
                msg_type = %body.msg_type(),
                tag,
                "dropping message: a value contains SOH, or a data field doesn't follow its length"
            );
            return;
        }
        let seq = self.peer().log.next_outgoing();
        let sending_time = UtcTimestamp::from(self.wall_clock()).with_precision(self.config.timestamp_precision);
        // During a resend, new messages wait for the rest of it.
        let holding = self.replay.is_some();
        let mut output = std::mem::take(if holding { &mut self.held } else { &mut self.output });
        let start = output.len();
        self.frame_into(&body, seq, sending_time, None, false, &mut output);
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
            self.frame_into(&body, seq, sending_time, None, true, &mut stored);
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
            return self.storage_failed(e);
        }
        debug_assert_eq!(self.peer().log.next_outgoing(), seq + 1, "recording a message uses its number");
        self.peer().metrics.next_outgoing(seq + 1);
        self.last_sent = now;
        self.emit(if holding { &self.held } else { &self.output }, start);
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

    fn set_next_incoming(&mut self, seq: u64) {
        match self.peer_mut().log.set_next_incoming(seq) {
            Ok(()) => self.peer().metrics.next_incoming(seq),
            Err(e) => self.storage_failed(e),
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

    /// The session cannot continue without durable state, so it disconnects.
    fn storage_failed(&mut self, e: io::Error) {
        error!("session store failed: {e}; disconnecting");
        self.store_failed = true;
        self.close();
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
        self.discard_pending();
        self.notify_logout();
        if let Some(peer) = self.peer.take() {
            peer.metrics.disconnected();
            // Close the log (releasing any file lock) before another connection can acquire it.
            drop(peer.log);
            self.registry.release(&peer.id);
        }
    }
}

/// Runs an application callback, catching a panic so that one failing callback can't end the
/// connection (and lose the message it was handling). Returns `None`, having logged and counted
/// the panic, if it panicked. With `panic = "abort"` the process stops instead.
fn guarded<R>(callback: &'static str, f: impl FnOnce() -> R) -> Option<R> {
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
