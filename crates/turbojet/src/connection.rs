//! Drives a [`Session`] over any byte stream.

use std::io;
use std::pin::Pin;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::task::JoinHandle;
use tokio::time::{Instant, Sleep};
use tracing::{Instrument, debug, warn};

use crate::codec::{DecodedInto, decode_into};
use crate::message::Message;
use crate::registry::{Command, CommandReceiver, Next, Sends};
use crate::session::Session;
use crate::shutdown::Signal;
use crate::store::{SentMessages, SessionLog};
use crate::telemetry;

// The simulator in crates/turbojet-sim (src/node.rs) drives sessions as this driver does, branch
// for branch: keep the two in step.

/// Application sends taken from a [`SessionHandle`](crate::SessionHandle) queue in one batch, so a
/// flood of sends can't starve reading from the peer.
const MAX_COMMANDS_PER_BATCH: usize = 256;

/// Initial capacity of the read buffer; a larger message still arrives, as the buffer grows.
///
/// Measured with 1,000 orders in flight: 8-16 KiB is fastest, and 64 KiB was 12-14% slower. Larger
/// reads make larger batches, so the peer waits longer for the first replies and the two ends
/// overlap less. Bigger is not better here.
const READ_BUFFER_SIZE: usize = 8 * 1024;

/// Longest the connection waits without calling `on_timer`: schedule boundaries are wall-clock
/// times, which [`Session::next_deadline`] doesn't cover.
const MAX_TIMER_SLEEP: Duration = Duration::from_secs(1);

/// While this much output is still unwritten, handle commands wait in their queue, so a burst of
/// application sends waits there rather than here.
const COMMANDS_PAUSE_AT: usize = 256 * 1024;

/// Most output left unwritten before the connection is dropped: the counterparty has stopped
/// reading. The driver keeps reading while output waits (so two ends writing at once never wait
/// for each other), so this grows only with replies to what it reads.
const MAX_UNWRITTEN: usize = 16 * 1024 * 1024;

/// Most reads that add input already arrived to a batch before it's committed, when commits wait
/// for the store: enough to take a burst into one commit, while a counterparty sending without
/// pause can't hold a commit off.
const MAX_READS_PER_BATCH: usize = 16;

/// Most input read but not yet processed, during a resend, before the connection is dropped.
/// Input waits for the end of a resend, so nothing new goes out in the middle of it, but it's
/// read meanwhile, so the counterparty's writes (its own resend, say) don't block. Input an inbound
/// Delay limit holds isn't read, so it can't build up here.
const MAX_UNPROCESSED: usize = 16 * 1024 * 1024;

/// Runs `session` over `stream` until either side disconnects.
///
/// Each wake-up (a read from the peer, a batch of handle commands, or a timer deadline) can produce
/// several outgoing messages; the session encodes them into one buffer, written as the stream
/// takes it, usually in one write and flush. The store commits what each wake-up did once, before
/// any of it is written; a commit that blocks (an fsync) runs on a blocking thread, and while it
/// does the connection reads but processes nothing. Reading goes on while output waits to be written, so
/// two ends writing to each other at once never each wait for the other to read; a counterparty
/// that stops reading altogether is disconnected once 16 MiB of output is waiting for it.
///
/// Works with any transport (plain TCP, TLS, in-memory duplex), so custom transports can reuse
/// the engine without going through [`Acceptor`](crate::Acceptor) or
/// [`Initiator`](crate::Initiator).
///
/// # Errors
///
/// The transport's error, if reading or writing failed, or [`TimedOut`](io::ErrorKind::TimedOut) if
/// the counterparty stopped reading.
pub async fn run<S>(stream: S, session: Session, commands: CommandReceiver) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    run_tracked(stream, session, commands, &mut false, None).await
}

/// [`run`], also recording in `logged_on` whether the session ever logged on, and following
/// `shutdown` (from an Acceptor or Initiator): logging out once it starts, and closing the
/// connection at once, whatever it's doing, if shutdown gives up waiting.
///
/// Runs inside a `session` span whose `id` field is filled in once the session knows its ID.
pub(crate) async fn run_tracked<S>(
    stream: S,
    session: Session,
    commands: CommandReceiver,
    logged_on: &mut bool,
    shutdown: Option<Signal>,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let span = tracing::info_span!("session", id = tracing::field::Empty);
    let mut closing = shutdown.clone();
    async {
        tokio::select! {
            // Boxed: its state is about 18 KB, which would otherwise be inline in every future that
            // awaits a connection (an Initiator's run, say), and copied as each is moved.
            result = Box::pin(drive(stream, session, commands, logged_on, shutdown)) => result,
            // Dropping the driver drops the session, which notifies the application, and the
            // stream, which closes the connection.
            () = async { closing.as_mut().expect("guarded by is_some").closing().await }, if closing.is_some() => {
                warn!("closing the connection: shutdown timed out waiting for the logout");
                Ok(())
            }
        }
    }
    .instrument(span)
    .await
}

async fn drive<S>(
    stream: S,
    session: Session,
    commands: CommandReceiver,
    logged_on: &mut bool,
    mut shutdown: Option<Signal>,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let (mut reader, mut writer) = tokio::io::split(stream);
    let mut d = Driver::new(session, commands);
    d.start(&mut shutdown);
    match serve(&mut d, &mut reader, &mut writer, logged_on, shutdown).await {
        Ok(Ended::Closed) => {
            // Everything the session sent, a Logout before a close included, has gone. Release
            // the session (and its store) before the peer sees the close, so an immediate
            // reconnect can log on again.
            drop(d);
            let _ = writer.shutdown().await;
            Ok(())
        }
        result => {
            d.session.on_disconnect(Instant::now().into_std());
            result.map(|_| ())
        }
    }
}

/// How [`serve`] ended without an error.
enum Ended {
    /// The session closed and everything it sent has been written.
    Closed,
    /// The counterparty closed the connection.
    Lost,
}

/// Runs the connection until the session closes and its output has gone, or the transport ends.
async fn serve<R, W>(
    d: &mut Driver,
    reader: &mut R,
    writer: &mut W,
    logged_on: &mut bool,
    mut shutdown: Option<Signal>,
) -> io::Result<Ended>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let timer = tokio::time::sleep(MAX_TIMER_SLEEP);
    tokio::pin!(timer);
    loop {
        *logged_on |= d.session.has_logged_on();
        d.process(reader)?;
        d.stage_output();
        d.write_now(writer)?;
        d.check_backlog()?;
        let closed = d.session.is_closed();
        // While the store works, everything but reading and writing waits.
        let committing = d.store.is_some();
        if closed && d.unwritten() == 0 && !d.unflushed && !committing {
            return Ok(Ended::Closed);
        }
        d.bring_timer_forward(timer.as_mut());

        // While resending, input waits in the buffer and commands in their queue, so nothing new
        // goes out in the middle of the range; each step waits until the last has been written.
        // While committing, everything but reading and writing waits.
        let resending = d.session.is_resending();
        let pending = &d.outbox[d.written..];
        let nothing_pending = pending.is_empty();
        // Application sends are taken once logged on (until then they wait in their bounded
        // queue), and not while resending or with output backed up, or while the outbound window
        // is full.
        let takes_sends =
            !closed && !committing && d.session.has_logged_on() && !resending && d.unwritten() < COMMANDS_PAUSE_AT;
        let (sends, sends_free_at) = sends_this_time(&d.session, &d.commands, takes_sends);
        // While the inbound window holds input, the socket isn't read: input waits there, in
        // order, and TCP slows the counterparty, rather than filling `buf` (MAX_UNPROCESSED).
        let input_held = input_held_until(&d.session);
        // Borrowed apart from `d`, for a future that only runs `if committing`: select! makes
        // every branch's future, and an async block would borrow all of `d`.
        let store = &mut d.store;
        tokio::select! {
            // Write what the stream takes, then flush: buffering transports (TLS in particular)
            // may hold written data until flushed. Each is cancel-safe, so another branch
            // finishing first loses nothing.
            result = async {
                if pending.is_empty() { writer.flush().await.map(|()| 0) } else { writer.write(pending).await }
            }, if !nothing_pending || d.unflushed => {
                match result? {
                    0 if !nothing_pending => return Err(io::ErrorKind::WriteZero.into()),
                    0 => d.unflushed = false,
                    n => {
                        d.written += n;
                        d.unflushed = true;
                    }
                }
            }
            // Not while input is held, so a peer that closes meanwhile is noticed once it ends.
            read = reader.read_buf(&mut d.buf), if input_held.is_none() => {
                match read? {
                    0 => return Ok(Ended::Lost),
                    read => d.on_read(read, closed, resending || committing),
                }
            }
            // Logout and operator commands, whatever else is going on (a Logout once the sends
            // queued before it have been taken, which the outbound limit may slow); application
            // sends when `takes_sends` and the outbound window allows.
            Some(next) = d.commands.next_with(sends), if !closed && !committing => d.on_next(next),
            // The outbound window has freed up for the sends it held: they're taken next time
            // round. Not the timer, which a stuck deadline can hold at the ceiling.
            () = async {
                tokio::time::sleep_until(Instant::from_std(sends_free_at.expect("guarded by is_some"))).await;
            }, if sends_free_at.is_some() => {}
            // The inbound window has freed up: next time round, the input it held is fed and the
            // socket read again. Not the timer, as for sends.
            () = async {
                tokio::time::sleep_until(Instant::from_std(input_held.expect("guarded by is_some"))).await;
            }, if input_held.is_some() => {}
            // One step of the resend each time round, once the last step has been written.
            () = std::future::ready(()), if resending && nothing_pending && !closed && !committing => {
                d.session.on_resume(Instant::now().into_std());
            }
            // Once only: after that the session's logout (or its timeout) ends the connection.
            text = async { shutdown.as_mut().expect("guarded by is_some").started().await }, if shutdown.is_some() && !closed && !committing => {
                shutdown = None;
                d.session.on_shutdown(text.as_deref(), Instant::now().into_std());
            }
            // The store's commit has ended, and what it covers can be written; or its read, and
            // the resend step goes on.
            done = async { store.as_mut().expect("guarded by is_some").finished().await }, if committing => {
                d.on_store_done(done);
            }
            () = &mut timer, if !closed && !committing => d.on_timer(timer.as_mut()),
        }
    }
}

/// A connection's state between wake-ups: the session, what's been read and not yet handled,
/// what it sent and hasn't been written, and the store's work under way.
struct Driver {
    session: Session,
    commands: CommandReceiver,
    /// After each read it keeps only an incomplete frame, which the codec caps at MAX_BODY_LENGTH
    /// plus header and trailer, so it grows to no more than that and one read, except during a
    /// resend, when input waits here (up to MAX_UNPROCESSED).
    buf: Vec<u8>,
    /// Input read during a resend or a commit, or held by an inbound Delay limit, waiting in `buf`
    /// until it ends.
    deferred: bool,
    /// Every inbound frame is decoded into this one message, which keeps its allocations.
    scratch: Message,
    /// Bytes read before the session is bound (an acceptor's Logon) are attributed once it is.
    unattributed_bytes: usize,
    /// What the session sent, written as the stream takes it while the driver goes on reading:
    /// `outbox[written..]` is still to go, and a flush is owed once it has (up to MAX_UNWRITTEN).
    outbox: Vec<u8>,
    written: usize,
    unflushed: bool,
    /// A deadline the session left in the past, which the timer waits out at its ceiling.
    stuck: Option<Instant>,
    /// The store's commit or read under way, off the connection's task: the session waits for it.
    store: Option<StoreTask>,
    /// The store's commits wait (an fsync), so input that has already arrived is worth taking
    /// into a batch before committing it. Commits made at once aren't: smaller batches let the
    /// counterparty start on the replies sooner (see READ_BUFFER_SIZE).
    commits_wait: bool,
    /// Whether application sends have waited for the outbound window since the queue was last
    /// empty: those taken meanwhile count as throttled.
    sends_held: bool,
    timings: Timings,
}

impl Driver {
    fn new(session: Session, commands: CommandReceiver) -> Self {
        Self {
            session,
            commands,
            buf: Vec::with_capacity(READ_BUFFER_SIZE),
            deferred: false,
            scratch: Message::default(),
            unattributed_bytes: 0,
            outbox: Vec::new(),
            written: 0,
            unflushed: false,
            stuck: None,
            store: None,
            commits_wait: false,
            sends_held: false,
            timings: Timings::default(),
        }
    }

    /// Starts the session: connects it, or, once shutdown has started, closes it without logging
    /// on.
    fn start(&mut self, shutdown: &mut Option<Signal>) {
        match shutdown.as_ref().and_then(Signal::started_now) {
            Some(text) => {
                *shutdown = None;
                self.session.on_shutdown(text.as_deref(), Instant::now().into_std());
            }
            None => self.session.on_connect(Instant::now().into_std()),
        }
    }

    /// Only ever brings the timer forward to the session's next deadline: most sends and
    /// receives push deadlines later, and a timer that fires early just finds nothing due.
    fn bring_timer_forward(&self, timer: Pin<&mut Sleep>) {
        if let Some(deadline) = self.session.next_deadline().map(Instant::from_std)
            && deadline < timer.deadline()
            && self.stuck != Some(deadline)
        {
            timer.reset(deadline);
        }
    }

    /// The timer has fired: the session's deadlines are checked, and the timer set for the next.
    fn on_timer(&mut self, timer: Pin<&mut Sleep>) {
        let now = Instant::now();
        timer.reset(now + MAX_TIMER_SLEEP);
        self.session.on_timer(now.into_std());
        // A deadline on_timer left in the past waits for the ceiling rather than spin.
        self.stuck = self.session.next_deadline().map(Instant::from_std).filter(|deadline| *deadline <= now);
    }

    /// Bytes the session sent that haven't been written yet.
    fn unwritten(&self) -> usize {
        self.outbox.len() - self.written
    }

    /// Whether input waiting in `buf` can be fed to the session now.
    fn can_feed(&self) -> bool {
        !self.session.is_resending()
            && !self.session.is_closed()
            && !self.session.is_waiting_on_store()
            && input_held_until(&self.session).is_none()
    }

    /// Everything the session can do before the driver waits: input that waited, control
    /// commands, input already arrived (when commits wait), and the store's next job.
    fn process<R: AsyncRead + Unpin>(&mut self, reader: &mut R) -> io::Result<()> {
        loop {
            // Input that waited for a resend, a commit or the inbound window is processed once it
            // has ended.
            if self.deferred && self.can_feed() {
                self.deferred = feed(&mut self.session, &mut self.buf, &mut self.scratch, Instant::now().into_std());
            }
            // A Logout that was waiting for the sends queued before it, now they've been taken.
            while !self.session.is_closed()
                && !self.session.is_waiting_on_store()
                && let Some(command) = self.commands.try_control()
            {
                self.session.on_command(command, Instant::now().into_std());
            }
            self.read_arrived(reader)?;
            self.start_store_job();
            // Input that stopped for a commit made at once goes on: each time round, the commit
            // opens the window it stopped for, or is left under way. Input the inbound window
            // holds waits for the select's wake-up.
            if !(self.deferred && self.store.is_none() && self.can_feed()) {
                return Ok(());
            }
        }
    }

    /// Input that has already arrived joins this batch rather than waiting out its commit to make
    /// one of its own, so a burst is committed together.
    fn read_arrived<R: AsyncRead + Unpin>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut reads = 0;
        while self.commits_wait
            && reads < MAX_READS_PER_BATCH
            && self.store.is_none()
            && !self.deferred
            && !self.session.is_closed()
            && !self.session.is_resending()
            && self.buf.len() < MAX_UNPROCESSED
            && input_held_until(&self.session).is_none()
            && let Some(read) = poll_once(reader.read_buf(&mut self.buf))
        {
            reads += 1;
            match read? {
                // The select's read sees the end of the input again, after this batch's commit.
                0 => break,
                read => {
                    self.unattributed_bytes += read;
                    let now = Instant::now();
                    self.timings.read(&self.session, now);
                    self.deferred = feed(&mut self.session, &mut self.buf, &mut self.scratch, now.into_std());
                }
            }
        }
        Ok(())
    }

    /// Whatever the session did since the last commit is committed before it's written; a resend
    /// step the store reads with a job, or a logon whose log it opens with one, waits for it.
    fn start_store_job(&mut self) {
        if self.store.is_some() {
            return;
        }
        if let Some(job) = self.session.take_commit(Instant::now().into_std()) {
            self.store = Some(StoreTask::Commit(job.spawn()));
            self.commits_wait = true;
            self.timings.commit_started(&self.session, self.deferred);
        } else if let Some(job) = self.session.take_fetch() {
            self.store = Some(StoreTask::Fetch(job.spawn()));
        } else if let Some(job) = self.session.take_open() {
            self.store = Some(StoreTask::Open(job.spawn()));
        }
        if self.store.is_none() && !self.deferred {
            // Committed at once: what was read has all been handled and can be written.
            self.timings.handled(&self.session);
        }
    }

    /// Moves what the session sent into the outbox, counting the bytes either way.
    fn stage_output(&mut self) {
        if let Some(metrics) = self.session.metrics() {
            metrics.bytes_received(std::mem::take(&mut self.unattributed_bytes));
            metrics.bytes_sent(self.session.output().len());
        }
        if !self.session.output().is_empty() {
            if self.written == self.outbox.len() {
                self.outbox.clear();
                self.written = 0;
            }
            self.outbox.extend_from_slice(self.session.output());
            self.session.clear_output();
        }
    }

    /// Most writes complete at once: tries, and a flush, before waiting for one in the select.
    fn write_now<W: AsyncWrite + Unpin>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.written < self.outbox.len()
            && let Some(result) = poll_once(writer.write(&self.outbox[self.written..]))
        {
            match result? {
                0 => return Err(io::ErrorKind::WriteZero.into()),
                n => self.written += n,
            }
            self.unflushed = true;
        }
        if self.written == self.outbox.len()
            && self.unflushed
            && let Some(result) = poll_once(writer.flush())
        {
            result?;
            self.unflushed = false;
        }
        debug_assert!(self.written <= self.outbox.len());
        Ok(())
    }

    /// Fails if output or input has backed up past its limit: the counterparty has stopped
    /// reading.
    fn check_backlog(&self) -> io::Result<()> {
        let (unwritten, unprocessed) = (self.unwritten(), self.buf.len());
        if unwritten > MAX_UNWRITTEN || unprocessed > MAX_UNPROCESSED {
            warn!(unwritten, unprocessed, "the counterparty has stopped reading; disconnecting");
            return Err(io::Error::new(io::ErrorKind::TimedOut, "the counterparty has stopped reading"));
        }
        Ok(())
    }

    /// `read` bytes arrived in `buf`: fed to the session, or left for later if it's `waiting`
    /// (resending or committing), or dropped once it's `closed`.
    fn on_read(&mut self, read: usize, closed: bool, waiting: bool) {
        debug_assert!(read > 0);
        self.unattributed_bytes += read;
        if closed {
            // Only the session's last output is still to go; what arrives is read so the
            // counterparty's writes don't block, and dropped.
            self.buf.clear();
        } else if waiting || self.deferred {
            self.deferred = true;
            if self.session.times_latency() {
                self.timings.read(&self.session, Instant::now());
            }
        } else {
            let now = Instant::now();
            self.timings.read(&self.session, now);
            self.deferred = feed(&mut self.session, &mut self.buf, &mut self.scratch, now.into_std());
        }
    }

    /// A command, or a send noticed waiting for the outbound window.
    fn on_next(&mut self, next: Next) {
        // A noticed send arms the wait for the window, next time round.
        let Next::Command(command) = next else {
            self.sends_held = true;
            self.session.on_sends_held();
            return;
        };
        let now = Instant::now().into_std();
        let sending = matches!(command, Command::Send(..));
        let mut sends = u64::from(sending);
        self.session.on_command(command, now);
        // Take whatever else is already queued, so a burst of sends becomes one write, up to what
        // the outbound window allows.
        for _ in (1..MAX_COMMANDS_PER_BATCH).take_while(|_| sending) {
            if !self.session.can_send(now) {
                break;
            }
            let Some(command) = self.commands.try_send() else { break };
            self.session.on_command(command, now);
            sends += 1;
        }
        // The noticed send and those queued behind it waited. Sends queued once the queue has
        // emptied haven't.
        if self.sends_held && sends > 0 {
            if let Some(metrics) = self.session.metrics() {
                metrics.throttled_outbound(sends);
            }
            self.sends_held = self.commands.has_sends();
        }
    }

    /// The store's job has ended.
    fn on_store_done(&mut self, done: StoreDone) {
        debug_assert!(self.store.is_some());
        self.store = None;
        let now = Instant::now();
        if let StoreDone::Committed(result) = &done {
            self.timings.commit_finished(&self.session, result.is_ok(), now);
        }
        done.deliver(&mut self.session, now.into_std());
    }
}

/// What the latency histograms time, while the session records them (see
/// [`telemetry`](crate::telemetry#latency-histograms)); otherwise nothing is set and the clock isn't
/// read for them.
#[derive(Default)]
struct Timings {
    /// When the oldest input not yet handled and committed was read.
    read_at: Option<Instant>,
    /// The commit under way: when it started, and when the oldest input it covers was read.
    commit: Option<(Instant, Option<Instant>)>,
}

impl Timings {
    /// Input was read at `now`.
    fn read(&mut self, session: &Session, now: Instant) {
        if session.times_latency() {
            self.read_at.get_or_insert(now);
        }
    }

    /// A commit job has started, covering the input read so far. Input left `deferred` waits for a
    /// later commit, so its read time is kept for that one too.
    fn commit_started(&mut self, session: &Session, deferred: bool) {
        if session.times_latency() {
            let read_at = if deferred { self.read_at } else { self.read_at.take() };
            self.commit = Some((Instant::now(), read_at));
        }
    }

    /// The commit job ended at `now`, and what it covers can be written if it `succeeded`.
    fn commit_finished(&mut self, session: &Session, succeeded: bool, now: Instant) {
        if let Some(latency) = session.latency_metrics()
            && let Some((started, read_at)) = self.commit.take()
        {
            latency.commit(now.saturating_duration_since(started));
            if succeeded && let Some(read_at) = read_at {
                latency.read_to_write(now.saturating_duration_since(read_at));
            }
        }
    }

    /// Everything read has been handled and committed at once, so can be written.
    fn handled(&mut self, session: &Session) {
        if let Some(latency) = session.latency_metrics()
            && let Some(read_at) = self.read_at.take()
        {
            latency.read_to_write(Instant::now().saturating_duration_since(read_at));
        }
    }
}

/// The store's work under way, off the connection's task.
enum StoreTask {
    Commit(JoinHandle<io::Result<()>>),
    Fetch(JoinHandle<io::Result<SentMessages>>),
    Open(JoinHandle<io::Result<Box<dyn SessionLog>>>),
}

/// The result of a [`StoreTask`], for the session.
enum StoreDone {
    Committed(io::Result<()>),
    Fetched(io::Result<SentMessages>),
    Opened(io::Result<Box<dyn SessionLog>>),
}

impl StoreTask {
    /// Waits for the task. Cancel-safe: the task runs on whether or not this is polled.
    async fn finished(&mut self) -> StoreDone {
        match self {
            Self::Commit(task) => StoreDone::Committed(joined(task.await)),
            Self::Fetch(task) => StoreDone::Fetched(joined(task.await)),
            Self::Open(task) => StoreDone::Opened(joined(task.await)),
        }
    }
}

impl StoreDone {
    fn deliver(self, session: &mut Session, now: std::time::Instant) {
        match self {
            Self::Committed(result) => session.on_committed(result, now),
            Self::Fetched(result) => session.on_fetched(result, now),
            Self::Opened(result) => session.on_opened(result, now),
        }
    }
}

/// A store task's result, or its panic as an error.
fn joined<T>(result: Result<io::Result<T>, tokio::task::JoinError>) -> io::Result<T> {
    result.unwrap_or_else(|e| Err(io::Error::other(format!("the store's job failed: {e}"))))
}

/// What the command branch does with application sends this time round, and when to wake for
/// sends the outbound window holds, if any. The clock is read only while the window is full.
///
/// With the window full, a send that arrives is noticed, not taken, so the driver goes round and
/// waits for the window rather than for the timer. The driver, not the session's deadline, owns
/// that wake-up: only it knows whether sends are waiting, and waking for the window with none
/// would cost a wake-up per message at a steady rate near the limit.
fn sends_this_time(
    session: &Session,
    commands: &CommandReceiver,
    takes_sends: bool,
) -> (Sends, Option<std::time::Instant>) {
    if !takes_sends {
        return (Sends::Ignore, None);
    }
    let Some(free_at) = session.send_free_at() else { return (Sends::Take, None) };
    let now = Instant::now().into_std();
    if free_at <= now {
        return (Sends::Take, None);
    }
    debug_assert!(!session.can_send(now), "a window that frees up later is full now");
    (Sends::Notice, commands.has_sends().then_some(free_at))
}

/// When input held by the inbound window (see [`Session::input_free_at`]) may go on, if it's held
/// now. The clock is read only while the window is full. As for sends, the driver owns the
/// wake-up, not the session's deadline: with no input held, waking for the window would cost a
/// wake-up per message at a steady rate near the limit, and a deadline the timer finds stuck in the
/// past waits for the once-a-second ceiling.
fn input_held_until(session: &Session) -> Option<std::time::Instant> {
    let free_at = session.input_free_at()?;
    (free_at > Instant::now().into_std()).then_some(free_at)
}

/// Polls `future` once, without waiting: its output if it's ready. Write and flush are
/// cancel-safe, so one that isn't ready has done nothing, and is tried again in the select.
fn poll_once<F: Future>(future: F) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    match future.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(output) => Some(output),
        std::task::Poll::Pending => None,
    }
}

/// Feeds `session` the complete messages in `buf`, each decoded into `scratch`, then drops the
/// consumed bytes once; a partial message at the end stays for the next read. The messages
/// arrived together, so they share one timestamp. Stops at a message that starts a resend, and
/// before one that must wait for a commit or for the inbound window: returns true if input is left
/// waiting.
fn feed(session: &mut Session, buf: &mut Vec<u8>, scratch: &mut Message, now: std::time::Instant) -> bool {
    let mut consumed = 0;
    // When the next message started, for the latency histograms: each message's end starts the
    // next, so timing them costs one clock read each.
    let mut started = now;
    // Ends: a message consumes its frame, garbled bytes skip at least one, and a resend or a
    // commit stops it.
    let deferred = loop {
        if session.is_resending() || session.is_closed() {
            break session.is_resending();
        }
        // The message that fills the inbound window has been handled; the next one waits.
        if consumed < buf.len() && (!session.ready_for_input() || session.input_free_at().is_some_and(|at| at > now)) {
            break true;
        }
        match decode_into(&buf[consumed..], session.data_fields(), scratch) {
            DecodedInto::Message(len) => {
                consumed += len;
                debug!(target: "turbojet::messages", direction = "in", "{}", scratch.redacted());
                session.on_message(scratch, now);
                if let Some(latency) = session.latency_metrics() {
                    let ended = Instant::now().into_std();
                    latency.inbound_message(ended.saturating_duration_since(started));
                    started = ended;
                }
            }
            DecodedInto::Incomplete => break false,
            DecodedInto::Garbled { skip, reason } => {
                warn!("discarding {skip} garbled bytes: {reason}");
                telemetry::garbled_message();
                consumed += skip;
                if session.latency_metrics().is_some() {
                    started = Instant::now().into_std();
                }
            }
        }
    };
    buf.drain(..consumed);
    deferred
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context as TaskContext, Poll};

    use tokio::io::{DuplexStream, ReadBuf, duplex};

    use super::*;
    use crate::application::{Application, Context, MessageReject};
    use crate::codec::{Decoded, encode, frame_with_raw_field};
    use crate::fields::MsgType;
    use crate::message::{tags, utc_timestamp};
    use crate::registry::SendError;
    use crate::registry::SessionRegistry;
    use crate::session::SessionConfig;
    use crate::store::SessionId;

    /// A stream that counts the write calls that move data.
    struct CountingStream {
        inner: DuplexStream,
        writes: Arc<AtomicUsize>,
    }

    impl AsyncRead for CountingStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut TaskContext<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for CountingStream {
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>, data: &[u8]) -> Poll<io::Result<usize>> {
            let result = Pin::new(&mut self.inner).poll_write(cx, data);
            if let Poll::Ready(Ok(n)) = result
                && n > 0
            {
                self.writes.fetch_add(1, Ordering::SeqCst);
            }
            result
        }
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_flush(cx)
        }
        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_shutdown(cx)
        }
    }

    /// Acknowledges every application message with an ExecutionReport.
    struct Acker;

    impl Application for Acker {
        fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
            ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
            Ok(())
        }
    }

    fn from_peer(seq: u64, body: Message) -> Vec<u8> {
        encode(&with_peer_header(seq, body)).unwrap()
    }

    /// `body` with the peer's header, ready to encode.
    fn with_peer_header(seq: u64, body: Message) -> Message {
        let mut msg = Message::default();
        msg.push(tags::BEGIN_STRING, "FIX.4.2");
        msg.push(tags::MSG_TYPE, body.msg_type());
        msg.push(tags::SENDER_COMP_ID, "PEER");
        msg.push(tags::TARGET_COMP_ID, "US");
        msg.push(tags::MSG_SEQ_NUM, seq);
        msg.push(tags::SENDING_TIME, utc_timestamp());
        for (tag, value) in body.fields().filter(|(t, _)| *t != tags::MSG_TYPE) {
            msg.push(tag, value);
        }
        msg
    }

    /// Reads from `peer` until `count` complete messages have arrived.
    async fn receive<R: AsyncRead + Unpin>(peer: &mut R, buf: &mut Vec<u8>, count: usize) -> Vec<Message> {
        let mut messages = Vec::new();
        while messages.len() < count {
            match crate::codec::decode(buf) {
                Decoded::Message(msg, len) => {
                    buf.drain(..len);
                    messages.push(msg);
                }
                Decoded::Incomplete => {
                    let read = tokio::time::timeout(Duration::from_secs(5), peer.read_buf(buf)).await;
                    assert!(read.expect("timed out").expect("read failed") > 0, "connection closed");
                }
                Decoded::Garbled { reason, .. } => panic!("garbled output: {reason}"),
            }
        }
        messages
    }

    /// Runs an acceptor and logs the peer on with HeartBtInt=1. Returns the peer's end and read
    /// buffer once the Logon reply has arrived.
    async fn logged_on_with_one_second_heartbeats() -> (DuplexStream, Vec<u8>) {
        let (peer, buf, _) = logged_on_with(SessionConfig::new("FIX.4.2", "US"), 1).await;
        (peer, buf)
    }

    /// Runs an acceptor with `config` and logs the peer on with `heartbeat` as HeartBtInt.
    /// Returns the peer's end and read buffer once the Logon reply has arrived, and a handle.
    async fn logged_on_with(config: SessionConfig, heartbeat: u64) -> (DuplexStream, Vec<u8>, crate::SessionHandle) {
        logged_on_following(config, heartbeat, None).await
    }

    /// [`logged_on_with`], the connection following `shutdown`.
    async fn logged_on_following(
        config: SessionConfig,
        heartbeat: u64,
        shutdown: Option<Signal>,
    ) -> (DuplexStream, Vec<u8>, crate::SessionHandle) {
        logged_on_over(1 << 20, config, heartbeat, shutdown).await
    }

    /// [`logged_on_following`] over a stream that buffers `capacity` bytes each way.
    async fn logged_on_over(
        capacity: usize,
        config: SessionConfig,
        heartbeat: u64,
        shutdown: Option<Signal>,
    ) -> (DuplexStream, Vec<u8>, crate::SessionHandle) {
        let (peer, buf, registry, _) = logged_on_running(capacity, config, Arc::new(Acker), heartbeat, shutdown).await;
        (peer, buf, registry.handle(peer_session()))
    }

    /// [`logged_on_over`] with `app`: the peer's end and read buffer, the registry, and the task
    /// running the connection.
    async fn logged_on_running(
        capacity: usize,
        config: SessionConfig,
        app: Arc<dyn Application>,
        heartbeat: u64,
        shutdown: Option<Signal>,
    ) -> (DuplexStream, Vec<u8>, Arc<SessionRegistry>, JoinHandle<io::Result<()>>) {
        let (ours, mut peer) = duplex(capacity);
        let registry = Arc::new(SessionRegistry::default());
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::acceptor(config, registry.clone(), app, now);
        let task = tokio::spawn(async move { run_tracked(ours, session, commands, &mut false, shutdown).await });

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, heartbeat);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);
        (peer, buf, registry, task)
    }

    /// The session [`logged_on_over`] logs on, as the acceptor sees it.
    fn peer_session() -> SessionId {
        SessionId { begin_string: "FIX.4.2".into(), sender_comp_id: "US".into(), target_comp_id: "PEER".into() }
    }

    /// An acceptor config sending at most `messages` application messages per `per`.
    fn with_outbound_limit(messages: u32, per: Duration) -> SessionConfig {
        let mut config = SessionConfig::new("FIX.4.2", "US");
        config.outbound_limit = Some(crate::RateLimit::new(messages, per));
        config
    }

    /// Reads `count` messages from `peer`, each with how long after `start` it was read. Time is
    /// paused in the tests that use it, so that is when it was written.
    async fn receive_timed<R: AsyncRead + Unpin>(
        peer: &mut R,
        buf: &mut Vec<u8>,
        count: usize,
        start: Instant,
    ) -> Vec<(Message, Duration)> {
        let mut timed = Vec::new();
        for _ in 0..count {
            let msg = receive(peer, buf, 1).await.remove(0);
            timed.push((msg, start.elapsed()));
        }
        timed
    }

    fn order(id: &str) -> Message {
        Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id)
    }

    /// Queued sends past the outbound limit wait for the window, and go out, in order, as soon as
    /// it frees: no 200 ms holds more than 5, and the queue drains in (20/5 - 1) windows, not a
    /// second each (the timer's ceiling).
    #[tokio::test(start_paused = true)]
    async fn queued_sends_go_out_no_faster_than_the_outbound_limit() {
        const LIMIT: usize = 5;
        const WINDOW: Duration = Duration::from_millis(200);
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(5, WINDOW), 30).await;
        let start = Instant::now();
        for i in 0..20 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }

        let sent = receive_timed(&mut peer, &mut buf, 20, start).await;
        let ids: Vec<_> = sent.iter().map(|(m, _)| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect();
        let expected: Vec<_> = (0..20).map(|i| format!("O{i}")).collect();
        assert_eq!(ids, expected, "all sent, in order");
        for (earlier, later) in sent.iter().zip(&sent[LIMIT..]) {
            assert!(later.1 - earlier.1 >= WINDOW, "more than {LIMIT} in a window: {:?} and {:?}", earlier.1, later.1);
        }
        let times: Vec<_> = sent.iter().map(|(_, at)| *at).collect();
        let windows: Vec<_> = (0..20u32).map(|i| WINDOW * (i / 5)).collect();
        assert_eq!(times, windows, "each 5 as soon as the window frees");
        assert_eq!(sent.last().unwrap().1, Duration::from_millis(600));
    }

    /// A Logout asked for after sends the outbound limit holds goes out after them, once they've
    /// drained, while the control queue goes on being read.
    #[tokio::test(start_paused = true)]
    async fn a_logout_waits_for_the_sends_the_limit_holds() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(5, Duration::from_millis(200)), 30).await;
        let start = Instant::now();
        let receipts: Vec<_> = (0..12).map(|i| handle.send(order(&format!("O{i}"))).unwrap()).collect();
        handle.logout(Some("done")).unwrap();
        // An operator command is answered at once, while the sends are held.
        let numbers = tokio::time::timeout(Duration::from_millis(1), handle.sequence_numbers()).await;
        assert!(numbers.expect("answered while sends wait").is_ok());

        let sent = receive_timed(&mut peer, &mut buf, 13, start).await;
        let seen: Vec<_> = sent.iter().map(|(m, _)| (m.msg_type(), m.get(tags::CL_ORD_ID).map(String::from))).collect();
        let mut expected: Vec<_> = (0..12).map(|i| (MsgType::NewOrderSingle, Some(format!("O{i}")))).collect();
        expected.push((MsgType::Logout, None));
        assert_eq!(seen, expected);
        assert_eq!(sent[12].1, Duration::from_millis(400), "with the last of the sends");
        let mut stored = Vec::new();
        for receipt in receipts {
            stored.push(receipt.await.unwrap());
        }
        assert_eq!(stored, (2..14).collect::<Vec<_>>());

        peer.write_all(&from_peer(2, Message::new(MsgType::Logout))).await.unwrap();
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
    }

    /// A send that arrives once the window has filled, with nothing queued, goes out as soon as
    /// the window frees, not when the timer next fires (a second after the last time it did).
    #[tokio::test(start_paused = true)]
    async fn a_send_after_a_full_window_waits_for_the_window_not_the_timer() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(5, Duration::from_millis(200)), 30).await;
        let start = Instant::now();
        for i in 0..5 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }
        assert!(receive_timed(&mut peer, &mut buf, 5, start).await.iter().all(|(_, at)| at.is_zero()));
        tokio::time::sleep(Duration::from_millis(10)).await;
        handle.send(order("late")).unwrap();

        let (late, at) = receive_timed(&mut peer, &mut buf, 1, start).await.remove(0);
        assert_eq!(late.get(tags::CL_ORD_ID), Some("late"));
        assert_eq!(at, Duration::from_millis(200));
    }

    /// With 5 per 200 ms: 7 sends at once, of which 2 wait; a second later 5 more, which fill
    /// the window without waiting, then 1 more, which waits. 3 sends wait, in two holds.
    async fn sends_waiting_in_two_holds() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(5, Duration::from_millis(200)), 30).await;
        let start = Instant::now();
        for i in 0..7 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }
        receive_timed(&mut peer, &mut buf, 7, start).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        for i in 7..12 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }
        receive_timed(&mut peer, &mut buf, 5, start).await;
        tokio::time::sleep(Duration::from_millis(10)).await;
        handle.send(order("late")).unwrap();
        let (late, at) = receive_timed(&mut peer, &mut buf, 1, start).await.remove(0);
        assert_eq!(late.get(tags::CL_ORD_ID), Some("late"));
        assert_eq!(at, Duration::from_millis(1_400), "a window after the second burst");
    }

    /// Each send that waited for the outbound window counts once: those that didn't, don't.
    #[cfg(feature = "metrics")]
    #[tokio::test(start_paused = true)]
    async fn sends_that_wait_for_the_outbound_window_are_counted() {
        let recorder = metrics_util::debugging::DebuggingRecorder::new();
        let snapshotter = recorder.snapshotter();
        // Thread-local: the test runtime is single-threaded, so the connection task records here.
        let _guard = metrics::set_default_local_recorder(&recorder);
        sends_waiting_in_two_holds().await;
        let throttled: Vec<_> = snapshotter
            .snapshot()
            .into_vec()
            .into_iter()
            .filter(|(key, ..)| key.key().name() == "turbojet_throttled_total")
            .map(|(key, _, _, value)| {
                let direction = key.key().labels().find(|l| l.key() == "direction").unwrap().value().to_string();
                (direction, value)
            })
            .collect();
        let count = |direction: &str| match throttled.iter().find(|(d, _)| d == direction) {
            Some((_, metrics_util::debugging::DebugValue::Counter(n))) => *n,
            other => panic!("{direction}: {other:?}"),
        };
        assert_eq!(count("outbound"), 3);
        assert_eq!(count("inbound"), 0);
    }

    /// Sends waiting for the outbound window log one warning on the connection, not one per
    /// send or per hold.
    #[tokio::test(start_paused = true)]
    async fn sends_waiting_for_the_outbound_window_warn_once() {
        let captured = Captured::default();
        let sink = captured.clone();
        let subscriber = tracing_subscriber::fmt().with_ansi(false).with_writer(move || sink.clone()).finish();
        // Thread-local: the test runtime is single-threaded, so the connection task logs here.
        // While only one dispatcher exists, tracing caches a new callsite's interest from the
        // default of whichever thread reaches it first: another test's thread, with none, would
        // turn it off here too. A second keeps it asking every dispatcher, this one included.
        let _second = tracing::Dispatch::new(tracing::subscriber::NoSubscriber::default());
        let _guard = tracing::subscriber::set_default(subscriber);
        sends_waiting_in_two_holds().await;
        let logged = String::from_utf8(captured.0.lock().unwrap().clone()).unwrap();
        let warnings: Vec<_> = logged.lines().filter(|line| line.contains("rate limit")).collect();
        assert_eq!(warnings.len(), 1, "{logged}");
        assert!(warnings[0].contains("WARN"), "{logged}");
        assert!(warnings[0].contains("outbound rate limit 5/200ms reached: sends wait"), "{logged}");
    }

    /// A log sink shared between a subscriber and the test that reads it.
    #[derive(Clone, Default)]
    struct Captured(Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Captured {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// A Logout asked for while a send that arrived after the window filled waits for it, goes
    /// out after it: the send is first in line, though not yet taken.
    #[tokio::test(start_paused = true)]
    async fn a_logout_waits_for_a_send_that_arrived_after_the_window_filled() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(5, Duration::from_millis(200)), 30).await;
        let start = Instant::now();
        for i in 0..5 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }
        receive_timed(&mut peer, &mut buf, 5, start).await;
        tokio::time::sleep(Duration::from_millis(10)).await;
        let receipt = handle.send(order("late")).unwrap();
        // Long enough for the connection to notice the send.
        tokio::time::sleep(Duration::from_millis(10)).await;
        handle.logout(Some("done")).unwrap();
        let numbers = tokio::time::timeout(Duration::from_millis(1), handle.sequence_numbers()).await;
        assert!(numbers.expect("answered while the send waits").is_ok());

        let sent = receive_timed(&mut peer, &mut buf, 2, start).await;
        let seen: Vec<_> =
            sent.iter().map(|(m, at)| (m.msg_type(), m.get(tags::CL_ORD_ID).map(String::from), *at)).collect();
        let at = Duration::from_millis(200);
        assert_eq!(seen, [(MsgType::NewOrderSingle, Some("late".into()), at), (MsgType::Logout, None, at)]);
        assert_eq!(receipt.await, Ok(7));
    }

    /// Sends the window holds when shutdown starts are dropped at once, not a window later: the
    /// session is logging out, so they would never go.
    #[tokio::test(start_paused = true)]
    async fn sends_held_by_the_window_are_dropped_at_once_on_shutdown() {
        let shutdown = crate::shutdown::Shutdown::new();
        let config = with_outbound_limit(1, Duration::from_secs(60));
        let (mut peer, mut buf, handle) = logged_on_following(config, 30, Some(shutdown.signal())).await;
        let start = Instant::now();
        let receipts: Vec<_> = (0..3).map(|i| handle.send(order(&format!("O{i}"))).unwrap()).collect();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].get(tags::CL_ORD_ID), Some("O0"));
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Nothing is tracked, so this only starts the shutdown.
        shutdown.run(Some("bye"), Duration::from_secs(10)).await;
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logout);
        let mut outcomes = Vec::new();
        for receipt in receipts {
            outcomes.push(receipt.await);
        }
        assert_eq!(outcomes, [Ok(2), Err(crate::Dropped::LoggingOut), Err(crate::Dropped::LoggingOut)]);
        assert_eq!(start.elapsed(), Duration::from_millis(10), "at once, not a window later");
    }

    /// A send noticed with the window full, and one queued behind it, both learn that the
    /// connection ended.
    #[tokio::test(start_paused = true)]
    async fn sends_held_when_the_peer_disconnects_are_dropped_as_disconnected() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(1, Duration::from_secs(60)), 30).await;
        let first = handle.send(order("A")).unwrap();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].get(tags::CL_ORD_ID), Some("A"));
        assert_eq!(first.await, Ok(2));
        // B is noticed, and C waits in the queue behind it.
        let receipts = [handle.send(order("B")).unwrap(), handle.send(order("C")).unwrap()];
        tokio::time::sleep(Duration::from_millis(10)).await;

        drop(peer);
        for receipt in receipts {
            let outcome = tokio::time::timeout(Duration::from_secs(5), receipt).await;
            assert_eq!(outcome.expect("answered"), Err(crate::Dropped::Disconnected));
        }
    }

    /// Heartbeats go out when due while the outbound window, much longer than HeartBtInt, holds
    /// sends: holding sends doesn't hold the timer.
    #[tokio::test(start_paused = true)]
    async fn heartbeats_go_out_on_time_while_the_limit_holds_sends() {
        let (mut peer, mut buf, handle) = logged_on_with(with_outbound_limit(2, Duration::from_millis(4_500)), 1).await;
        let start = Instant::now();
        for i in 0..6 {
            handle.send(order(&format!("O{i}"))).unwrap();
        }

        let mut seen = Vec::new();
        let mut seq = 2;
        while seen.iter().filter(|(kind, _)| *kind == "order").count() < 6 {
            let (msg, at) = receive_timed(&mut peer, &mut buf, 1, start).await.remove(0);
            let kind = match msg.msg_type() {
                MsgType::NewOrderSingle => "order",
                MsgType::Heartbeat => "heartbeat",
                other => panic!("unexpected {other} at {at:?} after {seen:?}"),
            };
            // The peer answers everything with a Heartbeat, so it is never probed.
            peer.write_all(&from_peer(seq, Message::new(MsgType::Heartbeat))).await.unwrap();
            seq += 1;
            seen.push((kind, at.as_millis()));
        }
        assert_eq!(
            seen,
            [
                ("order", 0),
                ("order", 0),
                ("heartbeat", 1_000),
                ("heartbeat", 2_000),
                ("heartbeat", 3_000),
                ("heartbeat", 4_000),
                ("order", 4_500),
                ("order", 4_500),
                ("heartbeat", 5_500),
                ("heartbeat", 6_500),
                ("heartbeat", 7_500),
                ("heartbeat", 8_500),
                ("order", 9_000),
                ("order", 9_000),
            ]
        );
    }

    /// An acceptor config holding input once `messages` application messages arrive within `per`.
    fn with_inbound_delay(messages: u32, per: Duration) -> SessionConfig {
        let mut config = SessionConfig::new("FIX.4.2", "US");
        config.inbound_limit = Some(crate::InboundLimit::Delay(crate::RateLimit::new(messages, per)));
        config
    }

    /// A burst of 100 orders, written at once, is handed to the application in order, 10 as soon
    /// as each window frees, over 9 windows, without the connection dropping. Each order's
    /// ExecutionReport is written as it's handled, so when the peer reads it says when.
    #[tokio::test(start_paused = true)]
    async fn inbound_messages_over_the_delay_limit_wait_for_the_window() {
        const WINDOW: Duration = Duration::from_millis(200);
        let (mut peer, mut buf, _handle) = logged_on_with(with_inbound_delay(10, WINDOW), 30).await;
        let start = Instant::now();
        let burst: Vec<u8> = (0..100u64).flat_map(|i| from_peer(i + 2, order(&format!("O{i}")))).collect();
        peer.write_all(&burst).await.unwrap();

        let acked = receive_timed(&mut peer, &mut buf, 100, start).await;
        assert!(acked.iter().all(|(m, _)| m.msg_type() == MsgType::ExecutionReport));
        let ids: Vec<_> = acked.iter().map(|(m, _)| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect();
        let expected: Vec<_> = (0..100).map(|i| format!("O{i}")).collect();
        assert_eq!(ids, expected, "all handled, in order");
        let times: Vec<_> = acked.iter().map(|(_, at)| *at).collect();
        let windows: Vec<_> = (0..100u32).map(|i| WINDOW * (i / 10)).collect();
        assert_eq!(times, windows, "each 10 as soon as the window frees, not when the timer fires");
        assert_eq!(acked.last().unwrap().1, Duration::from_millis(1_800));

        // Still connected: a TestRequest is answered.
        peer.write_all(&from_peer(102, Message::new(MsgType::TestRequest).with(tags::TEST_REQ_ID, "up")))
            .await
            .unwrap();
        let answer = receive(&mut peer, &mut buf, 1).await.remove(0);
        assert_eq!(answer.msg_type(), MsgType::Heartbeat);
        assert_eq!(answer.get(tags::TEST_REQ_ID), Some("up"));
    }

    /// A Heartbeat the peer sends behind orders the window holds waits behind them: it's handled
    /// only once the last of them is. With the window (2.5s) longer than HeartBtInt (1s), the peer
    /// says nothing else, yet it's neither probed nor dropped: its silence counts from when input
    /// goes on, not from the last message handled. (The window isn't a whole number of seconds,
    /// so it never frees up just as one of our Heartbeats falls due, which would make the order
    /// of the two depend on the select.)
    #[tokio::test(start_paused = true)]
    async fn a_heartbeat_behind_held_orders_waits_for_them_and_the_peer_is_not_probed() {
        let (mut peer, mut buf, handle) = logged_on_with(with_inbound_delay(2, Duration::from_millis(2_500)), 1).await;
        let start = Instant::now();
        let mut burst: Vec<u8> = (0..5u64).flat_map(|i| from_peer(i + 2, order(&format!("O{i}")))).collect();
        burst.extend(from_peer(7, Message::new(MsgType::Heartbeat)));
        peer.write_all(&burst).await.unwrap();

        let mut seen = Vec::new();
        while let Ok(mut received) =
            tokio::time::timeout_at(start + Duration::from_millis(5_900), receive_timed(&mut peer, &mut buf, 1, start))
                .await
        {
            let (msg, at) = received.remove(0);
            let kind = match msg.msg_type() {
                MsgType::ExecutionReport => msg.get(tags::CL_ORD_ID).unwrap().to_string(),
                MsgType::Heartbeat => "heartbeat".into(),
                other => panic!("unexpected {other} at {at:?} after {seen:?}"),
            };
            if at == Duration::from_millis(4_500) {
                // The Heartbeat arrived at once, but waits behind O4.
                let numbers = handle.sequence_numbers().await.unwrap();
                assert_eq!(numbers.next_incoming, 6, "O0 to O3 handled");
            }
            seen.push((kind, at.as_millis()));
        }
        let seen: Vec<_> = seen.iter().map(|(kind, at)| (kind.as_str(), *at)).collect();
        assert_eq!(
            seen,
            [
                ("O0", 0),
                ("O1", 0),
                ("heartbeat", 1_000),
                ("heartbeat", 2_000),
                ("O2", 2_500),
                ("O3", 2_500),
                ("heartbeat", 3_500),
                ("heartbeat", 4_500),
                ("O4", 5_000),
            ]
        );
        assert_eq!(handle.sequence_numbers().await.unwrap().next_incoming, 8, "the Heartbeat was handled after O4");
    }

    /// A counterparty that skips ahead and sends new orders as the "resend" we ask for is paced
    /// like any other: every application message counts as it's read, the gap's far end and the
    /// resend included. 99 messages at 10 per 200ms take 9 windows.
    #[tokio::test(start_paused = true)]
    async fn a_manufactured_gap_gets_no_more_through_the_delay_limit() {
        const WINDOW: Duration = Duration::from_millis(200);
        let (mut peer, mut buf, _handle) = logged_on_with(with_inbound_delay(10, WINDOW), 30).await;
        let start = Instant::now();
        peer.write_all(&from_peer(100, order("Z"))).await.unwrap();
        let request = receive(&mut peer, &mut buf, 1).await.remove(0);
        assert_eq!(request.msg_type(), MsgType::ResendRequest);
        let resend: Vec<u8> = (2..100u64).flat_map(|seq| from_peer(seq, order(&format!("O{seq}")))).collect();
        peer.write_all(&resend).await.unwrap();

        let acked = receive_timed(&mut peer, &mut buf, 99, start).await;
        let ids: Vec<_> = acked.iter().map(|(m, _)| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect();
        let mut expected: Vec<_> = (2..100).map(|seq| format!("O{seq}")).collect();
        expected.push("Z".into());
        assert_eq!(ids, expected, "in sequence, the queued one last");
        // Z counted first, as it arrived, so the order with MsgSeqNum s was the (s - 1)th read.
        let times: Vec<_> = acked.iter().map(|(_, at)| *at).collect();
        let mut windows: Vec<_> = (2..100u32).map(|seq| WINDOW * ((seq - 1) / 10)).collect();
        windows.push(WINDOW * 9);
        assert_eq!(times, windows);
    }

    /// While input is held, the socket isn't read: the peer's writes back up into its own
    /// buffer (here 1 KiB, a TCP window in miniature), so it's slowed, rather than ours filling.
    #[tokio::test(start_paused = true)]
    async fn the_socket_is_not_read_while_input_is_held() {
        let config = with_inbound_delay(10, Duration::from_millis(200));
        let (peer, mut buf, _handle) = logged_on_over(1024, config, 30, None).await;
        let (mut reader, mut writer) = tokio::io::split(peer);
        let written = Arc::new(AtomicUsize::new(0));
        let counter = written.clone();
        let writing = tokio::spawn(async move {
            for i in 0..100u64 {
                writer.write_all(&from_peer(i + 2, order(&format!("O{i}")))).await.unwrap();
                counter.fetch_add(1, Ordering::SeqCst);
            }
            writer
        });

        receive(&mut reader, &mut buf, 10).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let so_far = written.load(Ordering::SeqCst);
        assert!(so_far < 30, "{so_far} orders written while input was held");
        // Once the windows free up, all of them get through.
        receive(&mut reader, &mut buf, 90).await;
        let _writer = writing.await.unwrap();
        assert_eq!(written.load(Ordering::SeqCst), 100);
    }

    /// Shutting down while input is held logs out at once, and the peer's Logout reply is read
    /// and answered at once: a session logging out holds no input.
    #[tokio::test(start_paused = true)]
    async fn a_shutdown_while_input_is_held_logs_out_promptly() {
        let shutdown = crate::shutdown::Shutdown::new();
        let config = with_inbound_delay(1, Duration::from_secs(60));
        let (mut peer, mut buf, _handle) = logged_on_following(config, 30, Some(shutdown.signal())).await;
        let start = Instant::now();
        let burst: Vec<u8> = (0..3u64).flat_map(|i| from_peer(i + 2, order(&format!("O{i}")))).collect();
        peer.write_all(&burst).await.unwrap();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].get(tags::CL_ORD_ID), Some("O0"));
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Nothing is tracked, so this only starts the shutdown.
        shutdown.run(Some("bye"), Duration::from_secs(10)).await;
        let mut seen = Vec::new();
        while seen.last() != Some(&MsgType::Logout) {
            seen.push(receive(&mut peer, &mut buf, 1).await[0].msg_type());
        }
        peer.write_all(&from_peer(5, Message::new(MsgType::Logout))).await.unwrap();
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
        assert_eq!(start.elapsed(), Duration::from_millis(10), "at once, not a window later");
    }

    /// A peer that closes while input is held is noticed once the hold ends, after everything it
    /// sent before closing has been handled, in order; then the connection ends cleanly.
    #[tokio::test(start_paused = true)]
    async fn a_peer_that_closes_during_a_hold_is_noticed_after_its_input() {
        let (mut peer, mut buf, _handle) = logged_on_with(with_inbound_delay(2, Duration::from_secs(1)), 30).await;
        let start = Instant::now();
        let burst: Vec<u8> = (0..5u64).flat_map(|i| from_peer(i + 2, order(&format!("O{i}")))).collect();
        peer.write_all(&burst).await.unwrap();
        peer.shutdown().await.unwrap();

        let acked = receive_timed(&mut peer, &mut buf, 5, start).await;
        let seen: Vec<_> = acked.iter().map(|(m, at)| (m.get(tags::CL_ORD_ID).unwrap(), at.as_millis())).collect();
        assert_eq!(seen, [("O0", 0), ("O1", 0), ("O2", 1_000), ("O3", 1_000), ("O4", 2_000)]);
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
        assert!(rest.is_empty(), "nothing after the last ack");
        assert_eq!(start.elapsed(), Duration::from_secs(2), "once the last hold ended");
    }

    /// Nothing goes out before the store's commit of it, which runs off the connection's task:
    /// input that arrives while a commit is under way waits for it, then is answered.
    #[tokio::test]
    async fn output_waits_for_the_stores_commit() {
        let storage = crate::store::deferring::DeferringStorage::default();
        // Each commit waits for the test to let it through.
        let (permits, gate) = std::sync::mpsc::channel::<()>();
        let gate = Arc::new(std::sync::Mutex::new(gate));
        *storage.job.lock().unwrap() =
            Arc::new(move || gate.lock().unwrap().recv().map_err(|_| io::Error::other("the test ended")));
        let calls = storage.calls.clone();
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::new(Arc::new(storage)));
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let mut buf = Vec::new();
        let nothing_yet = async |peer: &mut DuplexStream, buf: &mut Vec<u8>| {
            let read = tokio::time::timeout(Duration::from_millis(100), peer.read_buf(buf)).await;
            assert!(read.is_err(), "nothing is written before its commit");
        };

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        nothing_yet(&mut peer, &mut buf).await;
        permits.send(()).unwrap();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);

        // An order: handed over at once (the Logon's commit recorded its window), its reply held
        // for its commit, and a second order arrives meanwhile, to be the next batch.
        let order = |id: &str| Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id);
        peer.write_all(&from_peer(2, order("A"))).await.unwrap();
        nothing_yet(&mut peer, &mut buf).await;
        peer.write_all(&from_peer(3, order("B"))).await.unwrap();
        nothing_yet(&mut peer, &mut buf).await;
        permits.send(()).unwrap(); // A's batch
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].get(tags::CL_ORD_ID), Some("A"));
        nothing_yet(&mut peer, &mut buf).await;
        permits.send(()).unwrap(); // B's
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].get(tags::CL_ORD_ID), Some("B"));
        let calls = calls.lock().unwrap().clone();
        let commits = calls.iter().filter(|c| *c == "commit").count();
        assert_eq!(commits, 3, "the Logon, then each order's batch: {calls:?}");
    }

    /// A resend longer than one step goes out in order over a connection whose buffer holds only
    /// part of it, and an order that arrives meanwhile is answered after it.
    #[tokio::test]
    async fn a_long_resend_is_written_in_steps_before_anything_new() {
        const ORDERS: u64 = 3_000;
        let (ours, mut peer) = duplex(16 * 1024);
        let registry = Arc::new(SessionRegistry::default());
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        receive(&mut peer, &mut buf, 1).await;
        // Our 2 onwards: an ExecutionReport per order, a hundred at a time.
        for first in (2..ORDERS + 2).step_by(100) {
            for seq in first..first + 100 {
                let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{seq}"));
                peer.write_all(&from_peer(seq, order)).await.unwrap();
            }
            receive(&mut peer, &mut buf, 100).await;
        }

        let request = Message::new(MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, 1u64).with(tags::END_SEQ_NO, 0u64);
        peer.write_all(&from_peer(ORDERS + 2, request)).await.unwrap();
        let late = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "LATE");
        peer.write_all(&from_peer(ORDERS + 3, late)).await.unwrap();

        let messages = receive(&mut peer, &mut buf, usize::try_from(ORDERS).unwrap() + 2).await;
        assert_eq!(messages[0].get(tags::NEW_SEQ_NO), Some("2"), "the Logon is gap-filled");
        for (expected, msg) in (2..).zip(&messages[1..messages.len() - 1]) {
            assert_eq!(msg.get(tags::CL_ORD_ID), Some(format!("O{expected}").as_str()));
            assert_eq!(msg.get(tags::POSS_DUP_FLAG), Some("Y"));
        }
        let last = messages.last().unwrap();
        assert_eq!((last.get(tags::CL_ORD_ID), last.get(tags::POSS_DUP_FLAG)), (Some("LATE"), None));
        assert_eq!(last.get(tags::MSG_SEQ_NUM), Some((ORDERS + 2).to_string().as_str()));
    }

    /// Over a store that opens, commits and reads with jobs: logon waits for the log to open, a
    /// resend goes out in order, step by step as each read ends, and an order that arrives
    /// meanwhile is answered after it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn logon_and_resends_wait_for_the_stores_jobs() {
        const ORDERS: u64 = 600;
        let storage = crate::store::deferring::DeferringStorage::deferring_all();
        // Every commit and read takes a while, so the driver must wait for it.
        *storage.job.lock().unwrap() = Arc::new(|| {
            std::thread::sleep(Duration::from_millis(1));
            Ok(())
        });
        let calls = storage.calls.clone();
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::new(Arc::new(storage)));
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        receive(&mut peer, &mut buf, 1).await;
        for seq in 2..ORDERS + 2 {
            let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{seq}"));
            peer.write_all(&from_peer(seq, order)).await.unwrap();
        }
        receive(&mut peer, &mut buf, usize::try_from(ORDERS).unwrap()).await;

        let request = Message::new(MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, 1u64).with(tags::END_SEQ_NO, 0u64);
        peer.write_all(&from_peer(ORDERS + 2, request)).await.unwrap();
        let late = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "LATE");
        peer.write_all(&from_peer(ORDERS + 3, late)).await.unwrap();

        let messages = receive(&mut peer, &mut buf, usize::try_from(ORDERS).unwrap() + 2).await;
        assert_eq!(messages[0].get(tags::NEW_SEQ_NO), Some("2"), "the Logon is gap-filled");
        for (expected, msg) in (2..).zip(&messages[1..messages.len() - 1]) {
            assert_eq!(msg.get(tags::CL_ORD_ID), Some(format!("O{expected}").as_str()));
            assert_eq!(msg.get(tags::POSS_DUP_FLAG), Some("Y"));
        }
        let last = messages.last().unwrap();
        assert_eq!((last.get(tags::CL_ORD_ID), last.get(tags::POSS_DUP_FLAG)), (Some("LATE"), None));
        let fetches: Vec<String> = calls.lock().unwrap().iter().filter(|c| c.starts_with("fetch")).cloned().collect();
        assert_eq!(fetches, ["fetch 1..=256", "fetch 257..=512", "fetch 513..=601"]);
    }

    /// Counts the ExecutionReports it receives, and keeps its session's handle.
    #[derive(Default)]
    struct Counter {
        reports: AtomicUsize,
        handle: std::sync::Mutex<Option<crate::SessionHandle>>,
        logged_on: tokio::sync::Notify,
    }

    impl Application for Counter {
        fn on_logon(&self, session: &crate::SessionHandle) {
            *self.handle.lock().unwrap() = Some(session.clone());
            self.logged_on.notify_one();
        }

        fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
            if msg.msg_type() == MsgType::ExecutionReport {
                self.reports.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        }
    }

    /// Both ends write faster than a small connection carries: orders one way, acknowledgements
    /// the other. Each driver must keep reading while its own output waits, or once both send
    /// buffers fill, each waits for the other to read, forever.
    #[tokio::test]
    async fn both_ends_writing_at_once_do_not_deadlock() {
        const ORDERS: usize = 2_000;
        let (ours, theirs) = duplex(4 * 1024);
        let acceptor_now = tokio::time::Instant::now().into_std();
        let (acceptor, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "GATEWAY"),
            Arc::new(SessionRegistry::default()),
            Arc::new(Acker),
            acceptor_now,
        );
        tokio::spawn(run(theirs, acceptor, commands));
        let app = Arc::new(Counter::default());
        let config = crate::InitiatorConfig::new(SessionConfig::new("FIX.4.2", "CLIENT"), "GATEWAY");
        let now = tokio::time::Instant::now().into_std();
        let (initiator, commands) = Session::initiator(&config, Arc::new(SessionRegistry::default()), app.clone(), now);
        tokio::spawn(run(ours, initiator, commands));
        tokio::time::timeout(Duration::from_secs(5), app.logged_on.notified()).await.expect("logged on");

        let handle = app.handle.lock().unwrap().clone().unwrap();
        for i in 0..ORDERS {
            let order =
                Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}")).with(tags::SYMBOL, "X");
            handle.send(order).unwrap();
        }
        let all_acked = async {
            while app.reports.load(Ordering::SeqCst) < ORDERS {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        };
        let acked = tokio::time::timeout(Duration::from_secs(10), all_acked).await;
        assert!(acked.is_ok(), "deadlocked after {} of {ORDERS} acknowledgements", app.reports.load(Ordering::SeqCst));
    }

    /// Sends made before logon completes wait in their bounded queue, then go out after it, in
    /// order; a logout asked for after them follows them.
    #[tokio::test]
    async fn sends_before_logon_go_out_after_it_then_the_logout() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let config = crate::InitiatorConfig::new(SessionConfig::new("FIX.4.2", "US"), "PEER");
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::initiator(&config, registry.clone(), Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let mut buf = Vec::new();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);

        let handle = registry.handle(SessionId {
            begin_string: "FIX.4.2".into(),
            sender_comp_id: "US".into(),
            target_comp_id: "PEER".into(),
        });
        let receipts: Vec<_> = ["A", "B"]
            .map(|id| handle.send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, id)).unwrap())
            .into();
        handle.logout(Some("done")).unwrap();
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let sent = receive(&mut peer, &mut buf, 3).await;
        let seen: Vec<_> = sent.iter().map(|m| (m.msg_type(), m.get(tags::CL_ORD_ID).map(String::from))).collect();
        assert_eq!(
            seen,
            [
                (MsgType::NewOrderSingle, Some("A".into())),
                (MsgType::NewOrderSingle, Some("B".into())),
                (MsgType::Logout, None)
            ]
        );
        // Each receipt gives the MsgSeqNum the message was stored as.
        let mut stored = Vec::new();
        for receipt in receipts {
            stored.push(receipt.await.unwrap());
        }
        assert_eq!(stored, [2, 3]);
    }

    /// A counterparty that stops reading backs output up, the connection stops taking sends, and
    /// the send queue fills: the application is told, rather than memory growing without limit.
    #[tokio::test]
    async fn sends_to_a_counterparty_that_stops_reading_fill_the_queue() {
        let (ours, mut peer) = duplex(4 * 1024);
        let registry = Arc::new(SessionRegistry::default());
        let mut config = SessionConfig::new("FIX.4.2", "US");
        config.send_queue = 100;
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::acceptor(config, registry.clone(), Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        receive(&mut peer, &mut buf, 1).await;

        // The peer reads nothing more.
        let handle = registry.handle(SessionId {
            begin_string: "FIX.4.2".into(),
            sender_comp_id: "US".into(),
            target_comp_id: "PEER".into(),
        });
        let mut sent = 0;
        let full = loop {
            let order = Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, format!("E{sent}"));
            match handle.send(order) {
                Ok(_) => sent += 1,
                Err(SendError::Full(_)) => break true,
                Err(e) => panic!("{e}"),
            }
            assert!(sent < 100_000, "never full");
            // Let the connection take what it will.
            tokio::task::yield_now().await;
        };
        assert!(full);
        // Unwritten output stops the connection taking sends at 256 KiB, and the queue holds 100.
        assert!(sent < 5_000, "{sent} queued before the queue filled");

        // What's still queued when the connection ends is dropped, and its receipt says so.
        let order = Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "late");
        let receipt = loop {
            match handle.send(order.clone()) {
                Ok(receipt) => break receipt,
                Err(SendError::Full(_)) => tokio::task::yield_now().await,
                Err(e) => panic!("{e}"),
            }
            // Make room by reading what's waiting.
            let mut sink = vec![0; 64 * 1024];
            let _ = tokio::time::timeout(Duration::from_millis(1), peer.read(&mut sink)).await;
        };
        drop(peer);
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(5), receipt).await.unwrap(),
            Err(crate::Dropped::Disconnected)
        );
    }

    /// A Heartbeat falls due HeartBtInt after the last send, not at the next whole-second tick.
    #[tokio::test(start_paused = true)]
    async fn heartbeat_goes_out_when_due_not_on_the_next_tick() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        tokio::time::advance(Duration::from_millis(300)).await;
        peer.write_all(&from_peer(2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A"))).await.unwrap();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::ExecutionReport);
        let acked = tokio::time::Instant::now();

        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(acked.elapsed(), Duration::from_secs(1));
    }

    #[tokio::test(start_paused = true)]
    async fn silent_counterparty_is_probed_then_dropped() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        let start = tokio::time::Instant::now();

        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(start.elapsed(), Duration::from_secs(1));
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::TestRequest);
        assert_eq!(start.elapsed(), Duration::from_millis(1200));

        // Unanswered: disconnected HeartBtInt later, with nothing else sent.
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
        assert!(buf.is_empty() && rest.is_empty(), "{:?}", String::from_utf8_lossy(&rest));
        assert_eq!(start.elapsed(), Duration::from_millis(2200));
    }

    /// Traffic keeps pushing the deadlines later, and the timer sends nothing meanwhile.
    #[tokio::test(start_paused = true)]
    async fn nothing_extra_is_sent_while_traffic_keeps_the_link_up() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        for seq in 2..12 {
            // Off the whole second, so a once-a-second tick would send the last Heartbeat late.
            // Sleep rather than advance, which would fire the timer late and shift its phase.
            tokio::time::sleep(Duration::from_millis(650)).await;
            let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{seq}"));
            peer.write_all(&from_peer(seq, order)).await.unwrap();
            let reply = receive(&mut peer, &mut buf, 1).await;
            assert_eq!(reply[0].msg_type(), MsgType::ExecutionReport, "at order {seq}");
        }
        let acked = tokio::time::Instant::now();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(acked.elapsed(), Duration::from_secs(1));
    }

    /// A deadline already past when it's learned goes out at once, even if the timer has fired
    /// since that deadline.
    #[tokio::test(start_paused = true)]
    async fn an_overdue_heartbeat_goes_out_as_soon_as_logon_completes() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let mut config = crate::InitiatorConfig::new(SessionConfig::new("FIX.4.2", "US"), "PEER");
        config.heartbeat_interval = Duration::from_secs(1);
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::initiator(&config, registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let mut buf = Vec::new();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);

        // Sleep rather than advance, so the timer fires at 1 s, before the reply: our Heartbeat
        // is then overdue.
        tokio::time::sleep(Duration::from_millis(1500)).await;
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 1u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let replied = tokio::time::Instant::now();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(replied.elapsed(), Duration::ZERO);
    }

    /// Spoils every outgoing Heartbeat with an SOH, so the session drops it and its Heartbeat stays
    /// due; counts the attempts.
    struct SpoilsHeartbeats {
        attempts: AtomicUsize,
        spinning: tokio::sync::Notify,
    }

    impl Application for SpoilsHeartbeats {
        fn to_admin(&self, _session: &crate::SessionHandle, msg: &mut Message) {
            if msg.msg_type() == MsgType::Heartbeat {
                msg.set(tags::TEXT, "a\x01b");
                if self.attempts.fetch_add(1, Ordering::SeqCst) == 100 {
                    self.spinning.notify_one();
                }
            }
        }
        fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
            Ok(())
        }
    }

    /// A timer that fires and finds its deadline still due waits for the next second rather than
    /// firing again at once.
    #[tokio::test(start_paused = true)]
    async fn a_deadline_that_does_not_move_is_retried_once_a_second() {
        let (ours, mut peer) = duplex(1 << 20);
        let app = Arc::new(SpoilsHeartbeats { attempts: AtomicUsize::new(0), spinning: tokio::sync::Notify::new() });
        let registry = Arc::new(SessionRegistry::default());
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, app.clone(), now);
        tokio::spawn(run(ours, session, commands));
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 1u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();

        // Keep the link up for 5 s, so no TestRequest goes out. A spinning loop never lets paused
        // time advance, so it is caught by the Notify rather than a timeout.
        let keep_alive = async {
            for seq in 2..12 {
                tokio::time::sleep(Duration::from_millis(500)).await;
                peer.write_all(&from_peer(seq, Message::new(MsgType::Heartbeat))).await.unwrap();
            }
        };
        tokio::select! {
            () = keep_alive => {}
            () = app.spinning.notified() => panic!("the timer is spinning"),
        }
        let attempts = app.attempts.load(Ordering::SeqCst);
        assert!((4..=6).contains(&attempts), "{attempts} Heartbeat attempts in 5 s");
    }

    #[tokio::test]
    async fn messages_split_across_reads_and_garbage_between_them_are_handled() {
        const ORDERS: u64 = 30;
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        for i in 0..ORDERS {
            if i == ORDERS / 2 {
                stream.extend_from_slice(b"garbage between messages");
            }
            stream
                .extend(from_peer(i + 2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}"))));
        }
        // Deliver in 13-byte pieces, so messages (and the garbage) straddle reads.
        for piece in stream.chunks(13) {
            peer.write_all(piece).await.unwrap();
            tokio::task::yield_now().await;
        }

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, usize::try_from(1 + ORDERS).unwrap()).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let acked: Vec<_> = replies[1..].iter().map(|m| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect();
        let expected: Vec<_> = (0..ORDERS).map(|i| format!("O{i}")).collect();
        assert_eq!(acked, expected, "every order acknowledged, in order, with no resend requested");
    }

    #[tokio::test]
    async fn message_with_an_empty_value_is_rejected_and_the_session_continues() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        stream.extend(from_peer(
            2,
            Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A").with(tags::TEXT, ""),
        ));
        stream.extend(from_peer(3, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "B")));
        peer.write_all(&stream).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, 3).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let reject = &replies[1];
        assert_eq!(reject.msg_type(), MsgType::Reject, "not ignored as garbled");
        assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(reject.get(tags::REF_TAG_ID), Some("58"));
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("4"), "tag specified without a value");
        assert_eq!(replies[2].msg_type(), MsgType::ExecutionReport, "no resend requested");
        assert_eq!(replies[2].get(tags::CL_ORD_ID), Some("B"));
    }

    #[tokio::test]
    async fn message_with_a_malformed_field_is_rejected_and_the_session_continues() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        let order = with_peer_header(2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A"));
        stream.extend(frame_with_raw_field(&order, b"x5=1"));
        stream.extend(from_peer(3, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "B")));
        peer.write_all(&stream).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, 3).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let reject = &replies[1];
        assert_eq!(reject.msg_type(), MsgType::Reject, "not ignored as garbled");
        assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(reject.get(tags::REF_TAG_ID), None);
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("0"), "invalid tag number");
        assert_eq!(reject.get(tags::TEXT), Some("Invalid tag 'x5'"));
        assert_eq!(replies[2].msg_type(), MsgType::ExecutionReport, "no resend requested");
        assert_eq!(replies[2].get(tags::CL_ORD_ID), Some("B"));
    }

    #[tokio::test]
    async fn each_batch_is_written_with_one_write() {
        const ORDERS: u64 = 20;
        let (ours, mut peer) = duplex(1 << 20);
        let writes = Arc::new(AtomicUsize::new(0));
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry.clone(),
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(CountingStream { inner: ours, writes: writes.clone() }, session, commands));

        // The peer logs on and pipelines orders, all arriving in one read.
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut burst = from_peer(1, logon);
        for i in 0..ORDERS {
            burst
                .extend(from_peer(i + 2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}"))));
        }
        peer.write_all(&burst).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, usize::try_from(1 + ORDERS).unwrap()).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        assert_eq!(replies.last().unwrap().get(tags::CL_ORD_ID), Some("O19"));
        assert_eq!(writes.load(Ordering::SeqCst), 1, "Logon reply and {ORDERS} acks should go out in one write");

        // A burst of sends through the handle is also written at once.
        let handle = registry.handle(SessionId {
            begin_string: "FIX.4.2".into(),
            sender_comp_id: "US".into(),
            target_comp_id: "PEER".into(),
        });
        for i in 0..50 {
            handle.send(Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, format!("E{i}"))).unwrap();
        }
        let sent = receive(&mut peer, &mut buf, 50).await;
        assert_eq!(sent.last().unwrap().get(tags::EXEC_ID), Some("E49"), "sent in order");
        assert_eq!(writes.load(Ordering::SeqCst), 2, "50 queued sends should go out in one write");

        // Logout is still written before the disconnect closes the stream.
        handle.logout(Some("bye")).unwrap();
        let logout = receive(&mut peer, &mut buf, 1).await;
        assert_eq!(logout[0].msg_type(), MsgType::Logout);
        peer.write_all(&from_peer(2 + ORDERS, Message::new(MsgType::Logout))).await.unwrap();
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
    }

    /// The Logout that answers the peer's Logout is queued in the same call that closes the
    /// session; it still reaches the peer before the connection closes.
    #[tokio::test]
    async fn a_logout_sent_while_closing_reaches_the_peer() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let now = Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut burst = from_peer(1, logon);
        burst.extend(from_peer(2, Message::new(MsgType::Logout)));
        peer.write_all(&burst).await.unwrap();

        let mut received = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut received)).await;
        closed.expect("did not close").unwrap();
        let mut sent = Vec::new();
        let mut rest = received.as_slice();
        while let Decoded::Message(msg, len) = crate::codec::decode(rest) {
            sent.push(msg.msg_type());
            rest = &rest[len..];
        }
        assert_eq!(sent, [MsgType::Logon, MsgType::Logout], "{}", String::from_utf8_lossy(&received));
        assert!(rest.is_empty(), "trailing bytes: {}", String::from_utf8_lossy(rest));
    }

    /// An acceptor config that cancels on disconnect after `grace`.
    fn cancelling_after(grace: Duration) -> SessionConfig {
        let mut config = SessionConfig::new("FIX.4.2", "US");
        config.cancel_on_disconnect =
            Some(crate::CancelOnDisconnect { trigger: crate::CancelTrigger::Disconnect, grace });
        config
    }

    /// The counterparty closing its end ends the connection as lost (`Ended::Lost`): the driver
    /// tells the session, whose cancel-on-disconnect countdown starts from the close.
    #[tokio::test(start_paused = true)]
    async fn a_counterparty_that_closes_starts_the_countdown() {
        let grace = Duration::from_secs(5);
        let (peer, _, registry, task) =
            logged_on_running(1 << 20, cancelling_after(grace), Arc::new(Acker), 30, None).await;
        tokio::time::advance(Duration::from_secs(2)).await;
        let closed = Instant::now();
        drop(peer);
        tokio::time::timeout(Duration::from_secs(5), task).await.expect("still running").unwrap().unwrap();
        assert_eq!(registry.next_cancel_deadline(), Some((closed + grace).into_std()));
        assert!(!registry.handle(peer_session()).is_connected());
    }

    /// Replies a mebibyte long, so a few unread fill the output backlog.
    struct Verbose;

    impl Application for Verbose {
        fn on_message(&self, ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
            ctx.send(Message::new(MsgType::ExecutionReport).with(tags::TEXT, "x".repeat(1 << 20)));
            Ok(())
        }
    }

    /// A counterparty that stops reading ends the connection with an error once output backs
    /// up: the driver tells the session, whose cancel-on-disconnect countdown starts then.
    #[tokio::test(start_paused = true)]
    async fn a_counterparty_that_stops_reading_starts_the_countdown() {
        let grace = Duration::from_secs(5);
        let (mut peer, _, registry, task) =
            logged_on_running(4 * 1024, cancelling_after(grace), Arc::new(Verbose), 30, None).await;
        let started = Instant::now();
        // More orders than MAX_UNWRITTEN holds replies to, and none of the replies read.
        for seq in 2..2 + 2 * u64::try_from(MAX_UNWRITTEN >> 20).unwrap() {
            if peer.write_all(&from_peer(seq, order(&format!("O{seq}")))).await.is_err() {
                break;
            }
        }
        let ended = tokio::time::timeout(Duration::from_secs(5), task).await.expect("still running").unwrap();
        assert_eq!(ended.unwrap_err().kind(), io::ErrorKind::TimedOut);
        let deadline = registry.next_cancel_deadline().expect("a countdown");
        assert!(deadline >= (started + grace).into_std(), "{:?}", deadline - started.into_std());
        assert!(deadline <= (Instant::now() + grace).into_std());
    }
}
