use turbojet_dictionary::{Category, Dictionary, Error, FieldType, Member, Protocol, Release, Value};

/// A value's code, official name, QuickFIX description and documentation, for comparing.
fn parts(value: &Value) -> (&str, Option<&str>, &str, Option<&str>) {
    (&value.code, value.name.as_deref(), &value.description, value.doc.as_deref())
}

/// A repository named `name`, version `version`: `sections` start on line 2.
fn repository(name: &str, version: &str, sections: &str) -> String {
    format!(
        "<fixr:repository xmlns:fixr='http://fixprotocol.io/2020/orchestra/repository' name='{name}' \
         version='{version}'>\n{sections}\n</fixr:repository>"
    )
}

/// A FIX 4.2 EP310 repository: `sections` start on line 2.
fn repo(sections: &str) -> String {
    repository("FIX.4.2", "FIX.4.2_EP310", sections)
}

fn load(sections: &str) -> Dictionary {
    Dictionary::from_orchestra(&repo(sections)).unwrap_or_else(|e| panic!("{e}"))
}

fn error(sections: &str) -> Error {
    Dictionary::from_orchestra(&repo(sections)).unwrap_err()
}

/// A SYNOPSIS annotation.
fn synopsis(text: &str) -> String {
    format!("<fixr:annotation><fixr:documentation purpose='SYNOPSIS'>{text}</fixr:documentation></fixr:annotation>")
}

#[test]
fn reads_versions() {
    let version = |name, version| Dictionary::from_orchestra(&repository(name, version, "")).unwrap().version;
    let v = version("FIX.4.2", "FIX.4.2_EP310");
    assert_eq!(v.protocol, Protocol::Fix);
    assert_eq!(v.release, Release::Numbered { major: 4, minor: 2, service_pack: 0 });
    assert_eq!(v.extension_pack, Some(310));
    assert_eq!(v.to_string(), "FIX 4.2 EP310");

    let latest = version("FIX.Latest", "FIX.Latest_EP312");
    assert_eq!((latest.protocol, latest.release, latest.extension_pack), (Protocol::Fix, Release::Latest, Some(312)));
    assert_eq!(latest.to_string(), "FIX Latest EP312");

    assert_eq!(version("FIXT.1.1", "FIXT.1.1").to_string(), "FIXT 1.1");
    assert_eq!(version("FIXT", "FIXT_EP312").to_string(), "FIXT 1.1 EP312");
    assert_eq!(version("FIX.5.0SP2", "FIX.5.0SP2_EP254").to_string(), "FIX 5.0 SP2 EP254");
}

#[test]
fn version_errors_name_the_line() {
    for (name, version, message) in [
        ("FOX.4.2", "FOX.4.2", "name 'FOX.4.2' is not a FIX version"),
        ("FIX.4", "FIX.4", "name 'FIX.4' is not a FIX version"),
        ("FIX.4.2", "FIX.4.2_EPx", "version 'FIX.4.2_EPx' has no extension pack number"),
    ] {
        let err = Dictionary::from_orchestra(&repository(name, version, "")).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(1), message));
    }
    let err =
        Dictionary::from_orchestra("<fixr:repository xmlns:fixr='http://fixprotocol.io/2020/orchestra/repository'/>")
            .unwrap_err();
    assert_eq!(err.message, "<repository> needs a name attribute");
}

#[test]
fn maps_datatypes_to_field_types() {
    let types = [
        ("int", FieldType::Int),
        ("Length", FieldType::Length),
        ("TagNum", FieldType::TagNum),
        ("SeqNum", FieldType::SeqNum),
        ("NumInGroup", FieldType::NumInGroup),
        ("DayOfMonth", FieldType::DayOfMonth),
        ("float", FieldType::Float),
        ("Qty", FieldType::Qty),
        ("Price", FieldType::Price),
        ("PriceOffset", FieldType::PriceOffset),
        ("Amt", FieldType::Amt),
        ("Percentage", FieldType::Percentage),
        ("char", FieldType::Char),
        ("Boolean", FieldType::Boolean),
        ("String", FieldType::String),
        ("MultipleCharValue", FieldType::MultipleCharValue),
        ("MultipleStringValue", FieldType::MultipleStringValue),
        ("MultipleValueString", FieldType::MultipleValueString),
        ("Country", FieldType::Country),
        ("Currency", FieldType::Currency),
        ("Exchange", FieldType::Exchange),
        ("Language", FieldType::Language),
        ("MonthYear", FieldType::MonthYear),
        ("UTCTimestamp", FieldType::UtcTimestamp),
        ("UTCTimeOnly", FieldType::UtcTimeOnly),
        ("UTCDateOnly", FieldType::UtcDateOnly),
        ("UTCDate", FieldType::UtcDate),
        ("LocalMktDate", FieldType::LocalMktDate),
        ("LocalMktTime", FieldType::LocalMktTime),
        ("TZTimeOnly", FieldType::TzTimeOnly),
        ("TZTimestamp", FieldType::TzTimestamp),
        ("data", FieldType::Data),
        ("XMLData", FieldType::XmlData),
        ("XID", FieldType::Xid),
        ("XIDREF", FieldType::XidRef),
    ];
    let fields: String = types
        .iter()
        .enumerate()
        .map(|(i, (name, _))| format!("<fixr:field id='{}' name='F{i}' type='{name}'/>", i + 1))
        .collect();
    let dict = load(&format!("<fixr:fields>{fields}</fixr:fields>"));
    for (i, (name, ty)) in types.iter().enumerate() {
        assert_eq!(&dict.field_by_tag(i as u32 + 1).unwrap().ty, ty, "{name}");
    }
}

#[test]
fn follows_base_types_to_a_known_one() {
    let dict = load(
        "<fixr:datatypes>\
         <fixr:datatype name='Deep' baseType='Middle'/>\
         <fixr:datatype name='Middle' baseType='Price'/>\
         <fixr:datatype name='Price' baseType='float'/>\
         <fixr:datatype name='Odd' baseType='Odder'/>\
         <fixr:datatype name='Odder'/>\
         <fixr:datatype name='Loop' baseType='Loop'/>\
         </fixr:datatypes>\n\
         <fixr:fields><fixr:field id='1' name='A' type='Deep'/><fixr:field id='2' name='B' type='Odd'/>\
         <fixr:field id='3' name='C' type='Nope'/><fixr:field id='4' name='D' type='Loop'/></fixr:fields>",
    );
    let ty = |name| dict.field(name).unwrap().ty.clone();
    assert_eq!(ty("A"), FieldType::Price);
    assert_eq!(ty("B"), FieldType::Other("Odd".into()));
    assert_eq!(ty("C"), FieldType::Other("Nope".into()));
    assert_eq!(ty("D"), FieldType::Other("Loop".into()));
}

#[test]
fn code_set_fields_are_enumerated() {
    let dict = load(&format!(
        "<fixr:datatypes><fixr:datatype name='Flag' baseType='char'/></fixr:datatypes>\n\
         <fixr:codeSets>\
         <fixr:codeSet id='434' name='CxlRejResponseToCodeSet' type='Flag'>{}\
         <fixr:code id='434001' name='OrderCancelRequest' value='1'>{}</fixr:code>\
         <fixr:code id='434002' name='OrderCancel' value='2' sort='2'/>\
         </fixr:codeSet></fixr:codeSets>\n\
         <fixr:fields><fixr:field id='434' name='CxlRejResponseTo' type='CxlRejResponseToCodeSet'/></fixr:fields>",
        synopsis("Code set doc"),
        synopsis("\n         Order Cancel Request\n      ")
    ));
    let field = dict.field("CxlRejResponseTo").unwrap();
    assert_eq!(field.ty, FieldType::Char);
    assert_eq!(
        field.values.iter().map(parts).collect::<Vec<_>>(),
        [("1", Some("OrderCancelRequest"), "", Some("Order Cancel Request")), ("2", Some("OrderCancel"), "", None)]
    );
}

#[test]
fn fields_share_a_code_set() {
    let dict = load(
        "<fixr:codeSets><fixr:codeSet id='1' name='FlagCodeSet' type='char'>\
         <fixr:code id='11' name='Yes' value='Y'/><fixr:code id='12' name='No' value='N'/></fixr:codeSet></fixr:codeSets>\n\
         <fixr:fields><fixr:field id='1' name='A' type='FlagCodeSet'/><fixr:field id='2' name='B' type='FlagCodeSet'/>\
         </fixr:fields>",
    );
    let (a, b) = (dict.field("A").unwrap(), dict.field("B").unwrap());
    let names: Vec<_> = a.values.iter().map(|v| (v.code.as_str(), v.name.as_deref())).collect();
    assert_eq!(names, [("Y", Some("Yes")), ("N", Some("No"))]);
    assert_eq!((&a.ty, &a.values), (&b.ty, &b.values));
}

#[test]
fn reads_field_docs_and_deprecation() {
    let doc = "<fixr:annotation>\
         <fixr:documentation purpose='ELABORATION'>Not this.</fixr:documentation>\
         <fixr:documentation purpose='SYNOPSIS'>\n   Side  of\t order &amp; <b>more</b>.  \n\n  \n Next\tline.\n</fixr:documentation>\
         <fixr:documentation>Nor this.</fixr:documentation>\
         <fixr:documentation purpose='SYNOPSIS'>Another one.</fixr:documentation>\
         </fixr:annotation>";
    let dict = load(&format!(
        "<fixr:fields>\
         <fixr:field id='54' name='Side' type='char'>{doc}</fixr:field>\
         <fixr:field id='20' name='ExecTransType' type='char' deprecated='FIX.4.3'/>\
         </fixr:fields>"
    ));
    let side = dict.field("Side").unwrap();
    assert_eq!(side.doc.as_deref(), Some("Side of order & more.\nNext line.\nAnother one."));
    assert!(!side.deprecated);
    let old = dict.field("ExecTransType").unwrap();
    assert_eq!(old.doc, None);
    assert!(old.deprecated);
}

#[test]
fn keeps_line_breaks_in_docs() {
    // As in OrchestraFIX42.xml's MaturityMonthYear: each line is a hard break.
    let doc = synopsis(
        "Month and Year of the maturity for SecurityType=FUT or SecurityType=OPT. Required if MaturityDay is specified.\n\
         Format: YYYYMM\n\
         (i.e. 199903)",
    );
    let dict = load(&format!(
        "<fixr:fields><fixr:field id='200' name='MaturityMonthYear' type='MonthYear'>{doc}</fixr:field></fixr:fields>"
    ));
    assert_eq!(
        dict.field("MaturityMonthYear").unwrap().doc.as_deref(),
        Some(
            "Month and Year of the maturity for SecurityType=FUT or SecurityType=OPT. Required if MaturityDay is \
             specified.\nFormat: YYYYMM\n(i.e. 199903)"
        )
    );
}

#[test]
fn field_errors_name_the_line() {
    let cases = [
        (
            "<fixr:fields>\n<fixr:field id='1' name='A' type='int'/>\n<fixr:field id='2' name='A' type='int'/>\n</fixr:fields>",
            (Some(4), "field A is defined twice"),
        ),
        (
            "<fixr:fields>\n<fixr:field id='1' name='A' type='int'/>\n<fixr:field id='1' name='B' type='int'/>\n</fixr:fields>",
            (Some(4), "tag 1 is both A and B"),
        ),
        ("<fixr:fields>\n<fixr:field id='x' name='A' type='int'/>\n</fixr:fields>", (Some(3), "field A has id 'x'")),
        ("<fixr:fields>\n<fixr:field id='1' name='A'/>\n</fixr:fields>", (Some(3), "<field> needs a type attribute")),
        (
            "<fixr:codeSets>\n<fixr:codeSet id='1' name='S' type='char'>\n<fixr:code id='11' name='X' value='1'/>\n\
             <fixr:code id='12' name='Y' value='1'/>\n</fixr:codeSet>\n</fixr:codeSets>",
            (Some(5), "code set S has value 1 twice"),
        ),
        (
            "<fixr:codeSets>\n<fixr:codeSet id='1' name='S' type='char'/>\n<fixr:codeSet id='2' name='S' type='int'/>\n\
             </fixr:codeSets>",
            (Some(4), "code set S is defined twice"),
        ),
    ];
    for (sections, expected) in cases {
        let err = error(sections);
        assert_eq!((err.line, err.message.as_str()), expected, "{sections}");
    }
}

#[test]
fn detects_the_format() {
    let quickfix = "<fix type='FIX' major='4' minor='4'><fields><field number='1' name='A' type='INT'/></fields></fix>";
    let orchestra = repo("<fixr:fields><fixr:field id='1' name='A' type='int'/></fixr:fields>");
    for xml in [quickfix, orchestra.as_str()] {
        let dict = Dictionary::from_xml(xml).unwrap();
        assert_eq!(dict.field("A").unwrap().ty, FieldType::Int);
    }
    assert_eq!(Dictionary::from_xml(&orchestra).unwrap().version.to_string(), "FIX 4.2 EP310");
    assert_eq!(Dictionary::from_quickfix(quickfix).unwrap().version.to_string(), "FIX 4.4");

    for xml in ["<fox/>", "<repository name='FIX.4.2'/>"] {
        let err = Dictionary::from_xml(xml).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(1), "not a QuickFIX or Orchestra dictionary"), "{xml}");
    }
    let old = "<fixr:repository xmlns:fixr='http://fixprotocol.io/2016/fixrepository' name='FIX.4.2'/>";
    for err in [Dictionary::from_xml(old).unwrap_err(), Dictionary::from_orchestra(old).unwrap_err()] {
        assert_eq!(
            (err.line, err.message.as_str()),
            (Some(1), "unsupported Orchestra namespace 'http://fixprotocol.io/2016/fixrepository'")
        );
    }
    assert!(Dictionary::from_quickfix(&orchestra).unwrap_err().message.contains("<fix>"));
    assert!(Dictionary::from_orchestra(quickfix).unwrap_err().message.contains("<fixr:repository>"));
}

#[test]
fn load_detects_the_format() {
    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("orchestra-detect.xml");
    std::fs::write(&path, repo("<fixr:fields><fixr:field id='1' name='A' type='int'/></fixr:fields>")).unwrap();
    let dict = Dictionary::load(&path).unwrap();
    assert_eq!(dict.version.to_string(), "FIX 4.2 EP310");
}

#[test]
fn orchestra_overlays_are_rejected() {
    let mut dict = load("<fixr:fields><fixr:field id='1' name='A' type='int'/></fixr:fields>");
    let err = dict.merge_xml(&repo("<fixr:fields><fixr:field id='2' name='B' type='int'/></fixr:fields>")).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(1), "Orchestra overlays aren't supported yet"));
    assert!(dict.field("B").is_none());
}

#[test]
fn quickfix_overlays_merge_onto_an_orchestra_base() {
    let mut dict = load("<fixr:fields><fixr:field id='1' name='A' type='int'/></fixr:fields>");
    dict.merge_xml("<fix><fields><field number='5001' name='VenueFlag' type='BOOLEAN'/></fields></fix>").unwrap();
    assert_eq!(dict.field_by_tag(5001).unwrap().ty, FieldType::Boolean);
    assert_eq!(dict.version.to_string(), "FIX 4.2 EP310");
}

const FIELDS: &str = "<fixr:fields>\
     <fixr:field id='8' name='BeginString' type='String'/><fixr:field id='35' name='MsgType' type='String'/>\
     <fixr:field id='10' name='CheckSum' type='String'/><fixr:field id='11' name='ClOrdID' type='String'/>\
     <fixr:field id='1' name='Account' type='String'/><fixr:field id='55' name='Symbol' type='String'/>\
     <fixr:field id='44' name='Price' type='Price'/><fixr:field id='78' name='NoAllocs' type='int'/>\
     <fixr:field id='79' name='AllocAccount' type='String'/><fixr:field id='80' name='AllocShares' type='Qty'/>\
     <fixr:field id='136' name='NoMiscFees' type='NumInGroup'/><fixr:field id='137' name='MiscFeeAmt' type='Amt'/>\
     <fixr:field id='146' name='NoRelatedSym' type='NumInGroup'/>\
     </fixr:fields>";

/// The header and trailer, with ids other than FIX 4.2's, and Instrument.
const COMPONENTS: &str = "<fixr:components>\
     <fixr:component id='1001' name='Instrument'><fixr:fieldRef id='55' presence='required'/></fixr:component>\
     <fixr:component id='1024' name='StandardHeader'>\
     <fixr:fieldRef id='8' presence='required'/><fixr:fieldRef id='35' presence='required'/></fixr:component>\
     <fixr:component id='1025' name='StandardTrailer'><fixr:fieldRef id='10' presence='required'/></fixr:component>\
     </fixr:components>";

const GROUPS: &str = "<fixr:groups>\
     <fixr:group id='2001' name='PreAllocGrp'><fixr:numInGroup id='78'/>\
     <fixr:fieldRef id='79'/><fixr:fieldRef id='80' presence='required'/></fixr:group>\
     <fixr:group id='2002' name='MiscFeesGrp'><fixr:numInGroup id='136'/>\
     <fixr:fieldRef id='137'/><fixr:groupRef id='2001'/></fixr:group>\
     <fixr:group id='2003' name='RelatedSymGrp'><fixr:numInGroup id='146'/>\
     <fixr:annotation><fixr:documentation purpose='SYNOPSIS'>Symbols.</fixr:documentation></fixr:annotation>\
     <fixr:componentRef id='1001' presence='required'/></fixr:group>\
     </fixr:groups>";

/// `first` starts on line 2, followed by the common fields, components and groups.
fn with_common(first: &str) -> String {
    repo(&format!("{first}\n{FIELDS}\n{COMPONENTS}\n{GROUPS}"))
}

/// A message M whose structure's refs, `refs`, are on line 3.
fn message(refs: &str) -> String {
    with_common(&format!(
        "<fixr:messages><fixr:message id='1' name='M' msgType='U1'><fixr:structure>\n{refs}\n\
         </fixr:structure></fixr:message></fixr:messages>"
    ))
}

fn members(refs: &str) -> Vec<Member> {
    let dict = Dictionary::from_orchestra(&message(refs)).unwrap_or_else(|e| panic!("{e}"));
    dict.message("M").unwrap().members.clone()
}

fn field(name: &str, required: bool) -> Member {
    Member::Field { name: name.into(), required }
}

fn group(count: &str, name: &str, required: bool, members: Vec<Member>) -> Member {
    Member::Group { name: count.into(), official_name: Some(name.into()), required, members }
}

fn pre_alloc_grp(required: bool) -> Member {
    group("NoAllocs", "PreAllocGrp", required, vec![field("AllocAccount", false), field("AllocShares", true)])
}

#[test]
fn reads_messages_header_and_trailer() {
    let dict = Dictionary::from_orchestra(&with_common(&format!(
        "<fixr:messages><fixr:message id='14' name='NewOrderSingle' msgType='D'><fixr:structure>\
         <fixr:componentRef id='1024' presence='required'/>\
         <fixr:fieldRef id='11' presence='required'><fixr:annotation><fixr:documentation>Ref doc</fixr:documentation>\
         </fixr:annotation></fixr:fieldRef>\
         <fixr:componentRef id='1001'/><fixr:groupRef id='2001' presence='required'/><fixr:fieldRef id='1'/>\
         <fixr:componentRef id='1025' presence='required'/>\
         </fixr:structure>{}</fixr:message></fixr:messages>",
        synopsis("An order.")
    )))
    .unwrap();
    assert_eq!(dict.header(), [field("BeginString", true), field("MsgType", true)]);
    assert_eq!(dict.trailer(), [field("CheckSum", true)]);
    let components: Vec<_> = dict.components().iter().map(|c| c.name.as_str()).collect();
    assert_eq!(components, ["Instrument"]);
    assert_eq!(dict.component("Instrument").unwrap().members, [field("Symbol", true)]);

    let order = dict.message("NewOrderSingle").unwrap();
    assert_eq!((order.msg_type.as_str(), order.category), ("D", Category::App));
    assert_eq!(order.doc.as_deref(), Some("An order."));
    assert_eq!(
        order.members,
        [
            field("ClOrdID", true),
            Member::Component { name: "Instrument".into(), required: false },
            pre_alloc_grp(true),
            field("Account", false),
        ]
    );
}

#[test]
fn expands_nested_groups() {
    assert_eq!(
        members("<fixr:groupRef id='2002'/><fixr:groupRef id='2003'/>"),
        [
            group("NoMiscFees", "MiscFeesGrp", false, vec![field("MiscFeeAmt", false), pre_alloc_grp(false)]),
            group(
                "NoRelatedSym",
                "RelatedSymGrp",
                false,
                vec![Member::Component { name: "Instrument".into(), required: true }]
            ),
        ]
    );
    let dict = Dictionary::from_orchestra(&with_common(
        "<fixr:components><fixr:component id='3001' name='Allocs'><fixr:groupRef id='2001'/></fixr:component>\
         </fixr:components>",
    ))
    .unwrap();
    assert_eq!(dict.component("Allocs").unwrap().members, [pre_alloc_grp(false)]);
}

#[test]
fn expands_groups_in_the_header() {
    let components = "<fixr:components>\
         <fixr:component id='1001' name='Instrument'><fixr:fieldRef id='55'/></fixr:component>\
         <fixr:component id='1024' name='StandardHeader'><fixr:fieldRef id='8' presence='required'/>\
         <fixr:groupRef id='2001'/></fixr:component>\
         <fixr:component id='1025' name='StandardTrailer'><fixr:fieldRef id='10' presence='required'/></fixr:component>\
         </fixr:components>";
    let dict = Dictionary::from_orchestra(&repo(&format!("{FIELDS}\n{components}\n{GROUPS}"))).unwrap();
    assert_eq!(dict.header(), [field("BeginString", true), pre_alloc_grp(false)]);
    assert!(dict.component("StandardHeader").is_none());
}

#[test]
fn reads_presence() {
    assert_eq!(
        members(
            "<fixr:fieldRef id='11'/><fixr:fieldRef id='1' presence='required'/><fixr:fieldRef id='44' presence='optional'/>"
        ),
        [field("ClOrdID", false), field("Account", true), field("Price", false)]
    );
}

/// Messages with these msgtypes and categories: which ones are admin.
fn admin(messages: &[(&str, Option<&str>)]) -> Vec<String> {
    let messages: String = messages
        .iter()
        .enumerate()
        .map(|(i, (msg_type, category))| {
            let category = category.map(|c| format!(" category='{c}'")).unwrap_or_default();
            format!(
                "<fixr:message id='{i}' name='M{i}' msgType='{msg_type}'{category}><fixr:structure/></fixr:message>"
            )
        })
        .collect();
    let dict = Dictionary::from_orchestra(&repo(&format!("<fixr:messages>{messages}</fixr:messages>"))).unwrap();
    dict.messages().iter().filter(|m| m.category == Category::Admin).map(|m| m.msg_type.clone()).collect()
}

#[test]
fn admin_messages_by_category() {
    let messages = [
        ("0", Some("Session")),
        ("A", Some("Session")),
        ("n", Some("Session")),
        ("1", Some("Common")),
        ("D", Some("SingleGeneralOrderHandling")),
        ("5", None),
    ];
    assert_eq!(admin(&messages), ["0", "A", "n"]);
}

#[test]
fn admin_messages_by_msgtype_without_categories() {
    let types = ["0", "1", "2", "3", "4", "5", "A", "D", "n", "8", "AA"];
    let messages: Vec<_> = types.iter().map(|t| (*t, None)).collect();
    assert_eq!(admin(&messages), ["0", "1", "2", "3", "4", "5", "A"]);
}

#[test]
fn reference_errors_name_the_line() {
    let cases = [
        ("<fixr:fieldRef id='99'/>", "unknown field id 99"),
        ("<fixr:componentRef id='99'/>", "unknown component id 99"),
        ("<fixr:groupRef id='99'/>", "unknown group id 99"),
        ("<fixr:fieldRef id='11' presence='constant'/>", "unsupported presence 'constant'"),
        ("<fixr:groupRef id='2001' presence='forbidden'/>", "unsupported presence 'forbidden'"),
        ("<fixr:thing/>", "unexpected <thing>"),
        ("<fixr:fieldRef id='11' scenario='Short'/>", "scenarios aren't supported yet"),
        ("<fixr:groupRef id='2001' scenario='Short'/>", "scenarios aren't supported yet"),
        ("<fixr:componentRef id='1001' scenario='Short'/>", "scenarios aren't supported yet"),
    ];
    for (refs, expected) in cases {
        let err = Dictionary::from_orchestra(&message(refs)).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(3), expected), "{refs}");
    }
}

#[test]
fn definition_errors_name_the_line() {
    let group =
        |body: &str| format!("<fixr:groups>\n<fixr:group id='3001' name='Bad'>{body}</fixr:group>\n</fixr:groups>");
    let cases = [
        (group("<fixr:numInGroup id='44'/><fixr:fieldRef id='1'/>"), "group Bad's count field Price must be NumInGroup or int"),
        (group("<fixr:numInGroup id='99'/><fixr:fieldRef id='1'/>"), "unknown field id 99"),
        (group("<fixr:numInGroup id='146'/>"), "group Bad has no members"),
        (group("<fixr:fieldRef id='1'/>"), "group Bad needs a <numInGroup>"),
        (group("<fixr:numInGroup id='146'/><fixr:groupRef id='3001'/>"), "group Bad includes itself (Bad -> Bad)"),
        (
            group("<fixr:numInGroup id='146'/><fixr:numInGroup id='136'/><fixr:fieldRef id='1'/>"),
            "group Bad has more than one <numInGroup>",
        ),
        (
            "<fixr:groups>\n<fixr:group id='3001' name='A'><fixr:numInGroup id='146'/><fixr:groupRef id='3002'/></fixr:group>\n\
             <fixr:group id='3002' name='B'><fixr:numInGroup id='136'/><fixr:groupRef id='3001'/></fixr:group>\n</fixr:groups>"
                .to_string(),
            "group A includes itself (A -> B -> A)",
        ),
        (
            "<fixr:messages>\n<fixr:message id='1' name='M' msgType='U1'/>\n</fixr:messages>".to_string(),
            "message M needs a <structure>",
        ),
        (
            "<fixr:components>\n<fixr:component id='3001' name='C'><fixr:componentRef id='1024'/></fixr:component>\n\
             </fixr:components>"
                .to_string(),
            "StandardHeader can only be used in a message",
        ),
        (
            "<fixr:messages>\n<fixr:message id='1' name='M' msgType='U1' scenario='Short'><fixr:structure/></fixr:message>\n\
             </fixr:messages>"
                .to_string(),
            "scenarios aren't supported yet",
        ),
        (
            "<fixr:fields>\n<fixr:field id='5001' name='V' type='int' scenario='Short'/>\n</fixr:fields>".to_string(),
            "scenarios aren't supported yet",
        ),
    ];
    for (first, expected) in cases {
        let err = Dictionary::from_orchestra(&with_common(&first)).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(3), expected), "{first}");
    }
    let base = "<fixr:messages><fixr:message id='1' name='M' msgType='U1' scenario='base'><fixr:structure>\
         <fixr:fieldRef id='11' scenario='base'/><fixr:groupRef id='2001' scenario='base'/>\
         <fixr:componentRef id='1001' scenario='base'/></fixr:structure></fixr:message></fixr:messages>";
    let dict = Dictionary::from_orchestra(&with_common(base)).unwrap();
    let names: Vec<_> = dict.message("M").unwrap().members.iter().map(Member::name).collect();
    assert_eq!(names, ["ClOrdID", "NoAllocs", "Instrument"]);
}

#[test]
fn duplicate_definitions_name_the_line() {
    let component =
        |id, name| format!("<fixr:component id='{id}' name='{name}'><fixr:fieldRef id='1'/></fixr:component>");
    let group = |id, name| {
        format!("<fixr:group id='{id}' name='{name}'><fixr:numInGroup id='146'/><fixr:fieldRef id='1'/></fixr:group>")
    };
    let message = |name, msg_type| {
        format!("<fixr:message id='1' name='{name}' msgType='{msg_type}'><fixr:structure/></fixr:message>")
    };
    let cases = [
        ("components", component(3001, "C"), component(3002, "C"), "component C is defined twice"),
        ("components", component(3001, "C"), component(3001, "D"), "component id 3001 is both C and D"),
        ("groups", group(3001, "G"), group(3002, "G"), "group G is defined twice"),
        ("groups", group(3001, "G"), group(3001, "H"), "group id 3001 is both G and H"),
        ("messages", message("M", "U1"), message("M", "U2"), "message M is defined twice"),
        ("messages", message("M", "U1"), message("N", "U1"), "msgtype U1 is both M and N"),
    ];
    for (section, first, second, expected) in cases {
        let xml = with_common(&format!("<fixr:{section}>\n{first}\n{second}\n</fixr:{section}>"));
        let err = Dictionary::from_orchestra(&xml).unwrap_err();
        assert_eq!((err.line, err.message.as_str()), (Some(4), expected), "{section}");
    }
}

#[test]
fn component_cycles_name_the_line() {
    let pair = "<fixr:components>\n\
         <fixr:component id='3001' name='X'><fixr:componentRef id='3002'/></fixr:component>\n\
         <fixr:component id='3002' name='Y'><fixr:componentRef id='3001'/></fixr:component>\n</fixr:components>";
    let err = Dictionary::from_orchestra(&with_common(pair)).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(3), "component X includes itself (X -> Y -> X)"));

    let via_group = "<fixr:components>\n\
         <fixr:component id='3001' name='X'><fixr:fieldRef id='1'/></fixr:component>\n\
         <fixr:component id='3002' name='Y'><fixr:groupRef id='3003'/></fixr:component>\n</fixr:components>\n\
         <fixr:groups><fixr:group id='3003' name='G'><fixr:numInGroup id='146'/><fixr:componentRef id='3002'/>\
         </fixr:group></fixr:groups>";
    let err = Dictionary::from_orchestra(&with_common(via_group)).unwrap_err();
    assert_eq!((err.line, err.message.as_str()), (Some(4), "component Y includes itself (Y -> Y)"));
}

/// A base whose NewOrderSingle uses PreAllocGrp and MiscFeesGrp (which nests PreAllocGrp), plus
/// `more` (components, messages or groups) on the line after.
fn overlay_base(more: &str) -> Dictionary {
    let order = "<fixr:messages><fixr:message id='14' name='NewOrderSingle' msgType='D'><fixr:structure>\
         <fixr:fieldRef id='11' presence='required'/><fixr:groupRef id='2001'/><fixr:groupRef id='2002'/>\
         </fixr:structure></fixr:message></fixr:messages>";
    Dictionary::from_orchestra(&with_common(&format!("{order}\n{more}"))).unwrap_or_else(|e| panic!("{e}"))
}

/// A QuickFIX NoAllocs group with AllocAccount and a required AllocShares, as PreAllocGrp has
/// them, and `extra` members.
fn quickfix_allocs(extra: &str) -> String {
    format!(
        "<group name='NoAllocs' required='N'><field name='AllocAccount' required='N'/>\
         <field name='AllocShares' required='Y'/>{extra}</group>"
    )
}

/// A venue re-listing NewOrderSingle with `members` after ClOrdID.
fn relisted(members: &str) -> String {
    format!(
        "<fix><messages><message name='NewOrderSingle' msgtype='D' msgcat='app'>\
         <field name='ClOrdID' required='Y'/>{members}</message></messages></fix>"
    )
}

fn unnamed(count: &str, required: bool, members: Vec<Member>) -> Member {
    Member::Group { name: count.into(), official_name: None, required, members }
}

#[test]
fn relisted_messages_keep_official_group_names() {
    let mut dict = overlay_base("");
    let misc_fees = format!(
        "<group name='NoMiscFees' required='N'><field name='MiscFeeAmt' required='N'/>{}</group>",
        quickfix_allocs("")
    );
    dict.merge_xml(&relisted(&format!("{}{misc_fees}", quickfix_allocs("")))).unwrap();
    assert_eq!(
        dict.message("NewOrderSingle").unwrap().members,
        [
            field("ClOrdID", true),
            pre_alloc_grp(false),
            group("NoMiscFees", "MiscFeesGrp", false, vec![field("MiscFeeAmt", false), pre_alloc_grp(false)]),
        ]
    );
}

#[test]
fn venue_components_and_header_groups_take_official_names() {
    let mut dict = overlay_base("");
    let allocs = quickfix_allocs("");
    dict.merge_xml(&format!(
        "<fix><header><field name='BeginString' required='Y'/>{allocs}</header>\
         <components><component name='VenueAllocs'>{allocs}</component></components></fix>"
    ))
    .unwrap();
    assert_eq!(dict.component("VenueAllocs").unwrap().members, [pre_alloc_grp(false)]);
    assert_eq!(dict.header(), [field("BeginString", true), pre_alloc_grp(false)]);
}

#[test]
fn groups_nested_in_a_venue_group_take_official_names() {
    let mut dict = overlay_base("");
    // MiscFees with an extra field is the venue's own; the PreAllocGrp inside it is still official.
    dict.merge_xml(&relisted(&format!(
        "<group name='NoMiscFees' required='N'><field name='MiscFeeAmt' required='N'/>\
         <field name='Account' required='N'/>{}</group>",
        quickfix_allocs("")
    )))
    .unwrap();
    assert_eq!(
        dict.message("NewOrderSingle").unwrap().members,
        [
            field("ClOrdID", true),
            unnamed(
                "NoMiscFees",
                false,
                vec![field("MiscFeeAmt", false), field("Account", false), pre_alloc_grp(false)]
            ),
        ]
    );
}

#[test]
fn venue_groups_that_differ_stay_unnamed() {
    let allocs = |required| vec![field("AllocAccount", false), field("AllocShares", required)];
    // An extra member, and a member whose requiredness differs.
    let cases = [
        (
            quickfix_allocs("<field name='Account' required='N'/>"),
            unnamed("NoAllocs", false, [allocs(true), vec![field("Account", false)]].concat()),
        ),
        (
            "<group name='NoAllocs' required='N'><field name='AllocAccount' required='N'/>\
             <field name='AllocShares' required='N'/></group>"
                .to_string(),
            unnamed("NoAllocs", false, allocs(false)),
        ),
    ];
    for (venue, expected) in cases {
        let mut dict = overlay_base("");
        dict.merge_xml(&relisted(&venue)).unwrap();
        assert_eq!(dict.message("NewOrderSingle").unwrap().members, [field("ClOrdID", true), expected], "{venue}");
    }
}

#[test]
fn venue_groups_matching_two_official_groups_stay_unnamed() {
    // AllocGrp is PreAllocGrp by another name.
    let mut dict = overlay_base(
        "<fixr:groups><fixr:group id='2009' name='AllocGrp'><fixr:numInGroup id='78'/>\
         <fixr:fieldRef id='79'/><fixr:fieldRef id='80' presence='required'/></fixr:group></fixr:groups>\
         <fixr:messages><fixr:message id='15' name='Allocation' msgType='J'><fixr:structure>\
         <fixr:groupRef id='2009'/></fixr:structure></fixr:message></fixr:messages>",
    );
    dict.merge_xml(&relisted(&quickfix_allocs(""))).unwrap();
    assert_eq!(
        dict.message("NewOrderSingle").unwrap().members,
        [
            field("ClOrdID", true),
            unnamed("NoAllocs", false, vec![field("AllocAccount", false), field("AllocShares", true)])
        ]
    );
}

#[test]
fn relisted_messages_keep_official_docs() {
    let mut dict = Dictionary::from_orchestra(&with_common(&format!(
        "<fixr:messages><fixr:message id='14' name='NewOrderSingle' msgType='D'><fixr:structure>\
         <fixr:fieldRef id='11' presence='required'/></fixr:structure>{}</fixr:message></fixr:messages>",
        synopsis("An order.")
    )))
    .unwrap_or_else(|e| panic!("{e}"));
    dict.merge_xml(&relisted("<field name='Account' required='N'/>")).unwrap();
    let order = dict.message("NewOrderSingle").unwrap();
    assert_eq!(order.members, [field("ClOrdID", true), field("Account", false)]);
    assert_eq!(order.doc.as_deref(), Some("An order."));
}
