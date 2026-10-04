//! Per-counterparty settings on an acceptor, over real connections.

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::store::StorageByCounterparty;
use turbojet::{
    Acceptor, Application, ConnectionInfo, Counterparties, Counterparty, Initiator, InitiatorConfig, MemoryStorage,
    Message, SessionConfig, SessionHandle, SessionId, SessionStorage,
};

/// Reports each logon by the counterparty's CompID.
struct Recorder(mpsc::UnboundedSender<String>);

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.0.send(session.id().target_comp_id.clone());
    }
}

/// Refuses "BLOCKED"; lets everyone else ask for a heartbeat of at most 10 seconds.
struct Rules;

impl Counterparties for Rules {
    fn resolve(
        &self,
        base: &SessionConfig,
        id: &SessionId,
        _: &Message,
        _: &ConnectionInfo,
    ) -> Result<Counterparty, String> {
        if id.target_comp_id == "BLOCKED" {
            return Err("blocked".into());
        }
        let mut counterparty = Counterparty::new(base.clone());
        counterparty.heartbeat = Duration::from_secs(1)..=Duration::from_secs(10);
        Ok(counterparty)
    }
}

async fn start_acceptor() -> (String, mpsc::UnboundedReceiver<String>) {
    let (tx, rx) = mpsc::unbounded_channel();
    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(Recorder(tx)))
            .with_counterparties(Arc::new(Rules));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));
    (addr, rx)
}

fn initiator(addr: &str, comp_id: &str, heartbeat: u64) -> Initiator {
    let (tx, _) = mpsc::unbounded_channel();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", comp_id), "SERVER");
    config.heartbeat_interval = Duration::from_secs(heartbeat);
    Initiator::new(addr, config, Arc::new(MemoryStorage::new()), Arc::new(Recorder(tx)))
}

#[tokio::test]
async fn each_counterparty_logs_on_under_its_own_settings() {
    let (addr, mut logons) = start_acceptor().await;
    assert!(initiator(&addr, "BLOCKED", 5).connect_once().await.is_err(), "refused by the resolver");
    assert!(initiator(&addr, "SLOW", 30).connect_once().await.is_err(), "heartbeat outside its range");
    let ok = initiator(&addr, "CLIENT", 5);
    let handle = ok.handle();
    let connection = tokio::spawn(async move { ok.connect_once().await });
    let logged_on = tokio::time::timeout(Duration::from_secs(5), logons.recv()).await.unwrap();
    assert_eq!(logged_on.as_deref(), Some("CLIENT"));
    assert!(logons.try_recv().is_err(), "only CLIENT logged on");
    handle.logout(None).unwrap();
    connection.await.unwrap().unwrap();
}

/// An operator's change to a disconnected session reaches its counterparty's own store.
#[tokio::test]
async fn an_offline_change_reaches_the_counterpartys_store() {
    let default = Arc::new(MemoryStorage::new());
    let broker = Arc::new(MemoryStorage::new());
    let storage = StorageByCounterparty::new(default.clone()).with("BROKER", broker.clone());
    let (tx, _) = mpsc::unbounded_channel();
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(storage), Arc::new(Recorder(tx)));
    acceptor.session("BROKER").set_next_outgoing(10).await.unwrap();
    let id = SessionId::new("FIX.4.4", "SERVER", "BROKER");
    assert_eq!(broker.open(&id).unwrap().next_outgoing(), 10);
    assert_eq!(default.open(&id).unwrap().next_outgoing(), 1);
}
