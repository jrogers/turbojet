//! Heap allocations per order → ack, wire to wire (decode, then the session, application and
//! store; the session encodes the ack), with the memory store and the disk store (without fsync),
//! each against an exact budget. `cargo test -p turbojet --test allocations -- --nocapture` prints
//! a per-stage table for each store.

#[path = "../benches/common/mod.rs"]
mod common;

use std::fmt::Write as _;
use std::io;
use std::sync::Arc;
use std::time::Instant;

use counting::{Counts, Stage};
use turbojet::codec::{Decoded, decode, encode};
use turbojet::fields::UtcTimestamp;
use turbojet::store::{SessionLog, SessionStorage};
use turbojet::{Application, Context, DiskStorage, MemoryStorage, Message, MessageReject, SessionId};

#[global_allocator]
static ALLOCATOR: counting::Counting = counting::Counting;

/// The benchmarks' application, its allocations attributed to the application stage. That includes
/// one of the engine's: the first `Context::send` grows the context's reply `Vec`. `Acker` overrides
/// only `on_message`; the other callbacks run at logon and logout, outside the counted orders.
#[derive(Default)]
struct StagedApp(common::Acker);

impl Application for StagedApp {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        counting::in_stage(Stage::Application, || self.0.on_message(ctx, msg))
    }
}

/// A store, its allocations attributed to the store stage.
struct StagedStorage<S>(S);

impl<S: SessionStorage> SessionStorage for StagedStorage<S> {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(StagedLog(self.0.open(id)?)))
    }
}

struct StagedLog(Box<dyn SessionLog>);

// Every method, including the provided ones, so the wrapped log's overrides are still used.
impl SessionLog for StagedLog {
    fn next_outgoing(&self) -> u64 {
        counting::in_stage(Stage::Store, || self.0.next_outgoing())
    }
    fn next_incoming(&self) -> u64 {
        counting::in_stage(Stage::Store, || self.0.next_incoming())
    }
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        counting::in_stage(Stage::Store, || self.0.set_next_incoming(seq))
    }
    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        counting::in_stage(Stage::Store, || self.0.record_outgoing(seq, msg))
    }
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        counting::in_stage(Stage::Store, || self.0.sent_messages(begin, end))
    }
    fn reset(&mut self) -> io::Result<()> {
        counting::in_stage(Stage::Store, || self.0.reset())
    }
    fn in_flight(&self) -> Option<u64> {
        counting::in_stage(Stage::Store, || self.0.in_flight())
    }
    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        counting::in_stage(Stage::Store, || self.0.set_in_flight(seq))
    }
    fn created_at(&self) -> Option<UtcTimestamp> {
        counting::in_stage(Stage::Store, || self.0.created_at())
    }
    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        counting::in_stage(Stage::Store, || self.0.set_created_at(at))
    }
}

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
    }

    impl Stage {
        pub const ALL: [Stage; 4] = [Stage::Decode, Stage::Session, Stage::Application, Stage::Store];

        pub fn name(self) -> &'static str {
            match self {
                Stage::Decode => "decode",
                Stage::Session => "session",
                Stage::Application => "application",
                Stage::Store => "store",
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
    // allocator can use them without recursing, even during thread teardown.
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
        if COUNTING.get() {
            let stage = STAGE.get() as usize;
            let mut counts = COUNTS.get();
            counts[stage] += Counts { allocs, reallocs, bytes: bytes as u64 };
            COUNTS.set(counts);
        }
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
    assert_eq!(counts[Stage::Store as usize], Counts::ZERO);
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

/// Orders processed, uncounted, before counting starts.
const WARM_UP: u64 = 100;
/// Orders counted; budgets are totals over these.
const COUNTED: u64 = 1_000;

/// Per-stage counts over COUNTED orders, each decoded from bytes, processed by a logged-on
/// session with `storage`, acknowledged by the application, and the ack recorded in the store and
/// encoded by the session into its output, which is cleared after each order as the connection
/// driver does.
fn order_to_ack(storage: impl SessionStorage + 'static) -> [Counts; Stage::ALL.len()] {
    let storage = Arc::new(StagedStorage(storage));
    let mut session = common::logged_on(storage, Arc::new(StagedApp::default()));
    let wire: Vec<Vec<u8>> = common::orders(WARM_UP + COUNTED).iter().map(|o| encode(o).unwrap()).collect();
    let now = Instant::now();
    for (i, bytes) in wire.iter().enumerate() {
        if i as u64 == WARM_UP {
            counting::take();
            counting::set_counting(true);
        }
        let msg = counting::in_stage(Stage::Decode, || match decode(bytes) {
            Decoded::Message(msg, _) => msg,
            _ => panic!("order {i} didn't decode"),
        });
        counting::in_stage(Stage::Session, || session.on_message(msg, now));
        let out = session.output();
        assert!(out.starts_with(b"8=FIX.4.2\x01") && out.windows(5).any(|w| w == b"\x0135=8"), "order {i}: no ack");
        session.clear_output();
    }
    counting::set_counting(false);
    counting::take()
}

/// A table of per-order counts by stage with `store`, with the engine's share (all but the
/// application) and the total.
fn report(store: &str, counts: &[Counts; Stage::ALL.len()]) -> String {
    let per = |n: u64| n as f64 / COUNTED as f64;
    let mut engine = Counts::ZERO;
    let mut total = Counts::ZERO;
    let mut table = format!(
        "Allocations per order → ack, {store} store ({} build, mean of {COUNTED} after {WARM_UP} warm-up)\n\n{:<12} {:>8} {:>8} {:>8}\n",
        if cfg!(debug_assertions) { "debug" } else { "release" },
        "stage",
        "allocs",
        "reallocs",
        "bytes",
    );
    let mut row = |name: &str, c: Counts| {
        writeln!(table, "{name:<12} {:>8.1} {:>8.1} {:>8.1}", per(c.allocs), per(c.reallocs), per(c.bytes)).unwrap();
    };
    for stage in Stage::ALL {
        let c = counts[stage as usize];
        row(stage.name(), c);
        if stage != Stage::Application {
            engine += c;
        }
        total += c;
    }
    row("engine", engine);
    row("total", total);
    table
}

/// Allocations and reallocs per stage, totalled over COUNTED orders.
type Budget = [(Stage, u64, u64); Stage::ALL.len()];

/// Budgets with each store. Exact: a change in either direction fails, so an improvement is
/// locked in by lowering the budget here. Counts also depend on std and dependencies (Vec growth,
/// BTreeMap node size), so a toolchain or dependency update can move them without a change to
/// Turbojet.
const MEMORY_BUDGET: Budget =
    [(Stage::Decode, 2000, 0), (Stage::Session, 0, 0), (Stage::Application, 8000, 2000), (Stage::Store, 1166, 0)];
const DISK_BUDGET: Budget =
    [(Stage::Decode, 2000, 0), (Stage::Session, 0, 0), (Stage::Application, 8000, 2000), (Stage::Store, 3166, 9000)];

#[test]
fn order_to_ack_allocates_exactly_its_budget() {
    let dir = tempfile::tempdir().unwrap();
    let disk = DiskStorage::new(dir.path(), false).unwrap();
    let runs =
        [("memory", order_to_ack(MemoryStorage::new()), MEMORY_BUDGET), ("disk", order_to_ack(disk), DISK_BUDGET)];
    let mut tables = Vec::new();
    let mut problems = Vec::new();
    for (store, counts, budget) in runs {
        let table = report(store, &counts);
        println!("{table}");
        tables.push(table);
        for (stage, allocs, reallocs) in budget {
            let c = counts[stage as usize];
            for (what, actual, budget) in [("allocations", c.allocs, allocs), ("reallocs", c.reallocs, reallocs)] {
                let stage = stage.name();
                if actual > budget {
                    problems.push(format!(
                        "{store} store, {stage}: {actual} {what} over {COUNTED} orders, budget {budget}"
                    ));
                } else if actual < budget {
                    problems.push(format!(
                        "{store} store, {stage}: {actual} {what} over {COUNTED} orders, fewer than the budget of {budget}: lower it"
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}\n\n{}", problems.join("\n"), tables.join("\n"));
}
