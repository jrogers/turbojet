//! Generated FIX 4.0 messages encode and decode, including groups.
#![cfg(feature = "fix40")]

use turbojet::message::FixMessage;
use turbojet_fix::fix40::*;

#[test]
fn order_single_round_trips() {
    let mut order = OrderSingle::new(OrderSingleRequired {
        cl_ord_id: "ORD1".into(),
        handl_inst: HandlInst::AutomatedExecutionNoIntervention.into(),
        symbol: "AAPL".into(),
        side: Side::Buy.into(),
        order_qty: 100,
        ord_type: OrdType::Limit.into(),
    });
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
fn news_lines_of_text_are_a_group() {
    // FIX 4.0 says only in prose that Text(58) repeats LinesOfText(33) times.
    let news = News::new(vec![LinesOfTextGrp::new("first"), LinesOfTextGrp::new("second")]);
    let msg = news.to_message();
    assert!(msg.to_string().contains("|33=2|58=first|58=second|"), "{msg}");
    assert_eq!(msg.parse::<News>().unwrap(), news);
}

#[test]
fn execution_report_round_trips() {
    let (px, avg_px) = ("12.34".parse().unwrap(), "12.34".parse().unwrap());
    let report = ExecutionReport::new(ExecutionReportRequired {
        order_id: "O1".into(),
        exec_id: 1,
        exec_trans_type: ExecTransType::New.into(),
        ord_status: OrdStatus::Filled.into(),
        symbol: "AAPL".into(),
        side: Side::Sell.into(),
        order_qty: 100,
        last_shares: 100,
        last_px: px,
        cum_qty: 100,
        avg_px,
    });
    let msg = report.to_message();
    let text = msg.to_string();
    for field in ["|37=O1|", "|17=1|", "|20=0|", "|39=2|", "|54=2|", "|32=100|", "|31=12.34|", "|14=100|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}
