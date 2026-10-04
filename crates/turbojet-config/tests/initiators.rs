//! Initiators from a sessions file, logging on to test acceptors, and reloaded.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::{Acceptor, Application, MemoryStorage, SessionConfig, SessionHandle, SessionId};
use turbojet_config::{Changes, Initiators, SessionsFile};

/// Reports logons and logouts by the counterparty's CompID.
struct Recorder(mpsc::UnboundedSender<String>);

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.0.send(format!("logon {}", session.id().target_comp_id));
    }
    fn on_logout(&self, session: &SessionId) {
        let _ = self.0.send(format!("logout {}", session.target_comp_id));
    }
}

/// A venue the initiators log on to, as "VENUE", reporting what it sees.
struct Venue {
    acceptor: Acceptor,
    addr: String,
    events: mpsc::UnboundedReceiver<String>,
}

impl Venue {
    async fn start() -> Self {
        let (tx, events) = mpsc::unbounded_channel();
        let acceptor = Acceptor::new(
            SessionConfig::new("FIX.4.4", "VENUE"),
            Arc::new(MemoryStorage::new()),
            Arc::new(Recorder(tx)),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        tokio::spawn(acceptor.clone().serve(listener));
        Self { acceptor, addr, events }
    }

    async fn next(&mut self) -> String {
        tokio::time::timeout(Duration::from_secs(5), self.events.recv()).await.expect("timed out").expect("closed")
    }
}

/// A file of initiators from FIRM, one per `(name, address)`, reconnecting quickly.
fn file(initiators: &[(&str, &str)]) -> String {
    let mut text = String::new();
    for (name, addr) in initiators {
        text += &format!(
            "[initiator.{name}]\nbegin_string = \"FIX.4.4\"\nsender_comp_id = \"{name}\"\ntarget_comp_id = \"VENUE\"\n\
             connect = [\"{addr}\"]\nreconnect = {{ initial = \"100ms\", max = \"500ms\" }}\n\n"
        );
    }
    text
}

struct Firm {
    _dir: tempfile::TempDir,
    path: PathBuf,
    sessions: SessionsFile,
    initiators: Initiators,
}

impl Firm {
    fn start(text: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sessions.toml");
        std::fs::write(&path, text).unwrap();
        let sessions = SessionsFile::load(&path).unwrap();
        let (tx, _) = mpsc::unbounded_channel();
        let initiators = sessions.initiators(Arc::new(Recorder(tx))).unwrap();
        Self { _dir: dir, path, sessions, initiators }
    }

    fn reload(&self, text: &str) -> Changes {
        std::fs::write(&self.path, text).unwrap();
        self.sessions.reload_all(None, &self.initiators).unwrap()
    }
}

#[tokio::test]
async fn initiators_start_and_log_on() {
    let mut venue = Venue::start().await;
    let firm = Firm::start(&file(&[("ALPHA", &venue.addr), ("BETA", &venue.addr)]));
    let mut logons = [venue.next().await, venue.next().await];
    logons.sort();
    assert_eq!(logons, ["logon ALPHA", "logon BETA"]);
    assert_eq!(firm.initiators.names(), ["ALPHA", "BETA"]);
    let handle = firm.initiators.handle("ALPHA").unwrap();
    assert!(handle.is_connected());
    assert!(firm.initiators.handle("GAMMA").is_none());
    firm.initiators.shutdown(Some("closing")).await;
    let mut logouts = [venue.next().await, venue.next().await];
    logouts.sort();
    assert_eq!(logouts, ["logout ALPHA", "logout BETA"]);
}

#[tokio::test]
async fn a_reload_starts_added_initiators_and_stops_removed_ones() {
    let mut venue = Venue::start().await;
    let firm = Firm::start(&file(&[("ALPHA", &venue.addr)]));
    assert_eq!(venue.next().await, "logon ALPHA");
    let changes = firm.reload(&file(&[("BETA", &venue.addr)]));
    assert_eq!(changes, Changes { started: vec!["BETA".into()], stopped: vec!["ALPHA".into()], ..Changes::default() });
    let mut events = [venue.next().await, venue.next().await];
    events.sort();
    assert_eq!(events, ["logon BETA", "logout ALPHA"]);
    assert_eq!(firm.initiators.names(), ["BETA"]);
}

#[tokio::test]
async fn a_new_address_applies_from_the_next_connection() {
    let (mut first, mut second) = (Venue::start().await, Venue::start().await);
    let firm = Firm::start(&file(&[("ALPHA", &first.addr)]));
    assert_eq!(first.next().await, "logon ALPHA");
    let handle = firm.initiators.handle("ALPHA").unwrap();
    let changes = firm.reload(&file(&[("ALPHA", &second.addr)]));
    assert_eq!(changes, Changes { reconfigured: vec!["ALPHA".into()], ..Changes::default() });
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(second.events.try_recv().is_err(), "the working session is left alone");
    // When the session ends, it reconnects to the new address.
    first.acceptor.session("ALPHA").logout(Some("moving")).unwrap();
    assert_eq!(first.next().await, "logout ALPHA");
    assert_eq!(second.next().await, "logon ALPHA");
    assert!(handle.is_connected(), "the handle from before still works");
}

#[tokio::test]
async fn reloading_initiators_needs_reload_all() {
    let venue = Venue::start().await;
    let firm = Firm::start(&format!(
        "[acceptor]\nbegin_string = \"FIX.4.4\"\nsender_comp_id = \"FIRM\"\nlisten = \"127.0.0.1:0\"\n\n{}",
        file(&[("ALPHA", &venue.addr)])
    ));
    let (tx, _) = mpsc::unbounded_channel();
    let acceptor = firm.sessions.acceptor(Arc::new(Recorder(tx))).unwrap();
    let error = firm.sessions.reload(&acceptor).unwrap_err().to_string();
    assert!(error.ends_with("initiator: section: reload a file with initiators with reload_all"), "{error}");
    assert!(firm.sessions.reload_all(Some(&acceptor), &firm.initiators).unwrap().is_empty());
}
