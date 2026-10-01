//! Generating from the official Orchestra files in the repository's dictionaries/.

use std::collections::HashSet;
use std::path::Path;

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};
use turbojet_codegen::{Generator, Modules};
use turbojet_dictionary::Dictionary;

fn load(name: &str) -> Dictionary {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dictionaries/orchestra").join(name);
    Dictionary::load(path).unwrap_or_else(|e| panic!("{e}"))
}

/// The code for `dict`, with its documentation.
fn modules(dict: &Dictionary) -> Modules {
    Generator::new(dict).with_docs(true).modules().unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn documentation_is_left_out_by_default() {
    let dict = load("OrchestraFIX42.xml");
    let with = modules(&dict).messages;
    let without = Generator::new(&dict).modules().unwrap().messages;
    // ClOrdID's synopsis.
    let synopsis = "Unique identifier for Order as assigned by institution";
    assert!(with.contains(synopsis));
    assert!(!without.contains(synopsis));
    assert!(without.contains("        /// ClOrdID(11).\n        cl_ord_id: req String = CL_ORD_ID,"), "{without}");
}

/// The `fix_message!` for the message named `name`.
fn message<'a>(messages: &'a str, name: &str) -> &'a str {
    let head = format!("    {name} / ");
    messages.split("turbojet::fix_message!").find(|m| m.contains(&head)).unwrap_or_else(|| panic!("no {name}"))
}

#[test]
fn a_relisted_message_keeps_the_official_groups() {
    // The usual way to add a venue field: re-list the message with it.
    let venue = "<fix><fields><field number='5001' name='VenueTag' type='STRING'/></fields>\
         <messages><message name='NewOrderSingle' msgtype='D' msgcat='app'>\
         <field name='ClOrdID' required='Y'/><field name='HandlInst' required='Y'/>\
         <group name='NoAllocs' required='N'><field name='AllocAccount' required='N'/>\
         <field name='AllocShares' required='N'/></group>\
         <group name='NoTradingSessions' required='N'><field name='TradingSessionID' required='N'/></group>\
         <field name='Symbol' required='Y'/><field name='Side' required='Y'/><field name='TransactTime' required='Y'/>\
         <field name='OrdType' required='Y'/><field name='VenueTag' required='N'/></message></messages></fix>";
    let base = load("OrchestraFIX42.xml");
    let mut merged = base.clone();
    merged.merge_xml(venue).unwrap();
    let (base, merged) = (modules(&base), modules(&merged));
    assert!(merged.groups == base.groups, "the venue's groups are generated alongside the official ones");
    let order = message(&merged.messages, "NewOrderSingle");
    for slot in ["allocs: group PreAllocGrp", "trading_sessions: group TrdgSesGrp", "venue_tag: opt String"] {
        assert!(order.contains(slot), "{slot}");
    }
}

/// Every doc comment in `source`, its lines joined: the Markdown rustdoc reads.
fn doc_comments(source: &str) -> Vec<String> {
    let mut comments = Vec::new();
    let mut lines: Vec<&str> = Vec::new();
    for line in source.lines() {
        match line.trim_start().strip_prefix("///") {
            Some(text) => lines.push(text.strip_prefix(' ').unwrap_or(text)),
            None if !lines.is_empty() => comments.push(std::mem::take(&mut lines).join("\n")),
            None => {}
        }
    }
    comments
}

/// A doc comment's paragraphs as rustdoc shows them, hard breaks as newlines, or the first thing
/// in it that's markup other than a hard break, a code span or an autolink.
fn paragraphs(markdown: &str) -> Result<Vec<String>, String> {
    // rustdoc's options.
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;
    let mut out: Vec<String> = Vec::new();
    for event in Parser::new_ext(markdown, options) {
        let last = out.last_mut();
        match (event, last) {
            (Event::Start(Tag::Paragraph), _) => out.push(String::new()),
            (Event::End(TagEnd::Paragraph | TagEnd::Link), _) => {}
            (Event::Start(Tag::Link { link_type: LinkType::Autolink, .. }), _) => {}
            (Event::Text(text), Some(last)) => last.push_str(&text),
            (Event::Code(code), Some(last)) => last.push_str(&format!("`{code}`")),
            (Event::HardBreak, Some(last)) => last.push('\n'),
            (other, _) => return Err(format!("{other:?}")),
        }
    }
    Ok(out)
}

/// Undoes rustdoc's smart punctuation, which changes how text looks rather than what it says.
fn plain(text: &str) -> String {
    text.replace(['\u{2018}', '\u{2019}'], "'")
        .replace(['\u{201c}', '\u{201d}'], "\"")
        .replace('\u{2014}', "---")
        .replace('\u{2013}', "--")
        .replace('\u{2026}', "...")
}

/// What the generator writes itself: `Name(tag).`, `An entry of Name(tag).`, and the rest.
fn generated(paragraph: &str) -> bool {
    let reference = |text: &str| {
        text.strip_suffix(").").and_then(|t| t.split_once('(')).is_some_and(|(name, tag)| {
            !name.is_empty()
                && name.bytes().all(|b| b.is_ascii_alphanumeric())
                && !tag.is_empty()
                && tag.bytes().all(|b| b.is_ascii_alphanumeric())
        })
    };
    reference(paragraph)
        || paragraph.strip_prefix("An entry of ").is_some_and(reference)
        || paragraph == "Deprecated in the FIX standard."
        || paragraph == "With the required fields and groups; optional ones empty."
}

/// Every doc comment generated from an official file reads, in rustdoc, as plain text: the
/// dictionary's documentation as written, line for line, or the generator's own lines.
fn docs_read_as_written(file: &str) {
    let dict = load(file);
    let mut docs: HashSet<String> = HashSet::new();
    for field in dict.fields() {
        docs.extend(field.doc.clone());
        docs.extend(field.values.iter().filter_map(|v| v.doc.clone()));
    }
    docs.extend(dict.messages().iter().filter_map(|m| m.doc.clone()));
    let docs: HashSet<String> = docs.iter().map(|d| plain(d)).collect();

    let m = modules(&dict);
    let mut problems = Vec::new();
    let mut checked = 0;
    for comment in [m.enums, m.groups, m.messages].iter().flat_map(|source| doc_comments(source)) {
        checked += 1;
        match paragraphs(&comment) {
            Ok(paragraphs) => {
                for paragraph in paragraphs.iter().map(|p| plain(p)) {
                    if !generated(&paragraph) && !docs.contains(&paragraph) {
                        problems.push(format!("{comment:?} reads as {paragraph:?}"));
                    }
                }
            }
            Err(markup) => problems.push(format!("{comment:?} has {markup}")),
        }
    }
    assert!(checked > 1000, "{file}: only {checked} doc comments");
    assert!(
        problems.is_empty(),
        "{file}: {} problems:\n{}",
        problems.len(),
        problems[..problems.len().min(20)].join("\n")
    );
}

#[test]
fn fix42_docs_read_as_written() {
    docs_read_as_written("OrchestraFIX42.xml");
}

#[test]
fn fix44_docs_read_as_written() {
    docs_read_as_written("OrchestraFIX44.xml");
}
