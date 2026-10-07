//! One session's log: its state as last read or changed, the changes not yet committed, and the
//! jobs that open, commit and read it.

use std::collections::BTreeMap;
use std::hash::{BuildHasher, Hasher};
use std::io;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sqlx::any::{AnyArguments, AnyRow};
use sqlx::query::Query;
use sqlx::{Any, Row};
use tracing::{debug, info, warn};
use turbojet::fields::{FromFix, ToFix, UtcTimestamp};
use turbojet::store::{Commit, Fetched, Job, SentMessages, SessionId, SessionLog};

use crate::Shared;

/// Rows inserted per statement: four parameters each, well under SQLite's limit of 32,766
/// parameters and PostgreSQL's 65,535, while a commit of a typical batch (up to 256 messages) is
/// one statement.
const ROWS_PER_INSERT: usize = 256;

/// No message is evicted: `evicted_through` as an atomic.
const NOT_EVICTED: u64 = 0;

/// Opens `id`'s log: creates its row if it has none, takes its lease, and reads its state.
pub async fn open(shared: Arc<Shared>, id: SessionId) -> io::Result<Box<dyn SessionLog>> {
    let token = new_token();
    let now = now_millis();
    let until = now.saturating_add(millis(shared.config.lease));
    let extra = id.key_suffix();
    let mut tx = shared.pool.begin().await.map_err(database("opening a session"))?;
    sqlx::query(
        "INSERT INTO turbojet_sessions (begin_string, sender_comp_id, target_comp_id, extra) VALUES ($1, $2, $3, $4)
         ON CONFLICT (begin_string, sender_comp_id, target_comp_id, extra) DO NOTHING",
    )
    .bind(&id.begin_string)
    .bind(&id.sender_comp_id)
    .bind(&id.target_comp_id)
    .bind(&extra)
    .execute(&mut *tx)
    .await
    .map_err(database("creating a session"))?;
    // Free, expired, or ours: this process's registry already lets one connection run a session.
    let taken = sqlx::query(
        "UPDATE turbojet_sessions SET lease_holder = $5, lease_token = $6, lease_until = $7
         WHERE begin_string = $1 AND sender_comp_id = $2 AND target_comp_id = $3 AND extra = $4
           AND (lease_holder IS NULL OR lease_until < $8 OR lease_holder = $5)",
    )
    .bind(&id.begin_string)
    .bind(&id.sender_comp_id)
    .bind(&id.target_comp_id)
    .bind(&extra)
    .bind(&shared.config.holder)
    .bind(&token)
    .bind(until)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(database("taking a session's lease"))?;
    let row = sqlx::query(
        "SELECT id, next_outgoing, next_incoming, in_flight, created_at, evicted_through, lease_holder, lease_until
         FROM turbojet_sessions
         WHERE begin_string = $1 AND sender_comp_id = $2 AND target_comp_id = $3 AND extra = $4",
    )
    .bind(&id.begin_string)
    .bind(&id.sender_comp_id)
    .bind(&id.target_comp_id)
    .bind(&extra)
    .fetch_one(&mut *tx)
    .await
    .map_err(database("reading a session"))?;
    if taken.rows_affected() == 0 {
        let holder: Option<String> = row.try_get("lease_holder").map_err(database("reading a lease"))?;
        let until: Option<i64> = row.try_get("lease_until").map_err(database("reading a lease"))?;
        let left = Duration::from_millis(until.map_or(0, |until| until.saturating_sub(now)).try_into().unwrap_or(0));
        return Err(io::Error::new(
            io::ErrorKind::ResourceBusy,
            format!("{id} is leased to {} for another {left:?}", holder.as_deref().unwrap_or("another gateway")),
        ));
    }
    let log = SqlLog::read(shared.clone(), &row, token)?;
    tx.commit().await.map_err(database("committing a session's lease"))?;
    debug!(session = %id, next_outgoing = log.state.next_outgoing, next_incoming = log.state.next_incoming, "opened");
    Ok(Box::new(log))
}

/// A session's log in a SQL database.
struct SqlLog {
    shared: Arc<Shared>,
    /// The session's row.
    key: i64,
    /// Identifies this log's lease: a commit that finds another has taken the lease fails.
    token: String,
    /// As changed, committed or not.
    state: State,
    /// What the store evicted, as last committed: shared with the commit under way, which may
    /// evict more.
    evicted_through: Arc<AtomicU64>,
    /// Changes not committed yet.
    changes: Changes,
}

/// What a session's row holds, apart from its lease.
#[derive(Debug, Clone, Copy)]
struct State {
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    created_at: Option<UtcTimestamp>,
}

/// Changes since the last commit.
#[derive(Default)]
struct Changes {
    /// The state changed.
    dirty: bool,
    /// The session was reset: its stored messages go before those below.
    reset: bool,
    /// Messages stored, in order.
    messages: Vec<(u64, Vec<u8>)>,
}

impl SqlLog {
    fn read(shared: Arc<Shared>, row: &AnyRow, token: String) -> io::Result<Self> {
        let number = |column: &str| -> io::Result<Option<u64>> {
            let value: Option<i64> = row.try_get(column).map_err(database("reading a session"))?;
            value.map(|value| u64::try_from(value).map_err(|_| corrupt(column, value))).transpose()
        };
        let created_at: Option<String> = row.try_get("created_at").map_err(database("reading a session"))?;
        let created_at = created_at
            .map(|text| UtcTimestamp::from_fix(&text).map_err(|_| corrupt("created_at", text)))
            .transpose()?;
        let state = State {
            next_outgoing: number("next_outgoing")?.ok_or_else(|| corrupt("next_outgoing", "NULL"))?,
            next_incoming: number("next_incoming")?.ok_or_else(|| corrupt("next_incoming", "NULL"))?,
            in_flight: number("in_flight")?,
            created_at,
        };
        assert!(state.next_outgoing >= 1, "a session's next outgoing number is at least 1");
        assert!(state.next_incoming >= 1, "a session's next incoming number is at least 1");
        let key = row.try_get("id").map_err(database("reading a session"))?;
        let evicted_through = number("evicted_through")?.unwrap_or(NOT_EVICTED);
        Ok(Self {
            shared,
            key,
            token,
            state,
            evicted_through: Arc::new(AtomicU64::new(evicted_through)),
            changes: Changes::default(),
        })
    }

    fn changed(&mut self) {
        self.changes.dirty = true;
    }
}

impl SessionLog for SqlLog {
    fn next_outgoing(&self) -> u64 {
        self.state.next_outgoing
    }

    fn next_incoming(&self) -> u64 {
        self.state.next_incoming
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.state.next_incoming = seq;
        self.state.in_flight = None;
        self.changed();
        Ok(())
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        debug_assert!(self.changes.messages.last().is_none_or(|(last, _)| *last < seq), "messages are stored in order");
        self.state.next_outgoing = seq + 1;
        if let Some(msg) = msg {
            self.changes.messages.push((seq, msg.to_vec()));
        }
        self.changed();
        Ok(())
    }

    /// Never called: the session reads with [`fetch`](SessionLog::fetch), which may need the
    /// database.
    fn sent_messages(&mut self, _begin: u64, _end: u64) -> io::Result<SentMessages> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "a SQL store reads with fetch"))
    }

    fn fetch(&mut self, begin: u64, end: u64) -> io::Result<Fetched> {
        let range = begin..=end;
        let uncommitted: Vec<(u64, Vec<u8>)> =
            self.changes.messages.iter().filter(|(seq, _)| range.contains(seq)).cloned().collect();
        // After a reset, or for a range that starts among them, nothing committed is read.
        let first_uncommitted = self.changes.messages.first().map(|(seq, _)| *seq);
        if self.changes.reset || first_uncommitted.is_some_and(|first| first <= begin) {
            return Ok(Fetched::Ready(uncommitted));
        }
        let (shared, key) = (self.shared.clone(), self.key);
        Ok(Fetched::Pending(Job::future(async move {
            let mut read = read_messages(&shared, key, range).await?;
            read.extend(uncommitted);
            Ok(read.into_iter().collect())
        })))
    }

    fn commit(&mut self) -> io::Result<Option<Commit>> {
        if !self.changes.dirty {
            debug_assert!(self.changes.messages.is_empty() && !self.changes.reset, "a change marks the log dirty");
            return Ok(None);
        }
        let commit = Committing {
            shared: self.shared.clone(),
            key: self.key,
            token: self.token.clone(),
            state: self.state,
            changes: std::mem::take(&mut self.changes),
            evicted_through: self.evicted_through.clone(),
        };
        Ok(Some(Job::future(commit.run())))
    }

    fn reset(&mut self) -> io::Result<()> {
        self.state = State { next_outgoing: 1, next_incoming: 1, in_flight: None, created_at: None };
        self.evicted_through.store(NOT_EVICTED, Ordering::Release);
        self.changes = Changes { dirty: true, reset: true, messages: Vec::new() };
        Ok(())
    }

    fn evicted_through(&self) -> Option<u64> {
        Some(self.evicted_through.load(Ordering::Acquire)).filter(|seq| *seq != NOT_EVICTED)
    }

    fn in_flight(&self) -> Option<u64> {
        self.state.in_flight
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.state.in_flight = Some(seq);
        self.changed();
        Ok(())
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.state.created_at
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        self.state.created_at = Some(at);
        self.changed();
        Ok(())
    }
}

impl Drop for SqlLog {
    /// Gives the lease up, if a runtime is there to do it on; otherwise it expires. A later log
    /// of the session has a new token, so this never gives up its lease.
    fn drop(&mut self) {
        let Ok(runtime) = tokio::runtime::Handle::try_current() else { return };
        let (shared, key, token) = (self.shared.clone(), self.key, std::mem::take(&mut self.token));
        runtime.spawn(async move {
            let released = sqlx::query(
                "UPDATE turbojet_sessions SET lease_holder = NULL, lease_token = NULL, lease_until = NULL
                 WHERE id = $1 AND lease_token = $2",
            )
            .bind(key)
            .bind(&token)
            .execute(&shared.pool)
            .await;
            if let Err(e) = released {
                warn!("couldn't give up a session's lease, which will expire instead: {e}");
            }
        });
    }
}

/// A commit's work: the state as it stands and the changes since the last commit, written in one
/// transaction.
struct Committing {
    shared: Arc<Shared>,
    key: i64,
    token: String,
    state: State,
    changes: Changes,
    evicted_through: Arc<AtomicU64>,
}

impl Committing {
    async fn run(self) -> io::Result<()> {
        let Self { shared, key, token, state, changes, evicted_through } = self;
        let added: u64 = changes.messages.iter().map(|(_, msg)| byte_count(msg)).sum();
        let mut tx = shared.pool.begin().await.map_err(database("starting a commit"))?;
        // The lease first: if another gateway has taken it, nothing else is written.
        let until = now_millis().saturating_add(millis(shared.config.lease));
        let row = sqlx::query(
            "UPDATE turbojet_sessions SET
                next_outgoing = $3, next_incoming = $4, in_flight = $5, created_at = $6,
                evicted_through = CASE WHEN $7 THEN NULL ELSE evicted_through END,
                stored_bytes = CASE WHEN $7 THEN 0 ELSE stored_bytes END + $8,
                lease_until = $9
             WHERE id = $1 AND lease_token = $2
             RETURNING stored_bytes",
        )
        .bind(key)
        .bind(&token)
        .bind(signed(state.next_outgoing)?)
        .bind(signed(state.next_incoming)?)
        .bind(state.in_flight.map(signed).transpose()?)
        .bind(state.created_at.map(|at| at.to_fix().to_string()))
        .bind(changes.reset)
        .bind(signed(added)?)
        .bind(until)
        .fetch_optional(&mut *tx)
        .await
        .map_err(database("committing a session"))?;
        let Some(row) = row else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "the session's lease has been taken by another gateway; not committing",
            ));
        };
        let stored_bytes: i64 = row.try_get("stored_bytes").map_err(database("committing a session"))?;
        if changes.reset {
            sqlx::query("DELETE FROM turbojet_messages WHERE session = $1")
                .bind(key)
                .execute(&mut *tx)
                .await
                .map_err(database("resetting a session"))?;
        }
        for chunk in changes.messages.chunks(ROWS_PER_INSERT) {
            let sql = insert_sql(chunk.len());
            insert_messages(&sql, chunk, key).execute(&mut *tx).await.map_err(database("storing messages"))?;
        }
        let max = shared.config.max_session_bytes;
        let evicted = if u64::try_from(stored_bytes).is_ok_and(|stored| stored > max) {
            evict(&mut tx, key, max - max / 8).await?
        } else {
            None
        };
        tx.commit().await.map_err(database("committing a session"))?;
        if let Some(through) = evicted {
            evicted_through.store(through, Ordering::Release);
        }
        Ok(())
    }
}

/// An INSERT of `rows` messages.
fn insert_sql(rows: usize) -> String {
    debug_assert!(rows > 0 && rows <= ROWS_PER_INSERT);
    let mut sql = String::from("INSERT INTO turbojet_messages (session, seq, size, body) VALUES ");
    for row in 0..rows {
        let first = row * 4 + 1;
        if row > 0 {
            sql.push_str(", ");
        }
        sql.push_str(&format!("(${first}, ${}, ${}, ${})", first + 1, first + 2, first + 3));
    }
    // A number used again after an operator moved the next outgoing number back replaces the
    // message stored under it.
    sql.push_str(" ON CONFLICT (session, seq) DO UPDATE SET size = excluded.size, body = excluded.body");
    sql
}

/// `sql`, from [`insert_sql`], with `messages` of session `key` bound.
fn insert_messages<'q>(sql: &'q str, messages: &'q [(u64, Vec<u8>)], key: i64) -> Query<'q, Any, AnyArguments<'q>> {
    let mut query = sqlx::query(sql);
    for (seq, msg) in messages {
        // In range: sequence numbers are below 2^63, as are message sizes.
        let seq = i64::try_from(*seq).unwrap_or(i64::MAX);
        let size = i64::try_from(msg.len()).unwrap_or(i64::MAX);
        query = query.bind(key).bind(seq).bind(size).bind(msg.as_slice());
    }
    query
}

/// Deletes session `key`'s oldest messages until those left take at most `keep` bytes, and
/// records the highest sequence number deleted, which it returns, if any.
async fn evict(tx: &mut sqlx::Transaction<'_, Any>, key: i64, keep: u64) -> io::Result<Option<u64>> {
    let cut = sqlx::query(
        "SELECT seq FROM (
            SELECT seq, SUM(size) OVER (ORDER BY seq DESC) AS kept FROM turbojet_messages WHERE session = $1
         ) AS newest_first WHERE kept > $2 ORDER BY seq DESC LIMIT 1",
    )
    .bind(key)
    .bind(signed(keep)?)
    .fetch_optional(&mut **tx)
    .await
    .map_err(database("finding messages to evict"))?;
    let Some(cut) = cut else { return Ok(None) };
    let through: i64 = cut.try_get("seq").map_err(database("finding messages to evict"))?;
    sqlx::query("DELETE FROM turbojet_messages WHERE session = $1 AND seq <= $2")
        .bind(key)
        .bind(through)
        .execute(&mut **tx)
        .await
        .map_err(database("evicting messages"))?;
    sqlx::query(
        "UPDATE turbojet_sessions SET evicted_through = $2,
            stored_bytes = (SELECT COALESCE(SUM(size), 0) FROM turbojet_messages WHERE session = $1)
         WHERE id = $1",
    )
    .bind(key)
    .bind(through)
    .execute(&mut **tx)
    .await
    .map_err(database("evicting messages"))?;
    let through = u64::try_from(through).map_err(|_| corrupt("seq", through))?;
    info!(through, "evicted the oldest stored messages to stay within the byte budget");
    Ok(Some(through))
}

/// Session `key`'s stored messages in `range`, in order.
async fn read_messages(shared: &Shared, key: i64, range: RangeInclusive<u64>) -> io::Result<BTreeMap<u64, Vec<u8>>> {
    let clamp = |seq: u64| i64::try_from(seq).unwrap_or(i64::MAX);
    let rows = sqlx::query("SELECT seq, body FROM turbojet_messages WHERE session = $1 AND seq BETWEEN $2 AND $3")
        .bind(key)
        .bind(clamp(*range.start()))
        .bind(clamp(*range.end()))
        .fetch_all(&shared.pool)
        .await
        .map_err(database("reading stored messages"))?;
    let mut read = BTreeMap::new();
    for row in rows {
        let seq: i64 = row.try_get("seq").map_err(database("reading stored messages"))?;
        let body: Vec<u8> = row.try_get("body").map_err(database("reading stored messages"))?;
        read.insert(u64::try_from(seq).map_err(|_| corrupt("seq", seq))?, body);
    }
    Ok(read)
}

/// Turns a database error into an I/O error, saying what was being done.
pub fn database(doing: &'static str) -> impl FnOnce(sqlx::Error) -> io::Error {
    move |e| io::Error::other(format!("{doing}: {e}"))
}

fn corrupt(column: &str, value: impl std::fmt::Debug) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("turbojet_sessions.{column} is corrupt: {value:?}"))
}

/// A number as the database stores it: sequence numbers and byte counts are far below 2^63.
fn signed(value: u64) -> io::Result<i64> {
    i64::try_from(value)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("{value} is too big to store")))
}

fn byte_count(msg: &[u8]) -> u64 {
    u64::try_from(msg.len()).unwrap_or(u64::MAX)
}

fn millis(duration: Duration) -> i64 {
    i64::try_from(duration.as_millis()).unwrap_or(i64::MAX)
}

/// The time on this gateway's clock, in milliseconds since the Unix epoch, as leases record it.
fn now_millis() -> i64 {
    millis(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default())
}

/// A token unique to one opening of a log: random, from the standard library's hasher keys.
fn new_token() -> String {
    static COUNT: AtomicU64 = AtomicU64::new(0);
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u64(COUNT.fetch_add(1, Ordering::Relaxed));
    format!("{:016x}", hasher.finish())
}
