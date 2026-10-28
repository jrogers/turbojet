//! Links over TCP, or TLS: the standby's listener, and the active's [`TcpLink`] to it.
//!
//! The active's link calls block: they're made in its store's jobs, which its sessions' drivers
//! run on blocking threads. The link does its I/O on a runtime of its own, on one thread, so a
//! caller can wait for it from any thread, a runtime's own included. The standby serves each
//! connection on a thread of its own, since it applies batches with its store's blocking calls.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, OnceLock, PoisonError};
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio::sync::{mpsc as async_mpsc, oneshot};
use tokio::task::AbortHandle;
use tracing::{debug, info, warn};
use turbojet::SessionId;
use turbojet::store::{Commit, SessionStorage};

use crate::active::{Link, deposed, lag_refused};
use crate::frame::{Batch, Frame, NackReason, Summary, VERSION};
use crate::standby::Replica;
use crate::sync::{AtomicBool, AtomicU64, Mutex, Ordering, mpsc};

/// How long the active waits for the standby's answer before giving the connection up: its
/// sessions' commits wait as long. Connecting, and the TLS handshake, are given as long too.
pub const DEFAULT_ANSWER_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the standby gives a connection's TLS handshake.
#[cfg(feature = "tls")]
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);

/// Serves the active instances that connect to `listener`, applying their batches to `replica`,
/// each connection on a thread of its own. Returns only if accepting fails.
///
/// # Errors
///
/// Accepting a connection failed.
pub async fn serve_standby<S>(listener: TcpListener, replica: Arc<std::sync::Mutex<Replica<S>>>) -> io::Result<()>
where
    S: SessionStorage + Send + 'static,
{
    loop {
        let (stream, peer) = listener.accept().await?;
        serve(stream, peer, replica.clone(), |stream| async { Ok(stream) });
    }
}

/// As [`serve_standby`], over TLS: from [`ServerTls::acceptor`](turbojet::tls::ServerTls::acceptor),
/// say, which can require the active to present a certificate. Each handshake must complete
/// within five seconds.
///
/// # Errors
///
/// Accepting a connection failed; a failed or slow handshake ends only its connection.
#[cfg(feature = "tls")]
pub async fn serve_standby_tls<S>(
    listener: TcpListener,
    tls: turbojet::tls::TlsAcceptor,
    replica: Arc<std::sync::Mutex<Replica<S>>>,
) -> io::Result<()>
where
    S: SessionStorage + Send + 'static,
{
    loop {
        let (stream, peer) = listener.accept().await?;
        let tls = tls.clone();
        serve(stream, peer, replica.clone(), move |stream| async move {
            let accepted = tokio::time::timeout(HANDSHAKE_TIMEOUT, tls.accept(stream)).await;
            let stream = accepted.unwrap_or_else(|_| Err(io::Error::new(io::ErrorKind::TimedOut, "timed out")));
            stream.inspect_err(|e| warn!(%peer, error = %e, "replication TLS handshake failed"))
        });
    }
}

/// Serves one connection, made ready by `upgrade` (a TLS handshake, say), on a thread of its own
/// with a runtime of its own: the thread that applies a batch is the one its arrival wakes.
fn serve<S, F, Fut, T>(stream: TcpStream, peer: SocketAddr, replica: Arc<std::sync::Mutex<Replica<S>>>, upgrade: F)
where
    S: SessionStorage + Send + 'static,
    F: FnOnce(TcpStream) -> Fut + Send + 'static,
    Fut: Future<Output = io::Result<T>>,
    T: AsyncRead + AsyncWrite + Unpin,
{
    let serving = move || -> io::Result<()> {
        let stream = stream.into_std()?;
        stream.set_nodelay(true)?;
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        runtime.block_on(async {
            let stream = upgrade(TcpStream::from_std(stream)?).await?;
            serve_connection(stream, &replica).await
        })
    };
    let spawned = std::thread::Builder::new().name("turbojet-replica-standby".into()).spawn(move || {
        if let Err(e) = serving() {
            debug!(%peer, error = %e, "replication connection ended");
        }
    });
    if let Err(e) = spawned {
        warn!(%peer, error = %e, "no thread to serve a replication connection: dropping it");
    }
}

/// One active's connection, until it ends: each frame applied, and its answer written back.
async fn serve_connection<S, T>(stream: T, replica: &std::sync::Mutex<Replica<S>>) -> io::Result<()>
where
    S: SessionStorage,
    T: AsyncRead + AsyncWrite + Unpin,
{
    // Read and written in turn, so one stream serves both.
    let mut frames = FrameReader::new(stream);
    let mut connection = None;
    let mut out = Vec::new();
    let result = loop {
        let frame = match frames.next().await {
            Ok(Some(frame)) => frame,
            Ok(None) => break Ok(()),
            Err(e) => break Err(e),
        };
        let answer = {
            let mut replica = replica.lock().unwrap_or_else(PoisonError::into_inner);
            let before = replica.connection();
            let answer = replica.on_frame(frame);
            if replica.connection() != before {
                connection = Some(replica.connection());
            }
            answer
        };
        match answer {
            Ok(Some(answer)) => {
                out.clear();
                answer.encode(&mut out);
                if let Err(e) = frames.inner.write_all(&out).await {
                    break Err(e);
                }
            }
            Ok(None) => {}
            Err(e) => break Err(e),
        }
    };
    if let Some(connection) = connection {
        replica.lock().unwrap_or_else(PoisonError::into_inner).disconnected_from(connection);
    }
    result
}

/// Frames as they arrive on a stream.
struct FrameReader<R> {
    inner: R,
    buf: Vec<u8>,
}

impl<R: AsyncRead + Unpin> FrameReader<R> {
    fn new(inner: R) -> Self {
        Self { inner, buf: Vec::new() }
    }

    /// The next frame, or `None` once the stream has ended between frames.
    async fn next(&mut self) -> io::Result<Option<Frame>> {
        loop {
            match Frame::decode(&self.buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? {
                Some((frame, used)) => {
                    self.buf.drain(..used);
                    return Ok(Some(frame));
                }
                None => {
                    self.buf.reserve(64 * 1024);
                    if self.inner.read_buf(&mut self.buf).await? == 0 {
                        return if self.buf.is_empty() { Ok(None) } else { Err(io::ErrorKind::UnexpectedEof.into()) };
                    }
                }
            }
        }
    }
}

/// The active's [`Link`] to a standby over TCP, or TLS ([`with_tls`](Self::with_tls)). It
/// connects when first used, and again after losing the connection; a session bound before it
/// was lost can't commit until it's opened again, which binds it afresh and sends the standby
/// whatever it lacks.
#[derive(Clone)]
pub struct TcpLink {
    shared: Arc<Shared>,
}

/// What a link is made with, kept so a builder method can make it afresh.
#[derive(Clone)]
struct Settings {
    addr: String,
    epoch: u64,
    node: String,
    timeout: Duration,
    #[cfg(feature = "tls")]
    tls: Option<turbojet::InitiatorTls>,
}

struct Shared {
    settings: Settings,
    /// The link's runtime, started at the first connection, and its stop: dropped with the link,
    /// which ends the runtime's thread.
    io: OnceLock<(Handle, oneshot::Sender<()>)>,
    conn: Mutex<Option<Arc<Conn>>>,
    connections: AtomicU64,
    /// The connection each handle was bound on.
    bound: Mutex<HashMap<u32, u64>>,
}

/// One connection to the standby.
struct Conn {
    number: u64,
    /// Frames for the task writing them, so a request goes out at once without its caller
    /// writing to the socket.
    outbox: Mutex<Option<async_mpsc::UnboundedSender<Vec<u8>>>>,
    /// The tasks reading and writing, stopped when the connection is lost.
    tasks: Mutex<Vec<AbortHandle>>,
    /// Who waits for each handle's answers, oldest first: the standby answers a connection's
    /// frames in order, so a handle's answers come in the order of its requests.
    waiting: Mutex<HashMap<u32, VecDeque<mpsc::SyncSender<io::Result<Frame>>>>>,
    alive: AtomicBool,
}

impl std::fmt::Debug for TcpLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = &self.shared.settings;
        f.debug_struct("TcpLink").field("addr", &settings.addr).field("epoch", &settings.epoch).finish_non_exhaustive()
    }
}

impl TcpLink {
    /// A link to the standby at `addr`, as the active `node` holding `epoch`. It connects when
    /// first used.
    pub fn new(addr: impl Into<String>, epoch: u64, node: impl Into<String>) -> Self {
        Self::from(Settings {
            addr: addr.into(),
            epoch,
            node: node.into(),
            timeout: DEFAULT_ANSWER_TIMEOUT,
            #[cfg(feature = "tls")]
            tls: None,
        })
    }

    /// Waits `timeout` for each of the standby's answers, rather than [`DEFAULT_ANSWER_TIMEOUT`].
    #[must_use]
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self::from(Settings { timeout, ..self.shared.settings.clone() })
    }

    /// Connects over TLS, verifying the standby's certificate against `tls.server_name` and, if
    /// its connector has an identity, presenting that.
    #[cfg(feature = "tls")]
    #[must_use]
    pub fn with_tls(self, tls: turbojet::InitiatorTls) -> Self {
        Self::from(Settings { tls: Some(tls), ..self.shared.settings.clone() })
    }

    fn from(settings: Settings) -> Self {
        Self {
            shared: Arc::new(Shared {
                settings,
                io: OnceLock::new(),
                conn: Mutex::new(None),
                connections: AtomicU64::new(0),
                bound: Mutex::new(HashMap::new()),
            }),
        }
    }
}

impl Shared {
    /// The live connection, connecting first if there's none.
    fn conn(&self) -> io::Result<Arc<Conn>> {
        let mut conn = self.conn.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(live) = conn.as_ref().filter(|c| c.alive.load(Ordering::Acquire)) {
            return Ok(live.clone());
        }
        let live = self.connect()?;
        *conn = Some(live.clone());
        Ok(live)
    }

    /// The link's runtime, started on a thread of its own if it isn't yet. Called only with the
    /// connection locked, so it's started once.
    fn io(&self) -> io::Result<&Handle> {
        if let Some((handle, _)) = self.io.get() {
            return Ok(handle);
        }
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        let handle = runtime.handle().clone();
        let (stop, stopped) = oneshot::channel::<()>();
        std::thread::Builder::new().name("turbojet-replica-link".into()).spawn(move || {
            runtime.block_on(async {
                let _ = stopped.await;
            });
        })?;
        Ok(&self.io.get_or_init(|| (handle, stop)).0)
    }

    /// Connects on the link's runtime, waiting here until it has.
    fn connect(&self) -> io::Result<Arc<Conn>> {
        let number = self.connections.load(Ordering::Relaxed) + 1;
        let settings = self.settings.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        self.io()?.spawn(async move {
            let connected = tokio::time::timeout(settings.timeout, open(&settings, number)).await;
            let _ = tx.send(connected.unwrap_or_else(|_| {
                Err(io::Error::new(io::ErrorKind::TimedOut, "the standby didn't answer the connection in time"))
            }));
        });
        let conn = rx.recv().map_err(|_| io::Error::other("the replication link's runtime stopped"))??;
        self.connections.store(number, Ordering::Relaxed);
        info!(standby = %self.settings.addr, epoch = self.settings.epoch, "connected to the standby");
        Ok(conn)
    }

    /// Sends `frame` for `handle` on `conn`, at once, returning where its answer will come.
    fn start(conn: &Arc<Conn>, handle: u32, frame: &Frame) -> io::Result<Answer> {
        let (tx, rx) = mpsc::sync_channel(1);
        conn.waiting.lock().unwrap_or_else(PoisonError::into_inner).entry(handle).or_default().push_back(tx);
        let mut out = Vec::new();
        frame.encode(&mut out);
        let queued = conn.outbox.lock().unwrap_or_else(PoisonError::into_inner).as_ref().map(|outbox| outbox.send(out));
        if !matches!(queued, Some(Ok(()))) {
            conn.lost();
            return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "lost the standby"));
        }
        Ok(Answer { conn: conn.clone(), rx })
    }

    /// Sends `frame` for `handle` on `conn` and waits for the standby's answer.
    fn request(&self, conn: &Arc<Conn>, handle: u32, frame: &Frame) -> io::Result<Frame> {
        Self::start(conn, handle, frame)?.wait(&self.settings.addr, self.settings.timeout)
    }
}

/// Connects to the standby, over TLS if the settings say so, as connection `number`.
async fn open(settings: &Settings, number: u64) -> io::Result<Arc<Conn>> {
    let stream = TcpStream::connect(&settings.addr).await?;
    stream.set_nodelay(true)?;
    #[cfg(feature = "tls")]
    if let Some(tls) = &settings.tls {
        let name = turbojet::tls::ServerName::try_from(tls.server_name.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let stream = tls.connector.connect(name, stream).await?;
        return greet(settings, stream, number).await;
    }
    greet(settings, stream, number).await
}

/// Says hello on `stream` and, welcomed, starts its connection's tasks.
async fn greet<T>(settings: &Settings, stream: T, number: u64) -> io::Result<Arc<Conn>>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
{
    let (read, mut writer) = tokio::io::split(stream);
    let mut frames = FrameReader::new(read);
    let mut hello = Vec::new();
    Frame::Hello { version: VERSION, epoch: settings.epoch, node: settings.node.clone(), app_record_kinds: Vec::new() }
        .encode(&mut hello);
    writer.write_all(&hello).await?;
    match frames.next().await? {
        Some(Frame::Welcome { epoch }) if epoch == settings.epoch => {}
        Some(Frame::Welcome { epoch }) => return Err(deposed(settings.epoch, epoch)),
        other => return Err(io::Error::other(format!("the standby answered hello with {other:?}"))),
    }
    let (outbox, mut frames_out) = async_mpsc::unbounded_channel::<Vec<u8>>();
    let conn = Arc::new(Conn::new(number, outbox));
    let reading = conn.clone();
    let read = tokio::spawn(async move { reading.read(frames).await });
    let writing = conn.clone();
    let write = tokio::spawn(async move {
        while let Some(frame) = frames_out.recv().await {
            if writer.write_all(&frame).await.is_err() {
                writing.lost();
                break;
            }
        }
    });
    conn.tasks.lock().unwrap_or_else(PoisonError::into_inner).extend([read.abort_handle(), write.abort_handle()]);
    Ok(conn)
}

/// A request sent, waiting for its answer.
struct Answer {
    conn: Arc<Conn>,
    rx: mpsc::Receiver<io::Result<Frame>>,
}

impl Answer {
    fn wait(self, standby: &str, timeout: Duration) -> io::Result<Frame> {
        match self.rx.recv_timeout(timeout) {
            Ok(answer) => answer,
            Err(_) => {
                warn!(%standby, "the standby didn't answer in time: dropping the connection");
                self.conn.lost();
                Err(io::Error::new(io::ErrorKind::TimedOut, "the standby didn't answer in time"))
            }
        }
    }
}

impl Conn {
    /// A connection whose frames go to `outbox`, its tasks yet to be started.
    fn new(number: u64, outbox: async_mpsc::UnboundedSender<Vec<u8>>) -> Self {
        Self {
            number,
            outbox: Mutex::new(Some(outbox)),
            tasks: Mutex::new(Vec::new()),
            waiting: Mutex::new(HashMap::new()),
            alive: AtomicBool::new(true),
        }
    }

    /// Reads the standby's answers until the connection ends, handing each to who waits for it.
    async fn read<R: AsyncRead + Unpin>(&self, mut frames: FrameReader<R>) {
        while let Ok(Some(frame)) = frames.next().await {
            if !self.answered(frame) {
                break;
            }
        }
        self.lost();
    }

    /// Hands the standby's answer to the oldest request waiting for it on its handle: false if
    /// `frame` isn't an answer, which ends the connection.
    fn answered(&self, frame: Frame) -> bool {
        let (Frame::Bound { handle, .. } | Frame::Ack { handle, .. } | Frame::Nack { handle, .. }) = &frame else {
            warn!(?frame, "the standby sent a frame it shouldn't");
            return false;
        };
        let waiter = match self.waiting.lock().unwrap_or_else(PoisonError::into_inner).entry(*handle) {
            Entry::Occupied(mut queue) => {
                let waiter = queue.get_mut().pop_front();
                if queue.get().is_empty() {
                    queue.remove();
                }
                waiter
            }
            Entry::Vacant(_) => None,
        };
        match waiter {
            Some(waiter) => {
                let _ = waiter.send(Ok(frame));
            }
            None => warn!(?frame, "the standby answered a request nobody waits for"),
        }
        true
    }

    /// The connection is lost: its tasks are stopped, which closes it, and everyone waiting is
    /// told so.
    fn lost(&self) {
        self.alive.store(false, Ordering::Release);
        self.outbox.lock().unwrap_or_else(PoisonError::into_inner).take();
        for task in self.tasks.lock().unwrap_or_else(PoisonError::into_inner).drain(..) {
            task.abort();
        }
        for waiter in self.waiting.lock().unwrap_or_else(PoisonError::into_inner).drain().flat_map(|(_, queue)| queue) {
            let _ = waiter.send(Err(io::Error::new(io::ErrorKind::ConnectionAborted, "lost the standby")));
        }
    }
}

impl Link for TcpLink {
    fn bind(&self, handle: u32, epoch: u64, max_lag: u64, session: &SessionId) -> io::Result<Summary> {
        let shared = &self.shared;
        let conn = shared.conn()?;
        match shared.request(&conn, handle, &Frame::Bind { handle, epoch, max_lag, session: session.clone() })? {
            Frame::Bound { summary, .. } => {
                shared.bound.lock().unwrap_or_else(PoisonError::into_inner).insert(handle, conn.number);
                Ok(summary)
            }
            Frame::Nack { reason: NackReason::StaleEpoch, .. } => Err(deposed(epoch, epoch + 1)),
            Frame::Nack { reason: NackReason::Lag, .. } => Err(lag_refused()),
            other => Err(io::Error::other(format!("the standby answered a bind with {other:?}"))),
        }
    }

    /// Sends the batch at once, so the standby commits it while the active commits its own copy;
    /// the job waits for the answer. Never connects: a session bound on a connection since lost
    /// must open again.
    fn send(&self, batch: Batch) -> Commit {
        let shared = &self.shared;
        let (handle, epoch) = (batch.handle, batch.epoch);
        let live = shared
            .conn
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
            .filter(|c| c.alive.load(Ordering::Acquire));
        let bound_on = shared.bound.lock().unwrap_or_else(PoisonError::into_inner).get(&handle).copied();
        let started = match live {
            Some(conn) if bound_on == Some(conn.number) => Shared::start(&conn, handle, &Frame::Batch(batch)),
            _ => Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "the connection to the standby was lost since the session opened: it must open again",
            )),
        };
        let (standby, timeout) = (shared.settings.addr.clone(), shared.settings.timeout);
        Commit::blocking(move || match started?.wait(&standby, timeout)? {
            Frame::Ack { .. } => Ok(()),
            Frame::Nack { reason: NackReason::StaleEpoch, .. } => Err(deposed(epoch, epoch + 1)),
            Frame::Nack { reason: NackReason::Diverged, .. } => {
                Err(io::Error::other("the standby's copy of the session has diverged"))
            }
            other => Err(io::Error::other(format!("the standby answered a batch with {other:?}"))),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::net::TcpStream;
    use std::sync::Mutex;

    use turbojet::MemoryStorage;
    use turbojet::store::{StoredSession, conformance};

    use super::*;
    use crate::ReplicatedStorage;
    use crate::frame::{Op, Part};
    use crate::standby::summary;

    type Standby = (SocketAddr, Arc<Mutex<Replica<Arc<MemoryStorage>>>>, Arc<MemoryStorage>);

    /// A standby on a port of its own, serving from a thread.
    fn standby() -> Standby {
        standby_accepting(0)
    }

    /// A standby as [`standby`], that accepts actives leaving up to `max_lag` unacknowledged.
    fn standby_accepting(max_lag: u64) -> Standby {
        let store = Arc::new(MemoryStorage::new());
        let replica = Arc::new(Mutex::new(Replica::new(store.clone()).accepting_lag(max_lag)));
        (serving(replica.clone()), replica, store)
    }

    /// Serves `replica` on a port of its own, from a thread with a runtime of its own.
    fn serving<S: SessionStorage + Send + 'static>(replica: Arc<Mutex<Replica<S>>>) -> SocketAddr {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        listener.set_nonblocking(true).unwrap();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            runtime.block_on(async { serve_standby(TcpListener::from_std(listener).unwrap(), replica).await })
        });
        addr
    }

    fn active(to: SocketAddr, epoch: u64) -> ReplicatedStorage<MemoryStorage> {
        ReplicatedStorage::new(MemoryStorage::new(), Arc::new(TcpLink::new(to.to_string(), epoch, "active")), epoch)
    }

    #[test]
    fn conforms_over_tcp() {
        let (addr, ..) = standby();
        conformance::check_blocking(&active(addr, 1));
        let (addr, ..) = standby();
        conformance::check_sessions_blocking(&active(addr, 1));
    }

    #[test]
    fn async_conforms_over_tcp() {
        let (addr, ..) = standby_accepting(8);
        conformance::check_blocking(&active(addr, 1).asynchronous(8));
        let (addr, ..) = standby_accepting(8);
        conformance::check_sessions_blocking(&active(addr, 1).asynchronous(8));
    }

    /// Over TCP a batch goes out as it's committed, though no commit has waited for its answer:
    /// the batches of a log dropped with answers unwaited for still reach the standby, or the
    /// session's next opening sends it whole. Either way the copies then match. Whether the
    /// standby had them all by the drop is a race, so it isn't asserted.
    #[test]
    fn async_batches_reach_the_standby_over_tcp() {
        let (addr, _replica, standby) = standby_accepting(8);
        let active = active(addr, 1).asynchronous(8);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        for seq in 1..=20 {
            log.record_outgoing(seq, Some(&conformance::app_message(seq))).unwrap();
            if let Some(job) = log.commit().unwrap() {
                job.run().unwrap();
            }
        }
        drop(log);

        let mut log = active.open(&id).unwrap();
        let mine = summary(log.as_mut(), 0).unwrap();
        let messages = log.sent_messages(1, u64::MAX).unwrap();
        drop(log);
        let mut copy = standby.open(&id).unwrap();
        assert_eq!(summary(copy.as_mut(), 0).unwrap(), mine);
        assert_eq!(copy.sent_messages(1, u64::MAX).unwrap(), messages);
    }

    /// As over the in-process link: the standby's copy, taken over at a session's second opening
    /// and every one after, holds what was committed.
    #[test]
    fn the_standby_holds_what_was_committed_over_tcp() {
        struct FailingOver {
            active: ReplicatedStorage<MemoryStorage>,
            standby: Arc<MemoryStorage>,
            opened: Mutex<HashSet<SessionId>>,
        }

        impl SessionStorage for FailingOver {
            fn open(&self, id: &SessionId) -> io::Result<Box<dyn StoredSession>> {
                if self.opened.lock().unwrap().insert(id.clone()) {
                    self.active.open(id)
                } else {
                    self.standby.open(id)
                }
            }

            fn is_durable(&self) -> bool {
                false
            }
        }

        let (addr, _replica, standby) = standby();
        let storage = FailingOver { active: active(addr, 1), standby, opened: Mutex::default() };
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(conformance::check_model(&storage, 0..20));
    }

    /// A TCP proxy between the two that the test can cut.
    struct Cuttable {
        addr: SocketAddr,
        streams: Arc<Mutex<Vec<TcpStream>>>,
    }

    impl Cuttable {
        fn to(upstream: SocketAddr) -> Self {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let streams: Arc<Mutex<Vec<TcpStream>>> = Arc::default();
            let kept = streams.clone();
            std::thread::spawn(move || {
                for client in listener.incoming() {
                    let client = client.unwrap();
                    let server = TcpStream::connect(upstream).unwrap();
                    kept.lock().unwrap().extend([client.try_clone().unwrap(), server.try_clone().unwrap()]);
                    for (mut from, mut to) in
                        [(client.try_clone().unwrap(), server.try_clone().unwrap()), (server, client)]
                    {
                        std::thread::spawn(move || {
                            let _ = io::copy(&mut from, &mut to);
                            let _ = to.shutdown(std::net::Shutdown::Both);
                        });
                    }
                }
            });
            Self { addr, streams }
        }

        fn cut(&self) {
            for stream in self.streams.lock().unwrap().drain(..) {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }
    }

    /// Losing the standby fails the commits of the sessions bound before; opening one again
    /// connects afresh and sends the standby what it missed.
    #[test]
    fn a_session_opens_again_after_the_connection_is_lost() {
        let (addr, _replica, standby) = standby();
        let proxy = Cuttable::to(addr);
        let active = active(proxy.addr, 1);
        let id = conformance::id("A");
        let mut log = active.open(&id).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        log.commit().unwrap().unwrap().run().unwrap();

        proxy.cut();
        log.record_outgoing(2, Some(&conformance::app_message(2))).unwrap();
        assert!(log.commit().unwrap().unwrap().run().is_err(), "the standby can't be reached");
        drop(log);

        let log = active.open(&id).unwrap();
        assert_eq!(log.next_outgoing(), 3);
        drop(log);
        let mut copy = standby.open(&id).unwrap();
        assert_eq!(copy.next_outgoing(), 3, "the standby was sent what it missed");
        assert_eq!(copy.sent_messages(1, 2).unwrap().len(), 2);
    }

    /// Promoted, a standby closes the logs it held, so its own store opens them (a disk store
    /// locks a session while it's open), and refuses the old active.
    #[test]
    fn a_promoted_standby_releases_its_store_and_refuses_the_old_active() {
        let dir = tempfile::tempdir().unwrap();
        let store = Arc::new(turbojet::DiskStorage::new(dir.path()).unwrap().with_fsync(false));
        let replica = Arc::new(Mutex::new(Replica::new(store.clone())));
        let addr = serving(replica.clone());

        let id = conformance::id("A");
        let old = active(addr, 1);
        let mut log = old.open(&id).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        log.commit().unwrap().unwrap().run().unwrap();
        assert!(store.open(&id).is_err(), "the standby holds the session open");

        replica.lock().unwrap().promote().unwrap();
        let taken = store.open(&id).expect("released");
        assert_eq!(taken.next_outgoing(), 2, "with what the old active committed");
        log.record_outgoing(2, Some(&conformance::app_message(2))).unwrap();
        assert!(log.commit().unwrap().unwrap().run().is_err(), "the old active is refused");
    }

    /// Batch `commit_no` for `handle`, recording outgoing message `commit_no`.
    fn numbered(handle: u32, commit_no: u64) -> Batch {
        let ops = vec![Op::Record { seq: commit_no, msg: Some(conformance::app_message(commit_no)) }];
        Batch { handle, epoch: 1, commit_no, part: Part::Commit, ops, app_records: Vec::new() }
    }

    /// A session's batches sent before any is answered are each answered, in order.
    #[test]
    fn a_link_carries_several_of_a_sessions_batches_at_once() {
        let (addr, _replica, standby) = standby();
        let link = TcpLink::new(addr.to_string(), 1, "active");
        let id = conformance::id("A");
        link.bind(1, 1, 0, &id).unwrap();
        let jobs: Vec<_> = (1..=3).map(|n| link.send(numbered(1, n))).collect();
        for job in jobs {
            job.run().unwrap();
        }
        let mut copy = standby.open(&id).unwrap();
        assert_eq!(copy.next_outgoing(), 4);
        assert_eq!(copy.sent_messages(1, 3).unwrap().len(), 3);
    }

    /// Losing the standby fails every batch still under way, not just the latest.
    #[test]
    fn losing_the_standby_fails_every_batch_under_way() {
        let (addr, replica, _) = standby();
        let proxy = Cuttable::to(addr);
        let link = TcpLink::new(proxy.addr.to_string(), 1, "active");
        link.bind(1, 1, 0, &conformance::id("A")).unwrap();
        // The standby answers nothing while its replica is held, so neither batch is answered.
        let held = replica.lock().unwrap();
        let jobs: Vec<_> = (1..=2).map(|n| link.send(numbered(1, n))).collect();
        proxy.cut();
        drop(held);
        for job in jobs {
            let failed = job.run().expect_err("lost");
            assert_eq!(failed.kind(), io::ErrorKind::ConnectionAborted, "{failed}");
        }
    }

    /// An active whose epoch the standby has seen pass can't connect.
    #[test]
    fn a_deposed_active_cant_connect() {
        let (addr, ..) = standby();
        let id = conformance::id("A");
        drop(active(addr, 2).open(&id).unwrap());
        let failed = active(addr, 1).open(&id).err().expect("deposed");
        assert_eq!(failed.kind(), io::ErrorKind::PermissionDenied, "{failed}");
    }

    #[cfg(shuttle)]
    mod shuttle_tests {
        use shuttle::scheduler::RandomScheduler;
        use shuttle::{Config, MaxSteps, Runner};
        use tokio::sync::mpsc::error::TryRecvError;

        use super::*;

        /// Runs `test` in many random interleavings of its threads, as `active.rs`'s do. A thread
        /// that waits with nothing left to wake it fails as a deadlock, or past the step limit if
        /// it spins.
        fn check(test: impl Fn() + Send + Sync + 'static) {
            let mut config = Config::new();
            config.max_steps = MaxSteps::FailAfter(20_000);
            Runner::new(RandomScheduler::new(5_000), config).run(test);
        }

        /// A session's commits on `handle`: batches 1 and 2 sent, then each waited for. Each gets
        /// exactly one result: its own Ack, or the connection lost.
        fn commits(conn: Arc<Conn>, handle: u32) -> shuttle::thread::JoinHandle<()> {
            shuttle::thread::spawn(move || {
                let started: Vec<_> =
                    (1..=2).map(|n| (n, Shared::start(&conn, handle, &Frame::Batch(numbered(handle, n))))).collect();
                for (commit_no, started) in started {
                    // Refused at once, as the connection was lost: that's its result.
                    let Ok(answer) = started else { continue };
                    match answer.rx.recv().expect("answered, or told the connection is lost") {
                        Ok(frame) => assert_eq!(frame, Frame::Ack { handle, commit_no }),
                        Err(e) => assert_eq!(e.kind(), io::ErrorKind::ConnectionAborted, "{e}"),
                    }
                    assert!(answer.rx.recv().is_err(), "a second result for {handle}/{commit_no}");
                }
            })
        }

        /// Two sessions' commits, the standby answering what reaches it in order, and the
        /// connection lost at any point: each request gets its own answer or the loss, once, and
        /// none waits forever. A handle's commits are made one after another, as a store's are
        /// (`StoredSession::commit`), so each session is one thread.
        #[test]
        fn every_request_gets_its_own_answer_or_the_loss() {
            check(|| {
                let (outbox, mut frames) = async_mpsc::unbounded_channel();
                let conn = Arc::new(Conn::new(1, outbox));
                let sessions = [commits(conn.clone(), 1), commits(conn.clone(), 2)];
                let standby = {
                    let conn = conn.clone();
                    shuttle::thread::spawn(move || {
                        // Until the connection is lost: its reader is stopped then, so what's
                        // still in the outbox is never answered.
                        while conn.alive.load(Ordering::Acquire) {
                            match frames.try_recv() {
                                Ok(bytes) => {
                                    let Ok(Some((Frame::Batch(batch), _))) = Frame::decode(&bytes) else {
                                        panic!("not a batch")
                                    };
                                    assert!(
                                        conn.answered(Frame::Ack { handle: batch.handle, commit_no: batch.commit_no })
                                    );
                                }
                                Err(TryRecvError::Empty | TryRecvError::Disconnected) => shuttle::thread::yield_now(),
                            }
                        }
                    })
                };
                let losing = {
                    let conn = conn.clone();
                    shuttle::thread::spawn(move || conn.lost())
                };
                for thread in sessions.into_iter().chain([standby, losing]) {
                    thread.join().unwrap();
                }
                assert!(!conn.alive.load(Ordering::Acquire));
                assert!(conn.waiting.lock().unwrap().is_empty(), "nobody left waiting");
            });
        }
    }
}
