//! Generated FIX 4.1 messages encode and decode, including groups.
#![cfg(feature = "fix41")]

use turbojet::message::FixMessage;
use turbojet_fix::fix41::*;

#[test]
fn order_single_round_trips() {
    let mut order =
        OrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, OrdType::Limit);
    order.order_qty = Some(100);
    order.price = Some("12.34".parse().unwrap());
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=D|11=ORD1|"), "{text}");
    for field in ["|21=1|", "|55=AAPL|", "|54=1|", "|38=100|", "|40=2|", "|44=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<OrderSingle>().unwrap(), order);
}

#[test]
fn groups_round_trip() {
    let news = News::new("Headline", vec![LinesOfTextGrp::new("first"), LinesOfTextGrp::new("second")]);
    let msg = news.to_message();
    assert!(msg.to_string().contains("|33=2|58=first|58=second|"), "{msg}");
    assert_eq!(msg.parse::<News>().unwrap(), news);
}

#[test]
fn execution_report_round_trips() {
    let (px, avg_px) = ("12.34".parse().unwrap(), "12.34".parse().unwrap());
    let report = ExecutionReport::new(
        "O1",
        "E1",
        ExecTransType::New,
        ExecType::Fill,
        OrdStatus::Filled,
        "AAPL",
        Side::Sell,
        100,
        100,
        px,
        0,
        100,
        avg_px,
    );
    let msg = report.to_message();
    let text = msg.to_string();
    for field in ["|37=O1|", "|17=E1|", "|150=2|", "|39=2|", "|151=0|", "|14=100|", "|6=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}
