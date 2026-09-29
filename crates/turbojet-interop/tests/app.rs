//! Application messages both ways, field for field, against QuickFIX/J.

use turbojet::message::tags;
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Setup, matrix};

matrix!(order_round_trip);

async fn order_round_trip(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;

    pair.handle.send(tj_order("ORD1")).unwrap();
    let theirs = pair.peer.received("D").await;
    assert_eq!(theirs.get(8), Some(setup.version.begin_string()), "{}", theirs.raw());
    for (tag, value) in
        [(11, "ORD1"), (21, "1"), (54, "1"), (60, "20260929-12:00:00.000"), (40, "1"), (55, "AAPL"), (38, "100")]
    {
        assert_eq!(theirs.get(tag), Some(value), "tag {tag}: {}", theirs.raw());
    }
    // The session's DefaultApplVerID applies; no per-message ApplVerID on the wire.
    let wire = pair.peer.wire_in("D", |_| true).await;
    assert_eq!(wire.get(1128), None, "{}", wire.raw());

    pair.peer.send(&peer_order("ORD2")).await;
    let ours = pair.tj_received("D").await;
    for (tag, value) in [
        (tags::CL_ORD_ID, "ORD2"),
        (tags::HANDL_INST, "1"),
        (tags::SIDE, "1"),
        (tags::TRANSACT_TIME, "20260929-12:00:00.000"),
        (tags::ORD_TYPE, "1"),
        (tags::SYMBOL, "AAPL"),
        (tags::ORDER_QTY, "100"),
    ] {
        assert_eq!(ours.get(tag), Some(value), "tag {tag}");
    }
    pair.finish().await;
}
