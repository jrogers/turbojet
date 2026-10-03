//! SequenceReset-Reset and a MsgSeqNum that is too low, against QuickFIX/J.

use std::time::Duration;

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Options, PeerEvent, Setup, TjEvent, matrix};

matrix!(sequence_reset_from_peer, sequence_reset_from_tj, seq_too_low_at_tj, seq_too_low_at_peer);

/// Longer than a scenario's 120s backstop.
const NO_RECONNECT: Options = Options { heartbeat_secs: 30, reset_on_logon: false, reconnect_secs: 300, proxy: false };

/// QuickFIX/J jumps its sequence to 20 with a SequenceReset-Reset: Turbojet accepts the next
/// message at 20 without asking for 3-19.
async fn sequence_reset_from_peer(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.cmd("sequence-reset 20").await;
    let reset = pair.peer.wire_out("4", |_| true).await;
    assert_eq!(reset.get(36), Some("20"), "{}", reset.raw());
    assert_ne!(reset.get(123), Some("Y"), "{}", reset.raw());
    pair.peer.send(&peer_order("ORD1")).await;

    let order = pair.tj_received("D").await;
    assert_eq!(order.get(tags::MSG_SEQ_NUM), Some("20"));
    assert_eq!(pair.handle.sequence_numbers().await.unwrap().next_incoming, 21);
    pair.barrier().await;
    pair.peer
        .expect_none("ResendRequest", |e| matches!(e, PeerEvent::In(m) if m.contains("|35=2|")), Duration::ZERO)
        .await;
    pair.finish().await;
}

/// Turbojet's operator moves its outgoing sequence to 20: QuickFIX/J is told with a SequenceReset
/// in gap-fill mode, in sequence, and accepts the next message at 20.
async fn sequence_reset_from_tj(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.handle.set_next_outgoing(20).await.unwrap();
    let reset = pair.peer.received("4").await;
    assert_eq!(reset.get(36), Some("20"), "{}", reset.raw());
    assert_eq!(reset.get(123), Some("Y"), "{}", reset.raw());
    pair.handle.send(tj_order("ORD1")).unwrap();

    let order = pair.peer.received("D").await;
    assert_eq!(order.seq(), 20, "{}", order.raw());
    pair.barrier().await;
    pair.peer
        .expect_none("ResendRequest", |e| matches!(e, PeerEvent::Out(m) if m.contains("|35=2|")), Duration::ZERO)
        .await;
    pair.finish().await;
}

/// Turbojet expects 50 but QuickFIX/J sends 2 without PossDupFlag: Turbojet logs out, giving the
/// reason, and disconnects (FIX 4.4 Vol 2: a serious error; terminate the session with a Logout).
///
/// This and [`seq_too_low_at_peer`] end logged out. The initiator would reconnect and fail again,
/// so it waits longer than the scenario lasts.
async fn seq_too_low_at_tj(setup: Setup) {
    let mut pair = setup.start_with(NO_RECONNECT).await;
    pair.logged_on().await;
    pair.handle.set_next_incoming(50).await.unwrap();
    pair.peer.send(&peer_order("ORD1")).await;

    let logout = pair.peer.wire_in("5", |_| true).await;
    assert!(logout.get(58).is_some_and(|t| t.contains("too low")), "{}", logout.raw());
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    let order = |e: &TjEvent| matches!(e, TjEvent::Message(m) if m.get(tags::CL_ORD_ID) == Some("ORD1"));
    pair.tj_expect_none("order ORD1", order, Duration::ZERO).await;
    pair.finish().await;
}

/// QuickFIX/J expects 50 but Turbojet sends 2: QuickFIX/J logs out, giving the reason, and
/// disconnects.
async fn seq_too_low_at_peer(setup: Setup) {
    let mut pair = setup.start_with(NO_RECONNECT).await;
    pair.logged_on().await;
    pair.peer.cmd("set-next-target-seq 50").await;
    pair.handle.send(tj_order("ORD1")).unwrap();

    let logout = pair.peer.wire_out("5", |_| true).await;
    assert!(logout.get(58).is_some_and(|t| t.contains("too low")), "{}", logout.raw());
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    for error in ["quickfix.SessionException MsgSeqNum too low", "Disconnecting: Verifying message failed"] {
        pair.peer.expect(error, |e| matches!(e, PeerEvent::QfjError(t) if t.starts_with(error))).await;
    }
    // QuickFIX/J disconnects without waiting for Turbojet's Logout reply (FIXT.1.1: waiting is
    // optional). If it still reads the reply, it logs an error for a Logout while logged out.
    pair.peer.tolerate_errors(|e| e.contains("Logon state is not valid for message (MsgType=5)"));
    pair.peer.expect_none("ORD1", |e| matches!(e, PeerEvent::FromApp(_)), Duration::ZERO).await;
    pair.finish().await;
}
