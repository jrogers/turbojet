//! The Orchestra files in `dictionaries/orchestra` load and have the expected shape.

use turbojet_dictionary::{Category, Dictionary, FieldType, Member};

fn load(name: &str) -> Dictionary {
    let path = format!("{}/../../dictionaries/orchestra/{name}.xml", env!("CARGO_MANIFEST_DIR"));
    Dictionary::load(&path).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn shipped_orchestra_files_load() {
    // (file, version, fields, messages, components, admin msgtypes). StandardHeader and
    // StandardTrailer become the header and trailer rather than components.
    let expected = [
        ("OrchestraFIX42", "FIX 4.2 EP310", 405, 46, 0, vec!["0", "1", "2", "3", "4", "5", "A"]),
        // Converted from the Unified Repository, which has no extension packs for FIX 4.3.
        ("OrchestraFIX43", "FIX 4.3", 635, 68, 7, vec!["0", "1", "2", "3", "4", "5", "A", "n"]),
        ("FIXTSession", "FIXT 1.1 EP247", 92, 8, 0, vec!["0", "1", "2", "3", "4", "5", "A", "n"]),
        ("OrchestraFIX44", "FIX 4.4 EP311", 912, 93, 13, vec!["0", "1", "2", "3", "4", "5", "A", "n"]),
    ];
    for (file, version, fields, messages, components, admin) in expected {
        let dict = load(file);
        assert_eq!(dict.version.to_string(), version, "{file}");
        assert_eq!(
            (dict.fields().len(), dict.messages().len(), dict.components().len()),
            (fields, messages, components),
            "{file}"
        );
        let unknown: Vec<_> = dict.fields().iter().filter(|f| matches!(f.ty, FieldType::Other(_))).collect();
        assert!(unknown.is_empty(), "{file}: unrecognised types {unknown:?}");
        assert_eq!(dict.header()[0].name(), "BeginString", "{file}");
        assert_eq!(dict.trailer().last().unwrap().name(), "CheckSum", "{file}");
        let mut session: Vec<_> =
            dict.messages().iter().filter(|m| m.category == Category::Admin).map(|m| m.msg_type.as_str()).collect();
        session.sort_unstable();
        assert_eq!(session, admin, "{file}");
    }
}

#[test]
fn orchestra_fix42_uses_the_official_names() {
    let dict = load("OrchestraFIX42");
    assert_eq!(dict.field_by_tag(23).unwrap().name, "IOIID");
    let values: Vec<_> =
        dict.field("CxlRejResponseTo").unwrap().values.iter().map(|v| (v.code.as_str(), v.name.as_deref())).collect();
    assert_eq!(values, [("1", Some("OrderCancelRequest")), ("2", Some("OrderCancel"))]);
    // The same count field names different groups in different messages.
    for (message, group) in [("NewOrderSingle", "PreAllocGrp"), ("Allocation", "AllocGrp")] {
        let allocs = dict.message(message).unwrap().members.iter().find(|m| m.name() == "NoAllocs").unwrap();
        assert!(matches!(allocs, Member::Group { official_name: Some(g), .. } if g == group), "{message}: {allocs:?}");
    }
}

#[test]
fn orchestra_fix44_has_what_order_entry_needs() {
    let dict = load("OrchestraFIX44");
    let order = dict.message("NewOrderSingle").unwrap();
    assert_eq!((order.msg_type.as_str(), order.category), ("D", Category::App));
    assert!(order.members.iter().any(|m| matches!(m, Member::Component { name, .. } if name == "Instrument")));
    assert_eq!(dict.field("ClOrdID").unwrap().tag, 11);
    assert!(order.doc.is_some());
}

#[test]
fn fix50sp2_runs_over_fixt() {
    // The Unified Repository's FIX 5.0 SP2 carries a copy of FIXT's header and session messages;
    // the transport's own replace the header and trailer.
    let app = load("OrchestraFIX50SP2");
    assert_eq!(app.version.to_string(), "FIX 5.0 SP2");
    assert_eq!((app.fields().len(), app.messages().len()), (1452, 116));
    let mut session: Vec<_> =
        app.messages().iter().filter(|m| m.category == Category::Admin).map(|m| m.msg_type.as_str()).collect();
    session.sort_unstable();
    assert_eq!(session, ["0", "1", "2", "3", "4", "5", "A", "n"]);
    let fixt = load("FIXTSession");
    let dict = app.with_transport(&fixt).unwrap();
    assert_eq!(dict.header(), fixt.header());
    assert_eq!(dict.header()[0].name(), "BeginString");
    assert_eq!(dict.trailer().last().unwrap().name(), "CheckSum");
    assert!(dict.field("ApplVerID").is_some());
}

#[test]
fn data_fields_pair_each_with_the_length_before_it() {
    let pairs = |file| load(file).data_fields();
    assert_eq!(
        pairs("FIXTSession"),
        [(93, 89), (90, 91), (95, 96), (212, 213), (354, 355), (1401, 1402), (1403, 1404), (2111, 2112)]
    );
    let fix42 = pairs("OrchestraFIX42");
    assert!(fix42.contains(&(95, 96)) && fix42.contains(&(93, 89)) && fix42.contains(&(90, 91)), "{fix42:?}");
    let fix50 = pairs("OrchestraFIX50SP2");
    for pair in [(1277, 1278), (1280, 1281), (1397, 1398), (1184, 1185)] {
        assert!(fix50.contains(&pair), "{pair:?} in {fix50:?}");
    }
}
