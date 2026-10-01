//! Lists every generated message type, per version, for the `typed` target: a parser for each,
//! read from the version crates' generated sources so the list can't fall behind them.

use std::fmt::Write;
use std::{env, fs};

const VERSIONS: &[(&str, &str, &str)] = &[
    ("FIX.4.2", "turbojet_fix42", "turbojet-fix42"),
    ("FIX.4.3", "turbojet_fix43", "turbojet-fix43"),
    ("FIX.4.4", "turbojet_fix44", "turbojet-fix44"),
    ("FIXT.1.1", "turbojet_fix50sp2", "turbojet-fix50sp2"),
];

fn main() {
    let mut out = String::from("pub const VERSIONS: &[(&str, &[Parse])] = &[\n");
    for (begin_string, krate, dir) in VERSIONS {
        let path = format!("../../{dir}/src/generated/messages.rs");
        println!("cargo::rerun-if-changed={path}");
        let source = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        writeln!(out, "    ({begin_string:?}, &[").unwrap();
        // Each message opens with `    Name / NameRef = "MsgType" {` inside `fix_message!`.
        let names: Vec<_> = source
            .lines()
            .filter_map(|line| line.strip_prefix("    ")?.strip_suffix("\" {")?.split_once(" / "))
            .map(|(name, _)| name)
            .collect();
        assert!(!names.is_empty(), "no messages found in {path}");
        for name in names {
            writeln!(out, "        check::<{krate}::{name}>,").unwrap();
        }
        writeln!(out, "    ]),").unwrap();
    }
    out.push_str("];\n");
    fs::write(format!("{}/versions.rs", env::var("OUT_DIR").unwrap()), out).unwrap();
}
