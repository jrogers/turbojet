//! SqlStorage against DiskStorage: storing messages and committing them, one per commit (one at
//! a time) or 100 (a busy connection's batch), and reading a resend step back. PostgreSQL runs
//! when `TURBOJET_POSTGRES_URL` names a database the benchmark may clear.

use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use turbojet::DiskStorage;
use turbojet::store::conformance::{app_message, commit, fetch, id, open};
use turbojet::store::{SessionLog, SessionStorage};
use turbojet_sql::{SqlConfig, SqlStorage};

/// Messages stored before an untimed reset, so a store doesn't grow without bound.
const CHUNK: u64 = 10_000;
/// Messages a resend step reads, as the session's `MAX_RESEND_BATCH`.
const STEP: u64 = 256;

/// The stores compared, each with a directory or database of its own.
fn stores(runtime: &Runtime, dir: &std::path::Path) -> Vec<(&'static str, Arc<dyn SessionStorage>)> {
    let sqlite = |name: &str, sync: bool| {
        let url = format!("sqlite://{}?mode=rwc", dir.join(format!("{name}.db")).display());
        let storage = runtime.block_on(SqlStorage::connect(&url, SqlConfig { sync, ..SqlConfig::default() })).unwrap();
        runtime.block_on(storage.migrate()).unwrap();
        Arc::new(storage) as Arc<dyn SessionStorage>
    };
    let mut stores: Vec<(&'static str, Arc<dyn SessionStorage>)> = vec![
        ("disk", Arc::new(DiskStorage::new(dir.join("disk"), false).unwrap())),
        ("disk + fsync", Arc::new(DiskStorage::new(dir.join("disk-sync"), true).unwrap())),
        ("sqlite (synchronous normal)", sqlite("normal", false)),
        ("sqlite (synchronous full)", sqlite("full", true)),
    ];
    if let Some(url) = std::env::var("TURBOJET_POSTGRES_URL").ok().filter(|url| !url.is_empty()) {
        let storage = runtime.block_on(SqlStorage::connect(&url, SqlConfig::default())).unwrap();
        runtime.block_on(storage.migrate()).unwrap();
        // An earlier run may have left its sessions leased.
        runtime.block_on(async {
            sqlx::any::install_default_drivers();
            let pool = sqlx::AnyPool::connect(&url).await.unwrap();
            sqlx::query("DELETE FROM turbojet_messages").execute(&pool).await.unwrap();
            sqlx::query("DELETE FROM turbojet_sessions").execute(&pool).await.unwrap();
        });
        stores.push(("postgres", Arc::new(storage)));
    }
    stores
}

/// Records `n` messages from `seq` on, committing after each `per_commit`, as the driver would.
async fn store_messages(log: &mut dyn SessionLog, report: &[u8], seq: u64, n: u64, per_commit: u64) {
    for i in 1..=n {
        log.record_outgoing(seq + i - 1, Some(report)).unwrap();
        if i % per_commit == 0 {
            commit(log).await.unwrap();
        }
    }
}

fn record_outgoing(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let report = app_message(2);
    let mut group = c.benchmark_group("sql record_outgoing");
    group.sample_size(10).measurement_time(Duration::from_secs(10));
    for (name, storage) in stores(&runtime, dir.path()) {
        let mut log = runtime.block_on(open(storage.as_ref(), &id("BENCH"))).unwrap();
        for per_commit in [1, 100] {
            group.throughput(Throughput::Elements(per_commit));
            group.bench_function(format!("{name}, {per_commit} per commit"), |b| {
                b.iter_custom(|iters| {
                    runtime.block_on(async {
                        let mut total = Duration::ZERO;
                        let mut seq = 1;
                        for _ in 0..iters {
                            if seq + per_commit > CHUNK {
                                log.reset().unwrap();
                                commit(log.as_mut()).await.unwrap();
                                seq = 1;
                            }
                            let start = Instant::now();
                            store_messages(log.as_mut(), &report, seq, per_commit, per_commit).await;
                            total += start.elapsed();
                            seq += per_commit;
                        }
                        total
                    })
                })
            });
        }
        runtime.block_on(async {
            log.reset().unwrap();
            commit(log.as_mut()).await.unwrap();
        });
    }
    group.finish();
}

/// Reading one resend step of a session that has stored `CHUNK` messages, from a log opened
/// after they were committed (as on reconnecting), so the store reads them back.
fn resend_step(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let mut group = c.benchmark_group("sql resend step");
    group.throughput(Throughput::Elements(STEP));
    for (name, storage) in stores(&runtime, dir.path()) {
        let mut log = runtime.block_on(async {
            let mut log = open(storage.as_ref(), &id("RESEND")).await.unwrap();
            // Each its own message: DiskStorage indexes them by their MsgSeqNum on opening.
            for seq in 1..=CHUNK {
                log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
            }
            commit(log.as_mut()).await.unwrap();
            drop(log);
            open(storage.as_ref(), &id("RESEND")).await.unwrap()
        });
        let mut begin = 1;
        group.bench_function(format!("{STEP} messages ({name})"), |b| {
            b.iter(|| {
                let read = runtime.block_on(fetch(log.as_mut(), begin, begin + STEP - 1)).unwrap();
                assert_eq!(read.len(), usize::try_from(STEP).unwrap(), "{name} from {begin}");
                black_box(read);
                begin = if begin + 2 * STEP > CHUNK { 1 } else { begin + STEP };
            })
        });
    }
    group.finish();
}

criterion_group!(benches, record_outgoing, resend_step);
criterion_main!(benches);
