//! Dictionary documentation as rustdoc Markdown.

/// Documentation lines as Markdown that rustdoc shows as written: each line ends with a hard
/// break (a trailing `\`) but the last, so the lines stay apart, and whatever Markdown would read
/// as markup is escaped. Lines are trimmed (leading whitespace would make code blocks, and
/// trailing whitespace hard breaks), blank ones are dropped, and control characters are dropped
/// but for whitespace (a tab), which becomes a space. Empty if there's nothing left.
pub(crate) fn markdown(doc: &str) -> String {
    let clean = |text: &str| -> String {
        text.chars().filter_map(|c| if c.is_control() { c.is_whitespace().then_some(' ') } else { Some(c) }).collect()
    };
    let lines: Vec<String> = doc.lines().map(clean).collect();
    lines.iter().map(|l| l.trim()).filter(|l| !l.is_empty()).map(line).collect::<Vec<_>>().join("\\\n")
}

/// One line. Balanced backticks are code spans, kept as written; a stray backtick is escaped.
fn line(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 8);
    let rest = block_start(text, &mut out);
    if text.matches('`').count().is_multiple_of(2) {
        for (i, part) in rest.split('`').enumerate() {
            if i.is_multiple_of(2) {
                inline(part, &mut out);
            } else {
                out.push('`');
                out.push_str(part);
                out.push('`');
            }
        }
    } else {
        inline(rest, &mut out);
    }
    out
}

/// Escapes what would make the line a heading, list item or setext underline (`# `, `- `, `+ `,
/// `---`, `===`, `1. `, `2) `; `>` and `*` are always escaped), and returns the rest of it.
fn block_start<'a>(text: &'a str, out: &mut String) -> &'a str {
    if text.starts_with(['#', '-', '+', '=']) {
        out.push('\\');
        return text;
    }
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    let after = &text[digits..];
    if (1..=9).contains(&digits) && after.starts_with(['.', ')']) && (after.len() == 1 || after[1..].starts_with(' ')) {
        out.push_str(&text[..digits]);
        out.push('\\');
        return after;
    }
    text
}

/// Escapes Markdown punctuation that could be markup: links and footnotes, HTML, emphasis and
/// strikethrough, code, and entities. URLs become autolinks (rustdoc warns about bare ones).
fn inline(text: &str, out: &mut String) {
    let mut i = 0;
    while let Some(c) = text[i..].chars().next() {
        if let Some(url) = url_at(text, i) {
            out.push('<');
            out.push_str(url);
            out.push('>');
            i += url.len();
            continue;
        }
        let entity = c == '&' && is_entity(&text[i + 1..]);
        if entity || matches!(c, '\\' | '[' | ']' | '<' | '>' | '*' | '_' | '~' | '`') {
            out.push('\\');
        }
        out.push(c);
        i += c.len_utf8();
    }
}

/// The `http://` or `https://` URL starting at `i`, if one does: up to whitespace or a quote or
/// angle bracket, less trailing punctuation and any unbalanced closing parenthesis.
fn url_at(text: &str, i: usize) -> Option<&str> {
    let rest = &text[i..];
    let starts_word = text[..i].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
    if !starts_word || !(rest.starts_with("http://") || rest.starts_with("https://")) {
        return None;
    }
    let end = rest.find(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '"' | '\'' | '`')).unwrap_or(rest.len());
    let mut url = &rest[..end];
    loop {
        let unbalanced = url.ends_with(')') && url.matches(')').count() > url.matches('(').count();
        if url.ends_with(['.', ',', ';', ':', '!', '?']) || unbalanced {
            url = &url[..url.len() - 1];
        } else {
            break;
        }
    }
    (!url.ends_with("//")).then_some(url)
}

/// Whether text after a `&` would make it an HTML entity: `amp;`, `#38;`, `#x26;`.
fn is_entity(after: &str) -> bool {
    let name = after.bytes().take_while(|b| b.is_ascii_alphanumeric() || *b == b'#').count();
    name > 0 && after[name..].starts_with(';')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_end_with_hard_breaks() {
        assert_eq!(markdown("One.\nTwo.\nThree."), "One.\\\nTwo.\\\nThree.");
        assert_eq!(markdown("One."), "One.");
    }

    #[test]
    fn lines_are_trimmed_and_empty_ones_dropped() {
        // Leading whitespace would make a line a code block, list item or heading.
        assert_eq!(markdown("One.\n    code\n  - item\n\t# Heading"), "One.\\\ncode\\\n\\- item\\\n\\# Heading");
        // An empty or blank line, last or not, leaves no stray hard break.
        assert_eq!(markdown("One.\n\n  \nTwo.\n \t"), "One.\\\nTwo.");
        assert_eq!(markdown(" \n\t\n"), "");
        assert_eq!(markdown("Two spaces  \nNext"), "Two spaces\\\nNext");
    }

    #[test]
    fn control_characters_are_dropped() {
        assert_eq!(markdown("A\u{7}B\u{0}C"), "ABC");
        assert_eq!(markdown("Tab\tand\rreturn"), "Tab and return");
        assert_eq!(markdown("\u{1b}- item"), "\\- item");
    }

    #[test]
    fn escapes_links_html_and_emphasis() {
        assert_eq!(markdown("Refer to SettlDate[64]"), "Refer to SettlDate\\[64\\]");
        assert_eq!(markdown("Second instance of <NestedParties>."), "Second instance of \\<NestedParties\\>.");
        assert_eq!(markdown("(Qty * Price) * Factor"), "(Qty \\* Price) \\* Factor");
        assert_eq!(markdown("e.g. \"CROSS_2\", _x_"), "e.g. \"CROSS\\_2\", \\_x\\_");
        assert_eq!(markdown("a~~b~~"), "a\\~\\~b\\~\\~");
        assert_eq!(markdown("C:\\dir"), "C:\\\\dir");
    }

    #[test]
    fn escapes_what_would_start_a_block() {
        assert_eq!(markdown("> value\n>= value"), "\\> value\\\n\\>= value");
        assert_eq!(markdown("* As a response"), "\\* As a response");
        assert_eq!(markdown("- one\n+ two\n---\n==="), "\\- one\\\n\\+ two\\\n\\---\\\n\\===");
        assert_eq!(markdown("# Heading"), "\\# Heading");
        assert_eq!(markdown("Order # 5"), "Order # 5");
        assert_eq!(markdown("1. Confirm\n2) Reject\n10. Ten"), "1\\. Confirm\\\n2\\) Reject\\\n10\\. Ten");
        assert_eq!(markdown("4(2)\n1000+ = Reserved\n2.5 percent"), "4(2)\\\n1000+ = Reserved\\\n2.5 percent");
    }

    #[test]
    fn keeps_balanced_code_spans_and_escapes_stray_backticks() {
        assert_eq!(markdown("Use `a_b` or `[c]`"), "Use `a_b` or `[c]`");
        assert_eq!(markdown("It`s a_b"), "It\\`s a\\_b");
    }

    #[test]
    fn urls_become_links() {
        assert_eq!(
            markdown("(i.e. http://www.XYZ.com/research_1.html)"),
            "(i.e. <http://www.XYZ.com/research_1.html>)"
        );
        assert_eq!(markdown("See https://fixtrading.org."), "See <https://fixtrading.org>.");
        assert_eq!(markdown("at www.iata.org."), "at www.iata.org.");
    }

    #[test]
    fn escapes_only_ampersands_that_would_be_entities() {
        assert_eq!(markdown("S&P, Tax & Revenue"), "S&P, Tax & Revenue");
        assert_eq!(markdown("&amp; &#38;"), "\\&amp; \\&#38;");
    }
}
