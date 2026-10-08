//! Operator control of sequence numbers, live over TCP and on stored state.

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::message::tags;
use turbojet::registry::{SequenceError, SequenceNumbers};
use turbojet::{
    Acceptor, Application, Context, Disconnect, DiskStorage, Initiator, InitiatorConfig, Message, MessageReject,
    MsgType, SessionConfig, SessionHandle, SessionRegistry,
};

#[derive(Debug)]
enum Event {
    LoggedOn,
    LoggedOut,
    Message(Message),
}

struct Recorder(mpsc::UnboundedSender<Event>);

impl Application for Recorder {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.0.send(Event::LoggedOn);
    }
    fn on_logout(&self, _session: &SessionHandle, _ended: Disconnect) {
        let _ = self.0.send(Event::LoggedOut);
    }
    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.0.send(Event::Message(msg.clone()));
        Ok(())
    }
}

async fn next(rx: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    tokio::time::timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out").expect("closed")
}

fn numbers(next_incoming: u64, next_outgoing: u64) -> SequenceNumbers {
    SequenceNumbers { next_incoming, next_outgoing }
}

#[tokio::test]
async fn operators_adjust_live_and_stored_sequence_numbers() {
    let dir = tempfile::tempdir().unwrap();
    let (server_tx, mut server) = mpsc::unbounded_channel();
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", "SERVER"),
        Arc::new(DiskStorage::new(dir.path(), false).unwrap()),
        Arc::new(Recorder(server_tx)),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.clone().serve(listener));
    let operator = acceptor.session("CLIENT");

    let (client_tx, mut client) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        addr,
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER"),
        Arc::new(turbojet::MemoryStorage::new()),
        Arc::new(Recorder(client_tx)),
    )
    .unwrap();
    let client_handle = initiator.handle();
    let connection = tokio::spawn(async move { initiator.connect_once().await });
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));
    assert_eq!(operator.sequence_numbers().await.unwrap(), numbers(2, 2));

    // Live: skip our outgoing numbers ahead. The client follows the SequenceReset, so the next
    // message arrives in sequence, with no ResendRequest.
    assert_eq!(operator.set_next_outgoing(100).await.unwrap(), numbers(2, 100));
    operator.send(Message::new(MsgType::ExecutionReport).with(tags::TEXT, "after the jump")).unwrap();
    match next(&mut client).await {
        Event::Message(msg) => assert_eq!(msg.get(tags::MSG_SEQ_NUM), Some("100")),
        other => panic!("{other:?}"),
    }
    assert_eq!(client_handle.sequence_numbers().await.unwrap().next_incoming, 101);
    assert_eq!(operator.sequence_numbers().await.unwrap().next_outgoing, 101);

    // A reset needs the session disconnected.
    assert!(matches!(operator.reset_sequence_numbers().await, Err(SequenceError::Connected)));

    client_handle.logout(None).unwrap();
    assert!(matches!(next(&mut server).await, Event::LoggedOut));
    connection.await.unwrap().unwrap();

    // Disconnected: the stored state can be read and changed directly.
    let stored = operator.sequence_numbers().await.unwrap();
    assert_eq!(stored.next_outgoing, 102, "101 was our Logout reply");
    assert!(matches!(operator.set_next_outgoing(5).await, Err(SequenceError::Invalid(_))), "never backwards");
    assert_eq!(operator.reset_sequence_numbers().await.unwrap(), numbers(1, 1));
    assert_eq!(operator.set_next_incoming(7).await.unwrap(), numbers(7, 1));

    // The same stored state, seen by a fresh registry over the same directory (as the gateway's
    // `seqnums` tool does).
    let registry = Arc::new(SessionRegistry::new(Arc::new(DiskStorage::new(dir.path(), false).unwrap())));
    let offline = registry.handle(operator.id().clone());
    assert_eq!(offline.sequence_numbers().await.unwrap(), numbers(7, 1));
}

/// An operator sees each connected session with when it logged on and the other side's address,
/// from the acceptor and from the initiator's handle, and nothing once it has logged out.
#[tokio::test]
async fn operators_see_connected_sessions_with_their_connections() {
    let (server_tx, mut server) = mpsc::unbounded_channel();
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", "SERVER"),
        Arc::new(turbojet::MemoryStorage::new()),
        Arc::new(Recorder(server_tx)),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server_addr = listener.local_addr().unwrap();
    tokio::spawn(acceptor.clone().serve(listener));
    assert!(acceptor.statuses().is_empty());

    let (client_tx, mut client) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        server_addr.to_string(),
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER"),
        Arc::new(turbojet::MemoryStorage::new()),
        Arc::new(Recorder(client_tx)),
    )
    .unwrap();
    let client_handle = initiator.handle();
    assert!(client_handle.status().is_none(), "not connected yet");
    let before = chrono::Utc::now();
    let connection = tokio::spawn(async move { initiator.connect_once().await });
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));

    let statuses = acceptor.statuses();
    assert_eq!(statuses.len(), 1);
    let status = &statuses[0];
    assert_eq!(status.id.to_string(), "FIX.4.2:SERVER->CLIENT");
    assert!(status.since >= before && status.since <= chrono::Utc::now(), "{:?}", status.since);
    let client_addr = status.connection.addr.expect("the client's address");
    assert!(client_addr.ip().is_loopback());
    assert_ne!(client_addr, server_addr);
    let status = client_handle.status().expect("connected");
    assert_eq!(status.connection.addr, Some(server_addr));

    client_handle.logout(None).unwrap();
    assert!(matches!(next(&mut server).await, Event::LoggedOut));
    connection.await.unwrap().unwrap();
    assert!(acceptor.statuses().is_empty());
    assert!(client_handle.status().is_none());
}

/// An operator asks the counterparty to resend orders the application already processed: they
/// come again, marked PossDupFlag=Y, in order, and new orders follow as usual.
#[tokio::test]
async fn operators_ask_for_processed_messages_again() {
    let (server_tx, mut server) = mpsc::unbounded_channel();
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", "SERVER"),
        Arc::new(turbojet::MemoryStorage::new()),
        Arc::new(Recorder(server_tx)),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.clone().serve(listener));
    let operator = acceptor.session("CLIENT");
    assert!(operator.request_resend(1).await.is_err(), "not logged on");

    let (client_tx, mut client) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        addr,
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER"),
        Arc::new(turbojet::MemoryStorage::new()),
        Arc::new(Recorder(client_tx)),
    )
    .unwrap();
    let client_handle = initiator.handle();
    tokio::spawn(async move { initiator.connect_once().await });
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));

    let order = |id: &str| Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id);
    for id in ["ORD1", "ORD2", "ORD3"] {
        client_handle.send(order(id)).unwrap();
    }
    for id in ["ORD1", "ORD2", "ORD3"] {
        assert_eq!(received(&mut server).await, (id.to_string(), None));
    }
    // ORD1 to ORD3 were 2 to 4.
    assert_eq!(operator.sequence_numbers().await.unwrap(), numbers(5, 2));

    let err = operator.request_resend(5).await.unwrap_err();
    assert!(err.to_string().contains("nothing to resend from 5"), "{err}");
    assert_eq!(operator.request_resend(3).await.unwrap().next_incoming, 3);
    for id in ["ORD2", "ORD3"] {
        assert_eq!(received(&mut server).await, (id.to_string(), Some("Y".to_string())));
    }
    client_handle.send(order("ORD4")).unwrap();
    assert_eq!(received(&mut server).await, ("ORD4".to_string(), None));
    assert_eq!(operator.sequence_numbers().await.unwrap().next_incoming, 6);
    // That resend is over, so another may be asked for.
    assert_eq!(operator.request_resend(5).await.unwrap().next_incoming, 5);
    assert_eq!(received(&mut server).await, ("ORD4".to_string(), Some("Y".to_string())));
}

/// The next order delivered: its ClOrdID and PossDupFlag, if any.
async fn received(events: &mut mpsc::UnboundedReceiver<Event>) -> (String, Option<String>) {
    match next(events).await {
        Event::Message(msg) => {
            (msg.get(tags::CL_ORD_ID).unwrap().to_string(), msg.get(tags::POSS_DUP_FLAG).map(str::to_string))
        }
        other => panic!("{other:?}"),
    }
}
