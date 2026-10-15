//! Decoding allocates nothing once warmed up: the message, the decoder's scratch buffer and its
//! previous values all keep their allocations. Counted exactly, as `turbojet`'s budget is.

#[path = "../benches/common/mod.rs"]
mod common;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

use turbojet::Message;
use turbojet_fast::Decoder;

struct Counting;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

// SAFETY: forwards to the system allocator, counting calls.
#[allow(unsafe_code, reason = "a global allocator is unsafe to implement; this one only counts")]
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        // SAFETY: the caller's contract is passed on unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: as above.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        // SAFETY: as above.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[test]
fn decoding_into_a_reused_message_allocates_nothing() {
    for entries in [1, 10, 50] {
        let (first, second) = common::encoded(entries);
        let mut decoder = Decoder::new(common::templates());
        let mut msg = Message::default();
        // Warm up: the first message of each kind sizes the buffers.
        decoder.decode_into(&first, &mut msg).expect("decodes");
        decoder.decode_into(&second, &mut msg).expect("decodes");
        decoder.reset();
        decoder.decode_into(&first, &mut msg).expect("decodes");

        let before = ALLOCATIONS.load(Ordering::Relaxed);
        for _ in 0..100 {
            decoder.decode_into(&second, &mut msg).expect("decodes");
            decoder.reset();
            decoder.decode_into(&first, &mut msg).expect("decodes");
        }
        let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before;
        assert_eq!(allocations, 0, "a refresh of {entries}");
    }
}
