//! FIX Orchestra repositories (`fixr:repository`), such as the FIX Trading Community's
//! `OrchestraFIX44.xml`.

use std::collections::{HashMap, HashSet};

use roxmltree::{Document, Node};

use crate::load::{self, Error};
use crate::model::{
    Category, Component, Dictionary, Field, FieldType, Member, Message, Protocol, Release, Value, Version,
};

/// The Orchestra repository namespace.
const NS: &str = "http://fixprotocol.io/2020/orchestra/repository";

/// Whether `node` is the Orchestra element `name`.
fn is(node: Node, name: &str) -> bool {
    node.is_element() && node.tag_name().namespace() == Some(NS) && node.tag_name().name() == name
}

pub(crate) fn is_repository(node: Node) -> bool {
    is(node, "repository")
}

/// Whether `node` looks like a repository in some namespace, supported or not.
pub(crate) fn is_any_repository(node: Node) -> bool {
    node.tag_name().name() == "repository" && node.tag_name().namespace().is_some()
}

/// Parses and validates a repository.
pub(crate) fn parse(doc: &Document) -> Result<Dictionary, Error> {
    let root = doc.root_element();
    let parser = Parser {
        doc,
        datatypes: HashMap::new(),
        fields: Vec::new(),
        tags: HashMap::new(),
        components: HashMap::new(),
        groups: HashMap::new(),
        expanded: HashMap::new(),
        expanding: Vec::new(),
        header: None,
        trailer: None,
    };
    if !is_repository(root) {
        let message = match root.tag_name().namespace() {
            Some(ns) if is_any_repository(root) => format!("unsupported Orchestra namespace '{ns}'"),
            _ => "the root element must be <fixr:repository>".into(),
        };
        return Err(parser.error(root, message));
    }
    parser.parse(root)
}

struct Parser<'a, 'input> {
    doc: &'a Document<'input>,
    /// Datatypes by name, with their base types.
    datatypes: HashMap<&'a str, Option<&'a str>>,
    fields: Vec<Field>,
    /// Indexes into `fields` by tag, which is the field's id.
    tags: HashMap<u32, usize>,
    /// Component names and elements by id.
    components: HashMap<&'a str, (&'a str, Node<'a, 'input>)>,
    /// Group names and elements by id.
    groups: HashMap<&'a str, (&'a str, Node<'a, 'input>)>,
    /// Groups expanded so far, by id: the count field's name and the members.
    expanded: HashMap<&'a str, (String, Vec<Member>)>,
    /// The ids of the groups being expanded, outermost first.
    expanding: Vec<&'a str>,
    /// The StandardHeader component's id.
    header: Option<&'a str>,
    /// The StandardTrailer component's id.
    trailer: Option<&'a str>,
}

/// The session-level msgtypes, for repositories without message categories (FIX 4.2).
const SESSION_MSG_TYPES: [&str; 7] = ["0", "1", "2", "3", "4", "5", "A"];

impl<'a, 'input> Parser<'a, 'input> {
    fn parse(mut self, root: Node<'a, 'input>) -> Result<Dictionary, Error> {
        let version = self.version(root)?;
        for el in section(root, "datatypes", "datatype") {
            self.datatypes.insert(self.attr(el, "name")?, el.attribute("baseType"));
        }
        let code_sets = self.code_sets(root)?;
        self.fields = self.read_fields(root, &code_sets)?;
        self.tags = self.fields.iter().enumerate().map(|(i, f)| (f.tag, i)).collect();
        // Index components and groups first: refs may point to ones defined later.
        let components = self.index(root, "components", "component")?;
        self.components = components.iter().map(|&(id, name, el)| (id, (name, el))).collect();
        self.header = components.iter().find(|c| c.1 == "StandardHeader").map(|c| c.0);
        self.trailer = components.iter().find(|c| c.1 == "StandardTrailer").map(|c| c.0);
        let groups = self.index(root, "groups", "group")?;
        self.groups = groups.iter().map(|&(id, name, el)| (id, (name, el))).collect();
        for &(id, _, el) in &groups {
            self.group(el, id, false)?;
        }
        let (mut header, mut trailer) = (Vec::new(), Vec::new());
        let mut list = Vec::new();
        let mut nodes = Vec::new();
        for &(id, name, el) in &components {
            let members = self.members(el, false)?;
            if Some(id) == self.header {
                header = members;
            } else if Some(id) == self.trailer {
                trailer = members;
            } else {
                list.push(Component { name: name.to_string(), members });
                nodes.push(el);
            }
        }
        if let Some((i, message)) = load::component_cycle(&[], &list) {
            return Err(self.error(nodes[i], message));
        }
        let messages = self.messages(root)?;
        let fields = std::mem::take(&mut self.fields);
        Ok(Dictionary::new(version, header, trailer, fields, list, messages))
    }

    /// The `item`s of the section `section_name`, in order, as (id, name, element), checking that
    /// ids and names are unique.
    fn index(
        &self,
        root: Node<'a, 'input>,
        section_name: &str,
        item: &'static str,
    ) -> Result<Vec<(&'a str, &'a str, Node<'a, 'input>)>, Error> {
        let mut items = Vec::new();
        let mut names = HashSet::new();
        let mut ids: HashMap<&str, &str> = HashMap::new();
        for el in section(root, section_name, item) {
            self.check_scenario(el)?;
            let name = self.attr(el, "name")?;
            let id = self.attr(el, "id")?;
            if !names.insert(name) {
                return Err(self.error(el, format!("{item} {name} is defined twice")));
            }
            if let Some(other) = ids.insert(id, name) {
                return Err(self.error(el, format!("{item} id {id} is both {other} and {name}")));
            }
            items.push((id, name, el));
        }
        Ok(items)
    }

    fn messages(&mut self, root: Node<'a, 'input>) -> Result<Vec<Message>, Error> {
        let has_categories = section(root, "messages", "message").any(|el| el.attribute("category").is_some());
        let mut messages = Vec::new();
        let mut names = HashSet::new();
        let mut msg_types: HashMap<&str, &str> = HashMap::new();
        for el in section(root, "messages", "message") {
            self.check_scenario(el)?;
            let name = self.attr(el, "name")?;
            let msg_type = self.attr(el, "msgType")?;
            if !names.insert(name) {
                return Err(self.error(el, format!("message {name} is defined twice")));
            }
            if let Some(other) = msg_types.insert(msg_type, name) {
                return Err(self.error(el, format!("msgtype {msg_type} is both {other} and {name}")));
            }
            let admin = match has_categories {
                true => el.attribute("category") == Some("Session"),
                false => SESSION_MSG_TYPES.contains(&msg_type),
            };
            let structure = el
                .children()
                .find(|n| is(*n, "structure"))
                .ok_or_else(|| self.error(el, format!("message {name} needs a <structure>")))?;
            let members = self.members(structure, true)?;
            messages.push(Message {
                name: name.to_string(),
                msg_type: msg_type.to_string(),
                category: if admin { Category::Admin } else { Category::App },
                members,
                doc: synopsis(el),
            });
        }
        Ok(messages)
    }

    /// The refs in a component, group or message structure. A message's refs to the header and
    /// trailer are left out: they're the dictionary's.
    fn members(&mut self, container: Node<'a, 'input>, in_message: bool) -> Result<Vec<Member>, Error> {
        let mut members = Vec::new();
        for el in container.children().filter(|n| n.is_element() && n.tag_name().namespace() == Some(NS)) {
            self.check_scenario(el)?;
            match el.tag_name().name() {
                "annotation" => {}
                "numInGroup" if is(container, "group") => {}
                "fieldRef" => {
                    let required = self.required(el)?;
                    let name = self.field_ref(el)?.name.clone();
                    members.push(Member::Field { name, required });
                }
                "groupRef" => {
                    let required = self.required(el)?;
                    let id = self.attr(el, "id")?;
                    members.push(self.group(el, id, required)?);
                }
                "componentRef" => {
                    let required = self.required(el)?;
                    let id = self.attr(el, "id")?;
                    let &(name, _) =
                        self.components.get(id).ok_or_else(|| self.error(el, format!("unknown component id {id}")))?;
                    if Some(id) == self.header || Some(id) == self.trailer {
                        if in_message {
                            continue;
                        }
                        return Err(self.error(el, format!("{name} can only be used in a message")));
                    }
                    members.push(Member::Component { name: name.to_string(), required });
                }
                other => return Err(self.error(el, format!("unexpected <{other}>"))),
            }
        }
        Ok(members)
    }

    /// The group with this id, expanded, as a member; `at` is the element referring to it.
    fn group(&mut self, at: Node, id: &'a str, required: bool) -> Result<Member, Error> {
        if let Some((count, members)) = self.expanded.get(id) {
            let group = Some(self.groups[id].0.to_string());
            return Ok(Member::Group { name: count.clone(), official_name: group, required, members: members.clone() });
        }
        let &(name, el) = self.groups.get(id).ok_or_else(|| self.error(at, format!("unknown group id {id}")))?;
        if let Some(start) = self.expanding.iter().position(|&g| g == id) {
            let mut cycle: Vec<&str> = self.expanding[start..].iter().map(|g| self.groups[g].0).collect();
            cycle.push(name);
            return Err(self.error(el, format!("group {name} includes itself ({})", cycle.join(" -> "))));
        }
        let mut counts = el.children().filter(|n| is(*n, "numInGroup"));
        let count_el = counts.next().ok_or_else(|| self.error(el, format!("group {name} needs a <numInGroup>")))?;
        if let Some(extra) = counts.next() {
            return Err(self.error(extra, format!("group {name} has more than one <numInGroup>")));
        }
        let count = self.field_ref(count_el)?;
        if !matches!(count.ty, FieldType::NumInGroup | FieldType::Int) {
            let message = format!("group {name}'s count field {} must be NumInGroup or int", count.name);
            return Err(self.error(count_el, message));
        }
        let count = count.name.clone();
        self.expanding.push(id);
        let members = self.members(el, false)?;
        self.expanding.pop();
        if members.is_empty() {
            return Err(self.error(el, format!("group {name} has no members")));
        }
        self.expanded.insert(id, (count.clone(), members.clone()));
        Ok(Member::Group { name: count, official_name: Some(name.to_string()), required, members })
    }

    /// The field a `fieldRef` or `numInGroup` refers to.
    fn field_ref(&self, el: Node) -> Result<&Field, Error> {
        let id = self.attr(el, "id")?;
        id.parse()
            .ok()
            .and_then(|tag: u32| self.tags.get(&tag))
            .map(|&i| &self.fields[i])
            .ok_or_else(|| self.error(el, format!("unknown field id {id}")))
    }

    /// Whether a ref's `presence` makes it required.
    fn required(&self, el: Node) -> Result<bool, Error> {
        match el.attribute("presence") {
            None | Some("optional") => Ok(false),
            Some("required") => Ok(true),
            Some(other) => Err(self.error(el, format!("unsupported presence '{other}'"))),
        }
    }

    /// Rejects a definition for a scenario other than the base one.
    fn check_scenario(&self, el: Node) -> Result<(), Error> {
        match el.attribute("scenario") {
            None | Some("base") => Ok(()),
            Some(_) => Err(self.error(el, "scenarios aren't supported yet".into())),
        }
    }

    /// From the repository's `name` (`FIX.4.2`, `FIX.5.0SP2`, `FIX.Latest`, `FIXT.1.1`, `FIXT`)
    /// and `version` (`FIX.4.2_EP310`).
    fn version(&self, root: Node) -> Result<Version, Error> {
        let name = self.attr(root, "name")?;
        let not_a_version = || self.error(root, format!("name '{name}' is not a FIX version"));
        let (protocol, rest) = match name.strip_prefix("FIXT") {
            Some(rest) => (Protocol::Fixt, rest),
            None => (Protocol::Fix, name.strip_prefix("FIX").ok_or_else(not_a_version)?),
        };
        let release = match (protocol, rest) {
            (Protocol::Fixt, "") => Release::Numbered { major: 1, minor: 1, service_pack: 0 },
            (Protocol::Fix, ".Latest") => Release::Latest,
            _ => {
                let rest = rest.strip_prefix('.').ok_or_else(not_a_version)?;
                let (numbers, service_pack) = match rest.split_once("SP") {
                    Some((numbers, sp)) => (numbers, sp.parse().map_err(|_| not_a_version())?),
                    None => (rest, 0),
                };
                let (major, minor) = numbers.split_once('.').ok_or_else(not_a_version)?;
                let major = major.parse().map_err(|_| not_a_version())?;
                let minor = minor.parse().map_err(|_| not_a_version())?;
                Release::Numbered { major, minor, service_pack }
            }
        };
        let mut version = Version { protocol, release, extension_pack: None };
        if let Some(text) = root.attribute("version")
            && let Some((_, ep)) = text.rsplit_once("_EP")
        {
            let ep =
                ep.parse().map_err(|_| self.error(root, format!("version '{text}' has no extension pack number")))?;
            version.extension_pack = Some(ep);
        }
        Ok(version)
    }

    /// The field type a datatype maps to, following `baseType`s until a known one.
    fn field_type(&self, name: &str) -> FieldType {
        let mut current = name;
        let mut seen = HashSet::new();
        loop {
            if let Some(ty) = builtin(current) {
                return ty;
            }
            match self.datatypes.get(current).copied().flatten() {
                Some(base) if seen.insert(current) => current = base,
                _ => return FieldType::Other(name.to_string()),
            }
        }
    }

    /// Code sets by name: each one's type and values.
    fn code_sets(&self, root: Node<'a, 'input>) -> Result<HashMap<&'a str, (FieldType, Vec<Value>)>, Error> {
        let mut code_sets = HashMap::new();
        for el in section(root, "codeSets", "codeSet") {
            self.check_scenario(el)?;
            let name = self.attr(el, "name")?;
            let ty = self.field_type(self.attr(el, "type")?);
            let mut values = Vec::new();
            let mut codes = HashSet::new();
            for code in el.children().filter(|n| is(*n, "code")) {
                let value = self.attr(code, "value")?;
                if !codes.insert(value) {
                    return Err(self.error(code, format!("code set {name} has value {value} twice")));
                }
                values.push(Value {
                    code: value.to_string(),
                    name: Some(self.attr(code, "name")?.to_string()),
                    description: String::new(),
                    doc: synopsis(code),
                });
            }
            if code_sets.insert(name, (ty, values)).is_some() {
                return Err(self.error(el, format!("code set {name} is defined twice")));
            }
        }
        Ok(code_sets)
    }

    fn read_fields(
        &self,
        root: Node<'a, 'input>,
        code_sets: &HashMap<&str, (FieldType, Vec<Value>)>,
    ) -> Result<Vec<Field>, Error> {
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        let mut tags: HashMap<u32, &str> = HashMap::new();
        for el in section(root, "fields", "field") {
            self.check_scenario(el)?;
            let name = self.attr(el, "name")?;
            let id = self.attr(el, "id")?;
            let tag: u32 = id.parse().map_err(|_| self.error(el, format!("field {name} has id '{id}'")))?;
            let type_name = self.attr(el, "type")?;
            if !names.insert(name) {
                return Err(self.error(el, format!("field {name} is defined twice")));
            }
            if let Some(other) = tags.insert(tag, name) {
                return Err(self.error(el, format!("tag {tag} is both {other} and {name}")));
            }
            let (ty, values) = match code_sets.get(type_name) {
                Some((ty, values)) => (ty.clone(), values.clone()),
                None => (self.field_type(type_name), Vec::new()),
            };
            fields.push(Field {
                tag,
                name: name.to_string(),
                ty,
                values,
                doc: synopsis(el),
                deprecated: el.attribute("deprecated").is_some(),
            });
        }
        Ok(fields)
    }

    fn attr(&self, el: Node<'a, 'input>, name: &str) -> Result<&'a str, Error> {
        el.attribute(name).ok_or_else(|| self.error(el, format!("<{}> needs a {name} attribute", el.tag_name().name())))
    }

    fn error(&self, node: Node, message: String) -> Error {
        load::error(self.doc, node, message)
    }
}

/// The `item` elements of the repository's section `name`.
fn section<'a, 'input>(
    root: Node<'a, 'input>,
    name: &str,
    item: &'static str,
) -> impl Iterator<Item = Node<'a, 'input>> {
    root.children().filter(move |n| is(*n, name)).flat_map(|s| s.children()).filter(move |n| is(*n, item))
}

/// The element's SYNOPSIS documentation, as lines: every newline in the files is a hard break
/// (lists, "Format:" lines, notes), so each line is trimmed, runs of spaces and tabs in it are made
/// single spaces, and empty lines are dropped. Several SYNOPSIS elements are separate lines.
fn synopsis(el: Node) -> Option<String> {
    let lines: Vec<String> = el
        .children()
        .filter(|n| is(*n, "annotation"))
        .flat_map(|a| a.children())
        .filter(|n| is(*n, "documentation") && n.attribute("purpose") == Some("SYNOPSIS"))
        .flat_map(|d| {
            let text: String = d.descendants().filter_map(|n| n.text().filter(|_| n.is_text())).collect();
            text.lines()
                .map(|line| line.split([' ', '\t']).filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" "))
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty())
        .collect();
    (!lines.is_empty()).then(|| lines.join("\n"))
}

/// The field type of a standard Orchestra datatype.
fn builtin(name: &str) -> Option<FieldType> {
    Some(match name {
        "int" => FieldType::Int,
        "Length" => FieldType::Length,
        "TagNum" => FieldType::TagNum,
        "SeqNum" => FieldType::SeqNum,
        "NumInGroup" => FieldType::NumInGroup,
        "DayOfMonth" => FieldType::DayOfMonth,
        "float" => FieldType::Float,
        "Qty" => FieldType::Qty,
        "Price" => FieldType::Price,
        "PriceOffset" => FieldType::PriceOffset,
        "Amt" => FieldType::Amt,
        "Percentage" => FieldType::Percentage,
        "char" => FieldType::Char,
        "Boolean" => FieldType::Boolean,
        "String" => FieldType::String,
        "MultipleCharValue" => FieldType::MultipleCharValue,
        "MultipleStringValue" => FieldType::MultipleStringValue,
        "MultipleValueString" => FieldType::MultipleValueString,
        "Country" => FieldType::Country,
        "Currency" => FieldType::Currency,
        "Exchange" => FieldType::Exchange,
        "Language" => FieldType::Language,
        "MonthYear" => FieldType::MonthYear,
        "UTCTimestamp" => FieldType::UtcTimestamp,
        "UTCTimeOnly" => FieldType::UtcTimeOnly,
        "UTCDateOnly" => FieldType::UtcDateOnly,
        "UTCDate" => FieldType::UtcDate,
        "LocalMktDate" => FieldType::LocalMktDate,
        "LocalMktTime" => FieldType::LocalMktTime,
        "TZTimeOnly" => FieldType::TzTimeOnly,
        "TZTimestamp" => FieldType::TzTimestamp,
        "data" => FieldType::Data,
        "XMLData" => FieldType::XmlData,
        "XID" => FieldType::Xid,
        "XIDREF" => FieldType::XidRef,
        _ => return None,
    })
}
