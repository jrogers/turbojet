//! The standby's side: applying the active's batches to a store of its own.

use std::collections::HashMap;
use std::io;

use turbojet::store::{Fetched, Opened, SessionStorage, StoredSession};

use crate::frame::{Batch, Frame, NackReason, Op, Summary, VERSION};

/// A standby's copy of an active instance's session state: each batch the active commits is
/// applied to `S` and committed there before it's acknowledged, so `S` holds what the active
/// committed, ready to be taken over.
///
/// Sans-IO: [`on_frame`](Self::on_frame) takes each frame from the active and returns the reply,
/// running `S`'s jobs on the calling thread, so a network driver runs it on a thread of its own.
pub struct Replica<S> {
    storage: S,
    /// The highest epoch seen: batches under a lower one come from a deposed active.
    epoch: u64,
    /// The sessions bound on the current connection, by handle.
    bound: HashMap<u32, Bound>,
    /// The connections accepted so far: the current one is the last.
    connections: u64,
    /// Taken over: no more frames are applied.
    promoted: bool,
    /// The most outgoing numbers an active may leave unacknowledged per session.
    max_lag: u64,
}

impl<S> std::fmt::Debug for Replica<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Replica").field("epoch", &self.epoch).field("bound", &self.bound.len()).finish_non_exhaustive()
    }
}

struct Bound {
    log: Box<dyn StoredSession>,
    /// The last batch committed: the next must follow it.
    commit_no: u64,
}

impl<S: SessionStorage> Replica<S> {
    /// A standby keeping its copy in `storage`, having seen no epoch.
    pub fn new(storage: S) -> Self {
        Self { storage, epoch: 0, bound: HashMap::new(), connections: 0, promoted: false, max_lag: 0 }
    }

    /// Accepts actives that leave up to `n` outgoing numbers per session unacknowledged, rather
    /// than none: a bind for more is refused. On promotion each session's next outgoing number
    /// is to be skipped ahead by `n`, past any the active sent that this copy lacks.
    #[must_use]
    pub fn accepting_lag(mut self, n: u64) -> Self {
        self.max_lag = n;
        self
    }

    /// The highest epoch seen.
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// The connection accepted last, by the [`Frame::Hello`] that opened it: its bindings are
    /// the ones kept. A network driver notes it for [`disconnected_from`](Self::disconnected_from).
    pub fn connection(&self) -> u64 {
        self.connections
    }

    /// The store holding the copy.
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// Handles a frame from the active, returning the reply to send, if any.
    ///
    /// # Errors
    ///
    /// The store's failure, or a frame the active shouldn't send (a batch for a handle it hasn't
    /// bound, an unknown version, application records this standby wasn't told of): the
    /// connection must end, failing the active's commits under way.
    pub fn on_frame(&mut self, frame: Frame) -> io::Result<Option<Frame>> {
        if self.promoted {
            return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "this standby has taken over"));
        }
        match frame {
            Frame::Hello { version, epoch, node, app_record_kinds } => {
                if version != VERSION {
                    return Err(refused(format!("active {node} speaks version {version}, not {VERSION}")));
                }
                if !app_record_kinds.is_empty() {
                    return Err(refused(format!(
                        "active {node} replicates application records, which this standby can't"
                    )));
                }
                // A deposed active is told the epoch that replaced it, and changes nothing.
                if epoch < self.epoch {
                    return Ok(Some(Frame::Welcome { epoch: self.epoch }));
                }
                self.bound.clear();
                self.connections += 1;
                self.epoch = epoch;
                Ok(Some(Frame::Welcome { epoch }))
            }
            // A deposed active's handles would collide with the active's: it may bind none.
            Frame::Bind { handle, epoch, .. } if epoch < self.epoch => {
                Ok(Some(Frame::Nack { handle, reason: NackReason::StaleEpoch }))
            }
            // Promotion can only skip past what this standby accepts the active leaves unacknowledged.
            Frame::Bind { handle, max_lag, .. } if max_lag > self.max_lag => {
                Ok(Some(Frame::Nack { handle, reason: NackReason::Lag }))
            }
            Frame::Bind { handle, max_lag, session, .. } => {
                let mut log = match self.storage.begin_open(&session)? {
                    Opened::Ready(log) => log,
                    Opened::Pending(job) => job.run()?,
                };
                let summary = summary(log.as_mut(), max_lag)?;
                self.bound.insert(handle, Bound { log, commit_no: 0 });
                Ok(Some(Frame::Bound { handle, summary }))
            }
            Frame::Batch(batch) => self.on_batch(batch).map(Some),
            Frame::Welcome { .. } | Frame::Bound { .. } | Frame::Ack { .. } | Frame::Nack { .. } => {
                Err(refused("the active sent a standby's frame".into()))
            }
        }
    }

    /// Connection `connection` has ended: if it's the current one, its handles go, and the logs
    /// they held are closed. A later connection's are kept.
    pub fn disconnected_from(&mut self, connection: u64) {
        if connection == self.connections {
            self.disconnected();
        }
    }

    /// This instance takes over: the sessions' logs it holds are closed, releasing the store for
    /// the instance's own registry, and every frame from now on is refused, ending the old
    /// active's connections. Take the lease first, under a higher epoch, so the old active is
    /// fenced off wherever it is.
    ///
    /// With [`accepting_lag`](Self::accepting_lag), each session's next outgoing number is then
    /// skipped ahead by the lag accepted: the old active may have sent numbers this copy lacks,
    /// and one sent again would reach the counterparty too low. The skipped numbers hold no
    /// messages, so they're gap-filled when the counterparty asks for them.
    ///
    /// # Errors
    ///
    /// The store's failure, listing the sessions or skipping one ahead: the instance mustn't take
    /// over, since a session left behind would resume too low.
    pub fn promote(&mut self) -> io::Result<()> {
        self.promoted = true;
        self.disconnected();
        if self.max_lag == 0 {
            return Ok(());
        }
        for id in self.storage.sessions().run()? {
            let mut log = match self.storage.begin_open(&id)? {
                Opened::Ready(log) => log,
                Opened::Pending(job) => job.run()?,
            };
            let next = log.next_outgoing();
            log.record_outgoing(next + self.max_lag - 1, None)?;
            if let Some(commit) = log.commit()? {
                commit.run()?;
            }
        }
        Ok(())
    }

    /// The current connection has ended: its handles go, and the logs they held are closed.
    pub fn disconnected(&mut self) {
        self.bound.clear();
    }

    fn on_batch(&mut self, batch: Batch) -> io::Result<Frame> {
        let Batch { handle, epoch, commit_no, ops, app_records } = batch;
        if !app_records.is_empty() {
            return Err(refused("a batch has application records, which this standby can't apply".into()));
        }
        if epoch < self.epoch {
            return Ok(Frame::Nack { handle, reason: NackReason::StaleEpoch });
        }
        self.epoch = epoch;
        let bound =
            self.bound.get_mut(&handle).ok_or_else(|| refused(format!("a batch for unbound handle {handle}")))?;
        if commit_no != bound.commit_no + 1 || !fits(bound.log.as_ref(), &ops) {
            return Ok(Frame::Nack { handle, reason: NackReason::Diverged });
        }
        for op in ops {
            apply(bound.log.as_mut(), op)?;
        }
        if let Some(commit) = bound.log.commit()? {
            commit.run()?;
        }
        bound.commit_no = commit_no;
        Ok(Frame::Ack { handle, commit_no })
    }
}

/// What the active compares its own copy of a session with, for an active that leaves up to
/// `max_lag` outgoing numbers unacknowledged.
pub(crate) fn summary(log: &mut dyn StoredSession, max_lag: u64) -> io::Result<Summary> {
    Ok(Summary {
        next_outgoing: log.next_outgoing(),
        next_incoming: log.next_incoming(),
        in_flight: log.in_flight(),
        created_at: log.created_at(),
        marks: log.marks(),
        last_outgoing: last_outgoing(log, max_lag)?,
    })
}

/// A digest of the last `n` outgoing numbers before the next (FNV-1a, stable across builds, as
/// the two sides may run different ones): each message stored there, with its number and length.
/// By the bound on what an active leaves unacknowledged, only there can its copy and the
/// standby's differ with the same numbers. 0, reading nothing, for none.
fn last_outgoing(log: &mut dyn StoredSession, n: u64) -> io::Result<u64> {
    let next = log.next_outgoing();
    let begin = next.saturating_sub(n).max(1);
    if n == 0 || begin >= next {
        return Ok(0);
    }
    let messages = match log.fetch(begin, next - 1)? {
        Fetched::Ready(messages) => messages,
        Fetched::Pending(job) => job.run()?,
    };
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let len = |msg: &[u8]| u64::try_from(msg.len()).expect("a message's length fits");
    for (seq, msg) in &messages {
        for byte in seq.to_le_bytes().iter().chain(&len(msg).to_le_bytes()).chain(msg) {
            hash = (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    Ok(hash)
}

/// Whether `ops` follow on from `log`: each outgoing number recorded is one not yet used, and a
/// message stored takes the next. A batch that doesn't was made against a copy this one isn't.
fn fits(log: &dyn StoredSession, ops: &[Op]) -> bool {
    let mut next_outgoing = log.next_outgoing();
    for op in ops {
        match op {
            Op::Record { seq, msg } => {
                if *seq < next_outgoing || (msg.is_some() && *seq != next_outgoing) {
                    return false;
                }
                next_outgoing = seq + 1;
            }
            Op::Reset => next_outgoing = 1,
            _ => {}
        }
    }
    true
}

/// Makes the change `op` names to `log`.
pub(crate) fn apply(log: &mut dyn StoredSession, op: Op) -> io::Result<()> {
    match op {
        Op::SetNextIncoming(seq) => log.set_next_incoming(seq),
        Op::SetNextIncomingInFlight { seq, in_flight } => log.set_next_incoming_in_flight(seq, in_flight),
        Op::SetInFlight(seq) => log.set_in_flight(seq),
        Op::Record { seq, msg } => log.record_outgoing(seq, msg.as_deref()),
        Op::Reset => log.reset(),
        Op::SetCreatedAt(at) => log.set_created_at(at),
        Op::SetMarks(marks) => log.set_marks(marks),
    }
}

fn refused(why: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, why)
}
