//! Deterministic simulation of FIXP sessions: a client and a server, each a
//! [`turbojet::fixp::FixpSession`] driven as the connection driver drives one, run against each
//! other over the simulated network from a seed, as the FIX simulation runs an initiator and an
//! acceptor. It shares that simulation's time, network, stores and files; the sessions, the
//! workload and the rules are FIXP's own. See the crate README.

pub mod app;
pub mod check;
pub mod node;
pub mod wire;
mod world;

pub use world::run;
