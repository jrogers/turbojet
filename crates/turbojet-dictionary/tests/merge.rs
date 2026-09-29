use turbojet_dictionary::{Dictionary, FieldType, Member};

const BASE: &str = include_str!("fixtures/base.xml");
const VENUE: &str = include_str!("fixtures/venue.xml");

fn merged() -> Dictionary {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    dict.merge_xml(VENUE).unwrap();
    dict
}

#[test]
fn adds_fields_messages_and_components() {
    let dict = merged();
    assert_eq!(dict.version.to_string(), "FIX 4.4", "an overlay keeps the base version");
    assert_eq!(dict.field_by_tag(5001).unwrap().name, "VenueFlag");
    assert_eq!(dict.message("VenueAck").unwrap().msg_type, "U1");
    assert!(dict.component("VenueParties").is_some());
    assert_eq!(dict.messages().len(), 3);
}

#[test]
fn extends_enum_values_and_takes_venue_descriptions() {
    let side = merged().field("Side").unwrap().clone();
    let values: Vec<_> = side.values.iter().map(|v| (v.code.as_str(), v.description.as_str())).collect();
    assert_eq!(values, [("1", "BUY"), ("2", "SELL_ALL"), ("5", "SELL_SHORT")]);
}

#[test]
fn keeps_base_descriptions_when_the_venue_has_none() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let venue = "<fix><fields><field number='54' name='Side' type='CHAR'>\
         <value enum='1'/><value enum='2' description=''/><value enum='9'/></field></fields></fix>";
    dict.merge_xml(venue).unwrap();
    let values: Vec<_> =
        dict.field("Side").unwrap().values.iter().map(|v| (v.code.as_str(), v.description.as_str())).collect();
    assert_eq!(values, [("1", "BUY"), ("2", "SELL"), ("9", "")]);
}

#[test]
fn replaces_messages_whole() {
    let dict = merged();
    let names: Vec<_> = dict.message("NewOrderSingle").unwrap().members.iter().map(Member::name).collect();
    assert_eq!(names, ["ClOrdID", "Side", "VenueFlag"]);
}

#[test]
fn keeps_header_and_trailer_when_the_venue_has_none() {
    let dict = merged();
    assert_eq!(dict.header().len(), 2);
    assert_eq!(dict.trailer().len(), 1);
}

#[test]
fn tag_conflicts_are_errors() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let renumbered = "<fix><fields>\n<field number='9054' name='Side' type='CHAR'/>\n</fields></fix>";
    let err = dict.merge_xml(renumbered).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(2), "field Side has tag 9054, but 54 in the base dictionary"));

    let reused = "<fix><fields>\n<field number='54' name='VenueSide' type='CHAR'/>\n</fields></fix>";
    let err = dict.merge_xml(reused).unwrap_err();
    assert_eq!(err.message, "tag 54 is VenueSide, but Side in the base dictionary");
}

#[test]
fn msgtype_conflicts_are_errors() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let reused = "<fix><messages>\n<message name='VenueOrder' msgtype='D' msgcat='app'>\
         <field name='ClOrdID' required='Y'/></message>\n</messages></fix>";
    let err = dict.merge_xml(reused).unwrap_err();
    assert_eq!(
        (err.line, err.message.as_str()),
        (Some(2), "msgtype D is VenueOrder, but NewOrderSingle in the base dictionary")
    );
    assert!(dict.message("VenueOrder").is_none());

    // Replacing a base message under its own name keeps its msgtype.
    let replaced = "<fix><messages><message name='NewOrderSingle' msgtype='D' msgcat='app'>\
         <field name='ClOrdID' required='Y'/></message></messages></fix>";
    dict.merge_xml(replaced).unwrap();
    assert_eq!(dict.message("NewOrderSingle").unwrap().members.len(), 1);
}

#[test]
fn group_count_fields_stay_countable() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let retyped = "<fix><fields>\n<field number='78' name='NoAllocs' type='STRING'/>\n</fields></fix>";
    let err = dict.merge_xml(retyped).unwrap_err();
    assert_eq!(
        (err.line, err.message.as_str()),
        (Some(2), "field NoAllocs counts a group in the base dictionary, so it must stay NUMINGROUP or INT")
    );
    assert_eq!(dict.field("NoAllocs").unwrap().ty, FieldType::NumInGroup);

    // A group in a component counts too.
    let retyped = "<fix><fields><field number='453' name='NoPartyIDs' type='STRING'/></fields></fix>";
    assert!(dict.merge_xml(retyped).unwrap_err().message.contains("NoPartyIDs"));

    let still_counts = "<fix><fields><field number='78' name='NoAllocs' type='INT'/></fields></fix>";
    dict.merge_xml(still_counts).unwrap();
    assert_eq!(dict.field("NoAllocs").unwrap().ty, FieldType::Int);

    let not_a_count = "<fix><fields><field number='79' name='AllocAccount' type='CHAR'/></fields></fix>";
    dict.merge_xml(not_a_count).unwrap();
    assert_eq!(dict.field("AllocAccount").unwrap().ty, FieldType::Char);
}

#[test]
fn a_failed_merge_leaves_the_dictionary_unchanged() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let bad = "<fix><messages><message name='X' msgtype='X' msgcat='app'><field name='Nope' required='Y'/></message></messages></fix>";
    assert!(dict.merge_xml(bad).is_err());
    assert!(dict.message("X").is_none());
}

#[test]
fn merge_file_reports_the_path() {
    let mut dict = Dictionary::from_xml(BASE).unwrap();
    let err = dict.merge_file("no/such/venue.xml").unwrap_err();
    assert!(err.to_string().starts_with("no/such/venue.xml: "), "{err}");
}

#[test]
fn component_cycles_through_the_base_are_errors() {
    let base = "<fix type='FIX' major='4' minor='4'><components>\
         <component name='Outer'><component name='Inner' required='Y'/></component>\
         <component name='Inner'><field name='A' required='Y'/></component>\
         </components><fields><field number='1' name='A' type='STRING'/></fields></fix>";
    let mut dict = Dictionary::from_xml(base).unwrap();
    let venue = "<fix><components>\n<component name='Inner'><component name='Outer' required='N'/></component>\n\
         </components></fix>";
    let err = dict.merge_xml(venue).unwrap_err();
    assert_eq!(
        (err.line, err.message.as_str()),
        (Some(2), "component Inner includes itself (Inner -> Outer -> Inner)")
    );
    assert_eq!(dict.component("Inner").unwrap().members[0].name(), "A");
}

const FIX50: &str = include_str!("fixtures/fix50.xml");
const FIXT11: &str = include_str!("fixtures/fixt11.xml");

#[test]
fn fix50_takes_its_header_and_trailer_from_fixt() {
    let fix50 = Dictionary::from_xml(FIX50).unwrap();
    assert!(fix50.header().is_empty() && fix50.trailer().is_empty(), "FIX 5.0 files have no header or trailer");
    let transport = Dictionary::from_xml(FIXT11).unwrap();
    let dict = fix50.with_transport(&transport).unwrap();
    assert_eq!(dict.version.to_string(), "FIX 5.0 SP2");
    assert_eq!(dict.header(), transport.header());
    assert_eq!(dict.trailer(), transport.trailer());
    // Every header and trailer field is now defined.
    fn names(members: &[Member], out: &mut Vec<String>) {
        for m in members {
            out.push(m.name().to_string());
            if let Member::Group { members, .. } = m {
                names(members, out);
            }
        }
    }
    let mut all = Vec::new();
    names(dict.header(), &mut all);
    names(dict.trailer(), &mut all);
    for name in all {
        assert!(dict.field(&name).is_some(), "{name}");
    }
}

#[test]
fn fix50_gains_the_transport_fields_and_components_it_lacks() {
    let fix50 = Dictionary::from_xml(FIX50).unwrap();
    assert!(fix50.field("ApplExtID").is_none());
    let dict = fix50.with_transport(&Dictionary::from_xml(FIXT11).unwrap()).unwrap();
    assert_eq!(dict.field("ApplExtID").unwrap().tag, 1156);
    assert_eq!(dict.field_by_tag(1156).unwrap().name, "ApplExtID");
    // Text is in both; the transport's other seven fields are added.
    assert_eq!(dict.fields().len(), 2 + 7);
    assert!(dict.component("HopGrp").is_some());
    // The transport's (admin) messages aren't.
    let messages: Vec<_> = dict.messages().iter().map(|m| m.name.as_str()).collect();
    assert_eq!(messages, ["NewOrderSingle"]);
}

#[test]
fn the_transport_must_be_fixt() {
    let fix50 = Dictionary::from_xml(FIX50).unwrap();
    let err = fix50.with_transport(&Dictionary::from_xml(FIX50).unwrap()).unwrap_err();
    assert_eq!(err.message, "the transport must be a FIXT dictionary, not FIX 5.0 SP2");
}
