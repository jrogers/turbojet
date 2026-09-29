use turbojet_dictionary::{Category, Dictionary, FieldType, Member, Protocol, Release};

const BASE: &str = include_str!("fixtures/base.xml");

/// A minimal document: `body` goes inside `<fix>`, starting on line 2.
fn doc(body: &str) -> String {
    format!("<fix type='FIX' major='4' minor='2'>\n{body}\n</fix>")
}

#[test]
fn reads_version_and_fields() {
    let dict = Dictionary::from_xml(BASE).unwrap();
    assert_eq!(dict.version.protocol, Protocol::Fix);
    assert_eq!(dict.version.release, Release::Numbered { major: 4, minor: 4, service_pack: 0 });
    assert_eq!(dict.version.extension_pack, None);
    assert_eq!(dict.version.to_string(), "FIX 4.4");
    assert_eq!(dict.fields().len(), 12);
    let side = dict.field("Side").unwrap();
    assert_eq!((side.tag, &side.ty), (54, &FieldType::Char));
    let sell = &side.values[1];
    assert_eq!((sell.code.as_str(), &sell.name, sell.description.as_str(), &sell.doc), ("2", &None, "SELL", &None));
    assert_eq!(dict.field_by_tag(80).unwrap().name, "AllocQty");
    assert_eq!(dict.field_by_tag(80).unwrap().ty, FieldType::Qty);
    assert!(dict.field("Nope").is_none());
}

#[test]
fn service_pack_and_fixt_versions_display() {
    let sp2 = Dictionary::from_xml("<fix type='FIX' major='5' minor='0' servicepack='2'/>").unwrap();
    assert_eq!(sp2.version.to_string(), "FIX 5.0 SP2");
    let fixt = Dictionary::from_xml("<fix type='FIXT' major='1' minor='1'/>").unwrap();
    assert_eq!(fixt.version.to_string(), "FIXT 1.1");
}

#[test]
fn recognises_the_quickfix_types() {
    // Every type QuickFIX's FIX 4.2 to 5.0 SP2 and FIXT 1.1 dictionaries use.
    let types = "AMT BOOLEAN CHAR COUNTRY CURRENCY DATA DAYOFMONTH EXCHANGE FLOAT INT LANGUAGE LENGTH \
         LOCALMKTDATE LOCALMKTTIME MONTHYEAR MULTIPLECHARVALUE MULTIPLESTRINGVALUE MULTIPLEVALUESTRING \
         NUMINGROUP PERCENTAGE PRICE PRICEOFFSET QTY SEQNUM STRING TAGNUM TZTIMEONLY TZTIMESTAMP UTCDATE \
         UTCDATEONLY UTCTIMEONLY UTCTIMESTAMP XID XIDREF XMLDATA";
    let fields: String = types
        .split_whitespace()
        .enumerate()
        .map(|(i, ty)| format!("<field number='{}' name='F{i}' type='{ty}'/>", i + 1))
        .collect();
    let dict = Dictionary::from_xml(&doc(&format!("<fields>{fields}</fields>"))).unwrap();
    assert_eq!(dict.fields().len(), 35);
    let unknown: Vec<_> = dict.fields().iter().filter(|f| matches!(f.ty, FieldType::Other(_))).collect();
    assert!(unknown.is_empty(), "{unknown:?}");
}

#[test]
fn unknown_types_are_kept() {
    let dict = Dictionary::from_xml(&doc("<fields>\n<field number='1' name='A' type='WEIRD'/>\n</fields>")).unwrap();
    assert_eq!(dict.field("A").unwrap().ty, FieldType::Other("WEIRD".into()));
}

#[test]
fn field_errors_name_the_line() {
    let err = Dictionary::from_xml(&doc("<fields>\n<field number='x' name='A' type='INT'/>\n</fields>")).unwrap_err();
    assert_eq!(err.line, Some(3));
    assert!(err.message.contains("number"), "{err}");

    let dup_name =
        "<fields>\n<field number='1' name='A' type='INT'/>\n<field number='2' name='A' type='INT'/>\n</fields>";
    let err = Dictionary::from_xml(&doc(dup_name)).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "field A is defined twice"));

    let dup_tag =
        "<fields>\n<field number='1' name='A' type='INT'/>\n<field number='1' name='B' type='INT'/>\n</fields>";
    let err = Dictionary::from_xml(&doc(dup_tag)).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "tag 1 is both A and B"));
    assert_eq!(err.to_string(), "line 4: tag 1 is both A and B");
}

#[test]
fn rejects_a_document_that_is_not_a_dictionary() {
    assert!(Dictionary::from_quickfix("<fox/>").unwrap_err().message.contains("<fix>"));
    assert!(Dictionary::from_xml("<fix").is_err());
    let err =
        Dictionary::from_xml("<fix type='FIX' major='4' minor='2'>\n<fields>\n<field number='1'\n</fix>").unwrap_err();
    assert_eq!(err.line, Some(4), "{err}");
    assert!(Dictionary::from_xml("<fix type='FIX' minor='2'/>").unwrap_err().message.contains("major"));
}

#[test]
fn load_reports_the_path() {
    let err = Dictionary::load("no/such/FIX99.xml").unwrap_err();
    assert!(err.to_string().starts_with("no/such/FIX99.xml: "), "{err}");
}

fn field(name: &str, required: bool) -> Member {
    Member::Field { name: name.into(), required }
}

#[test]
fn reads_header_trailer_components_and_messages() {
    let dict = Dictionary::from_xml(BASE).unwrap();
    assert_eq!(dict.header(), [field("BeginString", true), field("MsgType", true)]);
    assert_eq!(dict.trailer(), [field("CheckSum", true)]);

    let parties = dict.component("Parties").unwrap();
    assert_eq!(
        parties.members,
        [Member::Group {
            name: "NoPartyIDs".into(),
            official_name: None,
            required: false,
            members: vec![field("PartyID", false), field("PartyRole", false)],
        }]
    );

    assert_eq!(dict.messages().len(), 2);
    assert_eq!(dict.message("Heartbeat").unwrap().category, Category::Admin);
    let order = dict.message("NewOrderSingle").unwrap();
    assert_eq!((order.msg_type.as_str(), order.category), ("D", Category::App));
    let names: Vec<_> = order.members.iter().map(Member::name).collect();
    assert_eq!(names, ["ClOrdID", "Parties", "NoAllocs", "Side"]);
    assert!(matches!(&order.members[1], Member::Component { required: false, .. }));
    assert!(order.members[3].required());
}

/// `fields` holds field definitions, `body` goes inside a message; the message starts on line 2.
fn message_doc(body: &str, fields: &str) -> String {
    doc(&format!(
        "<messages><message name='M' msgtype='U1' msgcat='app'>\n{body}\n</message></messages>\n<fields>{fields}</fields>"
    ))
}

const COUNT_AND_A: &str = "<field number='1' name='A' type='STRING'/><field number='2' name='NoAs' type='NUMINGROUP'/>";

#[test]
fn reference_errors_name_the_line() {
    let cases = [
        ("<field name='Nope' required='Y'/>", "unknown field Nope"),
        ("<component name='Nope' required='N'/>", "unknown component Nope"),
        ("<group name='Nope' required='N'><field name='A' required='N'/></group>", "unknown field Nope"),
        (
            "<group name='A' required='N'><field name='A' required='N'/></group>",
            "group A's count field must be NUMINGROUP or INT",
        ),
        ("<group name='NoAs' required='N'></group>", "group NoAs has no members"),
        ("<thing name='A'/>", "unexpected <thing>"),
    ];
    for (body, message) in cases {
        let err = Dictionary::from_xml(&message_doc(body, COUNT_AND_A)).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(3), message), "{body}");
    }
}

#[test]
fn fix42_groups_count_with_int_fields() {
    let fields = "<field number='1' name='A' type='STRING'/><field number='2' name='NoAs' type='INT'/>";
    let body = "<group name='NoAs' required='N'><field name='A' required='N'/></group>";
    assert!(Dictionary::from_xml(&message_doc(body, fields)).is_ok());
}

#[test]
fn components_may_refer_to_components_defined_later() {
    let xml = doc("<components>\n<component name='Outer'><component name='Inner' required='Y'/></component>\n\
         <component name='Inner'><field name='A' required='Y'/></component>\n</components>\n\
         <fields><field number='1' name='A' type='STRING'/></fields>");
    let dict = Dictionary::from_xml(&xml).unwrap();
    assert_eq!(dict.components().len(), 2);
}

#[test]
fn duplicate_components_and_messages_name_the_line() {
    let fields = "<fields><field number='1' name='A' type='STRING'/></fields>";
    let components = "<components>\n<component name='C'><field name='A' required='Y'/></component>\n\
         <component name='C'><field name='A' required='N'/></component>\n</components>";
    let err = Dictionary::from_xml(&doc(&format!("{components}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "component C is defined twice"));

    let messages = "<messages>\n<message name='M' msgtype='U1' msgcat='app'><field name='A' required='Y'/></message>\n\
         <message name='M' msgtype='U2' msgcat='app'><field name='A' required='Y'/></message>\n</messages>";
    let err = Dictionary::from_xml(&doc(&format!("{messages}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "message M is defined twice"));

    let msgtypes = "<messages>\n<message name='M' msgtype='U1' msgcat='app'><field name='A' required='Y'/></message>\n\
         <message name='N' msgtype='U1' msgcat='app'><field name='A' required='Y'/></message>\n</messages>";
    let err = Dictionary::from_xml(&doc(&format!("{msgtypes}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "msgtype U1 is both M and N"));
}

#[test]
fn component_cycles_name_the_line() {
    let fields =
        "<fields><field number='1' name='A' type='STRING'/><field number='2' name='NoAs' type='NUMINGROUP'/></fields>";
    let own = "<components>\n<component name='C'><field name='A' required='Y'/></component>\n\
         <component name='S'><component name='S' required='Y'/></component>\n</components>";
    let err = Dictionary::from_xml(&doc(&format!("{own}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "component S includes itself (S -> S)"));

    let pair = "<components>\n<component name='X'><component name='Y' required='N'/></component>\n\
         <component name='Y'><component name='X' required='N'/></component>\n</components>";
    let err = Dictionary::from_xml(&doc(&format!("{pair}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(3), "component X includes itself (X -> Y -> X)"));

    let via_group = "<components>\n<component name='X'><field name='A' required='N'/></component>\n\
         <component name='Y'><group name='NoAs' required='N'><component name='Y' required='N'/></group></component>\n\
         </components>";
    let err = Dictionary::from_xml(&doc(&format!("{via_group}\n{fields}"))).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "component Y includes itself (Y -> Y)"));
}

#[test]
fn duplicate_enum_codes_name_the_line() {
    let fields = "<fields><field number='54' name='Side' type='CHAR'>\n<value enum='1' description='BUY'/>\n\
         <value enum='2' description='SELL'/>\n<value enum='2' description='SELL_AGAIN'/>\n</field></fields>";
    let err = Dictionary::from_xml(&doc(fields)).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(5), "field Side has value 2 twice"));
}
