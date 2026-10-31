//! A framed message parsed into a generated message type: any of every FIX version Turbojet
//! generates, repeating groups included. A message that parses must write out as one that parses
//! back to the same thing, its borrowed form must parse as it does, and its view must parse unless
//! the borrowed form fails on something other than a value.

#![no_main]

use std::fmt::Debug;

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode};
use turbojet::message::{FieldError, FieldErrorKind, FixMessageRef, FromMessage};
use turbojet::{FixMessage, Message};
use turbojet_fuzz::frame;

/// Parses a message as one generated type, checking what it can.
type Parse = fn(&Message);

// `VERSIONS`: each version's BeginString and a call of `check::<T>` for each of its message types.
include!(concat!(env!("OUT_DIR"), "/versions.rs"));

/// Parses `msg` as `T` whatever its MsgType, since the fuzzer picks the type; `view` and
/// `strict_view` are how its view parsed, leniently and strictly.
fn check<T: FixMessage + PartialEq + Debug>(msg: &Message, view: Option<FieldError>, strict_view: Option<FieldError>) {
    // Strict parsing agrees with lenient parsing, except that it may refuse an undefined tag.
    let (lenient, strict) = (T::from_message(msg), T::from_message_strict(msg));
    match (&lenient, &strict) {
        (Ok(a), Ok(b)) => assert_eq!(a, b),
        (Ok(_), Err(e)) => assert_eq!(e.kind, turbojet::message::FieldErrorKind::NotDefined, "{e}"),
        (Err(a), Err(b)) => assert_eq!(a, b),
        (Err(e), Ok(_)) => panic!("strict parsing accepted what lenient parsing refused: {e}"),
    }
    // The borrowed form, made owned, is the owned form, and fails alike.
    let borrowed = <T::Ref<'_> as FixMessageRef<'_>>::from_message(msg);
    assert_eq!(borrowed.map(FixMessageRef::into_owned), lenient);
    let borrowed = <T::Ref<'_> as FixMessageRef<'_>>::from_message_strict(msg);
    assert_eq!(borrowed.map(FixMessageRef::into_owned), strict);
    // The view converts values, and parses group entries, only as they're read: it parses whatever
    // the borrowed form does.
    check_view(&lenient, view);
    check_view(&strict, strict_view);
    let Ok(parsed) = lenient else { return };
    // Writing can normalise (timestamps are written to the millisecond), so compare the second
    // write with the first rather than the parse with the input.
    let written = parsed.to_message();
    let reparsed =
        T::from_message(&written).unwrap_or_else(|e| panic!("{parsed:?} wrote {written}, which fails: {e:?}"));
    assert_eq!(reparsed.to_message(), written, "{parsed:?}");
}

fn check_view<T>(parsed: &Result<T, FieldError>, view: Option<FieldError>) {
    match (parsed, view) {
        (Ok(_), Some(e)) => panic!("the view refused what the borrowed form parsed: {e}"),
        // An undefined tag is refused once the rest is valid, by both.
        (Err(e), None) => assert_ne!(e.kind, FieldErrorKind::NotDefined, "the view took an undefined tag"),
        // Both failed: the view may fail on a field missing, a malformed group, its count included,
        // or an undefined tag, before reaching the value the borrowed form failed on.
        _ => {}
    }
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
