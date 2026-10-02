//! File-backed session storage: [`DiskStorage`].

use std::collections::{BTreeMap, VecDeque};
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tracing::warn;

use super::{Commit, SessionId, SessionLog, SessionStorage};
use crate::codec::{Decoded, frame_stored};
use crate::fields::{FromFix, ToFix, UtcTimestamp};

/// Stores each session's state in files under one directory.
///
/// Each session has these files in the store directory, named after its [`SessionId`]:
///
/// - `<name>.seqnums`: the next outgoing and incoming sequence numbers and the incoming message in
///   flight to the application (0 for none), in two fixed-size slots written alternately, each
///   with a generation and a checksum; opening reads the valid one with the higher generation. A
///   write that a crash or power loss tears damages only the slot it was writing, so the store
///   reopens with the record before it. A file from before slots (one record, of two or three
///   numbers) still reads, and the first write after it goes to the other slot. The file is
///   locked while the session is open, so two gateway processes cannot share a session.
/// - `<name>.created`: when the state was created or last reset, as a FIX UTCTimestamp, once
///   recorded; used by session schedules. Replaced atomically.
/// - `<name>.body`, `<name>.body.1`, `<name>.body.2` and so on: sent application messages, in
///   segments, appended as the session stores them (see [`SessionLog::record_outgoing`]),
///   whatever their size. A segment that has reached [`with_segment_bytes`] (64 MiB by default)
///   is full, and the next batch starts another. Opening the log scans the segments to index
///   sequence numbers by where they are; resends then read messages back from disk.
///
/// Each session keeps at most [`with_max_session_bytes`] of messages (1 GiB by default): past it,
/// the oldest whole segments are deleted, and a resend that reaches back to their messages
/// gap-fills them, so the counterparty never receives them again (the session logs a warning and
/// counts it in `turbojet_resend_requests_evicted_total`).
///
/// Changes are kept in memory until the session commits them, once per batch of work: then the
/// messages stored since are appended to a segment in one write, and the sequence numbers written
/// once. Without `sync` that's done at once; with it the commit is handed to the connection
/// driver, which runs the writes and their `fsync`s on a blocking thread, so a batch of messages
/// costs one `fsync` of each file rather than two per message.
///
/// Recovery on open: a partially written message at the end of a segment (from a crash
/// mid-append) is truncated, and the next outgoing sequence number is advanced past the last
/// stored message in case the crash landed between the body append and the seqnums update. A
/// crash before a commit loses what it would have written, none of which has been sent.
///
/// [`with_segment_bytes`]: DiskStorage::with_segment_bytes
/// [`with_max_session_bytes`]: DiskStorage::with_max_session_bytes
pub struct DiskStorage {
    dir: PathBuf,
    sync: bool,
    sizes: Sizes,
}

/// How much of a session's messages a store keeps, and in what size of segment.
#[derive(Debug, Clone, Copy)]
struct Sizes {
    segment: u64,
    max: u64,
}

impl DiskStorage {
    /// The default byte budget for each session's stored messages: millions of typical orders and
    /// reports, so a long outage's resends are rarely cut short, while a session that runs for
    /// weeks without a sequence reset stops growing, on disk and in its index.
    pub const DEFAULT_MAX_SESSION_BYTES: u64 = 1 << 30;

    /// The default size of a segment: large enough that starting one is rare, small enough that
    /// the oldest going takes a small part of the budget with it.
    pub const DEFAULT_SEGMENT_BYTES: u64 = 64 << 20;

    /// Stores sessions under `dir`, creating it if needed, with the default sizes.
    ///
    /// With `sync`, every write is followed by `fsync`, so state survives power loss at the cost
    /// of latency. Without it, writes survive a process crash but not an OS crash.
    pub fn new(dir: impl Into<PathBuf>, sync: bool) -> io::Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        let sizes = Sizes { segment: Self::DEFAULT_SEGMENT_BYTES, max: Self::DEFAULT_MAX_SESSION_BYTES };
        Ok(Self { dir, sync, sizes })
    }

    /// Keeps at most `bytes` of each session's messages, deleting the oldest segments past it;
    /// segments are made no larger than it.
    ///
    /// # Panics
    ///
    /// If `bytes` is zero.
    #[must_use]
    pub fn with_max_session_bytes(mut self, bytes: u64) -> Self {
        assert!(bytes > 0, "max_session_bytes must be above zero");
        self.sizes = Sizes { max: bytes, segment: self.sizes.segment.min(bytes) };
        self
    }

    /// Starts a new segment once the current one holds `bytes` (a batch of messages isn't split).
    ///
    /// # Panics
    ///
    /// If `bytes` is zero, or more than the [budget](Self::with_max_session_bytes).
    #[must_use]
    pub fn with_segment_bytes(mut self, bytes: u64) -> Self {
        let max = self.sizes.max;
        assert!(bytes > 0 && bytes <= max, "segment_bytes must be from 1 to max_session_bytes ({max}), not {bytes}");
        self.sizes.segment = bytes;
        self
    }

    /// Whether `id` has stored state here (without creating it, as opening would).
    pub fn contains(&self, id: &SessionId) -> bool {
        self.dir.join(format!("{}.seqnums", file_stem(id))).exists()
    }
}

impl SessionStorage for DiskStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(DiskLog::open(&self.dir, id, self.sync, self.sizes)?))
    }
}

/// Where a stored message is. Sixteen bytes, as the index holds one per message kept.
#[derive(Debug, Clone, Copy)]
struct Location {
    offset: u64,
    /// Below 4 GiB: a `Message` can't grow past it.
    len: u32,
    segment: u32,
}

/// Offset and length of a stored message in a segment.
type Extent = (u64, usize);

/// A batch for a commit to write: the segment's file, the bytes, and whether the file is new (so
/// the directory needs a sync too).
type BatchWrite = (Arc<File>, Vec<u8>, bool);

/// One body segment: a file of messages in the order they were stored.
struct Segment {
    number: u32,
    /// Shared with a commit under way on another thread.
    file: Arc<File>,
    /// Its length on disk.
    len: u64,
    /// The first and last MsgSeqNum stored in it, if any.
    seqs: Option<(u64, u64)>,
}

struct DiskLog {
    dir: PathBuf,
    stem: String,
    sizes: Sizes,
    /// Shared with a commit under way on another thread.
    seqnums: Arc<File>,
    /// The segments kept, oldest first; empty until a message is stored. At most
    /// `sizes.max / sizes.segment` full ones and the one being written (two more while a commit
    /// that starts one also deletes one).
    segments: VecDeque<Segment>,
    /// The kept segments' length on disk.
    total_len: u64,
    /// Messages stored since the last commit, to be appended to a segment. Bounded by what a
    /// session sends in one batch of work.
    pending: Vec<u8>,
    /// The segment `pending` goes into, and where in it, chosen when it starts; and the first and
    /// last MsgSeqNum in it.
    pending_at: Option<(u32, u64)>,
    pending_seqs: Option<(u64, u64)>,
    /// Every message in the kept segments and `pending`: bounded by the budget.
    index: BTreeMap<u64, Location>,
    /// The highest MsgSeqNum of a message deleted with its segment.
    evicted_through: Option<u64>,
    /// The sequence numbers have changed since the last commit.
    dirty: bool,
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    /// The seqnums record's generation, and the slot holding it (none in a fresh file).
    generation: u64,
    slot: Option<usize>,
    created_path: PathBuf,
    created_at: Option<UtcTimestamp>,
    sync: bool,
}

impl DiskLog {
    fn open(dir: &Path, id: &SessionId, sync: bool, sizes: Sizes) -> io::Result<Self> {
        let stem = file_stem(id);
        let seqnums_path = dir.join(format!("{stem}.seqnums"));
        let created_path = dir.join(format!("{stem}.created"));

        let mut seqnums = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&seqnums_path)?;
        seqnums.try_lock().map_err(|e| match e {
            TryLockError::WouldBlock => io::Error::new(
                io::ErrorKind::WouldBlock,
                format!("{} is locked by another process", seqnums_path.display()),
            ),
            TryLockError::Error(e) => e,
        })?;
        let Record { mut next_outgoing, next_incoming, in_flight, generation, slot } =
            read_seqnums(&mut seqnums, &seqnums_path)?;

        let (segments, index) = open_segments(dir, &stem)?;
        if let Some((&last, _)) = index.last_key_value() {
            next_outgoing = next_outgoing.max(last + 1);
        }
        // Segments before the oldest kept were deleted, and their messages with them.
        let evicted_through = match segments.front() {
            Some(oldest) if oldest.number > 0 => {
                let first = index.first_key_value().map_or(next_outgoing, |(&seq, _)| seq);
                first.checked_sub(1).filter(|&seq| seq > 0)
            }
            _ => None,
        };
        let total_len = segments.iter().map(|s| s.len).sum();
        let created_at = read_created(&created_path)?;
        Ok(Self {
            dir: dir.to_path_buf(),
            stem,
            sizes,
            seqnums: Arc::new(seqnums),
            segments,
            total_len,
            pending: Vec::new(),
            pending_at: None,
            pending_seqs: None,
            index,
            evicted_through,
            dirty: false,
            next_outgoing,
            next_incoming,
            in_flight,
            generation,
            slot,
            created_path,
            created_at,
            sync,
        })
    }

    /// The next seqnums record and where it goes: the slot not holding the current one, so a torn
    /// write leaves that one whole. Counted as written: a failed write ends the session, and the
    /// next connection reads the file afresh.
    fn next_record(&mut self) -> ([u8; SLOT], u64) {
        let generation = self.generation + 1;
        let slot = self.slot.map_or(0, |current| 1 - current);
        let record = slot_record(generation, self.next_outgoing, self.next_incoming, self.in_flight.unwrap_or(0));
        (self.generation, self.slot) = (generation, Some(slot));
        self.dirty = false;
        (record, (slot * SLOT) as u64)
    }

    /// Writes the seqnums record now (a reset), syncing it if the store syncs.
    fn write_seqnums(&mut self) -> io::Result<()> {
        let (record, offset) = self.next_record();
        write_record(&self.seqnums, &record, offset)?;
        if self.sync {
            self.seqnums.sync_data()?;
        }
        Ok(())
    }

    fn segment_path(&self, number: u32) -> PathBuf {
        segment_path(&self.dir, &self.stem, number)
    }

    /// Where the next batch of messages goes: the end of the newest segment, or a new one if it's
    /// full.
    fn next_place(&self) -> (u32, u64) {
        match self.segments.back() {
            Some(newest) if newest.len < self.sizes.segment => (newest.number, newest.len),
            Some(newest) => (newest.number + 1, 0),
            None => (0, 0),
        }
    }

    /// Moves `pending` into its segment's count, opening the segment if it's new, and returns what
    /// the commit is to write.
    fn place_pending(&mut self) -> io::Result<Option<BatchWrite>> {
        let Some((number, offset)) = self.pending_at.take() else { return Ok(None) };
        let created = self.segments.back().is_none_or(|newest| newest.number != number);
        if created {
            let file = OpenOptions::new().read(true).append(true).create(true).open(self.segment_path(number))?;
            self.segments.push_back(Segment { number, file: Arc::new(file), len: 0, seqs: None });
        }
        let bytes = std::mem::take(&mut self.pending);
        let segment = self.segments.back_mut().expect("just placed");
        debug_assert_eq!((segment.number, segment.len), (number, offset), "a batch goes where it was placed");
        segment.len += bytes.len() as u64;
        segment.seqs = join_seqs(segment.seqs, self.pending_seqs.take());
        self.total_len += bytes.len() as u64;
        Ok(Some((segment.file.clone(), bytes, created)))
    }

    /// Deletes the oldest segments, never the newest, until what's kept is within the budget:
    /// their messages leave the index. Returns the files to remove.
    fn evict(&mut self) -> Vec<PathBuf> {
        let mut removed = Vec::new();
        while self.total_len > self.sizes.max && self.segments.len() > 1 {
            let oldest = self.segments.pop_front().expect("more than one");
            self.total_len -= oldest.len;
            if let Some((_, last)) = oldest.seqs {
                self.index = self.index.split_off(&(last + 1));
                self.evicted_through = self.evicted_through.max(Some(last));
            }
            removed.push(self.segment_path(oldest.number));
        }
        debug_assert!(self.segments.len() <= 1 || self.total_len <= self.sizes.max);
        removed
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
        self.dirty = true;
        Ok(())
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        if let Some(bytes) = msg {
            // One whole message, as sent_messages checks when reading it back.
            debug_assert_eq!(frame_stored(bytes), Ok(bytes.len()));
            if self.pending_at.is_none() {
                self.pending_at = Some(self.next_place());
            }
            let (segment, start) = self.pending_at.expect("just placed");
            let len = u32::try_from(bytes.len()).expect("a Message is below 4 GiB");
            self.index.insert(seq, Location { offset: start + self.pending.len() as u64, len, segment });
            self.pending.extend_from_slice(bytes);
            self.pending_seqs = join_seqs(self.pending_seqs, Some((seq, seq)));
        }
        self.next_outgoing = seq + 1;
        self.dirty = true;
        Ok(())
    }

    fn commit(&mut self) -> io::Result<Option<Commit>> {
        if !self.dirty && self.pending.is_empty() {
            return Ok(None);
        }
        let (record, offset) = self.next_record();
        let write = self.place_pending()?;
        let removed = self.evict();
        if !self.sync {
            // A few cheap system calls: no reason to leave the connection's task. Messages first,
            // so a crash before the record leaves messages the next open finds (see `open`).
            if let Some((file, mut bytes, _)) = write {
                (&*file).write_all(&bytes)?;
                bytes.clear();
                self.pending = bytes;
            }
            write_record(&self.seqnums, &record, offset)?;
            return remove_files(&removed).map(|()| None);
        }
        let (seqnums, dir) = (self.seqnums.clone(), self.dir.clone());
        Ok(Some(Commit::blocking(move || {
            if let Some((file, bytes, created)) = write {
                (&*file).write_all(&bytes)?;
                file.sync_data()?;
                if created {
                    // The new segment's name, so it survives a power loss too.
                    sync_dir(&dir)?;
                }
            }
            write_record(&seqnums, &record, offset)?;
            seqnums.sync_data()?;
            // A deleted segment a power loss brings back only means more is kept: no sync.
            remove_files(&removed)
        })))
    }

    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        let locations: Vec<(u64, Location)> = self.index.range(begin..=end).map(|(s, l)| (*s, *l)).collect();
        let mut messages = Vec::with_capacity(locations.len());
        for (seq, Location { offset, len, segment: number }) in locations {
            let len = len as usize;
            let bytes = match self.pending_at {
                Some((pending_number, start)) if pending_number == number && offset >= start => {
                    // Not written yet.
                    let at = usize::try_from(offset - start).expect("pending fits in memory");
                    self.pending[at..at + len].to_vec()
                }
                _ => {
                    let segment =
                        self.segments.iter().find(|s| s.number == number).expect("an indexed segment is kept");
                    (&*segment.file).seek(SeekFrom::Start(offset))?;
                    let mut bytes = vec![0; len];
                    (&*segment.file).read_exact(&mut bytes)?;
                    bytes
                }
            };
            match frame_stored(&bytes) {
                Ok(n) if n == len => messages.push((seq, bytes)),
                _ => {
                    return Err(invalid_data(format!(
                        "stored message {seq} in segment {number} at offset {offset} is corrupt"
                    )));
                }
            }
        }
        Ok(messages)
    }

    fn reset(&mut self) -> io::Result<()> {
        // Rare (a logon or schedule reset, an operator), so written at once.
        self.pending.clear();
        self.pending_at = None;
        self.pending_seqs = None;
        // Segment 0 is kept, emptied, for what's stored next; the rest go.
        let mut kept = None;
        let mut removed = Vec::new();
        for segment in self.segments.drain(..) {
            if segment.number == 0 {
                segment.file.set_len(0)?;
                if self.sync {
                    segment.file.sync_data()?;
                }
                kept = Some(Segment { len: 0, seqs: None, ..segment });
            } else {
                removed.push(segment_path(&self.dir, &self.stem, segment.number));
            }
        }
        remove_files(&removed)?;
        self.segments.extend(kept);
        self.index.clear();
        self.total_len = 0;
        self.evicted_through = None;
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

    fn evicted_through(&self) -> Option<u64> {
        self.evicted_through
    }

    fn created_at(&self) -> Option<UtcTimestamp> {
        self.created_at
    }

    fn in_flight(&self) -> Option<u64> {
        self.in_flight
    }

    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.in_flight = Some(seq);
        self.dirty = true;
        Ok(())
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
}

/// The path of segment `number` of the session whose files start `stem`: `<stem>.body` for the
/// first, as the single body file was named before segments.
fn segment_path(dir: &Path, stem: &str, number: u32) -> PathBuf {
    if number == 0 { dir.join(format!("{stem}.body")) } else { dir.join(format!("{stem}.body.{number}")) }
}

/// The segments of the session whose files start `stem`, oldest first, each scanned: a torn
/// message at the end of one is truncated. Returns them and the index of their messages.
fn open_segments(dir: &Path, stem: &str) -> io::Result<(VecDeque<Segment>, BTreeMap<u64, Location>)> {
    let mut numbers = Vec::new();
    let segment_prefix = format!("{stem}.body.");
    for entry in fs::read_dir(dir)? {
        let name = entry?.file_name();
        let Some(name) = name.to_str() else { continue };
        if name.strip_suffix(".body") == Some(stem) {
            numbers.push(0);
        } else if let Some(number) = name.strip_prefix(&segment_prefix).and_then(|n| n.parse::<u32>().ok()) {
            numbers.push(number);
        }
    }
    numbers.sort_unstable();
    let mut segments = VecDeque::with_capacity(numbers.len());
    let mut index = BTreeMap::new();
    for number in numbers {
        let path = segment_path(dir, stem, number);
        let mut file = OpenOptions::new().read(true).append(true).open(&path)?;
        let (found, valid_len) = scan_body(&mut file, &path)?;
        let file_len = file.metadata()?.len();
        if valid_len < file_len {
            warn!(
                path = %path.display(),
                discarded = file_len - valid_len,
                "truncating incomplete message at end of session store"
            );
            file.set_len(valid_len)?;
        }
        let seqs = found.first_key_value().zip(found.last_key_value()).map(|((&first, _), (&last, _))| (first, last));
        for (seq, (offset, len)) in found {
            let len = u32::try_from(len)
                .map_err(|_| invalid_data(format!("{}: message {seq} is too long", path.display())))?;
            index.insert(seq, Location { offset, len, segment: number });
        }
        segments.push_back(Segment { number, file: Arc::new(file), len: valid_len, seqs });
    }
    Ok((segments, index))
}

/// The first and last of two ranges of MsgSeqNum, either of which may be empty.
fn join_seqs(a: Option<(u64, u64)>, b: Option<(u64, u64)>) -> Option<(u64, u64)> {
    match (a, b) {
        (Some((first, _)), Some((_, last))) => Some((first, last)),
        (a, b) => a.or(b),
    }
}

/// Removes `paths`, any already gone.
fn remove_files(paths: &[PathBuf]) -> io::Result<()> {
    for path in paths {
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Makes a file created in `dir` survive a power loss: its name is in the directory.
fn sync_dir(dir: &Path) -> io::Result<()> {
    #[cfg(unix)]
    File::open(dir)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

/// Writes a seqnums record into its slot, in place, in one system call where the platform allows.
fn write_record(file: &File, record: &[u8; SLOT], offset: u64) -> io::Result<()> {
    #[cfg(unix)]
    std::os::unix::fs::FileExt::write_all_at(file, record, offset)?;
    #[cfg(not(unix))]
    {
        let mut file = file;
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(record)?;
    }
    Ok(())
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

/// Bytes in each of the seqnums file's two slots.
const SLOT: usize = 128;
/// What starts a slot's record, setting it apart from a record from before slots.
const SLOT_MARK: &str = "S2";

/// The seqnums file's latest record.
struct Record {
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    generation: u64,
    /// The slot it's in: `None` for a fresh file, and 0 for a record from before slots, which
    /// sits where slot 0 is, so the next write goes to slot 1 and leaves it whole.
    slot: Option<usize>,
}

/// A slot's record: mark, generation, the three numbers and a checksum of them, padded with
/// spaces to a newline. Written digit by digit on the stack: it's written for every message.
fn slot_record(generation: u64, next_outgoing: u64, next_incoming: u64, in_flight: u64) -> [u8; SLOT] {
    let mut record = [b' '; SLOT];
    record[..SLOT_MARK.len()].copy_from_slice(SLOT_MARK.as_bytes());
    // The numbers, each 20 digits (a u64 has at most 20) and a space apart, after the mark.
    let start = SLOT_MARK.len() + 1;
    for (i, n) in [generation, next_outgoing, next_incoming, in_flight].into_iter().enumerate() {
        put_digits(&mut record[start + i * 21..start + i * 21 + 20], n);
    }
    let end = start + 4 * 21 - 1;
    let sum = record_checksum(&record[start..end]);
    for (i, byte) in record[end + 1..end + 17].iter_mut().enumerate() {
        *byte = b"0123456789abcdef"[usize::try_from((sum >> (60 - 4 * i)) & 0xf).expect("a nibble")];
    }
    record[SLOT - 1] = b'\n';
    record
}

/// `n` in decimal, zero-padded to fill `out`.
fn put_digits(out: &mut [u8], mut n: u64) {
    for byte in out.iter_mut().rev() {
        *byte = b'0' + (n % 10) as u8;
        n /= 10;
    }
    debug_assert_eq!(n, 0, "fits");
}

/// FNV-1a over a record's numbers, so a torn or mixed one is recognised.
fn record_checksum(numbers: &[u8]) -> u64 {
    numbers.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &b| (hash ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01B3))
}

/// A slot's record, if it's whole: generation, next outgoing, next incoming, in flight.
fn parse_slot(bytes: &[u8]) -> Option<[u64; 4]> {
    let text = std::str::from_utf8(bytes.get(..SLOT)?).ok()?;
    let mut fields = text.split_whitespace();
    if fields.next()? != SLOT_MARK {
        return None;
    }
    let numbers: Vec<&str> = fields.by_ref().take(4).collect();
    let sum = u64::from_str_radix(fields.next()?, 16).ok()?;
    let numbers_text = numbers.join(" ");
    if fields.next().is_some() || numbers.len() != 4 || record_checksum(numbers_text.as_bytes()) != sum {
        return None;
    }
    let parsed: Vec<u64> = numbers.iter().map(|n| n.parse().ok()).collect::<Option<_>>()?;
    let [generation, out, inc, in_flight] = parsed.try_into().ok()?;
    (out > 0 && inc > 0).then_some([generation, out, inc, in_flight])
}

/// The latest record in the seqnums file: the valid slot with the higher generation; or a record
/// from before slots, of two numbers (from before the in-flight field) or three; or, for a
/// fresh file or one whose first write was torn, both numbers at 1.
fn read_seqnums(file: &mut File, path: &Path) -> io::Result<Record> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let slots = [0, 1].map(|slot| bytes.get(slot * SLOT..).and_then(parse_slot).map(|numbers| (slot, numbers)));
    if let Some((slot, [generation, out, inc, in_flight])) = slots.into_iter().flatten().max_by_key(|(_, n)| n[0]) {
        let in_flight = (in_flight > 0).then_some(in_flight);
        return Ok(Record { next_outgoing: out, next_incoming: inc, in_flight, generation, slot: Some(slot) });
    }
    // No slot is whole. A record from before slots ends at its newline; slot 1 may hold a torn
    // write made after it.
    let old = bytes.iter().position(|&b| b == b'\n').map(|end| &bytes[..end]);
    let old = old.and_then(|line| std::str::from_utf8(line).ok());
    let numbers: Option<Vec<u64>> = old.and_then(|line| line.split_whitespace().map(|n| n.parse().ok()).collect());
    let fresh = Record { next_outgoing: 1, next_incoming: 1, in_flight: None, generation: 0, slot: None };
    let old = |next_outgoing, next_incoming, in_flight| Record {
        next_outgoing,
        next_incoming,
        in_flight,
        slot: Some(0),
        ..fresh
    };
    match numbers.as_deref() {
        Some(&[out, inc]) if out > 0 && inc > 0 => Ok(old(out, inc, None)),
        Some(&[out, inc, in_flight]) if out > 0 && inc > 0 => Ok(old(out, inc, (in_flight > 0).then_some(in_flight))),
        // Nothing past where slot 0 would end: at most a torn first write, so nothing was recorded.
        _ if bytes.len() <= SLOT => Ok(fresh),
        _ => Err(invalid_data(format!("{} is corrupt: {:?}", path.display(), String::from_utf8_lossy(&bytes)))),
    }
}

/// Indexes every complete message in the body file. Returns the index and the length of the
/// valid prefix; anything after it is an incomplete trailing write.
fn scan_body(file: &mut File, path: &Path) -> io::Result<(BTreeMap<u64, Extent>, u64)> {
    file.seek(SeekFrom::Start(0))?;
    let mut index = BTreeMap::new();
    // Holds one message and a chunk. Stored messages aren't held to the codec's MAX_BODY_LENGTH,
    // but the store wrote this file itself, so the longest is the longest the session sent.
    let mut buf = Vec::new();
    let mut chunk = vec![0; 64 * 1024];
    let mut offset = 0u64;
    // Bytes of `buf` already indexed. Dropped once per read rather than once per message, which
    // would shift the rest of the buffer every time.
    let mut consumed = 0;
    loop {
        // Only the framing and MsgSeqNum are checked: the store never parses the body, which
        // takes the session's data fields. The session parses a message when it resends it.
        match frame_stored(&buf[consumed..]) {
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
    use super::super::commit_now;
    use super::super::conformance::{app_message, check, id};
    use super::*;

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
            commit_now(log.as_mut()).unwrap();
        }
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming()), (2, 3));
        assert_eq!(log.sent_messages(1, 1).unwrap()[0].1, app_message(1));
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
        commit_now(log.as_mut()).unwrap();
        drop(log);
        let log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming(), log.in_flight()), (5, 9, Some(9)));
    }

    fn seqnums_path(dir: &tempfile::TempDir) -> PathBuf {
        dir.path().join(format!("{}.seqnums", file_stem(&id("A"))))
    }

    /// The numbers a store for "A" opens with.
    fn numbers(dir: &tempfile::TempDir) -> (u64, u64, Option<u64>) {
        let log = storage(dir).open(&id("A")).unwrap();
        (log.next_outgoing(), log.next_incoming(), log.in_flight())
    }

    /// Runs `change` on the store for "A", then puts back the seqnums file as a power loss part-way
    /// through that write could leave it: for each length of what reached the device, the new
    /// bytes up to there over the old ones. Returns each reopened store's numbers.
    fn torn(dir: &tempfile::TempDir, change: impl Fn(&mut dyn SessionLog)) -> Vec<(u64, u64, Option<u64>)> {
        let path = seqnums_path(dir);
        let before = std::fs::read(&path).unwrap_or_default();
        {
            let mut log = storage(dir).open(&id("A")).unwrap();
            change(log.as_mut());
            commit_now(log.as_mut()).unwrap();
        }
        let after = std::fs::read(&path).unwrap();
        // The bytes the write changed, which a tear leaves partly new.
        let start = (0..after.len()).find(|&i| before.get(i) != Some(&after[i])).unwrap();
        let end = (0..after.len()).rev().find(|&i| before.get(i) != Some(&after[i])).unwrap() + 1;
        (start..=end)
            .map(|cut| {
                let mut bytes = after[..cut].to_vec();
                bytes.extend(before.iter().skip(cut));
                std::fs::write(&path, &bytes).unwrap();
                let opened = numbers(dir);
                std::fs::write(&path, &after).unwrap();
                opened
            })
            .collect()
    }

    /// A power loss during the first write leaves the store as it was before: fresh.
    #[test]
    fn a_torn_first_record_reads_as_before_it() {
        let dir = tempfile::tempdir().unwrap();
        let opened = torn(&dir, |log| log.set_next_incoming(2).unwrap());
        assert!(opened.iter().all(|n| [(1, 1, None), (1, 2, None)].contains(n)), "{opened:?}");
        assert_eq!(opened.first(), Some(&(1, 1, None)));
    }

    /// A torn rewrite never yields numbers that were never recorded, such as a mix of the old
    /// and new digits (19 becoming 20, torn after the 2, would read as 29).
    #[test]
    fn a_torn_record_reads_as_the_old_one_or_the_new() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            for seq in 2..=19 {
                log.set_next_incoming(seq).unwrap();
                commit_now(log.as_mut()).unwrap();
            }
        }
        for (change, new) in [(20, (1, 20, None)), (99, (1, 99, None))] {
            let old = numbers(&dir);
            let opened = torn(&dir, |log| log.set_next_incoming(change).unwrap());
            assert!(opened.iter().all(|n| *n == old || *n == new), "{old:?} to {new:?}: {opened:?}");
            assert_eq!(numbers(&dir), new);
        }
    }

    /// The first write after upgrading leaves a record from before slots intact until it's done.
    #[test]
    fn a_torn_first_write_over_an_old_record_keeps_it() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(seqnums_path(&dir), format!("{:020} {:020} {:020}\n", 5, 9, 0)).unwrap();
        let opened = torn(&dir, |log| log.set_next_incoming(10).unwrap());
        assert!(opened.iter().all(|n| [(5, 9, None), (5, 10, None)].contains(n)), "{opened:?}");
    }

    /// Nothing reaches the files until a commit, and a crash before one loses what it would have
    /// written (none of which has been sent).
    #[test]
    fn changes_wait_for_a_commit() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            log.record_outgoing(1, Some(&app_message(1))).unwrap();
            log.set_next_incoming(5).unwrap();
            assert!(!body_path(&dir, "A").exists(), "no segment until a commit writes one");
        }
        assert_eq!(numbers(&dir), (1, 1, None));
        assert!(storage(&dir).open(&id("A")).unwrap().sent_messages(1, 1).unwrap().is_empty());
    }

    /// With fsync, the commit is handed back to run on another thread, and writes nothing until
    /// it runs.
    #[test]
    fn a_synced_store_hands_back_its_commit() {
        let dir = tempfile::tempdir().unwrap();
        let synced = DiskStorage::new(dir.path(), true).unwrap();
        let mut log = synced.open(&id("A")).unwrap();
        for seq in 1..=3 {
            log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
        }
        assert_eq!(log.sent_messages(1, 3).unwrap().len(), 3, "reads see what isn't committed");
        let commit = log.commit().unwrap().expect("a commit to run");
        assert_eq!(fs::metadata(body_path(&dir, "A")).unwrap().len(), 0, "created, but not yet written");
        // No call is made of the log until the commit has run.
        commit.run().unwrap();
        assert_eq!(log.sent_messages(1, 3).unwrap().len(), 3);
        assert!(log.commit().unwrap().is_none(), "nothing more to commit");
        drop(log);
        let mut log = synced.open(&id("A")).unwrap();
        assert_eq!(log.next_outgoing(), 4);
        assert_eq!(log.sent_messages(1, 3).unwrap().len(), 3);
    }

    /// A store whose segments hold `per_segment` messages of [`app_message`]'s size (seq 1 to 9),
    /// keeping at most `kept` messages' worth.
    fn rotating(dir: &tempfile::TempDir, per_segment: u64, kept: u64) -> DiskStorage {
        let len = app_message(1).len() as u64;
        storage(dir).with_segment_bytes(per_segment * len).with_max_session_bytes(kept * len)
    }

    /// Stores `seqs` on `log`, committing each.
    fn store_each(log: &mut dyn SessionLog, seqs: std::ops::RangeInclusive<u64>) {
        for seq in seqs {
            log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
            commit_now(log).unwrap();
        }
    }

    fn stored(log: &mut dyn SessionLog) -> Vec<u64> {
        log.sent_messages(1, u64::MAX).unwrap().into_iter().map(|(seq, _)| seq).collect()
    }

    fn segment_path(dir: &tempfile::TempDir, n: u64) -> PathBuf {
        let body = body_path(dir, "A");
        if n == 0 { body } else { body.with_extension(format!("body.{n}")) }
    }

    #[test]
    fn a_full_segment_starts_another() {
        let dir = tempfile::tempdir().unwrap();
        let storage = rotating(&dir, 2, 100);
        {
            let mut log = storage.open(&id("A")).unwrap();
            store_each(log.as_mut(), 1..=5);
            assert_eq!(stored(log.as_mut()), [1, 2, 3, 4, 5]);
        }
        let len = app_message(1).len() as u64;
        let lens: Vec<u64> = (0..3).map(|n| fs::metadata(segment_path(&dir, n)).unwrap().len()).collect();
        assert_eq!(lens, [2 * len, 2 * len, len]);
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(stored(log.as_mut()), [1, 2, 3, 4, 5], "reopened across segments");
        assert_eq!((log.next_outgoing(), log.evicted_through()), (6, None));
    }

    #[test]
    fn the_oldest_segments_go_past_the_budget() {
        let dir = tempfile::tempdir().unwrap();
        let storage = rotating(&dir, 2, 4);
        {
            let mut log = storage.open(&id("A")).unwrap();
            store_each(log.as_mut(), 1..=7);
            // Segments {1, 2} and {3, 4} went as 5 and then 7 passed four messages' worth.
            assert_eq!(stored(log.as_mut()), [5, 6, 7]);
            assert_eq!(log.evicted_through(), Some(4));
        }
        assert!(!segment_path(&dir, 0).exists() && !segment_path(&dir, 1).exists());
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(stored(log.as_mut()), [5, 6, 7]);
        assert_eq!((log.next_outgoing(), log.evicted_through()), (8, Some(4)));
    }

    #[test]
    fn a_torn_write_in_the_newest_segment_is_truncated() {
        let dir = tempfile::tempdir().unwrap();
        let storage = rotating(&dir, 2, 100);
        {
            let mut log = storage.open(&id("A")).unwrap();
            store_each(log.as_mut(), 1..=3);
        }
        let partial = &app_message(4)[..20];
        OpenOptions::new().append(true).open(segment_path(&dir, 1)).unwrap().write_all(partial).unwrap();
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!(stored(log.as_mut()), [1, 2, 3]);
        store_each(log.as_mut(), 4..=4);
        assert_eq!(stored(log.as_mut()), [1, 2, 3, 4]);
    }

    #[test]
    fn a_reset_deletes_every_segment() {
        let dir = tempfile::tempdir().unwrap();
        let storage = rotating(&dir, 2, 4);
        let mut log = storage.open(&id("A")).unwrap();
        store_each(log.as_mut(), 1..=7);
        log.reset().unwrap();
        assert!(
            (0..4).all(|n| !segment_path(&dir, n).exists() || fs::metadata(segment_path(&dir, n)).unwrap().len() == 0)
        );
        assert_eq!((stored(log.as_mut()), log.evicted_through()), (vec![], None));
        store_each(log.as_mut(), 1..=1);
        assert_eq!(fs::metadata(segment_path(&dir, 0)).unwrap().len(), app_message(1).len() as u64);
    }

    #[test]
    fn conforms_with_small_segments() {
        let dir = tempfile::tempdir().unwrap();
        check(&storage(&dir).with_segment_bytes(300).with_max_session_bytes(1 << 20));
        check(&DiskStorage::new(dir.path().join("synced"), true).unwrap().with_segment_bytes(300));
    }

    #[test]
    #[should_panic(expected = "segment")]
    fn a_segment_larger_than_the_budget_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let _ = storage(&dir).with_max_session_bytes(5).with_segment_bytes(10);
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
            commit_now(log.as_mut()).unwrap();
        }
        let path = body_path(&dir, "A");
        let good_len = fs::metadata(&path).unwrap().len();
        let partial = &app_message(2)[..20];
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
            commit_now(log.as_mut()).unwrap();
        }
        // Simulate a crash between the body append and the seqnums update.
        let bytes = app_message(2);
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

    /// The 64 KiB limit on BodyLength(9) is for input from the counterparty: what the session
    /// sent is stored, reopened and read back whatever its size.
    #[test]
    fn stored_messages_over_64_kib_survive_reopening() {
        use crate::message::{Message, tags};
        let dir = tempfile::tempdir().unwrap();
        let big = crate::codec::encode(
            &Message::default()
                .with(tags::BEGIN_STRING, "FIX.4.4")
                .with(tags::MSG_TYPE, "8")
                .with(tags::MSG_SEQ_NUM, 1u64)
                .with(tags::TEXT, "x".repeat(70 * 1024)),
        )
        .unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            log.record_outgoing(1, Some(&big)).unwrap();
            commit_now(log.as_mut()).unwrap();
        }
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!(log.next_outgoing(), 2);
        assert_eq!(log.sent_messages(1, 1).unwrap(), [(1, big)]);
    }

    /// The store checks a stored message's framing, not its fields: parsing them is the session's
    /// job.
    #[test]
    fn stored_message_with_a_malformed_field_is_returned_as_stored() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = storage(&dir).open(&id("A")).unwrap();
        log.record_outgoing(1, Some(&app_message(1))).unwrap();
        commit_now(log.as_mut()).unwrap();
        // Make ExecID(17) non-UTF-8, with a valid CheckSum so only the field is at fault.
        let path = body_path(&dir, "A");
        let mut bytes = fs::read(&path).unwrap();
        let at = bytes.windows(5).position(|w| w == b"\x0117=E").unwrap() + 4;
        bytes[at] = 0xff;
        let trailer = bytes.len() - 7;
        let sum = crate::codec::checksum(&bytes[..trailer]);
        bytes[trailer..].copy_from_slice(format!("10={sum:03}\x01").as_bytes());
        fs::write(&path, &bytes).unwrap();

        assert_eq!(log.sent_messages(1, 1).unwrap(), [(1, bytes.clone())]);
        drop(log);
        let mut log = storage(&dir).open(&id("A")).unwrap();
        assert_eq!(log.sent_messages(1, 1).unwrap(), [(1, bytes)]);
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
