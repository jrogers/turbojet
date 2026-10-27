//! Replicated session state for [Turbojet](turbojet): an active instance copies each FIX
//! session's committed state to a hot standby, which can take the sessions over with no sequence
//! gap and no resend storm.
//!
//! [`ReplicatedStorage`] wraps the active's store: each session's commit is made there and copied
//! over a [`Link`] to a [`Replica`], which applies it to the standby's own store and acknowledges
//! it, and the commit, so everything it covers going out, waits for both, unless
//! [`asynchronous`](ReplicatedStorage::asynchronous). The standby serves
//! actives over TCP with [`serve_standby`], or TLS ([below](#tls)); the active connects with a
//! [`TcpLink`]. [`frame`] is
//! the wire form between the two.
//!
//! ```no_run
//! use std::sync::{Arc, Mutex};
//! use tokio::net::TcpListener;
//! use turbojet::{DiskStorage, SessionRegistry};
//! use turbojet_replica::{Lease, ManualLease, Replica, ReplicatedStorage, TcpLink, serve_standby};
//!
//! /// The standby's instance: keeps a copy of the active's sessions, until it takes over.
//! async fn standby() -> std::io::Result<()> {
//!     let store = Arc::new(DiskStorage::new("./store")?);
//!     let replica = Arc::new(Mutex::new(Replica::new(store)));
//!     serve_standby(TcpListener::bind("0.0.0.0:9800").await?, replica).await
//! }
//!
//! /// The active's instance: its sessions' store, copying each commit to the standby.
//! async fn active() -> std::io::Result<SessionRegistry> {
//!     let lease = Arc::new(ManualLease::new(1));
//!     let epoch = lease.acquire().run_async().await?;
//!     let link = TcpLink::new("standby.internal:9800", epoch, "gateway-a");
//!     let storage = ReplicatedStorage::new(DiskStorage::new("./store")?, Arc::new(link), epoch).with_lease(lease);
//!     Ok(SessionRegistry::new(Arc::new(storage)))
//! }
//! ```
//!
//! # Which instance is active
//!
//! A [`Lease`] decides: the active takes it under an epoch, raised each time it changes hands,
//! which every batch carries. A standby refuses batches, and bindings, under an epoch below one it
//! has seen, so an active that has been replaced can't commit while the standby can reach it; and
//! the active renews the lease in its commits once half of it has gone, failing them once another
//! instance has taken it or it has run out, so it stops even when the standby can't be reached.
//! `turbojet-sql`'s `SqlLease` (feature `replica`) keeps the lease in a database;
//! [`ManualLease`] leaves the epoch to an operator, and fencing to the standby.
//!
//! To fail over, the standby's instance takes the lease, under a higher epoch, calls
//! [`Replica::promote`], which closes the sessions the standby holds and refuses the old active
//! from then on, and starts its own registry over its store, replicating in turn to the old
//! active's instance once that's back.
//!
//! # Catching up
//!
//! When a session opens, the active compares the standby's copy of it with its own, and sends the
//! session whole if they differ (in batches of a megabyte of messages at most): a standby that
//! missed commits, or joins late, catches up. A reset is sent at once, as the built-in stores
//! write it at once, so a failover never finds one copy reset and the other not.
//!
//! By default a commit the standby can't take, because it can't be reached in time
//! ([`DEFAULT_ANSWER_TIMEOUT`]) or has seen a higher epoch, fails, as any store failure does: the
//! session disconnects, and when it opens again the standby is caught up. The active's
//! availability is then tied to the standby's.
//!
//! Made [`degrading`](ReplicatedStorage::degrading), the store keeps its sessions up through a
//! standby outage: before the first commit the standby won't have, it tells the lease the
//! standby is out of sync ([`Lease::set_in_sync`]), then commits alone. When the standby is back,
//! each open session sends itself whole at its next commit, and
//! [`catch_up`](ReplicatedStorage::catch_up), run every few seconds, sends the rest and tells the
//! lease once the standby is in sync. An instance about to take over checks
//! [`Lease::in_sync`] first. Degrading needs a lease that fences a replaced active, which the
//! standby can't while it's unreachable: [`ManualLease`] can't, so with one a commit the standby
//! can't take still fails.
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use turbojet_replica::ReplicatedStorage;
//! # fn example(storage: Arc<ReplicatedStorage<turbojet::DiskStorage>>) {
//! // Catch the standby up after an outage, every few seconds.
//! tokio::spawn(async move {
//!     loop {
//!         tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//!         if let Err(e) = storage.catch_up().run_async().await {
//!             tracing::warn!(error = %e, "catching the standby up");
//!         }
//!     }
//! });
//! # }
//! ```
//!
//! # Asynchronously
//!
//! Made [`asynchronous`](ReplicatedStorage::asynchronous), a session's commits don't wait for the
//! standby until it has more than a bound of outgoing sequence numbers unacknowledged, or 64
//! batches, for commits that carry only incoming state; a later commit waits for the answers
//! instead, so what it covers goes out at the active's own store's pace. A batch with more numbers
//! than the bound waits for itself, and a reset for the batches queued before it. The standby's
//! bound is the one that counts: each binding carries the active's, and a standby made
//! [`accepting_lag`](Replica::accepting_lag) less refuses it, failing the opening with
//! [`InvalidInput`](std::io::ErrorKind::InvalidInput).
//!
//! ```no_run
//! # use std::sync::{Arc, Mutex};
//! # use turbojet::DiskStorage;
//! # use turbojet_replica::{Replica, ReplicatedStorage, TcpLink};
//! # fn example() -> std::io::Result<()> {
//! // On the standby: accept actives up to 8 outgoing numbers per session behind.
//! let replica = Arc::new(Mutex::new(Replica::new(DiskStorage::new("./store")?).accepting_lag(8)));
//! // On the active: leave up to 8 unacknowledged.
//! let link = TcpLink::new("standby.internal:9800", 1, "gateway-a");
//! let storage = ReplicatedStorage::new(DiskStorage::new("./store")?, Arc::new(link), 1).asynchronous(8);
//! // Taking over, the standby skips each session's next outgoing number ahead by 8.
//! replica.lock().unwrap().promote()?;
//! # drop(storage);
//! # Ok(())
//! # }
//! ```
//!
//! What it trades is what a failover keeps. The old active may have sent up to the bound of a
//! session's outgoing numbers the standby lacks, so [`Replica::promote`] skips each session's next
//! outgoing number ahead by the lag it accepts: sent again, a number would reach the counterparty
//! too low, which ends the session; skipped too high, the counterparty asks for a resend and is
//! sent a gap fill. The messages in those numbers are lost to the new active, which the
//! application must reconcile (an order's status, say). An unacknowledged batch carries incoming
//! state too, so the new active may expect a lower incoming number than the old one had reached:
//! the counterparty resends, and the application is handed again, marked as possible duplicates,
//! messages the old active had already handed it. Promoting needs a store that lists its sessions
//! ([`SessionStorage::sessions`](turbojet::SessionStorage::sessions); the built-in ones do).
//!
//! A batch the standby fails to take fails the later commit of the session that waits for it,
//! with the session disconnecting and catching the standby up when it opens again, as
//! [above](#catching-up); made [`degrading`](ReplicatedStorage::degrading), that commit puts the
//! standby out of sync in the lease instead. [`in_sync`](ReplicatedStorage::in_sync) then means
//! within the bound, not holding every commit.
//!
//! # TLS
//!
//! With the feature `tls`, the standby serves with [`serve_standby_tls`] and the active connects
//! with [`TcpLink::with_tls`], each taking `turbojet::tls`'s configuration as an acceptor and an
//! initiator do. Have the standby require a client certificate
//! (`turbojet::tls::ClientTrust::Required`), so only an instance holding one can replicate to it.
//! Without TLS the connection is plain TCP: run it on a private network, or through a tunnel.

mod active;
pub mod frame;
mod lease;
mod net;
mod standby;
mod sync;

pub use active::{Link, LocalLink, ReplicatedStorage};
pub use lease::{Lease, ManualLease};
#[cfg(feature = "tls")]
pub use net::serve_standby_tls;
pub use net::{DEFAULT_ANSWER_TIMEOUT, TcpLink, serve_standby};
pub use standby::Replica;
