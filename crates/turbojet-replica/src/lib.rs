//! Replicated session state for [Turbojet](turbojet): an active instance copies each FIX
//! session's committed state to a hot standby, which can take the sessions over with no sequence
//! gap and no resend storm.
//!
//! Work in progress: [`frame`] is the wire format between the two.

pub mod frame;
