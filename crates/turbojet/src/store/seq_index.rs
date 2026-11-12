//! An index of stored messages by MsgSeqNum, kept in order in blocks of ring buffers.

use std::collections::VecDeque;

/// Entries a block is allocated for. A block never moves once allocated, so growing the index
/// by a block copies none of the entries already in it.
const BLOCK: usize = 4096;

/// Values by sequence number, in increasing order. Stores number what they keep in order, so an
/// insert is almost always the newest and goes on the back, and eviction takes the oldest off the
/// front: neither searches, nor allocates but for a new block every `BLOCK` entries. Anything else
/// finds its place by binary search, over the blocks and then within one.
///
/// In blocks rather than one ring buffer because a ring buffer that doubles copies everything it
/// holds: with a few million messages in a session, tens of megabytes, a pause of milliseconds on
/// an allocator whose `realloc` copies (jemalloc, mimalloc; glibc remaps large blocks instead).
#[derive(Debug)]
pub(crate) struct SeqIndex<T>(VecDeque<VecDeque<(u64, T)>>);

impl<T> Default for SeqIndex<T> {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

impl<T> SeqIndex<T> {
    /// Adds `value` at `seq`, returning the value it replaced.
    pub(crate) fn insert(&mut self, seq: u64, value: T) -> Option<T> {
        if self.last().is_none_or(|last| last < seq) {
            match self.0.back_mut() {
                Some(block) if block.len() < BLOCK => block.push_back((seq, value)),
                _ => {
                    let mut block = VecDeque::with_capacity(BLOCK);
                    block.push_back((seq, value));
                    self.0.push_back(block);
                }
            }
            return None;
        }
        // The first block that reaches `seq`; there is one, since the last entry is past it.
        // ponytail: out of order, a block grows past BLOCK and copies itself alone when it does;
        // only recovery inserts out of order. Split blocks if that ever matters.
        let reaching = self.0.partition_point(|b| b.back().is_some_and(|(s, _)| *s < seq));
        let block = &mut self.0[reaching];
        let at = block.partition_point(|(s, _)| *s < seq);
        if block[at].0 == seq {
            return Some(std::mem::replace(&mut block[at].1, value));
        }
        block.insert(at, (seq, value));
        None
    }

    pub(crate) fn first(&self) -> Option<u64> {
        self.0.front().and_then(VecDeque::front).map(|(seq, _)| *seq)
    }

    pub(crate) fn last(&self) -> Option<u64> {
        self.0.back().and_then(VecDeque::back).map(|(seq, _)| *seq)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn pop_first(&mut self) -> Option<(u64, T)> {
        let block = self.0.front_mut()?;
        let first = block.pop_front();
        if block.is_empty() {
            self.0.pop_front();
        }
        first
    }

    /// Removes every entry up to and including `seq`.
    pub(crate) fn remove_through(&mut self, seq: u64) {
        while let Some(block) = self.0.front_mut() {
            if block.back().is_some_and(|(s, _)| *s <= seq) {
                self.0.pop_front();
                continue;
            }
            let end = block.partition_point(|(s, _)| *s <= seq);
            block.drain(..end);
            return;
        }
    }

    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    /// The entries from `begin` to `end`, both included, in order.
    pub(crate) fn range(&self, begin: u64, end: u64) -> impl Iterator<Item = &(u64, T)> {
        let first = self.0.partition_point(|b| b.back().is_some_and(|(s, _)| *s < begin));
        let from = self.0.get(first).map_or(0, |b| b.partition_point(|(s, _)| *s < begin));
        self.0
            .range(first..)
            .enumerate()
            .flat_map(move |(i, b)| b.range(if i == 0 { from } else { 0 }..))
            .take_while(move |(s, _)| *s <= end)
    }
}

impl<T> FromIterator<(u64, T)> for SeqIndex<T> {
    /// Entries in any order; a later one replaces an earlier one at the same number.
    fn from_iter<I: IntoIterator<Item = (u64, T)>>(entries: I) -> Self {
        let mut index = Self::default();
        for (seq, value) in entries {
            index.insert(seq, value);
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_entries_in_order_whatever_order_they_arrive() {
        let mut index: SeqIndex<char> = [(5, 'a'), (7, 'b'), (2, 'c'), (6, 'd')].into_iter().collect();
        assert_eq!(index.insert(7, 'e'), Some('b'));
        assert_eq!(index.insert(9, 'f'), None);
        let all: Vec<_> = index.range(0, u64::MAX).copied().collect();
        assert_eq!(all, [(2, 'c'), (5, 'a'), (6, 'd'), (7, 'e'), (9, 'f')]);
        assert_eq!(index.range(3, 7).map(|(s, _)| *s).collect::<Vec<_>>(), [5, 6, 7]);
        assert_eq!(index.range(8, 8).count(), 0);
        index.remove_through(6);
        assert_eq!((index.first(), index.last()), (Some(7), Some(9)));
        assert_eq!(index.pop_first(), Some((7, 'e')));
        index.clear();
        assert!(index.is_empty());
    }

    /// Across block edges, every operation agrees with a sorted list of the same entries: in
    /// order, with gaps, with entries out of order and replaced, and evicted from the front.
    #[test]
    fn agrees_with_a_sorted_list_across_blocks() {
        let mut index = SeqIndex::default();
        let mut list: Vec<(u64, u64)> = Vec::new();
        let put = |index: &mut SeqIndex<u64>, list: &mut Vec<(u64, u64)>, seq: u64, value: u64| {
            let replaced = match list.binary_search_by_key(&seq, |(s, _)| *s) {
                Ok(at) => Some(std::mem::replace(&mut list[at].1, value)),
                Err(at) => {
                    list.insert(at, (seq, value));
                    None
                }
            };
            assert_eq!(index.insert(seq, value), replaced, "insert {seq}");
        };
        // In order with every third number missing, three blocks and a bit.
        for seq in (1..=4 * BLOCK as u64).filter(|s| s % 3 != 0) {
            put(&mut index, &mut list, seq, seq);
        }
        // Into the gaps, in the first block, on a block's edge and in the last, and replacing.
        for seq in [3, 6, BLOCK as u64 * 3 / 2, 2 * BLOCK as u64 + 1, 12_000, 7, 8] {
            put(&mut index, &mut list, seq, seq + 100_000);
        }
        let check = |index: &SeqIndex<u64>, list: &[(u64, u64)]| {
            assert_eq!(index.first(), list.first().map(|(s, _)| *s));
            assert_eq!(index.last(), list.last().map(|(s, _)| *s));
            assert_eq!(index.is_empty(), list.is_empty());
            for (begin, end) in [(0, u64::MAX), (1, 1), (5, 9), (4000, 4200), (6000, 9000), (12_000, 12_288)] {
                let got: Vec<_> = index.range(begin, end).copied().collect();
                let want: Vec<_> = list.iter().filter(|(s, _)| (begin..=end).contains(s)).copied().collect();
                assert_eq!(got, want, "range {begin}..={end}");
            }
        };
        check(&index, &list);
        // Evicted inside a block, then through a block's edge, then one at a time.
        for through in [10, BLOCK as u64 + 5, 2 * BLOCK as u64] {
            index.remove_through(through);
            list.retain(|(s, _)| *s > through);
            check(&index, &list);
        }
        while let Some(first) = index.pop_first() {
            assert_eq!(first, list.remove(0));
        }
        check(&index, &list);
    }
}
