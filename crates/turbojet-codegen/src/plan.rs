//! Decides every generated name and type: rendering only formats the plan.

use std::collections::{BTreeSet, HashMap, HashSet};

use turbojet_dictionary::{Category, Dictionary, Field, FieldType, Member};

use crate::{Error, docs, naming};

/// A member with components expanded into their fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Flat {
    Field {
        name: String,
        required: bool,
    },
    /// `name` is the count field's; `official_name` the official name, if any.
    Group {
        name: String,
        official_name: Option<String>,
        required: bool,
        members: Vec<Flat>,
    },
}

impl Flat {
    fn name(&self) -> &str {
        match self {
            Self::Field { name, .. } | Self::Group { name, .. } => name,
        }
    }

    fn required(&self) -> bool {
        match self {
            Self::Field { required, .. } | Self::Group { required, .. } => *required,
        }
    }

    /// The same definition, required or not.
    fn same_as(&self, other: &Flat) -> bool {
        match (self, other) {
            (Self::Field { name: a, .. }, Self::Field { name: b, .. }) => a == b,
            (
                Self::Group { name: a, official_name: g, members: m, .. },
                Self::Group { name: b, official_name: h, members: n, .. },
            ) => a == b && g == h && m == n,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Presence {
    Req,
    Opt,
    Group,
    ReqGroup,
}

impl Presence {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Req => "req",
            Self::Opt => "opt",
            Self::Group => "group",
            Self::ReqGroup => "req_group",
        }
    }

    pub(crate) fn is_group(self) -> bool {
        matches!(self, Self::Group | Self::ReqGroup)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Slot {
    pub ident: String,
    pub presence: Presence,
    pub ty: String,
    /// The tag constant: the field's, or the group's NumInGroup field's.
    pub tag: String,
    pub doc: String,
}

/// A group entry or message body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Struct {
    pub name: String,
    pub doc: String,
    pub slots: Vec<Slot>,
}

pub(crate) struct MessageDef {
    pub def: Struct,
    pub msg_type: String,
}

pub(crate) struct EnumDef {
    pub name: String,
    pub doc: String,
    pub variants: Vec<Variant>,
}

pub(crate) struct Variant {
    pub name: String,
    pub code: String,
    /// The value's documentation, as Markdown.
    pub doc: Option<String>,
}

pub(crate) struct Plan {
    /// (constant, dictionary name, tag) for every field, by tag.
    pub tags: Vec<(String, String, u32)>,
    pub enums: Vec<EnumDef>,
    pub groups: Vec<Struct>,
    pub messages: Vec<MessageDef>,
}

/// How to plan: see [`Generator`](crate::Generator)'s options.
#[derive(Default)]
pub(crate) struct Options<'a> {
    /// Messages to leave out.
    pub skip: &'a [String],
    /// Copy the dictionary's documentation into doc comments.
    pub docs: bool,
    /// Make every enumerated field lenient (`Code<E>`), not just those in `lenient`.
    pub lenient_all: bool,
    /// Enumerated fields to make lenient, by name.
    pub lenient: &'a [String],
}

/// Plans the code for `dict`'s application messages. With `docs`, the dictionary's documentation
/// goes into doc comments after our own `Name(tag).` lines.
pub(crate) fn build(dict: &Dictionary, options: &Options) -> Result<Plan, Error> {
    let Options { skip, docs, lenient_all, lenient } = *options;
    let mut tags: Vec<(String, String, u32)> = Vec::new();
    let mut constants: HashMap<String, &str> = HashMap::new();
    for field in dict.fields() {
        let constant = naming::constant(&field.name);
        if let Some(other) = constants.insert(constant.clone(), &field.name) {
            return Err(Error(format!("fields {other} and {} would both be {constant}", field.name)));
        }
        tags.push((constant, field.name.clone(), field.tag));
    }
    tags.sort_by_key(|&(_, _, tag)| tag);

    let messages: Vec<_> = dict
        .messages()
        .iter()
        .filter(|m| m.category == Category::App && !skip.contains(&m.name))
        .map(|m| {
            let mut flat = Vec::new();
            flatten(dict, &m.members, true, &mut flat);
            (m, flat)
        })
        .collect();
    if let Some((m, _)) = messages.iter().find(|(m, _)| naming::type_in_use(&m.name)) {
        return Err(Error(format!("message {} would shadow a type generated code uses", m.name)));
    }
    let message_names: HashSet<&str> = messages.iter().map(|(m, _)| m.name.as_str()).collect();

    // Enums for the fields generated messages use, in dictionary order. One named like a message,
    // or a type generated code uses, gets `Code`.
    let mut used = BTreeSet::new();
    for (_, flat) in &messages {
        used_fields(flat, &mut used);
    }
    let mut enums: Vec<EnumDef> = Vec::new();
    let mut enum_types: HashMap<&str, String> = HashMap::new();
    for field in dict.fields().iter().filter(|f| used.contains(f.name.as_str()) && is_enum(f)) {
        let mut def = enum_def(field, docs)?;
        if message_names.contains(def.name.as_str()) || naming::type_in_use(&def.name) {
            def.name.push_str("Code");
        }
        enum_types.insert(&field.name, def.name.clone());
        enums.push(def);
    }
    if let Some(name) = lenient.iter().find(|name| !enum_types.contains_key(name.as_str())) {
        return Err(Error(format!(
            "{name} is not an enumerated field of the generated messages, so it can't be lenient"
        )));
    }
    let enum_names: HashSet<&str> = enums.iter().map(|e| e.name.as_str()).collect();

    // Distinct group definitions, in first-use order, with the first message using each.
    let mut defs: Vec<GroupDef> = Vec::new();
    for (m, flat) in &messages {
        collect_groups(flat, &m.name, &mut defs);
    }
    let group_names = group_names(&defs, &enum_names, &message_names)?;
    let mut taken: HashSet<&str> = HashSet::new();
    for name in enums.iter().map(|e| e.name.as_str()).chain(defs.iter().map(|d| group_names[&d.key()].as_str())) {
        if !taken.insert(name) || message_names.contains(name) {
            return Err(Error(format!("two types would both be named {name}")));
        }
    }

    // A field repeated in one struct (through components, say) keeps its first slot, required if
    // any occurrence is: parsing takes a tag's first occurrence, so a second slot would stay empty.
    let slots = |owner: &str, members: &[Flat]| -> Result<Vec<Slot>, Error> {
        let mut out: Vec<Slot> = Vec::new();
        let mut firsts: Vec<&Flat> = Vec::new();
        for member in members {
            if let Some(i) = firsts.iter().position(|f| f.name() == member.name()) {
                if !firsts[i].same_as(member) {
                    return Err(Error(format!("{owner} has two different {}", member.name())));
                }
                if member.required() {
                    out[i].presence = if out[i].presence.is_group() { Presence::ReqGroup } else { Presence::Req };
                }
                continue;
            }
            let slot = match member {
                Flat::Field { name, required } => {
                    let field = dict.field(name).expect("the loader checks references");
                    let ty = match enum_types.get(name.as_str()) {
                        Some(ty) if lenient_all || lenient.contains(name) => format!("Code<{ty}>"),
                        Some(ty) => ty.clone(),
                        // Passwords print as *** so they can't leak into logs.
                        None if matches!(field.tag, 554 | 925) => "Secret".to_string(),
                        None => rust_type(&field.ty).to_string(),
                    };
                    Slot {
                        ident: naming::field_ident(name),
                        presence: if *required { Presence::Req } else { Presence::Opt },
                        ty,
                        tag: naming::constant(name),
                        doc: field_doc(field, docs),
                    }
                }
                Flat::Group { name, official_name: group, required, members } => {
                    let field = dict.field(name).expect("the loader checks references");
                    Slot {
                        ident: naming::group_field(name),
                        presence: if *required { Presence::ReqGroup } else { Presence::Group },
                        ty: group_names[&(name.as_str(), group.as_deref(), members.as_slice())].clone(),
                        tag: naming::constant(name),
                        doc: field_doc(field, docs),
                    }
                }
            };
            if out.iter().any(|s| s.ident == slot.ident) {
                return Err(Error(format!("{owner} has {} twice", slot.ident)));
            }
            out.push(slot);
            firsts.push(member);
        }
        Ok(out)
    };

    let mut groups = Vec::new();
    for def in &defs {
        let (name, members) = (def.count, def.members);
        let type_name = group_names[&def.key()].clone();
        let mut entry = slots(&type_name, members)?;
        // The first member delimits entries, so every entry must have it: a field is required, and
        // a nested group (whose NumInGroup is then the delimiter) required with an entry.
        let first = entry.first_mut().expect("the loader rejects empty groups");
        first.presence = if first.presence.is_group() { Presence::ReqGroup } else { Presence::Req };
        let tag = dict.field(name).expect("the loader checks references").tag;
        groups.push(Struct { name: type_name, doc: format!("An entry of {name}({tag})."), slots: entry });
    }

    let messages = messages
        .iter()
        .map(|(m, flat)| {
            Ok(MessageDef {
                def: Struct {
                    name: m.name.clone(),
                    doc: with_doc(format!("{}({}).", m.name, m.msg_type), m.doc.as_deref().filter(|_| docs)),
                    slots: slots(&m.name, flat)?,
                },
                msg_type: m.msg_type.clone(),
            })
        })
        .collect::<Result<_, Error>>()?;

    Ok(Plan { tags, enums, groups, messages })
}

/// Expands components. A component's fields are required only if it and they are; a group's
/// members are relative to their entry.
fn flatten(dict: &Dictionary, members: &[Member], required: bool, out: &mut Vec<Flat>) {
    for member in members {
        match member {
            Member::Field { name, required: r } => {
                out.push(Flat::Field { name: name.clone(), required: required && *r })
            }
            Member::Group { name, official_name: group, required: r, members } => {
                let mut entry = Vec::new();
                flatten(dict, members, true, &mut entry);
                out.push(Flat::Group {
                    name: name.clone(),
                    official_name: group.clone(),
                    required: required && *r,
                    members: entry,
                });
            }
            Member::Component { name, required: r } => {
                let component = dict.component(name).expect("the loader checks references");
                flatten(dict, &component.members, required && *r, out);
            }
        }
    }
}

fn used_fields<'a>(members: &'a [Flat], used: &mut BTreeSet<&'a str>) {
    for member in members {
        match member {
            Flat::Field { name, .. } => {
                used.insert(name);
            }
            Flat::Group { members, .. } => used_fields(members, used),
        }
    }
}

/// A group definition: the same count field, official name and members are the same definition.
type GroupKey<'a> = (&'a str, Option<&'a str>, &'a [Flat]);

#[derive(Debug, Clone, Copy)]
struct GroupDef<'a> {
    count: &'a str,
    official: Option<&'a str>,
    members: &'a [Flat],
    /// The first message using it.
    message: &'a str,
}

impl<'a> GroupDef<'a> {
    fn key(&self) -> GroupKey<'a> {
        (self.count, self.official, self.members)
    }
}

fn collect_groups<'a>(members: &'a [Flat], message: &'a str, defs: &mut Vec<GroupDef<'a>>) {
    for member in members {
        if let Flat::Group { name, official_name: group, members, .. } = member {
            let def = GroupDef { count: name, official: group.as_deref(), members, message };
            if !defs.iter().any(|d| d.key() == def.key()) {
                defs.push(def);
            }
            collect_groups(members, message, defs);
        }
    }
}

/// Each definition's struct name: its official name, or named after its count field
/// (`NoAllocs` → `Alloc`), prefixed with its first message when the count field has other
/// unnamed definitions. One that an enum, a message or generated code has gets `Entry`.
fn group_names<'a>(
    defs: &[GroupDef<'a>],
    enum_names: &HashSet<&str>,
    message_names: &HashSet<&str>,
) -> Result<HashMap<GroupKey<'a>, String>, Error> {
    let mut per_count: HashMap<&str, usize> = HashMap::new();
    for def in defs.iter().filter(|d| d.official.is_none()) {
        *per_count.entry(def.count).or_default() += 1;
    }
    let mut names = HashMap::new();
    for (i, def) in defs.iter().enumerate() {
        let mut type_name = match def.official {
            Some(official) => {
                if defs[..i].iter().any(|d| d.official == Some(official)) {
                    return Err(Error(format!("group {official} has two different definitions")));
                }
                naming::official(official)
                    .ok_or_else(|| Error(format!("group name {official} of {} is not a Rust identifier", def.count)))?
            }
            None if per_count[def.count] > 1 => format!("{}{}", def.message, naming::group_type(def.count)),
            None => naming::group_type(def.count),
        };
        if enum_names.contains(type_name.as_str())
            || message_names.contains(type_name.as_str())
            || naming::type_in_use(&type_name)
        {
            type_name.push_str("Entry");
        }
        names.insert(def.key(), type_name);
    }
    Ok(names)
}

/// Fields with values become enums, except booleans and multi-value fields (a list of codes).
fn is_enum(field: &Field) -> bool {
    !field.values.is_empty()
        && !matches!(
            field.ty,
            FieldType::Boolean
                | FieldType::MultipleCharValue
                | FieldType::MultipleStringValue
                | FieldType::MultipleValueString
        )
}

/// `Name(tag).`, then whether the standard deprecates the field, then (with `docs`) its
/// documentation.
fn field_doc(field: &Field, docs: bool) -> String {
    let mut doc = format!("{}({}).", field.name, field.tag);
    if field.deprecated {
        doc.push_str("\n\nDeprecated in the FIX standard.");
    }
    with_doc(doc, field.doc.as_deref().filter(|_| docs))
}

/// A doc with the dictionary's documentation, if any, as a paragraph after it.
fn with_doc(mut doc: String, text: Option<&str>) -> String {
    let text = text.map(docs::markdown).filter(|text| !text.is_empty());
    if let Some(text) = text {
        doc.push_str("\n\n");
        doc.push_str(&text);
    }
    doc
}

fn enum_def(field: &Field, docs: bool) -> Result<EnumDef, Error> {
    let mut variants: Vec<Variant> = Vec::new();
    let taken = |variants: &[Variant], name: &str| variants.iter().any(|v| v.name == name);
    for value in &field.values {
        // The official name, or one from the description. A repeated name gets the code, then a
        // number if that's taken too.
        let mut name = match &value.name {
            Some(official) => naming::official(official).ok_or_else(|| {
                Error(format!("value name {official} of field {} is not a Rust identifier", field.name))
            })?,
            None => naming::variant(&value.description),
        };
        if taken(&variants, &name) {
            name.extend(value.code.chars().filter(char::is_ascii_alphanumeric));
        }
        let with_code = name.clone();
        for n in 2.. {
            if !taken(&variants, &name) {
                break;
            }
            name = format!("{with_code}{n}");
        }
        variants.push(Variant {
            name,
            code: value.code.clone(),
            doc: value.doc.as_deref().filter(|_| docs).map(docs::markdown).filter(|d| !d.is_empty()),
        });
    }
    Ok(EnumDef { name: field.name.clone(), doc: field_doc(field, docs), variants })
}

/// The Rust type for a non-enum field. Dates, times and data stay `String` until core has types
/// for them.
fn rust_type(ty: &FieldType) -> &'static str {
    match ty {
        FieldType::Int
        | FieldType::Length
        | FieldType::TagNum
        | FieldType::SeqNum
        | FieldType::NumInGroup
        | FieldType::DayOfMonth => "i64",
        FieldType::Float
        | FieldType::Qty
        | FieldType::Price
        | FieldType::PriceOffset
        | FieldType::Amt
        | FieldType::Percentage => "Decimal",
        FieldType::UtcTimestamp => "UtcTimestamp",
        FieldType::Boolean => "bool",
        _ => "String",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plans with the given messages left out, and with or without docs.
    fn build(dict: &Dictionary, skip: &[String], docs: bool) -> Result<Plan, Error> {
        super::build(dict, &Options { skip, docs, ..Options::default() })
    }
    use turbojet_dictionary::Dictionary;

    fn plan() -> Plan {
        let dict = Dictionary::from_xml(include_str!("../tests/fixtures/mini.xml")).unwrap();
        build(&dict, &["BusinessMessageReject".to_string()], true).unwrap()
    }

    #[test]
    fn a_group_starting_with_a_group_requires_it() {
        let dict = dict(
            "<message name='StreamAssignmentRequest' msgtype='CC' msgcat='app'>\
             <group name='NoAsgnReqs' required='N'><group name='NoPartyIDs' required='N'>\
             <field name='PartyID' required='N'/></group><field name='Text' required='N'/></group></message>",
            "",
            "<field number='1499' name='NoAsgnReqs' type='NUMINGROUP'/><field number='453' name='NoPartyIDs' \
             type='NUMINGROUP'/><field number='448' name='PartyID' type='STRING'/><field number='58' name='Text' \
             type='STRING'/>",
        );
        let plan = build(&dict, &[], false).unwrap();
        let entry = plan.groups.iter().find(|g| g.name == "AsgnReq").unwrap();
        assert_eq!(slots(entry)[0], ("party_ids", Presence::ReqGroup, "PartyID", "NO_PARTY_IDS"));
        assert_eq!(slots(entry)[1].1, Presence::Opt);
    }

    #[test]
    fn passwords_are_secrets() {
        let dict = dict(
            "<message name='UserRequest' msgtype='BE' msgcat='app'>\
             <field name='Username' required='Y'/><field name='Password' required='N'/>\
             <field name='NewPassword' required='Y'/></message>",
            "",
            "<field number='553' name='Username' type='STRING'/><field number='554' name='Password' type='STRING'/>\
             <field number='925' name='NewPassword' type='STRING'/>",
        );
        let plan = build(&dict, &[], false).unwrap();
        let types: Vec<_> = plan.messages[0].def.slots.iter().map(|s| s.ty.as_str()).collect();
        assert_eq!(types, ["String", "Secret", "Secret"]);
    }

    #[test]
    fn lenient_fields_are_codes() {
        let dict = Dictionary::from_xml(include_str!("../tests/fixtures/mini.xml")).unwrap();
        let skip = ["BusinessMessageReject".to_string()];
        let lenient = |all, fields: &[String]| {
            super::build(&dict, &Options { skip: &skip, lenient_all: all, lenient: fields, ..Options::default() })
        };
        let side = |plan: &Plan| slots(&plan.messages[0].def)[3].2.to_string();
        let role = |plan: &Plan| slots(&plan.groups[0])[1].2.to_string();

        let all = lenient(true, &[]).unwrap();
        assert_eq!((side(&all), role(&all)), ("Code<Side>".to_string(), "Code<PartyRole>".to_string()));
        let one = lenient(false, &["PartyRole".to_string()]).unwrap();
        assert_eq!((side(&one), role(&one)), ("Side".to_string(), "Code<PartyRole>".to_string()));
        // The enums themselves are the same either way.
        assert_eq!(one.enums.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["Side", "PartyRole"]);

        let err = lenient(false, &["Text".to_string()]).map(|_| ()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Text is not an enumerated field of the generated messages, so it can't be lenient"
        );
    }

    fn slots(s: &Struct) -> Vec<(&str, Presence, &str, &str)> {
        s.slots.iter().map(|f| (f.ident.as_str(), f.presence, f.ty.as_str(), f.tag.as_str())).collect()
    }

    /// (variant, code)
    fn variants(e: &EnumDef) -> Vec<(&str, &str)> {
        e.variants.iter().map(|v| (v.name.as_str(), v.code.as_str())).collect()
    }

    #[test]
    fn skips_admin_and_listed_messages() {
        let names: Vec<_> = plan().messages.iter().map(|m| m.def.name.clone()).collect();
        assert_eq!(names, ["NewOrderSingle", "AllocationInstruction", "TradeReport"]);
    }

    #[test]
    fn flattens_components_and_types_fields() {
        let plan = plan();
        let order = &plan.messages[0];
        assert_eq!((order.msg_type.as_str(), order.def.doc.as_str()), ("D", "NewOrderSingle(D)."));
        assert_eq!(
            slots(&order.def),
            [
                ("cl_ord_id", Presence::Req, "String", "CL_ORD_ID"),
                ("party_ids", Presence::Group, "PartyID", "NO_PARTY_IDS"),
                ("allocs", Presence::Group, "NewOrderSingleAlloc", "NO_ALLOCS"),
                ("side", Presence::Req, "Side", "SIDE"),
                ("transact_time", Presence::Req, "UtcTimestamp", "TRANSACT_TIME"),
                ("order_qty", Presence::Opt, "Decimal", "ORDER_QTY"),
                ("text", Presence::Opt, "String", "TEXT"),
            ]
        );
        assert_eq!(order.def.slots[0].doc, "ClOrdID(11).");
    }

    #[test]
    fn names_groups() {
        let plan = plan();
        let names: Vec<_> = plan.groups.iter().map(|g| g.name.as_str()).collect();
        // Different NoAllocs definitions are prefixed with the first message using each; NoSides
        // would be `Side`, which the Side enum has, so it gets `Entry`.
        assert_eq!(names, ["PartyID", "NewOrderSingleAlloc", "AllocationInstructionAlloc", "SideEntry"]);
        assert_eq!(plan.groups[0].doc, "An entry of NoPartyIDs(453).");
    }

    #[test]
    fn group_delimiters_are_required() {
        let plan = plan();
        let party = &plan.groups[0];
        assert_eq!(
            slots(party),
            [
                ("party_id", Presence::Req, "String", "PARTY_ID"),
                ("party_role", Presence::Opt, "PartyRole", "PARTY_ROLE")
            ]
        );
        // A group starting with a component: its first field is the delimiter.
        let side = &plan.groups[3];
        assert_eq!(slots(side)[0], ("side", Presence::Req, "Side", "SIDE"));
        assert_eq!(slots(side)[2], ("odd_lot", Presence::Opt, "bool", "ODD_LOT"));
    }

    #[test]
    fn groups_keep_their_requiredness() {
        let plan = plan();
        let alloc = &plan.messages[1];
        assert_eq!(slots(&alloc.def)[1], ("allocs", Presence::ReqGroup, "AllocationInstructionAlloc", "NO_ALLOCS"));
        let trade = &plan.messages[2];
        assert_eq!(slots(&trade.def)[1], ("sides", Presence::ReqGroup, "SideEntry", "NO_SIDES"));
        // A required group in an optional component is optional.
        let xml = "<fix type='FIX' major='4' minor='2'><messages><message name='M' msgtype='U1' msgcat='app'>\
            <component name='C' required='N'/></message></messages><components><component name='C'>\
            <group name='NoXs' required='Y'><field name='X' required='Y'/></group></component></components>\
            <fields><field number='1' name='X' type='STRING'/><field number='2' name='NoXs' type='NUMINGROUP'/>\
            </fields></fix>";
        let plan = build(&Dictionary::from_xml(xml).unwrap(), &[], true).unwrap();
        assert_eq!(slots(&plan.messages[0].def), [("xs", Presence::Group, "X", "NO_XS")]);
    }

    #[test]
    fn keywords_and_numbers() {
        let plan = plan();
        let trade = &plan.messages[2];
        assert_eq!(slots(&trade.def)[2], ("r#yield", Presence::Opt, "Decimal", "YIELD"));
        assert_eq!(slots(&trade.def)[3], ("last_qty", Presence::Req, "Decimal", "LAST_QTY"));
    }

    #[test]
    fn enums_only_for_fields_messages_use() {
        let plan = plan();
        let names: Vec<_> = plan.enums.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["Side", "PartyRole"], "not MsgType (header only) or OddLot (BOOLEAN)");
        assert_eq!(plan.enums[1].doc, "PartyRole(452).");
        assert_eq!(variants(&plan.enums[1]), [("ExecutingFirm", "1"), ("ClientId", "3")]);
        assert!(plan.enums[1].variants.iter().all(|v| v.doc.is_none()));
    }

    #[test]
    fn tags_cover_every_field_in_tag_order() {
        let plan = plan();
        assert_eq!(plan.tags.len(), 21);
        assert_eq!(plan.tags[0], ("BEGIN_STRING".to_string(), "BeginString".to_string(), 8));
        assert!(plan.tags.windows(2).all(|w| w[0].2 < w[1].2));
    }

    #[test]
    fn duplicate_variants_get_their_code() {
        let xml = "<fix type='FIX' major='4' minor='2'><messages><message name='M' msgtype='U1' msgcat='app'>\
            <field name='A' required='Y'/></message></messages><fields><field number='1' name='A' type='CHAR'>\
            <value enum='1' description='SAME'/><value enum='2' description='SAME'/></field></fields></fix>";
        let dict = Dictionary::from_xml(xml).unwrap();
        let plan = build(&dict, &[], true).unwrap();
        assert_eq!(variants(&plan.enums[0]), [("Same", "1"), ("Same2", "2")]);
    }

    /// An Orchestra repository from its code sets, fields, groups and messages.
    fn orchestra(code_sets: &str, fields: &str, groups: &str, messages: &str) -> Dictionary {
        let xml = format!(
            "<fixr:repository xmlns:fixr='http://fixprotocol.io/2020/orchestra/repository' name='FIX.4.2' \
             version='FIX.4.2_EP310'><fixr:codeSets>{code_sets}</fixr:codeSets><fixr:fields>{fields}</fixr:fields>\
             <fixr:groups>{groups}</fixr:groups><fixr:messages>{messages}</fixr:messages></fixr:repository>"
        );
        Dictionary::from_orchestra(&xml).unwrap_or_else(|e| panic!("{e}"))
    }

    #[test]
    fn official_group_names_are_used_as_they_are() {
        let dict = orchestra(
            "",
            "<fixr:field id='1' name='Account' type='String'/><fixr:field id='78' name='NoAllocs' type='NumInGroup'/>\
             <fixr:field id='79' name='AllocAccount' type='String'/><fixr:field id='80' name='AllocShares' type='Qty'/>\
             <fixr:field id='386' name='NoTradingSessions' type='NumInGroup'/>\
             <fixr:field id='336' name='TradingSessionID' type='String'/>\
             <fixr:field id='136' name='NoMiscFees' type='NumInGroup'/><fixr:field id='137' name='MiscFeeAmt' type='Amt'/>",
            "<fixr:group id='2001' name='PreAllocGrp'><fixr:numInGroup id='78'/><fixr:fieldRef id='79'/></fixr:group>\
             <fixr:group id='2002' name='AllocGrp'><fixr:numInGroup id='78'/>\
             <fixr:fieldRef id='79'/><fixr:fieldRef id='80'/></fixr:group>\
             <fixr:group id='2003' name='TrdgSesGrp'><fixr:numInGroup id='386'/><fixr:fieldRef id='336'/></fixr:group>\
             <fixr:group id='2004' name='String'><fixr:numInGroup id='136'/><fixr:fieldRef id='137'/></fixr:group>",
            "<fixr:message id='1' name='NewOrderSingle' msgType='D'><fixr:structure>\
             <fixr:fieldRef id='1' presence='required'/><fixr:groupRef id='2001'/><fixr:groupRef id='2003'/>\
             </fixr:structure></fixr:message>\
             <fixr:message id='2' name='Allocation' msgType='J'><fixr:structure>\
             <fixr:groupRef id='2002' presence='required'/><fixr:groupRef id='2003'/><fixr:groupRef id='2004'/>\
             </fixr:structure></fixr:message>",
        );
        let plan = build(&dict, &[], true).unwrap();
        let names: Vec<_> = plan.groups.iter().map(|g| g.name.as_str()).collect();
        // Two NoAllocs definitions, but no message prefixes: the official names tell them apart. A
        // name generated code uses still gets `Entry`.
        assert_eq!(names, ["PreAllocGrp", "TrdgSesGrp", "AllocGrp", "StringEntry"]);
        assert_eq!(
            slots(&plan.messages[0].def),
            [
                ("account", Presence::Req, "String", "ACCOUNT"),
                ("allocs", Presence::Group, "PreAllocGrp", "NO_ALLOCS"),
                ("trading_sessions", Presence::Group, "TrdgSesGrp", "NO_TRADING_SESSIONS"),
            ]
        );
        assert_eq!(slots(&plan.messages[1].def)[0], ("allocs", Presence::ReqGroup, "AllocGrp", "NO_ALLOCS"));
        assert_eq!(plan.groups[2].doc, "An entry of NoAllocs(78).");
    }

    #[test]
    fn one_official_group_name_with_two_definitions_is_an_error() {
        let a = [Flat::Field { name: "A".into(), required: true }];
        let b = [Flat::Field { name: "B".into(), required: true }];
        let def = |members| GroupDef { count: "NoXs", official: Some("XGrp"), members, message: "M" };
        let none = HashSet::new();
        let err = group_names(&[def(&a), def(&b)], &none, &none).unwrap_err();
        assert_eq!(err.to_string(), "group XGrp has two different definitions");
        // Another count field is another definition too.
        let other = GroupDef { count: "NoYs", ..def(&a) };
        let err = group_names(&[def(&a), other], &none, &none).unwrap_err();
        assert_eq!(err.to_string(), "group XGrp has two different definitions");
    }

    #[test]
    fn official_value_names_are_used_as_they_are() {
        let dict = orchestra(
            "<fixr:codeSet id='1' name='ACodeSet' type='char'>\
             <fixr:code id='11' name='OrderCancelRequest' value='1'/><fixr:code id='12' name='VWAP' value='2'/>\
             <fixr:code id='13' name='3Day' value='3'/><fixr:code id='14' name='Self' value='4'/>\
             <fixr:code id='15' name='VWAP' value='5'/></fixr:codeSet>",
            "<fixr:field id='1' name='A' type='ACodeSet'/>",
            "",
            "<fixr:message id='1' name='M' msgType='U1'><fixr:structure>\
             <fixr:fieldRef id='1' presence='required'/></fixr:structure></fixr:message>",
        );
        let plan = build(&dict, &[], true).unwrap();
        assert_eq!(
            variants(&plan.enums[0]),
            [("OrderCancelRequest", "1"), ("VWAP", "2"), ("V3Day", "3"), ("Self_", "4"), ("VWAP5", "5")]
        );
    }

    #[test]
    fn official_names_must_be_identifiers() {
        let value = |name: &str| {
            orchestra(
                &format!(
                    "<fixr:codeSet id='1' name='ACodeSet' type='char'><fixr:code id='11' name='{name}' value='1'/></fixr:codeSet>"
                ),
                "<fixr:field id='1' name='A' type='ACodeSet'/>",
                "",
                "<fixr:message id='1' name='M' msgType='U1'><fixr:structure>\
                 <fixr:fieldRef id='1' presence='required'/></fixr:structure></fixr:message>",
            )
        };
        let group = |name: &str| {
            orchestra(
                "",
                "<fixr:field id='78' name='NoAllocs' type='NumInGroup'/><fixr:field id='79' name='AllocAccount' type='String'/>",
                &format!(
                    "<fixr:group id='2001' name='{name}'><fixr:numInGroup id='78'/><fixr:fieldRef id='79'/></fixr:group>"
                ),
                "<fixr:message id='1' name='M' msgType='U1'><fixr:structure><fixr:groupRef id='2001'/>\
                 </fixr:structure></fixr:message>",
            )
        };
        // Keywords are raw identifiers.
        assert_eq!(variants(&build(&value("type"), &[], true).unwrap().enums[0]), [("r#type", "1")]);
        assert_eq!(build(&group("type"), &[], true).unwrap().groups[0].name, "r#type");
        for name in ["Good-Till", "Day.Order", "Two Words"] {
            assert_eq!(error(&value(name)), format!("value name {name} of field A is not a Rust identifier"));
            assert_eq!(error(&group(name)), format!("group name {name} of NoAllocs is not a Rust identifier"));
        }
    }

    #[test]
    fn documents_fields_messages_and_values() {
        let synopsis = |text: &str| {
            format!(
                "<fixr:annotation><fixr:documentation purpose='SYNOPSIS'>{text}</fixr:documentation></fixr:annotation>"
            )
        };
        let dict = orchestra(
            &format!(
                "<fixr:codeSet id='1' name='SideCodeSet' type='char'><fixr:code id='11' name='Buy' value='1'>{}\
                 </fixr:code><fixr:code id='12' name='Sell' value='2'/></fixr:codeSet>",
                synopsis("Buy [or cover].")
            ),
            &format!(
                "<fixr:field id='54' name='Side' type='SideCodeSet'>{}</fixr:field>\
                 <fixr:field id='20' name='ExecTransType' type='char' deprecated='FIX.4.2'>{}</fixr:field>\
                 <fixr:field id='21' name='HandlInst' type='char' deprecated='FIX.4.2'/>\
                 <fixr:field id='78' name='NoAllocs' type='NumInGroup'>{}</fixr:field>\
                 <fixr:field id='79' name='AllocAccount' type='String'/>",
                synopsis("Side of order.\nSee &lt;Parties&gt;"),
                synopsis("Old."),
                synopsis("Number of accounts.")
            ),
            "<fixr:group id='2001' name='PreAllocGrp'><fixr:numInGroup id='78'/><fixr:fieldRef id='79'/></fixr:group>",
            &format!(
                "<fixr:message id='1' name='NewOrderSingle' msgType='D'><fixr:structure>\
                 <fixr:fieldRef id='54' presence='required'/><fixr:fieldRef id='20'/><fixr:fieldRef id='21'/>\
                 <fixr:groupRef id='2001'/></fixr:structure>{}</fixr:message>",
                synopsis("An order.")
            ),
        );
        let plan = build(&dict, &[], true).unwrap();
        let order = &plan.messages[0].def;
        assert_eq!(order.doc, "NewOrderSingle(D).\n\nAn order.");
        let docs: Vec<_> = order.slots.iter().map(|s| s.doc.as_str()).collect();
        assert_eq!(
            docs,
            [
                "Side(54).\n\nSide of order.\\\nSee \\<Parties\\>",
                "ExecTransType(20).\n\nDeprecated in the FIX standard.\n\nOld.",
                "HandlInst(21).\n\nDeprecated in the FIX standard.",
                "NoAllocs(78).\n\nNumber of accounts.",
            ]
        );
        // The enum has its field's docs, and each variant its value's.
        assert_eq!(plan.enums[0].doc, docs[0]);
        let variant_docs: Vec<_> = plan.enums[0].variants.iter().map(|v| v.doc.as_deref()).collect();
        assert_eq!(variant_docs, [Some("Buy \\[or cover\\]."), None]);
        // QuickFIX descriptions aren't docs.
        let quickfix: Vec<_> = self::plan().enums.iter().map(|e| e.doc.clone()).collect();
        assert_eq!(quickfix, ["Side(54).", "PartyRole(452)."]);

        // Without docs, only our own lines are left.
        let plan = build(&dict, &[], false).unwrap();
        let order = &plan.messages[0].def;
        assert_eq!(order.doc, "NewOrderSingle(D).");
        let docs: Vec<_> = order.slots.iter().map(|s| s.doc.as_str()).collect();
        assert_eq!(
            docs,
            [
                "Side(54).",
                "ExecTransType(20).\n\nDeprecated in the FIX standard.",
                "HandlInst(21).\n\nDeprecated in the FIX standard.",
                "NoAllocs(78).",
            ]
        );
        assert_eq!(plan.enums[0].doc, "Side(54).");
        assert!(plan.enums[0].variants.iter().all(|v| v.doc.is_none()));
    }

    /// A dictionary from its messages, components and fields.
    fn dict(messages: &str, components: &str, fields: &str) -> Dictionary {
        let xml = format!(
            "<fix type='FIX' major='4' minor='4'><messages>{messages}</messages>\
             <components>{components}</components><fields>{fields}</fields></fix>"
        );
        Dictionary::from_xml(&xml).unwrap()
    }

    fn error(dict: &Dictionary) -> String {
        match build(dict, &[], true) {
            Ok(_) => panic!("expected an error"),
            Err(e) => e.to_string(),
        }
    }

    #[test]
    fn enums_named_like_messages_get_code() {
        let dict = dict(
            "<message name='Status' msgtype='U1' msgcat='app'><field name='Status' required='Y'/></message>",
            "",
            "<field number='1' name='Status' type='CHAR'><value enum='1' description='UP'/></field>",
        );
        let plan = build(&dict, &[], true).unwrap();
        assert_eq!(plan.enums[0].name, "StatusCode");
        assert_eq!(plan.enums[0].doc, "Status(1).");
        assert_eq!(slots(&plan.messages[0].def), [("status", Presence::Req, "StatusCode", "STATUS")]);
    }

    #[test]
    fn remaining_type_name_clashes_are_errors() {
        let dict = dict(
            "<message name='Status' msgtype='U1' msgcat='app'><field name='Status' required='Y'/></message>\
             <message name='StatusCode' msgtype='U2' msgcat='app'><field name='Status' required='Y'/></message>",
            "",
            "<field number='1' name='Status' type='CHAR'><value enum='1' description='UP'/></field>",
        );
        assert_eq!(error(&dict), "two types would both be named StatusCode");
    }

    #[test]
    fn repeated_fields_keep_the_first_and_any_requiredness() {
        // X arrives through two components, optional first; NoYs twice, directly and by component.
        let dict = dict(
            "<message name='M' msgtype='U1' msgcat='app'><component name='A' required='N'/>\
             <field name='Z' required='Y'/><component name='B' required='Y'/></message>",
            "<component name='A'><field name='X' required='Y'/><group name='NoYs' required='N'>\
             <field name='Y' required='Y'/></group></component>\
             <component name='B'><field name='X' required='Y'/><group name='NoYs' required='Y'>\
             <field name='Y' required='Y'/></group></component>",
            "<field number='1' name='X' type='STRING'/><field number='2' name='Y' type='STRING'/>\
             <field number='3' name='Z' type='STRING'/><field number='4' name='NoYs' type='NUMINGROUP'/>",
        );
        let plan = build(&dict, &[], true).unwrap();
        assert_eq!(
            slots(&plan.messages[0].def),
            [
                ("x", Presence::Req, "String", "X"),
                ("ys", Presence::ReqGroup, "Y", "NO_YS"),
                ("z", Presence::Req, "String", "Z"),
            ]
        );
    }

    #[test]
    fn different_fields_with_one_identifier_are_errors() {
        let idents = dict(
            "<message name='M' msgtype='U1' msgcat='app'><field name='Xs' required='Y'/>\
             <group name='NoXs' required='N'><field name='X' required='Y'/></group></message>",
            "",
            "<field number='1' name='X' type='STRING'/><field number='2' name='Xs' type='STRING'/>\
             <field number='3' name='NoXs' type='NUMINGROUP'/>",
        );
        assert_eq!(error(&idents), "M has xs twice");
        // A group's count field on its own too.
        let groups = dict(
            "<message name='M' msgtype='U1' msgcat='app'><group name='NoXs' required='N'>\
             <field name='X' required='Y'/></group><field name='NoXs' required='N'/></message>",
            "",
            "<field number='1' name='X' type='STRING'/><field number='2' name='Xs' type='STRING'/>\
             <field number='3' name='NoXs' type='NUMINGROUP'/>",
        );
        assert_eq!(error(&groups), "M has two different NoXs");
    }

    #[test]
    fn reserved_type_names_get_a_suffix_or_are_errors() {
        let strings = dict(
            "<message name='M' msgtype='U1' msgcat='app'><field name='String' required='Y'/>\
             <group name='NoOptions' required='N'><field name='X' required='Y'/></group></message>",
            "",
            "<field number='1' name='String' type='CHAR'><value enum='1' description='UP'/></field>\
             <field number='2' name='X' type='STRING'/><field number='3' name='NoOptions' type='NUMINGROUP'/>",
        );
        let plan = build(&strings, &[], true).unwrap();
        assert_eq!(plan.enums[0].name, "StringCode");
        assert_eq!(plan.groups[0].name, "OptionEntry");
        assert_eq!(slots(&plan.messages[0].def)[1], ("options", Presence::Group, "OptionEntry", "NO_OPTIONS"));

        let vec = dict(
            "<message name='Vec' msgtype='U1' msgcat='app'><field name='X' required='Y'/></message>",
            "",
            "<field number='2' name='X' type='STRING'/>",
        );
        assert_eq!(error(&vec), "message Vec would shadow a type generated code uses");
    }

    #[test]
    fn variants_are_unique_however_descriptions_repeat() {
        let dict = dict(
            "<message name='M' msgtype='U1' msgcat='app'><field name='A' required='Y'/></message>",
            "",
            "<field number='1' name='A' type='CHAR'><value enum='1' description='SAME 2'/>\
             <value enum='3' description='SAME'/><value enum='2' description='SAME'/></field>",
        );
        let plan = build(&dict, &[], true).unwrap();
        let names: Vec<_> = variants(&plan.enums[0]).into_iter().map(|(v, _)| v).collect();
        // The third would be Same2 with its code, which the first already is.
        assert_eq!(names, ["Same2", "Same", "Same22"]);
    }

    #[test]
    fn tag_constants_must_be_unique() {
        let dict = dict(
            "<message name='M' msgtype='U1' msgcat='app'><field name='ClOrdID' required='Y'/></message>",
            "",
            "<field number='1' name='ClOrdID' type='STRING'/><field number='2' name='ClOrdId' type='STRING'/>",
        );
        assert_eq!(error(&dict), "fields ClOrdID and ClOrdId would both be CL_ORD_ID");
    }
}
