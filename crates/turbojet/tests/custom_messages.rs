//! Messages, groups and enums defined outside the crate with the exported macros, sent through a
//! real session.

use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::fields::SessionRejectReason;
use turbojet::message::{FieldErrorKind, FixMessage};
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle, fix_enum, fix_group, fix_message,
};

/// A venue's tags: the standard ones plus its own.
mod venue_tags {
    pub use turbojet::message::tags::*;
    pub const PRIORITY: u32 = 5001;
    pub const NO_LEGS: u32 = 5002;
    pub const LEG_SYMBOL: u32 = 5003;
    pub const LEG_RATIO: u32 = 5004;
}

fix_enum! {
    Priority {
        Normal = "N",
        Urgent = "U",
    }
}

fix_group! {
    Leg / LegRef {
        symbol: req String = venue_tags::LEG_SYMBOL,
        ratio: opt u32 = venue_tags::LEG_RATIO,
    }
}

fix_message! {
    /// A venue-specific spread order, MsgType U1.
    SpreadOrder / SpreadOrderRef = "U1" {
        cl_ord_id: req String = venue_tags::CL_ORD_ID,
        priority: opt Priority = venue_tags::PRIORITY,
        legs: group Leg = venue_tags::NO_LEGS,
        text: opt String = venue_tags::TEXT,
    }
}

fix_message! {
    /// The venue's acknowledgement, MsgType U2.
    SpreadAck / SpreadAckRef = "U2" {
        cl_ord_id: req String = venue_tags::CL_ORD_ID,
        legs_accepted: req u32 = venue_tags::NO_LEGS,
    }
}

/// Acknowledges spread orders with the number of legs it parsed.
struct Venue;

impl Application for Venue {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() != SpreadOrder::MSG_TYPE {
            return Err(MessageReject::unsupported_message_type());
        }
        let order: SpreadOrder = msg.parse()?;
        ctx.send(SpreadAck { cl_ord_id: order.cl_ord_id, legs_accepted: u32::try_from(order.legs.len()).unwrap() });
        Ok(())
    }
}

struct Client {
    events: mpsc::UnboundedSender<Option<Message>>,
}

impl Application for Client {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.events.send(None);
    }
    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(Some(msg.clone()));
        Ok(())
    }
}

/// The next event: `None` for a logon, `Some` for a message.
async fn next(rx: &mut mpsc::UnboundedReceiver<Option<Message>>) -> Option<Message> {
    tokio::time::timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out").expect("closed")
}

#[test]
fn custom_types_round_trip() {
    const U1: MsgType = MsgType::from_static("U1");
    assert_eq!(SpreadOrder::MSG_TYPE, U1);
    let order = SpreadOrder {
        cl_ord_id: "S1".into(),
        priority: Some(Priority::Urgent),
        legs: vec![Leg { symbol: "ESZ6".into(), ratio: Some(1) }, Leg { symbol: "ESH7".into(), ratio: None }],
        text: None,
    };
    let msg: Message = order.clone().into();
    assert_eq!(msg.to_string(), "35=U1|11=S1|5001=U|5002=2|5003=ESZ6|5004=1|5003=ESH7|");
    assert_eq!(msg.parse::<SpreadOrder>().unwrap(), order);
    // A known code given as a string still maps to its variant.
    fix_message! { Order / OrderRef = "D" { cl_ord_id: req String = venue_tags::CL_ORD_ID } }
    assert_eq!(Order::MSG_TYPE, MsgType::NewOrderSingle);
}

/// `parse_strict` refuses a body tag the message doesn't define, where `parse` ignores it.
#[test]
fn strict_parsing_refuses_tags_the_message_does_not_define() {
    let order = SpreadOrder {
        cl_ord_id: "S1".into(),
        priority: None,
        legs: vec![Leg { symbol: "ESZ6".into(), ratio: Some(1) }],
        text: None,
    };
    let msg: Message = order.clone().into();
    // Header fields aren't the body's, wherever they are.
    let msg = msg.with(venue_tags::SENDER_COMP_ID, "CLIENT").with(venue_tags::MSG_SEQ_NUM, 2u64);
    assert_eq!(msg.parse_strict::<SpreadOrder>().unwrap(), order);

    let extra = msg.clone().with(9999, "x");
    assert_eq!(extra.parse::<SpreadOrder>().unwrap(), order, "lenient by default");
    let err = extra.parse_strict::<SpreadOrder>().unwrap_err();
    assert_eq!((err.tag, &err.kind), (9999, &FieldErrorKind::NotDefined));
    assert_eq!(err.reject_reason(), SessionRejectReason::TagNotDefinedForMessageType);
    assert_eq!(err.to_string(), "Tag 9999 not defined for this message type");

    // Another error in the message is reported first.
    let missing = Message::new(MsgType::from_static("U1")).with(9999, "x");
    assert_eq!(missing.parse_strict::<SpreadOrder>().unwrap_err().kind, FieldErrorKind::Missing);
}

#[tokio::test]
async fn custom_messages_travel_through_a_session() {
    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.2", "VENUE"), Arc::new(MemoryStorage::new()), Arc::new(Venue));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));

    let (events, mut rx) = mpsc::unbounded_channel();
    let initiator = Initiator::new(
        addr,
        InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "VENUE"),
        Arc::new(MemoryStorage::new()),
        Arc::new(Client { events }),
    );
    let handle = initiator.handle();
    tokio::spawn(initiator.run());
    assert!(next(&mut rx).await.is_none(), "logged on");

    let legs = vec![Leg { symbol: "ESZ6".into(), ratio: Some(1) }, Leg { symbol: "ESH7".into(), ratio: Some(1) }];
    handle.send(SpreadOrder { cl_ord_id: "S1".into(), priority: None, legs, text: None }).unwrap();
    let ack = next(&mut rx).await.expect("a message").parse::<SpreadAck>().unwrap();
    assert_eq!(ack, SpreadAck { cl_ord_id: "S1".into(), legs_accepted: 2 });
}
