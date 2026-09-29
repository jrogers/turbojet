//! FIX 4.2 messages parsed from and written to the wire: field order, value sets, field errors and
//! repeating groups. Carried over from the hand-written `turbojet::fix42` module these types replace.

use turbojet::codec::{Decoded, decode, encode};
use turbojet::fields::{Decimal, FromFix, MsgType, SessionRejectReason, UtcTimestamp};
use turbojet::message::{FieldError, FieldErrorKind, FixMessage, Message};
use turbojet_fix42::*;

fn wire(text: &str) -> Message {
    let msg = Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
        let (tag, value) = f.split_once('=').unwrap();
        (tag.parse().unwrap(), value)
    }));
    // Round-trip through the codec to prove the message is well-formed on the wire.
    match decode(&encode(&msg).unwrap()) {
        Decoded::Message(decoded, _) => decoded,
        other => panic!("{other:?}"),
    }
}

const ORDER: &str = "8=FIX.4.2|35=D|49=C|56=G|34=2|52=20260927-03:20:48.544|\
    11=ORD1|21=1|55=AAPL|54=1|60=20260927-03:20:48.544|38=100|40=2|44=150.25|59=0|";

fn order() -> NewOrderSingle {
    let mut order = NewOrderSingle::new(
        "ORD1",
        HandlInst::AutomatedExecutionNoIntervention,
        "AAPL",
        Side::Buy,
        UtcTimestamp::from_fix("20260927-03:20:48.544").unwrap(),
        OrdType::Limit,
    );
    order.order_qty = Some(Decimal::from_fix("100").unwrap());
    order.price = Some(Decimal::from_fix("150.25").unwrap());
    order.time_in_force = Some(TimeInForce::Day);
    order
}

#[test]
fn parses_new_order_single_from_the_wire() {
    assert_eq!(wire(ORDER).parse::<NewOrderSingle>().unwrap(), order());
}

#[test]
fn writes_body_fields_in_definition_order() {
    let text = order().to_message().to_string();
    assert_eq!(text, "35=D|11=ORD1|21=1|55=AAPL|54=1|60=20260927-03:20:48.544|38=100|40=2|44=150.25|59=0|");
}

#[test]
fn field_errors_carry_tag_and_reject_reason() {
    let cases = [
        (ORDER.replace("21=1|", ""), tags::HANDL_INST, FieldErrorKind::Missing),
        (ORDER.replace("60=20260927-03:20:48.544|", ""), tags::TRANSACT_TIME, FieldErrorKind::Missing),
        // "A" is a FIX 4.4 Side, not FIX 4.2.
        (ORDER.replace("54=1", "54=A"), tags::SIDE, FieldErrorKind::IncorrectValue("A".into())),
        (ORDER.replace("38=100", "38=1e3"), tags::ORDER_QTY, FieldErrorKind::IncorrectFormat("1e3".into())),
    ];
    for (text, tag, kind) in cases {
        assert_eq!(wire(&text).parse::<NewOrderSingle>(), Err(FieldError { tag, kind }), "{text}");
    }
    let err = wire(&ORDER.replace("21=1|", "")).parse::<NewOrderSingle>().unwrap_err();
    assert_eq!(err.to_string(), "Required tag 21 missing");
}

#[test]
fn parse_checks_msg_type() {
    let err = wire(ORDER).parse::<OrderCancelRequest>().unwrap_err();
    assert_eq!(err.tag, tags::MSG_TYPE);
}

#[test]
fn new_order_single_repeating_groups_round_trip_through_the_wire() {
    let text = ORDER.replace("21=1|", "78=2|79=ACCT-A|80=60|79=ACCT-B|80=40|21=1|386=1|336=MORNING|");
    let order = wire(&text).parse::<NewOrderSingle>().unwrap();
    let allocs: Vec<_> =
        order.allocs.iter().map(|a| (a.alloc_account.as_str(), a.alloc_shares.unwrap().to_string())).collect();
    assert_eq!(allocs, [("ACCT-A", "60".to_string()), ("ACCT-B", "40".to_string())]);
    assert_eq!(order.trading_sessions, [TrdgSesGrp::new("MORNING")]);
    assert_eq!(order.handl_inst, HandlInst::AutomatedExecutionNoIntervention);
    // Written back in the same order, and parsed again identically.
    let written = order.to_message();
    assert!(
        written.to_string().contains("|11=ORD1|78=2|79=ACCT-A|80=60|79=ACCT-B|80=40|21=1|386=1|336=MORNING|55=AAPL|"),
        "{written}"
    );
    assert_eq!(wire(&format!("8=FIX.4.2|{written}")).parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn a_bad_group_is_a_field_error_with_its_reject_reason() {
    let err = wire(&ORDER.replace("21=1|", "78=2|79=ACCT-A|21=1|")).parse::<NewOrderSingle>().unwrap_err();
    assert_eq!(err.tag, tags::NO_ALLOCS);
    assert_eq!(err.reject_reason(), SessionRejectReason::IncorrectNumInGroupCount);
    let err = wire(&ORDER.replace("21=1|", "78=1|80=60|79=ACCT-A|21=1|")).parse::<NewOrderSingle>().unwrap_err();
    assert_eq!(err.reject_reason(), SessionRejectReason::RepeatingGroupFieldsOutOfOrder);
}

#[test]
fn replace_and_status_requests_parse_from_the_wire() {
    let replace = wire(
        "8=FIX.4.2|35=G|49=C|56=G|34=3|52=20260927-03:20:48.544|41=ORD1|11=ORD2|21=1|55=AAPL|54=1|\
         60=20260927-03:20:49.000|38=150|40=2|44=151.00|",
    )
    .parse::<OrderCancelReplaceRequest>()
    .unwrap();
    assert_eq!((replace.orig_cl_ord_id.as_str(), replace.cl_ord_id.as_str()), ("ORD1", "ORD2"));
    assert_eq!(replace.order_qty, Some(Decimal::from_fix("150").unwrap()));
    assert_eq!(replace.price.unwrap().to_string(), "151.00");

    let status = wire("8=FIX.4.2|35=H|49=C|56=G|34=4|52=20260927-03:20:48.544|11=ORD2|55=AAPL|54=1|")
        .parse::<OrderStatusRequest>()
        .unwrap();
    assert_eq!(status.cl_ord_id, "ORD2");
    assert_eq!(status.side, Side::Buy);
    // Missing Side is a field error.
    let err = wire("8=FIX.4.2|35=H|49=C|56=G|34=4|52=20260927-03:20:48.544|11=ORD2|55=AAPL|")
        .parse::<OrderStatusRequest>()
        .unwrap_err();
    assert_eq!(err.tag, tags::SIDE);
}

fn order_report() -> ExecutionReport {
    let mut report = ExecutionReport::new(
        "O1",
        "E1",
        ExecTransType::New,
        ExecType::Rejected,
        OrdStatus::Rejected,
        "AAPL",
        Side::Sell,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    report.cl_ord_id = Some("ORD1".into());
    report.ord_rej_reason = Some(OrdRejReason::DuplicateOrder);
    report.order_qty = Some(Decimal::from_fix("10").unwrap());
    report.ord_type = Some(OrdType::Market);
    report.text = Some("dup".into());
    report
}

#[test]
fn execution_report_contra_brokers_round_trip() {
    let mut report = order_report();
    let mut first = ContraGrp::new("BRKA");
    first.contra_trader = Some("T1".into());
    first.contra_trade_qty = Some(Decimal::from_fix("6").unwrap());
    first.contra_trade_time = Some(UtcTimestamp::from_fix("20260927-03:20:48.544").unwrap());
    report.contra_brokers = vec![first, ContraGrp::new("BRKB")];
    let msg: Message = report.clone().into();
    assert!(msg.to_string().contains("|382=2|375=BRKA|337=T1|437=6|438=20260927-03:20:48.544|375=BRKB|"), "{msg}");
    let decoded = wire(&format!("8=FIX.4.2|{msg}"));
    assert_eq!(decoded.parse::<ExecutionReport>().unwrap(), report);
}

#[test]
fn execution_report_round_trips() {
    let report = order_report();
    let msg: Message = report.clone().into();
    assert_eq!(msg.msg_type(), MsgType::ExecutionReport);
    assert_eq!(msg.get(tags::EXEC_TRANS_TYPE), Some("0"));
    assert_eq!(msg.get(tags::ORD_REJ_REASON), Some("6"));
    assert_eq!(msg.get(tags::LEAVES_QTY), Some("0"));
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}
