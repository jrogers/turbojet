//! The active's side: a store that copies each session's commits to a standby.

use std::io;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use turbojet::SessionId;
use turbojet::fields::UtcTimestamp;
use turbojet::store::{Commit, Fetched, Job, Marks, Opened, SentMessages, SessionStorage, StoredSession};

use crate::frame::{Batch, Frame, NackReason, Op, Summary, VERSION};
use crate::lease::{Held, Lease};
use crate::standby::{Replica, summary};

/// The most a batch of a session sent whole carries in messages: a large session is sent in
/// several, well within a frame each.
const SNAPSHOT_BATCH_BYTES: usize = 1024 * 1024;

/// How batches reach the standby. Its calls block until the standby answers, so a store using
/// it runs them on a blocking thread, in its jobs.
pub trait Link: Send + Sync {
    /// Binds `handle` to `session` on the standby, as the holder of `epoch`, returning its copy
    /// of the session.
    ///
    /// # Errors
    ///
    /// The standby can't be reached, or refused.
    fn bind(&self, handle: u32, epoch: u64, session: &SessionId) -> io::Result<Summary>;

    /// Sends a batch for a bound handle: a job that ends once the standby has committed it,
    /// failing if the standby refused it or can't be reached.
    fn send(&self, batch: Batch) -> Commit;
}

/// A [`SessionStorage`] that keeps each session in `S` and copies every commit to a standby over
/// a [`Link`]: a session's commit, and so everything it covers going out, waits for both.
///
/// A commit that one side made and the other didn't leaves that side ahead of what was sent, as
/// a crash between a commit and its write does, which a session's recovery already handles.
pub struct ReplicatedStorage<S> {
    inner: S,
    link: Arc<dyn Link>,
    /// The epoch of the lease this instance holds.
    epoch: u64,
    /// The lease itself, renewed in commits, if it was given.
    held: Option<Arc<Held>>,
    next_handle: AtomicU32,
}

impl<S: SessionStorage> ReplicatedStorage<S> {
    /// Keeps sessions in `inner`, copying their commits over `link`, as the holder of the lease
    /// for `epoch`.
    pub fn new(inner: S, link: Arc<dyn Link>, epoch: u64) -> Self {
        Self { inner, link, epoch, held: None, next_handle: AtomicU32::new(0) }
    }

    /// Renews `lease`, taken under this store's epoch, in its sessions' commits once half of it
    /// has gone; a commit fails once another instance has taken it, or it has run out unrenewed.
    #[must_use]
    pub fn with_lease(mut self, lease: Arc<dyn Lease>) -> Self {
        self.held = Some(Held::new(lease, self.epoch));
        self
    }

    /// Gives the lease up, so the standby's instance can take it at once: for an orderly
    /// handover, once this instance's sessions have closed. Nothing to do without one.
    pub fn release(&self) -> Job<()> {
        match &self.held {
            Some(held) => held.release(),
            None => Job::blocking(|| Ok(())),
        }
    }

    fn attacher(&self, id: &SessionId) -> Attacher {
        Attacher {
            link: self.link.clone(),
            session: id.clone(),
            handle: self.next_handle.fetch_add(1, Ordering::Relaxed),
            epoch: self.epoch,
            held: self.held.clone(),
        }
    }
}

/// What it takes to attach a session's log in `S` to the standby, once it's open.
struct Attacher {
    link: Arc<dyn Link>,
    session: SessionId,
    handle: u32,
    epoch: u64,
    held: Option<Arc<Held>>,
}

impl Attacher {
    /// Binds the session on the standby and, if its copy differs from `inner`, sends it the
    /// session whole: the standby may have missed changes, made and never committed, or
    /// committed here before a failure kept them from it.
    fn attach(self, inner: Box<dyn StoredSession>) -> io::Result<Box<dyn StoredSession>> {
        let Self { link, session, handle, epoch, held } = self;
        let theirs = link.bind(handle, epoch, &session)?;
        let mut log = ReplicatedLog { inner, link, handle, epoch, held, commit_no: 0, ops: Vec::new() };
        if theirs != summary(log.inner.as_ref()) {
            log.send_whole()?;
        }
        Ok(Box::new(log))
    }
}

impl<S> std::fmt::Debug for ReplicatedStorage<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplicatedStorage").field("epoch", &self.epoch).finish_non_exhaustive()
    }
}

impl<S: SessionStorage> SessionStorage for ReplicatedStorage<S> {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
        self.attacher(id).attach(self.inner.open(id)?)
    }

    /// Always a job: attaching waits for the standby.
    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        let attacher = self.attacher(id);
        let job = match self.inner.begin_open(id)? {
            Opened::Ready(inner) => Job::blocking(move || attacher.attach(inner)),
            Opened::Pending(open) => Job::blocking(move || attacher.attach(open.run()?)),
        };
        Ok(Opened::Pending(job))
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

/// A session's log in `S`, recording each change it makes for the next commit's batch.
struct ReplicatedLog {
    inner: Box<dyn StoredSession>,
    link: Arc<dyn Link>,
    handle: u32,
    epoch: u64,
    held: Option<Arc<Held>>,
    /// The commits replicated so far.
    commit_no: u64,
    /// The changes since the last commit.
    ops: Vec<Op>,
}

impl ReplicatedLog {
    /// The next batch, of `ops`.
    fn batch(&mut self, ops: Vec<Op>) -> Batch {
        self.commit_no += 1;
        Batch { handle: self.handle, epoch: self.epoch, commit_no: self.commit_no, ops, app_records: Vec::new() }
    }

    /// Sends the session as `S` holds it, replacing the standby's copy, in batches of
    /// [`SNAPSHOT_BATCH_BYTES`] of messages at most.
    fn send_whole(&mut self) -> io::Result<()> {
        let log = self.inner.as_mut();
        let messages = match log.fetch(1, u64::MAX)? {
            Fetched::Ready(messages) => messages,
            Fetched::Pending(job) => job.run()?,
        };
        let mut batches = vec![vec![Op::Reset]];
        if let Some(at) = log.created_at() {
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
        if log.next_outgoing() > next {
            ops.push(Op::Record { seq: log.next_outgoing() - 1, msg: None });
        }
        ops.push(match log.in_flight() {
            Some(in_flight) => Op::SetNextIncomingInFlight { seq: log.next_incoming(), in_flight },
            None => Op::SetNextIncoming(log.next_incoming()),
        });
        ops.push(Op::SetMarks(log.marks()));
        for ops in batches {
            let batch = self.batch(ops);
            self.link.send(batch).run()?;
        }
        Ok(())
    }

    /// Records `op` once `S` has made it.
    fn made(&mut self, result: io::Result<()>, op: Op) -> io::Result<()> {
        result?;
        self.ops.push(op);
        Ok(())
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
        let batch = self.batch(ops);
        let local = self.inner.commit()?;
        let mut remote = self.link.send(batch);
        if let Some(renewal) = self.held.as_ref().and_then(Held::renewal) {
            remote = renewal.join(remote);
        }
        Ok(Some(match local {
            Some(local) => local.join(remote),
            None => remote,
        }))
    }

    /// Written at once, as the built-in stores write a reset: the standby resets first, with the
    /// changes before it, so a standby that refuses (a deposed active) leaves both copies as they
    /// were, and a failover never finds one reset and the other not. Resets are rare (at logon,
    /// a new schedule period, an operator's), so the wait for the standby is too.
    fn reset(&mut self) -> io::Result<()> {
        let mut ops = std::mem::take(&mut self.ops);
        ops.push(Op::Reset);
        let batch = self.batch(ops);
        self.link.send(batch).run()?;
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
    fn bind(&self, handle: u32, epoch: u64, session: &SessionId) -> io::Result<Summary> {
        let bind = Frame::Bind { handle, epoch, session: session.clone() };
        let mut replica = self.replica.lock().unwrap_or_else(PoisonError::into_inner);
        match replica.on_frame(bind)? {
            Some(Frame::Bound { summary, .. }) => Ok(summary),
            Some(Frame::Nack { reason: NackReason::StaleEpoch, .. }) => Err(deposed(epoch, replica.epoch())),
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
        }

        let (active, _standby) = pair();
        let active = active.with_lease(Arc::new(Taken));
        let mut log = active.open(&conformance::id("A")).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        let failed = log.commit().unwrap().unwrap().run().unwrap_err();
        assert_eq!(failed.to_string(), "taken by gw-b");
    }

    /// Nothing changed, nothing to copy: the commit is the local store's alone.
    #[test]
    fn an_empty_commit_isnt_sent() {
        let (active, _standby) = pair();
        let mut log = active.open(&conformance::id("A")).unwrap();
        assert!(log.commit().unwrap().is_none());
    }
}
