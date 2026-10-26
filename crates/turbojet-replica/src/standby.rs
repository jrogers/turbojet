//! The standby's side: applying the active's batches to a store of its own.

use std::collections::HashMap;
use std::io;

use turbojet::store::{Opened, SessionStorage, StoredSession};

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
        Self { storage, epoch: 0, bound: HashMap::new(), connections: 0, promoted: false }
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
            Frame::Bind { handle, session, .. } => {
                let log = match self.storage.begin_open(&session)? {
                    Opened::Ready(log) => log,
                    Opened::Pending(job) => job.run()?,
                };
                let summary = summary(log.as_ref());
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
    pub fn promote(&mut self) {
        self.promoted = true;
        self.disconnected();
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

/// What the active compares its own copy of a session with.
pub(crate) fn summary(log: &dyn StoredSession) -> Summary {
    Summary {
        next_outgoing: log.next_outgoing(),
        next_incoming: log.next_incoming(),
        in_flight: log.in_flight(),
        created_at: log.created_at(),
        marks: log.marks(),
    }
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
