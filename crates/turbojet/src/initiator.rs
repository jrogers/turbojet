//! Connects out to a counterparty and keeps the session logged on, failing over between a
//! primary and backup endpoints.

use std::future::Future;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tracing::{Instrument, info, warn};

use crate::application::Application;
use crate::connection;
use crate::fields::Secret;
use crate::peer::ConnectionInfo;
use crate::reconnect::{Backoff, ReconnectPolicy};
use crate::registry::{SessionHandle, SessionRegistry};
use crate::session::{Session, SessionConfig};
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};

/// How an [`Initiator`] logs on and reconnects: [`new`](Self::new) gives the defaults, and the
/// fields are public to change them.
#[derive(Debug, Clone)]
pub struct InitiatorConfig {
    /// Settings shared with acceptor sessions, including our own CompID.
    pub session: SessionConfig,
    /// The counterparty's CompID.
    pub target_comp_id: String,
    /// HeartBtInt(108) to request: whole seconds from 1 to 3600, the range an acceptor accepts
    /// (see [`check`](Self::check)).
    pub heartbeat_interval: Duration,
    /// Send ResetSeqNumFlag(141)=Y on every logon, starting both sides from 1.
    pub reset_on_logon: bool,
    /// Send NextExpectedMsgSeqNum(789) in the Logon (FIX 4.4 and later): the counterparty then
    /// resends anything we missed straight after logon, and we resend what it missed, with no
    /// ResendRequest. A counterparty's 789 is honoured whether or not this is set.
    pub next_expected_msg_seq_num: bool,
    /// Username(553) to send in the Logon.
    pub username: Option<String>,
    /// Password(554) to send in the Logon. Shown as `***` by `Debug` and in the message log.
    pub password: Option<Secret>,
    /// How long a TCP connect to one endpoint may take before trying the next.
    pub connect_timeout: Duration,
    /// How long [`Initiator::run`] waits after a session ends, or after every endpoint has
    /// failed, before connecting again.
    pub reconnect: ReconnectPolicy,
}

impl InitiatorConfig {
    /// A configuration for logging on to `target_comp_id`: a 30-second heartbeat, no sequence
    /// reset, NextExpectedMsgSeqNum or credentials on logon, a 10-second connect timeout, and
    /// reconnection after 1 second, backing off to 60 with jitter while attempts fail (see
    /// [`ReconnectPolicy`]).
    pub fn new(session: SessionConfig, target_comp_id: impl Into<String>) -> Self {
        Self {
            session,
            target_comp_id: target_comp_id.into(),
            heartbeat_interval: Duration::from_secs(30),
            reset_on_logon: false,
            next_expected_msg_seq_num: false,
            username: None,
            password: None,
            connect_timeout: Duration::from_secs(10),
            reconnect: ReconnectPolicy::default(),
        }
    }

    /// Checks the configuration: the session's ([`SessionConfig::check`]), the reconnect
    /// policy's ([`ReconnectPolicy::check`]), and a heartbeat interval of whole seconds from 1 to
    /// 3600. HeartBtInt(108) is a whole number of seconds, so anything else would promise the
    /// counterparty a different interval from the one kept.
    pub fn check(&self) -> Result<(), String> {
        self.session.check()?;
        self.reconnect.check()?;
        let interval = self.heartbeat_interval;
        if interval.subsec_nanos() != 0 || !(1..=3600).contains(&interval.as_secs()) {
            return Err(format!("heartbeat_interval must be whole seconds from 1 to 3600, not {interval:?}"));
        }
        Ok(())
    }

    /// Panics with [`check`](Self::check)'s message if this configuration is invalid.
    pub(crate) fn assert_valid(&self) {
        if let Err(e) = self.check() {
            panic!("invalid initiator configuration: {e}");
        }
    }
}

/// An address the initiator can connect to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    /// `host:port`.
    pub addr: String,
    /// Name to verify the server's TLS certificate against, overriding the one given to
    /// `Initiator::with_tls`. Ignored without TLS.
    pub tls_server_name: Option<String>,
}

impl Endpoint {
    /// An endpoint at `addr` (`host:port`).
    pub fn new(addr: impl Into<String>) -> Self {
        Self { addr: addr.into(), tls_server_name: None }
    }

    /// Sets [`tls_server_name`](Self::tls_server_name).
    pub fn with_tls_server_name(mut self, name: impl Into<String>) -> Self {
        self.tls_server_name = Some(name.into());
        self
    }
}

impl From<&str> for Endpoint {
    fn from(addr: &str) -> Self {
        Self::new(addr)
    }
}

impl From<String> for Endpoint {
    fn from(addr: String) -> Self {
        Self::new(addr)
    }
}

impl From<&String> for Endpoint {
    fn from(addr: &String) -> Self {
        Self::new(addr.as_str())
    }
}

/// A FIX initiator for one session, with a primary endpoint and optional backups.
/// Cheap to clone; clones share the session and [shutdown](Initiator::shutdown).
///
/// All endpoints are assumed to serve the same FIX session (e.g. a counterparty's primary and
/// disaster-recovery sites), so sequence numbers carry over when failing over.
#[derive(Clone)]
pub struct Initiator {
    /// Primary first, then backups in priority order.
    endpoints: Vec<Endpoint>,
    config: Arc<InitiatorConfig>,
    registry: Arc<SessionRegistry>,
    app: Arc<dyn Application>,
    #[cfg(feature = "tls")]
    tls: Option<(crate::tls::TlsConnector, String)>,
    shutdown: Arc<Shutdown>,
}

/// How a connection attempt to one endpoint ended.
enum Attempt {
    /// The session logged on; holds how it eventually ended.
    Established(io::Result<()>),
    /// No session was established on this endpoint.
    Failed(io::Error),
}

impl Initiator {
    /// An initiator whose primary endpoint is `addr` (`host:port`).
    ///
    /// # Panics
    ///
    /// If `config` is invalid; see [`InitiatorConfig::check`].
    pub fn new(
        addr: impl Into<Endpoint>,
        config: InitiatorConfig,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn Application>,
    ) -> Self {
        config.assert_valid();
        let registry = Arc::new(SessionRegistry::new(storage).with_clock(config.session.clock.clone()));
        Self {
            endpoints: vec![addr.into()],
            config: Arc::new(config),
            registry,
            app,
            #[cfg(feature = "tls")]
            tls: None,
            shutdown: Arc::new(Shutdown::new()),
        }
    }

    /// Adds a backup endpoint, tried after the primary and any earlier backups.
    pub fn with_failover(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.endpoints.push(endpoint.into());
        self
    }

    /// Connects over TLS, verifying each server's certificate against `server_name` (a DNS name
    /// or IP address) unless the endpoint sets its own.
    #[cfg(feature = "tls")]
    pub fn with_tls(mut self, connector: crate::tls::TlsConnector, server_name: &str) -> io::Result<Self> {
        tls_server_name(server_name)?;
        for endpoint in &self.endpoints {
            if let Some(name) = &endpoint.tls_server_name {
                tls_server_name(name)?;
            }
        }
        self.tls = Some((connector, server_name.to_string()));
        Ok(self)
    }

    /// The endpoints in the order they are tried.
    pub fn endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// The session this initiator logs on to: its BeginString, our CompID and the target's.
    pub fn session_id(&self) -> SessionId {
        SessionId {
            begin_string: self.config.session.begin_string.clone(),
            sender_comp_id: self.config.session.sender_comp_id.clone(),
            target_comp_id: self.config.target_comp_id.clone(),
        }
    }

    /// A handle for sending on the session; valid across reconnects and failovers.
    pub fn handle(&self) -> SessionHandle {
        self.registry.handle(self.session_id())
    }

    /// Keeps the session connected: calls [`Initiator::connect_once`] repeatedly, waiting as the
    /// [reconnect policy](InitiatorConfig::reconnect) says between calls, and outside the session
    /// schedule (if any) waiting for the next period instead. Runs until [shutdown](Initiator::shutdown) (returning once the
    /// session has closed), or until the future is dropped or the task is aborted.
    pub async fn run(self) {
        let span = tracing::info_span!("initiator", session = %self.session_id());
        async {
            let mut waiting = false;
            let mut backoff = Backoff::new(self.config.reconnect);
            while !self.shutdown.is_started() {
                if let Some((wait, reason)) = self.schedule_wait() {
                    if !waiting {
                        info!("{reason}; waiting");
                        waiting = true;
                    }
                    // Re-check at least every second, so clock changes are noticed promptly.
                    self.unless_shutdown(tokio::time::sleep(wait.min(Duration::from_secs(1)))).await.ok();
                    continue;
                }
                waiting = false;
                let (logged_on, result) = self.connect().await;
                match result {
                    Ok(()) => info!("disconnected"),
                    Err(e) if self.shutdown.is_started() => info!("{e}"),
                    Err(e) => warn!("{e}"),
                }
                let delay = backoff.next_delay(logged_on);
                if !self.shutdown.is_started() {
                    info!(?delay, "reconnecting after a delay");
                }
                self.unless_shutdown(tokio::time::sleep(delay)).await.ok();
            }
            info!("shut down");
        }
        .instrument(span)
        .await
    }

    /// Tries the endpoints in order, starting with the primary, until one establishes a session,
    /// then runs that session until it disconnects. An endpoint fails over to the next if the
    /// TCP connect fails or times out, the TLS handshake fails, or the connection ends before
    /// logon. Fails if no endpoint establishes a session.
    ///
    /// Outside the session schedule, or after [shutdown](Initiator::shutdown) has started, fails
    /// immediately without connecting.
    pub async fn connect_once(&self) -> io::Result<()> {
        self.connect().await.1
    }

    /// [`connect_once`](Self::connect_once), also saying whether a session logged on.
    async fn connect(&self) -> (bool, io::Result<()>) {
        if let Some((_, reason)) = self.schedule_wait() {
            return (false, Err(io::Error::new(io::ErrorKind::NotConnected, reason)));
        }
        let mut errors = Vec::with_capacity(self.endpoints.len());
        for (index, endpoint) in self.endpoints.iter().enumerate() {
            if self.shutdown.is_started() {
                return (false, Err(shutting_down()));
            }
            let role = if index == 0 { "primary" } else { "backup" };
            let span = tracing::info_span!("endpoint", addr = %endpoint.addr, role);
            match self.attempt(endpoint).instrument(span).await {
                Attempt::Established(result) => return (true, result),
                Attempt::Failed(e) => {
                    let next = self.endpoints.get(index + 1).map_or("none left", |next| next.addr.as_str());
                    warn!(addr = %endpoint.addr, role, next, "endpoint failed: {e}");
                    errors.push(format!("{}: {e}", endpoint.addr));
                }
            }
        }
        let error =
            io::Error::new(io::ErrorKind::NotConnected, format!("no endpoint available ({})", errors.join("; ")));
        (false, Err(error))
    }

    /// Runs the session over an already-established stream (e.g. one from a custom transport),
    /// described by `info` for [`Application::verify_logon`]. After
    /// [shutdown](Initiator::shutdown) has started, disconnects without logging on.
    pub async fn run_stream<S>(&self, stream: S, info: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        let (mut session, commands) =
            Session::initiator(&self.config, self.registry.clone(), self.app.clone(), Instant::now());
        session.set_connection_info(info);
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this initiator and its clones: logs the session out (with `text` as the
    /// Logout's Text), or disconnects it if it hasn't logged on yet, and stops connecting;
    /// [`run`](Initiator::run) returns and [`connect_once`](Initiator::connect_once) fails.
    /// Returns once the connection has closed. A counterparty that doesn't answer the Logout
    /// within the [logout timeout](SessionConfig::logout_timeout) is disconnected, and shortly
    /// after that the connection is closed regardless.
    ///
    /// Shutdown is permanent. Calling it again waits for the same shutdown.
    pub async fn shutdown(&self, text: Option<&str>) {
        self.shutdown.run(text, self.config.session.logout_timeout).await
    }

    /// Runs `step` unless shutdown starts first, which fails it.
    async fn unless_shutdown<T>(&self, step: impl Future<Output = T>) -> io::Result<T> {
        let mut shutdown = self.shutdown.signal();
        tokio::select! {
            output = step => Ok(output),
            _ = shutdown.started() => Err(shutting_down()),
        }
    }

    /// How long until the schedule allows a session, and why, if it doesn't now.
    fn schedule_wait(&self) -> Option<(Duration, String)> {
        let schedule = self.config.session.schedule.as_ref()?;
        let now = self.config.session.clock.now();
        let reason = schedule.closed_reason(now)?;
        let wait =
            schedule.next_start(now).and_then(|next| (next - now).to_std().ok()).unwrap_or(Duration::from_secs(60));
        Some((wait, reason))
    }

    async fn attempt(&self, endpoint: &Endpoint) -> Attempt {
        // Counted from here, so shutdown also waits for a TLS handshake; the connect and the
        // handshake are abandoned as soon as it starts.
        let _open = self.shutdown.track();
        let connect = tokio::time::timeout(self.config.connect_timeout, TcpStream::connect(&endpoint.addr));
        let connect = match self.unless_shutdown(connect).await {
            Ok(connect) => connect,
            Err(e) => return Attempt::Failed(e),
        };
        let stream = match connect {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => return Attempt::Failed(e),
            Err(_) => {
                let timeout = self.config.connect_timeout;
                return Attempt::Failed(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("connect timed out after {timeout:?}"),
                ));
            }
        };
        if let Err(e) = stream.set_nodelay(true) {
            return Attempt::Failed(e);
        }
        let addr = stream.peer_addr().ok();
        info!("connected");

        #[cfg(feature = "tls")]
        if let Some((connector, default_name)) = &self.tls {
            let name = match tls_server_name(endpoint.tls_server_name.as_deref().unwrap_or(default_name)) {
                Ok(name) => name,
                Err(e) => return Attempt::Failed(e),
            };
            let handshake = tokio::time::timeout(self.config.session.logon_timeout, connector.connect(name, stream));
            let handshake = match self.unless_shutdown(handshake).await {
                Ok(handshake) => handshake,
                Err(e) => return Attempt::Failed(e),
            };
            let stream = match handshake {
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => return Attempt::Failed(io::Error::new(e.kind(), format!("TLS handshake failed: {e}"))),
                Err(_) => return Attempt::Failed(io::Error::new(io::ErrorKind::TimedOut, "TLS handshake timed out")),
            };
            info!("TLS handshake complete");
            let certificates = crate::tls::peer_certificates(stream.get_ref().1.peer_certificates());
            return self.run_session(stream, ConnectionInfo::new(addr, certificates)).await;
        }
        self.run_session(stream, ConnectionInfo::new(addr, Vec::new())).await
    }

    async fn run_session<S>(&self, stream: S, info: ConnectionInfo) -> Attempt
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (mut session, commands) =
            Session::initiator(&self.config, self.registry.clone(), self.app.clone(), Instant::now());
        session.set_connection_info(info);
        let mut logged_on = false;
        let signal = Some(self.shutdown.signal());
        let result = connection::run_tracked(stream, session, commands, &mut logged_on, signal).await;
        match (logged_on, result) {
            (true, result) => Attempt::Established(result),
            (false, Err(e)) => Attempt::Failed(e),
            (false, Ok(())) => {
                Attempt::Failed(io::Error::new(io::ErrorKind::ConnectionAborted, "disconnected before logon"))
            }
        }
    }
}

fn shutting_down() -> io::Error {
    io::Error::new(io::ErrorKind::Interrupted, "shutting down")
}

#[cfg(feature = "tls")]
fn tls_server_name(name: &str) -> io::Result<crate::tls::ServerName<'static>> {
    crate::tls::ServerName::try_from(name.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("invalid TLS server name '{name}': {e}")))
}
