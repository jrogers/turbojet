//! An index of stored messages by MsgSeqNum, kept in order in a ring buffer.

use std::collections::VecDeque;

/// Values by sequence number, in increasing order. Stores number what they keep in order, so an
/// insert is almost always the newest and goes on the back, and eviction takes the oldest off the
/// front: neither searches, nor allocates once the buffer has grown, as a `BTreeMap`'s nodes do.
/// Anything else finds its place by binary search.
#[derive(Debug)]
pub(crate) struct SeqIndex<T>(VecDeque<(u64, T)>);

impl<T> Default for SeqIndex<T> {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

impl<T> SeqIndex<T> {
    /// Adds `value` at `seq`, returning the value it replaced.
    pub(crate) fn insert(&mut self, seq: u64, value: T) -> Option<T> {
        if self.0.back().is_none_or(|(last, _)| *last < seq) {
            self.0.push_back((seq, value));
            return None;
        }
        let at = self.0.partition_point(|(s, _)| *s < seq);
        if self.0[at].0 == seq {
            return Some(std::mem::replace(&mut self.0[at].1, value));
        }
        self.0.insert(at, (seq, value));
        None
    }

    pub(crate) fn first(&self) -> Option<u64> {
        self.0.front().map(|(seq, _)| *seq)
    }

    pub(crate) fn last(&self) -> Option<u64> {
        self.0.back().map(|(seq, _)| *seq)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn pop_first(&mut self) -> Option<(u64, T)> {
        self.0.pop_front()
    }

    /// Removes every entry up to and including `seq`.
    pub(crate) fn remove_through(&mut self, seq: u64) {
        let end = self.0.partition_point(|(s, _)| *s <= seq);
        self.0.drain(..end);
    }

    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    /// The entries from `begin` to `end`, both included, in order.
    pub(crate) fn range(&self, begin: u64, end: u64) -> impl Iterator<Item = &(u64, T)> {
        let from = self.0.partition_point(|(s, _)| *s < begin);
        self.0.range(from..).take_while(move |(s, _)| *s <= end)
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
}
