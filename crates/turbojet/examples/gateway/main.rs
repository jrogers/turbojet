//! A FIX 4.2 order-entry gateway: an [`Acceptor`] serving [`GatewayApp`], an
//! [`Application`](turbojet::Application) that validates, books and cancels orders.
//!
//! `cargo run --example gateway --all-features -- --help`

mod app;
mod orders;
#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::net::TcpListener;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;
use turbojet::{
    Acceptor, DiskStorage, MemoryStorage, SequenceError, SessionConfig, SessionId, SessionRegistry, SessionStorage, tls,
};

use app::GatewayApp;
use orders::OrderManager;

const USAGE: &str = "\
Usage: gateway [OPTIONS]

Options:
  --listen ADDR        Address to listen on (default 0.0.0.0:9876)
  --comp-id ID         Our SenderCompID (default GATEWAY)
  --allow ID[,ID...]   Accept logons only from these counterparty CompIDs (required, unless
                       --allow-any)
  --allow-any          Accept logons from any CompID. Each one gets its own stored state, so
                       only use this where the network, or mutual TLS, limits who can connect
  --store-dir DIR      Persist session state in DIR (default: in memory, lost on restart)
  --fsync              fsync every store write (survives power loss; slower)
  --tls-cert FILE      Serve TLS with this PEM certificate chain (requires --tls-key)
  --tls-key FILE       PEM private key for --tls-cert
  --tls-client-ca FILE Verify client certificates against these PEM CAs (mutual TLS)
  --tls-client-auth M  With --tls-client-ca: `required` (default) refuses clients without a
                       certificate; `optional` admits them, but still refuses invalid ones
  --tls-match-comp-id  With --tls-client-ca: a client certificate must name the SenderCompID
                       (as its subject CN or a DNS name) that logs on with it
  --schedule S         Only allow sessions in these hours, resetting sequence numbers each
                       period, e.g. \"daily 08:00-17:00 mon-fri America/New_York\" or
                       \"weekly sun 17:00-fri 17:00 America/New_York\" (default: always open)
  --metrics-listen A   Serve Prometheus metrics at http://A/metrics
  --log-format F       `text` (default) or `json`
  -h, --help           Show this help

Logging is controlled by RUST_LOG (default `info`). To log every FIX message:
  RUST_LOG=info,turbojet::messages=debug

To inspect or change a disconnected session's stored sequence numbers, see
`gateway seqnums --help`.";

const SEQNUMS_USAGE: &str = "\
Usage: gateway seqnums --store-dir DIR --session COMP_ID [OPTIONS]

Shows, and optionally changes, a session's stored sequence numbers. Works while the gateway is
running, for a session that isn't connected (a connected session's store is locked).

Options:
  --store-dir DIR          The gateway's --store-dir
  --session COMP_ID        The counterparty's CompID
  --comp-id ID             Our SenderCompID (default GATEWAY)
  --reset                  Reset both to 1 and clear the resend store
  --set-next-incoming N    The next MsgSeqNum to expect from the counterparty
  --set-next-outgoing N    The next MsgSeqNum to send (can only move forward)
  -h, --help               Show this help

Changes are applied in the order: reset, incoming, outgoing.";

struct Args {
    listen: String,
    config: SessionConfig,
    allowed: Option<HashSet<String>>,
    store_dir: Option<PathBuf>,
    fsync: bool,
    tls: Option<TlsArgs>,
    tls_match_comp_id: bool,
    metrics_listen: Option<SocketAddr>,
    json_logs: bool,
}

struct TlsArgs {
    cert: PathBuf,
    key: PathBuf,
    client_ca: Option<PathBuf>,
    client_cert_required: bool,
}

/// Parses the gateway's arguments (without the program name).
#[expect(clippy::too_many_lines, reason = "one match arm per option")]
fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut listen = "0.0.0.0:9876".to_string();
    let mut config = SessionConfig::new("FIX.4.2", "GATEWAY");
    let mut allowed = None;
    let mut allow_any = false;
    let mut store_dir = None;
    let mut fsync = false;
    let (mut tls_cert, mut tls_key, mut tls_client_ca, mut tls_client_auth) = (None, None, None, None);
    let mut tls_match_comp_id = false;
    let mut metrics_listen = None;
    let mut json_logs = false;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("{arg} requires a value"));
        match arg.as_str() {
            "--listen" => listen = value()?,
            "--comp-id" => config.sender_comp_id = value()?,
            "--allow" => {
                let ids: HashSet<String> =
                    value()?.split(',').map(str::trim).filter(|s| !s.is_empty()).map(String::from).collect();
                allowed = Some(ids);
            }
            "--allow-any" => allow_any = true,
            "--store-dir" => store_dir = Some(PathBuf::from(value()?)),
            "--fsync" => fsync = true,
            "--tls-cert" => tls_cert = Some(PathBuf::from(value()?)),
            "--tls-key" => tls_key = Some(PathBuf::from(value()?)),
            "--tls-client-ca" => tls_client_ca = Some(PathBuf::from(value()?)),
            "--tls-match-comp-id" => tls_match_comp_id = true,
            "--schedule" => {
                let text = value()?;
                config.schedule = Some(text.parse().map_err(|e| format!("invalid --schedule '{text}': {e}"))?);
            }
            "--metrics-listen" => {
                let addr = value()?;
                metrics_listen = Some(addr.parse().map_err(|e| format!("invalid --metrics-listen '{addr}': {e}"))?);
            }
            "--log-format" => {
                json_logs = match value()?.as_str() {
                    "text" => false,
                    "json" => true,
                    other => return Err(format!("--log-format must be 'text' or 'json', not '{other}'")),
                }
            }
            "--tls-client-auth" => {
                tls_client_auth = Some(match value()?.as_str() {
                    "required" => true,
                    "optional" => false,
                    other => return Err(format!("--tls-client-auth must be 'required' or 'optional', not '{other}'")),
                })
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    match (&allowed, allow_any) {
        (None, false) => {
            return Err("--allow is required: list the counterparty CompIDs that may log on (or pass \
                        --allow-any to accept any CompID)"
                .into());
        }
        (Some(_), true) => return Err("--allow and --allow-any are mutually exclusive".into()),
        _ => {}
    }
    if fsync && store_dir.is_none() {
        return Err("--fsync requires --store-dir".into());
    }
    if tls_client_auth.is_some() && tls_client_ca.is_none() {
        return Err("--tls-client-auth requires --tls-client-ca".into());
    }
    if tls_match_comp_id && tls_client_ca.is_none() {
        return Err("--tls-match-comp-id requires --tls-client-ca".into());
    }
    let tls = match (tls_cert, tls_key) {
        (Some(cert), Some(key)) => {
            Some(TlsArgs { cert, key, client_ca: tls_client_ca, client_cert_required: tls_client_auth.unwrap_or(true) })
        }
        (None, None) if tls_client_ca.is_some() => return Err("--tls-client-ca requires --tls-cert".into()),
        (None, None) => None,
        _ => return Err("--tls-cert and --tls-key must be given together".into()),
    };
    Ok(Args { listen, config, allowed, store_dir, fsync, tls, tls_match_comp_id, metrics_listen, json_logs })
}

impl TlsArgs {
    fn client_auth(&self) -> tls::ClientAuth<'_> {
        match (&self.client_ca, self.client_cert_required) {
            (None, _) => tls::ClientAuth::None,
            (Some(ca), true) => tls::ClientAuth::Required(ca),
            (Some(ca), false) => tls::ClientAuth::Optional(ca),
        }
    }

    fn describe(&self) -> &'static str {
        match self.client_auth() {
            tls::ClientAuth::None => "server-only",
            tls::ClientAuth::Optional(_) => "client certificates optional",
            tls::ClientAuth::Required(_) => "client certificates required",
        }
    }
}

/// `gateway seqnums ...`: operator control of a disconnected session's sequence numbers.
async fn seqnums(args: &[String]) -> ExitCode {
    match run_seqnums(args).await {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Runs `gateway seqnums`, returning what to print.
async fn run_seqnums(args: &[String]) -> Result<String, String> {
    let (mut store_dir, mut target, mut comp_id) = (None, None, "GATEWAY".to_string());
    let (mut reset, mut incoming, mut outgoing) = (false, None, None);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || args.next().cloned().ok_or(format!("{arg} requires a value"));
        let number = |text: String| text.parse::<u64>().map_err(|_| format!("{arg} needs a number, not '{text}'"));
        match arg.as_str() {
            "--store-dir" => store_dir = Some(PathBuf::from(value()?)),
            "--session" => target = Some(value()?),
            "--comp-id" => comp_id = value()?,
            "--reset" => reset = true,
            "--set-next-incoming" => incoming = Some(number(value()?)?),
            "--set-next-outgoing" => outgoing = Some(number(value()?)?),
            "-h" | "--help" => return Ok(SEQNUMS_USAGE.to_string()),
            other => return Err(format!("unknown argument '{other}'\n\n{SEQNUMS_USAGE}")),
        }
    }
    let store_dir = store_dir.ok_or(format!("--store-dir is required\n\n{SEQNUMS_USAGE}"))?;
    let target = target.ok_or(format!("--session is required\n\n{SEQNUMS_USAGE}"))?;
    if !store_dir.is_dir() {
        return Err(format!("store directory {} does not exist", store_dir.display()));
    }
    // Operator edits are rare and should survive a crash: fsync them.
    let storage = DiskStorage::new(&store_dir, true).map_err(|e| e.to_string())?;
    let id = SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: comp_id, target_comp_id: target };
    if !storage.contains(&id) {
        return Err(format!("no stored session {id} in {}", store_dir.display()));
    }
    let session = Arc::new(SessionRegistry::new(Arc::new(storage))).handle(id.clone());
    let explain = |e: SequenceError| match e {
        SequenceError::Storage(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            format!("{id} is connected (its store is locked); disconnect it first")
        }
        other => other.to_string(),
    };

    let mut report = String::new();
    if reset {
        session.reset_sequence_numbers().await.map_err(explain)?;
        report += &format!("reset {id}\n");
    }
    if let Some(seq) = incoming {
        session.set_next_incoming(seq).await.map_err(explain)?;
    }
    if let Some(seq) = outgoing {
        session.set_next_outgoing(seq).await.map_err(explain)?;
    }
    let numbers = session.sequence_numbers().await.map_err(explain)?;
    report += &format!("{id}: next incoming {}, next outgoing {}", numbers.next_incoming, numbers.next_outgoing);
    Ok(report)
}

#[tokio::main]
#[expect(clippy::too_many_lines, reason = "an example's setup, read top to bottom")]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("seqnums") {
        return seqnums(&args[2..]).await;
    }

    let Args { listen, config, allowed, store_dir, fsync, tls, tls_match_comp_id, metrics_listen, json_logs } =
        match parse_args(args[1..].iter().cloned()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error: {e}\n\n{USAGE}");
                return ExitCode::FAILURE;
            }
        };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if json_logs {
        tracing_subscriber::fmt().json().with_env_filter(filter).init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }

    // Install the metrics recorder before anything creates metric handles.
    if let Some(addr) = metrics_listen {
        if let Err(e) = PrometheusBuilder::new().with_http_listener(addr).install() {
            error!("cannot start metrics endpoint on {addr}: {e}");
            return ExitCode::FAILURE;
        }
        turbojet::describe_metrics();
        app::describe_metrics();
        info!(%addr, "serving Prometheus metrics at /metrics");
    }
    let storage: Arc<dyn SessionStorage> = match &store_dir {
        Some(dir) => match DiskStorage::new(dir, fsync) {
            Ok(storage) => {
                info!(dir = %dir.display(), fsync, "persisting sessions to disk");
                Arc::new(storage)
            }
            Err(e) => {
                error!("cannot use store directory {}: {e}", dir.display());
                return ExitCode::FAILURE;
            }
        },
        None => {
            info!("session state is in memory and will be lost on restart; use --store-dir to persist it");
            Arc::new(MemoryStorage::new())
        }
    };
    let tls_mode = tls.as_ref().map_or("off", TlsArgs::describe);
    let tls = match &tls {
        Some(args) => match tls::acceptor(&args.cert, &args.key, args.client_auth()) {
            Ok(acceptor) => Some(acceptor),
            Err(e) => {
                error!("cannot load TLS configuration: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => None,
    };
    let listener = match TcpListener::bind(&listen).await {
        Ok(l) => l,
        Err(e) => {
            error!("failed to bind {listen}: {e}");
            return ExitCode::FAILURE;
        }
    };
    info!(
        addr = %listener.local_addr().map(|a| a.to_string()).unwrap_or(listen),
        comp_id = %config.sender_comp_id,
        begin_string = %config.begin_string,
        tls = tls_mode,
        schedule = %config.schedule.as_ref().map_or("always open".to_string(), |s| s.to_string()),
        tls_match_comp_id,
        "FIX gateway listening"
    );

    let app = Arc::new(
        GatewayApp::new(Arc::new(OrderManager::new()), allowed).with_certificate_comp_id_match(tls_match_comp_id),
    );
    let acceptor = Acceptor::new(config, storage, app);
    let serve = {
        let acceptor = acceptor.clone();
        async move {
            match tls {
                Some(tls) => acceptor.serve_tls(listener, tls).await,
                None => acceptor.serve(listener).await,
            }
        }
    };
    tokio::select! {
        result = serve => {
            if let Err(e) = result {
                error!("accept failed: {e}");
                return ExitCode::FAILURE;
            }
        }
        () = shutdown_signal() => {}
    }
    info!("shutting down: logging sessions out (signal again to exit at once)");
    tokio::select! {
        () = acceptor.shutdown(Some("gateway shutting down")) => info!("all sessions closed"),
        () = shutdown_signal() => warn!("exiting without waiting for sessions to log out"),
    }
    ExitCode::SUCCESS
}

/// Completes on Ctrl-C (SIGINT) or, on Unix, SIGTERM, which service managers and `docker stop`
/// send.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut terminate = signal(SignalKind::terminate()).expect("cannot listen for SIGTERM");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = terminate.recv() => {}
        }
    }
    #[cfg(not(unix))]
    let _ = tokio::signal::ctrl_c().await;
}
