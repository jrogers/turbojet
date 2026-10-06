//! The applications on each side: they record what they receive and what their sessions tell
//! them, and the server's acknowledges orders. Their records outlive any one session, standing in
//! for an application's own state.

use std::sync::{Arc, Mutex};

use turbojet::fixp::{Ended, FixpApplication, FixpContext, FixpHandle, Received, SbeMessage};
use turbojet::registry::{Receipt, SendError};
use turbojet::sbe::{Encode, SbeError};

use super::wire::{ORDER_BLOCK, ORDER_SCHEMA, ORDER_TEMPLATE};
use crate::app::Sent;

/// The workload's application message: an order, its body one u64, a unique id.
pub struct Order(pub u64);

impl Encode for Order {
    const TEMPLATE_ID: u16 = ORDER_TEMPLATE;

    fn encode_into(&self, out: &mut Vec<u8>) -> Result<(), SbeError> {
        for n in [ORDER_BLOCK, ORDER_TEMPLATE, ORDER_SCHEMA, 0] {
            out.extend_from_slice(&n.to_le_bytes());
        }
        out.extend_from_slice(&self.0.to_le_bytes());
        Ok(())
    }
}

/// Acknowledgements' ids: the server's application numbers them itself, apart from the world's.
pub const ACK: u64 = 1 << 63;

/// An application message as its receiver saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    pub id: u64,
    pub seq: Option<u64>,
    pub retransmitted: bool,
    pub redelivered: bool,
}

#[derive(Default)]
pub struct RecordingApp {
    /// Acknowledge each order with one of its own (the server's application).
    acks: bool,
    acked: Mutex<u64>,
    pub deliveries: Mutex<Vec<Delivery>>,
    pub handle: Mutex<Option<FixpHandle>>,
    /// Receipts for what this application queued, by order id, until they resolve.
    pub receipts: Mutex<Vec<(u64, Receipt)>>,
    /// Its own messages the counterparty didn't apply (an idempotent flow): first and count.
    pub not_applied: Mutex<Vec<(u64, u64)>>,
    /// How each established connection ended.
    pub ended: Mutex<Vec<Ended>>,
    /// The process is crashing: its sessions, dropped as it does, still tell it they ended, which
    /// a dead process never hears, so those calls are ignored.
    crashing: Mutex<bool>,
}

impl RecordingApp {
    pub fn server() -> Arc<Self> {
        Arc::new(Self { acks: true, ..Self::default() })
    }

    pub fn client() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// The process crashes, `drop_sessions` dropping its sessions: the handle goes with it, and
    /// what they tell it as they go is never heard. What it recorded survives.
    pub fn crash(&self, drop_sessions: impl FnOnce()) {
        *self.handle.lock().unwrap() = None;
        *self.crashing.lock().unwrap() = true;
        drop_sessions();
        *self.crashing.lock().unwrap() = false;
    }

    /// Sends order `id` through the session's handle: whether it was queued, refused as the queue
    /// is full, or not (no connection).
    pub fn send(&self, id: u64) -> Sent {
        let msg = SbeMessage::encode(&Order(id)).expect("an order encodes");
        match self.handle.lock().unwrap().as_ref().map(|handle| handle.send(msg)) {
            Some(Ok(receipt)) => {
                self.receipts.lock().unwrap().push((id, receipt));
                Sent::Queued
            }
            Some(Err(SendError::Full(_))) => Sent::Full,
            Some(Err(SendError::NotConnected(_))) | None => Sent::NotConnected,
            Some(Err(e)) => panic!("a send failed in a way the simulator doesn't know: {e}"),
        }
    }
}

impl FixpApplication for RecordingApp {
    fn on_established(&self, session: &FixpHandle) {
        *self.handle.lock().unwrap() = Some(session.clone());
    }

    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let id = super::wire::order_id_sbe(msg.bytes).expect("only orders are sent");
        if self.acks && id & ACK == 0 {
            let mut acked = self.acked.lock().unwrap();
            *acked += 1;
            ctx.send(&Order(ACK | *acked)).expect("an order encodes");
        }
        let delivery =
            Delivery { id, seq: msg.seq, retransmitted: msg.retransmitted, redelivered: msg.maybe_redelivered };
        self.deliveries.lock().unwrap().push(delivery);
    }

    fn on_not_applied(&self, _session: &FixpHandle, from: u64, count: u64) {
        self.not_applied.lock().unwrap().push((from, count));
    }

    fn on_ended(&self, _session: &FixpHandle, how: Ended) {
        if *self.crashing.lock().unwrap() {
            return;
        }
        self.ended.lock().unwrap().push(how);
    }
}
