//! Metrics recorded over a real connection, through a globally installed recorder.
#![cfg(feature = "metrics")]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};

const SERVER: &str = "FIX.4.2:SERVER->CLIENT";
const CLIENT: &str = "FIX.4.2:CLIENT->SERVER";

/// Running totals: snapshots reset counters, so counters are summed across snapshots.
#[derive(Default)]
struct Totals(HashMap<(String, String), f64>);

impl Totals {
    fn update(&mut self, snapshotter: &Snapshotter) {
        for (key, _, _, value) in snapshotter.snapshot().into_vec() {
            let key = key.key();
            let session =
                key.labels().find(|l| l.key() == "session").map(|l| l.value().to_string()).unwrap_or_default();
            let entry = self.0.entry((key.name().to_string(), session)).or_default();
            match value {
                DebugValue::Counter(n) => *entry += n as f64,
                DebugValue::Gauge(g) => *entry = g.into_inner(),
                DebugValue::Histogram(_) => {}
            }
        }
    }

    fn get(&self, name: &str, session: &str) -> f64 {
        self.0.get(&(name.to_string(), session.to_string())).copied().unwrap_or_default()
    }
}

struct Acker;

impl Application for Acker {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
        Ok(())
    }
}

struct Client {
    logged_on: mpsc::UnboundedSender<()>,
    acks: mpsc::UnboundedSender<()>,
}

impl Application for Client {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.logged_on.send(());
    }
    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        let _ = self.acks.send(());
        Ok(())
    }
}

#[tokio::test]
async fn both_ends_agree_on_messages_and_bytes() {
    const ORDERS: usize = 10;
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    metrics::set_global_recorder(recorder).expect("recorder already installed");

    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(Acker));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));

    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let (acks_tx, mut acks) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        addr,
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER"),
        Arc::new(MemoryStorage::new()),
        Arc::new(Client { logged_on, acks: acks_tx }),
    );
    let handle = initiator.handle();
    let connection = tokio::spawn(async move { initiator.connect_once().await });

    let wait = Duration::from_secs(5);
    tokio::time::timeout(wait, logons.recv()).await.unwrap().unwrap();
    for i in 0..ORDERS {
        handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}"))).unwrap();
    }
    for _ in 0..ORDERS {
        tokio::time::timeout(wait, acks.recv()).await.unwrap().unwrap();
    }
    handle.logout(None).unwrap();
    tokio::time::timeout(wait, connection).await.unwrap().unwrap().unwrap();

    // Wait until the acceptor side has disconnected too.
    let mut totals = Totals::default();
    tokio::time::timeout(wait, async {
        loop {
            totals.update(&snapshotter);
            if totals.get("turbojet_disconnects_total", SERVER) == 1.0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("acceptor did not disconnect");

    let get = |name, session| totals.get(name, session);
    // Logon, orders, Logout each way.
    assert_eq!(get("turbojet_messages_sent_total", CLIENT), (ORDERS + 2) as f64);
    assert_eq!(get("turbojet_messages_received_total", SERVER), (ORDERS + 2) as f64);
    assert_eq!(get("turbojet_messages_sent_total", SERVER), (ORDERS + 2) as f64);
    assert_eq!(get("turbojet_messages_received_total", CLIENT), (ORDERS + 2) as f64);
    // Every byte one side sent, the other received: including the acceptor's Logon, read before
    // it knew which session it belonged to.
    assert!(get("turbojet_bytes_sent_total", CLIENT) > 0.0);
    assert_eq!(get("turbojet_bytes_sent_total", CLIENT), get("turbojet_bytes_received_total", SERVER));
    assert_eq!(get("turbojet_bytes_sent_total", SERVER), get("turbojet_bytes_received_total", CLIENT));
    for session in [CLIENT, SERVER] {
        assert_eq!(get("turbojet_logons_total", session), 1.0, "{session}");
        assert_eq!(get("turbojet_disconnects_total", session), 1.0, "{session}");
        assert_eq!(get("turbojet_session_logged_on", session), 0.0, "{session}");
    }
}
