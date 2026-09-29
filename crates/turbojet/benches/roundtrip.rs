//! End to end over localhost: an Initiator sends NewOrderSingle through a SessionHandle, an
//! Acceptor's application acknowledges it, and the initiator's application receives the
//! ExecutionReport. Both sides use DiscardStorage (see benches/common) so long runs don't
//! accumulate messages; storage costs are measured in the session benchmark.

mod common;

use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, Message, MessageReject, SessionConfig, SessionHandle,
};

/// Orders in flight at once in the pipelined benchmark.
const WINDOW: u64 = 1_000;

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

/// Starts an acceptor and a logged-on initiator. With `tls`, both use TLS.
async fn connect(#[allow(unused)] tls: bool) -> Connection {
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", "GATEWAY"),
        Arc::new(common::DiscardStorage),
        Arc::new(common::Acker::default()),
    );
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

fn transport(c: &mut Criterion, name: &str, tls: bool) {
    let runtime = Runtime::new().unwrap();
    let mut conn = runtime.block_on(connect(tls));
    let mut next_id = 0u64;
    let mut group = c.benchmark_group(format!("roundtrip {name}"));

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

    // WINDOW orders in flight, then wait for all their acknowledgements.
    group.throughput(Throughput::Elements(WINDOW));
    group.bench_function(format!("pipelined x{WINDOW}"), |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    for _ in 0..WINDOW {
                        next_id += 1;
                        conn.handle.send(common::new_order_single(next_id)).unwrap();
                    }
                    for _ in 0..WINDOW {
                        conn.acks.recv().await.unwrap();
                    }
                }
                start.elapsed()
            })
        })
    });
    group.finish();
}

fn roundtrip(c: &mut Criterion) {
    transport(c, "tcp", false);
    #[cfg(feature = "tls")]
    transport(c, "tls", true);
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
