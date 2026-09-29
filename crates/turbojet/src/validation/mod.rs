//! Checking inbound messages against a data dictionary (feature `validation`).
//!
//! A [`Validator`] is built once from a [`Dictionary`] and set
//! on a session with [`SessionConfig::with_dictionary`](crate::SessionConfig::with_dictionary).
//! The session then checks every inbound application message before the application sees it, and
//! answers one that fails with a session Reject (35=3) carrying the reason and the offending tag;
//! the message counts as received but isn't delivered. Session-level messages the dictionary
//! defines are checked too, so an unknown tag in a Heartbeat is rejected, except Logon, which the
//! engine checks itself, and Logout, which always completes.
//!
//! Each check can be turned off with [`ValidationOptions`], since counterparties vary in how
//! closely they follow the spec. Corrections made to the dictionary first (such as
//! `Dictionary::make_optional`) apply.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::Arc;

use turbojet_dictionary::{Dictionary, Field, FieldType, Member, Protocol, Release};

use crate::fields::{Decimal, FromFix, MsgType, SessionRejectReason, UtcTimestamp};
use crate::message::{Message, is_header_or_trailer, tags};

#[cfg(test)]
mod tests;

/// A fast hasher for the validator's tables (tags and short codes): FxHash's mixing, as in
/// rustc. The keys come from the dictionary, not the counterparty, so there's no flooding risk
/// that would call for SipHash, the standard library's default.
#[derive(Default)]
struct FxHasher(u64);

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u64(u64::from(byte));
        }
    }

    fn write_u32(&mut self, n: u32) {
        self.write_u64(u64::from(n));
    }

    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

type Fast = BuildHasherDefault<FxHasher>;
type FastMap<K, V> = HashMap<K, V, Fast>;
type FastSet<T> = HashSet<T, Fast>;

/// Which checks a [`Validator`] makes; all on by default, except allowing user-defined tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationOptions {
    /// Reject a MsgType the dictionary doesn't define (SessionRejectReason 11).
    pub unknown_msg_types: bool,
    /// Reject a tag the dictionary doesn't define (0).
    pub undefined_tags: bool,
    /// Reject a tag the dictionary defines but not for this message (2).
    pub tags_not_in_message: bool,
    /// Reject a message missing a required tag, in its body or a group entry (1).
    pub required_tags: bool,
    /// Reject a value that isn't one of an enumerated field's codes (5).
    pub values: bool,
    /// Reject a value not in its field's data format, e.g. letters in a Qty (6).
    pub formats: bool,
    /// Reject a tag that appears more than once outside a group (13).
    pub repeated_tags: bool,
    /// Reject a repeating group whose count doesn't match its entries (16) or whose entries are
    /// out of order (15).
    pub groups: bool,
    /// Accept undefined user-defined tags (5000 and up), which venues often add, when
    /// `undefined_tags` would reject them.
    pub allow_user_defined_tags: bool,
    /// Check session-level messages the dictionary defines (not Logon or Logout), as well as
    /// application messages.
    pub admin_messages: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            unknown_msg_types: true,
            undefined_tags: true,
            tags_not_in_message: true,
            required_tags: true,
            values: true,
            formats: true,
            repeated_tags: true,
            groups: true,
            allow_user_defined_tags: false,
            admin_messages: true,
        }
    }
}

/// Why a message failed validation: what to put in the Reject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invalid {
    /// RefTagID(371): the offending tag, if there is one.
    pub tag: Option<u32>,
    /// SessionRejectReason(373). `None` where the reason was only added after the dictionary's
    /// FIX version (FIX 4.2 has reasons up to 11); the text still says what's wrong.
    pub reason: Option<SessionRejectReason>,
    /// Text(58).
    pub text: String,
}

/// A data dictionary compiled for checking messages; see the [module docs](self).
pub struct Validator {
    messages: FastMap<String, Rules>,
    fields: FastMap<u32, FieldRule>,
    /// Header and trailer tags, which aren't the body's to check.
    envelope: FastSet<u32>,
    /// Whether SessionRejectReason values above 11 exist in this FIX version.
    later_reasons: bool,
    options: ValidationOptions,
}

/// A message body's, or a group entry's, members.
struct Rules {
    /// Member tags, with the group each NumInGroup field introduces.
    members: FastMap<u32, Option<Arc<Group>>>,
    /// Tags that must be present, in dictionary order.
    required: Vec<u32>,
}

struct Group {
    /// The first member's tag, which starts every entry.
    delimiter: u32,
    entry: Rules,
}

struct FieldRule {
    ty: FieldType,
    /// Permitted codes, for enumerated fields.
    codes: Option<FastSet<String>>,
}

impl Validator {
    /// Compiles `dict` for checking messages, with every check on.
    pub fn new(dict: &Dictionary) -> Self {
        let fields = dict.fields().iter().map(|f| (f.tag, FieldRule { ty: f.ty.clone(), codes: codes(f) })).collect();
        let tag = |name: &str| dict.field(name).expect("the loader checks references").tag;
        let mut envelope = FastSet::default();
        for members in [dict.header(), dict.trailer()] {
            let mut flat = Vec::new();
            flatten(dict, members, true, &mut flat);
            collect_tags(&flat, &tag, &mut envelope);
        }
        let messages = dict
            .messages()
            .iter()
            .map(|m| {
                let mut flat = Vec::new();
                flatten(dict, &m.members, true, &mut flat);
                (m.msg_type.clone(), rules(&flat, &tag))
            })
            .collect();
        let later_reasons = match (dict.version.protocol, dict.version.release) {
            (Protocol::Fix, Release::Numbered { major, minor, .. }) => (major, minor) >= (4, 3),
            _ => true,
        };
        Self { messages, fields, envelope, later_reasons, options: ValidationOptions::default() }
    }

    /// The same validator with `options`.
    #[must_use]
    pub fn with_options(mut self, options: ValidationOptions) -> Self {
        self.options = options;
        self
    }

    /// Whether messages of this type are checked: application messages are, and session-level
    /// messages the dictionary defines unless [`ValidationOptions::admin_messages`] is off. Logon
    /// is the engine's to check, and a Logout always completes, so neither is. An application-only
    /// dictionary leaves session-level messages to the engine.
    pub fn applies_to(&self, msg_type: &MsgType) -> bool {
        if !msg_type.is_admin() {
            return true;
        }
        self.options.admin_messages
            && !matches!(msg_type, MsgType::Logon | MsgType::Logout)
            && self.messages.contains_key(msg_type.code())
    }

    /// Checks `msg`, which has already passed the session's own checks (header, sequence, no
    /// empty values).
    pub fn validate(&self, msg: &Message) -> Result<(), Invalid> {
        let msg_type = msg.get(tags::MSG_TYPE).unwrap_or_default();
        let Some(rules) = self.messages.get(msg_type) else {
            if self.options.unknown_msg_types {
                return Err(self.invalid(Some(tags::MSG_TYPE), SessionRejectReason::InvalidMsgType, "Invalid MsgType"));
            }
            return Ok(());
        };
        let fields: Vec<(u32, &str)> = msg.fields().filter(|(tag, _)| !self.in_envelope(*tag)).collect();
        // Few enough tags that a list beats a set.
        let mut seen: Vec<u32> = Vec::with_capacity(fields.len());
        let mut i = 0;
        while i < fields.len() {
            let (tag, value) = fields[i];
            let Some(member) = self.member(rules, tag)? else {
                i += 1;
                continue;
            };
            let repeated = seen.contains(&tag);
            seen.push(tag);
            if repeated && self.options.repeated_tags {
                let text = format!("Tag {tag} appears more than once");
                return Err(self.invalid(Some(tag), SessionRejectReason::TagAppearsMoreThanOnce, &text));
            }
            self.check_value(tag, value)?;
            i = match member {
                Some(group) => self.check_group(&fields, i, group)?,
                None => i + 1,
            };
        }
        self.check_required(rules, &seen)
    }

    fn in_envelope(&self, tag: u32) -> bool {
        self.envelope.contains(&tag) || is_header_or_trailer(tag)
    }

    /// `tag`'s member rule in `rules`: `Ok(None)` for a tag to skip (undefined or foreign but
    /// allowed), an error if it isn't allowed.
    fn member<'r>(&self, rules: &'r Rules, tag: u32) -> Result<Option<&'r Option<Arc<Group>>>, Invalid> {
        if let Some(member) = rules.members.get(&tag) {
            return Ok(Some(member));
        }
        if !self.fields.contains_key(&tag) {
            let user_defined = tag >= 5000 && self.options.allow_user_defined_tags;
            // Session test case 14a: "Invalid tag number" (0), as QuickFIX and QuickFIX/J also
            // answer, rather than "Undefined tag" (3).
            if self.options.undefined_tags && !user_defined {
                return Err(self.invalid(
                    Some(tag),
                    SessionRejectReason::InvalidTagNumber,
                    &format!("Invalid tag number {tag}"),
                ));
            }
        } else if self.options.tags_not_in_message {
            let text = format!("Tag {tag} not defined for this message type");
            return Err(self.invalid(Some(tag), SessionRejectReason::TagNotDefinedForMessageType, &text));
        }
        Ok(None)
    }

    /// Checks the group whose NumInGroup field is at `at`; returns the index after it.
    fn check_group(&self, fields: &[(u32, &str)], at: usize, group: &Group) -> Result<usize, Invalid> {
        let (count_tag, count) = fields[at];
        // A malformed count was reported by check_value when formats are checked.
        let count: usize = count.parse().unwrap_or(0);
        let mut i = at + 1;
        let mut entries = 0;
        while i < fields.len() && fields[i].0 == group.delimiter {
            entries += 1;
            let mut seen: Vec<u32> = Vec::new();
            let mut first = true;
            while i < fields.len() {
                let (tag, value) = fields[i];
                if tag == group.delimiter && !first {
                    break;
                }
                let Some(member) = group.entry.members.get(&tag) else { break };
                if seen.contains(&tag) {
                    if self.options.groups {
                        let text = format!("Tag {tag} repeated within a {count_tag} entry");
                        return Err(self.invalid(
                            Some(tag),
                            SessionRejectReason::RepeatingGroupFieldsOutOfOrder,
                            &text,
                        ));
                    }
                    break;
                }
                seen.push(tag);
                first = false;
                self.check_value(tag, value)?;
                i = match member {
                    Some(nested) => self.check_group(fields, i, nested)?,
                    None => i + 1,
                };
            }
            self.check_required(&group.entry, &seen)?;
        }
        if self.options.groups && entries != count {
            // A member where the delimiter should be is an entry out of order.
            if entries < count
                && let Some(&(tag, _)) = fields.get(i)
                && group.entry.members.contains_key(&tag)
            {
                let text = format!("Entry of group {count_tag} doesn't start with its delimiter {}", group.delimiter);
                return Err(self.invalid(Some(tag), SessionRejectReason::RepeatingGroupFieldsOutOfOrder, &text));
            }
            let text = format!("NumInGroup {count_tag} declares {count} entries but {entries} were found");
            return Err(self.invalid(Some(count_tag), SessionRejectReason::IncorrectNumInGroupCount, &text));
        }
        Ok(i)
    }

    fn check_required(&self, rules: &Rules, seen: &[u32]) -> Result<(), Invalid> {
        if !self.options.required_tags {
            return Ok(());
        }
        match rules.required.iter().find(|tag| !seen.contains(tag)) {
            Some(&tag) => Err(self.invalid(
                Some(tag),
                SessionRejectReason::RequiredTagMissing,
                &format!("Required tag {tag} missing"),
            )),
            None => Ok(()),
        }
    }

    fn check_value(&self, tag: u32, value: &str) -> Result<(), Invalid> {
        let Some(field) = self.fields.get(&tag) else { return Ok(()) };
        if let Some(codes) = &field.codes {
            if !self.options.values {
                return Ok(());
            }
            let multiple = is_multiple(&field.ty);
            let known =
                if multiple { value.split(' ').all(|code| codes.contains(code)) } else { codes.contains(value) };
            if !known {
                let text = format!("Value '{value}' is incorrect (out of range) for tag {tag}");
                return Err(self.invalid(Some(tag), SessionRejectReason::ValueIsIncorrect, &text));
            }
            return Ok(());
        }
        if self.options.formats && !in_format(&field.ty, value) {
            let text = format!("Incorrect data format for value '{value}' of tag {tag}");
            return Err(self.invalid(Some(tag), SessionRejectReason::IncorrectDataFormat, &text));
        }
        Ok(())
    }

    fn invalid(&self, tag: Option<u32>, reason: SessionRejectReason, text: &str) -> Invalid {
        // Reasons above 11 were added in FIX 4.3.
        let known = self.later_reasons || reason.code().parse::<u32>().is_ok_and(|code| code <= 11);
        Invalid { tag, reason: known.then_some(reason), text: text.to_string() }
    }
}

impl fmt::Debug for Validator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Validator")
            .field("messages", &self.messages.len())
            .field("fields", &self.fields.len())
            .field("options", &self.options)
            .finish()
    }
}

/// A member with components expanded, as codegen sees it.
enum Flat {
    Field { name: String, required: bool },
    Group { name: String, required: bool, members: Vec<Flat> },
}

/// Expands components. A component's fields are required only if it and they are; a group's
/// members are relative to their entry.
fn flatten(dict: &Dictionary, members: &[Member], required: bool, out: &mut Vec<Flat>) {
    for member in members {
        match member {
            Member::Field { name, required: r } => {
                out.push(Flat::Field { name: name.clone(), required: required && *r })
            }
            Member::Group { name, required: r, members, .. } => {
                let mut entry = Vec::new();
                flatten(dict, members, true, &mut entry);
                out.push(Flat::Group { name: name.clone(), required: required && *r, members: entry });
            }
            Member::Component { name, required: r } => {
                let component = dict.component(name).expect("the loader checks references");
                flatten(dict, &component.members, required && *r, out);
            }
        }
    }
}

fn rules(members: &[Flat], tag: &impl Fn(&str) -> u32) -> Rules {
    let mut out = Rules { members: FastMap::default(), required: Vec::new() };
    for member in members {
        let (name, required, group) = match member {
            Flat::Field { name, required } => (name, *required, None),
            Flat::Group { name, required, members } => {
                let entry = rules(members, tag);
                let delimiter = match &members[0] {
                    Flat::Field { name, .. } | Flat::Group { name, .. } => tag(name),
                };
                (name, *required, Some(Arc::new(Group { delimiter, entry })))
            }
        };
        let tag = tag(name);
        // A field repeated through two components keeps its first rule.
        if out.members.contains_key(&tag) {
            continue;
        }
        out.members.insert(tag, group);
        if required {
            out.required.push(tag);
        }
    }
    out
}

fn collect_tags(members: &[Flat], tag: &impl Fn(&str) -> u32, out: &mut FastSet<u32>) {
    for member in members {
        match member {
            Flat::Field { name, .. } => {
                out.insert(tag(name));
            }
            Flat::Group { name, members, .. } => {
                out.insert(tag(name));
                collect_tags(members, tag, out);
            }
        }
    }
}

/// The permitted codes of an enumerated field.
fn codes(field: &Field) -> Option<FastSet<String>> {
    (!field.values.is_empty()).then(|| field.values.iter().map(|v| v.code.clone()).collect())
}

fn is_multiple(ty: &FieldType) -> bool {
    matches!(ty, FieldType::MultipleCharValue | FieldType::MultipleStringValue | FieldType::MultipleValueString)
}

/// Whether `value` is in `ty`'s data format. Types without a checkable format pass.
fn in_format(ty: &FieldType, value: &str) -> bool {
    match ty {
        FieldType::Int => i64::from_fix(value).is_ok(),
        FieldType::Length | FieldType::TagNum | FieldType::SeqNum | FieldType::NumInGroup | FieldType::DayOfMonth => {
            u64::from_fix(value).is_ok()
        }
        FieldType::Float
        | FieldType::Qty
        | FieldType::Price
        | FieldType::PriceOffset
        | FieldType::Amt
        | FieldType::Percentage => Decimal::from_fix(value).is_ok(),
        FieldType::Boolean => bool::from_fix(value).is_ok(),
        FieldType::Char => value.chars().count() == 1,
        FieldType::UtcTimestamp => UtcTimestamp::from_fix(value).is_ok(),
        _ => true,
    }
}
