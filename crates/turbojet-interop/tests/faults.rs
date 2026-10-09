//! Faults on the link between Turbojet and QuickFIX/J, injected by the proxy.

use std::time::Duration;

use tokio::time::Instant;

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Dir, Engine, Fault, FixMsg, Options, Pair, PeerEvent, ProxyEvent, Setup, matrix};

matrix!(lost_order_to_peer, lost_order_to_tj, garbled_order_to_peer, garbled_order_to_tj);
matrix!(silent_peer, silent_tj, cut_order_to_peer, cut_order_to_tj);

/// HeartBtInt in the silent scenarios, so each side gives up within a few seconds.
const HEARTBEAT: Duration = Duration::from_secs(1);

async fn lost_order_to_peer(setup: Setup) {
    order_recovered_at_peer(setup, Fault::Drop).await;
}

async fn garbled_order_to_peer(setup: Setup) {
    order_recovered_at_peer(setup, Fault::Garble).await;
}

async fn lost_order_to_tj(setup: Setup) {
    order_recovered_at_tj(setup, Fault::Drop).await;
}

async fn garbled_order_to_tj(setup: Setup) {
    order_recovered_at_tj(setup, Fault::Garble).await;
}

async fn start(setup: Setup) -> Pair {
    let mut pair = setup.start_with(Options { proxy: true, ..Options::default() }).await;
    pair.logged_on().await;
    pair
}

/// Arms `fault` for the next order in `dir`.
fn arm(pair: &mut Pair, dir: Dir, fault: Fault) {
    match fault {
        Fault::Drop => pair.proxy().drop_next(dir, "D"),
        Fault::Garble => pair.proxy().garble_next(dir, "D"),
        _ => unreachable!("not a fault on one message"),
    }
}

/// The ResendRequest a receiver sent for the lost order `lost`. BeginSeqNo must be the lost
/// message; EndSeqNo may be it, anything after, or 0 for "all after".
fn check_resend_request(request: &FixMsg, lost: u64) {
    assert_eq!(request.get(7), Some(lost.to_string().as_str()), "BeginSeqNo: {}", request.raw());
    let end: u64 = request.get(16).and_then(|e| e.parse().ok()).unwrap_or_else(|| panic!("{}", request.raw()));
    assert!(end == 0 || end >= lost, "EndSeqNo: {}", request.raw());
}

/// The sequence number of a frame as the proxy reported it.
fn seq_of(frame: &str) -> u64 {
    FixMsg::parse(frame).seq()
}

/// Turbojet's order ORD1 is lost (or arrives garbled, which QuickFIX/J must discard without
/// disconnecting or counting it). ORD2 reveals the gap: QuickFIX/J asks for ORD1, Turbojet resends
/// it with PossDupFlag=Y and OrigSendingTime, and QuickFIX/J delivers ORD1 then ORD2, once each.
async fn order_recovered_at_peer(setup: Setup, fault: Fault) {
    let mut pair = start(setup).await;
    arm(&mut pair, Dir::ToPeer, fault);
    pair.handle.send(tj_order("ORD1")).unwrap();
    let frame = pair.proxy().applied(Dir::ToPeer, fault, "D").await;
    assert!(frame.contains("|11=ORD1|"), "{frame}");
    let lost = seq_of(&frame);
    if fault == Fault::Garble {
        // QuickFIX/J reads the frame but refuses it on its CheckSum.
        pair.peer.wire_in("D", |m| m.seq() == lost).await;
        if setup.engine == Engine::QuickFixGo {
            // quickfix-go doesn't check CheckSum, so it delivers the garbled order where the
            // session layer should discard it as garbled. Nothing is lost, so nothing to recover.
            let delivered = pair.peer.received_with("D", |m| m.get(11) == Some("ORD1")).await;
            assert_eq!(delivered.seq(), lost, "{}", delivered.raw());
            pair.finish().await;
            return;
        }
        pair.peer.expect("CheckSum error", |e| matches!(e, PeerEvent::QfjError(t) if t.contains("CheckSum"))).await;
    }
    pair.handle.send(tj_order("ORD2")).unwrap();

    let request = pair.peer.wire_out("2", |_| true).await;
    check_resend_request(&request, lost);
    let resent = pair.peer.wire_in("D", |m| m.get(11) == Some("ORD1") && m.get(43) == Some("Y")).await;
    assert_eq!(resent.seq(), lost, "{}", resent.raw());
    assert!(resent.get(122).is_some(), "no OrigSendingTime: {}", resent.raw());

    // Delivered in sequence: the resend of ORD1, then the original ORD2 queued behind the gap.
    let first = pair.peer.received_with("D", |_| true).await;
    assert_eq!(first.get(11), Some("ORD1"), "{}", first.raw());
    assert_eq!(first.get(43), Some("Y"), "{}", first.raw());
    let second = pair.peer.received_with("D", |_| true).await;
    assert_eq!(second.get(11), Some("ORD2"), "{}", second.raw());
    assert_eq!(second.seq(), lost + 1, "{}", second.raw());
    assert_eq!(second.get(43), None, "{}", second.raw());

    pair.barrier().await;
    pair.peer_delivers_no_more("ORD1").await;
    pair.peer_delivers_no_more("ORD2").await;
    pair.stayed_up().await;
    pair.finish().await;
}

/// QuickFIX/J's order ORD1 is lost (or arrives garbled, which Turbojet must discard without
/// disconnecting or counting it). ORD2 reveals the gap: Turbojet asks for ORD1, QuickFIX/J resends
/// it with PossDupFlag=Y and OrigSendingTime, and Turbojet delivers ORD1 then ORD2, once each.
/// Turbojet asking from ORD1 on shows it didn't count the garbled frame. (A barrier between the
/// two orders would itself reveal the gap, so there is no check that nothing happened before ORD2.)
async fn order_recovered_at_tj(setup: Setup, fault: Fault) {
    let mut pair = start(setup).await;
    arm(&mut pair, Dir::ToTj, fault);
    pair.peer.send(&peer_order("ORD1")).await;
    let frame = pair.proxy().applied(Dir::ToTj, fault, "D").await;
    assert!(frame.contains("|11=ORD1|"), "{frame}");
    let lost = seq_of(&frame);
    pair.peer.send(&peer_order("ORD2")).await;

    let request = pair.peer.wire_in("2", |_| true).await;
    check_resend_request(&request, lost);
    let resent = pair.peer.wire_out("D", |m| m.get(11) == Some("ORD1") && m.get(43) == Some("Y")).await;
    assert_eq!(resent.seq(), lost, "{}", resent.raw());
    assert!(resent.get(122).is_some(), "no OrigSendingTime: {}", resent.raw());

    // Delivered in sequence: the resend of ORD1, then the original ORD2 queued behind the gap.
    let first = pair.tj_received("D").await;
    assert_eq!(first.get(tags::CL_ORD_ID), Some("ORD1"));
    assert_eq!(first.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert!(first.get(tags::ORIG_SENDING_TIME).is_some());
    let second = pair.tj_received("D").await;
    assert_eq!(second.get(tags::CL_ORD_ID), Some("ORD2"));
    assert_eq!(second.get(tags::MSG_SEQ_NUM), Some((lost + 1).to_string().as_str()));
    assert_eq!(second.get(tags::POSS_DUP_FLAG), None);

    pair.barrier().await;
    pair.tj_delivers_no_more("ORD1").await;
    pair.tj_delivers_no_more("ORD2").await;
    pair.stayed_up().await;
    pair.finish().await;
}

/// QuickFIX/J goes silent: everything it sends is lost, but the connection stays up.
///
/// Turbojet probes with a TestRequest after HeartBtInt (plus 20% for transmission time), and,
/// with no answer within another HeartBtInt, disconnects. The proxy withholds that close, so
/// QuickFIX/J has to find the dead link on its own: it probes too and gives up. Then they
/// reconnect, the sequence numbers carry on, and what the silence swallowed is gap-filled.
async fn silent_peer(setup: Setup) {
    let mut pair = start_silent(setup).await;
    let silent_from = Instant::now();
    pair.proxy().blackhole(Dir::ToTj);

    // Turbojet's TestRequest reaches QuickFIX/J, and its answer is lost.
    let probe = pair.peer.wire_in("1", |_| true).await;
    let probed_at = Instant::now();
    let id = probe.get(112).unwrap_or_else(|| panic!("no TestReqID: {}", probe.raw())).to_string();
    pair.peer.wire_out("0", |m| m.get(112) == Some(id.as_str())).await;
    lost_in_blackhole(&mut pair, Dir::ToTj, "0", &id).await;
    pair.tj_logged_out().await;
    // It waited for an answer, and not for ever.
    let gave_up_at = Instant::now();
    assert!(gave_up_at - probed_at >= HEARTBEAT / 2, "gave up {:?} after probing", gave_up_at - probed_at);
    assert!(gave_up_at - silent_from <= HEARTBEAT * 4, "gave up {:?} after the silence", gave_up_at - silent_from);

    assert!(!pair.proxy().ended(Dir::ToPeer).await, "Turbojet's close should be withheld");
    // QuickFIX/J hears nothing more, probes in turn, and gives up by itself.
    pair.proxy()
        .expect(
            "QuickFIX/J's TestRequest",
            |e| matches!(e, ProxyEvent::Applied { dir: Dir::ToTj, msg_type, .. } if msg_type == "1"),
        )
        .await;
    peer_timed_out(&mut pair).await;
    assert!(pair.proxy().ended(Dir::ToTj).await);
    pair.proxy().disconnected().await;
    resume(pair, Dir::ToTj).await;
}

/// The mirror image: Turbojet goes silent. QuickFIX/J probes and disconnects; Turbojet, its close
/// withheld, probes in turn and gives up on its own HeartBtInt.
async fn silent_tj(setup: Setup) {
    let mut pair = start_silent(setup).await;
    pair.proxy().blackhole(Dir::ToPeer);

    // QuickFIX/J's TestRequest reaches Turbojet, and its answer is lost.
    let probe = pair.peer.wire_out("1", |_| true).await;
    let id = probe.get(112).unwrap_or_else(|| panic!("no TestReqID: {}", probe.raw())).to_string();
    lost_in_blackhole(&mut pair, Dir::ToPeer, "0", &id).await;
    peer_timed_out(&mut pair).await;

    assert!(!pair.proxy().ended(Dir::ToTj).await, "QuickFIX/J's close should be withheld");
    let silent_from = Instant::now();
    let probe = pair
        .proxy()
        .expect(
            "Turbojet's TestRequest",
            |e| matches!(e, ProxyEvent::Applied { dir: Dir::ToPeer, msg_type, .. } if msg_type == "1"),
        )
        .await;
    let probed_at = Instant::now();
    pair.tj_logged_out().await;
    let gave_up_at = Instant::now();
    assert!(gave_up_at - probed_at >= HEARTBEAT / 2, "gave up {:?} after probing: {probe:?}", gave_up_at - probed_at);
    assert!(gave_up_at - silent_from <= HEARTBEAT * 4, "gave up {:?} after the silence", gave_up_at - silent_from);
    assert!(pair.proxy().ended(Dir::ToPeer).await);
    pair.proxy().disconnected().await;
    resume(pair, Dir::ToPeer).await;
}

/// The peer disconnects on its own heartbeat timeout, not on a Logout or a TCP close.
async fn peer_timed_out(pair: &mut Pair) {
    let text = match pair.peer.engine() {
        Engine::QuickFixJ | Engine::QuickFixN => "Timed out waiting for heartbeat",
        Engine::QuickFixGo => "Session Timeout",
    };
    pair.peer.expect("the peer's heartbeat timeout", |e| e.logged().is_some_and(|t| t.contains(text))).await;
    pair.peer.logout().await;
}

async fn start_silent(setup: Setup) -> Pair {
    let heartbeat_secs = u32::try_from(HEARTBEAT.as_secs()).unwrap();
    let mut pair = setup.start_with(Options { heartbeat_secs, proxy: true, ..Options::default() }).await;
    pair.logged_on().await;
    pair
}

/// Waits for the blackhole in `dir` to swallow the `msg_type` with TestReqID `id`.
async fn lost_in_blackhole(pair: &mut Pair, dir: Dir, msg_type: &str, id: &str) {
    let field = format!("|112={id}|");
    let what = format!("35={msg_type} {id} lost {dir:?}");
    pair.proxy()
        .expect(&what, |e| {
            matches!(e, ProxyEvent::Applied { dir: d, fault: Fault::Blackhole, msg_type: t, frame }
                if *d == dir && t == msg_type && frame.contains(&field))
        })
        .await;
}

/// After a silence in `silent`: both sides log on again with their sequence numbers carried on,
/// the receiver of the silent direction asks for what it missed and is gap-filled (all it missed
/// was admin messages), and then an order each way is delivered once.
async fn resume(mut pair: Pair, silent: Dir) {
    pair.logged_on().await;
    for logon in [pair.peer.wire_in("A", |m| m.seq() > 1).await, pair.peer.wire_out("A", |m| m.seq() > 1).await] {
        assert_ne!(logon.get(141), Some("Y"), "{}", logon.raw());
    }
    let (request, fill) = match silent {
        Dir::ToTj => (pair.peer.wire_in("2", |_| true).await, pair.peer.wire_out("4", |_| true).await),
        Dir::ToPeer => (pair.peer.wire_out("2", |_| true).await, pair.peer.wire_in("4", |_| true).await),
    };
    assert_eq!(fill.get(123), Some("Y"), "only admin messages were lost: {}", fill.raw());
    assert_eq!(fill.seq().to_string(), request.get(7).unwrap_or_default(), "{} {}", request.raw(), fill.raw());
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.finish().await;
}

async fn cut_order_to_peer(setup: Setup) {
    cut_order(setup, Dir::ToPeer).await;
}

async fn cut_order_to_tj(setup: Setup) {
    cut_order(setup, Dir::ToTj).await;
}

/// The connection is cut halfway through ORD1. Both sides see the close, log out and reconnect;
/// the receiver's next expected sequence number is still ORD1's, so the sender's Logon reveals
/// the gap, and ORD1 is resent with PossDupFlag=Y and delivered once. Whether the half frame
/// reached the receiver doesn't matter: a partial frame is never a message.
async fn cut_order(setup: Setup, dir: Dir) {
    let mut pair = start(setup).await;
    pair.proxy().cut_mid(dir, "D");
    match dir {
        Dir::ToPeer => drop(pair.handle.send(tj_order("ORD1")).unwrap()),
        Dir::ToTj => pair.peer.send(&peer_order("ORD1")).await,
    }
    let frame = pair.proxy().applied(dir, Fault::Cut, "D").await;
    assert!(frame.contains("|11=ORD1|"), "{frame}");
    let lost = seq_of(&frame);
    pair.proxy().disconnected().await;
    pair.tj_logged_out().await;
    pair.peer.logout().await;

    pair.logged_on().await;
    let resend_pred = |m: &FixMsg| m.get(11) == Some("ORD1") && m.get(43) == Some("Y");
    let (request, resent) = match dir {
        Dir::ToPeer => (pair.peer.wire_out("2", |_| true).await, pair.peer.wire_in("D", resend_pred).await),
        Dir::ToTj => (pair.peer.wire_in("2", |_| true).await, pair.peer.wire_out("D", resend_pred).await),
    };
    check_resend_request(&request, lost);
    assert_eq!(resent.seq(), lost, "{}", resent.raw());
    assert!(resent.get(122).is_some(), "no OrigSendingTime: {}", resent.raw());
    match dir {
        Dir::ToPeer => {
            let order = pair.peer.received_with("D", |m| m.get(11) == Some("ORD1")).await;
            assert_eq!(order.get(43), Some("Y"), "{}", order.raw());
        }
        Dir::ToTj => {
            let order = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some("ORD1")).await;
            assert_eq!(order.get(tags::POSS_DUP_FLAG), Some("Y"));
        }
    }
    pair.barrier().await;
    pair.peer_delivers_no_more("ORD1").await;
    pair.tj_delivers_no_more("ORD1").await;
    pair.finish().await;
}
