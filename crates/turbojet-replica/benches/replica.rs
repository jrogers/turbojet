//! What replication adds to a commit: storing a sent message and committing it, alone and copied
//! to a standby over loopback TCP, synchronously and asynchronously, with memory stores and with
//! disk stores (with and without fsync); and many sessions committing at once over one link.

use std::sync::{Arc, Barrier, Mutex};
use std::time::{Duration, Instant};

use criterion::{Criterion, criterion_group, criterion_main};
use turbojet::store::conformance::{app_message, id};
use turbojet::store::{SessionStorage, StoredSession};
use turbojet::{DiskStorage, MemoryStorage};
use turbojet_replica::{Replica, ReplicatedStorage, TcpLink, serve_standby};

/// Messages per session before it's reset, untimed, so stores don't grow without bound.
const CHUNK: u64 = 10_000;

/// How far behind the asynchronous benchmarks let the standby be.
const LAG: u64 = 64;

/// A standby serving on loopback, keeping its copy in `store`, accepting up to `lag` behind.
fn standby<S: SessionStorage + Send + 'static>(store: S, lag: u64) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    listener.set_nonblocking(true).unwrap();
    let replica = Arc::new(Mutex::new(Replica::new(store).accepting_lag(lag)));
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(async { serve_standby(tokio::net::TcpListener::from_std(listener).unwrap(), replica).await })
    });
    addr
}

/// Times storing and committing one message each, as a session does after each send, from a log
/// reset (or new), leaving it reset.
fn store_and_commit(log: &mut dyn StoredSession, iters: u64) -> Duration {
    let msg = app_message(2);
    let mut total = Duration::ZERO;
    let mut remaining = iters;
    while remaining > 0 {
        let n = remaining.min(CHUNK);
        let start = Instant::now();
        for seq in 1..=n {
            log.record_outgoing(seq, Some(&msg)).unwrap();
            if let Some(commit) = log.commit().unwrap() {
                commit.run().unwrap();
            }
        }
        total += start.elapsed();
        log.reset().unwrap();
        remaining -= n;
    }
    total
}

fn commits(c: &mut Criterion) {
    let mut group = c.benchmark_group("replicated commit");
    let dir = tempfile::tempdir().unwrap();
    let disk = |name: &str, fsync: bool| DiskStorage::new(dir.path().join(name)).unwrap().with_fsync(fsync);
    let session = id("A");

    let mut alone = MemoryStorage::new().open(&session).unwrap();
    group.bench_function("memory", |b| b.iter_custom(|iters| store_and_commit(alone.as_mut(), iters)));
    for lag in [0, LAG] {
        let to = standby(MemoryStorage::new(), lag);
        let replicated =
            ReplicatedStorage::new(MemoryStorage::new(), Arc::new(TcpLink::new(to, 1, "bench")), 1).asynchronous(lag);
        let mut log = replicated.open(&session).unwrap();
        group.bench_function(format!("memory, replicated{}", label(lag)), |b| {
            b.iter_custom(|iters| store_and_commit(log.as_mut(), iters))
        });
    }

    for fsync in [false, true] {
        let name = if fsync { "disk + fsync" } else { "disk" };
        if fsync {
            group.sample_size(10);
        }
        let mut alone = disk(&format!("{name} alone"), fsync).open(&session).unwrap();
        group.bench_function(name, |b| b.iter_custom(|iters| store_and_commit(alone.as_mut(), iters)));
        for lag in [0, LAG] {
            let to = standby(disk(&format!("{name} standby{lag}"), fsync), lag);
            let active = disk(&format!("{name} active{lag}"), fsync);
            let replicated =
                ReplicatedStorage::new(active, Arc::new(TcpLink::new(to, 1, "bench")), 1).asynchronous(lag);
            let mut log = replicated.open(&session).unwrap();
            group.bench_function(format!("{name}, replicated{}", label(lag)), |b| {
                b.iter_custom(|iters| store_and_commit(log.as_mut(), iters))
            });
        }
    }
    group.finish();
}

/// The benchmark's suffix for a lag: none for synchronous, so its history carries on.
fn label(lag: u64) -> String {
    if lag == 0 { String::new() } else { format!(" async {lag}") }
}

/// `sessions` sessions of `storage`, opened once: the standby holds each open until its
/// connection ends, which a disk store locks.
fn open_sessions<S: SessionStorage>(storage: &S, sessions: u64) -> Vec<Box<dyn StoredSession>> {
    (0..sessions).map(|n| storage.open(&id(&format!("S{n}"))).unwrap()).collect()
}

/// Times `logs` each storing and committing their share of `iters` messages at once, on threads
/// of their own, as a gateway's busy sessions do: the slowest's time, scaled to `iters` when the
/// shares, at least one each, come to more.
fn at_once(logs: &mut [Box<dyn StoredSession>], iters: u64) -> Duration {
    let sessions = logs.len() as u64;
    let share = iters.div_ceil(sessions);
    let start = Barrier::new(logs.len());
    let slowest = std::thread::scope(|scope| {
        let threads: Vec<_> = logs
            .iter_mut()
            .map(|log| {
                let start = &start;
                scope.spawn(move || {
                    start.wait();
                    store_and_commit(log.as_mut(), share)
                })
            })
            .collect();
        threads.into_iter().map(|thread| thread.join().unwrap()).max().unwrap()
    });
    slowest.mul_f64(iters as f64 / (share * sessions) as f64)
}

/// What one link costs when many sessions commit at once: each session's batch is its own frame,
/// write and answer, and the standby commits each alone.
fn many_sessions(c: &mut Criterion) {
    let mut group = c.benchmark_group("replicated commits at once");
    let dir = tempfile::tempdir().unwrap();
    for sessions in [1, 8, 64] {
        for lag in [0, LAG] {
            let to = standby(MemoryStorage::new(), lag);
            let replicated = ReplicatedStorage::new(MemoryStorage::new(), Arc::new(TcpLink::new(to, 1, "bench")), 1)
                .asynchronous(lag);
            let mut logs = open_sessions(&replicated, sessions);
            group.bench_function(format!("memory, {sessions} sessions{}", label(lag)), |b| {
                b.iter_custom(|iters| at_once(&mut logs, iters))
            });
        }
    }
    group.sample_size(10);
    for sessions in [1, 8, 64] {
        let disk = |name: &str| DiskStorage::new(dir.path().join(format!("{name} {sessions}"))).unwrap();
        let to = standby(disk("standby"), 0);
        let replicated = ReplicatedStorage::new(disk("active"), Arc::new(TcpLink::new(to, 1, "bench")), 1);
        let mut logs = open_sessions(&replicated, sessions);
        group.bench_function(format!("disk + fsync, {sessions} sessions"), |b| {
            b.iter_custom(|iters| at_once(&mut logs, iters))
        });
    }
    group.finish();
}

criterion_group!(benches, commits, many_sessions);
criterion_main!(benches);
