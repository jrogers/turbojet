//! The locks, atomics and channels that the active's threads share, which `--cfg shuttle` swaps
//! for shuttle's: its tests then choose how the threads interleave (see `shuttle_tests` in
//! `active.rs` and `net.rs`). As in `turbojet`'s, shuttle runs every atomic as `SeqCst` and doesn't
//! switch threads inside tokio's primitives. Its `recv_timeout` never times out, so a wait for an
//! answer that never comes shows as a deadlock. What the standby holds, and the public API's
//! `Mutex<Replica>`, stay std's.

#[cfg(shuttle)]
pub(crate) use shuttle::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
#[cfg(shuttle)]
pub(crate) use shuttle::sync::{Condvar, Mutex, MutexGuard, mpsc};
#[cfg(not(shuttle))]
pub(crate) use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
#[cfg(not(shuttle))]
pub(crate) use std::sync::{Condvar, Mutex, MutexGuard, mpsc};
