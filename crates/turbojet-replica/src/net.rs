//! Links over TCP: the standby's listener, and the active's [`TcpLink`] to it.
//!
//! Both ends block: the active's link calls are made in its store's jobs, which its sessions'
//! drivers run on blocking threads, and the standby applies batches with its store's blocking
//! calls. Each connection has a thread reading it.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use std::time::Duration;

use tracing::{debug, info, warn};
use turbojet::SessionId;
use turbojet::store::{Commit, SessionStorage};

use crate::active::{Link, deposed};
use crate::frame::{Batch, Frame, NackReason, Summary, VERSION};
use crate::standby::Replica;

/// How long the active waits for the standby's answer before giving the connection up: its
/// sessions' commits wait as long.
pub const DEFAULT_ANSWER_TIMEOUT: Duration = Duration::from_secs(5);

/// Serves the active instances that connect to `listener`, applying their batches to `replica`,
/// each connection on a thread of its own. Returns only if accepting fails.
///
/// # Errors
///
/// Accepting a connection failed.
pub fn serve_standby<S>(listener: &TcpListener, replica: &Arc<Mutex<Replica<S>>>) -> io::Result<()>
where
    S: SessionStorage + Send + 'static,
{
    for stream in listener.incoming() {
        let stream = stream?;
        let replica = replica.clone();
        std::thread::spawn(move || {
            let peer = stream.peer_addr().ok();
            if let Err(e) = serve_connection(stream, &replica) {
                debug!(?peer, error = %e, "replication connection ended");
            }
        });
    }
    Ok(())
}

/// One active's connection, until it ends: each frame applied, and its answer written back.
fn serve_connection<S: SessionStorage>(stream: TcpStream, replica: &Mutex<Replica<S>>) -> io::Result<()> {
    stream.set_nodelay(true)?;
    let mut frames = FrameReader::new(stream.try_clone()?);
    let mut writer = stream;
    let mut connection = None;
    let mut out = Vec::new();
    let result = loop {
        let frame = match frames.next() {
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
                if let Err(e) = writer.write_all(&out) {
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

impl<R: Read> FrameReader<R> {
    fn new(inner: R) -> Self {
        Self { inner, buf: Vec::new() }
    }

    /// The next frame, or `None` once the stream has ended between frames.
    fn next(&mut self) -> io::Result<Option<Frame>> {
        loop {
            match Frame::decode(&self.buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? {
                Some((frame, used)) => {
                    self.buf.drain(..used);
                    return Ok(Some(frame));
                }
                None => {
                    let mut chunk = [0; 64 * 1024];
                    let read = self.inner.read(&mut chunk)?;
                    if read == 0 {
                        return if self.buf.is_empty() { Ok(None) } else { Err(io::ErrorKind::UnexpectedEof.into()) };
                    }
                    self.buf.extend_from_slice(&chunk[..read]);
                }
            }
        }
    }
}

/// The active's [`Link`] to a standby over TCP. It connects when first used, and again after
/// losing the connection; a session bound before it was lost can't commit until it's opened
/// again, which binds it afresh and sends the standby whatever it lacks.
#[derive(Clone)]
pub struct TcpLink {
    shared: Arc<Shared>,
}

struct Shared {
    addr: String,
    epoch: u64,
    node: String,
    timeout: Duration,
    conn: Mutex<Option<Arc<Conn>>>,
    connections: AtomicU64,
    /// The connection each handle was bound on.
    bound: Mutex<HashMap<u32, u64>>,
}

/// One connection to the standby.
struct Conn {
    number: u64,
    writer: Mutex<TcpStream>,
    /// Who waits for the answer for each handle: a session has one request at most under way.
    waiting: Mutex<HashMap<u32, mpsc::SyncSender<io::Result<Frame>>>>,
    alive: AtomicBool,
}

impl std::fmt::Debug for TcpLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpLink").field("addr", &self.shared.addr).field("epoch", &self.shared.epoch).finish()
    }
}

impl TcpLink {
    /// A link to the standby at `addr`, as the active `node` holding `epoch`. It connects when
    /// first used.
    pub fn new(addr: impl Into<String>, epoch: u64, node: impl Into<String>) -> Self {
        Self::with_timeout(addr, epoch, node, DEFAULT_ANSWER_TIMEOUT)
    }

    /// As [`new`](Self::new), waiting `timeout` for each of the standby's answers.
    pub fn with_timeout(addr: impl Into<String>, epoch: u64, node: impl Into<String>, timeout: Duration) -> Self {
        Self {
            shared: Arc::new(Shared {
                addr: addr.into(),
                epoch,
                node: node.into(),
                timeout,
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

    fn connect(&self) -> io::Result<Arc<Conn>> {
        let addr = self.addr.to_socket_addrs()?.next().ok_or_else(|| io::Error::other("no address for the standby"))?;
        let stream = TcpStream::connect_timeout(&addr, self.timeout)?;
        stream.set_nodelay(true)?;
        stream.set_read_timeout(Some(self.timeout))?;
        let mut frames = FrameReader::new(stream.try_clone()?);
        let mut hello = Vec::new();
        Frame::Hello { version: VERSION, epoch: self.epoch, node: self.node.clone(), app_record_kinds: Vec::new() }
            .encode(&mut hello);
        (&stream).write_all(&hello)?;
        match frames.next()? {
            Some(Frame::Welcome { epoch }) if epoch == self.epoch => {}
            Some(Frame::Welcome { epoch }) => return Err(deposed(self.epoch, epoch)),
            other => return Err(io::Error::other(format!("the standby answered hello with {other:?}"))),
        }
        stream.set_read_timeout(None)?;
        let number = self.connections.fetch_add(1, Ordering::Relaxed) + 1;
        info!(standby = %self.addr, epoch = self.epoch, "connected to the standby");
        let conn = Arc::new(Conn {
            number,
            writer: Mutex::new(stream),
            waiting: Mutex::new(HashMap::new()),
            alive: AtomicBool::new(true),
        });
        let reading = conn.clone();
        std::thread::spawn(move || reading.read(frames));
        Ok(conn)
    }

    /// Sends `frame` for `handle` on `conn` and waits for the standby's answer.
    fn request(&self, conn: &Conn, handle: u32, frame: &Frame) -> io::Result<Frame> {
        let (tx, rx) = mpsc::sync_channel(1);
        conn.waiting.lock().unwrap_or_else(PoisonError::into_inner).insert(handle, tx);
        let mut out = Vec::new();
        frame.encode(&mut out);
        let written = conn.writer.lock().unwrap_or_else(PoisonError::into_inner).write_all(&out);
        if let Err(e) = written {
            conn.lost();
            return Err(e);
        }
        match rx.recv_timeout(self.timeout) {
            Ok(answer) => answer,
            Err(_) => {
                warn!(standby = %self.addr, "the standby didn't answer in time: dropping the connection");
                conn.lost();
                Err(io::Error::new(io::ErrorKind::TimedOut, "the standby didn't answer in time"))
            }
        }
    }
}

impl Conn {
    /// Reads the standby's answers until the connection ends, handing each to who waits for it.
    fn read(&self, mut frames: FrameReader<TcpStream>) {
        while let Ok(Some(frame)) = frames.next() {
            let (Frame::Bound { handle, .. } | Frame::Ack { handle, .. } | Frame::Nack { handle, .. }) = &frame else {
                warn!(?frame, "the standby sent a frame it shouldn't");
                break;
            };
            let waiter = self.waiting.lock().unwrap_or_else(PoisonError::into_inner).remove(handle);
            match waiter {
                Some(waiter) => {
                    let _ = waiter.send(Ok(frame));
                }
                None => warn!(?frame, "the standby answered a request nobody waits for"),
            }
        }
        self.lost();
    }

    /// The connection is lost: everyone waiting is told so.
    fn lost(&self) {
        self.alive.store(false, Ordering::Release);
        let _ = self.writer.lock().unwrap_or_else(PoisonError::into_inner).shutdown(std::net::Shutdown::Both);
        for (_, waiter) in self.waiting.lock().unwrap_or_else(PoisonError::into_inner).drain() {
            let _ = waiter.send(Err(io::Error::new(io::ErrorKind::ConnectionAborted, "lost the standby")));
        }
    }
}

impl Link for TcpLink {
    fn bind(&self, handle: u32, epoch: u64, session: &SessionId) -> io::Result<Summary> {
        let shared = &self.shared;
        let conn = shared.conn()?;
        match shared.request(&conn, handle, &Frame::Bind { handle, epoch, session: session.clone() })? {
            Frame::Bound { summary, .. } => {
                shared.bound.lock().unwrap_or_else(PoisonError::into_inner).insert(handle, conn.number);
                Ok(summary)
            }
            Frame::Nack { reason: NackReason::StaleEpoch, .. } => Err(deposed(epoch, epoch + 1)),
            other => Err(io::Error::other(format!("the standby answered a bind with {other:?}"))),
        }
    }

    fn send(&self, batch: Batch) -> Commit {
        let shared = self.shared.clone();
        Commit::blocking(move || {
            let conn = shared.conn()?;
            let bound_on = shared.bound.lock().unwrap_or_else(PoisonError::into_inner).get(&batch.handle).copied();
            if bound_on != Some(conn.number) {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "the connection to the standby was lost since the session opened: it must open again",
                ));
            }
            let (handle, epoch) = (batch.handle, batch.epoch);
            match shared.request(&conn, handle, &Frame::Batch(batch))? {
                Frame::Ack { .. } => Ok(()),
                Frame::Nack { reason: NackReason::StaleEpoch, .. } => Err(deposed(epoch, epoch + 1)),
                Frame::Nack { reason: NackReason::Diverged, .. } => {
                    Err(io::Error::other("the standby's copy of the session has diverged"))
                }
                other => Err(io::Error::other(format!("the standby answered a batch with {other:?}"))),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::net::SocketAddr;

    use turbojet::MemoryStorage;
    use turbojet::store::{StoredSession, conformance};

    use super::*;
    use crate::ReplicatedStorage;

    type Standby = (SocketAddr, Arc<Mutex<Replica<Arc<MemoryStorage>>>>, Arc<MemoryStorage>);

    /// A standby on a port of its own, serving from a thread.
    fn standby() -> Standby {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let store = Arc::new(MemoryStorage::new());
        let replica = Arc::new(Mutex::new(Replica::new(store.clone())));
        let serving = replica.clone();
        std::thread::spawn(move || serve_standby(&listener, &serving));
        (addr, replica, store)
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
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
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
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let replica = Arc::new(Mutex::new(Replica::new(store.clone())));
        let serving = replica.clone();
        std::thread::spawn(move || serve_standby(&listener, &serving));

        let id = conformance::id("A");
        let old = active(addr, 1);
        let mut log = old.open(&id).unwrap();
        log.record_outgoing(1, Some(&conformance::app_message(1))).unwrap();
        log.commit().unwrap().unwrap().run().unwrap();
        assert!(store.open(&id).is_err(), "the standby holds the session open");

        replica.lock().unwrap().promote();
        let taken = store.open(&id).expect("released");
        assert_eq!(taken.next_outgoing(), 2, "with what the old active committed");
        log.record_outgoing(2, Some(&conformance::app_message(2))).unwrap();
        assert!(log.commit().unwrap().unwrap().run().is_err(), "the old active is refused");
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
}
