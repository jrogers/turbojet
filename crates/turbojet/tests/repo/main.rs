//! The engine's tests that need the repository's dictionaries/ or the SBE codecs generated into
//! tests/sbe, in one test binary; the package leaves them out. One binary rather than one per file,
//! so turbojet and its test dependencies link once, and B3's 11,000-line codec compiles once.

#[allow(dead_code)]
#[path = "../sbe/b3.rs"]
mod b3;

mod data_fields;
mod fixp;
mod fixt;
mod sbe;
