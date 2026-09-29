//! Operator control of sequence numbers, live over TCP and on stored state.

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::message::tags;
use turbojet::registry::{SequenceError, SequenceNumbers};
use turbojet::{
    Acceptor, Application, Context, DiskStorage, Initiator, InitiatorConfig, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle, SessionId, SessionRegistry,
};

#[derive(Debug)]
enum Event {
    LoggedOn,
    LoggedOut,
    Message(Message),
}

struct Recorder(mpsc::UnboundedSender<Event>);

impl Application for Recorder {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.0.send(Event::LoggedOn);
    }
    fn on_logout(&self, _session: &SessionId) {
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
    );
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
    );
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
