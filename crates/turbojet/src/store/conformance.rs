//! Behaviour every [`SessionStorage`] implementation must satisfy, for store authors to run
//! against their own stores (with the `conformance` feature) as Turbojet runs it against its
//! own.
//!
//! [`check`] goes through the store as the session does: it opens with
//! [`begin_open`](SessionStorage::begin_open), reads with [`fetch`](SessionLog::fetch) and
//! commits with [`commit`](SessionLog::commit), running whatever jobs they return. It panics on
//! the first difference, saying what was expected.
//!
//! ```no_run
//! # async fn example(storage: &dyn turbojet::SessionStorage) {
//! turbojet::store::conformance::check(storage).await;
//! # }
//! ```
#![allow(clippy::unwrap_used, reason = "a test suite, which panics on the first failure it finds")]

use std::io;

use super::{Fetched, Opened, SentMessages, SessionId, SessionLog, SessionStorage};
use crate::codec::encode;
use crate::fields::{MsgType, UtcTimestamp};
use crate::message::{Message, tags};

/// A session ID for the suite, with counterparty `target`. The suite uses targets `A` to `F`.
pub fn id(target: &str) -> SessionId {
    SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: target.into() }
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
pub async fn open(storage: &dyn SessionStorage, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
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
pub async fn commit(log: &mut dyn SessionLog) -> io::Result<()> {
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
pub async fn fetch(log: &mut dyn SessionLog, begin: u64, end: u64) -> io::Result<SentMessages> {
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
