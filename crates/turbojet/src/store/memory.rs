//! In-memory session storage: survives reconnects, not restarts.

use std::collections::{BTreeMap, HashMap};
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

use super::{SessionId, SessionLog, SessionStorage};
use crate::codec::frame_stored;
use crate::fields::UtcTimestamp;

#[derive(Debug)]
struct State {
    next_outgoing: u64,
    next_incoming: u64,
    /// The newest messages sent since a sequence reset, at most `max_session_bytes` of them.
    sent: BTreeMap<u64, Vec<u8>>,
    /// The total length of `sent`.
    bytes: usize,
    /// The highest sequence number evicted from `sent`.
    evicted_through: Option<u64>,
    created_at: Option<UtcTimestamp>,
    in_flight: Option<u64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_outgoing: 1,
            next_incoming: 1,
            sent: BTreeMap::new(),
            bytes: 0,
            evicted_through: None,
            created_at: None,
            in_flight: None,
        }
    }
}

impl State {
    /// Stores `msg`, then evicts the oldest messages until the total is within `max_bytes`. A
    /// message bigger than `max_bytes` evicts everything, itself included.
    fn store(&mut self, seq: u64, msg: &[u8], max_bytes: usize) {
        self.bytes += msg.len();
        if let Some(replaced) = self.sent.insert(seq, msg.to_vec()) {
            self.bytes -= replaced.len();
        }
        while self.bytes > max_bytes {
            let (evicted, bytes) = self.sent.pop_first().expect("bytes counts what's stored");
            self.bytes -= bytes.len();
            self.evicted_through = self.evicted_through.max(Some(evicted));
        }
        // Not a sum over `sent`: that would make each store cost as much as everything stored.
        debug_assert!(self.bytes <= max_bytes && self.sent.is_empty() == (self.bytes == 0));
        debug_assert!(self.sent.first_key_value().is_none_or(|(first, _)| Some(*first) > self.evicted_through));
    }
}

/// Keeps each session's state in memory: it survives reconnects, but not the process.
///
/// Both kinds of growth are bounded. Each session keeps its newest sent messages up to a byte
/// budget ([`DEFAULT_MAX_SESSION_BYTES`](Self::DEFAULT_MAX_SESSION_BYTES) unless set by
/// [`with_max_session_bytes`](Self::with_max_session_bytes)); older ones are evicted, and a resend
/// that reaches them gap-fills them, so the counterparty never receives them again (the session
/// logs a warning and counts it in `turbojet_resend_requests_evicted_total`). The store keeps at
/// most [`DEFAULT_MAX_SESSIONS`](Self::DEFAULT_MAX_SESSIONS) sessions (or [`with_max_sessions`](Self::with_max_sessions)),
/// never forgetting one: opening a new session past the cap fails with
/// [`io::ErrorKind::QuotaExceeded`], so its Logon is refused, while known sessions reopen as
/// usual.
pub struct MemoryStorage {
    /// One per session opened, never removed, at most `max_sessions`. An acceptor opens one for
    /// each CompID that [`Application::verify_logon`](crate::Application::verify_logon) accepts.
    sessions: Mutex<HashMap<SessionId, Arc<Mutex<State>>>>,
    max_sessions: usize,
    max_session_bytes: usize,
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self {
            sessions: Mutex::default(),
            max_sessions: Self::DEFAULT_MAX_SESSIONS,
            max_session_bytes: Self::DEFAULT_MAX_SESSION_BYTES,
        }
    }
}

impl MemoryStorage {
    /// The default byte budget for each session's stored messages: room for hundreds of thousands of
    /// typical orders and reports, so a day's resends are rarely cut short, while a session that
    /// sends for weeks without a sequence reset stops growing.
    pub const DEFAULT_MAX_SESSION_BYTES: usize = 64 * 1024 * 1024;

    /// The default number of sessions a store keeps: well above the counterparties one acceptor
    /// usually serves, and with [`DEFAULT_MAX_SESSION_BYTES`](Self::DEFAULT_MAX_SESSION_BYTES) a bound (64 GiB) on the whole store.
    pub const DEFAULT_MAX_SESSIONS: usize = 1024;

    /// An empty store with the default limits.
    pub fn new() -> Self {
        Self::default()
    }

    /// Keeps at most `bytes` of each session's sent messages, evicting the oldest first.
    #[must_use]
    pub fn with_max_session_bytes(mut self, bytes: usize) -> Self {
        self.max_session_bytes = bytes;
        self
    }

    /// Keeps at most `sessions` sessions, refusing to open a new one past it.
    #[must_use]
    pub fn with_max_sessions(mut self, sessions: usize) -> Self {
        self.max_sessions = sessions;
        self
    }
}

impl SessionStorage for MemoryStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        let mut sessions = self.sessions.lock().expect("memory storage lock poisoned");
        if !sessions.contains_key(id) && sessions.len() >= self.max_sessions {
            return Err(io::Error::new(
                io::ErrorKind::QuotaExceeded,
                format!("memory storage holds its maximum of {} sessions", self.max_sessions),
            ));
        }
        let state = sessions.entry(id.clone()).or_default().clone();
        debug_assert!(sessions.len() <= self.max_sessions);
        Ok(Box::new(MemoryLog { state, max_bytes: self.max_session_bytes }))
    }
}

struct MemoryLog {
    state: Arc<Mutex<State>>,
    max_bytes: usize,
}

impl MemoryLog {
    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().expect("memory storage lock poisoned")
    }
}

impl SessionLog for MemoryLog {
    fn next_outgoing(&self) -> u64 {
        self.state().next_outgoing
    }

    fn next_incoming(&self) -> u64 {
        self.state().next_incoming
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        let mut state = self.state();
        state.next_incoming = seq;
        state.in_flight = None;
        Ok(())
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        let mut state = self.state();
        if let Some(msg) = msg {
            debug_assert_eq!(frame_stored(msg), Ok(msg.len()), "one whole message");
            state.store(seq, msg, self.max_bytes);
        }
        state.next_outgoing = seq + 1;
        Ok(())
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        Ok(self.state().sent.range(begin..=end).map(|(seq, m)| (*seq, m.clone())).collect())
    }

    fn reset(&mut self) -> io::Result<()> {
        *self.state() = State::default();
        Ok(())
    }

    fn evicted_through(&self) -> Option<u64> {
        self.state().evicted_through
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.state().created_at
    }

    fn in_flight(&self) -> Option<u64> {
        self.state().in_flight
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.state().in_flight = Some(seq);
        Ok(())
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        self.state().created_at = Some(at);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::codec::encode;
    use crate::fields::MsgType;
    use crate::message::{Message, tags};
    use crate::store::conformance::{app_message, id};

    #[test]
    fn conforms() {
        super::super::conformance::check_blocking(&MemoryStorage::new());
    }

    fn stored(log: &mut Box<dyn SessionLog>) -> Vec<u64> {
        log.sent_messages(1, u64::MAX).unwrap().into_iter().map(|(seq, _)| seq).collect()
    }

    #[test]
    fn evicts_the_oldest_messages_past_the_byte_budget() {
        let len = app_message(1).len();
        let storage = MemoryStorage::new().with_max_session_bytes(2 * len + len / 2);
        let mut log = storage.open(&id("A")).unwrap();
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        log.record_outgoing(2, Some(&app_message(2))).unwrap();
        assert_eq!(log.evicted_through(), None);
        log.record_outgoing(3, None).unwrap();
        log.record_outgoing(4, Some(&app_message(4))).unwrap();
        assert_eq!(stored(&mut log), [2, 4]);
        assert_eq!(log.evicted_through(), Some(1));
        log.record_outgoing(5, Some(&app_message(5))).unwrap();
        assert_eq!(stored(&mut log), [4, 5]);
        assert_eq!(log.evicted_through(), Some(2));
        assert_eq!(log.next_outgoing(), 6);
    }

    #[test]
    fn a_message_bigger_than_the_budget_evicts_everything() {
        let len = app_message(1).len();
        let storage = MemoryStorage::new().with_max_session_bytes(len + 1);
        let mut log = storage.open(&id("A")).unwrap();
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        log.record_outgoing(2, Some(&app_message(2))).unwrap();
        assert_eq!(stored(&mut log), [2]);
        let big = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.4")
            .with(tags::MSG_TYPE, MsgType::ExecutionReport)
            .with(tags::MSG_SEQ_NUM, 3_u64)
            .with(tags::TEXT, "x".repeat(len));
        log.record_outgoing(3, Some(&encode(&big).unwrap())).unwrap();
        assert!(stored(&mut log).is_empty());
        assert_eq!(log.evicted_through(), Some(3));
        assert_eq!(log.next_outgoing(), 4);
    }

    #[test]
    fn reset_forgets_evictions() {
        let len = app_message(1).len();
        let storage = MemoryStorage::new().with_max_session_bytes(2 * len);
        let mut log = storage.open(&id("A")).unwrap();
        for seq in 1..=3 {
            log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
        }
        assert_eq!(log.evicted_through(), Some(1));
        log.reset().unwrap();
        assert_eq!(log.evicted_through(), None);
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        log.record_outgoing(2, Some(&app_message(2))).unwrap();
        assert_eq!(stored(&mut log), [1, 2], "the budget is whole again");
    }

    #[test]
    fn refuses_a_new_session_past_the_cap() {
        let storage = MemoryStorage::new().with_max_sessions(2);
        storage.open(&id("A")).unwrap().record_outgoing(1, None).unwrap();
        storage.open(&id("B")).unwrap();
        let err = storage.open(&id("C")).err().expect("a third session is refused");
        assert_eq!(err.kind(), io::ErrorKind::QuotaExceeded);
        assert_eq!(storage.open(&id("A")).unwrap().next_outgoing(), 2, "a known session reopens");
    }
}
