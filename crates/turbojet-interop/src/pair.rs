//! A Turbojet session connected to the QuickFIX/J peer, in either role.

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing_subscriber::EnvFilter;
use turbojet::message::tags;
use turbojet::tls::{ClientTls, ClientTrust, Identity, ServerTls, Trust};
use turbojet::{
    Acceptor, ApplVerId, Application, Context, Disconnect, Initiator, InitiatorConfig, MemoryStorage, Message,
    MessageReject, MsgType, SessionConfig, SessionHandle, SessionId,
};

use crate::mailbox::{Mailbox, Missing};
use crate::orders::{peer_order, tj_order};
use crate::peer::PeerTls;
use crate::pki::Pki;
use crate::{
    EVENT_TIMEOUT, Engine, FixMsg, Peer, PeerConfig, PeerEvent, Proxy, ProxyEvent, QFJ, QFJ_SUB, TJ, TJ_LOCATION,
    TJ_SUB,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    TjInitiator,
    TjAcceptor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Fix42,
    Fix43,
    Fix44,
    /// FIXT.1.1 carrying FIX 5.0 SP2.
    Fixt,
}

impl Version {
    pub fn begin_string(self) -> &'static str {
        match self {
            Self::Fix42 => "FIX.4.2",
            Self::Fix43 => "FIX.4.3",
            Self::Fix44 => "FIX.4.4",
            Self::Fixt => "FIXT.1.1",
        }
    }
}

/// One cell of the matrix, handed to each scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Setup {
    pub engine: Engine,
    pub role: Role,
    pub version: Version,
}

/// Session settings a scenario may change. Both sides get the same ones, except as noted.
#[derive(Debug, Clone)]
pub struct Options {
    pub heartbeat_secs: u32,
    /// The initiator (whichever side it is) logs on with ResetSeqNumFlag=Y, every time it logs
    /// on. The acceptor isn't configured to reset, so its ResetSeqNumFlag=Y is an echo.
    pub reset_on_logon: bool,
    /// How long the initiator (whichever side it is) waits before reconnecting.
    pub reconnect_secs: u32,
    /// Puts a [`Proxy`] between the two, as [`Pair::proxy`], to inject faults.
    pub proxy: bool,
    /// How far an inbound SendingTime may be from the receiver's clock before it is rejected:
    /// Turbojet's `max_latency`, QuickFIX/J's MaxLatency. 120 s, both engines' default.
    pub max_latency_secs: u32,
    /// Both sides' session IDs have SubIDs, and Turbojet's a LocationID ([`TJ_SUB`],
    /// [`TJ_LOCATION`], [`QFJ_SUB`]), so each must find its session by them.
    pub sub_ids: bool,
    /// Connects over TLS, configured so. Not with [`proxy`](Self::proxy), which can't read
    /// encrypted frames.
    pub tls: Option<Tls>,
}

/// TLS between the two, each side with a certificate from one CA (see `src/pki.rs`).
#[derive(Debug, Clone, Copy)]
pub struct Tls {
    /// The acceptor requires a client certificate (mutual TLS).
    pub client_auth: bool,
    /// The initiator presents its certificate.
    pub client_cert: bool,
    /// The initiator trusts the CA that issued the acceptor's certificate (or only another one).
    pub initiator_trusts_acceptor: bool,
    /// With `client_auth`, the acceptor trusts the CA that issued the initiator's certificate.
    pub acceptor_trusts_initiator: bool,
    /// Turbojet checks the peer's certificate against a CRL from their CA: `Some(true)` one
    /// revoking it, `Some(false)` one revoking nothing. Only Turbojet checks revocation.
    pub tj_crl_revokes_peer: Option<bool>,
}

impl Tls {
    /// The server proves who it is; the client doesn't.
    pub const SERVER: Self = Self {
        client_auth: false,
        client_cert: false,
        initiator_trusts_acceptor: true,
        acceptor_trusts_initiator: true,
        tj_crl_revokes_peer: None,
    };
    /// Both prove who they are.
    pub const MUTUAL: Self = Self { client_auth: true, client_cert: true, ..Self::SERVER };
}

impl Default for Options {
    fn default() -> Self {
        Self {
            heartbeat_secs: 30,
            reset_on_logon: false,
            reconnect_secs: 1,
            proxy: false,
            max_latency_secs: 120,
            sub_ids: false,
            tls: None,
        }
    }
}

#[derive(Debug)]
pub enum TjEvent {
    LoggedOn(SessionHandle),
    LoggedOut,
    Message(Message),
}

/// Forwards Turbojet callbacks to the test and accepts every application message.
struct Recorder {
    events: mpsc::UnboundedSender<TjEvent>,
}

impl Application for Recorder {
    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.events.send(TjEvent::LoggedOn(session.clone()));
    }

    fn on_logout(&self, _session: &SessionHandle, _ended: Disconnect) {
        let _ = self.events.send(TjEvent::LoggedOut);
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(TjEvent::Message(msg.clone()));
        Ok(())
    }
}

/// Turbojet and QuickFIX/J, connected (or connecting) to each other.
pub struct Pair {
    pub setup: Setup,
    pub peer: Peer,
    /// Turbojet's session with the peer; usable once logged on.
    pub handle: SessionHandle,
    /// The proxy between the two, with [`Options::proxy`]. The initiator connects to it, and it
    /// dials the acceptor.
    pub proxy: Option<Proxy>,
    tj: Mailbox<TjEvent>,
    task: JoinHandle<()>,
    finished: bool,
    /// The certificates, with [`Options::tls`]; their files last as long as the pair.
    _pki: Option<Pki>,
}

impl Setup {
    pub async fn start(self) -> Pair {
        self.start_with(Options::default()).await
    }

    pub async fn start_with(self, options: Options) -> Pair {
        init_tracing();
        let (events, tj) = mpsc::unbounded_channel();
        let app = Arc::new(Recorder { events });
        let storage = Arc::new(MemoryStorage::new());
        let begin_string = self.version.begin_string();
        let session = self.session_config(&options);
        let mut peer_config = self.peer_config(&options);
        let pki = options.tls.map(|tls| {
            assert!(!options.proxy, "the proxy can't read TLS");
            let pki = Pki::new();
            peer_config.tls = Some(peer_tls(&pki, tls, self.role));
            pki
        });
        let id = tj_session_id(begin_string, &options);
        let mut proxy = None;
        let (peer, handle, task) = match self.role {
            Role::TjInitiator => {
                let peer = Peer::spawn(peer_config).await;
                let mut addr = SocketAddr::from((Ipv4Addr::LOCALHOST, peer.port()));
                if options.proxy {
                    let started = Proxy::start(addr, self.role).await;
                    addr = started.addr();
                    proxy = Some(started);
                }
                let config = initiator_config(session, &id, &options);
                let mut initiator = Initiator::new(addr.to_string(), config, storage, app).unwrap();
                if let (Some(tls), Some(pki)) = (options.tls, &pki) {
                    initiator = initiator.with_tls(tj_client_tls(pki, tls).connector(), "localhost").unwrap();
                }
                let handle = initiator.handle();
                (peer, handle, tokio::spawn(initiator.run()))
            }
            Role::TjAcceptor => {
                let acceptor = Acceptor::new(session, storage, app).unwrap();
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                peer_config.port = Some(addr.port());
                if options.proxy {
                    let started = Proxy::start(addr, self.role).await;
                    peer_config.port = Some(started.port());
                    proxy = Some(started);
                }
                let handle = acceptor.handle(&id);
                let server = options.tls.zip(pki.as_ref()).map(|(tls, pki)| tj_server_tls(pki, tls));
                let task = tokio::spawn(async move {
                    let served = match server {
                        Some(server) => acceptor.serve_tls(listener, server.acceptor()).await,
                        None => acceptor.serve(listener).await,
                    };
                    if let Err(e) = served {
                        eprintln!("Turbojet acceptor stopped: {e}");
                    }
                });
                (Peer::spawn(peer_config).await, handle, task)
            }
        };
        Pair { setup: self, peer, handle, proxy, tj: Mailbox::new(tj), task, finished: false, _pki: pki }
    }
}

impl Setup {
    /// Turbojet's session settings.
    fn session_config(self, options: &Options) -> SessionConfig {
        let mut session = SessionConfig::new(self.version.begin_string(), TJ);
        if self.version == Version::Fixt {
            session = session.with_appl_ver_id(ApplVerId::Fix50Sp2);
        }
        session.max_latency = Some(Duration::from_secs(options.max_latency_secs.into()));
        session
    }

    /// The peer's settings, but for its port and TLS, which depend on Turbojet's side.
    fn peer_config(self, options: &Options) -> PeerConfig {
        PeerConfig {
            engine: self.engine,
            acceptor: self.role == Role::TjInitiator,
            begin_string: self.version.begin_string(),
            port: None,
            heartbeat_secs: options.heartbeat_secs,
            reset_on_logon: options.reset_on_logon && self.role == Role::TjAcceptor,
            reconnect_secs: options.reconnect_secs,
            max_latency_secs: options.max_latency_secs,
            sub_ids: options.sub_ids,
            tls: None,
        }
    }
}

/// Turbojet's tracing in the test output, once per test binary.
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_ansi(false)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "turbojet=debug".into()))
        .try_init()
        .ok();
}

/// The peer's side of `tls`: its certificate if it's the acceptor or presents one as initiator,
/// and the CA it trusts.
fn peer_tls(pki: &Pki, tls: Tls, role: Role) -> PeerTls {
    let peer_is_acceptor = role == Role::TjInitiator;
    let presents = peer_is_acceptor || tls.client_cert;
    let trusted = if peer_is_acceptor { tls.acceptor_trusts_initiator } else { tls.initiator_trusts_acceptor };
    PeerTls {
        identity: presents.then(|| pki.identity_paths("peer")),
        ca: pki.ca_path(trusted),
        need_client_auth: peer_is_acceptor && tls.client_auth,
    }
}

/// Turbojet as TLS client: the CA it trusts, and its certificate if it presents one.
fn tj_client_tls(pki: &Pki, tls: Tls) -> ClientTls {
    let trust = tj_trust(pki, tls, tls.initiator_trusts_acceptor);
    let identity = tls.client_cert.then(|| tj_identity(pki));
    ClientTls::new(trust, identity).unwrap()
}

/// Turbojet as TLS server: its certificate, and the CA it trusts for clients' if it asks for them.
fn tj_server_tls(pki: &Pki, tls: Tls) -> ServerTls {
    let client_trust = if tls.client_auth {
        ClientTrust::Required(tj_trust(pki, tls, tls.acceptor_trusts_initiator))
    } else {
        ClientTrust::None
    };
    ServerTls::new(tj_identity(pki), client_trust).unwrap()
}

/// The CA Turbojet trusts (the one that issued the peer's certificate, if `trusted`), with the
/// CRL `tls` says.
fn tj_trust(pki: &Pki, tls: Tls, trusted: bool) -> Trust {
    let trust = Trust::from_pem_files(&pki.ca_path(trusted)).unwrap();
    match tls.tj_crl_revokes_peer {
        Some(revokes) => trust.with_crls_pem_files(&pki.crl_path(revokes)).unwrap(),
        None => trust,
    }
}

fn tj_identity(pki: &Pki) -> Identity {
    let (cert, key) = pki.identity_paths("tj");
    Identity::from_pem_files(&cert, &key).unwrap()
}

/// Turbojet's session with the peer, as `options` has it.
fn tj_session_id(begin_string: &str, options: &Options) -> SessionId {
    let id = SessionId::new(begin_string, TJ, QFJ);
    if !options.sub_ids {
        return id;
    }
    id.with_sender_sub_id(TJ_SUB).with_sender_location_id(TJ_LOCATION).with_target_sub_id(QFJ_SUB)
}

/// Turbojet's configuration as the initiator of session `id`.
fn initiator_config(session: SessionConfig, id: &SessionId, options: &Options) -> InitiatorConfig {
    let mut config = InitiatorConfig::new(session, QFJ);
    config.sender_sub_id.clone_from(&id.sender_sub_id);
    config.sender_location_id.clone_from(&id.sender_location_id);
    config.target_sub_id.clone_from(&id.target_sub_id);
    config.heartbeat_interval = Duration::from_secs(options.heartbeat_secs.into());
    config.reset_on_logon = options.reset_on_logon;
    config.reconnect = turbojet::ReconnectPolicy::fixed(Duration::from_secs(options.reconnect_secs.into()));
    config
}

impl Pair {
    /// Waits until both sides report the session logged on.
    pub async fn logged_on(&mut self) {
        self.tj_expect("logon", |e| matches!(e, TjEvent::LoggedOn(_))).await;
        self.peer.logon().await;
    }

    /// Fails if either side logs on `within`: for a connection that must be refused.
    pub async fn never_logged_on(&mut self, within: Duration) {
        self.tj_expect_none("logon", |e| matches!(e, TjEvent::LoggedOn(_)), within).await;
        self.peer.expect_none("logon", |e| matches!(e, PeerEvent::Logon), Duration::ZERO).await;
    }

    pub async fn tj_logged_out(&mut self) {
        self.tj_expect("logout", |e| matches!(e, TjEvent::LoggedOut)).await;
    }

    /// The next application message of `msg_type` Turbojet delivered that satisfies `pred`.
    pub async fn tj_received_with(&mut self, msg_type: &str, pred: impl Fn(&Message) -> bool) -> Message {
        let what = format!("message 35={msg_type}");
        match self
            .tj_expect(&what, |e| matches!(e, TjEvent::Message(m) if m.msg_type().code() == msg_type && pred(m)))
            .await
        {
            TjEvent::Message(m) => m,
            _ => unreachable!(),
        }
    }

    pub async fn tj_received(&mut self, msg_type: &str) -> Message {
        self.tj_received_with(msg_type, |_| true).await
    }

    async fn tj_expect(&mut self, what: &str, pred: impl FnMut(&TjEvent) -> bool) -> TjEvent {
        match self.tj.expect(Instant::now() + EVENT_TIMEOUT, pred).await {
            Ok(event) => event,
            Err(Missing::Closed) => panic!("Turbojet's application went away waiting for {what}"),
            Err(Missing::TimedOut) => panic!("timed out waiting for Turbojet {what}"),
        }
    }

    /// Fails if Turbojet reports an event matching `pred`, already or `within`.
    pub async fn tj_expect_none(&mut self, what: &str, pred: impl FnMut(&TjEvent) -> bool, within: Duration) {
        if let Some(event) = self.tj.find_within(within, pred).await {
            panic!("expected no Turbojet {what}, got {event:?}");
        }
    }

    /// The proxy, for a pair started with [`Options::proxy`].
    pub fn proxy(&mut self) -> &mut Proxy {
        self.proxy.as_mut().expect("the pair was started without Options::proxy")
    }

    /// Exchanges a TestRequest and Heartbeat, so both sides have processed everything sent before.
    pub async fn barrier(&mut self) {
        self.peer.cmd("test-request BARRIER").await;
        self.peer.received_with("0", |m| m.get(112) == Some("BARRIER")).await;
    }

    /// Fails if Turbojet delivers the order `id` (again). Call after [`Pair::barrier`].
    pub async fn tj_delivers_no_more(&mut self, id: &str) {
        let order = |e: &TjEvent| matches!(e, TjEvent::Message(m) if m.get(tags::CL_ORD_ID) == Some(id));
        self.tj_expect_none(&format!("order {id}"), order, Duration::ZERO).await;
    }

    /// Fails if QuickFIX/J delivers the order `id` (again). Call after [`Pair::barrier`].
    pub async fn peer_delivers_no_more(&mut self, id: &str) {
        let order = |e: &PeerEvent| e.received().is_some_and(|m| m.msg_type() == "D" && m.get(11) == Some(id));
        self.peer.expect_none(&format!("order {id} at QuickFIX/J"), order, Duration::ZERO).await;
    }

    /// Fails unless neither side logged out or disconnected: no logout reported by either, no
    /// Logout on the wire, and no close seen by the proxy, if there is one. Call after
    /// [`Pair::barrier`].
    pub async fn stayed_up(&mut self) {
        self.tj_expect_none("logout", |e| matches!(e, TjEvent::LoggedOut), Duration::ZERO).await;
        self.peer.expect_none("logout", |e| matches!(e, PeerEvent::Logout), Duration::ZERO).await;
        let logout = |e: &PeerEvent| match e {
            PeerEvent::In(raw) | PeerEvent::Out(raw) => FixMsg::parse(raw).msg_type() == "5",
            _ => false,
        };
        self.peer.expect_none("Logout message", logout, Duration::ZERO).await;
        if let Some(proxy) = self.proxy.as_mut() {
            let close = |e: &ProxyEvent| matches!(e, ProxyEvent::Ended { .. } | ProxyEvent::Disconnected);
            proxy.expect_none("close", close, Duration::ZERO).await;
        }
    }

    /// Order `ours` from Turbojet and `theirs` from QuickFIX/J, each delivered once and not as a
    /// resend.
    pub async fn orders_each_way(&mut self, ours: &str, theirs: &str) {
        self.handle.send(tj_order(ours)).unwrap();
        let order = self.peer.received_with("D", |m| m.get(11) == Some(ours)).await;
        assert_eq!(order.get(43), None, "{}", order.raw());
        self.peer.send(&peer_order(theirs)).await;
        let order = self.tj_received_with("D", |m| m.get(tags::CL_ORD_ID) == Some(theirs)).await;
        assert_eq!(order.get(tags::POSS_DUP_FLAG), None);
        self.barrier().await;
        self.peer_delivers_no_more(ours).await;
        self.tj_delivers_no_more(theirs).await;
    }

    /// Fails on anything unexpected either side saw: see [`Peer::finish`]. Turbojet delivers
    /// BusinessMessageRejects to the application, so an unconsumed one fails too. Every scenario
    /// must end with this.
    pub async fn finish(mut self) {
        self.finished = true;
        self.peer.finish().await;
        for event in self.tj.drain() {
            assert!(
                !matches!(event, TjEvent::Message(m) if m.msg_type() == MsgType::BusinessMessageReject),
                "Turbojet received {event:?}"
            );
        }
    }
}

impl Drop for Pair {
    fn drop(&mut self) {
        // Stops the initiator or the accept loop; the acceptor's connection tasks end with the
        // test's runtime.
        self.task.abort();
        if !std::thread::panicking() {
            assert!(self.finished, "the scenario didn't call Pair::finish");
            return;
        }
        eprintln!("---- unconsumed Turbojet events ----");
        for event in self.tj.drain() {
            eprintln!("{event:?}");
        }
    }
}
