//! One `#[test]` per scenario script in definitions/, so each passes or fails on its own.

use std::fmt::Write;
use std::{env, fs};

const VERSIONS: &[&str] = &["fix42", "fix43", "fix44", "fix50sp2"];

fn main() {
    println!("cargo::rerun-if-changed=definitions");
    let mut out = String::new();
    for version in VERSIONS {
        let mut scripts: Vec<_> = fs::read_dir(format!("definitions/{version}"))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .filter(|name| name.ends_with(".def"))
            .collect();
        scripts.sort();
        for script in scripts {
            let test = format!("{version}_{}", snake_case(script.trim_end_matches(".def")));
            writeln!(
                out,
                "#[test]\nfn {test}() {{\n    turbojet_acceptance::check({:?});\n}}\n",
                format!("{version}/{script}")
            )
            .unwrap();
        }
    }
    fs::write(format!("{}/scenarios.rs", env::var("OUT_DIR").unwrap()), out).unwrap();
}

/// `2b_MsgSeqNumTooHigh` -> `2b_msg_seq_num_too_high`.
fn snake_case(name: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for c in name.chars() {
        if c.is_ascii_uppercase() && prev_lower {
            out.push('_');
        }
        prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        out.push(if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' });
    }
    out
}
