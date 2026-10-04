//! Cancel on disconnect, driven by an Acceptor or Initiator: a session that ends in a way its
//! trigger counts has its orders cancelled once the grace period passes without it logging back
//! on, and at once when its acceptor or initiator shuts down. Time is paused, and connections are
//! in-memory pipes, so a grace period passes in no time and the timings are exact.

use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream, duplex};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{Instant, advance, sleep, timeout};
use turbojet::codec::{Decoded, decode, encode};
use turbojet::message::{tags, utc_timestamp};
use turbojet::{
    Acceptor, Application, CancelOnDisconnect, CancelTrigger, ConnectionInfo, Disconnect, Initiator, InitiatorConfig,
    MemoryStorage, Message, MsgType, SessionConfig, SessionHandle, SessionId, SessionRegistry,
};

const GRACE: Duration = Duration::from_secs(5);

#[derive(Debug, PartialEq)]
enum Event {
    LoggedOn(String),
    LoggedOut(String, Disconnect),
    /// The cancel, and when it came.
    Cancel(String, Disconnect, Instant),
}

/// Records what happens to each session, by counterparty.
struct Recorder {
    events: mpsc::UnboundedSender<Event>,
}

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.events.send(Event::LoggedOn(session.id().target_comp_id.clone()));
    }

    fn on_logout(&self, session: &SessionId, ended: Disconnect) {
        let _ = self.events.send(Event::LoggedOut(session.target_comp_id.clone(), ended));
    }

    fn on_cancel_on_disconnect(&self, session: &SessionId, ended: Disconnect) {
        let _ = self.events.send(Event::Cancel(session.target_comp_id.clone(), ended, Instant::now()));
    }
}

struct Events(mpsc::UnboundedReceiver<Event>);

impl Events {
    async fn next(&mut self) -> Event {
        timeout(Duration::from_secs(3600), self.0.recv()).await.expect("no event").expect("channel closed")
    }

    /// Nothing has happened that hasn't been taken.
    fn assert_none(&mut self) {
        if let Ok(event) = self.0.try_recv() {
            panic!("unexpected {event:?}");
        }
    }
}

fn recorder() -> (Arc<Recorder>, Events) {
    let (events, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder { events }), Events(rx))
}

fn cancelling(config: &mut SessionConfig, trigger: CancelTrigger) {
    config.cancel_on_disconnect = Some(CancelOnDisconnect { trigger, grace: GRACE });
}

/// An acceptor, SERVER, cancelling its sessions' orders on `trigger`.
fn acceptor(trigger: CancelTrigger) -> (Acceptor, Events) {
    let (app, events) = recorder();
    let mut config = SessionConfig::new("FIX.4.2", "SERVER");
    config.logout_timeout = Duration::from_secs(2);
    cancelling(&mut config, trigger);
    (Acceptor::new(config, Arc::new(MemoryStorage::new()), app), events)
}

/// A counterparty driven by hand over an in-memory pipe, as `ours` talking to `theirs`.
struct Raw {
    stream: DuplexStream,
    buf: Vec<u8>,
    seq: u64,
    ours: &'static str,
    theirs: &'static str,
}

impl Raw {
    async fn send(&mut self, msg: Message) {
        let msg = msg
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::SENDER_COMP_ID, self.ours)
            .with(tags::TARGET_COMP_ID, self.theirs)
            .with(tags::MSG_SEQ_NUM, self.seq)
            .with(tags::SENDING_TIME, utc_timestamp());
        self.seq += 1;
        self.stream.write_all(&encode(&msg).unwrap()).await.unwrap();
    }

    fn logon_message() -> Message {
        Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, "30")
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
            match self.stream.read_buf(&mut self.buf).await {
                Ok(0) | Err(_) => return None,
                Ok(_) => {}
            }
        }
    }

    async fn expect(&mut self, msg_type: MsgType) {
        assert_eq!(self.recv().await.map(|m| m.msg_type()), Some(msg_type));
    }
}

/// A client connected to `acceptor` and logged on as `client`, and the task running the
/// connection.
async fn log_on(acceptor: &Acceptor, client: &'static str) -> (Raw, JoinHandle<std::io::Result<()>>) {
    log_on_at(acceptor, client, 1).await
}

/// [`log_on`] with the client's Logon numbered `seq`.
async fn log_on_at(acceptor: &Acceptor, client: &'static str, seq: u64) -> (Raw, JoinHandle<std::io::Result<()>>) {
    let (ours, theirs) = duplex(1 << 16);
    let task = tokio::spawn({
        let acceptor = acceptor.clone();
        async move { acceptor.accept_stream(ours, ConnectionInfo::new(None, Vec::new())).await }
    });
    let mut raw = Raw { stream: theirs, buf: Vec::new(), seq, ours: client, theirs: "SERVER" };
    raw.send(Raw::logon_message()).await;
    raw.expect(MsgType::Logon).await;
    (raw, task)
}

/// The client drops the connection: the countdown starts then, and the cancel comes when the
/// grace period ends, not before.
#[tokio::test(start_paused = true)]
async fn a_dropped_connection_cancels_after_the_grace_period() {
    let (acceptor, mut events) = acceptor(CancelTrigger::Disconnect);
    let (client, task) = log_on(&acceptor, "CLIENT").await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));

    let dropped = Instant::now();
    drop(client);
    task.await.unwrap().unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("CLIENT".into(), Disconnect::ConnectionLost));
    sleep(GRACE - Duration::from_millis(10)).await;
    events.assert_none();

    let Event::Cancel(client, ended, at) = events.next().await else { panic!("expected a cancel") };
    assert_eq!((client.as_str(), ended), ("CLIENT", Disconnect::ConnectionLost));
    assert!(at >= dropped + GRACE, "{:?} early", dropped + GRACE - at);
    // The timer rounds up to the millisecond.
    assert!(at <= dropped + GRACE + Duration::from_millis(1), "{:?} late", at - dropped - GRACE);
    sleep(GRACE * 4).await;
    events.assert_none();
}

/// The client reconnects and logs on within the grace period: no cancel, then or later.
#[tokio::test(start_paused = true)]
async fn logging_back_on_within_the_grace_period_stops_the_cancel() {
    let (acceptor, mut events) = acceptor(CancelTrigger::Disconnect);
    let (client, task) = log_on(&acceptor, "CLIENT").await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));
    drop(client);
    task.await.unwrap().unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("CLIENT".into(), Disconnect::ConnectionLost));

    advance(GRACE - Duration::from_secs(1)).await;
    let (_client, _task) = log_on_at(&acceptor, "CLIENT", 2).await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));
    // Long past the grace period, though short of a heartbeat the client would have to answer.
    sleep(GRACE * 4).await;
    events.assert_none();
    assert!(acceptor.session("CLIENT").is_connected());
}

/// The client logs out, and the acceptor answers. `trigger` says whether that counts. The
/// acceptor is kept, as the countdowns go with it.
async fn log_out(trigger: CancelTrigger) -> (Acceptor, Events) {
    let (acceptor, mut events) = acceptor(trigger);
    let (mut client, task) = log_on(&acceptor, "CLIENT").await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));
    client.send(Message::new(MsgType::Logout)).await;
    client.expect(MsgType::Logout).await;
    task.await.unwrap().unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("CLIENT".into(), Disconnect::CounterpartyLogout));
    (acceptor, events)
}

#[tokio::test(start_paused = true)]
async fn a_logout_is_not_a_disconnect() {
    let (_acceptor, mut events) = log_out(CancelTrigger::Disconnect).await;
    sleep(GRACE * 4).await;
    events.assert_none();
}

#[tokio::test(start_paused = true)]
async fn a_logout_counts_when_the_trigger_says_so() {
    let (_acceptor, mut events) = log_out(CancelTrigger::DisconnectOrLogout).await;
    let logged_out = Instant::now();
    let Event::Cancel(client, ended, at) = events.next().await else { panic!("expected a cancel") };
    assert_eq!((client.as_str(), ended), ("CLIENT", Disconnect::CounterpartyLogout));
    assert!(at >= logged_out + GRACE, "{:?} early", logged_out + GRACE - at);
}

/// Shutting down fires the countdowns pending at once, rather than when they're due: nothing
/// will be left to fire them.
#[tokio::test(start_paused = true)]
async fn shutdown_fires_pending_cancels_at_once() {
    let (acceptor, mut events) = acceptor(CancelTrigger::Disconnect);
    let (client, task) = log_on(&acceptor, "CLIENT").await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));
    drop(client);
    task.await.unwrap().unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("CLIENT".into(), Disconnect::ConnectionLost));

    let shutting_down = Instant::now();
    acceptor.shutdown(None).await;
    let Event::Cancel(client, ended, at) = events.next().await else { panic!("expected a cancel") };
    assert_eq!((client.as_str(), ended), ("CLIENT", Disconnect::ConnectionLost));
    assert_eq!(at, shutting_down, "fired by the shutdown, not the countdown");
    sleep(GRACE * 4).await;
    events.assert_none();
}

/// A counterparty that doesn't answer the shutdown's Logout is disconnected when the logout
/// times out. The session ended because we shut it down, so that's no reason to cancel.
#[tokio::test(start_paused = true)]
async fn a_shutdown_that_gives_up_on_the_logout_does_not_cancel() {
    let (acceptor, mut events) = acceptor(CancelTrigger::DisconnectOrLogout);
    let (mut client, _task) = log_on(&acceptor, "CLIENT").await;
    assert_eq!(events.next().await, Event::LoggedOn("CLIENT".into()));

    let shutdown = tokio::spawn({
        let acceptor = acceptor.clone();
        async move { acceptor.shutdown(None).await }
    });
    client.expect(MsgType::Logout).await;
    assert_eq!(client.recv().await.map(|m| m.msg_type()), None, "disconnected without an answer");
    shutdown.await.unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("CLIENT".into(), Disconnect::Shutdown));
    sleep(GRACE * 4).await;
    events.assert_none();
}

/// An initiator to `target`, cancelling on a disconnect, with its registry `registry` if given.
fn initiator(target: &str, registry: Option<&Arc<SessionRegistry>>) -> (Initiator, Events) {
    let (app, events) = recorder();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), target);
    config.session.logout_timeout = Duration::from_secs(2);
    cancelling(&mut config.session, CancelTrigger::Disconnect);
    let initiator = Initiator::new("127.0.0.1:1", config, Arc::new(MemoryStorage::new()), app);
    let initiator = match registry {
        Some(registry) => initiator.with_registry(registry.clone()),
        None => initiator,
    };
    (initiator, events)
}

/// `initiator` connected to a hand-driven `server` and logged on, and the task running it.
async fn connect(initiator: &Initiator, server: &'static str) -> (Raw, JoinHandle<std::io::Result<()>>) {
    let (ours, theirs) = duplex(1 << 16);
    let task = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.run_stream(ours, ConnectionInfo::new(None, Vec::new())).await }
    });
    let mut raw = Raw { stream: theirs, buf: Vec::new(), seq: 1, ours: server, theirs: "CLIENT" };
    raw.expect(MsgType::Logon).await;
    raw.send(Raw::logon_message()).await;
    (raw, task)
}

/// An initiator's countdowns are driven as an acceptor's are.
#[tokio::test(start_paused = true)]
async fn an_initiator_cancels_after_the_grace_period() {
    let (initiator, mut events) = initiator("SERVER", None);
    let (server, task) = connect(&initiator, "SERVER").await;
    assert_eq!(events.next().await, Event::LoggedOn("SERVER".into()));
    let dropped = Instant::now();
    drop(server);
    task.await.unwrap().unwrap();
    assert_eq!(events.next().await, Event::LoggedOut("SERVER".into(), Disconnect::ConnectionLost));

    let Event::Cancel(server, ended, at) = events.next().await else { panic!("expected a cancel") };
    assert_eq!((server.as_str(), ended), ("SERVER", Disconnect::ConnectionLost));
    assert!(at >= dropped + GRACE, "{:?} early", dropped + GRACE - at);
}

/// Two initiators share a registry. Shutting one down fires only its own countdowns: the other's
/// still waits out its grace period, or its own shutdown.
#[tokio::test(start_paused = true)]
async fn an_initiator_shutdown_fires_only_its_own_cancels() {
    let registry = Arc::new(SessionRegistry::new(Arc::new(MemoryStorage::new())));
    let (first, mut first_events) = initiator("FIRST", Some(&registry));
    let (second, mut second_events) = initiator("SECOND", Some(&registry));
    let (first_server, first_task) = connect(&first, "FIRST").await;
    let (mut second_server, _second_task) = connect(&second, "SECOND").await;
    assert_eq!(first_events.next().await, Event::LoggedOn("FIRST".into()));
    assert_eq!(second_events.next().await, Event::LoggedOn("SECOND".into()));

    drop(first_server);
    first_task.await.unwrap().unwrap();
    assert_eq!(first_events.next().await, Event::LoggedOut("FIRST".into(), Disconnect::ConnectionLost));
    let shutdown = tokio::spawn({
        let second = second.clone();
        async move { second.shutdown(None).await }
    });
    second_server.expect(MsgType::Logout).await;
    second_server.send(Message::new(MsgType::Logout)).await;
    shutdown.await.unwrap();
    assert_eq!(second_events.next().await, Event::LoggedOut("SECOND".into(), Disconnect::Shutdown));
    first_events.assert_none();
    assert!(registry.next_cancel_deadline().is_some(), "the first's countdown goes on");

    let shutting_down = Instant::now();
    first.shutdown(None).await;
    let Event::Cancel(server, _, at) = first_events.next().await else { panic!("expected a cancel") };
    assert_eq!((server.as_str(), at), ("FIRST", shutting_down));
    second_events.assert_none();
}
