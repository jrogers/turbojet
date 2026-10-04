//! Graceful shutdown: an Acceptor or Initiator logs its sessions out, waits (bounded by the logout
//! timeout) for them to close, and stops accepting or reconnecting.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream, duplex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::codec::{Decoded, decode, encode};
use turbojet::message::{tags, utc_timestamp};
use turbojet::{
    Acceptor, Application, ConnectionInfo, Disconnect, Initiator, InitiatorConfig, MemoryStorage, Message, MsgType,
    SessionConfig, SessionHandle, SessionId,
};

#[derive(Debug)]
enum Event {
    LoggedOn(SessionHandle),
    LoggedOut,
}

struct Recorder {
    events: mpsc::UnboundedSender<Event>,
}

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.events.send(Event::LoggedOn(session));
    }

    fn on_logout(&self, _session: &SessionId, _ended: Disconnect) {
        let _ = self.events.send(Event::LoggedOut);
    }
}

fn recorder() -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    let (events, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder { events }), rx)
}

async fn next(rx: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out waiting for event").expect("channel closed")
}

fn acceptor(logout_timeout: Duration) -> (Acceptor, mpsc::UnboundedReceiver<Event>) {
    let (app, events) = recorder();
    let mut config = SessionConfig::new("FIX.4.2", "SERVER");
    config.logout_timeout = logout_timeout;
    (Acceptor::new(config, Arc::new(MemoryStorage::new()), app).unwrap(), events)
}

/// Serves `acceptor` on a local port: the address, and the task running `serve`.
async fn serve(acceptor: &Acceptor) -> (String, tokio::task::JoinHandle<std::io::Result<()>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    (addr, tokio::spawn(acceptor.clone().serve(listener)))
}

/// A counterparty driven by hand, byte for byte.
struct Raw<S> {
    stream: S,
    buf: Vec<u8>,
    seq: u64,
}

impl Raw<TcpStream> {
    async fn connect(addr: &str) -> Self {
        Self::new(TcpStream::connect(addr).await.unwrap())
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> Raw<S> {
    fn new(stream: S) -> Self {
        Self { stream, buf: Vec::new(), seq: 1 }
    }

    async fn send(&mut self, msg: Message) {
        let msg = msg
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::SENDER_COMP_ID, "CLIENT")
            .with(tags::TARGET_COMP_ID, "SERVER")
            .with(tags::MSG_SEQ_NUM, self.seq)
            .with(tags::SENDING_TIME, utc_timestamp());
        self.seq += 1;
        self.stream.write_all(&encode(&msg).unwrap()).await.unwrap();
    }

    async fn logon(&mut self) {
        self.send(Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, "30")).await;
        let reply = self.recv().await.expect("disconnected instead of answering the Logon");
        assert_eq!(reply.msg_type(), MsgType::Logon);
    }

    /// The next message, or `None` once the connection is closed.
    async fn recv(&mut self) -> Option<Message> {
        loop {
            match decode(&self.buf) {
                Decoded::Message(msg, len) => {
                    self.buf.drain(..len);
                    return Some(msg);
                }
                Decoded::Incomplete => {}
                Decoded::Garbled { reason, .. } => panic!("garbled: {reason}"),
            }
            let read = timeout(Duration::from_secs(5), self.stream.read_buf(&mut self.buf));
            match read.await.expect("timed out reading") {
                Ok(0) | Err(_) => return None,
                Ok(_) => {}
            }
        }
    }
}

#[tokio::test]
async fn acceptor_logs_out_and_waits_for_the_reply() {
    let (acceptor, mut events) = acceptor(Duration::from_secs(5));
    let (addr, server) = serve(&acceptor).await;
    let mut client = Raw::connect(&addr).await;
    client.logon().await;
    assert!(matches!(next(&mut events).await, Event::LoggedOn(_)));

    let started = Instant::now();
    let shutdown = tokio::spawn({
        let acceptor = acceptor.clone();
        async move { acceptor.shutdown(Some("end of day")).await }
    });
    let logout = client.recv().await.expect("no Logout");
    assert_eq!(logout.msg_type(), MsgType::Logout);
    assert_eq!(logout.get(tags::TEXT), Some("end of day"));
    client.send(Message::new(MsgType::Logout)).await;
    assert!(client.recv().await.is_none(), "still connected after the Logout exchange");

    timeout(Duration::from_secs(1), shutdown).await.expect("shutdown didn't return once logged out").unwrap();
    assert!(started.elapsed() < Duration::from_secs(1), "waited for the logout timeout: {:?}", started.elapsed());
    assert!(matches!(next(&mut events).await, Event::LoggedOut));
    assert!(acceptor.sessions().is_empty());
    // serve returned, and the listener is closed.
    timeout(Duration::from_secs(1), server).await.expect("serve still running").unwrap().unwrap();
    assert!(TcpStream::connect(&addr).await.is_err(), "still accepting connections");
}

#[tokio::test]
async fn acceptor_gives_up_on_a_counterparty_that_does_not_reply() {
    let (acceptor, mut events) = acceptor(Duration::from_millis(300));
    let (addr, _server) = serve(&acceptor).await;
    let mut client = Raw::connect(&addr).await;
    client.logon().await;
    assert!(matches!(next(&mut events).await, Event::LoggedOn(_)));

    let started = Instant::now();
    timeout(Duration::from_secs(3), acceptor.shutdown(None)).await.expect("shutdown hung");
    let elapsed = started.elapsed();
    assert!(elapsed >= Duration::from_millis(300), "didn't wait for the reply: {elapsed:?}");
    assert_eq!(client.recv().await.map(|m| m.msg_type()), Some(MsgType::Logout));
    assert!(client.recv().await.is_none(), "still connected");
    assert!(matches!(next(&mut events).await, Event::LoggedOut));
}

#[tokio::test]
async fn acceptor_disconnects_connections_that_have_not_logged_on() {
    let (acceptor, _events) = acceptor(Duration::from_secs(5));
    let (addr, _server) = serve(&acceptor).await;
    let mut client = Raw::connect(&addr).await;
    // Let the acceptor take the connection before shutting down.
    tokio::time::sleep(Duration::from_millis(50)).await;

    let started = Instant::now();
    timeout(Duration::from_secs(1), acceptor.shutdown(Some("closing"))).await.expect("shutdown waited");
    assert!(started.elapsed() < Duration::from_secs(1));
    assert!(client.recv().await.is_none(), "not disconnected");
}

#[tokio::test]
async fn acceptor_refuses_connections_once_shut_down() {
    let (acceptor, _events) = acceptor(Duration::from_secs(5));
    acceptor.shutdown(None).await;
    // A second call is harmless.
    acceptor.shutdown(None).await;

    // A stream handed over directly is closed without a logon.
    let (ours, theirs) = duplex(4096);
    let mut client = Raw::new(theirs);
    let accept = tokio::spawn({
        let acceptor = acceptor.clone();
        async move { acceptor.accept_stream(ours, ConnectionInfo::new(None, Vec::new())).await }
    });
    client.send(Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, "30")).await;
    assert!(client.recv().await.is_none(), "logged on after shutdown");
    timeout(Duration::from_secs(1), accept).await.expect("accept_stream still running").unwrap().unwrap();

    // serve returns straight away.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    timeout(Duration::from_secs(1), acceptor.serve(listener)).await.expect("serve ran after shutdown").unwrap();
}

/// The counterparty stops reading, so the connection blocks writing and can't process its logout
/// timeout; shutdown closes it anyway.
#[tokio::test]
async fn acceptor_closes_a_connection_stuck_writing() {
    let (acceptor, mut events) = acceptor(Duration::from_millis(200));
    let (ours, theirs): (DuplexStream, DuplexStream) = duplex(1024);
    let accept = tokio::spawn({
        let acceptor = acceptor.clone();
        async move { acceptor.accept_stream(ours, ConnectionInfo::new(None, Vec::new())).await }
    });
    let mut client = Raw::new(theirs);
    client.logon().await;
    let Event::LoggedOn(session) = next(&mut events).await else { panic!("expected logon") };
    // Far more than the pipe holds; the client never reads them.
    for i in 0..200 {
        session.send(Message::new(MsgType::ExecutionReport).with(tags::TEXT, format!("filler {i:0>100}"))).unwrap();
    }
    // Let the connection take them and block writing; otherwise it may see the shutdown first
    // and drop them.
    tokio::time::sleep(Duration::from_millis(100)).await;

    let started = Instant::now();
    timeout(Duration::from_secs(3), acceptor.shutdown(None)).await.expect("shutdown hung on a blocked write");
    // Closed by force, a second after the logout timeout, not by the session's own timeout.
    let elapsed = started.elapsed();
    assert!(elapsed >= Duration::from_secs(1), "closed without forcing it: {elapsed:?}");
    timeout(Duration::from_secs(1), accept).await.expect("connection still open").unwrap().ok();
    assert!(matches!(next(&mut events).await, Event::LoggedOut));
    drop(client);
}

fn initiator(addr: &str, reconnect: Duration) -> (Initiator, mpsc::UnboundedReceiver<Event>) {
    let (app, events) = recorder();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
    config.reconnect = turbojet::ReconnectPolicy::fixed(reconnect);
    (Initiator::new(addr, config, Arc::new(MemoryStorage::new()), app).unwrap(), events)
}

#[tokio::test]
async fn initiator_logs_out_and_stops_reconnecting() {
    let (acceptor, mut server) = acceptor(Duration::from_secs(5));
    let (addr, _server) = serve(&acceptor).await;
    let (initiator, mut client) = initiator(&addr, Duration::from_millis(50));
    let run = tokio::spawn(initiator.clone().run());
    assert!(matches!(next(&mut client).await, Event::LoggedOn(_)));
    assert!(matches!(next(&mut server).await, Event::LoggedOn(_)));

    let started = Instant::now();
    timeout(Duration::from_secs(1), initiator.shutdown(Some("bye"))).await.expect("shutdown waited for the timeout");
    assert!(started.elapsed() < Duration::from_secs(1));
    assert!(matches!(next(&mut client).await, Event::LoggedOut));
    assert!(matches!(next(&mut server).await, Event::LoggedOut));
    timeout(Duration::from_secs(1), run).await.expect("run still going").unwrap();
    // No reconnect.
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(acceptor.sessions().is_empty(), "the initiator reconnected");
    assert!(client.try_recv().is_err());
}

#[tokio::test]
async fn initiator_stops_waiting_to_reconnect() {
    // Nothing listens here; run() waits a minute between attempts.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    drop(listener);
    let (initiator, _client) = initiator(&addr, Duration::from_secs(60));
    let run = tokio::spawn(initiator.clone().run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    timeout(Duration::from_secs(1), initiator.shutdown(None)).await.expect("shutdown hung");
    timeout(Duration::from_secs(1), run).await.expect("run kept waiting to reconnect").unwrap();
    assert!(initiator.connect_once().await.is_err(), "connected after shutdown");
}
