//! The fixtures' generated code matches the reviewed snapshots, and (in the
//! `*_compiles_and_round_trips` tests) the snapshots compile and round-trip. `mini` is a QuickFIX
//! dictionary; `orchestra-mini` an Orchestra repository, with official names and documentation.
//! Regenerate with `UPDATE_SNAPSHOTS=1`.

use turbojet_codegen::Generator;
use turbojet_dictionary::Dictionary;

/// The code for `fixtures/{name}.xml` matches `fixtures/{name}.rs`, which it first writes if
/// `UPDATE_SNAPSHOTS` is set, generated with `configure`'s options. The Orchestra snapshot is
/// generated with docs, to cover them, and `mini-lenient` with lenient enums.
fn check_snapshot(name: &str, xml: &str, configure: impl FnOnce(Generator) -> Generator) {
    let snapshot = format!("{}/tests/fixtures/{name}.rs", env!("CARGO_MANIFEST_DIR"));
    let dict = Dictionary::from_xml(xml).unwrap();
    let code = configure(Generator::new(&dict)).render().unwrap();
    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(&snapshot, &code).unwrap();
    }
    assert_eq!(code, std::fs::read_to_string(&snapshot).unwrap(), "rerun with UPDATE_SNAPSHOTS=1 and review the diff");
}

#[test]
fn matches_snapshot() {
    check_snapshot("mini", include_str!("fixtures/mini.xml"), |g| g);
}

#[test]
fn orchestra_matches_snapshot() {
    check_snapshot("orchestra-mini", include_str!("fixtures/orchestra-mini.xml"), |g| g.with_docs(true));
}

#[allow(dead_code)]
mod mini {
    include!("fixtures/mini.rs");
}

#[test]
fn lenient_matches_snapshot() {
    check_snapshot("mini-lenient", include_str!("fixtures/mini.xml"), |g| g.lenient_enums(true));
}

#[allow(dead_code)]
mod orchestra_mini {
    include!("fixtures/orchestra-mini.rs");
}

#[allow(dead_code)]
mod mini_lenient {
    include!("fixtures/mini-lenient.rs");
}

#[test]
fn snapshot_compiles_and_round_trips() {
    use mini::*;
    use turbojet::fields::{FromFix, UtcTimestamp};
    use turbojet::message::FixMessage;

    let time = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
    let mut order = NewOrderSingle::new("ORD1", Side::Buy, time);
    order.party_ids.push(PartyID::new("FIRM"));
    order.allocs.push(NewOrderSingleAlloc { alloc_account: "ACC".into(), alloc_qty: Some(10.into()) });
    order.text = Some("hi".into());
    let msg = order.to_message();
    assert!(msg.to_string().starts_with("35=D|11=ORD1|453=1|448=FIRM|78=1|79=ACC|80=10|54=1|60="), "{msg}");
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);

    // A required group is a new() parameter, in slot order, and must have an entry.
    let mut trade = TradeReport::new("T1", vec![SideEntry::new(Side::Sell)], 5.into());
    trade.r#yield = Some("1.5".parse().unwrap());
    assert!(format!("{trade:?}").contains("yield: Some(1.5)"), "raw identifiers print plain");
    let msg = trade.to_message();
    assert_eq!(msg.parse::<TradeReport>().unwrap(), trade);
    trade.sides.clear();
    let err = trade.to_message().parse::<TradeReport>().unwrap_err();
    assert_eq!((err.tag, err.kind), (tags::NO_SIDES, turbojet::message::FieldErrorKind::Missing));
    assert_eq!(tags::YIELD, 236);
}

#[test]
fn dates_times_and_lists_are_typed() {
    use mini::*;
    use turbojet::fields::{FromFix, MonthYear, NaiveDate, TzTimeOnly, TzTimestamp, UtcTimeOnly};
    use turbojet::message::FixMessage;

    let mut schedule = Schedule::new(vec![ExecInst::NotHeld, ExecInst::AllOrNone]);
    schedule.trade_date = Some(NaiveDate::from_fix("20260930").unwrap());
    schedule.maturity_month_year = Some(MonthYear::from_fix("202612w3").unwrap());
    schedule.md_entry_time = Some(UtcTimeOnly::from_fix("09:30:00.123456").unwrap());
    schedule.tz_transact_time = Some(TzTimestamp::from_fix("20260930-09:30:00-05").unwrap());
    schedule.session_open = Some(TzTimeOnly::from_fix("09:30-05").unwrap());
    schedule.venue_flag = Some('Q');
    schedule.labels = Some(vec!["a".into(), "b".into()]);
    let msg = schedule.to_message();
    assert_eq!(
        msg.to_string(),
        "35=U7|18=1 G|75=20260930|200=202612w3|273=09:30:00.123456|1132=20260930-09:30:00-05|5100=09:30-05|5101=Q|5102=a b|"
    );
    assert_eq!(msg.parse::<Schedule>().unwrap(), schedule);

    // Lenient, a list keeps codes the enum doesn't know.
    let odd = turbojet::Message::from_fields([(35, "U7"), (18, "1 Z")]);
    assert!(odd.parse::<Schedule>().is_err());
    let lenient = odd.parse::<mini_lenient::Schedule>().unwrap();
    assert_eq!(lenient.exec_inst[1], turbojet::fields::Code::Unknown("Z".into()));
}

#[test]
fn lenient_snapshot_keeps_unknown_codes() {
    use turbojet::fields::{Code, FromFix, UtcTimestamp};
    use turbojet::message::FixMessage;

    let time = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
    // A known value still goes straight into new().
    let order = mini_lenient::NewOrderSingle::new("ORD1", mini_lenient::Side::Buy, time);
    assert_eq!(order.side, mini_lenient::Side::Buy);

    // A code Side doesn't list: the strict message refuses it, the lenient one keeps it.
    let mut odd = order.clone();
    odd.side = Code::Unknown("Z".into());
    let msg = odd.to_message();
    assert!(msg.to_string().contains("|54=Z|"), "{msg}");
    assert!(msg.parse::<mini::NewOrderSingle>().is_err());
    assert_eq!(msg.parse::<mini_lenient::NewOrderSingle>().unwrap(), odd);
}

#[test]
fn orchestra_snapshot_compiles_and_round_trips() {
    use orchestra_mini::*;
    use turbojet::fields::{FromFix, UtcTimestamp};
    use turbojet::message::FixMessage;

    let time = UtcTimestamp::from_fix("20260928-12:00:00.000").unwrap();
    let mut order =
        NewOrderSingle::new("ORD1", HandlInst::AutomatedExecutionInterventionOK, "IBM", Side::SellShort, time);
    order.allocs.push(PreAllocGrp::new("ACC"));
    order.id_source = Some(IDSource::CUSIP);
    order.sending_date = Some("20260928".into());
    let msg = order.to_message();
    assert!(msg.to_string().starts_with("35=D|11=ORD1|78=1|79=ACC|21=2|55=IBM|22=1|54=5|60="), "{msg}");
    assert_eq!(msg.parse::<NewOrderSingle>().unwrap(), order);

    // A nested group, in a required one.
    let mut alloc = AllocGrp::new("ACC", 10.into());
    alloc.misc_fees.push(MiscFeesGrp::new("1.5".parse().unwrap()));
    let allocation = Allocation::new("A1", Side::Buy, vec![alloc]);
    let msg = allocation.to_message();
    assert_eq!(msg.to_string(), "35=J|70=A1|54=1|78=1|79=ACC|80=10|136=1|137=1.5|");
    assert_eq!(msg.parse::<Allocation>().unwrap(), allocation);
}

#[test]
fn write_errors_name_the_path() {
    let dict = Dictionary::from_xml(include_str!("fixtures/mini.xml")).unwrap();
    let dir = tempfile::tempdir().unwrap();
    // A file where the output directory should be.
    let file = dir.path().join("file");
    std::fs::write(&file, "").unwrap();
    let err = Generator::new(&dict).write_modules(&file).unwrap_err().to_string();
    assert!(err.starts_with(&file.display().to_string()), "{err}");
    let err = Generator::new(&dict).write_modules(&file.join("out")).unwrap_err().to_string();
    assert!(err.contains(&file.display().to_string()), "{err}");
}
