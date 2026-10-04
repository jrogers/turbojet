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
    Acceptor, Application, Context, DiskStorage, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject,
    MsgType, SessionConfig, SessionHandle,
};

const SERVER: &str = "FIX.4.2:SERVER->CLIENT";
const CLIENT: &str = "FIX.4.2:CLIENT->SERVER";

/// Running totals: snapshots reset counters, so counters are summed across snapshots. Histograms
/// count their samples, and their sums are kept apart.
#[derive(Default)]
struct Totals(HashMap<(String, String), f64>, HashMap<(String, String), f64>);

impl Totals {
    fn update(&mut self, snapshotter: &Snapshotter) {
        for (key, _, _, value) in snapshotter.snapshot().into_vec() {
            let key = key.key();
            let session =
                key.labels().find(|l| l.key() == "session").map(|l| l.value().to_string()).unwrap_or_default();
            let entry = self.0.entry((key.name().to_string(), session.clone())).or_default();
            match value {
                DebugValue::Counter(n) => *entry += n as f64,
                DebugValue::Gauge(g) => *entry = g.into_inner(),
                DebugValue::Histogram(samples) => {
                    assert!(samples.iter().all(|s| s.is_finite() && **s >= 0.0), "{} {samples:?}", key.name());
                    *entry += samples.len() as f64;
                    *self.1.entry((key.name().to_string(), session.clone())).or_default() +=
                        samples.iter().map(|s| **s).sum::<f64>();
                }
            }
        }
    }

    fn get(&self, name: &str, session: &str) -> f64 {
        self.0.get(&(name.to_string(), session.to_string())).copied().unwrap_or_default()
    }

    /// The sum of a histogram's samples.
    fn sum(&self, name: &str, session: &str) -> f64 {
        self.1.get(&(name.to_string(), session.to_string())).copied().unwrap_or_default()
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
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.logged_on.send(());
    }
    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        let _ = self.acks.send(());
        Ok(())
    }
}

/// Also the latency histograms, which the acceptor records, storing with fsync so its commits are
/// timed, and the initiator doesn't.
#[tokio::test]
async fn both_ends_agree_on_messages_and_bytes() {
    const ORDERS: usize = 10;
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    metrics::set_global_recorder(recorder).expect("recorder already installed");

    let dir = tempfile::tempdir().unwrap();
    let mut server = SessionConfig::new("FIX.4.2", "SERVER");
    server.latency_metrics = true;
    let acceptor =
        Acceptor::new(server, Arc::new(DiskStorage::new(dir.path(), true).unwrap()), Arc::new(Acker)).unwrap();
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
    )
    .unwrap();
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

    // Each message the acceptor handled once bound: its Logon binds it as it's handled.
    assert_eq!(get("turbojet_inbound_message_seconds", SERVER), (ORDERS + 2) as f64);
    assert!(totals.sum("turbojet_inbound_message_seconds", SERVER) > 0.0);
    // At least one commit, for one read; with fsync, both take time.
    for name in ["turbojet_commit_seconds", "turbojet_read_to_write_seconds"] {
        assert!(get(name, SERVER) >= 1.0, "{name}");
        assert!(totals.sum(name, SERVER) > 0.0, "{name}");
    }
    for name in ["turbojet_inbound_message_seconds", "turbojet_commit_seconds", "turbojet_read_to_write_seconds"] {
        assert_eq!(get(name, CLIENT), 0.0, "{name}: the initiator doesn't record latency");
    }
}
