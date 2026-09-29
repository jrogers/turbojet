//! Log structure: the session span, the message-traffic target, and application events.

use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};

/// A log sink shared between the subscriber and the test.
#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl io::Write for Captured {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Captured {
    fn lines(&self) -> Vec<String> {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap().lines().map(String::from).collect()
    }
}

/// Logs from inside callbacks, as an application would.
struct Server;

impl Application for Server {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        tracing::info!("application received order");
        ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
        Ok(())
    }
}

struct Client {
    events: mpsc::UnboundedSender<()>,
}

impl Application for Client {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.events.send(());
    }
    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(());
        Ok(())
    }
}

/// Runs a logon and one order round trip with logging captured at `filter`.
async fn round_trip_logging(filter: &str) -> Vec<String> {
    let captured = Captured::default();
    let sink = captured.clone();
    let subscriber =
        tracing_subscriber::fmt().with_env_filter(filter).with_ansi(false).with_writer(move || sink.clone()).finish();
    // Thread-local: the test runtime is single-threaded, so every task logs here.
    let _guard = tracing::subscriber::set_default(subscriber);

    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(Server));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));
    let (events, mut rx) = mpsc::unbounded_channel();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
    config.password = Some("hunter2".into());
    let initiator = Initiator::new(addr, config, Arc::new(MemoryStorage::new()), Arc::new(Client { events }));
    let handle = initiator.handle();
    tokio::spawn(initiator.run());

    let wait = Duration::from_secs(5);
    tokio::time::timeout(wait, rx.recv()).await.unwrap().unwrap(); // logged on
    handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "O1")).unwrap();
    tokio::time::timeout(wait, rx.recv()).await.unwrap().unwrap(); // acknowledged
    captured.lines()
}

fn any(lines: &[String], parts: &[&str]) -> bool {
    lines.iter().any(|line| parts.iter().all(|part| line.contains(part)))
}

#[tokio::test]
async fn logs_carry_the_session_span_and_message_target() {
    let lines = round_trip_logging("debug").await;
    let server = "session{id=FIX.4.2:SERVER->CLIENT}";
    let client = "session{id=FIX.4.2:CLIENT->SERVER}";

    // Message traffic on its own target, in both directions, labelled with the session.
    assert!(any(&lines, &[server, "turbojet::messages", "direction=\"in\"", "35=D"]), "{lines:#?}");
    assert!(any(&lines, &[server, "turbojet::messages", "direction=\"out\"", "35=8"]), "{lines:#?}");
    assert!(any(&lines, &[client, "turbojet::messages", "direction=\"out\"", "35=D"]), "{lines:#?}");
    // Engine events and application callbacks inherit the span.
    assert!(any(&lines, &[server, "logged on"]), "{lines:#?}");
    assert!(any(&lines, &[server, "application received order"]), "{lines:#?}");
}

#[tokio::test]
async fn passwords_are_masked_in_the_message_log() {
    let lines = round_trip_logging("debug").await;
    assert!(any(&lines, &["turbojet::messages", "direction=\"out\"", "35=A", "554=***"]), "{lines:#?}");
    assert!(any(&lines, &["turbojet::messages", "direction=\"in\"", "35=A", "554=***"]), "{lines:#?}");
    assert!(!any(&lines, &["hunter2"]), "{lines:#?}");
}

#[tokio::test]
async fn message_traffic_is_off_at_info() {
    let lines = round_trip_logging("info").await;
    assert!(!any(&lines, &["turbojet::messages"]), "{lines:#?}");
    assert!(any(&lines, &["session{id=FIX.4.2:SERVER->CLIENT}", "logged on"]), "{lines:#?}");
}
