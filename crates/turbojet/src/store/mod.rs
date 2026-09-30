//! Session persistence: sequence numbers and sent application messages, kept per session so a
//! counterparty can reconnect and recover missed messages.

mod disk;
mod memory;

use std::fmt;
use std::io;

pub use disk::DiskStorage;
pub use memory::MemoryStorage;

use crate::fields::UtcTimestamp;
use crate::message::{DataFields, Message};

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

/// Persistent state of one session. Each mutation is durable (to the implementation's
/// guarantee) when it returns, so the session calls it before the message goes on the wire.
pub trait SessionLog: Send {
    /// The MsgSeqNum of the next message to send.
    fn next_outgoing(&self) -> u64;

    /// The MsgSeqNum expected on the next message from the counterparty.
    fn next_incoming(&self) -> u64;

    /// Records the next incoming sequence number, which also clears
    /// [`in_flight`](SessionLog::in_flight).
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()>;

    /// Records that outgoing `seq` has been used, storing `msg` for resends when given.
    fn record_outgoing(&mut self, seq: u64, msg: Option<&Message>) -> io::Result<()>;

    /// Stored messages with sequence numbers in `begin..=end`, in ascending order.
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Message)>>;

    /// Resets both sequence numbers to 1, discards stored messages, and clears
    /// [`created_at`](SessionLog::created_at) and [`in_flight`](SessionLog::in_flight).
    fn reset(&mut self) -> io::Result<()>;

    /// The incoming message being handed to the application when this was last recorded, if it
    /// hadn't been handled by then: set by [`set_in_flight`](SessionLog::set_in_flight), cleared
    /// by [`set_next_incoming`](SessionLog::set_next_incoming). Still set after a crash, it tells
    /// the session that message may have been handled already. Stores that don't record it return
    /// `None` (the default), and redeliveries then go unmarked.
    fn in_flight(&self) -> Option<u64> {
        None
    }

    /// Records that incoming `seq` is about to be handed to the application. The default does
    /// nothing.
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

    /// Tells the log the session's data fields ([`SessionConfig::data_fields`]), for a store that
    /// decodes what it stored to know how long each data value is. The session calls it once it
    /// has opened the log. The default does nothing.
    ///
    /// [`SessionConfig::data_fields`]: crate::SessionConfig::data_fields
    fn set_data_fields(&mut self, _data: &DataFields) {}
}

/// Behaviour every [`SessionStorage`] implementation must satisfy.
#[cfg(test)]
pub(crate) mod conformance {
    use super::*;
    use crate::fields::MsgType;
    use crate::message::tags;

    pub fn id(target: &str) -> SessionId {
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: target.into() }
    }

    pub fn app_message(seq: u64) -> Message {
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
        }

        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (5, 7), "state survives reopen");
        let sent = log.sent_messages(1, 4).unwrap();
        let seqs: Vec<u64> = sent.iter().map(|(s, _)| *s).collect();
        assert_eq!(seqs, [2, 4]);
        assert_eq!(sent[1].1.get(tags::EXEC_ID), Some("E4"));
        assert_eq!(log.sent_messages(3, 3).unwrap().len(), 0);

        // Sessions are independent.
        let other = storage.open(&id("B")).unwrap();
        assert_eq!((other.next_outgoing(), other.next_incoming()), (1, 1));
        drop(other);

        log.reset().unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (1, 1));
        assert!(log.sent_messages(1, u64::MAX).unwrap().is_empty());
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        drop(log);

        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(log.next_outgoing(), 2, "reset persists");
        assert_eq!(log.sent_messages(1, u64::MAX).unwrap().len(), 1);

        // Creation time: unknown until recorded, kept across reopening, cleared by reset.
        assert_eq!(log.created_at(), None);
        let created = UtcTimestamp::from_timestamp(1_790_000_000, 123_000_000).unwrap();
        log.set_created_at(created).unwrap();
        drop(log);
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(log.created_at(), Some(created));
        log.reset().unwrap();
        assert_eq!(log.created_at(), None);
        drop(log);
        assert_eq!(storage.open(&id("A")).unwrap().created_at(), None, "reset persists");

        // The message in flight: kept across reopening, cleared by moving on or resetting.
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!(log.in_flight(), None);
        log.set_next_incoming(4).unwrap();
        log.set_in_flight(4).unwrap();
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        drop(log);
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!((log.next_incoming(), log.in_flight()), (4, Some(4)), "survives reopen");
        log.set_next_incoming(5).unwrap();
        assert_eq!(log.in_flight(), None);
        drop(log);
        let mut log = storage.open(&id("C")).unwrap();
        assert_eq!(log.in_flight(), None, "clearing persists");
        log.set_in_flight(5).unwrap();
        log.reset().unwrap();
        assert_eq!(log.in_flight(), None);
        drop(log);
        assert_eq!(storage.open(&id("C")).unwrap().in_flight(), None, "reset persists");

        // Data fields, a venue's own too, come back byte for byte.
        let data = DataFields::standard().with(5000, 5001);
        let msg = app_message(1).with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"\xff\x01\x0110=000\x01").with_data(
            5000,
            5001,
            b"a\x01\xfe",
        );
        {
            let mut log = storage.open(&id("D")).unwrap();
            log.set_data_fields(&data);
            log.record_outgoing(1, Some(&msg)).unwrap();
        }
        let mut log = storage.open(&id("D")).unwrap();
        log.set_data_fields(&data);
        let sent = log.sent_messages(1, 1).unwrap();
        for tag in [tags::RAW_DATA_LENGTH, tags::RAW_DATA, 5000, 5001] {
            assert_eq!(sent[0].1.get_bytes(tag), msg.get_bytes(tag), "{tag}");
        }
    }
}
