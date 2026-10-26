//! Which instance is active: a lease, held under an epoch that grows each time it changes hands.

use std::io;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use tracing::warn;
use turbojet::store::Job;

/// The lease that makes one instance the active, fencing off the rest. Each holder takes it under
/// a higher epoch than any before, which its batches carry, so a standby refuses a deposed
/// active's; and the active renews it as it commits, so one that can't stops committing even if
/// it can't reach the standby to be refused.
pub trait Lease: Send + Sync {
    /// How long the lease lasts once taken or renewed.
    fn duration(&self) -> Duration;

    /// Takes the lease for this instance, if it's free, expired or already this instance's,
    /// returning the epoch it now holds it under: higher than any before.
    fn acquire(&self) -> Job<u64>;

    /// Renews the lease held under `epoch`.
    ///
    /// # Errors
    ///
    /// [`io::ErrorKind::PermissionDenied`] if another instance has taken it; any other error if
    /// it couldn't be renewed this time (its store can't be reached).
    fn renew(&self, epoch: u64) -> Job<()>;

    /// Gives up the lease held under `epoch`, so another instance can take it at once.
    fn release(&self, epoch: u64) -> Job<()>;

    /// Records, as the holder under `epoch`, whether the standby holds everything committed:
    /// false before the first commit it won't have, true once it has caught up again.
    ///
    /// # Errors
    ///
    /// [`io::ErrorKind::PermissionDenied`] if another instance has taken the lease; any other
    /// error if it couldn't be recorded. The active then fails the commit, rather than make one
    /// the standby lacks without saying so.
    fn set_in_sync(&self, epoch: u64, in_sync: bool) -> Job<()>;

    /// Whether the standby was in sync when the lease's holder last said: an instance taking
    /// the lease over checks it first, since a standby out of sync lacks commits.
    fn in_sync(&self) -> Job<bool>;
}

/// A lease an operator decides: [`acquire`](Lease::acquire) takes the epoch given, which the
/// operator raises at each failover; renewing always succeeds, so fencing is left to the
/// standby, which refuses epochs below one it has seen. It records nothing of the standby being
/// in sync, which is the operator's to know: [`in_sync`](Lease::in_sync) is always true.
#[derive(Debug, Clone, Copy)]
pub struct ManualLease {
    epoch: u64,
}

impl ManualLease {
    /// The lease at `epoch`, which must be higher than any instance has held before.
    pub fn new(epoch: u64) -> Self {
        Self { epoch }
    }
}

impl Lease for ManualLease {
    fn duration(&self) -> Duration {
        Duration::MAX
    }

    fn acquire(&self) -> Job<u64> {
        let epoch = self.epoch;
        Job::blocking(move || Ok(epoch))
    }

    fn renew(&self, _epoch: u64) -> Job<()> {
        Job::blocking(|| Ok(()))
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

/// An active's hold on its lease: renewed in its commits once half of it has gone.
pub(crate) struct Held {
    lease: Arc<dyn Lease>,
    epoch: u64,
    until: Mutex<Instant>,
}

impl Held {
    /// The lease, just taken under `epoch`.
    pub(crate) fn new(lease: Arc<dyn Lease>, epoch: u64) -> Arc<Self> {
        let until = Instant::now().checked_add(lease.duration()).unwrap_or_else(far_future);
        Arc::new(Self { lease, epoch, until: Mutex::new(until) })
    }

    pub(crate) fn release(&self) -> Job<()> {
        self.lease.release(self.epoch)
    }

    pub(crate) fn set_in_sync(&self, in_sync: bool) -> Job<()> {
        self.lease.set_in_sync(self.epoch, in_sync)
    }

    pub(crate) fn in_sync(&self) -> Job<bool> {
        self.lease.in_sync()
    }

    /// A renewal, if the lease is half gone: it fails only if the lease was taken, or has run
    /// out with no renewal since. Another failure is borne, the lease being good a while yet.
    pub(crate) fn renewal(self: &Arc<Self>) -> Option<Job<()>> {
        if self.lease.duration() == Duration::MAX {
            return None;
        }
        let until = *self.until.lock().unwrap_or_else(PoisonError::into_inner);
        let half = self.lease.duration() / 2;
        if until.checked_duration_since(Instant::now()).is_some_and(|left| left > half) {
            return None;
        }
        let held = self.clone();
        let started = Instant::now();
        Some(self.lease.renew(self.epoch).map_result(move |renewed| match renewed {
            Ok(()) => {
                let until = started.checked_add(held.lease.duration()).unwrap_or_else(far_future);
                *held.until.lock().unwrap_or_else(PoisonError::into_inner) = until;
                Ok(())
            }
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Err(e),
            Err(e) => {
                let until = *held.until.lock().unwrap_or_else(PoisonError::into_inner);
                if Instant::now() >= until {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        format!("this instance's lease (epoch {}) ran out unrenewed: {e}", held.epoch),
                    ));
                }
                warn!(error = %e, epoch = held.epoch, "couldn't renew the lease; it holds a while yet");
                Ok(())
            }
        }))
    }
}

/// A deadline that never comes, for a lease that never ends.
fn far_future() -> Instant {
    Instant::now() + Duration::from_secs(100 * 365 * 24 * 3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A lease whose renewals go as the test says.
    struct Scripted {
        duration: Duration,
        next: Mutex<Vec<io::Result<()>>>,
    }

    impl Lease for Scripted {
        fn duration(&self) -> Duration {
            self.duration
        }

        fn acquire(&self) -> Job<u64> {
            Job::blocking(|| Ok(1))
        }

        fn renew(&self, _epoch: u64) -> Job<()> {
            let result = self.next.lock().unwrap().remove(0);
            Job::blocking(move || result)
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

    fn scripted(duration: Duration, next: Vec<io::Result<()>>) -> Arc<Held> {
        Held::new(Arc::new(Scripted { duration, next: Mutex::new(next) }), 1)
    }

    #[test]
    fn a_fresh_lease_isnt_renewed() {
        assert!(scripted(Duration::from_secs(60), vec![]).renewal().is_none());
        assert!(Held::new(Arc::new(ManualLease::new(3)), 3).renewal().is_none(), "a manual lease never ends");
    }

    #[test]
    fn a_lease_taken_fails_the_commit() {
        let taken = io::Error::new(io::ErrorKind::PermissionDenied, "taken");
        let held = scripted(Duration::ZERO, vec![Ok(()), Err(taken)]);
        held.renewal().expect("due").run().unwrap();
        assert_eq!(held.renewal().expect("due").run().unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    }

    /// A store that can't be reached is borne while the lease lasts, and not once it has run out.
    #[test]
    fn a_failed_renewal_counts_only_once_the_lease_has_run_out() {
        let unreachable = || io::Error::new(io::ErrorKind::TimedOut, "database down");
        let lasting = scripted(Duration::from_secs(60), vec![Err(unreachable())]);
        *lasting.until.lock().unwrap() = Instant::now() + Duration::from_secs(10);
        lasting.renewal().expect("over half gone").run().unwrap();

        let run_out = scripted(Duration::ZERO, vec![Err(unreachable())]);
        let failed = run_out.renewal().expect("due").run().unwrap_err();
        assert_eq!(failed.kind(), io::ErrorKind::PermissionDenied, "{failed}");
    }
}
