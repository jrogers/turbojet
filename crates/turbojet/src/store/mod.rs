//! Session persistence: sequence numbers and sent application messages, kept per session so a
//! counterparty can reconnect and recover missed messages.

mod disk;
mod memory;

use std::fmt;
use std::io;

pub use disk::DiskStorage;
pub use memory::MemoryStorage;

use crate::fields::UtcTimestamp;

/// Identifies a FIX session from the gateway's side.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId {
    /// BeginString(8).
    pub begin_string: String,
    /// Our CompID.
    pub sender_comp_id: String,
    /// The counterparty's CompID.
    pub target_comp_id: String,
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}->{}", self.begin_string, self.sender_comp_id, self.target_comp_id)
    }
}

/// Opens per-session logs. Shared by all connections.
pub trait SessionStorage: Send + Sync {
    /// Opens the log for `id`, creating it with both sequence numbers at 1 if it does not exist.
    /// The gateway holds at most one open log per session.
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>>;
}

/// Persistent state of one session.
///
/// A store may make each mutation durable as it's made, or buffer them until
/// [`commit`](SessionLog::commit), which the session calls once per batch of work (one read
/// from the counterparty, one batch of commands, one step of a resend), before anything that
/// batch sends goes on the wire. Reads see every mutation made, committed or not.
pub trait SessionLog: Send {
    /// The MsgSeqNum of the next message to send.
    fn next_outgoing(&self) -> u64;

    /// The MsgSeqNum expected on the next message from the counterparty.
    fn next_incoming(&self) -> u64;

    /// Records the next incoming sequence number, which also clears
    /// [`in_flight`](SessionLog::in_flight).
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()>;

    /// Records that outgoing `seq` has been used, storing `msg`, the message as sent (encoded,
    /// header and trailer included), for resends when given. On a FIXT session supporting more
    /// than one application version, a message in the default version is stored with its
    /// ApplVerID(1128) stated, so its bytes differ from those sent by that field.
    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()>;

    /// Stored messages with sequence numbers in `begin..=end`, in ascending order, as they were
    /// given to `record_outgoing`.
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>>;

    /// Makes the mutations made since the last commit durable (to the implementation's
    /// guarantee): at once, returning `Ok(None)`, or by running the returned [`Commit`], which
    /// the session's driver does off the async runtime and waits for before writing what depends
    /// on them. No other call is made while a returned commit runs. The default does nothing, for
    /// stores whose mutations are durable as they're made.
    ///
    /// A store that waits for a device (`fsync`) or a network should buffer its mutations and
    /// return a `Commit` here, so that a batch of messages costs one wait rather than one each.
    fn commit(&mut self) -> io::Result<Option<Commit>> {
        Ok(None)
    }

    /// Resets both sequence numbers to 1, discards stored messages, and clears
    /// [`created_at`](SessionLog::created_at), [`in_flight`](SessionLog::in_flight) and
    /// [`evicted_through`](SessionLog::evicted_through).
    fn reset(&mut self) -> io::Result<()>;

    /// The highest sequence number whose message the store has discarded to stay within a limit,
    /// if any: nothing at or below it can be resent, so a resend gap-fills it. The session logs a
    /// warning when a ResendRequest reaches it. Stores that keep every message until a reset
    /// return `None` (the default).
    fn evicted_through(&self) -> Option<u64> {
        None
    }

    /// The first of the incoming messages being handed to the application when this was last
    /// committed, if they hadn't all been handled by then: set by
    /// [`set_in_flight`](SessionLog::set_in_flight), cleared by
    /// [`set_next_incoming`](SessionLog::set_next_incoming). Still set after a crash, it tells
    /// the session that the messages from it on (up to a batch of them) may have been handled
    /// already. Stores that don't record it return `None` (the default), and redeliveries then go
    /// unmarked.
    fn in_flight(&self) -> Option<u64> {
        None
    }

    /// Records that incoming messages from `seq` on are about to be handed to the application.
    /// The default does nothing.
    fn set_in_flight(&mut self, _seq: u64) -> io::Result<()> {
        Ok(())
    }

    /// When the stored state was created or last reset, if recorded. Session schedules use it to
    /// tell whether the state belongs to the current session period. Stores that don't record it
    /// return `None` (the default), and schedule-based sequence resets are then skipped.
    fn created_at(&self) -> Option<UtcTimestamp> {
        None
    }

    /// Records when the stored state was created. The default does nothing.
    fn set_created_at(&mut self, _at: UtcTimestamp) -> io::Result<()> {
        Ok(())
    }
}

/// Work that makes a [`SessionLog`]'s buffered mutations durable, returned by
/// [`SessionLog::commit`] for its caller to run where blocking is harmless: the connection
/// driver runs it on a blocking thread.
pub struct Commit(Box<dyn FnOnce() -> io::Result<()> + Send>);

impl Commit {
    /// A commit that blocks the thread it runs on (writing and syncing files, say) until the
    /// mutations are durable.
    pub fn blocking(job: impl FnOnce() -> io::Result<()> + Send + 'static) -> Self {
        Self(Box::new(job))
    }

    /// Runs the commit on this thread, returning once the mutations are durable or it has failed.
    pub fn run(self) -> io::Result<()> {
        (self.0)()
    }
}

impl fmt::Debug for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Commit")
    }
}

/// Commits `log`'s mutations, running any [`Commit`] on this thread: for changes made outside a
/// connected session (operator commands on a disconnected one), which are rare.
pub(crate) fn commit_now(log: &mut dyn SessionLog) -> io::Result<()> {
    log.commit()?.map_or(Ok(()), Commit::run)
}

/// Behaviour every [`SessionStorage`] implementation must satisfy.
#[cfg(test)]
pub(crate) mod conformance {
    use super::*;
    use crate::codec::encode;
    use crate::fields::MsgType;
    use crate::message::{Message, tags};

    #[test]
    fn a_commit_runs_its_job() {
        assert!(Commit::blocking(|| Ok(())).run().is_ok());
        let failed = Commit::blocking(|| Err(io::Error::other("disk full"))).run();
        assert_eq!(failed.unwrap_err().to_string(), "disk full");
    }

    pub fn id(target: &str) -> SessionId {
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: target.into() }
    }

    pub fn app_message(seq: u64) -> Vec<u8> {
        encode(&message(seq)).unwrap()
    }

    fn message(seq: u64) -> Message {
        Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.4")
            .with(tags::MSG_TYPE, MsgType::ExecutionReport)
            .with(tags::MSG_SEQ_NUM, seq)
            .with(tags::EXEC_ID, format!("E{seq}"))
    }

    pub fn check(storage: &dyn SessionStorage) {
        {
            let mut log = storage.open(&id("A")).unwrap();
            assert_eq!((log.next_outgoing(), log.next_incoming()), (1, 1));
            log.record_outgoing(1, None).unwrap();
            log.record_outgoing(2, Some(&app_message(2))).unwrap();
            log.record_outgoing(3, None).unwrap();
            log.record_outgoing(4, Some(&app_message(4))).unwrap();
            log.set_next_incoming(7).unwrap();
            commit_now(log.as_mut()).unwrap();
        }

        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (5, 7), "state survives reopen");
        let sent = log.sent_messages(1, 4).unwrap();
        let seqs: Vec<u64> = sent.iter().map(|(s, _)| *s).collect();
        assert_eq!(seqs, [2, 4]);
        assert_eq!(sent[1].1, app_message(4));
        assert_eq!(log.sent_messages(3, 3).unwrap().len(), 0);

        // Sessions are independent.
        let other = storage.open(&id("B")).unwrap();
        assert_eq!((other.next_outgoing(), other.next_incoming()), (1, 1));
        drop(other);

        log.reset().unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (1, 1));
        assert!(log.sent_messages(1, u64::MAX).unwrap().is_empty());
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        assert_eq!(log.sent_messages(1, 1).unwrap().len(), 1, "reads see what isn't committed yet");
        commit_now(log.as_mut()).unwrap();
        drop(log);

        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(log.next_outgoing(), 2, "reset persists");
        assert_eq!(log.sent_messages(1, u64::MAX).unwrap().len(), 1);

        // Creation time: unknown until recorded, kept across reopening, cleared by reset.
        assert_eq!(log.created_at(), None);
        let created = UtcTimestamp::from_timestamp(1_790_000_000, 123_000_000).unwrap();
        log.set_created_at(created).unwrap();
        commit_now(log.as_mut()).unwrap();
        drop(log);
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(log.created_at(), Some(created));
        log.reset().unwrap();
        assert_eq!(log.created_at(), None);
        commit_now(log.as_mut()).unwrap();
        drop(log);
        assert_eq!(storage.open(&id("A")).unwrap().created_at(), None, "reset persists");

        check_in_flight(storage);
        check_data_fields(storage);
    }

    /// The messages in flight: kept across reopening, cleared by moving on or resetting.
    fn check_in_flight(storage: &dyn SessionStorage) {
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!(log.in_flight(), None);
        log.set_next_incoming(4).unwrap();
        log.set_in_flight(4).unwrap();
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        commit_now(log.as_mut()).unwrap();
        drop(log);
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!((log.next_incoming(), log.in_flight()), (4, Some(4)), "survives reopen");
        log.set_next_incoming(5).unwrap();
        assert_eq!(log.in_flight(), None);
        commit_now(log.as_mut()).unwrap();
        drop(log);
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!(log.in_flight(), None, "clearing persists");
        log.set_in_flight(5).unwrap();
        log.reset().unwrap();
        assert_eq!(log.in_flight(), None);
        commit_now(log.as_mut()).unwrap();
        drop(log);
        assert_eq!(storage.open(&id("C")).unwrap().in_flight(), None, "reset persists");
    }

    /// Data fields, a venue's own too, come back byte for byte.
    fn check_data_fields(storage: &dyn SessionStorage) {
        let msg = message(1).with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"\xff\x01\x0110=000\x01").with_data(
            5000,
            5001,
            b"a\x01\xfe",
        );
        {
            let mut log = storage.open(&id("D")).unwrap();
            log.record_outgoing(1, Some(&encode(&msg).unwrap())).unwrap();
            commit_now(log.as_mut()).unwrap();
        }
        let mut log = storage.open(&id("D")).unwrap();
        let sent = log.sent_messages(1, 1).unwrap();
        assert_eq!(sent[0].1, encode(&msg).unwrap());
    }
}
