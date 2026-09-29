//! Events from one side of a session, taken in whatever order the test asks for them.

use std::collections::VecDeque;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::{Instant, timeout_at};

/// Why [`Mailbox::expect`] returned without an item.
pub(crate) enum Missing {
    Closed,
    TimedOut,
}

/// Receives `M`s from a channel as `T`s. `expect` returns the first buffered or incoming item that
/// matches and leaves the rest buffered.
pub(crate) struct Mailbox<T, M = T> {
    rx: mpsc::UnboundedReceiver<M>,
    convert: fn(M) -> T,
    pending: VecDeque<T>,
}

impl<T> Mailbox<T> {
    pub(crate) fn new(rx: mpsc::UnboundedReceiver<T>) -> Self {
        Self::with_convert(rx, |t| t)
    }
}

impl<T, M> Mailbox<T, M> {
    /// `convert` runs on the test's task, so a panic in it fails the test.
    pub(crate) fn with_convert(rx: mpsc::UnboundedReceiver<M>, convert: fn(M) -> T) -> Self {
        Self { rx, convert, pending: VecDeque::new() }
    }

    /// The next item from the channel, ignoring the buffer.
    pub(crate) async fn recv(&mut self, deadline: Instant) -> Result<T, Missing> {
        match timeout_at(deadline, self.rx.recv()).await {
            Ok(Some(m)) => Ok((self.convert)(m)),
            Ok(None) => Err(Missing::Closed),
            Err(_) => Err(Missing::TimedOut),
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        self.pending.push_back(item);
    }

    pub(crate) async fn expect(&mut self, deadline: Instant, mut pred: impl FnMut(&T) -> bool) -> Result<T, Missing> {
        if let Some(i) = self.pending.iter().position(&mut pred) {
            return Ok(self.pending.remove(i).unwrap());
        }
        loop {
            let item = self.recv(deadline).await?;
            if pred(&item) {
                return Ok(item);
            }
            self.pending.push_back(item);
        }
    }

    /// The first item matching `pred` that is buffered or arrives `within`, if any. Takes nothing
    /// out of the buffer.
    pub(crate) async fn find_within(&mut self, within: Duration, mut pred: impl FnMut(&T) -> bool) -> Option<&T> {
        let deadline = Instant::now() + within;
        // Take what has already arrived first. `try_recv` isn't subject to tokio's coop budget, so
        // this doesn't rely on how a `recv` under an elapsed deadline behaves once the budget is
        // spent (it could otherwise time out with items still queued).
        self.drain();
        let mut i = self.pending.iter().position(&mut pred);
        while i.is_none() && !within.is_zero() {
            let Ok(item) = self.recv(deadline).await else { break };
            if pred(&item) {
                i = Some(self.pending.len());
            }
            self.pending.push_back(item);
        }
        i.map(|i| &self.pending[i])
    }

    /// Everything buffered, after taking what has already arrived.
    pub(crate) fn drain(&mut self) -> &VecDeque<T> {
        while let Ok(m) = self.rx.try_recv() {
            let item = (self.convert)(m);
            self.pending.push_back(item);
        }
        &self.pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// More items than tokio's coop budget (128): a zero-wait search must still see the last.
    #[tokio::test]
    async fn find_within_zero_sees_everything_queued() {
        let (tx, rx) = mpsc::unbounded_channel();
        for i in 0..1000 {
            tx.send(i).unwrap();
        }
        let mut mailbox = Mailbox::new(rx);
        assert_eq!(mailbox.find_within(Duration::ZERO, |&i| i == 999).await, Some(&999));
        assert_eq!(mailbox.find_within(Duration::ZERO, |&i| i == 1000).await, None);
        assert_eq!(mailbox.drain().len(), 1000);
    }
}
