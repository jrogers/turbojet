//! A replicated acceptor store, for the seeds that have one: an active instance's store copying
//! each commit to a standby's, and the failovers between them.
//!
//! Both instances keep their copy in a `MemoryStorage`, standing, as for the unreplicated seeds,
//! for files that survive a process crash. The acceptor node is whichever instance is active: a
//! failover crashes it and restarts it over the standby's copy, at the next epoch, and the old
//! active's copy becomes the new standby's, sent whatever it lacks as each session opens.
//!
//! A degrading pair commits alone while the standby is down, once its lease says the standby is
//! out of sync, and catches it up when it's back; a failover waits until the lease says it's in
//! sync again, as a promotion checks it.

use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use turbojet::store::{Commit, Job, Marks, Opened, SessionStorage, StoredSession};
use turbojet::{MemoryStorage, SessionId};
use turbojet_replica::frame::{Batch, Summary};
use turbojet_replica::{Lease, Link, LocalLink, Replica, ReplicatedStorage};

type Store = Arc<MemoryStorage>;

/// The two instances' copies and which is active.
pub struct Pair {
    stores: [Store; 2],
    active: usize,
    /// The epoch of the active's lease, or after [`depose`](Self::depose) of the instance that
    /// took it.
    epoch: u64,
    /// The store the acceptor uses: the active's copy, replicating to the standby's.
    current: Arc<ReplicatedStorage<Store>>,
    /// The standby the active replicates to, which holds the epoch it fences with.
    standby: Arc<Mutex<Replica<Store>>>,
    /// Another instance has taken the lease: the acceptor runs on as a deposed active until the
    /// failover that follows.
    deposed: bool,
    /// With the planted bug of a stale standby, the batches sent so far.
    planted: Option<Arc<AtomicU64>>,
    /// Commits alone while the standby is down.
    degrade: bool,
    /// The lease the instances share, which says whether the standby is in sync.
    lease: Arc<SimLease>,
    /// The standby can't be reached.
    down: Arc<AtomicBool>,
    /// The planted bug of failing over to a standby the lease says is out of sync.
    unsynced_failover: bool,
}

/// What a pair is, and the bugs planted in it.
#[derive(Debug, Clone, Copy, Default)]
pub struct Setup {
    pub degrade: bool,
    /// The standby stops getting batches from the 10th on, though told they're committed.
    pub stale: bool,
    /// A failover goes ahead though the lease says the standby is out of sync.
    pub unsynced_failover: bool,
}

impl Pair {
    pub fn new(setup: Setup) -> Arc<Mutex<Self>> {
        let stores = [Arc::new(MemoryStorage::new()), Arc::new(MemoryStorage::new())];
        let mut pair = Self {
            stores,
            active: 0,
            epoch: 1,
            current: Arc::new(ReplicatedStorage::new(Arc::default(), Arc::new(Unbound), 0)),
            standby: Arc::new(Mutex::new(Replica::new(Arc::default()))),
            deposed: false,
            planted: setup.stale.then(Arc::default),
            degrade: setup.degrade,
            lease: Arc::default(),
            down: Arc::default(),
            unsynced_failover: setup.unsynced_failover,
        };
        pair.replicate();
        Arc::new(Mutex::new(pair))
    }

    /// Whether the standby may take over: unless a bug is planted, only once the lease says it
    /// holds everything committed.
    pub fn may_fail_over(&self) -> bool {
        self.unsynced_failover || !self.lease.out_of_sync.load(Ordering::Acquire)
    }

    /// The standby takes over: its copy becomes the active's, at the next epoch (or the one taken
    /// when the active was deposed), and the old active's copy the standby's. The acceptor's
    /// process must restart over it, as after a crash.
    pub fn fail_over(&mut self) {
        if !self.deposed {
            self.epoch += 1;
        }
        self.deposed = false;
        self.active = 1 - self.active;
        self.replicate();
    }

    /// The active's copy, replicating to the standby's at the epoch, through whatever the seed's
    /// planted bugs and outages put between them.
    fn replicate(&mut self) {
        let (active, epoch) = (self.active, self.epoch);
        self.lease.epoch.store(epoch, Ordering::Release);
        self.standby = Arc::new(Mutex::new(Replica::new(self.stores[1 - active].clone())));
        let link = LocalLink::connect(self.standby.clone(), epoch, &format!("instance {active}"))
            .expect("a new epoch connects");
        let link: Arc<dyn Link> = match &self.planted {
            Some(sent) => Arc::new(Dropping { link, sent: sent.clone() }),
            None => Arc::new(link),
        };
        let link = Arc::new(Outage { link, down: self.down.clone() });
        let mut current =
            ReplicatedStorage::new(self.stores[active].clone(), link, epoch).with_lease(self.lease.clone());
        if self.degrade {
            current = current.degrading();
        }
        self.current = Arc::new(current);
    }

    /// The standby can't be reached, until [`standby_up`](Self::standby_up).
    pub fn standby_down(&self) {
        self.down.store(true, Ordering::Release);
    }

    pub fn standby_up(&self) {
        self.down.store(false, Ordering::Release);
    }

    /// One pass of catching the standby up, as the active's task runs it every few seconds.
    pub fn catch_up(&self) -> io::Result<()> {
        self.current.catch_up().run()
    }

    /// Another instance takes the lease, telling the standby the next epoch, while the acceptor
    /// runs on: its commits and openings must now fail.
    pub fn depose(&mut self) {
        if self.deposed {
            return;
        }
        self.deposed = true;
        self.epoch += 1;
        self.lease.epoch.store(self.epoch, Ordering::Release);
        // The instance taking over connects to the standby with the new epoch, which the standby
        // then fences the acceptor's links with.
        drop(LocalLink::connect(self.standby.clone(), self.epoch, "taking over").expect("a new epoch connects"));
    }

    pub fn deposed(&self) -> bool {
        self.deposed
    }

    pub fn describe(&self) -> String {
        format!(
            "instance {} active at epoch {}{}{}",
            self.active,
            self.epoch,
            if self.deposed { ", deposed" } else { "" },
            if self.lease.out_of_sync.load(Ordering::Acquire) { ", standby out of sync" } else { "" }
        )
    }
}

/// The lease the instances share: never ending, remembering what its holder said of the standby
/// being in sync, which only the holder under the current epoch may say, as `SqlLease` fences it.
/// A new holder doesn't change it: a standby out of sync stays so until caught up.
#[derive(Default)]
struct SimLease {
    out_of_sync: AtomicBool,
    /// The epoch of the current holder.
    epoch: AtomicU64,
}

impl Lease for SimLease {
    fn duration(&self) -> Duration {
        Duration::MAX
    }

    fn acquire(&self) -> Job<u64> {
        Job::blocking(|| Ok(0))
    }

    fn renew(&self, _epoch: u64) -> Job<()> {
        Job::blocking(|| Ok(()))
    }

    fn release(&self, _epoch: u64) -> Job<()> {
        Job::blocking(|| Ok(()))
    }

    fn set_in_sync(&self, epoch: u64, in_sync: bool) -> Job<()> {
        if epoch != self.epoch.load(Ordering::Acquire) {
            return Job::blocking(|| Err(io::Error::new(io::ErrorKind::PermissionDenied, "the lease was taken")));
        }
        self.out_of_sync.store(!in_sync, Ordering::Release);
        Job::blocking(|| Ok(()))
    }

    fn in_sync(&self) -> Job<bool> {
        let in_sync = !self.out_of_sync.load(Ordering::Acquire);
        Job::blocking(move || Ok(in_sync))
    }
}

/// A link whose standby can be down.
struct Outage {
    link: Arc<dyn Link>,
    down: Arc<AtomicBool>,
}

impl Link for Outage {
    fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary> {
        if self.down.load(Ordering::Acquire) {
            return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "the standby is down"));
        }
        self.link.bind(handle, epoch, max_lag, session)
    }

    fn send(&self, batch: Batch) -> Commit {
        if self.down.load(Ordering::Acquire) {
            return Commit::blocking(|| Err(io::Error::new(io::ErrorKind::ConnectionRefused, "the standby is down")));
        }
        self.link.send(batch)
    }
}

/// The link of a pair not yet replicating.
struct Unbound;

impl Link for Unbound {
    fn bind(&self, _handle: u32, _epoch: u64, _max_lag: u64, _session: &SessionId) -> io::Result<Summary> {
        Err(io::Error::other("unbound"))
    }

    fn send(&self, _batch: Batch) -> Commit {
        Commit::blocking(|| Err(io::Error::other("unbound")))
    }
}

/// The planted bug: a link that, from the 10th batch, tells the active each was committed without
/// sending it.
struct Dropping {
    link: LocalLink<Store>,
    sent: Arc<AtomicU64>,
}

impl Link for Dropping {
    fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary> {
        self.link.bind(handle, epoch, max_lag, session)
    }

    fn send(&self, batch: Batch) -> Commit {
        if self.sent.fetch_add(1, Ordering::Relaxed) >= 9 {
            return Commit::blocking(|| Ok(()));
        }
        self.link.send(batch)
    }
}

/// The acceptor's store: whichever instance's copy is active.
pub struct Switching(pub Arc<Mutex<Pair>>);

impl Switching {
    fn current(&self) -> Arc<dyn SessionStorage> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).current.clone()
    }
}

impl SessionStorage for Switching {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
        self.current().open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.current().begin_open(id)
    }

    fn is_durable(&self) -> bool {
        self.current().is_durable()
    }

    fn marked(&self) -> Job<Vec<(SessionId, Marks)>> {
        self.current().marked()
    }

    fn sessions(&self) -> Job<Vec<SessionId>> {
        self.current().sessions()
    }
}
