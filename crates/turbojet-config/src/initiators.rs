//! The initiators a sessions file lists, each running on its own task.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::{error, info};
use turbojet::{Application, Initiator, SessionEvent, SessionHandle, SessionId, SessionRegistry};

use crate::Error;
use crate::load::{Loaded, ResolvedInitiator};
use crate::reload::{Changes, InitiatorChanges};

/// The Text of the Logout an initiator removed from the file sends.
const REMOVED: &str = "no longer configured";

/// The initiators a [`SessionsFile`](crate::SessionsFile) lists, each keeping its session
/// connected on a task of its own, sharing one session registry: a [`SessionHandle`] stays valid
/// while its initiator is reconfigured or restarted by a reload.
///
/// Dropping it stops them at once, without logging out; [`shutdown`](Self::shutdown) logs them
/// out first.
pub struct Initiators {
    app: Arc<dyn Application>,
    registry: Arc<SessionRegistry>,
    running: Mutex<BTreeMap<String, Running>>,
}

/// One initiator, running.
struct Running {
    initiator: Initiator,
    task: JoinHandle<()>,
    #[cfg(feature = "tls")]
    tls: Option<turbojet::tls::ClientTls>,
}

impl Initiators {
    pub(crate) fn new(app: Arc<dyn Application>, registry: Arc<SessionRegistry>) -> Self {
        Self { app, registry, running: Mutex::default() }
    }

    /// The initiators running, by section name, in order.
    pub fn names(&self) -> Vec<String> {
        self.running().keys().cloned().collect()
    }

    /// Events for every initiator's session from now on, including those a reload starts later:
    /// see [`SessionRegistry::subscribe`].
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.registry.subscribe()
    }

    /// The session initiator `name` logs on to, if it's running.
    pub fn session_id(&self, name: &str) -> Option<SessionId> {
        self.running().get(name).map(|running| running.initiator.session_id())
    }

    /// A handle for sending on initiator `name`'s session, if it's running. It stays valid across
    /// reloads that reconfigure the initiator, and across reconnects.
    pub fn handle(&self, name: &str) -> Option<SessionHandle> {
        self.session_id(name).map(|id| self.registry.handle(id))
    }

    /// Logs every session out (with `text` as the Logout's Text) and stops every initiator,
    /// returning once all have stopped. See [`Initiator::shutdown`].
    pub async fn shutdown(&self, text: Option<&str>) {
        let running = std::mem::take(&mut *self.running());
        let mut stopping = tokio::task::JoinSet::new();
        for (_, running) in running {
            let text = text.map(str::to_string);
            stopping.spawn(async move {
                running.initiator.shutdown(text.as_deref()).await;
                let _ = running.task.await;
            });
        }
        while stopping.join_next().await.is_some() {}
    }

    /// Starts initiator `name`.
    pub(crate) fn start(&self, name: &str, resolved: &ResolvedInitiator) -> Result<(), Error> {
        #[cfg(feature = "tls")]
        let section = format!("initiator {name}");
        #[cfg(feature = "tls")]
        let tls = match &resolved.tls {
            Some(files) => Some(
                turbojet::tls::ClientTls::new(files.trust.clone(), files.identity.clone())
                    .map_err(|e| Error::at(&section, "tls", e))?,
            ),
            None => None,
        };
        #[cfg(feature = "tls")]
        let config = config(resolved, tls.as_ref());
        #[cfg(not(feature = "tls"))]
        let config = resolved.config.clone();
        let initiator = Initiator::with_registry(config, self.registry.clone(), self.app.clone())
            .expect("checked when the file loaded");
        let task = tokio::spawn(initiator.clone().run());
        info!(initiator = name, session = %initiator.session_id(), "initiator started");

        self.running().insert(
            name.to_string(),
            Running {
                initiator,
                task,
                #[cfg(feature = "tls")]
                tls,
            },
        );
        Ok(())
    }

    /// Stops initiator `name`, logging its session out in the background.
    fn stop(&self, name: &str) {
        let Some(running) = self.running().remove(name) else { return };
        info!(initiator = name, session = %running.initiator.session_id(), "stopping an initiator no longer configured");
        tokio::spawn(async move {
            running.initiator.shutdown(Some(REMOVED)).await;
            let _ = running.task.await;
        });
    }

    /// Gives initiator `name` its new settings, from its next connection.
    fn reconfigure(&self, name: &str, resolved: &ResolvedInitiator) -> Result<(), Error> {
        let section = format!("initiator {name}");
        let running = self.running();
        let Some(running) = running.get(name) else { return Ok(()) };
        #[cfg(feature = "tls")]
        if let (Some(tls), Some(files)) = (&running.tls, &resolved.tls) {
            tls.set_trust(files.trust.clone()).map_err(|e| Error::at(&section, "tls", e))?;
            tls.set_identity(files.identity.clone());
        }
        #[cfg(feature = "tls")]
        let config = config(resolved, running.tls.as_ref());
        #[cfg(not(feature = "tls"))]
        let config = resolved.config.clone();
        running.initiator.reconfigure(config).map_err(|e| Error::at(&section, "settings", e))?;
        info!(initiator = name, "initiator reconfigured; applies from its next connection");
        Ok(())
    }

    /// Applies a reload's `changes` with the file `loaded`, recording them in `report`. The file
    /// was checked whole before, so a failure here is logged, not returned: the rest still apply.
    pub(crate) fn apply(&self, changes: &InitiatorChanges, loaded: &Loaded, report: &mut Changes) {
        for name in &changes.stop {
            self.stop(name);
            report.stopped.push(name.clone());
        }
        for name in &changes.start {
            match self.start(name, &loaded.initiators[name]) {
                Ok(()) => report.started.push(name.clone()),
                Err(e) => error!("cannot start an initiator: {e}"),
            }
        }
        for name in &changes.reconfigure {
            match self.reconfigure(name, &loaded.initiators[name]) {
                Ok(()) => report.reconfigured.push(name.clone()),
                Err(e) => error!("cannot reconfigure an initiator: {e}"),
            }
        }
    }

    fn running(&self) -> MutexGuard<'_, BTreeMap<String, Running>> {
        self.running.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// An initiator's endpoints, each naming the server its TLS certificate must be for.
/// The initiator's configuration, with its TLS settings over `tls`.
#[cfg(feature = "tls")]
fn config(resolved: &ResolvedInitiator, tls: Option<&turbojet::tls::ClientTls>) -> turbojet::InitiatorConfig {
    let mut config = resolved.config.clone();
    #[cfg(feature = "tls")]
    if let (Some(tls), Some(files)) = (tls, &resolved.tls) {
        config.tls = Some(turbojet::InitiatorTls::new(tls.connector(), &files.server_name));
    }
    config
}

impl Drop for Initiators {
    fn drop(&mut self) {
        for running in self.running().values() {
            running.task.abort();
        }
    }
}

impl fmt::Debug for Initiators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Initiators").field("names", &self.names()).finish_non_exhaustive()
    }
}
