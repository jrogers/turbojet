//! Converting QuickFIX settings files: what each maps to, that the result loads, and that
//! everything that doesn't map is reported.

use turbojet_config::SessionsFile;
use turbojet_config::quickfix::convert;

/// An acceptor in the style of QuickFIX/J's Executor example: two sessions sharing one port, a
/// file store, and keys Turbojet sets in code.
const ACCEPTOR: &str = "\
# Executor
[DEFAULT]
ConnectionType=acceptor
SocketAcceptPort=9880
SenderCompID=EXEC
StartTime=00:00:00
EndTime=00:00:00
HeartBtInt=30
FileStorePath=data/executor
FileLogPath=data/executor/logs
SocketTcpNoDelay=Y
ResetOnLogout=N

[SESSION]
BeginString=FIX.4.4
TargetCompID=BANZAI

[SESSION]
BeginString=FIX.4.4
TargetCompID=CLIENT2
StartTime=08:00:00
EndTime=17:00:00
Weekdays=Mon,Tue,Wed,Thu,Fri
CheckLatency=N
";

const ACCEPTOR_TOML: &str = r#"# Converted from QuickFIX settings. Check it before use: paths are as the .cfg wrote them
# (QuickFIX reads them from where it runs, a sessions file from its own directory), and
# keys listed as not converted are set in code, or done Turbojet's way.

[acceptor]
begin_string = "FIX.4.4"
sender_comp_id = "EXEC"
listen = "0.0.0.0:9880"

[store.file1]
kind = "disk"
dir = "data/executor"
fsync = false

# From [SESSION] 1 (FIX.4.4:EXEC->BANZAI)
# HeartBtInt=30: an acceptor uses the HeartBtInt its counterparty's Logon asks for
# FileLogPath=data/executor/logs: not converted
# SocketTcpNoDelay=Y: not converted
[counterparty.BANZAI]
schedule = "daily 00:00:00-00:00:00"
store = "file1"

# From [SESSION] 2 (FIX.4.4:EXEC->CLIENT2)
# HeartBtInt=30: an acceptor uses the HeartBtInt its counterparty's Logon asks for
# FileLogPath=data/executor/logs: not converted
# SocketTcpNoDelay=Y: not converted
[counterparty.CLIENT2]
schedule = "daily 08:00:00-17:00:00 mon,tue,wed,thu,fri"
max_latency = "off"
store = "file1"
"#;

/// An initiator in the style of QuickFIX/J's Banzai example, with a failover address.
const INITIATOR: &str = "\
[DEFAULT]
ConnectionType=initiator
ReconnectInterval=5
HeartBtInt=30
StartDay=sun
StartTime=17:00:00
EndDay=Friday
EndTime=17:00:00
TimeZone=America/New_York

[SESSION]
BeginString=FIX.4.2
SenderCompID=BANZAI
TargetCompID=EXEC
SocketConnectHost=localhost
SocketConnectPort=9880
SocketConnectHost1=backup.example
SocketConnectPort1=9881
ResetOnLogon=Y
TimeStampPrecision=MICROS
";

const INITIATOR_TOML: &str = r#"
# From [SESSION] 1 (FIX.4.2:BANZAI->EXEC)
[initiator.EXEC]
begin_string = "FIX.4.2"
sender_comp_id = "BANZAI"
target_comp_id = "EXEC"
connect = ["localhost:9880", "backup.example:9881"]
heartbeat_interval = "30s"
reconnect = { initial = "5s", max = "5s" }
reset_on_logon = true
schedule = "weekly sun 17:00:00-fri 17:00:00 America/New_York"
timestamp_precision = "micros"
"#;

#[test]
fn an_acceptors_sessions_become_counterparties() {
    assert_eq!(convert(ACCEPTOR).unwrap(), ACCEPTOR_TOML);
}

#[test]
fn an_initiators_session_becomes_an_initiator() {
    let toml = convert(INITIATOR).unwrap();
    assert!(toml.ends_with(INITIATOR_TOML), "{toml}");
}

/// What the converter writes is a sessions file that loads.
#[test]
fn the_converted_file_loads() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sessions.toml");
    std::fs::write(&path, convert(ACCEPTOR).unwrap()).unwrap();
    let sessions = SessionsFile::load(&path).unwrap();
    assert_eq!(sessions.listen().as_deref(), Some("0.0.0.0:9880"));
    // Named time zones need the tz feature; the initiator's schedule has one.
    if cfg!(feature = "tz") {
        std::fs::write(&path, convert(INITIATOR).unwrap()).unwrap();
        assert_eq!(SessionsFile::load(&path).unwrap().initiator_names(), ["EXEC"]);
    }
}

/// A FIXT session's application dictionaries, by version.
#[test]
fn fixt_dictionaries_become_application_versions() {
    let cfg = "\
[SESSION]
ConnectionType=initiator
BeginString=FIXT.1.1
SenderCompID=A
TargetCompID=B
SocketConnectHost=h
SocketConnectPort=1
NonStopSession=Y
DefaultApplVerID=FIX.5.0SP2
TransportDataDictionary=FIXT11.xml
AppDataDictionary=FIX50SP2.xml
AppDataDictionary.FIX.4.4=FIX44.xml
";
    let toml = convert(cfg).unwrap();
    assert!(
        toml.contains(
            r#"appl_versions = [{ id = "9", dictionary = "FIX50SP2.xml" }, { id = "6", dictionary = "FIX44.xml" }]"#
        ),
        "{toml}"
    );
    assert!(toml.contains("# TransportDataDictionary=FIXT11.xml: not converted"), "{toml}");
    assert!(!toml.contains("schedule"), "a NonStopSession has none: {toml}");
}

/// Every problem is reported at once, each naming its session and key.
#[test]
fn what_doesnt_map_is_reported_together() {
    let cfg = "\
[DEFAULT]
ConnectionType=acceptor
SenderCompID=EXEC
NonStopSession=Y

[SESSION]
BeginString=FIX.4.4
TargetCompID=A
SocketAcceptPort=9880
ResetOnLogon=Y
ResetOnDisconnect=Y
SocketKeyStore=keys.jks
ValidOrderTypes=1,2

[SESSION]
BeginString=FIX.4.4
TargetCompID=B
SocketAcceptPort=9881
";
    let problems = convert(cfg).unwrap_err().problems;
    let expected = [
        "[SESSION] 1 (FIX.4.4:EXEC->A): ResetOnLogon=Y: Turbojet's acceptor resets sequence numbers only when the Logon asks it to",
        "[SESSION] 1 (FIX.4.4:EXEC->A): ResetOnDisconnect=Y: Turbojet doesn't reset sequence numbers on a disconnection",
        "[SESSION] 1 (FIX.4.4:EXEC->A): SocketKeyStore: set up TLS by hand, a sessions file's tls takes PEM files or a PKCS#12 bundle",
        "[SESSION] 1 (FIX.4.4:EXEC->A): ValidOrderTypes has no Turbojet equivalent",
        "[SESSION] 2 (FIX.4.4:EXEC->B): its acceptor settings differ from [SESSION] 1 (FIX.4.4:EXEC->A)'s; a sessions file has one acceptor",
    ];
    for problem in expected {
        assert!(problems.iter().any(|p| p == problem), "missing {problem:?} in {problems:#?}");
    }
    assert_eq!(problems.len(), expected.len(), "{problems:#?}");
}

#[test]
fn days_are_read_as_quickfix_writes_them() {
    let cfg = |days: &str| {
        format!(
            "[SESSION]\nConnectionType=initiator\nBeginString=FIX.4.4\nSenderCompID=A\nTargetCompID=B\n\
             SocketConnectHost=h\nSocketConnectPort=1\nStartTime=08:00:00\nEndTime=17:00:00\nWeekdays={days}\n"
        )
    };
    assert!(convert(&cfg("Monday, tu,WED")).unwrap().contains("daily 08:00:00-17:00:00 mon,tue,wed"));
    for bad in ["x", "é", "funday"] {
        let problems = convert(&cfg(bad)).unwrap_err().problems;
        assert_eq!(problems, [format!("[SESSION] 1 (FIX.4.4:A->B): '{bad}' isn't a day")], "{bad}");
    }
}

#[test]
fn syntax_errors_are_reported_by_line() {
    let problems = convert("HeartBtInt=30\n[SESSION]\nnot a key\n[OTHER]\n").unwrap_err().problems;
    assert_eq!(
        problems,
        [
            "line 1: HeartBtInt before any section",
            "line 3: not key=value: not a key",
            "line 4: unknown section [OTHER]"
        ]
    );
}

/// QuickFIX/J's proxy settings become `proxy`, the password left to `proxy_password_env`.
#[test]
fn proxies_convert_without_their_passwords() {
    let cfg = |settings: &str| {
        format!(
            "[SESSION]\nConnectionType=initiator\nBeginString=FIX.4.4\nSenderCompID=A\nTargetCompID=B\nSocketConnectHost=venue\nSocketConnectPort=9876\nNonStopSession=Y\n{settings}"
        )
    };
    let toml =
        convert(&cfg("ProxyType=socks\nProxyHost=proxy\nProxyPort=1080\nProxyUser=firm\nProxyPassword=pw\n")).unwrap();
    assert!(toml.contains("proxy = \"socks5://firm@proxy:1080\""), "{toml}");
    assert!(toml.contains("# ProxyPassword: not converted; set proxy_password_env"), "{toml}");
    assert!(!toml.contains("pw\""), "the password isn't written: {toml}");
    let toml = convert(&cfg("ProxyType=http\nProxyHost=proxy\nProxyPort=3128\n")).unwrap();
    assert!(toml.contains("proxy = \"http://proxy:3128\""), "{toml}");

    let problems = convert(&cfg("ProxyType=socks\nProxyVersion=4\nProxyHost=proxy\nProxyPort=1080\n")).unwrap_err();
    assert!(
        problems.to_string().contains("ProxyType socks version 4: Turbojet's proxies are HTTP and SOCKS5"),
        "{problems}"
    );
    let problems = convert(&cfg("ProxyType=http\nProxyHost=proxy\n")).unwrap_err();
    assert!(problems.to_string().contains("ProxyType needs ProxyHost and ProxyPort"), "{problems}");
}
