//! A disk store's files, as a power loss leaves them: what of a write in progress reached the
//! device, and, without fsync, how far the OS had written back.

use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

/// One session's `DiskStorage` files, in a directory holding only that session.
#[derive(Debug, Clone)]
pub struct DiskFiles {
    dir: PathBuf,
}

/// What the files held at a moment.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    seqnums: Vec<u8>,
    body_len: u64,
}

/// How a write in progress at a power loss is torn.
#[derive(Debug, Clone, Copy)]
pub struct Tear {
    /// How much of the call's writes (the body append, then the seqnums record) reached the
    /// device, in thousandths.
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

    pub fn snapshot(&self) -> Snapshot {
        let seqnums = self.path("seqnums").and_then(|p| fs::read(p).ok()).unwrap_or_default();
        let body_len = self.path("body").and_then(|p| fs::metadata(p).ok()).map_or(0, |m| m.len());
        Snapshot { seqnums, body_len }
    }

    /// The writes made since `before` were cut short by a power loss, as `tear` says.
    pub fn tear(&self, before: &Snapshot, tear: Tear) {
        let after = self.snapshot();
        let appended = after.body_len - before.body_len;
        let rewrote = before.seqnums != after.seqnums;
        let record = if rewrote { after.seqnums.len() as u64 } else { 0 };
        let reached = match tear.in_record {
            Some(part) => appended + record * part / 1000,
            None => (appended + record) * tear.cut / 1000,
        };
        let body_kept = reached.min(appended);
        self.truncate_body(before.body_len + body_kept);
        if rewrote {
            let written = usize::try_from(reached - body_kept).expect("a short record");
            let seqnums = if tear.sub_sector {
                // The new record's first bytes over the old one's.
                let mut mixed = after.seqnums[..written].to_vec();
                mixed.extend(before.seqnums.iter().skip(written));
                mixed
            } else if written == after.seqnums.len() {
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
    pub fn revert(&self, checkpoint: &Snapshot, kept: u64, new_seqnums: bool) {
        let now = self.snapshot();
        let since = now.body_len.saturating_sub(checkpoint.body_len);
        // Never longer than the file is: truncating can only lose what was written.
        self.truncate_body((checkpoint.body_len + since * kept / 1000).min(now.body_len));
        if !new_seqnums {
            self.write_seqnums(&checkpoint.seqnums);
        }
    }

    fn truncate_body(&self, len: u64) {
        if let Some(path) = self.path("body") {
            OpenOptions::new().write(true).open(&path).and_then(|f| f.set_len(len)).expect("the body file truncates");
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
