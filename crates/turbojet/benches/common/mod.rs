//! Shared fixtures for the benchmarks, and for `tests/allocations.rs`.
#![allow(dead_code)]

use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use turbojet::fields::Decimal;
use turbojet::fields::EncryptMethod;
use turbojet::fields::UtcTimestamp;
use turbojet::message::{Message, tags, utc_timestamp};
use turbojet::store::{SessionLog, SessionStorage};
use turbojet::{Application, Context, MessageReject, Session, SessionConfig, SessionId, SessionRegistry, admin};
use turbojet_fix42::{
    ExecTransType, ExecType, ExecutionReport, HandlInst, NewOrderSingle, OrdStatus, OrdType, Side, TimeInForce,
};

/// A typical limit order (~150 bytes on the wire).
pub fn new_order_single(cl_ord_id: u64) -> NewOrderSingle {
    let mut order = NewOrderSingle::new(
        format!("ORD{cl_ord_id}"),
        HandlInst::AutomatedExecutionNoIntervention,
        "AAPL",
        Side::Buy,
        UtcTimestamp::now(),
        OrdType::Limit,
    );
    order.account = Some("ACCT-001".into());
    order.order_qty = Some(Decimal::new(100, 0));
    order.price = Some(Decimal::new(15025, 2));
    order.time_in_force = Some(TimeInForce::Day);
    order
}

/// The acknowledgement an acceptor would send for `order`.
pub fn ack(order: NewOrderSingle, id: u64) -> ExecutionReport {
    let order_id = format!("O{id}");
    let exec_id = format!("E{id}");
    let leaves_qty = order.order_qty.unwrap_or_default();
    let cum_qty = Decimal::ZERO;
    let avg_px = Decimal::ZERO;
    let mut report = ExecutionReport::new(
        order_id,
        exec_id,
        ExecTransType::New,
        ExecType::New,
        OrdStatus::New,
        order.symbol,
        order.side,
        leaves_qty,
        cum_qty,
        avg_px,
    );
    report.cl_ord_id = Some(order.cl_ord_id);
    report.account = order.account;
    report.order_qty = order.order_qty;
    report.ord_type = Some(order.ord_type);
    report.price = order.price;
    report.time_in_force = order.time_in_force;
    report.transact_time = Some(UtcTimestamp::now());
    report
}

/// `body` with a standard header, as sent by `sender` to `target`.
pub fn with_header(sender: &str, target: &str, seq: u64, body: Message) -> Message {
    let mut msg = Message::default();
    msg.push(tags::BEGIN_STRING, "FIX.4.2");
    msg.push(tags::MSG_TYPE, body.msg_type());
    msg.push(tags::SENDER_COMP_ID, sender);
    msg.push(tags::TARGET_COMP_ID, target);
    msg.push(tags::MSG_SEQ_NUM, seq);
    msg.push(tags::SENDING_TIME, utc_timestamp());
    for (tag, value) in body.fields() {
        if tag != tags::MSG_TYPE {
            msg.push(tag, value);
        }
    }
    msg
}

/// A logged-on FIX 4.2 acceptor session with `storage` and `app`, whose counterparty's next
/// MsgSeqNum is 2.
pub fn logged_on(storage: Arc<dyn SessionStorage>, app: Arc<dyn Application>) -> Session {
    let registry = Arc::new(SessionRegistry::new(storage));
    let (mut session, _commands) =
        Session::acceptor(SessionConfig::new("FIX.4.2", "GATEWAY"), registry, app, Instant::now());
    let logon = admin::Logon {
        encrypt_method: EncryptMethod::None,
        heart_bt_int: 30,
        reset_seq_num_flag: None,
        next_expected_msg_seq_num: None,
        username: None,
        password: None,
        default_appl_ver_id: None,
    };
    session.on_message(with_header("CLIENT", "GATEWAY", 1, logon.into()), Instant::now());
    assert!(session.is_logged_on());
    session
}

/// `count` orders from CLIENT, MsgSeqNum 2 onwards.
pub fn orders(count: u64) -> Vec<Message> {
    (0..count).map(|i| with_header("CLIENT", "GATEWAY", i + 2, new_order_single(i).into())).collect()
}

/// Acknowledges every NewOrderSingle with an ExecutionReport: parses the typed order and builds a
/// typed reply, as a real application would.
#[derive(Default)]
pub struct Acker {
    next_id: AtomicU64,
}

impl Application for Acker {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let order: NewOrderSingle = msg.parse()?;
        ctx.send(ack(order, self.next_id.fetch_add(1, Ordering::Relaxed)));
        Ok(())
    }
}

/// Keeps sequence numbers but discards sent messages, so long benchmark runs don't accumulate
/// millions of messages in memory. Storage costs are measured separately.
#[derive(Default)]
pub struct DiscardStorage;

struct DiscardLog {
    seq: Arc<Mutex<(u64, u64)>>,
}

impl SessionStorage for DiscardStorage {
    fn open(&self, _id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(DiscardLog { seq: Arc::new(Mutex::new((1, 1))) }))
    }
}

impl SessionLog for DiscardLog {
    fn next_outgoing(&self) -> u64 {
        self.seq.lock().unwrap().0
    }
    fn next_incoming(&self) -> u64 {
        self.seq.lock().unwrap().1
    }
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.seq.lock().unwrap().1 = seq;
        Ok(())
    }
    fn record_outgoing(&mut self, seq: u64, _msg: Option<&Message>) -> io::Result<()> {
        self.seq.lock().unwrap().0 = seq + 1;
        Ok(())
    }
    fn sent_messages(&mut self, _begin: u64, _end: u64) -> io::Result<Vec<(u64, Message)>> {
        Ok(Vec::new())
    }
    fn reset(&mut self) -> io::Result<()> {
        *self.seq.lock().unwrap() = (1, 1);
        Ok(())
    }
}
