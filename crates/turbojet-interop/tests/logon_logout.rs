//! Logon and logout against QuickFIX/J.

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Options, Pair, Setup, Version, matrix};

matrix!(plain_logon, logon_with_reset, logout_from_tj, logout_from_peer, reconnect_continues_sequence);

async fn plain_logon(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    let theirs = pair.peer.sent("A").await;
    let ours = pair.peer.received("A").await;
    for logon in [&theirs, &ours] {
        assert_eq!(logon.seq(), 1, "{}", logon.raw());
        assert_eq!(logon.get(108), Some("30"), "{}", logon.raw());
    }
    if setup.version == Version::Fixt {
        // DefaultApplVerID(1137) 9 = FIX50SP2, sent by the initiator and echoed by the acceptor.
        assert_eq!(ours.get(1137), Some("9"), "{}", ours.raw());
        assert_eq!(theirs.get(1137), Some("9"), "{}", theirs.raw());
    }
    pair.finish().await;
}

/// The initiator logs on with ResetSeqNumFlag=Y each time: after some traffic and a dropped
/// connection, both Logons are at 1 again and the acceptor echoes the flag.
async fn logon_with_reset(setup: Setup) {
    let mut pair = setup.start_with(Options { reset_on_logon: true, ..Options::default() }).await;
    pair.logged_on().await;
    for logon in [pair.peer.wire_in("A", |_| true).await, pair.peer.wire_out("A", |_| true).await] {
        assert_eq!(logon.seq(), 1, "{}", logon.raw());
        assert_eq!(logon.get(141), Some("Y"), "{}", logon.raw());
    }
    exchange_orders(&mut pair, "ORD1", "ORD2").await;

    // Drop the connection without a Logout; whichever side is the initiator reconnects in ~1s.
    pair.peer.cmd("disconnect").await;
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    pair.logged_on().await;
    for logon in [pair.peer.wire_in("A", |_| true).await, pair.peer.wire_out("A", |_| true).await] {
        assert_eq!(logon.seq(), 1, "{}", logon.raw());
        assert_eq!(logon.get(141), Some("Y"), "{}", logon.raw());
    }
    exchange_orders(&mut pair, "ORD3", "ORD4").await;
    pair.finish().await;
}

/// Sends order `ours` from Turbojet and `theirs` from QuickFIX/J, checking both come
/// straight after the Logon, at MsgSeqNum 2.
async fn exchange_orders(pair: &mut Pair, ours: &str, theirs: &str) {
    pair.handle.send(tj_order(ours)).unwrap();
    let order = pair.peer.received_with("D", |m| m.get(11) == Some(ours)).await;
    assert_eq!(order.seq(), 2, "{}", order.raw());
    pair.peer.send(&peer_order(theirs)).await;
    let order = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some(theirs)).await;
    assert_eq!(order.get(tags::MSG_SEQ_NUM), Some("2"), "{order:?}");
}

async fn logout_from_tj(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.handle.logout(Some("done")).unwrap();
    let request = pair.peer.received("5").await;
    assert_eq!(request.get(58), Some("done"), "{}", request.raw());
    pair.peer.sent("5").await;
    pair.peer.logout().await;
    pair.tj_logged_out().await;
    pair.finish().await;
}

async fn logout_from_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.cmd("logout").await;
    pair.peer.sent("5").await;
    pair.peer.received("5").await;
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    pair.finish().await;
}

async fn reconnect_continues_sequence(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.sent("A").await;
    pair.peer.received("A").await;
    // Drop the connection without a Logout; whichever side is the initiator reconnects in ~1s.
    pair.peer.cmd("disconnect").await;
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    pair.logged_on().await;
    // Sequence numbers carry on: nothing was reset.
    for logon in [pair.peer.sent("A").await, pair.peer.received("A").await] {
        assert_eq!(logon.seq(), 2, "{}", logon.raw());
        assert_ne!(logon.get(141), Some("Y"), "{}", logon.raw());
    }
    pair.finish().await;
}
