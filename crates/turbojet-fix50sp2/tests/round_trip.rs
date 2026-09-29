//! Generated FIX 5.0 SP2 messages encode and decode, including flattened components and groups.

use turbojet::fields::{FromFix, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet_fix50sp2::*;

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

fn limit_order() -> NewOrderSingle {
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time(), OrdType::Limit);
    // From the Instrument and OrderQtyData components.
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order
}

#[test]
fn new_order_single_round_trips() {
    let mut order = limit_order();
    order.party_ids = vec![Parties::new("FIRM"), Parties::new("CLIENT")];
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=D|11=ORD1|"), "{text}");
    for field in ["|453=2|448=FIRM|448=CLIENT|", "|55=AAPL|", "|54=1|", "|38=100|", "|40=2|", "|44=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn execution_report_round_trips() {
    // AvgPx became optional in FIX 5.0.
    let mut report =
        ExecutionReport::new("O1", "E1", ExecType::Trade, OrdStatus::Filled, Side::Sell, 0.into(), 100.into());
    report.symbol = Some("AAPL".into());
    report.avg_px = Some("12.34".parse().unwrap());
    let msg = report.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=8|"), "{text}");
    for field in ["|37=O1|", "|17=E1|", "|150=F|", "|39=2|", "|54=2|", "|151=0|", "|14=100|", "|6=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}
