//! Heap allocations per order → ack, wire to wire (decode, session, application, store, encode),
//! against an exact budget. `cargo test -p turbojet --test allocations -- --nocapture` prints the
//! per-stage table.

#[path = "../benches/common/mod.rs"]
mod common;

use counting::{Counts, Stage};

#[global_allocator]
static ALLOCATOR: counting::Counting = counting::Counting;

mod counting {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    /// Where an allocation is attributed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Stage {
        Decode,
        Session,
        Application,
        Store,
        Encode,
    }

    impl Stage {
        pub const ALL: [Stage; 5] = [Stage::Decode, Stage::Session, Stage::Application, Stage::Store, Stage::Encode];

        #[allow(dead_code)] // used by the order → ack report
        pub fn name(self) -> &'static str {
            match self {
                Stage::Decode => "decode",
                Stage::Session => "session",
                Stage::Application => "application",
                Stage::Store => "store",
                Stage::Encode => "encode",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Counts {
        pub allocs: u64,
        pub reallocs: u64,
        /// Bytes requested: an allocation's size, or what a realloc grows by.
        pub bytes: u64,
    }

    impl Counts {
        pub const ZERO: Counts = Counts { allocs: 0, reallocs: 0, bytes: 0 };
    }

    impl std::ops::AddAssign for Counts {
        fn add_assign(&mut self, other: Counts) {
            self.allocs += other.allocs;
            self.reallocs += other.reallocs;
            self.bytes += other.bytes;
        }
    }

    // `const` thread-locals of `Copy` types neither allocate nor register destructors, so the
    // allocator can use them without recursing.
    thread_local! {
        static COUNTING: Cell<bool> = const { Cell::new(false) };
        static STAGE: Cell<Stage> = const { Cell::new(Stage::Session) };
        static COUNTS: Cell<[Counts; Stage::ALL.len()]> = const { Cell::new([Counts::ZERO; Stage::ALL.len()]) };
    }

    /// Starts or stops counting this thread's allocations.
    pub fn set_counting(on: bool) {
        COUNTING.set(on);
    }

    /// Runs `f` with its allocations attributed to `stage`.
    pub fn in_stage<T>(stage: Stage, f: impl FnOnce() -> T) -> T {
        let outer = STAGE.replace(stage);
        let result = f();
        STAGE.set(outer);
        result
    }

    /// This thread's counts since the last call, by stage (indexed by `Stage as usize`).
    pub fn take() -> [Counts; Stage::ALL.len()] {
        COUNTS.replace([Counts::ZERO; Stage::ALL.len()])
    }

    fn record(allocs: u64, reallocs: u64, bytes: usize) {
        // try_with: allocations during thread teardown are simply not counted.
        let _ = COUNTING.try_with(|counting| {
            if counting.get() {
                let stage = STAGE.get() as usize;
                let mut counts = COUNTS.get();
                counts[stage] += Counts { allocs, reallocs, bytes: bytes as u64 };
                COUNTS.set(counts);
            }
        });
    }

    /// The system allocator, counting on threads that have turned counting on.
    pub struct Counting;

    // SAFETY: every method forwards to `System` with the caller's arguments unchanged.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            record(1, 0, layout.size());
            unsafe { System.alloc(layout) }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            record(1, 0, layout.size());
            unsafe { System.alloc_zeroed(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            record(0, 1, new_size.saturating_sub(layout.size()));
            unsafe { System.realloc(ptr, layout, new_size) }
        }
    }
}

#[test]
fn counts_allocations_reallocs_and_bytes_by_stage() {
    counting::take();
    counting::set_counting(true);
    counting::in_stage(Stage::Decode, || {
        let mut v: Vec<u8> = Vec::with_capacity(10);
        v.reserve(20); // len 0, cap 10 → grows to 20: one realloc of 10 more bytes
        std::hint::black_box(&v);
    });
    counting::set_counting(false);
    let counts = counting::take();
    assert_eq!(counts[Stage::Decode as usize], Counts { allocs: 1, reallocs: 1, bytes: 20 });
    assert_eq!(counts[Stage::Encode as usize], Counts::ZERO);
}

#[test]
fn nothing_is_counted_when_off_or_on_other_threads() {
    counting::take();
    std::hint::black_box(vec![0u8; 100]);
    assert_eq!(counting::take(), [Counts::ZERO; Stage::ALL.len()]);

    counting::set_counting(true);
    let other = std::thread::spawn(|| {
        std::hint::black_box(vec![0u8; 100]);
        counting::take()
    })
    .join()
    .unwrap();
    counting::set_counting(false);
    assert_eq!(other, [Counts::ZERO; Stage::ALL.len()]);
}
