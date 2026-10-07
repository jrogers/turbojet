//! The tables, and the settings each SQLite connection gets.

use std::io;

use sqlx::{AnyConnection, AnyPool, Executor};

use crate::Backend;
use crate::log::database;

/// A session's state, keyed by its BeginString, CompIDs and `extra`, the rest of its ID
/// ([`SessionId::key_suffix`](turbojet::SessionId::key_suffix), empty for most): its sequence numbers, the incoming messages in flight to the application,
/// when it was created or last reset (a FIX UTCTimestamp), the highest sequence number whose
/// message was evicted, the bytes of messages stored, and its lease: who holds it, the token of
/// the log holding it, and until when (milliseconds since the Unix epoch).
const SESSIONS_SQLITE: &str = "CREATE TABLE IF NOT EXISTS turbojet_sessions (
    id INTEGER PRIMARY KEY,
    begin_string TEXT NOT NULL,
    sender_comp_id TEXT NOT NULL,
    target_comp_id TEXT NOT NULL,
    extra TEXT NOT NULL DEFAULT '',
    next_outgoing BIGINT NOT NULL DEFAULT 1,
    next_incoming BIGINT NOT NULL DEFAULT 1,
    in_flight BIGINT,
    created_at TEXT,
    evicted_through BIGINT,
    stored_bytes BIGINT NOT NULL DEFAULT 0,
    lease_holder TEXT,
    lease_token TEXT,
    lease_until BIGINT,
    CONSTRAINT turbojet_sessions_key UNIQUE (begin_string, sender_comp_id, target_comp_id, extra)
)";

const SESSIONS_POSTGRES: &str = "CREATE TABLE IF NOT EXISTS turbojet_sessions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    begin_string TEXT NOT NULL,
    sender_comp_id TEXT NOT NULL,
    target_comp_id TEXT NOT NULL,
    extra TEXT NOT NULL DEFAULT '',
    next_outgoing BIGINT NOT NULL DEFAULT 1,
    next_incoming BIGINT NOT NULL DEFAULT 1,
    in_flight BIGINT,
    created_at TEXT,
    evicted_through BIGINT,
    stored_bytes BIGINT NOT NULL DEFAULT 0,
    lease_holder TEXT,
    lease_token TEXT,
    lease_until BIGINT,
    CONSTRAINT turbojet_sessions_key UNIQUE (begin_string, sender_comp_id, target_comp_id, extra)
)";

/// Sent application messages, as sent, with their sizes for the byte budget. Clustered by
/// session and sequence number, so a resend reads a range in order.
const MESSAGES_SQLITE: &str = "CREATE TABLE IF NOT EXISTS turbojet_messages (
    session BIGINT NOT NULL REFERENCES turbojet_sessions (id),
    seq BIGINT NOT NULL,
    size BIGINT NOT NULL,
    body BLOB NOT NULL,
    PRIMARY KEY (session, seq)
) WITHOUT ROWID";

const MESSAGES_POSTGRES: &str = "CREATE TABLE IF NOT EXISTS turbojet_messages (
    session BIGINT NOT NULL REFERENCES turbojet_sessions (id),
    seq BIGINT NOT NULL,
    size BIGINT NOT NULL,
    body BYTEA NOT NULL,
    PRIMARY KEY (session, seq)
)";

pub async fn migrate(pool: &AnyPool, backend: Backend) -> io::Result<()> {
    let (sessions, messages) = match backend {
        Backend::Sqlite => (SESSIONS_SQLITE, MESSAGES_SQLITE),
        Backend::Postgres => (SESSIONS_POSTGRES, MESSAGES_POSTGRES),
    };
    let mut tx = pool.begin().await.map_err(database("starting the migration"))?;
    tx.execute(sessions).await.map_err(database("creating turbojet_sessions"))?;
    tx.execute(messages).await.map_err(database("creating turbojet_messages"))?;
    tx.commit().await.map_err(database("committing the migration"))?;
    match backend {
        Backend::Sqlite => add_extra_sqlite(pool).await,
        Backend::Postgres => add_extra_postgres(pool).await,
    }
}

/// Whether `turbojet_sessions` has the `extra` column, which tables made before 0.3 lack.
async fn has_extra(connection: &mut AnyConnection, backend: Backend) -> io::Result<bool> {
    let query = match backend {
        Backend::Sqlite => "SELECT COUNT(*) FROM pragma_table_info('turbojet_sessions') WHERE name = 'extra'",
        Backend::Postgres => {
            "SELECT COUNT(*) FROM information_schema.columns
             WHERE table_schema = current_schema() AND table_name = 'turbojet_sessions' AND column_name = 'extra'"
        }
    };
    let count: i64 = sqlx::query_scalar(query)
        .fetch_one(&mut *connection)
        .await
        .map_err(database("looking for turbojet_sessions.extra"))?;
    Ok(count > 0)
}

/// Adds `extra` to a table made before 0.3, keying sessions on it too. SQLite can't change a
/// table's constraints, so the table is rebuilt, as SQLite's documentation describes: with foreign
/// keys off (which can't change inside a transaction), in one transaction that takes the write
/// lock first, so a second gateway migrating at once waits and then finds the column there.
async fn add_extra_sqlite(pool: &AnyPool) -> io::Result<()> {
    let mut connection = pool.acquire().await.map_err(database("starting the migration"))?;
    if has_extra(&mut connection, Backend::Sqlite).await? {
        return Ok(());
    }
    connection.execute("PRAGMA foreign_keys = OFF").await.map_err(database("turning foreign keys off"))?;
    let rebuilt = rebuild_sqlite(&mut connection).await;
    if rebuilt.is_err() {
        let _ = connection.execute("ROLLBACK").await;
    }
    connection.execute("PRAGMA foreign_keys = ON").await.map_err(database("turning foreign keys on"))?;
    rebuilt
}

async fn rebuild_sqlite(connection: &mut AnyConnection) -> io::Result<()> {
    connection.execute("BEGIN IMMEDIATE").await.map_err(database("starting the migration"))?;
    if has_extra(connection, Backend::Sqlite).await? {
        return connection.execute("COMMIT").await.map(drop).map_err(database("committing the migration"));
    }
    let columns = "id, begin_string, sender_comp_id, target_comp_id, next_outgoing, next_incoming, in_flight, \
                   created_at, evicted_through, stored_bytes, lease_holder, lease_token, lease_until";
    let new_table = SESSIONS_SQLITE.replace("IF NOT EXISTS turbojet_sessions", "turbojet_sessions_new");
    for (step, sql) in [
        ("creating the new turbojet_sessions", new_table),
        (
            "copying turbojet_sessions",
            format!("INSERT INTO turbojet_sessions_new ({columns}) SELECT {columns} FROM turbojet_sessions"),
        ),
        ("dropping the old turbojet_sessions", "DROP TABLE turbojet_sessions".into()),
        ("renaming the new turbojet_sessions", "ALTER TABLE turbojet_sessions_new RENAME TO turbojet_sessions".into()),
    ] {
        connection.execute(sql.as_str()).await.map_err(database(step))?;
    }
    let broken: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pragma_foreign_key_check")
        .fetch_one(&mut *connection)
        .await
        .map_err(database("checking foreign keys"))?;
    if broken > 0 {
        return Err(io::Error::other(format!("migrating turbojet_sessions would break {broken} foreign keys")));
    }
    connection.execute("COMMIT").await.map(drop).map_err(database("committing the migration"))
}

/// Adds `extra` to a table made before 0.3 and swaps the unique key for one including it, under
/// a lock that makes a second gateway migrating at once wait and then find the column there.
async fn add_extra_postgres(pool: &AnyPool) -> io::Result<()> {
    let mut tx = pool.begin().await.map_err(database("starting the migration"))?;
    tx.execute("LOCK TABLE turbojet_sessions IN ACCESS EXCLUSIVE MODE")
        .await
        .map_err(database("locking turbojet_sessions"))?;
    if has_extra(&mut tx, Backend::Postgres).await? {
        return tx.commit().await.map_err(database("committing the migration"));
    }
    // The old key's name was chosen by PostgreSQL.
    let old_key: String = sqlx::query_scalar(
        "SELECT conname::text FROM pg_constraint WHERE conrelid = 'turbojet_sessions'::regclass AND contype = 'u'",
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(database("finding the key of turbojet_sessions"))?;
    for (step, sql) in [
        (
            "adding turbojet_sessions.extra",
            "ALTER TABLE turbojet_sessions ADD COLUMN extra TEXT NOT NULL DEFAULT ''".into(),
        ),
        (
            "dropping the old key",
            format!("ALTER TABLE turbojet_sessions DROP CONSTRAINT \"{}\"", old_key.replace('"', "\"\"")),
        ),
        (
            "adding the new key",
            "ALTER TABLE turbojet_sessions ADD CONSTRAINT turbojet_sessions_key \
             UNIQUE (begin_string, sender_comp_id, target_comp_id, extra)"
                .into(),
        ),
    ] {
        tx.execute(sql.as_str()).await.map_err(database(step))?;
    }
    tx.commit().await.map_err(database("committing the migration"))
}

/// Write-ahead logging, so readers don't wait for a commit; `synchronous` as configured; and a
/// wait for a busy database (another connection's commit) rather than an error.
///
/// With `sync`, also `fullfsync`: on macOS a plain fsync leaves writes in the drive's cache,
/// where a power loss takes them, and only F_FULLFSYNC (as Rust's `File::sync_all` uses, and so
/// `DiskStorage`) waits for the device. Other systems ignore it.
pub async fn configure_sqlite(connection: &mut AnyConnection, sync: bool) -> Result<(), sqlx::Error> {
    connection.execute("PRAGMA journal_mode = WAL").await?;
    connection.execute(if sync { "PRAGMA synchronous = FULL" } else { "PRAGMA synchronous = NORMAL" }).await?;
    connection.execute(if sync { "PRAGMA fullfsync = ON" } else { "PRAGMA fullfsync = OFF" }).await?;
    connection.execute("PRAGMA busy_timeout = 5000").await?;
    Ok(())
}
