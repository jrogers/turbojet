//! A replicated acceptor store, for the seeds that have one: an active instance's store copying
//! each commit to a standby's, and the failovers between them.
//!
//! Both instances keep their copy in a `MemoryStorage`, standing, as for the unreplicated seeds,
//! for files that survive a process crash. The acceptor node is whichever instance is active: a
//! failover crashes it and restarts it over the standby's copy, at the next epoch, and the old
//! active's copy becomes the new standby's, sent whatever it lacks as each session opens.

use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use turbojet::store::{Commit, Job, Marks, Opened, SessionStorage, StoredSession};
use turbojet::{MemoryStorage, SessionId};
use turbojet_replica::frame::{Batch, Summary};
use turbojet_replica::{Link, LocalLink, Replica, ReplicatedStorage};

type Store = Arc<MemoryStorage>;

/// The two instances' copies and which is active.
pub struct Pair {
    stores: [Store; 2],
    active: usize,
    /// The epoch of the active's lease, or after [`depose`](Self::depose) of the instance that
    /// took it.
    epoch: u64,
    /// The store the acceptor uses: the active's copy, replicating to the standby's.
    current: Arc<dyn SessionStorage>,
    /// The standby the active replicates to, which holds the epoch it fences with.
    standby: Arc<Mutex<Replica<Store>>>,
    /// Another instance has taken the lease: the acceptor runs on as a deposed active until the
    /// failover that follows.
    deposed: bool,
    /// With the planted bug of a stale standby, the batches sent so far.
    planted: Option<Arc<AtomicU64>>,
}

impl Pair {
    /// A pair whose standby, if `stale`, stops getting batches from the 10th on though told
    /// they're committed (the simulator's planted bug).
    pub fn new(stale: bool) -> Arc<Mutex<Self>> {
        let stores = [Arc::new(MemoryStorage::new()), Arc::new(MemoryStorage::new())];
        let planted = stale.then(Arc::default);
        let (current, standby) = replicating(&stores, 0, 1, planted.as_ref());
        Arc::new(Mutex::new(Self { stores, active: 0, epoch: 1, current, standby, deposed: false, planted }))
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
        (self.current, self.standby) = replicating(&self.stores, self.active, self.epoch, self.planted.as_ref());
    }

    /// Another instance takes the lease, telling the standby the next epoch, while the acceptor
    /// runs on: its commits and openings must now fail.
    pub fn depose(&mut self) {
        if self.deposed {
            return;
        }
        self.deposed = true;
        self.epoch += 1;
        // The instance taking over connects to the standby with the new epoch, which the standby
        // then fences the acceptor's links with.
        drop(LocalLink::connect(self.standby.clone(), self.epoch, "taking over").expect("a new epoch connects"));
    }

    pub fn deposed(&self) -> bool {
        self.deposed
    }

    pub fn describe(&self) -> String {
        format!(
            "instance {} active at epoch {}{}",
            self.active,
            self.epoch,
            if self.deposed { ", deposed" } else { "" }
        )
    }
}

/// The active's copy at `active`, replicating at `epoch` to a standby over the other's; through
/// a link that drops batches from the 10th on, counting them in `planted`, if given.
fn replicating(
    stores: &[Store; 2],
    active: usize,
    epoch: u64,
    planted: Option<&Arc<AtomicU64>>,
) -> (Arc<dyn SessionStorage>, Arc<Mutex<Replica<Store>>>) {
    let standby = Arc::new(Mutex::new(Replica::new(stores[1 - active].clone())));
    let link = LocalLink::connect(standby.clone(), epoch, &format!("instance {active}")).expect("a new epoch connects");
    let link: Arc<dyn Link> = match planted {
        Some(sent) => Arc::new(Dropping { link, sent: sent.clone() }),
        None => Arc::new(link),
    };
    (Arc::new(ReplicatedStorage::new(stores[active].clone(), link, epoch)), standby)
}

/// The planted bug: a link that, from the 10th batch, tells the active each was committed without
/// sending it.
struct Dropping {
    link: LocalLink<Store>,
    sent: Arc<AtomicU64>,
}

impl Link for Dropping {
    fn bind(&self, handle: u32, epoch: u64, session: &SessionId) -> io::Result<Summary> {
        self.link.bind(handle, epoch, session)
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
