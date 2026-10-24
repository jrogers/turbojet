//! Generated FIX 4.2 messages encode and decode.
#![cfg(feature = "fix42")]

use turbojet::fields::{FromFix, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet_fix::fix42::*;

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

#[test]
fn new_order_single_round_trips() {
    let mut order = NewOrderSingle::new(NewOrderSingleRequired {
        cl_ord_id: "ORD1".into(),
        handl_inst: HandlInst::AutomatedExecutionNoIntervention.into(),
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
        transact_time: time(),
        ord_type: OrdType::Limit.into(),
    });
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order.time_in_force = Some(TimeInForce::Day.into());
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
    let report = ExecutionReport::new(ExecutionReportRequired {
        order_id: "O1".into(),
        exec_id: "E1".into(),
        exec_trans_type: ExecTransType::New.into(),
        exec_type: ExecType::Fill.into(),
        ord_status: OrdStatus::Filled.into(),
        symbol: "AAPL".into(),
        side: Side::Sell.into(),
        leaves_qty: 0.into(),
        cum_qty: 100.into(),
        avg_px: "12.34".parse().unwrap(),
    });
    let msg = report.to_message();
    assert!(msg.to_string().starts_with("35=8|"));
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}

#[test]
fn groups_round_trip() {
    let mut order = NewOrderSingle::new(NewOrderSingleRequired {
        cl_ord_id: "ORD1".into(),
        handl_inst: HandlInst::ManualOrder.into(),
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
        transact_time: time(),
        ord_type: OrdType::Market.into(),
    });
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
    let msg = NewOrderSingle::new(NewOrderSingleRequired {
        cl_ord_id: "ORD1".into(),
        handl_inst: HandlInst::ManualOrder.into(),
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
        transact_time: time(),
        ord_type: OrdType::Market.into(),
    })
    .to_message();
    let text = msg.to_string().replace("|55=AAPL", "");
    let raw = turbojet::Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
        let (tag, value) = f.split_once('=').unwrap();
        (tag.parse::<u32>().unwrap(), value)
    }));
    assert_eq!(raw.parse::<NewOrderSingle>().unwrap_err().tag, tags::SYMBOL);
}

#[test]
fn required_groups_round_trip_with_nested_groups() {
    let mut first = ListOrdGrp::new(ListOrdGrpRequired {
        cl_ord_id: "ORD1".into(),
        list_seq_no: 1,
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
    });
    first.allocs = vec![PreAllocGrp::new("ACC1"), PreAllocGrp::new("ACC2")];
    first.trading_sessions.push(TrdgSesGrp::new("REG"));
    let mut second = ListOrdGrp::new(ListOrdGrpRequired {
        cl_ord_id: "ORD2".into(),
        list_seq_no: 2,
        symbol: "MSFT".into(),
        side: Side::Sell.into(),
    });
    second.order_qty = Some(50.into());
    // NoOrders is required, so it's a new() parameter.
    let list = NewOrderList::new(NewOrderListRequired {
        list_id: "LIST1".into(),
        bid_type: BidType::NoBiddingProcess.into(),
        tot_no_orders: 2,
        orders: vec![first, second],
    });
    let msg = list.to_message();
    let text = msg.to_string();
    assert!(text.contains("|73=2|11=ORD1|67=1|"), "{text}");
    assert!(text.contains("|78=2|79=ACC1|79=ACC2|"), "{text}");
    assert!(text.contains("|386=1|336=REG|"), "{text}");
    assert_eq!(msg.parse::<NewOrderList>().unwrap(), list);
}

#[test]
fn missing_required_groups_are_errors() {
    let mut list = NewOrderList::new(NewOrderListRequired {
        list_id: "LIST1".into(),
        bid_type: BidType::NoBiddingProcess.into(),
        tot_no_orders: 0,
        orders: Vec::new(),
    });
    let err = list.to_message().parse::<NewOrderList>().unwrap_err();
    assert_eq!((err.tag, err.kind), (tags::NO_ORDERS, turbojet::message::FieldErrorKind::Missing));
    list.orders.push(ListOrdGrp::new(ListOrdGrpRequired {
        cl_ord_id: "ORD1".into(),
        list_seq_no: 1,
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
    }));
    assert!(list.to_message().parse::<NewOrderList>().is_ok());
}

#[test]
fn cancel_all_quotes_needs_no_entries() {
    // The FIX 4.2 file marks NoQuoteEntries required; this crate is generated with it optional.
    let cancel = QuoteCancel::new(QuoteCancelRequired {
        quote_id: "Q1".into(),
        quote_cancel_type: QuoteCancelType::CancelAllQuotes.into(),
    });
    let msg = cancel.to_message();
    assert_eq!(msg.to_string(), "35=Z|117=Q1|298=4|");
    assert_eq!(msg.parse::<QuoteCancel>().unwrap(), cancel);
}
