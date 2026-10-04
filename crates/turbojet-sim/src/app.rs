//! The applications on each side: they record what they receive, and the acceptor's acknowledges
//! orders. Their records outlive any one session, standing in for an application's own state.

use std::sync::{Arc, Mutex};

use turbojet::message::tags;
use turbojet::{Application, Context, Disconnect, Message, MessageReject, MsgType, Receipt, SendError, SessionHandle};

use crate::time::{Clocks, SimTime};
use crate::world::{PLANTED_AT, Plant};

/// An application message as its receiver saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    /// ClOrdID of an order, ExecID of an ExecutionReport.
    pub id: String,
    pub seq: u64,
    pub redelivered: bool,
}

/// A session's logon, its ending, and a cancel on disconnect, as its application saw them, for
/// the cancel-on-disconnect rule; and the process crashing, which takes the countdowns under way
/// with its registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
    LoggedOn,
    LoggedOut(Disconnect),
    Cancel(Disconnect),
    Crashed,
}

#[derive(Default)]
pub struct RecordingApp {
    /// Acknowledge each order with an ExecutionReport (the acceptor's application).
    pub acks: bool,
    pub deliveries: Mutex<Vec<Delivery>>,
    pub handle: Mutex<Option<SessionHandle>>,
    /// Acknowledgements sent so far: each gets its own ExecID, since an order redelivered after a
    /// crash is acknowledged again.
    acked: Mutex<u64>,
    /// Receipts for what this application queued, by message id, until they resolve.
    pub receipts: Mutex<Vec<(String, Receipt)>>,
    /// Logons, endings, cancels and crashes, each when it happened.
    pub lifecycle: Mutex<Vec<(SimTime, Lifecycle)>>,
    clocks: Clocks,
    /// The process is crashing: its sessions, dropped as it does, still tell it they ended, which
    /// a dead process never hears, so those calls are ignored.
    crashing: Mutex<bool>,
    /// A planted bug for the checker's self-tests, and the deliveries counted towards it.
    plant: Option<Plant>,
    delivered: Mutex<u64>,
    cancels: Mutex<u64>,
    logouts: Mutex<u64>,
}

impl RecordingApp {
    pub fn acceptor(plant: Option<Plant>, clocks: Clocks) -> Arc<Self> {
        Arc::new(Self { acks: true, plant, clocks, ..Self::default() })
    }

    pub fn initiator(clocks: Clocks) -> Arc<Self> {
        Arc::new(Self { clocks, ..Self::default() })
    }

    /// The process crashes, `drop_sessions` dropping its sessions: the handle goes with it, and
    /// what they tell it as they go is never heard. What it recorded survives.
    pub fn crash(&self, drop_sessions: impl FnOnce()) {
        *self.handle.lock().unwrap() = None;
        self.record(Lifecycle::Crashed);
        *self.crashing.lock().unwrap() = true;
        drop_sessions();
        *self.crashing.lock().unwrap() = false;
    }

    fn record(&self, event: Lifecycle) {
        if !*self.crashing.lock().unwrap() {
            self.lifecycle.lock().unwrap().push((self.clocks.now(), event));
        }
    }

    /// Sends `msg` through the session's handle: whether it was queued, refused as the queue is
    /// full, or not (no connection).
    pub fn send(&self, msg: Message) -> Sent {
        let id = id_of(&msg).map(String::from);
        match self.handle.lock().unwrap().as_ref().map(|handle| handle.send(msg)) {
            Some(Ok(receipt)) => {
                if let Some(id) = id {
                    self.receipts.lock().unwrap().push((id, receipt));
                }
                Sent::Queued
            }
            Some(Err(SendError::Full(_))) => Sent::Full,
            Some(Err(SendError::NotConnected(_))) | None => Sent::NotConnected,
            Some(Err(e)) => panic!("a send failed in a way the simulator doesn't know: {e}"),
        }
    }
}

/// What became of an application's send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sent {
    Queued,
    Full,
    NotConnected,
}

/// An order the initiator's application sends.
pub fn order(id: &str) -> Message {
    Message::new(MsgType::NewOrderSingle)
        .with(tags::CL_ORD_ID, id)
        .with(tags::SYMBOL, "SIM")
        .with(tags::SIDE, "1")
        .with(tags::ORDER_QTY, "100")
        .with(tags::ORD_TYPE, "1")
}

/// An ExecutionReport the acceptor's application sends, unprompted or as an acknowledgement.
pub fn report(exec_id: &str, cl_ord_id: Option<&str>) -> Message {
    Message::new(MsgType::ExecutionReport)
        .with_opt(tags::CL_ORD_ID, cl_ord_id)
        .with(tags::EXEC_ID, exec_id)
        .with(tags::ORD_STATUS, "0")
        .with(tags::SYMBOL, "SIM")
}

/// What identifies an application message in a delivery: its ClOrdID or ExecID.
pub fn id_of(msg: &Message) -> Option<&str> {
    match msg.msg_type() {
        MsgType::NewOrderSingle => msg.get(tags::CL_ORD_ID),
        MsgType::ExecutionReport => msg.get(tags::EXEC_ID),
        _ => None,
    }
}

impl Application for RecordingApp {
    fn on_logon(&self, session: &SessionHandle) {
        *self.handle.lock().unwrap() = Some(session.clone());
        self.record(Lifecycle::LoggedOn);
    }

    fn on_logout(&self, _session: &SessionHandle, ended: Disconnect) {
        self.record(Lifecycle::LoggedOut(ended));
        if *self.crashing.lock().unwrap() {
            return;
        }
        let mut logouts = self.logouts.lock().unwrap();
        *logouts += 1;
        // The planted bug: a cancel as the first session ends, whatever the trigger and grace.
        if self.plant == Some(Plant::SpuriousCancel) && *logouts == 1 {
            self.record(Lifecycle::Cancel(ended));
        }
    }

    fn on_cancel_on_disconnect(&self, _session: &SessionHandle, ended: Disconnect) {
        if *self.crashing.lock().unwrap() {
            return;
        }
        let mut cancels = self.cancels.lock().unwrap();
        *cancels += 1;
        // The planted bug: the first cancel never reaches the application.
        if !(self.plant == Some(Plant::SkipCancel) && *cancels == 1) {
            self.record(Lifecycle::Cancel(ended));
        }
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let id = id_of(msg).ok_or_else(MessageReject::unsupported_message_type)?.to_string();
        let seq = msg.get(tags::MSG_SEQ_NUM).and_then(|s| s.parse().ok()).expect("the session checked it");
        if self.acks && msg.msg_type() == MsgType::NewOrderSingle {
            let mut acked = self.acked.lock().unwrap();
            *acked += 1;
            ctx.send(report(&format!("ack-{id}-{acked}"), Some(&id)));
        }
        let delivery = Delivery { id, seq, redelivered: ctx.maybe_redelivered() };
        let mut delivered = self.delivered.lock().unwrap();
        *delivered += 1;
        let mut deliveries = self.deliveries.lock().unwrap();
        match self.plant {
            Some(Plant::DropDelivery) if *delivered == PLANTED_AT => {}
            Some(Plant::DuplicateDelivery) if *delivered == PLANTED_AT => {
                deliveries.extend([delivery.clone(), delivery]);
            }
            _ => deliveries.push(delivery),
        }
        Ok(())
    }
}
