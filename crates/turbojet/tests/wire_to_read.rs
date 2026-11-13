//! Receive timestamps on a spinning connection, recorded as `turbojet_wire_to_read_seconds`.
//! Loopback has the kernel's stamps, if not a network card's.
#![cfg(all(feature = "metrics", target_os = "linux"))]

use std::sync::Arc;
use std::time::Duration;

use metrics_util::debugging::{DebugValue, DebuggingRecorder};
use tokio::sync::mpsc;
use turbojet::connection::SpinningStream;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, ConnectionInfo, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject,
    MsgType, SessionConfig, SessionHandle,
};

struct Acker;

impl Application for Acker {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)))?;
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_spinning_acceptor_with_receive_timestamps_records_wire_to_read() {
    const ORDERS: usize = 10;
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    metrics::set_global_recorder(recorder).expect("recorder already installed");

    let mut server = SessionConfig::new("FIX.4.2", "SERVER");
    server.latency_metrics = true;
    let acceptor = Acceptor::new(server, Arc::new(MemoryStorage::new()), Arc::new(Acker)).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let runtime = tokio::runtime::Handle::current();
    let spinning = std::thread::spawn(move || {
        let socket = SpinningStream::new(listener.accept()?.0)?.with_receive_timestamps()?;
        acceptor.accept_spinning(socket, ConnectionInfo::default(), &runtime)
    });

    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let (acks_tx, mut acks) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER").with_endpoint(addr),
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
    tokio::task::spawn_blocking(move || spinning.join().unwrap()).await.unwrap().unwrap();

    let samples: Vec<f64> = snapshotter
        .snapshot()
        .into_vec()
        .into_iter()
        .filter(|(key, ..)| key.key().name() == "turbojet_wire_to_read_seconds")
        .flat_map(|(key, _, _, value)| {
            assert!(key.key().labels().any(|l| l.key() == "session" && l.value() == "FIX.4.2:SERVER->CLIENT"));
            match value {
                DebugValue::Histogram(samples) => samples.into_iter().map(|s| *s).collect::<Vec<_>>(),
                other => panic!("{other:?}"),
            }
        })
        .collect();
    // At least the Logon's read, and every read of an order, though one read may take several.
    assert!(!samples.is_empty(), "no samples");
    assert!(samples.iter().all(|s| (0.0..1.0).contains(s)), "{samples:?}");
}
