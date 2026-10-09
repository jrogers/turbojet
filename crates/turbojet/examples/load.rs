//! A load generator: opens sessions to an acceptor, sends NewOrderSingles at a set rate across
//! them, and reports how long each took to be answered with an ExecutionReport.
//!
//! Start the gateway with `cargo run --release --example gateway --all-features -- --allow-any`,
//! then run `cargo run --release --example load -- [ADDR] [OPTIONS]` (see `--help`).
//!
//! The load is open loop: order k is due at a fixed time, `start + k / rate`, whatever happened to
//! the orders before it, and its latency runs from when it was due, not when it was sent. A stall
//! in the client or the acceptor then counts against every order that queued behind it, as it
//! would for a trading system sending at that rate; a tester that waits for each answer before
//! sending the next hides it ("coordinated omission").

use std::error::Error;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime};

use tokio::sync::mpsc;
use turbojet::fields::UtcTimestamp;
use turbojet::message::tags;
use turbojet::{
    Application, Context, Disconnect, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};

const USAGE: &str = "\
usage: load [ADDR] [OPTIONS]

Sends NewOrderSingles to an acceptor (default 127.0.0.1:9876) at a set rate and reports the
latency of their ExecutionReports, from when each order was due.

  --sessions N        sessions to open, as LOAD1 to LOADN (default 1)
  --rate R            orders per second, across all sessions (default 1000)
  --duration S        seconds of measured load (default 10)
  --warmup S          seconds of load before that, not measured (default 2)
  --target ID         the acceptor's CompID (default GATEWAY)
  --begin-string V    the FIX version (default FIX.4.2)";

/// The most latencies kept, 160 MB of them: a run that would measure more is refused.
const SAMPLES_MAX: u64 = 20_000_000;
/// How long to wait for the last orders' answers once all have been sent.
const ANSWER_WAIT: Duration = Duration::from_secs(5);
/// How long to wait for every session to log on.
const LOGON_WAIT: Duration = Duration::from_secs(10);
/// The sender sleeps until an order is this close to due, then spins: a sleep can overshoot by
/// about a millisecond.
const SPIN_FROM: Duration = Duration::from_millis(2);

#[derive(Debug, PartialEq)]
struct Args {
    addr: String,
    sessions: u32,
    rate: u64,
    duration: Duration,
    warmup: Duration,
    target: String,
    begin_string: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:9876".into(),
            sessions: 1,
            rate: 1000,
            duration: Duration::from_secs(10),
            warmup: Duration::from_secs(2),
            target: "GATEWAY".into(),
            begin_string: "FIX.4.2".into(),
        }
    }
}

impl Args {
    /// The arguments, `None` if they ask for help.
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Option<Self>, Box<dyn Error>> {
        let mut parsed = Args::default();
        let mut addr = None;
        while let Some(arg) = args.next() {
            let mut value = || args.next().ok_or(format!("{arg} requires a value"));
            match arg.as_str() {
                "--sessions" => parsed.sessions = value()?.parse()?,
                "--rate" => parsed.rate = value()?.parse()?,
                "--duration" => parsed.duration = Duration::from_secs(value()?.parse()?),
                "--warmup" => parsed.warmup = Duration::from_secs(value()?.parse()?),
                "--target" => parsed.target = value()?,
                "--begin-string" => parsed.begin_string = value()?,
                "-h" | "--help" => return Ok(None),
                other if !other.starts_with('-') && addr.is_none() => addr = Some(other.to_owned()),
                other => return Err(format!("unknown argument '{other}'").into()),
            }
        }
        parsed.addr = addr.unwrap_or(parsed.addr);
        if parsed.sessions == 0 || parsed.rate == 0 || parsed.duration.is_zero() {
            return Err("--sessions, --rate and --duration must be above 0".into());
        }
        if parsed.measured() > SAMPLES_MAX {
            return Err(format!("{} orders would be measured: at most {SAMPLES_MAX}", parsed.measured()).into());
        }
        Ok(Some(parsed))
    }

    /// The orders sent while warming up, which aren't measured.
    fn warmup_orders(&self) -> u64 {
        self.rate * self.warmup.as_secs()
    }

    /// The orders measured.
    fn measured(&self) -> u64 {
        self.rate * self.duration.as_secs()
    }
}

/// When each order is due: order `k`, counted across all sessions from 0, at
/// `start + k * period`.
#[derive(Debug, Clone, Copy)]
struct Schedule {
    start: Instant,
    period: Duration,
}

impl Schedule {
    fn new(start: Instant, rate: u64) -> Self {
        Self { start, period: Duration::from_nanos(1_000_000_000 / rate) }
    }

    fn due(&self, k: u64) -> Instant {
        // At most SAMPLES_MAX plus the warm-up orders, each a period apart: far from overflowing.
        let k = u32::try_from(k).expect("orders are counted in u32 range");
        self.start + self.period * k
    }
}

/// Records each ExecutionReport's latency, from when its order was due.
struct LoadApp {
    /// The ClOrdID prefix of this run's orders, so that an acceptor that remembers ClOrdIDs
    /// doesn't reject a second run's as duplicates.
    run: String,
    schedule: OnceLock<Schedule>,
    warmup_orders: u64,
    // ponytail: one lock across all sessions' reports; per-session vectors if it shows up at high rates.
    latencies: Mutex<Vec<u64>>,
    answered: AtomicU64,
    logons: mpsc::UnboundedSender<()>,
}

impl Application for LoadApp {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.logons.send(());
    }

    fn on_logout(&self, session: &SessionHandle, ended: Disconnect) {
        if ended != Disconnect::Logout {
            eprintln!("{session} ended: {ended:?}");
        }
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let now = Instant::now();
        let order = msg.get(tags::CL_ORD_ID).and_then(|id| id.strip_prefix(&self.run)).and_then(|k| k.parse().ok());
        let (Some(k), Some(schedule)) = (order, self.schedule.get()) else { return Ok(()) };
        if msg.msg_type() != MsgType::ExecutionReport {
            return Ok(());
        }
        self.answered.fetch_add(1, Ordering::Relaxed);
        if k >= self.warmup_orders {
            let latency = now.saturating_duration_since(schedule.due(k));
            let nanos = u64::try_from(latency.as_nanos()).unwrap_or(u64::MAX);
            self.latencies.lock().expect("latencies lock poisoned").push(nanos);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Some(args) = Args::parse(std::env::args().skip(1))? else {
        println!("{USAGE}");
        return Ok(());
    };
    let (logons, mut logged_on) = mpsc::unbounded_channel();
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let app = Arc::new(LoadApp {
        run: format!("{}-", since_epoch.as_secs()),
        schedule: OnceLock::new(),
        warmup_orders: args.warmup_orders(),
        latencies: Mutex::new(Vec::with_capacity(usize::try_from(args.measured())?)),
        answered: AtomicU64::new(0),
        logons,
    });
    let sessions = connect(&args, &app)?;
    for _ in 0..args.sessions {
        tokio::time::timeout(LOGON_WAIT, logged_on.recv()).await?.ok_or("logon failed")?;
    }
    let plural = if args.sessions == 1 { "" } else { "s" };
    println!("{} session{plural} logged on to {}", args.sessions, args.addr);

    let total = args.warmup_orders() + args.measured();
    let schedule = Schedule::new(Instant::now() + Duration::from_millis(100), args.rate);
    app.schedule.set(schedule).expect("set once");
    let run = app.run.clone();
    let handles = sessions.clone();
    let measured = args.warmup_orders()..total;
    let sender = tokio::task::spawn_blocking(move || send_all(&handles, schedule, measured, &run));
    let (lag_max, unsent) = sender.await?;

    let waited_from = Instant::now();
    while app.answered.load(Ordering::Relaxed) < total - unsent && waited_from.elapsed() < ANSWER_WAIT {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let elapsed = schedule.due(total).saturating_duration_since(schedule.start);
    let mut latencies = std::mem::take(&mut *app.latencies.lock().expect("latencies lock poisoned"));
    for session in &sessions {
        let _ = session.logout(None);
    }
    report(&args, total, unsent, app.answered.load(Ordering::Relaxed), elapsed, lag_max, &mut latencies);
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}

/// Starts a session to the acceptor for each of LOAD1 to LOADN, returning their handles.
fn connect(args: &Args, app: &Arc<LoadApp>) -> Result<Vec<SessionHandle>, Box<dyn Error>> {
    let storage = Arc::new(MemoryStorage::new());
    let mut sessions = Vec::new();
    for i in 1..=args.sessions {
        let mut config =
            InitiatorConfig::new(SessionConfig::new(args.begin_string.clone(), format!("LOAD{i}")), &args.target);
        config.reset_on_logon = true;
        let initiator = Initiator::new(args.addr.clone(), config, storage.clone(), app.clone())?;
        sessions.push(initiator.handle());
        tokio::spawn(async move {
            if let Err(e) = initiator.connect_once().await {
                eprintln!("connection ended: {e}");
            }
        });
    }
    Ok(sessions)
}

/// Sends each order when it's due, round the sessions, on this thread. Returns how far behind
/// schedule the sender fell at worst after the warm-up (the first sleep can overshoot by a few
/// milliseconds), and how many orders couldn't be sent (a session's queue full, or the session
/// gone).
fn send_all(sessions: &[SessionHandle], schedule: Schedule, orders: Range<u64>, run: &str) -> (Duration, u64) {
    let (mut lag_max, mut unsent) = (Duration::ZERO, 0);
    let mut session = sessions.iter().cycle();
    for k in 0..orders.end {
        let due = schedule.due(k);
        wait_until(due);
        if orders.contains(&k) {
            lag_max = lag_max.max(Instant::now() - due);
        }
        let order = Message::new(MsgType::NewOrderSingle)
            .with(tags::CL_ORD_ID, format!("{run}{k}"))
            .with(21, "1")
            .with(55, "AAPL")
            .with(54, "1")
            .with(60, UtcTimestamp::now())
            .with(38, "100")
            .with(40, "2")
            .with(44, "150.25");
        if session.next().expect("cycles for ever").send(order).is_err() {
            unsent += 1;
        }
    }
    (lag_max, unsent)
}

fn wait_until(due: Instant) {
    loop {
        let now = Instant::now();
        if now >= due {
            return;
        }
        if due - now > SPIN_FROM {
            std::thread::sleep(due - now - SPIN_FROM);
        } else {
            std::hint::spin_loop();
        }
    }
}

fn report(
    args: &Args,
    total: u64,
    unsent: u64,
    answered: u64,
    elapsed: Duration,
    lag_max: Duration,
    latencies: &mut [u64],
) {
    let sent = total - unsent;
    #[expect(clippy::cast_precision_loss, reason = "a rate to print")]
    let rate = sent as f64 / elapsed.as_secs_f64();
    println!("sent {sent} orders ({unsent} not sent) in {:.2} s: {rate:.0}/s", elapsed.as_secs_f64());
    println!("answered {answered}; {} unanswered after {} s", sent.saturating_sub(answered), ANSWER_WAIT.as_secs());
    latencies.sort_unstable();
    println!("latency from due time, {} orders after {} s of warm-up:", latencies.len(), args.warmup.as_secs());
    for (name, quantile) in [("p50", 0.5), ("p90", 0.9), ("p99", 0.99), ("p99.9", 0.999), ("p99.99", 0.9999)] {
        if let Some(nanos) = percentile(latencies, quantile) {
            println!("  {name:<7}{}", micros(nanos));
        }
    }
    if let Some(&max) = latencies.last() {
        println!("  {:<7}{}", "max", micros(max));
    }
    println!(
        "the sender fell behind schedule by at most {}",
        micros(u64::try_from(lag_max.as_nanos()).unwrap_or(u64::MAX))
    );
}

/// The nearest-rank percentile of `sorted`: the smallest value at least `quantile` of them are at
/// or below. `None` if there are none.
fn percentile(sorted: &[u64], quantile: f64) -> Option<u64> {
    #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a rank")]
    let rank = (quantile * sorted.len() as f64).ceil() as usize;
    sorted.get(rank.max(1) - 1).copied()
}

fn micros(nanos: u64) -> String {
    #[expect(clippy::cast_precision_loss, reason = "a time to print")]
    let micros = nanos as f64 / 1000.0;
    format!("{micros:.1} µs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles_are_nearest_rank() {
        let sorted: Vec<u64> = (1..=1000).collect();
        assert_eq!(percentile(&sorted, 0.5), Some(500));
        assert_eq!(percentile(&sorted, 0.99), Some(990));
        assert_eq!(percentile(&sorted, 0.9999), Some(1000));
        assert_eq!(percentile(&[7], 0.5), Some(7));
        assert_eq!(percentile(&[], 0.5), None);
    }

    #[test]
    fn orders_are_due_a_period_apart_from_the_start() {
        let start = Instant::now();
        let schedule = Schedule::new(start, 4000);
        assert_eq!(schedule.due(0), start);
        assert_eq!(schedule.due(4000), start + Duration::from_secs(1));
    }

    #[test]
    fn a_run_too_long_to_measure_is_refused() {
        let args = |list: &[&str]| Args::parse(list.iter().map(|s| (*s).to_owned()));
        assert!(args(&["--rate", "1000000", "--duration", "60"]).is_err());
        let parsed = args(&["10.0.0.1:9876", "--rate", "5000", "--sessions", "4"]).unwrap().unwrap();
        assert_eq!((parsed.addr.as_str(), parsed.rate, parsed.sessions), ("10.0.0.1:9876", 5000, 4));
        assert_eq!(parsed.warmup_orders(), 10_000);
    }
}
