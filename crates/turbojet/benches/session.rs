//! Session-layer costs without I/O: the state machine processing orders, and storage writes.

mod common;

use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use turbojet::codec::{Decoded, decode, encode};
use turbojet::fields::EncryptMethod;
use turbojet::session::Action;
use turbojet::store::{SessionLog, SessionStorage};
use turbojet::{DiskStorage, MemoryStorage, Message, Session, SessionConfig, SessionId, SessionRegistry, admin};

/// Messages per logged-on session; a fresh session (and store) is set up, untimed, for each chunk
/// so the in-memory resend store doesn't grow without bound.
const CHUNK: u64 = 10_000;

/// A logged-on acceptor session with `storage`, whose counterparty's next MsgSeqNum is 2.
fn logged_on(storage: Arc<dyn SessionStorage>) -> Session {
    let registry = Arc::new(SessionRegistry::new(storage));
    let (mut session, _commands) = Session::acceptor(
        SessionConfig::new("FIX.4.2", "GATEWAY"),
        registry,
        Arc::new(common::Acker::default()),
        Instant::now(),
    );
    let logon = admin::Logon {
        encrypt_method: EncryptMethod::None,
        heart_bt_int: 30,
        reset_seq_num_flag: None,
        next_expected_msg_seq_num: None,
        username: None,
        password: None,
        default_appl_ver_id: None,
    };
    session.on_message(common::with_header("CLIENT", "GATEWAY", 1, logon.into()), Instant::now());
    assert!(session.is_logged_on());
    session
}

fn orders(count: u64) -> Vec<Message> {
    (0..count).map(|i| common::with_header("CLIENT", "GATEWAY", i + 2, common::new_order_single(i).into())).collect()
}

/// Times `run` over `iters` messages, in chunks with untimed setup.
fn chunked<T>(iters: u64, setup: impl Fn(u64) -> T, mut run: impl FnMut(T)) -> Duration {
    let mut total = Duration::ZERO;
    let mut remaining = iters;
    while remaining > 0 {
        let n = remaining.min(CHUNK);
        let input = setup(n);
        let start = Instant::now();
        run(input);
        total += start.elapsed();
        remaining -= n;
    }
    total
}

fn session(c: &mut Criterion) {
    let mut group = c.benchmark_group("session");
    group.throughput(Throughput::Elements(1));

    // Order in, ExecutionReport out: sequencing, dispatch, typed parse and build in the
    // application, framing, and recording in the in-memory store.
    group.bench_function("order to ack (memory store)", |b| {
        b.iter_custom(|iters| {
            chunked(
                iters,
                |n| (logged_on(Arc::new(MemoryStorage::new())), orders(n)),
                |(mut session, orders)| {
                    let now = Instant::now();
                    for order in orders {
                        black_box(session.on_message(order, now));
                    }
                },
            )
        })
    });

    // The same with sequence numbers and sent messages written to disk (without fsync), as in
    // production: the store's writes per order, not only the state machine's work.
    group.bench_function("order to ack (disk store)", |b| {
        b.iter_custom(|iters| {
            chunked(
                iters,
                |n| {
                    let dir = tempfile::tempdir().unwrap();
                    let storage = Arc::new(DiskStorage::new(dir.path(), false).unwrap());
                    (logged_on(storage), orders(n), dir)
                },
                |(mut session, orders, _dir)| {
                    let now = Instant::now();
                    for order in orders {
                        black_box(session.on_message(order, now));
                    }
                },
            )
        })
    });

    // The same with a real (Prometheus) metrics recorder installed, so every counter and gauge
    // update is recorded rather than a no-op.
    #[cfg(feature = "metrics")]
    group.bench_function("order to ack (memory store, Prometheus recorder)", |b| {
        let recorder = metrics_exporter_prometheus::PrometheusBuilder::new().build_recorder();
        b.iter_custom(|iters| {
            chunked(
                iters,
                // Metric handles are created when the session binds, so bind under the recorder.
                |n| (metrics::with_local_recorder(&recorder, || logged_on(Arc::new(MemoryStorage::new()))), orders(n)),
                |(mut session, orders)| {
                    let now = Instant::now();
                    for order in orders {
                        black_box(session.on_message(order, now));
                    }
                },
            )
        })
    });

    // The same, plus decoding the order from bytes and encoding the reply: the full CPU cost of
    // a message, wire to wire, excluding the socket.
    group.bench_function("order to ack, wire to wire (memory store)", |b| {
        b.iter_custom(|iters| {
            chunked(
                iters,
                |n| {
                    let wire: Vec<Vec<u8>> = orders(n).iter().map(|order| encode(order).unwrap()).collect();
                    (logged_on(Arc::new(MemoryStorage::new())), wire)
                },
                |(mut session, wire)| {
                    let now = Instant::now();
                    for bytes in wire {
                        let Decoded::Message(msg, _) = decode(&bytes) else { panic!("bad order") };
                        for action in session.on_message(msg, now) {
                            if let Action::Send(reply) = action {
                                black_box(encode(&reply).unwrap());
                            }
                        }
                    }
                },
            )
        })
    });
    group.finish();
}

fn storage(c: &mut Criterion) {
    let id =
        SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() };
    let report: Message =
        common::with_header("GATEWAY", "CLIENT", 2, common::ack(common::new_order_single(1), 1).into());

    let mut group = c.benchmark_group("storage record_outgoing");
    group.throughput(Throughput::Elements(1));
    let memory = MemoryStorage::new();
    let dir = tempfile::tempdir().unwrap();
    let disk = DiskStorage::new(dir.path(), false).unwrap();
    for (name, storage) in [("memory", &memory as &dyn SessionStorage), ("disk", &disk as &dyn SessionStorage)] {
        let mut log: Box<dyn SessionLog> = storage.open(&id).unwrap();
        group.bench_function(name, |b| {
            b.iter_custom(|iters| {
                // Reset (untimed) every CHUNK writes so the store doesn't grow without bound.
                let mut total = Duration::ZERO;
                let mut remaining = iters;
                while remaining > 0 {
                    let n = remaining.min(CHUNK);
                    log.reset().unwrap();
                    let start = Instant::now();
                    for seq in 1..=n {
                        log.record_outgoing(seq, Some(&report)).unwrap();
                    }
                    total += start.elapsed();
                    remaining -= n;
                }
                total
            })
        });
    }
    group.finish();

    // fsync per write: dominated by the device, so fewer samples.
    let mut group = c.benchmark_group("storage record_outgoing");
    group.sample_size(10).measurement_time(Duration::from_secs(10));
    let dir = tempfile::tempdir().unwrap();
    let mut log = DiskStorage::new(dir.path(), true).unwrap().open(&id).unwrap();
    let mut seq = 0;
    group.bench_function("disk + fsync", |b| {
        b.iter(|| {
            seq += 1;
            log.record_outgoing(seq, Some(&report)).unwrap()
        })
    });
    group.finish();
}

criterion_group!(benches, session, storage);
criterion_main!(benches);
