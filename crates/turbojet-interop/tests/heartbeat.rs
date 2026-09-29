//! Heartbeats and TestRequest against QuickFIX/J.

use std::time::Duration;

use turbojet_interop::{Options, PeerEvent, Setup, matrix};

matrix!(heartbeats_both_ways, answers_test_request);

/// HeartBtInt=1 is the tightest case: QuickFIX/J sends a TestRequest after 1.5 s without hearing
/// from Turbojet, so Turbojet's Heartbeats must not slip.
async fn heartbeats_both_ways(setup: Setup) {
    let mut pair = setup.start_with(Options { heartbeat_secs: 1, ..Options::default() }).await;
    pair.logged_on().await;
    for _ in 0..2 {
        let ours = pair.peer.received("0").await;
        assert_eq!(ours.get(112), None, "unsolicited heartbeat with TestReqID: {}", ours.raw());
        pair.peer.sent("0").await;
    }
    // finish sends a TestRequest of its own, but only after this.
    let test_request = |e: &PeerEvent| e.sent().is_some_and(|m| m.msg_type() == "1");
    pair.peer.expect_none("TestRequest from QuickFIX/J", test_request, Duration::ZERO).await;
    pair.finish().await;
}

async fn answers_test_request(setup: Setup) {
    let mut pair = setup.start().await;
    pair.logged_on().await;
    pair.peer.cmd("test-request T1").await;
    pair.peer.received_with("0", |m| m.get(112) == Some("T1")).await;
    pair.finish().await;
}
