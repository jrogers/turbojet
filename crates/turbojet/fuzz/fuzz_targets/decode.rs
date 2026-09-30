//! Bytes straight from the network, decoded as the connection's read loop does: message after
//! message, skipping garbled input, until only an incomplete message is left. Once with the
//! standard data fields, once with a venue's too.

#![no_main]

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode_with};
use turbojet::message::DataFields;
use turbojet_fuzz::{round_trip_with, venue_data_fields};

fuzz_target!(|bytes: &[u8]| {
    for data in [DataFields::standard(), venue_data_fields()] {
        let mut rest = bytes;
        loop {
            match decode_with(rest, &data) {
                Decoded::Message(msg, len) => {
                    assert!(len > 0 && len <= rest.len(), "consumed {len} of {}", rest.len());
                    round_trip_with(&msg, &data);
                    rest = &rest[len..];
                }
                Decoded::Incomplete => break,
                // The read loop drops `skip` bytes and decodes again: it must make progress.
                Decoded::Garbled { skip, .. } => {
                    assert!(skip > 0 && skip <= rest.len(), "skip {skip} of {}", rest.len());
                    rest = &rest[skip..];
                }
            }
        }
    }
});
