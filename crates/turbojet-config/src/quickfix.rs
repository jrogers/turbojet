//! Converting a QuickFIX settings file (`.cfg`, as QuickFIX, QuickFIX/J and quickfix-go read it)
//! into a sessions file: [`convert`].

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Write as _};

/// Converts the QuickFIX settings in `cfg` into a sessions file's TOML, for review before use.
///
/// `[DEFAULT]` applies to each `[SESSION]`, whose keys are written out in full. Acceptor sessions
/// become one `[acceptor]`, which they must agree on (BeginString, SenderCompID, the address and
/// port, LogonTimeout), and a `[counterparty.TARGET]` each; initiator sessions become
/// `[initiator.TARGET]`. Keys that map are converted. Keys Turbojet sets in code (logging, socket
/// options) or does differently (validation switches) are listed in a comment, not converted.
/// Any other key, or a value that would make Turbojet behave differently from QuickFIX, is a
/// problem: every one is reported, and nothing is written.
///
/// Paths (stores, dictionaries) are copied as written. QuickFIX reads them from the directory
/// it runs in, a sessions file from its own, so check them.
///
/// # Errors
///
/// [`ConvertError`], listing every problem found.
pub fn convert(cfg: &str) -> Result<String, ConvertError> {
    let sessions = parse(cfg).map_err(|problems| ConvertError { problems })?;
    let mut out = Output::default();
    let mut problems = Vec::new();
    for mut session in sessions {
        match session.take("ConnectionType").as_deref() {
            Some("acceptor") => out.acceptor(session, &mut problems),
            Some("initiator") => out.initiator(session, &mut problems),
            Some(other) => {
                problems.push(format!("{}: ConnectionType '{other}' isn't acceptor or initiator", session.name))
            }
            None => problems.push(format!("{}: no ConnectionType", session.name)),
        }
    }
    if out.sections.is_empty() && problems.is_empty() {
        problems.push("no [SESSION]s".to_string());
    }
    if !problems.is_empty() {
        return Err(ConvertError { problems });
    }
    Ok(out.render())
}

/// Why a QuickFIX settings file couldn't be converted: every problem found, each naming its
/// session and key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertError {
    pub problems: Vec<String>,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "can't convert the QuickFIX settings:")?;
        for problem in &self.problems {
            write!(f, "\n  {problem}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ConvertError {}

/// Keys Turbojet sets in code or doesn't need, listed in a comment rather than converted.
const NOT_CONVERTED_PREFIXES: &[&str] = &["FileLog", "ScreenLog", "SLF4JLog", "Socket"];
const NOT_CONVERTED: &[&str] = &[
    "FileIncludeMilliseconds",
    "FileIncludeTimeStampForMessages",
    "TransportDataDictionary",
    "ValidateFieldsOutOfOrder",
    "ValidateFieldsHaveValues",
    "ValidateUserDefinedFields",
    "ValidateUnorderedGroupFields",
    "ValidateIncomingMessage",
    "AllowUnknownMsgFields",
    "RejectInvalidMessage",
];
/// Socket keys that are converted, or refused, rather than listed as not converted.
const SOCKET_KEYS_HANDLED: &[&str] =
    &["SocketAcceptPort", "SocketAcceptAddress", "SocketAcceptHost", "SocketConnectHost", "SocketConnectPort"];
/// Keys accepted only with their QuickFIX default, which is what Turbojet does.
const ONLY_DEFAULT: &[(&str, &str, &str)] = &[
    ("ResetOnLogout", "N", "Turbojet doesn't reset sequence numbers on a Logout"),
    ("ResetOnDisconnect", "N", "Turbojet doesn't reset sequence numbers on a disconnection"),
    ("RefreshOnLogon", "N", "Turbojet doesn't reload its store at a Logon"),
    ("PersistMessages", "Y", "Turbojet always stores what it sends, for resends"),
    ("SocketUseSSL", "N", "set up TLS by hand: a sessions file's tls takes PEM files or a PKCS#12 bundle"),
    ("SendRedundantResendRequests", "N", "Turbojet doesn't send redundant ResendRequests"),
    ("CheckCompID", "Y", "Turbojet always checks CompIDs"),
];

/// QuickFIX/J's ProxyType (`http`, or `socks` with ProxyVersion 5, its default), ProxyHost,
/// ProxyPort and ProxyUser, as a sessions file's `proxy` URL.
fn proxy(session: &mut Session, problems: &mut Vec<String>) -> Option<String> {
    let kind = session.take("ProxyType")?;
    let version = session.take("ProxyVersion");
    let (host, port, user) = (session.take("ProxyHost"), session.take("ProxyPort"), session.take("ProxyUser"));
    let scheme = match (kind.as_str(), version.as_deref()) {
        ("http", _) => "http",
        ("socks", None | Some("5")) => "socks5",
        _ => {
            let version = version.map(|v| format!(" version {v}")).unwrap_or_default();
            problems
                .push(format!("{}: ProxyType {kind}{version}: Turbojet's proxies are HTTP and SOCKS5", session.name));
            return None;
        }
    };
    let (Some(host), Some(port)) = (host, port) else {
        problems.push(format!("{}: ProxyType needs ProxyHost and ProxyPort", session.name));
        return None;
    };
    let user = user.map(|user| format!("{user}@")).unwrap_or_default();
    Some(format!("{scheme}://{user}{host}:{port}"))
}

/// A `[SESSION]`, with `[DEFAULT]` merged in, and what's been taken from it.
struct Session {
    /// For messages: `[SESSION] 2 (FIX.4.4:EXEC->BANZAI)`.
    name: String,
    keys: BTreeMap<String, String>,
    taken: BTreeSet<String>,
}

impl Session {
    fn take(&mut self, key: &str) -> Option<String> {
        let value = self.keys.get(key).cloned()?;
        self.taken.insert(key.to_string());
        Some(value)
    }

    /// `key` as Y or N.
    fn flag(&mut self, key: &str, problems: &mut Vec<String>) -> Option<bool> {
        match self.take(key)?.as_str() {
            "Y" => Some(true),
            "N" => Some(false),
            other => {
                problems.push(format!("{}: {key} is '{other}', not Y or N", self.name));
                None
            }
        }
    }

    /// `key` as a whole number of seconds, as a duration.
    fn seconds(&mut self, key: &str, problems: &mut Vec<String>) -> Option<String> {
        let value = self.take(key)?;
        match value.parse::<u64>() {
            Ok(seconds) => Some(format!("{seconds}s")),
            Err(_) => {
                problems.push(format!("{}: {key} is '{value}', not a whole number of seconds", self.name));
                None
            }
        }
    }

    /// The keys left over: those listed in a comment, and a problem for each of the rest.
    fn leftovers(&self, problems: &mut Vec<String>) -> Vec<String> {
        let mut not_converted = Vec::new();
        for (key, value) in self.keys.iter().filter(|(key, _)| !self.taken.contains(*key)) {
            let only_default = ONLY_DEFAULT.iter().find(|(k, _, _)| k == key);
            if let Some((_, default, why)) = only_default {
                if value != default {
                    problems.push(format!("{}: {key}={value}: {why}", self.name));
                }
            } else if NOT_CONVERTED.contains(&key.as_str())
                || (NOT_CONVERTED_PREFIXES.iter().any(|prefix| key.starts_with(prefix))
                    && !SOCKET_KEYS_HANDLED.contains(&key.as_str())
                    && !is_tls_key(key))
            {
                not_converted.push(format!("{key}={value}"));
            } else if is_tls_key(key) {
                problems.push(format!(
                    "{}: {key}: set up TLS by hand, a sessions file's tls takes PEM files or a PKCS#12 bundle",
                    self.name
                ));
            } else {
                problems.push(format!("{}: {key} has no Turbojet equivalent", self.name));
            }
        }
        not_converted
    }
}

/// QuickFIX/J's key and trust stores, and quickfix-go's PEM files.
fn is_tls_key(key: &str) -> bool {
    key.starts_with("SocketKeyStore")
        || key.starts_with("SocketTrustStore")
        || key.starts_with("SocketCertificate")
        || key.starts_with("SocketPrivateKey")
        || key.starts_with("SocketCA")
        || matches!(
            key,
            "KeyStoreType"
                | "TrustStoreType"
                | "NeedClientAuth"
                | "EnabledProtocols"
                | "CipherSuites"
                | "SocketServerName"
                | "SocketInsecureSkipVerify"
                | "SocketMinimumTLSVersion"
        )
}

/// The `[SESSION]`s in `cfg`, each with `[DEFAULT]` merged in, or every syntax problem.
fn parse(cfg: &str) -> Result<Vec<Session>, Vec<String>> {
    let mut default = BTreeMap::new();
    let mut sessions: Vec<BTreeMap<String, String>> = Vec::new();
    let mut in_default = None;
    let mut problems = Vec::new();
    for (index, line) in cfg.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') {
            match line.to_ascii_uppercase().as_str() {
                "[DEFAULT]" => in_default = Some(true),
                "[SESSION]" => {
                    in_default = Some(false);
                    sessions.push(BTreeMap::new());
                }
                _ => problems.push(format!("line {}: unknown section {line}", index + 1)),
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            problems.push(format!("line {}: not key=value: {line}", index + 1));
            continue;
        };
        let (key, value) = (key.trim().to_string(), value.trim().to_string());
        match in_default {
            Some(true) => default.insert(key, value),
            Some(false) => sessions.last_mut().expect("a [SESSION] was opened").insert(key, value),
            None => {
                problems.push(format!("line {}: {key} before any section", index + 1));
                None
            }
        };
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    Ok(sessions
        .into_iter()
        .enumerate()
        .map(|(index, own)| {
            let mut keys = default.clone();
            keys.extend(own);
            let id = |key: &str| keys.get(key).map_or("?", String::as_str).to_string();
            let name = format!(
                "[SESSION] {} ({}:{}->{})",
                index + 1,
                id("BeginString"),
                id("SenderCompID"),
                id("TargetCompID")
            );
            Session { name, keys, taken: BTreeSet::new() }
        })
        .collect())
}

/// The sessions file being built.
#[derive(Default)]
struct Output {
    /// `[acceptor]`'s lines, and the session that set them, which the others must agree with.
    acceptor: Option<(Vec<String>, String)>,
    /// Disk stores by directory: their name and whether they sync.
    stores: BTreeMap<String, (String, bool)>,
    /// `[counterparty.X]` and `[initiator.X]` sections, each with its comment lines.
    sections: Vec<(String, Vec<String>, Vec<String>)>,
    /// Counterparty sections by CompID, for sessions that differ only in SubIDs.
    counterparties: BTreeMap<String, usize>,
}

impl Output {
    fn acceptor(&mut self, mut session: Session, problems: &mut Vec<String>) {
        let mut header = Vec::new();
        for (key, toml) in [("BeginString", "begin_string"), ("SenderCompID", "sender_comp_id")] {
            match session.take(key) {
                Some(value) => header.push(format!("{toml} = {}", quote(&value))),
                None => problems.push(format!("{}: no {key}", session.name)),
            }
        }
        let host = session.take("SocketAcceptAddress").or_else(|| session.take("SocketAcceptHost"));
        match session.take("SocketAcceptPort") {
            Some(port) => {
                header.push(format!("listen = {}", quote(&format!("{}:{port}", host.as_deref().unwrap_or("0.0.0.0")))))
            }
            None => problems.push(format!("{}: no SocketAcceptPort", session.name)),
        }
        if let Some(timeout) = session.seconds("LogonTimeout", problems) {
            header.push(format!("logon_timeout = {}", quote(&timeout)));
        }
        match &self.acceptor {
            None => self.acceptor = Some((header, session.name.clone())),
            Some((first, first_name)) if *first != header => problems.push(format!(
                "{}: its acceptor settings differ from {first_name}'s; a sessions file has one acceptor",
                session.name
            )),
            Some(_) => {}
        }
        let mut comments = Vec::new();
        for key in ["SenderSubID", "SenderLocationID", "TargetSubID", "TargetLocationID"] {
            if let Some(value) = session.take(key) {
                comments
                    .push(format!("{key}={value}: Turbojet's acceptor takes SubIDs and LocationIDs from each Logon"));
            }
        }
        if let Some(value) = session.take("HeartBtInt") {
            comments
                .push(format!("HeartBtInt={value}: an acceptor uses the HeartBtInt its counterparty's Logon asks for"));
        }
        if session.flag("ResetOnLogon", problems) == Some(true) {
            problems.push(format!(
                "{}: ResetOnLogon=Y: Turbojet's acceptor resets sequence numbers only when the Logon asks it to",
                session.name
            ));
        }
        if session.take("SessionQualifier").is_some() {
            problems.push(format!("{}: SessionQualifier: an acceptor's sessions have no qualifier", session.name));
        }
        let target = session.take("TargetCompID");
        let lines = self.session_keys(&mut session, problems);
        comments.extend(session.leftovers(problems).into_iter().map(|key| format!("{key}: not converted")));
        let Some(target) = target else {
            problems.push(format!("{}: no TargetCompID", session.name));
            return;
        };
        match self.counterparties.get(&target) {
            Some(&index) if self.sections[index].1 != lines => problems.push(format!(
                "{}: its settings differ from another session's with {target}; a counterparty has one set",
                session.name
            )),
            Some(_) => {}
            None => {
                self.counterparties.insert(target.clone(), self.sections.len());
                let comments = [vec![format!("From {}", session.name)], comments].concat();
                self.sections.push((format!("counterparty.{}", bare_or_quoted(&target)), lines, comments));
            }
        }
    }

    fn initiator(&mut self, mut session: Session, problems: &mut Vec<String>) {
        let mut lines = Vec::new();
        for (key, toml) in [
            ("BeginString", "begin_string"),
            ("SenderCompID", "sender_comp_id"),
            ("TargetCompID", "target_comp_id"),
            ("SenderSubID", "sender_sub_id"),
            ("SenderLocationID", "sender_location_id"),
            ("TargetSubID", "target_sub_id"),
            ("TargetLocationID", "target_location_id"),
            ("SessionQualifier", "qualifier"),
        ] {
            match session.take(key) {
                Some(value) => lines.push(format!("{toml} = {}", quote(&value))),
                None if matches!(key, "BeginString" | "SenderCompID" | "TargetCompID") => {
                    problems.push(format!("{}: no {key}", session.name));
                }
                None => {}
            }
        }
        lines.push(format!(
            "connect = [{}]",
            connect(&mut session, problems).iter().map(|a| quote(a)).collect::<Vec<_>>().join(", ")
        ));
        if let Some(interval) = session.seconds("HeartBtInt", problems) {
            lines.push(format!("heartbeat_interval = {}", quote(&interval)));
        }
        if let Some(interval) = session.seconds("ReconnectInterval", problems) {
            lines.push(format!("reconnect = {{ initial = {0}, max = {0} }}", quote(&interval)));
        }
        if session.flag("ResetOnLogon", problems) == Some(true) {
            lines.push("reset_on_logon = true".to_string());
        }
        if session.flag("EnableNextExpectedMsgSeqNum", problems) == Some(true) {
            lines.push("next_expected_msg_seq_num = true".to_string());
        }
        if let Some(timeout) = session.seconds("LogonTimeout", problems) {
            lines.push(format!("logon_timeout = {}", quote(&timeout)));
        }
        let mut notes = Vec::new();
        if let Some(url) = proxy(&mut session, problems) {
            lines.push(format!("proxy = {}", quote(&url)));
            if session.take("ProxyPassword").is_some() {
                notes.push("ProxyPassword: not converted; set proxy_password_env to a variable holding it".into());
            }
        }
        lines.extend(self.session_keys(&mut session, problems));
        let mut comments = vec![format!("From {}", session.name)];
        comments.extend(notes);
        comments.extend(session.leftovers(problems).into_iter().map(|key| format!("{key}: not converted")));
        let target = session.keys.get("TargetCompID").cloned().unwrap_or_default();
        let name = (1..)
            .map(|n| if n == 1 { target.clone() } else { format!("{target}_{n}") })
            .find(|name| {
                !self.sections.iter().any(|(section, _, _)| *section == format!("initiator.{}", bare_or_quoted(name)))
            })
            .expect("some suffix is free");
        self.sections.push((format!("initiator.{}", bare_or_quoted(&name)), lines, comments));
    }

    /// The keys a counterparty and an initiator share.
    fn session_keys(&mut self, session: &mut Session, problems: &mut Vec<String>) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(schedule) = schedule(session, problems) {
            lines.push(format!("schedule = {}", quote(&schedule)));
        }
        if let Some(timeout) = session.seconds("LogoutTimeout", problems) {
            lines.push(format!("logout_timeout = {}", quote(&timeout)));
        }
        let max_latency = session.seconds("MaxLatency", problems);
        match (session.flag("CheckLatency", problems), max_latency) {
            (Some(false), _) => lines.push("max_latency = \"off\"".to_string()),
            (_, Some(latency)) => lines.push(format!("max_latency = {}", quote(&latency))),
            _ => {}
        }
        if let Some(precision) = precision(session, problems) {
            lines.push(format!("timestamp_precision = {}", quote(precision)));
        }
        if let Some(chunk) = session.take("ResendRequestChunkSize") {
            match chunk.parse::<u64>() {
                Ok(0) => {}
                Ok(chunk) => lines.push(format!("resend_request_chunk = {chunk}")),
                Err(_) => problems.push(format!("{}: ResendRequestChunkSize is '{chunk}'", session.name)),
            }
        }
        if let Some(dir) = session.take("FileStorePath") {
            let sync = session.flag("FileStoreSync", problems).unwrap_or(false);
            let count = self.stores.len();
            let (name, synced) = self.stores.entry(dir.clone()).or_insert_with(|| (format!("file{}", count + 1), sync));
            if *synced != sync {
                problems.push(format!(
                    "{}: FileStoreSync differs from another session's with FileStorePath {dir}",
                    session.name
                ));
            }
            lines.push(format!("store = {}", quote(name)));
        }
        lines.extend(dictionaries(session, problems));
        lines
    }

    fn render(&self) -> String {
        let mut out = String::from(
            "# Converted from QuickFIX settings. Check it before use: paths are as the .cfg wrote them\n\
             # (QuickFIX reads them from where it runs, a sessions file from its own directory), and\n\
             # keys listed as not converted are set in code, or done Turbojet's way.\n",
        );
        if let Some((lines, _)) = &self.acceptor {
            write_section(&mut out, "acceptor", lines, &[]);
        }
        for (dir, (name, sync)) in &self.stores {
            let lines = vec!["kind = \"disk\"".to_string(), format!("dir = {}", quote(dir)), format!("fsync = {sync}")];
            write_section(&mut out, &format!("store.{name}"), &lines, &[]);
        }
        for (name, lines, comments) in &self.sections {
            write_section(&mut out, name, lines, comments);
        }
        out
    }
}

fn write_section(out: &mut String, name: &str, lines: &[String], comments: &[String]) {
    out.push('\n');
    for comment in comments {
        let _ = writeln!(out, "# {comment}");
    }
    let _ = writeln!(out, "[{name}]");
    for line in lines {
        let _ = writeln!(out, "{line}");
    }
}

/// An initiator's addresses: SocketConnectHost and SocketConnectPort, then the failover ones
/// numbered from 1.
fn connect(session: &mut Session, problems: &mut Vec<String>) -> Vec<String> {
    let mut addresses = Vec::new();
    for suffix in std::iter::once(String::new()).chain((1..).map(|n: u32| n.to_string())) {
        let host = session.take(&format!("SocketConnectHost{suffix}"));
        let port = session.take(&format!("SocketConnectPort{suffix}"));
        match (host, port) {
            (Some(host), Some(port)) => addresses.push(format!("{host}:{port}")),
            (None, None) => break,
            _ => {
                problems.push(format!(
                    "{}: SocketConnectHost{suffix} and SocketConnectPort{suffix} go together",
                    session.name
                ));
                break;
            }
        }
    }
    if addresses.is_empty() {
        problems.push(format!("{}: no SocketConnectHost and SocketConnectPort", session.name));
    }
    addresses
}

/// The schedule from StartTime, EndTime, StartDay, EndDay, Weekdays and TimeZone, or none for a
/// NonStopSession.
fn schedule(session: &mut Session, problems: &mut Vec<String>) -> Option<String> {
    let non_stop = session.flag("NonStopSession", problems) == Some(true);
    let (start, end) = (session.take("StartTime"), session.take("EndTime"));
    let (start_day, end_day) = (session.take("StartDay"), session.take("EndDay"));
    let weekdays = session.take("Weekdays");
    let zone = session.take("TimeZone").map(|zone| format!(" {zone}")).unwrap_or_default();
    if non_stop {
        return None;
    }
    let (Some(start), Some(end)) = (start, end) else {
        problems.push(format!("{}: StartTime and EndTime are needed unless NonStopSession=Y", session.name));
        return None;
    };
    match (start_day, end_day, weekdays) {
        (Some(start_day), Some(end_day), None) => {
            let (start_day, end_day) = (day(&start_day, session, problems)?, day(&end_day, session, problems)?);
            Some(format!("weekly {start_day} {start}-{end_day} {end}{zone}"))
        }
        (None, None, Some(days)) => {
            let days = days.split(',').map(|d| day(d.trim(), session, problems)).collect::<Option<Vec<_>>>()?;
            Some(format!("daily {start}-{end} {}{zone}", days.join(",")))
        }
        (None, None, None) => Some(format!("daily {start}-{end}{zone}")),
        _ => {
            problems.push(format!("{}: StartDay and EndDay go together, and not with Weekdays", session.name));
            None
        }
    }
}

/// A QuickFIX day (`Monday`, `mon`, `Mo`) as a schedule writes it.
fn day(value: &str, session: &Session, problems: &mut Vec<String>) -> Option<&'static str> {
    const DAYS: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
    let lower = value.to_ascii_lowercase();
    let prefix = lower.get(..lower.len().min(3)).filter(|prefix| prefix.len() >= 2);
    let found = prefix.and_then(|prefix| DAYS.iter().find(|day| day.starts_with(prefix)));
    if found.is_none() {
        problems.push(format!("{}: '{value}' isn't a day", session.name));
    }
    found.copied()
}

/// The timestamp precision from TimeStampPrecision or MillisecondsInTimeStamp.
fn precision(session: &mut Session, problems: &mut Vec<String>) -> Option<&'static str> {
    let millis = session.flag("MillisecondsInTimeStamp", problems);
    match session.take("TimeStampPrecision").as_deref() {
        Some("SECONDS") => Some("seconds"),
        Some("MILLIS") => Some("millis"),
        Some("MICROS") => Some("micros"),
        Some("NANOS" | "PICOS") => Some("nanos"),
        Some(other) => {
            problems
                .push(format!("{}: TimeStampPrecision '{other}' isn't SECONDS, MILLIS, MICROS or NANOS", session.name));
            None
        }
        None => (millis == Some(false)).then_some("seconds"),
    }
}

/// The dictionaries messages are checked against: `dictionary`, or for FIXT `appl_versions`.
fn dictionaries(session: &mut Session, problems: &mut Vec<String>) -> Vec<String> {
    let default_version = session.take("DefaultApplVerID");
    let use_dictionary = session.flag("UseDataDictionary", problems).unwrap_or(true);
    let data = session.take("DataDictionary");
    let mut app: Vec<(String, String)> = Vec::new();
    for key in session.keys.keys().filter(|key| key.starts_with("AppDataDictionary")).cloned().collect::<Vec<_>>() {
        let path = session.take(&key).expect("the key is there");
        let version = key.strip_prefix("AppDataDictionary").unwrap_or_default().trim_start_matches('.').to_string();
        app.push((version, path));
    }
    let mut lines = Vec::new();
    if !use_dictionary {
        return lines;
    }
    if let Some(path) = data {
        lines.push(format!("dictionary = {}", quote(&path)));
    }
    let mut versions = Vec::new();
    if let Some(default) = &default_version {
        versions.push((
            appl_ver_id(default, session, problems),
            app.iter().find(|(v, _)| v.is_empty()).map(|(_, p)| p.clone()),
        ));
    }
    for (version, path) in app.into_iter().filter(|(v, _)| !v.is_empty()) {
        versions.push((appl_ver_id(&version, session, problems), Some(path)));
    }
    if default_version.is_none() && versions.is_empty() && session.keys.contains_key("AppDataDictionary") {
        problems.push(format!("{}: AppDataDictionary without DefaultApplVerID", session.name));
    }
    if !versions.is_empty() {
        let entries = versions.into_iter().filter_map(|(id, path)| {
            let id = id?;
            Some(match path {
                Some(path) => format!("{{ id = {}, dictionary = {} }}", quote(id), quote(&path)),
                None => quote(id),
            })
        });
        lines.push(format!("appl_versions = [{}]", entries.collect::<Vec<_>>().join(", ")));
    }
    lines
}

/// The ApplVerID code for a QuickFIX version name (`FIX.5.0SP2`) or code (`9`).
fn appl_ver_id(version: &str, session: &Session, problems: &mut Vec<String>) -> Option<&'static str> {
    let code = match version {
        "FIX.2.7" | "0" => "0",
        "FIX.3.0" | "1" => "1",
        "FIX.4.0" | "2" => "2",
        "FIX.4.1" | "3" => "3",
        "FIX.4.2" | "4" => "4",
        "FIX.4.3" | "5" => "5",
        "FIX.4.4" | "6" => "6",
        "FIX.5.0" | "7" => "7",
        "FIX.5.0SP1" | "8" => "8",
        "FIX.5.0SP2" | "9" => "9",
        _ => {
            problems.push(format!("{}: '{version}' isn't an application version", session.name));
            return None;
        }
    };
    Some(code)
}

/// A TOML string.
fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// A TOML key: bare if it can be.
fn bare_or_quoted(key: &str) -> String {
    if !key.is_empty() && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        key.to_string()
    } else {
        quote(key)
    }
}
