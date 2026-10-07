//! End to end over localhost: an Initiator sends NewOrderSingle through a SessionHandle, an
//! Acceptor's application acknowledges it, and the initiator's application receives the
//! ExecutionReport. Both sides use DiscardStorage (see benches/common) so long runs don't
//! accumulate messages, except in the disk store groups, where the acceptor stores to disk, with
//! and without fsync, to show what its writes cost end to end.
//!
//! The latency benchmarks send from the benchmark's task, through a SessionHandle, and hand each
//! acknowledgement back to it: a hop between tasks each way. "latency, replying from on_message"
//! has the initiator's application send each next order from `on_message` instead, as an
//! application reacting to what it receives would, so only the first order and the last
//! acknowledgement hop. "tcp, spinning" runs it with each end on a thread of its own, polling a
//! non-blocking socket without waiting (`run_spinning`). "tcp, message log" sets a message log that
//! does nothing at both ends, for what the hook costs; "tcp, file message log" a `FileMessageLog` at
//! both ends, writing under the target directory.

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use turbojet::connection::SpinningStream;
use turbojet::{
    Acceptor, Application, ConnectionInfo, Context, DiskStorage, FileLogOptions, FileMessageLog, Initiator,
    InitiatorConfig, Message, MessageLog, MessageReject, SessionConfig, SessionHandle, SessionId, SessionStorage,
};

/// Orders in flight at once in the pipelined benchmark.
const WINDOW: u64 = 1_000;
/// Orders in flight at once in the pipelined benchmark with fsync.
const FSYNC_WINDOW: u64 = 100;

/// Forwards logon and received messages to the benchmark. While `chain` is above zero, it answers
/// each message with the next order itself instead, counting `chain` down.
struct Client {
    logged_on: mpsc::UnboundedSender<()>,
    received: mpsc::UnboundedSender<Message>,
    chain: AtomicU64,
    next_id: AtomicU64,
}

impl Application for Client {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.logged_on.send(());
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        // Only this task counts it down, and the benchmark sets it only while nothing is in flight.
        let chain = self.chain.load(Ordering::Relaxed);
        if chain > 0 {
            self.chain.store(chain - 1, Ordering::Relaxed);
            ctx.send(common::new_order_single(self.next_id.fetch_add(1, Ordering::Relaxed)));
        } else {
            let _ = self.received.send(msg.clone());
        }
        Ok(())
    }
}

struct Connection {
    handle: SessionHandle,
    acks: mpsc::UnboundedReceiver<Message>,
    client: Arc<Client>,
}

/// Where the file message log benchmark writes, emptied before it runs.
const FILE_LOG_DIR: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/roundtrip-message-log");

/// A message log that does nothing, for what the hook costs the driver.
#[derive(Debug)]
struct NoLog;

impl MessageLog for NoLog {
    fn inbound(&self, _session: Option<&SessionId>, _frame: &[u8]) {}
    fn outbound(&self, _session: Option<&SessionId>, _frame: &[u8]) {}
}

/// A FIX 4.2 session sent as `sender`, changed by `tweak`.
fn config(sender: &str, tweak: fn(&mut SessionConfig)) -> SessionConfig {
    let mut config = SessionConfig::new("FIX.4.2", sender);
    tweak(&mut config);
    config
}

/// Starts an acceptor storing to `storage` and a logged-on initiator, both configured by `tweak`.
/// With `tls`, both use TLS.
async fn connect(
    #[allow(unused)] tls: bool,
    storage: Arc<dyn SessionStorage>,
    tweak: fn(&mut SessionConfig),
) -> Connection {
    let acceptor = Acceptor::new(config("GATEWAY", tweak), storage, Arc::new(common::Acker::default())).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();

    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let (received, acks) = mpsc::unbounded_channel();
    let mut config = InitiatorConfig::new(config("CLIENT", tweak), "GATEWAY");
    config.reset_on_logon = true;
    let client = Arc::new(Client { logged_on, received, chain: AtomicU64::new(0), next_id: AtomicU64::new(0) });
    let initiator = Initiator::new(addr, config, Arc::new(common::DiscardStorage), client.clone()).unwrap();

    #[cfg(feature = "tls")]
    let initiator = if tls {
        let pki = tls_pki::Pki::new();
        tokio::spawn(acceptor.serve_tls(listener, pki.acceptor()));
        initiator.with_tls(pki.connector(), "localhost").unwrap()
    } else {
        tokio::spawn(acceptor.serve(listener));
        initiator
    };
    #[cfg(not(feature = "tls"))]
    tokio::spawn(acceptor.serve(listener));

    let handle = initiator.handle();
    tokio::spawn(initiator.run());
    tokio::time::timeout(Duration::from_secs(5), logons.recv()).await.expect("logon timed out");
    Connection { handle, acks, client }
}

/// Starts an acceptor and a logged-on initiator, each driven by `run_spinning` on a thread of its
/// own. Shutting the initiator down ends both threads.
async fn connect_spinning() -> (Connection, Initiator) {
    let acceptor =
        Acceptor::new(config("GATEWAY", |_| {}), Arc::new(common::DiscardStorage), Arc::new(common::Acker::default()))
            .unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let (received, acks) = mpsc::unbounded_channel();
    let mut config = InitiatorConfig::new(config("CLIENT", |_| {}), "GATEWAY");
    config.reset_on_logon = true;
    let client = Arc::new(Client { logged_on, received, chain: AtomicU64::new(0), next_id: AtomicU64::new(0) });
    let initiator = Initiator::new(addr.to_string(), config, Arc::new(common::DiscardStorage), client.clone()).unwrap();

    let runtime = Handle::current();
    std::thread::spawn({
        let runtime = runtime.clone();
        move || {
            let socket = SpinningStream::new(listener.accept().unwrap().0).unwrap();
            acceptor.accept_spinning(socket, ConnectionInfo::default(), &runtime)
        }
    });
    std::thread::spawn({
        let initiator = initiator.clone();
        move || {
            let socket = SpinningStream::new(std::net::TcpStream::connect(addr).unwrap()).unwrap();
            initiator.run_spinning(socket, ConnectionInfo::default(), &runtime)
        }
    });
    tokio::time::timeout(Duration::from_secs(5), logons.recv()).await.expect("logon timed out");
    (Connection { handle: initiator.handle(), acks, client }, initiator)
}

/// One order at a time, each sent by the initiator's application as the last is acknowledged,
/// with both ends spinning.
fn spinning(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let (mut conn, initiator) = runtime.block_on(connect_spinning());
    let mut group = c.benchmark_group("roundtrip tcp, spinning");
    group.throughput(Throughput::Elements(1));
    let mut next_id = 0u64;
    group.bench_function("latency, replying from on_message", |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                conn.client.chain.store(iters - 1, Ordering::Relaxed);
                next_id += 1;
                conn.handle.send(common::new_order_single(next_id)).unwrap();
                conn.acks.recv().await.unwrap();
                start.elapsed()
            })
        })
    });
    group.finish();
    // Frees the spinning cores for the benchmarks after this one.
    runtime.block_on(initiator.shutdown(None));
}

/// The latency and pipelined benchmarks, `window` orders in flight in the latter.
fn transport(c: &mut Criterion, name: &str, tls: bool, storage: Arc<dyn SessionStorage>, window: u64) {
    transport_with(c, name, tls, storage, window, |_| {});
}

/// [`transport`], both ends' sessions configured by `tweak`.
fn transport_with(
    c: &mut Criterion,
    name: &str,
    tls: bool,
    storage: Arc<dyn SessionStorage>,
    window: u64,
    tweak: fn(&mut SessionConfig),
) {
    let runtime = Runtime::new().unwrap();
    let mut conn = runtime.block_on(connect(tls, storage, tweak));
    let mut next_id = 0u64;
    let mut group = c.benchmark_group(format!("roundtrip {name}"));
    if name.contains("fsync") {
        // Dominated by the device, so fewer samples.
        group.sample_size(10);
    }

    // One order at a time: send, then wait for its acknowledgement.
    group.throughput(Throughput::Elements(1));
    group.bench_function("latency", |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    next_id += 1;
                    conn.handle.send(common::new_order_single(next_id)).unwrap();
                    conn.acks.recv().await.unwrap();
                }
                start.elapsed()
            })
        })
    });

    // One order at a time, each sent by the initiator's application as the last is acknowledged.
    group.bench_function("latency, replying from on_message", |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                conn.client.chain.store(iters - 1, Ordering::Relaxed);
                next_id += 1;
                conn.handle.send(common::new_order_single(next_id)).unwrap();
                conn.acks.recv().await.unwrap();
                start.elapsed()
            })
        })
    });

    // `window` orders in flight, then wait for all their acknowledgements.
    group.throughput(Throughput::Elements(window));
    group.bench_function(format!("pipelined x{window}"), |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    for _ in 0..window {
                        next_id += 1;
                        conn.handle.send(common::new_order_single(next_id)).unwrap();
                    }
                    for _ in 0..window {
                        conn.acks.recv().await.unwrap();
                    }
                }
                start.elapsed()
            })
        })
    });
    group.finish();
}

/// Messages a raw client asks to have resent in the `resend` benchmark.
const RESENT: u64 = 100_000;

/// Reads from `stream` until `count` more messages have ended (a CheckSum field each), keeping
/// the last few bytes in `tail` so a CheckSum split across reads still counts.
async fn read_messages(stream: &mut TcpStream, buf: &mut Vec<u8>, count: u64) {
    let mut seen = 0;
    let mut tail = [0u8; 3];
    while seen < count {
        buf.clear();
        buf.extend_from_slice(&tail);
        let n = stream.read_buf(buf).await.unwrap();
        assert!(n > 0, "connection closed");
        seen += buf.windows(4).filter(|w| w == b"\x0110=").count() as u64;
        tail.copy_from_slice(&buf[buf.len() - 3..]);
    }
    assert_eq!(seen, count, "more arrived than expected");
}

/// A raw client's ResendRequest for 100,000 stored messages, answered by an acceptor over
/// localhost: the session's steps, the driver's writes, and the client reading every byte.
fn resend(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let (mut stream, mut seq) = runtime.block_on(async {
        let acceptor = Acceptor::new(
            SessionConfig::new("FIX.4.2", "GATEWAY"),
            Arc::new(turbojet::MemoryStorage::new()),
            Arc::new(common::Acker::default()),
        )
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(acceptor.serve(listener));
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.set_nodelay(true).unwrap();
        let logon = Message::new(turbojet::MsgType::Logon)
            .with(turbojet::message::tags::ENCRYPT_METHOD, "0")
            .with(turbojet::message::tags::HEART_BT_INT, 30u64);
        let encode = |seq, body| turbojet::codec::encode(&common::with_header("CLIENT", "GATEWAY", seq, body)).unwrap();
        stream.write_all(&encode(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        read_messages(&mut stream, &mut buf, 1).await;
        for first in (0..RESENT).step_by(1_000) {
            let mut wire = Vec::new();
            for id in first..first + 1_000 {
                wire.extend(encode(id + 2, common::new_order_single(id).into()));
            }
            stream.write_all(&wire).await.unwrap();
            read_messages(&mut stream, &mut buf, 1_000).await;
        }
        (stream, RESENT + 2)
    });
    let mut group = c.benchmark_group("resend tcp");
    group.throughput(Throughput::Elements(RESENT)).sample_size(10);
    group.bench_function(format!("{RESENT} messages (memory store)"), |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let mut buf = Vec::with_capacity(64 * 1024);
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let body = turbojet::admin::ResendRequest { begin_seq_no: 1, end_seq_no: 0 };
                    let request = turbojet::codec::encode(&common::with_header("CLIENT", "GATEWAY", seq, body.into()));
                    seq += 1;
                    let start = Instant::now();
                    stream.write_all(&request.unwrap()).await.unwrap();
                    // The Logon is gap-filled, then every order's ExecutionReport is resent.
                    read_messages(&mut stream, &mut buf, RESENT + 1).await;
                    total += start.elapsed();
                }
                total
            })
        })
    });
    group.finish();
}

fn roundtrip(c: &mut Criterion) {
    resend(c);
    transport(c, "tcp", false, Arc::new(common::DiscardStorage), WINDOW);
    transport_with(c, "tcp, message log", false, Arc::new(common::DiscardStorage), WINDOW, |config| {
        config.message_log = Some(Arc::new(NoLog));
    });
    let _ = std::fs::remove_dir_all(FILE_LOG_DIR);
    transport_with(c, "tcp, file message log", false, Arc::new(common::DiscardStorage), WINDOW, |config| {
        // Each end writes to a directory of its own, since two logs in one could race for a file name.
        static ENDS: AtomicUsize = AtomicUsize::new(0);
        let dir = format!("{FILE_LOG_DIR}/{}", ENDS.fetch_add(1, Ordering::Relaxed));
        config.message_log = Some(Arc::new(FileMessageLog::open(dir, FileLogOptions::default()).unwrap()));
    });
    spinning(c);
    #[cfg(feature = "tls")]
    transport(c, "tls", true, Arc::new(common::DiscardStorage), WINDOW);
    let dir = tempfile::tempdir().unwrap();
    let disk = |sync| Arc::new(DiskStorage::new(dir.path().join(format!("sync-{sync}")), sync).unwrap());
    transport(c, "tcp, disk store", false, disk(false), WINDOW);
    // An fsync takes milliseconds, so fewer orders are in flight.
    transport(c, "tcp, disk store + fsync", false, disk(true), FSYNC_WINDOW);
    // Last, since the recorder stays installed: what the latency histograms cost, against the
    // same recorder without them.
    #[cfg(feature = "metrics")]
    {
        metrics_exporter_prometheus::PrometheusBuilder::new().install_recorder().unwrap();
        transport(c, "tcp, Prometheus recorder", false, Arc::new(common::DiscardStorage), WINDOW);
        transport_with(
            c,
            "tcp, Prometheus recorder, latency metrics",
            false,
            Arc::new(common::DiscardStorage),
            WINDOW,
            |config| config.latency_metrics = true,
        );
    }
}

criterion_group!(benches, roundtrip);
criterion_main!(benches);

/// A throwaway CA and server certificate for `localhost`.
#[cfg(feature = "tls")]
mod tls_pki {
    use rcgen::{BasicConstraints, CertificateParams, IsCa, Issuer, KeyPair};
    use turbojet::tls;

    pub struct Pki {
        dir: tempfile::TempDir,
    }

    impl Pki {
        pub fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let ca_key = KeyPair::generate().unwrap();
            let mut ca_params = CertificateParams::new(Vec::<String>::new()).unwrap();
            ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
            let ca = ca_params.self_signed(&ca_key).unwrap();
            let issuer = Issuer::new(ca_params, ca_key);
            let key = KeyPair::generate().unwrap();
            let cert = CertificateParams::new(vec!["localhost".into()]).unwrap().signed_by(&key, &issuer).unwrap();
            std::fs::write(dir.path().join("ca.pem"), ca.pem()).unwrap();
            std::fs::write(dir.path().join("server.pem"), cert.pem()).unwrap();
            std::fs::write(dir.path().join("server.key"), key.serialize_pem()).unwrap();
            Self { dir }
        }

        pub fn acceptor(&self) -> tls::TlsAcceptor {
            let path = |name: &str| self.dir.path().join(name);
            tls::acceptor(&path("server.pem"), &path("server.key"), tls::ClientAuth::None).unwrap()
        }

        pub fn connector(&self) -> tls::TlsConnector {
            tls::connector(&self.dir.path().join("ca.pem"), None).unwrap()
        }
    }
}
