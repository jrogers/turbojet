//! The interop scenarios, one module each, in one test binary: as a binary each, they compiled and
//! linked the harness eleven times. The soak (`tests/soak.rs`) stays a binary of its own, run by
//! name and in release.

mod app;
mod delays;
mod faults;
mod gap_recovery;
mod harness;
mod heartbeat;
mod logon_logout;
mod seq_reset;
mod session_ids;
mod tls;
