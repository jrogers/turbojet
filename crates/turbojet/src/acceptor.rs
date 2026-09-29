//! Accepts inbound connections, each of which may log on as any permitted counterparty.

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tracing::{Instrument, info, warn};

use crate::application::Application;
use crate::connection;
use crate::peer::ConnectionInfo;
use crate::registry::{SessionHandle, SessionRegistry};
use crate::session::{Session, SessionConfig};
use crate::shutdown::Shutdown;
use crate::store::{SessionId, SessionStorage};

/// A FIX acceptor. Cheap to clone; clones share sessions, storage and [shutdown](Acceptor::shutdown).
///
/// Any counterparty that can reach the listener may log on under any SenderCompID unless
/// [`Application::verify_logon`] refuses it, and each CompID accepted gets its own stored state
/// (files, with [`DiskStorage`](crate::DiskStorage)) and metric series. Check the CompID, and
/// the client certificate if using mutual TLS, in `verify_logon`.
#[derive(Clone)]
pub struct Acceptor {
    config: SessionConfig,
    registry: Arc<SessionRegistry>,
    app: Arc<dyn Application>,
    shutdown: Arc<Shutdown>,
}

impl Acceptor {
    /// An acceptor serving `app`, keeping session state in `storage`.
    ///
    /// # Panics
    ///
    /// If `config` is invalid; see [`SessionConfig::check`].
    pub fn new(config: SessionConfig, storage: Arc<dyn SessionStorage>, app: Arc<dyn Application>) -> Self {
        config.assert_valid();
        let registry = Arc::new(SessionRegistry::new(storage).with_clock(config.clock.clone()));
        Self { config, registry, app, shutdown: Arc::new(Shutdown::new()) }
    }

    /// A handle to the session with counterparty `target_comp_id`, usable whenever it is
    /// connected.
    pub fn session(&self, target_comp_id: &str) -> SessionHandle {
        self.registry.handle(SessionId {
            begin_string: self.config.begin_string.clone(),
            sender_comp_id: self.config.sender_comp_id.clone(),
            target_comp_id: target_comp_id.into(),
        })
    }

    /// Sessions currently connected.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.registry.sessions()
    }

    /// Accepts plain TCP connections, running each on its own task, until
    /// [shutdown](Acceptor::shutdown) starts or the future is dropped; then returns `Ok`, closing
    /// the listener. Accept errors are logged and retried (after a short pause, unless they
    /// concern a single connection), so it doesn't return otherwise.
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
    async fn serve_with<L, F, Fut>(self, mut listener: L, handle: F) -> io::Result<()>
    where
        L: Listen,
        F: Fn(Acceptor, TcpStream) -> Fut,
        Fut: Future<Output = io::Result<()>> + Send + 'static,
    {
        let mut shutdown = self.shutdown.signal();
        loop {
            let accepted = tokio::select! {
                biased;
                _ = shutdown.started() => {
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
            if let Err(e) = stream.set_nodelay(true) {
                warn!(%addr, "cannot set TCP_NODELAY: {e}");
            }
            let connection = handle(self.clone(), stream);
            let span = tracing::info_span!("conn", %addr);
            // Counted from here, so shutdown also waits for TLS handshakes.
            let open = self.shutdown.track();
            tokio::spawn(
                async move {
                    let _open = open;
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

    /// Runs one acceptor session over an already-established stream (e.g. from a custom
    /// transport), described by `info` for [`Application::verify_logon`]. After
    /// [shutdown](Acceptor::shutdown) has started, disconnects without waiting for a Logon.
    pub async fn accept_stream<S>(&self, stream: S, info: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let _open = self.shutdown.track();
        self.run_connection(stream, info).await
    }

    /// [`accept_stream`](Acceptor::accept_stream) for a connection shutdown already counts.
    async fn run_connection<S>(&self, stream: S, info: ConnectionInfo) -> io::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (mut session, commands) =
            Session::acceptor(self.config.clone(), self.registry.clone(), self.app.clone(), Instant::now());
        session.set_connection_info(info);
        connection::run_tracked(stream, session, commands, &mut false, Some(self.shutdown.signal())).await
    }

    /// Shuts down this acceptor and its clones: stops accepting connections, logs every
    /// logged-on session out (with `text` as the Logout's Text), and disconnects connections that
    /// haven't logged on. Returns once every connection has closed. A counterparty that doesn't
    /// answer the Logout within the [logout timeout](SessionConfig::logout_timeout) is
    /// disconnected, and shortly after that any connection still open (one blocked writing to a
    /// counterparty that has stopped reading, say) is closed regardless.
    ///
    /// Shutdown is permanent: [`serve`](Acceptor::serve) returns, and connections made later are
    /// closed without logging on. Calling it again waits for the same shutdown.
    pub async fn shutdown(&self, text: Option<&str>) {
        self.shutdown.run(text, self.config.logout_timeout).await
    }
}

#[cfg(feature = "tls")]
fn shutting_down() -> io::Error {
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
trait Listen {
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
            Acceptor::new(SessionConfig::new("FIX.4.2", "US"), Arc::new(MemoryStorage::new()), Arc::new(Nothing));
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
}
