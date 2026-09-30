//! File-backed session storage: [`DiskStorage`].

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use tracing::warn;

use super::{SessionId, SessionLog, SessionStorage};
use crate::codec::{Decoded, decode_with, encode, frame};
use crate::fields::{FromFix, ToFix, UtcTimestamp};
use crate::message::{DataFields, Message};

/// Stores each session's state in files under one directory.
///
/// Each session has up to three files in the store directory, named after its [`SessionId`]:
///
/// - `<name>.seqnums`: the next outgoing and incoming sequence numbers and the incoming message in
///   flight to the application (0 for none), as one fixed-width record rewritten in place. A
///   record from before the third field was added reads as none in flight. The file is locked
///   while the session is open, so two gateway processes cannot share a session.
/// - `<name>.created`: when the state was created or last reset, as a FIX UTCTimestamp, once
///   recorded; used by session schedules. Replaced atomically.
/// - `<name>.body`: sent application messages, appended in wire format. Opening the log scans
///   it to index sequence numbers by file offset; resends then read messages back from disk.
///
/// Recovery on open: a partially written message at the end of the body file (from a crash
/// mid-append) is truncated, and the next outgoing sequence number is advanced past the last
/// stored message in case the crash landed between the body append and the seqnums update.
pub struct DiskStorage {
    dir: PathBuf,
    sync: bool,
}

impl DiskStorage {
    /// Stores sessions under `dir`, creating it if needed.
    ///
    /// With `sync`, every write is followed by `fsync`, so state survives power loss at the cost
    /// of latency. Without it, writes survive a process crash but not an OS crash.
    pub fn new(dir: impl Into<PathBuf>, sync: bool) -> io::Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        Ok(Self { dir, sync })
    }

    /// Whether `id` has stored state here (without creating it, as opening would).
    pub fn contains(&self, id: &SessionId) -> bool {
        self.dir.join(format!("{}.seqnums", file_stem(id))).exists()
    }
}

impl SessionStorage for DiskStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(DiskLog::open(&self.dir, id, self.sync)?))
    }
}

/// Offset and length of a stored message in the body file.
type Extent = (u64, usize);

struct DiskLog {
    seqnums: File,
    body: File,
    body_len: u64,
    index: BTreeMap<u64, Extent>,
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    created_path: PathBuf,
    created_at: Option<UtcTimestamp>,
    sync: bool,
    /// The session's data fields, to decode stored messages with.
    data: DataFields,
}

impl DiskLog {
    fn open(dir: &Path, id: &SessionId, sync: bool) -> io::Result<Self> {
        let stem = file_stem(id);
        let seqnums_path = dir.join(format!("{stem}.seqnums"));
        let body_path = dir.join(format!("{stem}.body"));
        let created_path = dir.join(format!("{stem}.created"));

        let mut seqnums = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&seqnums_path)?;
        seqnums.try_lock().map_err(|e| match e {
            TryLockError::WouldBlock => io::Error::new(
                io::ErrorKind::WouldBlock,
                format!("{} is locked by another process", seqnums_path.display()),
            ),
            TryLockError::Error(e) => e,
        })?;
        let (mut next_outgoing, next_incoming, in_flight) = read_seqnums(&mut seqnums, &seqnums_path)?;

        let mut body = OpenOptions::new().read(true).append(true).create(true).open(&body_path)?;
        let (index, valid_len) = scan_body(&mut body, &body_path)?;
        let file_len = body.metadata()?.len();
        if valid_len < file_len {
            warn!(
                path = %body_path.display(),
                discarded = file_len - valid_len,
                "truncating incomplete message at end of session store"
            );
            body.set_len(valid_len)?;
        }
        if let Some((&last, _)) = index.last_key_value() {
            next_outgoing = next_outgoing.max(last + 1);
        }

        let created_at = read_created(&created_path)?;
        Ok(Self {
            seqnums,
            body,
            body_len: valid_len,
            index,
            next_outgoing,
            next_incoming,
            in_flight,
            created_path,
            created_at,
            sync,
            data: DataFields::standard(),
        })
    }

    fn write_seqnums(&mut self) -> io::Result<()> {
        let in_flight = self.in_flight.unwrap_or(0);
        let record = format!("{:020} {:020} {in_flight:020}\n", self.next_outgoing, self.next_incoming);
        // In place, in one system call where the platform allows it.
        #[cfg(unix)]
        std::os::unix::fs::FileExt::write_all_at(&self.seqnums, record.as_bytes(), 0)?;
        #[cfg(not(unix))]
        {
            self.seqnums.seek(SeekFrom::Start(0))?;
            self.seqnums.write_all(record.as_bytes())?;
        }
        if self.sync {
            self.seqnums.sync_data()?;
        }
        Ok(())
    }
}

impl SessionLog for DiskLog {
    fn next_outgoing(&self) -> u64 {
        self.next_outgoing
    }

    fn next_incoming(&self) -> u64 {
        self.next_incoming
    }

    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.next_incoming = seq;
        self.in_flight = None;
        self.write_seqnums()
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&Message>) -> io::Result<()> {
        if let Some(msg) = msg {
            let bytes = encode(msg).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            self.body.write_all(&bytes)?;
            if self.sync {
                self.body.sync_data()?;
            }
            self.index.insert(seq, (self.body_len, bytes.len()));
            self.body_len += bytes.len() as u64;
        }
        self.next_outgoing = seq + 1;
        self.write_seqnums()
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Message)>> {
        let extents: Vec<(u64, Extent)> = self.index.range(begin..=end).map(|(s, e)| (*s, *e)).collect();
        let mut messages = Vec::with_capacity(extents.len());
        for (seq, (offset, len)) in extents {
            self.body.seek(SeekFrom::Start(offset))?;
            let mut bytes = vec![0; len];
            self.body.read_exact(&mut bytes)?;
            match decode_with(&bytes, &self.data) {
                Decoded::Message(msg, _) if msg.defect().is_none() => messages.push((seq, msg)),
                _ => return Err(invalid_data(format!("stored message {seq} at offset {offset} is corrupt"))),
            }
        }
        Ok(messages)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.body.set_len(0)?;
        if self.sync {
            self.body.sync_data()?;
        }
        self.index.clear();
        self.body_len = 0;
        self.next_outgoing = 1;
        self.next_incoming = 1;
        self.in_flight = None;
        self.write_seqnums()?;
        match fs::remove_file(&self.created_path) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        self.created_at = None;
        Ok(())
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.created_at
    }

    fn in_flight(&self) -> Option<u64> {
        self.in_flight
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.in_flight = Some(seq);
        self.write_seqnums()
    }

    fn set_created_at(&mut self, at: UtcTimestamp) -> io::Result<()> {
        // Write a temporary file and rename it over the old one, so a crash leaves either time.
        let temporary = self.created_path.with_extension("created.tmp");
        let mut file = File::create(&temporary)?;
        file.write_all(format!("{}\n", at.to_fix()).as_bytes())?;
        if self.sync {
            file.sync_all()?;
        }
        fs::rename(&temporary, &self.created_path)?;
        self.created_at = Some(at);
        Ok(())
    }

    fn set_data_fields(&mut self, data: &DataFields) {
        self.data = data.clone();
    }
}

fn read_created(path: &Path) -> io::Result<Option<UtcTimestamp>> {
    match fs::read_to_string(path) {
        Ok(text) => UtcTimestamp::from_fix(text.trim())
            .map(Some)
            .map_err(|_| invalid_data(format!("{} is corrupt: {text:?}", path.display()))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

/// The next outgoing and incoming numbers and the message in flight, from a record of two
/// numbers (written before the third was added) or three.
fn read_seqnums(file: &mut File, path: &Path) -> io::Result<(u64, u64, Option<u64>)> {
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    if text.trim().is_empty() {
        return Ok((1, 1, None));
    }
    let numbers: Result<Vec<u64>, _> = text.split_whitespace().map(str::parse::<u64>).collect();
    match numbers.as_deref() {
        Ok(&[out, inc]) if out > 0 && inc > 0 => Ok((out, inc, None)),
        Ok(&[out, inc, in_flight]) if out > 0 && inc > 0 => Ok((out, inc, (in_flight > 0).then_some(in_flight))),
        _ => Err(invalid_data(format!("{} is corrupt: {text:?}", path.display()))),
    }
}

/// Indexes every complete message in the body file. Returns the index and the length of the
/// valid prefix; anything after it is an incomplete trailing write.
fn scan_body(file: &mut File, path: &Path) -> io::Result<(BTreeMap<u64, Extent>, u64)> {
    file.seek(SeekFrom::Start(0))?;
    let mut index = BTreeMap::new();
    let mut buf = Vec::new();
    let mut chunk = vec![0; 64 * 1024];
    let mut offset = 0u64;
    // Bytes of `buf` already indexed. Dropped once per read rather than once per message, which
    // would shift the rest of the buffer every time.
    let mut consumed = 0;
    loop {
        // Only the framing and MsgSeqNum are checked here: the body can only be parsed knowing the
        // session's data fields, which the log is told once it's open. It's parsed when a message
        // is read for a resend.
        match frame(&buf[consumed..]) {
            Ok(len) => {
                let seq = msg_seq_num(&buf[consumed..consumed + len]).ok_or_else(|| {
                    invalid_data(format!("{}: message at offset {offset} has no valid MsgSeqNum", path.display()))
                })?;
                index.insert(seq, (offset, len));
                consumed += len;
                offset += len as u64;
            }
            Err(Decoded::Incomplete) => {
                let n = file.read(&mut chunk)?;
                if n == 0 {
                    return Ok((index, offset));
                }
                buf.drain(..consumed);
                consumed = 0;
                buf.extend_from_slice(&chunk[..n]);
            }
            Err(Decoded::Garbled { reason, .. }) => {
                return Err(invalid_data(format!("{}: corrupt message at offset {offset}: {reason}", path.display())));
            }
            Err(Decoded::Message(..)) => unreachable!("frame doesn't decode"),
        }
    }
}

/// MsgSeqNum(34) of a framed message the session stored, whose standard header, with MsgSeqNum,
/// precedes any data field.
fn msg_seq_num(frame: &[u8]) -> Option<u64> {
    let start = frame.windows(4).position(|w| w == b"\x0134=")? + 4;
    let end = start + frame[start..].iter().position(|&b| b == crate::message::SOH)?;
    u64::from_fix(std::str::from_utf8(&frame[start..end]).ok()?).ok()
}

/// `BEGINSTRING-SENDER-TARGET`, with anything other than ASCII alphanumerics, `.` and `_`
/// percent-encoded so CompIDs cannot escape the directory or collide via the separator.
fn file_stem(id: &SessionId) -> String {
    [&id.begin_string, &id.sender_comp_id, &id.target_comp_id]
        .iter()
        .map(|part| {
            part.bytes()
                .map(|b| match b {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' => (b as char).to_string(),
                    _ => format!("%{b:02X}"),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn invalid_data(msg: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

#[cfg(test)]
mod tests {
    use super::super::conformance::{app_message, check, id};
    use super::*;
    use crate::message::tags;

    fn storage(dir: &tempfile::TempDir) -> DiskStorage {
        DiskStorage::new(dir.path(), false).unwrap()
    }

    fn body_path(dir: &tempfile::TempDir, target: &str) -> PathBuf {
        dir.path().join(format!("{}.body", file_stem(&id(target))))
    }

    #[test]
    fn conforms() {
        let dir = tempfile::tempdir().unwrap();
        check(&storage(&dir));
        check(&DiskStorage::new(dir.path().join("synced"), true).unwrap());
    }

    #[test]
    fn state_survives_a_new_storage_instance() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            log.record_outgoing(1, Some(&app_message(1))).unwrap();
            log.set_next_incoming(3).unwrap();
        }
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (2, 3));
        assert_eq!(log.sent_messages(1, 1).unwrap()[0].1.get(tags::EXEC_ID), Some("E1"));
    }

    /// A record written before the in-flight field was added still reads, and gains the field
    /// on the next write.
    #[test]
    fn two_number_records_still_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(format!("{}.seqnums", file_stem(&id("A"))));
        std::fs::write(&path, format!("{:020} {:020}\n", 5, 9)).unwrap();
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming(), log.in_flight()), (5, 9, None));
        log.set_in_flight(9).unwrap();
        drop(log);
        assert_eq!(std::fs::read_to_string(&path).unwrap().split_whitespace().count(), 3);
    }

    #[test]
    fn session_is_locked_while_open() {
        let dir = tempfile::tempdir().unwrap();
        let storage = storage(&dir);
        let log = storage.open(&id("A")).unwrap();
        let err = storage.open(&id("A")).err().expect("second open should fail");
        assert_eq!(err.kind(), io::ErrorKind::WouldBlock);
        drop(log);
        storage.open(&id("A")).unwrap();
    }

    #[test]
    fn torn_trailing_write_is_truncated() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            log.record_outgoing(1, Some(&app_message(1))).unwrap();
        }
        let path = body_path(&dir, "A");
        let good_len = fs::metadata(&path).unwrap().len();
        let partial = &encode(&app_message(2)).unwrap()[..20];
        OpenOptions::new().append(true).open(&path).unwrap().write_all(partial).unwrap();

        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!(fs::metadata(&path).unwrap().len(), good_len);
        log.record_outgoing(2, Some(&app_message(2))).unwrap();
        let seqs: Vec<u64> = log.sent_messages(1, 2).unwrap().iter().map(|(s, _)| *s).collect();
        assert_eq!(seqs, [1, 2]);
    }

    #[test]
    fn stale_seqnums_are_advanced_past_stored_messages() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            log.record_outgoing(1, Some(&app_message(1))).unwrap();
        }
        // Simulate a crash between the body append and the seqnums update.
        let bytes = encode(&app_message(2)).unwrap();
        OpenOptions::new().append(true).open(body_path(&dir, "A")).unwrap().write_all(&bytes).unwrap();

        let log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!(log.next_outgoing(), 3);
    }

    #[test]
    fn corrupt_body_fails_to_open() {
        let dir = tempfile::tempdir().unwrap();
        storage(&dir).open(&id("A")).unwrap();
        fs::write(body_path(&dir, "A"), b"not a fix message at all, clearly corrupt").unwrap();
        let err = storage(&dir).open(&id("A")).err().expect("open should fail");
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn stored_message_with_a_malformed_field_is_corrupt() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = storage(&dir).open(&id("A")).unwrap();
        // A full header, so the message isn't refused for lacking one.
        let msg = app_message(1)
            .with(tags::SENDER_COMP_ID, "GW")
            .with(tags::TARGET_COMP_ID, "A")
            .with(tags::SENDING_TIME, "20260929-12:00:00.000");
        log.record_outgoing(1, Some(&msg)).unwrap();
        // Make ExecID(17) non-UTF-8, with a valid CheckSum so only the field is at fault.
        let path = body_path(&dir, "A");
        let mut bytes = fs::read(&path).unwrap();
        let at = bytes.windows(5).position(|w| w == b"\x0117=E").unwrap() + 4;
        bytes[at] = 0xff;
        let trailer = bytes.len() - 7;
        let sum = crate::codec::checksum(&bytes[..trailer]);
        bytes[trailer..].copy_from_slice(format!("10={sum:03}\x01").as_bytes());
        fs::write(&path, &bytes).unwrap();

        assert_eq!(log.sent_messages(1, 1).unwrap_err().kind(), io::ErrorKind::InvalidData);
        drop(log);
        // Opening checks only the framing, which is intact.
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!(log.sent_messages(1, 1).unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn file_names_are_sanitised() {
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GW".into(),
            target_comp_id: "../evil-co/x".into(),
        };
        assert_eq!(file_stem(&id), "FIX.4.4-GW-..%2Fevil%2Dco%2Fx");
    }
}
