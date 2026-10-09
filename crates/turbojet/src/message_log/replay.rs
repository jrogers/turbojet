//! Replaying a message log: the inbound messages a [`FileMessageLog`](super::FileMessageLog)
//! recorded, fed to an [`Application`] again through a session, at their recorded times.

use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

use super::{Direction, LogRecord};
use crate::codec::{Decoded, decode};
use crate::message::tags;
use crate::registry::{CommandReceiver, SessionRegistry};
use crate::session::{Session, SessionConfig};
use crate::store::{SessionId, SessionStorage};
use crate::{Application, Clock, MemoryStorage, Message, MsgType};

/// The longest a replayed session goes without a timer call, as a driver calls it at least once a
/// second.
const TIMER_INTERVAL_MAX: Duration = Duration::from_secs(1);

/// What [`replay`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Replayed {
    /// Connections replayed: one for each of the session's inbound Logons.
    pub connections: u64,
    /// Inbound messages fed to the session, Logons included.
    pub messages: u64,
    /// Inbound messages left out: other sessions', garbled ones (which the session never saw
    /// either), and the session's own while no connection was open, before its first Logon or
    /// after it closed.
    pub skipped: u64,
    /// Messages the log dropped rather than wrote (see [`LogRecord::Dropped`]), from all sessions:
    /// if any are the session's, the replay is missing them.
    pub dropped: u64,
}

/// Feeds the messages `session` received, as `records` recorded them, to `app` again: through a
/// [`Session`] as an acceptor with `config` (as in production), at the recorded times, to
/// reproduce a problem away from production. `session` is as the log names it,
/// `FIX.4.4:US->THEM`; `records` is [`FileMessageLog::read`](super::FileMessageLog::read), or
/// several files' records chained.
///
/// The log records no connections, so each inbound Logon starts one, ending any still open. Each
/// starts expecting the Logon's MsgSeqNum, and sending from its NextExpectedMsgSeqNum(789) if it
/// has one, so a log that starts mid-day replays in sequence; a gap the Logon itself revealed is
/// not reproduced. Between messages, time passes as recorded, with timer calls as a driver makes
/// them, so heartbeat timeouts happen as they did. What the session sends is discarded; the
/// session's store is in memory; and `config`'s clock, message log and cancel on disconnect are
/// replaced, the last turned off. Needs no async runtime.
///
/// # Errors
///
/// A record that can't be read, or a Logon that identifies a session other than `session` (a
/// `config` that doesn't match the one in production, say), which ends the replay.
pub fn replay(
    records: impl IntoIterator<Item = io::Result<LogRecord>>,
    session: &SessionId,
    config: SessionConfig,
    app: Arc<dyn Application>,
) -> io::Result<Replayed> {
    let mut replayer = Replayer::new(session, config, app);
    for record in records {
        match record? {
            LogRecord::Message { time, direction: Direction::Inbound, session: name, frame } => {
                replayer.inbound(time, name.as_deref(), &frame)?;
            }
            LogRecord::Dropped { count, .. } => replayer.replayed.dropped += count,
            _ => {}
        }
    }
    replayer.finish();
    Ok(replayer.replayed)
}

/// A connection being replayed. The receiver is kept so that the session's handle stays usable.
struct Connection {
    session: Session,
    _commands: CommandReceiver,
}

struct Replayer {
    id: SessionId,
    name: String,
    config: SessionConfig,
    app: Arc<dyn Application>,
    storage: Arc<MemoryStorage>,
    registry: Arc<SessionRegistry>,
    time: VirtualTime,
    open: Option<Connection>,
    replayed: Replayed,
}

impl Replayer {
    fn new(id: &SessionId, mut config: SessionConfig, app: Arc<dyn Application>) -> Self {
        let time = VirtualTime::new();
        config.clock = time.clock();
        config.message_log = None;
        config.cancel_on_disconnect = None;
        let storage = Arc::new(MemoryStorage::new());
        let registry = Arc::new(SessionRegistry::new(storage.clone()));
        let (name, open, replayed) = (id.to_string(), None, Replayed::default());
        Self { id: id.clone(), name, config, app, storage, registry, time, open, replayed }
    }

    fn inbound(&mut self, at: DateTime<Utc>, name: Option<&str>, frame: &[u8]) -> io::Result<()> {
        let Decoded::Message(msg, _) = decode(frame) else {
            self.replayed.skipped += 1;
            return Ok(());
        };
        // Until a Logon identifies it, a message is logged without its session.
        if !name.map_or_else(|| self.is_from_counterparty(&msg), |name| name == self.name) {
            self.replayed.skipped += 1;
            return Ok(());
        }
        let now = self.time.instant_at(at);
        self.run_timers(now);
        self.time.set(now);
        if msg.msg_type() == MsgType::Logon {
            self.disconnect();
            self.connect(&msg, now)?;
        }
        let Some(open) = &mut self.open else {
            self.replayed.skipped += 1;
            return Ok(());
        };
        open.session.on_message(&msg, now);
        settle(&mut open.session, now);
        self.replayed.messages += 1;
        if let Some(id) = open.session.session_id().filter(|id| **id != self.id) {
            let text = format!("a Logon identifies session {id}, not {}: is the config as in production?", self.id);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, text));
        }
        if open.session.is_closed() {
            self.open = None;
        }
        Ok(())
    }

    /// Whether `msg`'s header names the session, from the counterparty to us.
    fn is_from_counterparty(&self, msg: &Message) -> bool {
        msg.get(tags::BEGIN_STRING) == Some(self.id.begin_string.as_str())
            && msg.get(tags::SENDER_COMP_ID) == Some(self.id.target_comp_id.as_str())
            && msg.get(tags::TARGET_COMP_ID) == Some(self.id.sender_comp_id.as_str())
    }

    /// Starts a connection for `logon`, with the store set as the counterparty expects it.
    fn connect(&mut self, logon: &Message, now: Instant) -> io::Result<()> {
        prime(&*self.storage, &self.id, logon)?;
        let (mut session, commands) =
            Session::acceptor(self.config.clone(), self.registry.clone(), self.app.clone(), now);
        session.on_connect(now);
        self.open = Some(Connection { session, _commands: commands });
        self.replayed.connections += 1;
        Ok(())
    }

    /// Ends the open connection, if any, as its transport ending would.
    fn disconnect(&mut self) {
        if let Some(mut open) = self.open.take() {
            let now = self.time.now;
            open.session.on_disconnect(now);
            settle(&mut open.session, now);
        }
    }

    /// Calls the open session's timer as a driver would until `until`: at each deadline, and at
    /// least once a second. A session left silent times out, so this ends.
    fn run_timers(&mut self, until: Instant) {
        let Some(open) = &mut self.open else { return };
        let mut last = self.time.now;
        while !open.session.is_closed() {
            let ceiling = last + TIMER_INTERVAL_MAX;
            let next =
                open.session.next_deadline().filter(|deadline| *deadline > last).map_or(ceiling, |d| d.min(ceiling));
            if next > until {
                return;
            }
            self.time.set(next);
            open.session.on_timer(next);
            settle(&mut open.session, next);
            last = next;
        }
        self.open = None;
    }

    fn finish(&mut self) {
        self.disconnect();
    }
}

/// Runs the session's store work and the rest of any resend, discarding what it sends.
fn settle(session: &mut Session, now: Instant) {
    session.commit_blocking(now);
    session.clear_output();
    while session.is_resending() && !session.is_closed() {
        session.on_resume(now);
        session.commit_blocking(now);
        session.clear_output();
    }
}

/// Sets `id`'s stored sequence numbers as `logon` says the counterparty expects them: its
/// MsgSeqNum next from it, and its NextExpectedMsgSeqNum next from us if it has one.
fn prime(storage: &dyn SessionStorage, id: &SessionId, logon: &Message) -> io::Result<()> {
    let mut log = storage.open(id)?;
    let next_outgoing = logon.field::<u64>(tags::NEXT_EXPECTED_MSG_SEQ_NUM).unwrap_or(log.next_outgoing());
    // Numbers only move forward, and a later connection's may be lower.
    log.reset()?;
    if let Ok(seq) = logon.field::<u64>(tags::MSG_SEQ_NUM) {
        log.set_next_incoming(seq)?;
    }
    if next_outgoing > 1 {
        log.record_outgoing(next_outgoing - 1, None)?;
    }
    Ok(())
}

/// Time as recorded: an `Instant` for the session, and the wall clock its `Clock` reads, moving
/// together from the first record's time.
struct VirtualTime {
    start: Instant,
    /// The recorded time at `start`: the first record's, once there is one.
    origin: Option<DateTime<Utc>>,
    now: Instant,
    wall: Arc<Mutex<DateTime<Utc>>>,
}

impl VirtualTime {
    fn new() -> Self {
        let start = Instant::now();
        Self { start, origin: None, now: start, wall: Arc::new(Mutex::new(DateTime::UNIX_EPOCH)) }
    }

    fn clock(&self) -> Clock {
        let wall = self.wall.clone();
        Clock::from_fn(move || *wall.lock().expect("the replay's clock lock poisoned"))
    }

    /// The `Instant` of a record at `at`. A clock stepped back while logging doesn't take time
    /// backwards: the record is replayed at the time of the one before.
    fn instant_at(&mut self, at: DateTime<Utc>) -> Instant {
        let origin = *self.origin.get_or_insert(at);
        (self.start + (at - origin).to_std().unwrap_or_default()).max(self.now)
    }

    fn set(&mut self, now: Instant) {
        debug_assert!(now >= self.now, "replayed time moves forward");
        let origin = self.origin.expect("set after a record's instant_at, which fixes the origin");
        self.now = now;
        let since_start = chrono::TimeDelta::from_std(now - self.start).expect("a log spans less than chrono's range");
        *self.wall.lock().expect("the replay's clock lock poisoned") = origin + since_start;
    }
}
