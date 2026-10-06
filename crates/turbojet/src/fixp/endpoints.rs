//! FIXP over TCP: a server accepting clients, and a client connecting and reconnecting.

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tracing::{Instrument, info, warn};

use super::{FixpApplication, FixpConfig, FixpHandle, FixpRegistry, FixpSession, Role};
use crate::acceptor::{DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_CONNECTIONS_PER_IP, Limits, accept_loop};
use crate::connection;
use crate::reconnect::{Backoff, ReconnectPolicy};
use crate::session::ConfigError;
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};

/// A FIXP server: accepts connections and runs a server session on each, for any client that
/// negotiates or establishes. Cheap to clone; clones share sessions, storage and
/// [shutdown](Self::shutdown).
///
/// Any client that can reach the listener may negotiate unless [`FixpApplication::verify`]
/// refuses it, and each session gets its own stored state. It keeps at most
/// [`DEFAULT_MAX_CONNECTIONS`] connections open at once, and at most
/// [`DEFAULT_MAX_CONNECTIONS_PER_IP`] from one IP address, as the FIX
/// [`Acceptor`](crate::Acceptor) does.
#[derive(Clone)]
pub struct FixpAcceptor {
    config: FixpConfig,
    registry: Arc<FixpRegistry>,
    app: Arc<dyn FixpApplication>,
    shutdown: Arc<Shutdown>,
    limits: Arc<Limits>,
}

impl std::fmt::Debug for FixpAcceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixpAcceptor").field("role", &self.config.role).finish_non_exhaustive()
    }
}

impl FixpAcceptor {
    /// A server serving `app`, keeping session state in `storage`.
    ///
    /// # Errors
    ///
    /// If `config` is invalid (see [`FixpConfig::check`]) or isn't a server's.
    pub fn new(
        config: FixpConfig,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn FixpApplication>,
    ) -> Result<Self, ConfigError> {
        config.check()?;
        if !matches!(config.role, Role::Server(_)) {
            return Err(ConfigError::from("a FixpAcceptor needs a server's configuration".to_string()));
        }
        let registry = Arc::new(FixpRegistry::with_storage(storage).with_clock(config.clock.clone()));
        let limits = Arc::new(Limits::new(DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_CONNECTIONS_PER_IP));
        Ok(Self { config, registry, app, shutdown: Arc::new(Shutdown::new()), limits })
    }

    /// Keeps at most `connections` connections open at once, and at most `per_ip` from one IP
    /// address; see [`Acceptor::with_max_connections`](crate::Acceptor::with_max_connections).
    #[must_use]
    pub fn with_max_connections(mut self, connections: usize, per_ip: usize) -> Self {
        self.limits = Arc::new(Limits::new(connections, per_ip));
        self
    }

    /// A handle to session `id` (as [`sessions`](Self::sessions) lists them), usable whenever it's
    /// connected.
    #[must_use]
    pub fn session(&self, id: SessionId) -> FixpHandle {
        self.registry.handle(id)
    }

    /// Sessions currently connected.
    #[must_use]
    pub fn sessions(&self) -> Vec<SessionId> {
        self.registry.sessions()
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
            async move { acceptor.run_connection(stream).await }
        })
        .await
    }

    /// Runs one server session over an already-established stream. After
    /// [shutdown](Self::shutdown) has started, closes it at once.
    ///
    /// # Errors
    ///
    /// The transport's error, if reading or writing failed, or the client stopped reading.
    pub async fn accept_stream<S>(&self, stream: S) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        self.run_connection(stream).await
    }

    async fn run_connection<S>(&self, stream: S) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (session, commands) =
            FixpSession::new(self.config.clone(), self.registry.clone(), self.app.clone(), Instant::now());
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this server and its clones: stops accepting connections, terminates every
    /// established session, and closes the rest. Returns once every connection has closed, a
    /// client that doesn't answer its `Terminate` being disconnected after the handshake timeout.
    pub async fn shutdown(&self) {
        self.shutdown.run(None, self.config.handshake_timeout).await;
    }
}

/// A FIXP client of one server: connects, negotiates or establishes, and reconnects as its
/// policy says, until [shutdown](Self::shutdown). Cheap to clone; clones share the session and
/// shutdown.
#[derive(Clone)]
pub struct FixpInitiator {
    addr: String,
    config: FixpConfig,
    reconnect: ReconnectPolicy,
    connect_timeout: Duration,
    registry: Arc<FixpRegistry>,
    app: Arc<dyn FixpApplication>,
    shutdown: Arc<Shutdown>,
}

impl std::fmt::Debug for FixpInitiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixpInitiator").field("addr", &self.addr).field("session", &self.session_id()).finish()
    }
}

impl FixpInitiator {
    /// A client of the server at `addr` (`host:port`), keeping its state in `storage`, which
    /// reconnects after 1 s and then backs off to 60 s (see [`ReconnectPolicy`]), and gives a
    /// TCP connect 10 s.
    ///
    /// # Errors
    ///
    /// If `config` is invalid (see [`FixpConfig::check`]) or isn't a client's.
    pub fn new(
        addr: impl Into<String>,
        config: FixpConfig,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn FixpApplication>,
    ) -> Result<Self, ConfigError> {
        config.check()?;
        if !matches!(config.role, Role::Client(_)) {
            return Err(ConfigError::from("a FixpInitiator needs a client's configuration".to_string()));
        }
        let registry = Arc::new(FixpRegistry::with_storage(storage).with_clock(config.clock.clone()));
        Ok(Self {
            addr: addr.into(),
            config,
            reconnect: ReconnectPolicy::default(),
            connect_timeout: Duration::from_secs(10),
            registry,
            app,
            shutdown: Arc::new(Shutdown::new()),
        })
    }

    /// Reconnects as `policy` says.
    ///
    /// # Errors
    ///
    /// If `policy` is invalid; see [`ReconnectPolicy::check`].
    pub fn with_reconnect(mut self, policy: ReconnectPolicy) -> Result<Self, ConfigError> {
        policy.check()?;
        self.reconnect = policy;
        Ok(self)
    }

    /// The ID of the session's log: `FIXP <name> → <server>`.
    #[must_use]
    pub fn session_id(&self) -> SessionId {
        let Role::Client(client) = &self.config.role else { unreachable!("checked by new") };
        SessionId::new("FIXP", client.name.clone(), client.server.clone())
    }

    /// A handle to the session, usable whenever it's connected.
    #[must_use]
    pub fn handle(&self) -> FixpHandle {
        self.registry.handle(self.session_id())
    }

    /// Keeps the session connected: connects, runs the session until the connection ends, and
    /// waits as the reconnect policy says before the next, until [shutdown](Self::shutdown)
    /// (returning once the session has closed) or the future is dropped.
    pub async fn run(self) {
        let span = tracing::info_span!("fixp initiator", addr = %self.addr);
        async {
            let mut backoff = Backoff::new(self.reconnect);
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

    /// Connects once and runs the session until the connection ends.
    ///
    /// # Errors
    ///
    /// If the connect fails or times out, or shutdown has started; or the transport's error that
    /// ended the session.
    pub async fn connect_once(&self) -> io::Result<()> {
        self.connect().await.1
    }

    /// [`connect_once`](Self::connect_once), also saying whether the session was established.
    async fn connect(&self) -> (bool, io::Result<()>) {
        if self.shutdown.is_started() {
            return (false, Err(io::Error::new(io::ErrorKind::Interrupted, "shutting down")));
        }
        let _open = self.shutdown.track();
        let stream = match tokio::time::timeout(self.connect_timeout, TcpStream::connect(&self.addr)).await {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => return (false, Err(e)),
            Err(_) => {
                let timeout = self.connect_timeout;
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
        let (session, commands) =
            FixpSession::new(self.config.clone(), self.registry.clone(), self.app.clone(), Instant::now());
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
        let (session, commands) =
            FixpSession::new(self.config.clone(), self.registry.clone(), self.app.clone(), Instant::now());
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this client and its clones: terminates the session, or closes the connection if
    /// it isn't established, and stops reconnecting. Returns once the connection has closed.
    pub async fn shutdown(&self) {
        self.shutdown.run(None, self.config.handshake_timeout).await;
    }
}
