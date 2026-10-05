//! Bytes decoded as SBE messages, by the codecs generated from SBE's example schema and B3's
//! Binary Entrypoint (`crates/turbojet/tests/sbe`). Wrapping checks every bound, so reading every
//! field of what wraps, groups and data included (which `Debug` does), must not panic.

#![no_main]

#[allow(dead_code)]
#[path = "../../tests/sbe/b3.rs"]
mod b3;

#[allow(dead_code)]
#[path = "../../tests/sbe/car.rs"]
mod car;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    if let Ok((message, len)) = b3::decode(bytes) {
        assert!(len <= bytes.len(), "decoded {len} of {}", bytes.len());
        std::hint::black_box(format!("{message:?}"));
    }
    if let Ok((message, len)) = car::decode(bytes) {
        assert!(len <= bytes.len(), "decoded {len} of {}", bytes.len());
        std::hint::black_box(format!("{message:?}"));
    }
});
