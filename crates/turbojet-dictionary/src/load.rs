use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

use roxmltree::{Document, Node};

use crate::model::{
    Category, Component, Dictionary, Field, FieldType, Member, Message, Protocol, Release, Value, Version,
};
use crate::orchestra;

/// Why a dictionary couldn't be loaded or merged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// The file, when loaded from one.
    pub path: Option<PathBuf>,
    /// The line of the offending element, when there is one.
    pub line: Option<u32>,
    /// What was wrong.
    pub message: String,
}

/// `path: line N: message`, leaving out what isn't known.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}: ", path.display())?;
        }
        if let Some(line) = self.line {
            write!(f, "line {line}: ")?;
        }
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn in_file(mut self, path: &Path) -> Self {
        self.path = Some(path.to_path_buf());
        self
    }
}

impl Dictionary {
    /// Parses and validates a dictionary in either format, told apart by the root element:
    /// `<fix>` for QuickFIX, `<fixr:repository>` for FIX Orchestra.
    ///
    /// # Errors
    ///
    /// If the root element is neither format's, or as [`from_quickfix`](Self::from_quickfix) and
    /// [`from_orchestra`](Self::from_orchestra).
    pub fn from_xml(xml: &str) -> Result<Self, Error> {
        let doc = document(xml)?;
        let root = doc.root_element();
        if root.tag_name().name() == "fix" {
            quickfix(&doc, None)
        } else if orchestra::is_any_repository(root) {
            orchestra::parse(&doc)
        } else {
            Err(error(&doc, root, "not a QuickFIX or Orchestra dictionary".into()))
        }
    }

    /// Parses and validates a QuickFIX-format dictionary.
    ///
    /// # Errors
    ///
    /// The XML doesn't parse, or the dictionary isn't valid: a member that isn't defined, a
    /// component that includes itself, and the like; the error says where.
    pub fn from_quickfix(xml: &str) -> Result<Self, Error> {
        parse(xml, None)
    }

    /// Parses and validates a FIX Orchestra repository, such as the FIX Trading Community's
    /// `OrchestraFIX44.xml`.
    ///
    /// # Errors
    ///
    /// The XML doesn't parse, or the dictionary isn't valid: a member that isn't defined, a
    /// component that includes itself, and the like; the error says where.
    pub fn from_orchestra(xml: &str) -> Result<Self, Error> {
        orchestra::parse(&document(xml)?)
    }

    /// Reads, parses and validates a dictionary file in either format, like
    /// [`from_xml`](Self::from_xml).
    ///
    /// # Errors
    ///
    /// If the file can't be read, or as [`from_xml`](Self::from_xml), naming the file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let xml = read(path)?;
        Self::from_xml(&xml).map_err(|e| e.in_file(path))
    }
}

pub(crate) fn read(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|e| Error {
        path: Some(path.to_path_buf()),
        line: None,
        message: e.to_string(),
    })
}

/// Parses a QuickFIX-format dictionary. With a `base`, names may also refer to the base's fields
/// and components (a venue overlay), and the root's version attributes may be left out.
pub(crate) fn parse(xml: &str, base: Option<&Dictionary>) -> Result<Dictionary, Error> {
    let doc = document(xml)?;
    if base.is_some() && orchestra::is_repository(doc.root_element()) {
        return Err(error(&doc, doc.root_element(), "Orchestra overlays aren't supported yet".into()));
    }
    quickfix(&doc, base)
}

pub(crate) fn document(xml: &str) -> Result<Document<'_>, Error> {
    Document::parse(xml).map_err(|e| Error { path: None, line: Some(e.pos().row), message: e.to_string() })
}

/// An error at `node`'s line.
pub(crate) fn error(doc: &Document, node: Node, message: String) -> Error {
    let line = doc.text_pos_at(node.range().start).row;
    Error { path: None, line: Some(line), message }
}

fn quickfix(doc: &Document, base: Option<&Dictionary>) -> Result<Dictionary, Error> {
    Parser { doc, base, fields: HashMap::new(), components: HashSet::new() }.parse()
}

struct Parser<'a, 'input> {
    doc: &'a Document<'input>,
    base: Option<&'a Dictionary>,
    /// This document's fields by name, with their types.
    fields: HashMap<String, FieldType>,
    /// This document's component names.
    components: HashSet<String>,
}

impl Parser<'_, '_> {
    fn parse(mut self) -> Result<Dictionary, Error> {
        let root = self.doc.root_element();
        if root.tag_name().name() != "fix" {
            return Err(self.error(root, "the root element must be <fix>".into()));
        }
        let version = self.version(root)?;
        let fields = match child(root, "fields") {
            Some(node) => self.fields(node)?,
            None => Vec::new(),
        };
        self.fields = fields.iter().map(|f| (f.name.clone(), f.ty.clone())).collect();
        // Component names first: members may refer to components defined later in the file.
        if let Some(node) = child(root, "components") {
            for el in elements(node) {
                let name = self.attr(el, "name")?;
                if !self.components.insert(name.to_string()) {
                    return Err(self.error(el, format!("component {name} is defined twice")));
                }
            }
        }
        let header = self.section(root, "header")?;
        let trailer = self.section(root, "trailer")?;
        let mut components = Vec::new();
        if let Some(node) = child(root, "components") {
            for el in elements(node) {
                components.push(Component { name: self.attr(el, "name")?.to_string(), members: self.members(el)? });
            }
            self.check_cycles(node, &components)?;
        }
        let mut messages = Vec::new();
        if let Some(node) = child(root, "messages") {
            let mut names = HashSet::new();
            let mut msg_types: HashMap<&str, &str> = HashMap::new();
            for el in elements(node) {
                let name = self.attr(el, "name")?;
                let msg_type = self.attr(el, "msgtype")?;
                if !names.insert(name) {
                    return Err(self.error(el, format!("message {name} is defined twice")));
                }
                if let Some(other) = msg_types.insert(msg_type, name) {
                    return Err(self.error(el, format!("msgtype {msg_type} is both {other} and {name}")));
                }
                self.check_message_against_base(el, name, msg_type)?;
                let category = match el.attribute("msgcat") {
                    Some("admin") => Category::Admin,
                    _ => Category::App,
                };
                messages.push(Message {
                    name: name.to_string(),
                    msg_type: msg_type.to_string(),
                    category,
                    members: self.members(el)?,
                    doc: None,
                });
            }
        }
        Ok(Dictionary::new(version, header, trailer, fields, components, messages))
    }

    fn section(&self, root: Node, name: &str) -> Result<Vec<Member>, Error> {
        match child(root, name) {
            Some(node) => self.members(node),
            None => Ok(Vec::new()),
        }
    }

    fn members(&self, node: Node) -> Result<Vec<Member>, Error> {
        elements(node).map(|el| self.member(el)).collect()
    }

    fn member(&self, el: Node) -> Result<Member, Error> {
        let name = self.attr(el, "name")?.to_string();
        let required = el.attribute("required") == Some("Y");
        match el.tag_name().name() {
            "field" => {
                self.field_type(el, &name)?;
                Ok(Member::Field { name, required })
            }
            "group" => {
                let ty = self.field_type(el, &name)?;
                if !matches!(ty, FieldType::NumInGroup | FieldType::Int) {
                    return Err(self.error(el, format!("group {name}'s count field must be NUMINGROUP or INT")));
                }
                let members = self.members(el)?;
                if members.is_empty() {
                    return Err(self.error(el, format!("group {name} has no members")));
                }
                Ok(Member::Group { name, official_name: None, required, members })
            }
            "component" => {
                let known = self.components.contains(&name) || self.base.is_some_and(|b| b.component(&name).is_some());
                if !known {
                    return Err(self.error(el, format!("unknown component {name}")));
                }
                Ok(Member::Component { name, required })
            }
            other => Err(self.error(el, format!("unexpected <{other}>"))),
        }
    }

    /// The type of a field defined here or in the base.
    fn field_type(&self, el: Node, name: &str) -> Result<FieldType, Error> {
        self.fields
            .get(name)
            .cloned()
            .or_else(|| self.base.and_then(|b| b.field(name)).map(|f| f.ty.clone()))
            .ok_or_else(|| self.error(el, format!("unknown field {name}")))
    }

    fn version(&self, root: Node) -> Result<Version, Error> {
        if let Some(base) = self.base
            && root.attribute("major").is_none()
        {
            return Ok(base.version);
        }
        let protocol = match root.attribute("type") {
            Some("FIXT") => Protocol::Fixt,
            _ => Protocol::Fix,
        };
        let number = |name: &str, default: Option<u32>| match root.attribute(name) {
            Some(text) => text.parse().map_err(|_| self.error(root, format!("{name}='{text}' is not a number"))),
            None => default.ok_or_else(|| self.error(root, format!("<fix> needs a {name} attribute"))),
        };
        let release = Release::Numbered {
            major: number("major", None)?,
            minor: number("minor", None)?,
            service_pack: number("servicepack", Some(0))?,
        };
        Ok(Version { protocol, release, extension_pack: None })
    }

    fn fields(&self, node: Node) -> Result<Vec<Field>, Error> {
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        let mut tags: HashMap<u32, String> = HashMap::new();
        for el in elements(node) {
            let name = self.attr(el, "name")?.to_string();
            let number = self.attr(el, "number")?;
            let tag: u32 = number.parse().map_err(|_| self.error(el, format!("field {name} has number '{number}'")))?;
            if !names.insert(name.clone()) {
                return Err(self.error(el, format!("field {name} is defined twice")));
            }
            if let Some(other) = tags.insert(tag, name.clone()) {
                return Err(self.error(el, format!("tag {tag} is both {other} and {name}")));
            }
            let ty = FieldType::parse(self.attr(el, "type")?);
            self.check_against_base(el, &name, tag, &ty)?;
            let mut values = Vec::new();
            let mut codes = HashSet::new();
            for v in elements(el) {
                let code = self.attr(v, "enum")?;
                if !codes.insert(code) {
                    return Err(self.error(v, format!("field {name} has value {code} twice")));
                }
                let description = v.attribute("description").unwrap_or_default().to_string();
                values.push(Value { code: code.to_string(), name: None, description, doc: None });
            }
            fields.push(Field { tag, name, ty, values, doc: None, deprecated: false });
        }
        Ok(fields)
    }

    /// A venue may extend a base field, but not renumber it, reuse its tag for another name, or
    /// retype a count field the base's groups rely on.
    fn check_against_base(&self, el: Node, name: &str, tag: u32, ty: &FieldType) -> Result<(), Error> {
        let Some(base) = self.base else { return Ok(()) };
        if let Some(existing) = base.field(name)
            && existing.tag != tag
        {
            return Err(
                self.error(el, format!("field {name} has tag {tag}, but {} in the base dictionary", existing.tag))
            );
        }
        if let Some(existing) = base.field_by_tag(tag)
            && existing.name != name
        {
            return Err(self.error(el, format!("tag {tag} is {name}, but {} in the base dictionary", existing.name)));
        }
        let is_count = |ty: &FieldType| matches!(ty, FieldType::NumInGroup | FieldType::Int);
        if let Some(existing) = base.field(name)
            && is_count(&existing.ty)
            && !is_count(ty)
            && counts_group(base, name)
        {
            return Err(self.error(
                el,
                format!("field {name} counts a group in the base dictionary, so it must stay NUMINGROUP or INT"),
            ));
        }
        Ok(())
    }

    /// A venue may replace a base message, but not give its msgtype to another name.
    fn check_message_against_base(&self, el: Node, name: &str, msg_type: &str) -> Result<(), Error> {
        let Some(base) = self.base else { return Ok(()) };
        if let Some(existing) = base.messages().iter().find(|m| m.msg_type == msg_type)
            && existing.name != name
        {
            return Err(
                self.error(el, format!("msgtype {msg_type} is {name}, but {} in the base dictionary", existing.name))
            );
        }
        Ok(())
    }

    /// Rejects a component that includes itself (see [`component_cycle`]), reporting it where
    /// it's defined.
    fn check_cycles(&self, node: Node, components: &[Component]) -> Result<(), Error> {
        let base = self.base.map(Dictionary::components).unwrap_or_default();
        match component_cycle(base, components) {
            Some((i, message)) => Err(self.error(elements(node).nth(i).unwrap_or(node), message)),
            None => Ok(()),
        }
    }

    fn attr<'n>(&self, el: Node<'n, '_>, name: &str) -> Result<&'n str, Error> {
        el.attribute(name).ok_or_else(|| self.error(el, format!("<{}> needs a {name} attribute", el.tag_name().name())))
    }

    fn error(&self, node: Node, message: String) -> Error {
        error(self.doc, node, message)
    }
}

/// Whether any group in `dict` is counted by the field `name`.
fn counts_group(dict: &Dictionary, name: &str) -> bool {
    fn any(members: &[Member], name: &str) -> bool {
        members.iter().any(|m| match m {
            Member::Group { name: count, members, .. } => count == name || any(members, name),
            _ => false,
        })
    }
    any(&dict.header, name)
        || any(&dict.trailer, name)
        || dict.components().iter().any(|c| any(&c.members, name))
        || dict.messages().iter().any(|m| any(&m.members, name))
}

/// A component that includes itself, directly or through others: its index in `components` and
/// the error message. `components` are added to or replace `base`'s; the base has no cycles, so
/// any cycle passes through one of `components`, and is reported there.
pub(crate) fn component_cycle(base: &[Component], components: &[Component]) -> Option<(usize, String)> {
    let all: HashMap<&str, &[Member]> =
        base.iter().chain(components).map(|c| (c.name.as_str(), c.members.as_slice())).collect();
    let defined: HashMap<&str, usize> = components.iter().enumerate().map(|(i, c)| (c.name.as_str(), i)).collect();
    let mut done = HashSet::new();
    for component in components {
        let Some(mut cycle) = find_cycle(&all, &component.name, &mut Vec::new(), &mut done) else { continue };
        // Start the cycle at a component defined here, and report it there.
        let first = cycle.iter().position(|name| defined.contains_key(name)).unwrap_or(0);
        cycle.rotate_left(first);
        let name = cycle[0];
        cycle.push(name);
        return Some((defined[name], format!("component {name} includes itself ({})", cycle.join(" -> "))));
    }
    None
}

/// A cycle of components reachable from `name`, each included by the one before it (and the first
/// by the last). `path` is the chain of components being expanded; `done` are those known to
/// reach no cycle.
fn find_cycle<'a>(
    all: &HashMap<&'a str, &'a [Member]>,
    name: &'a str,
    path: &mut Vec<&'a str>,
    done: &mut HashSet<&'a str>,
) -> Option<Vec<&'a str>> {
    fn includes<'a>(members: &'a [Member], out: &mut Vec<&'a str>) {
        for member in members {
            match member {
                Member::Component { name, .. } => out.push(name),
                Member::Group { members, .. } => includes(members, out),
                Member::Field { .. } => {}
            }
        }
    }
    if done.contains(name) {
        return None;
    }
    if let Some(start) = path.iter().position(|&p| p == name) {
        return Some(path[start..].to_vec());
    }
    path.push(name);
    let mut children = Vec::new();
    includes(all.get(name).copied().unwrap_or_default(), &mut children);
    for child in children {
        if let Some(cycle) = find_cycle(all, child, path, done) {
            return Some(cycle);
        }
    }
    path.pop();
    done.insert(name);
    None
}

fn elements<'a, 'input>(node: Node<'a, 'input>) -> impl Iterator<Item = Node<'a, 'input>> {
    node.children().filter(Node::is_element)
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    elements(node).find(|n| n.tag_name().name() == name)
}
