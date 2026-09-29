use super::*;
use crate::fields::MsgType;
use crate::message::tags;

const DICT: &str = "<fix type='FIX' major='4' minor='4'>
 <header>
  <field name='BeginString' required='Y'/><field name='MsgType' required='Y'/>
  <field name='SenderCompID' required='Y'/><field name='MsgSeqNum' required='Y'/>
 </header>
 <trailer><field name='CheckSum' required='Y'/></trailer>
 <messages>
  <message name='NewOrderSingle' msgtype='D' msgcat='app'>
   <field name='ClOrdID' required='Y'/>
   <field name='Side' required='Y'/>
   <field name='OrderQty' required='N'/>
   <field name='ExecInst' required='N'/>
   <field name='TransactTime' required='N'/>
   <component name='Instrument' required='Y'/>
   <group name='NoAllocs' required='N'>
    <field name='AllocAccount' required='Y'/>
    <field name='AllocQty' required='Y'/>
    <group name='NoNotes' required='N'><field name='Note' required='Y'/></group>
   </group>
  </message>
  <message name='Heartbeat' msgtype='0' msgcat='admin'/>
 </messages>
 <components>
  <component name='Instrument'><field name='Symbol' required='Y'/></component>
 </components>
 <fields>
  <field number='8' name='BeginString' type='STRING'/>
  <field number='35' name='MsgType' type='STRING'/>
  <field number='49' name='SenderCompID' type='STRING'/>
  <field number='34' name='MsgSeqNum' type='SEQNUM'/>
  <field number='10' name='CheckSum' type='STRING'/>
  <field number='11' name='ClOrdID' type='STRING'/>
  <field number='54' name='Side' type='CHAR'><value enum='1' description='BUY'/><value enum='2' description='SELL'/></field>
  <field number='38' name='OrderQty' type='QTY'/>
  <field number='18' name='ExecInst' type='MULTIPLEVALUESTRING'>
   <value enum='1' description='NOT_HELD'/><value enum='G' description='ALL_OR_NONE'/>
  </field>
  <field number='60' name='TransactTime' type='UTCTIMESTAMP'/>
  <field number='55' name='Symbol' type='STRING'/>
  <field number='78' name='NoAllocs' type='NUMINGROUP'/>
  <field number='79' name='AllocAccount' type='STRING'/>
  <field number='80' name='AllocQty' type='QTY'/>
  <field number='5001' name='NoNotes' type='NUMINGROUP'/>
  <field number='5002' name='Note' type='STRING'/>
  <field number='58' name='Text' type='STRING'/>
 </fields>
</fix>";

fn validator() -> Validator {
    Validator::new(&turbojet_dictionary::Dictionary::from_xml(DICT).unwrap())
}

/// A message from `text` (`tag=value|...`), with a standard header.
fn msg(body: &str) -> Message {
    let text = format!("8=FIX.4.4|35=D|49=CLIENT|34=2|{body}10=000|");
    Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
        let (tag, value) = f.split_once('=').unwrap();
        (tag.parse::<u32>().unwrap(), value)
    }))
}

/// The tag and reason of a validation failure.
fn failure(v: &Validator, body: &str) -> (Option<u32>, Option<SessionRejectReason>) {
    let err = v.validate(&msg(body)).unwrap_err();
    (err.tag, err.reason)
}

const GOOD: &str = "11=A|54=1|55=AAPL|";

#[test]
fn accepts_a_valid_message() {
    let v = validator();
    v.validate(&msg(GOOD)).unwrap();
    v.validate(&msg(
        "11=A|54=2|38=100|18=1 G|60=20260928-12:00:00.000|55=AAPL|78=2|79=X|80=5|79=Y|80=6|5001=1|5002=n|",
    ))
    .unwrap();
}

#[test]
fn unknown_msg_type() {
    let v = validator();
    let err = v.validate(&msg(GOOD).with(tags::MSG_TYPE, "ZZ")).unwrap_err();
    assert_eq!((err.tag, err.reason), (Some(35), Some(SessionRejectReason::InvalidMsgType)));
}

#[test]
fn undefined_and_foreign_tags() {
    let v = validator();
    assert_eq!(failure(&v, "11=A|54=1|55=AAPL|9999=x|"), (Some(9999), Some(SessionRejectReason::InvalidTagNumber)));
    // Text is in the dictionary, but not in NewOrderSingle.
    assert_eq!(
        failure(&v, "11=A|54=1|55=AAPL|58=hi|"),
        (Some(58), Some(SessionRejectReason::TagNotDefinedForMessageType))
    );
}

#[test]
fn required_tags_including_components_and_group_entries() {
    let v = validator();
    assert_eq!(failure(&v, "54=1|55=AAPL|"), (Some(11), Some(SessionRejectReason::RequiredTagMissing)));
    // Symbol is required through the required Instrument component.
    assert_eq!(failure(&v, "11=A|54=1|"), (Some(55), Some(SessionRejectReason::RequiredTagMissing)));
    // Inside a group entry.
    assert_eq!(failure(&v, "11=A|54=1|55=AAPL|78=1|79=X|"), (Some(80), Some(SessionRejectReason::RequiredTagMissing)));
}

#[test]
fn values_and_formats() {
    let v = validator();
    assert_eq!(failure(&v, "11=A|54=9|55=AAPL|"), (Some(54), Some(SessionRejectReason::ValueIsIncorrect)));
    assert_eq!(failure(&v, "11=A|54=1|18=1 Z|55=AAPL|"), (Some(18), Some(SessionRejectReason::ValueIsIncorrect)));
    for bad in ["38=ten|", "38=1e3|", "60=yesterday|"] {
        let body = format!("11=A|54=1|{bad}55=AAPL|");
        assert_eq!(failure(&v, &body).1, Some(SessionRejectReason::IncorrectDataFormat), "{bad}");
    }
}

#[test]
fn repeated_tags_and_groups() {
    let v = validator();
    assert_eq!(failure(&v, "11=A|11=B|54=1|55=AAPL|"), (Some(11), Some(SessionRejectReason::TagAppearsMoreThanOnce)));
    assert_eq!(
        failure(&v, "11=A|54=1|55=AAPL|78=2|79=X|80=5|"),
        (Some(78), Some(SessionRejectReason::IncorrectNumInGroupCount))
    );
    // An entry that doesn't start with the delimiter.
    assert_eq!(
        failure(&v, "11=A|54=1|55=AAPL|78=1|80=5|79=X|"),
        (Some(80), Some(SessionRejectReason::RepeatingGroupFieldsOutOfOrder))
    );
    assert_eq!(failure(&v, "11=A|54=1|55=AAPL|78=x|").1, Some(SessionRejectReason::IncorrectDataFormat));
}

#[test]
fn each_check_can_be_turned_off() {
    let all_off = ValidationOptions {
        unknown_msg_types: false,
        undefined_tags: false,
        tags_not_in_message: false,
        required_tags: false,
        values: false,
        formats: false,
        repeated_tags: false,
        groups: false,
        allow_user_defined_tags: false,
        admin_messages: false,
    };
    let v = validator().with_options(all_off);
    for body in ["54=1|55=AAPL|", "11=A|54=9|55=AAPL|9999=x|58=hi|38=ten|11=B|", "11=A|54=1|55=AAPL|78=2|79=X|80=5|"] {
        v.validate(&msg(body)).unwrap_or_else(|e| panic!("{body}: {e:?}"));
    }
    v.validate(&msg(GOOD).with(tags::MSG_TYPE, "ZZ")).unwrap();
}

#[test]
fn user_defined_tags_can_be_allowed() {
    let options = ValidationOptions { allow_user_defined_tags: true, ..ValidationOptions::default() };
    let v = validator().with_options(options);
    v.validate(&msg("11=A|54=1|55=AAPL|9999=x|")).unwrap();
    // Below 5000 an undefined tag is still refused.
    assert_eq!(failure(&v, "11=A|54=1|55=AAPL|4999=x|").1, Some(SessionRejectReason::InvalidTagNumber));
}

#[test]
fn fix42_dictionaries_leave_out_reasons_added_later() {
    let dict = turbojet_dictionary::Dictionary::from_xml(&DICT.replace("minor='4'", "minor='2'")).unwrap();
    let v = Validator::new(&dict);
    // 13 was added in FIX 4.3, so a FIX 4.2 reject says so in words only.
    let err = v.validate(&msg("11=A|11=B|54=1|55=AAPL|")).unwrap_err();
    assert_eq!((err.tag, err.reason), (Some(11), None));
    assert!(err.text.contains("more than once"), "{}", err.text);
    // 1 is FIX 4.2's.
    assert_eq!(failure(&v, "54=1|55=AAPL|").1, Some(SessionRejectReason::RequiredTagMissing));
}

#[test]
fn admin_messages_the_dictionary_defines_are_checked() {
    let v = validator();
    assert!(v.applies_to(&MsgType::NewOrderSingle));
    // Session test case 14a: an unknown tag in a Heartbeat is rejected.
    assert!(v.applies_to(&MsgType::Heartbeat));
    let heartbeat = msg("9999=x|").with(tags::MSG_TYPE, "0");
    assert_eq!(v.validate(&heartbeat).unwrap_err().reason, Some(SessionRejectReason::InvalidTagNumber));
    // Not in the dictionary: the engine's alone.
    assert!(!v.applies_to(&MsgType::TestRequest));
    // Logon is the engine's, and Logout always completes.
    let dict = DICT.replace("</messages>", "<message name='Logon' msgtype='A' msgcat='admin'/><message name='Logout' msgtype='5' msgcat='admin'/></messages>");
    let v = Validator::new(&turbojet_dictionary::Dictionary::from_xml(&dict).unwrap());
    assert!(!v.applies_to(&MsgType::Logon));
    assert!(!v.applies_to(&MsgType::Logout));
    // And the whole check can be turned off.
    let v = v.with_options(ValidationOptions { admin_messages: false, ..ValidationOptions::default() });
    assert!(!v.applies_to(&MsgType::Heartbeat));
}
