#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
// Library code handles every error or names the invariant that rules it out, in an `expect`
// (docs/STYLE.md). Tests may unwrap.
#![warn(clippy::unwrap_used)]

// The macros' tests define messages as generated code does, naming `turbojet::…`.
extern crate self as turbojet;

#[macro_use]
mod macros;

pub mod codec;
pub mod fields;
pub mod message;
