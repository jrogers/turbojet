//! Bytes straight from the network, decoded as the connection's read loop does: message after
//! message into one reused message, skipping garbled input, until only an incomplete message is
//! left, and each checked against decoding afresh. Once with the standard data fields, once with
//! a venue's too.

#![no_main]

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, DecodedInto, decode_into, decode_with};
use turbojet::message::{DataFields, Message};
use turbojet_fuzz::{round_trip_with, venue_data_fields};

fuzz_target!(|bytes: &[u8]| {
    for data in [DataFields::standard(), venue_data_fields()] {
        // One message reused for every frame, as the read loop does, must decode each frame
        // exactly as a fresh one: nothing may carry over from the previous message.
        let mut scratch = Message::default();
        let mut rest = bytes;
        loop {
            let reused = decode_into(rest, &data, &mut scratch);
            match decode_with(rest, &data) {
                Decoded::Message(msg, len) => {
                    assert_eq!(reused, DecodedInto::Message(len));
                    assert_eq!(msg, scratch);
                    assert!(msg.fields().eq(scratch.fields()), "text fields differ");
                    assert_eq!(msg.is_malformed(), scratch.is_malformed());
                    assert!(len > 0 && len <= rest.len(), "consumed {len} of {}", rest.len());
                    round_trip_with(&msg, &data);
                    rest = &rest[len..];
                }
                Decoded::Incomplete => {
                    assert_eq!(reused, DecodedInto::Incomplete);
                    break;
                }
                // The read loop drops `skip` bytes and decodes again: it must make progress.
                Decoded::Garbled { skip, reason } => {
                    assert_eq!(reused, DecodedInto::Garbled { skip, reason });
                    assert!(skip > 0 && skip <= rest.len(), "skip {skip} of {}", rest.len());
                    rest = &rest[skip..];
                }
            }
        }
    }
});
