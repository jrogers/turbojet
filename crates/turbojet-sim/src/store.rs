//! A store wrapper that tells the checker what each side committed to sending: every MsgSeqNum
//! it used and the message it stored, as the session recorded them, whether or not they reached
//! the wire.

use std::io;
use std::sync::{Arc, Mutex};

use turbojet::SessionId;

use crate::files::{DiskFiles, Snapshot, Tear};
use turbojet::fields::UtcTimestamp;
use turbojet::store::{SessionLog, SessionStorage};

/// One change to a side's outgoing sequence, in the order the session made them.
#[derive(Debug, Clone)]
pub enum Stored {
    /// MsgSeqNum `seq` was used: for an application message, with the message as sent.
    Sent { seq: u64, bytes: Option<Vec<u8>> },
    /// The next incoming MsgSeqNum was recorded as `seq`.
    Incoming { seq: u64 },
    /// Both sequence numbers went back to 1, and stored messages were discarded.
    Reset,
    /// The store was opened (at logon), with these numbers.
    Opened { next_outgoing: u64, next_incoming: u64 },
    /// The store failed to open.
    OpenFailed(String),
    /// A power loss tore this change: it took effect or not, as the next open shows.
    Uncertain(Box<Stored>),
}

pub type Ledger = Arc<Mutex<Vec<Stored>>>;

/// A store call a trap can catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Call {
    RecordOutgoing,
    SetNextIncoming,
    SetInFlight,
}

/// What happens at the next call of a kind: it takes effect or not, then fails; and with
/// `crash`, the process dies there too. With `tear` (a disk store, power lost), the call's writes
/// are cut short instead: what reached the device decides whether it took effect.
#[derive(Debug, Clone, Copy)]
pub struct Trap {
    pub call: Call,
    pub applies: bool,
    pub crash: bool,
    pub tear: Option<Tear>,
}

#[derive(Default)]
struct Traps {
    armed: Option<Trap>,
    /// A trap went off: whether it crashed the process.
    sprung: Option<bool>,
}

/// Wraps a store, appending to `ledger` whatever its logs record.
pub struct LedgerStorage {
    inner: Arc<dyn SessionStorage>,
    pub ledger: Ledger,
    traps: Arc<Mutex<Traps>>,
    /// A disk store's files, which a power loss tears.
    files: Option<Arc<DiskFiles>>,
    /// The files as the OS last wrote them back (a store without fsync).
    written_back: Mutex<Snapshot>,
}

impl LedgerStorage {
    pub fn new(inner: Arc<dyn SessionStorage>, files: Option<DiskFiles>) -> Self {
        let files = files.map(Arc::new);
        Self { inner, ledger: Ledger::default(), traps: Arc::default(), files, written_back: Mutex::default() }
    }

    /// The OS writes back what's been written so far.
    pub fn write_back(&self) {
        if let Some(files) = &self.files {
            *self.written_back.lock().unwrap() = files.snapshot();
        }
    }

    /// Power is lost (the process is already gone): the files keep what the OS had written back,
    /// and some of what followed.
    pub fn lose_power(&self, kept: u64, new_seqnums: bool) {
        if let Some(files) = &self.files {
            let mut written_back = self.written_back.lock().unwrap();
            files.revert(&written_back, kept, new_seqnums);
            // What survived is on the device now.
            *written_back = files.snapshot();
        }
    }

    /// Sets `trap` for the next call of its kind, replacing any not yet sprung.
    pub fn arm(&self, trap: Trap) {
        self.traps.lock().unwrap().armed = Some(trap);
    }

    /// Whether a trap has gone off since the last call, and if so whether it crashed the process.
    pub fn sprung(&self) -> Option<bool> {
        self.traps.lock().unwrap().sprung.take()
    }
}

impl SessionStorage for LedgerStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        let inner =
            self.inner.open(id).inspect_err(|e| self.ledger.lock().unwrap().push(Stored::OpenFailed(e.to_string())))?;
        let opened = Stored::Opened { next_outgoing: inner.next_outgoing(), next_incoming: inner.next_incoming() };
        self.ledger.lock().unwrap().push(opened);
        Ok(Box::new(LedgerLog {
            inner,
            ledger: self.ledger.clone(),
            traps: self.traps.clone(),
            files: self.files.clone(),
        }))
    }
}

struct LedgerLog {
    inner: Box<dyn SessionLog>,
    ledger: Ledger,
    traps: Arc<Mutex<Traps>>,
    files: Option<Arc<DiskFiles>>,
}

impl LedgerLog {
    /// Runs `call` through `f` unless a trap is armed for it: then `f` runs only if the trap says
    /// it applies, and the call fails either way.
    fn call(&mut self, call: Call, f: impl FnOnce(&mut Self) -> io::Result<()>) -> io::Result<()> {
        let trap = {
            let mut traps = self.traps.lock().unwrap();
            match traps.armed {
                Some(trap) if trap.call == call => {
                    traps.armed = None;
                    traps.sprung = Some(trap.crash);
                    Some(trap)
                }
                _ => None,
            }
        };
        match trap {
            None => f(self),
            Some(Trap { tear: Some(tear), .. }) if self.files.is_some() => {
                let files = self.files.clone().expect("checked");
                let before = files.snapshot();
                let recorded = self.ledger.lock().unwrap().len();
                f(self)?;
                files.tear(&before, tear);
                // What the call recorded may not have survived: the next open says.
                let mut ledger = self.ledger.lock().unwrap();
                if ledger.len() > recorded {
                    let change = ledger.pop().expect("one change");
                    ledger.push(Stored::Uncertain(Box::new(change)));
                }
                Err(io::Error::other(format!("power lost in {call:?}")))
            }
            Some(trap) => {
                if trap.applies {
                    f(self)?;
                }
                Err(io::Error::other(format!("simulated failure in {call:?}")))
            }
        }
    }
}

impl SessionLog for LedgerLog {
    fn next_outgoing(&self) -> u64 {
        self.inner.next_outgoing()
    }

    fn next_incoming(&self) -> u64 {
        self.inner.next_incoming()
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.call(Call::SetNextIncoming, |log| {
            log.inner.set_next_incoming(seq)?;
            log.ledger.lock().unwrap().push(Stored::Incoming { seq });
            Ok(())
        })
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        self.call(Call::RecordOutgoing, |log| {
            log.inner.record_outgoing(seq, msg)?;
            log.ledger.lock().unwrap().push(Stored::Sent { seq, bytes: msg.map(<[u8]>::to_vec) });
            Ok(())
        })
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
        self.call(Call::SetInFlight, |log| log.inner.set_in_flight(seq))
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.inner.created_at()
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        self.inner.set_created_at(at)
    }
}
