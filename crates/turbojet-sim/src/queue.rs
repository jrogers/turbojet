//! Events in time order, ties in the order they were scheduled, but for those pushed first.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use crate::time::SimTime;

pub struct Queue<E> {
    /// Each event's time, 0 for one pushed first (1 otherwise), and the order it was pushed in.
    heap: BinaryHeap<Reverse<(SimTime, u8, u64)>>,
    events: BTreeMap<u64, E>,
    next: u64,
}

impl<E> Queue<E> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new(), events: BTreeMap::new(), next: 0 }
    }

    pub fn push(&mut self, at: SimTime, event: E) {
        self.push_ranked(at, 1, event);
    }

    /// Pushes `event` ahead of every other at `at` but those pushed first before it: for events
    /// held back, which run before anything that happens at the same time, in the order held.
    pub fn push_first(&mut self, at: SimTime, event: E) {
        self.push_ranked(at, 0, event);
    }

    fn push_ranked(&mut self, at: SimTime, rank: u8, event: E) {
        self.heap.push(Reverse((at, rank, self.next)));
        self.events.insert(self.next, event);
        self.next += 1;
    }

    pub fn pop(&mut self) -> Option<(SimTime, E)> {
        let Reverse((at, _, id)) = self.heap.pop()?;
        let event = self.events.remove(&id).expect("every queued id has its event");
        Some((at, event))
    }

    pub fn peek_time(&self) -> Option<SimTime> {
        self.heap.peek().map(|Reverse((at, ..))| *at)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl<E> Default for Queue<E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_in_time_order_then_scheduling_order() {
        let mut queue = Queue::new();
        queue.push(SimTime(5), "b");
        queue.push(SimTime(1), "a");
        queue.push(SimTime(5), "c");
        let order: Vec<_> = std::iter::from_fn(|| queue.pop()).collect();
        assert_eq!(order, [(SimTime(1), "a"), (SimTime(5), "b"), (SimTime(5), "c")]);
        assert!(queue.is_empty());
    }

    #[test]
    fn events_pushed_first_go_ahead_of_others_at_their_time_in_order() {
        let mut queue = Queue::new();
        queue.push(SimTime(5), "c");
        queue.push_first(SimTime(5), "a");
        queue.push_first(SimTime(5), "b");
        queue.push(SimTime(1), "first");
        let order: Vec<_> = std::iter::from_fn(|| queue.pop()).map(|(_, e)| e).collect();
        assert_eq!(order, ["first", "a", "b", "c"]);
    }
}
