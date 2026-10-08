//! [`FileMessageLog`]: a [`MessageLog`] that writes every message to files, one per day, and
//! deletes them after a retention period.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
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

/// How a [`FileMessageLog`] rotates, retains and buffers its files.
#[derive(Clone, Debug)]
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
}

impl Default for FileLogOptions {
    fn default() -> Self {
        Self {
            file_bytes_max: DEFAULT_FILE_BYTES_MAX,
            buffer_bytes_max: DEFAULT_BUFFER_BYTES_MAX,
            retention: None,
            clock: Clock::system(),
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
/// Records are written with no `fsync`: they survive the process crashing, not the machine. The
/// bytes are raw, passwords included. Dropping the log writes what it holds and waits for its
/// thread to finish.
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
}

impl FileMessageLog {
    /// Opens a log in `dir`, creating it if need be, deletes files past retention and starts a
    /// new file.
    ///
    /// # Errors
    ///
    /// If the directory can't be created or read, the first file can't be created, or the
    /// retention period is too long to compute with.
    pub fn open(dir: impl Into<PathBuf>, options: FileLogOptions) -> io::Result<Self> {
        assert!(options.file_bytes_max > 0, "file_bytes_max must be positive");
        assert!(options.buffer_bytes_max > 0, "buffer_bytes_max must be positive");
        let retention = options
            .retention
            .map(TimeDelta::from_std)
            .transpose()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "retention is too long"))?;
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        let mut files = Files { dir, file_bytes_max: options.file_bytes_max, retention, current: None };
        files.rotate(options.clock.now())?;
        let shared = Arc::new(Shared::default());
        let writer = {
            let shared = Arc::clone(&shared);
            let clock = options.clock.clone();
            thread::Builder::new()
                .name("turbojet-message-log".into())
                .spawn(move || write_until_closed(&shared, &mut files, &clock))?
        };
        Ok(Self { shared, clock: options.clock, buffer_bytes_max: options.buffer_bytes_max, writer: Some(writer) })
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
fn write_until_closed(shared: &Shared, files: &mut Files, clock: &Clock) {
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

/// The log's directory and the file being written.
struct Files {
    dir: PathBuf,
    file_bytes_max: u64,
    retention: Option<TimeDelta>,
    current: Option<Current>,
}

struct Current {
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
            self.current = None;
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
        let file = OpenOptions::new().append(true).create_new(true).open(path)?;
        self.current = Some(Current { file, day, number: number_next, bytes: 0 });
        Ok(())
    }
}

/// The day and number in a log file's name, `messages-YYYYMMDD-NNNNNN.log`, or `None` for any
/// other file, which the log leaves alone.
pub(super) fn parse_name(path: &Path) -> Option<(NaiveDate, u32)> {
    let name = path.file_name()?.to_str()?;
    let (day, number) = name.strip_prefix("messages-")?.strip_suffix(".log")?.split_once('-')?;
    if day.len() != 8 || number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((NaiveDate::parse_from_str(day, "%Y%m%d").ok()?, number.parse().ok()?))
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
        Files { dir: dir.to_owned(), file_bytes_max: 10, retention, current: None }
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
}
