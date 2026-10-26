//! The active's side: a store that copies each session's commits to a standby.

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

use tracing::{info, warn};
use turbojet::SessionId;
use turbojet::fields::UtcTimestamp;
use turbojet::store::{Commit, Fetched, Job, Marks, Opened, SentMessages, SessionStorage, StoredSession};

use crate::frame::{Batch, Frame, NackReason, Op, Summary, VERSION};
use crate::lease::{Held, Lease};
use crate::standby::{Replica, summary};

/// The most a batch of a session sent whole carries in messages: a large session is sent in
/// several, well within a frame each.
const SNAPSHOT_BATCH_BYTES: usize = 1024 * 1024;

/// The most batches an asynchronous session leaves unacknowledged, however few outgoing numbers
/// they cover: the bound for commits that carry none (inbound only), which the lag doesn't count.
/// Enough that a session's own pace, not the cap, sets when such commits wait.
// ponytail: a fixed cap, raised or made a setting only if inbound-only commits dominate.
const MAX_QUEUED: usize = 64;

/// How batches reach the standby. Its calls block until the standby answers, so a store using
/// it runs them on a blocking thread, in its jobs.
pub trait Link: Send + Sync {
    /// Binds `handle` to `session` on the standby, as the holder of `epoch` that leaves up to
    /// `max_lag` of the session's outgoing numbers unacknowledged, returning its copy of the
    /// session.
    ///
    /// # Errors
    ///
    /// The standby can't be reached, or refused: [`io::ErrorKind::InvalidInput`] if it accepts
    /// less lag than `max_lag`.
    fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary>;

    /// Sends a batch for a bound handle: a job that ends once the standby has committed it,
    /// failing if the standby refused it or can't be reached.
    fn send(&self, batch: Batch) -> Commit;
}

/// A [`SessionStorage`] that keeps each session in `S` and copies every commit to a standby over
/// a [`Link`]: a session's commit, and so everything it covers going out, waits for both, unless
/// [`asynchronous`](Self::asynchronous).
///
/// A commit that one side made and the other didn't leaves that side ahead of what was sent, as
/// a crash between a commit and its write does, which a session's recovery already handles.
///
/// By default a commit the standby can't take fails, and the session disconnects. Made
/// [`degrading`](Self::degrading), the store commits alone instead, once its lease says the
/// standby is out of sync, and catches the standby up when it's back: each open session at its
/// next commit, and the rest in [`catch_up`](Self::catch_up).
pub struct ReplicatedStorage<S> {
    inner: Arc<S>,
    shared: Arc<Replication>,
    next_handle: Arc<AtomicU32>,
}

/// What a replicating store and the logs it opens share.
struct Replication {
    link: Arc<dyn Link>,
    /// The epoch of the lease this instance holds.
    epoch: u64,
    /// The lease itself, renewed in commits and told when the standby is out of sync, if given.
    held: Option<Arc<Held>>,
    /// A commit the standby can't take is made alone, rather than failing.
    degrade: bool,
    /// The most outgoing numbers a session may leave unacknowledged by the standby, which each
    /// bind tells it: none unless [`asynchronous`](ReplicatedStorage::asynchronous).
    max_lag: u64,
    state: Mutex<State>,
    /// Signalled when [`catch_up`](ReplicatedStorage::catch_up) is done with a session.
    caught: Condvar,
}

#[derive(Default)]
struct State {
    /// The lease has been told the standby is out of sync.
    out_of_sync: bool,
    /// Whether the lease has been asked, at the first catch-up, if an earlier run left the
    /// standby out of sync.
    checked: bool,
    /// Sessions whose copy on the standby lacks commits.
    behind: HashSet<SessionId>,
    /// The logs open on each session.
    open: HashMap<SessionId, usize>,
    /// The session the catch-up is sending, which an opening waits for.
    catching_up: Option<SessionId>,
}

impl Replication {
    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The standby's copy of `session` lacks what `cause` kept from it. Degrading, that's
    /// recorded, and before the first commit the standby won't have, the lease is told it's out
    /// of sync; otherwise, or if the lease can't be told, or `cause` is that another instance
    /// holds the lease, it's the error.
    fn fall_behind(&self, session: &SessionId, cause: io::Error) -> io::Result<()> {
        if !self.degrade || cause.kind() == io::ErrorKind::PermissionDenied {
            return Err(cause);
        }
        let mut state = self.state();
        if !state.out_of_sync {
            match &self.held {
                Some(held) => held.set_in_sync(false).run()?,
                None => warn!("no lease records that the standby is out of sync"),
            }
            state.out_of_sync = true;
            warn!(error = %cause, epoch = self.epoch, "the standby is out of sync: committing without it");
        }
        state.behind.insert(session.clone());
        Ok(())
    }

    fn caught_up(&self, session: &SessionId) {
        self.state().behind.remove(session);
    }

    fn is_behind(&self, session: &SessionId) -> bool {
        self.state().behind.contains(session)
    }

    /// A log on `session` is opening: it waits while the catch-up is sending the session.
    fn opening(&self, session: &SessionId) {
        let mut state = self.state();
        while state.catching_up.as_ref() == Some(session) {
            state = self.caught.wait(state).unwrap_or_else(PoisonError::into_inner);
        }
        *state.open.entry(session.clone()).or_default() += 1;
    }

    fn closed(&self, session: &SessionId) {
        let mut state = self.state();
        if let Some(open) = state.open.get_mut(session) {
            *open -= 1;
            if *open == 0 {
                state.open.remove(session);
            }
        }
    }
}

impl<S: SessionStorage> ReplicatedStorage<S> {
    /// Keeps sessions in `inner`, copying their commits over `link`, as the holder of the lease
    /// for `epoch`.
    pub fn new(inner: S, link: Arc<dyn Link>, epoch: u64) -> Self {
        let shared = Replication {
            link,
            epoch,
            held: None,
            degrade: false,
            max_lag: 0,
            state: Mutex::default(),
            caught: Condvar::new(),
        };
        Self { inner: Arc::new(inner), shared: Arc::new(shared), next_handle: Arc::default() }
    }

    fn configure(&mut self) -> &mut Replication {
        Arc::get_mut(&mut self.shared).expect("configured before any session opens")
    }

    /// Renews `lease`, taken under this store's epoch, in its sessions' commits once half of it
    /// has gone; a commit fails once another instance has taken it, or it has run out unrenewed.
    #[must_use]
    pub fn with_lease(mut self, lease: Arc<dyn Lease>) -> Self {
        let epoch = self.shared.epoch;
        self.configure().held = Some(Held::new(lease, epoch));
        self
    }

    /// Commits alone when the standby can't take a commit (it can't be reached, or its copy has
    /// diverged), rather than fail the session: before the first such commit the lease is told
    /// the standby is out of sync ([`Lease::set_in_sync`]), and if it can't be told, the commit
    /// fails after all. The standby is caught up as it comes back: each open session at its next
    /// commit, sending the session whole, and the rest by [`catch_up`](Self::catch_up), which
    /// tells the lease once all are.
    #[must_use]
    pub fn degrading(mut self) -> Self {
        self.configure().degrade = true;
        self
    }

    /// Commits don't wait for the standby until a session has more than `max_unacked` outgoing
    /// numbers it hasn't acknowledged: a session's output goes out at its own store's pace. What
    /// it trades: a failover can lose up to that many of a session's outgoing numbers, which
    /// [`Replica::promote`] skips, so the standby must accept the lag
    /// ([`Replica::accepting_lag`]) or opening a session fails. A batch the standby fails to take
    /// fails a later commit of the session, the one that waits for it, or, made
    /// [`degrading`](Self::degrading), puts the standby out of sync.
    #[must_use]
    pub fn asynchronous(mut self, max_unacked: u64) -> Self {
        self.configure().max_lag = max_unacked;
        self
    }

    /// Gives the lease up, so the standby's instance can take it at once: for an orderly
    /// handover, once this instance's sessions have closed. Nothing to do without one.
    pub fn release(&self) -> Job<()> {
        match &self.shared.held {
            Some(held) => held.release(),
            None => Job::blocking(|| Ok(())),
        }
    }

    /// Whether the standby holds everything this instance committed, as far as it knows.
    pub fn in_sync(&self) -> bool {
        !self.shared.state().out_of_sync
    }

    fn handle(&self) -> u32 {
        self.next_handle.fetch_add(1, Ordering::Relaxed)
    }
}

impl<S: SessionStorage + 'static> ReplicatedStorage<S> {
    /// One pass of catching the standby up, for a degrading store: each session behind on the
    /// standby that isn't open is opened and sent whole (an opening of it waits meanwhile), and
    /// once none is behind, the lease is told the standby is in sync again. The first pass also
    /// asks the lease whether an earlier run left the standby out of sync, and if so counts every
    /// session behind. Run it every few seconds; it stops at the first session it can't send.
    pub fn catch_up(&self) -> Job<()> {
        let (inner, shared, next_handle) = (self.inner.clone(), self.shared.clone(), self.next_handle.clone());
        Job::blocking(move || {
            if !shared.state().checked {
                let in_sync = shared.held.as_ref().map_or(Ok(true), |held| held.in_sync().run())?;
                if !in_sync {
                    let sessions = inner.sessions().run()?;
                    let mut state = shared.state();
                    state.out_of_sync = true;
                    state.behind.extend(sessions);
                }
                shared.state().checked = true;
            }
            loop {
                let next = {
                    let mut state = shared.state();
                    let next = state.behind.iter().find(|id| !state.open.contains_key(*id)).cloned();
                    state.catching_up.clone_from(&next);
                    next
                };
                let Some(id) = next else { break };
                let handle = next_handle.fetch_add(1, Ordering::Relaxed);
                let sent = open_inner(inner.as_ref(), &id).and_then(|log| {
                    shared.state().open.insert(id.clone(), 1);
                    attach(&shared, &id, handle, log)
                });
                let caught = matches!(&sent, Ok(log) if log.bound.load(Ordering::Acquire));
                drop(sent);
                shared.state().catching_up = None;
                shared.caught.notify_all();
                if !caught {
                    return Ok(());
                }
            }
            let mut state = shared.state();
            if state.out_of_sync && state.behind.is_empty() {
                if let Some(held) = &shared.held {
                    held.set_in_sync(true).run()?;
                }
                state.out_of_sync = false;
                info!(epoch = shared.epoch, "the standby is in sync again");
            }
            Ok(())
        })
    }
}

/// Opens `id` in `inner`, running its job on this thread if it hands one back.
fn open_inner<S: SessionStorage + ?Sized>(inner: &S, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
    match inner.begin_open(id)? {
        Opened::Ready(log) => Ok(log),
        Opened::Pending(job) => job.run(),
    }
}

/// Binds `session`'s log in `S`, counted open, on the standby and, if its copy differs (or the
/// session is behind), sends it the session whole. Degrading, a standby that can't be reached
/// leaves the log unbound and the session behind, to be sent at its next commit.
fn attach(
    shared: &Arc<Replication>,
    session: &SessionId,
    handle: u32,
    inner: Box<dyn StoredSession>,
) -> io::Result<ReplicatedLog> {
    let mut log = ReplicatedLog {
        inner,
        shared: shared.clone(),
        session: session.clone(),
        handle,
        commit_no: Arc::default(),
        bound: Arc::new(AtomicBool::new(false)),
        ops: Vec::new(),
        queued: VecDeque::new(),
        drained_through: 0,
    };
    log.drained_through = log.inner.next_outgoing() - 1;
    let sent = shared.link.bind(handle, shared.epoch, shared.max_lag, session).and_then(|theirs| {
        if theirs != summary(log.inner.as_ref()) || shared.is_behind(session) {
            let Whole { header, fetched } = Whole::of(log.inner.as_mut())?;
            send_whole(shared, handle, &log.commit_no, header, messages(fetched)?)?;
        }
        Ok(())
    });
    match sent {
        Ok(()) => {
            log.bound.store(true, Ordering::Release);
            shared.caught_up(session);
        }
        Err(e) => shared.fall_behind(session, e)?,
    }
    Ok(log)
}

impl<S> std::fmt::Debug for ReplicatedStorage<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplicatedStorage")
            .field("epoch", &self.shared.epoch)
            .field("degrade", &self.shared.degrade)
            .finish_non_exhaustive()
    }
}

impl<S: SessionStorage + 'static> SessionStorage for ReplicatedStorage<S> {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
        open_attached(self.inner.as_ref(), &self.shared, id, self.handle())
    }

    /// Always a job: opening may wait for the catch-up, and attaching for the standby.
    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        let (inner, shared, session, handle) = (self.inner.clone(), self.shared.clone(), id.clone(), self.handle());
        Ok(Opened::Pending(Job::blocking(move || open_attached(inner.as_ref(), &shared, &session, handle))))
    }

    fn is_durable(&self) -> bool {
        self.inner.is_durable()
    }

    fn marked(&self) -> Job<Vec<(SessionId, Marks)>> {
        self.inner.marked()
    }

    fn sessions(&self) -> Job<Vec<SessionId>> {
        self.inner.sessions()
    }
}

/// Opens `session` in `inner`, once the catch-up isn't sending it (so a store that locks a
/// session open finds it free), counted open, and attaches it.
fn open_attached<S: SessionStorage + ?Sized>(
    inner: &S,
    shared: &Arc<Replication>,
    session: &SessionId,
    handle: u32,
) -> io::Result<Box<dyn StoredSession>> {
    shared.opening(session);
    let log = match open_inner(inner, session) {
        Ok(log) => log,
        Err(e) => {
            shared.closed(session);
            return Err(e);
        }
    };
    // A log that fails to attach is dropped, uncounting it.
    Ok(Box::new(attach(shared, session, handle, log)?))
}

/// What sending a session whole takes from its log: everything but its messages, read now, and
/// its messages, read now or by a job.
struct Whole {
    header: Header,
    fetched: Fetched,
}

/// A session's state but its messages.
struct Header {
    created_at: Option<UtcTimestamp>,
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    marks: Marks,
}

impl Whole {
    fn of(log: &mut dyn StoredSession) -> io::Result<Self> {
        let header = Header {
            created_at: log.created_at(),
            next_outgoing: log.next_outgoing(),
            next_incoming: log.next_incoming(),
            in_flight: log.in_flight(),
            marks: log.marks(),
        };
        Ok(Self { header, fetched: log.fetch(1, u64::MAX)? })
    }
}

/// The messages `fetched` has, reading them if it hands a job back.
fn messages(fetched: Fetched) -> io::Result<SentMessages> {
    match fetched {
        Fetched::Ready(messages) => Ok(messages),
        Fetched::Pending(job) => job.run(),
    }
}

/// Sends a session as `header` and `messages` have it, replacing the standby's copy, in batches
/// of [`SNAPSHOT_BATCH_BYTES`] of messages at most, numbered on from `commit_no`.
fn send_whole(
    shared: &Replication,
    handle: u32,
    commit_no: &AtomicU64,
    header: Header,
    messages: SentMessages,
) -> io::Result<()> {
    let mut batches = vec![vec![Op::Reset]];
    if let Some(at) = header.created_at {
        batches[0].push(Op::SetCreatedAt(at));
    }
    let (mut next, mut bytes) = (1, 0);
    for (seq, msg) in messages {
        if bytes > SNAPSHOT_BATCH_BYTES {
            batches.push(Vec::new());
            bytes = 0;
        }
        let ops = batches.last_mut().expect("one at least");
        // Numbers used without a message stored, or evicted, before this one.
        if seq > next {
            ops.push(Op::Record { seq: seq - 1, msg: None });
        }
        bytes += msg.len();
        ops.push(Op::Record { seq, msg: Some(msg) });
        next = seq + 1;
    }
    let ops = batches.last_mut().expect("one at least");
    if header.next_outgoing > next {
        ops.push(Op::Record { seq: header.next_outgoing - 1, msg: None });
    }
    ops.push(match header.in_flight {
        Some(in_flight) => Op::SetNextIncomingInFlight { seq: header.next_incoming, in_flight },
        None => Op::SetNextIncoming(header.next_incoming),
    });
    ops.push(Op::SetMarks(header.marks));
    for ops in batches {
        shared.link.send(batch(handle, shared.epoch, commit_no, ops)).run()?;
    }
    Ok(())
}

/// The next batch on `handle`, of `ops`.
fn batch(handle: u32, epoch: u64, commit_no: &AtomicU64, ops: Vec<Op>) -> Batch {
    let commit_no = commit_no.fetch_add(1, Ordering::AcqRel) + 1;
    Batch { handle, epoch, commit_no, ops, app_records: Vec::new() }
}

/// A session's log in `S`, recording each change it makes for the next commit's batch.
struct ReplicatedLog {
    inner: Box<dyn StoredSession>,
    shared: Arc<Replication>,
    session: SessionId,
    handle: u32,
    /// The commits sent on this binding: the standby starts counting again at each.
    commit_no: Arc<AtomicU64>,
    /// Bound on the standby, and sent the session: false while the standby is behind on it.
    bound: Arc<AtomicBool>,
    /// The changes since the last commit.
    ops: Vec<Op>,
    /// Asynchronous, the batches sent that no commit has waited for yet, oldest first, each with
    /// the highest outgoing number it covers. Dropped with the log unrun: the next opening
    /// compares the standby's copy and sends the session whole if it lacks them.
    queued: VecDeque<(u64, Commit)>,
    /// The highest outgoing number covered by a batch a commit has waited for.
    drained_through: u64,
}

impl ReplicatedLog {
    /// Records `op` once `S` has made it.
    fn made(&mut self, result: io::Result<()>, op: Op) -> io::Result<()> {
        result?;
        self.ops.push(op);
        Ok(())
    }

    /// Whether the standby is behind on this session: it's sent whole at the next commit.
    fn behind(&self) -> bool {
        !self.bound.load(Ordering::Acquire) || self.shared.is_behind(&self.session)
    }

    /// Queues `remote`, the batch covering outgoing numbers through `last`, and returns a job
    /// waiting for the oldest batches queued, in order, until the session's unacknowledged
    /// numbers are within the bound again: a batch with more numbers than the bound waits for
    /// itself.
    fn queue(&mut self, last: u64, remote: Commit) -> Option<Commit> {
        self.queued.push_back((last, remote));
        let mut drain = Vec::new();
        while last.saturating_sub(self.drained_through) > self.shared.max_lag || self.queued.len() > MAX_QUEUED {
            let (through, job) = self.queued.pop_front().expect("over the bound with batches queued");
            self.drained_through = through;
            drain.push(job);
        }
        if drain.is_empty() {
            return None;
        }
        // The numbers drained count as held from now: a job that panics, or is dropped unrun,
        // unbinds the session so its next commit sends it whole. An error has been through
        // `lost` already.
        let unbinding = Unbinding(Some(self.bound.clone()));
        Some(Commit::blocking(move || {
            let ran = drain.into_iter().try_for_each(Job::run);
            unbinding.disarm();
            ran
        }))
    }

    /// The standby's half of a commit of `ops`, made in `S` already: the batch, or the session
    /// whole if the standby is behind on it.
    fn remote(&mut self, ops: Vec<Op>) -> io::Result<Commit> {
        let (shared, session, bound) = (self.shared.clone(), self.session.clone(), self.bound.clone());
        if !self.behind() {
            let sent = shared.link.send(batch(self.handle, shared.epoch, &self.commit_no, ops));
            return Ok(sent.map_result(move |sent| match sent {
                Ok(()) => Ok(()),
                Err(e) => lost(&shared, &session, &bound, e),
            }));
        }
        // What this commit makes is in the session as `S` now has it.
        let Whole { header, fetched } = Whole::of(self.inner.as_mut())?;
        let (handle, commit_no) = (self.handle, self.commit_no.clone());
        Ok(Commit::blocking(move || {
            let sent = shared.link.bind(handle, shared.epoch, shared.max_lag, &session).and_then(|_| {
                commit_no.store(0, Ordering::Release);
                send_whole(&shared, handle, &commit_no, header, messages(fetched)?)
            });
            match sent {
                Ok(()) => {
                    bound.store(true, Ordering::Release);
                    shared.caught_up(&session);
                    Ok(())
                }
                Err(e) => lost(&shared, &session, &bound, e),
            }
        }))
    }
}

/// Unbinds a session when dropped holding its flag.
struct Unbinding(Option<Arc<AtomicBool>>);

impl Unbinding {
    fn disarm(mut self) {
        self.0.take();
    }
}

impl Drop for Unbinding {
    fn drop(&mut self) {
        if let Some(bound) = self.0.take() {
            bound.store(false, Ordering::Release);
        }
    }
}

/// A send of `session`'s failed with `e`: unless another instance holds the lease (the binding
/// stands, and the standby refuses it again), the standby is behind on it, as
/// [`Replication::fall_behind`] has it.
fn lost(shared: &Replication, session: &SessionId, bound: &AtomicBool, e: io::Error) -> io::Result<()> {
    if e.kind() != io::ErrorKind::PermissionDenied {
        bound.store(false, Ordering::Release);
    }
    shared.fall_behind(session, e)
}

impl Drop for ReplicatedLog {
    fn drop(&mut self) {
        self.shared.closed(&self.session);
    }
}

impl StoredSession for ReplicatedLog {
    fn next_outgoing(&self) -> u64 {
        self.inner.next_outgoing()
    }

    fn next_incoming(&self) -> u64 {
        self.inner.next_incoming()
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        let result = self.inner.set_next_incoming(seq);
        self.made(result, Op::SetNextIncoming(seq))
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        let result = self.inner.record_outgoing(seq, msg);
        self.made(result, Op::Record { seq, msg: msg.map(<[u8]>::to_vec) })
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<SentMessages> {
        self.inner.sent_messages(begin, end)
    }

    fn fetch(&mut self, begin: u64, end: u64) -> io::Result<Fetched> {
        self.inner.fetch(begin, end)
    }

    fn commit(&mut self) -> io::Result<Option<Commit>> {
        if self.ops.is_empty() {
            return self.inner.commit();
        }
        let ops = std::mem::take(&mut self.ops);
        let local = self.inner.commit()?;
        let was_behind = self.behind();
        let remote = self.remote(ops)?;
        let last = self.inner.next_outgoing() - 1;
        let mut remote = if self.shared.max_lag == 0 {
            Some(remote)
        } else if was_behind {
            // The batches queued went on a connection since lost, or are moot: this sends the
            // session whole.
            self.queued.clear();
            self.drained_through = last;
            Some(remote)
        } else {
            self.queue(last, remote)
        };
        if let Some(renewal) = self.shared.held.as_ref().and_then(Held::renewal) {
            remote = Some(match remote {
                Some(remote) => renewal.join(remote),
                None => renewal,
            });
        }
        Ok(match (local, remote) {
            (Some(local), Some(remote)) => Some(local.join(remote)),
            (local, remote) => local.or(remote),
        })
    }

    /// Written at once, as the built-in stores write a reset: the standby resets first, with the
    /// changes before it, so a standby that refuses (a deposed active) leaves both copies as they
    /// were, and a failover never finds one reset and the other not. Resets are rare (at logon,
    /// a new schedule period, an operator's), so the wait for the standby is too. Degrading, a
    /// standby behind on the session, or that can't take the reset, gets the session whole later.
    fn reset(&mut self) -> io::Result<()> {
        let mut ops = std::mem::take(&mut self.ops);
        ops.push(Op::Reset);
        // The standby takes the batches queued before it first, or it would refuse them after.
        let drained = std::mem::take(&mut self.queued).into_iter().try_for_each(|(_, job)| job.run());
        self.drained_through = 0;
        let sent = if let Err(e) = drained {
            Err(e)
        } else if self.behind() {
            Err(io::Error::other("the standby is behind on the session"))
        } else {
            self.shared.link.send(batch(self.handle, self.shared.epoch, &self.commit_no, ops)).run()
        };
        if let Err(e) = sent {
            lost(&self.shared, &self.session, &self.bound, e)?;
        }
        self.inner.reset()
    }

    fn evicted_through(&self) -> Option<u64> {
        self.inner.evicted_through()
    }

    fn in_flight(&self) -> Option<u64> {
        self.inner.in_flight()
    }

    fn set_next_incoming_in_flight(&mut self, seq: u64, in_flight: u64) -> io::Result<()> {
        let result = self.inner.set_next_incoming_in_flight(seq, in_flight);
        self.made(result, Op::SetNextIncomingInFlight { seq, in_flight })
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        let result = self.inner.set_in_flight(seq);
        self.made(result, Op::SetInFlight(seq))
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.inner.created_at()
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        let result = self.inner.set_created_at(at);
        self.made(result, Op::SetCreatedAt(at))
    }

    fn marks(&self) -> Marks {
        self.inner.marks()
    }

    fn set_marks(&mut self, marks: Marks) -> io::Result<()> {
        let result = self.inner.set_marks(marks);
        self.made(result, Op::SetMarks(marks))
    }
}

/// A [`Link`] to a [`Replica`] in the same process, applying each batch on the committing
/// thread: for tests, and for the simulator, whose store jobs must block.
pub struct LocalLink<S> {
    replica: Arc<Mutex<Replica<S>>>,
}

impl<S> std::fmt::Debug for LocalLink<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalLink").finish_non_exhaustive()
    }
}

impl<S: SessionStorage + 'static> LocalLink<S> {
    /// Connects to `replica` as the active holding `epoch`.
    ///
    /// # Errors
    ///
    /// [`io::ErrorKind::PermissionDenied`] if the replica has seen a higher epoch: another
    /// instance has taken over.
    pub fn connect(replica: Arc<Mutex<Replica<S>>>, epoch: u64, node: &str) -> io::Result<Self> {
        let hello = Frame::Hello { version: VERSION, epoch, node: node.into(), app_record_kinds: Vec::new() };
        match replica.lock().unwrap_or_else(PoisonError::into_inner).on_frame(hello)? {
            Some(Frame::Welcome { epoch: seen }) if seen == epoch => {}
            Some(Frame::Welcome { epoch: seen }) => return Err(deposed(epoch, seen)),
            other => return Err(io::Error::other(format!("the standby answered hello with {other:?}"))),
        }
        Ok(Self { replica })
    }
}

impl<S: SessionStorage + Send + 'static> Link for LocalLink<S> {
    fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary> {
        let bind = Frame::Bind { handle, epoch, max_lag, session: session.clone() };
        let mut replica = self.replica.lock().unwrap_or_else(PoisonError::into_inner);
        match replica.on_frame(bind)? {
            Some(Frame::Bound { summary, .. }) => Ok(summary),
            Some(Frame::Nack { reason: NackReason::StaleEpoch, .. }) => Err(deposed(epoch, replica.epoch())),
            Some(Frame::Nack { reason: NackReason::Lag, .. }) => Err(lag_refused()),
            other => Err(io::Error::other(format!("the standby answered a bind with {other:?}"))),
        }
    }

    fn send(&self, batch: Batch) -> Commit {
        let replica = self.replica.clone();
        Commit::blocking(move || {
            let mut replica = replica.lock().unwrap_or_else(PoisonError::into_inner);
            let epoch = batch.epoch;
            match replica.on_frame(Frame::Batch(batch))? {
                Some(Frame::Ack { .. }) => Ok(()),
                Some(Frame::Nack { reason: NackReason::StaleEpoch, .. }) => Err(deposed(epoch, replica.epoch())),
                Some(Frame::Nack { reason: NackReason::Diverged, .. }) => {
                    Err(io::Error::other("the standby's copy of the session has diverged"))
                }
                other => Err(io::Error::other(format!("the standby answered a batch with {other:?}"))),
            }
        })
    }
}

/// The error for a bind the standby refused: it accepts less lag than the active's bound.
pub(crate) fn lag_refused() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "the standby accepts less lag than this active's bound")
}

/// The error for an active whose epoch is behind the standby's: another instance holds the lease.
pub(crate) fn deposed(epoch: u64, seen: u64) -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("this instance's lease (epoch {epoch}) has been taken over (epoch {seen})"),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::atomic::AtomicBool;

    use turbojet::MemoryStorage;
    use turbojet::store::conformance;

    use super::*;

    fn pair() -> (ReplicatedStorage<MemoryStorage>, Arc<Mutex<Replica<MemoryStorage>>>) {
        let replica = Arc::new(Mutex::new(Replica::new(MemoryStorage::new())));
        let link = LocalLink::connect(replica.clone(), 1, "active").unwrap();
        (ReplicatedStorage::new(MemoryStorage::new(), Arc::new(link), 1), replica)
    }

    #[test]
    fn conforms() {
        conformance::check_blocking(&pair().0);
        conformance::check_marks_blocking(&pair().0);
        conformance::check_sessions_blocking(&pair().0);
    }

    /// Takes over at a session's second opening, and every one after: the conformance model
    /// then reads, and goes on in, the standby's copy, which must hold what was committed.
    struct FailingOver {
        active: ReplicatedStorage<MemoryStorage>,
        standby: Arc<Mutex<Replica<MemoryStorage>>>,
        opened: Mutex<HashSet<SessionId>>,
    }

    impl SessionStorage for FailingOver {
        fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
            if self.opened.lock().unwrap().insert(id.clone()) {
                self.active.open(id)
            } else {
                self.standby.lock().unwrap().storage().open(id)
            }
        }

        fn is_durable(&self) -> bool {
            false
        }
    }

    #[test]
    fn the_standby_holds_what_was_committed() {
        let (active, standby) = pair();
        let storage = FailingOver { active, standby, opened: Mutex::default() };
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(conformance::check_model(&storage, 0..50));
    }

    #[test]
    fn a_deposed_active_cant_commit() {
        let (active, standby) = pair();
        let mut log = active.open(&conformance::id("A")).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        log.commit().unwrap().unwrap().run().unwrap();

        let _taken = LocalLink::connect(standby.clone(), 2, "other").unwrap();
        // Its handles would collide with the new active's, so it can't bind a session either.
        let failed = active.open(&conformance::id("B")).err().expect("a stale bind is refused");
        assert_eq!(failed.kind(), io::ErrorKind::PermissionDenied, "{failed}");
        assert_eq!(
            LocalLink::connect(standby, 1, "again").err().map(|e| e.kind()),
            Some(io::ErrorKind::PermissionDenied),
            "the old epoch can't connect again"
        );
        log.record_outgoing(2, Some(&conformance::app_message(2))).unwrap();
        let failed = log.commit().unwrap().unwrap().run().unwrap_err();
        assert_eq!(failed.kind(), io::ErrorKind::PermissionDenied, "{failed}");

        // Nor reset: neither copy is, so a failover can't find one reset and the other not.
        let next = log.next_outgoing();
        assert_eq!(log.reset().unwrap_err().kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(log.next_outgoing(), next, "its own copy as it was");
    }

    /// A session the standby lacks is sent whole: its numbers, the numbers used without a message
    /// between those stored, the in-flight marker, created time and marks, and its messages, in
    /// batches when they're many.
    #[test]
    fn a_session_the_standby_lacks_is_sent_whole() {
        let id = conformance::id("A");
        let inner = MemoryStorage::new();
        let mut log = inner.open(&id).unwrap();
        let mut seq = 1;
        for _ in 0..40_000 {
            log.record_outgoing(seq, None).unwrap();
            log.record_outgoing(seq + 1, Some(&conformance::app_message(seq + 1))).unwrap();
            log.record_outgoing(seq + 2, Some(&conformance::app_message(seq + 2))).unwrap();
            seq += 3;
        }
        log.record_outgoing(seq + 4, None).unwrap();
        log.set_next_incoming_in_flight(70, 66).unwrap();
        log.set_created_at(UtcTimestamp::from_timestamp(1_800_000_000, 0).unwrap()).unwrap();
        log.set_marks("paused".parse().unwrap()).unwrap();
        let mine = summary(log.as_ref());
        let messages = log.sent_messages(1, u64::MAX).unwrap();
        assert!(messages.iter().map(|(_, msg)| msg.len()).sum::<usize>() > 2 * SNAPSHOT_BATCH_BYTES);
        drop(log);

        let replica = Arc::new(Mutex::new(Replica::new(MemoryStorage::new())));
        let active = ReplicatedStorage::new(inner, Arc::new(LocalLink::connect(replica.clone(), 1, "a").unwrap()), 1);
        drop(active.open(&id).unwrap());
        let mut copy = replica.lock().unwrap().storage().open(&id).unwrap();
        assert_eq!(summary(copy.as_ref()), mine);
        assert_eq!(copy.sent_messages(1, u64::MAX).unwrap(), messages);
        drop(copy);

        // The same again isn't sent: nothing to replace.
        assert!(active.open(&id).unwrap().commit().unwrap().is_none());
    }

    /// A lease another instance has taken fails the commit, though the standby would take it:
    /// fencing that doesn't need the standby.
    #[test]
    fn a_lease_taken_fails_commits() {
        struct Taken;

        impl Lease for Taken {
            fn duration(&self) -> std::time::Duration {
                std::time::Duration::ZERO
            }

            fn acquire(&self) -> Job<u64> {
                Job::blocking(|| Ok(1))
            }

            fn renew(&self, _epoch: u64) -> Job<()> {
                Job::blocking(|| Err(io::Error::new(io::ErrorKind::PermissionDenied, "taken by gw-b")))
            }

            fn release(&self, _epoch: u64) -> Job<()> {
                Job::blocking(|| Ok(()))
            }

            fn set_in_sync(&self, _epoch: u64, _in_sync: bool) -> Job<()> {
                Job::blocking(|| Ok(()))
            }

            fn in_sync(&self) -> Job<bool> {
                Job::blocking(|| Ok(true))
            }
        }

        let (active, _standby) = pair();
        let active = active.with_lease(Arc::new(Taken));
        let mut log = active.open(&conformance::id("A")).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        let failed = log.commit().unwrap().unwrap().run().unwrap_err();
        assert_eq!(failed.to_string(), "taken by gw-b");
    }

    /// A standby refuses a bind from an active that may leave more unacknowledged than it accepts.
    #[test]
    fn a_standby_refuses_more_lag_than_it_accepts() {
        let bind = |replica: Replica<MemoryStorage>| {
            let mut replica = replica;
            let hello = Frame::Hello { version: VERSION, epoch: 1, node: "a".into(), app_record_kinds: Vec::new() };
            replica.on_frame(hello).unwrap();
            replica.on_frame(Frame::Bind { handle: 1, epoch: 1, max_lag: 4, session: conformance::id("A") }).unwrap()
        };
        assert_eq!(bind(Replica::new(MemoryStorage::new())), Some(Frame::Nack { handle: 1, reason: NackReason::Lag }));
        assert!(matches!(bind(Replica::new(MemoryStorage::new()).accepting_lag(4)), Some(Frame::Bound { .. })));
    }

    #[test]
    fn promotion_skips_each_session_ahead_by_the_lag_accepted() {
        let promoted = |max_lag| {
            let mut replica = Replica::new(MemoryStorage::new()).accepting_lag(max_lag);
            let hello = Frame::Hello { version: VERSION, epoch: 1, node: "a".into(), app_record_kinds: Vec::new() };
            replica.on_frame(hello).unwrap();
            let bind = Frame::Bind { handle: 1, epoch: 1, max_lag, session: conformance::id("A") };
            replica.on_frame(bind).unwrap();
            let ops = (1..=5).map(|seq| Op::Record { seq, msg: Some(conformance::app_message(seq)) }).collect();
            let reply = replica.on_frame(Frame::Batch(batch(1, 1, &AtomicU64::new(0), ops))).unwrap();
            assert_eq!(reply, Some(Frame::Ack { handle: 1, commit_no: 1 }));
            replica.promote().unwrap();
            replica.storage().open(&conformance::id("A")).unwrap()
        };

        let mut log = promoted(3);
        assert_eq!(log.next_outgoing(), 9, "past what the active may have sent unacknowledged");
        assert!(log.sent_messages(6, 9).unwrap().is_empty(), "the skipped numbers are gap-filled");
        assert_eq!(log.sent_messages(1, 5).unwrap().len(), 5);
        assert_eq!(promoted(0).next_outgoing(), 6);
    }

    /// A link the test can take down and bring back.
    struct Outage {
        link: LocalLink<MemoryStorage>,
        down: AtomicBool,
    }

    impl Link for Outage {
        fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary> {
            if self.down.load(Ordering::Acquire) {
                return Err(io::ErrorKind::ConnectionRefused.into());
            }
            self.link.bind(handle, epoch, max_lag, session)
        }

        fn send(&self, batch: Batch) -> Commit {
            if self.down.load(Ordering::Acquire) {
                return Commit::blocking(|| Err(io::ErrorKind::ConnectionRefused.into()));
            }
            self.link.send(batch)
        }
    }

    /// A lease that remembers whether it was told the standby is in sync, and can't be told when
    /// `unreachable`.
    #[derive(Default)]
    struct Remembering {
        out_of_sync: AtomicBool,
        unreachable: AtomicBool,
    }

    impl Lease for Remembering {
        fn duration(&self) -> std::time::Duration {
            std::time::Duration::MAX
        }

        fn acquire(&self) -> Job<u64> {
            Job::blocking(|| Ok(1))
        }

        fn renew(&self, _epoch: u64) -> Job<()> {
            Job::blocking(|| Ok(()))
        }

        fn release(&self, _epoch: u64) -> Job<()> {
            Job::blocking(|| Ok(()))
        }

        fn set_in_sync(&self, _epoch: u64, in_sync: bool) -> Job<()> {
            if self.unreachable.load(Ordering::Acquire) {
                return Job::blocking(|| Err(io::Error::other("database down")));
            }
            self.out_of_sync.store(!in_sync, Ordering::Release);
            Job::blocking(|| Ok(()))
        }

        fn in_sync(&self) -> Job<bool> {
            let in_sync = !self.out_of_sync.load(Ordering::Acquire);
            Job::blocking(move || Ok(in_sync))
        }
    }

    struct Degrading {
        active: ReplicatedStorage<Arc<MemoryStorage>>,
        own: Arc<MemoryStorage>,
        standby: Arc<Mutex<Replica<MemoryStorage>>>,
        link: Arc<Outage>,
        lease: Arc<Remembering>,
    }

    fn degrading(own: Arc<MemoryStorage>, lease: Arc<Remembering>) -> Degrading {
        let standby = Arc::new(Mutex::new(Replica::new(MemoryStorage::new())));
        let link = Arc::new(Outage {
            link: LocalLink::connect(standby.clone(), 1, "a").unwrap(),
            down: AtomicBool::new(false),
        });
        let active = ReplicatedStorage::new(own.clone(), link.clone(), 1).with_lease(lease.clone()).degrading();
        Degrading { active, own, standby, link, lease }
    }

    fn store_and_commit(log: &mut dyn StoredSession, seq: u64) -> io::Result<()> {
        log.record_outgoing(seq, Some(&conformance::app_message(seq)))?;
        log.commit()?.map_or(Ok(()), Commit::run)
    }

    fn standby_has(standby: &Mutex<Replica<MemoryStorage>>, id: &SessionId) -> u64 {
        standby.lock().unwrap().storage().open(id).unwrap().next_outgoing()
    }

    /// With the standby down, a degrading store commits alone, once the lease says the standby
    /// is out of sync; back, an open session sends itself whole at its next commit, and the
    /// catch-up the rest, then tells the lease the standby is in sync.
    #[test]
    fn a_degrading_store_commits_alone_and_catches_the_standby_up() {
        let d = degrading(Arc::default(), Arc::default());
        let (open_id, closed_id) = (conformance::id("A"), conformance::id("B"));
        let mut open = d.active.open(&open_id).unwrap();
        let mut closed = d.active.open(&closed_id).unwrap();
        store_and_commit(open.as_mut(), 1).unwrap();
        store_and_commit(closed.as_mut(), 1).unwrap();

        d.link.down.store(true, Ordering::Release);
        store_and_commit(open.as_mut(), 2).expect("committed alone");
        store_and_commit(closed.as_mut(), 2).expect("committed alone");
        assert!(d.lease.out_of_sync.load(Ordering::Acquire), "told before the commit counted");
        assert!(!d.active.in_sync());
        drop(closed);
        d.active.catch_up().run().unwrap();
        assert!(!d.active.in_sync(), "the standby can't be caught up while it's down");

        d.link.down.store(false, Ordering::Release);
        store_and_commit(open.as_mut(), 3).unwrap();
        assert_eq!(standby_has(&d.standby, &open_id), 4, "the open session sent itself whole");
        assert_eq!(standby_has(&d.standby, &closed_id), 2, "the closed one waits for the catch-up");
        assert!(!d.active.in_sync());
        d.active.catch_up().run().unwrap();
        assert_eq!(standby_has(&d.standby, &closed_id), 3);
        assert!(d.active.in_sync());
        assert!(!d.lease.out_of_sync.load(Ordering::Acquire));
    }

    /// A manual lease can't fence a replaced active while the standby is down, so degrading with
    /// one fails the commit rather than make it alone.
    #[test]
    fn a_manual_lease_doesnt_degrade() {
        let standby = Arc::new(Mutex::new(Replica::new(MemoryStorage::new())));
        let link =
            Arc::new(Outage { link: LocalLink::connect(standby, 1, "a").unwrap(), down: AtomicBool::new(false) });
        let active = ReplicatedStorage::new(MemoryStorage::new(), link.clone(), 1)
            .with_lease(Arc::new(crate::ManualLease::new(1)))
            .degrading();
        let mut log = active.open(&conformance::id("A")).unwrap();
        link.down.store(true, Ordering::Release);
        let failed = store_and_commit(log.as_mut(), 1).unwrap_err();
        assert_eq!(failed.kind(), io::ErrorKind::Unsupported, "{failed}");
    }

    /// A commit the standby lacks is made only once the lease says so.
    #[test]
    fn a_lease_that_cant_be_told_fails_the_commit() {
        let d = degrading(Arc::default(), Arc::default());
        let mut log = d.active.open(&conformance::id("A")).unwrap();
        d.link.down.store(true, Ordering::Release);
        d.lease.unreachable.store(true, Ordering::Release);
        assert!(store_and_commit(log.as_mut(), 1).is_err());
    }

    /// Opened while the standby is down, a session is behind from the start, and sent whole once
    /// it's back.
    #[test]
    fn a_session_opens_while_the_standby_is_down() {
        let d = degrading(Arc::default(), Arc::default());
        d.link.down.store(true, Ordering::Release);
        let id = conformance::id("A");
        let mut log = d.active.open(&id).expect("opened alone");
        store_and_commit(log.as_mut(), 1).unwrap();
        d.link.down.store(false, Ordering::Release);
        store_and_commit(log.as_mut(), 2).unwrap();
        assert_eq!(standby_has(&d.standby, &id), 3);
    }

    /// A store started while its lease says the standby is out of sync counts every session it
    /// holds behind, and catches the standby up.
    #[test]
    fn a_restart_out_of_sync_catches_every_session_up() {
        let own: Arc<MemoryStorage> = Arc::default();
        let id = conformance::id("A");
        let mut log = own.open(&id).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        drop(log);
        let lease = Arc::new(Remembering::default());
        lease.out_of_sync.store(true, Ordering::Release);
        let d = degrading(own, lease);
        d.active.catch_up().run().unwrap();
        assert_eq!(standby_has(&d.standby, &id), 2);
        assert!(d.active.in_sync() && !d.lease.out_of_sync.load(Ordering::Acquire));
        drop(d.own);
    }

    /// A pair whose active leaves up to `max_lag` outgoing numbers unacknowledged, and whose
    /// standby accepts it.
    fn async_pair(max_lag: u64) -> (ReplicatedStorage<MemoryStorage>, Arc<Mutex<Replica<MemoryStorage>>>) {
        let replica = Arc::new(Mutex::new(Replica::new(MemoryStorage::new()).accepting_lag(max_lag)));
        let link = LocalLink::connect(replica.clone(), 1, "active").unwrap();
        (ReplicatedStorage::new(MemoryStorage::new(), Arc::new(link), 1).asynchronous(max_lag), replica)
    }

    #[test]
    fn async_commits_conform() {
        conformance::check_blocking(&async_pair(4).0);
        conformance::check_marks_blocking(&async_pair(4).0);
        conformance::check_sessions_blocking(&async_pair(4).0);
    }

    /// `LocalLink` applies a batch when its job runs, so the standby's copy shows which of the
    /// batches queued a commit's job has waited for.
    #[test]
    fn the_lag_is_bounded() {
        let (active, standby) = async_pair(3);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        for seq in 1..=3 {
            store_and_commit(log.as_mut(), seq).unwrap();
        }
        assert_eq!(standby_has(&standby, &id), 1, "three unacknowledged is within the bound");
        store_and_commit(log.as_mut(), 4).unwrap();
        assert_eq!(standby_has(&standby, &id), 2, "the fourth waits for the first");
    }

    #[test]
    fn a_batch_over_the_bound_waits_for_itself() {
        let (active, standby) = async_pair(2);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        for seq in 1..=3 {
            log.record_outgoing(seq, Some(&conformance::app_message(seq))).unwrap();
        }
        log.commit().unwrap().unwrap().run().unwrap();
        assert_eq!(standby_has(&standby, &id), 4);
    }

    /// A batch sent while the standby is down fails the commit that waits for it, two later with
    /// a bound of two; degrading, that commit is made alone instead, and once the standby is
    /// back the session is sent whole, the batches queued during the outage dropped unrun.
    #[test]
    fn a_failed_answer_fails_a_later_commit() {
        let run = |degrade: bool| {
            let standby = Arc::new(Mutex::new(Replica::new(MemoryStorage::new()).accepting_lag(2)));
            let link = Arc::new(Outage {
                link: LocalLink::connect(standby.clone(), 1, "a").unwrap(),
                down: AtomicBool::new(false),
            });
            let mut active = ReplicatedStorage::new(MemoryStorage::new(), link.clone(), 1).asynchronous(2);
            if degrade {
                active = active.with_lease(Arc::new(Remembering::default())).degrading();
            }
            let mut log = active.open(&conformance::id("A")).unwrap();
            store_and_commit(log.as_mut(), 1).unwrap();
            link.down.store(true, Ordering::Release);
            store_and_commit(log.as_mut(), 2).unwrap();
            store_and_commit(log.as_mut(), 3).expect("waits only for the batch sent before the outage");
            (store_and_commit(log.as_mut(), 4), active, log, link, standby)
        };
        let (failed, ..) = run(false);
        assert_eq!(failed.unwrap_err().kind(), io::ErrorKind::ConnectionRefused);
        let (made, active, mut log, link, standby) = run(true);
        made.expect("made alone");
        assert!(!active.in_sync());
        // Its first pass asks the lease, which counts every session behind, this one included.
        active.catch_up().run().unwrap();

        link.down.store(false, Ordering::Release);
        store_and_commit(log.as_mut(), 5).unwrap();
        assert_eq!(standby_has(&standby, &conformance::id("A")), log.next_outgoing(), "sent whole");
        // Far enough past the bound that a batch from the outage, still queued, would be waited for.
        for seq in 6..=8 {
            store_and_commit(log.as_mut(), seq).unwrap();
        }
        active.catch_up().run().unwrap();
        assert!(active.in_sync());
    }

    /// The worst case for a failover: `LocalLink` applies a batch only when a commit waits for it,
    /// so the standby lacks the last batches the active sent. Promoted, it skips past them: the
    /// next number it sends is above every number the active sent, and it holds only messages
    /// the active stored, none in the numbers skipped.
    #[test]
    fn an_async_failover_resumes_above_every_number_sent() {
        let (active, standby) = async_pair(3);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        for seq in 1..=10 {
            store_and_commit(log.as_mut(), seq).unwrap();
        }
        let sent = log.sent_messages(1, u64::MAX).unwrap();
        assert_eq!(sent.iter().map(|(seq, _)| *seq).collect::<Vec<_>>(), (1..=10).collect::<Vec<_>>());
        drop(log);

        let held_through = standby_has(&standby, &id) - 1;
        assert!(held_through < 10, "the standby lacks the batches no commit waited for");
        standby.lock().unwrap().promote().unwrap();
        let mut taken = standby.lock().unwrap().storage().open(&id).unwrap();
        assert!(taken.next_outgoing() > 10, "{}", taken.next_outgoing());
        let held = taken.sent_messages(1, u64::MAX).unwrap();
        assert!(!held.is_empty());
        for message in &held {
            assert!(sent.contains(message), "{} isn't one the active stored", message.0);
            assert!(message.0 <= held_through, "{} is in the numbers skipped", message.0);
        }
    }

    /// A commit's job dropped unrun leaves the batches it would have waited for unacknowledged:
    /// the next commit sends the session whole rather than count them held.
    #[test]
    fn a_commit_dropped_unrun_sends_the_session_whole_next() {
        let (active, standby) = async_pair(2);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        for seq in 1..=3 {
            log.record_outgoing(seq, Some(&conformance::app_message(seq))).unwrap();
        }
        drop(log.commit().unwrap().unwrap());
        store_and_commit(log.as_mut(), 4).unwrap();
        assert_eq!(standby_has(&standby, &id), 5);
    }

    #[test]
    fn an_async_active_needs_a_standby_that_accepts_it() {
        let replica = Arc::new(Mutex::new(Replica::new(MemoryStorage::new())));
        let link = LocalLink::connect(replica, 1, "active").unwrap();
        let active = ReplicatedStorage::new(MemoryStorage::new(), Arc::new(link), 1).asynchronous(4);
        let failed = active.open(&conformance::id("A")).err().expect("the bind is refused");
        assert_eq!(failed.kind(), io::ErrorKind::InvalidInput, "{failed}");
    }

    /// The standby takes the batches queued before a reset first: in order, so it doesn't refuse
    /// them as diverged, and the reset leaves its copy as the active's.
    #[test]
    fn a_reset_runs_the_batches_queued_first() {
        let (active, standby) = async_pair(4);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        // One batch over the bound, which waits for itself, then two queued.
        for seq in 1..=10 {
            log.record_outgoing(seq, Some(&conformance::app_message(seq))).unwrap();
        }
        log.commit().unwrap().unwrap().run().unwrap();
        store_and_commit(log.as_mut(), 11).unwrap();
        store_and_commit(log.as_mut(), 12).unwrap();
        log.reset().unwrap();
        assert_eq!(standby_has(&standby, &id), 1);

        // The lag counts from the reset, not from the numbers before it.
        for seq in 1..=4 {
            store_and_commit(log.as_mut(), seq).unwrap();
        }
        assert_eq!(standby_has(&standby, &id), 1);
        store_and_commit(log.as_mut(), 5).unwrap();
        assert_eq!(standby_has(&standby, &id), 2);
    }

    /// Nothing changed, nothing to copy: the commit is the local store's alone.
    #[test]
    fn an_empty_commit_isnt_sent() {
        let (active, _standby) = pair();
        let mut log = active.open(&conformance::id("A")).unwrap();
        assert!(log.commit().unwrap().is_none());
    }
}
