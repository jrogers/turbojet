//! Messages built with the generated FIX version crates pass validation against the official
//! dictionaries they were generated from. Needs the repository's dictionaries/.
#![cfg(feature = "validation")]

use std::path::Path;

use turbojet::fields::{FromFix, SessionRejectReason, UtcTimestamp};
use turbojet::message::FixMessage;
use turbojet::validation::Validator;
use turbojet_dictionary::Dictionary;

fn dictionary(name: &str) -> Dictionary {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dictionaries/orchestra").join(name);
    Dictionary::load(path).unwrap_or_else(|e| panic!("{e}"))
}

fn time() -> UtcTimestamp {
    UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap()
}

#[test]
fn fix40_messages_pass() {
    use turbojet_fix::fix40::*;
    let validator = Validator::new(&dictionary("OrchestraFIX40.xml"));
    let mut order =
        OrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, 100, OrdType::Limit);
    order.price = Some("12.34".parse().unwrap());
    order.exec_inst = Some(vec![ExecInst::NotHeld.into(), ExecInst::Work.into()]);
    validator.validate(&order.to_message()).unwrap();
    let news = News::new(vec![LinesOfTextGrp::new("first"), LinesOfTextGrp::new("second")]);
    validator.validate(&news.to_message()).unwrap();
    // FIX 4.0 has no ResetSeqNumFlag(141), nor any tag above 140.
    let err = validator.validate(&order.to_message().with(141, "Y")).unwrap_err();
    assert_eq!(err.tag, Some(141));
}

#[test]
fn fix41_messages_pass() {
    use turbojet_fix::fix41::*;
    let validator = Validator::new(&dictionary("OrchestraFIX41.xml"));
    let mut order =
        OrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, "AAPL", Side::Buy, OrdType::Limit);
    order.order_qty = Some(100);
    order.price = Some("12.34".parse().unwrap());
    validator.validate(&order.to_message()).unwrap();
    let mut report = ExecutionReport::new(
        "O1",
        "E1",
        ExecTransType::New,
        ExecType::Fill,
        OrdStatus::Filled,
        "AAPL",
        Side::Sell,
        100,
        100,
        "12.34".parse().unwrap(),
        0,
        100,
        "12.34".parse().unwrap(),
    );
    report.transact_time = Some(time());
    validator.validate(&report.to_message()).unwrap();
}

#[test]
fn fix42_messages_pass() {
    use turbojet_fix::fix42::*;
    let mut dict = dictionary("OrchestraFIX42.xml");
    // turbojet-fix's FIX 4.2 is generated with this correction.
    dict.make_optional("QuoteCancel", "NoQuoteEntries").unwrap();
    let validator = Validator::new(&dict);

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
    order.time_in_force = Some(TimeInForce::Day.into());
    order.allocs.push(PreAllocGrp::new("ACC1"));
    validator.validate(&order.to_message()).unwrap();

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
    validator.validate(&report.to_message()).unwrap();
    validator.validate(&QuoteCancel::new("Q1", QuoteCancelType::CancelAllQuotes).to_message()).unwrap();

    // Without the correction, the cancel-all request is missing NoQuoteEntries.
    let strict = Validator::new(&dictionary("OrchestraFIX42.xml"));
    let err = strict.validate(&QuoteCancel::new("Q1", QuoteCancelType::CancelAllQuotes).to_message()).unwrap_err();
    assert_eq!((err.tag, err.reason), (Some(295), Some(SessionRejectReason::RequiredTagMissing)));

    // What typed parsing ignores, validation refuses: ExecType isn't a NewOrderSingle field.
    let err = validator.validate(&order.to_message().with(150, "0")).unwrap_err();
    assert_eq!((err.tag, err.reason), (Some(150), Some(SessionRejectReason::TagNotDefinedForMessageType)));
}

#[test]
fn fix43_messages_pass() {
    use turbojet_fix::fix43::*;
    let validator = Validator::new(&dictionary("OrchestraFIX43.xml"));

    let mut order =
        NewOrderSingle::new("ORD1", HandlInst::AutomatedExecutionNoIntervention, Side::Buy, time(), OrdType::Limit);
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    order.party_ids.push(Parties::new("FIRM"));
    order.security_alt_id.push(SecAltIDGrp::new("US0378331005"));
    validator.validate(&order.to_message()).unwrap();

    let mut report = ExecutionReport::new(
        "O1",
        "E1",
        ExecType::Fill,
        OrdStatus::Filled,
        Side::Sell,
        0.into(),
        100.into(),
        "12.34".parse().unwrap(),
    );
    report.symbol = Some("AAPL".into());
    validator.validate(&report.to_message()).unwrap();
}

#[test]
fn fix50sp2_messages_pass_over_fixt() {
    use turbojet_fix::fix50sp2::*;
    let dict = dictionary("OrchestraFIX50SP2.xml").with_transport(&dictionary("FIXTSession.xml")).unwrap();
    let validator = Validator::new(&dict);
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time(), OrdType::Limit);
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.party_ids.push(Parties::new("FIRM"));
    // FIXT's header field ApplVerID(1128) is the transport's, not the body's.
    validator.validate(&order.to_message().with(1128, "9")).unwrap();
}

#[test]
fn fix44_messages_pass() {
    use turbojet_fix::fix44::*;
    let validator = Validator::new(&dictionary("OrchestraFIX44.xml"));

    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time(), OrdType::Limit);
    order.symbol = Some("AAPL".into());
    order.order_qty = Some(100.into());
    order.price = Some("12.34".parse().unwrap());
    let mut firm = Parties::new("FIRM");
    firm.party_role = Some(PartyRole::ExecutingFirm.into());
    firm.party_sub_ids.push(PtysSubGrp::new("DESK1"));
    order.party_ids.push(firm);
    validator.validate(&order.to_message()).unwrap();

    let mut report = ExecutionReport::new(
        "O1",
        "E1",
        ExecType::Trade,
        OrdStatus::Filled,
        Side::Sell,
        0.into(),
        100.into(),
        "12.34".parse().unwrap(),
    );
    report.symbol = Some("AAPL".into());
    validator.validate(&report.to_message()).unwrap();
}
