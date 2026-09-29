//! Generated FIX 4.2 messages encode and decode.

use turbojet::fields::{FromFix, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet_fix42::*;

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

#[test]
fn new_order_single_round_trips() {
    let mut order = NewOrderSingle::new(
        "ORD1",
        HandlInst::AutomatedExecutionNoIntervention,
        "AAPL",
        Side::Buy,
        time(),
        OrdType::Limit,
    );
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order.time_in_force = Some(TimeInForce::Day);
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=D|11=ORD1|"), "{text}");
    for field in ["|21=1|", "|55=AAPL|", "|54=1|", "|40=2|", "|38=100|", "|44=12.34|", "|59=0|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn execution_report_round_trips() {
    let report = ExecutionReport::new(
        "O1",
        "E1",
        ExecTransType::New,
        ExecType::Fill,
        OrdStatus::Filled,
        "AAPL",
        Side::Sell,
        0.into(),
        100.into(),
        "12.34".parse().unwrap(),
    );
    let msg = report.to_message();
    assert!(msg.to_string().starts_with("35=8|"));
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}

#[test]
fn groups_round_trip() {
    let mut order = NewOrderSingle::new("ORD1", HandlInst::ManualOrder, "AAPL", Side::Buy, time(), OrdType::Market);
    // Orchestra's official name for NewOrderSingle's NoAllocs group.
    let mut alloc = PreAllocGrp::new("ACC1");
    alloc.alloc_shares = Some(100.into());
    order.allocs = vec![alloc, PreAllocGrp::new("ACC2")];
    order.trading_sessions.push(TrdgSesGrp::new("REG"));
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.contains("|78=2|79=ACC1|80=100|79=ACC2|"), "{text}");
    assert!(text.contains("|386=1|336=REG|"), "{text}");
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn missing_required_fields_are_errors() {
    let msg =
        NewOrderSingle::new("ORD1", HandlInst::ManualOrder, "AAPL", Side::Buy, time(), OrdType::Market).to_message();
    let text = msg.to_string().replace("|55=AAPL", "");
    let raw = turbojet::Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
        let (tag, value) = f.split_once('=').unwrap();
        (tag.parse::<u32>().unwrap(), value)
    }));
    assert_eq!(raw.parse::<NewOrderSingle>().unwrap_err().tag, tags::SYMBOL);
}

#[test]
fn required_groups_round_trip_with_nested_groups() {
    let mut first = ListOrdGrp::new("ORD1", 1, "AAPL", Side::Buy);
    first.allocs = vec![PreAllocGrp::new("ACC1"), PreAllocGrp::new("ACC2")];
    first.trading_sessions.push(TrdgSesGrp::new("REG"));
    let mut second = ListOrdGrp::new("ORD2", 2, "MSFT", Side::Sell);
    second.order_qty = Some(50.into());
    // NoOrders is required, so it's a new() parameter.
    let list = NewOrderList::new("LIST1", BidType::NoBiddingProcess, 2, vec![first, second]);
    let msg = list.to_message();
    let text = msg.to_string();
    assert!(text.contains("|73=2|11=ORD1|67=1|"), "{text}");
    assert!(text.contains("|78=2|79=ACC1|79=ACC2|"), "{text}");
    assert!(text.contains("|386=1|336=REG|"), "{text}");
    assert_eq!(msg.parse::<NewOrderList>().unwrap(), list);
}

#[test]
fn missing_required_groups_are_errors() {
    let mut list = NewOrderList::new("LIST1", BidType::NoBiddingProcess, 0, Vec::new());
    let err = list.to_message().parse::<NewOrderList>().unwrap_err();
    assert_eq!((err.tag, err.kind), (tags::NO_ORDERS, turbojet::message::FieldErrorKind::Missing));
    list.orders.push(ListOrdGrp::new("ORD1", 1, "AAPL", Side::Buy));
    assert!(list.to_message().parse::<NewOrderList>().is_ok());
}

#[test]
fn cancel_all_quotes_needs_no_entries() {
    // The FIX 4.2 file marks NoQuoteEntries required; this crate is generated with it optional.
    let cancel = QuoteCancel::new("Q1", QuoteCancelType::CancelAllQuotes);
    let msg = cancel.to_message();
    assert_eq!(msg.to_string(), "35=Z|117=Q1|298=4|");
    assert_eq!(msg.parse::<QuoteCancel>().unwrap(), cancel);
}
