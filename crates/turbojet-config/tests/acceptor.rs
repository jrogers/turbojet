//! An acceptor made from a sessions file, over real connections, and reloaded.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::{
    Acceptor, Application, Disconnect, Initiator, InitiatorConfig, MemoryStorage, SessionConfig, SessionHandle,
    SessionId, SessionStorage,
};
use turbojet_config::{Changes, SessionsFile};

/// Reports logons, logouts and cancels on disconnect by the counterparty's CompID.
struct Recorder(mpsc::UnboundedSender<String>);

impl Application for Recorder {
    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.0.send(format!("logon {}", session.id().target_comp_id));
    }
    fn on_logout(&self, session: &SessionHandle, _ended: Disconnect) {
        let _ = self.0.send(format!("logout {}", session.id().target_comp_id));
    }
    fn on_cancel_on_disconnect(&self, session: &SessionHandle, _ended: Disconnect) {
        let _ = self.0.send(format!("cancel {}", session.id().target_comp_id));
    }
}

const FILE: &str = r#"
[acceptor]
begin_string = "FIX.4.4"
sender_comp_id = "VENUE"
listen = "127.0.0.1:0"

[defaults]
heartbeat = { min = "1s", max = "60s" }

[counterparty.BROKER]
store = "broker"

[counterparty.FUND]
"#;

struct Venue {
    _dir: tempfile::TempDir,
    path: PathBuf,
    sessions: SessionsFile,
    acceptor: Acceptor,
    addr: String,
    events: mpsc::UnboundedReceiver<String>,
    broker_store: Arc<MemoryStorage>,
}

impl Venue {
    async fn start(text: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sessions.toml");
        std::fs::write(&path, text).unwrap();
        let broker_store = Arc::new(MemoryStorage::new());
        let sessions = SessionsFile::builder(&path).with_store("broker", broker_store.clone()).load().unwrap();
        let (tx, events) = mpsc::unbounded_channel();
        let acceptor = sessions.acceptor(Arc::new(Recorder(tx))).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        tokio::spawn(acceptor.clone().serve(listener));
        Self { _dir: dir, path, sessions, acceptor, addr, events, broker_store }
    }

    fn rewrite(&self, text: &str) {
        std::fs::write(&self.path, text).unwrap();
    }

    fn reload(&self) -> Result<Changes, String> {
        self.sessions.reload(&self.acceptor).map_err(|e| e.to_string())
    }

    async fn next(&mut self) -> String {
        tokio::time::timeout(Duration::from_secs(5), self.events.recv()).await.expect("timed out").expect("closed")
    }

    fn initiator(&self, comp_id: &str, heartbeat: u64) -> Initiator {
        let (tx, _) = mpsc::unbounded_channel();
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", comp_id), "VENUE");
        config.heartbeat_interval = Duration::from_secs(heartbeat);
        Initiator::new(self.addr.clone(), config, Arc::new(MemoryStorage::new()), Arc::new(Recorder(tx))).unwrap()
    }

    /// Logs `comp_id` on and leaves it connected.
    async fn log_on(&mut self, comp_id: &str) -> SessionHandle {
        let initiator = self.initiator(comp_id, 30);
        let handle = initiator.handle();
        tokio::spawn(async move { initiator.connect_once().await });
        assert_eq!(self.next().await, format!("logon {comp_id}"));
        handle
    }
}

fn id(comp_id: &str) -> SessionId {
    SessionId::new("FIX.4.4", "VENUE", comp_id)
}

#[tokio::test]
async fn listed_counterparties_log_on_into_their_own_stores() {
    let mut venue = Venue::start(FILE).await;
    assert!(venue.initiator("STRANGER", 30).connect_once().await.is_err(), "unknown counterparties are refused");
    let broker = venue.log_on("BROKER").await;
    assert_eq!(broker.sequence_numbers().await.unwrap().next_outgoing, 2);
    broker.logout(None).unwrap();
    assert_eq!(venue.next().await, "logout BROKER");
    // Our Logon was 1 and our Logout 2.
    assert_eq!(venue.broker_store.open(&id("BROKER")).unwrap().next_outgoing(), 3);
    assert_eq!(venue.sessions.counterparties(), ["BROKER", "FUND"]);
}

#[tokio::test]
async fn a_reload_applies_from_the_next_logon() {
    let mut venue = Venue::start(FILE).await;
    venue.rewrite(
        &FILE.replace("[counterparty.FUND]", "[counterparty.FUND]\nheartbeat = { min = \"1s\", max = \"10s\" }"),
    );
    let changes = venue.reload().unwrap();
    assert_eq!(changes, Changes { changed: vec!["FUND".into()], ..Changes::default() });
    assert!(venue.initiator("FUND", 30).connect_once().await.is_err(), "30 s is now outside its range");
    venue.log_on("BROKER").await;
}

/// Connects `initiator`, leaving it to run.
fn connect(initiator: &Initiator) {
    let initiator = initiator.clone();
    tokio::spawn(async move { initiator.connect_once().await });
}

#[tokio::test]
async fn cancel_on_disconnect_reloaded_applies_from_the_next_logon() {
    let mut venue = Venue::start(FILE).await;
    // One initiator throughout, so its sequence numbers carry on from one logon to the next.
    let fund = venue.initiator("FUND", 30);
    connect(&fund);
    assert_eq!(venue.next().await, "logon FUND");
    venue.rewrite(
        &FILE.replace("[counterparty.FUND]", "[counterparty.FUND]\ncancel_on_disconnect = \"disconnect_or_logout\""),
    );
    assert_eq!(venue.reload().unwrap(), Changes { changed: vec!["FUND".into()], ..Changes::default() });
    // The session connected keeps its settings: its Logout cancels nothing. A cancel for it, or
    // for BROKER, would come straight after its logout, failing the next logon assert.
    fund.handle().logout(None).unwrap();
    assert_eq!(venue.next().await, "logout FUND");
    venue.log_on("BROKER").await.logout(None).unwrap();
    assert_eq!(venue.next().await, "logout BROKER");
    // From the next Logon, it's on. The grace is the default, none: the cancel follows the logout
    // at once.
    connect(&fund);
    assert_eq!(venue.next().await, "logon FUND");
    fund.handle().logout(None).unwrap();
    assert_eq!([venue.next().await, venue.next().await], ["logout FUND", "cancel FUND"]);
}

#[tokio::test]
async fn a_counterparty_removed_is_logged_out() {
    let mut venue = Venue::start(FILE).await;
    venue.log_on("FUND").await;
    venue.rewrite(&FILE.replace("[counterparty.FUND]", "[counterparty.NEWCO]"));
    let changes = venue.reload().unwrap();
    assert_eq!(changes.added, ["NEWCO"]);
    assert_eq!(changes.removed, ["FUND"]);
    assert_eq!(changes.logged_out, ["FUND"]);
    assert_eq!(venue.next().await, "logout FUND");
    venue.log_on("NEWCO").await;
}

#[tokio::test]
async fn a_bad_reload_keeps_the_file_in_use() {
    let mut venue = Venue::start(FILE).await;
    for (text, error) in [
        (
            FILE.replace("[counterparty.FUND]", "[counterparty.FUND]\nmax_latency = \"soon\""),
            "counterparty FUND: max_latency",
        ),
        (FILE.replace("\"VENUE\"", "\"OTHER\""), "acceptor: sender_comp_id: can't change until a restart"),
        (FILE.replace("store = \"broker\"", "store = \"memory\""), "counterparty BROKER: store: can't change"),
        ("[acceptor".to_string(), "TOML parse error"),
    ] {
        venue.rewrite(&text);
        let reloaded = venue.reload().unwrap_err();
        assert!(reloaded.contains(error), "{reloaded}\n  should mention {error}");
        assert!(reloaded.starts_with(&venue.path.display().to_string()), "names the file: {reloaded}");
    }
    venue.log_on("FUND").await;
}

#[tokio::test]
async fn under_admit_unlisted_counterparties_get_the_defaults() {
    let mut venue = Venue::start(&FILE.replace("listen", "unknown = \"admit\"\nlisten")).await;
    venue.log_on("STRANGER").await;
    // Removing a counterparty then logs no one out: it's admitted with the defaults.
    venue.log_on("FUND").await;
    venue.rewrite(&FILE.replace("listen", "unknown = \"admit\"\nlisten").replace("[counterparty.FUND]", ""));
    assert_eq!(venue.reload().unwrap(), Changes { removed: vec!["FUND".into()], ..Changes::default() });
}

#[test]
fn a_missing_file_is_named() {
    let error = SessionsFile::load(Path::new("/nonexistent/sessions.toml")).unwrap_err().to_string();
    assert!(error.starts_with("/nonexistent/sessions.toml: "), "{error}");
}

#[tokio::test]
async fn allow_limits_where_connections_come_from_and_reloads() {
    let elsewhere = FILE.replace("listen = \"127.0.0.1:0\"", "listen = \"127.0.0.1:0\"\nallow = [\"10.0.0.0/8\"]");
    let mut venue = Venue::start(&elsewhere).await;
    let refused = venue.initiator("BROKER", 30).connect_once().await.unwrap_err();
    assert!(venue.events.try_recv().is_err(), "closed before a Logon: {refused}");

    venue.rewrite(&elsewhere.replace("[\"10.0.0.0/8\"]", "[\"10.0.0.0/8\", \"127.0.0.1\"]"));
    venue.reload().unwrap();
    venue.log_on("BROKER").await;

    venue.rewrite(&FILE.replace("listen = \"127.0.0.1:0\"", "listen = \"127.0.0.1:0\"\nallow = [\"10.1.2.0/16\"]"));
    assert!(venue.reload().unwrap_err().contains("acceptor: allow: '10.1.2.0/16' has bits set past its prefix"));
}
