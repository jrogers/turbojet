//! Connects out to a counterparty and keeps the session logged on, failing over between a
//! primary and backup endpoints.

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, PoisonError, RwLock};
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpSocket, TcpStream};
use tracing::{Instrument, info, warn};

use crate::application::Application;
use crate::connection;
use crate::fields::Secret;
use crate::peer::ConnectionInfo;
use crate::reconnect::{Backoff, ReconnectPolicy};
use crate::registry::{CommandReceiver, SessionHandle, SessionRegistry};
use crate::session::{Session, SessionConfig};
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};

/// How an [`Initiator`] logs on and reconnects: [`new`](Self::new) gives the defaults, and the
/// fields are public to change them.
#[derive(Debug, Clone)]
#[non_exhaustive]
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
    /// The local address to connect from, for a host with several networks or a counterparty
    /// that only accepts known source addresses; port 0 lets the system choose the port. Only
    /// the endpoints' addresses of the same family (IPv4 or IPv6) are tried. `None` (the default)
    /// leaves both to the system.
    pub local_addr: Option<SocketAddr>,
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
            local_addr: None,
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
#[non_exhaustive]
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
    #[must_use]
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
    /// What it connects to and with, shared by clones and replaced whole by
    /// [`reconfigure`](Self::reconfigure); each connection attempt takes the one current then.
    plan: Arc<RwLock<Arc<Plan>>>,
    registry: Arc<SessionRegistry>,
    app: Arc<dyn Application>,
    #[cfg(feature = "tls")]
    tls: Option<(crate::tls::TlsConnector, String)>,
    shutdown: Arc<Shutdown>,
    /// Addresses whose TCP connect never completes, as with a host that silently drops packets,
    /// which a real socket can't reliably be made to do.
    #[cfg(test)]
    unreachable: Vec<String>,
}

impl std::fmt::Debug for Initiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Initiator")
            .field("session", &self.session_id())
            .field("endpoints", &self.endpoints())
            .finish_non_exhaustive()
    }
}

/// An initiator's endpoints and configuration.
struct Plan {
    /// Primary first, then backups in priority order.
    endpoints: Vec<Endpoint>,
    config: InitiatorConfig,
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
            plan: Arc::new(RwLock::new(Arc::new(Plan { endpoints: vec![addr.into()], config }))),
            registry,
            app,
            #[cfg(feature = "tls")]
            tls: None,
            shutdown: Arc::new(Shutdown::new()),
            #[cfg(test)]
            unreachable: Vec::new(),
        }
    }

    /// Adds a backup endpoint, tried after the primary and any earlier backups.
    #[must_use]
    pub fn with_failover(self, endpoint: impl Into<Endpoint>) -> Self {
        let plan = self.plan();
        let mut endpoints = plan.endpoints.clone();
        endpoints.push(endpoint.into());
        self.set_plan(Plan { endpoints, config: plan.config.clone() });
        self
    }

    /// Keeps the session's state in `registry` rather than one of its own, so that
    /// [`SessionHandle`]s from it stay valid if this initiator is replaced by another for the same
    /// session (with different settings, say), and operator changes go through it. The registry's
    /// storage is used in place of the one given to [`new`](Self::new). Set it before running or
    /// cloning.
    #[must_use]
    pub fn with_registry(mut self, registry: Arc<SessionRegistry>) -> Self {
        self.registry = registry;
        self
    }

    /// Connects with `config` and to `endpoints` (primary first) from the next connection
    /// attempt on, here and in clones; a session already connected carries on with the settings
    /// it connected with. The reconnect policy applies from the next wait.
    ///
    /// # Errors
    ///
    /// If `config` is invalid ([`InitiatorConfig::check`]), `endpoints` is empty, a TLS server
    /// name in it is invalid, or `config` names another session (BeginString or either CompID) or
    /// another clock than the one this initiator was made with: those make another initiator.
    pub fn reconfigure(&self, config: InitiatorConfig, endpoints: Vec<Endpoint>) -> Result<(), String> {
        config.check()?;
        if endpoints.is_empty() {
            return Err("an initiator needs an endpoint".into());
        }
        let current = self.plan();
        let (now, then) = (&config.session, &current.config.session);
        let fixed = [
            ("begin_string", now.begin_string == then.begin_string),
            ("sender_comp_id", now.sender_comp_id == then.sender_comp_id),
            ("target_comp_id", config.target_comp_id == current.config.target_comp_id),
            ("clock", now.clock.same_as(&then.clock)),
        ];
        if let Some((field, _)) = fixed.iter().find(|(_, same)| !same) {
            return Err(format!("{field} can't change: that's another initiator"));
        }
        #[cfg(feature = "tls")]
        if self.tls.is_some() {
            for name in endpoints.iter().filter_map(|e| e.tls_server_name.as_deref()) {
                tls_server_name(name).map_err(|e| e.to_string())?;
            }
        }
        self.set_plan(Plan { endpoints, config });
        Ok(())
    }

    /// The endpoints and configuration current now.
    fn plan(&self) -> Arc<Plan> {
        self.plan.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn set_plan(&self, plan: Plan) {
        *self.plan.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(plan);
    }

    /// Connects over TLS, verifying each server's certificate against `server_name` (a DNS name
    /// or IP address) unless the endpoint sets its own.
    #[cfg(feature = "tls")]
    pub fn with_tls(mut self, connector: crate::tls::TlsConnector, server_name: &str) -> io::Result<Self> {
        tls_server_name(server_name)?;
        for endpoint in &self.plan().endpoints {
            if let Some(name) = &endpoint.tls_server_name {
                tls_server_name(name)?;
            }
        }
        self.tls = Some((connector, server_name.to_string()));
        Ok(self)
    }

    /// The endpoints in the order they are tried, as of now.
    pub fn endpoints(&self) -> Vec<Endpoint> {
        self.plan().endpoints.clone()
    }

    /// The session this initiator logs on to: its BeginString, our CompID and the target's.
    pub fn session_id(&self) -> SessionId {
        let config = &self.plan().config;
        SessionId {
            begin_string: config.session.begin_string.clone(),
            sender_comp_id: config.session.sender_comp_id.clone(),
            target_comp_id: config.target_comp_id.clone(),
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
            let mut backoff = Backoff::new(self.plan().config.reconnect);
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
                backoff.set_policy(self.plan().config.reconnect);
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
        // One plan for the whole attempt, even if reconfigured meanwhile.
        let plan = self.plan();
        let mut errors = Vec::with_capacity(plan.endpoints.len());
        for (index, endpoint) in plan.endpoints.iter().enumerate() {
            if self.shutdown.is_started() {
                return (false, Err(shutting_down()));
            }
            let role = if index == 0 { "primary" } else { "backup" };
            let span = tracing::info_span!("endpoint", addr = %endpoint.addr, role);
            match self.attempt(endpoint, &plan.config).instrument(span).await {
                Attempt::Established(result) => return (true, result),
                Attempt::Failed(e) => {
                    let next = plan.endpoints.get(index + 1).map_or("none left", |next| next.addr.as_str());
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
        let (session, commands) = self.session(&self.plan().config, info);
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this initiator and its clones: logs the session out (with `text` as the
    /// Logout's Text), or disconnects it if it hasn't logged on yet, and stops connecting;
    /// [`run`](Initiator::run) returns and [`connect_once`](Initiator::connect_once) fails.
    /// Returns once the connection has closed. A counterparty that doesn't answer the Logout
    /// within the [logout timeout](SessionConfig::logout_timeout) is disconnected, and shortly
    /// after that the connection is closed regardless.
    ///
    /// Then the session's [cancel-on-disconnect](SessionConfig::cancel_on_disconnect) countdown,
    /// if one is under way, fires at once rather than wait out its grace period, even if the
    /// [registry](Self::with_registry) lives on. So replacing an initiator with another for the
    /// same session (on a configuration reload, say) during a grace period cancels early: that
    /// errs on the side of orders cancelled rather than left working. Countdowns of other
    /// initiators sharing the registry carry on. A logout for the shutdown doesn't start one: our
    /// side chose to end the session.
    ///
    /// Shutdown is permanent. Calling it again waits for the same shutdown.
    pub async fn shutdown(&self, text: Option<&str>) {
        self.shutdown.run(text, self.plan().config.session.logout_timeout).await;
        // Every connection has closed, so no more countdowns start here.
        let session = self.session_id();
        self.registry.run_cancels_now(|id| *id == session);
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
        let plan = self.plan();
        let schedule = plan.config.session.schedule.as_ref()?;
        let now = plan.config.session.clock.now();
        let reason = schedule.closed_reason(now)?;
        let wait =
            schedule.next_start(now).and_then(|next| (next - now).to_std().ok()).unwrap_or(Duration::from_secs(60));
        Some((wait, reason))
    }

    async fn attempt(&self, endpoint: &Endpoint, config: &InitiatorConfig) -> Attempt {
        // Counted from here, so shutdown also waits for a TLS handshake; the connect and the
        // handshake are abandoned as soon as it starts.
        let _open = self.shutdown.track();
        let connect = tokio::time::timeout(config.connect_timeout, self.tcp_connect(&endpoint.addr, config.local_addr));
        let connect = match self.unless_shutdown(connect).await {
            Ok(connect) => connect,
            Err(e) => return Attempt::Failed(e),
        };
        let stream = match connect {
            Ok(Ok(stream)) => stream,
            Ok(Err(e)) => return Attempt::Failed(e),
            Err(_) => {
                let timeout = config.connect_timeout;
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
            let handshake = tokio::time::timeout(config.session.logon_timeout, connector.connect(name, stream));
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
            return self.run_session(stream, ConnectionInfo::new(addr, certificates), config).await;
        }
        self.run_session(stream, ConnectionInfo::new(addr, Vec::new()), config).await
    }

    async fn tcp_connect(&self, addr: &str, local: Option<SocketAddr>) -> io::Result<TcpStream> {
        #[cfg(test)]
        if self.unreachable.iter().any(|unreachable| unreachable == addr) {
            return std::future::pending().await;
        }
        let Some(local) = local else { return TcpStream::connect(addr).await };
        let mut failed = None;
        for remote in tokio::net::lookup_host(addr).await?.filter(|remote| remote.is_ipv4() == local.is_ipv4()) {
            let socket = if local.is_ipv4() { TcpSocket::new_v4() } else { TcpSocket::new_v6() }?;
            // A fixed port may still be in TIME_WAIT from the last connection.
            socket.set_reuseaddr(true)?;
            socket.bind(local)?;
            match socket.connect(remote).await {
                Ok(stream) => return Ok(stream),
                Err(e) => failed = Some(e),
            }
        }
        Err(failed.unwrap_or_else(|| {
            let family = if local.is_ipv4() { "IPv4" } else { "IPv6" };
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{addr} has no {family} address to connect to from {local}"),
            )
        }))
    }

    /// A session for a new connection, described by `info`, with the task that fires its
    /// cancel-on-disconnect countdowns running.
    fn session(&self, config: &InitiatorConfig, info: ConnectionInfo) -> (Session, CommandReceiver) {
        self.registry.spawn_cancel_task();
        let (mut session, commands) =
            Session::initiator(config, self.registry.clone(), self.app.clone(), Instant::now());
        session.set_connection_info(info);
        (session, commands)
    }

    async fn run_session<S>(&self, stream: S, info: ConnectionInfo, config: &InitiatorConfig) -> Attempt
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (session, commands) = self.session(config, info);
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

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;
    use crate::store::MemoryStorage;

    struct Nothing;
    impl Application for Nothing {}

    const UNREACHABLE: &str = "192.0.2.1:9876";

    fn initiator(primary: &str, connect_timeout: Duration) -> Initiator {
        initiator_with(primary, |config| config.connect_timeout = connect_timeout)
    }

    /// An initiator of `primary`, its config adjusted by `adjust`.
    fn initiator_with(primary: &str, adjust: impl FnOnce(&mut InitiatorConfig)) -> Initiator {
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
        adjust(&mut config);
        let mut initiator = Initiator::new(primary, config, Arc::new(MemoryStorage::new()), Arc::new(Nothing));
        initiator.unreachable.push(UNREACHABLE.to_string());
        initiator
    }

    #[tokio::test]
    async fn connects_from_the_local_address() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        // A free port: taken, then let go.
        let local = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        let initiator = initiator_with(&addr, |config| config.local_addr = Some(local));
        let connecting = tokio::spawn(async move { initiator.connect_once().await });
        let (stream, peer) = listener.accept().await.unwrap();
        assert_eq!(peer, local);
        drop(stream);
        let _ = connecting.await;
    }

    #[tokio::test]
    async fn a_local_address_of_the_other_family_fails_to_connect() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        let initiator = initiator_with(&addr, |config| config.local_addr = Some("[::1]:0".parse().unwrap()));
        let err = initiator.connect_once().await.unwrap_err().to_string();
        assert!(err.contains("no IPv6 address"), "{err}");
    }

    #[tokio::test]
    async fn a_connect_that_never_completes_times_out_and_fails_over() {
        let backup = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let backup_addr = backup.local_addr().unwrap().to_string();
        // The backup takes the connection and closes it before logon, so the attempt ends there.
        let accepted = tokio::spawn(async move {
            let (_stream, _) = backup.accept().await.unwrap();
            Instant::now()
        });
        let connect_timeout = Duration::from_millis(200);
        let initiator = initiator(UNREACHABLE, connect_timeout).with_failover(backup_addr.as_str());

        let started = Instant::now();
        let err = tokio::time::timeout(Duration::from_secs(5), initiator.connect_once())
            .await
            .expect("the connect times out rather than hanging")
            .unwrap_err();
        let message = err.to_string();
        assert!(message.contains(&format!("{UNREACHABLE}: connect timed out after 200ms")), "{message}");
        assert!(message.contains(&backup_addr), "the backup is tried: {message}");
        let accepted =
            tokio::time::timeout(Duration::from_secs(5), accepted).await.expect("the backup connects").unwrap();
        assert!(accepted - started >= connect_timeout, "the backup is tried only after the timeout");
    }

    #[tokio::test]
    async fn shutdown_abandons_a_connect_that_never_completes() {
        let initiator = Arc::new(initiator(UNREACHABLE, Duration::from_secs(60)));
        let connecting = tokio::spawn({
            let initiator = initiator.clone();
            async move { initiator.connect_once().await }
        });
        tokio::time::sleep(Duration::from_millis(50)).await;
        tokio::time::timeout(Duration::from_secs(5), initiator.shutdown(None)).await.expect("shutdown is prompt");
        let err = tokio::time::timeout(Duration::from_secs(5), connecting)
            .await
            .expect("the connect is abandoned, not waited out")
            .unwrap()
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotConnected, "{err}");
        assert!(err.to_string().contains("shutting down"), "{err}");
    }

    /// A listener that takes each connection and closes it before logon.
    async fn closing_listener() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                drop(stream);
            }
        });
        addr
    }

    #[tokio::test]
    async fn reconfiguring_applies_from_the_next_connection() {
        let (first, second) = (closing_listener().await, closing_listener().await);
        let initiator = initiator(&first, Duration::from_secs(5));
        let clone = initiator.clone();
        let err = initiator.connect_once().await.unwrap_err().to_string();
        assert!(err.contains(&first), "{err}");
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
        config.session.clock = initiator.plan().config.session.clock.clone();
        config.heartbeat_interval = Duration::from_secs(10);
        initiator.reconfigure(config, vec![Endpoint::new(&second)]).unwrap();
        assert_eq!(clone.endpoints(), [Endpoint::new(&second)], "clones share it");
        let err = clone.connect_once().await.unwrap_err().to_string();
        assert!(err.contains(&second) && !err.contains(&first), "{err}");
        assert_eq!(clone.plan().config.heartbeat_interval, Duration::from_secs(10));
    }

    #[test]
    fn reconfiguring_keeps_the_session_and_clock() {
        let initiator = initiator("127.0.0.1:1", Duration::from_secs(5));
        let same = || {
            let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "SERVER");
            config.session.clock = initiator.plan().config.session.clock.clone();
            config
        };
        type Change = fn(&mut InitiatorConfig);
        let changes: [(&str, Change); 4] = [
            ("begin_string", |c| c.session.begin_string = "FIX.4.4".into()),
            ("sender_comp_id", |c| c.session.sender_comp_id = "OTHER".into()),
            ("target_comp_id", |c| c.target_comp_id = "OTHER".into()),
            ("clock", |c| c.session.clock = crate::Clock::system()),
        ];
        for (field, change) in changes {
            let mut config = same();
            change(&mut config);
            let err = initiator.reconfigure(config, vec![Endpoint::new("127.0.0.1:2")]).unwrap_err();
            assert_eq!(err, format!("{field} can't change: that's another initiator"));
        }
        assert!(initiator.reconfigure(same(), Vec::new()).is_err(), "no endpoint");
        let mut invalid = same();
        invalid.heartbeat_interval = Duration::ZERO;
        assert!(initiator.reconfigure(invalid, vec![Endpoint::new("127.0.0.1:2")]).is_err());
        assert_eq!(initiator.endpoints(), [Endpoint::new("127.0.0.1:1")], "unchanged by refusals");
    }

    #[tokio::test]
    async fn a_shared_registry_holds_the_session() {
        let registry = Arc::new(SessionRegistry::new(Arc::new(MemoryStorage::new())));
        let initiator = initiator("127.0.0.1:1", Duration::from_secs(5)).with_registry(registry.clone());
        registry.handle(initiator.session_id()).set_next_outgoing(5).await.unwrap();
        assert_eq!(initiator.handle().sequence_numbers().await.unwrap().next_outgoing, 5);
    }
}
