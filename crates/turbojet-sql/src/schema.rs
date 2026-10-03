//! The tables, and the settings each SQLite connection gets.

use std::io;

use sqlx::{AnyConnection, AnyPool, Executor};

use crate::Backend;
use crate::log::database;

/// A session's state: its sequence numbers, the incoming messages in flight to the application,
/// when it was created or last reset (a FIX UTCTimestamp), the highest sequence number whose
/// message was evicted, the bytes of messages stored, and its lease: who holds it, the token of
/// the log holding it, and until when (milliseconds since the Unix epoch).
const SESSIONS_SQLITE: &str = "CREATE TABLE IF NOT EXISTS turbojet_sessions (
    id INTEGER PRIMARY KEY,
    begin_string TEXT NOT NULL,
    sender_comp_id TEXT NOT NULL,
    target_comp_id TEXT NOT NULL,
    next_outgoing BIGINT NOT NULL DEFAULT 1,
    next_incoming BIGINT NOT NULL DEFAULT 1,
    in_flight BIGINT,
    created_at TEXT,
    evicted_through BIGINT,
    stored_bytes BIGINT NOT NULL DEFAULT 0,
    lease_holder TEXT,
    lease_token TEXT,
    lease_until BIGINT,
    UNIQUE (begin_string, sender_comp_id, target_comp_id)
)";

const SESSIONS_POSTGRES: &str = "CREATE TABLE IF NOT EXISTS turbojet_sessions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    begin_string TEXT NOT NULL,
    sender_comp_id TEXT NOT NULL,
    target_comp_id TEXT NOT NULL,
    next_outgoing BIGINT NOT NULL DEFAULT 1,
    next_incoming BIGINT NOT NULL DEFAULT 1,
    in_flight BIGINT,
    created_at TEXT,
    evicted_through BIGINT,
    stored_bytes BIGINT NOT NULL DEFAULT 0,
    lease_holder TEXT,
    lease_token TEXT,
    lease_until BIGINT,
    UNIQUE (begin_string, sender_comp_id, target_comp_id)
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
    tx.commit().await.map_err(database("committing the migration"))
}

/// Write-ahead logging, so readers don't wait for a commit; `synchronous` as configured; and a
/// wait for a busy database (another connection's commit) rather than an error.
pub async fn configure_sqlite(connection: &mut AnyConnection, sync: bool) -> Result<(), sqlx::Error> {
    connection.execute("PRAGMA journal_mode = WAL").await?;
    connection.execute(if sync { "PRAGMA synchronous = FULL" } else { "PRAGMA synchronous = NORMAL" }).await?;
    connection.execute("PRAGMA busy_timeout = 5000").await?;
    Ok(())
}
