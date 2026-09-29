//! Checks on the harness itself, in one cell of the matrix.

use turbojet::{Message, MsgType};
use turbojet_interop::{Role, Setup, Version, enabled};

/// QuickFIX/J rejects the order (no Side, etc.) after Turbojet has moved on; finish must wait
/// for that reject rather than pass.
#[tokio::test]
async fn finish_catches_a_late_reject() {
    if !enabled() {
        return;
    }
    let scenario = tokio::spawn(async {
        let mut pair = Setup { role: Role::TjInitiator, version: Version::Fix44 }.start().await;
        pair.logged_on().await;
        pair.handle.send(Message::new(MsgType::NewOrderSingle).with(11, "X")).unwrap();
        pair.finish().await;
    });
    let panic = scenario.await.expect_err("finish passed despite the reject").into_panic();
    let message = panic.downcast_ref::<String>().map(String::as_str).unwrap_or_default();
    // QuickFIX/J logs the reason, then sends the Reject; finish reports whichever it finds first.
    assert!(message.contains("35=3") || message.contains("Required tag missing"), "wrong failure: {message}");
}
