//! Gaps and resends in both directions against QuickFIX/J.

use std::time::Duration;

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Setup, matrix};

matrix!(gap_fill_from_peer, gap_fill_to_peer, resend_to_peer, resend_from_peer);

/// QuickFIX/J skips sequence numbers 2-9: Turbojet asks for them and QuickFIX/J, having sent
/// nothing, fills the gap. Then the order that revealed the gap, queued meanwhile, is delivered,
/// once.
async fn gap_fill_from_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.cmd("set-next-sender-seq 10").await;
    pair.peer.send(&peer_order("ORD1")).await;

    let request = pair.peer.wire_in("2", |_| true).await;
    assert_eq!(request.get(7), Some("2"), "{}", request.raw());
    let fill = pair.peer.wire_out("4", |m| m.get(123) == Some("Y")).await;
    assert_eq!(fill.seq(), 2, "{}", fill.raw());
    assert_eq!(fill.get(36), Some("10"), "{}", fill.raw());
    assert_eq!(fill.get(43), Some("Y"), "{}", fill.raw());

    // Turbojet queues the message that revealed the gap, so the order it delivers is the
    // original; QuickFIX/J's resend of it (the request was open-ended) is then a duplicate.
    let order = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some("ORD1")).await;
    assert_eq!(order.get(tags::MSG_SEQ_NUM), Some("10"));
    assert_eq!(order.get(tags::POSS_DUP_FLAG), None);
    assert_eq!(pair.handle.sequence_numbers().await.unwrap().next_incoming, 11);
    pair.barrier().await;
    pair.tj_delivers_no_more("ORD1").await;
    pair.finish().await;
}

/// QuickFIX/J forgets it received Turbojet's Logon (seq 1): Turbojet must gap-fill over the
/// Logon rather than resend it.
async fn gap_fill_to_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.cmd("set-next-target-seq 1").await;
    pair.handle.send(tj_order("ORD1")).unwrap();

    let request = pair.peer.wire_out("2", |_| true).await;
    assert_eq!(request.get(7), Some("1"), "{}", request.raw());
    let fill = pair.peer.wire_in("4", |m| m.get(123) == Some("Y")).await;
    assert_eq!(fill.seq(), 1, "{}", fill.raw());
    assert_eq!(fill.get(36), Some("2"), "{}", fill.raw());
    assert_eq!(fill.get(43), Some("Y"), "{}", fill.raw());
    assert!(fill.get(122).is_some(), "no OrigSendingTime: {}", fill.raw());

    // QuickFIX/J queues the order that revealed the gap and processes it once the gap is filled;
    // the resend of it then comes in too low and, being PossDupFlag=Y, is ignored.
    let order = pair.peer.received_with("D", |m| m.get(11) == Some("ORD1")).await;
    assert_eq!(order.seq(), 2, "{}", order.raw());
    // The original, not the resend: QuickFIX/J's re-serialization keeps PossDupFlag.
    assert_eq!(order.get(43), None, "{}", order.raw());
    let resent = pair.peer.wire_in("D", |m| m.get(43) == Some("Y")).await;
    assert_eq!(resent.seq(), 2, "{}", resent.raw());
    pair.barrier().await;
    pair.peer
        .expect_none("second ORD1", |e| e.received().is_some_and(|m| m.get(11) == Some("ORD1")), Duration::ZERO)
        .await;
    pair.finish().await;
}

/// QuickFIX/J loses three orders from Turbojet: Turbojet resends them with PossDupFlag and
/// OrigSendingTime.
async fn resend_to_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    for id in ["ORD1", "ORD2", "ORD3"] {
        pair.handle.send(tj_order(id)).unwrap();
        pair.peer.received_with("D", |m| m.get(11) == Some(id)).await;
    }
    pair.peer.cmd("set-next-target-seq 2").await;
    pair.handle.send(tj_order("ORD4")).unwrap();

    let request = pair.peer.wire_out("2", |_| true).await;
    assert_eq!(request.get(7), Some("2"), "{}", request.raw());
    for (seq, id) in [(2, "ORD1"), (3, "ORD2"), (4, "ORD3")] {
        let resent = pair.peer.wire_in("D", |m| m.get(11) == Some(id) && m.get(43) == Some("Y")).await;
        assert_eq!(resent.seq(), seq, "{}", resent.raw());
        assert!(resent.get(122).is_some(), "no OrigSendingTime: {}", resent.raw());
        let delivered = pair.peer.received_with("D", |m| m.get(11) == Some(id)).await;
        assert_eq!(delivered.get(43), Some("Y"), "{}", delivered.raw());
    }
    let last = pair.peer.received_with("D", |m| m.get(11) == Some("ORD4")).await;
    assert_eq!(last.seq(), 5, "{}", last.raw());
    // The original, unlike the resends above.
    assert_eq!(last.get(43), None, "{}", last.raw());
    // QuickFIX/J processed the original from its queue; the resend (the ResendRequest was
    // open-ended) comes in too low, PossDupFlag=Y, and is ignored.
    pair.peer.wire_in("D", |m| m.get(11) == Some("ORD4") && m.get(43) == Some("Y")).await;
    pair.barrier().await;
    pair.peer
        .expect_none("second ORD4", |e| e.received().is_some_and(|m| m.get(11) == Some("ORD4")), Duration::ZERO)
        .await;
    pair.finish().await;
}

/// Turbojet loses three orders from QuickFIX/J: QuickFIX/J resends them and Turbojet delivers
/// them again, marked PossDupFlag=Y, then the order that revealed the gap.
async fn resend_from_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    for id in ["ORD1", "ORD2", "ORD3"] {
        pair.peer.send(&peer_order(id)).await;
        pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some(id)).await;
    }
    pair.handle.set_next_incoming(2).await.unwrap();
    pair.peer.send(&peer_order("ORD4")).await;

    let request = pair.peer.wire_in("2", |_| true).await;
    assert_eq!(request.get(7), Some("2"), "{}", request.raw());
    for id in ["ORD1", "ORD2", "ORD3"] {
        let resent = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some(id)).await;
        assert_eq!(resent.get(tags::POSS_DUP_FLAG), Some("Y"));
        assert!(resent.get(tags::ORIG_SENDING_TIME).is_some());
    }
    let last = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some("ORD4")).await;
    assert_eq!(last.get(tags::POSS_DUP_FLAG), None, "the original, queued behind the gap");
    pair.barrier().await;
    pair.tj_delivers_no_more("ORD4").await;
    pair.finish().await;
}
