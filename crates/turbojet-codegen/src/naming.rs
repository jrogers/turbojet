//! Rust names for dictionary names.

const KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn", "else", "enum",
    "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move",
    "mut", "override", "priv", "pub", "ref", "return", "static", "struct", "trait", "true", "try", "type", "typeof",
    "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Keywords that can't be raw identifiers.
const RESERVED: &[&str] = &["crate", "self", "Self", "super"];

/// Names generated types can't take: they would shadow the prelude, or the types the generated
/// modules import, where the generated code or its users refer to them.
const TYPES_IN_USE: &[&str] = &[
    "AsMut",
    "AsRef",
    "Box",
    "Clone",
    "Code",
    "Copy",
    "Decimal",
    "Default",
    "DoubleEndedIterator",
    "Drop",
    "Eq",
    "ExactSizeIterator",
    "Extend",
    "Fn",
    "FnMut",
    "FnOnce",
    "From",
    "FromIterator",
    "Into",
    "IntoIterator",
    "Iterator",
    "Option",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "Result",
    "Secret",
    "Self",
    "Send",
    "Sized",
    "String",
    "Sync",
    "ToOwned",
    "ToString",
    "TryFrom",
    "TryInto",
    "Unpin",
    "UtcTimestamp",
    "Vec",
];

/// Whether a generated type can't be named `name`.
pub(crate) fn type_in_use(name: &str) -> bool {
    TYPES_IN_USE.contains(&name)
}

/// `ClOrdID` → `cl_ord_id`. A word starts at a capital after a lowercase letter or digit, or at an
/// acronym's last capital when a lowercase letter follows (`MDEntry` → `md_entry`), unless that's a
/// lone `s` pluralising the acronym (`PartyIDs` → `party_ids`). A lowercase `id` ending the name or
/// a word, straight after an acronym of two or more capitals, is a word of its own (`IOIid` →
/// `ioi_id`).
pub(crate) fn snake(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let lower_at = |i: usize| chars.get(i).is_some_and(|c| c.is_ascii_lowercase());
    let upper_at = |i: usize| chars.get(i).is_some_and(|c| c.is_ascii_uppercase());
    let id_at = |i: usize| {
        i >= 2
            && chars.get(i) == Some(&'i')
            && chars.get(i + 1) == Some(&'d')
            && upper_at(i - 1)
            && upper_at(i - 2)
            && chars.get(i + 2).is_none_or(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    };
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let plural = chars.get(i + 1) == Some(&'s') && !lower_at(i + 2);
            let word_follows = lower_at(i + 1) && !plural && !id_at(i + 1);
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() || (prev.is_ascii_uppercase() && word_follows) {
                out.push('_');
            }
        } else if id_at(i) {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// A usable identifier: keywords become raw identifiers, or get a `_` when they can't be.
pub(crate) fn ident(name: String) -> String {
    if RESERVED.contains(&name.as_str()) {
        name + "_"
    } else if KEYWORDS.contains(&name.as_str()) {
        format!("r#{name}")
    } else {
        name
    }
}

/// The struct field for a dictionary field.
pub(crate) fn field_ident(name: &str) -> String {
    ident(snake(name))
}

/// The tag constant for a dictionary field.
pub(crate) fn constant(name: &str) -> String {
    snake(name).to_ascii_uppercase()
}

/// An enum variant from a value's description: `SELL_SHORT_EXEMPT` → `SellShortExempt`.
pub(crate) fn variant(description: &str) -> String {
    let mut out: String = description
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            let lower = w.to_ascii_lowercase();
            let mut chars = lower.chars();
            let first = chars.next().expect("non-empty").to_ascii_uppercase();
            std::iter::once(first).chain(chars).collect::<String>()
        })
        .collect();
    if out.is_empty() {
        out = "Unnamed".into();
    }
    official(&out).expect("ASCII letters and digits")
}

/// An enum variant or group struct from an official name, as it is but for the identifier rules:
/// a leading digit gets a `V` (`3Day` → `V3Day`), a keyword becomes a raw identifier (`r#type`),
/// and a reserved word gets a `_`. `None` if it isn't made of ASCII letters, digits and `_`, or is
/// just `_`.
pub(crate) fn official(name: &str) -> Option<String> {
    if name.is_empty() || name == "_" || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return None;
    }
    let mut out = name.to_string();
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, 'V');
    }
    Some(ident(out))
}

/// `NoAllocs` → `Allocs`.
fn strip_no(count_field: &str) -> &str {
    match count_field.strip_prefix("No") {
        Some(rest) if rest.starts_with(|c: char| c.is_ascii_uppercase()) => rest,
        _ => count_field,
    }
}

/// The struct for one entry of a group: `NoAllocs` → `Alloc`, `NoMDEntries` → `MDEntry`.
pub(crate) fn group_type(count_field: &str) -> String {
    let name = strip_no(count_field);
    if let Some(stem) = name.strip_suffix("ies") {
        format!("{stem}y")
    } else if name.ends_with('s') && !name.ends_with("ss") && !name.ends_with("us") {
        name[..name.len() - 1].to_string()
    } else {
        name.to_string()
    }
}

/// The message field holding a group's entries: `NoAllocs` → `allocs`.
pub(crate) fn group_field(count_field: &str) -> String {
    ident(snake(strip_no(count_field)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_splits_words_and_acronyms() {
        assert_eq!(snake("ClOrdID"), "cl_ord_id");
        assert_eq!(snake("MDEntryType"), "md_entry_type");
        assert_eq!(snake("SecurityIDSource"), "security_id_source");
        assert_eq!(snake("IOIID"), "ioiid");
        assert_eq!(snake("Side"), "side");
        assert_eq!(snake("LegPrice2"), "leg_price2");
        assert_eq!(snake("NoPartyIDs"), "no_party_ids");
        assert_eq!(snake("NoNested2PartySubIDs"), "no_nested2_party_sub_ids");
        assert_eq!(snake("IDsSource"), "ids_source");
    }

    #[test]
    fn a_lowercase_id_after_an_acronym_is_its_own_word() {
        assert_eq!(snake("IOIid"), "ioi_id");
        assert_eq!(snake("IOIidSource"), "ioi_id_source");
        assert_eq!(constant("IOIid"), "IOI_ID");
        assert_eq!(snake("MDEntry"), "md_entry");
        assert_eq!(snake("IOIOthSvc"), "ioi_oth_svc");
        assert_eq!(snake("CPRegType"), "cp_reg_type");
        assert_eq!(snake("NoPartyIDs"), "no_party_ids");
        assert_eq!(snake("Valid"), "valid");
        assert_eq!(snake("IDsSource"), "ids_source");
    }

    #[test]
    fn identifiers_avoid_keywords() {
        assert_eq!(field_ident("Yield"), "r#yield");
        assert_eq!(field_ident("Type"), "r#type");
        assert_eq!(field_ident("Self"), "self_");
        assert_eq!(field_ident("ClOrdID"), "cl_ord_id");
    }

    #[test]
    fn constants_are_screaming_snake_case() {
        assert_eq!(constant("ClOrdID"), "CL_ORD_ID");
        assert_eq!(constant("NoAllocs"), "NO_ALLOCS");
        assert_eq!(constant("Yield"), "YIELD");
    }

    #[test]
    fn variants_come_from_descriptions() {
        assert_eq!(variant("SELL_SHORT_EXEMPT"), "SellShortExempt");
        assert_eq!(variant("BUY"), "Buy");
        assert_eq!(variant("Good Till Cancel (GTC)"), "GoodTillCancelGtc");
        assert_eq!(variant("30_DAYS"), "V30Days");
        assert_eq!(variant(""), "Unnamed");
        assert_eq!(variant("SELF"), "Self_");
    }

    #[test]
    fn official_names_are_kept_but_for_the_identifier_rules() {
        assert_eq!(official("UnknownOrderID").as_deref(), Some("UnknownOrderID"));
        assert_eq!(official("VWAP").as_deref(), Some("VWAP"));
        assert_eq!(official("3Day").as_deref(), Some("V3Day"));
        assert_eq!(official("Self").as_deref(), Some("Self_"));
        assert_eq!(official("type").as_deref(), Some("r#type"));
        assert_eq!(official("_Pre_Alloc2").as_deref(), Some("_Pre_Alloc2"));
    }

    #[test]
    fn official_names_that_arent_identifiers_are_none() {
        for name in ["Good-Till", "a.b", "Two Words", "", "_", "Caf\u{e9}", "r#type"] {
            assert_eq!(official(name), None, "{name}");
        }
    }

    #[test]
    fn group_names_drop_no_and_the_plural() {
        assert_eq!(group_type("NoAllocs"), "Alloc");
        assert_eq!(group_type("NoMDEntries"), "MDEntry");
        assert_eq!(group_type("NoPartyIDs"), "PartyID");
        assert_eq!(group_type("NoDlvyInst"), "DlvyInst");
        assert_eq!(group_type("NoStatus"), "Status");
        assert_eq!(group_type("NoAddress"), "Address");
        assert_eq!(group_type("Nothing"), "Nothing");
        assert_eq!(group_field("NoAllocs"), "allocs");
        assert_eq!(group_field("NoMDEntries"), "md_entries");
        assert_eq!(group_field("NoPartyIDs"), "party_ids");
    }
}
