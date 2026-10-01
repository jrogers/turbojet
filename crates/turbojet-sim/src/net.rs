//! The simulated network between the two nodes: connections carrying a byte stream each way, as
//! TCP does. Within a connection bytes are never reordered, duplicated or corrupted, but they
//! arrive in arbitrary pieces after varying delays, a direction can stall, the send buffer
//! fills, and a connection can reset (both ends told) or fall into a black hole (neither told).

use std::collections::BTreeMap;
use std::time::Duration;

use crate::Side;
use crate::rng::Rng;
use crate::time::SimTime;

/// Identifies a connection, so bytes and closes from an old one are recognised as stale.
pub type ConnId = u64;

/// One seed's network.
#[derive(Debug, Clone)]
pub struct Params {
    /// Each segment's delay, drawn from this range.
    pub latency: (Duration, Duration),
    /// Bytes a direction holds, in flight and unread at the receiver, before the writer blocks.
    pub capacity: usize,
    /// Most segments one write is split into.
    pub max_segments: u64,
}

pub struct Net {
    params: Params,
    rng: Rng,
    next_id: ConnId,
    conns: BTreeMap<ConnId, Conn>,
}

struct Conn {
    /// Each direction, by the side that writes it.
    dirs: [Dir; 2],
    /// Nothing is delivered either way any more, and neither end is told.
    black_hole: bool,
}

#[derive(Default)]
struct Dir {
    /// When the last segment written this way arrives: later ones never overtake it.
    last_arrival: SimTime,
    /// Bytes written this way and not yet read by the receiver.
    buffered: usize,
    /// Nothing arrives this way before then.
    stalled_until: SimTime,
}

/// What became of a write.
pub struct Written {
    /// How many bytes the send buffer took: the rest waits, the writer blocked.
    pub accepted: usize,
    /// The pieces to deliver, and when.
    pub segments: Vec<(SimTime, Vec<u8>)>,
}

impl Net {
    pub fn new(params: Params, rng: Rng) -> Self {
        Self { params, rng, next_id: 1, conns: BTreeMap::new() }
    }

    pub fn connect(&mut self) -> ConnId {
        let id = self.next_id;
        self.next_id += 1;
        self.conns.insert(id, Conn { dirs: Default::default(), black_hole: false });
        id
    }

    pub fn is_open(&self, conn: ConnId) -> bool {
        self.conns.contains_key(&conn)
    }

    pub fn is_black_hole(&self, conn: ConnId) -> bool {
        self.conns.get(&conn).is_some_and(|c| c.black_hole)
    }

    /// The open connections.
    pub fn conns(&self) -> impl Iterator<Item = ConnId> + '_ {
        self.conns.keys().copied()
    }

    /// `from` writes `bytes` on `conn`. Writes on a connection that's gone succeed and vanish, as
    /// they do on a socket whose peer has closed; the writer learns of it from its read.
    pub fn write(&mut self, conn: ConnId, from: Side, bytes: &[u8], now: SimTime) -> Written {
        let Some(c) = self.conns.get_mut(&conn) else {
            return Written { accepted: bytes.len(), segments: Vec::new() };
        };
        let black_hole = c.black_hole;
        let dir = &mut c.dirs[from.index()];
        let accepted = bytes.len().min(self.params.capacity.saturating_sub(dir.buffered));
        dir.buffered += accepted;
        let mut segments = Vec::new();
        if !black_hole && accepted > 0 {
            let pieces = self.rng.between(1, self.params.max_segments.min(accepted as u64));
            let mut cuts: Vec<usize> =
                (1..pieces).map(|_| usize::try_from(self.rng.between(1, accepted as u64 - 1)).expect("fits")).collect();
            cuts.sort_unstable();
            cuts.dedup();
            let mut start = 0;
            for end in cuts.into_iter().chain([accepted]) {
                let (lo, hi) = self.params.latency;
                let latency = Duration::from_nanos(self.rng.between(nanos(lo), nanos(hi)));
                dir.last_arrival = dir.last_arrival.max(now.after(latency)).max(dir.stalled_until);
                segments.push((dir.last_arrival, bytes[start..end].to_vec()));
                start = end;
            }
        }
        Written { accepted, segments }
    }

    /// `reader` has read `n` bytes from `conn`: room in the other side's send buffer.
    pub fn read(&mut self, conn: ConnId, reader: Side, n: usize) {
        if let Some(c) = self.conns.get_mut(&conn) {
            let dir = &mut c.dirs[reader.other().index()];
            dir.buffered = dir.buffered.checked_sub(n).expect("read no more than was written");
        }
    }

    /// When a close (FIN) `from` writes now reaches the other end: after everything already
    /// written, or never on a black hole. The connection is gone for both ends from now: the
    /// other end's writes vanish, and it learns of the close by reading.
    pub fn close(&mut self, conn: ConnId, from: Side, now: SimTime) -> Option<SimTime> {
        let c = self.conns.remove(&conn)?;
        let dir = &c.dirs[from.index()];
        (!c.black_hole).then(|| dir.last_arrival.max(now).max(dir.stalled_until))
    }

    /// Resets `conn`: both ends lose what's in flight and unread, and are told.
    pub fn reset(&mut self, conn: ConnId) -> bool {
        self.conns.remove(&conn).is_some()
    }

    /// Nothing more is delivered on `conn` either way, and no one is told.
    pub fn black_hole(&mut self, conn: ConnId) {
        if let Some(c) = self.conns.get_mut(&conn) {
            c.black_hole = true;
        }
    }

    /// Nothing written by `from` on `conn` arrives before `until`.
    pub fn stall(&mut self, conn: ConnId, from: Side, until: SimTime) {
        if let Some(c) = self.conns.get_mut(&conn) {
            let dir = &mut c.dirs[from.index()];
            dir.stalled_until = dir.stalled_until.max(until);
        }
    }
}

fn nanos(d: Duration) -> u64 {
    SimTime::from_duration(d).0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn net(capacity: usize) -> Net {
        let params =
            Params { latency: (Duration::from_micros(10), Duration::from_millis(5)), capacity, max_segments: 8 };
        Net::new(params, Rng::new(3))
    }

    #[test]
    fn segments_arrive_in_order_and_whole() {
        let mut net = net(1 << 20);
        let conn = net.connect();
        let bytes: Vec<u8> = (0..=255).collect();
        let mut arrived = Vec::new();
        let mut last = SimTime(0);
        for now in [SimTime(0), SimTime(1)] {
            let written = net.write(conn, Side::Initiator, &bytes, now);
            assert_eq!(written.accepted, bytes.len());
            for (at, piece) in written.segments {
                assert!(at >= last, "a segment overtook another");
                last = at;
                arrived.extend(piece);
            }
        }
        assert_eq!(arrived, [bytes.clone(), bytes].concat());
    }

    #[test]
    fn a_full_send_buffer_takes_no_more_until_read() {
        let mut net = net(100);
        let conn = net.connect();
        assert_eq!(net.write(conn, Side::Acceptor, &[0; 80], SimTime(0)).accepted, 80);
        assert_eq!(net.write(conn, Side::Acceptor, &[0; 80], SimTime(0)).accepted, 20);
        assert_eq!(net.write(conn, Side::Acceptor, &[0; 1], SimTime(0)).accepted, 0);
        net.read(conn, Side::Initiator, 50);
        assert_eq!(net.write(conn, Side::Acceptor, &[0; 80], SimTime(0)).accepted, 50);
    }

    #[test]
    fn a_black_hole_delivers_nothing_and_closes_unseen() {
        let mut net = net(1 << 20);
        let conn = net.connect();
        net.black_hole(conn);
        assert!(net.write(conn, Side::Initiator, b"hello", SimTime(0)).segments.is_empty());
        assert_eq!(net.close(conn, Side::Initiator, SimTime(0)), None);
        assert!(!net.is_open(conn));
    }

    #[test]
    fn a_stall_holds_back_what_follows() {
        let mut net = net(1 << 20);
        let conn = net.connect();
        net.stall(conn, Side::Initiator, SimTime(1_000_000_000));
        let written = net.write(conn, Side::Initiator, b"x", SimTime(0));
        assert!(written.segments.iter().all(|(at, _)| *at >= SimTime(1_000_000_000)));
    }
}
