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

/// A value alone on its cache lines, so another session's writes nearby don't invalidate it.
/// 128 bytes everywhere: Apple silicon's lines are 128, and x86 and Neoverse cores fetch 64-byte
/// lines in pairs, so 64 left Graviton as slow as no padding (DESIGN.md, Many sessions).
#[derive(Debug, Default)]
#[repr(align(128))]
pub(crate) struct CachePadded<T>(pub(crate) T);

impl<T> std::ops::Deref for CachePadded<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
