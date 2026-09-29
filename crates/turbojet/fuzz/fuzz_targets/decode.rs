//! Bytes straight from the network, decoded as the connection's read loop does: message after
//! message, skipping garbled input, until only an incomplete message is left.

#![no_main]

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode};
use turbojet_fuzz::round_trip;

fuzz_target!(|data: &[u8]| {
    let mut rest = data;
    loop {
        match decode(rest) {
            Decoded::Message(msg, len) => {
                assert!(len > 0 && len <= rest.len(), "consumed {len} of {}", rest.len());
                round_trip(&msg);
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
});
