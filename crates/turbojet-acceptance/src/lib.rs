//! Runs QuickFIX's scripted session acceptance scenarios against a Turbojet acceptor, without
//! sockets and on a virtual clock. See the crate README for the script format and how results are
//! compared.

use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::{DateTime, TimeDelta, Utc};
use turbojet::codec::{Decoded, decode, encode};
use turbojet::message::{is_header_or_trailer, tags};
use turbojet::registry::CommandReceiver;
use turbojet::session::Action;
use turbojet::{
    ApplVerId, Application, Clock, ConnectionInfo, Context, MemoryStorage, Message, MessageReject, Session,
    SessionConfig, SessionId, SessionRegistry,
};
use turbojet_dictionary::Dictionary;

/// How long an expectation waits, in virtual time, for a message or a disconnect.
const WAIT_LIMIT: Duration = Duration::from_secs(65);

/// The longest the connection driver sleeps between timer calls; see `turbojet::connection`.
const TIMER_CEILING: Duration = Duration::from_secs(1);

/// Fields left out of comparisons: BodyLength and CheckSum depend on everything else, and Text's
/// wording is each engine's own (the reject reason codes carry the meaning).
const IGNORED: &[u32] = &[tags::BODY_LENGTH, tags::CHECK_SUM, tags::TEXT];

/// Timestamps, compared by form only: SendingTime, OrigSendingTime, TransactTime, OrigTime.
const TIMESTAMPS: &[u32] = &[tags::SENDING_TIME, tags::ORIG_SENDING_TIME, tags::TRANSACT_TIME, 42];

/// RefTagID(371), which Turbojet adds to a Reject whenever one tag is at fault; QuickFIX leaves it
/// out of some. The spec asks for it where it applies, so an extra one is accepted.
const REF_TAG_ID: u32 = 371;

/// Runs `script` (a path under definitions/, e.g. `fix42/2b_MsgSeqNumTooHigh.def`) and checks the
/// result against known_failures.txt: panics if it fails unexpectedly, or passes although listed.
pub fn check(script: &str) {
    let known = known_failure(script);
    match (run(script), known) {
        (Ok(()), None) => {}
        (Ok(()), Some(reason)) => {
            panic!("{script} passes now; remove it from known_failures.txt (listed as: {reason})")
        }
        (Err(failure), Some(reason)) => eprintln!("known failure ({reason}):\n{failure}"),
        (Err(failure), None) => panic!("{failure}"),
    }
}

fn known_failure(script: &str) -> Option<&'static str> {
    include_str!("../known_failures.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.trim() == script)
        .map(|(_, reason)| reason.trim())
}

/// Why a scenario failed, with everything that happened up to that point.
#[derive(Debug)]
pub struct Failure {
    script: String,
    line: usize,
    text: String,
    reason: String,
    transcript: Vec<String>,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} line {}: {}", self.script, self.line, self.reason)?;
        writeln!(f, "  {}", printable(&self.text))?;
        writeln!(f, "transcript:")?;
        for entry in &self.transcript {
            writeln!(f, "  {entry}")?;
        }
        Ok(())
    }
}

/// Runs one scenario script.
pub fn run(script: &str) -> Result<(), Failure> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions").join(script);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let version = Version::of(script);
    let mut runner = Runner::new(version);
    for (index, line) in text.lines().enumerate() {
        let Some(step) = Step::parse(line) else { continue };
        runner.step(&step).map_err(|reason| Failure {
            script: script.to_string(),
            line: index + 1,
            text: line.to_string(),
            reason,
            transcript: std::mem::take(&mut runner.transcript),
        })?;
    }
    Ok(())
}

// ---- Scripts ----

/// One line of a script. Lines may name a connection (`I2,8=FIX...`); the default is 1.
#[derive(Debug, PartialEq)]
enum Step {
    /// `iCONNECT`: the counterparty connects.
    Connect(u32),
    /// `iDISCONNECT`: the counterparty closes the connection.
    Disconnect(u32),
    /// `eDISCONNECT`: the engine closes the connection, without sending anything more first.
    ExpectDisconnect(u32),
    /// `I...`: the counterparty sends a message. BodyLength and CheckSum are added unless given.
    Send(u32, String),
    /// `E...`: the engine sends this message next.
    Expect(u32, String),
}

impl Step {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim_end_matches('\r');
        let kind = line.chars().next()?;
        let rest = &line[1..];
        let (conn, rest) = match rest.split_once(',') {
            Some((n, rest)) if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) => (n.parse().unwrap(), rest),
            _ => (1, rest),
        };
        // A directive may be followed by a comment on the same line.
        let directive = rest.split('#').next().unwrap().trim();
        Some(match (kind, directive) {
            ('#', _) => return None,
            ('i', "CONNECT") => Step::Connect(conn),
            ('i', "DISCONNECT") => Step::Disconnect(conn),
            ('e', "DISCONNECT") => Step::ExpectDisconnect(conn),
            ('I', _) => Step::Send(conn, rest.to_string()),
            ('E', _) => Step::Expect(conn, rest.to_string()),
            _ if line.trim().is_empty() => return None,
            _ => panic!("unknown script line: {}", printable(line)),
        })
    }
}

/// Replaces `<TIME>`, `<TIME+n>` and `<TIME-n>` (seconds) with UTC timestamps, as QuickFIX's
/// runner does.
fn timeify(text: &str, now: DateTime<Utc>) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("<TIME") {
        out.push_str(&rest[..start]);
        let end = start + rest[start..].find('>').expect("unterminated <TIME");
        let offset: i64 = match &rest[start + 5..end] {
            "" => 0,
            n => n.trim_start_matches('+').parse().expect("bad <TIME> offset"),
        };
        out.push_str(&(now + TimeDelta::seconds(offset)).format("%Y%m%d-%H:%M:%S").to_string());
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

/// Adds BodyLength and CheckSum to a message unless it has them, exactly as QuickFIX's runner
/// does (Reflector.rb `fixify!`), so scripts that give their own, wrong on purpose, keep them.
fn fixify(text: &str) -> Vec<u8> {
    let Some(head_end) = text.find('\x01').filter(|_| text.starts_with("8=")) else {
        return text.as_bytes().to_vec();
    };
    let has_length = text.contains("\x019=");
    let (head, rest) = text.split_at(head_end + 1);
    let (body, checksum) = match rest.find("\x0110=").filter(|_| rest.ends_with('\x01')) {
        Some(at) => (&rest[..at], Some(&rest[at..])),
        None => (rest, None),
    };
    let length = if has_length { String::new() } else { format!("9={}\x01", body.len()) };
    let mut out = format!("{head}{length}{body}").into_bytes();
    match checksum {
        Some(checksum) => out.extend_from_slice(checksum.as_bytes()),
        None => {
            let sum = out.iter().fold(0u8, |sum, b| sum.wrapping_add(*b));
            out.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
        }
    }
    out
}

/// A message or script line with SOH shown as `|`.
fn printable(text: &str) -> String {
    text.replace('\x01', "|")
}

// ---- The engine under test ----

#[derive(Debug, Clone, Copy)]
enum Version {
    Fix42,
    Fix43,
    Fix44,
    Fix50Sp2,
}

impl Version {
    fn of(script: &str) -> Self {
        match script.split('/').next() {
            Some("fix42") => Self::Fix42,
            Some("fix43") => Self::Fix43,
            Some("fix44") => Self::Fix44,
            Some("fix50sp2") => Self::Fix50Sp2,
            _ => panic!("no FIX version for {script}"),
        }
    }

    /// The counterparty QuickFIX's test server is configured for.
    fn counterparty(self) -> &'static str {
        match self {
            Self::Fix42 => "TW42",
            Self::Fix43 => "TW43",
            Self::Fix44 => "TW44",
            Self::Fix50Sp2 => "TW50SP2",
        }
    }

    /// The engine's session configuration, validating against the official dictionary. Built
    /// once per version: compiling the dictionary takes a while.
    fn config(self) -> SessionConfig {
        static CONFIGS: [OnceLock<SessionConfig>; 4] = [const { OnceLock::new() }; 4];
        CONFIGS[self as usize]
            .get_or_init(|| {
                let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dictionaries/orchestra");
                let load = |name: &str| Dictionary::load(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
                let (begin_string, dictionary) = match self {
                    Self::Fix42 => ("FIX.4.2", load("OrchestraFIX42.xml")),
                    Self::Fix43 => ("FIX.4.3", load("OrchestraFIX43.xml")),
                    Self::Fix44 => ("FIX.4.4", load("OrchestraFIX44.xml")),
                    Self::Fix50Sp2 => {
                        let dictionary = load("OrchestraFIX50SP2.xml").with_transport(&load("FIXTSession.xml"));
                        ("FIXT.1.1", dictionary.unwrap())
                    }
                };
                let mut config = SessionConfig::new(begin_string, "ISLD");
                if let Self::Fix50Sp2 = self {
                    config = config.with_appl_ver_id(ApplVerId::Fix50Sp2);
                }
                config.with_dictionary(&dictionary)
            })
            .clone()
    }
}

/// QuickFIX's test application (at_application.h): echoes NewOrderSingle, except a PossResend of
/// an order already seen, and echoes SecurityDefinition and Email; rejects other application
/// messages as unsupported. Refuses Logons from any counterparty but the configured one.
struct Reflector {
    counterparty: &'static str,
    orders: Mutex<HashSet<String>>,
}

impl Reflector {
    /// The message's body sent back. QuickFIX's echo also keeps PossResend(97) from the header.
    fn echo(ctx: &mut Context<'_>, msg: &Message) {
        let mut echo = Message::new(msg.msg_type());
        if let Some(poss_resend) = msg.get(tags::POSS_RESEND) {
            echo.push(tags::POSS_RESEND, poss_resend);
        }
        for (tag, value) in msg.fields().filter(|(tag, _)| !is_header_or_trailer(*tag)) {
            echo.push(tag, value);
        }
        ctx.send(echo);
    }
}

impl Application for Reflector {
    fn verify_logon(&self, session: &SessionId, _logon: &Message, _info: &ConnectionInfo) -> Result<(), String> {
        if session.target_comp_id == self.counterparty { Ok(()) } else { Err("unknown counterparty".into()) }
    }

    fn on_logout(&self, _session: &SessionId) {
        self.orders.lock().unwrap().clear();
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        match msg.msg_type().code() {
            "D" => {
                let id = msg.get(tags::CL_ORD_ID).unwrap_or_default().to_string();
                let seen = !self.orders.lock().unwrap().insert(id);
                if !(seen && msg.flag(tags::POSS_RESEND)) {
                    Self::echo(ctx, msg);
                }
                Ok(())
            }
            "d" | "C" => {
                Self::echo(ctx, msg);
                Ok(())
            }
            _ => Err(MessageReject::unsupported_message_type()),
        }
    }
}

/// Virtual time, shared with the sessions' clock.
struct VirtualTime {
    start: Instant,
    start_utc: DateTime<Utc>,
    elapsed: Arc<Mutex<Duration>>,
}

impl VirtualTime {
    fn now(&self) -> Instant {
        self.start + *self.elapsed.lock().unwrap()
    }

    fn utc(&self) -> DateTime<Utc> {
        self.start_utc + TimeDelta::from_std(*self.elapsed.lock().unwrap()).unwrap()
    }

    fn set(&self, now: Instant) {
        *self.elapsed.lock().unwrap() = now - self.start;
    }

    fn clock(&self) -> Clock {
        let (start, elapsed) = (self.start_utc, self.elapsed.clone());
        Clock::from_fn(move || start + TimeDelta::from_std(*elapsed.lock().unwrap()).unwrap())
    }
}

/// One of the counterparty's connections.
struct Connection {
    /// `None` once the connection has closed.
    session: Option<Session>,
    _commands: CommandReceiver,
    /// Bytes from the counterparty not yet decoded.
    buf: Vec<u8>,
    /// What the engine has sent and the script hasn't consumed yet.
    sent: VecDeque<Message>,
}

struct Runner {
    config: SessionConfig,
    app: Arc<Reflector>,
    time: VirtualTime,
    registry: Arc<SessionRegistry>,
    connections: Vec<(u32, Connection)>,
    transcript: Vec<String>,
}

impl Runner {
    fn new(version: Version) -> Self {
        let time = VirtualTime { start: Instant::now(), start_utc: Utc::now(), elapsed: Arc::default() };
        let mut config = version.config();
        config.clock = time.clock();
        Self {
            config,
            app: Arc::new(Reflector { counterparty: version.counterparty(), orders: Mutex::default() }),
            time,
            registry: Arc::new(SessionRegistry::default()),
            connections: Vec::new(),
            transcript: Vec::new(),
        }
    }

    fn step(&mut self, step: &Step) -> Result<(), String> {
        match step {
            Step::Connect(id) => self.connect(*id),
            Step::Disconnect(id) => {
                self.log(*id, "counterparty disconnects".into());
                self.connection(*id)?.session = None;
                Ok(())
            }
            Step::Send(id, text) => self.send(*id, text),
            Step::Expect(id, text) => self.expect(*id, text),
            Step::ExpectDisconnect(id) => self.expect_disconnect(*id),
        }
    }

    fn connect(&mut self, id: u32) -> Result<(), String> {
        // QuickFIX's test server resets each session on logon (ResetOnLogon=Y): a connection made
        // while no other is open starts from fresh storage.
        if self.connections.iter().all(|(_, conn)| conn.session.is_none()) {
            self.registry = Arc::new(SessionRegistry::new(Arc::new(MemoryStorage::new())));
        }
        let now = self.time.now();
        let (mut session, commands) =
            Session::acceptor(self.config.clone(), self.registry.clone(), self.app.clone(), now);
        let actions = session.on_connect(now);
        self.connections.retain(|(other, _)| *other != id);
        let conn = Connection { session: Some(session), _commands: commands, buf: Vec::new(), sent: VecDeque::new() };
        self.connections.push((id, conn));
        self.log(id, "counterparty connects".into());
        self.apply(id, actions);
        Ok(())
    }

    fn send(&mut self, id: u32, text: &str) -> Result<(), String> {
        let bytes = fixify(&timeify(text, self.time.utc()));
        self.log(id, format!("-> {}", printable(&String::from_utf8_lossy(&bytes))));
        let now = self.time.now();
        let conn = self.connection(id)?;
        if conn.session.is_none() {
            // Over TCP, a write just after the other end closed usually succeeds and is lost.
            self.log(id, "   (lost: the engine has disconnected)".into());
            return Ok(());
        }
        conn.buf.extend_from_slice(&bytes);
        // Decode as the connection's read loop does, one message at a time, since a message may
        // end the connection.
        let mut garbled = Vec::new();
        loop {
            let conn = self.connection(id)?;
            let Some(session) = conn.session.as_mut() else { break };
            match decode(&conn.buf) {
                Decoded::Message(msg, len) => {
                    conn.buf.drain(..len);
                    let actions = session.on_message(msg, now);
                    self.apply(id, actions);
                }
                Decoded::Incomplete => break,
                Decoded::Garbled { skip, reason } => {
                    conn.buf.drain(..skip);
                    garbled.push(reason);
                }
            }
        }
        for reason in garbled {
            self.log(id, format!("   (garbled input ignored: {reason})"));
        }
        Ok(())
    }

    /// Carries out what a session asked for: queues what it sent, and closes the connection on
    /// Disconnect (ignoring anything after, as the driver does).
    fn apply(&mut self, id: u32, actions: Vec<Action>) {
        for action in actions {
            match action {
                Action::Send(msg) => {
                    let wire = encode(&msg).unwrap();
                    self.log(id, format!("<- {}", printable(&String::from_utf8_lossy(&wire))));
                    let Decoded::Message(msg, _) = decode(&wire) else { panic!("the engine sent garbage: {msg}") };
                    self.connection(id).unwrap().sent.push_back(msg);
                }
                Action::Disconnect => {
                    self.log(id, "engine disconnects".into());
                    self.connection(id).unwrap().session = None;
                    return;
                }
            }
        }
    }

    fn expect(&mut self, id: u32, text: &str) -> Result<(), String> {
        let expected = timeify(text, self.time.utc());
        let waited_from = self.time.now();
        loop {
            let conn = self.connection(id)?;
            if let Some(msg) = conn.sent.pop_front() {
                return compare(&expected, &msg);
            }
            if conn.session.is_none() {
                return Err("expected a message, but the engine disconnected".into());
            }
            if self.time.now() - waited_from >= WAIT_LIMIT {
                return Err(format!("expected a message, but nothing came within {WAIT_LIMIT:?}"));
            }
            self.tick();
        }
    }

    fn expect_disconnect(&mut self, id: u32) -> Result<(), String> {
        let waited_from = self.time.now();
        loop {
            let conn = self.connection(id)?;
            if let Some(msg) = conn.sent.pop_front() {
                return Err(format!("expected a disconnect, but the engine sent {}", printable(&msg.to_string())));
            }
            if conn.session.is_none() {
                return Ok(());
            }
            if self.time.now() - waited_from >= WAIT_LIMIT {
                return Err(format!("expected a disconnect, but the connection was still open after {WAIT_LIMIT:?}"));
            }
            self.tick();
        }
    }

    /// Moves time on to the next timer call of any open connection, as their drivers would make
    /// it: at a session's next deadline, or at most a second after the last call.
    fn tick(&mut self) {
        let now = self.time.now();
        let next = self
            .connections
            .iter()
            .filter_map(|(_, conn)| conn.session.as_ref()?.next_deadline())
            .filter(|deadline| *deadline > now)
            .fold(now + TIMER_CEILING, Instant::min);
        self.time.set(next);
        let ids: Vec<u32> = self.connections.iter().map(|(id, _)| *id).collect();
        for id in ids {
            let Some(session) = self.connection(id).unwrap().session.as_mut() else { continue };
            let actions = session.on_timer(next);
            if !actions.is_empty() {
                let elapsed = next - self.time.start;
                self.log(id, format!("   (timer at +{:.1}s)", elapsed.as_secs_f64()));
            }
            self.apply(id, actions);
        }
    }

    fn connection(&mut self, id: u32) -> Result<&mut Connection, String> {
        self.connections
            .iter_mut()
            .find(|(other, _)| *other == id)
            .map(|(_, conn)| conn)
            .ok_or_else(|| format!("connection {id} was never opened"))
    }

    fn log(&mut self, id: u32, entry: String) {
        self.transcript.push(format!("[{id}] {entry}"));
    }
}

/// Compares a message the engine sent with the script's expectation: the same fields with the
/// same values, in any order, except those in [`IGNORED`], [`TIMESTAMPS`] only by form, an extra
/// [`REF_TAG_ID`] allowed, and the TestReqID of a TestRequest (the engine's choice) any value.
fn compare(expected: &str, actual: &Message) -> Result<(), String> {
    let test_request = actual.msg_type().code() == "1";
    let normalise = |tag: u32, value: &str| {
        if test_request && tag == tags::TEST_REQ_ID {
            "<any>".to_string()
        } else if TIMESTAMPS.contains(&tag) && is_timestamp(value) {
            "<timestamp>".to_string()
        } else {
            value.to_string()
        }
    };
    let mut want: Vec<(u32, String)> = expected
        .split('\x01')
        .filter(|field| !field.is_empty())
        .map(|field| {
            let (tag, value) = field.split_once('=').unwrap_or_else(|| panic!("bad expected field {field:?}"));
            let tag: u32 = tag.parse().unwrap_or_else(|_| panic!("bad expected tag {tag:?}"));
            (tag, normalise(tag, value))
        })
        .filter(|(tag, _)| !IGNORED.contains(tag))
        .collect();
    let mut got: Vec<(u32, String)> = actual
        .fields()
        .filter(|(tag, _)| !IGNORED.contains(tag))
        .map(|(tag, value)| (tag, normalise(tag, value)))
        .collect();
    if !want.iter().any(|(tag, _)| *tag == REF_TAG_ID) {
        got.retain(|(tag, _)| *tag != REF_TAG_ID);
    }
    want.sort();
    got.sort();
    if want == got {
        return Ok(());
    }
    let missing: Vec<String> = want.iter().filter(|f| !got.contains(f)).map(|(t, v)| format!("{t}={v}")).collect();
    let extra: Vec<String> = got.iter().filter(|f| !want.contains(f)).map(|(t, v)| format!("{t}={v}")).collect();
    Err(format!(
        "wrong message: expected {}\n  got {}\n  missing {missing:?}, unexpected {extra:?}",
        printable(expected),
        printable(&actual.to_string())
    ))
}

/// `YYYYMMDD-HH:MM:SS`, with or without fractional seconds.
fn is_timestamp(value: &str) -> bool {
    let b = value.as_bytes();
    let digits = |range: std::ops::Range<usize>| b.get(range).is_some_and(|d| d.iter().all(u8::is_ascii_digit));
    b.len() >= 17
        && digits(0..8)
        && b[8] == b'-'
        && digits(9..11)
        && b[11] == b':'
        && digits(12..14)
        && b[14] == b':'
        && digits(15..17)
        && (b.len() == 17 || (b[17] == b'.' && b.len() > 18 && digits(18..b.len())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_steps_with_connections_and_comments() {
        assert_eq!(Step::parse("iCONNECT"), Some(Step::Connect(1)));
        assert_eq!(Step::parse("i2,CONNECT"), Some(Step::Connect(2)));
        assert_eq!(Step::parse("i1,DISCONNECT# If a bad SenderCompID"), Some(Step::Disconnect(1)));
        assert_eq!(Step::parse("eDISCONNECT"), Some(Step::ExpectDisconnect(1)));
        assert_eq!(Step::parse("E2,8=FIX.4.2\x0135=0\x01"), Some(Step::Expect(2, "8=FIX.4.2\x0135=0\x01".into())));
        assert_eq!(Step::parse("# comment"), None);
        assert_eq!(Step::parse(""), None);
    }

    #[test]
    fn fixify_adds_length_and_checksum_unless_given() {
        let msg = fixify("8=FIX.4.2\x0135=0\x0134=2\x01");
        assert!(matches!(decode(&msg), Decoded::Message(..)), "{}", printable(&String::from_utf8_lossy(&msg)));
        // A given (wrong) CheckSum is kept.
        let msg = fixify("8=FIX.4.2\x019=10\x0135=0\x0134=2\x0110=000\x01");
        assert!(msg.ends_with(b"10=000\x01"));
        assert!(matches!(decode(&msg), Decoded::Garbled { .. }));
    }

    #[test]
    fn timeify_substitutes_offsets() {
        let now = DateTime::parse_from_rfc3339("2026-09-29T12:00:00Z").unwrap().with_timezone(&Utc);
        assert_eq!(
            timeify("52=<TIME>|122=<TIME-121>|60=<TIME+10>", now),
            "52=20260929-12:00:00|122=20260929-11:57:59|60=20260929-12:00:10"
        );
    }

    #[test]
    fn compares_regardless_of_order_length_checksum_text_and_time() {
        let actual = Message::default()
            .with(8, "FIX.4.2")
            .with(9, "99")
            .with(35, "3")
            .with(49, "ISLD")
            .with(52, "20260929-12:00:00.123")
            .with(45, "2")
            .with(58, "our wording")
            .with(10, "123");
        assert!(
            compare(
                "8=FIX.4.2\x019=1\x0135=3\x0145=2\x0152=00000000-00:00:00.000\x0149=ISLD\x0158=theirs\x0110=0\x01",
                &actual
            )
            .is_ok()
        );
        assert!(compare("8=FIX.4.2\x0135=3\x0145=3\x0152=00000000-00:00:00\x0149=ISLD\x01", &actual).is_err());
        assert!(compare("8=FIX.4.2\x0135=3\x0145=2\x0152=00000000-00:00:00\x0149=ISLD\x01373=1\x01", &actual).is_err());
    }
}
