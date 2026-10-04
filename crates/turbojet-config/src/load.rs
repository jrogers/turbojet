//! Turning the file as written into what an acceptor runs on, checking every value on the way:
//! a file that loads has nothing left to fail at a counterparty's Logon.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
#[cfg(feature = "validation")]
use std::path::PathBuf;
use std::sync::Arc;

use turbojet::fields::{ApplVerId, FromFix, Precision};
use turbojet::{
    Clock, Counterparty, DiskStorage, HolidayCalendar, InboundLimit, MemoryStorage, RateLimit, SessionConfig,
    SessionSchedule, SessionStorage,
};

use crate::Error;
#[cfg(feature = "tls")]
use crate::raw::ClientCertificate;
use crate::raw::{
    OverLimit, RawAcceptor, RawApplVersion, RawFile, RawPrecision, RawSettings, RawStore, parse_duration,
};

/// The store every file has, without defining it.
pub(crate) const MEMORY: &str = "memory";

/// A counterparty's settings, and the name of the store its sessions are kept in.
#[derive(Clone)]
pub(crate) struct Resolved {
    pub counterparty: Counterparty,
    pub store: String,
}

/// A loaded file: everything converted and checked.
pub(crate) struct Loaded {
    pub raw: RawFile,
    /// The acceptor's configuration: `[acceptor]` and `[defaults]`.
    pub base: SessionConfig,
    /// Each listed counterparty's settings, by CompID.
    pub listed: HashMap<String, Resolved>,
    /// An unlisted counterparty's settings, `[defaults]` alone; it logs on with them only under
    /// `unknown = "admit"`.
    pub unlisted: Resolved,
    /// Every store, by name: those the file defines, those registered in code, and `memory`.
    pub stores: HashMap<String, Arc<dyn SessionStorage>>,
    /// `[acceptor.tls]`, read and checked.
    #[cfg(feature = "tls")]
    pub tls: Tls,
}

impl Loaded {
    /// The settings for counterparty `comp_id`, listed or not.
    pub fn settings(&self, comp_id: &str) -> &Resolved {
        self.listed.get(comp_id).unwrap_or(&self.unlisted)
    }

    /// The store counterparty `comp_id`'s sessions are kept in.
    pub fn store_for(&self, comp_id: &str) -> &Arc<dyn SessionStorage> {
        let name = &self.settings(comp_id).store;
        self.stores.get(name).expect("loading checked that every store named exists")
    }
}

/// What loading needs besides the file.
pub(crate) struct Context<'a> {
    /// The directory relative paths in the file are relative to.
    pub dir: &'a Path,
    /// The clock every session uses, kept across reloads: the acceptor compares by identity.
    pub clock: &'a Clock,
    /// Stores registered in code, by name.
    pub registered: &'a HashMap<String, Arc<dyn SessionStorage>>,
    /// The stores of the file loaded before, to keep rather than open again.
    pub previous: Option<&'a HashMap<String, Arc<dyn SessionStorage>>>,
    /// The `[store]` sections of the file loaded before.
    pub previous_defined: Option<&'a BTreeMap<String, RawStore>>,
}

/// Parses `text` and loads it.
pub(crate) fn parse(text: &str, context: &Context<'_>) -> Result<Loaded, Error> {
    let raw: RawFile = toml::from_str(text).map_err(Error::file)?;
    load(raw, context)
}

fn load(raw: RawFile, context: &Context<'_>) -> Result<Loaded, Error> {
    let fixed = fixed(&raw.acceptor, context.clock)?;
    #[cfg(feature = "tls")]
    let tls = tls(&raw, context.dir)?;
    #[cfg(not(feature = "tls"))]
    refuse_tls(&raw)?;
    let stores = stores(&raw.store, context)?;
    let mut dictionaries = Dictionaries::default();
    let mut base = fixed.clone();
    apply(&raw.defaults, &mut base, "defaults", context.dir, &mut dictionaries)?;
    base.check().map_err(|e| Error::at("defaults", "settings", e))?;
    let unlisted = resolve(&raw.defaults, &fixed, "defaults", context, &stores, &mut dictionaries)?;
    let mut listed = HashMap::new();
    for (comp_id, settings) in &raw.counterparty {
        let section = format!("counterparty {comp_id}");
        let merged = settings.or(&raw.defaults);
        let resolved = resolve(&merged, &fixed, &section, context, &stores, &mut dictionaries)?;
        listed.insert(comp_id.clone(), resolved);
    }
    Ok(Loaded {
        raw,
        base,
        listed,
        unlisted,
        stores,
        #[cfg(feature = "tls")]
        tls,
    })
}

/// The acceptor's certificate and the client CAs it trusts, if it serves TLS.
#[cfg(feature = "tls")]
pub(crate) type Tls = Option<(turbojet::tls::Identity, turbojet::tls::ClientTrust)>;

/// `[acceptor.tls]`'s certificate, key and client CAs, read and checked.
#[cfg(feature = "tls")]
fn tls(raw: &RawFile, dir: &Path) -> Result<Tls, Error> {
    use turbojet::tls::{ClientTrust, Identity, ServerTls, Trust};
    let Some(tls) = &raw.acceptor.tls else { return Ok(None) };
    let at = |path: &Path, e: std::io::Error| Error::at("acceptor", "tls", format!("{}: {e}", path.display()));
    let (cert, key) = (dir.join(&tls.cert), dir.join(&tls.key));
    let identity = Identity::from_pem_files(&cert, &key).map_err(|e| at(&cert, e))?;
    let client_trust = match &tls.client_ca {
        None if tls.client_certificate == ClientCertificate::Required => {
            return Err(Error::at("acceptor", "tls", "client_certificate = \"required\" needs a client_ca"));
        }
        None => ClientTrust::None,
        Some(ca) => {
            let ca = dir.join(ca);
            let trust = Trust::from_pem_files(&ca).map_err(|e| at(&ca, e))?;
            match tls.client_certificate {
                ClientCertificate::Optional => ClientTrust::Optional(trust),
                ClientCertificate::Required => ClientTrust::Required(trust),
            }
        }
    };
    // Checks what only a server checks, such as the key matching the certificate.
    ServerTls::new(identity.clone(), client_trust.clone()).map_err(|e| Error::at("acceptor", "tls", e))?;
    Ok(Some((identity, client_trust)))
}

/// Without the tls feature, `[acceptor.tls]` can't be served.
#[cfg(not(feature = "tls"))]
fn refuse_tls(raw: &RawFile) -> Result<(), Error> {
    match raw.acceptor.tls {
        Some(_) => Err(Error::at("acceptor", "tls", "needs turbojet-config's tls feature")),
        None => Ok(()),
    }
}

/// The acceptor's settings fixed until a restart.
fn fixed(acceptor: &RawAcceptor, clock: &Clock) -> Result<SessionConfig, Error> {
    let at = |key, e| Error::at("acceptor", key, e);
    let mut config = SessionConfig::new(&acceptor.begin_string, &acceptor.sender_comp_id);
    config.clock = clock.clone();
    if let Some(timeout) = &acceptor.logon_timeout {
        config.logon_timeout = parse_duration(timeout).map_err(|e| at("logon_timeout", e))?;
    }
    if let Some(queue) = acceptor.send_queue {
        if queue == 0 {
            return Err(at("send_queue", "must be at least 1".into()));
        }
        config.send_queue = queue;
    }
    for (key, limit) in
        [("max_connections", acceptor.max_connections), ("max_connections_per_ip", acceptor.max_connections_per_ip)]
    {
        if limit == Some(0) {
            return Err(at(key, "must be at least 1".into()));
        }
    }
    Ok(config)
}

/// One counterparty's settings, from `settings` (already merged over the defaults).
fn resolve(
    settings: &RawSettings,
    fixed: &SessionConfig,
    section: &str,
    context: &Context<'_>,
    stores: &HashMap<String, Arc<dyn SessionStorage>>,
    dictionaries: &mut Dictionaries,
) -> Result<Resolved, Error> {
    let mut config = fixed.clone();
    apply(settings, &mut config, section, context.dir, dictionaries)?;
    let mut counterparty = Counterparty::new(config);
    if let Some(heartbeat) = &settings.heartbeat {
        let at = |e| Error::at(section, "heartbeat", e);
        counterparty.heartbeat =
            parse_duration(&heartbeat.min).map_err(at)?..=parse_duration(&heartbeat.max).map_err(at)?;
    }
    if let Some(required) = settings.require_client_certificate {
        counterparty.require_client_certificate = required;
    }
    counterparty.check(fixed).map_err(|e| Error::at(section, "settings", e))?;
    let store = settings.store.clone().unwrap_or_else(|| MEMORY.into());
    if !stores.contains_key(&store) {
        return Err(Error::at(section, "store", format!("no store named '{store}'")));
    }
    Ok(Resolved { counterparty, store })
}

/// Applies `settings` to `config`.
fn apply(
    settings: &RawSettings,
    config: &mut SessionConfig,
    section: &str,
    dir: &Path,
    dictionaries: &mut Dictionaries,
) -> Result<(), Error> {
    let at = |key: &str, e: String| Error::at(section, key, e);
    if let Some(timeout) = &settings.logout_timeout {
        config.logout_timeout = parse_duration(timeout).map_err(|e| at("logout_timeout", e))?;
    }
    config.schedule = schedule(settings, section, dir)?;
    if let Some(latency) = &settings.max_latency {
        config.max_latency = off_or(latency, parse_duration).map_err(|e| at("max_latency", e))?;
    }
    if let Some(check) = settings.check_orig_sending_time {
        config.check_orig_sending_time = check;
    }
    if let Some(check) = settings.check_header_order {
        config.check_header_order = check;
    }
    if let Some(precision) = settings.timestamp_precision {
        config.timestamp_precision = match precision {
            RawPrecision::Seconds => Precision::Seconds,
            RawPrecision::Millis => Precision::Millis,
            RawPrecision::Micros => Precision::Micros,
            RawPrecision::Nanos => Precision::Nanos,
        };
    }
    for &[length_tag, data_tag] in settings.data_fields.iter().flatten() {
        if length_tag == 0 || data_tag == 0 || length_tag == data_tag {
            return Err(at("data_fields", format!("[{length_tag}, {data_tag}] isn't a Length tag and a data tag")));
        }
        update(config, |c| c.with_data_field(length_tag, data_tag));
    }
    if let Some(limit) = &settings.outbound_limit {
        config.outbound_limit = off_or(limit, str::parse::<RateLimit>).map_err(|e| at("outbound_limit", e))?;
    }
    config.inbound_limit = inbound_limit(settings, section)?;
    validation(settings, config, section, dir, dictionaries)?;
    if let Some(latency) = settings.latency_metrics {
        #[cfg(feature = "metrics")]
        {
            config.latency_metrics = latency;
        }
        #[cfg(not(feature = "metrics"))]
        if latency {
            return Err(at("latency_metrics", "needs turbojet-config's metrics feature".into()));
        }
    }
    Ok(())
}

/// Applies one of `SessionConfig`'s builder methods, which take it by value, in place.
fn update(config: &mut SessionConfig, build: impl FnOnce(SessionConfig) -> SessionConfig) {
    *config = build(config.clone());
}

/// `off`, or a value `parse` reads.
fn off_or<T>(text: &str, parse: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    if text == "off" { Ok(None) } else { parse(text).map(Some) }
}

fn schedule(settings: &RawSettings, section: &str, dir: &Path) -> Result<Option<SessionSchedule>, Error> {
    let Some(text) = &settings.schedule else {
        return match &settings.holidays {
            Some(_) => Err(Error::at(section, "holidays", "needs a schedule")),
            None => Ok(None),
        };
    };
    let schedule: SessionSchedule = text.parse().map_err(|e| Error::at(section, "schedule", e))?;
    let Some(path) = &settings.holidays else { return Ok(Some(schedule)) };
    let path = dir.join(path);
    let at = |e: String| Error::at(section, "holidays", format!("{}: {e}", path.display()));
    let text = std::fs::read_to_string(&path).map_err(|e| at(e.to_string()))?;
    let holidays: HolidayCalendar = text.parse().map_err(at)?;
    Ok(Some(schedule.with_holidays(holidays)))
}

fn inbound_limit(settings: &RawSettings, section: &str) -> Result<Option<InboundLimit>, Error> {
    let limit = match &settings.inbound_limit {
        Some(text) => off_or(text, str::parse::<RateLimit>).map_err(|e| Error::at(section, "inbound_limit", e))?,
        None => None,
    };
    Ok(limit.map(|limit| match settings.over_limit {
        Some(OverLimit::Reject) => InboundLimit::Reject(limit),
        Some(OverLimit::Delay) | None => InboundLimit::Delay(limit),
    }))
}

/// Dictionaries loaded so far, by path, so each file is read once per load.
#[derive(Default)]
struct Dictionaries {
    #[cfg(feature = "validation")]
    loaded: HashMap<PathBuf, Arc<turbojet_dictionary::Dictionary>>,
}

impl Dictionaries {
    #[cfg(feature = "validation")]
    fn get(&mut self, path: PathBuf) -> Result<Arc<turbojet_dictionary::Dictionary>, String> {
        if let Some(dictionary) = self.loaded.get(&path) {
            return Ok(dictionary.clone());
        }
        let dictionary = Arc::new(turbojet_dictionary::Dictionary::load(&path).map_err(|e| e.to_string())?);
        self.loaded.insert(path, dictionary.clone());
        Ok(dictionary)
    }
}

/// Why `key` can't be used: this build has no dictionaries.
#[cfg(not(feature = "validation"))]
fn needs_validation(section: &str, key: &str) -> Error {
    Error::at(section, key, "needs turbojet-config's validation feature")
}

/// The application versions, and the dictionaries messages are checked against.
#[cfg_attr(not(feature = "validation"), expect(unused_variables, reason = "dictionaries need the validation feature"))]
fn validation(
    settings: &RawSettings,
    config: &mut SessionConfig,
    section: &str,
    dir: &Path,
    dictionaries: &mut Dictionaries,
) -> Result<(), Error> {
    if let Some(versions) = &settings.appl_versions {
        config.appl_versions.clear();
        for version in versions {
            let (code, dictionary) = match version {
                RawApplVersion::Code(code) => (code, None),
                RawApplVersion::WithDictionary { id, dictionary } => (id, Some(dictionary)),
            };
            let id = ApplVerId::from_fix(code)
                .map_err(|_| Error::at(section, "appl_versions", format!("'{code}' isn't an ApplVerID code")))?;
            update(config, |c| c.with_appl_ver_id(id));
            if let Some(path) = dictionary {
                #[cfg(feature = "validation")]
                {
                    let path = dir.join(path);
                    let loaded = dictionaries.get(path).map_err(|e| Error::at(section, "appl_versions", e))?;
                    update(config, |c| c.with_dictionary(&loaded));
                }
                #[cfg(not(feature = "validation"))]
                return Err(needs_validation(section, "appl_versions"));
            }
        }
    }
    if let Some(path) = &settings.dictionary {
        if config.is_fixt() {
            return Err(Error::at(
                section,
                "dictionary",
                "a FIXT session checks each version's messages: use appl_versions = [{ id, dictionary }]",
            ));
        }
        #[cfg(feature = "validation")]
        {
            let loaded = dictionaries.get(dir.join(path)).map_err(|e| Error::at(section, "dictionary", e))?;
            update(config, |c| c.with_dictionary(&loaded));
        }
        #[cfg(not(feature = "validation"))]
        return Err(needs_validation(section, "dictionary"));
    }
    Ok(())
}

/// Every store, by name. A store the previous file defined the same way is kept, not opened
/// again: it may hold sessions' logs open.
fn stores(
    defined: &BTreeMap<String, RawStore>,
    context: &Context<'_>,
) -> Result<HashMap<String, Arc<dyn SessionStorage>>, Error> {
    let mut stores: HashMap<String, Arc<dyn SessionStorage>> = HashMap::new();
    let memory = context.previous.and_then(|p| p.get(MEMORY).cloned());
    stores.insert(MEMORY.into(), memory.unwrap_or_else(|| Arc::new(MemoryStorage::new())));
    for (name, storage) in context.registered {
        stores.insert(name.clone(), storage.clone());
    }
    for (name, store) in defined {
        let section = format!("store {name}");
        if stores.contains_key(name) {
            let whose = if name == MEMORY { "built in" } else { "registered in code" };
            return Err(Error::at(&section, "name", format!("'{name}' is {whose}")));
        }
        let kept = context.previous_defined.and_then(|d| d.get(name)).filter(|before| *before == store);
        if let (Some(_), Some(previous)) = (kept, context.previous) {
            stores.insert(name.clone(), previous[name].clone());
            continue;
        }
        let storage: Arc<dyn SessionStorage> = match store {
            RawStore::Memory => Arc::new(MemoryStorage::new()),
            RawStore::Disk { dir, fsync } => {
                let dir = context.dir.join(dir);
                let disk = DiskStorage::new(&dir, *fsync)
                    .map_err(|e| Error::at(&section, "dir", format!("{}: {e}", dir.display())))?;
                Arc::new(disk)
            }
        };
        stores.insert(name.clone(), storage);
    }
    Ok(stores)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const ACCEPTOR: &str = r#"
        [acceptor]
        begin_string = "FIX.4.4"
        sender_comp_id = "VENUE"
        listen = "127.0.0.1:0"
    "#;

    fn load_in(dir: &Path, text: &str) -> Result<Loaded, Error> {
        let clock = Clock::system();
        let registered = HashMap::new();
        let context = Context { dir, clock: &clock, registered: &registered, previous: None, previous_defined: None };
        parse(&format!("{ACCEPTOR}{text}"), &context)
    }

    fn load_text(text: &str) -> Result<Loaded, Error> {
        load_in(Path::new("."), text)
    }

    fn error(text: &str) -> String {
        load_text(text).err().expect("an error").to_string()
    }

    #[test]
    fn a_counterparty_inherits_the_defaults_and_overrides_them() {
        let loaded = load_text(
            r#"
            [defaults]
            max_latency = "30s"
            outbound_limit = "100/1s"
            heartbeat = { min = "5s", max = "60s" }

            [counterparty.BROKER]
            max_latency = "off"
            require_client_certificate = true

            [counterparty.FUND]
            "#,
        )
        .unwrap();
        assert_eq!(loaded.base.max_latency, Some(Duration::from_secs(30)), "the defaults are the acceptor's");
        let broker = &loaded.settings("BROKER").counterparty;
        assert_eq!(broker.config.max_latency, None);
        assert_eq!(broker.config.outbound_limit, Some(RateLimit::new(100, Duration::from_secs(1))));
        assert_eq!(broker.heartbeat, Duration::from_secs(5)..=Duration::from_secs(60));
        assert!(broker.require_client_certificate);
        let fund = &loaded.settings("FUND").counterparty;
        assert_eq!(fund.config.max_latency, Some(Duration::from_secs(30)));
        assert!(!fund.require_client_certificate);
        let other = &loaded.settings("OTHER").counterparty;
        assert_eq!(other.config.max_latency, Some(Duration::from_secs(30)), "unlisted: the defaults");
        assert_eq!(loaded.settings("OTHER").store, MEMORY);
    }

    #[test]
    fn values_convert() {
        let loaded = load_text(
            r#"
            [counterparty.A]
            logout_timeout = "2s"
            schedule = "daily 08:00-17:00 mon-fri"
            check_orig_sending_time = false
            check_header_order = false
            timestamp_precision = "micros"
            data_fields = [[5000, 5001]]
            inbound_limit = "50/1s"
            over_limit = "reject"
            "#,
        )
        .unwrap();
        let config = &loaded.settings("A").counterparty.config;
        assert_eq!(config.logout_timeout, Duration::from_secs(2));
        assert_eq!(config.schedule, Some("daily 08:00-17:00 mon-fri".parse().unwrap()));
        assert!(!config.check_orig_sending_time && !config.check_header_order);
        assert_eq!(config.timestamp_precision, Precision::Micros);
        assert_eq!(config.data_fields.length_tag(5001), Some(5000));
        assert_eq!(config.inbound_limit, Some(InboundLimit::Reject(RateLimit::new(50, Duration::from_secs(1)))));
    }

    #[test]
    fn errors_name_the_section_and_key() {
        let cases = [
            ("[defaults]\nmax_latency = \"soon\"", "defaults: max_latency: invalid duration 'soon'"),
            ("[counterparty.A]\nschedule = \"hourly\"", "counterparty A: schedule:"),
            ("[counterparty.A]\noutbound_limit = \"0/1s\"", "counterparty A: outbound_limit:"),
            ("[counterparty.A]\nholidays = \"h.txt\"", "counterparty A: holidays: needs a schedule"),
            ("[counterparty.A]\nstore = \"tape\"", "counterparty A: store: no store named 'tape'"),
            (
                "[counterparty.A]\nheartbeat = { min = \"0s\", max = \"10s\" }",
                "counterparty A: settings: heartbeat range",
            ),
            ("[counterparty.A]\nappl_versions = [\"9\"]", "counterparty A: settings: application versions need a FIXT"),
            ("[counterparty.A]\ndata_fields = [[5, 5]]", "counterparty A: data_fields:"),
            ("[store.memory]\nkind = \"memory\"", "store memory: name: 'memory' is built in"),
        ];
        for (text, expected) in cases {
            let error = error(text);
            assert!(error.starts_with(expected), "{text}\n  gave {error}\n  not {expected}");
        }
        assert!(error("[defaults]\nmax_latnecy = \"1s\"").contains("max_latnecy"), "TOML errors pass through");
    }

    #[test]
    fn the_acceptors_own_settings_are_checked() {
        let fixt = r#"
            [acceptor]
            begin_string = "FIXT.1.1"
            sender_comp_id = "VENUE"
            listen = "127.0.0.1:0"
        "#;
        let clock = Clock::system();
        let registered = HashMap::new();
        let context = Context {
            dir: Path::new("."),
            clock: &clock,
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
        let error = parse(fixt, &context).err().unwrap().to_string();
        assert!(error.starts_with("defaults: settings: a FIXT session needs an application version"), "{error}");
        let loaded = parse(&format!("{fixt}\n[defaults]\nappl_versions = [\"9\"]"), &context).unwrap();
        assert_eq!(loaded.base.appl_versions.len(), 1);
        let zero = ACCEPTOR.replace("listen", "send_queue = 0\nlisten");
        assert!(parse(&zero, &context).err().unwrap().to_string().starts_with("acceptor: send_queue"));
    }

    #[test]
    fn holidays_are_read_relative_to_the_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("holidays.txt"), "2026-12-25 # Christmas\n").unwrap();
        let loaded =
            load_in(dir.path(), "[defaults]\nschedule = \"daily 08:00-17:00\"\nholidays = \"holidays.txt\"").unwrap();
        assert_eq!(loaded.base.schedule.unwrap().holidays().len(), 1);
        let error = load_in(dir.path(), "[defaults]\nschedule = \"daily 08:00-17:00\"\nholidays = \"missing.txt\"")
            .err()
            .unwrap()
            .to_string();
        assert!(error.starts_with("defaults: holidays: ") && error.contains("missing.txt"), "{error}");
    }

    #[test]
    fn stores_are_named() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = load_in(
            dir.path(),
            r#"
            [store.main]
            kind = "disk"
            dir = "store"

            [defaults]
            store = "main"

            [counterparty.SCRATCH]
            store = "memory"
            "#,
        )
        .unwrap();
        assert_eq!(loaded.settings("OTHER").store, "main");
        assert_eq!(loaded.settings("SCRATCH").store, MEMORY);
        assert!(dir.path().join("store").is_dir(), "opened relative to the file");
        assert!(!Arc::ptr_eq(loaded.store_for("OTHER"), loaded.store_for("SCRATCH")));
    }

    #[cfg(not(feature = "tls"))]
    #[test]
    fn tls_needs_the_tls_feature() {
        let text = ACCEPTOR.replace("listen", "tls = { cert = \"c.pem\", key = \"k.pem\" }\nlisten");
        let clock = Clock::system();
        let registered = HashMap::new();
        let context = Context {
            dir: Path::new("."),
            clock: &clock,
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
        let error = parse(&text, &context).err().unwrap().to_string();
        assert_eq!(error, "acceptor: tls: needs turbojet-config's tls feature");
    }

    #[cfg(not(feature = "validation"))]
    #[test]
    fn a_dictionary_needs_the_validation_feature() {
        assert!(error("[defaults]\ndictionary = \"FIX44.xml\"").starts_with("defaults: dictionary: needs"));
    }

    #[cfg(feature = "validation")]
    #[test]
    fn dictionaries_are_loaded() {
        let orchestra = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dictionaries/orchestra");
        let loaded = load_in(&orchestra, "[counterparty.A]\ndictionary = \"OrchestraFIX44.xml\"").unwrap();
        assert!(loaded.settings("A").counterparty.config.validator.is_some());
        assert!(loaded.settings("B").counterparty.config.validator.is_none());
        let error = error("[defaults]\ndictionary = \"missing.xml\"");
        assert!(error.starts_with("defaults: dictionary: ") && error.contains("missing.xml"), "{error}");
    }
}
