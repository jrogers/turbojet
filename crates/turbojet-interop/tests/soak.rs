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
//! Replicated, the server is a pair: an active over a `ReplicatedStorage` and a standby serving its
//! link, each a process of its own, the link through proxies of its own and the lease in SQLite.
//! Either node crashes, the link takes the proxy's faults, and an operator takes over by the
//! documented sequence (the lease, `in_sync` under it, `Replica::promote`) from an active that
//! crashed or that a partition has cut off from the lease and the standby while the client can
//! still reach it, whose application must then get nothing more. Each run picks from its seed
//! whether the active degrades when the standby can't be reached.
//!
//! It runs only with `TURBOJET_SOAK=<seconds>` (each test: TCP and TLS, one acceptor or a pair), in
//! the nightly simulation workflow; with no value set, every test passes at once.

use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant, SystemTime};

use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use turbojet::message::tags;
use turbojet::store::Job;
use turbojet::tls::{ClientTls, ClientTrust, Identity, ServerTls, TlsAcceptor, Trust};
use turbojet::{
    Acceptor, Application, Context, DiskStorage, Initiator, InitiatorConfig, InitiatorTls, Message, MessageReject,
    MsgType, ReconnectPolicy, SessionConfig,
};
use turbojet_interop::{Dir, Pki, Proxy, Role};
use turbojet_replica::{Lease, Replica, ReplicatedStorage, TcpLink, serve_standby, serve_standby_tls};
use turbojet_sql::{SqlConfig, SqlStorage};

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
    /// Replicated: the node an operator has just replaced while it ran, and when.
    deposed: Option<(usize, Instant)>,
    /// Replicated: orders a node's application received well after it was replaced.
    fenced: Vec<(usize, u64)>,
}

type Shared = Arc<Mutex<Seen>>;

fn lock(seen: &Shared) -> std::sync::MutexGuard<'_, Seen> {
    seen.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The order number a message carries in ClOrdID.
fn number(msg: &Message) -> Option<u64> {
    msg.get(tags::CL_ORD_ID)?.parse().ok()
}

/// How long a replaced active may still hand over an order whose in-flight mark the standby took
/// just before its promotion. After that, nothing reaches its application: its commits are refused.
const FENCING: Duration = Duration::from_secs(1);

/// The server acknowledges each order with an ExecutionReport carrying its number. Replicated,
/// it knows which node it runs on.
struct Server(Shared, Option<usize>);

impl Application for Server {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::NewOrderSingle
            && let Some(n) = number(msg)
        {
            let mut seen = lock(&self.0);
            if let (Some(node), Some((deposed, at))) = (self.1, seen.deposed)
                && node == deposed
                && at.elapsed() > FENCING
            {
                seen.fenced.push((node, n));
            }
            seen.orders.push((n, ctx.maybe_redelivered()));
            drop(seen);
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
    server: SocketAddr,
    proxy: SocketAddr,
    pki: Option<Pki>,
    seen: Shared,
}

impl Setup {
    fn server(&self) -> Process {
        let storage = store(&self.dir, "server");
        let seen = self.seen.clone();
        let tls = self.pki.as_ref().map(|pki| server_tls(pki, ClientTrust::None));
        let listener = bound(self.server);
        Process::start(move || async move {
            let listener = TcpListener::from_std(listener).unwrap();
            let acceptor =
                Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), storage, Arc::new(Server(seen, None))).unwrap();
            let served = match tls {
                Some(tls) => acceptor.serve_tls(listener, tls).await,
                None => acceptor.serve(listener).await,
            };
            served.unwrap();
        })
    }

    fn client(&self) -> Process {
        client(store(&self.dir, "client"), &[self.proxy], self.pki.as_ref(), self.seen.clone())
    }
}

/// `side`'s store, in a directory of its own under `dir`.
fn store(dir: &tempfile::TempDir, side: &str) -> Arc<DiskStorage> {
    let dir: PathBuf = dir.path().join(side);
    // Without fsync: a process crash keeps what was written, and the soak needs no power loss.
    Arc::new(DiskStorage::new(dir).unwrap().with_fsync(false))
}

/// TLS for a server (FIX, or a standby's end of the link), with Turbojet's certificate.
fn server_tls(pki: &Pki, clients: ClientTrust) -> TlsAcceptor {
    let (cert, key) = pki.identity_paths("tj");
    ServerTls::new(Identity::from_pem_files(&cert, &key).unwrap(), clients).unwrap().acceptor()
}

/// A listener on `addr`, bound again after each crash: the port is a proxy's upstream.
fn bound(addr: SocketAddr) -> std::net::TcpListener {
    let listener = std::net::TcpListener::bind(addr).unwrap();
    listener.set_nonblocking(true).unwrap();
    listener
}

/// The client: an initiator to `endpoints`, trying each in turn, sending orders as fast as its
/// queue takes them, each number once, however many crashes.
fn client(storage: Arc<DiskStorage>, endpoints: &[SocketAddr], pki: Option<&Pki>, seen: Shared) -> Process {
    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
    for endpoint in endpoints {
        config = config.with_endpoint(endpoint.to_string());
    }
    config.heartbeat_interval = Duration::from_secs(1);
    config.reconnect = ReconnectPolicy::fixed(Duration::from_millis(200));
    config.connect_timeout = Duration::from_secs(2);
    if let Some(pki) = pki {
        let trust = Trust::from_pem_files(&pki.ca_path(true)).unwrap();
        config.tls = Some(InitiatorTls::new(ClientTls::new(trust, None).unwrap().connector(), "localhost"));
    }
    Process::start(move || async move {
        let initiator = Initiator::new(config, storage, Arc::new(Client(seen.clone()))).unwrap();
        let handle = initiator.handle();
        tokio::spawn(initiator.run());
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
        settle(&mut [&mut proxy], &setup.seen).await;
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
async fn settle(proxies: &mut [&mut Proxy], seen: &Shared) {
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
        for proxy in proxies.iter_mut() {
            proxy.clear_events();
        }
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

#[test]
fn soak_replicated_over_tcp() {
    match duration() {
        Some(lasts) => soak_replicated(false, lasts),
        None => eprintln!("skipped: set TURBOJET_SOAK=<seconds> to soak"),
    }
}

#[test]
fn soak_replicated_over_tls() {
    match duration() {
        Some(lasts) => soak_replicated(true, lasts),
        None => eprintln!("skipped: set TURBOJET_SOAK=<seconds> to soak"),
    }
}

/// The replicated soak's kinds of fault: the soak's on the active's FIX connection, the client
/// crashing, either node crashing, faults on the link to the standby, and an operator failing over
/// from an active that crashed or from one cut off from the client but still running.
const REPLICATED_KINDS: [&str; 18] = [
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
    "crash active",
    "crash standby",
    "cut link",
    "blackhole link",
    "stall link",
    "slow link",
    "fail over",
    "depose",
];

/// What an operator's takeover came to: done, or why not. Each run needs some of each done.
const TAKEN_OVER: [&str; 2] = ["failed over", "deposed"];

/// How long the active's lease lasts: a takeover waits for it to run out.
const LEASE: Duration = Duration::from_secs(2);

/// How long a takeover may wait for the lease: one degrading active cut off from the client holds
/// it until its session times out, then for a lease.
const TAKE_LEASE: Duration = Duration::from_secs(30);

/// The nodes' names, also their lease holders and their stores' directories.
const NODES: [&str; 2] = ["a", "b"];

type SharedReplica = Arc<Mutex<Replica<Arc<DiskStorage>>>>;

/// A node's lease, which a partition can cut off from the database: every call fails, as one to a
/// database out of reach does, until it heals.
struct Partitioned {
    lease: turbojet_sql::SqlLease,
    cut_off: Arc<AtomicBool>,
}

impl Partitioned {
    fn call<T: Send + 'static>(&self, job: Job<T>) -> Job<T> {
        if self.cut_off.load(Ordering::Relaxed) {
            Job::blocking(|| Err(io::Error::new(io::ErrorKind::TimedOut, "the database is out of reach")))
        } else {
            job
        }
    }
}

impl Lease for Partitioned {
    fn duration(&self) -> Duration {
        self.lease.duration()
    }

    fn acquire(&self) -> Job<u64> {
        self.call(self.lease.acquire())
    }

    fn renew(&self, epoch: u64) -> Job<()> {
        self.call(self.lease.renew(epoch))
    }

    fn release(&self, epoch: u64) -> Job<()> {
        self.call(self.lease.release(epoch))
    }

    fn set_in_sync(&self, epoch: u64, in_sync: bool) -> Job<()> {
        self.call(self.lease.set_in_sync(epoch, in_sync))
    }

    fn in_sync(&self, epoch: u64) -> Job<bool> {
        self.call(self.lease.in_sync(epoch))
    }
}

/// A replicated pair's fixed parts: each node's FIX and link ports, the proxies in front of them,
/// the lease's database, and whether the active degrades when the standby can't be reached.
struct Pair {
    dir: tempfile::TempDir,
    fix: [SocketAddr; 2],
    link: [SocketAddr; 2],
    fix_proxies: [SocketAddr; 2],
    link_proxies: [SocketAddr; 2],
    pki: Option<Pki>,
    seen: Shared,
    database: String,
    degrading: bool,
    /// Whether each node is cut off from the lease's database.
    cut_off: [Arc<AtomicBool>; 2],
}

impl Pair {
    /// The lease, as node `node` holds it.
    async fn lease(&self, node: usize) -> turbojet_sql::SqlLease {
        lease(&self.database, node).await
    }

    /// Node `node` as the active: it takes the lease (once the other's has run out), replicates to
    /// the other through its link proxy, catches the standby up every couple of seconds, and
    /// accepts FIX sessions.
    fn active(&self, node: usize) -> Process {
        let storage = store(&self.dir, NODES[node]);
        let seen = self.seen.clone();
        let tls = self.pki.as_ref().map(|pki| server_tls(pki, ClientTrust::None));
        let link_tls = self.pki.as_ref().map(|pki| {
            let trust = Trust::from_pem_files(&pki.ca_path(true)).unwrap();
            let (cert, key) = pki.identity_paths("peer");
            let identity = Identity::from_pem_files(&cert, &key).unwrap();
            InitiatorTls::new(ClientTls::new(trust, Some(identity)).unwrap().connector(), "localhost")
        });
        let (standby, database, degrading) = (self.link_proxies[1 - node], self.database.clone(), self.degrading);
        let cut_off = self.cut_off[node].clone();
        let listener = bound(self.fix[node]);
        Process::start(move || async move {
            let lease = Arc::new(Partitioned { lease: lease(&database, node).await, cut_off });
            let epoch = loop {
                match lease.acquire().run_async().await {
                    Ok(epoch) => break epoch,
                    Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
                }
            };
            let mut link = TcpLink::new(standby.to_string(), epoch, NODES[node]);
            if let Some(tls) = link_tls {
                link = link.with_tls(tls);
            }
            let mut replicated = ReplicatedStorage::new(storage, Arc::new(link), epoch).with_lease(lease);
            if degrading {
                replicated = replicated.degrading();
            }
            let replicated = Arc::new(replicated);
            let catching_up = replicated.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let _ = catching_up.catch_up().run_async().await;
                }
            });
            let listener = TcpListener::from_std(listener).unwrap();
            let config = SessionConfig::new("FIX.4.4", "SERVER");
            let acceptor = Acceptor::new(config, replicated, Arc::new(Server(seen, Some(node)))).unwrap();
            let served = match tls {
                Some(tls) => acceptor.serve_tls(listener, tls).await,
                None => acceptor.serve(listener).await,
            };
            served.unwrap();
        })
    }

    /// Node `node` as the standby, serving the active's link, requiring its client certificate
    /// over TLS. The replica is the operator's to promote.
    fn standby(&self, node: usize) -> (Process, SharedReplica) {
        let replica = Arc::new(Mutex::new(Replica::new(store(&self.dir, NODES[node]))));
        let tls = self
            .pki
            .as_ref()
            .map(|pki| server_tls(pki, ClientTrust::Required(Trust::from_pem_files(&pki.ca_path(true)).unwrap())));
        let listener = bound(self.link[node]);
        let serving = replica.clone();
        let process = Process::start(move || async move {
            let listener = TcpListener::from_std(listener).unwrap();
            let served = match tls {
                Some(tls) => serve_standby_tls(listener, tls, serving).await,
                None => serve_standby(listener, serving).await,
            };
            served.unwrap();
        });
        (process, replica)
    }
}

async fn lease(database: &str, node: usize) -> turbojet_sql::SqlLease {
    let mut config = SqlConfig::default();
    config.lease = LEASE;
    config.holder = NODES[node].into();
    config.sync = false;
    SqlStorage::connect(database, config).await.unwrap().lease("soak")
}

/// The pair as it runs: which node is active, both processes, the standby's replica, and the
/// proxies.
struct Nodes {
    active: usize,
    processes: [Process; 2],
    replica: Option<SharedReplica>,
    fix: [Proxy; 2],
    link: [Proxy; 2],
}

impl Nodes {
    fn standby(&self) -> usize {
        1 - self.active
    }

    /// Node `node`'s process crashes. Its sockets close with it, the link's too, which the
    /// standby serves on threads outside its runtime: the link proxy in front of the standby is
    /// cut, as the kernel would close them.
    fn crash(&mut self, node: usize) {
        self.processes[node].crash();
        self.link[1 - node].cut();
        self.link[node].cut();
    }

    /// The standby crashes, and once its replica is let go (its link threads ended), its store
    /// is free.
    async fn crash_standby(&mut self) {
        let standby = self.standby();
        self.crash(standby);
        let replica = self.replica.take().expect("a standby has a replica");
        let deadline = Instant::now() + Duration::from_secs(10);
        while Arc::strong_count(&replica) > 1 {
            assert!(Instant::now() < deadline, "the standby's link threads outlived it");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    fn heal(&mut self) {
        for proxy in self.fix.iter().chain(&self.link) {
            proxy.heal();
        }
    }

    fn clear_events(&mut self) {
        for proxy in self.fix.iter_mut().chain(&mut self.link) {
            proxy.clear_events();
        }
    }
}

fn soak_replicated(tls: bool, lasts: Duration) {
    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
    runtime.block_on(async {
        let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let mut rng = Rng(u64::try_from(seed % u128::from(u64::MAX)).unwrap());
        let seed = rng.0;
        let degrading = rng.next().is_multiple_of(2);
        eprintln!(
            "replicated soak over {}: {lasts:?} of faults, seed {seed}, {}degrading",
            if tls { "TLS" } else { "TCP" },
            if degrading { "" } else { "not " }
        );
        let port = || std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        let (fix, link) = ([port(), port()], [port(), port()]);
        // The proxies run on this runtime, apart from the processes.
        let fix_proxies =
            [Proxy::start(fix[0], Role::TjInitiator).await, Proxy::start(fix[1], Role::TjInitiator).await];
        let link_proxies =
            [Proxy::start(link[0], Role::TjInitiator).await, Proxy::start(link[1], Role::TjInitiator).await];
        let dir = tempfile::TempDir::with_prefix("turbojet-soak-").unwrap();
        let database = format!("sqlite://{}?mode=rwc", dir.path().join("lease.db").display());
        let mut config = SqlConfig::default();
        config.sync = false;
        SqlStorage::connect(&database, config).await.unwrap().migrate().await.unwrap();
        let pair = Pair {
            dir,
            fix,
            link,
            fix_proxies: [fix_proxies[0].addr(), fix_proxies[1].addr()],
            link_proxies: [link_proxies[0].addr(), link_proxies[1].addr()],
            pki: tls.then(Pki::new),
            seen: Shared::default(),
            database,
            degrading,
            cut_off: Default::default(),
        };
        let (standby, replica) = pair.standby(1);
        let mut nodes = Nodes {
            active: 0,
            processes: [pair.active(0), standby],
            replica: Some(replica),
            fix: fix_proxies,
            link: link_proxies,
        };
        let mut client_process =
            client(store(&pair.dir, "client"), &pair.fix_proxies, pair.pki.as_ref(), pair.seen.clone());

        run_replicated_faults(&pair, &mut nodes, &mut rng, lasts, &mut client_process).await;
        let [fix_a, fix_b] = &mut nodes.fix;
        let [link_a, link_b] = &mut nodes.link;
        settle(&mut [fix_a, fix_b, link_a, link_b], &pair.seen).await;
        let seen = lock(&pair.seen);
        check(&seen);
        assert!(seen.fenced.is_empty(), "replaced actives handed orders over: {:?}", seen.fenced);
        let marked = seen.orders.iter().filter(|(_, m)| *m).count();
        eprintln!(
            "{} orders stored, {} delivered ({marked} marked possibly handled), {} acknowledgements; seed {seed}",
            seen.stored.len(),
            seen.orders.len(),
            seen.acks.len()
        );
        assert!(seen.stored.len() > 1_000, "only {} orders went through", seen.stored.len());
        drop(seen);
        drop(client_process);
        drop(nodes);
    });
}

/// Faults, one at a time, until `lasts` is up, every kind has come and each kind of takeover has
/// been done at least once.
async fn run_replicated_faults(
    pair: &Pair,
    nodes: &mut Nodes,
    rng: &mut Rng,
    lasts: Duration,
    client_process: &mut Process,
) {
    let start = Instant::now();
    let mut faults: BTreeMap<&str, u64> = BTreeMap::new();
    let done =
        |faults: &BTreeMap<&str, u64>| REPLICATED_KINDS.iter().chain(&TAKEN_OVER).all(|kind| faults.contains_key(kind));
    while start.elapsed() < lasts || !done(&faults) {
        assert!(start.elapsed() < lasts + Duration::from_secs(600), "no takeover of each kind: {faults:?}");
        let kind = REPLICATED_KINDS[usize::try_from(rng.next() % REPLICATED_KINDS.len() as u64).unwrap()];
        *faults.entry(kind).or_default() += 1;
        let (dir, which) = if kind.ends_with("to server") {
            (Dir::ToPeer, kind.trim_end_matches(" to server"))
        } else if kind.ends_with(" link") {
            (if rng.next().is_multiple_of(2) { Dir::ToPeer } else { Dir::ToTj }, kind)
        } else {
            (Dir::ToTj, kind.trim_end_matches(" to client"))
        };
        let lasts = rng.millis(200, 3_000);
        let (active, standby) = (nodes.active, nodes.standby());
        match which {
            "cut" => nodes.fix[active].cut(),
            "blackhole" => nodes.fix[active].blackhole(dir),
            "stall" => nodes.fix[active].stall(dir),
            "hold" => nodes.fix[active].hold(dir),
            "slow" => nodes.fix[active].bandwidth(dir, u32::try_from(rng.next() % 20_000).unwrap() + 1_000),
            "cut link" => nodes.link[standby].cut(),
            "blackhole link" => nodes.link[standby].blackhole(dir),
            "stall link" => nodes.link[standby].stall(dir),
            "slow link" => nodes.link[standby].bandwidth(dir, u32::try_from(rng.next() % 20_000).unwrap() + 1_000),
            "crash client" => client_process.crash(),
            "crash active" => nodes.crash(active),
            "crash standby" => nodes.crash_standby().await,
            "fail over" | "depose" => {
                let outcome = take_over(pair, nodes, which == "depose", lasts).await;
                *faults.entry(outcome).or_default() += 1;
            }
            other => unreachable!("{other}"),
        }
        if !matches!(which, "fail over" | "depose") {
            tokio::time::sleep(lasts).await;
        }
        match which {
            "crash client" => {
                *client_process =
                    client(store(&pair.dir, "client"), &pair.fix_proxies, pair.pki.as_ref(), pair.seen.clone())
            }
            "crash active" => nodes.processes[active] = pair.active(active),
            "crash standby" => {
                let (process, replica) = pair.standby(standby);
                nodes.processes[standby] = process;
                nodes.replica = Some(replica);
            }
            _ => nodes.heal(),
        }
        nodes.clear_events();
        tokio::time::sleep(rng.millis(200, 1_000)).await;
    }
    nodes.heal();
    lock(&pair.seen).stopped = true;
    eprintln!("faults: {faults:?}");
}

/// An operator's takeover, by the documented sequence, from an active that crashed or, with
/// `depose`, one still running but cut off from the lease's database and the standby, though the
/// client may still reach it: take the lease (once the active's has run out), read whether the standby is in sync under it, promote the standby, and start the
/// active over its store. The old active, back after `lasts` (if it crashed) or stopped then (if
/// it ran on, refused), becomes the standby. If the standby can't take over, the operator gives
/// the lease back and restarts the old active. Returns what it came to.
async fn take_over(pair: &Pair, nodes: &mut Nodes, depose: bool, lasts: Duration) -> &'static str {
    let (old, new) = (nodes.active, nodes.standby());
    if depose {
        pair.cut_off[old].store(true, Ordering::Relaxed);
        nodes.link[new].blackhole(Dir::ToPeer);
        nodes.link[new].blackhole(Dir::ToTj);
    } else {
        nodes.crash(old);
    }
    let lease = pair.lease(new).await;
    let taking = Instant::now();
    let epoch = loop {
        match lease.acquire().run_async().await {
            Ok(epoch) => break Some(epoch),
            Err(_) if taking.elapsed() < TAKE_LEASE => tokio::time::sleep(Duration::from_millis(100)).await,
            Err(_) => break None,
        }
    };
    let refused = match epoch {
        None => Some("takeover refused: lease held"),
        Some(epoch) => {
            let refused = match lease.in_sync(epoch).run_async().await {
                Ok(true) => {
                    let replica = nodes.replica.as_ref().expect("a standby has a replica");
                    let promoted = replica.lock().unwrap_or_else(PoisonError::into_inner).promote();
                    promoted.err().map(|_| "takeover refused: standby unchecked")
                }
                Ok(false) => Some("takeover refused: standby out of sync"),
                Err(_) => Some("takeover refused: in sync unread"),
            };
            if refused.is_some() {
                lease.release(epoch).run_async().await.unwrap();
            }
            refused
        }
    };
    if let Some(refused) = refused {
        // The old active can no longer renew a lease taken from it: it starts again, and takes it.
        nodes.processes[old].crash();
        pair.cut_off[old].store(false, Ordering::Relaxed);
        nodes.heal();
        tokio::time::sleep(lasts).await;
        nodes.processes[old] = pair.active(old);
        return refused;
    }
    if depose {
        lock(&pair.seen).deposed = Some((old, Instant::now()));
    }
    nodes.crash_standby().await;
    nodes.processes[new] = pair.active(new);
    nodes.active = new;
    // The old active, if it runs on, can reach the client still, but neither the lease nor its
    // standby, now the active: its commits fail.
    tokio::time::sleep(lasts).await;
    if depose {
        nodes.crash(old);
        pair.cut_off[old].store(false, Ordering::Relaxed);
        nodes.heal();
        lock(&pair.seen).deposed = None;
    }
    let (process, replica) = pair.standby(old);
    nodes.processes[old] = process;
    nodes.replica = Some(replica);
    if depose { "deposed" } else { "failed over" }
}
