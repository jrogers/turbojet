//! Sessions identified by SubIDs and LocationIDs as well as CompIDs, against QuickFIX/J.

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Options, QFJ_SUB, Setup, TJ_LOCATION, TJ_SUB, matrix};

matrix!(sub_ids_identify_the_session);

/// Each side finds its session by the other's SubIDs and LocationID, and every message either
/// sends, the Logon and the orders, carries its own.
async fn sub_ids_identify_the_session(setup: Setup) {
    let mut pair = setup.start_with(Options { sub_ids: true, ..Options::default() }).await;
    pair.logged_on().await;
    pair.handle.send(tj_order("ORD1")).unwrap();
    pair.peer.received_with("D", |m| m.get(11) == Some("ORD1")).await;
    pair.peer.send(&peer_order("ORD2")).await;
    let theirs = pair.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some("ORD2")).await;
    assert_eq!(theirs.get(tags::SENDER_SUB_ID), Some(QFJ_SUB), "{theirs:?}");
    assert_eq!(theirs.get(tags::TARGET_SUB_ID), Some(TJ_SUB), "{theirs:?}");

    for msg_type in ["A", "D"] {
        let ours = pair.peer.wire_in(msg_type, |_| true).await;
        assert_eq!(ours.get(50), Some(TJ_SUB), "{}", ours.raw());
        assert_eq!(ours.get(142), Some(TJ_LOCATION), "{}", ours.raw());
        assert_eq!(ours.get(57), Some(QFJ_SUB), "{}", ours.raw());
        let sent = pair.peer.wire_out(msg_type, |_| true).await;
        assert_eq!(sent.get(50), Some(QFJ_SUB), "{}", sent.raw());
        assert_eq!(sent.get(57), Some(TJ_SUB), "{}", sent.raw());
        assert_eq!(sent.get(143), Some(TJ_LOCATION), "{}", sent.raw());
    }
    pair.finish().await;
}
