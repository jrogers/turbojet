//! FIX Orchestra repositories (`fixr:repository`), such as the FIX Trading Community's
//! `OrchestraFIX44.xml`.

use std::collections::{HashMap, HashSet};

use roxmltree::{Document, Node};

use crate::load::{self, Error};
use crate::merge;
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

/// Parses and validates a repository, returning it and the groups it defines, expanded. With a
/// `base` (an overlay), refs may also name the base's fields by tag and its components and groups
/// by Orchestra id, and it may not renumber the base's fields or give their tags or its msgtypes
/// other names.
pub(crate) fn parse(doc: &Document, base: Option<&Dictionary>) -> Result<(Dictionary, Vec<Member>), Error> {
    let root = doc.root_element();
    let parser = Parser {
        doc,
        base,
        datatypes: HashMap::new(),
        fields: Vec::new(),
        tags: HashMap::new(),
        components: HashMap::new(),
        groups: HashMap::new(),
        expanded: HashMap::new(),
        expanding: Vec::new(),
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

/// A definition's key, name and elements, one per scenario with the base first.
type Item<'a, 'input> = (&'a str, &'a str, Vec<Node<'a, 'input>>);

struct Parser<'a, 'input> {
    doc: &'a Document<'input>,
    base: Option<&'a Dictionary>,
    /// Datatypes by name, with their base types.
    datatypes: HashMap<&'a str, Option<&'a str>>,
    fields: Vec<Field>,
    /// Indexes into `fields` by tag, which is the field's id.
    tags: HashMap<u32, usize>,
    /// Component names and elements, one per scenario with the base first, by id.
    components: HashMap<&'a str, (&'a str, Vec<Node<'a, 'input>>)>,
    /// Group names and elements, one per scenario with the base first, by id.
    groups: HashMap<&'a str, (&'a str, Vec<Node<'a, 'input>>)>,
    /// Groups expanded so far, by id: the count field's name and the members.
    expanded: HashMap<&'a str, (String, Vec<Member>)>,
    /// The ids of the groups being expanded, outermost first.
    expanding: Vec<&'a str>,
}

/// Whether a component is the header or the trailer, which are the dictionary's own.
fn is_header_or_trailer(name: &str) -> bool {
    name == "StandardHeader" || name == "StandardTrailer"
}

/// The session-level msgtypes, for repositories without message categories (FIX 4.2).
const SESSION_MSG_TYPES: [&str; 7] = ["0", "1", "2", "3", "4", "5", "A"];

impl<'a, 'input> Parser<'a, 'input> {
    fn parse(mut self, root: Node<'a, 'input>) -> Result<(Dictionary, Vec<Member>), Error> {
        let version = self.version(root)?;
        for el in section(root, "datatypes", "datatype") {
            self.datatypes.insert(self.attr(el, "name")?, el.attribute("baseType"));
        }
        let code_sets = self.code_sets(root)?;
        self.fields = self.read_fields(root, &code_sets)?;
        self.tags = self.fields.iter().enumerate().map(|(i, f)| (f.tag, i)).collect();
        // Index components and groups first: refs may point to ones defined later.
        let components = self.index(root, "components", "component", "id")?;
        self.check_ids(&components, "component", |base| &base.ids.components)?;
        self.components = components.iter().map(|(id, name, els)| (*id, (*name, els.clone()))).collect();
        let groups = self.index(root, "groups", "group", "id")?;
        self.check_ids(&groups, "group", |base| &base.ids.groups)?;
        self.groups = groups.iter().map(|(id, name, els)| (*id, (*name, els.clone()))).collect();
        for (id, _, els) in &groups {
            self.group(els[0], id, false)?;
        }
        let (mut header, mut trailer) = (Vec::new(), Vec::new());
        let mut list = Vec::new();
        let mut nodes = Vec::new();
        for (_, name, els) in components {
            let members = self.merged(&els, false)?;
            let el = els[0];
            if name == "StandardHeader" {
                header = members;
            } else if name == "StandardTrailer" {
                trailer = members;
            } else {
                list.push(Component { name: name.to_string(), members });
                nodes.push(el);
            }
        }
        let base_components = self.base.map(Dictionary::components).unwrap_or_default();
        if let Some((i, message)) = load::component_cycle(base_components, &list) {
            return Err(self.error(nodes[i], message));
        }
        let messages = self.messages(root)?;
        let fields = std::mem::take(&mut self.fields);
        let mut dict = Dictionary::new(version, header, trailer, fields, list, messages);
        dict.ids.components =
            self.components.iter().map(|(id, (name, _))| (id.to_string(), name.to_string())).collect();
        dict.ids.groups = self.groups.iter().map(|(id, (name, _))| (id.to_string(), name.to_string())).collect();
        let defined = groups
            .iter()
            .map(|(id, name, _)| {
                let (count, members) = self.expanded[id].clone();
                Member::Group { name: count, official_name: Some(name.to_string()), required: false, members }
            })
            .collect();
        Ok((dict, defined))
    }

    /// Checks that an overlay gives none of the base's component or group ids to another name.
    fn check_ids(
        &self,
        items: &[Item],
        item: &str,
        ids: impl Fn(&Dictionary) -> &HashMap<String, String>,
    ) -> Result<(), Error> {
        let Some(base) = self.base else { return Ok(()) };
        for (id, name, els) in items {
            if let Some(existing) = ids(base).get(*id)
                && existing != name
            {
                return Err(
                    self.error(els[0], format!("{item} id {id} is {name}, but {existing} in the base dictionary"))
                );
            }
        }
        Ok(())
    }

    /// The `item`s of the section `section_name`, in order, as (key, name, elements): `key` is the
    /// attribute that identifies one (`id`, or a message's `msgType`), and the elements are its
    /// scenarios, the base first. Each key has one name and each name one key, and each scenario
    /// of an item is defined once.
    fn index(
        &self,
        root: Node<'a, 'input>,
        section_name: &str,
        item: &'static str,
        key: &str,
    ) -> Result<Vec<Item<'a, 'input>>, Error> {
        let mut items: Vec<(&str, &str, Vec<Node>)> = Vec::new();
        let mut names: HashMap<&str, &str> = HashMap::new();
        let mut keys: HashMap<&str, usize> = HashMap::new();
        for el in section(root, section_name, item) {
            let name = self.attr(el, "name")?;
            let id = self.attr(el, key)?;
            if names.insert(name, id).is_some_and(|other| other != id) {
                return Err(self.error(el, format!("{item} {name} is defined twice")));
            }
            let Some(&i) = keys.get(id) else {
                keys.insert(id, items.len());
                items.push((id, name, vec![el]));
                continue;
            };
            let (_, other, els) = &mut items[i];
            if *other != name {
                let key = if key == "id" { format!("{item} id") } else { key.to_lowercase() };
                return Err(self.error(el, format!("{key} {id} is both {other} and {name}")));
            }
            if els.iter().any(|e| scenario(*e) == scenario(el)) {
                return Err(self.error(el, format!("{item} {name} is defined twice{}", in_scenario(el))));
            }
            els.push(el);
        }
        for (_, _, els) in &mut items {
            els.sort_by_key(|el| scenario(*el) != "base");
        }
        Ok(items)
    }

    /// The members of each scenario of a component (or a message's structures), merged.
    fn merged(&mut self, containers: &[Node<'a, 'input>], in_message: bool) -> Result<Vec<Member>, Error> {
        let mut merged: Option<Vec<Member>> = None;
        for &container in containers {
            let members = self.members(container, in_message)?;
            merged = Some(match merged {
                Some(merged) => union(merged, members),
                None => members,
            });
        }
        Ok(merged.unwrap_or_default())
    }

    fn messages(&mut self, root: Node<'a, 'input>) -> Result<Vec<Message>, Error> {
        let has_categories = section(root, "messages", "message").any(|el| el.attribute("category").is_some());
        let mut messages = Vec::new();
        for (msg_type, name, els) in self.index(root, "messages", "message", "msgType")? {
            let el = els[0];
            if let Some(base) = self.base {
                load::check_message(base, name, msg_type).map_err(|message| self.error(el, message))?;
            }
            let admin = match has_categories {
                true => el.attribute("category") == Some("Session"),
                false => SESSION_MSG_TYPES.contains(&msg_type),
            };
            let structures = els
                .iter()
                .map(|el| {
                    el.children()
                        .find(|n| is(*n, "structure"))
                        .ok_or_else(|| self.error(*el, format!("message {name} needs a <structure>")))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let members = self.merged(&structures, true)?;
            messages.push(Message {
                name: name.to_string(),
                msg_type: msg_type.to_string(),
                category: if admin { Category::Admin } else { Category::App },
                members,
                doc: els.iter().find_map(|el| synopsis(*el)),
            });
        }
        Ok(messages)
    }

    /// The refs in a component, group or message structure. A message's refs to the header and
    /// trailer are left out: they're the dictionary's, as are forbidden ones.
    fn members(&mut self, container: Node<'a, 'input>, in_message: bool) -> Result<Vec<Member>, Error> {
        let mut members = Vec::new();
        for el in container.children().filter(|n| n.is_element() && n.tag_name().namespace() == Some(NS)) {
            let member = match el.tag_name().name() {
                "annotation" => continue,
                "numInGroup" if is(container, "group") => continue,
                "fieldRef" => {
                    let required = self.required(el)?;
                    let name = self.field_ref(el)?.name.clone();
                    required.map(|required| Member::Field { name, required })
                }
                "groupRef" => {
                    let required = self.required(el)?;
                    let id = self.attr(el, "id")?;
                    if let Some((name, els)) = self.groups.get(id) {
                        self.check_ref_scenario(el, name, els)?;
                    }
                    let group = self.group(el, id, required.unwrap_or(false))?;
                    required.map(|_| group)
                }
                "componentRef" => {
                    let required = self.required(el)?;
                    let id = self.attr(el, "id")?;
                    let name = match self.components.get(id) {
                        Some((name, els)) => {
                            self.check_ref_scenario(el, name, els)?;
                            *name
                        }
                        None => self
                            .base
                            .and_then(|base| base.ids.components.get(id))
                            .ok_or_else(|| self.error(el, format!("unknown component id {id}")))?,
                    };
                    if is_header_or_trailer(name) {
                        if in_message {
                            continue;
                        }
                        return Err(self.error(el, format!("{name} can only be used in a message")));
                    }
                    required.map(|required| Member::Component { name: name.to_string(), required })
                }
                other => return Err(self.error(el, format!("unexpected <{other}>"))),
            };
            members.extend(member);
        }
        Ok(members)
    }

    /// Checks that the component or group a ref names has the scenario the ref asks for.
    fn check_ref_scenario(&self, el: Node, name: &str, els: &[Node]) -> Result<(), Error> {
        let wanted = scenario(el);
        match els.iter().any(|e| scenario(*e) == wanted) {
            true => Ok(()),
            false => Err(self.error(el, format!("{name} has no scenario '{wanted}'"))),
        }
    }

    /// The group with this id, expanded and its scenarios merged, as a member; `at` is the element
    /// referring to it.
    fn group(&mut self, at: Node, id: &'a str, required: bool) -> Result<Member, Error> {
        if let Some((count, members)) = self.expanded.get(id) {
            let group = Some(self.groups[id].0.to_string());
            return Ok(Member::Group { name: count.clone(), official_name: group, required, members: members.clone() });
        }
        let Some((name, els)) = self.groups.get(id).cloned() else {
            // The base's, already expanded.
            let group = self.base.and_then(|base| merge::official_group(base, base.ids.groups.get(id)?));
            let mut group = group.ok_or_else(|| self.error(at, format!("unknown group id {id}")))?;
            *required_mut(&mut group) = required;
            return Ok(group);
        };
        if let Some(start) = self.expanding.iter().position(|&g| g == id) {
            let mut cycle: Vec<&str> = self.expanding[start..].iter().map(|g| self.groups[g].0).collect();
            cycle.push(name);
            return Err(self.error(els[0], format!("group {name} includes itself ({})", cycle.join(" -> "))));
        }
        let mut count: Option<String> = None;
        for &el in &els {
            let mut counts = el.children().filter(|n| is(*n, "numInGroup"));
            let count_el = counts.next().ok_or_else(|| self.error(el, format!("group {name} needs a <numInGroup>")))?;
            if let Some(extra) = counts.next() {
                return Err(self.error(extra, format!("group {name} has more than one <numInGroup>")));
            }
            let field = self.field_ref(count_el)?;
            if !matches!(field.ty, FieldType::NumInGroup | FieldType::Int) {
                let message = format!("group {name}'s count field {} must be NumInGroup or int", field.name);
                return Err(self.error(count_el, message));
            }
            if let Some(first) = &count
                && *first != field.name
            {
                let message = format!("group {name} is counted by {first} and{} by {}", in_scenario(el), field.name);
                return Err(self.error(count_el, message));
            }
            count = Some(field.name.clone());
        }
        let count = count.unwrap_or_default();
        self.expanding.push(id);
        let mut members: Option<Vec<Member>> = None;
        for &el in &els {
            let these = self.members(el, false)?;
            let Some(first) = these.first() else {
                return Err(self.error(el, format!("group {name} has no members{}", in_scenario(el))));
            };
            members = Some(match members {
                Some(members) if members[0].name() != first.name() => {
                    let message = format!(
                        "group {name} starts with {} and{} with {}",
                        members[0].name(),
                        in_scenario(el),
                        first.name()
                    );
                    return Err(self.error(el, message));
                }
                Some(members) => union(members, these),
                None => these,
            });
        }
        self.expanding.pop();
        let members = members.unwrap_or_default();
        self.expanded.insert(id, (count.clone(), members.clone()));
        Ok(Member::Group { name: count, official_name: Some(name.to_string()), required, members })
    }

    /// The field a `fieldRef` or `numInGroup` refers to.
    fn field_ref(&self, el: Node) -> Result<&Field, Error> {
        let id = self.attr(el, "id")?;
        id.parse()
            .ok()
            .and_then(|tag: u32| self.tags.get(&tag).map(|&i| &self.fields[i]).or_else(|| self.base?.field_by_tag(tag)))
            .ok_or_else(|| self.error(el, format!("unknown field id {id}")))
    }

    /// Whether a ref's `presence` makes it required, or `None` if it's forbidden. A constant is
    /// required, its value unchecked; an ignored one is optional.
    fn required(&self, el: Node) -> Result<Option<bool>, Error> {
        match el.attribute("presence") {
            None | Some("optional" | "ignored") => Ok(Some(false)),
            Some("required" | "constant") => Ok(Some(true)),
            Some("forbidden") => Ok(None),
            Some(other) => Err(self.error(el, format!("unknown presence '{other}'"))),
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
        let mut code_sets: HashMap<&str, (FieldType, Vec<Value>)> = HashMap::new();
        let mut defined = HashSet::new();
        for el in section(root, "codeSets", "codeSet") {
            let name = self.attr(el, "name")?;
            if !defined.insert((name, scenario(el))) {
                return Err(self.error(el, format!("code set {name} is defined twice{}", in_scenario(el))));
            }
            let ty = self.field_type(self.attr(el, "type")?);
            // A code set's scenarios are one code set, with every scenario's values.
            let (_, values) = code_sets.entry(name).or_insert((ty, Vec::new()));
            let mut codes = HashSet::new();
            for code in el.children().filter(|n| is(*n, "code")) {
                let value = self.attr(code, "value")?;
                if !codes.insert(value) {
                    return Err(self.error(code, format!("code set {name} has value {value} twice")));
                }
                if values.iter().any(|v| v.code == value) {
                    continue;
                }
                values.push(Value {
                    code: value.to_string(),
                    name: Some(self.attr(code, "name")?.to_string()),
                    description: String::new(),
                    doc: synopsis(code),
                    deprecated: code.attribute("deprecated").is_some(),
                });
            }
        }
        Ok(code_sets)
    }

    fn read_fields(
        &self,
        root: Node<'a, 'input>,
        code_sets: &HashMap<&str, (FieldType, Vec<Value>)>,
    ) -> Result<Vec<Field>, Error> {
        let mut fields: Vec<Field> = Vec::new();
        let mut names: HashMap<&str, u32> = HashMap::new();
        let mut tags: HashMap<u32, usize> = HashMap::new();
        let mut defined = HashSet::new();
        for el in section(root, "fields", "field") {
            let name = self.attr(el, "name")?;
            let id = self.attr(el, "id")?;
            let tag: u32 = id.parse().map_err(|_| self.error(el, format!("field {name} has id '{id}'")))?;
            let type_name = self.attr(el, "type")?;
            if let Some(&i) = tags.get(&tag)
                && fields[i].name != name
            {
                return Err(self.error(el, format!("tag {tag} is both {} and {name}", fields[i].name)));
            }
            if names.insert(name, tag).is_some_and(|other| other != tag) || !defined.insert((tag, scenario(el))) {
                return Err(self.error(el, format!("field {name} is defined twice{}", in_scenario(el))));
            }
            let (ty, values) = match code_sets.get(type_name) {
                Some((ty, values)) => (ty.clone(), values.clone()),
                None => (self.field_type(type_name), Vec::new()),
            };
            if let Some(base) = self.base {
                load::check_field(base, name, tag, &ty).map_err(|message| self.error(el, message))?;
            }
            if let Some(&i) = tags.get(&tag) {
                // Another scenario of a field already read: it takes this one's values too.
                let field = &mut fields[i];
                for value in values {
                    if !field.values.iter().any(|v| v.code == value.code) {
                        field.values.push(value);
                    }
                }
                continue;
            }
            tags.insert(tag, fields.len());
            fields.push(Field {
                tag,
                name: name.to_string(),
                ty,
                values,
                allow_other_values: false,
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

/// The scenario an element defines, or a ref names.
fn scenario<'a>(el: Node<'a, '_>) -> &'a str {
    el.attribute("scenario").unwrap_or("base")
}

/// ` in scenario X`, for errors about an element of a scenario other than the base one.
fn in_scenario(el: Node) -> String {
    match scenario(el) {
        "base" => String::new(),
        other => format!(" in scenario {other}"),
    }
}

/// Two scenarios' members of one definition, as one: every member of either, in `a`'s order,
/// with `b`'s others after the member they follow in `b` (and after `a`'s first, which stays
/// first: it delimits a group's entries). A member is required only if both require it, and a
/// group's members are merged the same way.
fn union(mut a: Vec<Member>, b: Vec<Member>) -> Vec<Member> {
    let mut in_both = vec![false; a.len()];
    let mut at = a.len().min(1);
    for mut member in b {
        let same =
            |m: &Member| std::mem::discriminant(m) == std::mem::discriminant(&member) && m.name() == member.name();
        match a.iter().position(same) {
            Some(i) => {
                in_both[i] = true;
                let required = member.required();
                if let (Member::Group { members, .. }, Member::Group { members: others, .. }) = (&mut a[i], member) {
                    *members = union(std::mem::take(members), others);
                }
                *required_mut(&mut a[i]) &= required;
                at = i + 1;
            }
            None => {
                *required_mut(&mut member) = false;
                a.insert(at, member);
                in_both.insert(at, true);
                at += 1;
            }
        }
    }
    for (member, _) in a.iter_mut().zip(in_both).filter(|(_, both)| !both) {
        *required_mut(member) = false;
    }
    a
}

fn required_mut(member: &mut Member) -> &mut bool {
    match member {
        Member::Field { required, .. } | Member::Group { required, .. } | Member::Component { required, .. } => {
            required
        }
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
