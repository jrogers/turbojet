//! Deterministic simulation testing for Turbojet: an initiator and an acceptor, each a
//! [`turbojet::Session`] driven as the connection driver drives one, run against each other in
//! one thread from a seed. Time, the network and the workload are simulated, every choice comes
//! from the seed, and a checker looks for broken invariants after every event, so any failure
//! replays from its seed. See the crate README.
