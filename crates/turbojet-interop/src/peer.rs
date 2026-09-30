//! Spawns the QuickFIX/J peer (peer/) and talks to it: commands on stdin, events on stdout.

use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs};

use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::mpsc;
use tokio::time::{Instant, timeout};

use crate::mailbox::{Mailbox, Missing};

/// The peer's CompID; Turbojet's is [`TJ`](crate::TJ).
pub const QFJ: &str = "QFJ";

/// How long to wait for any one event. Generous: CI runners are slow and JVMs start cold.
pub const EVENT_TIMEOUT: Duration = Duration::from_secs(10);

const HISTORY: usize = 300;

/// A FIX message as the peer printed it, `|` for SOH, and `\x01` for SOH inside XmlData(213). For
/// the `from_*` and `to_*` events this is
/// QuickFIX/J's re-serialization of the message, not necessarily the bytes on the wire; those are
/// [`PeerEvent::In`] and [`PeerEvent::Out`].
#[derive(Debug, Clone)]
pub struct FixMsg {
    raw: String,
    fields: Vec<(u32, String)>,
}

impl FixMsg {
    pub fn parse(raw: &str) -> Self {
        let fields = raw
            .split('|')
            .filter(|f| !f.is_empty())
            .map(|f| {
                let (tag, value) = f.split_once('=').unwrap_or_else(|| panic!("bad field {f:?} in {raw}"));
                let tag = tag.parse().unwrap_or_else(|_| panic!("bad tag {tag:?} in {raw}"));
                (tag, value.replace("\\x01", "\x01"))
            })
            .collect();
        Self { raw: raw.to_string(), fields }
    }

    /// The first value of `tag`.
    pub fn get(&self, tag: u32) -> Option<&str> {
        self.fields.iter().find(|(t, _)| *t == tag).map(|(_, v)| v.as_str())
    }

    pub fn msg_type(&self) -> &str {
        self.get(35).unwrap_or("")
    }

    /// MsgSeqNum(34).
    pub fn seq(&self) -> u64 {
        self.get(34).and_then(|s| s.parse().ok()).unwrap_or_else(|| panic!("no MsgSeqNum in {}", self.raw))
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

/// One line of the peer's output.
#[derive(Debug, Clone)]
pub enum PeerEvent {
    Ready(u16),
    Logon,
    Logout,
    /// QuickFIX/J accepted a session-level message and passed it to the application.
    FromAdmin(FixMsg),
    /// QuickFIX/J accepted an application message and passed it to the application.
    FromApp(FixMsg),
    ToAdmin(FixMsg),
    ToApp(FixMsg),
    /// Bytes QuickFIX/J read off the wire, `|` for SOH, whether or not it accepted them.
    In(String),
    /// Bytes QuickFIX/J wrote to the wire, `|` for SOH, resends included.
    Out(String),
    /// An error QuickFIX/J logged, e.g. why it rejected or ignored a message.
    QfjError(String),
    Ok(String),
    Error(String),
}

impl PeerEvent {
    pub fn parse(line: &str) -> Self {
        let (kind, payload) = line.split_once('\t').unwrap_or((line, ""));
        match kind {
            "ready" => Self::Ready(payload.parse().expect("ready without a port")),
            "logon" => Self::Logon,
            "logout" => Self::Logout,
            "from_admin" => Self::FromAdmin(FixMsg::parse(payload)),
            "from_app" => Self::FromApp(FixMsg::parse(payload)),
            "to_admin" => Self::ToAdmin(FixMsg::parse(payload)),
            "to_app" => Self::ToApp(FixMsg::parse(payload)),
            "in" => Self::In(payload.to_string()),
            "out" => Self::Out(payload.to_string()),
            "qfj_error" => Self::QfjError(payload.to_string()),
            "ok" => Self::Ok(payload.to_string()),
            "error" => Self::Error(payload.to_string()),
            _ => panic!("unknown peer event {line:?}"),
        }
    }

    /// The message QuickFIX/J received and accepted, if this is one.
    pub fn received(&self) -> Option<&FixMsg> {
        match self {
            Self::FromAdmin(m) | Self::FromApp(m) => Some(m),
            _ => None,
        }
    }

    /// The message QuickFIX/J sent, if this is one.
    pub fn sent(&self) -> Option<&FixMsg> {
        match self {
            Self::ToAdmin(m) | Self::ToApp(m) => Some(m),
            _ => None,
        }
    }
}

/// How to start the peer.
#[derive(Debug, Clone)]
pub struct PeerConfig {
    pub acceptor: bool,
    pub begin_string: &'static str,
    /// Where to connect, for an initiator.
    pub port: Option<u16>,
    pub heartbeat_secs: u32,
    pub reset_on_logon: bool,
    pub reconnect_secs: u32,
}

/// A running QuickFIX/J peer. Killed on drop.
pub struct Peer {
    child: Child,
    stdin: ChildStdin,
    events: Mailbox<PeerEvent, String>,
    history: Arc<Mutex<VecDeque<String>>>,
    /// Whether QuickFIX/J has reported logon and not since logout, as of its latest line.
    logged_on: Arc<AtomicBool>,
    /// Errors QuickFIX/J may or may not log, which [`Peer::finish`] lets pass.
    tolerated: Vec<fn(&str) -> bool>,
    port: u16,
    dir: Option<TempDir>,
}

/// The peer jar: `INTEROP_PEER_JAR`, or the Gradle build output.
fn jar() -> PathBuf {
    if let Some(path) = env::var_os("INTEROP_PEER_JAR") {
        return path.into();
    }
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("peer/build/libs/turbojet-interop-peer-all.jar");
    assert!(
        path.exists(),
        "QuickFIX/J peer not built ({} is missing): run scripts/interop.sh, or crates/turbojet-interop/peer/gradlew -q -p crates/turbojet-interop/peer shadowJar",
        path.display()
    );
    path
}

impl Peer {
    pub async fn spawn(config: PeerConfig) -> Self {
        let dir = TempDir::with_prefix("turbojet-interop-").unwrap();
        let stderr = File::create(dir.path().join("stderr.log")).unwrap();
        let mut command = Command::new("java");
        command
            .arg("-jar")
            .arg(jar())
            .arg(format!("role={}", if config.acceptor { "acceptor" } else { "initiator" }))
            .arg(format!("begin={}", config.begin_string))
            .arg(format!("sender={QFJ}"))
            .arg(format!("target={}", crate::TJ))
            .arg(format!("heartbeat={}", config.heartbeat_secs))
            .arg(format!("reset-on-logon={}", if config.reset_on_logon { "Y" } else { "N" }))
            .arg(format!("reconnect={}", config.reconnect_secs))
            .arg(format!("log-dir={}", dir.path().join("qfj").display()));
        if let Some(port) = config.port {
            command.arg(format!("port={port}"));
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .kill_on_drop(true)
            .spawn()
            .expect("could not start java; is a JDK installed?");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (tx, events) = mpsc::unbounded_channel();
        let history = Arc::new(Mutex::new(VecDeque::new()));
        let log = history.clone();
        let logged_on = Arc::new(AtomicBool::new(false));
        let session = logged_on.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                match line.split('\t').next() {
                    Some("logon") => session.store(true, Ordering::SeqCst),
                    Some("logout") => session.store(false, Ordering::SeqCst),
                    _ => {}
                }
                let mut log = log.lock().unwrap();
                if log.len() == HISTORY {
                    log.pop_front();
                }
                log.push_back(line.clone());
                drop(log);
                if tx.send(line).is_err() {
                    break;
                }
            }
        });

        let events = Mailbox::with_convert(events, |line| PeerEvent::parse(&line));
        let mut peer =
            Self { child, stdin, events, history, logged_on, tolerated: Vec::new(), port: 0, dir: Some(dir) };
        // The JVM can take a while to start on a cold runner.
        let deadline = Instant::now() + Duration::from_secs(30);
        peer.port = match peer.next(deadline, "ready").await {
            PeerEvent::Ready(port) => port,
            other => panic!("expected ready, got {other:?}"),
        };
        peer
    }

    /// The acceptor's port (or the port the initiator connects to).
    pub fn port(&self) -> u16 {
        self.port
    }

    async fn next(&mut self, deadline: Instant, what: &str) -> PeerEvent {
        match self.events.recv(deadline).await {
            Ok(event) => event,
            Err(missing) => self.missing(missing, what).await,
        }
    }

    async fn missing(&mut self, missing: Missing, what: &str) -> ! {
        match missing {
            Missing::TimedOut => panic!("timed out waiting for peer {what}"),
            Missing::Closed => {
                // Its stdout closes a moment before it exits.
                let status = match timeout(Duration::from_secs(1), self.child.wait()).await {
                    Ok(Ok(status)) => status.to_string(),
                    Ok(Err(e)) => format!("status unknown: {e}"),
                    Err(_) => "still running".to_string(),
                };
                panic!("peer exited ({status}) while waiting for {what}")
            }
        }
    }

    /// Sends a command and waits for it to be applied. Panics if the peer reports an error.
    pub async fn cmd(&mut self, line: &str) {
        self.stdin.write_all(format!("{line}\n").as_bytes()).await.unwrap();
        self.stdin.flush().await.unwrap();
        let name = line.split(' ').next().unwrap();
        let deadline = Instant::now() + EVENT_TIMEOUT;
        loop {
            match self.next(deadline, &format!("ok for {line:?}")).await {
                PeerEvent::Ok(done) if done == name => return,
                PeerEvent::Error(e) => panic!("peer command {line:?} failed: {e}"),
                other => self.events.push(other),
            }
        }
    }

    /// Sends an application or admin message: `fields` like `35=D|11=ORD1|...`.
    pub async fn send(&mut self, fields: &str) {
        self.cmd(&format!("send {fields}")).await;
    }

    /// The first event matching `pred`, buffered or yet to come; others stay buffered.
    pub async fn expect(&mut self, what: &str, pred: impl FnMut(&PeerEvent) -> bool) -> PeerEvent {
        match self.events.expect(Instant::now() + EVENT_TIMEOUT, pred).await {
            Ok(event) => event,
            Err(missing) => self.missing(missing, what).await,
        }
    }

    /// Fails if an event matching `pred` is buffered or arrives `within`.
    pub async fn expect_none(&mut self, what: &str, pred: impl FnMut(&PeerEvent) -> bool, within: Duration) {
        if let Some(event) = self.events.find_within(within, pred).await {
            panic!("expected no {what}, got {event:?}");
        }
    }

    pub async fn logon(&mut self) {
        self.expect("logon", |e| matches!(e, PeerEvent::Logon)).await;
    }

    pub async fn logout(&mut self) {
        self.expect("logout", |e| matches!(e, PeerEvent::Logout)).await;
    }

    /// The next message of `msg_type` QuickFIX/J received that satisfies `pred`.
    pub async fn received_with(&mut self, msg_type: &str, pred: impl Fn(&FixMsg) -> bool) -> FixMsg {
        let what = format!("received 35={msg_type}");
        let event = self.expect(&what, |e| e.received().is_some_and(|m| m.msg_type() == msg_type && pred(m))).await;
        event.received().unwrap().clone()
    }

    /// The next message of `msg_type` QuickFIX/J received.
    pub async fn received(&mut self, msg_type: &str) -> FixMsg {
        self.received_with(msg_type, |_| true).await
    }

    /// The next message of `msg_type` QuickFIX/J sent that satisfies `pred`.
    pub async fn sent_with(&mut self, msg_type: &str, pred: impl Fn(&FixMsg) -> bool) -> FixMsg {
        let what = format!("sent 35={msg_type}");
        let event = self.expect(&what, |e| e.sent().is_some_and(|m| m.msg_type() == msg_type && pred(m))).await;
        event.sent().unwrap().clone()
    }

    /// The next message of `msg_type` QuickFIX/J sent.
    pub async fn sent(&mut self, msg_type: &str) -> FixMsg {
        self.sent_with(msg_type, |_| true).await
    }

    /// The next message of `msg_type` satisfying `pred` that QuickFIX/J read off the wire, whether
    /// or not it accepted it.
    pub async fn wire_in(&mut self, msg_type: &str, pred: impl Fn(&FixMsg) -> bool) -> FixMsg {
        let what = format!("35={msg_type} in off the wire");
        self.wire(&what, |e| if let PeerEvent::In(raw) = e { Some(raw) } else { None }, msg_type, pred).await
    }

    /// The next message of `msg_type` satisfying `pred` that QuickFIX/J wrote to the wire, resends
    /// included.
    pub async fn wire_out(&mut self, msg_type: &str, pred: impl Fn(&FixMsg) -> bool) -> FixMsg {
        let what = format!("35={msg_type} out on the wire");
        self.wire(&what, |e| if let PeerEvent::Out(raw) = e { Some(raw) } else { None }, msg_type, pred).await
    }

    async fn wire(
        &mut self,
        what: &str,
        raw: fn(&PeerEvent) -> Option<&String>,
        msg_type: &str,
        pred: impl Fn(&FixMsg) -> bool,
    ) -> FixMsg {
        let parse = |e: &PeerEvent| raw(e).map(|r| FixMsg::parse(r));
        let event = self.expect(what, |e| parse(e).is_some_and(|m| m.msg_type() == msg_type && pred(&m))).await;
        parse(&event).unwrap()
    }

    /// Lets [`Peer::finish`] pass errors QuickFIX/J logs that match `pred`, for errors that depend
    /// on timing. An error that is certain should be consumed with [`Peer::expect`] instead.
    pub fn tolerate_errors(&mut self, pred: fn(&str) -> bool) {
        self.tolerated.push(pred);
    }

    /// Fails if QuickFIX/J sent or received a reject, or logged or reported an error, that the test didn't
    /// consume. So that a reject of anything sent before is in the buffer, it first exchanges a
    /// TestRequest and Heartbeat if logged on; if not, it stops the peer and waits for the end of
    /// its output.
    pub async fn finish(&mut self) {
        if self.logged_on.load(Ordering::SeqCst) {
            self.cmd("test-request FINISH").await;
            self.received_with("0", |m| m.get(112) == Some("FINISH")).await;
        } else {
            // If the peer has already gone, its output has ended too.
            let _ = self.stdin.write_all(b"quit\n").await;
            let _ = self.stdin.flush().await;
            let deadline = Instant::now() + EVENT_TIMEOUT;
            loop {
                match self.events.recv(deadline).await {
                    Ok(event) => self.events.push(event),
                    Err(Missing::Closed) => break,
                    Err(Missing::TimedOut) => panic!("timed out waiting for the peer to quit"),
                }
            }
        }
        for event in self.events.drain() {
            let bad = match event {
                PeerEvent::Error(_) => true,
                PeerEvent::QfjError(e) => !self.tolerated.iter().any(|ok| ok(e)),
                e => e.received().or(e.sent()).is_some_and(|m| matches!(m.msg_type(), "3" | "j")),
            };
            assert!(!bad, "unexpected {event:?}");
        }
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            return;
        }
        eprintln!("---- last QuickFIX/J peer events ----");
        for line in self.history.lock().unwrap().iter() {
            eprintln!("{line}");
        }
        if let Some(dir) = self.dir.take() {
            let path = dir.keep();
            eprintln!("---- peer stderr ----");
            eprintln!("{}", fs::read_to_string(path.join("stderr.log")).unwrap_or_default());
            for entry in fs::read_dir(path.join("qfj")).into_iter().flatten().flatten() {
                if entry.file_name().to_string_lossy().ends_with(".event.log") {
                    eprintln!("---- {} ----", entry.file_name().to_string_lossy());
                    eprintln!("{}", fs::read_to_string(entry.path()).unwrap_or_default());
                }
            }
            eprintln!("QuickFIX/J logs kept in {}", path.join("qfj").display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_events() {
        assert!(matches!(PeerEvent::parse("ready\t4321"), PeerEvent::Ready(4321)));
        assert!(matches!(PeerEvent::parse("logon\t"), PeerEvent::Logon));
        let PeerEvent::FromApp(msg) = PeerEvent::parse("from_app\t8=FIX.4.4|9=5|35=D|34=7|11=A|10=000|") else {
            panic!("expected from_app");
        };
        assert_eq!(msg.msg_type(), "D");
        assert_eq!(msg.seq(), 7);
        assert_eq!(msg.get(11), Some("A"));
        assert_eq!(msg.get(12), None);
        assert!(matches!(PeerEvent::parse("error\tsend: boom"), PeerEvent::Error(e) if e == "send: boom"));
        assert!(matches!(PeerEvent::parse("in\t8=FIX.4.4|35=0|"), PeerEvent::In(raw) if raw == "8=FIX.4.4|35=0|"));
        assert!(matches!(PeerEvent::parse("qfj_error\tbad"), PeerEvent::QfjError(e) if e == "bad"));
    }
}
