//! End to end over localhost: an Initiator sends NewOrderSingle through a SessionHandle, an
//! Acceptor's application acknowledges it, and the initiator's application receives the
//! ExecutionReport. Both sides use DiscardStorage (see benches/common) so long runs don't
//! accumulate messages, except in the disk store groups, where the acceptor stores to disk, with
//! and without fsync, to show what its writes cost end to end.

mod common;

use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use turbojet::{
    Acceptor, Application, Context, DiskStorage, Initiator, InitiatorConfig, Message, MessageReject, SessionConfig,
    SessionHandle, SessionStorage,
};

/// Orders in flight at once in the pipelined benchmark.
const WINDOW: u64 = 1_000;
/// Orders in flight at once in the pipelined benchmark with fsync.
const FSYNC_WINDOW: u64 = 100;

/// Forwards logon and every received message to the benchmark.
struct Client {
    logged_on: mpsc::UnboundedSender<()>,
    received: mpsc::UnboundedSender<Message>,
}

impl Application for Client {
    fn on_logon(&self, _session: SessionHandle) {
        let _ = self.logged_on.send(());
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.received.send(msg.clone());
        Ok(())
    }
}

struct Connection {
    handle: SessionHandle,
    acks: mpsc::UnboundedReceiver<Message>,
}

/// Starts an acceptor storing to `storage` and a logged-on initiator. With `tls`, both use TLS.
async fn connect(#[allow(unused)] tls: bool, storage: Arc<dyn SessionStorage>) -> Connection {
    let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "GATEWAY"), storage, Arc::new(common::Acker::default()));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();

    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let (received, acks) = mpsc::unbounded_channel();
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "GATEWAY");
    config.reset_on_logon = true;
    let initiator =
        Initiator::new(addr, config, Arc::new(common::DiscardStorage), Arc::new(Client { logged_on, received }));

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
    Connection { handle, acks }
}

/// The latency and pipelined benchmarks, `window` orders in flight in the latter.
fn transport(c: &mut Criterion, name: &str, tls: bool, storage: Arc<dyn SessionStorage>, window: u64) {
    let runtime = Runtime::new().unwrap();
    let mut conn = runtime.block_on(connect(tls, storage));
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
        );
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
    #[cfg(feature = "tls")]
    transport(c, "tls", true, Arc::new(common::DiscardStorage), WINDOW);
    let dir = tempfile::tempdir().unwrap();
    let disk = |sync| Arc::new(DiskStorage::new(dir.path().join(format!("sync-{sync}")), sync).unwrap());
    transport(c, "tcp, disk store", false, disk(false), WINDOW);
    // An fsync takes milliseconds, so fewer orders are in flight.
    transport(c, "tcp, disk store + fsync", false, disk(true), FSYNC_WINDOW);
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
