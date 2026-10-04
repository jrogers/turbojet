//! Deterministic simulation testing for Turbojet: an initiator and an acceptor, each a
//! [`turbojet::Session`] driven as the connection driver drives one, run against each other in
//! one thread from a seed. Time, the network and the workload are simulated, every choice comes
//! from the seed, and a checker looks for broken invariants after every event, so any failure
//! replays from its seed. See the crate README.
#![allow(missing_debug_implementations, reason = "a test harness, not published")]

pub mod app;
pub mod check;
pub mod files;
pub mod hostile;
pub mod net;
pub mod node;
pub mod queue;
pub mod rng;
pub mod store;
pub mod time;
pub mod world;

pub use world::{Failure, Options, Plant, Report, WRITE_DEADLOCK, run};

/// One end of the simulated connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Initiator,
    Acceptor,
}

impl Side {
    pub fn index(self) -> usize {
        match self {
            Side::Initiator => 0,
            Side::Acceptor => 1,
        }
    }

    pub fn other(self) -> Side {
        match self {
            Side::Initiator => Side::Acceptor,
            Side::Acceptor => Side::Initiator,
        }
    }
}
