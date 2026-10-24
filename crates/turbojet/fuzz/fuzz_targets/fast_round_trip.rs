//! Bytes decoded as a FAST stream (templates in `fast.xml`); each message decoded is encoded
//! again, and a second decoder must read the encoding back as the same message. That pairs the
//! decoder's handling of previous values with the encoder's, message after message.

#![no_main]

use std::sync::{Arc, LazyLock};

use libfuzzer_sys::fuzz_target;
use turbojet::Message;
use turbojet_fast::{Decoder, Encoder, ErrorKind, Templates};

static TEMPLATES: LazyLock<Arc<Templates>> =
    LazyLock::new(|| Arc::new(Templates::from_xml(include_str!("../fast.xml")).expect("valid templates")));

fuzz_target!(|bytes: &[u8]| {
    let mut decoder = Decoder::new(Arc::clone(&TEMPLATES));
    let mut encoder = Encoder::new(Arc::clone(&TEMPLATES));
    let mut again = Decoder::new(Arc::clone(&TEMPLATES));
    let (mut msg, mut decoded_again) = (Message::default(), Message::default());
    let mut encoded = Vec::new();
    let mut rest = bytes;
    while let Ok(decoded) = decoder.decode_into(rest, &mut msg) {
        encoded.clear();
        match encoder.encode_into(decoded.template_id, &msg, &mut encoded) {
            Ok(()) => {}
            // A message from a dynamic template reference doesn't say which template it used.
            Err(e) if e.kind() == ErrorKind::Unsupported => return,
            Err(e) => panic!("can't encode what was decoded: {e}\n{msg}"),
        }
        let read = again.decode_into(&encoded, &mut decoded_again).unwrap_or_else(|e| panic!("{e}\n{msg}"));
        assert!(read.rest.is_empty(), "the encoding is one message");
        assert_eq!(read.template_id, decoded.template_id);
        let fields = |m: &Message| m.fields_bytes().map(|(tag, value)| (tag, value.to_vec())).collect::<Vec<_>>();
        assert_eq!(fields(&decoded_again), fields(&msg));
        rest = decoded.rest;
        if rest.is_empty() {
            break;
        }
    }
});
