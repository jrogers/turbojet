//! A FIXP client and server run by the connection driver over an in-memory stream, exchanging
//! application messages of another schema: B3 NewOrderSingles, from the codec generated from B3's
//! schema (`sbe/b3.rs`), carried by standard FIXP session messages.

#[allow(dead_code)]
#[path = "sbe/b3.rs"]
mod b3;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use turbojet::MemoryStorage;
use turbojet::fixp::{
    self, ClientConfig, Ended, FixpApplication, FixpConfig, FixpContext, FixpHandle, FixpRegistry, FixpSession,
    Received, Role, SbeMessage, ServerConfig, TerminationCode,
};
use turbojet::sbe::pad;

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
        sender_location: pad(b"DMA"),
        entering_trader: *b"TRADR",
    }
}

/// Reports what happens to it on a channel; as the server, answers each order with the same
/// order, its ClOrdID plus 1000.
struct Events {
    server: bool,
    events: mpsc::UnboundedSender<Event>,
}

#[derive(Debug)]
enum Event {
    Established(FixpHandle),
    Order { cl_ord_id: u64, seq: u64 },
    Ended(Ended),
}

/// Events compare by what they say, handles by their session.
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Established(a), Self::Established(b)) => a.id() == b.id(),
            (Self::Order { cl_ord_id: a, seq: x }, Self::Order { cl_ord_id: b, seq: y }) => a == b && x == y,
            (Self::Ended(a), Self::Ended(b)) => a == b,
            _ => false,
        }
    }
}

impl FixpApplication for Events {
    fn on_established(&self, session: &FixpHandle) {
        let _ = self.events.send(Event::Established(session.clone()));
    }
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let (b3::Decoded::NewOrderSingle(order), _) = b3::decode(msg.bytes).unwrap() else { panic!("not an order") };
        let _ =
            self.events.send(Event::Order { cl_ord_id: order.cl_ord_id(), seq: msg.seq.expect("a recoverable flow") });
        if self.server {
            ctx.send(&self::order(order.cl_ord_id() + 1000)).unwrap();
        }
    }
    fn on_ended(&self, _session: &FixpHandle, how: Ended) {
        let _ = self.events.send(Event::Ended(how));
    }
}

fn end(
    role: Role,
    server: bool,
) -> (FixpSession, turbojet::registry::CommandReceiver<SbeMessage>, mpsc::UnboundedReceiver<Event>) {
    let (events, received) = mpsc::unbounded_channel();
    let registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
    let app = Arc::new(Events { server, events });
    let (session, commands) = FixpSession::new(FixpConfig::new(role), registry, app, std::time::Instant::now());
    (session, commands, received)
}

async fn next(events: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    tokio::time::timeout(Duration::from_secs(5), events.recv()).await.expect("an event in time").expect("an event")
}

#[tokio::test]
async fn a_client_and_server_exchange_orders_over_the_driver() {
    let (client_stream, server_stream) = tokio::io::duplex(64 * 1024);
    let (server, server_commands, mut server_events) = end(Role::Server(ServerConfig::new("SERVER")), true);
    let (client, client_commands, mut client_events) = end(Role::Client(ClientConfig::new("CLIENT", "SERVER")), false);
    let server_task = tokio::spawn(fixp::run(server_stream, server, server_commands));
    let client_task = tokio::spawn(fixp::run(client_stream, client, client_commands));

    let Event::Established(handle) = next(&mut client_events).await else { panic!("not established") };
    assert!(matches!(next(&mut server_events).await, Event::Established(_)));
    let mut receipts = Vec::new();
    for cl_ord_id in 1..=3 {
        receipts.push(handle.send(SbeMessage::encode(&order(cl_ord_id)).unwrap()).unwrap());
    }
    for (receipt, seq) in receipts.into_iter().zip(1..) {
        assert_eq!(receipt.await, Ok(seq));
    }
    for cl_ord_id in 1..=3 {
        assert_eq!(next(&mut server_events).await, Event::Order { cl_ord_id, seq: cl_ord_id });
        assert_eq!(next(&mut client_events).await, Event::Order { cl_ord_id: cl_ord_id + 1000, seq: cl_ord_id });
    }

    handle.logout(None).unwrap();
    let finished = TerminationCode::Finished;
    assert_eq!(next(&mut client_events).await, Event::Ended(Ended::TerminatedByUs(finished)));
    assert_eq!(next(&mut server_events).await, Event::Ended(Ended::TerminatedByPeer(finished)));
    client_task.await.unwrap().unwrap();
    server_task.await.unwrap().unwrap();
}

/// A server restarted on the same port and store: the client reconnects by itself and
/// re-establishes the same session, numbering carrying on.
#[tokio::test]
async fn a_client_reconnects_over_tcp_and_reestablishes_the_session() {
    use turbojet::fixp::{FixpAcceptor, FixpInitiator};
    use turbojet::{ReconnectPolicy, SessionStorage};

    let server_store: Arc<dyn SessionStorage> = Arc::new(MemoryStorage::new());
    let (server_events, mut server_received) = mpsc::unbounded_channel();
    let server_app: Arc<dyn FixpApplication> = Arc::new(Events { server: true, events: server_events });
    let serve = |listener: tokio::net::TcpListener| {
        let config = FixpConfig::new(Role::Server(ServerConfig::new("SERVER")));
        let acceptor = FixpAcceptor::new(config, server_store.clone(), server_app.clone()).unwrap();
        tokio::spawn(acceptor.clone().serve(listener));
        acceptor
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let first = serve(listener);

    let (client_events, mut client_received) = mpsc::unbounded_channel();
    let client_config = FixpConfig::new(Role::Client(ClientConfig::new("CLIENT", "SERVER")));
    let client_app = Arc::new(Events { server: false, events: client_events });
    let initiator = FixpInitiator::new(addr.to_string(), client_config, Arc::new(MemoryStorage::new()), client_app)
        .unwrap()
        .with_reconnect(ReconnectPolicy::fixed(Duration::from_millis(50)))
        .unwrap();
    tokio::spawn(initiator.clone().run());

    assert!(matches!(next(&mut client_received).await, Event::Established(_)));
    assert!(matches!(next(&mut server_received).await, Event::Established(_)));
    let handle = initiator.handle();
    assert_eq!(handle.send(SbeMessage::encode(&order(1)).unwrap()).unwrap().await, Ok(1));
    assert_eq!(next(&mut server_received).await, Event::Order { cl_ord_id: 1, seq: 1 });
    assert_eq!(next(&mut client_received).await, Event::Order { cl_ord_id: 1001, seq: 1 });

    // The server shuts down, terminating the session; a new one takes its port and store.
    first.shutdown().await;
    let finished = TerminationCode::Finished;
    assert_eq!(next(&mut client_received).await, Event::Ended(Ended::TerminatedByPeer(finished)));
    assert_eq!(next(&mut server_received).await, Event::Ended(Ended::TerminatedByUs(finished)));
    let second = serve(tokio::net::TcpListener::bind(addr).await.unwrap());

    assert!(matches!(next(&mut client_received).await, Event::Established(_)));
    assert!(matches!(next(&mut server_received).await, Event::Established(_)));
    assert_eq!(handle.send(SbeMessage::encode(&order(2)).unwrap()).unwrap().await, Ok(2));
    assert_eq!(next(&mut server_received).await, Event::Order { cl_ord_id: 2, seq: 2 });
    assert_eq!(next(&mut client_received).await, Event::Order { cl_ord_id: 1002, seq: 2 });

    initiator.shutdown().await;
    second.shutdown().await;
}
