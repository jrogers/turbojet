//! Decoding allocates nothing once warmed up: the message, the decoder's scratch buffer and its
//! previous values all keep their allocations. Counted exactly, as `turbojet`'s budget is, and on
//! the test's own thread only: the harness allocates on others.
#![cfg(feature = "fast")]

#[path = "../benches/fast_refresh/mod.rs"]
mod fast_refresh;

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use turbojet::Message;
use turbojet::fast::Decoder;

struct Counting;

thread_local! {
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

// SAFETY: forwards to the system allocator, counting calls.
#[allow(unsafe_code, reason = "a global allocator is unsafe to implement; this one only counts")]
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.set(ALLOCATIONS.get() + 1);
        // SAFETY: the caller's contract is passed on unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: as above.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.set(ALLOCATIONS.get() + 1);
        // SAFETY: as above.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[test]
fn decoding_into_a_reused_message_allocates_nothing() {
    for entries in [1, 10, 50] {
        let (first, second) = fast_refresh::encoded(entries);
        let mut decoder = Decoder::new(fast_refresh::templates());
        let mut msg = Message::default();
        // Warm up: the first message of each kind sizes the buffers.
        decoder.decode_into(&first, &mut msg).expect("decodes");
        decoder.decode_into(&second, &mut msg).expect("decodes");
        decoder.reset();
        decoder.decode_into(&first, &mut msg).expect("decodes");

        let before = ALLOCATIONS.get();
        for _ in 0..100 {
            decoder.decode_into(&second, &mut msg).expect("decodes");
            decoder.reset();
            decoder.decode_into(&first, &mut msg).expect("decodes");
        }
        let allocations = ALLOCATIONS.get() - before;
        assert_eq!(allocations, 0, "a refresh of {entries}");
    }
}
