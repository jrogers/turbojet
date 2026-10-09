//! [`FileMessageLog`]: a [`MessageLog`] that writes every message to files, one per day, and
//! deletes them after a retention period; optionally compresses each finished file and hands it
//! to a hook.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{fmt, mem};

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use tracing::error;

use crate::fields::{Precision, ToFix, UtcTimestamp};
use crate::message_log::MessageLog;
use crate::{Clock, SessionId};

/// Where a [`FileMessageLog`] rotates to a new file. A batch is written whole, so a file can pass
/// it by up to [`FileLogOptions::buffer_bytes_max`].
pub const DEFAULT_FILE_BYTES_MAX: u64 = 256 * 1024 * 1024;

/// How many bytes a [`FileMessageLog`] holds for its writer by default: about a second of a
/// busy session's traffic at 100,000 messages of 160 bytes, so a slow disk write doesn't drop any.
pub const DEFAULT_BUFFER_BYTES_MAX: usize = 16 * 1024 * 1024;

/// How long the writer thread gathers records before writing them. Waking it for every message
/// would cost each one a system call on the session's task; a millisecond keeps that to one per
/// millisecond and still puts records in the file well within a second.
const GATHER_INTERVAL: Duration = Duration::from_millis(1);

/// Called with each file a [`FileMessageLog`] has finished: see [`FileLogOptions::on_finished`].
pub type FinishedHook = Arc<dyn Fn(&Path) + Send + Sync>;

/// How a [`FileMessageLog`] rotates, retains, buffers and finishes its files.
#[derive(Clone)]
pub struct FileLogOptions {
    /// A new file is started once the current one reaches this size (and at each UTC midnight).
    pub file_bytes_max: u64,
    /// The bytes held for the writer thread. A message that would pass it is dropped, counted,
    /// and the count written to the file in its place, so the gap shows.
    pub buffer_bytes_max: usize,
    /// Files whose day ended longer ago than this are deleted, at open and at each new file.
    /// `None`, the default, keeps them.
    pub retention: Option<Duration>,
    /// The time that stamps each message and names each file.
    pub clock: Clock,
    /// Compress each finished file with gzip, to `messages-YYYYMMDD-NNNNNN.log.gz`, deleting the
    /// original. Off by default. Needs the `gzip` feature: without it, opening the log fails.
    pub compress: bool,
    /// Called with the path of each finished file (compressed, with `compress`), to archive it,
    /// say: the hook may move or delete it. `None`, the default, calls nothing.
    pub on_finished: Option<FinishedHook>,
    /// Write passwords and credentials as `*`s, as [`mask_secrets`](super::mask_secrets) does.
    /// On by default; off, the log holds every message exactly as on the wire.
    pub mask_secrets: bool,
}

impl fmt::Debug for FileLogOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileLogOptions")
            .field("file_bytes_max", &self.file_bytes_max)
            .field("buffer_bytes_max", &self.buffer_bytes_max)
            .field("retention", &self.retention)
            .field("clock", &self.clock)
            .field("compress", &self.compress)
            .field("on_finished", &self.on_finished.as_ref().map(|_| "Fn(&Path)"))
            .field("mask_secrets", &self.mask_secrets)
            .finish()
    }
}

impl Default for FileLogOptions {
    fn default() -> Self {
        Self {
            file_bytes_max: DEFAULT_FILE_BYTES_MAX,
            buffer_bytes_max: DEFAULT_BUFFER_BYTES_MAX,
            retention: None,
            clock: Clock::system(),
            compress: false,
            on_finished: None,
            mask_secrets: true,
        }
    }
}

/// A [`MessageLog`] that writes every message a session receives and sends to files in one
/// directory, for an audit trail that's kept apart from the store and from diagnostic logging.
///
/// The calls copy each message into a buffer, and a thread of the log's own writes it out, so
/// they don't wait for the disk. Files are named `messages-YYYYMMDD-NNNNNN.log`, by UTC day and a
/// number within it; a new one is started each UTC day, when the current one reaches
/// [`FileLogOptions::file_bytes_max`], and when the log is opened (an old file is never appended
/// to). Each message is a record: a line `<time> <in|out> <length> <session>`, with the time in
/// UTC to the microsecond and the session `-` until it's known, then the message's bytes, then a
/// newline. A FIX message is readable as it stands; a FIXP one is binary, so read it by its
/// length. Messages dropped for a full buffer, or lost to a failed write, leave a line
/// `<time> dropped <count>`.
///
/// A file is finished when the next is started, and when the log is dropped. With
/// [`compress`](FileLogOptions::compress) or [`on_finished`](FileLogOptions::on_finished), a second
/// thread of the log's own compresses each finished file and then calls the hook, so the writer
/// never waits for either. With `compress`, opening the log finishes the uncompressed files an
/// earlier one left (a crash leaves its last file so), and starts again a compression a crash cut
/// short; the hook may then be called again for a file it was called for before the crash.
///
/// Records are written with no `fsync`: they survive the process crashing, not the machine.
/// Passwords and credentials are written as `*`s unless [`mask_secrets`](FileLogOptions::mask_secrets)
/// is off; everything else is as on the wire. Dropping the log writes what it holds, then waits for its
/// threads to finish, compressing and handing over the last file included.
///
/// ```no_run
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// use turbojet::{FileLogOptions, FileMessageLog, SessionConfig};
///
/// let options = FileLogOptions { retention: Some(Duration::from_secs(7 * 365 * 86_400)), ..Default::default() };
/// let log = FileMessageLog::open("/var/log/fix", options)?;
/// let mut config = SessionConfig::new("FIX.4.4", "ME");
/// config.message_log = Some(Arc::new(log));
/// # Ok::<(), std::io::Error>(())
/// ```
pub struct FileMessageLog {
    shared: Arc<Shared>,
    clock: Clock,
    buffer_bytes_max: usize,
    writer: Option<JoinHandle<()>>,
    /// Compresses finished files and calls the hook, if either is asked for.
    finisher: Option<JoinHandle<()>>,
}

impl FileMessageLog {
    /// Opens a log in `dir`, creating it if need be, deletes files past retention and starts a
    /// new file.
    ///
    /// # Errors
    ///
    /// If the directory can't be created or read, the first file can't be created, the
    /// retention period is too long to compute with, or [`compress`](FileLogOptions::compress) is
    /// asked for without the `gzip` feature ([`Unsupported`](io::ErrorKind::Unsupported)).
    pub fn open(dir: impl Into<PathBuf>, options: FileLogOptions) -> io::Result<Self> {
        assert!(options.file_bytes_max > 0, "file_bytes_max must be positive");
        assert!(options.buffer_bytes_max > 0, "buffer_bytes_max must be positive");
        if options.compress && !cfg!(feature = "gzip") {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "compressing the message log needs turbojet's gzip feature",
            ));
        }
        let retention = options
            .retention
            .map(TimeDelta::from_std)
            .transpose()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "retention is too long"))?;
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        let (finished, finisher) = match options.compress || options.on_finished.is_some() {
            true => {
                let (finished, finisher) = start_finisher(options.compress, options.on_finished.clone())?;
                if options.compress {
                    finish_leftovers(&dir, &finished)?;
                }
                (Some(finished), Some(finisher))
            }
            false => (None, None),
        };
        let mut files = Files { dir, file_bytes_max: options.file_bytes_max, retention, current: None, finished };
        files.rotate(options.clock.now())?;
        let shared = Arc::new(Shared::default());
        let writer = {
            let shared = Arc::clone(&shared);
            let clock = options.clock.clone();
            let mask = options.mask_secrets;
            thread::Builder::new()
                .name("turbojet-message-log".into())
                .spawn(move || write_until_closed(&shared, &mut files, &clock, mask))?
        };
        Ok(Self {
            shared,
            clock: options.clock,
            buffer_bytes_max: options.buffer_bytes_max,
            writer: Some(writer),
            finisher,
        })
    }

    /// Copies one record into the buffer, or counts it as dropped if the buffer is full.
    fn append(&self, direction: &str, session: Option<&SessionId>, frame: &[u8]) {
        let now = self.clock.now();
        let mut state = self.shared.lock();
        let state = &mut *state;
        let start = state.pending.len();
        state.stamp.clear();
        UtcTimestamp::new(now, Precision::Micros).write_fix(&mut state.stamp);
        state.pending.extend_from_slice(state.stamp.as_bytes());
        let header = match session {
            Some(session) => writeln!(state.pending, " {direction} {} {session}", frame.len()),
            None => writeln!(state.pending, " {direction} {} -", frame.len()),
        };
        debug_assert!(header.is_ok(), "writing to a Vec doesn't fail");
        if state.pending.len() + frame.len() + 1 > self.buffer_bytes_max {
            state.pending.truncate(start);
            state.dropped += 1;
        } else {
            state.pending.extend_from_slice(frame);
            state.pending.push(b'\n');
            state.records += 1;
        }
        if state.waiting {
            state.waiting = false;
            self.shared.wake.notify_one();
        }
    }
}

impl MessageLog for FileMessageLog {
    fn inbound(&self, session: Option<&SessionId>, frame: &[u8]) {
        self.append("in", session, frame);
    }

    fn outbound(&self, session: Option<&SessionId>, frame: &[u8]) {
        self.append("out", session, frame);
    }
}

impl Drop for FileMessageLog {
    fn drop(&mut self) {
        self.shared.lock().closing = true;
        self.shared.wake.notify_one();
        if let Some(writer) = self.writer.take()
            && writer.join().is_err()
        {
            error!("the message log's writer panicked");
        }
        // The writer has handed over its last file, and closed the channel by ending.
        if let Some(finisher) = self.finisher.take()
            && finisher.join().is_err()
        {
            error!("the message log's finisher panicked (in on_finished?)");
        }
    }
}

impl fmt::Debug for FileMessageLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileMessageLog").field("buffer_bytes_max", &self.buffer_bytes_max).finish_non_exhaustive()
    }
}

/// What the sessions' calls and the writer thread share.
#[derive(Default)]
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().expect("message log lock poisoned")
    }
}

#[derive(Default)]
struct State {
    /// Records waiting to be written.
    pending: Vec<u8>,
    /// How many records `pending` holds.
    records: u64,
    /// Records dropped since the last write, for a full buffer.
    dropped: u64,
    /// Whether the writer is waiting for a record, so the next one must wake it.
    waiting: bool,
    closing: bool,
    /// Each record's time, formatted; kept to reuse its allocation.
    stamp: String,
}

/// The writer thread: takes what's pending and writes it, until the log is dropped and nothing
/// is left.
fn write_until_closed(shared: &Shared, files: &mut Files, clock: &Clock, mask: bool) {
    // Swapped with the shared buffer, so both keep their capacity and a warm log doesn't allocate.
    let mut batch = Vec::new();
    loop {
        let (records, mut dropped) = {
            let mut state = shared.lock();
            while state.pending.is_empty() && state.dropped == 0 && !state.closing {
                state.waiting = true;
                state = shared.wake.wait(state).expect("message log lock poisoned");
            }
            state.waiting = false;
            if state.pending.is_empty() && state.dropped == 0 {
                debug_assert!(state.closing);
                files.finish_current();
                return;
            }
            if !state.closing {
                // Let more records gather, so a busy log writes in batches.
                drop(state);
                thread::sleep(GATHER_INTERVAL);
                state = shared.lock();
            }
            mem::swap(&mut state.pending, &mut batch);
            (mem::take(&mut state.records), mem::take(&mut state.dropped))
        };
        if mask {
            // Here rather than as each message is logged, so the sessions don't pay for it.
            mask_records(&mut batch);
        }
        let now = clock.now();
        if let Err(e) = files.write(now, &batch) {
            error!(error = %e, records, "the message log couldn't write; its records are lost");
            dropped += records;
        }
        if dropped > 0 {
            let line = format!("{} dropped {dropped}\n", UtcTimestamp::new(now, Precision::Micros).to_fix());
            if let Err(e) = files.write(now, line.as_bytes()) {
                error!(error = %e, dropped, "the message log couldn't record dropped messages");
            }
        }
        batch.clear();
    }
}

/// Masks the secrets in the message of each record in `batch`, records as `append` writes them:
/// a line `<time> <in|out> <length> <session>`, the message, a newline.
fn mask_records(batch: &mut [u8]) {
    let mut at = 0;
    while let Some(newline) = batch[at..].iter().position(|&b| b == b'\n') {
        let length = batch[at..at + newline]
            .split(|&b| b == b' ')
            .nth(2)
            .and_then(|length| std::str::from_utf8(length).ok()?.parse::<usize>().ok());
        let frame = at + newline + 1;
        let Some(message) = length.and_then(|length| batch.get_mut(frame..frame + length)) else {
            debug_assert!(false, "a record `append` didn't write at {at}");
            return;
        };
        let length = message.len();
        super::mask_secrets(message);
        at = frame + length + 1;
    }
}

/// The log's directory and the file being written.
struct Files {
    dir: PathBuf,
    file_bytes_max: u64,
    retention: Option<TimeDelta>,
    current: Option<Current>,
    /// Where finished files go to be compressed and handed to the hook, if either is asked for.
    finished: Option<mpsc::Sender<PathBuf>>,
}

struct Current {
    path: PathBuf,
    file: File,
    day: NaiveDate,
    number: u32,
    bytes: u64,
}

impl Files {
    /// Writes `bytes` to the current file, starting a new one first if the day has changed or the
    /// file is full. After a failure, the next write starts a new file.
    fn write(&mut self, now: DateTime<Utc>, bytes: &[u8]) -> io::Result<()> {
        let full = self.current.as_ref().is_none_or(|c| c.day != now.date_naive() || c.bytes >= self.file_bytes_max);
        if full {
            self.rotate(now)?;
        }
        let current = self.current.as_mut().expect("rotate opened a file");
        let written = current.file.write_all(bytes);
        if written.is_err() {
            self.finish_current();
        } else {
            current.bytes += u64::try_from(bytes.len()).expect("a batch fits in u64");
        }
        written
    }

    /// Deletes files past retention and opens the next file for `now`'s day.
    fn rotate(&mut self, now: DateTime<Utc>) -> io::Result<()> {
        let day = now.date_naive();
        let mut number_next = match &self.current {
            Some(current) if current.day == day => current.number + 1,
            _ => 0,
        };
        let cutoff = self.retention.map(|retention| now - retention);
        for entry in fs::read_dir(&self.dir)? {
            let path = entry?.path();
            let Some((file_day, number)) = parse_name(&path) else { continue };
            if file_day == day {
                number_next = number_next.max(number + 1);
            }
            let ended = file_day.succ_opt().map(|next| next.and_time(chrono::NaiveTime::MIN).and_utc());
            if let (Some(cutoff), Some(ended)) = (cutoff, ended)
                && ended <= cutoff
                && let Err(e) = fs::remove_file(&path)
            {
                error!(error = %e, path = %path.display(), "the message log couldn't delete a file past retention");
            }
        }
        let path = self.dir.join(format!("messages-{}-{number_next:06}.log", day.format("%Y%m%d")));
        let file = OpenOptions::new().append(true).create_new(true).open(&path)?;
        self.finish_current();
        self.current = Some(Current { path, file, day, number: number_next, bytes: 0 });
        Ok(())
    }

    /// Closes the current file, if any, and hands it to the finisher, if there is one.
    fn finish_current(&mut self) {
        let Some(current) = self.current.take() else { return };
        drop(current.file);
        if let Some(finished) = &self.finished
            && finished.send(current.path).is_err()
        {
            error!("the message log's finisher has stopped (a panic in on_finished?); files are left as they are");
        }
    }
}

/// Starts the thread that compresses each finished file sent to it, if `compress`, and calls
/// `hook` with it. It ends once the sender is dropped and what was sent is done.
fn start_finisher(compress: bool, hook: Option<FinishedHook>) -> io::Result<(mpsc::Sender<PathBuf>, JoinHandle<()>)> {
    let (finished, receive) = mpsc::channel::<PathBuf>();
    let finisher = thread::Builder::new().name("turbojet-message-log-finisher".into()).spawn(move || {
        for path in receive {
            let path = match compress {
                true => gzip(&path).unwrap_or_else(|e| {
                    error!(error = %e, path = %path.display(), "the message log couldn't compress a file; it's left as it is");
                    path
                }),
                false => path,
            };
            if let Some(hook) = &hook {
                hook(&path);
            }
        }
    })?;
    Ok((finished, finisher))
}

/// Sends the uncompressed files an earlier log left in `dir` to be finished, and deletes
/// compressions a crash cut short. Every file there is finished: opening starts a new one.
fn finish_leftovers(dir: &Path, finished: &mpsc::Sender<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
        if name.starts_with("messages-") && name.ends_with(GZIP_PARTIAL) {
            fs::remove_file(&path)?;
        } else if parse_name(&path).is_some() && !is_compressed(&path) {
            finished.send(path).expect("the finisher runs until the sender is dropped");
        }
    }
    Ok(())
}

/// The suffix of a file being compressed, renamed once it's whole.
const GZIP_PARTIAL: &str = ".log.gz.partial";

/// Compresses the file at `path` (`<name>.log`) to `<name>.log.gz`, then deletes it. Returns the
/// new path.
#[cfg(feature = "gzip")]
fn gzip(path: &Path) -> io::Result<PathBuf> {
    use flate2::Compression;
    use flate2::write::GzEncoder;

    let partial = path.with_extension(&GZIP_PARTIAL[1..]);
    let mut encoder = GzEncoder::new(io::BufWriter::new(File::create(&partial)?), Compression::default());
    io::copy(&mut File::open(path)?, &mut encoder)?;
    encoder.finish()?.into_inner().map_err(io::IntoInnerError::into_error)?;
    let compressed = path.with_extension("log.gz");
    fs::rename(&partial, &compressed)?;
    fs::remove_file(path)?;
    Ok(compressed)
}

#[cfg(not(feature = "gzip"))]
fn gzip(_: &Path) -> io::Result<PathBuf> {
    unreachable!("opening refuses compress without the gzip feature")
}

/// The day and number in a log file's name, `messages-YYYYMMDD-NNNNNN.log` or, compressed,
/// `.log.gz`, or `None` for any other file, which the log leaves alone.
pub(super) fn parse_name(path: &Path) -> Option<(NaiveDate, u32)> {
    let name = path.file_name()?.to_str()?;
    let name = name.strip_suffix(".gz").unwrap_or(name);
    let (day, number) = name.strip_prefix("messages-")?.strip_suffix(".log")?.split_once('-')?;
    if day.len() != 8 || number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((NaiveDate::parse_from_str(day, "%Y%m%d").ok()?, number.parse().ok()?))
}

/// Whether a log file's name says it's compressed.
pub(super) fn is_compressed(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "gz")
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::TimeZone;

    use super::*;

    fn at(day: u32, hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, day, hour, 0, 0).unwrap()
    }

    fn fixed_clock(now: DateTime<Utc>) -> Clock {
        Clock::from_fn(move || now)
    }

    /// The log's files in `dir`, by name, with their contents.
    fn read_files(dir: &Path) -> Vec<(String, Vec<u8>)> {
        let mut files: Vec<_> = fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .map(|p| (p.file_name().unwrap().to_str().unwrap().to_owned(), fs::read(&p).unwrap()))
            .collect();
        files.sort();
        files
    }

    fn files(dir: &Path, retention: Option<Duration>) -> Files {
        let retention = retention.map(|r| TimeDelta::from_std(r).unwrap());
        Files { dir: dir.to_owned(), file_bytes_max: 10, retention, current: None, finished: None }
    }

    #[test]
    fn records_carry_time_direction_length_session_and_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { clock: fixed_clock(at(7, 12)), ..Default::default() };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        let session = SessionId::new("FIX.4.2", "US", "PEER");
        log.inbound(None, b"8=FIX.4.2\x0135=A\x01");
        log.outbound(Some(&session), b"\x00\n\x01binary");
        drop(log);

        let files = read_files(dir.path());
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "messages-20261007-000000.log");
        let expected = b"20261007-12:00:00.000000 in 15 -\n8=FIX.4.2\x0135=A\x01\n\
            20261007-12:00:00.000000 out 9 FIX.4.2:US->PEER\n\x00\n\x01binary\n";
        assert_eq!(files[0].1, expected);
    }

    #[test]
    fn passwords_are_masked_unless_asked_not_to() {
        let logon = b"8=FIX.4.4\x019=30\x0135=A\x01553=me\x01554=hunter2\x0110=000\x01";
        for (mask_secrets, password) in [(true, &b"554=*******"[..]), (false, &b"554=hunter2"[..])] {
            let dir = tempfile::tempdir().unwrap();
            let options = FileLogOptions { clock: fixed_clock(at(7, 12)), mask_secrets, ..Default::default() };
            let log = FileMessageLog::open(dir.path(), options).unwrap();
            log.inbound(None, logon);
            log.outbound(None, b"8=FIX.4.4\x019=5\x0135=0\x0110=000\x01");
            drop(log);
            let written = &read_files(dir.path())[0].1;
            assert!(written.windows(password.len()).any(|w| w == password), "{mask_secrets}");
            assert!(written.ends_with(b"35=0\x0110=000\x01\n"), "later records are intact");
        }
    }

    #[test]
    fn a_full_buffer_drops_messages_and_says_how_many() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { buffer_bytes_max: 10, clock: fixed_clock(at(7, 12)), ..Default::default() };
        let log = Arc::new(FileMessageLog::open(dir.path(), options).unwrap());
        for _ in 0..3 {
            log.inbound(None, b"too long for the buffer");
        }
        drop(log);

        let files = read_files(dir.path());
        let text = String::from_utf8(files[0].1.clone()).unwrap();
        let dropped: u64 = text
            .lines()
            .map(|line| line.strip_prefix("20261007-12:00:00.000000 dropped ").unwrap().parse::<u64>().unwrap())
            .sum();
        assert_eq!(dropped, 3);
    }

    #[test]
    fn a_new_file_each_day_when_full_and_at_open() {
        let dir = tempfile::tempdir().unwrap();
        let mut files_ = files(dir.path(), None);
        files_.write(at(7, 12), b"0123456789").unwrap();
        files_.write(at(7, 13), b"full").unwrap();
        files_.write(at(8, 0), b"next day").unwrap();
        drop(files_);
        // Opening again doesn't append to the last file, nor reuse its number.
        files(dir.path(), None).write(at(8, 1), b"reopened").unwrap();

        let names: Vec<_> =
            read_files(dir.path()).into_iter().map(|(name, bytes)| (name, String::from_utf8(bytes).unwrap())).collect();
        let expected = [
            ("messages-20261007-000000.log", "0123456789"),
            ("messages-20261007-000001.log", "full"),
            ("messages-20261008-000000.log", "next day"),
            ("messages-20261008-000001.log", "reopened"),
        ];
        assert_eq!(names, expected.map(|(n, b)| (n.to_owned(), b.to_owned())));
    }

    #[test]
    fn files_whose_day_ended_before_retention_are_deleted_and_others_kept() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["messages-20261005-000000.log", "messages-20261006-000003.log", "notes.txt", "messages-x.log"] {
            fs::write(dir.path().join(name), b"").unwrap();
        }
        // A day's retention at 1:00 on the 8th: the 6th ended 25 hours ago, the 7th 1 hour ago.
        files(dir.path(), Some(Duration::from_secs(86_400))).rotate(at(8, 1)).unwrap();
        fs::write(dir.path().join("messages-20261007-000000.log"), b"").unwrap();
        files(dir.path(), Some(Duration::from_secs(86_400))).rotate(at(8, 1)).unwrap();

        let names: Vec<_> = read_files(dir.path()).into_iter().map(|(name, _)| name).collect();
        let expected = [
            "messages-20261007-000000.log",
            "messages-20261008-000000.log",
            "messages-20261008-000001.log",
            "messages-x.log",
            "notes.txt",
        ];
        assert_eq!(names, expected);
    }

    /// A hook that records the names it's called with.
    fn recording_hook() -> (FinishedHook, Arc<Mutex<Vec<String>>>) {
        let called = Arc::new(Mutex::new(Vec::new()));
        let hook = {
            let called = Arc::clone(&called);
            Arc::new(move |path: &Path| called.lock().unwrap().push(path.file_name().unwrap().to_str().unwrap().into()))
        };
        (hook, called)
    }

    #[test]
    fn the_hook_is_called_with_each_finished_file() {
        let dir = tempfile::tempdir().unwrap();
        let (hook, called) = recording_hook();
        let options = FileLogOptions {
            file_bytes_max: 1,
            clock: fixed_clock(at(7, 12)),
            on_finished: Some(hook),
            ..Default::default()
        };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        log.inbound(None, b"one");
        // Written before the next is logged, so the two go to different files.
        while read_files(dir.path())[0].1.is_empty() {
            thread::sleep(Duration::from_millis(1));
        }
        log.inbound(None, b"two");
        drop(log);
        // The first file filled up, the second was finished by the drop.
        assert_eq!(*called.lock().unwrap(), ["messages-20261007-000000.log", "messages-20261007-000001.log"]);
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn finished_files_are_compressed_and_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let (hook, called) = recording_hook();
        let options = FileLogOptions {
            clock: fixed_clock(at(7, 12)),
            compress: true,
            on_finished: Some(hook),
            ..Default::default()
        };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        log.outbound(None, b"8=FIX.4.2\x0135=0\x01");
        drop(log);

        let name = "messages-20261007-000000.log.gz";
        assert_eq!(*called.lock().unwrap(), [name]);
        let files = FileMessageLog::files(dir.path()).unwrap();
        assert_eq!(files.len(), 1, "the original is gone: {files:?}");
        assert!(files[0].compressed);
        let records: Vec<_> = FileMessageLog::read(dir.path().join(name)).unwrap().map(Result::unwrap).collect();
        assert!(
            matches!(&records[..], [crate::LogRecord::Message { frame, .. }] if frame == b"8=FIX.4.2\x0135=0\x01"),
            "{records:?}"
        );
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn opening_compresses_what_an_earlier_log_left() {
        let dir = tempfile::tempdir().unwrap();
        let plain = FileLogOptions { clock: fixed_clock(at(7, 12)), ..Default::default() };
        let log = FileMessageLog::open(dir.path(), plain).unwrap();
        log.inbound(None, b"left");
        drop(log);
        fs::write(dir.path().join("messages-20261007-000009.log.gz.partial"), b"cut short").unwrap();

        let options = FileLogOptions { clock: fixed_clock(at(7, 13)), compress: true, ..Default::default() };
        drop(FileMessageLog::open(dir.path(), options).unwrap());
        let names: Vec<String> = read_files(dir.path()).into_iter().map(|(name, _)| name).collect();
        assert_eq!(names, ["messages-20261007-000000.log.gz", "messages-20261007-000001.log.gz"]);
    }

    #[cfg(not(feature = "gzip"))]
    #[test]
    fn compressing_needs_the_gzip_feature() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { compress: true, ..Default::default() };
        let err = FileMessageLog::open(dir.path(), options).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Unsupported);
    }
}
