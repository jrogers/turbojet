//! The locks and atomics that threads share, which `--cfg shuttle` swaps for shuttle's: its tests
//! then choose how the threads interleave (see `shuttle_tests`). Shuttle runs every atomic as
//! `SeqCst`, so it finds interleavings that go wrong, not orderings too weak for one. Nor does it
//! switch threads inside tokio's primitives (`mpsc`, `oneshot`, `Notify`), which aren't swapped:
//! a race whose window lies between one of ours and one of tokio's isn't found.

#[cfg(shuttle)]
pub(crate) use shuttle::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
#[cfg(shuttle)]
pub(crate) use shuttle::sync::{Mutex, MutexGuard};
#[cfg(not(shuttle))]
pub(crate) use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
#[cfg(not(shuttle))]
pub(crate) use std::sync::{Mutex, MutexGuard};
