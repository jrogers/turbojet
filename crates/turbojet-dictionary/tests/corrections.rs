//! Correcting a dictionary's mistakes: making fields and groups optional.

use turbojet_dictionary::{Dictionary, Member};

const DICT: &str = "<fix type='FIX' major='4' minor='2'>
 <messages>
  <message name='QuoteCancel' msgtype='Z' msgcat='app'>
   <field name='QuoteID' required='Y'/>
   <field name='QuoteCancelType' required='Y'/>
   <group name='NoQuoteEntries' required='Y'>
    <field name='Symbol' required='Y'/>
    <field name='UnderlyingSymbol' required='Y'/>
   </group>
   <component name='Instrument' required='Y'/>
  </message>
  <message name='NewOrderSingle' msgtype='D' msgcat='app'>
   <field name='QuoteID' required='Y'/>
   <component name='Instrument' required='Y'/>
  </message>
 </messages>
 <components>
  <component name='Instrument'>
   <field name='SecurityID' required='Y'/>
  </component>
 </components>
 <fields>
  <field number='117' name='QuoteID' type='STRING'/>
  <field number='298' name='QuoteCancelType' type='INT'/>
  <field number='295' name='NoQuoteEntries' type='INT'/>
  <field number='55' name='Symbol' type='STRING'/>
  <field number='311' name='UnderlyingSymbol' type='STRING'/>
  <field number='48' name='SecurityID' type='STRING'/>
 </fields>
</fix>";

fn dict() -> Dictionary {
    Dictionary::from_xml(DICT).unwrap()
}

/// Whether `name` is required among `members`, searching groups too.
fn required(members: &[Member], name: &str) -> Option<bool> {
    members.iter().find_map(|m| match m {
        _ if m.name() == name => Some(m.required()),
        Member::Group { members, .. } => required(members, name),
        _ => None,
    })
}

fn message<'a>(dict: &'a Dictionary, name: &str) -> &'a [Member] {
    &dict.message(name).unwrap().members
}

#[test]
fn makes_a_group_optional() {
    let mut dict = dict();
    dict.make_optional("QuoteCancel", "NoQuoteEntries").unwrap();
    assert_eq!(required(message(&dict, "QuoteCancel"), "NoQuoteEntries"), Some(false));
    // Its entries' members keep their requiredness.
    assert_eq!(required(message(&dict, "QuoteCancel"), "Symbol"), Some(true));
    assert_eq!(required(message(&dict, "QuoteCancel"), "QuoteID"), Some(true));
}

#[test]
fn makes_a_field_inside_a_group_optional() {
    let mut dict = dict();
    dict.make_optional("QuoteCancel", "UnderlyingSymbol").unwrap();
    assert_eq!(required(message(&dict, "QuoteCancel"), "UnderlyingSymbol"), Some(false));
    assert_eq!(required(message(&dict, "QuoteCancel"), "NoQuoteEntries"), Some(true));
}

#[test]
fn only_touches_the_named_message() {
    let mut dict = dict();
    dict.make_optional("QuoteCancel", "QuoteID").unwrap();
    assert_eq!(required(message(&dict, "QuoteCancel"), "QuoteID"), Some(false));
    assert_eq!(required(message(&dict, "NewOrderSingle"), "QuoteID"), Some(true));
}

#[test]
fn a_component_member_is_corrected_on_the_component_for_every_message() {
    let mut dict = dict();
    let err = dict.make_optional("QuoteCancel", "SecurityID").unwrap_err();
    assert_eq!(
        err.message,
        "QuoteCancel has no field or group SecurityID of its own; if it's in a component, correct the component"
    );
    dict.make_optional("Instrument", "SecurityID").unwrap();
    assert_eq!(required(&dict.component("Instrument").unwrap().members, "SecurityID"), Some(false));
    // A component reference itself can be made optional too.
    dict.make_optional("QuoteCancel", "Instrument").unwrap();
    assert_eq!(required(message(&dict, "QuoteCancel"), "Instrument"), Some(false));
}

#[test]
fn unknown_names_are_errors() {
    let mut dict = dict();
    let err = dict.make_optional("QuoteCancell", "NoQuoteEntries").unwrap_err();
    assert_eq!(err.message, "no message or component QuoteCancell");
    let err = dict.make_optional("QuoteCancel", "NoQuoteEntry").unwrap_err();
    assert!(err.message.starts_with("QuoteCancel has no field or group NoQuoteEntry"), "{err}");
}
