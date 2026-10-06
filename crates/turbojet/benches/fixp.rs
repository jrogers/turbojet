//! FIXP end to end over localhost, beside the FIX round trips in `roundtrip.rs`: a FixpInitiator
//! sends B3 NewOrderSingles (from the codec generated from B3's schema, under standard FIXP 1.0
//! session messages), the FixpAcceptor's application answers each with an order back, and the
//! initiator's application receives it. Recoverable flows both ways, so both ends store what they
//! send, in memory (a FIXP server must remember a session was negotiated to establish it, which
//! the FIX benchmarks' discarding store wouldn't).
//!
//! "latency, replying from on_message" has the initiator's application send each next order from
//! `on_message`, as the FIX benchmark of that name does; "pipelined" keeps a window in flight.

#[allow(dead_code)]
#[path = "../tests/sbe/b3.rs"]
mod b3;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use turbojet::MemoryStorage;
use turbojet::fixp::{
    ClientConfig, FixpAcceptor, FixpApplication, FixpConfig, FixpContext, FixpHandle, FixpInitiator, Received, Role,
    SbeMessage, ServerConfig,
};

/// Orders in flight at once in the pipelined benchmark.
const WINDOW: u64 = 1_000;

fn order(cl_ord_id: u64) -> b3::NewOrderSingle {
    b3::NewOrderSingle {
        cl_ord_id,
        security_id: 4001,
        price: b3::PriceOptional { mantissa: Some(1_502_500) },
        order_qty: 100,
        account: Some(1),
        market_segment_id: 1,
        side: b3::Side::Buy,
        ord_type: b3::OrdType::Limit,
        time_in_force: b3::TimeInForce::Day,
        ord_tag_id: None,
        mm_protection_reset: None,
        routing_instruction: None,
        self_trade_prevention_instruction: None,
        stop_px: b3::PriceOptional { mantissa: None },
        min_qty: None,
        max_floor: None,
        investor_id: None,
        custodian_info: b3::CustodianInfo { custodian: None, custody_account: None, custody_allocation_type: None },
        expire_date: None,
        sender_location: turbojet::sbe::pad(b"DMA"),
        entering_trader: *b"TRADR",
    }
}

/// Answers each order with an order of the same ClOrdID.
struct Server;

impl FixpApplication for Server {
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let Ok((b3::Decoded::NewOrderSingle(received), _)) = b3::decode(msg.bytes) else { return };
        ctx.send(&order(received.cl_ord_id())).unwrap();
    }
}

/// Forwards establishment and received orders to the benchmark. While `chain` is above zero, it
/// answers each order with the next itself instead, counting `chain` down.
struct Client {
    established: mpsc::UnboundedSender<()>,
    received: mpsc::UnboundedSender<()>,
    chain: AtomicU64,
    next_id: AtomicU64,
}

impl FixpApplication for Client {
    fn on_established(&self, _session: &FixpHandle) {
        let _ = self.established.send(());
    }

    fn on_message(&self, ctx: &mut FixpContext<'_>, _msg: Received<'_>) {
        // Only this task counts it down, and the benchmark sets it only while nothing is in flight.
        let chain = self.chain.load(Ordering::Relaxed);
        if chain > 0 {
            self.chain.store(chain - 1, Ordering::Relaxed);
            ctx.send(&order(self.next_id.fetch_add(1, Ordering::Relaxed))).unwrap();
        } else {
            let _ = self.received.send(());
        }
    }
}

fn fixp(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let (established, mut established_rx) = mpsc::unbounded_channel();
    let (received, mut acks) = mpsc::unbounded_channel();
    let client = Arc::new(Client { established, received, chain: AtomicU64::new(0), next_id: AtomicU64::new(1) });
    let handle = runtime.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let config = FixpConfig::new(Role::Server(ServerConfig::new("SERVER")));
        let acceptor = FixpAcceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(Server)).unwrap();
        tokio::spawn(acceptor.serve(listener));
        let config = FixpConfig::new(Role::Client(ClientConfig::new("CLIENT", "SERVER")));
        let initiator =
            FixpInitiator::new(addr.to_string(), config, Arc::new(MemoryStorage::new()), client.clone()).unwrap();
        tokio::spawn(initiator.clone().run());
        established_rx.recv().await.unwrap();
        initiator.handle()
    });
    let send = |id: u64| handle.send(SbeMessage::encode(&order(id)).unwrap()).unwrap();

    let mut group = c.benchmark_group("roundtrip fixp tcp");
    group.throughput(Throughput::Elements(1));
    group.bench_function("latency, replying from on_message", |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                client.chain.store(iters - 1, Ordering::Relaxed);
                drop(send(client.next_id.fetch_add(1, Ordering::Relaxed)));
                acks.recv().await.unwrap();
                start.elapsed()
            })
        })
    });
    group.throughput(Throughput::Elements(WINDOW));
    group.bench_function(format!("pipelined x{WINDOW}"), |b| {
        b.iter_custom(|iters| {
            runtime.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    for _ in 0..WINDOW {
                        drop(send(client.next_id.fetch_add(1, Ordering::Relaxed)));
                    }
                    for _ in 0..WINDOW {
                        acks.recv().await.unwrap();
                    }
                }
                start.elapsed()
            })
        })
    });
    group.finish();
}

criterion_group!(benches, fixp);
criterion_main!(benches);
