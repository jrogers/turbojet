//! Lists every generated message type, per version, for the `typed` target: a parser for each,
//! read from turbojet-fix's generated sources so the list can't fall behind them.

use std::fmt::Write;
use std::{env, fs};

const VERSIONS: &[(&str, &str)] = &[
    ("FIX.4.0", "fix40"),
    ("FIX.4.1", "fix41"),
    ("FIX.4.2", "fix42"),
    ("FIX.4.3", "fix43"),
    ("FIX.4.4", "fix44"),
    ("FIXT.1.1", "fix50sp2"),
    ("FIXT.1.1", "fix50"),
    ("FIXT.1.1", "fix50sp1"),
];

fn main() {
    let mut out = String::from("pub const VERSIONS: &[(&str, &[Parse])] = &[\n");
    for (begin_string, module) in VERSIONS {
        let path = format!("../../turbojet-fix/src/{module}/messages.rs");
        println!("cargo::rerun-if-changed={path}");
        let source = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        writeln!(out, "    ({begin_string:?}, &[").unwrap();
        // Each message opens with `Name / NameRef / NameView = "MsgType" {` inside `fix_message!`.
        let names: Vec<_> = source.lines().filter_map(message_names).collect();
        assert!(!names.is_empty(), "no messages found in {path}");
        for (name, view) in names {
            // A closure, since the view's lifetime is the message's.
            let view = format!("turbojet_fix::{module}::{view}");
            writeln!(
                out,
                "        |msg| check::<turbojet_fix::{module}::{name}>(msg, {view}::parse_from(msg).err(), \
                 {view}::parse_strict_from(msg).err()),"
            )
            .unwrap();
        }
        writeln!(out, "    ]),").unwrap();
    }
    out.push_str("];\n");
    fs::write(format!("{}/versions.rs", env::var("OUT_DIR").unwrap()), out).unwrap();
}

/// The message and view a line opens, if it's `Name / NameRef / NameView = "MsgType" {`, indented.
fn message_names(line: &str) -> Option<(&str, &str)> {
    let (names, msg_type) = line.trim_start().strip_suffix("\" {")?.split_once(" = \"")?;
    let (name, twins) = names.split_once(" / ")?;
    let (ref_name, view) = twins.split_once(" / ")?;
    let identifier = |s: &str| {
        s.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
            && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    };
    (identifier(name) && identifier(ref_name) && identifier(view) && !msg_type.contains('"')).then_some((name, view))
}
