//! Initiator failover between a primary and backup acceptors.

use std::io;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, ConnectionInfo, Context, Disconnect, Endpoint, Initiator, InitiatorConfig, MemoryStorage,
    Message, MessageReject, MsgType, SessionConfig, SessionHandle, SessionId, SessionStorage,
};

#[derive(Debug, PartialEq)]
enum Event {
    LoggedOn(&'static str),
    LoggedOut(&'static str),
    Message(&'static str, Message),
}

/// Reports callbacks tagged with the name of the side they happened on.
struct Recorder {
    name: &'static str,
    events: mpsc::UnboundedSender<Event>,
    refuse_logon: bool,
}

impl Application for Recorder {
    fn verify_logon(&self, _session: &SessionId, _logon: &Message, _connection: &ConnectionInfo) -> Result<(), String> {
        if self.refuse_logon { Err(format!("{} is not accepting logons", self.name)) } else { Ok(()) }
    }

    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.events.send(Event::LoggedOn(self.name));
    }

    fn on_logout(&self, _session: &SessionId, _ended: Disconnect) {
        let _ = self.events.send(Event::LoggedOut(self.name));
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(Event::Message(self.name, msg.clone()));
        Ok(())
    }
}

struct Harness {
    events: mpsc::UnboundedReceiver<Event>,
    sender: mpsc::UnboundedSender<Event>,
    /// Shared by all acceptors, as a replicated primary/DR pair would share session state.
    acceptor_storage: Arc<dyn SessionStorage>,
}

impl Harness {
    fn new() -> Self {
        let (sender, events) = mpsc::unbounded_channel();
        Self { events, sender, acceptor_storage: Arc::new(MemoryStorage::new()) }
    }

    fn app(&self, name: &'static str, refuse_logon: bool) -> Arc<Recorder> {
        Arc::new(Recorder { name, events: self.sender.clone(), refuse_logon })
    }

    /// Starts an acceptor on `addr` (or a free port) and returns its address.
    async fn acceptor(&self, name: &'static str, addr: Option<&str>, refuse_logon: bool) -> (Acceptor, String) {
        let config = SessionConfig::new("FIX.4.2", "SERVER");
        let acceptor = Acceptor::new(config, self.acceptor_storage.clone(), self.app(name, refuse_logon));
        let listener = TcpListener::bind(addr.unwrap_or("127.0.0.1:0")).await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        tokio::spawn(acceptor.clone().serve(listener));
        (acceptor, addr)
    }

    fn initiator(&self, primary: &str) -> Initiator {
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
        config.reconnect = turbojet::ReconnectPolicy::fixed(Duration::from_millis(100));
        config.connect_timeout = Duration::from_secs(2);
        Initiator::new(primary, config, Arc::new(MemoryStorage::new()), self.app("client", false))
    }

    async fn next(&mut self) -> Event {
        timeout(Duration::from_secs(5), self.events.recv()).await.expect("timed out waiting for event").unwrap()
    }

    /// Waits for the named side to log on, skipping unrelated events.
    async fn logged_on(&mut self, name: &'static str) {
        loop {
            if self.next().await == Event::LoggedOn(name) {
                return;
            }
        }
    }
}

/// An address with nothing listening on it.
async fn dead_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().to_string()
}

#[tokio::test]
async fn fails_over_when_the_primary_is_down() {
    let mut h = Harness::new();
    let primary = dead_address().await;
    let (_backup, backup_addr) = h.acceptor("backup", None, false).await;
    let initiator = h.initiator(&primary).with_failover(backup_addr.as_str());
    tokio::spawn(initiator.run());

    h.logged_on("backup").await;
    h.logged_on("client").await;
}

#[tokio::test]
async fn fails_over_when_the_primary_refuses_logon() {
    let mut h = Harness::new();
    let (_primary, primary_addr) = h.acceptor("primary", None, true).await;
    let (_backup, backup_addr) = h.acceptor("backup", None, false).await;
    let initiator = h.initiator(&primary_addr).with_failover(backup_addr.as_str());
    tokio::spawn(initiator.run());

    h.logged_on("backup").await;
}

#[tokio::test]
async fn tries_backups_in_order() {
    let mut h = Harness::new();
    let (_second, second_addr) = h.acceptor("second-backup", None, false).await;
    let (_first, first_addr) = h.acceptor("first-backup", None, false).await;
    let initiator = h
        .initiator(&dead_address().await)
        .with_failover(Endpoint::new(first_addr))
        .with_failover(Endpoint::new(second_addr));
    assert_eq!(initiator.endpoints().len(), 3);
    tokio::spawn(initiator.run());

    h.logged_on("first-backup").await;
}

#[tokio::test]
async fn fails_back_to_the_primary_once_it_is_available() {
    let mut h = Harness::new();
    let primary_addr = dead_address().await;
    let (backup, backup_addr) = h.acceptor("backup", None, false).await;
    let initiator = h.initiator(&primary_addr).with_failover(backup_addr.as_str());
    let handle = initiator.handle();
    tokio::spawn(initiator.run());
    h.logged_on("backup").await;

    // The primary comes back; the live session on the backup is left alone until it ends.
    let (_primary, _) = h.acceptor("primary", Some(&primary_addr), false).await;
    backup.session("CLIENT").logout(Some("maintenance")).unwrap();
    h.logged_on("primary").await;
    // The client may still be awaiting the Logon reply here; the send is queued until it arrives.

    // Same FIX session: the client's sequence numbers carry on from the backup session
    // (Logon=1, Logout=2, Logon=3), so its next message is 4.
    handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A")).unwrap();
    loop {
        if let Event::Message("primary", msg) = h.next().await {
            assert_eq!(msg.field::<u64>(tags::MSG_SEQ_NUM), Ok(4));
            break;
        }
    }
}

#[tokio::test]
async fn connect_once_reports_every_failed_endpoint() {
    let h = Harness::new();
    let (primary, backup) = (dead_address().await, dead_address().await);
    let initiator = h.initiator(&primary).with_failover(backup.as_str());
    let err = timeout(Duration::from_secs(5), initiator.connect_once()).await.unwrap().unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotConnected);
    let text = err.to_string();
    assert!(text.contains(&primary) && text.contains(&backup), "{text}");
}

#[tokio::test]
async fn keeps_retrying_until_an_endpoint_comes_up() {
    let mut h = Harness::new();
    let (primary_addr, backup_addr) = (dead_address().await, dead_address().await);
    let initiator = h.initiator(&primary_addr).with_failover(backup_addr.as_str());
    tokio::spawn(initiator.run());

    tokio::time::sleep(Duration::from_millis(300)).await;
    let (_backup, _) = h.acceptor("backup", Some(&backup_addr), false).await;
    h.logged_on("backup").await;
}

/// While attempts keep failing, the initiator waits longer each time, as its reconnect policy says.
#[tokio::test]
async fn failing_attempts_back_off() {
    // Accepts each connection and closes it at once: no session logs on.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let (attempts, mut attempted) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            drop(stream);
            let _ = attempts.send(tokio::time::Instant::now());
        }
    });
    let h = Harness::new();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
    config.reconnect = turbojet::ReconnectPolicy {
        jitter: false,
        ..turbojet::ReconnectPolicy::exponential(Duration::from_millis(100), Duration::from_millis(400))
    };
    let initiator = Initiator::new(addr.as_str(), config, Arc::new(MemoryStorage::new()), h.app("client", false));
    tokio::spawn(initiator.run());
    let mut at = Vec::new();
    for _ in 0..5 {
        at.push(timeout(Duration::from_secs(5), attempted.recv()).await.unwrap().unwrap());
    }
    let gaps: Vec<u128> = at.windows(2).map(|w| (w[1] - w[0]).as_millis()).collect();
    for (gap, expected) in gaps.iter().zip([100, 200, 400, 400]) {
        assert!((expected..expected + 150).contains(gap), "gaps {gaps:?}: expected about {expected} ms");
    }
}
