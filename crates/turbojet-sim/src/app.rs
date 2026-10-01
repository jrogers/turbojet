//! The applications on each side: they record what they receive, and the acceptor's acknowledges
//! orders. Their records outlive any one session, standing in for an application's own state.

use std::sync::{Arc, Mutex};

use turbojet::message::tags;
use turbojet::{Application, Context, Message, MessageReject, MsgType, SessionHandle};

/// An application message as its receiver saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    /// ClOrdID of an order, ExecID of an ExecutionReport.
    pub id: String,
    pub seq: u64,
    pub redelivered: bool,
}

#[derive(Default)]
pub struct RecordingApp {
    /// Acknowledge each order with an ExecutionReport (the acceptor's application).
    pub acks: bool,
    pub deliveries: Mutex<Vec<Delivery>>,
    pub handle: Mutex<Option<SessionHandle>>,
}

impl RecordingApp {
    pub fn acceptor() -> Arc<Self> {
        Arc::new(Self { acks: true, ..Self::default() })
    }

    pub fn initiator() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Sends `msg` through the session's handle; false if it isn't connected.
    pub fn send(&self, msg: Message) -> bool {
        self.handle.lock().unwrap().as_ref().is_some_and(|handle| handle.send(msg).is_ok())
    }
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
    fn on_logon(&self, session: SessionHandle) {
        *self.handle.lock().unwrap() = Some(session);
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let id = id_of(msg).ok_or_else(MessageReject::unsupported_message_type)?.to_string();
        let seq = msg.get(tags::MSG_SEQ_NUM).and_then(|s| s.parse().ok()).expect("the session checked it");
        if self.acks && msg.msg_type() == MsgType::NewOrderSingle {
            ctx.send(report(&format!("ack-{id}"), Some(&id)));
        }
        self.deliveries.lock().unwrap().push(Delivery { id, seq, redelivered: ctx.maybe_redelivered() });
        Ok(())
    }
}
