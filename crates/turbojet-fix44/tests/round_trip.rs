//! Generated FIX 4.4 messages encode and decode, including flattened components and nested groups.

use turbojet::fields::{FromFix, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet_fix44::*;

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

fn raw(text: &str) -> turbojet::Message {
    turbojet::Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
        let (tag, value) = f.split_once('=').unwrap();
        (tag.parse::<u32>().unwrap(), value)
    }))
}

fn limit_order() -> NewOrderSingle {
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time(), OrdType::Limit);
    // From the Instrument and OrderQtyData components.
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order.time_in_force = Some(TimeInForce::Day);
    order
}

#[test]
fn new_order_single_round_trips() {
    let order = limit_order();
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=D|11=ORD1|"), "{text}");
    for field in ["|55=AAPL|", "|54=1|", "|60=20260928-12:00:00.000|", "|38=100|", "|40=2|", "|44=12.34|", "|59=0|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn parties_round_trip_with_their_nested_sub_ids() {
    let mut order = limit_order();
    let mut firm = Parties::new("FIRM");
    firm.party_id_source = Some(PartyIDSource::Proprietary);
    firm.party_role = Some(PartyRole::ExecutingFirm);
    let mut desk = PtysSubGrp::new("DESK1");
    desk.party_sub_id_type = Some(PartySubIDType::Application);
    firm.party_sub_ids.push(desk);
    order.party_ids = vec![firm, Parties::new("CLIENT")];

    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.contains("|453=2|448=FIRM|447=D|452=1|802=1|523=DESK1|803=4|448=CLIENT|"), "{text}");
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn execution_report_round_trips() {
    let (order_id, exec_id, leaves_qty, cum_qty, avg_px) = ("O1", "E1", 0.into(), 100.into(), "12.34".parse().unwrap());
    let mut report = ExecutionReport::new(
        order_id,
        exec_id,
        ExecType::Trade,
        OrdStatus::Filled,
        Side::Sell,
        leaves_qty,
        cum_qty,
        avg_px,
    );
    report.symbol = Some("AAPL".into());
    report.party_ids.push(Parties::new("FIRM"));
    let msg = report.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=8|"), "{text}");
    for field in ["|37=O1|", "|17=E1|", "|150=F|", "|39=2|", "|54=2|", "|151=0|", "|14=100|", "|6=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}

#[test]
fn order_cancel_request_round_trips() {
    let mut cancel = OrderCancelRequest::new("ORD1", "ORD2", Side::Buy, time());
    cancel.symbol = Some("AAPL".into());
    let msg = cancel.to_message();
    assert!(msg.to_string().starts_with("35=F|41=ORD1|"), "{msg}");
    assert_eq!(msg.parse::<OrderCancelRequest>().unwrap(), cancel);
}

#[test]
fn missing_required_fields_are_errors() {
    let text = limit_order().to_message().to_string().replace("|40=2", "");
    assert_eq!(raw(&text).parse::<NewOrderSingle>().unwrap_err().tag, tags::ORD_TYPE);
}

#[test]
fn unknown_enum_codes_are_errors() {
    let text = limit_order().to_message().to_string().replace("|54=1|", "|54=?|");
    let err = raw(&text).parse::<NewOrderSingle>().unwrap_err();
    assert_eq!(err.tag, tags::SIDE);
    assert!(matches!(err.kind, turbojet::message::FieldErrorKind::IncorrectValue(ref v) if v == "?"), "{err:?}");
}
