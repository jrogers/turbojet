//! Replaying a message log: a log written by FileMessageLog, fed again to an application.

use std::io;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use turbojet::registry::SessionHandle;
use turbojet::{
    Application, Clock, Context, Disconnect, FileLogOptions, FileMessageLog, LogRecord, Message, MessageLog,
    MessageReject, Replayed, SessionConfig, SessionId, replay,
};

/// What the application saw, in order.
#[derive(Default)]
struct Recorder(Mutex<Vec<String>>);

impl Recorder {
    fn seen(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
}

impl Application for Recorder {
    fn on_logon(&self, _session: &SessionHandle) {
        self.0.lock().unwrap().push("logon".into());
    }

    fn on_logout(&self, _session: &SessionHandle, ended: Disconnect) {
        self.0.lock().unwrap().push(format!("logout {ended:?}"));
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        self.0.lock().unwrap().push(format!("order {}", msg.get(11).unwrap_or_default()));
        Ok(())
    }
}

fn noon() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 8, 12, 0, 0).unwrap()
}

/// A log being written, its clock set by each message's time.
struct Log {
    log: FileMessageLog,
    now: Arc<Mutex<DateTime<Utc>>>,
    dir: tempfile::TempDir,
}

impl Log {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let now = Arc::new(Mutex::new(noon()));
        let clock = now.clone();
        let options = FileLogOptions { clock: Clock::from_fn(move || *clock.lock().unwrap()), ..Default::default() };
        Self { log: FileMessageLog::open(dir.path(), options).unwrap(), now, dir }
    }

    /// Logs a message from THEM to US at `secs` past noon, with SendingTime then. `session` is
    /// whether it's logged with its session, as everything after the Logon is.
    fn inbound(&self, secs: i64, session: bool, seq: u64, body: &str) {
        let at = noon() + TimeDelta::seconds(secs);
        *self.now.lock().unwrap() = at;
        let header = format!("49=THEM|56=US|34={seq}|52={}|", at.format("%Y%m%d-%H:%M:%S%.3f"));
        let id = SessionId::new("FIX.4.4", "US", "THEM");
        self.log.inbound(session.then_some(&id), &frame(&format!("{}{header}{}", &body[..5], &body[5..])));
    }

    /// The records written, read back.
    fn records(self) -> Vec<io::Result<LogRecord>> {
        drop(self.log);
        let files = FileMessageLog::files(self.dir.path()).unwrap();
        files.iter().flat_map(|file| FileMessageLog::read(&file.path).unwrap()).collect()
    }
}

/// A FIX.4.4 frame of `body` (fields separated by `|`, MsgType first), with BodyLength and
/// CheckSum.
fn frame(body: &str) -> Vec<u8> {
    let body = body.replace('|', "\x01");
    let mut frame = format!("8=FIX.4.4\x019={}\x01{body}", body.len()).into_bytes();
    let sum = frame.iter().map(|&b| u32::from(b)).sum::<u32>() % 256;
    frame.extend(format!("10={sum:03}\x01").bytes());
    frame
}

fn run(log: Log, config: SessionConfig) -> (io::Result<Replayed>, Vec<String>) {
    let app = Arc::new(Recorder::default());
    let id = SessionId::new("FIX.4.4", "US", "THEM");
    let replayed = replay(log.records(), &id, config, app.clone());
    (replayed, app.seen())
}

fn config() -> SessionConfig {
    SessionConfig::new("FIX.4.4", "US")
}

#[test]
fn messages_are_delivered_as_the_session_received_them() {
    let log = Log::new();
    log.inbound(0, false, 500, "35=A|98=0|108=30|");
    log.inbound(1, true, 501, "35=D|11=A|");
    // A duplicate, which the session ignores, then a gap: what follows it waits for a resend.
    log.inbound(2, true, 501, "35=D|43=Y|122=20261008-12:00:01.000|11=A|");
    log.inbound(3, true, 503, "35=D|11=C|");
    let (replayed, seen) = run(log, config());
    assert_eq!(seen, ["logon", "order A", "logout ConnectionLost"]);
    let replayed = replayed.unwrap();
    assert_eq!(replayed.connections, 1);
    assert_eq!(replayed.messages, 4);
    assert_eq!(replayed.skipped, 0);
}

#[test]
fn each_logon_starts_a_connection_and_time_passes_as_recorded() {
    let log = Log::new();
    log.inbound(0, false, 1, "35=A|98=0|108=30|");
    log.inbound(1, true, 2, "35=D|11=A|");
    // Silent for ten minutes: the session times out, and what comes before the next Logon is
    // left out.
    log.inbound(600, true, 3, "35=D|11=B|");
    log.inbound(601, false, 4, "35=A|98=0|108=30|789=7|");
    log.inbound(602, true, 5, "35=D|11=C|");
    let (replayed, seen) = run(log, config());
    let expected = ["logon", "order A", "logout HeartbeatTimeout", "logon", "order C", "logout ConnectionLost"];
    assert_eq!(seen, expected);
    let replayed = replayed.unwrap();
    assert_eq!(replayed.connections, 2);
    assert_eq!(replayed.skipped, 1);
}

#[test]
fn other_sessions_are_left_out() {
    let log = Log::new();
    log.inbound(0, false, 1, "35=A|98=0|108=30|");
    let other = SessionId::new("FIX.4.4", "US", "OTHER");
    log.log.inbound(Some(&other), &frame("35=D|49=OTHER|56=US|34=2|52=20261008-12:00:00.000|11=X|"));
    let (replayed, seen) = run(log, config());
    assert_eq!(seen, ["logon", "logout ConnectionLost"]);
    assert_eq!(replayed.unwrap().skipped, 1);
}

#[test]
fn a_config_that_names_another_session_is_an_error() {
    let log = Log::new();
    log.inbound(0, false, 1, "35=A|98=0|108=30|");
    // A qualifier is ours alone: no Logon names it.
    let id = SessionId::new("FIX.4.4", "US", "THEM").with_qualifier("second");
    let err = replay(log.records(), &id, config(), Arc::new(Recorder::default())).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
}
