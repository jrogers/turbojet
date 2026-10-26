//! What replication adds to a commit: storing a sent message and committing it, alone and copied
//! to a standby over loopback TCP, with memory stores and with disk stores (with and without
//! fsync).

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use criterion::{Criterion, criterion_group, criterion_main};
use turbojet::store::conformance::{app_message, id};
use turbojet::store::{SessionStorage, StoredSession};
use turbojet::{DiskStorage, MemoryStorage};
use turbojet_replica::{Replica, ReplicatedStorage, TcpLink, serve_standby};

/// Messages per session before it's reset, untimed, so stores don't grow without bound.
const CHUNK: u64 = 10_000;

/// A standby serving on loopback, keeping its copy in `store`.
fn standby<S: SessionStorage + Send + 'static>(store: S) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let replica = Arc::new(Mutex::new(Replica::new(store)));
    std::thread::spawn(move || serve_standby(&listener, &replica));
    addr
}

/// Times storing and committing one message each, as a session does after each send.
fn store_and_commit(log: &mut dyn StoredSession, iters: u64) -> Duration {
    let msg = app_message(2);
    let mut total = Duration::ZERO;
    let mut remaining = iters;
    while remaining > 0 {
        let n = remaining.min(CHUNK);
        log.reset().unwrap();
        let start = Instant::now();
        for seq in 1..=n {
            log.record_outgoing(seq, Some(&msg)).unwrap();
            if let Some(commit) = log.commit().unwrap() {
                commit.run().unwrap();
            }
        }
        total += start.elapsed();
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
    let to = standby(MemoryStorage::new());
    let replicated = ReplicatedStorage::new(MemoryStorage::new(), Arc::new(TcpLink::new(to, 1, "bench")), 1);
    let mut log = replicated.open(&session).unwrap();
    group.bench_function("memory, replicated", |b| b.iter_custom(|iters| store_and_commit(log.as_mut(), iters)));

    for fsync in [false, true] {
        let name = if fsync { "disk + fsync" } else { "disk" };
        if fsync {
            group.sample_size(10);
        }
        let mut alone = disk(&format!("{name} alone"), fsync).open(&session).unwrap();
        group.bench_function(name, |b| b.iter_custom(|iters| store_and_commit(alone.as_mut(), iters)));
        let to = standby(disk(&format!("{name} standby"), fsync));
        let active = disk(&format!("{name} active"), fsync);
        let replicated = ReplicatedStorage::new(active, Arc::new(TcpLink::new(to, 1, "bench")), 1);
        let mut log = replicated.open(&session).unwrap();
        group.bench_function(format!("{name}, replicated"), |b| {
            b.iter_custom(|iters| store_and_commit(log.as_mut(), iters))
        });
    }
    group.finish();
}

criterion_group!(benches, commits);
criterion_main!(benches);
