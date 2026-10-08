//! Turning the file as written into what an acceptor and initiators run on, checking every value
//! on the way: a file that loads has nothing left to fail at a counterparty's Logon or an
//! initiator's connection.

use std::collections::{BTreeMap, HashMap};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
#[cfg(feature = "validation")]
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use turbojet::fields::{ApplVerId, FromFix, Precision};
use turbojet::{
    CancelOnDisconnect, CancelTrigger, Clock, Counterparty, DiskStorage, Endpoint, HolidayCalendar, InboundLimit,
    InitiatorConfig, MAX_CANCEL_GRACE, MemoryStorage, MessageLog, RateLimit, ReconnectPolicy, SessionConfig, SessionId,
    SessionSchedule, SessionStorage,
};

use crate::Error;
#[cfg(feature = "tls")]
use crate::raw::ClientCertificate;
#[cfg(feature = "tls")]
use crate::raw::RawInitiatorKeys;
use crate::raw::{
    OverLimit, RawAcceptor, RawApplVersion, RawCancelTrigger, RawFile, RawInitiator, RawPrecision, RawSettings,
    RawStore, Unknown, parse_duration,
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
    /// `[acceptor]` and its counterparties, if the file has one.
    pub acceptor: Option<AcceptorPart>,
    /// Each `[initiator.NAME]`, by name.
    pub initiators: BTreeMap<String, ResolvedInitiator>,
    /// Every store, by name: those the file defines, those registered in code, and `memory`.
    pub stores: HashMap<String, Arc<dyn SessionStorage>>,
}

/// An acceptor's settings and its counterparties'.
pub(crate) struct AcceptorPart {
    /// The acceptor's configuration: `[acceptor]` and `[defaults]`.
    pub base: SessionConfig,
    /// Each listed counterparty's settings, by CompID.
    pub listed: HashMap<String, Resolved>,
    /// An unlisted counterparty's settings, `[defaults]` alone; it logs on with them only under
    /// `unknown = "admit"`.
    pub unlisted: Resolved,
    /// `[acceptor.tls]`, read and checked.
    #[cfg(feature = "tls")]
    pub tls: Tls,
}

impl AcceptorPart {
    /// The settings for counterparty `comp_id`, listed or not.
    pub fn settings(&self, comp_id: &str) -> &Resolved {
        self.listed.get(comp_id).unwrap_or(&self.unlisted)
    }
}

/// An initiator's settings: its configuration, endpoints, store and TLS.
#[derive(Clone)]
pub(crate) struct ResolvedInitiator {
    pub config: InitiatorConfig,
    pub endpoints: Vec<Endpoint>,
    pub store: String,
    /// The CAs to trust, the certificate to present if any, and the server name to check.
    #[cfg(feature = "tls")]
    pub tls: Option<ClientTlsFiles>,
}

/// An initiator's TLS, read and checked.
#[cfg(feature = "tls")]
#[derive(Clone)]
pub(crate) struct ClientTlsFiles {
    pub trust: turbojet::tls::Trust,
    pub identity: Option<turbojet::tls::Identity>,
    pub server_name: String,
}

impl ResolvedInitiator {
    /// The session it logs on to.
    pub fn id(&self) -> SessionId {
        self.config.session_id()
    }

    /// Whether it connects over TLS.
    pub fn uses_tls(&self) -> bool {
        #[cfg(feature = "tls")]
        return self.tls.is_some();
        #[cfg(not(feature = "tls"))]
        false
    }
}

impl Loaded {
    /// The acceptor's part, for a file that has one.
    pub fn acceptor(&self) -> &AcceptorPart {
        self.acceptor.as_ref().expect("an acceptor is made only from a file with [acceptor], which a reload keeps")
    }

    /// What happens to a counterparty the file doesn't list.
    pub fn unknown(&self) -> Unknown {
        self.raw.acceptor.as_ref().map_or(Unknown::Refuse, |a| a.unknown)
    }

    /// The store named `name`.
    pub fn store(&self, name: &str) -> &Arc<dyn SessionStorage> {
        self.stores.get(name).expect("loading checked that every store named exists")
    }

    /// The store the acceptor keeps counterparty `comp_id`'s sessions in.
    pub fn counterparty_store(&self, comp_id: &str) -> &Arc<dyn SessionStorage> {
        self.store(&self.acceptor().settings(comp_id).store)
    }

    /// The store an initiator keeps session `id` in; `memory` for a session no initiator has.
    pub fn initiator_store(&self, id: &SessionId) -> &Arc<dyn SessionStorage> {
        let initiator = self.initiators.values().find(|initiator| initiator.id() == *id);
        self.store(initiator.map_or(MEMORY, |initiator| initiator.store.as_str()))
    }
}

/// What loading needs besides the file.
pub(crate) struct Context<'a> {
    /// The directory relative paths in the file are relative to.
    pub dir: &'a Path,
    /// The clock every session uses, kept across reloads: the acceptor compares by identity.
    pub clock: &'a Clock,
    /// The message log every session shows its messages to, if any.
    pub message_log: Option<&'a Arc<dyn MessageLog>>,
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
    let stores = stores(&raw.store, context)?;
    let mut dictionaries = Dictionaries::default();
    let acceptor = match &raw.acceptor {
        Some(acceptor) => Some(load_acceptor(&raw, acceptor, context, &stores, &mut dictionaries)?),
        None => match raw.counterparty.keys().next() {
            Some(comp_id) => {
                return Err(Error::at(&format!("counterparty {comp_id}"), "section", "needs an [acceptor]"));
            }
            None => None,
        },
    };
    let mut initiators = BTreeMap::new();
    for (name, initiator) in &raw.initiator {
        let resolved = resolve_initiator(name, initiator, &raw, context, &stores, &mut dictionaries)?;
        initiators.insert(name.clone(), resolved);
    }
    check_sessions_unique(&raw, &initiators)?;
    Ok(Loaded { raw, acceptor, initiators, stores })
}

fn load_acceptor(
    raw: &RawFile,
    acceptor: &RawAcceptor,
    context: &Context<'_>,
    stores: &HashMap<String, Arc<dyn SessionStorage>>,
    dictionaries: &mut Dictionaries,
) -> Result<AcceptorPart, Error> {
    let fixed = fixed(acceptor, context)?;
    #[cfg(feature = "tls")]
    let tls = tls(acceptor, context.dir)?;
    #[cfg(not(feature = "tls"))]
    refuse_tls(acceptor.tls.is_some(), "acceptor")?;
    let mut base = fixed.clone();
    apply(&raw.defaults, &mut base, "defaults", context.dir, dictionaries)?;
    base.check().map_err(|e| Error::at("defaults", "settings", e))?;
    let unlisted = resolve(&raw.defaults, &fixed, "defaults", context, stores, dictionaries)?;
    let mut listed = HashMap::new();
    for (comp_id, settings) in &raw.counterparty {
        let section = format!("counterparty {comp_id}");
        let merged = settings.or(&raw.defaults);
        grace_needs_cancel(settings, &merged, &section)?;
        let resolved = resolve(&merged, &fixed, &section, context, stores, dictionaries)?;
        listed.insert(comp_id.clone(), resolved);
    }
    Ok(AcceptorPart {
        base,
        listed,
        unlisted,
        #[cfg(feature = "tls")]
        tls,
    })
}

/// No two initiators log on to one session, and none to a session the acceptor serves for a
/// listed counterparty: the two would fight over its sequence numbers.
fn check_sessions_unique(raw: &RawFile, initiators: &BTreeMap<String, ResolvedInitiator>) -> Result<(), Error> {
    let mut seen: HashMap<SessionId, &str> = HashMap::new();
    for (name, initiator) in initiators {
        let id = initiator.id();
        let section = format!("initiator {name}");
        if let Some(other) = seen.insert(id.clone(), name) {
            return Err(Error::at(&section, "target_comp_id", format!("initiator {other} already logs on to {id}")));
        }
        if let Some(acceptor) = &raw.acceptor
            && acceptor.begin_string == id.begin_string
            && acceptor.sender_comp_id == id.sender_comp_id
            && raw.counterparty.contains_key(&id.target_comp_id)
        {
            return Err(Error::at(&section, "target_comp_id", format!("the acceptor serves {id}")));
        }
    }
    Ok(())
}

/// The acceptor's certificate and the client CAs it trusts, if it serves TLS.
#[cfg(feature = "tls")]
pub(crate) type Tls = Option<(turbojet::tls::Identity, turbojet::tls::ClientTrust)>;

/// `[acceptor.tls]`'s certificate, key and client CAs, read and checked.
#[cfg(feature = "tls")]
fn tls(acceptor: &RawAcceptor, dir: &Path) -> Result<Tls, Error> {
    use turbojet::tls::{ClientTrust, Identity, ServerTls, Trust};
    let Some(tls) = &acceptor.tls else { return Ok(None) };
    let at = |path: &Path, e: std::io::Error| Error::at("acceptor", "tls", format!("{}: {e}", path.display()));
    let (cert, key) = (dir.join(&tls.cert), dir.join(&tls.key));
    let identity = Identity::from_pem_files(&cert, &key).map_err(|e| at(&cert, e))?;
    let client_trust = match &tls.client_ca {
        None if tls.client_certificate == ClientCertificate::Required => {
            return Err(Error::at("acceptor", "tls", "client_certificate = \"required\" needs a client_ca"));
        }
        None if tls.client_crl.is_some() => {
            return Err(Error::at("acceptor", "tls", "client_crl needs a client_ca"));
        }
        None => ClientTrust::None,
        Some(ca) => {
            let ca = dir.join(ca);
            let mut trust = Trust::from_pem_files(&ca).map_err(|e| at(&ca, e))?;
            if let Some(crl) = &tls.client_crl {
                let crl = dir.join(crl);
                trust = trust.with_crls_pem_files(&crl).map_err(|e| at(&crl, e))?;
            }
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

/// Without the tls feature, a `tls` key can't be used.
#[cfg(not(feature = "tls"))]
fn refuse_tls(set: bool, section: &str) -> Result<(), Error> {
    match set {
        true => Err(Error::at(section, "tls", "needs turbojet-config's tls feature")),
        false => Ok(()),
    }
}

/// Session settings fixed until a restart: `logon_timeout` and `send_queue`.
fn fixed_keys(
    config: &mut SessionConfig,
    section: &str,
    logon_timeout: Option<&String>,
    send_queue: Option<usize>,
) -> Result<(), Error> {
    if let Some(timeout) = logon_timeout {
        config.logon_timeout = parse_duration(timeout).map_err(|e| Error::at(section, "logon_timeout", e))?;
    }
    if let Some(queue) = send_queue {
        if queue == 0 {
            return Err(Error::at(section, "send_queue", "must be at least 1"));
        }
        config.send_queue = queue;
    }
    Ok(())
}

/// The acceptor's settings fixed until a restart.
fn fixed(acceptor: &RawAcceptor, context: &Context<'_>) -> Result<SessionConfig, Error> {
    let mut config = SessionConfig::new(&acceptor.begin_string, &acceptor.sender_comp_id);
    config.clock = context.clock.clone();
    config.message_log = context.message_log.cloned();
    fixed_keys(&mut config, "acceptor", acceptor.logon_timeout.as_ref(), acceptor.send_queue)?;
    for (key, limit) in
        [("max_connections", acceptor.max_connections), ("max_connections_per_ip", acceptor.max_connections_per_ip)]
    {
        if limit == Some(0) {
            return Err(Error::at("acceptor", key, "must be at least 1"));
        }
    }
    Ok(config)
}

/// Initiator `name`'s settings: its own keys, and its session keys over `[defaults]`.
fn resolve_initiator(
    name: &str,
    initiator: &RawInitiator,
    raw: &RawFile,
    context: &Context<'_>,
    stores: &HashMap<String, Arc<dyn SessionStorage>>,
    dictionaries: &mut Dictionaries,
) -> Result<ResolvedInitiator, Error> {
    let section = format!("initiator {name}");
    let at = |key: &str, e: String| Error::at(&section, key, e);
    let (own, settings) = (&initiator.own, &initiator.settings);
    if settings.heartbeat.is_some() {
        return Err(at("heartbeat", "is a counterparty's range: an initiator asks for heartbeat_interval".into()));
    }
    if settings.require_client_certificate.is_some() {
        return Err(at("require_client_certificate", "is for counterparties".into()));
    }
    let acceptor = raw.acceptor.as_ref();
    let begin_string = own.begin_string.clone().or_else(|| acceptor.map(|a| a.begin_string.clone()));
    let begin_string = begin_string.ok_or_else(|| at("begin_string", "needed without an [acceptor]".into()))?;
    let sender_comp_id = own.sender_comp_id.clone().or_else(|| acceptor.map(|a| a.sender_comp_id.clone()));
    let sender_comp_id = sender_comp_id.ok_or_else(|| at("sender_comp_id", "needed without an [acceptor]".into()))?;
    let mut session = SessionConfig::new(begin_string, sender_comp_id);
    session.clock = context.clock.clone();
    session.message_log = context.message_log.cloned();
    fixed_keys(&mut session, &section, own.logon_timeout.as_ref(), own.send_queue)?;
    let merged = settings.or(&raw.defaults);
    grace_needs_cancel(settings, &merged, &section)?;
    apply(&merged, &mut session, &section, context.dir, dictionaries)?;
    let mut config = InitiatorConfig::new(session, &own.target_comp_id);
    config.sender_sub_id.clone_from(&own.sender_sub_id);
    config.sender_location_id.clone_from(&own.sender_location_id);
    config.target_sub_id.clone_from(&own.target_sub_id);
    config.target_location_id.clone_from(&own.target_location_id);
    config.qualifier.clone_from(&own.qualifier);
    if let Some(interval) = &own.heartbeat_interval {
        config.heartbeat_interval = parse_duration(interval).map_err(|e| at("heartbeat_interval", e))?;
    }
    config.reset_on_logon = own.reset_on_logon.unwrap_or(false);
    config.next_expected_msg_seq_num = own.next_expected_msg_seq_num.unwrap_or(false);
    config.username = own.username.clone();
    if let Some(variable) = &own.password_env {
        let password =
            std::env::var(variable).map_err(|e| at("password_env", format!("environment variable {variable}: {e}")))?;
        config.password = Some(password.into());
    }
    if let Some(timeout) = &own.connect_timeout {
        config.connect_timeout = parse_duration(timeout).map_err(|e| at("connect_timeout", e))?;
    }
    if let Some(local) = &own.local_address {
        // An address alone leaves the port to the system.
        let parsed = local.parse().or_else(|_| local.parse::<IpAddr>().map(|ip| SocketAddr::new(ip, 0)));
        config.local_addr = Some(parsed.map_err(|_| at("local_address", format!("'{local}' isn't an IP address")))?);
    }
    if let Some(reconnect) = &own.reconnect {
        let duration = |text: &str| parse_duration(text).map_err(|e| at("reconnect", e));
        let mut policy = ReconnectPolicy::exponential(duration(&reconnect.initial)?, duration(&reconnect.max)?);
        policy.multiplier = reconnect.multiplier.unwrap_or(policy.multiplier);
        policy.jitter = reconnect.jitter.unwrap_or(policy.jitter);
        config.reconnect = policy;
    }
    config.check().map_err(|e| at("settings", e.to_string()))?;
    if own.connect.is_empty() {
        return Err(at("connect", "needs an address".into()));
    }
    let endpoints = own.connect.iter().map(Endpoint::new).collect();
    let store = merged.store.clone().unwrap_or_else(|| MEMORY.into());
    if !stores.contains_key(&store) {
        return Err(at("store", format!("no store named '{store}'")));
    }
    #[cfg(not(feature = "tls"))]
    refuse_tls(own.tls.is_some(), &section)?;
    Ok(ResolvedInitiator {
        config,
        endpoints,
        store,
        #[cfg(feature = "tls")]
        tls: client_tls(own, &section, context.dir)?,
    })
}

/// An initiator's `tls`, read and checked.
#[cfg(feature = "tls")]
fn client_tls(own: &RawInitiatorKeys, section: &str, dir: &Path) -> Result<Option<ClientTlsFiles>, Error> {
    use turbojet::tls::{ClientTls, Identity, ServerName, Trust};
    let Some(tls) = &own.tls else { return Ok(None) };
    let at = |e: String| Error::at(section, "tls", e);
    let file_error = |path: &Path, e: std::io::Error| at(format!("{}: {e}", path.display()));
    let ca = dir.join(&tls.ca);
    let mut trust = Trust::from_pem_files(&ca).map_err(|e| file_error(&ca, e))?;
    if let Some(crl) = &tls.crl {
        let crl = dir.join(crl);
        trust = trust.with_crls_pem_files(&crl).map_err(|e| file_error(&crl, e))?;
    }
    let identity = match (&tls.cert, &tls.key) {
        (Some(cert), Some(key)) => {
            let (cert, key) = (dir.join(cert), dir.join(key));
            Some(Identity::from_pem_files(&cert, &key).map_err(|e| file_error(&cert, e))?)
        }
        (None, None) => None,
        _ => return Err(at("cert and key go together".into())),
    };
    let server_name = match &tls.server_name {
        Some(name) => name.clone(),
        None => host(&own.connect[0]).to_string(),
    };
    ServerName::try_from(server_name.clone()).map_err(|e| at(format!("invalid server_name '{server_name}': {e}")))?;
    ClientTls::new(trust.clone(), identity.clone()).map_err(|e| at(e.to_string()))?;
    Ok(Some(ClientTlsFiles { trust, identity, server_name }))
}

/// The host of `host:port` (or `[v6]:port`).
#[cfg(feature = "tls")]
fn host(addr: &str) -> &str {
    let host = addr.rsplit_once(':').map_or(addr, |(host, _)| host);
    host.trim_start_matches('[').trim_end_matches(']')
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
    if let Some(chunk) = settings.resend_request_chunk {
        config.resend_request_chunk = Some(chunk);
    }
    if let Some(max) = settings.max_sessions_per_counterparty {
        config.max_sessions_per_counterparty = max;
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
    config.cancel_on_disconnect = cancel_on_disconnect(settings, section)?;
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

/// Cancel on disconnect, from `settings` already merged over the defaults. The grace is checked
/// even with cancel on disconnect off, so a `[defaults]` grace no section uses is still a valid
/// one. [`SessionConfig::check`] checks its limit again.
fn cancel_on_disconnect(settings: &RawSettings, section: &str) -> Result<Option<CancelOnDisconnect>, Error> {
    let at = |e: String| Error::at(section, "cancel_grace", e);
    let grace = match &settings.cancel_grace {
        Some(text) => {
            let grace = parse_duration(text).map_err(at)?;
            // The message says 1h.
            const _: () = assert!(MAX_CANCEL_GRACE.as_secs() == 3600);
            if grace > MAX_CANCEL_GRACE {
                return Err(at(format!("'{text}' must be at most 1h")));
            }
            grace
        }
        None => Duration::ZERO,
    };
    let trigger = match settings.cancel_on_disconnect {
        None | Some(RawCancelTrigger::Off) => return Ok(None),
        Some(RawCancelTrigger::Disconnect) => CancelTrigger::Disconnect,
        Some(RawCancelTrigger::DisconnectOrLogout) => CancelTrigger::DisconnectOrLogout,
    };
    Ok(Some(CancelOnDisconnect::new(trigger, grace)))
}

/// A counterparty's or initiator's own `cancel_grace` with cancel on disconnect off, its own
/// setting or the defaults', is an error: the grace would do nothing. A grace in `[defaults]`
/// isn't, used or not: it's for the sections that turn cancel on disconnect on.
fn grace_needs_cancel(own: &RawSettings, merged: &RawSettings, section: &str) -> Result<(), Error> {
    let on = !matches!(merged.cancel_on_disconnect, None | Some(RawCancelTrigger::Off));
    match own.cancel_grace {
        Some(_) if !on => Err(Error::at(section, "cancel_grace", "needs cancel_on_disconnect on")),
        _ => Ok(()),
    }
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
        let context = Context {
            dir,
            clock: &clock,
            message_log: None,
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
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
        assert_eq!(
            loaded.acceptor().base.max_latency,
            Some(Duration::from_secs(30)),
            "the defaults are the acceptor's"
        );
        let broker = &loaded.acceptor().settings("BROKER").counterparty;
        assert_eq!(broker.config.max_latency, None);
        assert_eq!(broker.config.outbound_limit, Some(RateLimit::new(100, Duration::from_secs(1))));
        assert_eq!(broker.heartbeat, Duration::from_secs(5)..=Duration::from_secs(60));
        assert!(broker.require_client_certificate);
        let fund = &loaded.acceptor().settings("FUND").counterparty;
        assert_eq!(fund.config.max_latency, Some(Duration::from_secs(30)));
        assert!(!fund.require_client_certificate);
        let other = &loaded.acceptor().settings("OTHER").counterparty;
        assert_eq!(other.config.max_latency, Some(Duration::from_secs(30)), "unlisted: the defaults");
        assert_eq!(loaded.acceptor().settings("OTHER").store, MEMORY);
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
            resend_request_chunk = 100
            max_sessions_per_counterparty = 4
            "#,
        )
        .unwrap();
        let config = &loaded.acceptor().settings("A").counterparty.config;
        assert_eq!(config.logout_timeout, Duration::from_secs(2));
        assert_eq!(config.schedule, Some("daily 08:00-17:00 mon-fri".parse().unwrap()));
        assert!(!config.check_orig_sending_time && !config.check_header_order);
        assert_eq!(config.resend_request_chunk, Some(100));
        assert_eq!(config.max_sessions_per_counterparty, 4);
        assert_eq!(config.timestamp_precision, Precision::Micros);
        assert_eq!(config.data_fields.length_tag(5001), Some(5000));
        assert_eq!(config.inbound_limit, Some(InboundLimit::Reject(RateLimit::new(50, Duration::from_secs(1)))));
    }

    #[test]
    fn cancel_on_disconnect_converts_and_a_counterparty_overrides_it() {
        let loaded = load_text(
            r#"
            [defaults]
            cancel_on_disconnect = "disconnect"
            cancel_grace = "5s"

            [counterparty.BROKER]
            cancel_on_disconnect = "disconnect_or_logout"

            [counterparty.FUND]
            cancel_on_disconnect = "off"

            [counterparty.DESK]
            cancel_grace = "500ms"
            "#,
        )
        .unwrap();
        let cancel = |comp_id| loaded.acceptor().settings(comp_id).counterparty.config.cancel_on_disconnect;
        let grace = |grace| Some(CancelOnDisconnect::new(CancelTrigger::Disconnect, grace));
        assert_eq!(loaded.acceptor().base.cancel_on_disconnect, grace(Duration::from_secs(5)));
        assert_eq!(
            cancel("BROKER"),
            Some(CancelOnDisconnect::new(CancelTrigger::DisconnectOrLogout, Duration::from_secs(5)))
        );
        assert_eq!(cancel("FUND"), None, "turned off, the defaults' grace unused");
        assert_eq!(cancel("DESK"), grace(Duration::from_millis(500)));
        assert_eq!(cancel("OTHER"), grace(Duration::from_secs(5)), "unlisted: the defaults");

        let loaded =
            load_text("[counterparty.A]\ncancel_on_disconnect = \"disconnect_or_logout\"\n[counterparty.B]").unwrap();
        let a = loaded.acceptor().settings("A").counterparty.config.cancel_on_disconnect;
        assert_eq!(a, Some(CancelOnDisconnect::new(CancelTrigger::DisconnectOrLogout, Duration::ZERO)));
        assert_eq!(loaded.acceptor().settings("B").counterparty.config.cancel_on_disconnect, None, "off by default");

        let opt_in = "[defaults]\ncancel_grace = \"2s\"\n[counterparty.A]\ncancel_on_disconnect = \"disconnect\"";
        let loaded = load_text(opt_in).unwrap();
        assert_eq!(loaded.acceptor().base.cancel_on_disconnect, None, "the defaults' grace alone turns nothing on");
        let a = loaded.acceptor().settings("A").counterparty.config.cancel_on_disconnect;
        assert_eq!(a, Some(CancelOnDisconnect::new(CancelTrigger::Disconnect, Duration::from_secs(2))));
        assert_eq!(loaded.acceptor().settings("OTHER").counterparty.config.cancel_on_disconnect, None);
    }

    #[test]
    fn cancel_on_disconnect_errors_name_the_section_and_key() {
        let cases = [
            (
                "[counterparty.A]\ncancel_on_disconnect = \"off\"\ncancel_grace = \"5s\"",
                "counterparty A: cancel_grace: needs cancel_on_disconnect on",
            ),
            (
                "[defaults]\ncancel_on_disconnect = \"off\"\n[counterparty.A]\ncancel_grace = \"5s\"",
                "counterparty A: cancel_grace: needs cancel_on_disconnect on",
            ),
            (
                "[defaults]\ncancel_on_disconnect = \"disconnect\"\n[counterparty.A]\ncancel_on_disconnect = \"off\"\ncancel_grace = \"5s\"",
                "counterparty A: cancel_grace: needs cancel_on_disconnect on",
            ),
            (
                "[counterparty.A]\ncancel_on_disconnect = \"disconnect_or_logout\"\ncancel_grace = \"soon\"",
                "counterparty A: cancel_grace: invalid duration 'soon'",
            ),
            (
                "[defaults]\ncancel_on_disconnect = \"disconnect_or_logout\"\ncancel_grace = \"61m\"",
                "defaults: cancel_grace: '61m' must be at most 1h",
            ),
            (
                "[counterparty.A]\ncancel_on_disconnect = \"disconnect_or_logout\"\ncancel_grace = \"2h\"",
                "counterparty A: cancel_grace: '2h' must be at most 1h",
            ),
            ("[defaults]\ncancel_grace = \"soon\"", "defaults: cancel_grace: invalid duration 'soon'"),
            ("[defaults]\ncancel_grace = \"2h\"", "defaults: cancel_grace: '2h' must be at most 1h"),
        ];
        for (text, expected) in cases {
            let error = error(text);
            assert!(error.starts_with(expected), "{text}\n  gave {error}\n  not {expected}");
        }
        let error = error("[counterparty.A]\ncancel_on_disconnect = \"always\"");
        assert!(error.contains("cancel_on_disconnect") && error.contains("always"), "{error}");
        let hour = "[defaults]\ncancel_on_disconnect = \"disconnect_or_logout\"\ncancel_grace = \"1h\"";
        assert_eq!(load_text(hour).unwrap().acceptor().base.cancel_on_disconnect.unwrap().grace, MAX_CANCEL_GRACE);
    }

    #[test]
    fn errors_name_the_section_and_key() {
        let cases = [
            ("[defaults]\nmax_latency = \"soon\"", "defaults: max_latency: invalid duration 'soon'"),
            ("[defaults]\nresend_request_chunk = 0", "defaults: settings: resend_request_chunk must be at least 1"),
            (
                "[defaults]\nmax_sessions_per_counterparty = 0",
                "defaults: settings: max_sessions_per_counterparty must be at least 1",
            ),
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
            message_log: None,
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
        let error = parse(fixt, &context).err().unwrap().to_string();
        assert!(error.starts_with("defaults: settings: a FIXT session needs an application version"), "{error}");
        let loaded = parse(&format!("{fixt}\n[defaults]\nappl_versions = [\"9\"]"), &context).unwrap();
        assert_eq!(loaded.acceptor().base.appl_versions.len(), 1);
        let zero = ACCEPTOR.replace("listen", "send_queue = 0\nlisten");
        assert!(parse(&zero, &context).err().unwrap().to_string().starts_with("acceptor: send_queue"));
    }

    #[test]
    fn holidays_are_read_relative_to_the_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("holidays.txt"), "2026-12-25 # Christmas\n").unwrap();
        let loaded =
            load_in(dir.path(), "[defaults]\nschedule = \"daily 08:00-17:00\"\nholidays = \"holidays.txt\"").unwrap();
        assert_eq!(loaded.acceptor().base.schedule.as_ref().unwrap().holidays().len(), 1);
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
        assert_eq!(loaded.acceptor().settings("OTHER").store, "main");
        assert_eq!(loaded.acceptor().settings("SCRATCH").store, MEMORY);
        assert!(dir.path().join("store").is_dir(), "opened relative to the file");
        assert!(!Arc::ptr_eq(loaded.counterparty_store("OTHER"), loaded.counterparty_store("SCRATCH")));
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
            message_log: None,
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
        assert!(loaded.acceptor().settings("A").counterparty.config.validator.is_some());
        assert!(loaded.acceptor().settings("B").counterparty.config.validator.is_none());
        let error = error("[defaults]\ndictionary = \"missing.xml\"");
        assert!(error.starts_with("defaults: dictionary: ") && error.contains("missing.xml"), "{error}");
    }

    const INITIATOR: &str = r#"
        [initiator.LSE]
        target_comp_id = "LSE"
        connect = ["primary:9876", "backup:9876"]
    "#;

    #[test]
    fn initiators_to_one_counterparty_are_told_apart_by_their_ids() {
        let loaded = load_text(&format!(
            "{INITIATOR}sender_sub_id = \"DESK1\"\ntarget_location_id = \"LDN\"\n\
             [initiator.LSE2]\ntarget_comp_id = \"LSE\"\nsender_sub_id = \"DESK2\"\nqualifier = \"b\"\nconnect = [\"x:1\"]"
        ))
        .unwrap();
        assert_eq!(loaded.initiators["LSE"].id().to_string(), "FIX.4.4:VENUE/DESK1->LSE//LDN");
        assert_eq!(loaded.initiators["LSE2"].id().to_string(), "FIX.4.4:VENUE/DESK2->LSE:b");
    }

    #[test]
    fn an_initiator_takes_the_acceptors_identity_and_the_defaults() {
        let loaded = load_text(&format!(
            "[defaults]\nmax_latency = \"30s\"\nstore = \"memory\"\n{INITIATOR}heartbeat_interval = \"20s\"\nreset_on_logon = true\nusername = \"firm\"\npassword_env = \"PATH\"\nreconnect = {{ initial = \"100ms\", max = \"5s\", jitter = false }}\nlocal_address = \"10.0.0.5\""
        ))
        .unwrap();
        let lse = &loaded.initiators["LSE"];
        assert_eq!(lse.id().to_string(), "FIX.4.4:VENUE->LSE");
        assert_eq!(lse.config.session.max_latency, Some(Duration::from_secs(30)));
        assert_eq!(lse.config.heartbeat_interval, Duration::from_secs(20));
        assert!(lse.config.reset_on_logon && !lse.config.next_expected_msg_seq_num);
        assert_eq!(lse.config.username.as_deref(), Some("firm"));
        assert_eq!(lse.config.password.as_ref().map(|p| p.expose().to_string()), std::env::var("PATH").ok());
        assert_eq!(lse.config.reconnect.initial, Duration::from_millis(100));
        assert!(!lse.config.reconnect.jitter);
        assert_eq!(lse.endpoints, [Endpoint::new("primary:9876"), Endpoint::new("backup:9876")]);
        assert_eq!(lse.config.local_addr, Some("10.0.0.5:0".parse().unwrap()));
        let loaded = load_text(&format!("{INITIATOR}local_address = \"[::1]:4000\"")).unwrap();
        assert_eq!(loaded.initiators["LSE"].config.local_addr, Some("[::1]:4000".parse().unwrap()));
    }

    #[test]
    fn an_initiators_cancel_on_disconnect_is_over_the_defaults() {
        let defaults = "[defaults]\ncancel_on_disconnect = \"disconnect\"\ncancel_grace = \"5s\"\n";
        let loaded =
            load_text(&format!("{defaults}{INITIATOR}cancel_on_disconnect = \"disconnect_or_logout\"")).unwrap();
        assert_eq!(
            loaded.initiators["LSE"].config.session.cancel_on_disconnect,
            Some(CancelOnDisconnect::new(CancelTrigger::DisconnectOrLogout, Duration::from_secs(5)))
        );
        let loaded = load_text(&format!("{defaults}{INITIATOR}cancel_on_disconnect = \"off\"")).unwrap();
        assert_eq!(loaded.initiators["LSE"].config.session.cancel_on_disconnect, None);
        let opt_in = load_text(&format!(
            "[defaults]\ncancel_grace = \"5s\"\n{INITIATOR}cancel_on_disconnect = \"disconnect_or_logout\""
        ));
        let lse = opt_in.unwrap().initiators["LSE"].config.session.cancel_on_disconnect;
        assert_eq!(lse.map(|c| c.grace), Some(Duration::from_secs(5)), "the defaults' grace");
        let unused = error(&format!("{INITIATOR}cancel_grace = \"5s\""));
        assert_eq!(unused, "initiator LSE: cancel_grace: needs cancel_on_disconnect on");
        let long = error(&format!("{INITIATOR}cancel_on_disconnect = \"disconnect_or_logout\"\ncancel_grace = \"2h\""));
        assert_eq!(long, "initiator LSE: cancel_grace: '2h' must be at most 1h");
    }

    #[test]
    fn every_session_gets_the_message_log() {
        #[derive(Debug)]
        struct Discard;
        impl MessageLog for Discard {
            fn inbound(&self, _: Option<&SessionId>, _: &[u8]) {}
            fn outbound(&self, _: Option<&SessionId>, _: &[u8]) {}
        }
        let log: Arc<dyn MessageLog> = Arc::new(Discard);
        let clock = Clock::system();
        let registered = HashMap::new();
        let context = Context {
            dir: Path::new("."),
            clock: &clock,
            message_log: Some(&log),
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
        let loaded = parse(&format!("{ACCEPTOR}[counterparty.BROKER]\n{INITIATOR}"), &context).unwrap();
        let has_log = |config: &SessionConfig| config.message_log.as_ref().is_some_and(|l| Arc::ptr_eq(l, &log));
        assert!(has_log(&loaded.acceptor().base), "the acceptor's own, for a Logon");
        assert!(has_log(&loaded.acceptor().settings("BROKER").counterparty.config));
        assert!(has_log(&loaded.acceptor().settings("OTHER").counterparty.config), "unlisted");
        assert!(has_log(&loaded.initiators["LSE"].config.session));
    }

    #[test]
    fn a_file_may_hold_only_initiators() {
        let clock = Clock::system();
        let registered = HashMap::new();
        let context = Context {
            dir: Path::new("."),
            clock: &clock,
            message_log: None,
            registered: &registered,
            previous: None,
            previous_defined: None,
        };
        let text = format!("{INITIATOR}begin_string = \"FIX.4.2\"\nsender_comp_id = \"FIRM\"");
        let loaded = parse(&text, &context).unwrap();
        assert!(loaded.acceptor.is_none());
        assert_eq!(loaded.initiators["LSE"].id().to_string(), "FIX.4.2:FIRM->LSE");
        let error = parse(INITIATOR, &context).err().unwrap().to_string();
        assert_eq!(error, "initiator LSE: begin_string: needed without an [acceptor]");
        let error = parse(
            &format!("{INITIATOR}begin_string = \"FIX.4.2\"\nsender_comp_id = \"F\"\n[counterparty.X]"),
            &context,
        )
        .err()
        .unwrap()
        .to_string();
        assert_eq!(error, "counterparty X: section: needs an [acceptor]");
    }

    #[test]
    fn initiator_errors_name_the_section_and_key() {
        let cases = [
            (
                format!("{INITIATOR}heartbeat = {{ min = \"1s\", max = \"2s\" }}"),
                "initiator LSE: heartbeat: is a counterparty's range",
            ),
            (format!("{INITIATOR}require_client_certificate = true"), "initiator LSE: require_client_certificate:"),
            (
                format!("{INITIATOR}heartbeat_interval = \"500ms\""),
                "initiator LSE: settings: heartbeat_interval must be whole seconds",
            ),
            (
                format!("{INITIATOR}password_env = \"TURBOJET_CONFIG_SURELY_UNSET\""),
                "initiator LSE: password_env: environment variable",
            ),
            (format!("{INITIATOR}store = \"tape\""), "initiator LSE: store: no store named 'tape'"),
            (
                format!("{INITIATOR}local_address = \"eth0\""),
                "initiator LSE: local_address: 'eth0' isn't an IP address",
            ),
            (
                INITIATOR.replace("[\"primary:9876\", \"backup:9876\"]", "[]"),
                "initiator LSE: connect: needs an address",
            ),
            (format!("{INITIATOR}conect = []"), "conect"),
            (
                format!("{INITIATOR}\n[initiator.AGAIN]\ntarget_comp_id = \"LSE\"\nconnect = [\"x:1\"]"),
                "initiator LSE: target_comp_id: initiator AGAIN already logs on to FIX.4.4:VENUE->LSE",
            ),
            (
                format!(
                    "{INITIATOR}sender_sub_id = \"D\"\n[initiator.AGAIN]\ntarget_comp_id = \"LSE\"\nsender_sub_id = \"D\"\nconnect = [\"x:1\"]"
                ),
                "initiator LSE: target_comp_id: initiator AGAIN already logs on to FIX.4.4:VENUE/D->LSE",
            ),
            (
                format!("[counterparty.LSE]\n{INITIATOR}"),
                "initiator LSE: target_comp_id: the acceptor serves FIX.4.4:VENUE->LSE",
            ),
        ];
        for (text, expected) in cases {
            let error = error(&text);
            assert!(error.contains(expected), "{text}\n  gave {error}\n  not {expected}");
        }
    }

    #[cfg(not(feature = "tls"))]
    #[test]
    fn an_initiators_tls_needs_the_tls_feature() {
        let error = error(&format!("{INITIATOR}tls = {{ ca = \"ca.pem\" }}"));
        assert_eq!(error, "initiator LSE: tls: needs turbojet-config's tls feature");
    }
}
