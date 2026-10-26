//! The frames an active instance and its standby exchange, and their wire form.
//!
//! Each frame is a little-endian `u32` length, of what follows it, then a kind byte and the
//! frame's fields. Integers are little-endian; byte strings and text are a `u32` length and their
//! bytes. Decoding never trusts a length: a frame longer than [`MAX_FRAME_BYTES`], or one whose
//! fields run past its end, is an error rather than an allocation.
//!
//! The active sends [`Frame::Hello`], is answered with [`Frame::Welcome`], then [`Frame::Bind`]s
//! a handle to each session it replicates, answered with the standby's [`Summary`] of it, and
//! sends a [`Frame::Batch`] for each of its commits, which the standby answers with
//! [`Frame::Ack`] once its own store has committed it, or [`Frame::Nack`].

use std::fmt;

use turbojet::SessionId;
use turbojet::fields::{FromFix, ToFix, UtcTimestamp};
use turbojet::store::Marks;

/// The version of the wire form this crate speaks.
pub const VERSION: u16 = 2;

/// The longest frame either side sends or accepts, length prefix excluded. A batch holds what one
/// commit stored, which a session's driver bounds well below this; a standby catching up is sent
/// what it lacks in batches of [`MAX_FRAME_BYTES`] at most.
pub const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

/// A frame between an active instance and its standby.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    /// From the active, first: who it is and the epoch it holds the lease for, and the kinds of
    /// application record its batches may carry.
    Hello { version: u16, epoch: u64, node: String, app_record_kinds: Vec<u16> },
    /// From the standby: it accepts the active, at the highest epoch it has seen.
    Welcome { epoch: u64 },
    /// From the active, holding `epoch`: `handle` names `session` in the batches that follow, of
    /// which the active may leave up to `max_lag` outgoing numbers unacknowledged.
    Bind { handle: u32, epoch: u64, max_lag: u64, session: SessionId },
    /// From the standby: its copy of the session `handle` names, for the active to compare.
    Bound { handle: u32, summary: Summary },
    /// From the active: one commit of a session's changes.
    Batch(Batch),
    /// From the standby: its store has committed the session's batch `commit_no`.
    Ack { handle: u32, commit_no: u64 },
    /// From the standby: it refused a batch, and why.
    Nack { handle: u32, reason: NackReason },
}

/// A session's stored state, as far as an active and its standby compare it: when they differ,
/// the active sends the standby the whole session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub next_outgoing: u64,
    pub next_incoming: u64,
    pub in_flight: Option<u64>,
    pub created_at: Option<UtcTimestamp>,
    pub marks: Marks,
}

/// One commit's changes to a session's stored state, in the order they were made, and the
/// application's records committed with them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub handle: u32,
    /// The epoch of the lease the active held when it committed.
    pub epoch: u64,
    /// The session's commits on this connection, counted from 1: one missing means the standby
    /// has diverged.
    pub commit_no: u64,
    pub ops: Vec<Op>,
    /// Opaque to Turbojet, applied atomically with `ops`; of the kinds named in the
    /// [`Frame::Hello`].
    pub app_records: Vec<AppRecord>,
}

/// A change to a session's stored state: one call of [`StoredSession`](turbojet::store::StoredSession)'s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    SetNextIncoming(u64),
    SetNextIncomingInFlight {
        seq: u64,
        in_flight: u64,
    },
    SetInFlight(u64),
    /// Outgoing `seq` used, with the message stored for resends if there is one.
    Record {
        seq: u64,
        msg: Option<Vec<u8>>,
    },
    Reset,
    SetCreatedAt(UtcTimestamp),
    SetMarks(Marks),
}

/// An application's own record, replicated with a session's commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppRecord {
    pub kind: u16,
    pub bytes: Vec<u8>,
}

/// Why a standby refused a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NackReason {
    /// Its epoch is below one the standby has seen: the active has been deposed.
    StaleEpoch,
    /// It doesn't follow the last batch the standby committed: the session must be sent whole.
    Diverged,
    /// Its bind's `max_lag` is more than the standby accepts.
    Lag,
}

/// A frame that can't be decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    /// Its length is past [`MAX_FRAME_BYTES`].
    TooLong(usize),
    /// Its kind, or a field's, isn't one this version knows.
    Unknown(&'static str, u8),
    /// It ends before its fields do, or has bytes past them.
    Truncated,
    /// A field's value isn't valid: text that isn't UTF-8, a timestamp, marks or session that
    /// don't parse.
    Invalid(&'static str),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong(len) => write!(f, "replication frame of {len} bytes, past the limit of {MAX_FRAME_BYTES}"),
            Self::Unknown(what, value) => write!(f, "unknown replication {what} {value}"),
            Self::Truncated => f.write_str("replication frame ends before its fields do"),
            Self::Invalid(what) => write!(f, "replication frame has an invalid {what}"),
        }
    }
}

impl std::error::Error for FrameError {}

mod kind {
    pub const HELLO: u8 = 1;
    pub const WELCOME: u8 = 2;
    pub const BIND: u8 = 3;
    pub const BATCH: u8 = 4;
    pub const ACK: u8 = 5;
    pub const NACK: u8 = 6;
    pub const BOUND: u8 = 7;
}

mod op {
    pub const SET_NEXT_INCOMING: u8 = 1;
    pub const SET_NEXT_INCOMING_IN_FLIGHT: u8 = 2;
    pub const SET_IN_FLIGHT: u8 = 3;
    pub const RECORD: u8 = 4;
    pub const RECORD_NONE: u8 = 5;
    pub const RESET: u8 = 6;
    pub const SET_CREATED_AT: u8 = 7;
    pub const SET_MARKS: u8 = 8;
}

impl Frame {
    /// Appends the frame's wire form, length prefix included, to `out`.
    ///
    /// # Panics
    ///
    /// If the frame is longer than [`MAX_FRAME_BYTES`]: the sender must split what it sends.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let start = out.len();
        out.extend_from_slice(&[0; 4]);
        let mut w = Writer(out);
        match self {
            Self::Hello { version, epoch, node, app_record_kinds } => {
                w.u8(kind::HELLO);
                w.u16(*version);
                w.u64(*epoch);
                w.bytes(node.as_bytes());
                w.u32(len_u32(app_record_kinds.len()));
                for kind in app_record_kinds {
                    w.u16(*kind);
                }
            }
            Self::Welcome { epoch } => {
                w.u8(kind::WELCOME);
                w.u64(*epoch);
            }
            Self::Bind { handle, epoch, max_lag, session } => {
                w.u8(kind::BIND);
                w.u32(*handle);
                w.u64(*epoch);
                w.u64(*max_lag);
                for part in [&session.begin_string, &session.sender_comp_id, &session.target_comp_id] {
                    w.bytes(part.as_bytes());
                }
                w.bytes(session.key_suffix().as_bytes());
            }
            Self::Bound { handle, summary } => {
                w.u8(kind::BOUND);
                w.u32(*handle);
                w.u64(summary.next_outgoing);
                w.u64(summary.next_incoming);
                w.u64(summary.in_flight.unwrap_or(0));
                w.bytes(summary.created_at.map(|at| at.to_fix()).unwrap_or_default().as_bytes());
                w.bytes(summary.marks.to_string().as_bytes());
            }
            Self::Batch(batch) => {
                w.u8(kind::BATCH);
                w.u32(batch.handle);
                w.u64(batch.epoch);
                w.u64(batch.commit_no);
                w.u32(len_u32(batch.ops.len()));
                for op in &batch.ops {
                    w.op(op);
                }
                w.u32(len_u32(batch.app_records.len()));
                for record in &batch.app_records {
                    w.u16(record.kind);
                    w.bytes(&record.bytes);
                }
            }
            Self::Ack { handle, commit_no } => {
                w.u8(kind::ACK);
                w.u32(*handle);
                w.u64(*commit_no);
            }
            Self::Nack { handle, reason } => {
                w.u8(kind::NACK);
                w.u32(*handle);
                w.u8(match reason {
                    NackReason::StaleEpoch => 1,
                    NackReason::Diverged => 2,
                    NackReason::Lag => 3,
                });
            }
        }
        let len = out.len() - start - 4;
        assert!(len <= MAX_FRAME_BYTES, "a replication frame of {len} bytes must be split");
        out[start..start + 4].copy_from_slice(&len_u32(len).to_le_bytes());
    }

    /// Decodes the frame at the start of `buf`: the frame and the bytes it took, `None` if `buf`
    /// doesn't hold all of it yet.
    ///
    /// # Errors
    ///
    /// A [`FrameError`] if the frame is too long or malformed: the connection can't go on.
    pub fn decode(buf: &[u8]) -> Result<Option<(Self, usize)>, FrameError> {
        let Some(prefix) = buf.first_chunk::<4>() else { return Ok(None) };
        let len = u32::from_le_bytes(*prefix) as usize;
        if len > MAX_FRAME_BYTES {
            return Err(FrameError::TooLong(len));
        }
        let Some(body) = buf.get(4..4 + len) else { return Ok(None) };
        let mut r = Reader(body);
        let frame = match r.u8()? {
            kind::HELLO => {
                let (version, epoch, node) = (r.u16()?, r.u64()?, r.text()?);
                let count = r.count(2)?;
                let app_record_kinds = (0..count).map(|_| r.u16()).collect::<Result<_, _>>()?;
                Self::Hello { version, epoch, node, app_record_kinds }
            }
            kind::WELCOME => Self::Welcome { epoch: r.u64()? },
            kind::BIND => {
                let (handle, epoch, max_lag) = (r.u32()?, r.u64()?, r.u64()?);
                let (begin_string, sender, target, suffix) = (r.text()?, r.text()?, r.text()?, r.text()?);
                let session =
                    SessionId::from_key(begin_string, sender, target, &suffix).ok_or(FrameError::Invalid("session"))?;
                Self::Bind { handle, epoch, max_lag, session }
            }
            kind::BOUND => {
                let (handle, next_outgoing, next_incoming, in_flight) = (r.u32()?, r.u64()?, r.u64()?, r.u64()?);
                let created_at = r.text()?;
                let created_at = (!created_at.is_empty())
                    .then(|| UtcTimestamp::from_fix(&created_at).map_err(|_| FrameError::Invalid("timestamp")))
                    .transpose()?;
                let marks = r.text()?.parse().map_err(|_| FrameError::Invalid("marks"))?;
                let in_flight = (in_flight > 0).then_some(in_flight);
                Self::Bound { handle, summary: Summary { next_outgoing, next_incoming, in_flight, created_at, marks } }
            }
            kind::BATCH => {
                let (handle, epoch, commit_no) = (r.u32()?, r.u64()?, r.u64()?);
                let count = r.count(1)?;
                let ops = (0..count).map(|_| r.op()).collect::<Result<_, _>>()?;
                let count = r.count(6)?;
                let app_records = (0..count)
                    .map(|_| Ok(AppRecord { kind: r.u16()?, bytes: r.bytes()?.to_vec() }))
                    .collect::<Result<_, _>>()?;
                Self::Batch(Batch { handle, epoch, commit_no, ops, app_records })
            }
            kind::ACK => Self::Ack { handle: r.u32()?, commit_no: r.u64()? },
            kind::NACK => {
                let handle = r.u32()?;
                let reason = match r.u8()? {
                    1 => NackReason::StaleEpoch,
                    2 => NackReason::Diverged,
                    3 => NackReason::Lag,
                    other => return Err(FrameError::Unknown("refusal", other)),
                };
                Self::Nack { handle, reason }
            }
            other => return Err(FrameError::Unknown("frame kind", other)),
        };
        if !r.0.is_empty() {
            return Err(FrameError::Truncated);
        }
        Ok(Some((frame, 4 + len)))
    }
}

/// A count or length as the wire's `u32`: frames are far shorter than 4 GiB.
fn len_u32(len: usize) -> u32 {
    u32::try_from(len).expect("below MAX_FRAME_BYTES")
}

struct Writer<'a>(&'a mut Vec<u8>);

impl Writer<'_> {
    fn u8(&mut self, value: u8) {
        self.0.push(value);
    }

    fn u16(&mut self, value: u16) {
        self.0.extend_from_slice(&value.to_le_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.0.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.0.extend_from_slice(&value.to_le_bytes());
    }

    fn bytes(&mut self, bytes: &[u8]) {
        self.u32(len_u32(bytes.len()));
        self.0.extend_from_slice(bytes);
    }

    fn op(&mut self, op: &Op) {
        match op {
            Op::SetNextIncoming(seq) => {
                self.u8(op::SET_NEXT_INCOMING);
                self.u64(*seq);
            }
            Op::SetNextIncomingInFlight { seq, in_flight } => {
                self.u8(op::SET_NEXT_INCOMING_IN_FLIGHT);
                self.u64(*seq);
                self.u64(*in_flight);
            }
            Op::SetInFlight(seq) => {
                self.u8(op::SET_IN_FLIGHT);
                self.u64(*seq);
            }
            Op::Record { seq, msg: Some(msg) } => {
                self.u8(op::RECORD);
                self.u64(*seq);
                self.bytes(msg);
            }
            Op::Record { seq, msg: None } => {
                self.u8(op::RECORD_NONE);
                self.u64(*seq);
            }
            Op::Reset => self.u8(op::RESET),
            Op::SetCreatedAt(at) => {
                self.u8(op::SET_CREATED_AT);
                self.bytes(at.to_fix().as_bytes());
            }
            Op::SetMarks(marks) => {
                self.u8(op::SET_MARKS);
                self.bytes(marks.to_string().as_bytes());
            }
        }
    }
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N], FrameError> {
        let (head, rest) = self.0.split_first_chunk::<N>().ok_or(FrameError::Truncated)?;
        self.0 = rest;
        Ok(*head)
    }

    fn u8(&mut self) -> Result<u8, FrameError> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, FrameError> {
        self.take().map(u16::from_le_bytes)
    }

    fn u32(&mut self) -> Result<u32, FrameError> {
        self.take().map(u32::from_le_bytes)
    }

    fn u64(&mut self) -> Result<u64, FrameError> {
        self.take().map(u64::from_le_bytes)
    }

    /// A count of items each at least `min` bytes long: one that couldn't fit in what's left is
    /// refused before anything is allocated for it.
    fn count(&mut self, min: usize) -> Result<usize, FrameError> {
        let count = self.u32()? as usize;
        if count.saturating_mul(min) > self.0.len() {
            return Err(FrameError::Truncated);
        }
        Ok(count)
    }

    fn bytes(&mut self) -> Result<&'a [u8], FrameError> {
        let len = self.u32()? as usize;
        let (bytes, rest) = self.0.split_at_checked(len).ok_or(FrameError::Truncated)?;
        self.0 = rest;
        Ok(bytes)
    }

    fn text(&mut self) -> Result<String, FrameError> {
        let bytes = self.bytes()?;
        std::str::from_utf8(bytes).map(String::from).map_err(|_| FrameError::Invalid("text"))
    }

    fn op(&mut self) -> Result<Op, FrameError> {
        Ok(match self.u8()? {
            op::SET_NEXT_INCOMING => Op::SetNextIncoming(self.u64()?),
            op::SET_NEXT_INCOMING_IN_FLIGHT => Op::SetNextIncomingInFlight { seq: self.u64()?, in_flight: self.u64()? },
            op::SET_IN_FLIGHT => Op::SetInFlight(self.u64()?),
            op::RECORD => Op::Record { seq: self.u64()?, msg: Some(self.bytes()?.to_vec()) },
            op::RECORD_NONE => Op::Record { seq: self.u64()?, msg: None },
            op::RESET => Op::Reset,
            op::SET_CREATED_AT => {
                let text = self.text()?;
                Op::SetCreatedAt(UtcTimestamp::from_fix(&text).map_err(|_| FrameError::Invalid("timestamp"))?)
            }
            op::SET_MARKS => {
                let text = self.text()?;
                Op::SetMarks(text.parse().map_err(|_| FrameError::Invalid("marks"))?)
            }
            other => return Err(FrameError::Unknown("change", other)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_frame() -> Vec<Frame> {
        let session = SessionId::new("FIX.4.4", "GATEWAY", "CLIENT").with_target_sub_id("DESK 1").with_qualifier("a;b");
        let paused = "paused".parse::<Marks>().unwrap();
        vec![
            Frame::Hello { version: VERSION, epoch: 7, node: "gw-a".into(), app_record_kinds: vec![1, 300] },
            Frame::Welcome { epoch: 7 },
            Frame::Bind { handle: 3, epoch: 7, max_lag: 4, session },
            Frame::Bound {
                handle: 3,
                summary: Summary {
                    next_outgoing: 12,
                    next_incoming: 9,
                    in_flight: Some(8),
                    created_at: Some(UtcTimestamp::from_fix("20261026-09:30:00.125").unwrap()),
                    marks: paused,
                },
            },
            Frame::Bound {
                handle: 4,
                summary: Summary {
                    next_outgoing: 1,
                    next_incoming: 1,
                    in_flight: None,
                    created_at: None,
                    marks: Marks::default(),
                },
            },
            Frame::Batch(Batch {
                handle: 3,
                epoch: 7,
                commit_no: 42,
                ops: vec![
                    Op::Reset,
                    Op::SetCreatedAt(UtcTimestamp::from_fix("20261026-09:30:00.125").unwrap()),
                    Op::Record { seq: 4, msg: None },
                    Op::Record { seq: 5, msg: Some(b"8=FIX.4.4\x019=5\x0135=0\x0110=000\x01".to_vec()) },
                    Op::SetNextIncoming(9),
                    Op::SetNextIncomingInFlight { seq: 10, in_flight: 8 },
                    Op::SetInFlight(9),
                    Op::SetMarks(paused),
                ],
                app_records: vec![AppRecord { kind: 300, bytes: vec![0, 1, 2] }],
            }),
            Frame::Batch(Batch { handle: 0, epoch: 0, commit_no: 1, ops: vec![], app_records: vec![] }),
            Frame::Ack { handle: 3, commit_no: 42 },
            Frame::Nack { handle: 3, reason: NackReason::StaleEpoch },
            Frame::Nack { handle: 3, reason: NackReason::Diverged },
            Frame::Nack { handle: 3, reason: NackReason::Lag },
        ]
    }

    #[test]
    fn every_frame_reads_back_as_written() {
        for frame in every_frame() {
            let mut wire = Vec::new();
            frame.encode(&mut wire);
            assert_eq!(Frame::decode(&wire), Ok(Some((frame.clone(), wire.len()))), "{frame:?}");
        }
    }

    /// Frames arrive split anywhere: a decoder waits for the rest, then reads each in turn.
    #[test]
    fn a_partial_frame_waits_for_the_rest() {
        let mut wire = Vec::new();
        let frames = every_frame();
        for frame in &frames {
            frame.encode(&mut wire);
        }
        for cut in 0..wire.len() {
            if let Some((_, used)) = Frame::decode(&wire[..cut]).unwrap() {
                assert!(used <= cut);
            }
        }
        let mut rest = wire.as_slice();
        for frame in &frames {
            let (decoded, used) = Frame::decode(rest).unwrap().unwrap();
            assert_eq!(&decoded, frame);
            rest = &rest[used..];
        }
        assert!(rest.is_empty());
    }

    #[test]
    fn a_frame_past_the_limit_is_refused_before_it_arrives() {
        let wire = u32::try_from(MAX_FRAME_BYTES + 1).unwrap().to_le_bytes();
        assert_eq!(Frame::decode(&wire), Err(FrameError::TooLong(MAX_FRAME_BYTES + 1)));
    }

    /// Lengths and counts inside a frame are checked against what it holds, so a corrupt one is
    /// an error, never a large allocation or a read past its end.
    #[test]
    fn corrupt_frames_are_errors() {
        let mut wire = Vec::new();
        every_frame()[5].encode(&mut wire);
        for at in 4..wire.len() {
            for byte in [0x00, 0x7f, 0xff] {
                let mut corrupt = wire.clone();
                corrupt[at] = byte;
                let _ = Frame::decode(&corrupt);
            }
        }
        let mut huge_count = vec![0; 4];
        huge_count.extend_from_slice(&[kind::BATCH, 0, 0, 0, 0]);
        huge_count.extend_from_slice(&[0; 16]);
        huge_count.extend_from_slice(&u32::MAX.to_le_bytes());
        let len = u32::try_from(huge_count.len() - 4).unwrap();
        huge_count[..4].copy_from_slice(&len.to_le_bytes());
        assert_eq!(Frame::decode(&huge_count), Err(FrameError::Truncated));

        let mut trailing = Vec::new();
        Frame::Welcome { epoch: 1 }.encode(&mut trailing);
        trailing.push(0);
        trailing[0] += 1;
        assert_eq!(Frame::decode(&trailing), Err(FrameError::Truncated));
        assert_eq!(Frame::decode(&[1, 0, 0, 0, 99]), Err(FrameError::Unknown("frame kind", 99)));
    }
}
