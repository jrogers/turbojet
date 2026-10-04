//! Settings an [`Acceptor`](crate::Acceptor) gives each counterparty, decided at Logon.
//!
//! An acceptor learns who it's talking to only from the Logon's SenderCompID, so a
//! [`Counterparties`] resolver runs then, and the session uses the [`Counterparty`] it returns
//! from there on. What was decided before Logon stays the acceptor's: the logon timeout, the
//! clock and the send queue, and the checks on the Logon itself (its SendingTime against
//! [`max_latency`](SessionConfig::max_latency), its data fields).

use std::collections::HashMap;
use std::fmt;
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

    /// Why these settings can't be used on an acceptor configured with `base`, if they can't:
    /// the checks a session makes on what a resolver returns, for a resolver to make sooner
    /// (when it loads its settings, say).
    ///
    /// # Errors
    ///
    /// The first problem found: a field in use before Logon that differs from `base`'s, the
    /// config failing [`SessionConfig::check`], or a heartbeat range outside
    /// [`DEFAULT_HEARTBEAT`](Self::DEFAULT_HEARTBEAT).
    pub fn check(&self, base: &SessionConfig) -> Result<(), String> {
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

/// Changes a counterparty's settings from the acceptor's.
type Adjust = Box<dyn Fn(&mut Counterparty) + Send + Sync>;

/// [`Counterparties`] by CompID: each listed counterparty gets the acceptor's settings changed
/// its own way, and others get the acceptor's as they are, or are refused.
///
/// ```
/// # use std::sync::Arc;
/// # use turbojet::{Acceptor, MemoryStorage, SessionConfig};
/// # use turbojet::CounterpartyMap;
/// # struct App;
/// # impl turbojet::Application for App {}
/// let counterparties = CounterpartyMap::new()
///     .with("BROKER", |c| c.config.schedule = Some("daily 08:00-17:00".parse().unwrap()))
///     .with("FUND", |c| c.require_client_certificate = true)
///     .refuse_unknown();
/// let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "VENUE"), Arc::new(MemoryStorage::new()), Arc::new(App))
///     .with_counterparties(Arc::new(counterparties));
/// ```
pub struct CounterpartyMap {
    entries: HashMap<String, Adjust>,
    refuse_unknown: bool,
}

impl CounterpartyMap {
    /// An empty map, which gives every counterparty the acceptor's settings.
    pub fn new() -> Self {
        Self { entries: HashMap::new(), refuse_unknown: false }
    }

    /// Gives counterparty `comp_id` the acceptor's settings changed by `adjust`, replacing any
    /// earlier entry for it.
    #[must_use]
    pub fn with(
        mut self,
        comp_id: impl Into<String>,
        adjust: impl Fn(&mut Counterparty) + Send + Sync + 'static,
    ) -> Self {
        self.entries.insert(comp_id.into(), Box::new(adjust));
        self
    }

    /// Refuses counterparties that aren't listed, rather than give them the acceptor's settings.
    #[must_use]
    pub fn refuse_unknown(mut self) -> Self {
        self.refuse_unknown = true;
        self
    }
}

impl Default for CounterpartyMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CounterpartyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comp_ids: Vec<_> = self.entries.keys().collect();
        comp_ids.sort();
        f.debug_struct("CounterpartyMap")
            .field("comp_ids", &comp_ids)
            .field("refuse_unknown", &self.refuse_unknown)
            .finish()
    }
}

impl Counterparties for CounterpartyMap {
    fn resolve(
        &self,
        base: &SessionConfig,
        id: &SessionId,
        _: &Message,
        _: &ConnectionInfo,
    ) -> Result<Counterparty, String> {
        let mut counterparty = Counterparty::new(base.clone());
        match self.entries.get(&id.target_comp_id) {
            Some(adjust) => adjust(&mut counterparty),
            None if self.refuse_unknown => return Err(format!("unknown counterparty '{}'", id.target_comp_id)),
            None => {}
        }
        Ok(counterparty)
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

    fn resolve(map: &CounterpartyMap, comp_id: &str) -> Result<Counterparty, String> {
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GATEWAY".into(),
            target_comp_id: comp_id.into(),
        };
        map.resolve(&base(), &id, &Message::default(), &ConnectionInfo::default())
    }

    #[test]
    fn a_listed_counterparty_gets_its_own_settings() {
        let map = CounterpartyMap::new()
            .with("BROKER", |c| c.config.max_latency = None)
            .with("FUND", |c| c.require_client_certificate = true);
        let broker = resolve(&map, "BROKER").unwrap();
        assert_eq!((broker.config.max_latency, broker.require_client_certificate), (None, false));
        let fund = resolve(&map, "FUND").unwrap();
        assert_eq!((fund.config.max_latency, fund.require_client_certificate), (base().max_latency, true));
    }

    #[test]
    fn an_unlisted_counterparty_gets_the_acceptors_settings_or_is_refused() {
        let map = CounterpartyMap::new().with("BROKER", |c| c.config.max_latency = None);
        let other = resolve(&map, "OTHER").unwrap();
        assert_eq!(other.config.max_latency, base().max_latency);
        assert_eq!(other.heartbeat, Counterparty::DEFAULT_HEARTBEAT);
        let map = map.refuse_unknown();
        assert_eq!(resolve(&map, "OTHER").unwrap_err(), "unknown counterparty 'OTHER'");
        assert!(resolve(&map, "BROKER").is_ok());
    }

    #[test]
    fn a_later_entry_replaces_an_earlier_one() {
        let map = CounterpartyMap::new()
            .with("BROKER", |c| c.require_client_certificate = true)
            .with("BROKER", |c| c.config.max_latency = None);
        let broker = resolve(&map, "BROKER").unwrap();
        assert_eq!((broker.config.max_latency, broker.require_client_certificate), (None, false));
    }

    #[test]
    fn debug_lists_the_comp_ids() {
        let map = CounterpartyMap::new().with("B", |_| {}).with("A", |_| {}).refuse_unknown();
        assert_eq!(format!("{map:?}"), r#"CounterpartyMap { comp_ids: ["A", "B"], refuse_unknown: true }"#);
    }
}
