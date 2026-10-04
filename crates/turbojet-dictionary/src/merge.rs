use std::path::Path;

use crate::load::{self, Error};
use crate::model::{Dictionary, Member, Message, Protocol};

impl Dictionary {
    /// Applies a venue's QuickFIX-format dictionary, whose names may refer to this one's (whatever
    /// format this one was loaded from). Orchestra overlays aren't supported yet.
    ///
    /// New fields, components and messages are added. An existing field gains the venue's enum
    /// values and takes its type, and an existing value takes the venue's description where it
    /// gives one; the field's and its values' official names and documentation are kept, since a
    /// QuickFIX dictionary has none. An existing component or message is replaced whole, except
    /// that a message keeps its documentation. A header or trailer in the venue replaces this
    /// one's.
    ///
    /// A QuickFIX group has no official name; where the venue re-lists a message or component
    /// (or brings a header or trailer) with a group that's the same as exactly one of this
    /// dictionary's official groups (Orchestra's `PreAllocGrp`) — the same count field, and the
    /// same members, in order, with the same requiredness, however nested — it takes that official
    /// name, as do its nested groups. A group the venue changed, or one matching several official
    /// groups, stays unnamed (its nested groups are named by the same rule).
    ///
    /// Renumbering a field, giving one of its tags another name, giving a message's msgtype to
    /// another message, or making a field that counts one of this dictionary's groups something
    /// other than NUMINGROUP or INT, or making a component include itself, is an error, and on
    /// error nothing is changed.
    ///
    /// # Errors
    ///
    /// If the XML doesn't parse or the venue's dictionary isn't valid, or for any of the conflicts
    /// above; on error nothing is changed.
    pub fn merge_xml(&mut self, xml: &str) -> Result<(), Error> {
        let venue = load::parse(xml, Some(self))?;
        self.apply(venue);
        Ok(())
    }

    /// [`merge_xml`](Self::merge_xml) with a file.
    ///
    /// # Errors
    ///
    /// If the file can't be read, or as [`merge_xml`](Self::merge_xml), naming the file.
    pub fn merge_file(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        let xml = load::read(path)?;
        self.merge_xml(&xml).map_err(|e| e.in_file(path))
    }

    /// A FIX 5.0+ application dictionary with the header and trailer of its transport (FIXT.1.1),
    /// plus the transport's fields and components that this dictionary lacks. The transport must
    /// be a FIXT dictionary.
    ///
    /// # Errors
    ///
    /// If `transport` isn't a FIXT dictionary.
    pub fn with_transport(mut self, transport: &Dictionary) -> Result<Self, Error> {
        if transport.version.protocol != Protocol::Fixt {
            let message = format!("the transport must be a FIXT dictionary, not {}", transport.version);
            return Err(Error { path: None, line: None, message });
        }
        self.header = transport.header.clone();
        self.trailer = transport.trailer.clone();
        for field in transport.fields() {
            if self.field(&field.name).is_none() {
                self.fields.push(field.clone());
            }
        }
        for component in transport.components() {
            if self.component(&component.name).is_none() {
                self.components.push(component.clone());
            }
        }
        self.reindex();
        Ok(self)
    }

    fn apply(&mut self, mut venue: Dictionary) {
        let official = official_groups(self);
        let sections = venue.components.iter_mut().map(|c| &mut c.members);
        let sections = sections.chain(venue.messages.iter_mut().map(|m| &mut m.members));
        for members in sections.chain([&mut venue.header, &mut venue.trailer]) {
            name_groups(members, &official);
        }
        for field in venue.fields {
            match self.fields.iter_mut().find(|f| f.name == field.name) {
                Some(existing) => {
                    existing.ty = field.ty;
                    if field.doc.is_some() {
                        existing.doc = field.doc;
                    }
                    for value in field.values {
                        match existing.values.iter_mut().find(|v| v.code == value.code) {
                            Some(v) => {
                                if !value.description.is_empty() {
                                    v.description = value.description;
                                }
                                if value.name.is_some() {
                                    v.name = value.name;
                                }
                                if value.doc.is_some() {
                                    v.doc = value.doc;
                                }
                            }
                            None => existing.values.push(value),
                        }
                    }
                }
                None => self.fields.push(field),
            }
        }
        for component in venue.components {
            match self.components.iter_mut().find(|c| c.name == component.name) {
                Some(existing) => *existing = component,
                None => self.components.push(component),
            }
        }
        for message in venue.messages {
            match self.messages.iter_mut().find(|m| m.name == message.name) {
                Some(existing) => {
                    let doc = message.doc.or_else(|| existing.doc.take());
                    *existing = Message { doc, ..message };
                }
                None => self.messages.push(message),
            }
        }
        if !venue.header.is_empty() {
            self.header = venue.header;
        }
        if !venue.trailer.is_empty() {
            self.trailer = venue.trailer;
        }
        self.reindex();
    }
}

/// An official group definition: the count field, the official name and the members.
type Official<'a> = (&'a str, &'a str, &'a [Member]);

/// Every official group definition in `dict`, however nested.
fn official_groups(dict: &Dictionary) -> Vec<Official<'_>> {
    fn collect<'a>(members: &'a [Member], out: &mut Vec<Official<'a>>) {
        for member in members {
            if let Member::Group { name, official_name, members, .. } = member {
                if let Some(group) = official_name {
                    out.push((name, group, members));
                }
                collect(members, out);
            }
        }
    }
    let mut out = Vec::new();
    let sections = dict.components.iter().map(|c| &c.members).chain(dict.messages.iter().map(|m| &m.members));
    for members in sections.chain([&dict.header, &dict.trailer]) {
        collect(members, &mut out);
    }
    out
}

/// Gives each unnamed group the official name of the one official group it's the same as (see
/// [`same_members`]), with its members, and so their groups' names. A group that matches none, or
/// several official groups, stays unnamed, and its own groups are named by the same rule.
fn name_groups(members: &mut [Member], official: &[Official]) {
    for member in members {
        let Member::Group { name, official_name: group @ None, members, .. } = member else { continue };
        let mut matches =
            official.iter().filter(|(count, _, official)| count == name && same_members(members, official));
        match matches.next() {
            Some(&(_, first, definition)) if matches.all(|&(_, other, _)| other == first) => {
                *group = Some(first.to_string());
                *members = definition.to_vec();
            }
            _ => name_groups(members, official),
        }
    }
}

/// The same members: kind, name and requiredness, in order, and for groups, their members, by the
/// same rule; nested groups' official names aside. Requiredness counts, as it does for generated
/// code: a group whose members are required differently is a different definition.
fn same_members(a: &[Member], b: &[Member]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|pair| match pair {
            (Member::Field { name: a, required: r }, Member::Field { name: b, required: s })
            | (Member::Component { name: a, required: r }, Member::Component { name: b, required: s }) => {
                a == b && r == s
            }
            (
                Member::Group { name: a, required: r, members: m, .. },
                Member::Group { name: b, required: s, members: n, .. },
            ) => a == b && r == s && same_members(m, n),
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use crate::model::*;

    fn version() -> Version {
        let release = Release::Numbered { major: 4, minor: 2, service_pack: 0 };
        Version { protocol: Protocol::Fix, release, extension_pack: None }
    }

    fn value(code: &str, name: Option<&str>, description: &str, doc: Option<&str>) -> Value {
        Value {
            code: code.into(),
            name: name.map(Into::into),
            description: description.into(),
            doc: doc.map(Into::into),
        }
    }

    fn side(doc: Option<&str>, values: Vec<Value>) -> Field {
        Field { tag: 54, name: "Side".into(), ty: FieldType::Char, values, doc: doc.map(Into::into), deprecated: false }
    }

    fn dict(field: Field) -> Dictionary {
        Dictionary::new(version(), Vec::new(), Vec::new(), vec![field], Vec::new(), Vec::new())
    }

    /// Official names and docs, as an Orchestra base has them.
    fn base() -> Dictionary {
        dict(side(
            Some("Side of order."),
            vec![value("1", Some("Buy"), "", Some("Buy.")), value("2", Some("Sell"), "", Some("Sell."))],
        ))
    }

    #[test]
    fn keeps_names_and_docs_the_venue_leaves_out() {
        let mut dict = base();
        dict.merge_xml(
            "<fix><fields><field number='54' name='Side' type='CHAR'><value enum='2' description='SELL_ALL'/>\
             <value enum='5' description='SELL_SHORT'/></field></fields></fix>",
        )
        .unwrap();
        let side = dict.field("Side").unwrap();
        assert_eq!(side.doc.as_deref(), Some("Side of order."));
        // A QuickFIX description sits beside the official docs rather than replacing them.
        assert_eq!(
            side.values,
            [
                value("1", Some("Buy"), "", Some("Buy.")),
                value("2", Some("Sell"), "SELL_ALL", Some("Sell.")),
                value("5", None, "SELL_SHORT", None)
            ]
        );
    }

    #[test]
    fn takes_names_and_docs_the_venue_gives() {
        let mut merged = base();
        merged.apply(dict(side(Some("Venue side."), vec![value("1", Some("VenueBuy"), "", Some("Venue buy."))])));
        let side = merged.field("Side").unwrap();
        assert_eq!(side.doc.as_deref(), Some("Venue side."));
        assert_eq!(
            side.values,
            [value("1", Some("VenueBuy"), "", Some("Venue buy.")), value("2", Some("Sell"), "", Some("Sell."))]
        );
    }
}
