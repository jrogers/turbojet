//! SqlLease, the lease on a whole instance that replication's active holds: on SQLite always,
//! and on PostgreSQL when `TURBOJET_POSTGRES_URL` names a database the tests may use.
#![cfg(all(feature = "replica", any(feature = "sqlite", feature = "postgres")))]

use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use turbojet_replica::Lease;
use turbojet_sql::{SqlConfig, SqlStorage};

async fn store(url: &str, holder: &str, lease: Duration) -> SqlStorage {
    let mut config = SqlConfig::default();
    config.holder = holder.into();
    config.lease = lease;
    let storage = SqlStorage::connect(url, config).await.unwrap();
    storage.migrate().await.unwrap();
    storage
}

/// A lease name no earlier run has used, so a shared database needs no clearing.
fn fresh_name() -> String {
    format!("gateway-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())
}

/// One instance at a time holds the lease; each taking it raises the epoch; the instance it was
/// taken from can't renew it.
async fn one_holder_at_a_time(url: &str) {
    let name = fresh_name();
    let a = store(url, "gw-a", Duration::from_secs(60)).await.lease(&name);
    let b = store(url, "gw-b", Duration::from_secs(60)).await.lease(&name);
    let first = a.acquire().run_async().await.unwrap();
    let busy = b.acquire().run_async().await.unwrap_err();
    assert_eq!(busy.kind(), io::ErrorKind::ResourceBusy, "{busy}");
    a.renew(first).run_async().await.unwrap();

    a.release(first).run_async().await.unwrap();
    let second = b.acquire().run_async().await.unwrap();
    assert!(second > first, "a new holder, a higher epoch: {first} then {second}");
    let fenced = a.renew(first).run_async().await.unwrap_err();
    assert_eq!(fenced.kind(), io::ErrorKind::PermissionDenied, "{fenced}");
    a.release(first).run_async().await.unwrap();
    b.renew(second).run_async().await.expect("a stale release changes nothing");
}

/// The holder records whether its standby is in sync, for whoever takes the lease over to see;
/// an instance it was taken from can't. Only the holder reads it, so an instance taking over
/// reads it once it holds the lease, when no one else can change it.
async fn in_sync_is_the_holders_to_say(url: &str) {
    let name = fresh_name();
    let a = store(url, "gw-a", Duration::from_secs(60)).await.lease(&name);
    let b = store(url, "gw-b", Duration::from_secs(60)).await.lease(&name);
    let first = a.acquire().run_async().await.unwrap();
    assert!(a.in_sync(first).run_async().await.unwrap(), "a new lease is in sync");
    let unheld = b.in_sync(first).run_async().await.unwrap_err();
    assert_eq!(unheld.kind(), io::ErrorKind::PermissionDenied, "only the holder reads");
    let refused = b.set_in_sync(first, true).run_async().await.unwrap_err();
    assert_eq!(refused.kind(), io::ErrorKind::PermissionDenied, "only the holder says");
    a.set_in_sync(first, false).run_async().await.unwrap();
    assert!(!a.in_sync(first).run_async().await.unwrap());
    a.release(first).run_async().await.unwrap();

    let second = b.acquire().run_async().await.unwrap();
    assert!(!b.in_sync(second).run_async().await.unwrap(), "a new holder sees what the last said");
    let stale = a.in_sync(first).run_async().await.unwrap_err();
    assert_eq!(stale.kind(), io::ErrorKind::PermissionDenied, "a holder replaced can't read it");
    assert_eq!(a.set_in_sync(first, true).run_async().await.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    b.set_in_sync(second, true).run_async().await.unwrap();
    assert!(b.in_sync(second).run_async().await.unwrap());
}

/// A lease that has run out is taken over without being given up.
async fn an_expired_lease_is_taken_over(url: &str) {
    let name = fresh_name();
    let a = store(url, "gw-a", Duration::from_millis(200)).await.lease(&name);
    let b = store(url, "gw-b", Duration::from_millis(200)).await.lease(&name);
    let first = a.acquire().run_async().await.unwrap();
    tokio::time::sleep(Duration::from_millis(300)).await;
    let second = b.acquire().run_async().await.expect("expired");
    assert!(second > first);
    assert_eq!(a.renew(first).run_async().await.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
}

#[cfg(feature = "sqlite")]
#[tokio::test(flavor = "multi_thread")]
async fn sqlite() {
    let dir = tempfile::tempdir().unwrap();
    let url = format!("sqlite://{}?mode=rwc", dir.path().join("leases.db").display());
    one_holder_at_a_time(&url).await;
    an_expired_lease_is_taken_over(&url).await;
    in_sync_is_the_holders_to_say(&url).await;
}

#[cfg(feature = "postgres")]
#[tokio::test(flavor = "multi_thread")]
async fn postgres() {
    let Some(url) = std::env::var("TURBOJET_POSTGRES_URL").ok().filter(|url| !url.is_empty()) else { return };
    one_holder_at_a_time(&url).await;
    an_expired_lease_is_taken_over(&url).await;
    in_sync_is_the_holders_to_say(&url).await;
}
