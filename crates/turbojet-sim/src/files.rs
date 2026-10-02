//! A disk store's files, as a power loss leaves them: what of a write in progress reached the
//! device, and, without fsync, how far the OS had written back.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

/// One session's `DiskStorage` files, in a directory holding only that session.
#[derive(Debug, Clone)]
pub struct DiskFiles {
    dir: PathBuf,
}

/// What the files held at a moment: the seqnums record, and each body segment's length by its
/// number.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    seqnums: Vec<u8>,
    segments: BTreeMap<u64, u64>,
}

/// How a write in progress at a power loss is torn.
#[derive(Debug, Clone, Copy)]
pub struct Tear {
    /// How much of the commit's writes (the append to the newest segment, then the seqnums
    /// record) reached the device, in thousandths.
    pub cut: u64,
    /// Instead: all of the body append, and this much (thousandths) of the seqnums record. A
    /// record torn part-way is where a sub-sector tear can mix old and new digits.
    pub in_record: Option<u64>,
    /// A seqnums record partly written is a mix of new and old bytes, rather than one or the other
    /// whole: as if the device didn't write a sector atomically.
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

    /// The body segments, by number: `<stem>.body` is 0, `<stem>.body.<n>` is n.
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

    /// The writes made since `before` were cut short by a power loss, as `tear` says.
    pub fn tear(&self, before: &Snapshot, tear: Tear) {
        let after = self.snapshot();
        // A commit appends to one segment, the newest; segments it deleted stay deleted (a
        // deletion a power loss undid would only mean more kept).
        let (newest, newest_len) = after.segments.last_key_value().map_or((0, 0), |(n, len)| (*n, *len));
        let before_len = before.segments.get(&newest).copied().unwrap_or(0);
        let appended = newest_len - before_len;
        // The bytes of the seqnums file the write changed, wherever in the file they are.
        let changed = (0..after.seqnums.len()).filter(|&i| before.seqnums.get(i) != Some(&after.seqnums[i]));
        let (lo, hi) = changed
            .fold(None, |range, i| match range {
                None => Some((i, i + 1)),
                Some((lo, _)) => Some((lo, i + 1)),
            })
            .unwrap_or((0, 0));
        let record = (hi - lo) as u64;
        let reached = match tear.in_record {
            Some(part) => appended + record * part / 1000,
            None => (appended + record) * tear.cut / 1000,
        };
        let body_kept = reached.min(appended);
        self.truncate_segment(newest, before_len + body_kept);
        if record > 0 {
            let written = usize::try_from(reached - body_kept).expect("a short record");
            let seqnums = if tear.sub_sector {
                // The new bytes reached the device up to `written`, the old ones are left after.
                let mut mixed = after.seqnums[..lo + written].to_vec();
                mixed.extend(before.seqnums.iter().skip(lo + written));
                mixed
            } else if written as u64 == record {
                after.seqnums
            } else {
                before.seqnums.clone()
            };
            self.write_seqnums(&seqnums);
        }
    }

    /// A power loss without fsync: the OS had written back everything up to `checkpoint`, and
    /// some of what followed. `kept` (thousandths) says how much of the body appended since;
    /// `new_seqnums`, whether the seqnums record's latest rewrite got there.
    /// Each segment keeps what it had at the checkpoint (none, for one made since) and that
    /// part of what was appended since; deletions since stay made.
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

    fn write_seqnums(&self, bytes: &[u8]) {
        if let Some(path) = self.path("seqnums") {
            write(&path, bytes);
        }
    }
}

fn write(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("the seqnums file writes");
}
