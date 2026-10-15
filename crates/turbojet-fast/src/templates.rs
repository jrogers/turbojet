//! Loading templates from the concrete XML syntax (section 6 and Appendix 1) and compiling each
//! into a flat list of field instructions, with operators and dictionary slots resolved.

use std::collections::HashMap;
use std::sync::Arc;

use roxmltree::{Document, Node};
use turbojet::MsgType;

use crate::convert;
use crate::{Code, Error};

/// The template definition namespace of FAST 1.1.
const TD_NAMESPACE: &str = "http://www.fixprotocol.org/ns/fast/td/1.1";
/// The namespace of earlier drafts, which the specification's own introduction still uses.
const DRAFT_NAMESPACE: &str = "http://www.fixprotocol.org/ns/template-definition";

/// MsgType(35), which a template sets with a constant.
const MSG_TYPE: u32 = 35;

/// A set of FAST templates, loaded once and shared by the decoders using them.
#[derive(Debug)]
pub struct Templates {
    by_id: HashMap<u32, Arc<Template>>,
    /// The dictionary slots the templates' operators use, so a decoder can make one of each.
    pub(crate) slots: usize,
}

/// One template, compiled.
#[derive(Debug)]
pub(crate) struct Template {
    pub(crate) name: Box<str>,
    /// MsgType(35), from the template's constant; empty if it has none.
    pub(crate) msg_type: MsgType,
    pub(crate) fields: Vec<Field>,
    /// The most presence map bits the fields can use, after the template identifier's.
    pub(crate) presence_bits: usize,
}

/// A field instruction.
#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name: Box<str>,
    /// The FIX tag, from the field's `id`. A field without one is decoded but left out.
    pub(crate) tag: Option<u32>,
    pub(crate) optional: bool,
    pub(crate) kind: Kind,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Integer(IntegerType, Operator),
    /// A decimal with one operator for the whole value.
    Decimal(Operator),
    /// A decimal whose exponent (an `int32`) and mantissa (an `int64`) have operators of their own.
    DecimalParts {
        exponent: Operator,
        mantissa: Operator,
    },
    Ascii(Operator),
    Unicode(Operator),
    /// A byte vector. With a `<length>` that has an `id`, it becomes a FIX data field; without,
    /// its value is written as hexadecimal text (section 8.4).
    Bytes {
        operator: Operator,
        length_tag: Option<u32>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntegerType {
    Int32,
    UInt32,
    Int64,
    UInt64,
}

impl IntegerType {
    pub(crate) fn range(self) -> (i128, i128) {
        match self {
            Self::Int32 => (i128::from(i32::MIN), i128::from(i32::MAX)),
            Self::UInt32 => (0, i128::from(u32::MAX)),
            Self::Int64 => (i128::from(i64::MIN), i128::from(i64::MAX)),
            Self::UInt64 => (0, i128::from(u64::MAX)),
        }
    }

    pub(crate) fn is_signed(self) -> bool {
        matches!(self, Self::Int32 | Self::Int64)
    }
}

/// A field's operator, its initial value, and the dictionary slot of its previous value.
#[derive(Debug)]
pub(crate) struct Operator {
    pub(crate) kind: OperatorKind,
    pub(crate) initial: Option<Value>,
    /// The previous value's slot, for the operators that keep one.
    pub(crate) slot: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperatorKind {
    None,
    Constant,
    Default,
    Copy,
}

/// A value from a template: an initial value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Value {
    Integer(i128),
    Decimal {
        exponent: i32,
        mantissa: i64,
    },
    /// An ASCII or Unicode string, or a byte vector.
    Bytes(Box<[u8]>),
}

impl Templates {
    /// Loads templates from their XML definition: a `<templates>` element holding them, or a
    /// single `<template>`. Each template is found in the stream by its `id`, and each field's
    /// `id` is its FIX tag; MsgType(35) is a constant field, as in the specification's examples.
    ///
    /// # Errors
    ///
    /// A static error (S1-S5) if the XML isn't a valid definition, with the template and field;
    /// and an error for what this version doesn't handle yet: sequences, groups, template
    /// references and the increment, delta and tail operators.
    pub fn from_xml(xml: &str) -> Result<Self, Error> {
        let document = Document::parse(xml).map_err(|e| Error::spec(Code::S1).with_detail(e.to_string()))?;
        let root = document.root_element();
        let mut loader = Loader { slots: HashMap::new() };
        let mut by_id = HashMap::new();
        let inherited = Context::default().inherit(root);
        let templates: Vec<Node<'_, '_>> = match local_name(root) {
            Some("templates") => td_children(root).collect(),
            Some("template") => vec![root],
            _ => return Err(Error::spec(Code::S1).with_detail("the root isn't <templates> or <template>")),
        };
        for node in templates {
            if local_name(node) != Some("template") {
                return Err(Error::spec(Code::S1).with_detail("<templates> holds only <template> elements"));
            }
            let (id, template) = loader.template(node, &inherited)?;
            let name = template.name.clone();
            let Some(id) = id else { continue };
            if by_id.insert(id, Arc::new(template)).is_some() {
                return Err(Error::spec(Code::S1)
                    .with_template(&name)
                    .with_detail(format!("template id {id} is used twice")));
            }
        }
        Ok(Self { by_id, slots: loader.slots.len() })
    }

    pub(crate) fn get(&self, id: u32) -> Option<&Arc<Template>> {
        self.by_id.get(&id)
    }
}

/// Which dictionary a previous value lives in (section 6.3.1), with the name that scopes it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dictionary {
    Global,
    Template(String),
    Type(String),
    User(String),
}

/// What an element inherits from its ancestors: namespaces, the dictionary and the application
/// type.
#[derive(Debug, Clone, Default)]
struct Context {
    ns: String,
    template_ns: String,
    dictionary: Option<String>,
    /// The current application type, `ns` and name; "any" until a `<typeRef>`.
    application_type: Option<String>,
}

impl Context {
    fn inherit(&self, node: Node<'_, '_>) -> Self {
        let mut context = self.clone();
        if let Some(ns) = node.attribute("ns") {
            context.ns = ns.to_string();
        }
        if let Some(ns) = node.attribute("templateNs") {
            context.template_ns = ns.to_string();
        }
        if let Some(dictionary) = node.attribute("dictionary") {
            context.dictionary = Some(dictionary.to_string());
        }
        context
    }
}

/// The key of one previous value: its dictionary, then the key's namespace and name. A decimal's
/// exponent and mantissa have their own, unique to the decimal's name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SlotKey {
    dictionary: Dictionary,
    ns: String,
    key: String,
    part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Part {
    Whole,
    Exponent,
    Mantissa,
}

struct Loader {
    slots: HashMap<SlotKey, usize>,
}

impl Loader {
    fn template(&mut self, node: Node<'_, '_>, inherited: &Context) -> Result<(Option<u32>, Template), Error> {
        let name = required_attribute(node, "name")?;
        let mut context = inherited.inherit(node);
        let id = node.attribute("id").map(|id| parse_id(id).map_err(|e| e.with_template(name))).transpose()?;
        let template_key = format!("{}\u{1}{name}", context.template_ns);
        let mut fields = Vec::new();
        for child in td_children(node) {
            let field = match local_name(child) {
                Some("typeRef") => {
                    let ns = child.attribute("ns").unwrap_or(&context.ns);
                    context.application_type = Some(format!("{ns}\u{1}{}", required_attribute(child, "name")?));
                    continue;
                }
                _ => self.field(child, &context, &template_key),
            };
            fields.push(field.map_err(|e| e.with_template(name))?);
        }
        let msg_type = msg_type(&fields).map_err(|e| e.with_template(name))?;
        let presence_bits = fields.iter().map(Field::presence_bits).sum();
        Ok((id, Template { name: name.into(), msg_type, fields, presence_bits }))
    }

    fn field(&mut self, node: Node<'_, '_>, inherited: &Context, template_key: &str) -> Result<Field, Error> {
        let name = required_attribute(node, "name")?;
        let tag = node.attribute("id").map(parse_id).transpose().map_err(|e| e.with_field(name))?;
        let optional = match node.attribute("presence") {
            None | Some("mandatory") => false,
            Some("optional") => true,
            Some(other) => {
                return Err(Error::spec(Code::S1).with_field(name).with_detail(format!("presence {other:?}")));
            }
        };
        let context = inherited.inherit(node);
        let scope = Scope { loader: self, context: &context, template_key, name, optional };
        let kind = scope.kind(node).map_err(|e| e.with_field(name))?;
        Ok(Field { name: name.into(), tag, optional, kind })
    }

    /// The slot for `key`, made the first time it's asked for.
    fn slot(&mut self, key: SlotKey) -> usize {
        let next = self.slots.len();
        *self.slots.entry(key).or_insert(next)
    }
}

/// The value type a field or one of its parts has, for checking operators and initial values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Type {
    Integer(IntegerType),
    Decimal,
    Ascii,
    Unicode,
    Bytes,
}

/// One field being compiled: what its operators need from around it.
struct Scope<'l, 'c> {
    loader: &'l mut Loader,
    context: &'c Context,
    template_key: &'c str,
    name: &'c str,
    optional: bool,
}

impl Scope<'_, '_> {
    fn kind(mut self, node: Node<'_, '_>) -> Result<Kind, Error> {
        let scope = &mut self;
        let element = local_name(node).unwrap_or_default();
        let kind = match element {
            "int32" => {
                Kind::Integer(IntegerType::Int32, scope.operator(node, Type::Integer(IntegerType::Int32), Part::Whole)?)
            }
            "uInt32" => Kind::Integer(
                IntegerType::UInt32,
                scope.operator(node, Type::Integer(IntegerType::UInt32), Part::Whole)?,
            ),
            "int64" => {
                Kind::Integer(IntegerType::Int64, scope.operator(node, Type::Integer(IntegerType::Int64), Part::Whole)?)
            }
            "uInt64" => Kind::Integer(
                IntegerType::UInt64,
                scope.operator(node, Type::Integer(IntegerType::UInt64), Part::Whole)?,
            ),
            "decimal" => scope.decimal(node)?,
            "string" => match node.attribute("charset") {
                None | Some("ascii") => Kind::Ascii(scope.operator(node, Type::Ascii, Part::Whole)?),
                Some("unicode") => Kind::Unicode(scope.operator(node, Type::Unicode, Part::Whole)?),
                Some(other) => return Err(Error::spec(Code::S1).with_detail(format!("charset {other:?}"))),
            },
            "byteVector" => {
                let length_tag = td_children(node)
                    .find(|child| local_name(*child) == Some("length"))
                    .and_then(|length| length.attribute("id"))
                    .map(parse_id)
                    .transpose()?;
                Kind::Bytes { operator: scope.operator(node, Type::Bytes, Part::Whole)?, length_tag }
            }
            "sequence" | "group" | "templateRef" => {
                return Err(Error::unsupported(format!("<{element}> isn't supported yet")));
            }
            _ => return Err(Error::spec(Code::S1).with_detail(format!("<{element}> isn't a field"))),
        };
        Ok(kind)
    }

    fn decimal(&mut self, node: Node<'_, '_>) -> Result<Kind, Error> {
        let exponent = td_children(node).find(|child| local_name(*child) == Some("exponent"));
        let mantissa = td_children(node).find(|child| local_name(*child) == Some("mantissa"));
        if exponent.is_none() && mantissa.is_none() {
            return Ok(Kind::Decimal(self.operator(node, Type::Decimal, Part::Whole)?));
        }
        if operator_node(node)?.is_some() {
            return Err(Error::spec(Code::S1).with_detail("a decimal has one operator or operators for its parts"));
        }
        let exponent = match exponent {
            Some(node) => self.operator(node, Type::Integer(IntegerType::Int32), Part::Exponent)?,
            None => Operator { kind: OperatorKind::None, initial: None, slot: None },
        };
        // The mantissa is mandatory: it's present whenever the exponent is.
        let optional = std::mem::replace(&mut self.optional, false);
        let mantissa = match mantissa {
            Some(node) => self.operator(node, Type::Integer(IntegerType::Int64), Part::Mantissa),
            None => Ok(Operator { kind: OperatorKind::None, initial: None, slot: None }),
        };
        self.optional = optional;
        if exponent.initial.as_ref().is_some_and(|value| !convert::exponent_in_range(value)) {
            return Err(Error::spec(Code::S3).with_detail("the exponent's initial value is outside -63 to 63"));
        }
        Ok(Kind::DecimalParts { exponent, mantissa: mantissa? })
    }

    /// The operator among `node`'s children, compiled for a value of `ty`.
    fn operator(&mut self, node: Node<'_, '_>, ty: Type, part: Part) -> Result<Operator, Error> {
        let Some(op) = operator_node(node)? else {
            return Ok(Operator { kind: OperatorKind::None, initial: None, slot: None });
        };
        let kind = match local_name(op) {
            Some("constant") => OperatorKind::Constant,
            Some("default") => OperatorKind::Default,
            Some("copy") => OperatorKind::Copy,
            Some(other @ ("increment" | "delta" | "tail")) => {
                return Err(Error::unsupported(format!("the {other} operator isn't supported yet")));
            }
            _ => unreachable!("operator_node finds only operators"),
        };
        let initial = op.attribute("value").map(|value| convert::initial(value, ty)).transpose()?;
        match kind {
            OperatorKind::Constant if initial.is_none() => return Err(Error::spec(Code::S4)),
            OperatorKind::Default if initial.is_none() && !self.optional => return Err(Error::spec(Code::S5)),
            _ => {}
        }
        let slot = (kind == OperatorKind::Copy).then(|| {
            let context = self.context.inherit(op);
            let key = SlotKey {
                dictionary: self.dictionary(&context),
                ns: op.attribute("ns").map_or_else(|| context.ns.clone(), str::to_string),
                key: op.attribute("key").unwrap_or(self.name).to_string(),
                part: if op.attribute("key").is_some() { Part::Whole } else { part },
            };
            self.loader.slot(key)
        });
        Ok(Operator { kind, initial, slot })
    }

    fn dictionary(&self, context: &Context) -> Dictionary {
        match context.dictionary.as_deref() {
            None | Some("global") => Dictionary::Global,
            Some("template") => Dictionary::Template(self.template_key.to_string()),
            Some("type") => Dictionary::Type(context.application_type.clone().unwrap_or_else(|| "any".to_string())),
            Some(name) => Dictionary::User(name.to_string()),
        }
    }
}

impl Field {
    /// The most presence map bits the field uses (section 10.5.1).
    fn presence_bits(&self) -> usize {
        let bits = |operator: &Operator, optional: bool| match operator.kind {
            OperatorKind::None => 0,
            OperatorKind::Constant => usize::from(optional),
            OperatorKind::Default | OperatorKind::Copy => 1,
        };
        match &self.kind {
            Kind::Integer(_, operator)
            | Kind::Decimal(operator)
            | Kind::Ascii(operator)
            | Kind::Unicode(operator)
            | Kind::Bytes { operator, .. } => bits(operator, self.optional),
            Kind::DecimalParts { exponent, mantissa } => bits(exponent, self.optional) + bits(mantissa, false),
        }
    }
}

/// MsgType(35) from the template's constant field, or empty if it has none: a template such as a
/// feed's reset message carries no MsgType.
fn msg_type(fields: &[Field]) -> Result<MsgType, Error> {
    let Some(field) = fields.iter().find(|field| field.tag == Some(MSG_TYPE)) else {
        return Ok(MsgType::from_static(""));
    };
    let fail = || Error::unsupported("MsgType(35) must be a mandatory ASCII string constant").with_field(&field.name);
    let (Kind::Ascii(operator), false) = (&field.kind, field.optional) else { return Err(fail()) };
    match (&operator.kind, &operator.initial) {
        (OperatorKind::Constant, Some(Value::Bytes(code))) => {
            Ok(MsgType::from_code(std::str::from_utf8(code).map_err(|_| fail())?))
        }
        _ => Err(fail()),
    }
}

/// The single field operator among `node`'s children, if any.
fn operator_node<'a, 'i>(node: Node<'a, 'i>) -> Result<Option<Node<'a, 'i>>, Error> {
    let mut operators = td_children(node).filter(|child| {
        matches!(local_name(*child), Some("constant" | "default" | "copy" | "increment" | "delta" | "tail"))
    });
    let first = operators.next();
    if operators.next().is_some() {
        return Err(Error::spec(Code::S1).with_detail("a field has at most one operator"));
    }
    Ok(first)
}

/// An `id` as a FIX tag or template identifier.
fn parse_id(id: &str) -> Result<u32, Error> {
    id.trim().parse().map_err(|_| Error::unsupported(format!("id {id:?} isn't a number")))
}

fn required_attribute<'a>(node: Node<'a, '_>, name: &str) -> Result<&'a str, Error> {
    node.attribute(name)
        .ok_or_else(|| Error::spec(Code::S1).with_detail(format!("<{}> needs a {name}", node.tag_name().name())))
}

/// The element's name if it's in the template definition namespace. Elements in other
/// namespaces are extensions (section 9), skipped. Templates often leave the namespace out, and
/// the specification's own first example uses a draft's, so both count.
fn local_name<'a>(node: Node<'a, '_>) -> Option<&'a str> {
    let name = node.tag_name();
    matches!(name.namespace(), None | Some(TD_NAMESPACE | DRAFT_NAMESPACE)).then_some(name.name())
}

/// The child elements in the template definition namespace.
fn td_children<'a, 'i>(node: Node<'a, 'i>) -> impl Iterator<Item = Node<'a, 'i>> {
    node.children().filter(|child| child.is_element() && local_name(*child).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(fields: &str) -> Result<Templates, Error> {
        Templates::from_xml(&format!(
            r#"<templates xmlns="{TD_NAMESPACE}"><template name="T" id="1">{fields}</template></templates>"#
        ))
    }

    #[test]
    fn loads_the_specification_example_up_to_its_sequence() {
        // Section 1's example, without the sequence that this version can't decode yet.
        let templates = Templates::from_xml(
            r#"<templates xmlns="http://www.fixprotocol.org/ns/template-definition"
                 templateNs="http://www.fixprotocol.org/ns/templates/sample" ns="http://www.fixprotocol.org/ns/fix">
               <template name="MDRefreshSample" id="7">
                 <typeRef name="MarketDataIncrementalRefresh"/>
                 <string name="BeginString" id="8"> <constant value="FIX4.4"/> </string>
                 <string name="MessageType" id="35"> <constant value="X"/> </string>
                 <string name="SenderCompID" id="49"> <copy/> </string>
               </template>
             </templates>"#,
        )
        .expect("valid");
        let template = templates.get(7).expect("loaded by id");
        assert_eq!(template.msg_type.code(), "X");
        assert_eq!(template.fields.len(), 3);
        // Only the copy needs a bit.
        assert_eq!(template.presence_bits, 1);
        assert_eq!(templates.slots, 1);
    }

    #[test]
    fn static_errors_are_reported_with_their_codes() {
        assert_eq!(Templates::from_xml("<templates>").unwrap_err().code(), Some(Code::S1));
        let code = |fields| load(fields).unwrap_err().code();
        assert_eq!(code(r#"<uInt32 name="A"><constant/></uInt32>"#), Some(Code::S4));
        assert_eq!(code(r#"<uInt32 name="A"><default/></uInt32>"#), Some(Code::S5));
        assert_eq!(code(r#"<uInt32 name="A"><copy value="x"/></uInt32>"#), Some(Code::S3));
        assert_eq!(code(r#"<int32 name="A"><copy value="2147483648"/></int32>"#), Some(Code::S3));
        assert_eq!(code(r#"<uInt32 name="A"><copy/><default value="1"/></uInt32>"#), Some(Code::S1));
        assert_eq!(code(r#"<uInt32 name="A" presence="maybe"/>"#), Some(Code::S1));
        assert!(load(r#"<uInt32 name="A" presence="optional"><default/></uInt32>"#).is_ok());
    }

    #[test]
    fn errors_name_the_template_and_field() {
        let error = load(r#"<uInt32 name="Qty"><constant/></uInt32>"#).unwrap_err();
        assert_eq!(error.to_string(), "ERR S4 in template T, field Qty");
    }

    #[test]
    fn what_later_stages_add_is_refused_for_now() {
        for fields in [
            r#"<sequence name="S"><uInt32 name="A"/></sequence>"#,
            r#"<uInt32 name="A"><increment/></uInt32>"#,
            r#"<string name="A"><tail/></string>"#,
        ] {
            assert_eq!(load(fields).unwrap_err().code(), None, "{fields}");
        }
    }

    #[test]
    fn copies_share_a_slot_by_key_and_dictionary() {
        let templates = load(
            r#"<uInt32 name="A"><copy/></uInt32>
               <uInt32 name="B"><copy key="A"/></uInt32>
               <uInt32 name="C"><copy key="A" dictionary="template"/></uInt32>
               <decimal name="D"><exponent><copy/></exponent><mantissa><copy/></mantissa></decimal>"#,
        )
        .expect("valid");
        // A and B share one; C is A's key in another dictionary; D's parts have one each.
        assert_eq!(templates.slots, 4);
    }

    #[test]
    fn foreign_elements_are_skipped() {
        let templates = load(r#"<x:note xmlns:x="urn:example"><x:anything/></x:note><uInt32 name="A"/>"#);
        assert_eq!(templates.expect("valid").get(1).expect("loaded").fields.len(), 1);
    }

    #[test]
    fn msg_type_must_be_a_constant() {
        assert!(load(r#"<string name="MsgType" id="35"><copy/></string>"#).is_err());
        let templates = load(r#"<uInt32 name="A"/>"#).expect("valid");
        assert_eq!(templates.get(1).expect("loaded").msg_type, MsgType::from_static(""));
    }
}
