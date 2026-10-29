# TLS

Enable the `tls` feature of `turbojet` (rustls with the `ring` provider; nothing TLS-related is
compiled without it).

## From files

An acceptor needs its certificate and key, and says whether it asks for client certificates
(mutual TLS); an initiator trusts a CA, verifies the server's name, and may present a certificate
of its own:

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
# let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
use turbojet::tls::ClientAuth;

// Acceptor: server certificate, and client authentication (mutual TLS):
//   ClientAuth::None           - no client certificates
//   ClientAuth::Optional(ca)   - request one; admit clients without, refuse invalid ones
//   ClientAuth::Required(ca)   - refuse clients without a valid certificate
let tls = turbojet::tls::acceptor("server.pem".as_ref(), "server.key".as_ref(), ClientAuth::Required("ca.pem".as_ref()))?;
let listener = tokio::net::TcpListener::bind("0.0.0.0:9876").await?;
tokio::spawn(acceptor.serve_tls(listener, tls));

// Initiator: trust a CA, verify the server name, optionally present a client certificate.
let tls = turbojet::tls::connector("ca.pem".as_ref(), Some(("client.pem".as_ref(), "client.key".as_ref())))?;
config.tls = Some(InitiatorTls::new(tls, "gateway.example.com"));
# Ok(())
# }
```

Handshakes run on each connection's own task and are bounded by the logon timeout. So that an
acceptor can't be served without TLS by mistake, set `AcceptorConfig::require_tls`: its plain
`serve` then fails at once rather than accepting connections. An acceptor from a sessions file with
`[acceptor.tls]` has it set.

## From memory, replaced while running

Certificates can also come from memory (a secrets manager, say) and be replaced while running.
`ServerTls` and `ClientTls` hold them: a change applies from the next handshake, and sessions
already connected carry on. Every handshake is a full one, checked against the certificates
current then (sessions aren't resumed).

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
# let listener = tokio::net::TcpListener::bind("0.0.0.0:9876").await?;
# let (cert_pem, key_pem, ca_pem, new_cert_pem, new_key_pem) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
use turbojet::tls::{ClientTrust, Identity, ServerTls, Trust};

let server = ServerTls::new(Identity::from_pem(&cert_pem, &key_pem)?, ClientTrust::Required(Trust::from_pem(&ca_pem)?))?;
tokio::spawn(acceptor.serve_tls(listener, server.acceptor()));
// Later, when the certificate is renewed:
server.set_identity(Identity::from_pem(&new_cert_pem, &new_key_pem)?);
# Ok(())
# }
```

`ClientTls` does the same for an initiator (`set_trust`, `set_identity`). An `Identity` is
checked when it's built (the key must be the certificate's), so a bad renewal is refused and
the old one kept. The example gateway reloads its certificate, key, client CAs and CRLs from
their files on SIGHUP.

## PKCS#12 bundles

With the `pkcs12` feature, an `Identity` can also come from a PKCS#12 bundle (`.p12`, `.pfx`),
as Java and Windows tools export them, in the AES form or the older 3DES and RC2 one:

```rust,no_run
# let password = String::new();
use turbojet::tls::Identity;

let identity = Identity::from_pkcs12_file("gateway.p12".as_ref(), &password)?;
# Ok::<(), std::io::Error>(())
```

The bundle must hold one private key and its certificate chain; trusted CAs still come as PEM.
In a sessions file, `pkcs12 = "gateway.p12"` takes the place of `cert` and `key`, and
`pkcs12_password_env` names the environment variable holding its password.

## Revocation

Revocation is checked only if you give certificate revocation lists (CRLs); without them,
nothing changes. Add them to a `Trust`, and certificates they revoke are refused at the
handshake:

```rust,no_run
# let identity = turbojet::tls::Identity::from_pem_files("server.pem".as_ref(), "server.key".as_ref())?;
use turbojet::tls::{ClientTrust, ServerTls, Trust};

let trust = Trust::from_pem_files("ca.pem".as_ref())?.with_crls_pem_files("ca.crl.pem".as_ref())?;
let server = ServerTls::new(identity, ClientTrust::Required(trust))?;
// When the CA publishes a new CRL:
server.set_client_trust(ClientTrust::Required(Trust::from_pem_files("ca.pem".as_ref())?.with_crls_pem_files("new.crl.pem".as_ref())?))?;
# Ok::<(), std::io::Error>(())
```

With CRLs given, every certificate in the chain is checked, and one that no CRL covers is
refused: give a CRL from each CA in the chain, intermediates included. Turbojet doesn't fetch
CRLs or check OCSP; refreshing them is up to you. A CRL past its next update is still used, so a
late refresh doesn't lock out your counterparties, but it's logged as a warning when it's put in
use and at each handshake it covers, since a certificate revoked after it was issued gets in.
`Trust::refuse_expired_crls()` refuses it instead: putting the trust in use is then an error, and
a CRL that expires while in use fails each handshake until you give a current one. In a sessions
file, `client_crl` in `[acceptor.tls]` and `crl` in an initiator's `tls` name a PEM file of
CRLs, and `refuse_expired_client_crls = true` and `refuse_expired_crls = true` refuse expired ones.

## Other setups

For other setups (system roots, custom verifiers, other protocol versions or cipher suites), build a
`rustls::ServerConfig` or `ClientConfig` yourself and wrap it with `TlsAcceptor::from` or
`TlsConnector::from`.

## Certificates and CompIDs

`Application::verify_logon` receives a `ConnectionInfo` with the remote address and the peer's
verified certificate chain (the client's on an acceptor, the server's on an initiator). With
the `tls` feature, `PeerCertificate::subject_common_name()` and `dns_names()` make it easy to tie
a certificate to a CompID. The example gateway does this with `--tls-match-comp-id`: a client that
presents a certificate may only log on as the SenderCompID named by its CN or a DNS name.
