//! Bytes decoded as a FAST stream, against templates using every field type, operator and
//! structure (`fast.xml`). Decoding must return an error or a message, never panic, and each
//! message takes at least one byte.

#![no_main]

use std::sync::{Arc, LazyLock};

use libfuzzer_sys::fuzz_target;
use turbojet::Message;
use turbojet_fast::{Decoder, Templates};

static TEMPLATES: LazyLock<Arc<Templates>> =
    LazyLock::new(|| Arc::new(Templates::from_xml(include_str!("../fast.xml")).expect("valid templates")));

fuzz_target!(|bytes: &[u8]| {
    let mut decoder = Decoder::new(Arc::clone(&TEMPLATES));
    let mut msg = Message::default();
    let mut rest = bytes;
    while let Ok(decoded) = decoder.decode_into(rest, &mut msg) {
        assert!(decoded.rest.len() < rest.len(), "a message takes at least its presence map");
        std::hint::black_box(msg.to_string());
        rest = decoded.rest;
        if rest.is_empty() {
            break;
        }
    }
});
