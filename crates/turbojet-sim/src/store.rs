//! A store wrapper that tells the checker what each side committed to sending: every MsgSeqNum
//! it used and the message it stored, as the session recorded them, whether or not they reached
//! the wire. Changes reach the ledger when the store commits them; ones that may or may not have
//! taken effect (a commit cut short, or changes a store kept though they were never committed)
//! reach it as uncertain, for the next open to settle.

use std::io;
use std::sync::{Arc, Mutex};

use turbojet::SessionId;

use crate::files::{DiskFiles, Snapshot, Tear};
use turbojet::fields::UtcTimestamp;
use turbojet::store::{Commit, SessionLog, SessionStorage};

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
    /// These changes, in order, may have taken effect, wholly or in part, as the next open shows:
    /// a power loss tore their commit, or the session ended before committing them.
    Uncertain(Vec<Stored>),
}

pub type Ledger = Arc<Mutex<Vec<Stored>>>;

/// A store call a trap can catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Call {
    RecordOutgoing,
    SetNextIncoming,
    SetInFlight,
    Commit,
}

/// What happens at the next call of a kind: it takes effect or not, then fails; and with
/// `crash`, the process dies there too. With `tear` (a disk store's commit, power lost), its
/// writes are cut short instead: what reached the device decides what took effect.
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
    /// A planted bug: from this many messages stored on, keep each one's number but not the message.
    forget: Arc<Mutex<Option<u64>>>,
    /// Commits take a while: the store hands each back to the driver to run, as one with fsync
    /// does, and the world decides when it finishes.
    slow: bool,
    /// A planted bug: a commit reaches the ledger only with the next one, so it's reported done
    /// before it is.
    early: bool,
}

impl LedgerStorage {
    pub fn new(inner: Arc<dyn SessionStorage>, files: Option<DiskFiles>) -> Self {
        let files = files.map(Arc::new);
        Self {
            inner,
            ledger: Ledger::default(),
            traps: Arc::default(),
            files,
            written_back: Mutex::default(),
            forget: Arc::default(),
            slow: false,
            early: false,
        }
    }

    /// Commits take a while to finish; with `early`, the planted bug of reporting them done first.
    pub fn slow_commits(mut self, early: bool) -> Self {
        self.slow = true;
        self.early = early;
        self
    }

    /// Plants a bug: from the `n`th message stored, keep its number but not the message.
    pub fn forget_messages_from(&self, n: u64) {
        *self.forget.lock().unwrap() = Some(n);
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
            forget: self.forget.clone(),
            pending: Vec::new(),
            late: Vec::new(),
            slow: self.slow,
            early: self.early,
        }))
    }
}

struct LedgerLog {
    inner: Box<dyn SessionLog>,
    ledger: Ledger,
    traps: Arc<Mutex<Traps>>,
    files: Option<Arc<DiskFiles>>,
    forget: Arc<Mutex<Option<u64>>>,
    /// Changes since the last commit.
    pending: Vec<Stored>,
    /// The planted bug: the last commit's changes, reported done but not yet in the ledger.
    late: Vec<Stored>,
    slow: bool,
    early: bool,
}

/// A commit's changes, on their way to the ledger: there once the commit has run, or uncertain
/// if it never does (the connection or the process ended first), since the store may have
/// written them already.
struct InCommit {
    changes: Vec<Stored>,
    ledger: Ledger,
}

impl InCommit {
    fn done(mut self) {
        self.ledger.lock().unwrap().append(&mut self.changes);
    }
}

impl Drop for InCommit {
    fn drop(&mut self) {
        if !self.changes.is_empty() {
            let changes = std::mem::take(&mut self.changes);
            self.ledger.lock().unwrap().push(Stored::Uncertain(changes));
        }
    }
}

impl Drop for LedgerLog {
    /// Changes never committed may still have taken effect: a memory store makes them at once.
    fn drop(&mut self) {
        let mut ledger = self.ledger.lock().unwrap();
        ledger.append(&mut self.late);
        if !self.pending.is_empty() {
            ledger.push(Stored::Uncertain(std::mem::take(&mut self.pending)));
        }
    }
}

impl LedgerLog {
    /// The trap armed for `call`, if any, which this call springs.
    fn spring(&self, call: Call) -> Option<Trap> {
        let mut traps = self.traps.lock().unwrap();
        match traps.armed {
            Some(trap) if trap.call == call => {
                traps.armed = None;
                traps.sprung = Some(trap.crash);
                Some(trap)
            }
            _ => None,
        }
    }

    /// Runs `call` through `f` unless a trap is armed for it: then `f` runs only if the trap says
    /// it applies, and the call fails either way.
    fn call(&mut self, call: Call, f: impl FnOnce(&mut Self) -> io::Result<()>) -> io::Result<()> {
        match self.spring(call) {
            None => f(self),
            Some(trap) => {
                if trap.applies {
                    f(self)?;
                }
                Err(io::Error::other(format!("simulated failure in {call:?}")))
            }
        }
    }

    /// The changes since the last commit, as one about to finish: with the planted bug, the
    /// last commit's instead, these held back for the next.
    fn committing(&mut self) -> Vec<Stored> {
        let changes = std::mem::take(&mut self.pending);
        if self.early { std::mem::replace(&mut self.late, changes) } else { changes }
    }

    /// The store's commit, sprung by a trap: it takes effect or not and fails, or with `tear`,
    /// its writes are cut short by a power loss, and what took effect is uncertain.
    fn trapped_commit(&mut self, trap: Trap) -> io::Result<Option<Commit>> {
        match (trap.tear, self.files.clone()) {
            (Some(tear), Some(files)) => {
                let before = files.snapshot();
                // A disk store without fsync, as the simulator's are, writes in the call.
                assert!(self.inner.commit()?.is_none(), "the simulator's disk stores commit at once");
                files.tear(&before, tear);
                let changes = std::mem::take(&mut self.pending);
                self.ledger.lock().unwrap().push(Stored::Uncertain(changes));
                Err(io::Error::other("power lost in Commit"))
            }
            _ => {
                if trap.applies {
                    assert!(self.inner.commit()?.is_none(), "the simulator's stores commit at once");
                    let changes = std::mem::take(&mut self.pending);
                    self.ledger.lock().unwrap().extend(changes);
                } else {
                    // Not committed, so lost with the log: a memory store keeps them, though.
                    let changes = std::mem::take(&mut self.pending);
                    self.ledger.lock().unwrap().push(Stored::Uncertain(changes));
                }
                Err(io::Error::other("simulated failure in Commit"))
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
            log.pending.push(Stored::Incoming { seq });
            Ok(())
        })
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        self.call(Call::RecordOutgoing, |log| {
            // The planted bug: the number is used, the message isn't kept, the ledger is told it was.
            let forget = msg.is_some()
                && log.forget.lock().unwrap().as_mut().is_some_and(|n| {
                    *n = n.saturating_sub(1);
                    *n == 0
                });
            log.inner.record_outgoing(seq, if forget { None } else { msg })?;
            log.pending.push(Stored::Sent { seq, bytes: msg.map(<[u8]>::to_vec) });
            Ok(())
        })
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        self.inner.sent_messages(begin, end)
    }

    /// Written at once, superseding what came before it, committed or not.
    fn reset(&mut self) -> io::Result<()> {
        self.inner.reset()?;
        let mut ledger = self.ledger.lock().unwrap();
        ledger.append(&mut self.late);
        ledger.append(&mut self.pending);
        ledger.push(Stored::Reset);
        Ok(())
    }

    fn commit(&mut self) -> io::Result<Option<Commit>> {
        if let Some(trap) = self.spring(Call::Commit) {
            return self.trapped_commit(trap);
        }
        // The inner stores commit at once (a disk store without fsync, or memory); a slow one
        // hands the commit back, finishing when the world says. With nothing to commit, there's
        // nothing to wait for.
        assert!(self.inner.commit()?.is_none(), "the simulator's stores commit at once");
        if self.pending.is_empty() {
            return Ok(None);
        }
        let in_commit = InCommit { changes: self.committing(), ledger: self.ledger.clone() };
        if self.slow {
            Ok(Some(Commit::blocking(move || {
                in_commit.done();
                Ok(())
            })))
        } else {
            in_commit.done();
            Ok(None)
        }
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
