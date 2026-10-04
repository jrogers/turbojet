//! A FIXT.1.1 initiator and acceptor over TCP, exchanging typed FIX 5.0 SP2 messages, the acceptor
//! validating them against the official dictionaries. Needs the repository's dictionaries/.
#![cfg(feature = "validation")]

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::fields::{ApplVerId, FromFix, UtcTimestamp};
use turbojet::message::tags;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_dictionary::Dictionary;
use turbojet_fix50sp2::*;

#[derive(Debug)]
enum Event {
    LoggedOn(SessionHandle),
    Message(Message),
}

/// Forwards callbacks to a channel; optionally fills every NewOrderSingle.
struct Recorder {
    events: mpsc::UnboundedSender<Event>,
    fill_orders: bool,
}

impl Application for Recorder {
    fn on_logon(&self, session: SessionHandle) {
        let _ = self.events.send(Event::LoggedOn(session));
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if self.fill_orders && msg.msg_type() == MsgType::NewOrderSingle {
            let order: NewOrderSingle = msg.parse()?;
            let mut report =
                ExecutionReport::new("O1", "E1", ExecType::Trade, OrdStatus::Filled, order.side, 0.into(), 100.into());
            report.cl_ord_id = Some(order.cl_ord_id);
            ctx.send(report);
        }
        let _ = self.events.send(Event::Message(msg.clone()));
        Ok(())
    }
}

fn recorder(fill_orders: bool) -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    let (events, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder { events, fill_orders }), rx)
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

fn dictionary() -> Dictionary {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dictionaries/orchestra");
    let load = |name| Dictionary::load(dir.join(name)).unwrap_or_else(|e| panic!("{e}"));
    load("OrchestraFIX50SP2.xml").with_transport(&load("FIXTSession.xml")).unwrap()
}

#[tokio::test]
async fn fix50sp2_orders_are_validated_and_filled_over_fixt() {
    // The acceptor supports two versions and validates FIX 5.0 SP2 against its dictionary.
    let (server_app, mut server) = recorder(true);
    let config = SessionConfig::new("FIXT.1.1", "SERVER")
        .with_appl_ver_id(ApplVerId::Fix50Sp1)
        .with_appl_ver_id(ApplVerId::Fix50Sp2)
        .with_dictionary(&dictionary());
    let acceptor = Acceptor::new(config, Arc::new(MemoryStorage::new()), server_app).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.clone().serve(listener));

    let (client_app, mut client) = recorder(false);
    let session = SessionConfig::new("FIXT.1.1", "CLIENT").with_appl_ver_id(ApplVerId::Fix50Sp2);
    let initiator =
        Initiator::new(&addr, InitiatorConfig::new(session, "SERVER"), Arc::new(MemoryStorage::new()), client_app)
            .unwrap();
    tokio::spawn(initiator.clone().run());

    // Both sides agree on the version the initiator asked for.
    let client_handle = logged_on(&mut client).await;
    let server_handle = logged_on(&mut server).await;
    assert_eq!(client_handle.appl_ver_id(), Some(ApplVerId::Fix50Sp2));
    assert_eq!(server_handle.appl_ver_id(), Some(ApplVerId::Fix50Sp2));

    let time = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time, OrdType::Market);
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.party_ids.push(Parties::new("FIRM"));
    client_handle.send(order).unwrap();

    // Delivered, so it passed validation on the acceptor; then the fill comes back.
    let received = message(&mut server).await;
    assert_eq!(received.msg_type(), MsgType::NewOrderSingle);
    assert_eq!(received.get(tags::CL_ORD_ID), Some("ORD1"));
    assert_eq!(received.get(tags::BEGIN_STRING), Some("FIXT.1.1"));
    let fill: ExecutionReport = message(&mut client).await.parse().unwrap();
    assert_eq!(fill.cl_ord_id.as_deref(), Some("ORD1"));
    assert_eq!(fill.ord_status, OrdStatus::Filled);
}
