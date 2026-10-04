//! A FIX client built on turbojet's `Initiator`: logs on to the gateway, submits a limit
//! order, cancels it and logs out.
//!
//! Start the gateway with `cargo run --example gateway --all-features -- --allow CLIENT1`, then run
//! `cargo run --example client --features tls -- [ADDR]`.
//!
//! Add `--failover ADDR` (repeatable) for backup gateways tried in order if the primary fails.
//! For TLS, add `--tls-ca CA.pem` (and `--tls-server-name NAME` if it differs from `localhost`);
//! for mutual TLS also `--tls-cert CLIENT.pem --tls-key CLIENT.key`.

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use turbojet::fields::UtcTimestamp;
use turbojet::tls;
use turbojet::{
    Application, Context, Disconnect, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_fix42::{
    ExecutionReportRef, HandlInst, NewOrderSingle, OrdType, OrderCancelRejectRef, OrderCancelRequest, Side, TimeInForce,
};

enum Event {
    LoggedOn,
    LoggedOut,
    Message(Message),
}

/// Forwards engine callbacks to `main` over a channel.
struct ClientApp {
    events: mpsc::UnboundedSender<Event>,
}

impl Application for ClientApp {
    fn on_logon(&self, _session: &SessionHandle) {
        let _ = self.events.send(Event::LoggedOn);
    }

    fn on_logout(&self, _session: &SessionHandle, _ended: Disconnect) {
        let _ = self.events.send(Event::LoggedOut);
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.events.send(Event::Message(msg.clone()));
        Ok(())
    }
}

async fn next(events: &mut mpsc::UnboundedReceiver<Event>) -> Result<Event, Box<dyn Error>> {
    match tokio::time::timeout(Duration::from_secs(5), events.recv()).await {
        Ok(Some(event)) => Ok(event),
        Ok(None) => Err("session ended".into()),
        Err(_) => Err("timed out waiting for the gateway".into()),
    }
}

/// Waits for the next application message and prints it in typed form.
async fn reply(events: &mut mpsc::UnboundedReceiver<Event>) -> Result<(), Box<dyn Error>> {
    let Event::Message(msg) = next(events).await? else {
        return Err("session ended before the reply arrived".into());
    };
    match msg.msg_type() {
        MsgType::ExecutionReport => {
            let report: ExecutionReportRef = msg.parse()?;
            println!(
                "<- ExecutionReport order={} exec_type={:?} status={:?} leaves={} text={:?}",
                report.order_id, report.exec_type, report.ord_status, report.leaves_qty, report.text
            );
        }
        MsgType::OrderCancelReject => {
            let reject: OrderCancelRejectRef = msg.parse()?;
            println!("<- OrderCancelReject reason={:?} text={:?}", reject.cxl_rej_reason, reject.text);
        }
        _ => println!("<- {msg}"),
    }
    Ok(())
}

fn send(session: &SessionHandle, msg: impl Into<Message>) -> Result<(), Box<dyn Error>> {
    let msg = msg.into();
    println!("-> {msg}");
    // The receipt (the MsgSeqNum once stored, or why it was dropped) isn't needed here.
    session.send(msg)?;
    Ok(())
}

#[derive(Default)]
struct Args {
    addr: Option<String>,
    failover: Vec<String>,
    tls_ca: Option<PathBuf>,
    tls_server_name: Option<String>,
    tls_cert: Option<PathBuf>,
    tls_key: Option<PathBuf>,
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut parsed = Args::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} requires a value"));
        match arg.as_str() {
            "--failover" => parsed.failover.push(value()?),
            "--tls-ca" => parsed.tls_ca = Some(value()?.into()),
            "--tls-server-name" => parsed.tls_server_name = Some(value()?),
            "--tls-cert" => parsed.tls_cert = Some(value()?.into()),
            "--tls-key" => parsed.tls_key = Some(value()?.into()),
            addr if !addr.starts_with('-') && parsed.addr.is_none() => parsed.addr = Some(addr.into()),
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }
    Ok(parsed)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let addr = args.addr.unwrap_or_else(|| "127.0.0.1:9876".into());
    let (tx, mut events) = mpsc::unbounded_channel();

    let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT1"), "GATEWAY");
    config.reset_on_logon = true;
    let mut initiator =
        Initiator::new(addr, config, Arc::new(MemoryStorage::new()), Arc::new(ClientApp { events: tx }))?;
    for backup in args.failover {
        initiator = initiator.with_failover(backup);
    }
    if let Some(ca) = &args.tls_ca {
        let identity = match (&args.tls_cert, &args.tls_key) {
            (Some(cert), Some(key)) => Some((cert.as_path(), key.as_path())),
            (None, None) => None,
            _ => return Err("--tls-cert and --tls-key must be given together".into()),
        };
        let server_name = args.tls_server_name.as_deref().unwrap_or("localhost");
        initiator = initiator.with_tls(tls::connector(ca, identity)?, server_name)?;
    }
    let session = initiator.handle();

    let mut connection = tokio::spawn({
        let initiator = initiator.clone();
        async move { initiator.connect_once().await }
    });
    tokio::select! {
        event = next(&mut events) => match event? {
            Event::LoggedOn => println!("logged on as {}", session.id()),
            _ => return Err("logon failed".into()),
        },
        result = &mut connection => return Err(format!("connection ended before logon: {:?}", result?).into()),
    }

    let mut order = NewOrderSingle::new(
        "ORD1",
        HandlInst::AutomatedExecutionNoIntervention,
        "AAPL",
        Side::Buy,
        UtcTimestamp::now(),
        OrdType::Limit,
    );
    order.order_qty = Some("100".parse()?);
    order.price = Some("150.25".parse()?);
    order.time_in_force = Some(TimeInForce::Day);
    send(&session, order)?;
    reply(&mut events).await?;

    let (orig_cl_ord_id, cl_ord_id) = ("ORD1", "CXL1");
    send(&session, OrderCancelRequest::new(orig_cl_ord_id, cl_ord_id, "AAPL", Side::Buy, UtcTimestamp::now()))?;
    reply(&mut events).await?;

    session.logout(None)?;
    if let Event::LoggedOut = next(&mut events).await? {
        println!("logged out");
    }
    connection.await??;
    Ok(())
}
