//! An acceptor for benchmarks: answers each NewOrderSingle with an ExecutionReport and keeps
//! nothing, so `load` measures the engine's round trip rather than an application's bookkeeping.
//!
//! Run `cargo run --release --example ack`, then `cargo run --release --example load` (see
//! `--help` for each). Any CompID may log on, FIX 4.2 only. Each order is parsed in its borrowed
//! typed form and answered with a typed ExecutionReport, as a real application would, but nothing
//! is booked: a cancel or a repeated ClOrdID isn't checked, and nothing is logged per order. The
//! gateway example is the one that keeps orders, up to a million, rejecting new ones after that.

use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::net::TcpListener;
use turbojet::connection::SpinningStream;
use turbojet::fields::{Decimal, UtcTimestamp, format_compact};
use turbojet::{
    Acceptor, AcceptorConfig, Application, ConnectionInfo, Context, Disconnect, FieldError, MemoryStorage, Message,
    MessageReject, MsgType, SessionConfig, SessionHandle,
};
use turbojet_fix::fix42::{
    ExecTransType, ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingleRef, OrdStatus,
};

const USAGE: &str = "\
usage: ack [OPTIONS]

A FIX 4.2 acceptor that answers each NewOrderSingle with an ExecutionReport and keeps nothing.

  --listen ADDR       where to listen (default 127.0.0.1:9876)
  --comp-id ID        our CompID (default GATEWAY)
  --spin              drive each connection on a thread of its own, named spin-N, that polls its
                      socket without waiting (accept_spinning), rather than on the tokio runtime
  --metrics-listen ADDR
                      serve Prometheus metrics at http://ADDR/metrics, the latency histograms
                      included (built with --features metrics)
  --rx-timestamps     with --spin, on Linux: timestamp each received packet, by the network card if
                      it can, recorded as turbojet_wire_to_read_seconds";

/// The most connections from one address: `load` opens all its sessions from one.
const CONNECTIONS_PER_IP_MAX: usize = 1024;

/// Answers each NewOrderSingle; other application messages are ignored.
#[derive(Default)]
struct Acker {
    next_id: AtomicU64,
}

impl Application for Acker {
    fn on_logon(&self, session: &SessionHandle) {
        eprintln!("{session} logged on");
    }

    fn on_logout(&self, session: &SessionHandle, ended: Disconnect) {
        eprintln!("{session} ended: {ended:?}");
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        if msg.msg_type() == MsgType::NewOrderSingle {
            ctx.send(ack(msg.parse()?, self.next_id.fetch_add(1, Ordering::Relaxed))?)?;
        }
        Ok(())
    }
}

/// The ExecutionReport accepting `order`, as order `id`, or the field that doesn't convert.
fn ack(order: NewOrderSingleRef<'_>, id: u64) -> Result<ExecutionReport, FieldError> {
    let leaves_qty = order.order_qty()?.unwrap_or_default();
    let mut report = ExecutionReport::new(ExecutionReportRequired {
        order_id: format_compact!("O{id}"),
        exec_id: format_compact!("E{id}"),
        exec_trans_type: ExecTransType::New.into(),
        exec_type: ExecType::New.into(),
        ord_status: OrdStatus::New.into(),
        symbol: order.symbol()?.into(),
        side: order.side()?.into(),
        leaves_qty,
        cum_qty: Decimal::ZERO,
        avg_px: Decimal::ZERO,
    });
    report.cl_ord_id = Some(order.cl_ord_id()?.into());
    report.account = order.account()?.map(Into::into);
    report.order_qty = order.order_qty()?;
    report.ord_type = Some(order.ord_type()?.into());
    report.price = order.price()?;
    report.time_in_force = order.time_in_force()?.map(Into::into);
    report.transact_time = Some(UtcTimestamp::now());
    Ok(report)
}

/// Accepts each connection onto a spinning thread of its own. Unlike [`Acceptor::serve`], it doesn't
/// limit connections per address.
async fn serve_spinning(acceptor: Acceptor, listener: TcpListener, rx_timestamps: bool) -> std::io::Result<()> {
    let runtime = tokio::runtime::Handle::current();
    for n in 1.. {
        let (socket, _) = listener.accept().await?;
        let socket = SpinningStream::new(socket.into_std()?)?;
        let socket = if rx_timestamps { receive_timestamps(socket)? } else { socket };
        let (acceptor, runtime) = (acceptor.clone(), runtime.clone());
        std::thread::Builder::new().name(format!("spin-{n}")).spawn(move || {
            if let Err(e) = acceptor.accept_spinning(socket, ConnectionInfo::default(), &runtime) {
                eprintln!("connection ended: {e}");
            }
        })?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn receive_timestamps(socket: SpinningStream) -> std::io::Result<SpinningStream> {
    socket.with_receive_timestamps()
}

#[cfg(not(target_os = "linux"))]
fn receive_timestamps(_socket: SpinningStream) -> std::io::Result<SpinningStream> {
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "receive timestamps are Linux only"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut listen, mut comp_id, mut spin) = ("127.0.0.1:9876".to_owned(), "GATEWAY".to_owned(), false);
    let (mut metrics_listen, mut rx_timestamps) = (None, false);
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} requires a value"));
        match arg.as_str() {
            "--listen" => listen = value()?,
            "--comp-id" => comp_id = value()?,
            "--spin" => spin = true,
            "--metrics-listen" => metrics_listen = Some(value()?),
            "--rx-timestamps" => rx_timestamps = true,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }
    if rx_timestamps && !spin {
        return Err("--rx-timestamps needs --spin".into());
    }
    #[allow(unused_mut, reason = "set only with the metrics feature")]
    let mut session = SessionConfig::new("FIX.4.2", comp_id.as_str());
    if let Some(addr) = metrics_listen {
        #[cfg(feature = "metrics")]
        {
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .with_http_listener(addr.parse::<std::net::SocketAddr>()?)
                .install()?;
            turbojet::describe_metrics();
            session.latency_metrics = true;
        }
        #[cfg(not(feature = "metrics"))]
        return Err(format!("--metrics-listen {addr} needs ack built with --features metrics").into());
    }
    let mut config = AcceptorConfig::new(session);
    config.max_connections_per_ip = CONNECTIONS_PER_IP_MAX;
    let acceptor = Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(Acker::default()))?;
    let listener = TcpListener::bind(&listen).await?;
    eprintln!("{comp_id} listening on {}", listener.local_addr()?);
    let served = if spin {
        tokio::spawn(serve_spinning(acceptor, listener, rx_timestamps))
    } else {
        tokio::spawn(acceptor.serve(listener))
    };
    tokio::select! {
        result = served => result??,
        result = tokio::signal::ctrl_c() => result?,
    }
    Ok(())
}
