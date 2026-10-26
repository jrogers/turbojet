//! Behaviour every [`SessionStorage`] implementation must satisfy, for store authors to run
//! against their own stores (with the `conformance` feature) as Turbojet runs it against its
//! own.
//!
//! [`check`] goes through the store as the session does: it opens with
//! [`begin_open`](SessionStorage::begin_open), reads with [`fetch`](StoredSession::fetch) and
//! commits with [`commit`](StoredSession::commit), running whatever jobs they return. It panics on
//! the first difference, saying what was expected. A store that keeps [`Marks`] runs
//! [`check_marks`] too.
//!
//! ```no_run
//! # async fn example(storage: &dyn turbojet::SessionStorage) {
//! turbojet::store::conformance::check(storage).await;
//! # }
//! ```
#![allow(clippy::unwrap_used, reason = "a test suite, which panics on the first failure it finds")]

use std::collections::BTreeMap;
use std::io;

use super::{Fetched, Marks, Opened, SentMessages, SessionId, SessionStorage, StoredSession};
use crate::codec::encode;
use crate::fields::{MsgType, UtcTimestamp};
use crate::message::{Message, tags};

/// A session ID for the suite, with counterparty `target`. The suite uses targets `A` to `G`, `H`
/// for [`check_marks`], and `M0` on for [`check_model`].
pub fn id(target: &str) -> SessionId {
    SessionId::new("FIX.4.4", "GATEWAY", target)
}

/// An encoded ExecutionReport with MsgSeqNum `seq`, as a session would store it.
pub fn app_message(seq: u64) -> Vec<u8> {
    encode(&message(seq)).expect("the message has only valid fields")
}

fn message(seq: u64) -> Message {
    Message::default()
        .with(tags::BEGIN_STRING, "FIX.4.4")
        .with(tags::MSG_TYPE, MsgType::ExecutionReport)
        .with(tags::MSG_SEQ_NUM, seq)
        .with(tags::EXEC_ID, format!("E{seq}"))
}

/// Opens `id`'s log, running the store's job if it returns one.
///
/// # Errors
///
/// The store's error opening the log.
pub async fn open(storage: &dyn SessionStorage, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
    match storage.begin_open(id)? {
        Opened::Ready(log) => Ok(log),
        Opened::Pending(job) => job.run_async().await,
    }
}

/// Commits `log`'s mutations, running the store's job if it returns one.
///
/// # Errors
///
/// The store's error committing.
pub async fn commit(log: &mut dyn StoredSession) -> io::Result<()> {
    match log.commit()? {
        None => Ok(()),
        Some(job) => job.run_async().await,
    }
}

/// Reads `log`'s stored messages `begin..=end`, running the store's job if it returns one.
///
/// # Errors
///
/// The store's error reading.
pub async fn fetch(log: &mut dyn StoredSession, begin: u64, end: u64) -> io::Result<SentMessages> {
    match log.fetch(begin, end)? {
        Fetched::Ready(read) => Ok(read),
        Fetched::Pending(job) => job.run_async().await,
    }
}

/// Runs the whole suite against `storage`, which must hold none of the suite's sessions (see
/// [`id`]) to begin with. Needs a tokio runtime with a blocking pool, which every runtime has.
///
/// # Panics
///
/// On the first behaviour that differs from what a session relies on.
pub async fn check(storage: &dyn SessionStorage) {
    check_round_trip(storage).await;
    check_created_at(storage).await;
    check_in_flight(storage).await;
    check_data_fields(storage).await;
    check_uncommitted_reads(storage).await;
    check_full_ids(storage).await;
    check_model(storage, 0..20).await;
}

/// [`check`] on a runtime of its own, for synchronous tests.
///
/// # Panics
///
/// As [`check`] does, or if a runtime can't be started.
pub fn check_blocking(storage: &dyn SessionStorage) {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime starts");
    runtime.block_on(check(storage));
}

/// For a store that keeps [`Marks`]: they survive reopening and a reset, sessions with any are
/// listed by [`marked`](SessionStorage::marked) with their full IDs, and cleared ones aren't.
/// `storage` must hold no marks to begin with. Needs a tokio runtime, as [`check`] does.
///
/// # Panics
///
/// On the first behaviour that differs from what a registry relies on.
pub async fn check_marks(storage: &dyn SessionStorage) {
    assert!(storage.marked().run_async().await.unwrap().is_empty(), "no marks to begin with");
    let full = id("H").with_target_sub_id("TRADER 7").with_qualifier("a;b");
    let cancel = crate::CancelOnDisconnect::new(
        crate::CancelTrigger::DisconnectOrLogout,
        std::time::Duration::from_millis(2500),
    );
    let marks = Marks { paused: true, cancel_on_disconnect: Some(cancel) };
    let mut log = open(storage, &full).await.unwrap();
    assert_eq!(log.marks(), Marks::default());
    log.set_marks(marks).unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    let mut log = open(storage, &full).await.unwrap();
    assert_eq!(log.marks(), marks, "marks survive reopening");
    log.reset().unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    assert_eq!(open(storage, &full).await.unwrap().marks(), marks, "and a reset");
    assert_eq!(open(storage, &id("H")).await.unwrap().marks(), Marks::default(), "they're the session's own");
    assert_eq!(storage.marked().run_async().await.unwrap(), [(full.clone(), marks)]);

    let mut log = open(storage, &full).await.unwrap();
    log.set_marks(Marks::default()).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    assert_eq!(open(storage, &full).await.unwrap().marks(), Marks::default());
    assert!(storage.marked().run_async().await.unwrap().is_empty(), "a session without marks isn't listed");
}

/// [`check_marks`] on a runtime of its own, for synchronous tests.
///
/// # Panics
///
/// As [`check_marks`] does, or if a runtime can't be started.
pub fn check_marks_blocking(storage: &dyn SessionStorage) {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime starts");
    runtime.block_on(check_marks(storage));
}

/// For a store that lists its sessions: every session opened is listed by
/// [`sessions`](SessionStorage::sessions) with its full ID, once, whether it stored anything or
/// not. Needs a tokio runtime, as [`check`] does.
///
/// # Panics
///
/// On the first behaviour that differs from what a tool listing sessions relies on.
pub async fn check_sessions(storage: &dyn SessionStorage) {
    let before = storage.sessions().run_async().await.unwrap();
    let full = id("L").with_target_sub_id("TRADER 7").with_qualifier("a;b");
    let mut log = open(storage, &full).await.unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    drop(open(storage, &id("M")).await.unwrap());
    drop(open(storage, &full).await.unwrap());
    let mut added: Vec<SessionId> =
        storage.sessions().run_async().await.unwrap().into_iter().filter(|id| !before.contains(id)).collect();
    added.sort_by_key(ToString::to_string);
    let mut expected = vec![full, id("M")];
    expected.sort_by_key(ToString::to_string);
    assert_eq!(added, expected, "each session opened, listed once with its full ID");
}

/// [`check_sessions`] on a runtime of its own, for synchronous tests.
///
/// # Panics
///
/// As [`check_sessions`] does, or if a runtime can't be started.
pub fn check_sessions_blocking(storage: &dyn SessionStorage) {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime starts");
    runtime.block_on(check_sessions(storage));
}

/// Sequence numbers and messages survive reopening; sessions are independent; a reset clears
/// both and persists.
async fn check_round_trip(storage: &dyn SessionStorage) {
    {
        let mut log = open(storage, &id("A")).await.unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (1, 1));
        log.record_outgoing(1, None).unwrap();
        log.record_outgoing(2, Some(&app_message(2))).unwrap();
        log.record_outgoing(3, None).unwrap();
        log.record_outgoing(4, Some(&app_message(4))).unwrap();
        log.set_next_incoming(7).unwrap();
        commit(log.as_mut()).await.unwrap();
    }

    let mut log = open(storage, &id("A")).await.unwrap();
    assert_eq!((log.next_outgoing(), log.next_incoming()), (5, 7), "state survives reopen");
    let sent = fetch(log.as_mut(), 1, 4).await.unwrap();
    let seqs: Vec<u64> = sent.iter().map(|(s, _)| *s).collect();
    assert_eq!(seqs, [2, 4]);
    assert_eq!(sent[1].1, app_message(4));
    assert_eq!(fetch(log.as_mut(), 3, 3).await.unwrap().len(), 0);

    // Sessions are independent.
    let other = open(storage, &id("B")).await.unwrap();
    assert_eq!((other.next_outgoing(), other.next_incoming()), (1, 1));
    drop(other);

    log.reset().unwrap();
    assert_eq!((log.next_outgoing(), log.next_incoming()), (1, 1));
    assert!(fetch(log.as_mut(), 1, u64::MAX).await.unwrap().is_empty());
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    assert_eq!(fetch(log.as_mut(), 1, 1).await.unwrap().len(), 1, "reads see what isn't committed yet");
    commit(log.as_mut()).await.unwrap();
    drop(log);

    let mut log = open(storage, &id("A")).await.unwrap();
    assert_eq!(log.next_outgoing(), 2, "reset persists");
    assert_eq!(fetch(log.as_mut(), 1, u64::MAX).await.unwrap().len(), 1);
}

/// Creation time: unknown until recorded, kept across reopening, cleared by reset.
async fn check_created_at(storage: &dyn SessionStorage) {
    let mut log = open(storage, &id("E")).await.unwrap();
    assert_eq!(log.created_at(), None);
    let created = UtcTimestamp::from_timestamp(1_790_000_000, 123_000_000).expect("in range");
    log.set_created_at(created).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    let mut log = open(storage, &id("E")).await.unwrap();
    assert_eq!(log.created_at(), Some(created));
    log.reset().unwrap();
    assert_eq!(log.created_at(), None);
    commit(log.as_mut()).await.unwrap();
    drop(log);
    assert_eq!(open(storage, &id("E")).await.unwrap().created_at(), None, "reset persists");
}

/// The messages in flight: kept across reopening, cleared by moving on or resetting.
async fn check_in_flight(storage: &dyn SessionStorage) {
    let mut log = open(storage, &id("C")).await.unwrap();
    assert_eq!(log.in_flight(), None);
    log.set_next_incoming(4).unwrap();
    log.set_in_flight(4).unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    let mut log = open(storage, &id("C")).await.unwrap();
    assert_eq!((log.next_incoming(), log.in_flight()), (4, Some(4)), "survives reopen");
    log.set_next_incoming(5).unwrap();
    assert_eq!(log.in_flight(), None);
    commit(log.as_mut()).await.unwrap();
    drop(log);
    let mut log = open(storage, &id("C")).await.unwrap();
    assert_eq!(log.in_flight(), None, "clearing persists");
    log.set_next_incoming_in_flight(7, 6).unwrap();
    assert_eq!((log.next_incoming(), log.in_flight()), (7, Some(6)));
    commit(log.as_mut()).await.unwrap();
    drop(log);
    let mut log = open(storage, &id("C")).await.unwrap();
    assert_eq!((log.next_incoming(), log.in_flight()), (7, Some(6)), "both survive reopen");
    log.set_in_flight(5).unwrap();
    log.reset().unwrap();
    assert_eq!(log.in_flight(), None);
    commit(log.as_mut()).await.unwrap();
    drop(log);
    assert_eq!(open(storage, &id("C")).await.unwrap().in_flight(), None, "reset persists");
}

/// Data fields, a venue's own too, come back byte for byte.
async fn check_data_fields(storage: &dyn SessionStorage) {
    let msg = message(1).with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"\xff\x01\x0110=000\x01").with_data(
        5000,
        5001,
        b"a\x01\xfe",
    );
    let encoded = encode(&msg).expect("the message has only valid fields");
    {
        let mut log = open(storage, &id("D")).await.unwrap();
        log.record_outgoing(1, Some(&encoded)).unwrap();
        commit(log.as_mut()).await.unwrap();
    }
    let mut log = open(storage, &id("D")).await.unwrap();
    let sent = fetch(log.as_mut(), 1, 1).await.unwrap();
    assert_eq!(sent[0].1, encoded);
}

/// Reads see every mutation made, committed or not: a range across both, and after a reset
/// that isn't committed, only what was stored since. A log dropped without committing loses
/// what it hadn't committed, and keeps what it had.
async fn check_uncommitted_reads(storage: &dyn SessionStorage) {
    let mut log = open(storage, &id("F")).await.unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    log.record_outgoing(2, Some(&app_message(2))).unwrap();
    commit(log.as_mut()).await.unwrap();
    log.record_outgoing(3, Some(&app_message(3))).unwrap();
    log.record_outgoing(4, None).unwrap();
    log.record_outgoing(5, Some(&app_message(5))).unwrap();
    let sent = fetch(log.as_mut(), 2, 5).await.unwrap();
    let seqs: Vec<u64> = sent.iter().map(|(s, _)| *s).collect();
    assert_eq!(seqs, [2, 3, 5], "committed and not, in order");
    assert_eq!(sent[1].1, app_message(3));
    commit(log.as_mut()).await.unwrap();

    log.reset().unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    let sent = fetch(log.as_mut(), 1, u64::MAX).await.unwrap();
    assert_eq!(sent.len(), 1, "only what was stored since the reset");
    commit(log.as_mut()).await.unwrap();
    log.record_outgoing(2, Some(&app_message(2))).unwrap();
    // Some stores make each change as it's made, so only what was committed is certain.
    drop(log);
    let mut log = open(storage, &id("F")).await.unwrap();
    assert!(log.next_outgoing() >= 2, "committed numbers survive");
    let sent = fetch(log.as_mut(), 1, 1).await.unwrap();
    assert_eq!(sent.len(), 1, "committed messages survive");
}

/// Sessions that differ only in a SubID, a LocationID or the qualifier are independent, and one
/// with all of them keeps its state across reopening.
async fn check_full_ids(storage: &dyn SessionStorage) {
    let plain = id("G");
    let full = plain
        .clone()
        .with_sender_sub_id("DESK")
        .with_sender_location_id("NY")
        .with_target_sub_id("TRADER 7")
        .with_target_location_id("LDN")
        .with_qualifier("b");
    let variants = [
        plain.clone().with_sender_sub_id("DESK"),
        plain.clone().with_sender_location_id("NY"),
        plain.clone().with_target_sub_id("TRADER 7"),
        plain.clone().with_target_location_id("LDN"),
        plain.clone().with_qualifier("b"),
        full.clone(),
    ];
    let mut log = open(storage, &plain).await.unwrap();
    log.set_next_incoming(5).unwrap();
    commit(log.as_mut()).await.unwrap();
    drop(log);
    for (n, variant) in (2..).zip(&variants) {
        let mut log = open(storage, variant).await.unwrap();
        assert_eq!(log.next_incoming(), 1, "{variant} is a session of its own");
        log.set_next_incoming(n).unwrap();
        commit(log.as_mut()).await.unwrap();
    }
    assert_eq!(open(storage, &plain).await.unwrap().next_incoming(), 5, "the plain session is untouched");
    assert_eq!(open(storage, &full).await.unwrap().next_incoming(), 7, "{full} keeps its state");
}

/// Random sequences of what a session does to its log, one per seed in `seeds`, each on a session
/// of its own (target `M` and the seed). After every step the store must agree with a model of
/// the log: its numbers, in-flight marker, creation time and stored messages, as a whole and in a
/// range. A store may evict its oldest messages to stay within a limit, so it holds exactly the
/// model's messages above [`evicted_through`](StoredSession::evicted_through). Reopened, it holds
/// what was committed, or what some later step left: a store may make each change as it's made.
/// The creation time may be any it had since the commit, as a store may record it at once while
/// it buffers the rest (only a schedule reads it).
/// [`check`] runs seeds `0..20`; a store with limits of its own runs more with them small.
///
/// # Panics
///
/// On the first difference, with its seed and the steps that led to it.
pub async fn check_model(storage: &dyn SessionStorage, seeds: std::ops::Range<u64>) {
    for seed in seeds {
        ModelRun::new(storage, seed).await.run().await;
    }
}

/// What the session does to its log in one step.
#[derive(Debug, Clone, Copy)]
enum Step {
    /// The next number, with a message of `padding` extra bytes.
    Store {
        padding: usize,
    },
    /// The next number, without a message (a session message).
    Skip,
    /// Numbers up to this one used without messages (an operator's skip ahead).
    SkipTo(u64),
    SetNextIncoming(u64),
    SetInFlight(u64),
    SetNextIncomingInFlight(u64, u64),
    SetCreatedAt(i64),
    Commit,
    Reset,
    Reopen,
}

/// The log as the session sees it.
#[derive(Debug, Clone, PartialEq)]
struct Model {
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    created_at: Option<UtcTimestamp>,
    sent: BTreeMap<u64, Vec<u8>>,
}

impl Model {
    fn new() -> Self {
        Self { next_outgoing: 1, next_incoming: 1, in_flight: None, created_at: None, sent: BTreeMap::new() }
    }
}

/// One seed's run: the store's log, the model, and what the run has done so far.
struct ModelRun<'a> {
    storage: &'a dyn SessionStorage,
    id: SessionId,
    seed: u64,
    rng: u64,
    log: Box<dyn StoredSession>,
    model: Model,
    /// The model as committed, then after each step since: what a reopened store may hold.
    since_commit: Vec<Model>,
    steps: Vec<Step>,
}

impl<'a> ModelRun<'a> {
    const STEPS: usize = 150;

    async fn new(storage: &'a dyn SessionStorage, seed: u64) -> Self {
        let id = id(&format!("M{seed}"));
        let log = open(storage, &id).await.unwrap();
        let mut run = Self {
            storage,
            id,
            seed,
            rng: seed,
            log,
            model: Model::new(),
            since_commit: vec![Model::new()],
            steps: Vec::new(),
        };
        run.agrees(&run.model.clone(), "a new session").await;
        run
    }

    /// splitmix64: enough to vary the steps, and the same for a seed everywhere.
    fn next(&mut self) -> u64 {
        self.rng = self.rng.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A number in `low..high`.
    fn below(&mut self, low: u64, high: u64) -> u64 {
        low + self.next() % (high - low)
    }

    fn step(&mut self) -> Step {
        let next_incoming = self.model.next_incoming;
        match self.below(0, 100) {
            0..35 => Step::Store { padding: usize::try_from(self.below(0, 400)).expect("small") },
            35..43 => Step::Skip,
            43..46 => Step::SkipTo(self.model.next_outgoing + self.below(1, 5)),
            46..56 => Step::SetNextIncoming(self.below(1, next_incoming + 5)),
            56..61 => Step::SetInFlight(self.below(1, next_incoming + 1)),
            61..65 => {
                let seq = self.below(1, next_incoming + 5);
                Step::SetNextIncomingInFlight(seq, self.below(1, seq + 1))
            }
            65..68 => Step::SetCreatedAt(i64::try_from(self.below(1_700_000_000, 1_900_000_000)).expect("small")),
            68..86 => Step::Commit,
            86..89 => Step::Reset,
            _ => Step::Reopen,
        }
    }

    async fn run(mut self) {
        for _ in 0..Self::STEPS {
            let step = self.step();
            self.steps.push(step);
            self.apply(step).await;
            let range = {
                let end = self.below(1, self.model.next_outgoing + 2);
                (self.below(1, end + 1), end)
            };
            self.agrees_in(&self.model.clone(), range).await;
        }
        drop(self.log);
    }

    async fn apply(&mut self, step: Step) {
        let model = &mut self.model;
        match step {
            Step::Store { padding } => {
                let seq = model.next_outgoing;
                let msg = encode(&message(seq).with(tags::TEXT, "x".repeat(padding))).expect("valid fields");
                self.log.record_outgoing(seq, Some(&msg)).unwrap();
                model.sent.insert(seq, msg);
                model.next_outgoing = seq + 1;
            }
            Step::Skip => {
                self.log.record_outgoing(model.next_outgoing, None).unwrap();
                model.next_outgoing += 1;
            }
            Step::SkipTo(seq) => {
                self.log.record_outgoing(seq - 1, None).unwrap();
                model.next_outgoing = seq;
            }
            Step::SetNextIncoming(seq) => {
                self.log.set_next_incoming(seq).unwrap();
                (model.next_incoming, model.in_flight) = (seq, None);
            }
            Step::SetInFlight(seq) => {
                self.log.set_in_flight(seq).unwrap();
                model.in_flight = Some(seq);
            }
            Step::SetNextIncomingInFlight(seq, in_flight) => {
                self.log.set_next_incoming_in_flight(seq, in_flight).unwrap();
                (model.next_incoming, model.in_flight) = (seq, Some(in_flight));
            }
            Step::SetCreatedAt(seconds) => {
                let at = UtcTimestamp::from_timestamp(seconds, 0).expect("in range");
                self.log.set_created_at(at).unwrap();
                model.created_at = Some(at);
            }
            Step::Commit => {
                commit(self.log.as_mut()).await.unwrap();
                self.since_commit.clear();
            }
            Step::Reset => {
                self.log.reset().unwrap();
                *model = Model::new();
            }
            Step::Reopen => {
                // The old log goes first: a store may lock a session while it's open.
                self.log = Box::new(Closed);
                self.log = open(self.storage, &self.id).await.unwrap();
                let candidates: Vec<Model> = self.since_commit.iter().chain([&self.model]).cloned().collect();
                let created_at = self.log.created_at();
                let mut held = None;
                if candidates.iter().any(|candidate| candidate.created_at == created_at) {
                    for candidate in candidates {
                        let candidate = Model { created_at, ..candidate };
                        if self.differs(&candidate, (1, u64::MAX)).await.is_none() {
                            held = Some(candidate);
                            break;
                        }
                    }
                }
                let Some(held) = held else {
                    let committed = self.since_commit.first().cloned().unwrap_or_else(|| self.model.clone());
                    return self.agrees(&committed, "reopened, as committed or after a later step").await;
                };
                self.model = held;
                self.since_commit.clear();
            }
        }
        if self.since_commit.is_empty() || !matches!(step, Step::Commit | Step::Reopen) {
            self.since_commit.push(self.model.clone());
        }
    }

    async fn agrees(&mut self, model: &Model, what: &str) {
        if let Some(difference) = self.differs(model, (1, u64::MAX)).await {
            self.fail(&format!("{what}: {difference}"));
        }
    }

    async fn agrees_in(&mut self, model: &Model, range: (u64, u64)) {
        for range in [(1, u64::MAX), range] {
            if let Some(difference) = self.differs(model, range).await {
                self.fail(&difference);
            }
        }
    }

    /// How the store differs from `model`, reading the messages in `range`, if it does.
    async fn differs(&mut self, model: &Model, (begin, end): (u64, u64)) -> Option<String> {
        let log = &mut self.log;
        let numbers = |next_outgoing, next_incoming, in_flight, created_at| {
            format!(
                "next outgoing {next_outgoing}, next incoming {next_incoming}, in flight {in_flight:?}, created {created_at:?}"
            )
        };
        let (store, expected) = (
            numbers(log.next_outgoing(), log.next_incoming(), log.in_flight(), log.created_at()),
            numbers(model.next_outgoing, model.next_incoming, model.in_flight, model.created_at),
        );
        if store != expected {
            return Some(format!("the store has {store}; expected {expected}"));
        }
        let kept_after = log.evicted_through().unwrap_or(0);
        let sent = fetch(log.as_mut(), begin, end).await.unwrap();
        let first = begin.max(kept_after + 1);
        let expected: Vec<_> = if first <= end {
            model.sent.range(first..=end).map(|(seq, msg)| (*seq, msg.clone())).collect()
        } else {
            Vec::new()
        };
        if sent != expected {
            let seqs = |messages: &[(u64, Vec<u8>)]| messages.iter().map(|(seq, _)| *seq).collect::<Vec<_>>();
            return Some(format!(
                "messages {begin}..={end} are {:?}; expected {:?} (evicted through {kept_after})",
                seqs(&sent),
                seqs(&expected)
            ));
        }
        None
    }

    fn fail(&self, difference: &str) -> ! {
        panic!("seed {}, after {} steps {:?}: {difference}", self.seed, self.steps.len(), self.steps);
    }
}

/// Stands in for a log while it's reopened, so the old one is dropped first.
struct Closed;

impl StoredSession for Closed {
    fn next_outgoing(&self) -> u64 {
        unreachable!("replaced at once")
    }
    fn next_incoming(&self) -> u64 {
        unreachable!("replaced at once")
    }
    fn set_next_incoming(&mut self, _: u64) -> io::Result<()> {
        unreachable!("replaced at once")
    }
    fn record_outgoing(&mut self, _: u64, _: Option<&[u8]>) -> io::Result<()> {
        unreachable!("replaced at once")
    }
    fn sent_messages(&mut self, _: u64, _: u64) -> io::Result<SentMessages> {
        unreachable!("replaced at once")
    }
    fn reset(&mut self) -> io::Result<()> {
        unreachable!("replaced at once")
    }
    fn in_flight(&self) -> Option<u64> {
        unreachable!("replaced at once")
    }
    fn set_in_flight(&mut self, _: u64) -> io::Result<()> {
        unreachable!("replaced at once")
    }
    fn created_at(&self) -> Option<UtcTimestamp> {
        unreachable!("replaced at once")
    }
    fn set_created_at(&mut self, _: UtcTimestamp) -> io::Result<()> {
        unreachable!("replaced at once")
    }
}
