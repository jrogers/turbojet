//! FIXP session metrics over a real connection, through a globally installed recorder: a test
//! binary of its own, as a process installs one recorder.
#![cfg(feature = "metrics")]

#[allow(dead_code)]
#[path = "sbe/b3.rs"]
mod b3;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::fixp::{
    ClientConfig, FixpAcceptor, FixpApplication, FixpConfig, FixpContext, FixpHandle, FixpInitiator, Received, Role,
    SbeMessage, ServerConfig,
};
use turbojet::{DiskStorage, MemoryStorage};

const CLIENT: &str = "FIXP:CLIENT->SERVER";

/// Running totals by metric and session, as in `metrics.rs`: counters summed across snapshots,
/// gauges as last set, histograms by their samples.
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
                    *entry += samples.len() as f64;
                    *self.1.entry((key.name().to_string(), session)).or_default() +=
                        samples.iter().map(|s| **s).sum::<f64>();
                }
            }
        }
    }

    fn get(&self, name: &str, session: &str) -> f64 {
        self.0.get(&(name.to_string(), session.to_string())).copied().unwrap_or_default()
    }

    fn sum(&self, name: &str, session: &str) -> f64 {
        self.1.get(&(name.to_string(), session.to_string())).copied().unwrap_or_default()
    }

    /// The server's session label: its log's ID, named after the session ID it negotiated.
    fn server_session(&self) -> Option<String> {
        self.0.keys().map(|(_, session)| session).find(|s| s.starts_with("FIXP:SERVER->")).cloned()
    }
}

fn order(cl_ord_id: u64) -> b3::NewOrderSingle {
    b3::NewOrderSingle {
        cl_ord_id,
        security_id: 4001,
        price: b3::PriceOptional { mantissa: Some(1_502_500) },
        order_qty: 100,
        account: Some(1),
        market_segment_id: 1,
        side: b3::Side::Buy,
        ord_type: b3::OrdType::Limit,
        time_in_force: b3::TimeInForce::Day,
        ord_tag_id: None,
        mm_protection_reset: None,
        routing_instruction: None,
        self_trade_prevention_instruction: None,
        stop_px: b3::PriceOptional { mantissa: None },
        min_qty: None,
        max_floor: None,
        investor_id: None,
        custodian_info: b3::CustodianInfo { custodian: None, custody_account: None, custody_allocation_type: None },
        expire_date: None,
        sender_location: turbojet::sbe::pad(b"DMA"),
        entering_trader: *b"TRADR",
    }
}

/// Answers each order with one of the same ClOrdID.
struct Server;

impl FixpApplication for Server {
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let Ok((b3::Decoded::NewOrderSingle(received), _)) = b3::decode(msg.bytes) else { return };
        ctx.send(&order(received.cl_ord_id())).unwrap();
    }
}

struct Client {
    established: mpsc::UnboundedSender<()>,
    answers: mpsc::UnboundedSender<()>,
}

impl FixpApplication for Client {
    fn on_established(&self, _session: &FixpHandle) {
        let _ = self.established.send(());
    }
    fn on_message(&self, _ctx: &mut FixpContext<'_>, _msg: Received<'_>) {
        let _ = self.answers.send(());
    }
}

/// Runs a client and server over TCP, the client sending `orders` and logging out once each is
/// answered: the totals once the server has disconnected too, and the server's session label.
/// The server records the latency histograms, storing with fsync so its commits are timed.
async fn exchange(orders: u64, snapshotter: &Snapshotter) -> (Totals, String) {
    // Keepalives long enough that none goes out during the test.
    let dir = tempfile::tempdir().unwrap();
    let mut server = FixpConfig::new(Role::Server(ServerConfig::new("SERVER")));
    server.keepalive = Duration::from_secs(30);
    server.latency_metrics = true;
    let storage = Arc::new(DiskStorage::new(dir.path(), true).unwrap());
    let acceptor = FixpAcceptor::new(server, storage, Arc::new(Server)).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));

    let (established, mut establishments) = mpsc::unbounded_channel();
    let (answers_tx, mut answers) = mpsc::unbounded_channel();
    let mut client = FixpConfig::new(Role::Client(ClientConfig::new("CLIENT", "SERVER")));
    client.keepalive = Duration::from_secs(30);
    let app = Arc::new(Client { established, answers: answers_tx });
    let initiator = FixpInitiator::new(addr, client, Arc::new(MemoryStorage::new()), app).unwrap();
    let handle = initiator.handle();
    let connection = tokio::spawn(async move { initiator.connect_once().await });

    let wait = Duration::from_secs(5);
    tokio::time::timeout(wait, establishments.recv()).await.unwrap().unwrap();
    for i in 0..orders {
        handle.send(SbeMessage::encode(&order(i)).unwrap()).unwrap();
    }
    for _ in 0..orders {
        tokio::time::timeout(wait, answers.recv()).await.unwrap().unwrap();
    }
    handle.logout(None).unwrap();
    tokio::time::timeout(wait, connection).await.unwrap().unwrap().unwrap();

    let mut totals = Totals::default();
    let server = tokio::time::timeout(wait, async {
        loop {
            totals.update(snapshotter);
            if let Some(server) = totals.server_session()
                && totals.get("turbojet_disconnects_total", &server) == 1.0
            {
                break server;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the server did not disconnect");
    (totals, server)
}

/// Also the latency histograms, which the server records and the client doesn't.
#[tokio::test]
async fn both_ends_agree_on_messages_and_bytes() {
    const ORDERS: u64 = 10;
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    metrics::set_global_recorder(recorder).expect("recorder already installed");
    let (totals, server) = exchange(ORDERS, &snapshotter).await;

    let get = |name, session: &str| totals.get(name, session);
    // Negotiate, Establish, Sequence, the orders and Terminate; the server's answers likewise.
    let messages = (ORDERS + 4) as f64;
    assert_eq!(get("turbojet_messages_sent_total", CLIENT), messages);
    assert_eq!(get("turbojet_messages_received_total", &server), messages);
    assert_eq!(get("turbojet_messages_sent_total", &server), messages);
    assert_eq!(get("turbojet_messages_received_total", CLIENT), messages);
    // Every byte one side sent, the other received: the server's Negotiate included, read before
    // it knew which session it belonged to.
    assert!(get("turbojet_bytes_sent_total", CLIENT) > 0.0);
    assert_eq!(get("turbojet_bytes_sent_total", CLIENT), get("turbojet_bytes_received_total", &server));
    assert_eq!(get("turbojet_bytes_sent_total", &server), get("turbojet_bytes_received_total", CLIENT));
    for session in [CLIENT, &server] {
        assert_eq!(get("turbojet_logons_total", session), 1.0, "{session}");
        assert_eq!(get("turbojet_disconnects_total", session), 1.0, "{session}");
        assert_eq!(get("turbojet_session_logged_on", session), 0.0, "{session}");
    }
    let next = (ORDERS + 1) as f64;
    assert_eq!(get("turbojet_next_outgoing_seq", CLIENT), next);
    assert_eq!(get("turbojet_next_incoming_seq", &server), next);
    assert_eq!(get("turbojet_next_outgoing_seq", &server), next);
    assert_eq!(get("turbojet_next_incoming_seq", CLIENT), next);

    // Each message the server handled once bound: its Negotiate binds it as it's handled.
    assert_eq!(get("turbojet_inbound_message_seconds", &server), messages);
    assert!(totals.sum("turbojet_inbound_message_seconds", &server) > 0.0);
    for name in ["turbojet_commit_seconds", "turbojet_read_to_write_seconds"] {
        assert!(get(name, &server) >= 1.0, "{name}");
        assert!(totals.sum(name, &server) > 0.0, "{name}");
    }
    for name in ["turbojet_inbound_message_seconds", "turbojet_commit_seconds", "turbojet_read_to_write_seconds"] {
        assert_eq!(get(name, CLIENT), 0.0, "{name}: the client doesn't record latency");
    }
}
