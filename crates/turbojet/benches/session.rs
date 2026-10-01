//! Session-layer costs without I/O: the state machine processing orders, and storage writes.

mod common;

use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use turbojet::codec::{DecodedInto, decode_into, encode};
use turbojet::message::DataFields;
use turbojet::store::{SessionLog, SessionStorage};
use turbojet::{DiskStorage, MemoryStorage, Message, SessionId};

/// Messages per logged-on session; a fresh session (and store) is set up, untimed, for each chunk
/// so the in-memory resend store doesn't grow without bound.
const CHUNK: u64 = 10_000;

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

#[expect(clippy::too_many_lines, reason = "one benchmark group, read top to bottom")]
fn session(c: &mut Criterion) {
    let mut group = c.benchmark_group("session");
    group.throughput(Throughput::Elements(1));

    // Order in, ExecutionReport out: sequencing, dispatch, typed parse and build in the
    // application, framing, and recording in the in-memory store.
    group.bench_function("order to ack (memory store)", |b| {
        b.iter_custom(|iters| {
            chunked(
                iters,
                |n| {
                    (
                        common::logged_on(Arc::new(MemoryStorage::new()), Arc::new(common::Acker::default())),
                        common::orders(n),
                    )
                },
                |(mut session, orders)| {
                    let now = Instant::now();
                    for order in orders {
                        session.on_message(&order, now);
                        black_box(session.output());
                        session.clear_output();
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
                    (common::logged_on(storage, Arc::new(common::Acker::default())), common::orders(n), dir)
                },
                |(mut session, orders, _dir)| {
                    let now = Instant::now();
                    for order in orders {
                        session.on_message(&order, now);
                        black_box(session.output());
                        session.clear_output();
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
                |n| {
                    (
                        metrics::with_local_recorder(&recorder, || {
                            common::logged_on(Arc::new(MemoryStorage::new()), Arc::new(common::Acker::default()))
                        }),
                        common::orders(n),
                    )
                },
                |(mut session, orders)| {
                    let now = Instant::now();
                    for order in orders {
                        session.on_message(&order, now);
                        black_box(session.output());
                        session.clear_output();
                    }
                },
            )
        })
    });

    // The same, plus decoding the order from bytes (the session encodes the reply): the full CPU
    // cost of a message, wire to wire, excluding the socket.
    group.bench_function("order to ack, wire to wire (memory store)", |b| {
        b.iter_custom(|iters| {
            chunked(
                iters,
                |n| {
                    let wire: Vec<Vec<u8>> = common::orders(n).iter().map(|order| encode(order).unwrap()).collect();
                    (common::logged_on(Arc::new(MemoryStorage::new()), Arc::new(common::Acker::default())), wire)
                },
                |(mut session, wire)| {
                    let now = Instant::now();
                    let mut msg = Message::default();
                    let data = DataFields::standard();
                    for bytes in wire {
                        let DecodedInto::Message(_) = decode_into(&bytes, &data, &mut msg) else { panic!("bad order") };
                        session.on_message(&msg, now);
                        black_box(session.output());
                        session.clear_output();
                    }
                },
            )
        })
    });
    group.finish();
}

/// Messages a counterparty asks to have resent in the `resend` benchmarks.
const RESENT: u64 = 100_000;

/// Answering a ResendRequest for everything a session has sent: reading the stored messages,
/// decoding them and framing them again as resends. Each sample resends the same range of the
/// same session.
fn resend(c: &mut Criterion) {
    let mut group = c.benchmark_group("resend");
    group.throughput(Throughput::Elements(RESENT)).sample_size(10);
    let dir = tempfile::tempdir().unwrap();
    let stores: [(&str, Arc<dyn SessionStorage>); 2] = [
        ("memory store", Arc::new(MemoryStorage::new())),
        ("disk store", Arc::new(DiskStorage::new(dir.path(), false).unwrap())),
    ];
    for (name, storage) in stores {
        let mut session = common::logged_on(storage, Arc::new(common::Acker::default()));
        let now = Instant::now();
        for order in common::orders(RESENT) {
            session.on_message(&order, now);
            session.clear_output();
        }
        let mut seq = RESENT + 2;
        group.bench_function(format!("{RESENT} messages ({name})"), |b| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let body = turbojet::admin::ResendRequest { begin_seq_no: 1, end_seq_no: 0 };
                    let request = common::with_header("CLIENT", "GATEWAY", seq, body.into());
                    seq += 1;
                    let start = Instant::now();
                    session.on_message(&request, now);
                    black_box(session.output());
                    session.clear_output();
                    total += start.elapsed();
                }
                total
            })
        });
    }
    group.finish();
}

fn storage(c: &mut Criterion) {
    let id =
        SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() };
    let report: Message = common::with_header("GATEWAY", "CLIENT", 2, common::ack_of(1).into());
    let report = encode(&report).unwrap();

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

criterion_group!(benches, session, resend, storage);
criterion_main!(benches);
