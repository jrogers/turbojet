//! In-memory session storage: survives reconnects, not restarts.

use std::collections::{BTreeMap, HashMap};
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

use super::{SessionId, SessionLog, SessionStorage};
use crate::fields::UtcTimestamp;

#[derive(Debug)]
struct State {
    next_outgoing: u64,
    next_incoming: u64,
    sent: BTreeMap<u64, Vec<u8>>,
    created_at: Option<UtcTimestamp>,
    in_flight: Option<u64>,
}

impl Default for State {
    fn default() -> Self {
        Self { next_outgoing: 1, next_incoming: 1, sent: BTreeMap::new(), created_at: None, in_flight: None }
    }
}

/// Keeps each session's state in memory: it survives reconnects, but not the process.
#[derive(Default)]
pub struct MemoryStorage {
    sessions: Mutex<HashMap<SessionId, Arc<Mutex<State>>>>,
}

impl MemoryStorage {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl SessionStorage for MemoryStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        let mut sessions = self.sessions.lock().expect("memory storage lock poisoned");
        let state = sessions.entry(id.clone()).or_default().clone();
        Ok(Box::new(MemoryLog { state }))
    }
}

struct MemoryLog {
    state: Arc<Mutex<State>>,
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
            state.sent.insert(seq, msg.to_vec());
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

    #[test]
    fn conforms() {
        super::super::conformance::check(&MemoryStorage::new());
    }
}
