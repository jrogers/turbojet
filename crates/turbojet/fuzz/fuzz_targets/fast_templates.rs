//! Template XML loaded, then bytes decoded against whatever loads. Loading must return an error
//! or templates, never panic or recurse without bound, and so must decoding with them.

#![no_main]

use std::sync::Arc;

use libfuzzer_sys::fuzz_target;
use turbojet::Message;
use turbojet::fast::{Decoder, Templates};

fuzz_target!(|input: (&str, &[u8])| {
    let (xml, bytes) = input;
    let Ok(templates) = Templates::from_xml(xml) else { return };
    let mut decoder = Decoder::new(Arc::new(templates));
    let mut msg = Message::default();
    let mut rest = bytes;
    while let Ok(decoded) = decoder.decode_into(rest, &mut msg) {
        assert!(decoded.rest.len() < rest.len(), "a message takes at least its presence map");
        rest = decoded.rest;
        if rest.is_empty() {
            break;
        }
    }
});
