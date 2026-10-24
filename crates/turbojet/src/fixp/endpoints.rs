//! FIXP over TCP: a server accepting clients, and a client connecting and reconnecting.

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tracing::{Instrument, info, warn};

use super::{ClientConfig, FixpApplication, FixpConfig, FixpHandle, FixpRegistry, FixpSession, ServerConfig};
use crate::acceptor::{AcceptorConfig, Limits, accept_loop};
use crate::allowed::AllowedIps;
use crate::connection;
use crate::initiator::{Endpoint, endpoints_problem};
use crate::peer::ConnectionInfo;
use crate::reconnect::{Backoff, ReconnectPolicy};
use crate::session::ConfigError;
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};

/// A [`FixpAcceptor`]'s configuration: the server's identity, its sessions' settings, and how it
/// accepts connections, as an [`AcceptorConfig`] is a FIX acceptor's. Fixed once the acceptor is
/// made, except the allowed addresses ([`FixpAcceptor::set_allowed_ips`]).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct FixpAcceptorConfig {
    /// The server's name and credentials.
    pub server: ServerConfig,
    /// Settings for its sessions.
    pub session: FixpConfig,
    /// The most connections kept open at once; see [`AcceptorConfig::max_connections`].
    pub max_connections: usize,
    /// The most connections from one IP address kept open at once; see
    /// [`AcceptorConfig::max_connections_per_ip`].
    pub max_connections_per_ip: usize,
    /// Accepts connections only from these addresses; see [`AcceptorConfig::allowed_ips`].
    pub allowed_ips: Option<AllowedIps>,
}

impl FixpAcceptorConfig {
    /// A configuration for `server` with the default session settings ([`FixpConfig::new`]),
    /// from any address, with the FIX acceptor's default connection limits.
    pub fn new(server: ServerConfig) -> Self {
        Self {
            server,
            session: FixpConfig::new(),
            max_connections: AcceptorConfig::DEFAULT_MAX_CONNECTIONS,
            max_connections_per_ip: AcceptorConfig::DEFAULT_MAX_CONNECTIONS_PER_IP,
            allowed_ips: None,
        }
    }
}

impl From<ServerConfig> for FixpAcceptorConfig {
    fn from(server: ServerConfig) -> Self {
        Self::new(server)
    }
}

/// A FIXP server: accepts connections and runs a server session on each, for any client that
/// negotiates or establishes. Cheap to clone; clones share sessions, storage and
/// [shutdown](Self::shutdown).
///
/// Any client that can reach the listener may negotiate unless [`FixpApplication::verify`]
/// refuses it, and each session gets its own stored state. It keeps at most
/// [`FixpAcceptorConfig::max_connections`] connections open at once, and at most
/// [`max_connections_per_ip`](FixpAcceptorConfig::max_connections_per_ip) from one IP address,
/// as the FIX [`Acceptor`](crate::Acceptor) does.
#[derive(Clone)]
pub struct FixpAcceptor {
    config: FixpConfig,
    server: ServerConfig,
    registry: Arc<FixpRegistry>,
    app: Arc<dyn FixpApplication>,
    shutdown: Arc<Shutdown>,
    limits: Arc<Limits>,
}

impl std::fmt::Debug for FixpAcceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixpAcceptor").field("server", &self.server).finish_non_exhaustive()
    }
}

impl FixpAcceptor {
    /// A server serving `app`, keeping session state in `storage`. `config` is a
    /// [`FixpAcceptorConfig`], or a [`ServerConfig`] for the default settings.
    ///
    /// # Errors
    ///
    /// If the session settings are invalid; see [`FixpConfig::check`].
    pub fn new(
        config: impl Into<FixpAcceptorConfig>,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn FixpApplication>,
    ) -> Result<Self, ConfigError> {
        let FixpAcceptorConfig { server, session: config, max_connections, max_connections_per_ip, allowed_ips } =
            config.into();
        config.check()?;
        let registry = Arc::new(FixpRegistry::with_storage(storage).with_clock(config.clock.clone()));
        let limits = Arc::new(Limits::new(max_connections, max_connections_per_ip));
        limits.set_allowed(allowed_ips);
        Ok(Self { config, server, registry, app, shutdown: Arc::new(Shutdown::new()), limits })
    }

    /// Replaces the addresses connections are accepted from, while serving: `None` accepts any.
    /// Connections already open stay open.
    pub fn set_allowed_ips(&self, allowed: Option<crate::AllowedIps>) {
        self.limits.set_allowed(allowed);
    }

    /// A handle to session `id` (as [`sessions`](Self::sessions) lists them), usable whenever it's
    /// connected.
    #[must_use]
    pub fn handle(&self, id: &SessionId) -> FixpHandle {
        self.registry.handle(id.clone())
    }

    /// Sessions currently connected.
    #[must_use]
    pub fn sessions(&self) -> Vec<SessionId> {
        self.registry.sessions()
    }

    /// Sessions currently connected, each with when and how it connected and where it stands:
    /// see [`SessionRegistry::statuses`](crate::SessionRegistry::statuses).
    #[must_use]
    pub fn statuses(&self) -> Vec<crate::SessionStatus> {
        self.registry.statuses()
    }

    /// Events for this acceptor's sessions from now on: see
    /// [`SessionRegistry::subscribe`](crate::SessionRegistry::subscribe).
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<crate::SessionEvent> {
        self.registry.subscribe()
    }

    /// Accepts TCP connections, running each on its own task, until [shutdown](Self::shutdown)
    /// starts or the future is dropped; then returns `Ok`.
    ///
    /// # Errors
    ///
    /// None in practice: accept errors are logged and retried, and a connection's own errors end
    /// only that connection.
    pub async fn serve(self, listener: TcpListener) -> io::Result<()> {
        let (shutdown, limits) = (self.shutdown.clone(), self.limits.clone());
        accept_loop(listener, &shutdown, &limits, |stream| {
            let acceptor = self.clone();
            async move {
                let connection = ConnectionInfo::new(stream.peer_addr().ok(), Vec::new());
                acceptor.run_connection(stream, connection).await
            }
        })
        .await
    }

    /// Like [`serve`](Self::serve), for TLS connections; [`FixpApplication::verify`] sees the
    /// certificate a client presented, if `tls` asks for one. Each handshake runs on the
    /// connection's own task and must complete within the configuration's `handshake_timeout`.
    ///
    /// # Errors
    ///
    /// As [`serve`](Self::serve); a failed or slow handshake ends only its connection.
    #[cfg(feature = "tls")]
    pub async fn serve_tls(self, listener: TcpListener, tls: crate::tls::TlsAcceptor) -> io::Result<()> {
        let (shutdown, limits) = (self.shutdown.clone(), self.limits.clone());
        accept_loop(listener, &shutdown, &limits, |stream| {
            let (acceptor, tls) = (self.clone(), tls.clone());
            async move {
                let addr = stream.peer_addr().ok();
                let handshake = tokio::time::timeout(acceptor.config.handshake_timeout, tls.accept(stream));
                let mut shutdown = acceptor.shutdown.signal();
                let stream = tokio::select! {
                    handshake = handshake => handshake,
                    _ = shutdown.started() => return Err(crate::acceptor::shutting_down()),
                };
                let stream = stream
                    .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "TLS handshake timed out"))?
                    .map_err(|e| io::Error::new(e.kind(), format!("TLS handshake failed: {e}")))?;
                let certificates = crate::tls::peer_certificates(stream.get_ref().1.peer_certificates());
                info!(client_certificate = !certificates.is_empty(), "TLS handshake complete");
                acceptor.run_connection(stream, ConnectionInfo::new(addr, certificates)).await
            }
        })
        .await
    }

    /// Runs one server session over an already-established stream, described by `connection`
    /// for [`FixpApplication::verify`]. After [shutdown](Self::shutdown) has started, closes it
    /// at once.
    ///
    /// # Errors
    ///
    /// The transport's error, if reading or writing failed, or the client stopped reading.
    pub async fn accept_stream<S>(&self, stream: S, connection: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        self.run_connection(stream, connection).await
    }

    async fn run_connection<S>(&self, stream: S, connection: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (config, server) = (self.config.clone(), self.server.clone());
        let (session, commands) =
            FixpSession::server(config, server, self.registry.clone(), self.app.clone(), Instant::now());
        let session = session.with_connection(connection);
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this server and its clones: stops accepting connections, terminates every
    /// established session, and closes the rest. Returns once every connection has closed, a
    /// client that doesn't answer its `Terminate` being disconnected after the handshake timeout.
    pub async fn shutdown(&self) {
        self.shutdown.run(None, self.config.handshake_timeout).await;
    }
}

/// A [`FixpInitiator`]'s configuration: the client's identity, its session's settings, and where
/// and how it connects, as an [`InitiatorConfig`](crate::InitiatorConfig) is a FIX initiator's.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct FixpInitiatorConfig {
    /// The client's name, its server's, and its credentials.
    pub client: ClientConfig,
    /// Settings for its session.
    pub session: FixpConfig,
    /// Where to connect, in the order tried: the primary, then backups. At least one. All are
    /// assumed to serve the same sessions (a venue's primary and backup gateways), sharing their
    /// state.
    pub endpoints: Vec<Endpoint>,
    /// How long a TCP connect to one endpoint may take before trying the next.
    pub connect_timeout: Duration,
    /// A proxy to reach every endpoint through; see
    /// [`InitiatorConfig::proxy`](crate::InitiatorConfig::proxy).
    pub proxy: Option<crate::Proxy>,
    /// How long [`FixpInitiator::run`] waits before connecting again.
    pub reconnect: ReconnectPolicy,
    /// Connects over TLS, verifying each server's certificate; the handshake must complete within
    /// the session's `handshake_timeout`. `None` (the default) connects in plain TCP.
    #[cfg(feature = "tls")]
    pub tls: Option<crate::InitiatorTls>,
}

impl FixpInitiatorConfig {
    /// A configuration for `client` with the default session settings ([`FixpConfig::new`]), a
    /// 10-second connect timeout, and reconnection after 1 s, backing off to 60 s (see
    /// [`ReconnectPolicy`]). It has no endpoints yet: add them with
    /// [`with_endpoint`](Self::with_endpoint).
    pub fn new(client: ClientConfig) -> Self {
        Self {
            client,
            session: FixpConfig::new(),
            endpoints: Vec::new(),
            connect_timeout: Duration::from_secs(10),
            proxy: None,
            reconnect: ReconnectPolicy::default(),
            #[cfg(feature = "tls")]
            tls: None,
        }
    }

    /// Adds an endpoint, tried after those added before: the first is the primary, the rest
    /// backups.
    #[must_use]
    pub fn with_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.endpoints.push(endpoint.into());
        self
    }

    /// Checks the configuration: the session's ([`FixpConfig::check`]), the reconnect policy's,
    /// at least one endpoint, and TLS server names that are DNS names or IP addresses.
    ///
    /// # Errors
    ///
    /// The first problem found.
    pub fn check(&self) -> Result<(), ConfigError> {
        self.session.check()?;
        self.reconnect.check()?;
        #[cfg(feature = "tls")]
        return endpoints_problem(&self.endpoints, self.tls.as_ref()).map_err(ConfigError::new);
        #[cfg(not(feature = "tls"))]
        return endpoints_problem(&self.endpoints).map_err(ConfigError::new);
    }
}

/// A FIXP client of one server: connects, negotiates or establishes, and reconnects as its
/// policy says, until [shutdown](Self::shutdown). Cheap to clone; clones share the session and
/// shutdown.
#[derive(Clone)]
pub struct FixpInitiator {
    config: Arc<FixpInitiatorConfig>,
    registry: Arc<FixpRegistry>,
    app: Arc<dyn FixpApplication>,
    shutdown: Arc<Shutdown>,
}

impl std::fmt::Debug for FixpInitiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixpInitiator")
            .field("endpoints", &self.config.endpoints)
            .field("session", &self.session_id())
            .finish()
    }
}

impl FixpInitiator {
    /// A client connecting as `config` says, keeping its state in `storage`.
    ///
    /// # Errors
    ///
    /// If `config` is invalid; see [`FixpInitiatorConfig::check`].
    pub fn new(
        config: FixpInitiatorConfig,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn FixpApplication>,
    ) -> Result<Self, ConfigError> {
        config.check()?;
        let registry = Arc::new(FixpRegistry::with_storage(storage).with_clock(config.session.clock.clone()));
        Ok(Self { config: Arc::new(config), registry, app, shutdown: Arc::new(Shutdown::new()) })
    }

    /// The endpoints in the order they're tried.
    #[must_use]
    pub fn endpoints(&self) -> Vec<Endpoint> {
        self.config.endpoints.clone()
    }

    /// The ID of the session's log: `FIXP <name> → <server>`.
    #[must_use]
    pub fn session_id(&self) -> SessionId {
        let client = &self.config.client;
        SessionId::new("FIXP", client.name.clone(), client.server.clone())
    }

    /// A handle to the session, usable whenever it's connected.
    #[must_use]
    pub fn handle(&self) -> FixpHandle {
        self.registry.handle(self.session_id())
    }

    /// Events for this initiator's session from now on: see
    /// [`SessionRegistry::subscribe`](crate::SessionRegistry::subscribe).
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<crate::SessionEvent> {
        self.registry.subscribe()
    }

    /// Keeps the session connected: connects, runs the session until the connection ends, and
    /// waits as the reconnect policy says before the next, until [shutdown](Self::shutdown)
    /// (returning once the session has closed) or the future is dropped.
    pub async fn run(self) {
        let span = tracing::info_span!("fixp initiator", addr = %self.config.endpoints[0].addr);
        async {
            let mut backoff = Backoff::new(self.config.reconnect);
            while !self.shutdown.is_started() {
                let (established, result) = self.connect().await;
                match result {
                    Ok(()) => info!("disconnected"),
                    Err(e) if self.shutdown.is_started() => info!("{e}"),
                    Err(e) => warn!("{e}"),
                }
                let delay = backoff.next_delay(established);
                if !self.shutdown.is_started() {
                    info!(?delay, "reconnecting after a delay");
                }
                let mut signal = self.shutdown.signal();
                tokio::select! {
                    () = tokio::time::sleep(delay) => {}
                    _ = signal.started() => {}
                }
            }
            info!("shut down");
        }
        .instrument(span)
        .await;
    }

    /// Tries the endpoints in order, starting with the primary, until one establishes the session,
    /// then runs it until the connection ends. An endpoint fails over to the next if the TCP
    /// connect fails or times out, the TLS handshake fails, or the connection ends before the
    /// session is established.
    ///
    /// # Errors
    ///
    /// Every endpoint's failure if none established the session, or shutdown has started; or the
    /// transport's error that ended the session.
    pub async fn connect_once(&self) -> io::Result<()> {
        self.connect().await.1
    }

    /// [`connect_once`](Self::connect_once), also saying whether the session was established.
    async fn connect(&self) -> (bool, io::Result<()>) {
        let mut errors = Vec::with_capacity(self.config.endpoints.len());
        for (index, endpoint) in self.config.endpoints.iter().enumerate() {
            if self.shutdown.is_started() {
                return (false, Err(io::Error::new(io::ErrorKind::Interrupted, "shutting down")));
            }
            let role = if index == 0 { "primary" } else { "backup" };
            let span = tracing::info_span!("endpoint", addr = %endpoint.addr, role);
            match self.attempt(endpoint).instrument(span).await {
                (true, result) => return (true, result),
                (false, result) => {
                    let e = result.err().unwrap_or_else(|| {
                        io::Error::new(io::ErrorKind::ConnectionAborted, "the connection ended before establishing")
                    });
                    let next = self.config.endpoints.get(index + 1).map_or("none left", |next| next.addr.as_str());
                    warn!(addr = %endpoint.addr, role, next, "endpoint failed: {e}");
                    errors.push(format!("{}: {e}", endpoint.addr));
                }
            }
        }
        let error =
            io::Error::new(io::ErrorKind::NotConnected, format!("no endpoint available ({})", errors.join("; ")));
        (false, Err(error))
    }

    /// Connects to `addr`, through the proxy if there is one.
    async fn open(&self, addr: &str) -> io::Result<TcpStream> {
        let Some(proxy) = &self.config.proxy else { return TcpStream::connect(addr).await };
        let mut stream = TcpStream::connect(&proxy.addr).await?;
        crate::proxy::tunnel(&mut stream, proxy, addr).await?;
        Ok(stream)
    }

    /// Connects to `endpoint` and runs the session there: whether it was established, and how
    /// the connection ended.
    async fn attempt(&self, endpoint: &Endpoint) -> (bool, io::Result<()>) {
        let _open = self.shutdown.track();
        let stream = match tokio::time::timeout(self.config.connect_timeout, self.open(&endpoint.addr)).await {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => return (false, Err(e)),
            Err(_) => {
                let timeout = self.config.connect_timeout;
                return (
                    false,
                    Err(io::Error::new(io::ErrorKind::TimedOut, format!("connect timed out after {timeout:?}"))),
                );
            }
        };
        if let Err(e) = stream.set_nodelay(true) {
            return (false, Err(e));
        }
        info!("connected");
        #[cfg(feature = "tls")]
        if let Some(crate::InitiatorTls { connector, server_name, .. }) = &self.config.tls {
            let name = endpoint.tls_server_name.as_deref().unwrap_or(server_name);
            let stream = match self.tls_handshake(connector, name, stream).await {
                Ok(stream) => stream,
                Err(e) => return (false, Err(e)),
            };
            return self.run_session(stream).await;
        }
        self.run_session(stream).await
    }

    /// The TLS handshake over `stream`, within the handshake timeout, unless shutdown starts.
    #[cfg(feature = "tls")]
    async fn tls_handshake(
        &self,
        connector: &crate::tls::TlsConnector,
        name: &str,
        stream: TcpStream,
    ) -> io::Result<tokio_rustls::client::TlsStream<TcpStream>> {
        let name = crate::initiator::tls_server_name(name)?;
        let handshake = tokio::time::timeout(self.config.session.handshake_timeout, connector.connect(name, stream));
        let mut shutdown = self.shutdown.signal();
        let handshake = tokio::select! {
            handshake = handshake => handshake,
            _ = shutdown.started() => return Err(crate::acceptor::shutting_down()),
        };
        let stream = handshake
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "TLS handshake timed out"))?
            .map_err(|e| io::Error::new(e.kind(), format!("TLS handshake failed: {e}")))?;
        info!("TLS handshake complete");
        Ok(stream)
    }

    /// Runs a session over `stream` until the connection ends: whether it was established, and how
    /// it ended.
    async fn run_session<S>(&self, stream: S) -> (bool, io::Result<()>)
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (session, commands) = self.session();
        let mut established = false;
        let signal = Some(self.shutdown.signal());
        let result = connection::run_tracked(stream, session, commands, &mut established, signal).await;
        (established, result)
    }

    /// Runs the session over an already-established stream (e.g. from a custom transport).
    ///
    /// # Errors
    ///
    /// The transport's error, if reading or writing failed, or the server stopped reading.
    pub async fn run_stream<S>(&self, stream: S) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        let (session, commands) = self.session();
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this client and its clones: terminates the session, or closes the connection if
    /// it isn't established, and stops reconnecting. Returns once the connection has closed.
    pub async fn shutdown(&self) {
        self.shutdown.run(None, self.config.session.handshake_timeout).await;
    }

    /// A client session for a new connection.
    fn session(&self) -> (FixpSession, crate::registry::CommandReceiver<super::SbeMessage>) {
        let (config, client) = (self.config.session.clone(), self.config.client.clone());
        FixpSession::client(config, client, self.registry.clone(), self.app.clone(), Instant::now())
    }
}
