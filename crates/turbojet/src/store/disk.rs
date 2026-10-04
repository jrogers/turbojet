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
/// - `<name>.body`, `<name>.body.1`, `<name>.body.2` and so on: a journal, in segments, of sent
///   application messages (see [`SessionLog::record_outgoing`]), whatever their size, each
///   commit's followed by a record of the session's state: the next outgoing and incoming
///   sequence numbers and the incoming message in flight to the application (0 for none), with a
///   generation and a checksum. A segment that has reached [`with_segment_bytes`] (64 MiB by
///   default) is full, and the next commit that stores messages starts another. Opening the log
///   scans the segments to index sequence numbers by where they are; resends then read messages
///   back from disk.
/// - `<name>.seqnums`: the records of commits that store no messages, in two fixed-size slots
///   written alternately, so a write that a crash or power loss tears damages only the slot it
///   was writing. Kept apart so that sessions that commit often without sending (heartbeats,
///   inbound flow) don't fill the budget with records. A file from before slots (one record, of
///   two or three numbers) still reads. The file is locked while the session is open, so two
///   gateway processes cannot share a session.
/// - `<name>.created`: when the state was created or last reset, as a FIX UTCTimestamp, once
///   recorded; used by session schedules. Replaced atomically.
///
/// Opening takes the valid record with the highest generation, in the journal or the seqnums
/// file. Turbojet 0.1 kept every record in the seqnums file: its stores open as they were, and
/// gain journal records as they store messages. 0.1 can't open a store with a journal record: it
/// reports the segment as corrupt.
///
/// Each session keeps at most [`with_max_session_bytes`] of journal (1 GiB by default): past it,
/// the oldest segments are deleted, and a resend that reaches back to their messages gap-fills
/// them, so the counterparty never receives them again (the session logs a warning and counts it
/// in `turbojet_resend_requests_evicted_total`).
///
/// Changes are kept in memory until the session commits them, once per batch of work: then the
/// messages stored since and their record are appended to a segment in one write, or, if there
/// are none, the record is written to a slot. Without `sync` that's done at once; with it the
/// commit is handed to the connection driver, which runs the write and its `fsync` on a blocking
/// thread, so a commit costs one `fsync`, whatever it holds.
///
/// Recovery on open: a partially written message or record at the end of a segment (from a crash
/// mid-append) is truncated, and the next outgoing sequence number is advanced past the last
/// stored message, in case the crash kept a commit's messages but not its record. A crash before
/// a commit loses what it would have written, none of which has been sent.
///
/// [`with_segment_bytes`]: DiskStorage::with_segment_bytes
/// [`with_max_session_bytes`]: DiskStorage::with_max_session_bytes
#[derive(Debug)]
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
    ///
    /// # Errors
    ///
    /// Any failure creating `dir`.
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
    /// The seqnums file, which holds the records of commits that store no messages. Shared with a
    /// commit under way on another thread.
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
    /// The latest record's generation, in the journal or the seqnums file: 0 if there's none.
    generation: u64,
    /// The seqnums file's slot holding its latest record (none in a fresh file).
    slot: Option<usize>,
    created_path: PathBuf,
    created_at: Option<UtcTimestamp>,
    sync: bool,
}

impl DiskLog {
    fn open(dir: &Path, id: &SessionId, sync: bool, sizes: Sizes) -> io::Result<Self> {
        let stem = file_stem(id);
        let created_path = dir.join(format!("{stem}.created"));
        let seqnums_path = dir.join(format!("{stem}.seqnums"));
        let mut seqnums = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&seqnums_path)?;
        try_lock(&seqnums, &seqnums_path)?;
        let (slotted, slot) = read_seqnums(&mut seqnums, &seqnums_path)?;
        let (segments, index, journal) = open_segments(dir, &stem)?;
        // The latest record is in the journal or the seqnums file, whichever has the higher
        // generation; a 0.1 store's is in the seqnums file until it first stores a message.
        let Record { mut next_outgoing, next_incoming, in_flight, generation } =
            journal.into_iter().chain([slotted]).max_by_key(|r| r.generation).expect("the seqnums file has one");
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

    /// Adds the next record to `pending`, after the messages stored since the last commit, or
    /// alone for a reset. Counted as written: a failed write ends the session, and the next
    /// connection reads the files afresh.
    fn push_record(&mut self) {
        self.generation += 1;
        let record = journal_record(self.generation, self.next_outgoing, self.next_incoming, self.in_flight);
        if self.pending_at.is_none() {
            self.pending_at = Some(self.next_place());
        }
        self.pending.extend_from_slice(&record);
        self.dirty = false;
    }

    /// The next seqnums record, for a commit that stores no messages, and where it goes: the slot
    /// not holding the file's latest, so a torn write leaves that one whole. Counted as written,
    /// as [`push_record`](Self::push_record)'s is.
    fn next_slot_record(&mut self) -> ([u8; SLOT], u64) {
        self.generation += 1;
        let slot = self.slot.map_or(0, |current| 1 - current);
        let record = slot_record(self.generation, self.next_outgoing, self.next_incoming, self.in_flight.unwrap_or(0));
        self.slot = Some(slot);
        self.dirty = false;
        (record, (slot * SLOT) as u64)
    }

    /// Commits a record alone to the seqnums file: a commit that stores no messages leaves the
    /// segments be, so its record doesn't take messages' place in the budget.
    fn commit_slot(&mut self) -> io::Result<Option<Commit>> {
        let (record, offset) = self.next_slot_record();
        if !self.sync {
            write_record(&self.seqnums, &record, offset)?;
            return Ok(None);
        }
        let seqnums = self.seqnums.clone();
        Ok(Some(Commit::blocking(move || {
            write_record(&seqnums, &record, offset)?;
            seqnums.sync_data()
        })))
    }

    /// Appends a record to the newest segment now (a reset), syncing it if the store syncs.
    fn write_record_now(&mut self) -> io::Result<()> {
        debug_assert!(self.pending.is_empty(), "nothing else is pending");
        self.push_record();
        let (file, mut bytes, created) = self.place_pending()?.expect("a record is pending");
        (&*file).write_all(&bytes)?;
        if self.sync {
            file.sync_data()?;
            if created {
                sync_dir(&self.dir)?;
            }
        }
        bytes.clear();
        self.pending = bytes;
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
        debug_assert!(seq >= 1, "sequence numbers start at 1");
        self.next_incoming = seq;
        self.in_flight = None;
        self.dirty = true;
        Ok(())
    }

    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        // One whole message, as sent_messages checks when reading it back.
        super::debug_check_record(seq, self.next_outgoing, msg);
        if let Some(bytes) = msg {
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
        if self.pending.is_empty() {
            return self.commit_slot();
        }
        self.push_record();
        let (file, mut bytes, created) = self.place_pending()?.expect("a record is pending");
        let removed = self.evict();
        if !self.sync {
            // A few cheap system calls: no reason to leave the connection's task.
            (&*file).write_all(&bytes)?;
            bytes.clear();
            self.pending = bytes;
            return remove_files(&removed).map(|()| None);
        }
        let dir = self.dir.clone();
        Ok(Some(Commit::blocking(move || {
            // The messages, then the record that covers them, in one write: a crash keeps a
            // prefix of it, so never a record without its messages (see `open`).
            (&*file).write_all(&bytes)?;
            file.sync_data()?;
            if created {
                // The new segment's name, so it survives a power loss too.
                sync_dir(&dir)?;
            }
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
        super::debug_check_sent(&messages, begin, end, self.evicted_through);
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
        self.write_record_now()?;
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

/// A session's journal as opened: its segments, oldest first, the index of their messages, and
/// the latest record in any.
type Journal = (VecDeque<Segment>, BTreeMap<u64, Location>, Option<Record>);

/// The segments of the session whose files start `stem`, oldest first, each scanned: a torn
/// message or record at the end of one is truncated. Returns them, the index of their messages,
/// and the latest record in any.
///
/// The latest record is the one with the highest generation, wherever it is. It's in the newest
/// segment holding one, except after a reset, which starts again in segment 0: a later segment
/// that a power loss brought back after the reset deleted it holds older records, and is deleted
/// again here. A later segment without a record holds the messages of a commit torn before its
/// record, which are kept.
fn open_segments(dir: &Path, stem: &str) -> io::Result<Journal> {
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
    let mut scanned = Vec::with_capacity(numbers.len());
    for number in numbers {
        let path = segment_path(dir, stem, number);
        let mut file = OpenOptions::new().read(true).append(true).open(&path)?;
        let (found, record, valid_len) = scan_body(&mut file, &path)?;
        let file_len = file.metadata()?.len();
        if valid_len < file_len {
            warn!(
                path = %path.display(),
                discarded = file_len - valid_len,
                "truncating incomplete write at end of session store"
            );
            file.set_len(valid_len)?;
        }
        scanned.push((number, path, file, found, record, valid_len));
    }
    let latest = scanned
        .iter()
        .filter_map(|(number, _, _, _, record, _)| record.map(|r| (*number, r)))
        .max_by_key(|(_, r)| r.generation);
    let mut segments = VecDeque::with_capacity(scanned.len());
    let mut index = BTreeMap::new();
    for (number, path, file, found, record, valid_len) in scanned {
        if let Some((holder, _)) = latest
            && number > holder
            && record.is_some()
        {
            warn!(path = %path.display(), "deleting a segment from before the session store was reset");
            drop(file);
            remove_files(&[path])?;
            continue;
        }
        let seqs = found.first_key_value().zip(found.last_key_value()).map(|((&first, _), (&last, _))| (first, last));
        for (seq, (offset, len)) in found {
            let len = u32::try_from(len)
                .map_err(|_| invalid_data(format!("{}: message {seq} is too long", path.display())))?;
            index.insert(seq, Location { offset, len, segment: number });
        }
        segments.push_back(Segment { number, file: Arc::new(file), len: valid_len, seqs });
    }
    Ok((segments, index, latest.map(|(_, record)| record)))
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

/// Locks `file` for this process, or fails if another holds it.
fn try_lock(file: &File, path: &Path) -> io::Result<()> {
    file.try_lock().map_err(|e| match e {
        TryLockError::WouldBlock => {
            io::Error::new(io::ErrorKind::WouldBlock, format!("{} is locked by another process", path.display()))
        }
        TryLockError::Error(e) => e,
    })
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
/// What starts a journal record. A stored message starts `8=`, so the two can't be mistaken.
const JOURNAL_MARK: &str = "J1";
/// Bytes in a journal record: its mark, the four numbers of 20 digits, a checksum of 16 hex
/// digits, each after a space, and a newline.
pub(crate) const JOURNAL_RECORD: usize = JOURNAL_MARK.len() + 4 * 21 + 17 + 1;

/// The session's state as a record holds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Record {
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
    /// Higher in each record written: the latest is the valid one with the highest.
    generation: u64,
}

/// A slot's record: mark, generation, the three numbers and a checksum of them, padded with
/// spaces to a newline. Written digit by digit on the stack: it's written for every commit that
/// stores no messages.
fn slot_record(generation: u64, next_outgoing: u64, next_incoming: u64, in_flight: u64) -> [u8; SLOT] {
    let mut record = [b' '; SLOT];
    let line = journal_record(generation, next_outgoing, next_incoming, (in_flight > 0).then_some(in_flight));
    // The same fields as a journal record's, after the slot's mark: the journal's is as long.
    debug_assert_eq!(SLOT_MARK.len(), JOURNAL_MARK.len());
    record[..SLOT_MARK.len()].copy_from_slice(SLOT_MARK.as_bytes());
    record[SLOT_MARK.len()..JOURNAL_RECORD - 1].copy_from_slice(&line[JOURNAL_MARK.len()..JOURNAL_RECORD - 1]);
    record[SLOT - 1] = b'\n';
    record
}

/// A journal record: mark, generation, the three numbers and a checksum of them, each after a
/// space, and a newline. Written digit by digit on the stack: it's written for every commit.
fn journal_record(
    generation: u64,
    next_outgoing: u64,
    next_incoming: u64,
    in_flight: Option<u64>,
) -> [u8; JOURNAL_RECORD] {
    let mut record = [b' '; JOURNAL_RECORD];
    record[..JOURNAL_MARK.len()].copy_from_slice(JOURNAL_MARK.as_bytes());
    // The numbers, each 20 digits (a u64 has at most 20) and a space apart, after the mark.
    let start = JOURNAL_MARK.len() + 1;
    for (i, n) in [generation, next_outgoing, next_incoming, in_flight.unwrap_or(0)].into_iter().enumerate() {
        put_digits(&mut record[start + i * 21..start + i * 21 + 20], n);
    }
    let end = start + 4 * 21 - 1;
    let sum = record_checksum(&record[start..end]);
    for (i, byte) in record[end + 1..end + 17].iter_mut().enumerate() {
        *byte = b"0123456789abcdef"[usize::try_from((sum >> (60 - 4 * i)) & 0xf).expect("a nibble")];
    }
    record[JOURNAL_RECORD - 1] = b'\n';
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
    parse_record(bytes.get(..SLOT)?, SLOT_MARK)
}

/// A record starting `mark`, if it's whole: generation, next outgoing, next incoming, in flight.
fn parse_record(bytes: &[u8], mark: &str) -> Option<[u64; 4]> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut fields = text.split_whitespace();
    if fields.next()? != mark {
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

/// A parsed record's numbers (generation, next outgoing, next incoming, in flight) as a [`Record`].
fn record_of([generation, next_outgoing, next_incoming, in_flight]: [u64; 4]) -> Record {
    Record { next_outgoing, next_incoming, in_flight: (in_flight > 0).then_some(in_flight), generation }
}

/// The latest record in a 0.1 store's seqnums file: the valid slot with the higher generation; or a record
/// from before slots, of two numbers (from before the in-flight field) or three; or, for a
/// fresh file or one whose first write was torn, both numbers at 1.
fn read_seqnums(file: &mut File, path: &Path) -> io::Result<(Record, Option<usize>)> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let slots = [0, 1].map(|slot| bytes.get(slot * SLOT..).and_then(parse_slot).map(|numbers| (slot, numbers)));
    if let Some((slot, numbers)) = slots.into_iter().flatten().max_by_key(|(_, n)| n[0]) {
        return Ok((record_of(numbers), Some(slot)));
    }
    // No slot is whole. A record from before slots ends at its newline; slot 1 may hold a torn
    // write made after it.
    let old = bytes.iter().position(|&b| b == b'\n').map(|end| &bytes[..end]);
    let old = old.and_then(|line| std::str::from_utf8(line).ok());
    let numbers: Option<Vec<u64>> = old.and_then(|line| line.split_whitespace().map(|n| n.parse().ok()).collect());
    let fresh = Record { next_outgoing: 1, next_incoming: 1, in_flight: None, generation: 0 };
    let old =
        |next_outgoing, next_incoming, in_flight| Record { next_outgoing, next_incoming, in_flight, generation: 0 };
    match numbers.as_deref() {
        // It sits where slot 0 is, so the next write goes to slot 1 and leaves it whole.
        Some(&[out, inc]) if out > 0 && inc > 0 => Ok((old(out, inc, None), Some(0))),
        Some(&[out, inc, in_flight]) if out > 0 && inc > 0 => {
            Ok((old(out, inc, (in_flight > 0).then_some(in_flight)), Some(0)))
        }
        // Nothing past where slot 0 would end: at most a torn first write, so nothing was recorded.
        _ if bytes.len() <= SLOT => Ok((fresh, None)),
        _ => Err(invalid_data(format!("{} is corrupt: {:?}", path.display(), String::from_utf8_lossy(&bytes)))),
    }
}

/// What a segment holds: its messages' places by MsgSeqNum, its latest record, and the length
/// of its valid prefix.
type Scanned = (BTreeMap<u64, Extent>, Option<Record>, u64);

/// Indexes every complete message in a segment, and finds its latest record. Anything after the
/// valid prefix is an incomplete trailing write: part of a message, or of a record, or a whole
/// record that doesn't check, last in the file (a write torn below a sector).
fn scan_body(file: &mut File, path: &Path) -> io::Result<Scanned> {
    let file_len = file.metadata()?.len();
    file.seek(SeekFrom::Start(0))?;
    let mut index = BTreeMap::new();
    let mut record: Option<Record> = None;
    // Holds one message and a chunk. Stored messages aren't held to the codec's MAX_BODY_LENGTH,
    // but the store wrote this file itself, so the longest is the longest the session sent.
    let mut buf = Vec::new();
    let mut chunk = vec![0; 64 * 1024];
    let mut offset = 0u64;
    // Bytes of `buf` already scanned, dropped by `read_chunk`.
    let mut consumed = 0;
    loop {
        if buf.get(consumed) == Some(&JOURNAL_MARK.as_bytes()[0]) {
            if buf.len() - consumed < JOURNAL_RECORD {
                if read_chunk(file, &mut buf, &mut consumed, &mut chunk)? {
                    continue;
                }
                return Ok((index, record, offset));
            }
            match parse_record(&buf[consumed..consumed + JOURNAL_RECORD], JOURNAL_MARK) {
                Some(numbers) => {
                    let found = record_of(numbers);
                    record = record.into_iter().chain([found]).max_by_key(|r| r.generation);
                    consumed += JOURNAL_RECORD;
                    offset += JOURNAL_RECORD as u64;
                }
                None if offset + JOURNAL_RECORD as u64 >= file_len => return Ok((index, record, offset)),
                None => return Err(invalid_data(format!("{}: corrupt record at offset {offset}", path.display()))),
            }
            continue;
        }
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
                if !read_chunk(file, &mut buf, &mut consumed, &mut chunk)? {
                    return Ok((index, record, offset));
                }
            }
            Err(Decoded::Garbled { reason, .. }) => {
                return Err(invalid_data(format!("{}: corrupt message at offset {offset}: {reason}", path.display())));
            }
            Err(Decoded::Message(..)) => unreachable!("frame doesn't decode"),
        }
    }
}

/// Reads the next chunk of `file` onto `buf`, first dropping the `consumed` bytes before it: once
/// per read rather than once per message, which would shift the rest of the buffer every time.
/// False at the end of the file.
fn read_chunk(file: &mut File, buf: &mut Vec<u8>, consumed: &mut usize, chunk: &mut [u8]) -> io::Result<bool> {
    let n = file.read(chunk)?;
    if n == 0 {
        return Ok(false);
    }
    buf.drain(..*consumed);
    *consumed = 0;
    buf.extend_from_slice(&chunk[..n]);
    Ok(true)
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
    use super::super::conformance::{app_message, check_blocking, id};
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
        check_blocking(&storage(&dir));
        check_blocking(&DiskStorage::new(dir.path().join("synced"), true).unwrap());
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

    /// What reopening a store finds: its numbers and the messages stored.
    type Reopened = ((u64, u64, Option<u64>), Vec<u64>);

    /// What reopening "A" finds.
    fn reopened(dir: &tempfile::TempDir) -> Reopened {
        let mut log = storage(dir).open(&id("A")).unwrap();
        ((log.next_outgoing(), log.next_incoming(), log.in_flight()), stored(log.as_mut()))
    }

    /// Runs `change` on the store for "A" and commits it, then cuts the commit's append to the
    /// journal short at each length a power loss could leave. Returns what each reopened store
    /// finds, shortest cut first.
    fn torn_journal(dir: &tempfile::TempDir, change: impl Fn(&mut dyn SessionLog)) -> Vec<Reopened> {
        let path = body_path(dir, "A");
        let before = fs::metadata(&path).map_or(0, |m| m.len());
        let seqnums = fs::read(seqnums_path(dir)).unwrap_or_default();
        {
            let mut log = storage(dir).open(&id("A")).unwrap();
            change(log.as_mut());
            commit_now(log.as_mut()).unwrap();
        }
        assert_eq!(
            fs::read(seqnums_path(dir)).unwrap_or_default(),
            seqnums,
            "a commit with messages writes only the journal"
        );
        let after = fs::read(&path).unwrap();
        let opened = (before..=after.len() as u64)
            .map(|cut| {
                fs::write(&path, &after[..usize::try_from(cut).unwrap()]).unwrap();
                reopened(dir)
            })
            .collect();
        fs::write(&path, &after).unwrap();
        opened
    }

    /// Runs `change` on the store for "A" and commits it, then puts back the seqnums file as a power
    /// loss part-way through that write could leave it: for each length of what reached the device,
    /// the new bytes up to there over the old ones. Returns each reopened store's numbers.
    fn torn_slot(dir: &tempfile::TempDir, change: impl Fn(&mut dyn SessionLog)) -> Vec<(u64, u64, Option<u64>)> {
        let path = seqnums_path(dir);
        let before = fs::read(&path).unwrap_or_default();
        {
            let mut log = storage(dir).open(&id("A")).unwrap();
            change(log.as_mut());
            commit_now(log.as_mut()).unwrap();
        }
        let after = fs::read(&path).unwrap();
        // The bytes the write changed, which a tear leaves partly new.
        let start = (0..after.len()).find(|&i| before.get(i) != Some(&after[i])).unwrap();
        let end = (0..after.len()).rev().find(|&i| before.get(i) != Some(&after[i])).unwrap() + 1;
        (start..=end)
            .map(|cut| {
                let mut bytes = after[..cut].to_vec();
                bytes.extend(before.iter().skip(cut));
                fs::write(&path, &bytes).unwrap();
                let opened = numbers(dir);
                fs::write(&path, &after).unwrap();
                opened
            })
            .collect()
    }

    /// A power loss during the first write leaves the store as it was before: fresh.
    #[test]
    fn a_torn_first_record_reads_as_before_it() {
        let dir = tempfile::tempdir().unwrap();
        let opened = torn_slot(&dir, |log| log.set_next_incoming(2).unwrap());
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
            let opened = torn_slot(&dir, |log| log.set_next_incoming(change).unwrap());
            assert!(opened.iter().all(|n| *n == old || *n == new), "{old:?} to {new:?}: {opened:?}");
            assert_eq!(numbers(&dir), new);
        }
    }

    /// The first write after upgrading leaves a record from before slots intact until it's done.
    #[test]
    fn a_torn_first_write_over_an_old_record_keeps_it() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(seqnums_path(&dir), format!("{:020} {:020} {:020}\n", 5, 9, 0)).unwrap();
        let opened = torn_slot(&dir, |log| log.set_next_incoming(10).unwrap());
        assert!(opened.iter().all(|n| [(5, 9, None), (5, 10, None)].contains(n)), "{opened:?}");
    }

    /// A commit that stores messages, cut anywhere, reopens as before it or after it: never numbers
    /// that weren't recorded. Its messages are kept as far as they reached, and the next outgoing
    /// number is past them, so none is sent again under its number.
    #[test]
    fn a_torn_commit_reads_as_before_or_after_it() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            store_each(log.as_mut(), 1..=2);
            log.set_next_incoming(19).unwrap();
            commit_now(log.as_mut()).unwrap();
        }
        let opened = torn_journal(&dir, |log| {
            for seq in 3..=4 {
                log.record_outgoing(seq, Some(&app_message(seq))).unwrap();
            }
            log.set_next_incoming(20).unwrap();
            log.set_in_flight(20).unwrap();
        });
        let expected = [
            ((3, 19, None), vec![1, 2]),
            ((4, 19, None), vec![1, 2, 3]),
            ((5, 19, None), vec![1, 2, 3, 4]),
            ((5, 20, Some(20)), vec![1, 2, 3, 4]),
        ];
        assert!(opened.iter().all(|found| expected.contains(found)), "{opened:?}");
        assert_eq!(opened.first(), Some(&expected[0]));
        assert_eq!(opened.last(), Some(&expected[3]));
        assert!(expected.iter().all(|e| opened.contains(e)), "every prefix is seen: {opened:?}");
    }

    /// Records go wherever the commit writes: with its messages in the journal, or alone in the
    /// seqnums file. The latest, by generation, wins, whichever file it's in.
    #[test]
    fn the_latest_record_wins_wherever_it_is() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = storage(&dir).open(&id("A")).unwrap();
        store_each(log.as_mut(), 1..=1);
        assert_eq!(fs::metadata(seqnums_path(&dir)).unwrap().len(), 0, "the record went with the message");
        log.set_next_incoming(7).unwrap();
        commit_now(log.as_mut()).unwrap();
        let journal_len = fs::metadata(body_path(&dir, "A")).unwrap().len();
        drop(log);
        assert_eq!(numbers(&dir), (2, 7, None), "the seqnums file's record is later");
        let mut log = storage(&dir).open(&id("A")).unwrap();
        store_each(log.as_mut(), 2..=2);
        assert!(fs::metadata(body_path(&dir, "A")).unwrap().len() > journal_len);
        drop(log);
        assert_eq!(numbers(&dir), (3, 7, None), "the journal's record is later");
    }

    /// A store from 0.1, whose records are all in a slotted seqnums file, opens as it was, and its
    /// generations go on: a journal record written later is the latest.
    #[test]
    fn a_store_from_0_1_opens_and_carries_on() {
        let dir = tempfile::tempdir().unwrap();
        // Generation 41, in slot 1; slot 0 holds an older one.
        let mut old = vec![b' '; 2 * SLOT];
        old[..SLOT].copy_from_slice(&slot_record(40, 6, 8, 0));
        old[SLOT..].copy_from_slice(&slot_record(41, 7, 8, 0));
        fs::write(seqnums_path(&dir), &old).unwrap();
        assert_eq!(numbers(&dir), (7, 8, None));
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            store_each(log.as_mut(), 7..=7);
        }
        assert_eq!(fs::read(seqnums_path(&dir)).unwrap(), old, "the message's commit didn't touch it");
        assert_eq!(numbers(&dir), (8, 8, None), "generation 42 is later than 41");
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
    /// each committed alone, keeping at most `kept` messages' worth.
    fn rotating(dir: &tempfile::TempDir, per_segment: u64, kept: u64) -> DiskStorage {
        // Each committed alone, with its record.
        let len = (app_message(1).len() + JOURNAL_RECORD) as u64;
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
        let len = (app_message(1).len() + JOURNAL_RECORD) as u64;
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
        assert!((1..4).all(|n| !segment_path(&dir, n).exists()));
        assert_eq!(fs::metadata(segment_path(&dir, 0)).unwrap().len(), JOURNAL_RECORD as u64, "the reset's record");
        assert_eq!((stored(log.as_mut()), log.evicted_through()), (vec![], None));
        store_each(log.as_mut(), 1..=1);
        let len = (app_message(1).len() + 2 * JOURNAL_RECORD) as u64;
        assert_eq!(fs::metadata(segment_path(&dir, 0)).unwrap().len(), len);
    }

    /// A segment a reset deleted that a power loss brings back holds records older than the
    /// reset's: it's deleted again, so neither its numbers nor its messages come back.
    #[test]
    fn a_segment_from_before_a_reset_is_deleted_again() {
        let dir = tempfile::tempdir().unwrap();
        let storage = rotating(&dir, 2, 100);
        let mut log = storage.open(&id("A")).unwrap();
        store_each(log.as_mut(), 1..=5);
        let resurrected = fs::read(segment_path(&dir, 2)).unwrap();
        log.reset().unwrap();
        store_each(log.as_mut(), 1..=1);
        drop(log);
        fs::write(segment_path(&dir, 2), &resurrected).unwrap();
        let mut log = storage.open(&id("A")).unwrap();
        assert_eq!((log.next_outgoing(), log.next_incoming(), stored(log.as_mut())), (2, 1, vec![1]));
        assert!(!segment_path(&dir, 2).exists());
    }

    #[test]
    fn conforms_with_small_segments() {
        let dir = tempfile::tempdir().unwrap();
        check_blocking(&storage(&dir).with_segment_bytes(300).with_max_session_bytes(1 << 20));
        check_blocking(&DiskStorage::new(dir.path().join("synced"), true).unwrap().with_segment_bytes(300));
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

    /// A record that doesn't check is a torn write if it's the last thing in its segment, and
    /// corruption anywhere else.
    #[test]
    fn a_damaged_record_is_torn_at_the_end_and_corrupt_before_it() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut log = storage(&dir).open(&id("A")).unwrap();
            store_each(log.as_mut(), 1..=2);
        }
        let path = body_path(&dir, "A");
        let whole = fs::read(&path).unwrap();
        let first_record = app_message(1).len() + JOURNAL_RECORD - 2;
        for (at, opens) in [(whole.len() - 2, true), (first_record, false)] {
            let mut bytes = whole.clone();
            bytes[at] = b'x';
            fs::write(&path, &bytes).unwrap();
            match storage(&dir).open(&id("A")) {
                Ok(mut log) => {
                    assert!(opens, "damage at {at} should fail");
                    assert_eq!((log.next_outgoing(), stored(log.as_mut())), (3, vec![1, 2]));
                    assert_eq!(fs::metadata(&path).unwrap().len(), (whole.len() - JOURNAL_RECORD) as u64);
                }
                Err(e) => {
                    assert!(!opens, "damage at {at} should be truncated: {e}");
                    assert_eq!(e.kind(), io::ErrorKind::InvalidData);
                }
            }
        }
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
        let mut segment = fs::read(&path).unwrap();
        let bytes = &mut segment[..app_message(1).len()];
        let at = bytes.windows(5).position(|w| w == b"\x0117=E").unwrap() + 4;
        bytes[at] = 0xff;
        let trailer = bytes.len() - 7;
        let sum = crate::codec::checksum(&bytes[..trailer]);
        bytes[trailer..].copy_from_slice(format!("10={sum:03}\x01").as_bytes());
        let bytes = bytes.to_vec();
        fs::write(&path, &segment).unwrap();

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
