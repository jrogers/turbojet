//! Generated FIX 4.3 messages encode and decode, including flattened components and groups.

use turbojet::fields::{FromFix, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet_fix43::*;

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

fn limit_order() -> NewOrderSingle {
    let mut order =
        NewOrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, Side::Buy, time(), OrdType::Limit);
    // From the Instrument and OrderQtyData components.
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order
}

#[test]
fn new_order_single_round_trips() {
    let order = limit_order();
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=D|11=ORD1|"), "{text}");
    for field in ["|21=1|", "|55=AAPL|", "|54=1|", "|38=100|", "|40=2|", "|44=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn groups_round_trip() {
    let mut order = limit_order();
    order.party_ids = vec![Parties::new("FIRM"), Parties::new("CLIENT")];
    // Instrument's NoSecurityAltID, which FIX 4.3's repository declares inline.
    let mut isin = SecAltIDGrp::new("US0378331005");
    isin.security_alt_id_source = Some("4".into());
    order.security_alt_id.push(isin);
    let msg = order.to_message();
    let text = msg.to_string();
    assert!(text.contains("|453=2|448=FIRM|448=CLIENT|"), "{text}");
    assert!(text.contains("|454=1|455=US0378331005|456=4|"), "{text}");
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);
}

#[test]
fn execution_report_round_trips() {
    let (leaves_qty, cum_qty, avg_px) = (0.into(), 100.into(), "12.34".parse().unwrap());
    let mut report =
        ExecutionReport::new("O1", "E1", ExecType::Fill, OrdStatus::Filled, Side::Sell, leaves_qty, cum_qty, avg_px);
    report.symbol = Some("AAPL".into());
    let msg = report.to_message();
    let text = msg.to_string();
    assert!(text.starts_with("35=8|"), "{text}");
    for field in ["|37=O1|", "|17=E1|", "|150=2|", "|39=2|", "|54=2|", "|151=0|", "|14=100|", "|6=12.34|"] {
        assert!(text.contains(field), "{field} in {text}");
    }
    assert_eq!(msg.parse::<ExecutionReport>().unwrap(), report);
}
