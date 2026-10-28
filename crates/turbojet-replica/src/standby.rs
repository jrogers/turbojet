//! The standby's side: applying the active's batches to a store of its own.

use std::collections::HashMap;
use std::io;

use turbojet::store::{Fetched, Opened, SessionStorage, StoredSession};

use crate::frame::{Batch, Frame, NackReason, Op, Part, Summary, VERSION};

/// The most message bytes a standby holds of a session sent whole before its last part: four
/// times `DiskStorage`'s default budget for a session. A session past it is refused, ending the
/// connection, so the standby's memory is bounded by its connection's sessions, one at a time.
const MAX_WHOLE_SESSION_BYTES: usize = 4 << 30;

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
    /// The last batch committed, or held: the next must follow it.
    commit_no: u64,
    /// The parts of a session sent whole that have arrived, held until the last.
    held: Vec<Op>,
    /// The message bytes in `held`.
    held_bytes: usize,
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
                self.bound.insert(handle, Bound { log, commit_no: 0, held: Vec::new(), held_bytes: 0 });
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
    /// messages, so they're gap-filled when the counterparty asks for them. Skipping needs a
    /// store that lists its sessions ([`SessionStorage::sessions`]; the built-in ones do): with
    /// one that doesn't, this fails.
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
        let Batch { handle, epoch, commit_no, part, ops, app_records } = batch;
        if !app_records.is_empty() {
            return Err(refused("a batch has application records, which this standby can't apply".into()));
        }
        if epoch < self.epoch {
            return Ok(Frame::Nack { handle, reason: NackReason::StaleEpoch });
        }
        self.epoch = epoch;
        let bound =
            self.bound.get_mut(&handle).ok_or_else(|| refused(format!("a batch for unbound handle {handle}")))?;
        if commit_no != bound.commit_no + 1 {
            bound.held.clear();
            bound.held_bytes = 0;
            return Ok(Frame::Nack { handle, reason: NackReason::Diverged });
        }
        let ops = match part {
            Part::Commit if !bound.held.is_empty() => {
                return Err(refused(format!("a commit on handle {handle} while its session is sent whole")));
            }
            Part::Commit => ops,
            Part::Whole | Part::LastWhole => {
                hold(bound, handle, ops)?;
                if part == Part::Whole {
                    bound.commit_no = commit_no;
                    return Ok(Frame::Ack { handle, commit_no });
                }
                bound.held_bytes = 0;
                std::mem::take(&mut bound.held)
            }
        };
        if !fits(bound.log.as_ref(), &ops) {
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

/// Holds a part of a session sent whole, until the last: the first must begin with a reset, and
/// all of them hold no more than [`MAX_WHOLE_SESSION_BYTES`] of messages.
fn hold(bound: &mut Bound, handle: u32, ops: Vec<Op>) -> io::Result<()> {
    if bound.held.is_empty() && ops.first() != Some(&Op::Reset) {
        return Err(refused(format!("a session sent whole on handle {handle} that doesn't begin with a reset")));
    }
    let bytes: usize =
        ops.iter().map(|op| if let Op::Record { msg: Some(msg), .. } = op { msg.len() } else { 0 }).sum();
    bound.held_bytes = bound.held_bytes.saturating_add(bytes);
    if bound.held_bytes > MAX_WHOLE_SESSION_BYTES {
        return Err(refused(format!(
            "a session sent whole on handle {handle} is past {MAX_WHOLE_SESSION_BYTES} bytes of messages"
        )));
    }
    bound.held.extend(ops);
    Ok(())
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

#[cfg(test)]
mod tests {
    use turbojet::MemoryStorage;
    use turbojet::store::conformance;

    use super::*;

    fn bound() -> Replica<MemoryStorage> {
        let mut replica = Replica::new(MemoryStorage::new());
        let hello = Frame::Hello { version: VERSION, epoch: 1, node: "a".into(), app_record_kinds: Vec::new() };
        replica.on_frame(hello).unwrap();
        replica.on_frame(Frame::Bind { handle: 1, epoch: 1, max_lag: 0, session: conformance::id("A") }).unwrap();
        replica
    }

    fn send(
        replica: &mut Replica<MemoryStorage>,
        commit_no: u64,
        part: Part,
        ops: Vec<Op>,
    ) -> io::Result<Option<Frame>> {
        replica.on_frame(Frame::Batch(Batch { handle: 1, epoch: 1, commit_no, part, ops, app_records: Vec::new() }))
    }

    fn records(seqs: std::ops::RangeInclusive<u64>) -> Vec<Op> {
        seqs.map(|seq| Op::Record { seq, msg: Some(conformance::app_message(seq)) }).collect()
    }

    fn next_outgoing(replica: &Replica<MemoryStorage>) -> u64 {
        replica.storage().open(&conformance::id("A")).unwrap().next_outgoing()
    }

    /// The parts of a session sent whole change nothing until the last: one cut short, by a lost
    /// connection, leaves the copy as it was rather than reset and partly refilled, which a
    /// failover would resume from below numbers already sent.
    #[test]
    fn a_session_sent_whole_is_applied_only_at_its_last_part() {
        let mut replica = bound();
        send(&mut replica, 1, Part::Commit, records(1..=5)).unwrap();
        let first = [vec![Op::Reset], records(1..=3)].concat();
        assert_eq!(
            send(&mut replica, 2, Part::Whole, first.clone()).unwrap(),
            Some(Frame::Ack { handle: 1, commit_no: 2 })
        );
        assert_eq!(next_outgoing(&replica), 6, "held, not applied");
        replica.disconnected();
        assert_eq!(next_outgoing(&replica), 6, "a send cut short changes nothing");

        let mut replica = bound();
        send(&mut replica, 1, Part::Commit, records(1..=5)).unwrap();
        send(&mut replica, 2, Part::Whole, first).unwrap();
        assert_eq!(
            send(&mut replica, 3, Part::LastWhole, records(4..=7)).unwrap(),
            Some(Frame::Ack { handle: 1, commit_no: 3 })
        );
        assert_eq!(next_outgoing(&replica), 8);
        assert_eq!(
            send(&mut replica, 4, Part::Commit, records(8..=8)).unwrap(),
            Some(Frame::Ack { handle: 1, commit_no: 4 })
        );
    }

    #[test]
    fn a_session_sent_whole_must_begin_with_a_reset_and_not_be_interleaved() {
        assert!(send(&mut bound(), 1, Part::Whole, records(1..=2)).is_err(), "no reset");
        assert!(send(&mut bound(), 1, Part::LastWhole, records(1..=2)).is_err(), "no reset");
        let mut replica = bound();
        send(&mut replica, 1, Part::Whole, vec![Op::Reset]).unwrap();
        assert!(send(&mut replica, 2, Part::Commit, records(1..=1)).is_err(), "a commit inside a session sent whole");
    }

    /// A part out of order drops what was held: the active sends the session whole again.
    #[test]
    fn a_part_out_of_order_drops_what_was_held() {
        let mut replica = bound();
        send(&mut replica, 1, Part::Whole, vec![Op::Reset]).unwrap();
        assert_eq!(
            send(&mut replica, 3, Part::LastWhole, records(1..=1)).unwrap(),
            Some(Frame::Nack { handle: 1, reason: NackReason::Diverged })
        );
        assert_eq!(next_outgoing(&replica), 1);
    }
}
