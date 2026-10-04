//! Cancel on disconnect: telling the application to cancel a session's orders when the session
//! ends and the counterparty doesn't log back on in time.

use std::time::Duration;

use crate::application::Disconnect;

/// Cancel on disconnect for a session: when it ends in a way `trigger` counts, the application is
/// told to cancel the session's orders unless the counterparty logs back on within `grace`.
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
