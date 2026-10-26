//! PROTOTYPE: application sends made on the caller's thread rather than queued for the
//! connection's task, saving the wake-up between them.
//!
//! While its driver waits with nothing to do (nothing unwritten, no store job, sends allowed), it
//! lends its session here. A [`SessionHandle`](crate::SessionHandle) send that finds it lent, and
//! nothing queued ahead of it, gives the message to the session, commits, and writes it to a
//! duplicate of the socket without blocking; what the socket doesn't take waits here for the
//! driver, woken to write it. Otherwise the send is queued as before. The driver takes its session
//! back whenever it wakes. Plain TCP only, for now: a transport that buffers (TLS) can't be
//! written from two places.

use std::any::Any;
use std::fmt;
use std::io::Write;
use std::net::TcpStream;

use tokio::sync::Notify;

use crate::connection::Driven;
use crate::registry::{Command, ReceiptSender};
use crate::sync::Mutex;

/// A connection's session, while its driver lends it, and where callers write.
pub(crate) struct DirectSlot<T> {
    lent: Mutex<Lent<T>>,
    /// Wakes the driver for output the socket didn't take.
    pub(crate) wake: Notify,
    /// TEMPORARY: sends made here, and those refused to be queued.
    made: std::sync::atomic::AtomicU64,
    refused: std::sync::atomic::AtomicU64,
}

struct Lent<T> {
    session: Option<Box<dyn Target<T>>>,
    /// A duplicate of the connection's socket, if it's plain TCP.
    socket: Option<TcpStream>,
    /// Output the socket didn't take, for the driver to write: while there's any, sends queue.
    spill: Vec<u8>,
    /// When the last send was made here: one soon after it is part of a burst, which queues to
    /// be written together.
    last: Option<std::time::Instant>,
    /// The driver's `session` span, entered for a send here as the driver's own sends are in it.
    span: tracing::Span,
}

/// Sends closer together than this are a burst: after the first, they queue, to be written
/// together. ponytail: fixed guess, measured only on localhost.
const BURST_GAP: std::time::Duration = std::time::Duration::from_micros(10);

/// A send that couldn't be made at once, given back to be queued.
pub(crate) type Refused<T> = (T, Option<ReceiptSender>);

impl<T> Default for DirectSlot<T> {
    fn default() -> Self {
        Self { lent: Mutex::new(Lent { session: None, socket: None, spill: Vec::new(), last: None, span: tracing::Span::none() }),
            wake: Notify::new(),
            made: Default::default(),
            refused: Default::default(),
        }
    }
}

impl<T> fmt::Debug for DirectSlot<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirectSlot").finish_non_exhaustive()
    }
}

impl<T: 'static> DirectSlot<T> {
    /// Sends `msg` on this thread if the session is lent and nothing waits to be written.
    pub(crate) fn try_send(&self, msg: T, reply: Option<ReceiptSender>) -> Result<(), Refused<T>> {
        use std::sync::atomic::Ordering::Relaxed;
        let result = self.try_send_inner(msg, reply);
        if result.is_ok() { self.made.fetch_add(1, Relaxed) } else { self.refused.fetch_add(1, Relaxed) };
        let (made, refused) = (self.made.load(Relaxed), self.refused.load(Relaxed));
        if (made + refused) % 10_000 == 0 && std::env::var_os("TURBOJET_DIRECT_STATS").is_some() {
            eprintln!("direct sends: {made} made, {refused} queued");
        }
        result
    }

    fn try_send_inner(&self, msg: T, reply: Option<ReceiptSender>) -> Result<(), Refused<T>> {
        let Ok(mut lent) = self.lent.try_lock() else { return Err((msg, reply)) };
        let Lent { session: Some(session), socket: Some(socket), spill, last, span } = &mut *lent else {
            return Err((msg, reply));
        };
        let now = std::time::Instant::now();
        if !spill.is_empty() || last.is_some_and(|last| now - last < BURST_GAP) {
            return Err((msg, reply));
        }
        span.in_scope(|| session.send_now(msg, reply, socket, spill))?;
        *last = Some(now);
        if !spill.is_empty() {
            self.wake.notify_one();
        }
        Ok(())
    }

    /// The socket callers write to: a duplicate of the connection's, or none to queue every send.
    pub(crate) fn set_socket(&self, socket: Option<TcpStream>) {
        self.lock().socket = socket;
    }

    pub(crate) fn has_socket(&self) -> bool {
        self.lock().socket.is_some()
    }

    /// Lends the driver's session to callers until [`reclaim`](Self::reclaim).
    pub(crate) fn lend<P: Driven<Item = T>>(&self, session: Box<P>) {
        let mut lent = self.lock();
        debug_assert!(lent.session.is_none() && lent.spill.is_empty());
        if lent.span.is_none() {
            lent.span = tracing::Span::current();
        }
        lent.session = Some(session);
    }

    /// Takes the session back, and adds what callers sent that the socket didn't take to `outbox`.
    pub(crate) fn reclaim<P: Driven<Item = T>>(&self, outbox: &mut Vec<u8>) -> Box<P> {
        let mut lent = self.lock();
        outbox.append(&mut lent.spill);
        let session = lent.session.take().expect("lent");
        session.into_any().downcast().expect("the driver's own session")
    }

    /// The connection has ended: callers queue, and the socket's duplicate is closed.
    pub(crate) fn close(&self) {
        if std::env::var_os("TURBOJET_DIRECT_STATS").is_some() {
            use std::sync::atomic::Ordering::Relaxed;
            eprintln!("direct sends: {} made, {} queued", self.made.load(Relaxed), self.refused.load(Relaxed));
        }
        let mut lent = self.lock();
        lent.socket = None;
        lent.session = None;
    }

    fn lock(&self) -> crate::sync::MutexGuard<'_, Lent<T>> {
        self.lent.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// What a caller needs of a lent session.
trait Target<T>: Send {
    fn send_now(
        &mut self,
        msg: T,
        reply: Option<ReceiptSender>,
        socket: &TcpStream,
        spill: &mut Vec<u8>,
    ) -> Result<(), Refused<T>>;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<P: Driven> Target<P::Item> for P {
    fn send_now(
        &mut self,
        msg: P::Item,
        reply: Option<ReceiptSender>,
        mut socket: &TcpStream,
        spill: &mut Vec<u8>,
    ) -> Result<(), Refused<P::Item>> {
        let now = std::time::Instant::now();
        if !self.has_logged_on() || self.is_closed() || !self.can_send(now) {
            return Err((msg, reply));
        }
        self.on_command(Command::Send(msg, reply), now);
        // A store that waits for its device is waited for here, on the caller's thread.
        while let Some(commit) = self.take_commit(now) {
            self.on_committed(commit.run(), now);
        }
        // As the driver's `stage_output` does. ponytail: latency histograms not recorded here.
        let output = self.output();
        if let Some(log) = self.message_log() {
            let id = self.message_log_id();
            let mut rest = output;
            while !rest.is_empty() {
                let len = P::frame_len(rest);
                log.outbound(id, &rest[..len]);
                rest = &rest[len..];
            }
        }
        if let Some(metrics) = self.metrics() {
            metrics.bytes_sent(output.len());
        }
        // An error is the driver's to find, on its next write or read.
        let written = socket.write(output).unwrap_or(0);
        spill.extend_from_slice(&output[written..]);
        self.clear_output();
        Ok(())
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
