//! A soak: Turbojet against itself for a while, through a proxy that cuts, blackholes, stalls,
//! holds and slows each direction, with either process crashing now and then, over disk stores,
//! by plain TCP and by TLS. Where the simulator (`turbojet-sim`) drives sessions as the connection
//! driver does, this runs the real `Initiator`, `Acceptor`, tokio driver and TLS.
//!
//! The client sends orders as fast as its queue takes them, and the server acknowledges each.
//! Every order the client's store recorded must reach the server's application, once each and in
//! order (a repeat only marked as possibly handled), and every order the server's application saw
//! must be acknowledged, likewise. It goes on until every kind of fault has come and the time is
//! up, then must settle within a couple of minutes.
//!
//! It runs only with `TURBOJET_SOAK=<seconds>` (each way, TCP and TLS), in the nightly simulation
//! workflow; with no value set, both tests pass at once.

use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant, SystemTime};

use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use turbojet::message::tags;
use turbojet::tls::{ClientTls, ClientTrust, Identity, ServerTls, Trust};
use turbojet::{
    Acceptor, Application, Context, DiskStorage, Initiator, InitiatorConfig, InitiatorTls, Message, MessageReject,
    MsgType, ReconnectPolicy, SessionConfig,
};
use turbojet_interop::{Dir, Pki, Proxy, Role};

/// How long the faults go on, from `TURBOJET_SOAK`, if set.
fn duration() -> Option<Duration> {
    let secs = std::env::var("TURBOJET_SOAK").ok()?.parse().expect("TURBOJET_SOAK is a number of seconds");
    Some(Duration::from_secs(secs))
}

#[test]
fn soak_over_tcp() {
    match duration() {
        Some(lasts) => soak(false, lasts),
        None => eprintln!("skipped: set TURBOJET_SOAK=<seconds> to soak"),
    }
}

#[test]
fn soak_over_tls() {
    match duration() {
        Some(lasts) => soak(true, lasts),
        None => eprintln!("skipped: set TURBOJET_SOAK=<seconds> to soak"),
    }
}

/// The kinds of fault, each lasting a while and then undone: the proxy's, on the connection or one
/// direction of it (to the server or to the client), and either process crashing, to restart after.
const KINDS: [&str; 11] = [
    "cut",
    "blackhole to server",
    "blackhole to client",
    "stall to server",
    "stall to client",
    "hold to server",
    "hold to client",
    "slow to server",
    "slow to client",
    "crash client",
    "crash server",
];

/// How long the sessions have to settle once the faults stop.
const SETTLE: Duration = Duration::from_secs(120);

/// What each side's application saw, shared with the test.
#[derive(Default)]
struct Seen {
    /// The last order number the client tried to send.
    sent: u64,
    /// The client stops sending, for the sessions to settle.
    stopped: bool,
    /// Orders the client's store recorded, by their receipts.
    stored: BTreeSet<u64>,
    /// Orders the server's application received, in order, each with whether it was marked as
    /// possibly handled already.
    orders: Vec<(u64, bool)>,
    /// Acknowledgements the client's application received, likewise.
    acks: Vec<(u64, bool)>,
}

type Shared = Arc<Mutex<Seen>>;

fn lock(seen: &Shared) -> std::sync::MutexGuard<'_, Seen> {
    seen.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The order number a message carries in ClOrdID.
fn number(msg: &Message) -> Option<u64> {
    msg.get(tags::CL_ORD_ID)?.parse().ok()
}

/// The server acknowledges each order with an ExecutionReport carrying its number.
struct Server(Shared);

impl Application for Server {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::NewOrderSingle
            && let Some(n) = number(msg)
        {
            lock(&self.0).orders.push((n, ctx.maybe_redelivered()));
            let ack = Message::new(MsgType::ExecutionReport)
                .with(tags::CL_ORD_ID, n)
                .with(tags::EXEC_ID, n)
                .with(tags::ORD_STATUS, "0");
            // Refused only as the session ends: a redelivered order acknowledges it again.
            let _ = ctx.send(ack);
        }
        Ok(())
    }
}

struct Client(Shared);

impl Application for Client {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::ExecutionReport
            && let Some(n) = number(msg)
        {
            lock(&self.0).acks.push((n, ctx.maybe_redelivered()));
        }
        Ok(())
    }
}

/// A process: a runtime of its own, which a crash drops with everything on it, sessions and
/// connections, without a word to the other side. Its store's files stay, as a process crash
/// leaves them.
struct Process(Option<Runtime>);

impl Process {
    fn start<F: Future<Output = ()> + Send + 'static>(run: impl FnOnce() -> F) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
        {
            let _entered = runtime.enter();
            runtime.spawn(run());
        }
        Self(Some(runtime))
    }

    fn crash(&mut self) {
        if let Some(runtime) = self.0.take() {
            runtime.shutdown_background();
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.crash();
    }
}

/// The two ends' fixed parts: their stores' directories, the server's address, the proxy's, and
/// TLS if over it.
struct Setup {
    dir: tempfile::TempDir,
    server: std::net::SocketAddr,
    proxy: std::net::SocketAddr,
    pki: Option<Pki>,
    seen: Shared,
}

impl Setup {
    fn store(&self, side: &str) -> Arc<DiskStorage> {
        let dir: PathBuf = self.dir.path().join(side);
        // Without fsync: a process crash keeps what was written, and the soak needs no power loss.
        Arc::new(DiskStorage::new(dir).unwrap().with_fsync(false))
    }

    fn server(&self) -> Process {
        let storage = self.store("server");
        let seen = self.seen.clone();
        let tls = self.pki.as_ref().map(|pki| {
            let (cert, key) = pki.identity_paths("tj");
            ServerTls::new(Identity::from_pem_files(&cert, &key).unwrap(), ClientTrust::None).unwrap().acceptor()
        });
        // Bound again after a crash: the port is the proxy's upstream.
        let listener = std::net::TcpListener::bind(self.server).unwrap();
        listener.set_nonblocking(true).unwrap();
        Process::start(move || async move {
            let listener = TcpListener::from_std(listener).unwrap();
            let acceptor =
                Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), storage, Arc::new(Server(seen))).unwrap();
            let served = match tls {
                Some(tls) => acceptor.serve_tls(listener, tls).await,
                None => acceptor.serve(listener).await,
            };
            served.unwrap();
        })
    }

    fn client(&self) -> Process {
        let storage = self.store("client");
        let seen = self.seen.clone();
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER")
            .with_endpoint(self.proxy.to_string());
        config.heartbeat_interval = Duration::from_secs(1);
        config.reconnect = ReconnectPolicy::fixed(Duration::from_millis(200));
        config.connect_timeout = Duration::from_secs(2);
        if let Some(pki) = &self.pki {
            let trust = Trust::from_pem_files(&pki.ca_path(true)).unwrap();
            config.tls = Some(InitiatorTls::new(ClientTls::new(trust, None).unwrap().connector(), "localhost"));
        }
        Process::start(move || async move {
            let initiator = Initiator::new(config, storage, Arc::new(Client(seen.clone()))).unwrap();
            let handle = initiator.handle();
            tokio::spawn(initiator.run());
            // Orders as fast as the queue takes them, each number once, however many crashes.
            loop {
                let n = {
                    let mut seen = lock(&seen);
                    if seen.stopped {
                        return;
                    }
                    seen.sent += 1;
                    seen.sent
                };
                let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, n).with(tags::SIDE, "1");
                match handle.send(order) {
                    Ok(receipt) => {
                        let seen = seen.clone();
                        tokio::spawn(async move {
                            if receipt.await.is_ok() {
                                lock(&seen).stored.insert(n);
                            }
                        });
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                    Err(_) => tokio::time::sleep(Duration::from_millis(50)).await,
                }
            }
        })
    }
}

/// A small random stream, seeded from the time and printed, so a failure says which run it was.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn millis(&mut self, lo: u64, hi: u64) -> Duration {
        Duration::from_millis(lo + self.next() % (hi - lo + 1))
    }
}

fn soak(tls: bool, lasts: Duration) {
    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
    runtime.block_on(async {
        let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let mut rng = Rng(u64::try_from(seed % u128::from(u64::MAX)).unwrap());
        eprintln!("soak over {}: {lasts:?} of faults, seed {}", if tls { "TLS" } else { "TCP" }, rng.0);
        let server = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        // The proxy runs on this runtime, apart from both processes; it sees Turbojet initiating.
        let mut proxy = Proxy::start(server, Role::TjInitiator).await;
        let setup = Setup {
            dir: tempfile::TempDir::with_prefix("turbojet-soak-").unwrap(),
            server,
            proxy: proxy.addr(),
            pki: tls.then(Pki::new),
            seen: Shared::default(),
        };
        let mut server_process = setup.server();
        let mut client_process = setup.client();

        run_faults(&mut proxy, &setup, &mut rng, lasts, &mut server_process, &mut client_process).await;
        settle(&mut proxy, &setup.seen).await;
        let seen = lock(&setup.seen);
        check(&seen);
        let marked = seen.orders.iter().filter(|(_, m)| *m).count();
        eprintln!(
            "{} orders stored, {} delivered ({marked} marked possibly handled), {} acknowledgements",
            seen.stored.len(),
            seen.orders.len(),
            seen.acks.len()
        );
        assert!(seen.stored.len() > 1_000, "only {} orders went through", seen.stored.len());
        drop(seen);
        drop(client_process);
        drop(server_process);
    });
}

/// Faults, one at a time, until `lasts` is up and every kind has come.
async fn run_faults(
    proxy: &mut Proxy,
    setup: &Setup,
    rng: &mut Rng,
    lasts: Duration,
    server_process: &mut Process,
    client_process: &mut Process,
) {
    let start = Instant::now();
    let mut faults: BTreeMap<&str, u64> = BTreeMap::new();
    while start.elapsed() < lasts || faults.len() < KINDS.len() {
        let kind = KINDS[usize::try_from(rng.next() % KINDS.len() as u64).unwrap()];
        *faults.entry(kind).or_default() += 1;
        let (dir, which) = if kind.ends_with("to server") {
            (Dir::ToPeer, kind.trim_end_matches(" to server"))
        } else {
            (Dir::ToTj, kind.trim_end_matches(" to client"))
        };
        let lasts = rng.millis(200, 3_000);
        match which {
            "cut" => proxy.cut(),
            "blackhole" => proxy.blackhole(dir),
            "stall" => proxy.stall(dir),
            "hold" => proxy.hold(dir),
            "slow" => proxy.bandwidth(dir, u32::try_from(rng.next() % 20_000).unwrap() + 1_000),
            "crash client" => client_process.crash(),
            "crash server" => server_process.crash(),
            other => unreachable!("{other}"),
        }
        tokio::time::sleep(lasts).await;
        match which {
            "crash client" => *client_process = setup.client(),
            "crash server" => *server_process = setup.server(),
            _ => proxy.heal(),
        }
        proxy.clear_events();
        tokio::time::sleep(rng.millis(200, 1_000)).await;
    }
    proxy.heal();
    lock(&setup.seen).stopped = true;
    eprintln!("faults: {faults:?}");
}

/// Waits for everything stored to be delivered and everything delivered acknowledged.
async fn settle(proxy: &mut Proxy, seen: &Shared) {
    let settling = Instant::now();
    loop {
        {
            let seen = lock(seen);
            let orders: BTreeSet<u64> = seen.orders.iter().map(|(n, _)| *n).collect();
            let acks: BTreeSet<u64> = seen.acks.iter().map(|(n, _)| *n).collect();
            if seen.stored.is_subset(&orders) && orders.is_subset(&acks) {
                break;
            }
            assert!(
                settling.elapsed() < SETTLE,
                "not settled after {SETTLE:?}: {} stored but not delivered, {} delivered but not acknowledged \
                 (first {:?}, {:?})",
                seen.stored.difference(&orders).count(),
                orders.difference(&acks).count(),
                seen.stored.difference(&orders).next(),
                orders.difference(&acks).next(),
            );
        }
        proxy.clear_events();
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Each side's application got everything once, in order: a number seen before only marked as
/// possibly handled (for an acknowledgement, or its order seen twice by the server), and each new
/// one after the last new one; and only orders the client sent.
fn check(seen: &Seen) {
    let mut orders_seen: BTreeMap<u64, u32> = BTreeMap::new();
    let mut last = 0;
    for &(n, marked) in &seen.orders {
        assert!(n <= seen.sent, "the server got order {n}, never sent");
        let times = orders_seen.entry(n).or_default();
        *times += 1;
        if *times > 1 {
            assert!(marked, "the server got order {n} again, unmarked");
        } else {
            assert!(n > last, "the server got order {n} after {last}");
            last = n;
        }
    }
    let mut acks_seen = BTreeSet::new();
    let mut last = 0;
    for &(n, marked) in &seen.acks {
        assert!(orders_seen.contains_key(&n), "the client got an acknowledgement of {n}, which the server never saw");
        if !acks_seen.insert(n) {
            assert!(marked || orders_seen[&n] > 1, "the client got the acknowledgement of {n} again, unmarked");
        } else {
            assert!(n > last, "the client got the acknowledgement of {n} after {last}");
            last = n;
        }
    }
}
