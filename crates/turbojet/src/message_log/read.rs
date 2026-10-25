//! Reading what a [`FileMessageLog`] wrote: [`FileMessageLog::files`], [`FileMessageLog::read`]
//! and [`FileMessageLog::read_from`].

use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDate, Utc};

use super::file::{FileMessageLog, is_compressed, parse_name};
use crate::fields::{FromFix, UtcTimestamp};

/// The longest record header read: a time, a direction, a length and a session ID, with room for
/// long SubIDs and LocationIDs. A longer line isn't one the log wrote.
const MAX_HEADER: usize = 4096;

/// The longest message read: well past the most any FIX or FIXP message is, so a corrupt length
/// fails rather than allocating without limit.
const MAX_FRAME: usize = 64 * 1024 * 1024;

/// One of a [`FileMessageLog`]'s files, as [`FileMessageLog::files`] lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LogFile {
    /// Where it is.
    pub path: PathBuf,
    /// The UTC day its records were written on.
    pub day: NaiveDate,
    /// Its number within the day, from 0: a new file starts at each size limit and opening.
    pub number: u32,
    /// Whether it's compressed (`.log.gz`): see [`FileLogOptions::compress`](super::FileLogOptions::compress).
    pub compressed: bool,
}

/// Which way a logged message went.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Received from the counterparty.
    Inbound,
    /// Sent to the counterparty.
    Outbound,
}

/// One record of a [`FileMessageLog`] file.
///
/// ```no_run
/// use turbojet::{Direction, FileMessageLog, LogRecord};
///
/// for file in FileMessageLog::files("/var/log/fix")? {
///     for record in FileMessageLog::read(&file.path)? {
///         match record? {
///             LogRecord::Message { time, direction: Direction::Inbound, session, frame } => {
///                 println!("{time} from {session:?}: {}", String::from_utf8_lossy(&frame));
///             }
///             LogRecord::Dropped { time, count } => println!("{time}: {count} not logged"),
///             _ => {}
///         }
///     }
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogRecord {
    /// A message, as on the wire.
    Message {
        /// When it was logged, to the microsecond.
        time: DateTime<Utc>,
        /// Received or sent.
        direction: Direction,
        /// The session, as [`SessionId`](crate::SessionId) displays it, or `None` if it wasn't
        /// known yet (see [`MessageLog`](crate::MessageLog)).
        session: Option<String>,
        /// The message's bytes: a FIX message from BeginString(8) to CheckSum(10), or a FIXP
        /// message with its framing header.
        frame: Vec<u8>,
    },
    /// Input from the counterparty ignored as garbled that framed as a message: see
    /// [`MessageLog::garbled`](crate::MessageLog::garbled).
    Garbled {
        /// When it was logged, to the microsecond.
        time: DateTime<Utc>,
        /// The session, as for [`Message`](Self::Message).
        session: Option<String>,
        /// Its bytes, from BeginString(8) to CheckSum(10).
        frame: Vec<u8>,
        /// Why it was ignored.
        reason: String,
    },
    /// Messages dropped (for a full buffer or a failed write) before this point, not logged.
    Dropped {
        /// When the count was written.
        time: DateTime<Utc>,
        /// How many messages were dropped.
        count: u64,
    },
}

impl FileMessageLog {
    /// The log files in `dir`, oldest first: by day, then number. Other files are left out.
    ///
    /// # Errors
    ///
    /// If `dir` can't be read.
    pub fn files(dir: impl AsRef<Path>) -> io::Result<Vec<LogFile>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if let Some((day, number)) = parse_name(&path) {
                let compressed = is_compressed(&path);
                files.push(LogFile { path, day, number, compressed });
            }
        }
        files.sort_by_key(|file| (file.day, file.number));
        Ok(files)
    }

    /// The records in the log file at `path`, in the order they were written, decompressing a
    /// `.log.gz` file. It can be read while being written: the records end at the last whole one
    /// written so far, and reading the file again, from the start or from
    /// [`LogRecords::position`] with [`read_from`](Self::read_from), includes what's been written
    /// since.
    ///
    /// # Errors
    ///
    /// If the file can't be opened, or is compressed and the `gzip` feature is off
    /// ([`Unsupported`](io::ErrorKind::Unsupported)). Each record read is an error
    /// ([`InvalidData`](io::ErrorKind::InvalidData)) if it isn't one the log writes, and the
    /// records end there.
    pub fn read(path: impl AsRef<Path>) -> io::Result<LogRecords> {
        Self::read_from(path, 0)
    }

    /// The records in the log file at `path` from `position`, a [`LogRecords::position`] taken
    /// from an earlier read of the same file, so a reader can carry on where it stopped without
    /// reading the file again from the start. Otherwise as [`read`](Self::read).
    ///
    /// Positions count bytes of the file as written, before compression, so one taken while the
    /// file was `.log` still holds once it's been compressed to `.log.gz`. A `.log` file seeks
    /// straight to it; a `.log.gz` file is decompressed up to it, which takes time in proportion
    /// to `position` but does no work per record.
    ///
    /// A position at the end of the file (all of it read so far) gives no records until more are
    /// written. Pass only positions [`LogRecords::position`] gave: any other offset is likely to
    /// be in the middle of a record, which reads as an [`InvalidData`](io::ErrorKind::InvalidData)
    /// error (or, rarely, as a record that was never written).
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read), and [`UnexpectedEof`](io::ErrorKind::UnexpectedEof) if `position`
    /// is past the end of the file, as it is for a position from some other file.
    pub fn read_from(path: impl AsRef<Path>, position: u64) -> io::Result<LogRecords> {
        let path = path.as_ref();
        let mut file = File::open(path)?;
        let past_end = || io::Error::new(io::ErrorKind::UnexpectedEof, format!("position {position} is past the end"));
        let reader: Box<dyn Read + Send> = match is_compressed(path) {
            #[cfg(feature = "gzip")]
            true => {
                let mut decoder = flate2::read::GzDecoder::new(file);
                if io::copy(&mut (&mut decoder).take(position), &mut io::sink())? < position {
                    return Err(past_end());
                }
                Box::new(decoder)
            }
            #[cfg(not(feature = "gzip"))]
            true => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "reading a compressed message log file needs turbojet's gzip feature",
                ));
            }
            false => {
                if file.metadata()?.len() < position {
                    return Err(past_end());
                }
                file.seek(SeekFrom::Start(position))?;
                Box::new(file)
            }
        };
        Ok(LogRecords { reader: BufReader::new(reader), header: Vec::new(), position, failed: false })
    }
}

/// The records of one log file: see [`FileMessageLog::read`].
pub struct LogRecords {
    reader: BufReader<Box<dyn Read + Send>>,
    header: Vec<u8>,
    /// Where the next record starts, in bytes of the file as written (decompressed).
    position: u64,
    failed: bool,
}

impl fmt::Debug for LogRecords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogRecords")
            .field("position", &self.position)
            .field("failed", &self.failed)
            .finish_non_exhaustive()
    }
}

impl Iterator for LogRecords {
    type Item = io::Result<LogRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }
        let record = self.read_record().transpose();
        self.failed = matches!(record, Some(Err(_)));
        record
    }
}

impl LogRecords {
    /// Where the next record starts: the bytes of the file (as written, before any compression)
    /// that the records returned so far took up, counting from the start of the file. Pass it to
    /// [`FileMessageLog::read_from`] to carry on from here. A record still being written, or one
    /// that's an error, doesn't move it.
    #[must_use]
    pub fn position(&self) -> u64 {
        self.position
    }

    /// The next record, `None` at the end of what's whole, or why the next isn't a record.
    fn read_record(&mut self) -> io::Result<Option<LogRecord>> {
        self.header.clear();
        let read = (&mut self.reader).take(MAX_HEADER as u64).read_until(b'\n', &mut self.header)?;
        if read == 0 || self.header.last() != Some(&b'\n') {
            // The end, or a header still being written.
            if read == MAX_HEADER {
                return Err(invalid("a record header is too long"));
            }
            return Ok(None);
        }
        let header =
            std::str::from_utf8(&self.header[..read - 1]).map_err(|_| invalid("a record header isn't text"))?;
        let mut parts = header.splitn(4, ' ');
        let (time, kind) = (parts.next().unwrap_or_default(), parts.next().unwrap_or_default());
        let time = UtcTimestamp::from_fix(time).map_err(|_| invalid(&format!("bad time in {header:?}")))?.time();
        let direction = match kind {
            "dropped" => {
                let count = parts
                    .next()
                    .and_then(|c| c.parse().ok())
                    .ok_or_else(|| invalid(&format!("bad count in {header:?}")))?;
                self.position += read as u64;
                return Ok(Some(LogRecord::Dropped { time, count }));
            }
            "in" => Some(Direction::Inbound),
            "out" => Some(Direction::Outbound),
            "garbled" => None,
            _ => return Err(invalid(&format!("bad direction in {header:?}"))),
        };
        let length: usize =
            parts.next().and_then(|l| l.parse().ok()).ok_or_else(|| invalid(&format!("bad length in {header:?}")))?;
        if length > MAX_FRAME {
            return Err(invalid(&format!("a {length}-byte message is longer than any the log writes")));
        }
        let mut rest = parts.next();
        // A garbled record's reason comes before the session, quoted: a session may hold spaces.
        let mut reason = String::new();
        if direction.is_none() {
            let quoted = rest.and_then(|r| r.strip_prefix('"')).and_then(|r| r.split_once("\" "));
            let Some((why, session)) = quoted else { return Err(invalid(&format!("bad reason in {header:?}"))) };
            (reason, rest) = (why.to_string(), Some(session));
        }
        let session = match rest {
            Some("-") => None,
            Some(session) => Some(session.to_string()),
            None => return Err(invalid(&format!("no session in {header:?}"))),
        };
        let mut frame = vec![0; length + 1];
        match self.reader.read_exact(&mut frame) {
            Ok(()) => {}
            // Still being written.
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }
        if frame.pop() != Some(b'\n') {
            return Err(invalid(&format!("the message after {header:?} isn't {length} bytes")));
        }
        self.position += (read + length + 1) as u64;
        Ok(Some(match direction {
            Some(direction) => LogRecord::Message { time, direction, session, frame },
            None => LogRecord::Garbled { time, session, frame, reason },
        }))
    }
}

fn invalid(reason: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, reason.to_string())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::message_log::{FileLogOptions, MessageLog};
    use crate::{Clock, SessionId};

    fn at_noon() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 8, 12, 0, 0).unwrap()
    }

    /// Records read back are what was logged, binary frames (with newlines in them) included.
    #[test]
    fn records_read_back_as_logged() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { clock: Clock::from_fn(at_noon), ..FileLogOptions::default() };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        let session = SessionId::new("FIX.4.4", "US", "THEM");
        log.inbound(None, b"8=FIX.4.4\x0135=A\x01");
        log.outbound(Some(&session), b"\x00\n\x01binary\n");
        drop(log);

        let files = FileMessageLog::files(dir.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!((files[0].day, files[0].number), (at_noon().date_naive(), 0));
        let records: Vec<_> = FileMessageLog::read(&files[0].path).unwrap().collect::<io::Result<_>>().unwrap();
        let expected = [
            LogRecord::Message {
                time: at_noon(),
                direction: Direction::Inbound,
                session: None,
                frame: b"8=FIX.4.4\x0135=A\x01".to_vec(),
            },
            LogRecord::Message {
                time: at_noon(),
                direction: Direction::Outbound,
                session: Some("FIX.4.4:US->THEM".into()),
                frame: b"\x00\n\x01binary\n".to_vec(),
            },
        ];
        assert_eq!(records, expected);
    }

    /// A garbled record reads back with its reason, whatever the session holds and the reason
    /// says; its secrets are masked, as a message's are; and a filter keeps it as inbound.
    #[test]
    fn garbled_records_read_back_with_their_reasons() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { clock: Clock::from_fn(at_noon), ..FileLogOptions::default() };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        let spaced = SessionId::new("FIX.4.4", "US DESK", "THEM");
        let logon = b"8=FIX.4.4\x019=20\x0135=A\x01554=secret\x0110=999\x01";
        log.garbled(Some(&spaced), logon, "CheckSum \"mismatch\"\nhere");
        log.garbled(None, b"8=FIX.4.4\x0135=0\x01", "MsgType");
        drop(log);

        let files = FileMessageLog::files(dir.path()).unwrap();
        let records: Vec<_> = FileMessageLog::read(&files[0].path).unwrap().collect::<io::Result<_>>().unwrap();
        let masked = b"8=FIX.4.4\x019=20\x0135=A\x01554=******\x0110=999\x01".to_vec();
        let expected = [
            LogRecord::Garbled {
                time: at_noon(),
                session: Some("FIX.4.4:US DESK->THEM".into()),
                frame: masked,
                reason: "CheckSum 'mismatch''here".into(),
            },
            LogRecord::Garbled {
                time: at_noon(),
                session: None,
                frame: b"8=FIX.4.4\x0135=0\x01".to_vec(),
                reason: "MsgType".into(),
            },
        ];
        assert_eq!(records, expected);

        let mut filter = crate::LogFilter {
            direction: Some(Direction::Inbound),
            msg_types: vec!["D".into()],
            ..crate::LogFilter::default()
        };
        assert!(filter.matches(&records[0]), "its fields can't be trusted to rule it out");
        filter.direction = Some(Direction::Outbound);
        assert!(!filter.matches(&records[0]));
    }

    #[test]
    fn files_are_listed_oldest_first_and_others_left_out() {
        let dir = tempfile::tempdir().unwrap();
        for name in [
            "messages-20261008-000001.log",
            "messages-20261007-000003.log",
            "messages-20261008-000000.log",
            "notes.txt",
        ] {
            fs::write(dir.path().join(name), "").unwrap();
        }
        let names: Vec<_> = FileMessageLog::files(dir.path())
            .unwrap()
            .into_iter()
            .map(|file| file.path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            ["messages-20261007-000003.log", "messages-20261008-000000.log", "messages-20261008-000001.log"]
        );
    }

    fn read_bytes(contents: &[u8]) -> Vec<io::Result<LogRecord>> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("messages-20261008-000000.log");
        fs::write(&path, contents).unwrap();
        FileMessageLog::read(&path).unwrap().collect()
    }

    /// A record still being written (or cut short by a crash) ends the records, without error;
    /// the dropped count before it is read.
    #[test]
    fn records_end_at_the_last_whole_one() {
        let whole = b"20261008-12:00:00.000000 dropped 3\n";
        for partial in [&b"20261008-12:00:00.000000 in 5 -\nab"[..], b"20261008-12:00:00.0000"] {
            let records = read_bytes(&[&whole[..], partial].concat());
            assert_eq!(records.len(), 1, "{partial:?}");
            assert!(matches!(records[0], Ok(LogRecord::Dropped { count: 3, .. })));
        }
    }

    /// What the log doesn't write is an error, and the records end there.
    #[test]
    fn what_isnt_a_record_is_an_error() {
        for bad in [
            &b"not a record\nmore\n"[..],
            b"20261008-12:00:00.000000 sideways 1 -\nx\n",
            b"20261008-12:00:00.000000 in 1 -\nxy\n",
            b"20261008-12:00:00.000000 in 99999999999 -\n",
            &[b'x'; MAX_HEADER + 1],
        ] {
            let records = read_bytes(bad);
            assert_eq!(records.len(), 1, "{:?}", String::from_utf8_lossy(bad));
            assert_eq!(records[0].as_ref().unwrap_err().kind(), io::ErrorKind::InvalidData);
        }
    }

    /// Three records, one a dropped count, and each record's position: where it starts.
    fn three_records() -> (Vec<u8>, Vec<u64>) {
        let records: [&[u8]; 3] = [
            b"20261008-12:00:00.000000 in 5 -\nab\ncd\n",
            b"20261008-12:00:00.000000 dropped 3\n",
            b"20261008-12:00:00.000000 out 2 FIX.4.4:US->THEM\nxy\n",
        ];
        let mut positions = vec![0];
        for record in records {
            positions.push(positions.last().unwrap() + record.len() as u64);
        }
        (records.concat(), positions)
    }

    /// After each record, the position is the bytes read so far; a partial record after them
    /// doesn't move it.
    #[test]
    fn position_counts_the_bytes_of_whole_records() {
        let (bytes, positions) = three_records();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("messages-20261008-000000.log");
        fs::write(&path, [&bytes[..], b"20261008-12:00:00.000000 in 9 -\nab"].concat()).unwrap();
        let mut records = FileMessageLog::read(&path).unwrap();
        assert_eq!(records.position(), 0);
        for position in &positions[1..] {
            records.next().unwrap().unwrap();
            assert_eq!(records.position(), *position);
        }
        assert!(records.next().is_none());
        assert_eq!(records.position(), bytes.len() as u64);
    }

    /// Reading from each record's position gives the records from there on, the same as reading
    /// from the start and skipping those before.
    fn check_read_from_each_position(path: &Path, positions: &[u64]) {
        let all: Vec<_> = FileMessageLog::read(path).unwrap().map(Result::unwrap).collect();
        for (skip, position) in positions.iter().enumerate() {
            let mut records = FileMessageLog::read_from(path, *position).unwrap();
            assert_eq!(records.position(), *position);
            let rest: Vec<_> = records.by_ref().map(Result::unwrap).collect();
            assert_eq!(rest, all[skip..], "from {position}");
            assert_eq!(records.position(), *positions.last().unwrap());
        }
        let past = positions.last().unwrap() + 1;
        let err = FileMessageLog::read_from(path, past).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof, "from {past}");
    }

    #[test]
    fn read_from_carries_on_at_a_position() {
        let (bytes, positions) = three_records();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("messages-20261008-000000.log");
        fs::write(&path, bytes).unwrap();
        check_read_from_each_position(&path, &positions);
    }

    /// Positions are in the file as written, so they hold once the log has compressed it.
    #[cfg(feature = "gzip")]
    #[test]
    fn read_from_a_compressed_file_takes_positions_from_before() {
        let dir = tempfile::tempdir().unwrap();
        let options = FileLogOptions { clock: Clock::from_fn(at_noon), ..FileLogOptions::default() };
        let log = FileMessageLog::open(dir.path(), options).unwrap();
        log.inbound(None, b"8=FIX.4.4\x0135=A\x01");
        log.outbound(Some(&SessionId::new("FIX.4.4", "US", "THEM")), b"\x00\n\x01binary\n");
        log.inbound(None, b"8=FIX.4.4\x0135=0\x01");
        drop(log);
        let plain = &FileMessageLog::files(dir.path()).unwrap()[0].path;
        let mut records = FileMessageLog::read(plain).unwrap();
        let mut positions = vec![records.position()];
        while records.next().is_some() {
            positions.push(records.position());
        }

        let options = FileLogOptions { clock: Clock::from_fn(at_noon), compress: true, ..FileLogOptions::default() };
        drop(FileMessageLog::open(dir.path(), options).unwrap());
        let files = FileMessageLog::files(dir.path()).unwrap();
        assert!(files[0].compressed, "{files:?}");
        check_read_from_each_position(&files[0].path, &positions);
    }
}
