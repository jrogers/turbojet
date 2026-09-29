use crate::load::Error;
use crate::model::{Dictionary, Member};

impl Dictionary {
    /// Makes `member`, a field, group or component reference, optional wherever it appears in the
    /// message or component `owner`, including inside its groups: for dictionaries that mark
    /// something required that counterparties leave out. (FIX 4.2 marks QuoteCancel's
    /// NoQuoteEntries required, though a cancel-all request has no entries; FIX 4.4 corrected it.)
    ///
    /// A member of a component is corrected on the component, and so for every message that
    /// uses it: `make_optional("Instrument", "Symbol")`. Fails if there's no message or component
    /// `owner`, or it has no member `member` of its own.
    pub fn make_optional(&mut self, owner: &str, member: &str) -> Result<(), Error> {
        let members = match self.messages.iter_mut().find(|m| m.name == owner) {
            Some(message) => &mut message.members,
            None => match self.components.iter_mut().find(|c| c.name == owner) {
                Some(component) => &mut component.members,
                None => return Err(error(format!("no message or component {owner}"))),
            },
        };
        if relax(members, member) {
            Ok(())
        } else {
            Err(error(format!(
                "{owner} has no field or group {member} of its own; if it's in a component, correct the component"
            )))
        }
    }
}

/// Makes every member named `name` optional, searching groups too. Whether there was one.
fn relax(members: &mut [Member], name: &str) -> bool {
    let mut found = false;
    for member in members {
        if member.name() == name {
            match member {
                Member::Field { required, .. }
                | Member::Group { required, .. }
                | Member::Component { required, .. } => {
                    *required = false;
                }
            }
            found = true;
        }
        if let Member::Group { members, .. } = member {
            found |= relax(members, name);
        }
    }
    found
}

fn error(message: String) -> Error {
    Error { path: None, line: None, message }
}
