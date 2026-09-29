//! A framed message parsed into a generated message type: any of every FIX version Turbojet
//! generates, repeating groups included. A message that parses must write out as one that parses
//! back to the same thing.

#![no_main]

use std::fmt::Debug;

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode};
use turbojet::{FixMessage, Message};
use turbojet_fuzz::frame;

/// Parses a message as one generated type, checking what it can.
type Parse = fn(&Message);

// `VERSIONS`: each version's BeginString and a `check::<T>` for each of its message types.
include!(concat!(env!("OUT_DIR"), "/versions.rs"));

/// Parses `msg` as `T` whatever its MsgType, since the fuzzer picks the type.
fn check<T: FixMessage + PartialEq + Debug>(msg: &Message) {
    // Strict parsing agrees with lenient parsing, except that it may refuse an undefined tag.
    let (lenient, strict) = (T::from_message(msg), T::from_message_strict(msg));
    match (&lenient, &strict) {
        (Ok(a), Ok(b)) => assert_eq!(a, b),
        (Ok(_), Err(e)) => assert_eq!(e.kind, turbojet::message::FieldErrorKind::NotDefined, "{e}"),
        (Err(a), Err(b)) => assert_eq!(a, b),
        (Err(e), Ok(_)) => panic!("strict parsing accepted what lenient parsing refused: {e}"),
    }
    let Ok(parsed) = lenient else { return };
    // Writing can normalise (timestamps are written to the millisecond), so compare the second
    // write with the first rather than the parse with the input.
    let written = parsed.to_message();
    let reparsed =
        T::from_message(&written).unwrap_or_else(|e| panic!("{parsed:?} wrote {written}, which fails: {e:?}"));
    assert_eq!(reparsed.to_message(), written, "{parsed:?}");
}

fuzz_target!(|data: &[u8]| {
    // Which version and message type, then the body.
    let [version, hi, lo, body @ ..] = data else { return };
    let (begin_string, parsers) = VERSIONS[*version as usize % VERSIONS.len()];
    let parse = parsers[usize::from(u16::from_be_bytes([*hi, *lo])) % parsers.len()];
    if let Decoded::Message(msg, _) = decode(&frame(begin_string, body)) {
        parse(&msg);
    }
});
