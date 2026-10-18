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
use turbojet::fields::{Decimal, UtcTimestamp, format_compact};
use turbojet::{
    Acceptor, Application, Context, Disconnect, MemoryStorage, Message, MessageReject, MsgType, SessionConfig,
    SessionHandle,
};
use turbojet_fix::fix42::{ExecTransType, ExecType, ExecutionReport, NewOrderSingleRef, OrdStatus};

const USAGE: &str = "\
usage: ack [OPTIONS]

A FIX 4.2 acceptor that answers each NewOrderSingle with an ExecutionReport and keeps nothing.

  --listen ADDR       where to listen (default 127.0.0.1:9876)
  --comp-id ID        our CompID (default GATEWAY)";

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
            ctx.send(ack(msg.parse()?, self.next_id.fetch_add(1, Ordering::Relaxed)));
        }
        Ok(())
    }
}

/// The ExecutionReport accepting `order`, as order `id`.
fn ack(order: NewOrderSingleRef<'_>, id: u64) -> ExecutionReport {
    let leaves_qty = order.order_qty.unwrap_or_default();
    let mut report = ExecutionReport::new(
        format_compact!("O{id}"),
        format_compact!("E{id}"),
        ExecTransType::New,
        ExecType::New,
        OrdStatus::New,
        order.symbol,
        order.side,
        leaves_qty,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    report.cl_ord_id = Some(order.cl_ord_id.into());
    report.account = order.account.map(Into::into);
    report.order_qty = order.order_qty;
    report.ord_type = Some(order.ord_type.into());
    report.price = order.price;
    report.time_in_force = order.time_in_force.map(Into::into);
    report.transact_time = Some(UtcTimestamp::now());
    report
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut listen, mut comp_id) = ("127.0.0.1:9876".to_owned(), "GATEWAY".to_owned());
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} requires a value"));
        match arg.as_str() {
            "--listen" => listen = value()?,
            "--comp-id" => comp_id = value()?,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }
    let acceptor = Acceptor::new(
        SessionConfig::new("FIX.4.2", comp_id.as_str()),
        Arc::new(MemoryStorage::new()),
        Arc::new(Acker::default()),
    )?
    .with_max_connections_per_ip(CONNECTIONS_PER_IP_MAX);
    let listener = TcpListener::bind(&listen).await?;
    eprintln!("{comp_id} listening on {}", listener.local_addr()?);
    tokio::select! {
        result = acceptor.serve(listener) => result?,
        result = tokio::signal::ctrl_c() => result?,
    }
    Ok(())
}
