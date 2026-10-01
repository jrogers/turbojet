//! The simulated network between the two nodes: one connection at a time, a byte stream each way.
//!
//! Stage 1 is a perfect network: each write arrives whole, after a fixed latency, in order.

use std::time::Duration;

use crate::Side;
use crate::time::SimTime;

/// Identifies a connection, so bytes and closes from an old one are recognised as stale.
pub type ConnId = u64;

/// How long a write takes to reach the other end.
const LATENCY: Duration = Duration::from_micros(50);

pub struct Net {
    next_id: ConnId,
    open: Option<Open>,
}

struct Open {
    id: ConnId,
    /// When the last segment written each way arrives: later ones never overtake it.
    last_arrival: [SimTime; 2],
}

impl Net {
    pub fn new() -> Self {
        Self { next_id: 1, open: None }
    }

    /// Opens a new connection, replacing none: the caller closes the old one first.
    pub fn connect(&mut self) -> ConnId {
        assert!(self.open.is_none(), "one connection at a time");
        let id = self.next_id;
        self.next_id += 1;
        self.open = Some(Open { id, last_arrival: [SimTime(0); 2] });
        id
    }

    pub fn is_open(&self, conn: ConnId) -> bool {
        self.open.as_ref().is_some_and(|open| open.id == conn)
    }

    /// When bytes `from` writes on `conn` now arrive at the other end, or `None` if the
    /// connection has closed.
    pub fn write(&mut self, conn: ConnId, from: Side, now: SimTime) -> Option<SimTime> {
        let open = self.open.as_mut().filter(|open| open.id == conn)?;
        let arrival = &mut open.last_arrival[from.index()];
        *arrival = (*arrival).max(now.after(LATENCY));
        Some(*arrival)
    }

    /// Closes `conn` from `from`'s end: returns when the other end sees it closed, after
    /// everything already written to it.
    pub fn close(&mut self, conn: ConnId, from: Side, now: SimTime) -> Option<SimTime> {
        let eof = self.write(conn, from, now)?;
        self.open = None;
        Some(eof)
    }
}

impl Default for Net {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_arrive_in_order_and_not_after_a_close() {
        let mut net = Net::new();
        let conn = net.connect();
        let first = net.write(conn, Side::Initiator, SimTime(1_000_000)).unwrap();
        let second = net.write(conn, Side::Initiator, SimTime(0)).unwrap();
        assert!(second >= first);
        assert!(net.close(conn, Side::Acceptor, SimTime(5)).is_some());
        assert_eq!(net.write(conn, Side::Initiator, SimTime(6)), None);
        assert!(!net.is_open(conn));
    }
}
