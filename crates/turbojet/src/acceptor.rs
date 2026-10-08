//! Accepts inbound connections, each of which may log on as any permitted counterparty.

use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tracing::{Instrument, info, warn};

use crate::application::Application;
use crate::connection;
use crate::counterparty::Counterparties;
use crate::peer::ConnectionInfo;
use crate::registry::{CommandReceiver, SessionEvent, SessionHandle, SessionRegistry, SessionStatus};
use crate::session::{ConfigError, Session, SessionConfig};
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};
use crate::telemetry;

/// The most connections an [`Acceptor`] keeps open at once by default: well above the
/// counterparties one usually serves, while bounding the tasks a flood of connections can start.
pub const DEFAULT_MAX_CONNECTIONS: usize = 1024;

/// The most connections from one IP address an [`Acceptor`] keeps open at once by default: room
/// for a counterparty's sessions and reconnects, while one address can't take every place.
pub const DEFAULT_MAX_CONNECTIONS_PER_IP: usize = 16;

/// How often a refused connection is reported in the log, at most: a flood shouldn't flood it.
const REFUSALS_REPORTED_EVERY: Duration = Duration::from_secs(60);

/// A FIX acceptor. Cheap to clone; clones share sessions, storage and [shutdown](Acceptor::shutdown).
///
/// Any counterparty that can reach the listener may log on under any SenderCompID unless
/// [`Application::verify_logon`] refuses it, and each CompID accepted gets its own stored state
/// (files, with [`DiskStorage`](crate::DiskStorage)) and metric series. Check the CompID, and
/// the client certificate if using mutual TLS, in `verify_logon`.
///
/// It keeps at most [`DEFAULT_MAX_CONNECTIONS`] connections open at once, and at most
/// [`DEFAULT_MAX_CONNECTIONS_PER_IP`] from one IP address, closing any past either as soon as it's
/// accepted (see [`with_max_connections`](Acceptor::with_max_connections)).
#[derive(Clone)]
pub struct Acceptor {
    config: SessionConfig,
    registry: Arc<SessionRegistry>,
    app: Arc<dyn Application>,
    shutdown: Arc<Shutdown>,
    limits: Arc<Limits>,
    counterparties: Option<Arc<dyn Counterparties>>,
}

impl std::fmt::Debug for Acceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Acceptor")
            .field("begin_string", &self.config.begin_string)
            .field("sender_comp_id", &self.config.sender_comp_id)
            .finish_non_exhaustive()
    }
}

impl Acceptor {
    /// An acceptor serving `app`, keeping session state in `storage`.
    ///
    /// # Errors
    ///
    /// If `config` is invalid; see [`SessionConfig::check`].
    pub fn new(
        config: SessionConfig,
        storage: Arc<dyn SessionStorage>,
        app: Arc<dyn Application>,
    ) -> Result<Self, ConfigError> {
        config.check()?;
        let registry = Arc::new(SessionRegistry::new(storage).with_clock(config.clock.clone()));
        let limits = Arc::new(Limits::new(DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_CONNECTIONS_PER_IP));
        Ok(Self { config, registry, app, shutdown: Arc::new(Shutdown::new()), limits, counterparties: None })
    }

    /// Keeps at most `connections` connections open at once, closing any more as soon as they're
    /// accepted, before a TLS handshake or a task. Every connection counts, from accept until it
    /// closes, logged on or not. Set it before serving or cloning: clones share the limits.
    #[must_use]
    pub fn with_max_connections(mut self, connections: usize) -> Self {
        self.limits = Arc::new(Limits::new(connections, self.limits.max_per_ip));
        self
    }

    /// Keeps at most `connections` connections from one IP address open at once, as
    /// [`with_max_connections`](Acceptor::with_max_connections) does overall. Counterparties
    /// behind one address (a NAT, or a hub serving several firms) share it.
    #[must_use]
    pub fn with_max_connections_per_ip(mut self, connections: usize) -> Self {
        self.limits = Arc::new(Limits::new(self.limits.max_total, connections));
        self
    }

    /// Gives each counterparty the settings `counterparties` resolves at its Logon, in place of
    /// this acceptor's configuration, or refuses it; see [`Counterparties`]. Set it before
    /// serving or cloning: connections accepted by a clone made earlier don't use it.
    ///
    /// To give counterparties different stores, use a store that routes by CompID, such as
    /// [`StorageByCounterparty`](crate::store::StorageByCounterparty): a store is opened for
    /// operator changes to disconnected sessions too, when there's no Logon to resolve.
    #[must_use]
    pub fn with_counterparties(mut self, counterparties: Arc<dyn Counterparties>) -> Self {
        self.counterparties = Some(counterparties);
        self
    }

    /// A handle to the session with counterparty `target_comp_id` and no SubIDs, LocationIDs or
    /// qualifier, usable whenever it is connected. For one with them, see
    /// [`handle`](Self::handle).
    pub fn session(&self, target_comp_id: &str) -> SessionHandle {
        self.registry.handle(SessionId::new(
            self.config.begin_string.clone(),
            self.config.sender_comp_id.clone(),
            target_comp_id,
        ))
    }

    /// A handle to session `id`, usable whenever it is connected: one from
    /// [`sessions`](Self::sessions), say, or one whose counterparty logs on with a SubID.
    pub fn handle(&self, id: &SessionId) -> SessionHandle {
        self.registry.handle(id.clone())
    }

    /// Sessions currently connected.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.registry.sessions()
    }

    /// Events for this acceptor's sessions from now on: see [`SessionRegistry::subscribe`].
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<SessionEvent> {
        self.registry.subscribe()
    }

    /// Sessions an operator has paused: see [`SessionHandle::pause`].
    pub fn paused(&self) -> Vec<SessionId> {
        self.registry.paused()
    }

    /// Sessions currently connected, each with when it logged on and over what connection: see
    /// [`SessionRegistry::statuses`].
    pub fn statuses(&self) -> Vec<SessionStatus> {
        self.registry.statuses()
    }

    /// Accepts plain TCP connections, running each on its own task, until
    /// [shutdown](Acceptor::shutdown) starts or the future is dropped; then returns `Ok`, closing
    /// the listener. Accept errors are logged and retried (after a short pause, unless they
    /// concern a single connection), so it doesn't return otherwise.
    ///
    /// # Errors
    ///
    /// None in practice: accept errors are logged and retried, and a connection's own errors end
    /// only that connection.
    pub async fn serve(self, listener: TcpListener) -> io::Result<()> {
        self.serve_listener(listener).await
    }

    async fn serve_listener(self, listener: impl Listen) -> io::Result<()> {
        self.serve_with(listener, |acceptor, stream| async move {
            let connection = ConnectionInfo::new(stream.peer_addr().ok(), Vec::new());
            acceptor.run_connection(stream, connection).await
        })
        .await
    }

    /// Like [`serve`](Acceptor::serve), for TLS connections. Each handshake runs on the
    /// connection's own task and must complete within the logon timeout.
    ///
    /// # Errors
    ///
    /// As [`serve`](Acceptor::serve); a failed or slow handshake ends only its connection.
    #[cfg(feature = "tls")]
    pub async fn serve_tls(self, listener: TcpListener, tls: crate::tls::TlsAcceptor) -> io::Result<()> {
        self.serve_with(listener, move |acceptor, stream| {
            let tls = tls.clone();
            async move {
                let addr = stream.peer_addr().ok();
                let handshake = tokio::time::timeout(acceptor.config.logon_timeout, tls.accept(stream));
                let mut shutdown = acceptor.shutdown.signal();
                let stream = tokio::select! {
                    handshake = handshake => handshake,
                    _ = shutdown.started() => return Err(shutting_down()),
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

    /// Accept loop: spawns `handle` on its own task for each connection.
    async fn serve_with<L, F, Fut>(self, listener: L, handle: F) -> io::Result<()>
    where
        L: Listen,
        F: Fn(Acceptor, TcpStream) -> Fut,
        Fut: Future<Output = io::Result<()>> + Send + 'static,
    {
        let (shutdown, limits) = (self.shutdown.clone(), self.limits.clone());
        accept_loop(listener, &shutdown, &limits, |stream| handle(self.clone(), stream)).await
    }

    /// Runs one acceptor session over an already-established stream (e.g. from a custom
    /// transport), described by `info` for [`Application::verify_logon`]. After
    /// [shutdown](Acceptor::shutdown) has started, disconnects without waiting for a Logon.
    ///
    /// # Errors
    ///
    /// The transport's error, if reading or writing failed, or the counterparty stopped reading.
    pub async fn accept_stream<S>(&self, stream: S, info: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        self.run_connection(stream, info).await
    }

    /// [`accept_stream`](Acceptor::accept_stream) on the calling thread, which it keeps busy
    /// polling `stream` until the connection ends: see [`connection::run_spinning`], which says
    /// what `stream` must be, and on which store jobs and cancel-on-disconnect run on `runtime`.
    ///
    /// # Errors
    ///
    /// As for [`accept_stream`](Acceptor::accept_stream).
    pub fn accept_spinning<S>(&self, stream: S, info: ConnectionInfo, runtime: &Handle) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _runtime = runtime.enter();
        let _open = self.shutdown.track();
        let (session, commands) = self.new_session(info);
        connection::run_spinning_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal()))
    }

    /// [`accept_stream`](Acceptor::accept_stream) for a connection shutdown already counts.
    async fn run_connection<S>(&self, stream: S, info: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (session, commands) = self.new_session(info);
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// A session for a connection described by `info`.
    fn new_session(&self, info: ConnectionInfo) -> (Session, CommandReceiver) {
        self.registry.spawn_cancel_task();
        let (mut session, commands) =
            Session::acceptor(self.config.clone(), self.registry.clone(), self.app.clone(), Instant::now());
        session.set_connection_info(info);
        if let Some(counterparties) = &self.counterparties {
            session.set_counterparties(counterparties.clone());
        }
        (session, commands)
    }

    /// Shuts down this acceptor and its clones: stops accepting connections, logs every
    /// logged-on session out (with `text` as the Logout's Text), and disconnects connections that
    /// haven't logged on. Returns once every connection has closed. A counterparty that doesn't
    /// answer the Logout within the [logout timeout](SessionConfig::logout_timeout) is
    /// disconnected, and shortly after that any connection still open (one blocked writing to a
    /// counterparty that has stopped reading, say) is closed regardless.
    ///
    /// Then the [cancel-on-disconnect](SessionConfig::cancel_on_disconnect) countdowns still
    /// under way for this acceptor's sessions fire at once, rather than wait out their grace
    /// periods: that errs on the side of orders cancelled rather than left working. Those of other
    /// sessions in a shared registry carry on. Sessions it logs out don't start one: our side
    /// chose to end them.
    ///
    /// Shutdown is permanent: [`serve`](Acceptor::serve) returns, and connections made later are
    /// closed without logging on. Calling it again waits for the same shutdown.
    pub async fn shutdown(&self, text: Option<&str>) {
        self.shutdown.run(text, self.config.logout_timeout).await;
        // Every connection has closed, so no more countdowns start here. An acceptor's registry
        // is its own, never shared, so today this matches every countdown in it. Matching on the
        // session is still the right filter: it names what's ours whatever else comes to share
        // the registry, and every session this acceptor runs has this BeginString and
        // SenderCompID, which a counterparty's own settings can't change (see `Counterparties`).
        let (begin_string, sender_comp_id) = (&self.config.begin_string, &self.config.sender_comp_id);
        self.registry.run_cancels_now(|id| id.begin_string == *begin_string && id.sender_comp_id == *sender_comp_id);
    }
}

/// Accepts connections from `listener` until `shutdown` starts, running `handle` for each on a
/// task of its own, within `limits`: what an [`Acceptor`] and a FIXP acceptor share. Accept errors
/// are logged and retried (after a short pause, unless they concern a single connection).
pub(crate) async fn accept_loop<L, F, Fut>(
    mut listener: L,
    shutdown: &Arc<Shutdown>,
    limits: &Arc<Limits>,
    handle: F,
) -> io::Result<()>
where
    L: Listen,
    F: Fn(TcpStream) -> Fut,
    Fut: Future<Output = io::Result<()>> + Send + 'static,
{
    let mut signal = shutdown.signal();
    // One task per connection, at most `limits.max_total` of them; each session bounds how long
    // it waits for the counterparty's first message.
    loop {
        let accepted = tokio::select! {
            biased;
            _ = signal.started() => {
                info!("shutting down; no longer accepting connections");
                return Ok(());
            }
            accepted = listener.accept() => accepted,
        };
        let (stream, addr) = match accepted {
            Ok(accepted) => accepted,
            // One connection failed before it was accepted (e.g. the client reset): carry on.
            Err(e) if is_connection_error(&e) => {
                warn!("failed to accept a connection: {e}");
                continue;
            }
            // Others, such as running out of file descriptors, may persist for a while: pause
            // rather than spin, then try again. Existing connections are unaffected.
            Err(e) => {
                warn!("accept failed, retrying in {ACCEPT_RETRY_DELAY:?}: {e}");
                tokio::time::sleep(ACCEPT_RETRY_DELAY).await;
                continue;
            }
        };
        // Past a limit, closed at once: no handshake, no task.
        let Some(place) = limits.admit(addr.ip()) else { continue };
        if let Err(e) = stream.set_nodelay(true) {
            warn!(%addr, "cannot set TCP_NODELAY: {e}");
        }
        let connection = handle(stream);
        let span = tracing::info_span!("conn", %addr);
        // Counted from here, so shutdown also waits for TLS handshakes.
        let open = shutdown.track();
        tokio::spawn(
            async move {
                let _open = open;
                let _place = place;
                info!("connection accepted");
                match connection.await {
                    Ok(()) => info!("connection closed"),
                    Err(e) => warn!("connection closed with error: {e}"),
                }
            }
            .instrument(span),
        );
    }
}

/// How many connections an acceptor has open, overall and by IP address, against its limits.
pub(crate) struct Limits {
    max_total: usize,
    max_per_ip: usize,
    open: Mutex<Open>,
}

#[derive(Default)]
struct Open {
    total: usize,
    /// Only addresses with a connection open: an entry goes when its count reaches zero, so the
    /// map holds at most `max_total` entries.
    per_ip: HashMap<IpAddr, usize>,
    /// Refusals since the last reported, and when that was.
    refused: u64,
    reported: Option<Instant>,
}

impl Limits {
    pub(crate) fn new(max_total: usize, max_per_ip: usize) -> Self {
        Self { max_total, max_per_ip, open: Mutex::default() }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Open> {
        self.open.lock().expect("connection limits lock poisoned")
    }

    /// A place for a connection from `ip`, given back when it's dropped; or `None`, counted and
    /// (now and then) logged, if either limit is reached.
    fn admit(self: &Arc<Self>, ip: IpAddr) -> Option<Place> {
        let mut open = self.lock();
        let from_ip = open.per_ip.get(&ip).copied().unwrap_or(0);
        let reason = if open.total >= self.max_total {
            "total"
        } else if from_ip >= self.max_per_ip {
            "per_ip"
        } else {
            open.total += 1;
            *open.per_ip.entry(ip).or_default() += 1;
            debug_assert!(open.total <= self.max_total && open.per_ip.len() <= open.total);
            return Some(Place { limits: self.clone(), ip });
        };
        telemetry::connection_refused(reason);
        open.refused += 1;
        if open.reported.is_none_or(|at| at.elapsed() >= REFUSALS_REPORTED_EVERY) {
            let busiest = open.per_ip.iter().max_by_key(|(_, n)| **n).map(|(ip, n)| format!("{ip} ({n})"));
            warn!(
                refused = open.refused,
                open = open.total,
                max = self.max_total,
                max_per_ip = self.max_per_ip,
                busiest = busiest.as_deref().unwrap_or("none"),
                last = %ip,
                "refusing connections past the acceptor's limit ({reason})"
            );
            open.refused = 0;
            open.reported = Some(Instant::now());
        }
        None
    }

    /// Connections open, and addresses they're from: for tests.
    #[cfg(test)]
    fn open(&self) -> (usize, usize) {
        let open = self.lock();
        (open.total, open.per_ip.len())
    }
}

/// One open connection's place in its acceptor's limits.
struct Place {
    limits: Arc<Limits>,
    ip: IpAddr,
}

impl Drop for Place {
    fn drop(&mut self) {
        let mut open = self.limits.lock();
        open.total -= 1;
        let from_ip = open.per_ip.get_mut(&self.ip).expect("a place is counted");
        *from_ip -= 1;
        if *from_ip == 0 {
            open.per_ip.remove(&self.ip);
        }
    }
}

#[cfg(feature = "tls")]
pub(crate) fn shutting_down() -> io::Error {
    io::Error::new(io::ErrorKind::Interrupted, "shutting down")
}

/// How long to wait before accepting again after an error that isn't about one connection.
const ACCEPT_RETRY_DELAY: Duration = Duration::from_millis(100);

fn is_connection_error(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::ConnectionAborted | io::ErrorKind::ConnectionReset | io::ErrorKind::ConnectionRefused
    )
}

/// Where accepted connections come from: a [`TcpListener`], or a stand-in in tests.
pub(crate) trait Listen {
    fn accept(&mut self) -> impl Future<Output = io::Result<(TcpStream, SocketAddr)>> + Send;
}

impl Listen for TcpListener {
    fn accept(&mut self) -> impl Future<Output = io::Result<(TcpStream, SocketAddr)>> + Send {
        TcpListener::accept(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;
    use crate::codec::{Decoded, decode, encode};
    use crate::fields::MsgType;
    use crate::message::{Message, tags, utc_timestamp};
    use crate::store::MemoryStorage;

    /// Fails with each of `errors` in turn, then accepts from `inner`.
    struct FlakyListener {
        errors: VecDeque<io::Error>,
        inner: TcpListener,
    }

    impl Listen for FlakyListener {
        fn accept(&mut self) -> impl Future<Output = io::Result<(TcpStream, SocketAddr)>> + Send {
            let error = self.errors.pop_front();
            let inner = &self.inner;
            async move {
                match error {
                    Some(e) => Err(e),
                    None => inner.accept().await,
                }
            }
        }
    }

    struct Nothing;
    impl Application for Nothing {}

    #[tokio::test]
    async fn accept_errors_do_not_stop_the_acceptor() {
        let inner = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = inner.local_addr().unwrap();
        let errors = VecDeque::from([
            io::Error::from(io::ErrorKind::ConnectionAborted), // a client reset before accept
            io::Error::from_raw_os_error(24),                  // EMFILE: out of file descriptors
        ]);
        let acceptor =
            Acceptor::new(SessionConfig::new("FIX.4.2", "US"), Arc::new(MemoryStorage::new()), Arc::new(Nothing))
                .unwrap();
        let serving = tokio::spawn(acceptor.serve_listener(FlakyListener { errors, inner }));

        let mut client = TcpStream::connect(addr).await.unwrap();
        let logon = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::MSG_TYPE, MsgType::Logon)
            .with(tags::SENDER_COMP_ID, "PEER")
            .with(tags::TARGET_COMP_ID, "US")
            .with(tags::MSG_SEQ_NUM, 1u64)
            .with(tags::SENDING_TIME, utc_timestamp())
            .with(tags::ENCRYPT_METHOD, "0")
            .with(tags::HEART_BT_INT, 30u64);
        client.write_all(&encode(&logon).unwrap()).await.unwrap();

        let mut buf = Vec::new();
        let reply = loop {
            if let Decoded::Message(msg, _) = decode(&buf) {
                break msg;
            }
            let read = tokio::time::timeout(Duration::from_secs(5), client.read_buf(&mut buf)).await;
            assert!(read.expect("no Logon reply").unwrap() > 0, "connection closed; serve returned {serving:?}");
        };
        assert_eq!(reply.msg_type(), MsgType::Logon);
        assert!(!serving.is_finished(), "still serving");
    }
    /// Serves `acceptor` on a free local port, returning its address.
    async fn serving(acceptor: Acceptor) -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(acceptor.serve(listener));
        addr
    }

    fn quiet_acceptor() -> Acceptor {
        Acceptor::new(SessionConfig::new("FIX.4.2", "US"), Arc::new(MemoryStorage::new()), Arc::new(Nothing)).unwrap()
    }

    /// Whether the acceptor closed `client` at once: a read returns end-of-file rather than
    /// waiting (a connection it kept waits silently for a Logon).
    async fn refused(client: &mut TcpStream) -> bool {
        let mut buf = [0; 16];
        match tokio::time::timeout(Duration::from_millis(300), client.read(&mut buf)).await {
            Ok(Ok(0) | Err(_)) => true,
            Ok(Ok(_)) => panic!("the acceptor sent something before a Logon"),
            Err(_) => false,
        }
    }

    #[tokio::test]
    async fn connections_past_the_limit_are_closed_at_once() {
        let acceptor = quiet_acceptor().with_max_connections(2).with_max_connections_per_ip(10);
        let limits = acceptor.limits.clone();
        let addr = serving(acceptor).await;
        let mut first = TcpStream::connect(addr).await.unwrap();
        let mut second = TcpStream::connect(addr).await.unwrap();
        let mut third = TcpStream::connect(addr).await.unwrap();
        assert!(refused(&mut third).await, "a third is past the limit");
        assert!(!refused(&mut first).await && !refused(&mut second).await, "the first two wait for a Logon");

        // Closing one gives its place back.
        drop(first);
        tokio::time::sleep(Duration::from_millis(100)).await;
        let mut again = TcpStream::connect(addr).await.unwrap();
        assert!(!refused(&mut again).await);
        drop((second, again));
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(limits.open(), (0, 0), "every place is given back");
    }

    #[tokio::test]
    async fn connections_past_the_per_ip_limit_are_closed_at_once() {
        let addr = serving(quiet_acceptor().with_max_connections_per_ip(1)).await;
        let mut first = TcpStream::connect(addr).await.unwrap();
        let mut second = TcpStream::connect(addr).await.unwrap();
        assert!(refused(&mut second).await, "one from 127.0.0.1 at a time");
        assert!(!refused(&mut first).await);
    }

    #[test]
    fn the_limits_are_bounded_by_default() {
        let acceptor =
            Acceptor::new(SessionConfig::new("FIX.4.2", "US"), Arc::new(MemoryStorage::new()), Arc::new(Nothing))
                .unwrap();
        assert_eq!(
            (acceptor.limits.max_total, acceptor.limits.max_per_ip),
            (DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_CONNECTIONS_PER_IP)
        );
    }
}
