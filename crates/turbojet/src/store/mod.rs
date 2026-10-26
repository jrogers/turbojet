//! Session persistence: sequence numbers and sent application messages, kept per session so a
//! counterparty can reconnect and recover missed messages.

mod by_counterparty;
#[cfg(any(test, feature = "conformance"))]
pub mod conformance;
pub(crate) mod disk;
mod memory;
mod seq_index;

use std::fmt;
use std::future::Future;
use std::io;
use std::pin::Pin;

pub use by_counterparty::StorageByCounterparty;
pub use disk::DiskStorage;
pub use memory::MemoryStorage;

use crate::cancel::CancelOnDisconnect;
use crate::fields::UtcTimestamp;

/// Identifies a FIX session from the gateway's side.
///
/// Two sessions with the same CompIDs are different sessions if their SubIDs, LocationIDs or
/// qualifier differ, as in QuickFIX: an acceptor takes them from each Logon, and an initiator
/// from its [`InitiatorConfig`](crate::InitiatorConfig).
///
/// ```
/// use turbojet::SessionId;
///
/// // BeginString, our CompID, then the counterparty's: what an acceptor VENUE calls CLIENT1's session.
/// let id = SessionId::new("FIX.4.4", "VENUE", "CLIENT1").with_target_sub_id("DESK2");
/// assert_eq!(id.target_comp_id, "CLIENT1");
/// println!("{id}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SessionId {
    /// BeginString(8).
    pub begin_string: String,
    /// Our CompID.
    pub sender_comp_id: String,
    /// Our SenderSubID(50), if the session has one.
    pub sender_sub_id: Option<String>,
    /// Our SenderLocationID(142), if the session has one.
    pub sender_location_id: Option<String>,
    /// The counterparty's CompID.
    pub target_comp_id: String,
    /// The counterparty's SubID, our TargetSubID(57), if the session has one.
    pub target_sub_id: Option<String>,
    /// The counterparty's LocationID, our TargetLocationID(143), if the session has one.
    pub target_location_id: Option<String>,
    /// Tells apart sessions whose other fields are the same. Local: never sent.
    pub qualifier: Option<String>,
}

impl SessionId {
    /// The session with BeginString `begin_string`, our CompID `sender_comp_id` and the
    /// counterparty's `target_comp_id`, and no SubIDs, LocationIDs or qualifier.
    pub fn new(
        begin_string: impl Into<String>,
        sender_comp_id: impl Into<String>,
        target_comp_id: impl Into<String>,
    ) -> Self {
        Self {
            begin_string: begin_string.into(),
            sender_comp_id: sender_comp_id.into(),
            sender_sub_id: None,
            sender_location_id: None,
            target_comp_id: target_comp_id.into(),
            target_sub_id: None,
            target_location_id: None,
            qualifier: None,
        }
    }

    /// With our SenderSubID(50).
    #[must_use]
    pub fn with_sender_sub_id(mut self, id: impl Into<String>) -> Self {
        self.sender_sub_id = Some(id.into());
        self
    }

    /// With our SenderLocationID(142).
    #[must_use]
    pub fn with_sender_location_id(mut self, id: impl Into<String>) -> Self {
        self.sender_location_id = Some(id.into());
        self
    }

    /// With the counterparty's SubID, our TargetSubID(57).
    #[must_use]
    pub fn with_target_sub_id(mut self, id: impl Into<String>) -> Self {
        self.target_sub_id = Some(id.into());
        self
    }

    /// With the counterparty's LocationID, our TargetLocationID(143).
    #[must_use]
    pub fn with_target_location_id(mut self, id: impl Into<String>) -> Self {
        self.target_location_id = Some(id.into());
        self
    }

    /// With a qualifier, to tell it apart from a session whose other fields are the same.
    #[must_use]
    pub fn with_qualifier(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifier = Some(qualifier.into());
        self
    }

    /// What a store keys the session on beyond its BeginString and CompIDs: its SubIDs,
    /// LocationIDs and qualifier, as `ss=..;sl=..;ts=..;tl=..;q=..` with only those it has, in
    /// that order, each value [`escaped`](Self::escape). Empty if it has none, so such a session
    /// keeps the key it had before these fields existed.
    pub fn key_suffix(&self) -> String {
        let parts = [
            ("ss", &self.sender_sub_id),
            ("sl", &self.sender_location_id),
            ("ts", &self.target_sub_id),
            ("tl", &self.target_location_id),
            ("q", &self.qualifier),
        ];
        let present =
            parts.iter().filter_map(|(key, value)| Some(format!("{key}={}", Self::escape(value.as_deref()?))));
        present.collect::<Vec<_>>().join(";")
    }

    /// The session a store keyed on these parts, the last a [`key_suffix`](Self::key_suffix):
    /// `None` if `suffix` isn't one.
    pub fn from_key(
        begin_string: impl Into<String>,
        sender_comp_id: impl Into<String>,
        target_comp_id: impl Into<String>,
        suffix: &str,
    ) -> Option<Self> {
        let mut id = Self::new(begin_string, sender_comp_id, target_comp_id);
        for part in suffix.split(';').filter(|part| !part.is_empty()) {
            let (key, value) = part.split_once('=')?;
            let value = Some(Self::unescape(value)?);
            let field = match key {
                "ss" => &mut id.sender_sub_id,
                "sl" => &mut id.sender_location_id,
                "ts" => &mut id.target_sub_id,
                "tl" => &mut id.target_location_id,
                "q" => &mut id.qualifier,
                _ => return None,
            };
            *field = value;
        }
        // Paired with `key_suffix`: a key read back is the key it was made from.
        (id.key_suffix() == suffix).then_some(id)
    }

    /// `value` with anything other than ASCII alphanumerics, `.` and `_` percent-encoded, so it
    /// is safe in a file name and can't be confused with a separator.
    pub fn escape(value: &str) -> String {
        value
            .bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' => (b as char).to_string(),
                _ => format!("%{b:02X}"),
            })
            .collect()
    }
}

impl SessionId {
    /// Undoes [`escape`](Self::escape): `None` if `value` isn't escaped text.
    pub(crate) fn unescape(value: &str) -> Option<String> {
        let mut bytes = Vec::with_capacity(value.len());
        let mut rest = value.as_bytes();
        while let Some((&byte, after)) = rest.split_first() {
            if byte == b'%' {
                let hex = std::str::from_utf8(after.get(..2)?).ok()?;
                bytes.push(u8::from_str_radix(hex, 16).ok()?);
                rest = &after[2..];
            } else {
                bytes.push(byte);
                rest = after;
            }
        }
        String::from_utf8(bytes).ok()
    }
}

/// `FIX.4.4:GATEWAY->CLIENT`, or as QuickFIX writes one with more:
/// `FIX.4.4:GATEWAY/DESK/NY->CLIENT/TRADER7:qualifier`, a LocationID without a SubID following an
/// empty one (`GATEWAY//NY`).
impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn side(
            f: &mut fmt::Formatter<'_>,
            comp: &str,
            sub: Option<&String>,
            location: Option<&String>,
        ) -> fmt::Result {
            f.write_str(comp)?;
            if sub.is_some() || location.is_some() {
                write!(f, "/{}", sub.map_or("", String::as_str))?;
            }
            if let Some(location) = location {
                write!(f, "/{location}")?;
            }
            Ok(())
        }
        write!(f, "{}:", self.begin_string)?;
        side(f, &self.sender_comp_id, self.sender_sub_id.as_ref(), self.sender_location_id.as_ref())?;
        f.write_str("->")?;
        side(f, &self.target_comp_id, self.target_sub_id.as_ref(), self.target_location_id.as_ref())?;
        if let Some(qualifier) = &self.qualifier {
            write!(f, ":{qualifier}")?;
        }
        Ok(())
    }
}

/// Opens each session's [`StoredSession`]. Shared by all connections.
///
/// A store that opens at once (memory, local files) implements [`open`](SessionStorage::open);
/// one that waits for a network also overrides [`begin_open`](SessionStorage::begin_open), and
/// opens in `open` by running that job on the calling thread.
///
/// ```
/// use std::io;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use turbojet::store::StoredSession;
/// use turbojet::{DiskStorage, SessionId, SessionStorage};
///
/// /// Counts the sessions opened, and keeps them in a `DiskStorage`.
/// struct Counted {
///     inner: DiskStorage,
///     opened: AtomicUsize,
/// }
///
/// impl SessionStorage for Counted {
///     fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
///         self.opened.fetch_add(1, Ordering::Relaxed);
///         self.inner.open(id)
///     }
///
///     fn is_durable(&self) -> bool {
///         true
///     }
/// }
/// ```
pub trait SessionStorage: Send + Sync {
    /// Opens session `id`'s stored state, creating it with both sequence numbers at 1 if it does
    /// not exist, on this thread. The engine holds at most one open per session.
    ///
    /// # Errors
    ///
    /// Any failure opening it; the logon is refused.
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>>;

    /// Opens session `id`'s stored state as [`open`](SessionStorage::open) does: at once, or by a [`Job`]
    /// the session's driver runs off the async runtime while the logon waits. The session calls
    /// this when it learns its ID, at logon. The default opens at once with `open`.
    ///
    /// # Errors
    ///
    /// As [`open`](SessionStorage::open).
    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.open(id).map(Opened::Ready)
    }

    /// Whether what this store keeps survives the process ending: false for
    /// [`MemoryStorage`]. A store that routes sessions to others is durable
    /// only if every store it can route to is.
    fn is_durable(&self) -> bool;

    /// The sessions with any [`Marks`] set, as a [`Job`] since it may be a query: a registry
    /// runs it once, before its first logon, so pauses and cancel-on-disconnect countdowns
    /// survive a restart. The default finds none, for a store that doesn't keep marks (see
    /// [`StoredSession::marks`]).
    fn marked(&self) -> Job<Vec<(SessionId, Marks)>> {
        Job::blocking(|| Ok(Vec::new()))
    }

    /// Every session this store holds state for, in no particular order, as a [`Job`] since it
    /// may be a query: for tools that work on all of them, such as copying a store to a standby.
    /// The default fails with [`io::ErrorKind::Unsupported`].
    fn sessions(&self) -> Job<Vec<SessionId>> {
        Job::blocking(|| Err(io::Error::new(io::ErrorKind::Unsupported, "this store can't list its sessions")))
    }
}

/// A shared store is the store it shares: for one store given to several users, such as a
/// replicating store and, after a failover, the registry that takes it over.
impl<T: SessionStorage + ?Sized> SessionStorage for std::sync::Arc<T> {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
        (**self).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        (**self).begin_open(id)
    }

    fn is_durable(&self) -> bool {
        (**self).is_durable()
    }

    fn marked(&self) -> Job<Vec<(SessionId, Marks)>> {
        (**self).marked()
    }

    fn sessions(&self) -> Job<Vec<SessionId>> {
        (**self).sessions()
    }
}

/// What an operator or the session itself has marked a session with, which a store may keep so
/// that a restart finds it: see [`StoredSession::marks`]. A sequence reset leaves them as they
/// are.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Marks {
    /// Paused by an operator ([`SessionHandle::pause`](crate::SessionHandle::pause)): its Logons
    /// are refused until it's resumed.
    pub paused: bool,
    /// Logged on with cancel on disconnect, and not ended since in a way that doesn't count, nor
    /// cancelled: after a restart its countdown starts again, with this grace.
    pub cancel_on_disconnect: Option<CancelOnDisconnect>,
}

impl Marks {
    /// Whether no mark is set.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// A short text form for stores to keep: words separated by spaces, `paused` and
/// `cancel_on_disconnect=TRIGGER:GRACE_MS` (TRIGGER `disconnect` or `disconnect_or_logout`),
/// empty for none. [`FromStr`](std::str::FromStr) reads it back.
impl fmt::Display for Marks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut words = Vec::new();
        if self.paused {
            words.push("paused".to_owned());
        }
        if let Some(cancel) = self.cancel_on_disconnect {
            words.push(format!("cancel_on_disconnect={}:{}", cancel.trigger.label(), cancel.grace.as_millis()));
        }
        f.write_str(&words.join(" "))
    }
}

impl std::str::FromStr for Marks {
    type Err = io::Error;

    /// Reads the [`Display`](fmt::Display) form back.
    fn from_str(s: &str) -> io::Result<Self> {
        let invalid = |why: String| io::Error::new(io::ErrorKind::InvalidData, format!("marks '{s}': {why}"));
        let mut marks = Self::default();
        for word in s.split_whitespace() {
            match word.split_once('=') {
                None if word == "paused" => marks.paused = true,
                Some(("cancel_on_disconnect", value)) => {
                    let (trigger, grace) =
                        value.split_once(':').ok_or_else(|| invalid("expected TRIGGER:GRACE_MS".into()))?;
                    let grace = grace.parse().map_err(|_| invalid(format!("grace '{grace}' isn't milliseconds")))?;
                    let grace = std::time::Duration::from_millis(grace);
                    if grace > crate::cancel::MAX_CANCEL_GRACE {
                        return Err(invalid(format!("grace {grace:?} is over the maximum")));
                    }
                    marks.cancel_on_disconnect =
                        Some(CancelOnDisconnect::new(trigger.parse().map_err(invalid)?, grace));
                }
                _ => return Err(invalid(format!("unknown mark '{word}'"))),
            }
        }
        Ok(marks)
    }
}

/// A [`StoredSession`] being opened: see [`SessionStorage::begin_open`].
pub enum Opened {
    /// Opened at once.
    Ready(Box<dyn StoredSession>),
    /// To be opened by running the job, which the session's driver does off the async runtime.
    Pending(Job<Box<dyn StoredSession>>),
}

impl fmt::Debug for Opened {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ready(_) => f.write_str("Opened::Ready"),
            Self::Pending(job) => write!(f, "Opened::Pending({job:?})"),
        }
    }
}

/// Persistent state of one session.
///
/// A store may make each mutation durable as it's made, or buffer them until
/// [`commit`](StoredSession::commit), which the session calls once per batch of work (one read
/// from the counterparty, one batch of commands, one step of a resend), before anything that
/// batch sends goes on the wire. Reads see every mutation made, committed or not.
pub trait StoredSession: Send {
    /// The MsgSeqNum of the next message to send.
    fn next_outgoing(&self) -> u64;

    /// The MsgSeqNum expected on the next message from the counterparty.
    fn next_incoming(&self) -> u64;

    /// Records the next incoming sequence number, which also clears
    /// [`in_flight`](StoredSession::in_flight).
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()>;

    /// Records that outgoing `seq` has been used, storing `msg`, the message as sent (encoded,
    /// header and trailer included), for resends when given. On a FIXT session supporting more
    /// than one application version, a message in the default version is stored with its
    /// ApplVerID(1128) stated, so its bytes differ from those sent by that field.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()>;

    /// Stored messages with sequence numbers in `begin..=end`, in ascending order, as they were
    /// given to `record_outgoing`.
    ///
    /// # Errors
    ///
    /// Any failure reading the store: the session disconnects rather than resend part of the range.
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<SentMessages>;

    /// Stored messages with sequence numbers in `begin..=end`, as
    /// [`sent_messages`](StoredSession::sent_messages) returns them: at once, or from a [`Job`] the
    /// session's driver runs off the async runtime, while the resend waits. The session calls
    /// this for each step of a resend, and makes no other call while a returned job runs. The
    /// default reads [`sent_messages`](StoredSession::sent_messages) at once, which suits stores
    /// that read from memory or local files.
    ///
    /// A store that waits for a network to read should return a job here, and read in
    /// `sent_messages` by running it on the calling thread. Its reads must see every mutation
    /// made, committed or not, as `sent_messages` does.
    ///
    /// # Errors
    ///
    /// As [`sent_messages`](StoredSession::sent_messages).
    fn fetch(&mut self, begin: u64, end: u64) -> io::Result<Fetched> {
        self.sent_messages(begin, end).map(Fetched::Ready)
    }

    /// Makes the mutations made since the last commit durable (to the implementation's
    /// guarantee): at once, returning `Ok(None)`, or by running the returned [`Commit`], which
    /// the session's driver does off the async runtime and waits for before writing what depends
    /// on them. No other call is made while a returned commit runs. The default does nothing, for
    /// stores whose mutations are durable as they're made.
    ///
    /// A store that waits for a device (`fsync`) or a network should buffer its mutations and
    /// return a `Commit` here, so that a batch of messages costs one wait rather than one each.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn commit(&mut self) -> io::Result<Option<Commit>> {
        Ok(None)
    }

    /// Resets both sequence numbers to 1, discards stored messages, and clears
    /// [`created_at`](StoredSession::created_at), [`in_flight`](StoredSession::in_flight) and
    /// [`evicted_through`](StoredSession::evicted_through).
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn reset(&mut self) -> io::Result<()>;

    /// The highest sequence number whose message the store has discarded to stay within a limit,
    /// if any: nothing at or below it can be resent, so a resend gap-fills it. The session logs a
    /// warning when a ResendRequest reaches it. Stores that keep every message until a reset
    /// return `None` (the default).
    fn evicted_through(&self) -> Option<u64> {
        None
    }

    /// The first of the incoming messages being handed to the application when this was last
    /// committed, if they hadn't all been handled by then: set by
    /// [`set_in_flight`](StoredSession::set_in_flight), cleared by
    /// [`set_next_incoming`](StoredSession::set_next_incoming). Still set after a crash, it tells
    /// the session that the messages from it on (up to a batch of them) may have been handled
    /// already, and it marks them [`maybe_redelivered`](crate::Context::maybe_redelivered).
    fn in_flight(&self) -> Option<u64>;

    /// Records the next incoming sequence number with the in-flight marker at `in_flight`, as
    /// [`set_next_incoming`](StoredSession::set_next_incoming) then
    /// [`set_in_flight`](StoredSession::set_in_flight) would. A store that makes each change durable
    /// as it's made should make this one change, so that a crash never leaves the number without
    /// the marker; the default makes the two calls, enough for a store that commits them together.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_next_incoming_in_flight(&mut self, seq: u64, in_flight: u64) -> io::Result<()> {
        self.set_next_incoming(seq)?;
        self.set_in_flight(in_flight)
    }

    /// Records that incoming messages from `seq` on are about to be handed to the application.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_in_flight(&mut self, seq: u64) -> io::Result<()>;

    /// When the stored state was created or last reset, if recorded: `None` before the first
    /// [`set_created_at`](StoredSession::set_created_at). Session schedules use it to tell whether
    /// the state belongs to the current session period.
    fn created_at(&self) -> Option<UtcTimestamp>;

    /// Records when the stored state was created.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()>;

    /// The session's marks, as last set: kept through a [`reset`](StoredSession::reset). A store
    /// that keeps them overrides this, [`set_marks`](StoredSession::set_marks) and
    /// [`SessionStorage::marked`] together; the defaults keep none, so pauses and countdowns
    /// last only as long as the process, as with [`MemoryStorage`].
    fn marks(&self) -> Marks {
        Marks::default()
    }

    /// Records the session's marks, durable by the next [`commit`](StoredSession::commit) at the
    /// latest. The session sets them rarely: at logon, at its end, and for an operator's pause.
    ///
    /// # Errors
    ///
    /// Any failure of the store.
    fn set_marks(&mut self, marks: Marks) -> io::Result<()> {
        let _ = marks;
        Ok(())
    }
}

/// Stored messages and their sequence numbers, in ascending order, as given to
/// [`StoredSession::record_outgoing`].
pub type SentMessages = Vec<(u64, Vec<u8>)>;

/// Stored messages read for a resend: see [`StoredSession::fetch`].
#[derive(Debug)]
pub enum Fetched {
    /// Read at once.
    Ready(SentMessages),
    /// To be read by running the job, which the session's driver does off the async runtime.
    Pending(Job<SentMessages>),
}

/// Work that makes a [`StoredSession`]'s buffered mutations durable, returned by
/// [`StoredSession::commit`] for its caller to run off the connection's task.
pub type Commit = Job<()>;

/// Store work handed to the session's driver rather than done on the connection's task: either
/// a closure that blocks (writing and syncing files, say), which the driver runs on a blocking
/// thread, or a future (a database round trip), which it spawns on the runtime.
pub struct Job<T>(Work<T>);

enum Work<T> {
    Blocking(Box<dyn FnOnce() -> io::Result<T> + Send>),
    Future(Pin<Box<dyn Future<Output = io::Result<T>> + Send>>),
}

impl<T: Send + 'static> Job<T> {
    /// A job that blocks the thread it runs on until it's done.
    pub fn blocking(job: impl FnOnce() -> io::Result<T> + Send + 'static) -> Self {
        Self(Work::Blocking(Box::new(job)))
    }

    /// A job that's a future, for stores that wait on a network rather than a device.
    pub fn future(job: impl Future<Output = io::Result<T>> + Send + 'static) -> Self {
        Self(Work::Future(Box::pin(job)))
    }

    /// Runs the job on this thread, returning once it's done or has failed: for drivers that may
    /// block (tests, tools). A future job needs a multi-threaded tokio runtime current on this
    /// thread, whose worker it blocks meanwhile; without one it fails.
    ///
    /// # Errors
    ///
    /// The job's own error, or, for a future job, an error if no multi-threaded tokio runtime is
    /// current. A panic in the job isn't caught.
    pub fn run(self) -> io::Result<T> {
        match self.0 {
            Work::Blocking(job) => job(),
            Work::Future(job) => {
                let handle = tokio::runtime::Handle::try_current()
                    .map_err(|_| io::Error::other("a store's future job needs a tokio runtime to run on"))?;
                if handle.runtime_flavor() != tokio::runtime::RuntimeFlavor::MultiThread {
                    return Err(io::Error::other("a store's future job can't block a current-thread runtime"));
                }
                tokio::task::block_in_place(|| handle.block_on(job))
            }
        }
    }

    /// Runs the job without blocking the runtime: a blocking job on a blocking thread, a future
    /// as a task of its own.
    ///
    /// # Errors
    ///
    /// The job's own error, or a panic in it as an error.
    pub async fn run_async(self) -> io::Result<T> {
        self.spawn().await.unwrap_or_else(|e| Err(io::Error::other(format!("the store's job failed: {e}"))))
    }

    /// Runs the job in this task: a blocking job on this thread, and a future awaited here. For
    /// rare work outside a connection (operator changes to a disconnected session), which mustn't
    /// depend on a tokio runtime to spawn on, nor go on once its caller gives up. On a
    /// multi-threaded runtime's worker, a blocking job hands the worker's other tasks to another
    /// thread first, so an fsync doesn't hold them up; on a current-thread runtime it blocks.
    pub(crate) async fn run_here(self) -> io::Result<T> {
        match self.0 {
            Work::Blocking(job) => match tokio::runtime::Handle::try_current() {
                Ok(handle) if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread => {
                    tokio::task::block_in_place(job)
                }
                _ => job(),
            },
            Work::Future(job) => job.await,
        }
    }

    /// Starts the job on the current runtime.
    pub(crate) fn spawn(self) -> tokio::task::JoinHandle<io::Result<T>> {
        match self.0 {
            Work::Blocking(job) => tokio::task::spawn_blocking(job),
            Work::Future(job) => tokio::spawn(job),
        }
    }
}

impl<T: Send + 'static> Job<T> {
    /// The job, with `f` applied to what it returns: still blocking if it was, for a store
    /// wrapping another's jobs.
    pub fn map<U: Send + 'static>(self, f: impl FnOnce(T) -> U + Send + 'static) -> Job<U> {
        self.map_result(move |result| result.map(f))
    }

    /// The job, with `f` applied to its result, its error included: still blocking if it was.
    /// For work whose failure its caller can sometimes bear.
    pub fn map_result<U: Send + 'static>(
        self,
        f: impl FnOnce(io::Result<T>) -> io::Result<U> + Send + 'static,
    ) -> Job<U> {
        match self.0 {
            Work::Blocking(job) => Job::blocking(move || f(job())),
            Work::Future(job) => Job::future(async move { f(job.await) }),
        }
    }
}

impl<T: Send + 'static> Job<Vec<T>> {
    /// Runs `jobs` one after another, joining what they return: a blocking job if they all are,
    /// otherwise a future, which runs the blocking ones on blocking threads.
    pub(crate) fn concat(jobs: Vec<Self>) -> Self {
        if jobs.iter().all(|job| matches!(job.0, Work::Blocking(_))) {
            return Self::blocking(move || {
                let mut all = Vec::new();
                for job in jobs {
                    all.extend(job.run()?);
                }
                Ok(all)
            });
        }
        Self::future(async move {
            let mut all = Vec::new();
            for job in jobs {
                all.extend(job.run_async().await?);
            }
            Ok(all)
        })
    }
}

impl Job<()> {
    /// Runs this job and `other`, done once both are, failing with the first's error if either
    /// fails: a blocking job running one after the other if both are, otherwise a future running
    /// them at once, a blocking one on a blocking thread. For a store whose commit is two others'
    /// (a local store's and a copy's elsewhere).
    #[must_use]
    pub fn join(self, other: Self) -> Self {
        if matches!((&self.0, &other.0), (Work::Blocking(_), Work::Blocking(_))) {
            return Self::blocking(move || {
                let first = self.run();
                let second = other.run();
                first.and(second)
            });
        }
        Self::future(async move {
            let (first, second) = tokio::join!(self.run_async(), other.run_async());
            first.and(second)
        })
    }
}

impl<T> fmt::Debug for Job<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            Work::Blocking(_) => "Job::Blocking",
            Work::Future(_) => "Job::Future",
        })
    }
}

/// Checks, in debug builds, a store's own answer to `sent_messages(begin, end)`: ascending,
/// within the range, nothing at or below `evicted_through`, and each a whole message. The session
/// checks what any store returns as it parses it; a store we write checks its own answer too,
/// so a bug there is caught where it happens.
pub(crate) fn debug_check_sent(messages: &SentMessages, begin: u64, end: u64, evicted_through: Option<u64>) {
    if !cfg!(debug_assertions) {
        return;
    }
    for pair in messages.windows(2) {
        debug_assert!(pair[0].0 < pair[1].0, "stored messages in ascending order");
    }
    for (seq, bytes) in messages {
        debug_assert!(*seq >= begin, "stored message {seq} below {begin}");
        debug_assert!(*seq <= end, "stored message {seq} above {end}");
        debug_assert!(Some(*seq) > evicted_through, "stored message {seq} was evicted");
        debug_assert!(is_one_message(bytes), "stored message {seq} is one message");
    }
}

/// Checks, in debug builds, a call to [`StoredSession::record_outgoing`] against the log's
/// `next_outgoing` before it: numbers are never reused, and a message stored takes the next one
/// (numbers recorded without a message may skip ahead, as an operator does).
pub(crate) fn debug_check_record(seq: u64, next_outgoing: u64, msg: Option<&[u8]>) {
    debug_assert!(seq >= next_outgoing, "outgoing {seq} recorded again: the next is {next_outgoing}");
    if let Some(bytes) = msg {
        debug_assert_eq!(seq, next_outgoing, "a message stored takes the next number");
        debug_assert!(is_one_message(bytes), "one whole message");
    }
}

/// Whether `bytes` is one whole message as sent: a FIX session's frame, or a FIXP session's.
pub(crate) fn is_one_message(bytes: &[u8]) -> bool {
    // FIXP's check first: cheap, and it allocates nothing, so debug builds count as release ones do.
    crate::fixp::is_one_frame(bytes) || crate::codec::frame_stored(bytes) == Ok(bytes.len())
}

/// Commits `log`'s mutations, running any [`Commit`] on this thread: for changes made outside a
/// connected session (operator commands on a disconnected one), which are rare.
#[cfg(test)]
pub(crate) fn commit_now(log: &mut dyn StoredSession) -> io::Result<()> {
    log.commit()?.map_or(Ok(()), Commit::run)
}

/// A store over [`MemoryStorage`] whose commits run a job the test chooses, recording the
/// mutations and commits made of it: the session must wait for each commit before writing what it
/// covers. With [`deferring_all`](deferring::DeferringStorage::deferring_all), its resend reads
/// and openings are jobs too, which run the same job before returning what was read or opened.
#[cfg(test)]
pub(crate) mod deferring {
    use std::sync::{Arc, Mutex};

    use super::*;

    /// What each commit runs.
    pub type Job = Arc<dyn Fn() -> io::Result<()> + Send + Sync>;

    pub struct DeferringStorage {
        inner: MemoryStorage,
        pub calls: Arc<Mutex<Vec<String>>>,
        pub job: Arc<Mutex<Job>>,
        deferred: bool,
    }

    impl DeferringStorage {
        /// Returns resend reads and openings as jobs as well as commits.
        pub fn deferring_all() -> Self {
            Self { deferred: true, ..Self::default() }
        }
    }

    impl Default for DeferringStorage {
        fn default() -> Self {
            let job: Job = Arc::new(|| Ok(()));
            Self { inner: MemoryStorage::new(), calls: Arc::default(), job: Arc::new(Mutex::new(job)), deferred: false }
        }
    }

    struct DeferringLog {
        inner: Box<dyn StoredSession>,
        calls: Arc<Mutex<Vec<String>>>,
        job: Arc<Mutex<Job>>,
        /// Mutations since the last commit.
        dirty: bool,
        deferred: bool,
    }

    impl SessionStorage for DeferringStorage {
        fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
            let (calls, job) = (self.calls.clone(), self.job.clone());
            let inner = self.inner.open(id)?;
            Ok(Box::new(DeferringLog { inner, calls, job, dirty: false, deferred: self.deferred }))
        }

        fn is_durable(&self) -> bool {
            self.inner.is_durable()
        }

        fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
            let log = self.open(id)?;
            if !self.deferred {
                return Ok(Opened::Ready(log));
            }
            self.calls.lock().unwrap().push(format!("open {id}"));
            let job = self.job.clone();
            Ok(Opened::Pending(super::Job::blocking(move || (job.lock().unwrap().clone())().map(|()| log))))
        }
    }

    impl DeferringLog {
        fn mutated(&mut self, call: String) {
            self.calls.lock().unwrap().push(call);
            self.dirty = true;
        }
    }

    impl StoredSession for DeferringLog {
        fn next_outgoing(&self) -> u64 {
            self.inner.next_outgoing()
        }
        fn next_incoming(&self) -> u64 {
            self.inner.next_incoming()
        }
        fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
            self.mutated(format!("incoming {seq}"));
            self.inner.set_next_incoming(seq)
        }
        fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
            self.mutated(format!("outgoing {seq}"));
            self.inner.record_outgoing(seq, msg)
        }
        fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<SentMessages> {
            self.inner.sent_messages(begin, end)
        }
        fn fetch(&mut self, begin: u64, end: u64) -> io::Result<Fetched> {
            let read = self.inner.sent_messages(begin, end)?;
            if !self.deferred {
                return Ok(Fetched::Ready(read));
            }
            self.calls.lock().unwrap().push(format!("fetch {begin}..={end}"));
            // The job the test has chosen when the read runs.
            let job = self.job.clone();
            Ok(Fetched::Pending(super::Job::blocking(move || (job.lock().unwrap().clone())().map(|()| read))))
        }
        fn reset(&mut self) -> io::Result<()> {
            self.mutated("reset".into());
            self.inner.reset()
        }
        fn in_flight(&self) -> Option<u64> {
            self.inner.in_flight()
        }
        fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
            self.mutated(format!("in flight {seq}"));
            self.inner.set_in_flight(seq)
        }
        fn created_at(&self) -> Option<UtcTimestamp> {
            self.inner.created_at()
        }
        fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
            self.mutated("created".into());
            self.inner.set_created_at(at)
        }
        fn commit(&mut self) -> io::Result<Option<Commit>> {
            if !std::mem::take(&mut self.dirty) {
                return Ok(None);
            }
            self.calls.lock().unwrap().push("commit".into());
            let job = self.job.lock().unwrap().clone();
            Ok(Some(Commit::blocking(move || job())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_session_id_shows_only_the_fields_it_has() {
        let plain = SessionId::new("FIX.4.4", "GATEWAY", "CLIENT");
        assert_eq!(plain.to_string(), "FIX.4.4:GATEWAY->CLIENT");
        assert_eq!(plain.key_suffix(), "", "a plain session keeps its old key");

        let full = plain
            .clone()
            .with_sender_sub_id("DESK")
            .with_sender_location_id("NY")
            .with_target_sub_id("TRADER 7")
            .with_qualifier("b");
        assert_eq!(full.to_string(), "FIX.4.4:GATEWAY/DESK/NY->CLIENT/TRADER 7:b");
        assert_eq!(full.key_suffix(), "ss=DESK;sl=NY;ts=TRADER%207;q=b");

        let location_only = plain.with_target_location_id("LDN");
        assert_eq!(location_only.to_string(), "FIX.4.4:GATEWAY->CLIENT//LDN");
        assert_eq!(location_only.key_suffix(), "tl=LDN");
    }

    #[test]
    fn a_commit_runs_its_job() {
        assert!(Commit::blocking(|| Ok(())).run().is_ok());
        let failed = Commit::blocking(|| Err(io::Error::other("disk full"))).run();
        assert_eq!(failed.unwrap_err().to_string(), "disk full");
    }

    /// A future job runs as a task, or blocks a multi-threaded runtime's worker when run on the
    /// thread; with no runtime to run on, or only a current-thread one, it fails rather than hang.
    #[test]
    fn a_future_job_runs_on_a_runtime() {
        let future = || Job::future(async { Ok(7) });
        assert!(future().run().is_err(), "no runtime");

        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(1).build().unwrap();
        assert_eq!(runtime.block_on(future().run_async()).unwrap(), 7);
        assert_eq!(runtime.block_on(async { future().run() }).unwrap(), 7);
        assert_eq!(runtime.block_on(Job::blocking(|| Ok(8)).run_async()).unwrap(), 8);

        let current = tokio::runtime::Builder::new_current_thread().build().unwrap();
        assert!(current.block_on(async { future().run() }).is_err(), "would deadlock");
    }

    /// Joined, two jobs both run, even when the first fails, which fails the whole; two blocking
    /// jobs make a blocking one, and a future with either makes a future.
    #[test]
    fn joined_jobs_both_run() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU64, Ordering};

        let ran = Arc::new(AtomicU64::new(0));
        let counting = |fail: bool| {
            let ran = ran.clone();
            Commit::blocking(move || {
                ran.fetch_add(1, Ordering::Relaxed);
                if fail { Err(io::Error::other("disk full")) } else { Ok(()) }
            })
        };
        let joined = counting(false).join(counting(false));
        assert_eq!(format!("{joined:?}"), "Job::Blocking");
        assert!(joined.run().is_ok());
        assert_eq!(counting(true).join(counting(false)).run().unwrap_err().to_string(), "disk full");
        assert_eq!(counting(false).join(counting(true)).run().unwrap_err().to_string(), "disk full");
        assert_eq!(ran.load(Ordering::Relaxed), 6, "both halves ran each time");

        let joined = counting(false).join(Commit::future(async { Err(io::Error::other("link down")) }));
        assert_eq!(format!("{joined:?}"), "Job::Future");
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(1).build().unwrap();
        assert_eq!(runtime.block_on(joined.run_async()).unwrap_err().to_string(), "link down");
        assert_eq!(ran.load(Ordering::Relaxed), 7);
    }

    /// The suite runs a store's jobs, whatever it returns as one.
    #[test]
    fn a_store_of_jobs_conforms() {
        conformance::check_blocking(&deferring::DeferringStorage::deferring_all());
    }

    #[test]
    fn marks_read_back_from_their_text() {
        assert_eq!(Marks::default().to_string(), "");
        let cancel = CancelOnDisconnect::new(crate::CancelTrigger::Disconnect, std::time::Duration::from_millis(1500));
        let marks = Marks { paused: true, cancel_on_disconnect: Some(cancel) };
        assert_eq!(marks.to_string(), "paused cancel_on_disconnect=disconnect:1500");
        for marks in [marks, Marks::default()] {
            assert_eq!(marks.to_string().parse::<Marks>().unwrap(), marks);
        }
        for bad in [
            "stopped",
            "cancel_on_disconnect=disconnect",
            "cancel_on_disconnect=never:5",
            "cancel_on_disconnect=disconnect:3600001",
        ] {
            assert!(bad.parse::<Marks>().is_err(), "{bad}");
        }
    }

    #[test]
    fn keys_read_back_as_their_sessions() {
        let full = SessionId::new("FIX.4.4", "GW", "CO")
            .with_sender_sub_id("D;1")
            .with_sender_location_id("NY")
            .with_target_sub_id("T=7")
            .with_target_location_id("%")
            .with_qualifier("q");
        for id in [SessionId::new("FIX.4.4", "GW", "CO"), full] {
            assert_eq!(SessionId::from_key("FIX.4.4", "GW", "CO", &id.key_suffix()), Some(id));
        }
        for bad in ["zz=1", "ss", "ss=%2", "ts=1;ss=2", "ss=a b"] {
            assert_eq!(SessionId::from_key("FIX.4.4", "GW", "CO", bad), None, "{bad}");
        }
    }
}
