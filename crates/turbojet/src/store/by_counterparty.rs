//! A store per counterparty.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::sync::Arc;

use super::{Opened, SessionId, SessionLog, SessionStorage};

/// Keeps each listed counterparty's sessions in its own store, and the rest in a default one,
/// choosing by the counterparty's CompID ([`SessionId::target_comp_id`]).
///
/// An [`Acceptor`](crate::Acceptor) opens a session's log at Logon, and also for an operator's
/// change to a disconnected session, when there's no Logon to decide from: so a store is chosen
/// by the session's ID alone, here, rather than with the rest of a counterparty's settings in
/// [`Counterparties`](crate::Counterparties).
///
/// ```
/// # use std::sync::Arc;
/// # use turbojet::{DiskStorage, MemoryStorage};
/// # use turbojet::store::StorageByCounterparty;
/// # let dir = tempfile::tempdir().unwrap();
/// let storage = StorageByCounterparty::new(Arc::new(MemoryStorage::new()))
///     .with("BROKER", Arc::new(DiskStorage::new(dir.path(), true).unwrap()));
/// ```
pub struct StorageByCounterparty {
    default: Arc<dyn SessionStorage>,
    by_comp_id: HashMap<String, Arc<dyn SessionStorage>>,
}

impl StorageByCounterparty {
    /// Keeps every session in `default`, until counterparties are listed with
    /// [`with`](Self::with).
    pub fn new(default: Arc<dyn SessionStorage>) -> Self {
        Self { default, by_comp_id: HashMap::new() }
    }

    /// Keeps counterparty `comp_id`'s sessions in `storage`, replacing any earlier entry for it.
    #[must_use]
    pub fn with(mut self, comp_id: impl Into<String>, storage: Arc<dyn SessionStorage>) -> Self {
        self.by_comp_id.insert(comp_id.into(), storage);
        self
    }

    fn route(&self, id: &SessionId) -> &dyn SessionStorage {
        self.by_comp_id.get(&id.target_comp_id).unwrap_or(&self.default).as_ref()
    }
}

impl SessionStorage for StorageByCounterparty {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        self.route(id).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.route(id).begin_open(id)
    }
}

impl fmt::Debug for StorageByCounterparty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comp_ids: Vec<_> = self.by_comp_id.keys().collect();
        comp_ids.sort();
        f.debug_struct("StorageByCounterparty").field("comp_ids", &comp_ids).finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemoryStorage;
    use crate::store::conformance::{self, id};

    #[test]
    fn passes_the_conformance_suite() {
        // The suite uses counterparties A to F: some routed, some not.
        let storage = StorageByCounterparty::new(Arc::new(MemoryStorage::new()))
            .with("A", Arc::new(MemoryStorage::new()))
            .with("C", Arc::new(MemoryStorage::new()));
        conformance::check_blocking(&storage);
    }

    #[test]
    fn a_listed_counterpartys_sessions_are_kept_in_its_own_store() {
        let default = Arc::new(MemoryStorage::new());
        let broker = Arc::new(MemoryStorage::new());
        let storage = StorageByCounterparty::new(default.clone()).with("BROKER", broker.clone());
        for (comp_id, next) in [("BROKER", 5), ("OTHER", 7)] {
            let mut log = storage.open(&id(comp_id)).unwrap();
            log.set_next_incoming(next).unwrap();
            assert!(log.commit().unwrap().is_none(), "memory commits at once");
        }
        assert_eq!(broker.open(&id("BROKER")).unwrap().next_incoming(), 5);
        assert_eq!(default.open(&id("BROKER")).unwrap().next_incoming(), 1, "not in the default");
        assert_eq!(default.open(&id("OTHER")).unwrap().next_incoming(), 7);
        assert_eq!(broker.open(&id("OTHER")).unwrap().next_incoming(), 1, "not in BROKER's");
        assert_eq!(format!("{storage:?}"), r#"StorageByCounterparty { comp_ids: ["BROKER"], .. }"#);
    }
}
