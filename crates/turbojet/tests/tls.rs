//! Acceptor and Initiator over TLS, with certificates generated per test run.
#![cfg(feature = "tls")]

use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rcgen::{BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;
use turbojet::message::tags;
use turbojet::tls;
use turbojet::{
    Acceptor, Application, ConnectionInfo, Context, Endpoint, Initiator, InitiatorConfig, MemoryStorage, Message,
    MessageReject, MsgType, SessionConfig, SessionHandle,
};

/// A CA plus a server certificate for `localhost` and a client certificate, as PEM files.
struct Pki {
    dir: tempfile::TempDir,
}

impl Pki {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let pki = Self { dir };
        let ca = pki.ca("ca");
        pki.issue("server", &ca, vec!["localhost".into()], ExtendedKeyUsagePurpose::ServerAuth);
        pki.issue("client", &ca, vec!["client.example".into()], ExtendedKeyUsagePurpose::ClientAuth);
        // Trusted by TLS, but not the identity the application expects.
        pki.issue("mallory", &ca, vec!["mallory.example".into()], ExtendedKeyUsagePurpose::ClientAuth);
        // An unrelated CA and a client certificate it issued, for trust failures.
        let other = pki.ca("other-ca");
        pki.issue("stranger", &other, vec!["stranger.example".into()], ExtendedKeyUsagePurpose::ClientAuth);
        // A renewal of the server's certificate, and one from the other CA.
        pki.issue("server-renewed", &ca, vec!["localhost".into()], ExtendedKeyUsagePurpose::ServerAuth);
        pki.issue("server-other", &other, vec!["localhost".into()], ExtendedKeyUsagePurpose::ServerAuth);
        pki
    }

    /// A file's contents, as certificates arrive from a secrets store rather than a file.
    fn pem(&self, name: &str) -> Vec<u8> {
        std::fs::read(self.path(name)).unwrap()
    }

    /// The named certificate and key, from memory.
    fn identity(&self, name: &str) -> tls::Identity {
        tls::Identity::from_pem(&self.pem(&format!("{name}.pem")), &self.pem(&format!("{name}.key"))).unwrap()
    }

    fn trust(&self, ca: &str) -> tls::Trust {
        tls::Trust::from_pem(&self.pem(&format!("{ca}.pem"))).unwrap()
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    /// Writes a self-signed CA certificate and returns it as an issuer for signing others.
    fn ca(&self, name: &str) -> Issuer<'static, KeyPair> {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.distinguished_name.push(DnType::CommonName, name);
        let cert = params.self_signed(&key).unwrap();
        std::fs::write(self.path(&format!("{name}.pem")), cert.pem()).unwrap();
        Issuer::new(params, key)
    }

    fn issue(&self, name: &str, issuer: &Issuer<'_, KeyPair>, names: Vec<String>, usage: ExtendedKeyUsagePurpose) {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(names).unwrap();
        params.distinguished_name.push(DnType::CommonName, name);
        params.extended_key_usages = vec![usage];
        let cert = params.signed_by(&key, issuer).unwrap();
        std::fs::write(self.path(&format!("{name}.pem")), cert.pem()).unwrap();
        std::fs::write(self.path(&format!("{name}.key")), key.serialize_pem()).unwrap();
    }

    fn acceptor(&self, client_auth: Auth) -> tls::TlsAcceptor {
        let ca = self.path("ca.pem");
        let client_auth = match client_auth {
            Auth::None => tls::ClientAuth::None,
            Auth::Optional => tls::ClientAuth::Optional(&ca),
            Auth::Required => tls::ClientAuth::Required(&ca),
        };
        tls::acceptor(&self.path("server.pem"), &self.path("server.key"), client_auth).unwrap()
    }

    /// A connector trusting the CA in `trust`, presenting the named client certificate if any.
    fn connector(&self, trust: &str, client_cert: Option<&str>) -> tls::TlsConnector {
        let paths = client_cert.map(|name| (self.path(&format!("{name}.pem")), self.path(&format!("{name}.key"))));
        let identity = paths.as_ref().map(|(cert, key)| (cert.as_path(), key.as_path()));
        tls::connector(&self.path(trust), identity).unwrap()
    }
}

#[derive(Clone, Copy)]
enum Auth {
    None,
    Optional,
    Required,
}

#[derive(Debug)]
enum Event {
    LoggedOn,
    Message(Message),
}

/// Forwards logons and messages to a channel; acknowledges NewOrderSingle. Records the
/// ConnectionInfo seen at logon and, with `required_cn`, refuses peers whose certificate CN
/// differs.
struct Recorder {
    events: mpsc::UnboundedSender<Event>,
    verified: Mutex<Vec<ConnectionInfo>>,
    required_cn: Option<&'static str>,
}

impl Recorder {
    fn verified(&self) -> Vec<ConnectionInfo> {
        self.verified.lock().unwrap().clone()
    }
}

impl Application for Recorder {
    fn verify_logon(
        &self,
        _session: &SessionHandle,
        _logon: &Message,
        connection: &ConnectionInfo,
    ) -> Result<(), String> {
        self.verified.lock().unwrap().push(connection.clone());
        let Some(required) = self.required_cn else { return Ok(()) };
        let cn = connection.peer_certificate().and_then(|cert| cert.subject_common_name());
        if cn.as_deref() == Some(required) { Ok(()) } else { Err(format!("certificate CN {cn:?} is not {required}")) }
    }

    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.events.send(Event::LoggedOn);
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::NewOrderSingle {
            ctx.send(
                Message::new(MsgType::ExecutionReport)
                    .with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID))
                    .with_opt(tags::TEXT, msg.get(tags::TEXT)),
            );
        }
        let _ = self.events.send(Event::Message(msg.clone()));
        Ok(())
    }
}

fn recorder() -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    recorder_requiring(None)
}

fn recorder_requiring(required_cn: Option<&'static str>) -> (Arc<Recorder>, mpsc::UnboundedReceiver<Event>) {
    let (events, rx) = mpsc::unbounded_channel();
    (Arc::new(Recorder { events, verified: Mutex::default(), required_cn }), rx)
}

async fn next(rx: &mut mpsc::UnboundedReceiver<Event>) -> Event {
    timeout(Duration::from_secs(5), rx.recv()).await.expect("timed out waiting for event").expect("channel closed")
}

async fn start_acceptor(tls: tls::TlsAcceptor) -> (String, mpsc::UnboundedReceiver<Event>) {
    let (addr, events, _) = start_acceptor_with(tls, recorder()).await;
    (addr, events)
}

async fn start_acceptor_with(
    tls: tls::TlsAcceptor,
    (app, events): (Arc<Recorder>, mpsc::UnboundedReceiver<Event>),
) -> (String, mpsc::UnboundedReceiver<Event>, Arc<Recorder>) {
    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app.clone()).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve_tls(listener, tls));
    (addr, events, app)
}

fn initiator(addr: impl Into<Endpoint>) -> (Initiator, mpsc::UnboundedReceiver<Event>) {
    let (initiator, events, _) = initiator_with(addr);
    (initiator, events)
}

fn initiator_with(addr: impl Into<Endpoint>) -> (Initiator, mpsc::UnboundedReceiver<Event>, Arc<Recorder>) {
    let (app, events) = recorder();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
    config.session.logon_timeout = Duration::from_secs(2);
    // Each test initiator starts with fresh storage, so reset rather than resume sequence numbers.
    config.reset_on_logon = true;
    (Initiator::new(addr, config, Arc::new(MemoryStorage::new()), app.clone()).unwrap(), events, app)
}

/// Runs one connection attempt to completion and returns its result.
async fn attempt(initiator: &Initiator) -> io::Result<()> {
    timeout(Duration::from_secs(5), initiator.connect_once()).await.expect("connection attempt hung")
}

#[tokio::test]
async fn session_runs_over_tls() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::None)).await;
    let (initiator, mut client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    tokio::spawn(initiator.clone().run());

    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));
    initiator.handle().send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A")).unwrap();
    assert!(matches!(next(&mut server).await, Event::Message(_)));
    match next(&mut client).await {
        Event::Message(ack) => assert_eq!(ack.get(tags::CL_ORD_ID), Some("A")),
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn mutual_tls_admits_clients_with_a_certificate() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::Required)).await;
    let (initiator, mut client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", Some("client")), "localhost").unwrap();
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));
}

#[tokio::test]
async fn mutual_tls_refuses_clients_without_a_certificate() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::Required)).await;
    let (initiator, _client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    // TLS 1.3 completes the client side of the handshake first; the server's refusal arrives
    // as an alert on the first read.
    let err = attempt(&initiator).await.unwrap_err();
    assert!(err.to_string().contains("CertificateRequired"), "{err}");
    assert!(server.try_recv().is_err(), "server must not see a logon");
}

#[tokio::test]
async fn mutual_tls_refuses_certificates_from_an_untrusted_ca() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::Required)).await;
    let (initiator, _client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", Some("stranger")), "localhost").unwrap();
    assert!(attempt(&initiator).await.is_err());
    assert!(server.try_recv().is_err());
}

#[tokio::test]
async fn optional_client_auth_admits_clients_with_or_without_a_certificate() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::Optional)).await;
    for client_cert in [None, Some("client")] {
        let (initiator, mut client) = initiator(&addr);
        let initiator = initiator.with_tls(pki.connector("ca.pem", client_cert), "localhost").unwrap();
        let handle = initiator.handle();
        let connection = tokio::spawn(async move { initiator.connect_once().await });
        assert!(matches!(next(&mut client).await, Event::LoggedOn), "{client_cert:?}");
        assert!(matches!(next(&mut server).await, Event::LoggedOn), "{client_cert:?}");
        handle.logout(None).unwrap();
        connection.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn optional_client_auth_still_refuses_an_invalid_certificate() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::Optional)).await;
    let (initiator, _client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", Some("stranger")), "localhost").unwrap();
    assert!(attempt(&initiator).await.is_err());
    assert!(server.try_recv().is_err());
}

#[tokio::test]
async fn client_refuses_untrusted_server_certificate() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::None)).await;
    let (initiator, _client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("other-ca.pem", None), "localhost").unwrap();
    let err = attempt(&initiator).await.unwrap_err();
    assert!(err.to_string().contains("TLS handshake failed"), "{err}");
    assert!(server.try_recv().is_err());
}

#[tokio::test]
async fn client_refuses_wrong_server_name() {
    let pki = Pki::new();
    let (addr, _server) = start_acceptor(pki.acceptor(Auth::None)).await;
    let (initiator, _client) = initiator(&addr);
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "gateway.example").unwrap();
    let err = attempt(&initiator).await.unwrap_err();
    assert!(err.to_string().contains("TLS handshake failed"), "{err}");
}

#[tokio::test]
async fn plain_client_cannot_log_on_to_tls_acceptor() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::None)).await;
    let (initiator, _client) = initiator(&addr);
    let _ = attempt(&initiator).await;
    assert!(server.try_recv().is_err());
}

#[test]
fn loading_reports_the_offending_file() {
    let pki = Pki::new();
    let missing = pki.path("missing.pem");
    let err = tls::connector(&missing, None).err().unwrap();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);
    assert!(err.to_string().contains("missing.pem"), "{err}");

    // A key file holds no certificates.
    let err = tls::connector(&pki.path("client.key"), None).err().unwrap();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);

    // A certificate is not a private key.
    let err = tls::acceptor(&pki.path("server.pem"), &pki.path("server.pem"), tls::ClientAuth::None).err().unwrap();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    let no_key = Path::new("/nonexistent/key.pem");
    assert!(tls::acceptor(&pki.path("server.pem"), no_key, tls::ClientAuth::None).is_err());
    let no_ca = tls::ClientAuth::Optional(Path::new("/nonexistent/ca.pem"));
    assert!(tls::acceptor(&pki.path("server.pem"), &pki.path("server.key"), no_ca).is_err());
}

#[tokio::test]
async fn failover_endpoints_verify_their_own_server_names() {
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::None)).await;
    // The primary is verified against a name its certificate doesn't carry, so its handshake
    // fails; the backup (the same server here) uses the default name and succeeds.
    let primary = Endpoint::new(addr.clone()).with_tls_server_name("primary.example");
    let (initiator, mut client) = initiator(primary);
    let initiator =
        initiator.with_failover(addr.as_str()).with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));
}

#[test]
fn invalid_server_names_are_rejected_up_front() {
    let pki = Pki::new();
    let (default_name, _) = initiator("127.0.0.1:1");
    assert!(default_name.with_tls(pki.connector("ca.pem", None), "not a hostname!").is_err());

    let (endpoint_name, _) = initiator(Endpoint::new("127.0.0.1:1").with_tls_server_name("bad name"));
    assert!(endpoint_name.with_tls(pki.connector("ca.pem", None), "localhost").is_err());
}

// ---- Peer certificates in verify_logon ----

#[tokio::test]
async fn verify_logon_sees_each_sides_peer_certificate() {
    let pki = Pki::new();
    let (addr, mut server_events, server) = start_acceptor_with(pki.acceptor(Auth::Required), recorder()).await;
    let (initiator, mut client_events, client) = initiator_with(addr.as_str());
    let initiator = initiator.with_tls(pki.connector("ca.pem", Some("client")), "localhost").unwrap();
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut server_events).await, Event::LoggedOn));
    assert!(matches!(next(&mut client_events).await, Event::LoggedOn));

    // The acceptor sees the client's certificate chain, leaf first, and its address.
    let seen = &server.verified()[0];
    assert_eq!(seen.addr.map(|a| a.ip().to_string()).as_deref(), Some("127.0.0.1"));
    let leaf = seen.peer_certificate().expect("client certificate");
    assert_eq!(leaf.subject_common_name().as_deref(), Some("client"));
    assert_eq!(leaf.dns_names(), ["client.example"]);
    assert!(!leaf.der().is_empty());

    // The initiator sees the server's certificate when verifying the Logon reply.
    let seen = &client.verified()[0];
    assert_eq!(seen.addr.map(|a| a.to_string()), Some(addr));
    let leaf = seen.peer_certificate().expect("server certificate");
    assert_eq!(leaf.subject_common_name().as_deref(), Some("server"));
    assert_eq!(leaf.dns_names(), ["localhost"]);
}

#[tokio::test]
async fn verify_logon_sees_no_certificate_when_none_was_presented() {
    let pki = Pki::new();
    let (addr, mut events, server) = start_acceptor_with(pki.acceptor(Auth::Optional), recorder()).await;
    let (initiator, _client) = initiator(addr.as_str());
    tokio::spawn(initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap().run());
    assert!(matches!(next(&mut events).await, Event::LoggedOn));
    let seen = &server.verified()[0];
    assert!(seen.peer_certificates.is_empty());
    assert!(seen.addr.is_some());
}

#[tokio::test]
async fn application_can_refuse_a_trusted_certificate_for_the_wrong_identity() {
    let pki = Pki::new();
    let (addr, mut events, server) =
        start_acceptor_with(pki.acceptor(Auth::Required), recorder_requiring(Some("client"))).await;

    // Mallory's certificate passes TLS verification, but the application rejects the logon.
    let (mallory, _client) = initiator(addr.as_str());
    let mallory = mallory.with_tls(pki.connector("ca.pem", Some("mallory")), "localhost").unwrap();
    let _ = attempt(&mallory).await;
    assert!(events.try_recv().is_err(), "mallory must not log on");
    let cn = server.verified()[0].peer_certificate().and_then(|c| c.subject_common_name());
    assert_eq!(cn.as_deref(), Some("mallory"));

    // The expected identity logs on.
    let (initiator, _client) = initiator(addr.as_str());
    tokio::spawn(initiator.with_tls(pki.connector("ca.pem", Some("client")), "localhost").unwrap().run());
    assert!(matches!(next(&mut events).await, Event::LoggedOn));
}

#[tokio::test]
async fn plain_tcp_connections_report_address_without_certificates() {
    let (app, mut events) = recorder();
    let acceptor =
        Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app.clone()).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));
    let (initiator, _client) = initiator(addr.as_str());
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut events).await, Event::LoggedOn));
    let seen = &app.verified()[0];
    assert!(seen.addr.is_some());
    assert!(seen.peer_certificates.is_empty());
}

/// Regression: bursts of messages over TLS must not stall. The connection driver has to flush
/// the TLS writer after each batch, or encrypted data can sit in rustls' buffer indefinitely.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pipelined_bursts_over_tls_do_not_stall() {
    const BURSTS: usize = 20;
    const BURST: usize = 1_000;
    let pki = Pki::new();
    let (addr, mut server) = start_acceptor(pki.acceptor(Auth::None)).await;
    let (initiator, mut client) = initiator(addr.as_str());
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    let handle = initiator.handle();
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut server).await, Event::LoggedOn));

    // Realistically sized orders (~170 bytes on the wire, like a full NewOrderSingle).
    let padding = "x".repeat(100);
    for burst in 0..BURSTS {
        for i in 0..BURST {
            let order = Message::new(MsgType::NewOrderSingle)
                .with(tags::CL_ORD_ID, format!("B{burst}-{i}"))
                .with(tags::TEXT, padding.as_str());
            handle.send(order).unwrap();
        }
        let acked = async {
            let mut acks = 0;
            while acks < BURST {
                if let Event::Message(_) = client.recv().await.expect("channel closed") {
                    acks += 1;
                }
            }
        };
        timeout(Duration::from_secs(3), acked).await.unwrap_or_else(|_| panic!("burst {burst} stalled"));
    }
}

// ---- Shutdown ----
//
// A TLS handshake in progress is abandoned as soon as shutdown starts, rather than holding it up
// until the handshake times out (the 10 s logon timeout) or shutdown gives up waiting (the 5 s
// logout timeout, plus a second).

#[tokio::test]
async fn acceptor_shutdown_abandons_a_tls_handshake() {
    let pki = Pki::new();
    let (app, _events) = recorder();
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(acceptor.clone().serve_tls(listener, pki.acceptor(Auth::None)));
    // Connects, but never sends a ClientHello.
    let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let started = std::time::Instant::now();
    timeout(Duration::from_secs(3), acceptor.shutdown(None)).await.expect("shutdown hung");
    assert!(started.elapsed() < Duration::from_millis(500), "waited for the handshake: {:?}", started.elapsed());
    let read = timeout(Duration::from_secs(1), tokio::io::AsyncReadExt::read(&mut client, &mut [0; 16])).await;
    assert_eq!(read.expect("connection still open").unwrap_or(0), 0);
    server.await.unwrap().unwrap();
}

#[tokio::test]
async fn initiator_shutdown_abandons_a_tls_handshake() {
    let pki = Pki::new();
    // Accepts the connection, but never answers the ClientHello.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let silent = tokio::spawn(async move { listener.accept().await.unwrap() });
    let (initiator, _client) = initiator(addr.as_str());
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    let connecting = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.connect_once().await }
    });
    let _server_side = silent.await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let started = std::time::Instant::now();
    timeout(Duration::from_secs(3), initiator.shutdown(None)).await.expect("shutdown hung");
    assert!(started.elapsed() < Duration::from_millis(500), "waited for the handshake: {:?}", started.elapsed());
    let result = timeout(Duration::from_secs(1), connecting).await.expect("still connecting").unwrap();
    assert!(result.is_err(), "connected after shutdown");
}

/// A connection past the acceptor's limit is closed before any TLS handshake: the initiator's
/// fails at once rather than waiting out the handshake timeout.
#[tokio::test]
async fn a_connection_past_the_limit_gets_no_handshake() {
    let pki = Pki::new();
    let (app, _events) = recorder();
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app)
        .unwrap()
        .with_max_connections(1);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve_tls(listener, pki.acceptor(Auth::None)));
    // Holds the only place, waiting silently in its handshake.
    let _held = tokio::net::TcpStream::connect(&addr).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let (initiator, _client) = initiator(addr.as_str());
    let initiator = initiator.with_tls(pki.connector("ca.pem", None), "localhost").unwrap();
    let started = std::time::Instant::now();
    let err = attempt(&initiator).await.expect_err("refused");
    assert!(started.elapsed() < Duration::from_secs(1), "refused at once, not after {:?}: {err}", started.elapsed());
}

// ---- Certificates replaced while running ----

/// Serves `server` on a free port with an acceptor that records events, returning the address.
async fn serve(server: &tls::ServerTls) -> (String, mpsc::UnboundedReceiver<Event>) {
    let (app, events) = recorder();
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), app).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve_tls(listener, server.acceptor()));
    (addr, events)
}

/// The leaf certificate a server at `addr` presents now, by a handshake trusting `connector`.
async fn presented(addr: &str, connector: &tls::TlsConnector) -> Vec<u8> {
    let tcp = tokio::net::TcpStream::connect(addr).await.unwrap();
    let name = tls::ServerName::try_from("localhost").unwrap();
    let stream = timeout(Duration::from_secs(5), connector.connect(name, tcp)).await.unwrap().unwrap();
    stream.get_ref().1.peer_certificates().unwrap()[0].to_vec()
}

fn der(pem: &[u8]) -> Vec<u8> {
    use rustls::pki_types::pem::PemObject;
    rustls::pki_types::CertificateDer::from_pem_slice(pem).unwrap().to_vec()
}

use tls::rustls;

#[tokio::test]
async fn a_new_server_certificate_is_presented_from_the_next_handshake() {
    let pki = Pki::new();
    let server = tls::ServerTls::new(pki.identity("server"), tls::ClientTrust::None).unwrap();
    let (addr, mut events) = serve(&server).await;
    let (initiator, mut client) = initiator(addr.as_str());
    let client_tls = tls::ClientTls::new(pki.trust("ca"), None).unwrap();
    let initiator = initiator.with_tls(client_tls.connector(), "localhost").unwrap();
    let handle = initiator.handle();
    tokio::spawn(initiator.run());
    assert!(matches!(next(&mut client).await, Event::LoggedOn));
    assert!(matches!(next(&mut events).await, Event::LoggedOn));
    assert_eq!(presented(&addr, &client_tls.connector()).await, der(&pki.pem("server.pem")));

    server.set_identity(pki.identity("server-renewed"));
    assert_eq!(presented(&addr, &client_tls.connector()).await, der(&pki.pem("server-renewed.pem")));
    // The session connected before carries on.
    handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "after")).unwrap();
    loop {
        if let Event::Message(msg) = next(&mut client).await
            && msg.get(tags::CL_ORD_ID) == Some("after")
        {
            break;
        }
    }
}

#[test]
fn a_key_that_isnt_the_certificates_is_refused() {
    let pki = Pki::new();
    let mismatched = tls::Identity::from_pem(&pki.pem("server-renewed.pem"), &pki.pem("server.key"));
    assert_eq!(mismatched.unwrap_err().kind(), io::ErrorKind::InvalidData);
    assert!(tls::Trust::from_pem(b"").is_err(), "no CAs");
}

#[tokio::test]
async fn an_initiator_trusts_a_new_ca_once_given_it() {
    let pki = Pki::new();
    let server = tls::ServerTls::new(pki.identity("server-other"), tls::ClientTrust::None).unwrap();
    let (addr, _events) = serve(&server).await;
    let client_tls = tls::ClientTls::new(pki.trust("ca"), None).unwrap();
    let (initiator, _client) = initiator(addr.as_str());
    let initiator = initiator.with_tls(client_tls.connector(), "localhost").unwrap();
    assert!(attempt(&initiator).await.is_err(), "the server's CA isn't trusted");

    client_tls.set_trust(pki.trust("other-ca")).unwrap();
    let mut client = tokio::spawn(async move { initiator.connect_once().await });
    // Logged on: the session runs until the test ends.
    assert!(timeout(Duration::from_millis(500), &mut client).await.is_err(), "connected and running");
}

#[tokio::test]
async fn an_acceptor_trusts_new_client_cas_once_given_them() {
    let pki = Pki::new();
    let server = tls::ServerTls::new(pki.identity("server"), tls::ClientTrust::Required(pki.trust("ca"))).unwrap();
    let (addr, _events) = serve(&server).await;
    let client_tls = tls::ClientTls::new(pki.trust("ca"), Some(pki.identity("stranger"))).unwrap();
    let (initiator, _client) = initiator(addr.as_str());
    let initiator = initiator.with_tls(client_tls.connector(), "localhost").unwrap();
    assert!(attempt(&initiator).await.is_err(), "the client's CA isn't trusted");

    server.set_client_trust(tls::ClientTrust::Required(pki.trust("other-ca"))).unwrap();
    let mut client = tokio::spawn(async move { initiator.connect_once().await });
    assert!(timeout(Duration::from_millis(500), &mut client).await.is_err(), "connected and running");
}
