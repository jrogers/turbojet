//! Settings an [`Acceptor`](crate::Acceptor) gives each counterparty, decided at Logon.
//!
//! An acceptor learns who it's talking to only from the Logon's SenderCompID, so a
//! [`Counterparties`] resolver runs then, and the session uses the [`Counterparty`] it returns
//! from there on. What was decided before Logon stays the acceptor's: the logon timeout, the
//! clock and the send queue, and the checks on the Logon itself (its SendingTime against
//! [`max_latency`](SessionConfig::max_latency), its data fields).

use std::ops::RangeInclusive;
use std::time::Duration;

use crate::message::Message;
use crate::peer::ConnectionInfo;
use crate::session::SessionConfig;
use crate::store::SessionId;

/// Decides, at Logon, whether a counterparty may log on and with which settings. Set one with
/// [`Acceptor::with_counterparties`](crate::Acceptor::with_counterparties).
pub trait Counterparties: Send + Sync {
    /// Whether `id`'s counterparty may log on with `logon` over `connection`, and its settings,
    /// starting from the acceptor's `base`: `Counterparty::new(base.clone())` and changes to it.
    /// `Err` refuses the logon with that reason, which is logged; the connection is closed
    /// without a reply, as when [`Application::verify_logon`](crate::Application::verify_logon)
    /// refuses. It runs before `verify_logon`, and a panic refuses the logon.
    ///
    /// # Errors
    ///
    /// The reason the counterparty may not log on.
    fn resolve(
        &self,
        base: &SessionConfig,
        id: &SessionId,
        logon: &Message,
        connection: &ConnectionInfo,
    ) -> Result<Counterparty, String>;
}

/// One counterparty's settings: see [`Counterparties`].
#[derive(Debug, Clone)]
pub struct Counterparty {
    /// The session's configuration. It must keep the acceptor's `begin_string`,
    /// `sender_comp_id`, `clock` (or a clone of it), `logon_timeout` and `send_queue`, which
    /// are in use before Logon, and pass [`SessionConfig::check`]; otherwise the logon is
    /// refused.
    pub config: SessionConfig,
    /// The HeartBtInt(108) the counterparty may ask for, within
    /// [`DEFAULT_HEARTBEAT`](Self::DEFAULT_HEARTBEAT); a Logon outside it is refused.
    pub heartbeat: RangeInclusive<Duration>,
    /// Refuse the logon unless the connection presented a TLS client certificate.
    pub require_client_certificate: bool,
}

impl Counterparty {
    /// The HeartBtInt range an acceptor allows by default, and the widest it may allow: 1 second
    /// to an hour.
    pub const DEFAULT_HEARTBEAT: RangeInclusive<Duration> =
        RangeInclusive::new(Duration::from_secs(1), Duration::from_secs(3600));

    /// `config`, any HeartBtInt in [`DEFAULT_HEARTBEAT`](Self::DEFAULT_HEARTBEAT), and no
    /// client certificate required.
    pub fn new(config: SessionConfig) -> Self {
        Self { config, heartbeat: Self::DEFAULT_HEARTBEAT, require_client_certificate: false }
    }

    /// Why these settings can't be used on an acceptor configured with `base`, if they can't.
    pub(crate) fn check(&self, base: &SessionConfig) -> Result<(), String> {
        let config = &self.config;
        let fixed = [
            ("begin_string", config.begin_string == base.begin_string),
            ("sender_comp_id", config.sender_comp_id == base.sender_comp_id),
            ("clock", config.clock.same_as(&base.clock)),
            ("logon_timeout", config.logon_timeout == base.logon_timeout),
            ("send_queue", config.send_queue == base.send_queue),
        ];
        if let Some((field, _)) = fixed.iter().find(|(_, same)| !same) {
            return Err(format!("the counterparty's {field} differs from the acceptor's"));
        }
        config.check()?;
        let (low, high) = (*self.heartbeat.start(), *self.heartbeat.end());
        let widest = Self::DEFAULT_HEARTBEAT;
        if low > high || low < *widest.start() || high > *widest.end() {
            return Err(format!("heartbeat range {low:?}..={high:?} isn't within {widest:?}"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedule::Clock;
    use crate::throttle::RateLimit;

    fn base() -> SessionConfig {
        SessionConfig::new("FIX.4.4", "GATEWAY")
    }

    #[test]
    fn the_acceptors_own_settings_are_valid() {
        let base = base();
        assert_eq!(Counterparty::new(base.clone()).check(&base), Ok(()));
        let mut changed = Counterparty::new(base.clone());
        changed.config.max_latency = None;
        changed.config.logout_timeout = Duration::from_secs(1);
        changed.heartbeat = Duration::from_secs(5)..=Duration::from_secs(60);
        changed.require_client_certificate = true;
        assert_eq!(changed.check(&base), Ok(()));
    }

    #[test]
    fn settings_in_use_before_logon_stay_the_acceptors() {
        let base = base();
        type Change = fn(&mut SessionConfig);
        let changes: [(&str, Change); 5] = [
            ("begin_string", |c| c.begin_string = "FIX.4.2".into()),
            ("sender_comp_id", |c| c.sender_comp_id = "OTHER".into()),
            ("clock", |c| c.clock = Clock::system()),
            ("logon_timeout", |c| c.logon_timeout = Duration::from_secs(1)),
            ("send_queue", |c| c.send_queue = 1),
        ];
        for (field, change) in changes {
            let mut counterparty = Counterparty::new(base.clone());
            change(&mut counterparty.config);
            let error = format!("the counterparty's {field} differs from the acceptor's");
            assert_eq!(counterparty.check(&base), Err(error), "{field}");
        }
    }

    #[test]
    fn an_invalid_config_is_refused() {
        let base = base();
        let mut counterparty = Counterparty::new(base.clone());
        counterparty.config.outbound_limit = Some(RateLimit { messages: 0, per: Duration::from_secs(1) });
        assert!(counterparty.check(&base).unwrap_err().starts_with("outbound_limit"));
    }

    #[test]
    fn the_heartbeat_range_is_within_the_default() {
        let base = base();
        let secs = Duration::from_secs;
        for (range, ok) in [
            (secs(5)..=secs(60), true),
            (secs(1)..=secs(3600), true),
            (secs(10)..=secs(10), true),
            (secs(10)..=secs(5), false),
            (secs(0)..=secs(30), false),
            (secs(30)..=secs(3601), false),
        ] {
            let mut counterparty = Counterparty::new(base.clone());
            counterparty.heartbeat = range.clone();
            assert_eq!(counterparty.check(&base).is_ok(), ok, "{range:?}");
        }
    }
}
