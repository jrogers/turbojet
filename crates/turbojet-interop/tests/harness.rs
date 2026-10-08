//! Checks on the harness itself.

use turbojet::message::tags;
use turbojet::{Message, MsgType};
use turbojet_interop::orders::{peer_order, tj_order};
use turbojet_interop::{Engine, Options, Role, Setup, Version, enabled, matrix};

matrix!(order_round_trip_through_proxy);

/// With no faults set, the proxy passes the session through untouched: a logon and an order each
/// way.
async fn order_round_trip_through_proxy(setup: Setup) {
    let mut pair = setup.start_with(Options { proxy: true, ..Options::default() }).await;
    pair.proxy.as_mut().unwrap().connected().await;
    pair.logged_on().await;

    pair.handle.send(tj_order("ORD1")).unwrap();
    let theirs = pair.peer.received("D").await;
    assert_eq!(theirs.get(11), Some("ORD1"), "{}", theirs.raw());
    pair.peer.send(&peer_order("ORD2")).await;
    let ours = pair.tj_received("D").await;
    assert_eq!(ours.get(tags::CL_ORD_ID), Some("ORD2"));
    pair.finish().await;
}

/// In one cell per engine: the peer rejects the order (no Side, etc.) after Turbojet has moved on;
/// finish must wait for that reject rather than pass.
#[tokio::test]
async fn finish_catches_a_late_reject() {
    if !enabled() {
        return;
    }
    for engine in [Engine::QuickFixJ, Engine::QuickFixGo] {
        let scenario = tokio::spawn(async move {
            let mut pair = Setup { engine, role: Role::TjInitiator, version: Version::Fix44 }.start().await;
            pair.logged_on().await;
            pair.handle.send(Message::new(MsgType::NewOrderSingle).with(11, "X")).unwrap();
            pair.finish().await;
        });
        let panic = scenario.await.expect_err("finish passed despite the reject").into_panic();
        let message = panic.downcast_ref::<String>().map(String::as_str).unwrap_or_default();
        // QuickFIX/J logs the reason, then sends the Reject; finish reports whichever it finds first.
        assert!(message.contains("35=3") || message.contains("Required tag missing"), "{engine:?}: {message}");
    }
}
