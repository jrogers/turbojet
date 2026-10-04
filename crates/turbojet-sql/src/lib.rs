//! SQL session storage for [Turbojet](turbojet): [`SqlStorage`] keeps each FIX session's sequence
//! numbers and sent messages in SQLite or PostgreSQL, through [sqlx](https://docs.rs/sqlx).
//!
//! It suits deployments that can't rely on local disk, or that want session state next to their
//! other data. Each session's state lives in a row of `turbojet_sessions`, and its stored
//! messages in `turbojet_messages`; [`SqlStorage::migrate`] creates both.
//!
//! ```no_run
//! # async fn example() -> std::io::Result<()> {
//! use std::sync::Arc;
//! use turbojet_sql::{SqlConfig, SqlStorage};
//!
//! let storage = SqlStorage::connect("sqlite://sessions.db?mode=rwc", SqlConfig::default()).await?;
//! storage.migrate().await?;
//! let registry = turbojet::SessionRegistry::new(Arc::new(storage));
//! # Ok(())
//! # }
//! ```
//!
//! # How it stores
//!
//! A session's changes are kept in memory until the session commits them, once per batch of
//! work; each commit is one transaction. Opening a session's log, committing and reading
//! messages for a resend all run off the connection's task, as jobs the connection's driver
//! awaits (see [`turbojet::store::Job`]), so a database round trip never blocks the runtime.
//!
//! Each session keeps at most [`SqlConfig::max_session_bytes`] of messages: past it, the oldest
//! are deleted, and a resend that reaches back to them gap-fills them, as with
//! [`DiskStorage`](turbojet::DiskStorage).
//!
//! # One gateway per session
//!
//! A gateway's registry lets one connection at a time run a session. Across gateways sharing a
//! database, a lease does: opening a session's log takes its lease, for [`SqlConfig::lease`],
//! and each commit renews it. A gateway whose lease has expired and been taken by another can't
//! commit, so its session fails and disconnects rather than send under numbers the other is
//! using. A session's commits come at least once per heartbeat interval while it's connected
//! (each heartbeat is stored), so the lease must be longer than the longest heartbeat interval,
//! with a margin for a slow database. Closing the log gives the lease up; a gateway that stops
//! without closing it leaves it to expire.
//!
//! Leases compare times from each gateway's clock, so gateways sharing a database need clocks
//! in step to well within the lease.
//!
//! # Backends
//!
//! Each is a feature: `sqlite` (the default, bundled) and `postgres`, with `tls` for TLS to
//! PostgreSQL. The URL given to [`SqlStorage::connect`] picks one: `sqlite://path?mode=rwc` or
//! `postgres://user:password@host/database`.

mod log;
mod schema;

use std::io;
use std::sync::Arc;
use std::time::Duration;

use sqlx::AnyPool;
use sqlx::any::AnyPoolOptions;
use turbojet::store::{Job, Opened, SessionId, SessionStorage};

/// How a [`SqlStorage`] keeps sessions.
#[derive(Debug, Clone)]
pub struct SqlConfig {
    /// How long a session's lease lasts from opening its log or its last commit, after which
    /// another gateway may take the session over. Longer than the longest heartbeat interval,
    /// with a margin: see the [crate documentation](crate#one-gateway-per-session).
    pub lease: Duration,
    /// Who holds the leases this store takes, shown to a gateway refused one: by default the
    /// host name, the process ID and the time the store was configured.
    pub holder: String,
    /// The most bytes of messages each session keeps. Past it, the oldest go, down to 7/8 of it,
    /// so that deleting them is rare.
    pub max_session_bytes: u64,
    /// SQLite only: whether each commit waits for the database file to reach the device
    /// (`synchronous = FULL`), so state survives power loss, or only for the OS to have it
    /// (`NORMAL`), so it survives a process crash. PostgreSQL's durability is the server's.
    pub sync: bool,
    /// The most connections the store opens to the database.
    pub max_connections: u32,
}

impl SqlConfig {
    /// The default byte budget for each session's messages, as
    /// [`DiskStorage::DEFAULT_MAX_SESSION_BYTES`](turbojet::DiskStorage::DEFAULT_MAX_SESSION_BYTES).
    pub const DEFAULT_MAX_SESSION_BYTES: u64 = 1 << 30;

    /// The default lease: two minutes, comfortably longer than the usual 30-second heartbeat.
    pub const DEFAULT_LEASE: Duration = Duration::from_secs(120);
}

impl Default for SqlConfig {
    fn default() -> Self {
        Self {
            lease: Self::DEFAULT_LEASE,
            holder: default_holder(),
            max_session_bytes: Self::DEFAULT_MAX_SESSION_BYTES,
            sync: true,
            max_connections: 8,
        }
    }
}

/// Stores sessions in a SQL database: see the [crate documentation](crate).
#[derive(Clone)]
pub struct SqlStorage {
    shared: Arc<Shared>,
}

impl std::fmt::Debug for SqlStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlStorage").finish_non_exhaustive()
    }
}

/// What every log of a store shares.
struct Shared {
    pool: AnyPool,
    config: SqlConfig,
    backend: Backend,
}

/// The database behind a store, whose SQL differs in places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Backend {
    Sqlite,
    Postgres,
}

impl SqlStorage {
    /// Connects to the database at `url`: `sqlite://path` (with `?mode=rwc` to create the file)
    /// or `postgres://…`, whichever this build has the feature for.
    ///
    /// # Errors
    ///
    /// If the URL names a database this build doesn't support, or the database can't be reached.
    pub async fn connect(url: &str, config: SqlConfig) -> io::Result<Self> {
        let backend = Backend::of(url)?;
        sqlx::any::install_default_drivers();
        let sync = config.sync;
        let pool = AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .after_connect(move |connection, _| {
                Box::pin(async move {
                    if backend == Backend::Sqlite {
                        schema::configure_sqlite(connection, sync).await?;
                    }
                    Ok(())
                })
            })
            .connect(url)
            .await
            .map_err(log::database("connecting"))?;
        Ok(Self { shared: Arc::new(Shared { pool, config, backend }) })
    }

    /// Creates the tables if they don't exist yet. Run it once before the first session opens,
    /// or create them yourself from what it runs.
    ///
    /// # Errors
    ///
    /// If the database refuses the statements.
    pub async fn migrate(&self) -> io::Result<()> {
        schema::migrate(&self.shared.pool, self.shared.backend).await
    }
}

impl SessionStorage for SqlStorage {
    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        Ok(Opened::Pending(Job::future(log::open(self.shared.clone(), id.clone()))))
    }
}

impl Backend {
    fn of(url: &str) -> io::Result<Self> {
        let scheme = url.split_once(':').map_or(url, |(scheme, _)| scheme);
        match scheme {
            "sqlite" if cfg!(feature = "sqlite") => Ok(Self::Sqlite),
            "postgres" | "postgresql" if cfg!(feature = "postgres") => Ok(Self::Postgres),
            _ => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("turbojet-sql doesn't support '{scheme}' databases with the features it was built with"),
            )),
        }
    }
}

/// The host, the process and the time: unique among gateways sharing a database, and readable
/// in an error that names who holds a lease.
fn default_holder() -> String {
    let host = std::env::var("HOSTNAME").ok().filter(|h| !h.is_empty()).unwrap_or_else(|| "unknown-host".into());
    let started = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{host}:{}:{started}", std::process::id())
}
