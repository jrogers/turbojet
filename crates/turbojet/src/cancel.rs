//! Cancel on disconnect: telling the application to cancel a session's orders when the session
//! ends and the counterparty doesn't log back on in time.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use tokio::sync::Notify;
use tracing::info;

use crate::application::{Application, Disconnect, guarded};
use crate::store::SessionId;

/// Cancel on disconnect for a session: when it ends in a way `trigger` counts, the application is
/// told to cancel the session's orders, through
/// [`on_cancel_on_disconnect`](crate::Application::on_cancel_on_disconnect), unless the
/// counterparty logs back on within `grace`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelOnDisconnect {
    /// Which endings count.
    pub trigger: CancelTrigger,
    /// How long the counterparty has to log back on before the cancel; zero cancels at once.
    /// At most [`MAX_CANCEL_GRACE`].
    pub grace: Duration,
}

/// Which endings of a session count for [`CancelOnDisconnect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelTrigger {
    /// Endings without a Logout: the connection lost, a heartbeat timeout, an error.
    Disconnect,
    /// Those, and a Logout from either side.
    DisconnectOrLogout,
}

/// The longest grace period: an hour. Venues give seconds; a longer wait leaves orders working
/// for a counterparty that has gone.
pub const MAX_CANCEL_GRACE: Duration = Duration::from_secs(3600);

impl CancelTrigger {
    /// Whether a session that ended with `ended` counts. Our own shutdown, or the end of the
    /// schedule, never does: our side chose to end it.
    pub fn counts(self, ended: Disconnect) -> bool {
        match ended {
            Disconnect::ConnectionLost | Disconnect::HeartbeatTimeout | Disconnect::Error => true,
            Disconnect::Logout | Disconnect::CounterpartyLogout => self == Self::DisconnectOrLogout,
            Disconnect::Shutdown => false,
        }
    }

    /// The metrics label.
    fn label(self) -> &'static str {
        match self {
            Self::Disconnect => "disconnect",
            Self::DisconnectOrLogout => "disconnect_or_logout",
        }
    }
}

/// The countdowns under way, one per session at most, shared by the sessions of a registry.
///
/// It has no limit of its own: an entry is made only by a session that logged on, so there are at
/// most as many as the counterparties the application accepts, and dropping one would leave a
/// counterparty's orders working.
pub(crate) struct CancelTracker {
    pending: Mutex<HashMap<SessionId, Pending>>,
    /// Wakes the task driving the tracker when a countdown starts.
    pub(crate) wake: Arc<Notify>,
}

/// A countdown: the cancel due at `deadline`, unless the session logs on first.
struct Pending {
    deadline: Instant,
    ended: Disconnect,
    trigger: CancelTrigger,
    /// The session's own application: initiators sharing a registry may each have their own.
    app: Arc<dyn Application>,
}

impl CancelTracker {
    pub(crate) fn new() -> Self {
        Self { pending: Mutex::default(), wake: Arc::new(Notify::new()) }
    }

    /// Starts the countdown for `id`, which ended with `ended`, to cancel at `deadline`. A
    /// countdown already under way keeps its deadline: the first ending is when the counterparty
    /// went, however often it has failed to log back on since.
    pub(crate) fn start(
        &self,
        id: SessionId,
        ended: Disconnect,
        trigger: CancelTrigger,
        deadline: Instant,
        app: Arc<dyn Application>,
    ) {
        assert!(trigger.counts(ended), "only an ending the trigger counts starts a countdown");
        {
            let mut pending = self.lock();
            pending.entry(id).or_insert(Pending { deadline, ended, trigger, app });
            crate::telemetry::cancels_pending(pending.len());
        }
        self.wake.notify_one();
    }

    /// The session `id` has logged on: its countdown, if any, stops. A cancel in progress
    /// holds the lock, so this waits for it, and the logon's `on_logon` comes after it.
    pub(crate) fn logged_on(&self, id: &SessionId) {
        let mut pending = self.lock();
        if pending.remove(id).is_some() {
            info!(session = %id, "logged back on within the grace period; no cancel on disconnect");
            crate::telemetry::cancels_pending(pending.len());
        }
    }

    /// When the next countdown ends, if any is under way.
    pub(crate) fn next_deadline(&self) -> Option<Instant> {
        self.lock().values().map(|p| p.deadline).min()
    }

    /// Fires the countdowns that have ended by `now`.
    pub(crate) fn run_due(&self, now: Instant) {
        self.run(|p| p.deadline <= now);
    }

    /// Fires every countdown under way, as when shutting down.
    pub(crate) fn run_all(&self) {
        self.run(|_| true);
    }

    /// Fires the countdowns `due` picks, in deadline order, then by session for ties.
    fn run(&self, due: impl Fn(&Pending) -> bool) {
        // The lock is held while the callbacks run, so a logon of the same session waits in
        // `logged_on` until its cancel has returned: the cancel can't come after that logon's
        // `on_logon`. It is never held across an await, and callbacks must not block.
        let mut pending = self.lock();
        let mut fired: Vec<(SessionId, Pending)> = pending.extract_if(|_, p| due(p)).collect();
        debug_assert!(pending.values().all(|p| !due(p)), "every due countdown is taken");
        crate::telemetry::cancels_pending(pending.len());
        fired.sort_by(|(a, pa), (b, pb)| pa.deadline.cmp(&pb.deadline).then_with(|| order(a).cmp(&order(b))));
        for (id, Pending { ended, trigger, app, .. }) in fired {
            info!(session = %id, ?ended, "cancel on disconnect: the counterparty didn't log back on in time");
            crate::telemetry::cancel_on_disconnect(trigger.label());
            guarded("on_cancel_on_disconnect", || app.on_cancel_on_disconnect(&id, ended));
        }
        drop(pending);
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<SessionId, Pending>> {
        // Callbacks run under `guarded`, so a panicking one doesn't poison it.
        self.pending.lock().expect("cancel tracker lock poisoned")
    }
}

/// A session's place among cancels due at the same time: a fixed order, so runs repeat.
fn order(id: &SessionId) -> (&str, &str, &str) {
    (&id.begin_string, &id.sender_comp_id, &id.target_comp_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Records each cancel as "cancel <target> <ended>".
    #[derive(Default)]
    struct Recorder(Mutex<Vec<String>>);

    impl Application for Recorder {
        fn on_cancel_on_disconnect(&self, session: &SessionId, ended: Disconnect) {
            self.0.lock().unwrap().push(format!("cancel {} {ended:?}", session.target_comp_id));
        }
    }

    impl Recorder {
        fn take(&self) -> Vec<String> {
            std::mem::take(&mut self.0.lock().unwrap())
        }
    }

    fn id(target: &str) -> SessionId {
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: target.into() }
    }

    fn secs(n: u64) -> Duration {
        Duration::from_secs(n)
    }

    /// A tracker, its recording application and a starting time.
    fn tracker() -> (CancelTracker, Arc<Recorder>, Instant) {
        (CancelTracker::new(), Arc::default(), Instant::now())
    }

    fn start(tracker: &CancelTracker, app: &Arc<Recorder>, target: &str, ended: Disconnect, deadline: Instant) {
        tracker.start(id(target), ended, CancelTrigger::DisconnectOrLogout, deadline, app.clone());
    }

    #[test]
    fn fires_once_after_the_grace_period() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(5));
        tracker.run_due(t + secs(4));
        assert_eq!(app.take(), [] as [String; 0]);
        tracker.run_due(t + secs(5));
        assert_eq!(app.take(), ["cancel A ConnectionLost"]);
        tracker.run_due(t + secs(6));
        assert_eq!(app.take(), [] as [String; 0]);
        assert_eq!(tracker.next_deadline(), None);
    }

    #[test]
    fn logon_within_the_grace_period_stops_it() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(5));
        tracker.logged_on(&id("A"));
        tracker.run_due(t + secs(10));
        assert_eq!(app.take(), [] as [String; 0]);
        assert_eq!(tracker.next_deadline(), None);
    }

    /// A session that ends again before its countdown fires (a reconnect that failed) keeps the
    /// first deadline, and the first ending.
    #[test]
    fn a_second_end_keeps_the_first_deadline() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(5));
        start(&tracker, &app, "A", Disconnect::HeartbeatTimeout, t + secs(8));
        assert_eq!(tracker.next_deadline(), Some(t + secs(5)));
        tracker.run_due(t + secs(5));
        assert_eq!(app.take(), ["cancel A ConnectionLost"]);
        tracker.run_due(t + secs(8));
        assert_eq!(app.take(), [] as [String; 0]);
    }

    #[test]
    fn next_deadline_is_the_earliest() {
        let (tracker, app, t) = tracker();
        assert_eq!(tracker.next_deadline(), None);
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(9));
        start(&tracker, &app, "B", Disconnect::ConnectionLost, t + secs(3));
        start(&tracker, &app, "C", Disconnect::ConnectionLost, t + secs(6));
        assert_eq!(tracker.next_deadline(), Some(t + secs(3)));
        tracker.run_due(t + secs(3));
        assert_eq!(tracker.next_deadline(), Some(t + secs(6)));
    }

    /// At shutdown every countdown fires, however far off its deadline.
    #[test]
    fn run_all_fires_everything_pending() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(60));
        start(&tracker, &app, "B", Disconnect::Error, t + secs(3600));
        tracker.run_all();
        assert_eq!(app.take(), ["cancel A ConnectionLost", "cancel B Error"]);
        assert_eq!(tracker.next_deadline(), None);
    }

    #[test]
    fn the_callback_gets_the_reason() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "A", Disconnect::CounterpartyLogout, t);
        start(&tracker, &app, "B", Disconnect::HeartbeatTimeout, t + secs(1));
        tracker.run_due(t + secs(1));
        assert_eq!(app.take(), ["cancel A CounterpartyLogout", "cancel B HeartbeatTimeout"]);
    }

    /// Cancels due together fire by deadline, then by session, whatever order they started in.
    #[test]
    fn due_cancels_fire_in_deadline_order() {
        let (tracker, app, t) = tracker();
        start(&tracker, &app, "D", Disconnect::ConnectionLost, t + secs(2));
        start(&tracker, &app, "C", Disconnect::ConnectionLost, t + secs(1));
        start(&tracker, &app, "B", Disconnect::ConnectionLost, t + secs(2));
        start(&tracker, &app, "A", Disconnect::ConnectionLost, t + secs(3));
        tracker.run_due(t + secs(3));
        assert_eq!(
            app.take(),
            [
                "cancel C ConnectionLost",
                "cancel B ConnectionLost",
                "cancel D ConnectionLost",
                "cancel A ConnectionLost"
            ]
        );
    }

    /// Each countdown calls its own session's application.
    #[test]
    fn each_session_has_its_own_application() {
        let (tracker, first, t) = tracker();
        let second = Arc::<Recorder>::default();
        start(&tracker, &first, "A", Disconnect::ConnectionLost, t);
        start(&tracker, &second, "B", Disconnect::ConnectionLost, t);
        tracker.run_due(t);
        assert_eq!(first.take(), ["cancel A ConnectionLost"]);
        assert_eq!(second.take(), ["cancel B ConnectionLost"]);
    }

    /// A panicking callback is caught, and the cancels after it still fire.
    #[test]
    fn a_panicking_cancel_does_not_stop_the_rest() {
        struct Panics;
        impl Application for Panics {
            fn on_cancel_on_disconnect(&self, _session: &SessionId, _ended: Disconnect) {
                panic!("on_cancel_on_disconnect panicked");
            }
        }
        let (tracker, app, t) = tracker();
        tracker.start(id("A"), Disconnect::ConnectionLost, CancelTrigger::Disconnect, t, Arc::new(Panics));
        start(&tracker, &app, "B", Disconnect::ConnectionLost, t);
        tracker.run_due(t);
        assert_eq!(app.take(), ["cancel B ConnectionLost"]);
        // The lock isn't poisoned.
        assert_eq!(tracker.next_deadline(), None);
    }

    #[test]
    #[should_panic(expected = "only an ending the trigger counts")]
    fn an_ending_the_trigger_does_not_count_is_refused() {
        let (tracker, app, t) = tracker();
        tracker.start(id("A"), Disconnect::Logout, CancelTrigger::Disconnect, t, app);
    }

    #[test]
    fn which_endings_count() {
        use Disconnect::*;
        let all = [Logout, CounterpartyLogout, ConnectionLost, HeartbeatTimeout, Error, Shutdown];
        let counted = |trigger: CancelTrigger| all.into_iter().filter(|&r| trigger.counts(r)).collect::<Vec<_>>();
        assert_eq!(counted(CancelTrigger::Disconnect), [ConnectionLost, HeartbeatTimeout, Error]);
        assert_eq!(
            counted(CancelTrigger::DisconnectOrLogout),
            [Logout, CounterpartyLogout, ConnectionLost, HeartbeatTimeout, Error]
        );
    }
}
