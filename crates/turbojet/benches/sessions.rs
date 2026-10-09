//! Many sessions at once, over localhost: one Acceptor serving N initiators, each its own session,
//! all on one multi-threaded runtime. Each session's sender, a task of its own, keeps `WINDOW`
//! orders in flight through its SessionHandle and waits for their acknowledgements, all N at once,
//! so the throughput shows how the engine scales across cores. Both ends keep sent messages in a
//! MemoryStorage shared by all their sessions, as an acceptor would.
//!
//! This is where data shared between sessions shows up: the session registry, the shared store,
//! the allocator, and any cache line two cores write. Per-session throughput that falls as
//! sessions are added, beyond what the cores allow, points at one of them. On a machine with C
//! cores, the counts past C/2 (each session is two connections) share cores and say more about
//! scheduling than contention.

mod common;

use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, SessionConfig,
    SessionHandle,
};

/// Orders each session keeps in flight.
const WINDOW: u64 = 1_000;
/// The session counts measured.
const SESSIONS: [usize; 6] = [1, 2, 4, 8, 16, 32];

/// Forwards logon and each acknowledgement to the benchmark.
struct Client {
    logged_on: mpsc::UnboundedSender<()>,
    acked: mpsc::UnboundedSender<()>,
}

impl Application for Client {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.logged_on.send(());
    }

    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        let _ = self.acked.send(());
        Ok(())
    }
}

struct Sender {
    handle: SessionHandle,
    acks: mpsc::UnboundedReceiver<()>,
}

/// An acceptor and `count` logged-on initiators, CLIENT0 onwards.
async fn connect(count: usize) -> Vec<Sender> {
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", "GATEWAY"),
        Arc::new(MemoryStorage::new()),
        Arc::new(common::Acker::default()),
    )
    .unwrap()
    // All the initiators connect from 127.0.0.1.
    .with_max_connections_per_ip(count);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(acceptor.serve(listener));

    let storage = Arc::new(MemoryStorage::new());
    let (logged_on, mut logons) = mpsc::unbounded_channel();
    let mut senders = Vec::with_capacity(count);
    for i in 0..count {
        let (acked, acks) = mpsc::unbounded_channel();
        let client = Arc::new(Client { logged_on: logged_on.clone(), acked });
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", format!("CLIENT{i}")), "GATEWAY");
        config.reset_on_logon = true;
        let initiator = Initiator::new(addr.clone(), config, storage.clone(), client).unwrap();
        senders.push(Sender { handle: initiator.handle(), acks });
        tokio::spawn(initiator.run());
    }
    for _ in 0..count {
        tokio::time::timeout(Duration::from_secs(5), logons.recv()).await.expect("logon timed out");
    }
    senders
}

/// Each sender sends `iters` windows, waiting for each window's acknowledgements, all at once.
async fn run(senders: Vec<Sender>, iters: u64) -> Vec<Sender> {
    let tasks: Vec<_> = senders
        .into_iter()
        .enumerate()
        .map(|(i, mut sender)| {
            tokio::spawn(async move {
                let mut id = (i as u64) << 40;
                for _ in 0..iters {
                    for _ in 0..WINDOW {
                        id += 1;
                        sender.handle.send(common::new_order_single(id)).unwrap();
                    }
                    for _ in 0..WINDOW {
                        sender.acks.recv().await.unwrap();
                    }
                }
                sender
            })
        })
        .collect();
    let mut senders = Vec::with_capacity(tasks.len());
    for task in tasks {
        senders.push(task.await.unwrap());
    }
    senders
}

fn sessions(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let mut all = runtime.block_on(connect(SESSIONS[SESSIONS.len() - 1]));
    let mut group = c.benchmark_group("sessions tcp");
    for count in SESSIONS {
        group.throughput(Throughput::Elements(count as u64 * WINDOW));
        group.bench_function(format!("pipelined x{WINDOW}, {count} sessions"), |b| {
            b.iter_custom(|iters| {
                runtime.block_on(async {
                    let idle = all.split_off(count);
                    let start = Instant::now();
                    all = run(std::mem::take(&mut all), iters).await;
                    let elapsed = start.elapsed();
                    all.extend(idle);
                    elapsed
                })
            })
        });
    }
    group.finish();
}

criterion_group!(benches, sessions);
criterion_main!(benches);
