//! An acceptor over SqlStorage (SQLite), with a real initiator: sessions log on, carry on
//! across reconnecting, and resend from the database.
#![cfg(feature = "sqlite")]

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_sql::{SqlConfig, SqlStorage};

/// Answers each order with an ExecutionReport.
struct Venue;

impl Application for Venue {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::NewOrderSingle {
            let id = msg.get(tags::CL_ORD_ID).unwrap_or_default().to_string();
            ctx.send(Message::new(MsgType::ExecutionReport).with(tags::CL_ORD_ID, id));
        }
        Ok(())
    }
}

/// Reports logons and the ExecutionReports it receives, with whether each was resent.
struct Client(mpsc::UnboundedSender<Option<(String, bool)>>);

impl Application for Client {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.0.send(None);
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::ExecutionReport {
            let id = msg.get(tags::CL_ORD_ID).unwrap_or_default().to_string();
            let _ = self.0.send(Some((id, msg.get(tags::POSS_DUP_FLAG) == Some("Y"))));
        }
        Ok(())
    }
}

async fn next(events: &mut mpsc::UnboundedReceiver<Option<(String, bool)>>) -> Option<(String, bool)> {
    timeout(Duration::from_secs(10), events.recv()).await.expect("timed out").expect("the client is gone")
}

/// The acceptor's state lives in SQLite: after the client reconnects expecting the venue's
/// reports again, the venue resends them from the database.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_reconnected_client_gets_its_reports_resent_from_the_database() {
    let dir = tempfile::tempdir().unwrap();
    let url = format!("sqlite://{}?mode=rwc", dir.path().join("venue.db").display());
    let storage = SqlStorage::connect(&url, SqlConfig::default()).await.unwrap();
    storage.migrate().await.unwrap();
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "VENUE"), Arc::new(storage), Arc::new(Venue));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.clone().serve(listener));

    let (sender, mut events) = mpsc::unbounded_channel();
    let config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "VENUE");
    let initiator = Initiator::new(addr.as_str(), config, Arc::new(MemoryStorage::new()), Arc::new(Client(sender)));
    let handle = initiator.handle();

    let first = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.connect_once().await }
    });
    assert_eq!(next(&mut events).await, None, "logged on");
    for id in ["A", "B", "C"] {
        handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id)).unwrap();
        assert_eq!(next(&mut events).await, Some((id.to_string(), false)));
    }
    handle.logout(None).unwrap();
    first.await.unwrap().unwrap();

    // The venue's Logon was 1, its reports 2 to 4: the client asks for them again.
    handle.set_next_incoming(2).await.unwrap();
    let second = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.connect_once().await }
    });
    assert_eq!(next(&mut events).await, None, "logged on again");
    for id in ["A", "B", "C"] {
        assert_eq!(next(&mut events).await, Some((id.to_string(), true)), "resent");
    }
    handle.logout(None).unwrap();
    second.await.unwrap().unwrap();
    acceptor.shutdown(None).await;
}
