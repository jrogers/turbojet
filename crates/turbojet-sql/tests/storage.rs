//! SqlStorage against the store conformance suite, and its leases, eviction and recovery: on
//! SQLite always, and on PostgreSQL when `TURBOJET_POSTGRES_URL` names a database the tests may
//! clear (`scripts/check.sh --postgres` sets it).
#![cfg(any(feature = "sqlite", feature = "postgres"))]

use std::io;
use std::time::Duration;

use turbojet::store::conformance::{self, app_message, commit, fetch, id, open};
use turbojet_sql::{SqlConfig, SqlStorage};

/// A database for one test: a SQLite file in a directory of its own, or the PostgreSQL database
/// cleared.
struct Database {
    url: String,
    _dir: Option<tempfile::TempDir>,
}

impl Database {
    #[cfg(feature = "sqlite")]
    fn sqlite() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let url = format!("sqlite://{}?mode=rwc", dir.path().join("sessions.db").display());
        Self { url, _dir: Some(dir) }
    }

    /// The PostgreSQL database, if one is configured: the tests on it run one at a time, in one
    /// test, each clearing it first.
    #[cfg(feature = "postgres")]
    fn postgres() -> Option<Self> {
        let url = std::env::var("TURBOJET_POSTGRES_URL").ok().filter(|url| !url.is_empty())?;
        Some(Self { url, _dir: None })
    }

    /// A store over the database, as one gateway, `holder`.
    async fn store(&self, holder: &str, adjust: impl FnOnce(&mut SqlConfig)) -> SqlStorage {
        let mut config = SqlConfig { holder: holder.into(), ..SqlConfig::default() };
        adjust(&mut config);
        let storage = SqlStorage::connect(&self.url, config).await.unwrap();
        storage.migrate().await.unwrap();
        storage
    }

    #[cfg(feature = "postgres")]
    async fn clear(&self) {
        // The tables exist once a store has migrated.
        self.store("clearing", |_| {}).await;
        sqlx::any::install_default_drivers();
        let pool = sqlx::AnyPool::connect(&self.url).await.unwrap();
        sqlx::query("DELETE FROM turbojet_messages").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM turbojet_sessions").execute(&pool).await.unwrap();
    }
}

/// Every check, on `database`, clearing it before each.
#[cfg(feature = "postgres")]
async fn all(database: &Database) {
    for check in CHECKS {
        database.clear().await;
        check(database).await;
    }
}

type Check = for<'a> fn(&'a Database) -> std::pin::Pin<Box<dyn Future<Output = ()> + 'a>>;

const CHECKS: [Check; 6] = [
    |d| Box::pin(conforms(d)),
    |d| Box::pin(a_held_lease_refuses_another_gateway(d)),
    |d| Box::pin(an_expired_lease_passes_and_fences_the_old_holder(d)),
    |d| Box::pin(closing_a_log_gives_its_lease_up(d)),
    |d| Box::pin(what_was_not_committed_is_lost(d)),
    |d| Box::pin(the_oldest_messages_go_past_the_budget(d)),
];

async fn conforms(database: &Database) {
    conformance::check(&database.store("A", |_| {}).await).await;
}

/// While one gateway holds a session's lease, another can't open it; the same gateway can
/// (its registry keeps sessions apart).
async fn a_held_lease_refuses_another_gateway(database: &Database) {
    let first = database.store("gateway-1", |_| {}).await;
    let second = database.store("gateway-2", |_| {}).await;
    let _held = open(&first, &id("A")).await.unwrap();
    let refused = open(&second, &id("A")).await.err().expect("leased to the first");
    assert_eq!(refused.kind(), io::ErrorKind::ResourceBusy);
    assert!(refused.to_string().contains("gateway-1"), "names the holder: {refused}");
    assert!(open(&second, &id("B")).await.is_ok(), "other sessions are free");
    assert!(open(&first, &id("A")).await.is_ok(), "the holder may open it again");
}

/// Once a lease expires, another gateway takes the session, and the first can no longer commit
/// to it.
async fn an_expired_lease_passes_and_fences_the_old_holder(database: &Database) {
    let short = |config: &mut SqlConfig| config.lease = Duration::from_millis(300);
    let first = database.store("gateway-1", short).await;
    let second = database.store("gateway-2", short).await;
    let mut stale = open(&first, &id("A")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(400)).await;
    let mut taken = open(&second, &id("A")).await.expect("the lease has expired");
    taken.record_outgoing(1, Some(&app_message(1))).unwrap();
    commit(taken.as_mut()).await.unwrap();

    stale.record_outgoing(1, Some(&app_message(9))).unwrap();
    let fenced = commit(stale.as_mut()).await.expect_err("the lease has been taken");
    assert_eq!(fenced.kind(), io::ErrorKind::PermissionDenied);
    let sent = fetch(taken.as_mut(), 1, 1).await.unwrap();
    assert_eq!(sent[0].1, app_message(1), "the old holder wrote nothing");
}

/// Closing a log gives up its lease at once, rather than leaving it to expire.
async fn closing_a_log_gives_its_lease_up(database: &Database) {
    let first = database.store("gateway-1", |_| {}).await;
    let second = database.store("gateway-2", |_| {}).await;
    drop(open(&first, &id("A")).await.unwrap());
    for _ in 0..100 {
        if open(&second, &id("A")).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("the lease was never given up");
}

/// A log closed without committing (a crash) loses its changes since the last commit, and
/// nothing of them was sent.
async fn what_was_not_committed_is_lost(database: &Database) {
    let storage = database.store("A", |_| {}).await;
    let mut log = open(&storage, &id("A")).await.unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    log.set_next_incoming(3).unwrap();
    commit(log.as_mut()).await.unwrap();
    log.record_outgoing(2, Some(&app_message(2))).unwrap();
    log.set_next_incoming(4).unwrap();
    drop(log);
    let mut log = open(&storage, &id("A")).await.unwrap();
    assert_eq!((log.next_outgoing(), log.next_incoming()), (2, 3));
    assert_eq!(fetch(log.as_mut(), 1, u64::MAX).await.unwrap().len(), 1);
}

/// Past its byte budget a session's oldest messages go, down to 7/8 of it, and stay gone.
async fn the_oldest_messages_go_past_the_budget(database: &Database) {
    let size = u64::try_from(app_message(10).len()).unwrap();
    let storage = database.store("A", |config| config.max_session_bytes = 8 * size).await;
    let mut log = open(&storage, &id("A")).await.unwrap();
    for seq in 10..19 {
        log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
        commit(log.as_mut()).await.unwrap();
    }
    // The ninth went past 8: down to 7 (all are about the same size).
    let evicted = log.evicted_through().expect("evicted");
    let kept: Vec<u64> = fetch(log.as_mut(), 1, u64::MAX).await.unwrap().iter().map(|(seq, _)| *seq).collect();
    assert_eq!(kept.first(), Some(&(evicted + 1)));
    assert_eq!(kept.last(), Some(&18));
    assert!(kept.len() <= 7, "{kept:?}");
    drop(log);
    let log = open(&storage, &id("A")).await.unwrap();
    assert_eq!(log.evicted_through(), Some(evicted), "survives reopening");
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn sqlite() {
    for check in CHECKS {
        check(&Database::sqlite()).await;
    }
}

#[cfg(feature = "postgres")]
#[tokio::test]
async fn postgres() {
    let Some(database) = Database::postgres() else {
        eprintln!("TURBOJET_POSTGRES_URL is unset; skipping");
        return;
    };
    all(&database).await;
}
