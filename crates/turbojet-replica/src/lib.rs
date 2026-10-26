//! Replicated session state for [Turbojet](turbojet): an active instance copies each FIX
//! session's committed state to a hot standby, which can take the sessions over with no sequence
//! gap and no resend storm.
//!
//! [`ReplicatedStorage`] wraps the active's store: each session's commit is made there and copied
//! over a [`Link`] to a [`Replica`], which applies it to the standby's own store and acknowledges
//! it, and the commit, so everything it covers going out, waits for both. [`frame`] is the wire
//! form between the two.
//!
//! Work in progress: links run in-process only ([`LocalLink`]) so far.

mod active;
pub mod frame;
mod standby;

pub use active::{Link, LocalLink, ReplicatedStorage};
pub use standby::Replica;
