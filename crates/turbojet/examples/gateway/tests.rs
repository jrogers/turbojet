//! End-to-end tests against a gateway listening on a real TCP socket.

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use turbojet::admin::{Logon, ResendRequest};
use turbojet::codec::{Decoded, decode, encode};
use turbojet::fields::EncryptMethod;
use turbojet::fields::UtcTimestamp;
use turbojet::message::{Message, tags, utc_timestamp};
use turbojet::{Acceptor, DiskStorage, MemoryStorage, MsgType, SessionConfig, SessionId, SessionStorage};
use turbojet_fix42::{
    CxlRejReason, CxlRejResponseTo, ExecTransType, ExecType, ExecutionReport, HandlInst, NewOrderSingle, OrdRejReason,
    OrdStatus, OrdType, OrderCancelReject, OrderCancelReplaceRequest, OrderCancelRequest, OrderStatusRequest,
    PreAllocGrp, Side,
};

use crate::app::GatewayApp;
use crate::orders::OrderManager;

const TIMEOUT: Duration = Duration::from_secs(5);

async fn start_gateway() -> SocketAddr {
    serve(gateway(Arc::new(MemoryStorage::new()))).await.0
}

fn gateway(storage: Arc<dyn SessionStorage>) -> Acceptor {
    let app = Arc::new(GatewayApp::new(Arc::new(OrderManager::new()), None));
    Acceptor::new(SessionConfig::new("FIX.4.2", "GATEWAY"), storage, app)
}

async fn serve(acceptor: Acceptor) -> (SocketAddr, tokio::task::JoinHandle<io::Result<()>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    (addr, tokio::spawn(acceptor.serve(listener)))
}

struct Client {
    stream: TcpStream,
    buf: Vec<u8>,
    comp_id: String,
    next_seq: u64,
}

impl Client {
    async fn connect(addr: SocketAddr, comp_id: &str) -> Self {
        Self { stream: TcpStream::connect(addr).await.unwrap(), buf: Vec::new(), comp_id: comp_id.into(), next_seq: 1 }
    }

    /// Connects and completes a Logon with sequence reset.
    async fn logged_on(addr: SocketAddr, comp_id: &str) -> Self {
        let mut client = Self::connect(addr, comp_id).await;
        client.send(logon(true)).await;
        let reply = client.recv().await;
        assert_eq!(reply.msg_type(), MsgType::Logon);
        client
    }

    async fn send(&mut self, body: impl Into<Message>) {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.send_with_seq(body, seq).await;
    }

    async fn send_with_seq(&mut self, body: impl Into<Message>, seq: u64) {
        let body = body.into();
        let mut msg = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::MSG_TYPE, body.msg_type())
            .with(tags::SENDER_COMP_ID, self.comp_id.as_str())
            .with(tags::TARGET_COMP_ID, "GATEWAY")
            .with(tags::MSG_SEQ_NUM, seq)
            .with(tags::SENDING_TIME, utc_timestamp());
        for (tag, value) in body.fields().filter(|(t, _)| *t != tags::MSG_TYPE) {
            msg.push(tag, value);
        }
        self.send_raw(&encode(&msg).unwrap()).await;
    }

    async fn send_raw(&mut self, bytes: &[u8]) {
        self.stream.write_all(bytes).await.unwrap();
    }

    async fn try_recv(&mut self) -> io::Result<Message> {
        loop {
            match decode(&self.buf) {
                Decoded::Message(msg, len) => {
                    self.buf.drain(..len);
                    assert_eq!(msg.get(tags::SENDER_COMP_ID), Some("GATEWAY"));
                    assert_eq!(msg.get(tags::TARGET_COMP_ID), Some(self.comp_id.as_str()));
                    return Ok(msg);
                }
                Decoded::Incomplete => {
                    let n = timeout(TIMEOUT, self.stream.read_buf(&mut self.buf))
                        .await
                        .map_err(|_| io::Error::from(io::ErrorKind::TimedOut))??;
                    if n == 0 {
                        return Err(io::ErrorKind::UnexpectedEof.into());
                    }
                }
                Decoded::Garbled { reason, .. } => panic!("gateway sent garbled data: {reason}"),
            }
        }
    }

    async fn recv(&mut self) -> Message {
        self.try_recv().await.expect("expected a message from the gateway")
    }

    async fn expect_closed(&mut self) {
        match self.try_recv().await {
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof || e.kind() == io::ErrorKind::ConnectionReset => {}
            other => panic!("expected connection to close, got {other:?}"),
        }
    }
}

fn logon(reset: bool) -> Logon {
    Logon {
        encrypt_method: EncryptMethod::None,
        heart_bt_int: 30,
        reset_seq_num_flag: reset.then_some(true),
        next_expected_msg_seq_num: None,
        username: None,
        password: None,
        default_appl_ver_id: None,
    }
}

fn limit_order(cl_ord_id: &str, qty: &str, price: &str) -> NewOrderSingle {
    let mut order = NewOrderSingle::new(
        cl_ord_id,
        HandlInst::AutomatedExecutionNoIntervention,
        "AAPL",
        Side::Buy,
        UtcTimestamp::now(),
        OrdType::Limit,
    );
    order.order_qty = Some(qty.parse().unwrap());
    order.price = Some(price.parse().unwrap());
    order
}

fn cancel(cl_ord_id: &str, orig_cl_ord_id: &str) -> OrderCancelRequest {
    OrderCancelRequest::new(orig_cl_ord_id, cl_ord_id, "AAPL", Side::Buy, UtcTimestamp::now())
}

#[tokio::test]
async fn order_lifecycle() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;

    client.send(limit_order("ORD1", "100", "150.25")).await;
    let raw = client.recv().await;
    assert_eq!(raw.get(tags::MSG_SEQ_NUM), Some("2"));
    // Decimals keep the scale they were sent with.
    assert_eq!(raw.get(tags::PRICE), Some("150.25"));
    let ack: ExecutionReport = raw.parse().unwrap();
    assert_eq!(ack.cl_ord_id.as_deref(), Some("ORD1"));
    assert_eq!(ack.exec_type, ExecType::New);
    assert_eq!(ack.ord_status, OrdStatus::New);
    assert_eq!(ack.leaves_qty, "100".parse().unwrap());

    client.send(cancel("CXL1", "ORD1")).await;
    let canceled: ExecutionReport = client.recv().await.parse().unwrap();
    assert_eq!(canceled.order_id, ack.order_id);
    assert_eq!(canceled.ord_status, OrdStatus::Canceled);

    client.send(cancel("CXL2", "ORD1")).await;
    let too_late: OrderCancelReject = client.recv().await.parse().unwrap();
    assert_eq!(too_late.cxl_rej_reason, Some(CxlRejReason::TooLateToCancel));

    client.send(Message::new(MsgType::Logout)).await;
    assert_eq!(client.recv().await.msg_type(), MsgType::Logout);
    client.expect_closed().await;
}

#[tokio::test]
async fn invalid_orders_are_rejected_without_dropping_the_session() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;

    // Missing Symbol(55): session-level Reject.
    let mut no_symbol = Message::default();
    for (tag, value) in Message::from(limit_order("A", "100", "1")).fields().filter(|(t, _)| *t != tags::SYMBOL) {
        no_symbol.push(tag, value);
    }
    client.send(no_symbol).await;
    let reject = client.recv().await;
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("55"));

    // Zero quantity: rejected ExecutionReport.
    client.send(limit_order("B", "0", "1")).await;
    let er: ExecutionReport = client.recv().await.parse().unwrap();
    assert_eq!(er.ord_status, OrdStatus::Rejected);
    assert_eq!(er.ord_rej_reason, Some(OrdRejReason::BrokerCredit));

    // A FIX 4.4-only Side value is not valid FIX 4.2: session-level Reject naming the field.
    client.send(Message::from(limit_order("B2", "1", "1")).with(tags::SIDE, "A")).await;
    let reject = client.recv().await;
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("54"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("5"));

    // The session is still healthy.
    client.send(limit_order("C", "5", "1")).await;
    assert_eq!(client.recv().await.get(tags::ORD_STATUS), Some("0"));
}

#[tokio::test]
async fn orders_with_repeating_groups() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;
    let alloc = |account: &str, shares: &str| {
        let mut alloc = PreAllocGrp::new(account);
        alloc.alloc_shares = Some(shares.parse().unwrap());
        alloc
    };
    let with_allocs = |cl_ord_id: &str| NewOrderSingle {
        allocs: vec![alloc("ACCT-A", "60"), alloc("ACCT-B", "40")],
        ..limit_order(cl_ord_id, "100", "10")
    };

    // A well-formed group is accepted.
    client.send(with_allocs("G1")).await;
    let ack: ExecutionReport = client.recv().await.parse().unwrap();
    assert_eq!(ack.ord_status, OrdStatus::New);

    // A NoAllocs count that disagrees with its entries gets a session Reject naming the group.
    // FIX 4.2 has no reason code for it (16 came in 4.3), so the Text says what's wrong.
    client.send(Message::from(with_allocs("G2")).with(tags::NO_ALLOCS, 3u64)).await;
    let reject = client.recv().await;
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("78"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), None);
    assert!(reject.get(tags::TEXT).unwrap().contains("declares 3 entries but 2 were found"));

    // The session carries on.
    client.send(limit_order("G3", "1", "1")).await;
    assert_eq!(client.recv().await.parse::<ExecutionReport>().unwrap().ord_status, OrdStatus::New);
}

fn replace(orig: &str, cl_ord_id: &str, qty: &str, price: &str) -> OrderCancelReplaceRequest {
    let order = limit_order(cl_ord_id, qty, price);
    let mut request = OrderCancelReplaceRequest::new(
        orig,
        order.cl_ord_id,
        order.handl_inst,
        order.symbol,
        order.side,
        order.transact_time,
        order.ord_type,
    );
    request.order_qty = order.order_qty;
    request.price = order.price;
    request
}

fn status(cl_ord_id: &str) -> OrderStatusRequest {
    OrderStatusRequest::new(cl_ord_id, "AAPL", Side::Buy)
}

#[tokio::test]
async fn replace_and_status_through_the_order_lifecycle() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;

    client.send(limit_order("A", "100", "150.25")).await;
    let new: ExecutionReport = client.recv().await.parse().unwrap();

    client.send(replace("A", "B", "250", "151.00")).await;
    let replaced: ExecutionReport = client.recv().await.parse().unwrap();
    assert_eq!((replaced.exec_type, replaced.ord_status), (ExecType::Replaced, OrdStatus::Replaced));
    assert_eq!((replaced.order_id.as_str(), replaced.leaves_qty), (new.order_id.as_str(), "250".parse().unwrap()));

    client.send(status("B")).await;
    let open: ExecutionReport = client.recv().await.parse().unwrap();
    assert_eq!(
        (open.exec_trans_type, open.ord_status, open.price),
        (ExecTransType::Status, OrdStatus::New, Some("151.00".parse().unwrap()))
    );

    // Changing the side isn't allowed: an OrderCancelReject answering the replace.
    client.send(OrderCancelReplaceRequest { side: Side::Sell, ..replace("B", "C", "1", "1") }).await;
    let reject: OrderCancelReject = client.recv().await.parse().unwrap();
    assert_eq!(reject.cxl_rej_response_to, CxlRejResponseTo::OrderCancel);

    client.send(cancel("X", "B")).await;
    assert_eq!(client.recv().await.parse::<ExecutionReport>().unwrap().ord_status, OrdStatus::Canceled);
    client.send(status("A")).await;
    assert_eq!(client.recv().await.parse::<ExecutionReport>().unwrap().ord_status, OrdStatus::Canceled);
}

#[tokio::test]
async fn garbled_message_is_ignored() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;
    client.send_raw(b"8=FIX.4.2\x019=5\x01garbage\x0110=000\x01").await;
    client.send(Message::new(MsgType::TestRequest).with(tags::TEST_REQ_ID, "ping")).await;
    let hb = client.recv().await;
    assert_eq!(hb.msg_type(), MsgType::Heartbeat);
    assert_eq!(hb.get(tags::TEST_REQ_ID), Some("ping"));
}

#[tokio::test]
async fn first_message_must_be_logon() {
    let addr = start_gateway().await;
    let mut client = Client::connect(addr, "CLIENT1").await;
    client.send(limit_order("A", "1", "1")).await;
    client.expect_closed().await;
}

#[tokio::test]
async fn second_concurrent_logon_is_refused() {
    let addr = start_gateway().await;
    let _first = Client::logged_on(addr, "CLIENT1").await;
    let mut second = Client::connect(addr, "CLIENT1").await;
    second.send(logon(true)).await;
    second.expect_closed().await;
}

#[tokio::test]
async fn sequence_numbers_resume_after_reconnect() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;
    client.send(limit_order("A", "1", "1")).await;
    client.recv().await;
    client.send(Message::new(MsgType::Logout)).await;
    client.recv().await;
    client.expect_closed().await;

    // Their next seq is 4 (Logon, order, Logout); ours is 4 (Logon, ER, Logout).
    let mut again = Client::connect(addr, "CLIENT1").await;
    again.next_seq = 4;
    again.send(logon(false)).await;
    let reply = again.recv().await;
    assert_eq!(reply.msg_type(), MsgType::Logon);
    assert_eq!(reply.get(tags::MSG_SEQ_NUM), Some("4"));
}

#[tokio::test]
async fn gap_is_recovered_by_resend() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;

    // Skip seq 2: the gateway asks for a resend and holds the order.
    client.send_with_seq(limit_order("A", "1", "1"), 3).await;
    let req = client.recv().await;
    assert_eq!(req.msg_type(), MsgType::ResendRequest);
    assert_eq!(req.get(tags::BEGIN_SEQ_NO), Some("2"));

    // Gap-fill 2 and resend 3 as a possible duplicate.
    client
        .send_with_seq(
            Message::new(MsgType::SequenceReset)
                .with(tags::GAP_FILL_FLAG, "Y")
                .with(tags::NEW_SEQ_NO, "3")
                .with(tags::POSS_DUP_FLAG, "Y"),
            2,
        )
        .await;
    client.send_with_seq(Message::from(limit_order("A", "1", "1")).with(tags::POSS_DUP_FLAG, "Y"), 3).await;
    let ack = client.recv().await;
    assert_eq!(ack.msg_type(), MsgType::ExecutionReport);
    assert_eq!(ack.get(tags::ORD_STATUS), Some("0"));
}

#[tokio::test]
async fn resend_request_replays_execution_reports() {
    let addr = start_gateway().await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;
    client.send(limit_order("A", "1", "1")).await;
    let original = client.recv().await;

    client.send(ResendRequest { begin_seq_no: 1, end_seq_no: 0 }).await;
    let fill = client.recv().await;
    assert_eq!(fill.msg_type(), MsgType::SequenceReset);
    assert_eq!(fill.get(tags::NEW_SEQ_NO), Some("2"));
    let replay = client.recv().await;
    assert_eq!(replay.get(tags::MSG_SEQ_NUM), Some("2"));
    assert_eq!(replay.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(replay.get(tags::EXEC_ID), original.get(tags::EXEC_ID));
}

#[tokio::test]
async fn disk_store_survives_gateway_restart() {
    let dir = tempfile::tempdir().unwrap();
    let disk_gateway = || gateway(Arc::new(DiskStorage::new(dir.path(), false).unwrap()));

    let (addr, server) = serve(disk_gateway()).await;
    let mut client = Client::logged_on(addr, "CLIENT1").await;
    client.send(limit_order("A", "1", "1")).await;
    let original = client.recv().await;
    assert_eq!(original.get(tags::MSG_SEQ_NUM), Some("2"));
    client.send(Message::new(MsgType::Logout)).await;
    client.recv().await;
    client.expect_closed().await;
    server.abort();

    // A fresh gateway process on the same directory picks up where the session left off.
    let (addr, _server) = serve(disk_gateway()).await;
    let mut client = Client::connect(addr, "CLIENT1").await;
    client.next_seq = 4;
    client.send(logon(false)).await;
    let reply = client.recv().await;
    assert_eq!(reply.msg_type(), MsgType::Logon);
    assert_eq!(reply.get(tags::MSG_SEQ_NUM), Some("4"));

    // The execution report sent before the restart can still be replayed.
    client.send(ResendRequest { begin_seq_no: 2, end_seq_no: 2 }).await;
    let replay = client.recv().await;
    assert_eq!(replay.msg_type(), MsgType::ExecutionReport);
    assert_eq!(replay.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(replay.get(tags::EXEC_ID), original.get(tags::EXEC_ID));
    assert_eq!(replay.get(tags::ORIG_SENDING_TIME), original.get(tags::SENDING_TIME));
}

#[tokio::test]
async fn allowlist_refuses_unknown_counterparty() {
    let app = Arc::new(GatewayApp::new(Arc::new(OrderManager::new()), Some(["CLIENT1".to_string()].into())));
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "GATEWAY"), Arc::new(MemoryStorage::new()), app);
    let (addr, _server) = serve(acceptor).await;

    let mut stranger = Client::connect(addr, "STRANGER").await;
    stranger.send(logon(false)).await;
    stranger.expect_closed().await;

    Client::logged_on(addr, "CLIENT1").await;
}

fn client1() -> SessionId {
    SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT1".into() }
}

/// Runs `gateway seqnums --store-dir DIR ARGS...`, returning success and its output.
async fn seqnums(dir: &std::path::Path, args: &[&str]) -> (bool, String) {
    let mut all = vec!["--store-dir".to_string(), dir.to_str().unwrap().to_string()];
    all.extend(args.iter().map(|s| s.to_string()));
    match crate::run_seqnums(&all).await {
        Ok(out) => (true, out),
        Err(out) => (false, out),
    }
}

#[tokio::test]
async fn operator_tool_views_and_changes_stored_sequence_numbers() {
    let dir = tempfile::tempdir().unwrap();
    {
        // A session that has sent 4 messages and received 2.
        let mut log = DiskStorage::new(dir.path(), false).unwrap().open(&client1()).unwrap();
        log.record_outgoing(4, None).unwrap();
        log.set_next_incoming(3).unwrap();
        // Without fsync the store commits at once.
        assert!(log.commit().unwrap().is_none());
    }

    let (ok, out) = seqnums(dir.path(), &["--session", "CLIENT1"]).await;
    assert!(ok, "{out}");
    assert!(out.contains("FIX.4.2:GATEWAY->CLIENT1: next incoming 3, next outgoing 5"), "{out}");

    let (ok, out) =
        seqnums(dir.path(), &["--session", "CLIENT1", "--set-next-incoming", "9", "--set-next-outgoing", "20"]).await;
    assert!(ok, "{out}");
    assert!(out.contains("next incoming 9, next outgoing 20"), "{out}");

    let (ok, out) = seqnums(dir.path(), &["--session", "CLIENT1", "--set-next-outgoing", "10"]).await;
    assert!(!ok);
    assert!(out.contains("only move forward"), "{out}");

    let (ok, out) = seqnums(dir.path(), &["--session", "CLIENT1", "--reset", "--set-next-incoming", "4"]).await;
    assert!(ok, "{out}");
    assert!(
        out.contains("reset FIX.4.2:GATEWAY->CLIENT1") && out.contains("next incoming 4, next outgoing 1"),
        "{out}"
    );
}

#[tokio::test]
async fn operator_tool_refuses_unknown_and_connected_sessions() {
    let dir = tempfile::tempdir().unwrap();
    let storage = DiskStorage::new(dir.path(), false).unwrap();

    let (ok, out) = seqnums(dir.path(), &["--session", "NOBODY"]).await;
    assert!(!ok);
    assert!(out.contains("no stored session FIX.4.2:GATEWAY->NOBODY"), "{out}");
    assert!(!storage.contains(&SessionId { target_comp_id: "NOBODY".into(), ..client1() }), "nothing created");

    // An open log (a connected session) holds the store's lock.
    let _connected = storage.open(&client1()).unwrap();
    let (ok, out) = seqnums(dir.path(), &["--session", "CLIENT1", "--reset"]).await;
    assert!(!ok);
    assert!(out.contains("is connected (its store is locked)"), "{out}");
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn gateway_requires_an_allow_list_unless_told_to_accept_anyone() {
    let err = crate::parse_args(args(&["--listen", "127.0.0.1:0"])).err().expect("refused without --allow");
    assert!(err.contains("--allow"), "{err}");

    let allowed = crate::parse_args(args(&["--allow", "CLIENT1,CLIENT2"])).unwrap().allowed;
    assert_eq!(allowed, Some(["CLIENT1".to_string(), "CLIENT2".to_string()].into()));

    assert_eq!(crate::parse_args(args(&["--allow-any"])).unwrap().allowed, None);
    assert!(crate::parse_args(args(&["--allow-any", "--allow", "CLIENT1"])).is_err(), "contradictory");
}
