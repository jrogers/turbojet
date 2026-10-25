//! Loading templates from the concrete XML syntax (section 6 and Appendix 1) and compiling each
//! into a flat list of field instructions, with operators and dictionary slots resolved.

use std::collections::HashMap;
use std::sync::Arc;

use crate::MsgType;
use roxmltree::{Document, Node};

use crate::fast::convert;
use crate::fast::{Code, Error};

/// The template definition namespace of FAST 1.1.
const TD_NAMESPACE: &str = "http://www.fixprotocol.org/ns/fast/td/1.1";
/// The namespace of earlier drafts, which the specification's own introduction still uses.
const DRAFT_NAMESPACE: &str = "http://www.fixprotocol.org/ns/template-definition";

/// MsgType(35), which a template sets with a constant.
const MSG_TYPE: u32 = 35;

/// How deeply sequences, groups and template references may nest, in templates and, through
/// dynamic template references, in the stream. Feeds nest two or three deep; the limit stops a
/// reference cycle in templates, and recursion that input controls.
pub(crate) const MAX_NESTING: usize = 16;

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
    /// Its fields; the presence map bits they use come after the template identifier's.
    pub(crate) body: Group,
}

/// The fields of a template, group or sequence element: a segment with a presence map of its own
/// if any of them uses a bit (sections 6.2.5, 6.2.6 and 10).
#[derive(Debug)]
pub(crate) struct Group {
    pub(crate) fields: Vec<Field>,
    /// The most presence map bits the fields can use.
    pub(crate) presence_bits: usize,
}

impl Group {
    fn new(fields: Vec<Field>) -> Self {
        let presence_bits = fields.iter().map(Field::presence_bits).sum();
        Self { fields, presence_bits }
    }
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
    /// A group, whose fields are written into the message in its place. Optional, it takes a bit.
    Group(Group),
    /// A sequence: a FIX repeating group, its length the NumInGroup field `length_tag`.
    Sequence {
        length: Operator,
        length_tag: u32,
        element: Group,
    },
    /// A dynamic template reference: a segment of its own, of the template it identifies.
    TemplateRef,
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
    Increment,
    Delta,
    Tail,
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
    /// D8 for a static reference to a template that isn't there; and an error for a template
    /// this crate can't map to FIX: a field `id` that isn't a number, a sequence without a
    /// `<length>` `id`, a MsgType(35) that isn't a constant, or references nested more than 16
    /// deep (a cycle, say).
    pub fn from_xml(xml: &str) -> Result<Self, Error> {
        let document = Document::parse(xml).map_err(|e| Error::spec(Code::S1).with_detail(e.to_string()))?;
        let root = document.root_element();
        let inherited = Context::default().inherit(root);
        let templates: Vec<Node<'_, '_>> = match local_name(root) {
            Some("templates") => td_children(root).collect(),
            Some("template") => vec![root],
            _ => return Err(Error::spec(Code::S1).with_detail("the root isn't <templates> or <template>")),
        };
        let mut loader = Loader { slots: HashMap::new(), by_name: HashMap::new() };
        for &node in &templates {
            if local_name(node) != Some("template") {
                return Err(Error::spec(Code::S1).with_detail("<templates> holds only <template> elements"));
            }
            let context = inherited.inherit(node);
            let name = (context.template_ns.clone(), required_attribute(node, "name")?.to_string());
            loader.by_name.insert(name, (node, context));
        }
        let mut by_id = HashMap::new();
        for node in templates {
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
    /// A sequence's length, when it has no name of its own.
    Length,
}

struct Loader<'d, 'i> {
    slots: HashMap<SlotKey, usize>,
    /// Every template's element and what it inherits, by `templateNs` and name, for references.
    by_name: HashMap<(String, String), (Node<'d, 'i>, Context)>,
}

impl<'d, 'i> Loader<'d, 'i> {
    fn template(&mut self, node: Node<'d, 'i>, inherited: &Context) -> Result<(Option<u32>, Template), Error> {
        let name = required_attribute(node, "name")?;
        let context = inherited.inherit(node);
        let id = node.attribute("id").map(|id| parse_id(id).map_err(|e| e.with_template(name))).transpose()?;
        let fields = self.template_fields(node, &context, 0).map_err(|e| e.with_template(name))?;
        let msg_type = msg_type(&fields).map_err(|e| e.with_template(name))?;
        Ok((id, Template { name: name.into(), msg_type, body: Group::new(fields) }))
    }

    /// A template's instructions, compiled in its own context: a static reference to it from
    /// another template makes it the current template (section 6.4).
    fn template_fields(&mut self, node: Node<'d, 'i>, context: &Context, depth: usize) -> Result<Vec<Field>, Error> {
        let name = required_attribute(node, "name")?;
        let template_key = format!("{}\u{1}{name}", context.template_ns);
        self.instructions(node, context, &template_key, depth)
    }

    /// The instructions among `parent`'s children, static template references expanded.
    fn instructions(
        &mut self,
        parent: Node<'d, 'i>,
        inherited: &Context,
        template_key: &str,
        depth: usize,
    ) -> Result<Vec<Field>, Error> {
        if depth > MAX_NESTING {
            return Err(Error::unsupported(format!("nesting deeper than {MAX_NESTING} (a reference cycle?)")));
        }
        let mut context = inherited.clone();
        let mut fields = Vec::new();
        for child in td_children(parent) {
            match local_name(child) {
                Some("typeRef") => {
                    let ns = child.attribute("ns").unwrap_or(&context.ns);
                    context.application_type = Some(format!("{ns}\u{1}{}", required_attribute(child, "name")?));
                }
                // A sequence's length, compiled with the sequence.
                Some("length") if local_name(parent) == Some("sequence") => {}
                Some("templateRef") => match child.attribute("name") {
                    Some(name) => fields.extend(self.static_reference(child, name, &context, depth)?),
                    None => fields.push(Field {
                        name: "templateRef".into(),
                        tag: None,
                        optional: false,
                        kind: Kind::TemplateRef,
                    }),
                },
                _ => fields.push(self.field(child, &context, template_key, depth)?),
            }
        }
        Ok(fields)
    }

    /// The instructions of the template a static reference names [ERR D8 if none].
    fn static_reference(
        &mut self,
        node: Node<'d, 'i>,
        name: &str,
        context: &Context,
        depth: usize,
    ) -> Result<Vec<Field>, Error> {
        let ns = node.attribute("templateNs").unwrap_or(&context.template_ns);
        let Some((template, template_context)) = self.by_name.get(&(ns.to_string(), name.to_string())).cloned() else {
            return Err(Error::spec(Code::D8).with_detail(format!("no template {name}")));
        };
        self.template_fields(template, &template_context, depth + 1)
    }

    fn field(
        &mut self,
        node: Node<'d, 'i>,
        inherited: &Context,
        template_key: &str,
        depth: usize,
    ) -> Result<Field, Error> {
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
        let scope = Scope { loader: self, context: &context, template_key, name, optional, depth };
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
struct Scope<'l, 'c, 'd, 'i> {
    loader: &'l mut Loader<'d, 'i>,
    context: &'c Context,
    template_key: &'c str,
    name: &'c str,
    optional: bool,
    depth: usize,
}

impl<'d, 'i> Scope<'_, '_, 'd, 'i> {
    fn kind(mut self, node: Node<'d, 'i>) -> Result<Kind, Error> {
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
            "group" => {
                let fields = scope.loader.instructions(node, scope.context, scope.template_key, scope.depth + 1)?;
                Kind::Group(Group::new(fields))
            }
            "sequence" => scope.sequence(node)?,
            _ => return Err(Error::spec(Code::S1).with_detail(format!("<{element}> isn't a field"))),
        };
        Ok(kind)
    }

    /// A sequence: its length field, a `uInt32` with an `id` for its NumInGroup tag, then the
    /// element's instructions.
    fn sequence(&mut self, node: Node<'d, 'i>) -> Result<Kind, Error> {
        let length = td_children(node).find(|child| local_name(*child) == Some("length"));
        let length_tag = length.and_then(|length| length.attribute("id")).map(parse_id).transpose()?;
        let Some(length_tag) = length_tag else {
            return Err(Error::unsupported("a sequence needs a <length> with an id, its NumInGroup tag"));
        };
        let length = length.expect("found with its id");
        // The length's previous value is keyed by its name, or a name unique to the sequence's.
        let (name, part) = match length.attribute("name") {
            Some(name) => (name, Part::Whole),
            None => (self.name, Part::Length),
        };
        let mut length_scope = Scope { loader: &mut *self.loader, name, ..*self };
        let operator = length_scope.operator(length, Type::Integer(IntegerType::UInt32), part);
        let fields = self.loader.instructions(node, self.context, self.template_key, self.depth + 1)?;
        Ok(Kind::Sequence { length: operator?, length_tag, element: Group::new(fields) })
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
            Some("increment") => OperatorKind::Increment,
            Some("delta") => OperatorKind::Delta,
            Some("tail") => OperatorKind::Tail,
            _ => unreachable!("operator_node finds only operators"),
        };
        // Section 6.3: increment is for integers, tail for strings and byte vectors.
        let applicable = match kind {
            OperatorKind::Increment => matches!(ty, Type::Integer(_)),
            OperatorKind::Tail => matches!(ty, Type::Ascii | Type::Unicode | Type::Bytes),
            _ => true,
        };
        if !applicable {
            return Err(Error::spec(Code::S2));
        }
        let initial = op.attribute("value").map(|value| convert::initial(value, ty)).transpose()?;
        match kind {
            OperatorKind::Constant if initial.is_none() => return Err(Error::spec(Code::S4)),
            OperatorKind::Default if initial.is_none() && !self.optional => return Err(Error::spec(Code::S5)),
            _ => {}
        }
        let keeps_previous = !matches!(kind, OperatorKind::None | OperatorKind::Constant | OperatorKind::Default);
        let slot = keeps_previous.then(|| {
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
            OperatorKind::Default | OperatorKind::Copy | OperatorKind::Increment | OperatorKind::Tail => 1,
            OperatorKind::Delta => 0,
        };
        match &self.kind {
            Kind::Integer(_, operator)
            | Kind::Decimal(operator)
            | Kind::Ascii(operator)
            | Kind::Unicode(operator)
            | Kind::Bytes { operator, .. } => bits(operator, self.optional),
            Kind::DecimalParts { exponent, mantissa } => bits(exponent, self.optional) + bits(mantissa, false),
            Kind::Group(_) => usize::from(self.optional),
            Kind::Sequence { length, .. } => bits(length, self.optional),
            Kind::TemplateRef => 0,
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
        assert_eq!(template.body.fields.len(), 3);
        // Only the copy needs a bit.
        assert_eq!(template.body.presence_bits, 1);
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
        assert_eq!(code(r#"<string name="A"><increment/></string>"#), Some(Code::S2));
        assert_eq!(code(r#"<decimal name="A"><increment/></decimal>"#), Some(Code::S2));
        assert_eq!(code(r#"<uInt32 name="A"><tail/></uInt32>"#), Some(Code::S2));
        assert_eq!(code(r#"<decimal name="A"><mantissa><tail/></mantissa></decimal>"#), Some(Code::S2));
        assert!(load(r#"<decimal name="A"><mantissa><increment/></mantissa></decimal>"#).is_ok());
        assert!(load(r#"<uInt32 name="A" presence="optional"><default/></uInt32>"#).is_ok());
    }

    #[test]
    fn errors_name_the_template_and_field() {
        let error = load(r#"<uInt32 name="Qty"><constant/></uInt32>"#).unwrap_err();
        assert_eq!(error.to_string(), "ERR S4 in template T, field Qty");
    }

    #[test]
    fn references_and_sequences_are_checked_as_they_load() {
        // A static reference to an unknown template, a cycle, and a sequence without a NumInGroup.
        assert_eq!(load(r#"<templateRef name="Missing"/>"#).unwrap_err().code(), Some(Code::D8));
        let cycle = Templates::from_xml(
            r#"<templates><template name="A"><templateRef name="B"/></template>
                          <template name="B"><templateRef name="A"/></template></templates>"#,
        );
        assert_eq!(cycle.unwrap_err().kind(), crate::fast::ErrorKind::Unsupported);
        let error = load(r#"<sequence name="S"><uInt32 name="A"/></sequence>"#).unwrap_err();
        assert_eq!(error.kind(), crate::fast::ErrorKind::Unsupported);
    }

    #[test]
    fn static_references_inline_the_template_and_count_its_bits() {
        let templates = Templates::from_xml(
            r#"<templates>
                 <template name="Header"><uInt32 name="MsgSeqNum" id="34"><increment/></uInt32></template>
                 <template name="Heartbeat" id="1"><templateRef name="Header"/><string name="Text" id="58"/></template>
               </templates>"#,
        )
        .expect("valid");
        let heartbeat = templates.get(1).expect("loaded");
        assert_eq!(heartbeat.body.fields.len(), 2);
        assert_eq!(heartbeat.body.presence_bits, 1);
    }

    #[test]
    fn groups_and_sequences_count_their_own_bits() {
        let templates = load(
            r#"<group name="G" presence="optional"><uInt32 name="A"><copy/></uInt32></group>
               <sequence name="S"><length name="N" id="9"><copy/></length><uInt32 name="B"/></sequence>"#,
        )
        .expect("valid");
        let template = templates.get(1).expect("loaded");
        // The optional group's bit and the length's copy; the group's own copy is in its own map.
        assert_eq!(template.body.presence_bits, 2);
        let Kind::Group(group) = &template.body.fields[0].kind else { panic!("a group") };
        assert_eq!(group.presence_bits, 1);
        let Kind::Sequence { element, length_tag, .. } = &template.body.fields[1].kind else { panic!("a sequence") };
        assert_eq!((*length_tag, element.presence_bits), (9, 0));
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
        assert_eq!(templates.expect("valid").get(1).expect("loaded").body.fields.len(), 1);
    }

    #[test]
    fn msg_type_must_be_a_constant() {
        assert!(load(r#"<string name="MsgType" id="35"><copy/></string>"#).is_err());
        let templates = load(r#"<uInt32 name="A"/>"#).expect("valid");
        assert_eq!(templates.get(1).expect("loaded").msg_type, MsgType::from_static(""));
    }
}
