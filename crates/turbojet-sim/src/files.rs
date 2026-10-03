//! A disk store's files, as a power loss leaves them: what of a write in progress reached the
//! device, and, without fsync, how far the OS had written back.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

/// The length of a journal record, which ends each append to the journal (`DiskStorage`'s
/// format).
const JOURNAL_RECORD: u64 = 104;

/// One session's `DiskStorage` files, in a directory holding only that session.
#[derive(Debug, Clone)]
pub struct DiskFiles {
    dir: PathBuf,
}

/// What the files held at a moment: the seqnums file, and each journal segment's length by its
/// number.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    seqnums: Vec<u8>,
    segments: BTreeMap<u64, u64>,
}

/// The journal segments' contents at a moment, to put back the ones a commit deleted.
#[derive(Debug, Default)]
pub struct Backup {
    segments: BTreeMap<PathBuf, Vec<u8>>,
}

/// How a write in progress at a power loss is torn. A commit writes either the journal (its
/// messages, then their record, appended to the newest segment) or, storing no messages, a record
/// in the seqnums file.
#[derive(Debug, Clone, Copy)]
pub struct Tear {
    /// How much of the commit's write reached the device, in thousandths.
    pub cut: u64,
    /// Instead: for the journal, all of the messages and this much (thousandths) of their
    /// record; for the seqnums file, this much of its record.
    pub in_record: Option<u64>,
    /// A record torn part-way is a mix of new and old bytes rather than one or the other whole,
    /// as if the device didn't write a sector atomically. In the journal the old bytes are zeros:
    /// the record keeps its full length, as if the file's length reached the device before all of
    /// its data.
    pub sub_sector: bool,
}

impl DiskFiles {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// The file with `extension`, if the session has made it.
    fn path(&self, extension: &str) -> Option<PathBuf> {
        fs::read_dir(&self.dir)
            .ok()?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .find(|p| p.extension().is_some_and(|e| e == extension))
    }

    /// The journal segments, by number: `<stem>.body` is 0, `<stem>.body.<n>` is n.
    fn segments(&self) -> BTreeMap<u64, PathBuf> {
        let Ok(entries) = fs::read_dir(&self.dir) else { return BTreeMap::new() };
        entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter_map(|path| {
                let name = path.file_name()?.to_str()?;
                let number = if name.ends_with(".body") { 0 } else { name.rsplit_once(".body.")?.1.parse().ok()? };
                Some((number, path))
            })
            .collect()
    }

    pub fn snapshot(&self) -> Snapshot {
        let seqnums = self.path("seqnums").and_then(|p| fs::read(p).ok()).unwrap_or_default();
        let segments = self
            .segments()
            .into_iter()
            .map(|(number, path)| (number, fs::metadata(path).map_or(0, |m| m.len())))
            .collect();
        Snapshot { seqnums, segments }
    }

    pub fn backup(&self) -> Backup {
        let segments = self.segments().into_values().filter_map(|p| Some((p.clone(), fs::read(&p).ok()?))).collect();
        Backup { segments }
    }

    /// The write made since `before` (whose segments' contents are `backup`) was cut short by a
    /// power loss, as `tear` says. A synced commit deletes the segments it evicts only once its
    /// write is durable, so a torn one deleted none: they're put back.
    pub fn tear(&self, before: &Snapshot, backup: &Backup, tear: Tear) {
        for (path, bytes) in &backup.segments {
            if !path.exists() {
                fs::write(path, bytes).expect("a segment writes");
            }
        }
        let after = self.snapshot();
        if after.seqnums != before.seqnums {
            self.tear_seqnums(before, &after, tear);
            return;
        }
        // A commit appends to one segment, the newest, ending with the record.
        let (newest, newest_len) = after.segments.last_key_value().map_or((0, 0), |(n, len)| (*n, *len));
        let before_len = before.segments.get(&newest).copied().unwrap_or(0);
        let appended = newest_len - before_len;
        let record_start = appended.saturating_sub(JOURNAL_RECORD);
        let reached = match tear.in_record {
            Some(part) => record_start + (appended - record_start) * part / 1000,
            None => appended * tear.cut / 1000,
        };
        if tear.sub_sector && reached > record_start && reached < appended {
            self.zero_segment_tail(newest, before_len + reached, before_len + appended);
        } else {
            self.truncate_segment(newest, before_len + reached);
        }
    }

    /// A commit's write of a seqnums record, torn: the bytes it changed, wherever in the file they
    /// are, partly new.
    fn tear_seqnums(&self, before: &Snapshot, after: &Snapshot, tear: Tear) {
        let changed = (0..after.seqnums.len()).filter(|&i| before.seqnums.get(i) != Some(&after.seqnums[i]));
        let (lo, hi) = changed
            .fold(None, |range, i| match range {
                None => Some((i, i + 1)),
                Some((lo, _)) => Some((lo, i + 1)),
            })
            .unwrap_or((0, 0));
        let record = hi - lo;
        let written = record * usize::try_from(tear.in_record.unwrap_or(tear.cut)).expect("small") / 1000;
        let seqnums = if tear.sub_sector {
            // The new bytes reached the device up to `written`, the old ones are left after.
            let mut mixed = after.seqnums[..lo + written].to_vec();
            mixed.extend(before.seqnums.iter().skip(lo + written));
            mixed
        } else if written == record {
            after.seqnums.clone()
        } else {
            before.seqnums.clone()
        };
        self.write_seqnums(&seqnums);
    }

    /// A power loss without fsync: the OS had written back everything up to `checkpoint`, and
    /// some of what followed. `kept` (thousandths) says how much of what was appended to the
    /// journal since; `new_seqnums`, whether the seqnums file's latest rewrite got there. Each
    /// segment keeps what it had at the checkpoint (none, for one made since) and that part of
    /// what was appended since; deletions since stay made.
    pub fn revert(&self, checkpoint: &Snapshot, kept: u64, new_seqnums: bool) {
        let now = self.snapshot();
        for (&number, &len) in &now.segments {
            let before = checkpoint.segments.get(&number).copied().unwrap_or(0).min(len);
            // Never longer than the file is: truncating can only lose what was written.
            self.truncate_segment(number, before + (len - before) * kept / 1000);
        }
        if !new_seqnums {
            self.write_seqnums(&checkpoint.seqnums);
        }
    }

    fn truncate_segment(&self, number: u64, len: u64) {
        if let Some(path) = self.segments().remove(&number) {
            OpenOptions::new().write(true).open(&path).and_then(|f| f.set_len(len)).expect("a segment truncates");
        }
    }

    /// Replaces segment `number`'s bytes from `from` to `to` (its end) with zeros.
    fn zero_segment_tail(&self, number: u64, from: u64, to: u64) {
        if let Some(path) = self.segments().remove(&number) {
            let mut bytes = fs::read(&path).expect("a segment reads");
            let (from, to) = (usize::try_from(from).expect("small"), usize::try_from(to).expect("small"));
            bytes[from..to].fill(0);
            fs::write(&path, &bytes).expect("a segment writes");
        }
    }

    fn write_seqnums(&self, bytes: &[u8]) {
        if let Some(path) = self.path("seqnums") {
            write(&path, bytes);
        }
    }
}

fn write(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("the seqnums file writes");
}
