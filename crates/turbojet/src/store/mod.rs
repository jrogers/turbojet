//! Session persistence: sequence numbers and sent application messages, kept per session so a
//! counterparty can reconnect and recover missed messages.

mod by_counterparty;
#[cfg(any(test, feature = "conformance"))]
pub mod conformance;
pub(crate) mod disk;
mod memory;

use std::fmt;
use std::future::Future;
use std::io;
use std::pin::Pin;

pub use by_counterparty::StorageByCounterparty;
pub use disk::DiskStorage;
pub use memory::MemoryStorage;

use crate::fields::UtcTimestamp;

/// Identifies a FIX session from the gateway's side.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SessionId {
    /// BeginString(8).
    pub begin_string: String,
    /// Our CompID.
    pub sender_comp_id: String,
    /// The counterparty's CompID.
    pub target_comp_id: String,
}

impl SessionId {
    /// The session with BeginString `begin_string`, our CompID `sender_comp_id` and the
    /// counterparty's `target_comp_id`.
    pub fn new(
        begin_string: impl Into<String>,
        sender_comp_id: impl Into<String>,
        target_comp_id: impl Into<String>,
    ) -> Self {
        Self {
            begin_string: begin_string.into(),
            sender_comp_id: sender_comp_id.into(),
            target_comp_id: target_comp_id.into(),
        }
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}->{}", self.begin_string, self.sender_comp_id, self.target_comp_id)
    }
}

/// Opens per-session logs. Shared by all connections.
///
/// Implement [`open`](SessionStorage::open) for a store that opens at once (memory, local
/// files), or [`begin_open`](SessionStorage::begin_open) for one that waits for a network.
pub trait SessionStorage: Send + Sync {
    /// Opens the log for `id`, creating it with both sequence numbers at 1 if it does not exist.
    /// The gateway holds at most one open log per session. The default fails, for stores that
    /// open only with [`begin_open`](SessionStorage::begin_open).
    ///
    /// # Errors
    ///
    /// Any failure opening the log, and by default [`Unsupported`](io::ErrorKind::Unsupported); the
    /// logon is refused.
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Err(io::Error::new(io::ErrorKind::Unsupported, format!("{id}: this store opens only with begin_open")))
    }

    /// Opens the log for `id` as [`open`](SessionStorage::open) does: at once, or by a [`Job`]
    /// the session's driver runs off the async runtime while the logon waits. The session calls
    /// this when it learns its ID, at logon. The default opens at once with `open`.
    ///
    /// # Errors
    ///
    /// As [`open`](SessionStorage::open).
    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.open(id).map(Opened::Ready)
    }
}

/// A [`SessionLog`] being opened: see [`SessionStorage::begin_open`].
pub enum Opened {
    /// Opened at once.
    Ready(Box<dyn SessionLog>),
    /// To be opened by running the job, which the session's driver does off the async runtime.
    Pending(Job<Box<dyn SessionLog>>),
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
/// [`commit`](SessionLog::commit), which the session calls once per batch of work (one read
/// from the counterparty, one batch of commands, one step of a resend), before anything that
/// batch sends goes on the wire. Reads see every mutation made, committed or not.
pub trait SessionLog: Send {
    /// The MsgSeqNum of the next message to send.
    fn next_outgoing(&self) -> u64;

    /// The MsgSeqNum expected on the next message from the counterparty.
    fn next_incoming(&self) -> u64;

    /// Records the next incoming sequence number, which also clears
    /// [`in_flight`](SessionLog::in_flight).
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
    /// [`sent_messages`](SessionLog::sent_messages) returns them: at once, or from a [`Job`] the
    /// session's driver runs off the async runtime, while the resend waits. The session calls
    /// this for each step of a resend, and makes no other call while a returned job runs. The
    /// default reads [`sent_messages`](SessionLog::sent_messages) at once, which suits stores
    /// that read from memory or local files.
    ///
    /// A store that waits for a network to read should return a job here. Its reads must see
    /// every mutation made, committed or not, as `sent_messages` does.
    ///
    /// # Errors
    ///
    /// As [`sent_messages`](SessionLog::sent_messages).
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
    /// [`created_at`](SessionLog::created_at), [`in_flight`](SessionLog::in_flight) and
    /// [`evicted_through`](SessionLog::evicted_through).
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
    /// [`set_in_flight`](SessionLog::set_in_flight), cleared by
    /// [`set_next_incoming`](SessionLog::set_next_incoming). Still set after a crash, it tells
    /// the session that the messages from it on (up to a batch of them) may have been handled
    /// already. Stores that don't record it return `None` (the default), and redeliveries then go
    /// unmarked.
    fn in_flight(&self) -> Option<u64> {
        None
    }

    /// Records that incoming messages from `seq` on are about to be handed to the application.
    /// The default does nothing.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_in_flight(&mut self, _seq: u64) -> io::Result<()> {
        Ok(())
    }

    /// When the stored state was created or last reset, if recorded. Session schedules use it to
    /// tell whether the state belongs to the current session period. Stores that don't record it
    /// return `None` (the default), and schedule-based sequence resets are then skipped.
    fn created_at(&self) -> Option<UtcTimestamp> {
        None
    }

    /// Records when the stored state was created. The default does nothing.
    ///
    /// # Errors
    ///
    /// Any failure of the store: the session drops what it couldn't store and disconnects, rather
    /// than send it.
    fn set_created_at(&mut self, _at: UtcTimestamp) -> io::Result<()> {
        Ok(())
    }
}

/// Stored messages and their sequence numbers, in ascending order, as given to
/// [`SessionLog::record_outgoing`].
pub type SentMessages = Vec<(u64, Vec<u8>)>;

/// Stored messages read for a resend: see [`SessionLog::fetch`].
#[derive(Debug)]
pub enum Fetched {
    /// Read at once.
    Ready(SentMessages),
    /// To be read by running the job, which the session's driver does off the async runtime.
    Pending(Job<SentMessages>),
}

/// Work that makes a [`SessionLog`]'s buffered mutations durable, returned by
/// [`SessionLog::commit`] for its caller to run off the connection's task.
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

    /// Runs the job in this task: a blocking job on this thread, which it blocks, and a future
    /// awaited here. For rare work outside a connection (operator changes to a disconnected
    /// session), which mustn't depend on a tokio runtime to spawn on.
    pub(crate) async fn run_here(self) -> io::Result<T> {
        match self.0 {
            Work::Blocking(job) => job(),
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

/// Checks, in debug builds, a call to [`SessionLog::record_outgoing`] against the log's
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
    crate::codec::frame_stored(bytes) == Ok(bytes.len()) || crate::fixp::is_one_frame(bytes)
}

/// Commits `log`'s mutations, running any [`Commit`] on this thread: for changes made outside a
/// connected session (operator commands on a disconnected one), which are rare.
#[cfg(test)]
pub(crate) fn commit_now(log: &mut dyn SessionLog) -> io::Result<()> {
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
        inner: Box<dyn SessionLog>,
        calls: Arc<Mutex<Vec<String>>>,
        job: Arc<Mutex<Job>>,
        /// Mutations since the last commit.
        dirty: bool,
        deferred: bool,
    }

    impl SessionStorage for DeferringStorage {
        fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
            let (calls, job) = (self.calls.clone(), self.job.clone());
            let inner = self.inner.open(id)?;
            Ok(Box::new(DeferringLog { inner, calls, job, dirty: false, deferred: self.deferred }))
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

    impl SessionLog for DeferringLog {
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

    /// The suite runs a store's jobs, whatever it returns as one.
    #[test]
    fn a_store_of_jobs_conforms() {
        conformance::check_blocking(&deferring::DeferringStorage::deferring_all());
    }
}
