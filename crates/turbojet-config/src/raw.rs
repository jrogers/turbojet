//! The file as written: TOML deserialized as is, before any value is converted or checked.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;

/// The whole file.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawFile {
    pub acceptor: Option<RawAcceptor>,
    #[serde(default)]
    pub store: BTreeMap<String, RawStore>,
    #[serde(default)]
    pub defaults: RawSettings,
    #[serde(default)]
    pub counterparty: BTreeMap<String, RawSettings>,
    #[serde(default)]
    pub initiator: BTreeMap<String, RawInitiator>,
}

/// `[initiator.NAME]`: the initiator's own keys, and session keys over `[defaults]`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RawInitiator {
    pub own: RawInitiatorKeys,
    pub settings: RawSettings,
}

/// The keys only an initiator has.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawInitiatorKeys {
    pub begin_string: Option<String>,
    pub sender_comp_id: Option<String>,
    pub target_comp_id: String,
    pub connect: Vec<String>,
    pub tls: Option<RawClientTls>,
    pub heartbeat_interval: Option<String>,
    pub reset_on_logon: Option<bool>,
    pub next_expected_msg_seq_num: Option<bool>,
    pub username: Option<String>,
    pub password_env: Option<String>,
    pub connect_timeout: Option<String>,
    pub local_address: Option<String>,
    pub logon_timeout: Option<String>,
    pub send_queue: Option<usize>,
    pub reconnect: Option<RawReconnect>,
}

/// The names of [`RawInitiatorKeys`]' fields: an initiator's table is split by them, since
/// serde can't flatten into a struct that denies unknown fields.
const INITIATOR_KEYS: &[&str] = &[
    "begin_string",
    "sender_comp_id",
    "target_comp_id",
    "connect",
    "tls",
    "heartbeat_interval",
    "reset_on_logon",
    "next_expected_msg_seq_num",
    "username",
    "password_env",
    "connect_timeout",
    "local_address",
    "logon_timeout",
    "send_queue",
    "reconnect",
];

impl<'de> Deserialize<'de> for RawInitiator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let table = toml::Table::deserialize(deserializer)?;
        let (own, settings): (toml::Table, toml::Table) =
            table.into_iter().partition(|(key, _)| INITIATOR_KEYS.contains(&key.as_str()));
        let own = RawInitiatorKeys::deserialize(toml::Value::Table(own)).map_err(D::Error::custom)?;
        let settings = RawSettings::deserialize(toml::Value::Table(settings)).map_err(D::Error::custom)?;
        Ok(Self { own, settings })
    }
}

/// An initiator's `tls`: the CAs its counterparty's certificate must chain to, and its own
/// certificate if the counterparty asks for one.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawClientTls {
    pub ca: PathBuf,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
    pub server_name: Option<String>,
}

/// `reconnect = { initial = "1s", max = "60s", multiplier = 2, jitter = true }`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawReconnect {
    pub initial: String,
    pub max: String,
    pub multiplier: Option<u32>,
    pub jitter: Option<bool>,
}

/// `[acceptor]`: what's fixed until a restart, and how unlisted counterparties are treated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawAcceptor {
    pub begin_string: String,
    pub sender_comp_id: String,
    pub listen: String,
    #[serde(default)]
    pub unknown: Unknown,
    pub logon_timeout: Option<String>,
    pub send_queue: Option<usize>,
    pub max_connections: Option<usize>,
    pub max_connections_per_ip: Option<usize>,
    pub tls: Option<RawTls>,
}

/// What happens to a counterparty the file doesn't list.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Unknown {
    /// Its logon is refused.
    #[default]
    Refuse,
    /// It logs on with `[defaults]`.
    Admit,
}

/// `[acceptor.tls]`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawTls {
    pub cert: PathBuf,
    pub key: PathBuf,
    pub client_ca: Option<PathBuf>,
    #[serde(default)]
    pub client_certificate: ClientCertificate,
}

/// Whether a client must present a certificate, given `client_ca`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ClientCertificate {
    #[default]
    Optional,
    Required,
}

/// `[store.NAME]`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub(crate) enum RawStore {
    Memory,
    Disk {
        dir: PathBuf,
        #[serde(default = "yes")]
        fsync: bool,
    },
}

fn yes() -> bool {
    true
}

/// `[defaults]` and `[counterparty.COMPID]`: every key optional, a counterparty's taking
/// precedence over the defaults' (see [`RawSettings::or`]).
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawSettings {
    pub store: Option<String>,
    pub logout_timeout: Option<String>,
    pub schedule: Option<String>,
    pub holidays: Option<PathBuf>,
    pub max_latency: Option<String>,
    pub check_orig_sending_time: Option<bool>,
    pub check_header_order: Option<bool>,
    pub timestamp_precision: Option<RawPrecision>,
    pub data_fields: Option<Vec<[u32; 2]>>,
    pub outbound_limit: Option<String>,
    pub inbound_limit: Option<String>,
    pub over_limit: Option<OverLimit>,
    pub appl_versions: Option<Vec<RawApplVersion>>,
    pub dictionary: Option<PathBuf>,
    pub latency_metrics: Option<bool>,
    pub heartbeat: Option<RawHeartbeat>,
    pub require_client_certificate: Option<bool>,
    pub cancel_on_disconnect: Option<RawCancelTrigger>,
    pub cancel_grace: Option<String>,
    pub resend_request_chunk: Option<u64>,
    pub max_sessions_per_counterparty: Option<usize>,
}

/// Field by field, `self`'s keys over `defaults`'.
macro_rules! or_fields {
    ($self:ident, $defaults:ident, $($field:ident),* $(,)?) => {
        RawSettings { $($field: $self.$field.clone().or_else(|| $defaults.$field.clone())),* }
    };
}

impl RawSettings {
    /// These settings over `defaults`: each key set here wins, whole (a list isn't merged).
    pub fn or(&self, defaults: &RawSettings) -> RawSettings {
        or_fields!(
            self,
            defaults,
            store,
            logout_timeout,
            schedule,
            holidays,
            max_latency,
            check_orig_sending_time,
            check_header_order,
            timestamp_precision,
            data_fields,
            outbound_limit,
            inbound_limit,
            over_limit,
            appl_versions,
            dictionary,
            latency_metrics,
            heartbeat,
            require_client_certificate,
            cancel_on_disconnect,
            cancel_grace,
            resend_request_chunk,
            max_sessions_per_counterparty,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RawPrecision {
    Seconds,
    Millis,
    Micros,
    Nanos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OverLimit {
    Delay,
    Reject,
}

/// `cancel_on_disconnect`: which endings of a session count, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RawCancelTrigger {
    /// No ending counts.
    Off,
    /// Endings without a Logout.
    Disconnect,
    /// Endings with or without a Logout.
    DisconnectOrLogout,
}

/// An application version: its ApplVerID code, or the code and a dictionary to check its
/// messages against.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawApplVersion {
    Code(String),
    WithDictionary { id: String, dictionary: PathBuf },
}

/// `heartbeat = { min = "1s", max = "60s" }`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawHeartbeat {
    pub min: String,
    pub max: String,
}

/// Parses a duration: a whole number and a unit, `ms`, `s`, `m` or `h` (`500ms`, `120s`).
pub(crate) fn parse_duration(s: &str) -> Result<Duration, String> {
    let invalid = || format!("invalid duration '{s}' (expected e.g. 500ms, 30s, 5m or 1h)");
    let digits_end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (count, unit) = s.split_at(digits_end);
    if count.is_empty() {
        return Err(invalid());
    }
    // All digits, so parsing fails only on overflow.
    let count: u64 = count.parse().map_err(|_| format!("duration '{s}' is too long"))?;
    let unit_ms: u64 = match unit {
        "ms" => 1,
        "s" => 1_000,
        "m" => 60_000,
        "h" => 3_600_000,
        _ => return Err(invalid()),
    };
    let ms = count.checked_mul(unit_ms).ok_or_else(|| format!("duration '{s}' is too long"))?;
    Ok(Duration::from_millis(ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<RawFile, String> {
        toml::from_str(text).map_err(|e| e.to_string())
    }

    const MINIMAL: &str = r#"
        [acceptor]
        begin_string = "FIX.4.4"
        sender_comp_id = "VENUE"
        listen = "0.0.0.0:9876"
    "#;

    #[test]
    fn a_minimal_file_lists_nothing_and_refuses_the_unknown() {
        let file = parse(MINIMAL).unwrap();
        assert_eq!(file.acceptor.as_ref().unwrap().unknown, Unknown::Refuse);
        assert!(file.store.is_empty() && file.counterparty.is_empty());
        assert_eq!(file.defaults, RawSettings::default());
    }

    #[test]
    fn every_key_parses() {
        let text = format!(
            "{MINIMAL}{}",
            r#"
            unknown = "admit"
            logon_timeout = "5s"
            send_queue = 100
            max_connections = 10
            max_connections_per_ip = 2
            tls = { cert = "c.pem", key = "k.pem", client_ca = "ca.pem", client_certificate = "required" }

            [store.main]
            kind = "disk"
            dir = "./store"

            [store.scratch]
            kind = "memory"

            [defaults]
            store = "main"
            logout_timeout = "2s"
            schedule = "daily 08:00-17:00"
            holidays = "holidays.txt"
            max_latency = "off"
            check_orig_sending_time = false
            check_header_order = false
            timestamp_precision = "micros"
            data_fields = [[5000, 5001]]
            outbound_limit = "100/1s"
            inbound_limit = "50/1s"
            over_limit = "reject"
            dictionary = "FIX44.xml"
            latency_metrics = true
            heartbeat = { min = "10s", max = "60s" }
            require_client_certificate = true
            cancel_on_disconnect = "disconnect_or_logout"
            cancel_grace = "5s"

            [counterparty.FUND]
            appl_versions = ["9", { id = "8", dictionary = "FIX50SP1.xml" }]
            "#
        );
        let file = parse(&text).unwrap();
        assert_eq!(file.acceptor.as_ref().unwrap().unknown, Unknown::Admit);
        assert_eq!(file.acceptor.unwrap().tls.unwrap().client_certificate, ClientCertificate::Required);
        assert_eq!(file.store["main"], RawStore::Disk { dir: "./store".into(), fsync: true });
        assert_eq!(file.store["scratch"], RawStore::Memory);
        assert_eq!(file.defaults.timestamp_precision, Some(RawPrecision::Micros));
        assert_eq!(file.defaults.data_fields, Some(vec![[5000, 5001]]));
        assert_eq!(file.defaults.cancel_on_disconnect, Some(RawCancelTrigger::DisconnectOrLogout));
        assert_eq!(file.defaults.cancel_grace.as_deref(), Some("5s"));
        assert_eq!(
            file.counterparty["FUND"].appl_versions,
            Some(vec![
                RawApplVersion::Code("9".into()),
                RawApplVersion::WithDictionary { id: "8".into(), dictionary: "FIX50SP1.xml".into() }
            ])
        );
    }

    #[test]
    fn unknown_keys_are_errors() {
        for (extra, key) in [
            ("\nlistn = \"x\"", "listn"),
            ("\n[defaults]\nmax_latnecy = \"1s\"", "max_latnecy"),
            ("\n[counterparty.A]\nstroe = \"x\"", "stroe"),
            ("\n[store.x]\nkind = \"disk\"\ndir = \"d\"\nsync = true", "sync"),
            ("\n[acceptors]", "acceptors"),
        ] {
            let error = parse(&format!("{MINIMAL}{extra}")).unwrap_err();
            assert!(error.contains(key), "{key}: {error}");
        }
    }

    #[test]
    fn values_of_the_wrong_kind_are_errors() {
        for extra in [
            "\n[defaults]\ncheck_header_order = \"yes\"",
            "\n[defaults]\ntimestamp_precision = \"picos\"",
            "\n[defaults]\ncancel_on_disconnect = true",
            "\nunknown = \"maybe\"",
            "\n[store.x]\nkind = \"tape\"",
        ] {
            assert!(parse(&format!("{MINIMAL}{extra}")).is_err(), "{extra}");
        }
    }

    #[test]
    fn cancel_on_disconnect_is_off_disconnect_or_logout() {
        for (value, trigger) in [
            ("off", RawCancelTrigger::Off),
            ("disconnect", RawCancelTrigger::Disconnect),
            ("disconnect_or_logout", RawCancelTrigger::DisconnectOrLogout),
        ] {
            let file = parse(&format!("{MINIMAL}\n[defaults]\ncancel_on_disconnect = \"{value}\"")).unwrap();
            assert_eq!(file.defaults.cancel_on_disconnect, Some(trigger), "{value}");
        }
        let error = parse(&format!("{MINIMAL}\n[defaults]\ncancel_on_disconnect = \"always\"")).unwrap_err();
        assert!(error.contains("cancel_on_disconnect") && error.contains("always"), "{error}");
    }

    #[test]
    fn a_counterpartys_keys_win_over_the_defaults() {
        let defaults = RawSettings {
            store: Some("main".into()),
            max_latency: Some("120s".into()),
            data_fields: Some(vec![[1, 2], [3, 4]]),
            ..RawSettings::default()
        };
        let own =
            RawSettings { max_latency: Some("off".into()), data_fields: Some(vec![[5, 6]]), ..RawSettings::default() };
        let merged = own.or(&defaults);
        assert_eq!(merged.store.as_deref(), Some("main"));
        assert_eq!(merged.max_latency.as_deref(), Some("off"));
        assert_eq!(merged.data_fields, Some(vec![[5, 6]]), "a list is replaced, not merged");
    }

    #[test]
    fn durations() {
        assert_eq!(parse_duration("500ms"), Ok(Duration::from_millis(500)));
        assert_eq!(parse_duration("120s"), Ok(Duration::from_secs(120)));
        assert_eq!(parse_duration("5m"), Ok(Duration::from_secs(300)));
        assert_eq!(parse_duration("1h"), Ok(Duration::from_secs(3600)));
        assert_eq!(parse_duration("0s"), Ok(Duration::ZERO));
        for bad in ["", "s", "10", "1.5s", "-1s", "10 s", "1d", "99999999999999999999s"] {
            assert!(parse_duration(bad).is_err(), "{bad}");
        }
        assert!(parse_duration("18446744073709551615h").unwrap_err().contains("too long"));
    }
}
