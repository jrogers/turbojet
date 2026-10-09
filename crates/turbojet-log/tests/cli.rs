//! Runs the turbojet-log binary on a log written by FileMessageLog.

use std::path::Path;
use std::process::Command;

use chrono::{TimeZone, Utc};
use turbojet::{Clock, FileLogOptions, FileMessageLog, MessageLog, SessionId};

/// A FIX frame of `body` (fields separated by `|`), with BodyLength and CheckSum.
fn frame(begin_string: &str, body: &str) -> Vec<u8> {
    let body = body.replace('|', "\x01");
    let mut frame = format!("8={begin_string}\x019={}\x01{body}", body.len()).into_bytes();
    let sum = frame.iter().map(|&b| u32::from(b)).sum::<u32>() % 256;
    frame.extend(format!("10={sum:03}\x01").bytes());
    frame
}

/// Writes a log at 12:00 UTC: an order in and its execution report out on one session, a
/// Heartbeat on another, and a FIXP frame.
fn write_log(dir: &Path) {
    let options = FileLogOptions {
        clock: Clock::from_fn(|| Utc.with_ymd_and_hms(2026, 10, 8, 12, 0, 0).unwrap()),
        ..FileLogOptions::default()
    };
    let log = FileMessageLog::open(dir, options).unwrap();
    let us = SessionId::new("FIX.4.4", "US", "THEM");
    let other = SessionId::new("FIX.4.2", "US", "OTHER");
    let sent = "52=20261008-12:00:00.000|";
    log.inbound(Some(&us), &frame("FIX.4.4", &format!("35=D|49=THEM|56=US|34=2|{sent}11=ORD1|54=1|")));
    log.outbound(Some(&us), &frame("FIX.4.4", &format!("35=8|49=US|56=THEM|34=2|{sent}11=ORD1|39=0|")));
    log.inbound(Some(&other), &frame("FIX.4.2", &format!("35=0|49=OTHER|56=US|34=5|{sent}")));
    log.outbound(None, b"\x10\x00\xeb\xcafixp");
}

fn run(args: &[&str]) -> (String, bool) {
    let dir = tempfile::tempdir().unwrap();
    write_log(dir.path());
    let output = Command::new(env!("CARGO_BIN_EXE_turbojet-log")).args(args).arg(dir.path()).output().unwrap();
    (String::from_utf8(output.stdout).unwrap(), output.status.success())
}

#[test]
fn messages_print_with_field_and_value_names() {
    let (out, ok) = run(&["--type", "D"]);
    assert!(ok);
    assert!(out.starts_with("2026-10-08 12:00:00.000000 in  FIX.4.4:US->THEM D NewOrderSingle\n"), "{out}");
    assert!(out.contains("  11 ClOrdID=ORD1\n"), "{out}");
    assert!(out.contains("  54 Side=1 (Buy)\n"), "{out}");
    assert!(!out.contains("ExecutionReport"), "{out}");
}

#[test]
fn filters_combine() {
    // Each filter alone, then together: the header lines that pass.
    let headers = |args: &[&str]| -> Vec<String> {
        let (out, _) = run(args);
        out.lines().filter(|l| !l.starts_with(' ')).map(|l| l[27..].to_owned()).collect()
    };
    assert_eq!(headers(&["--out"]), ["out FIX.4.4:US->THEM 8 ExecutionReport", "out - FIXP, 8 bytes"]);
    assert_eq!(headers(&["--session", "FIX.4.2:US->OTHER"]), ["in  FIX.4.2:US->OTHER 0 Heartbeat"]);
    assert_eq!(headers(&["--tag", "11=ORD1", "--out"]), ["out FIX.4.4:US->THEM 8 ExecutionReport"]);
    assert!(headers(&["--since", "12:00:01"]).is_empty());
    assert_eq!(headers(&["--until", "2026-10-08T12:00:01Z", "--type", "0"]), ["in  FIX.4.2:US->OTHER 0 Heartbeat"]);
}

#[test]
fn raw_prints_one_line_per_message() {
    let (out, _) = run(&["--raw"]);
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(lines.len(), 4);
    assert!(lines[0].contains(" 8=FIX.4.4|9="), "{out}");
    assert!(lines[0].contains("|35=D|49=THEM|"), "{out}");
    assert!(lines[3].ends_with(" 1000ebca66697870"), "{out}");
}

#[test]
fn a_file_that_cannot_be_read_fails_the_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_turbojet-log")).arg("/nonexistent/log").output().unwrap();
    assert!(!output.status.success());
}
