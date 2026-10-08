//! Reading what a [`FileMessageLog`] wrote: [`FileMessageLog::files`] and
//! [`FileMessageLog::read`].

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDate, Utc};

use super::file::{FileMessageLog, parse_name};
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
                files.push(LogFile { path, day, number });
            }
        }
        files.sort_by_key(|file| (file.day, file.number));
        Ok(files)
    }

    /// The records in the log file at `path`, in the order they were written. It can be read
    /// while being written: the records end at the last whole one written so far, and reading
    /// the file again, from the start, includes what's been written since.
    ///
    /// # Errors
    ///
    /// If the file can't be opened. Each record read is an error
    /// ([`InvalidData`](io::ErrorKind::InvalidData)) if it isn't one the log writes, and the
    /// records end there.
    pub fn read(path: impl AsRef<Path>) -> io::Result<LogRecords> {
        Ok(LogRecords { reader: BufReader::new(File::open(path)?), header: Vec::new(), failed: false })
    }
}

/// The records of one log file: see [`FileMessageLog::read`].
#[derive(Debug)]
pub struct LogRecords {
    reader: BufReader<File>,
    header: Vec<u8>,
    failed: bool,
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
                return Ok(Some(LogRecord::Dropped { time, count }));
            }
            "in" => Direction::Inbound,
            "out" => Direction::Outbound,
            _ => return Err(invalid(&format!("bad direction in {header:?}"))),
        };
        let length: usize =
            parts.next().and_then(|l| l.parse().ok()).ok_or_else(|| invalid(&format!("bad length in {header:?}")))?;
        if length > MAX_FRAME {
            return Err(invalid(&format!("a {length}-byte message is longer than any the log writes")));
        }
        let session = match parts.next() {
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
        Ok(Some(LogRecord::Message { time, direction, session, frame }))
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
}
