//! Graceful shutdown, shared by an [`Acceptor`](crate::Acceptor) or [`Initiator`](crate::Initiator)
//! and its clones.

use std::future::pending;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::watch;
use tracing::{info, warn};

/// How much longer than the logout timeout shutdown waits before closing connections itself.
/// Sessions time out their own logouts, so normally they have all closed by then; this catches
/// one that can't, such as a connection blocked writing to a counterparty that stopped reading.
const FORCE_CLOSE_GRACE: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
    Running,
    /// Sessions log out, with this Text(58).
    LoggingOut(Option<String>),
    /// Connections still open are closed without waiting.
    Closing,
}

#[derive(Debug)]
pub(crate) struct Shutdown {
    phase: watch::Sender<Phase>,
    /// Connections still open, including TLS handshakes in progress.
    open: watch::Sender<usize>,
}

impl Shutdown {
    pub(crate) fn new() -> Self {
        Self { phase: watch::Sender::new(Phase::Running), open: watch::Sender::new(0) }
    }

    pub(crate) fn is_started(&self) -> bool {
        *self.phase.borrow() != Phase::Running
    }

    pub(crate) fn signal(&self) -> Signal {
        Signal(self.phase.subscribe())
    }

    /// Counts a connection as open until the guard is dropped; [`Shutdown::run`] waits for it.
    pub(crate) fn track(self: &Arc<Self>) -> Open {
        self.open.send_modify(|open| *open += 1);
        Open(self.clone())
    }

    /// Starts shutting down, if it hasn't already, and waits until every connection has closed:
    /// by logging out, or after `logout_timeout` (and a little more) by force.
    pub(crate) async fn run(&self, text: Option<&str>, logout_timeout: Duration) {
        let started = self.phase.send_if_modified(|phase| {
            let start = *phase == Phase::Running;
            if start {
                *phase = Phase::LoggingOut(text.map(String::from));
            }
            start
        });
        let mut open = self.open.subscribe();
        if started {
            info!(connections = *open.borrow(), "shutting down; logging sessions out");
        }
        let wait = logout_timeout + FORCE_CLOSE_GRACE;
        let timed_out = tokio::time::timeout(wait, open.wait_for(|n| *n == 0)).await.is_err();
        if timed_out {
            warn!(connections = *open.borrow(), "connections still open after the logout timeout; closing them");
            self.phase.send_replace(Phase::Closing);
            // The sender is `self.open`, so this can't fail.
            let _ = open.wait_for(|n| *n == 0).await;
        }
    }
}

/// An open connection, counted by [`Shutdown::track`].
pub(crate) struct Open(Arc<Shutdown>);

impl Drop for Open {
    fn drop(&mut self) {
        self.0.open.send_modify(|open| *open -= 1);
    }
}

/// What one connection watches to learn that shutdown has started.
#[derive(Debug, Clone)]
pub(crate) struct Signal(watch::Receiver<Phase>);

impl Signal {
    /// Completes once shutdown starts (at once if it has), with the Logout's Text. Never
    /// completes if the Acceptor or Initiator, and every clone of it, has been dropped.
    pub(crate) async fn started(&mut self) -> Option<String> {
        // The borrowed phase must be released before waiting any longer.
        let text = match self.0.wait_for(|phase| *phase != Phase::Running).await {
            Ok(phase) => match &*phase {
                Phase::LoggingOut(text) => Some(text.clone()),
                _ => Some(None),
            },
            Err(_) => None,
        };
        match text {
            Some(text) => text,
            None => pending().await,
        }
    }

    /// If shutdown has started, the Logout's Text; `None` if it hasn't.
    pub(crate) fn started_now(&self) -> Option<Option<String>> {
        match &*self.0.borrow() {
            Phase::Running => None,
            Phase::LoggingOut(text) => Some(text.clone()),
            Phase::Closing => Some(None),
        }
    }

    /// Completes once connections still open are to be closed without waiting.
    pub(crate) async fn closing(&mut self) {
        if self.0.wait_for(|phase| *phase == Phase::Closing).await.is_err() {
            pending().await
        }
    }
}
