//! The engine talking to itself: an Initiator and an Acceptor over real TCP.

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle, SessionId,
};

#[derive(Debug)]
enum Event {
    LoggedOn(SessionHandle),
    LoggedOut,
    Message(Message),
}

/// Forwards callbacks to a channel; optionally acknowledges NewOrderSingle.
struct Recorder {
    events: mpsc::UnboundedSender<Event>,
    ack_orders: bool,
}

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.events.send(Event::LoggedOn(session));
    }

    fn on_logout(&self, _session: &SessionId) {
        let _ = self.events.send(Event::LoggedOut);
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if self.ack_orders && msg.msg_type() == MsgType::NewOrderSingle {
            ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
        }
        let _ = self.events.send(Event::Message(msg.clone()));
        Ok(())
    }
}

fn recorder(ack_orders: bool) -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    let (events, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder { events, ack_orders }), rx)
}

async fn next(rx: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out waiting for event").expect("channel closed")
}

async fn logged_on(rx: &mut mpsc::UnboundedReceiver<Event>) -> SessionHandle {
    match next(rx).await {
        Event::LoggedOn(handle) => handle,
        other => panic!("expected logon, got {other:?}"),
    }
}

async fn message(rx: &mut mpsc::UnboundedReceiver<Event>) -> Message {
    match next(rx).await {
        Event::Message(msg) => msg,
        other => panic!("expected message, got {other:?}"),
    }
}

async fn start_acceptor(app: Arc<Recorder>) -> (Acceptor, String) {
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.clone().serve(listener));
    (acceptor, addr)
}

fn initiator(addr: &str, app: Arc<Recorder>) -> Initiator {
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
    config.reconnect_interval = Duration::from_millis(100);
    Initiator::new(addr, config, Arc::new(MemoryStorage::new()), app)
}

fn order(cl_ord_id: &str) -> Message {
    Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, cl_ord_id).with(tags::SYMBOL, "AAPL")
}

#[tokio::test]
async fn initiator_and_acceptor_exchange_messages_both_ways() {
    let (server_app, mut server) = recorder(true);
    let (acceptor, addr) = start_acceptor(server_app).await;
    let (client_app, mut client) = recorder(false);
    let initiator = initiator(&addr, client_app);
    tokio::spawn(initiator.clone().run());

    let client_handle = logged_on(&mut client).await;
    let server_handle = logged_on(&mut server).await;
    assert_eq!(client_handle.id().target_comp_id, "SERVER");
    assert_eq!(server_handle.id().target_comp_id, "CLIENT");
    assert_eq!(acceptor.sessions().len(), 1);

    // Client sends through its handle; the server's application replies through Context.
    initiator.handle().send(order("A")).unwrap();
    assert_eq!(message(&mut server).await.get(tags::CL_ORD_ID), Some("A"));
    let ack = message(&mut client).await;
    assert_eq!(ack.msg_type(), MsgType::ExecutionReport);
    assert_eq!(ack.get(tags::CL_ORD_ID), Some("A"));
    assert_eq!(ack.get(tags::SENDER_COMP_ID), Some("SERVER"));

    // The server pushes an unsolicited message by counterparty name.
    acceptor.session("CLIENT").send(Message::new(MsgType::ExecutionReport).with(tags::TEXT, "pushed")).unwrap();
    assert_eq!(message(&mut client).await.get(tags::TEXT), Some("pushed"));

    // Orderly logout from the client side notifies both applications.
    client_handle.logout(Some("done")).unwrap();
    assert!(matches!(next(&mut client).await, Event::LoggedOut));
    assert!(matches!(next(&mut server).await, Event::LoggedOut));
}

#[tokio::test]
async fn initiator_reconnects_and_continues_the_sequence() {
    let (server_app, mut server) = recorder(true);
    let (acceptor, addr) = start_acceptor(server_app).await;
    let (client_app, mut client) = recorder(false);
    let initiator = initiator(&addr, client_app);
    tokio::spawn(initiator.clone().run());

    logged_on(&mut client).await;
    logged_on(&mut server).await;
    initiator.handle().send(order("A")).unwrap();
    message(&mut server).await;
    let first = message(&mut client).await;

    // The server ends the session; the initiator reconnects on its own.
    acceptor.session("CLIENT").logout(Some("maintenance")).unwrap();
    assert!(matches!(next(&mut server).await, Event::LoggedOut));
    assert!(matches!(next(&mut client).await, Event::LoggedOut));
    logged_on(&mut client).await;
    logged_on(&mut server).await;

    initiator.handle().send(order("B")).unwrap();
    message(&mut server).await;
    let second = message(&mut client).await;
    let seq = |m: &Message| m.field::<u64>(tags::MSG_SEQ_NUM).unwrap();
    assert!(seq(&second) > seq(&first), "sequence numbers carry across the reconnect");
}

#[tokio::test]
async fn handle_is_not_connected_before_logon() {
    let (client_app, _client) = recorder(false);
    let initiator = initiator("127.0.0.1:1", client_app);
    assert!(!initiator.handle().is_connected());
    assert!(initiator.handle().send(order("A")).is_err());
}

#[tokio::test]
async fn messages_sent_during_logon_are_delivered_once_it_completes() {
    let (server_app, mut server) = recorder(true);
    let (_acceptor, addr) = start_acceptor(server_app).await;
    let (client_app, mut client) = recorder(false);
    let initiator = initiator(&addr, client_app);
    let handle = initiator.handle();
    tokio::spawn(initiator.run());

    // Send the moment the connection claims the session, which is before the Logon reply.
    while !handle.is_connected() {
        tokio::task::yield_now().await;
    }
    handle.send(order("early")).unwrap();

    logged_on(&mut server).await;
    assert_eq!(message(&mut server).await.get(tags::CL_ORD_ID), Some("early"));
    logged_on(&mut client).await;
    assert_eq!(message(&mut client).await.get(tags::CL_ORD_ID), Some("early"));
}
