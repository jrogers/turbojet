//! Delays on the link between Turbojet and QuickFIX/J, injected by the proxy: latency spikes and
//! messages held past the receiver's SendingTime tolerance.

use std::time::Duration;

use tokio::time::{Instant, sleep_until};
use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Dir, Fault, FixMsg, Options, Pair, PeerEvent, ProxyEvent, Setup, TjEvent, matrix};

matrix!(latency_spike_to_tj, latency_spike_to_peer, stale_sending_time_to_tj, stale_sending_time_to_peer);
matrix!(slow_link, stalled_reader_at_tj, stalled_reader_at_peer);

/// HeartBtInt in the latency-spike scenarios, so the timers run in a second or two.
const HEARTBEAT: Duration = Duration::from_secs(1);

/// How long the latency spike to Turbojet lasts: past its probe point (1.2 × HeartBtInt of silence)
/// and short of its give-up point (a further HeartBtInt).
const SPIKE_TO_TJ: Duration = Duration::from_millis(1600);

/// The SendingTime tolerance in the stale-SendingTime scenarios, and how long the proxy holds the
/// order: a second past it.
const MAX_LATENCY_SECS: u32 = 2;
const STALE_BY: Duration = Duration::from_secs(3);

async fn start(setup: Setup, options: Options) -> Pair {
    let mut pair = setup.start_with(Options { proxy: true, ..options }).await;
    pair.logged_on().await;
    pair
}

fn heartbeat_options() -> Options {
    Options { heartbeat_secs: u32::try_from(HEARTBEAT.as_secs()).unwrap(), ..Options::default() }
}

/// QuickFIX/J's messages are held for [`SPIKE_TO_TJ`], starting just after Turbojet received one.
/// Turbojet probes with a TestRequest after 1.2 × HeartBtInt; QuickFIX/J's answer is held too, but
/// is released before Turbojet would give up, so the session stays up.
async fn latency_spike_to_tj(setup: Setup) {
    let mut pair = start(setup, heartbeat_options()).await;
    // Turbojet's silence starts now, with the hold.
    pair.peer.send(&peer_order("ORD0")).await;
    pair.tj_received("D").await;
    let held_from = Instant::now();
    pair.proxy().hold(Dir::ToTj);

    let probe = pair.peer.wire_in("1", |_| true).await;
    let probed = held_from.elapsed();
    let id = test_req_id(&probe);
    pair.peer.wire_out("0", |m| m.get(112) == Some(id.as_str())).await;
    held(&mut pair, Dir::ToTj, &id).await;
    // No TestRequest before HeartBtInt of silence (FIX: plus a reasonable transmission time).
    assert!(probed >= HEARTBEAT, "Turbojet probed after {probed:?} of silence");
    assert!(held_from.elapsed() < SPIKE_TO_TJ, "the probe came too late for this spike: {probed:?}");
    sleep_until(held_from + SPIKE_TO_TJ).await;
    pair.proxy().release(Dir::ToTj);

    pair.barrier().await;
    pair.stayed_up().await;
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.finish().await;
}

/// Turbojet's messages are held, starting just after QuickFIX/J received one, until QuickFIX/J
/// probes and Turbojet's answer is held too; then they are released, and the session stays up.
///
/// QuickFIX/J checks its timers once a second (and on each message it receives). It probes once
/// 1.5 × HeartBtInt has passed since the last message it received, and gives up at 2.4 ×, but when
/// a Heartbeat falls due on the same check it sends that instead and probes on the next. At
/// HeartBtInt=1 that can leave no check between the two: in one run here it sent a Heartbeat at
/// 1.5 s and gave up at 2.5 s without probing. So this direction runs at HeartBtInt=3, where the
/// probe comes by 6.5 s and giving up not before 7.2 s, and the hold lasts until the probe rather
/// than a fixed time.
async fn latency_spike_to_peer(setup: Setup) {
    const HEARTBEAT_TO_PEER: Duration = Duration::from_secs(3);
    let heartbeat_secs = u32::try_from(HEARTBEAT_TO_PEER.as_secs()).unwrap();
    let mut pair = start(setup, Options { heartbeat_secs, ..Options::default() }).await;
    pair.handle.send(tj_order("ORD0")).unwrap();
    pair.peer.received_with("D", |m| m.get(11) == Some("ORD0")).await;
    let held_from = Instant::now();
    pair.proxy().hold(Dir::ToPeer);

    let probe = pair.peer.wire_out("1", |_| true).await;
    let probed = held_from.elapsed();
    let id = test_req_id(&probe);
    held(&mut pair, Dir::ToPeer, &id).await;
    pair.proxy().release(Dir::ToPeer);
    assert!(probed >= HEARTBEAT_TO_PEER, "QuickFIX/J probed after {probed:?} of silence");
    pair.peer.wire_in("0", |m| m.get(112) == Some(id.as_str())).await;

    pair.barrier().await;
    pair.stayed_up().await;
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.finish().await;
}

fn test_req_id(probe: &FixMsg) -> String {
    probe.get(112).unwrap_or_else(|| panic!("no TestReqID: {}", probe.raw())).to_string()
}

/// Waits for the proxy to hold the Heartbeat answering TestRequest `id` in `dir`.
async fn held(pair: &mut Pair, dir: Dir, id: &str) {
    let field = format!("|112={id}|");
    let what = format!("Heartbeat {id} held {dir:?}");
    pair.proxy()
        .expect(&what, |e| {
            matches!(e, ProxyEvent::Applied { dir: d, fault: Fault::Hold, msg_type, frame }
                if *d == dir && msg_type == "0" && frame.contains(&field))
        })
        .await;
}

fn stale_options() -> Options {
    Options { max_latency_secs: MAX_LATENCY_SECS, ..Options::default() }
}

/// QuickFIX/J's order ORD1 reaches Turbojet with its SendingTime [`STALE_BY`] old, past Turbojet's
/// tolerance. FIX requires a Reject with SessionRejectReason 10 (SendingTime accuracy problem),
/// the inbound MsgSeqNum incremented, and a Logout. The order isn't delivered, then or after the
/// reconnect, and it isn't asked for again.
async fn stale_sending_time_to_tj(setup: Setup) {
    let mut pair = start(setup, stale_options()).await;
    pair.proxy().delay_next(Dir::ToTj, "D", STALE_BY);
    pair.peer.send(&peer_order("ORD1")).await;
    let stale = FixMsg::parse(&pair.proxy().applied(Dir::ToTj, Fault::Delay, "D").await).seq();

    let reject = pair.peer.received("3").await;
    check_reject(&reject, stale);
    let logout = pair.peer.wire_in("5", |_| true).await;
    assert_eq!(logout.seq(), reject.seq() + 1, "the Logout follows the Reject: {}", logout.raw());
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    reconnected_without_resend(pair, "ORD1").await;
}

/// The mirror image: Turbojet's order is stale when QuickFIX/J reads it.
async fn stale_sending_time_to_peer(setup: Setup) {
    let mut pair = start(setup, stale_options()).await;
    pair.proxy().delay_next(Dir::ToPeer, "D", STALE_BY);
    pair.handle.send(tj_order("ORD1")).unwrap();
    let stale = FixMsg::parse(&pair.proxy().applied(Dir::ToPeer, Fault::Delay, "D").await).seq();

    let reject = pair.peer.sent("3").await;
    check_reject(&reject, stale);
    let error = |e: &PeerEvent| matches!(e, PeerEvent::QfjError(t) if t.contains("SendingTime accuracy problem"));
    pair.peer.expect("QuickFIX/J's SendingTime error", error).await;
    let logout = pair.peer.wire_out("5", |_| true).await;
    assert_eq!(logout.seq(), reject.seq() + 1, "the Logout follows the Reject: {}", logout.raw());
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    reconnected_without_resend(pair, "ORD1").await;
}

/// A session-level Reject of message `stale` for a SendingTime accuracy problem.
fn check_reject(reject: &FixMsg, stale: u64) {
    assert_eq!(reject.get(45), Some(stale.to_string().as_str()), "RefSeqNum: {}", reject.raw());
    assert_eq!(reject.get(373), Some("10"), "SessionRejectReason: {}", reject.raw());
}

/// After the logout, the initiator reconnects. The stale order took its sequence number, so
/// neither side asks for a resend, and it is never delivered; new orders are.
async fn reconnected_without_resend(mut pair: Pair, stale: &str) {
    pair.logged_on().await;
    pair.barrier().await;
    let resend_request = |e: &PeerEvent| match e {
        PeerEvent::In(raw) | PeerEvent::Out(raw) => FixMsg::parse(raw).msg_type() == "2",
        _ => false,
    };
    pair.peer.expect_none("ResendRequest", resend_request, Duration::ZERO).await;
    pair.tj_delivers_no_more(stale).await;
    pair.peer_delivers_no_more(stale).await;
    pair.orders_each_way("ORD2", "ORD3").await;
    pair.finish().await;
}

/// The slow link's rate each way, and how many orders cross it each way: about 40 KB, 2 s.
const SLOW_LINK_RATE: u32 = 20_000;
const SLOW_LINK_ORDERS: usize = 300;

/// A 20 KB/s link both ways at HeartBtInt=1, with a few hundred orders sent each way at once.
/// Heartbeats queue behind them, but neither side is ever silent for long, so neither probes or
/// gives up; every order is delivered once, in order.
async fn slow_link(setup: Setup) {
    let mut pair = start(setup, heartbeat_options()).await;
    pair.proxy().bandwidth(Dir::ToPeer, SLOW_LINK_RATE);
    pair.proxy().bandwidth(Dir::ToTj, SLOW_LINK_RATE);
    let started = Instant::now();
    for i in 0..SLOW_LINK_ORDERS {
        pair.handle.send_when_ready(tj_order(&format!("T{i}"))).await.unwrap();
    }
    pair.peer.cmd(&format!("send-many {SLOW_LINK_ORDERS} {}", peer_order("P{i}"))).await;
    peer_receives_in_order(&mut pair, "T", SLOW_LINK_ORDERS).await;
    tj_receives_in_order(&mut pair, "P", SLOW_LINK_ORDERS).await;
    // An order is over 100 bytes on the wire, so the link really was slow.
    let floor = Duration::from_secs(1) * u32::try_from(SLOW_LINK_ORDERS * 100).unwrap() / SLOW_LINK_RATE;
    assert!(started.elapsed() >= floor, "{SLOW_LINK_ORDERS} orders crossed in {:?}", started.elapsed());

    pair.barrier().await;
    no_more_orders(&mut pair).await;
    pair.stayed_up().await;
    pair.finish().await;
}

/// How long Turbojet's send queue must stay full before its writes count as blocked.
const BLOCKED_FOR: Duration = Duration::from_millis(500);
/// More orders than any socket buffers plus Turbojet's send queue hold; reaching it means the stall
/// never pushed back.
const STALL_LIMIT: usize = 200_000;
/// Orders sent to the stalled side while it can't write.
const ORDERS_TO_STALLED: usize = 200;

// Neither stall test checks that the open direction delivers *during* the stall. Linux TCP
// doesn't guarantee it: the stalled socket's receive buffer overfills, its kernel drops what keeps
// arriving there, and the far side's segments then land outside the shut window and are discarded
// whole, ACKs and all. The proxy's sends on that socket go unacknowledged, and it backs off (cwnd 1,
// RTO in seconds) until the stall ends. CI saw exactly that (runs 37138539634, 37144369183 and
// 37145728414): the receiver got the first ten orders, the initial window, and then nothing. So
// each test checks back-pressure during the stall and delivery after it. That an engine keeps
// reading while its writes are blocked (the write deadlock) is checked without TCP's part in it:
// `both_ends_writing_at_once_do_not_deadlock` in the connection driver, and the simulator.

/// The proxy stops reading what Turbojet sends. Turbojet sends until its writes block: its socket
/// fills, then its send queue, which `send` reports as full. QuickFIX/J sends to Turbojet
/// meanwhile. Then the proxy reads again, and every order both ways is delivered once, in order.
async fn stalled_reader_at_tj(setup: Setup) {
    let mut pair = start(setup, Options::default()).await;
    pair.proxy().stall(Dir::ToPeer);
    let sent = send_until_blocked(&pair).await;
    eprintln!("Turbojet's writes blocked after {sent} orders");

    pair.peer.cmd(&format!("send-many {ORDERS_TO_STALLED} {}", peer_order("P{i}"))).await;
    assert!(pair.handle.send(tj_order("EXTRA")).is_err(), "Turbojet's writes unblocked during the stall");

    pair.proxy().unstall(Dir::ToPeer);
    tj_receives_in_order(&mut pair, "P", ORDERS_TO_STALLED).await;
    peer_receives_in_order(&mut pair, "T", sent).await;
    pair.barrier().await;
    no_more_orders(&mut pair).await;
    pair.stayed_up().await;
    pair.finish().await;
}

/// Sends orders `T0`, `T1`, ... until Turbojet's send queue has stayed full for [`BLOCKED_FOR`],
/// and returns how many were queued.
async fn send_until_blocked(pair: &Pair) -> usize {
    let mut sent = 0;
    let mut full_since: Option<Instant> = None;
    loop {
        assert!(sent < STALL_LIMIT, "sent {sent} orders into a stalled proxy without blocking");
        match pair.handle.send(tj_order(&format!("T{sent}"))) {
            Ok(_) => {
                sent += 1;
                full_since = None;
                // The connection runs on this thread too: let it write.
                tokio::task::yield_now().await;
            }
            Err(turbojet::SendError::Full(_)) => {
                if full_since.get_or_insert_with(Instant::now).elapsed() >= BLOCKED_FOR {
                    return sent;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(e) => panic!("send failed: {e:?}"),
        }
    }
}

/// How many orders QuickFIX/J sends into a stall: about 4 MB, twice what blocked a plain socket on
/// macOS loopback (see the proxy's `stall_backs_up_the_sender`). QuickFIX/J writes asynchronously
/// and never reports that it is blocked, so this can't be measured as it is for Turbojet.
const PEER_ORDERS_INTO_STALL: usize = 30_000;
/// How long Turbojet must receive nothing while the proxy doesn't read QuickFIX/J.
const STALL_QUIET: Duration = Duration::from_millis(200);

/// The mirror image: the proxy stops reading what QuickFIX/J sends, QuickFIX/J sends more than the
/// sockets hold, and Turbojet sends to QuickFIX/J meanwhile. Nothing reaches Turbojet until the
/// proxy reads again; then every order both ways is delivered once, in order.
async fn stalled_reader_at_peer(setup: Setup) {
    let mut pair = start(setup, Options::default()).await;
    pair.proxy().stall(Dir::ToTj);
    pair.peer.cmd(&format!("send-many {PEER_ORDERS_INTO_STALL} {}", peer_order("P{i}"))).await;

    for i in 0..ORDERS_TO_STALLED {
        pair.handle.send_when_ready(tj_order(&format!("T{i}"))).await.unwrap();
    }
    pair.tj_expect_none("order during the stall", |e| matches!(e, TjEvent::Message(_)), STALL_QUIET).await;

    pair.proxy().unstall(Dir::ToTj);
    peer_receives_in_order(&mut pair, "T", ORDERS_TO_STALLED).await;
    tj_receives_in_order(&mut pair, "P", PEER_ORDERS_INTO_STALL).await;
    pair.barrier().await;
    no_more_orders(&mut pair).await;
    pair.stayed_up().await;
    pair.finish().await;
}

/// QuickFIX/J delivers orders `{prefix}0` to `{prefix}{count - 1}`, in that order.
async fn peer_receives_in_order(pair: &mut Pair, prefix: &str, count: usize) {
    for i in 0..count {
        let order = pair.peer.received("D").await;
        assert_eq!(order.get(11), Some(format!("{prefix}{i}").as_str()), "{}", order.raw());
        assert_eq!(order.get(43), None, "{}", order.raw());
    }
}

/// Turbojet delivers orders `{prefix}0` to `{prefix}{count - 1}`, in that order.
async fn tj_receives_in_order(pair: &mut Pair, prefix: &str, count: usize) {
    for i in 0..count {
        let order = pair.tj_received("D").await;
        assert_eq!(order.get(tags::CL_ORD_ID), Some(format!("{prefix}{i}").as_str()));
        assert_eq!(order.get(tags::POSS_DUP_FLAG), None);
    }
}

/// Neither side delivers another order. Call after [`Pair::barrier`].
async fn no_more_orders(pair: &mut Pair) {
    pair.tj_expect_none("order", |e| matches!(e, TjEvent::Message(m) if m.msg_type().code() == "D"), Duration::ZERO)
        .await;
    pair.peer.expect_none("order", |e| e.received().is_some_and(|m| m.msg_type() == "D"), Duration::ZERO).await;
}
