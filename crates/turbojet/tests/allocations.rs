//! Heap allocations per order → ack, wire to wire (decode into one reused message, as the
//! connection does, then the session, application and store; the session encodes the ack), with
//! the memory store and the disk store (without fsync), each against an exact budget; the same
//! for a FIXP server answering B3 orders; and typed parsing alone, borrowed and owned.
//! `cargo test -p turbojet --test allocations -- --nocapture` prints a per-stage table for each
//! store and a per-parse table.

#[allow(dead_code)]
#[path = "sbe/b3.rs"]
mod b3;
#[path = "../benches/common/mod.rs"]
mod common;

use std::fmt::Write as _;
use std::io;
use std::sync::Arc;
use std::time::Instant;

use counting::{Counts, Stage};
use turbojet::codec::{DecodedInto, decode_into, encode};
use turbojet::fields::{Decimal, UtcTimestamp};
use turbojet::fixp::{
    ClientConfig, FixpApplication, FixpConfig, FixpContext, FixpRegistry, FixpSession, Received, Role, ServerConfig,
};
use turbojet::message::DataFields;
use turbojet::store::{SessionLog, SessionStorage};
use turbojet::{
    Application, Context, DiskStorage, MemoryStorage, Message, MessageLog, MessageReject, SessionConfig, SessionId,
};
use turbojet_fix42::{NewOrderSingle, NewOrderSingleRef, PreAllocGrp};

#[global_allocator]
static ALLOCATOR: counting::Counting = counting::Counting;

/// The benchmarks' application, its allocations attributed to the application stage. That includes
/// `Context::send` writing the typed ack into a message the session reuses, which allocates nothing
/// once warmed up. `Acker` overrides only `on_message`; the other callbacks run at logon and
/// logout, outside the counted orders.
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
    fn commit(&mut self) -> io::Result<Option<turbojet::store::Commit>> {
        counting::in_stage(Stage::Store, || self.0.commit())
    }
    fn evicted_through(&self) -> Option<u64> {
        counting::in_stage(Stage::Store, || self.0.evicted_through())
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
    #[allow(unsafe_code, reason = "a counting global allocator can only be written with unsafe")]
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

/// A message log that does nothing, set on every session here, and called as the connection driver
/// calls it, so that the budgets cover the hook: the engine's side of it must allocate nothing.
#[derive(Debug)]
struct NoLog;

impl MessageLog for NoLog {
    fn inbound(&self, _session: Option<&SessionId>, _frame: &[u8]) {}
    fn outbound(&self, _session: Option<&SessionId>, _frame: &[u8]) {}
}

/// Per-stage counts over COUNTED orders, each decoded from bytes into one reused message as the
/// connection does (the warm-up grows it before counting starts), processed by a logged-on
/// session with `storage`, acknowledged by the application, and the ack recorded in the store and
/// encoded by the session into its output, which is cleared after each order as the connection
/// driver does.
fn order_to_ack(storage: impl SessionStorage + 'static) -> [Counts; Stage::ALL.len()] {
    let storage = Arc::new(StagedStorage(storage));
    let mut config = SessionConfig::new("FIX.4.2", "GATEWAY");
    config.message_log = Some(Arc::new(NoLog));
    let mut session = common::logged_on_with(config, storage, Arc::new(StagedApp::default()));
    let wire: Vec<Vec<u8>> = common::orders(WARM_UP + COUNTED).iter().map(|o| encode(o).unwrap()).collect();
    let now = Instant::now();
    let data = DataFields::standard();
    let mut msg = Message::default();
    for (i, bytes) in wire.iter().enumerate() {
        if i as u64 == WARM_UP {
            counting::take();
            counting::set_counting(true);
        }
        counting::in_stage(Stage::Decode, || match decode_into(bytes, &data, &mut msg) {
            DecodedInto::Message(len) => {
                let log = session.message_log().expect("a message log is set");
                log.inbound(session.session_id(), &bytes[..len]);
            }
            _ => panic!("order {i} didn't decode"),
        });
        counting::in_stage(Stage::Session, || {
            session.on_message(&msg, now);
            session.commit_blocking(now);
        });
        let out = session.output();
        assert!(out.starts_with(b"8=FIX.4.2\x01") && out.windows(5).any(|w| w == b"\x0135=8"), "order {i}: no ack");
        // The ack is the only message out, so the output is its frame.
        assert_eq!(out.windows(4).filter(|w| w == b"\x0110=").count(), 1, "order {i}: more than the ack");
        counting::in_stage(Stage::Session, || session.message_log().unwrap().outbound(session.session_id(), out));
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
    [(Stage::Decode, 0, 0), (Stage::Session, 0, 0), (Stage::Application, 0, 0), (Stage::Store, 1166, 0)];
const DISK_BUDGET: Budget =
    [(Stage::Decode, 0, 0), (Stage::Session, 0, 0), (Stage::Application, 0, 0), (Stage::Store, 166, 0)];

#[test]
fn order_to_ack_allocates_exactly_its_budget() {
    let dir = tempfile::tempdir().unwrap();
    let disk = DiskStorage::new(dir.path(), false).unwrap();
    check_budgets([
        ("memory", order_to_ack(MemoryStorage::new()), MEMORY_BUDGET),
        ("disk", order_to_ack(disk), DISK_BUDGET),
    ]);
}

/// Prints a table per run, and fails on any stage whose count differs from its budget.
fn check_budgets(runs: [(&str, [Counts; Stage::ALL.len()], Budget); 2]) {
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

/// The FIXP benchmark's B3 order (`benches/fixp.rs`).
fn b3_order(cl_ord_id: u64) -> b3::NewOrderSingle {
    b3::NewOrderSingle {
        cl_ord_id,
        security_id: 4001,
        price: b3::PriceOptional { mantissa: Some(1_502_500) },
        order_qty: 100,
        account: Some(1),
        market_segment_id: 1,
        side: b3::Side::Buy,
        ord_type: b3::OrdType::Limit,
        time_in_force: b3::TimeInForce::Day,
        ord_tag_id: None,
        mm_protection_reset: None,
        routing_instruction: None,
        self_trade_prevention_instruction: None,
        stop_px: b3::PriceOptional { mantissa: None },
        min_qty: None,
        max_floor: None,
        investor_id: None,
        custodian_info: b3::CustodianInfo { custodian: None, custody_account: None, custody_allocation_type: None },
        expire_date: None,
        sender_location: turbojet::sbe::pad(b"DMA"),
        entering_trader: *b"TRADR",
    }
}

/// The FIXP benchmark's server application, answering each order with an order of the same
/// ClOrdID, its allocations attributed to the application stage.
struct FixpAcker;

impl FixpApplication for FixpAcker {
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        counting::in_stage(Stage::Application, || {
            let Ok((b3::Decoded::NewOrderSingle(received), _)) = b3::decode(msg.bytes) else { return };
            ctx.send(&b3_order(received.cl_ord_id())).unwrap();
        });
    }
}

/// `msg`, framed as a FIXP session receives it: SOFH (length, then 0x5BE0), then the SBE message.
fn sofh_framed(msg: &impl turbojet::sbe::Encode) -> Vec<u8> {
    let mut sbe = Vec::new();
    msg.encode_into(&mut sbe).unwrap();
    let mut frame = u32::try_from(sbe.len() + 6).unwrap().to_be_bytes().to_vec();
    frame.extend_from_slice(&0x5BE0u16.to_be_bytes());
    frame.extend_from_slice(&sbe);
    frame
}

/// Feeds `bytes` to `session` through `buf` (the driver's input buffer, reused), committing
/// whenever input waits for it, as the driver does; then shows what it wrote to its message log
/// and clears it, as the driver does once it's written, returning how many bytes that was.
fn fixp_feed(session: &mut FixpSession, buf: &mut Vec<u8>, bytes: &[u8], now: Instant) -> usize {
    buf.extend_from_slice(bytes);
    while session.feed(buf, now) {
        assert!(session.take_commit().is_none(), "the stores commit at once");
    }
    assert!(session.take_commit().is_none(), "the stores commit at once");
    let written = session.output().len();
    // One order, one answer, so the output is its frame. The session's id isn't public; passing it
    // would cost nothing more.
    session.message_log().expect("a message log is set").outbound(None, session.output());
    session.clear_output();
    written
}

/// As `fixp_feed`, returning what `session` wrote, for the handshake.
fn fixp_step(session: &mut FixpSession, bytes: &[u8], now: Instant) -> Vec<u8> {
    let mut buf = bytes.to_vec();
    while session.feed(&mut buf, now) {
        assert!(session.take_commit().is_none(), "the stores commit at once");
    }
    assert!(session.take_commit().is_none(), "the stores commit at once");
    let out = session.output().to_vec();
    session.clear_output();
    out
}

/// Per-stage counts over COUNTED orders fed to an established FIXP server keeping its state in
/// `storage` (recoverable flows both ways, so it stores each answer), as the connection driver
/// feeds them: framing and decoding the session messages happen in the session, so there's no
/// decode stage. A client session establishes it, uncounted.
fn fixp_order_to_ack(storage: impl SessionStorage + 'static) -> [Counts; Stage::ALL.len()] {
    let now = Instant::now();
    let server_registry = Arc::new(FixpRegistry::with_storage(Arc::new(StagedStorage(storage))));
    let config = FixpConfig::new(Role::Server(ServerConfig::new("SERVER")));
    let (server, _server_commands) = FixpSession::new(config, server_registry, Arc::new(FixpAcker), now);
    // `feed` shows each message to the log; `fixp_feed` shows it the answers.
    let mut server = server.with_message_log(Arc::new(NoLog));
    let client_registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
    let config = FixpConfig::new(Role::Client(ClientConfig::new("CLIENT", "SERVER")));
    let (mut client, _client_commands) = FixpSession::new(config, client_registry, Arc::new(FixpAcker), now);
    server.on_connect(now);
    client.on_connect(now);
    let mut to_server = fixp_step(&mut client, &[], now);
    while !(server.is_established() && client.is_established()) {
        let to_client = fixp_step(&mut server, &to_server, now);
        to_server = fixp_step(&mut client, &to_client, now);
    }
    let wire: Vec<Vec<u8>> = (0..WARM_UP + COUNTED).map(|i| sofh_framed(&b3_order(i))).collect();
    let mut buf = Vec::new();
    for (i, bytes) in wire.iter().enumerate() {
        if i as u64 == WARM_UP {
            counting::take();
            counting::set_counting(true);
        }
        let written = counting::in_stage(Stage::Session, || fixp_feed(&mut server, &mut buf, bytes, now));
        assert!(written > 0, "order {i}: no answer");
    }
    counting::set_counting(false);
    counting::take()
}

/// FIXP budgets with each store, exact as above: as FIX's, the stores keeping the answer.
const FIXP_MEMORY_BUDGET: Budget =
    [(Stage::Decode, 0, 0), (Stage::Session, 0, 0), (Stage::Application, 0, 0), (Stage::Store, 1166, 0)];
const FIXP_DISK_BUDGET: Budget =
    [(Stage::Decode, 0, 0), (Stage::Session, 0, 0), (Stage::Application, 0, 0), (Stage::Store, 166, 0)];

#[test]
fn fixp_order_to_ack_allocates_exactly_its_budget() {
    let dir = tempfile::tempdir().unwrap();
    let disk = DiskStorage::new(dir.path(), false).unwrap();
    check_budgets([
        ("FIXP, memory", fixp_order_to_ack(MemoryStorage::new()), FIXP_MEMORY_BUDGET),
        ("FIXP, disk", fixp_order_to_ack(disk), FIXP_DISK_BUDGET),
    ]);
}

/// The benchmark order (`common::orders`), with a three-entry NoAllocs group if `with_allocs`, as
/// received: with a standard header.
fn order_message(with_allocs: bool) -> Message {
    let mut order = common::new_order_single(1);
    if with_allocs {
        let alloc = |account: &str, shares: i64| {
            let mut alloc = PreAllocGrp::new(account);
            alloc.alloc_shares = Some(Decimal::new(shares, 0));
            alloc
        };
        order.allocs = vec![alloc("ACCT-A", 50), alloc("ACCT-B", 30), alloc("ACCT-C", 20)];
    }
    common::with_header("CLIENT", "GATEWAY", 2, order.into())
}

/// Allocations and reallocs of `parse`, totalled over COUNTED calls after WARM_UP.
fn parse_counts(mut parse: impl FnMut()) -> Counts {
    for _ in 0..WARM_UP {
        parse();
    }
    counting::take();
    counting::set_counting(true);
    for _ in 0..COUNTED {
        parse();
    }
    counting::set_counting(false);
    counting::take().into_iter().fold(Counts::ZERO, |mut sum, c| {
        sum += c;
        sum
    })
}

/// Typed parsing alone, which the order → ack counts can't separate from building the reply: the
/// borrowed twin allocates nothing, groups included, and the owned message copies out its
/// strings and group entries. Budgets are allocations and reallocs over COUNTED parses, exact as
/// above.
#[test]
fn typed_parsing_allocates_exactly_its_budget() {
    let plain = order_message(false);
    let with_allocs = order_message(true);
    let parses: [(&str, Counts, u64, u64); 5] = [
        (
            "NewOrderSingleRef",
            parse_counts(|| {
                std::hint::black_box(plain.parse::<NewOrderSingleRef>().unwrap());
            }),
            0,
            0,
        ),
        (
            "NewOrderSingleRef, 3 allocs",
            parse_counts(|| {
                std::hint::black_box(with_allocs.parse::<NewOrderSingleRef>().unwrap());
            }),
            0,
            0,
        ),
        (
            "NewOrderSingleRef, 3 allocs read",
            parse_counts(|| {
                let order = with_allocs.parse::<NewOrderSingleRef>().unwrap();
                assert_eq!(order.allocs.len(), 3);
                for alloc in order.allocs.iter() {
                    std::hint::black_box((alloc.alloc_account, alloc.alloc_shares));
                }
            }),
            0,
            0,
        ),
        (
            "NewOrderSingle",
            parse_counts(|| {
                std::hint::black_box(plain.parse::<NewOrderSingle>().unwrap());
            }),
            // ClOrdID, Symbol and Account are short enough to be kept inline.
            0,
            0,
        ),
        (
            "NewOrderSingle, 3 allocs",
            parse_counts(|| {
                std::hint::black_box(with_allocs.parse::<NewOrderSingle>().unwrap());
            }),
            // The entries' Vec; the strings, each entry's AllocAccount included, are inline.
            1000,
            0,
        ),
    ];
    check_parses(&parses);
}

/// Prints a table of per-parse counts and fails on any that differ from their budget.
fn check_parses(parses: &[(&str, Counts, u64, u64)]) {
    let mut table = format!(
        "Allocations per typed parse ({} build, mean of {COUNTED} after {WARM_UP} warm-up)\n\n{:<34} {:>8} {:>8} {:>8}\n",
        if cfg!(debug_assertions) { "debug" } else { "release" },
        "parse",
        "allocs",
        "reallocs",
        "bytes",
    );
    let per = |n: u64| n as f64 / COUNTED as f64;
    let mut problems = Vec::new();
    for &(name, c, allocs, reallocs) in parses {
        writeln!(table, "{name:<34} {:>8.1} {:>8.1} {:>8.1}", per(c.allocs), per(c.reallocs), per(c.bytes)).unwrap();
        for (what, actual, budget) in [("allocations", c.allocs, allocs), ("reallocs", c.reallocs, reallocs)] {
            if actual > budget {
                problems.push(format!("{name}: {actual} {what} over {COUNTED} parses, budget {budget}"));
            } else if actual < budget {
                problems.push(format!(
                    "{name}: {actual} {what} over {COUNTED} parses, fewer than the budget of {budget}: lower it"
                ));
            }
        }
    }
    println!("{table}");
    assert!(problems.is_empty(), "{}\n\n{table}", problems.join("\n"));
}
