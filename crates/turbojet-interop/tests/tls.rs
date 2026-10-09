//! Sessions over TLS, with both sides' certificates from one CA: the server's
//! certificate alone, and mutual TLS, carrying orders and surviving a reconnect; and handshakes
//! that must fail, so that neither side logs on.

use std::time::Duration;

use turbojet_interop::{Engine, Options, Pair, PeerEvent, Role, Setup, Tls, matrix};

matrix!(
    orders_over_tls,
    mutual_tls_survives_a_reconnect,
    initiator_refuses_an_untrusted_server,
    acceptor_refuses_a_client_without_a_certificate,
    acceptor_refuses_an_untrusted_client_certificate,
    a_crl_that_revokes_nothing_lets_the_peer_in,
    tj_refuses_a_revoked_peer_certificate,
);

/// How long a refused initiator keeps trying, reconnecting each second, without logging on.
const REFUSED_FOR: Duration = Duration::from_secs(3);

fn over(tls: Tls) -> Options {
    Options { tls: Some(tls), ..Options::default() }
}

async fn orders_over_tls(setup: Setup) {
    let mut pair = setup.start_with(over(Tls::SERVER)).await;
    pair.logged_on().await;
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.finish().await;
}

/// Dropped without a Logout, the connection is made again with a new handshake, each side
/// checking the other's certificate again, and the sequence numbers carry on.
async fn mutual_tls_survives_a_reconnect(setup: Setup) {
    let mut pair = setup.start_with(over(Tls::MUTUAL)).await;
    pair.logged_on().await;
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.peer.cmd("disconnect").await;
    pair.tj_logged_out().await;
    pair.peer.logout().await;
    pair.logged_on().await;
    // Both sent a Logon, an order and the barrier's TestRequest or Heartbeat before.
    for logon in [pair.peer.sent_with("A", |m| m.seq() > 1).await, pair.peer.received_with("A", |m| m.seq() > 1).await]
    {
        assert_eq!(logon.seq(), 4, "{}", logon.raw());
        assert_ne!(logon.get(141), Some("Y"), "{}", logon.raw());
    }
    pair.orders_each_way("ORD3", "ORD4").await;
    pair.finish().await;
}

/// The initiator trusts only a CA that didn't issue the acceptor's certificate.
async fn initiator_refuses_an_untrusted_server(setup: Setup) {
    let mut pair = setup.start_with(over(Tls { initiator_trusts_acceptor: false, ..Tls::SERVER })).await;
    refused(&mut pair, Refuser::Initiator).await;
    pair.finish().await;
}

/// The acceptor requires a client certificate and the initiator has none.
async fn acceptor_refuses_a_client_without_a_certificate(setup: Setup) {
    let mut pair = setup.start_with(over(Tls { client_cert: false, ..Tls::MUTUAL })).await;
    refused(&mut pair, Refuser::Acceptor).await;
    pair.finish().await;
}

/// The acceptor trusts only a CA that didn't issue the initiator's certificate.
async fn acceptor_refuses_an_untrusted_client_certificate(setup: Setup) {
    let mut pair = setup.start_with(over(Tls { acceptor_trusts_initiator: false, ..Tls::MUTUAL })).await;
    refused(&mut pair, Refuser::Acceptor).await;
    pair.finish().await;
}

/// Turbojet checks the peer's certificate (server or client) against its CA's CRL, which doesn't
/// revoke it: the session runs as without one.
async fn a_crl_that_revokes_nothing_lets_the_peer_in(setup: Setup) {
    let mut pair = setup.start_with(over(Tls { tj_crl_revokes_peer: Some(false), ..Tls::MUTUAL })).await;
    pair.logged_on().await;
    pair.orders_each_way("ORD1", "ORD2").await;
    pair.finish().await;
}

/// The CRL revokes the peer's certificate, so Turbojet refuses it: as initiator the server's, as
/// acceptor the client's.
async fn tj_refuses_a_revoked_peer_certificate(setup: Setup) {
    let mut pair = setup.start_with(over(Tls { tj_crl_revokes_peer: Some(true), ..Tls::MUTUAL })).await;
    let refuser = match setup.role {
        Role::TjInitiator => Refuser::Initiator,
        Role::TjAcceptor => Refuser::Acceptor,
    };
    refused(&mut pair, refuser).await;
    pair.finish().await;
}

/// Which side refuses the handshake.
#[derive(PartialEq, Eq)]
enum Refuser {
    Initiator,
    Acceptor,
}

/// Neither side logs on while the initiator keeps trying. As initiator, QuickFIX/J and QuickFIX/n
/// log each failed handshake, and quickfix-go each one it refused itself: which shows the refusal
/// came in TLS and not later. With TLS 1.3 a client learns that its certificate was refused only
/// after its side of the handshake, so quickfix-go then logs just the disconnection, and
/// QuickFIX/n the alert that ends its first read (on macOS, .NET's TLS 1.2 learns it in the
/// handshake). As acceptors, none logs a failed handshake.
async fn refused(pair: &mut Pair, refuser: Refuser) {
    pair.never_logged_on(REFUSED_FOR).await;
    if pair.setup.role == Role::TjInitiator {
        return;
    }
    match pair.setup.engine {
        Engine::QuickFixJ => {
            let handshake = |e: &PeerEvent| matches!(e, PeerEvent::QfjError(t) if t.contains("javax.net.ssl.SSLHandshakeException"));
            pair.peer.expect("QuickFIX/J's failed handshake", handshake).await;
            // One for each attempt, and it tries each second.
            pair.peer.tolerate_errors(|e| e.contains("javax.net.ssl.SSLHandshakeException"));
        }
        Engine::QuickFixGo if refuser == Refuser::Initiator => {
            let handshake = |e: &PeerEvent| e.logged().is_some_and(|t| t.starts_with("Failed handshake"));
            pair.peer.expect("quickfix-go's failed handshake", handshake).await;
        }
        Engine::QuickFixGo => {}
        Engine::QuickFixN => {
            let handshake = |e: &PeerEvent| {
                e.logged().is_some_and(|t| match refuser {
                    // Its own refusal fails its certificate check.
                    Refuser::Initiator => {
                        t.starts_with("Unable to perform authentication")
                            && t.contains("rejected by the provided RemoteCertificateValidationCallback")
                    }
                    // Turbojet's comes back as an alert: in the handshake with TLS 1.2 (.NET on
                    // macOS), or, with TLS 1.3 (on Linux), at its first read after it.
                    Refuser::Acceptor => {
                        (t.starts_with("Unable to perform authentication") && t.contains("Authentication failed"))
                            || (t.contains("disconnecting") && t.contains("alert"))
                    }
                })
            };
            pair.peer.expect("QuickFIX/n's failed handshake", handshake).await;
        }
    }
}
