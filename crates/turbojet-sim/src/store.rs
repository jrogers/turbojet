//! A store wrapper that tells the checker what each side committed to sending: every MsgSeqNum
//! it used and the message it stored, as the session recorded them, whether or not they reached
//! the wire.

use std::io;
use std::sync::{Arc, Mutex};

use turbojet::SessionId;
use turbojet::fields::UtcTimestamp;
use turbojet::store::{SessionLog, SessionStorage};

/// One change to a side's outgoing sequence, in the order the session made them.
#[derive(Debug, Clone)]
pub enum Stored {
    /// MsgSeqNum `seq` was used: for an application message, with the message as sent.
    Sent { seq: u64, bytes: Option<Vec<u8>> },
    /// Both sequence numbers went back to 1, and stored messages were discarded.
    Reset,
}

pub type Ledger = Arc<Mutex<Vec<Stored>>>;

/// Wraps a store, appending to `ledger` whatever its logs record.
pub struct LedgerStorage {
    inner: Arc<dyn SessionStorage>,
    pub ledger: Ledger,
}

impl LedgerStorage {
    pub fn new(inner: Arc<dyn SessionStorage>) -> Self {
        Self { inner, ledger: Ledger::default() }
    }
}

impl SessionStorage for LedgerStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(LedgerLog { inner: self.inner.open(id)?, ledger: self.ledger.clone() }))
    }
}

struct LedgerLog {
    inner: Box<dyn SessionLog>,
    ledger: Ledger,
}

impl SessionLog for LedgerLog {
    fn next_outgoing(&self) -> u64 {
        self.inner.next_outgoing()
    }

    fn next_incoming(&self) -> u64 {
        self.inner.next_incoming()
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.inner.set_next_incoming(seq)
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        self.inner.record_outgoing(seq, msg)?;
        self.ledger.lock().unwrap().push(Stored::Sent { seq, bytes: msg.map(<[u8]>::to_vec) });
        Ok(())
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        self.inner.sent_messages(begin, end)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.inner.reset()?;
        self.ledger.lock().unwrap().push(Stored::Reset);
        Ok(())
    }

    fn in_flight(&self) -> Option<u64> {
        self.inner.in_flight()
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.inner.set_in_flight(seq)
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.inner.created_at()
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        self.inner.set_created_at(at)
    }
}
