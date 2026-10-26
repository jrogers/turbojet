//! A store per counterparty.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::sync::Arc;

use super::{Job, Marks, Opened, SessionId, SessionStorage, StoredSession};

/// Keeps each listed counterparty's sessions in its own store, and the rest in a default one,
/// choosing by the counterparty's CompID ([`SessionId::target_comp_id`]), or for a session listed
/// whole ([`with_session`](Self::with_session)), by its ID.
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
///     .with("BROKER", Arc::new(DiskStorage::new(dir.path()).unwrap()));
/// ```
pub struct StorageByCounterparty {
    default: Arc<dyn SessionStorage>,
    by_comp_id: HashMap<String, Arc<dyn SessionStorage>>,
    by_session: HashMap<SessionId, Arc<dyn SessionStorage>>,
}

impl StorageByCounterparty {
    /// Keeps every session in `default`, until counterparties are listed with
    /// [`with`](Self::with).
    pub fn new(default: Arc<dyn SessionStorage>) -> Self {
        Self { default, by_comp_id: HashMap::new(), by_session: HashMap::new() }
    }

    /// Keeps counterparty `comp_id`'s sessions in `storage`, replacing any earlier entry for it.
    #[must_use]
    pub fn with(mut self, comp_id: impl Into<String>, storage: Arc<dyn SessionStorage>) -> Self {
        self.by_comp_id.insert(comp_id.into(), storage);
        self
    }

    /// Keeps session `id` in `storage`, over any entry for its CompID, replacing any earlier
    /// entry for the session.
    #[must_use]
    pub fn with_session(mut self, id: SessionId, storage: Arc<dyn SessionStorage>) -> Self {
        self.by_session.insert(id, storage);
        self
    }

    fn route(&self, id: &SessionId) -> &Arc<dyn SessionStorage> {
        self.by_session.get(id).or_else(|| self.by_comp_id.get(&id.target_comp_id)).unwrap_or(&self.default)
    }

    fn stores(&self) -> impl Iterator<Item = &Arc<dyn SessionStorage>> {
        std::iter::once(&self.default).chain(self.by_comp_id.values()).chain(self.by_session.values())
    }
}

/// Whether `a` and `b` are the same store.
fn same(a: &Arc<dyn SessionStorage>, b: &Arc<dyn SessionStorage>) -> bool {
    std::ptr::addr_eq(Arc::as_ptr(a), Arc::as_ptr(b))
}

impl SessionStorage for StorageByCounterparty {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
        self.route(id).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.route(id).begin_open(id)
    }

    fn is_durable(&self) -> bool {
        self.stores().all(|storage| storage.is_durable())
    }

    /// Each store's marked sessions that route to it: one a store keeps for a counterparty
    /// since routed elsewhere is left out, as opening the session wouldn't find it there.
    fn marked(&self) -> Job<Vec<(SessionId, Marks)>> {
        let mut stores: Vec<&Arc<dyn SessionStorage>> = Vec::new();
        for store in self.stores() {
            if !stores.iter().any(|seen| same(seen, store)) {
                stores.push(store);
            }
        }
        let jobs = stores
            .into_iter()
            .map(|store| {
                let (store, routes) = (store.clone(), self.clone_routes());
                store
                    .marked()
                    .map(move |marked| marked.into_iter().filter(|(id, _)| same(routes.route(id), &store)).collect())
            })
            .collect();
        Job::concat(jobs)
    }

    /// Each store's sessions that route to it, as for [`marked`](SessionStorage::marked).
    fn sessions(&self) -> Job<Vec<SessionId>> {
        let mut stores: Vec<&Arc<dyn SessionStorage>> = Vec::new();
        for store in self.stores() {
            if !stores.iter().any(|seen| same(seen, store)) {
                stores.push(store);
            }
        }
        let jobs = stores
            .into_iter()
            .map(|store| {
                let (store, routes) = (store.clone(), self.clone_routes());
                store.sessions().map(move |ids| ids.into_iter().filter(|id| same(routes.route(id), &store)).collect())
            })
            .collect();
        Job::concat(jobs)
    }
}

impl StorageByCounterparty {
    fn clone_routes(&self) -> Self {
        Self { default: self.default.clone(), by_comp_id: self.by_comp_id.clone(), by_session: self.by_session.clone() }
    }
}

impl fmt::Debug for StorageByCounterparty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comp_ids: Vec<_> = self.by_comp_id.keys().collect();
        comp_ids.sort();
        let mut sessions: Vec<String> = self.by_session.keys().map(ToString::to_string).collect();
        sessions.sort();
        f.debug_struct("StorageByCounterparty")
            .field("comp_ids", &comp_ids)
            .field("sessions", &sessions)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemoryStorage;
    use crate::store::conformance::{self, id};

    /// Durable only if the default and every routed store are.
    #[test]
    fn is_durable_if_every_store_is() {
        struct Durable;
        impl SessionStorage for Durable {
            fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
                MemoryStorage::new().open(id)
            }
            fn is_durable(&self) -> bool {
                true
            }
        }
        let durable = || -> Arc<dyn SessionStorage> { Arc::new(Durable) };
        assert!(!MemoryStorage::new().is_durable());
        assert!(StorageByCounterparty::new(durable()).with("A", durable()).is_durable());
        assert!(!StorageByCounterparty::new(durable()).with("A", Arc::new(MemoryStorage::new())).is_durable());
        assert!(!StorageByCounterparty::new(Arc::new(MemoryStorage::new())).with("A", durable()).is_durable());
    }

    #[test]
    fn passes_the_conformance_suite() {
        // The suite uses counterparties A to F: some routed, some not.
        let storage = StorageByCounterparty::new(Arc::new(MemoryStorage::new()))
            .with("A", Arc::new(MemoryStorage::new()))
            .with("C", Arc::new(MemoryStorage::new()));
        conformance::check_blocking(&storage);
        let routed =
            StorageByCounterparty::new(Arc::new(MemoryStorage::new())).with("H", Arc::new(MemoryStorage::new()));
        conformance::check_marks_blocking(&routed);
        // L's store is shared with N, so it's asked once; M goes to the default.
        let shared: Arc<dyn SessionStorage> = Arc::new(MemoryStorage::new());
        let routed =
            StorageByCounterparty::new(Arc::new(MemoryStorage::new())).with("L", shared.clone()).with("N", shared);
        conformance::check_sessions_blocking(&routed);
    }

    /// Each store's marks are listed once, however many counterparties share it, and only for
    /// sessions that route to it.
    #[test]
    fn marked_sessions_come_from_the_store_they_route_to() {
        let shared: Arc<dyn SessionStorage> = Arc::new(MemoryStorage::new());
        let default: Arc<dyn SessionStorage> = Arc::new(MemoryStorage::new());
        let paused = Marks { paused: true, ..Marks::default() };
        for (store, target) in [(&shared, "A"), (&default, "A"), (&default, "Z")] {
            store.open(&id(target)).unwrap().set_marks(paused).unwrap();
        }
        let storage = StorageByCounterparty::new(default).with("A", shared.clone()).with("B", shared);
        let mut marked = storage.marked().run().unwrap();
        marked.sort_by(|a, b| a.0.target_comp_id.cmp(&b.0.target_comp_id));
        assert_eq!(marked, [(id("A"), paused), (id("Z"), paused)], "the default's A is routed elsewhere");
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
        assert_eq!(format!("{storage:?}"), r#"StorageByCounterparty { comp_ids: ["BROKER"], sessions: [], .. }"#);
    }

    /// A session listed whole is kept in its own store, over its CompID's.
    #[test]
    fn a_session_listed_whole_is_kept_in_its_own_store() {
        let (broker, desk_store) = (Arc::new(MemoryStorage::new()), Arc::new(MemoryStorage::new()));
        let desk = id("BROKER").with_target_sub_id("DESK2");
        let storage = StorageByCounterparty::new(Arc::new(MemoryStorage::new()))
            .with("BROKER", broker.clone())
            .with_session(desk.clone(), desk_store.clone());
        storage.open(&desk).unwrap().set_next_incoming(5).unwrap();
        storage.open(&desk.clone().with_target_sub_id("DESK3")).unwrap().set_next_incoming(6).unwrap();
        assert_eq!(desk_store.open(&desk).unwrap().next_incoming(), 5);
        assert_eq!(broker.open(&desk).unwrap().next_incoming(), 1, "not in the CompID's store");
        assert_eq!(broker.open(&desk.with_target_sub_id("DESK3")).unwrap().next_incoming(), 6);
    }
}
