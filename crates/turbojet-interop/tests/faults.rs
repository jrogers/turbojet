//! Faults on the link between Turbojet and QuickFIX/J, injected by the proxy.

use std::time::Duration;

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Dir, Fault, FixMsg, Options, Pair, PeerEvent, ProxyEvent, Setup, TjEvent, matrix};

matrix!(lost_order_to_peer, lost_order_to_tj, garbled_order_to_peer, garbled_order_to_tj);

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
    stayed_up(&mut pair).await;
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
    stayed_up(&mut pair).await;
    pair.finish().await;
}

/// Whether `event` is a `msg_type` message on the wire, either way.
fn on_wire(event: &PeerEvent, msg_type: &str) -> bool {
    matches!(event, PeerEvent::In(raw) | PeerEvent::Out(raw) if FixMsg::parse(raw).msg_type() == msg_type)
}

/// Neither side logged out or disconnected. Call after [`Pair::barrier`].
async fn stayed_up(pair: &mut Pair) {
    pair.tj_expect_none("logout", |e| matches!(e, TjEvent::LoggedOut), Duration::ZERO).await;
    pair.peer.expect_none("logout", |e| matches!(e, PeerEvent::Logout), Duration::ZERO).await;
    pair.peer.expect_none("Logout message", |e| on_wire(e, "5"), Duration::ZERO).await;
    let close = |e: &ProxyEvent| matches!(e, ProxyEvent::Ended { .. } | ProxyEvent::Disconnected);
    pair.proxy().expect_none("close", close, Duration::ZERO).await;
}
