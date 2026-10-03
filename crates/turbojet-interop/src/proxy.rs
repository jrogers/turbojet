//! A TCP proxy between Turbojet and QuickFIX/J that injects faults a peer can't produce on request:
//! lost, garbled, truncated and delayed messages, slow links, back-pressure and a link that goes
//! dead.
//!
//! The initiator, whichever engine it is, connects to the proxy, and the proxy dials the acceptor
//! (`upstream`). It handles one connection pair at a time: a reconnect waits in the listen backlog
//! until the previous pair has ended, then gets a fresh pair with no faults.
//!
//! In each direction the proxy reads whole FIX frames (`8=...|9=len|...|10=xxx|`) and forwards
//! them, so a fault applies to one message by its MsgType. Bytes it can't frame are forwarded as
//! they are, up to the next `8=FIX`, so garbage never grows its buffers.
//!
//! # Faults
//!
//! Faults are set per [`Dir`]. All of them clear when the connection pair ends, so set a fault for
//! the next connection only after [`Proxy::disconnected`] (or before the first connection).
//!
//! - [`Proxy::blackhole`]: everything in that direction is read and discarded, so the sender's
//!   writes don't block, as on a dead link. It lasts until the pair ends or [`Proxy::heal`].
//! - [`Proxy::drop_next`], [`Proxy::garble_next`], [`Proxy::cut_mid`]: one-shot, each applies to
//!   the next frame of its MsgType in its direction. Garbling keeps the frame's length and makes its
//!   CheckSum wrong. Cutting forwards the first half of the frame and then closes both sides.
//! - [`Proxy::delay_next`]: one-shot, holds the next frame of its MsgType for a while. What comes
//!   after it queues behind it, as on a TCP connection.
//! - [`Proxy::hold`] and [`Proxy::release`]: keeps reading, but holds everything in that direction
//!   (up to [`MAX_HELD`] bytes) until released, then forwards it in order.
//! - [`Proxy::bandwidth`]: paces forwarding to a byte rate.
//! - [`Proxy::stall`] and [`Proxy::unstall`]: stops reading from the sender (after the read in
//!   progress), so its writes back up into TCP and block, then reads again.
//!
//! Each direction runs as a reader, which frames what arrives and applies the faults on single
//! frames, and a writer, which applies holds, delays and pacing. Between them is a queue of at most
//! [`QUEUE_PIECES`] pieces, so a writer that is held up stops the reader in turn, and the sender
//! feels back-pressure, as it would from a slow network.
//!
//! # Closes
//!
//! Without a blackhole, a close from either side closes the other, after forwarding whatever
//! arrived before it.
//!
//! While either direction is blackholed, a close from either side is withheld: the proxy keeps the
//! other side's socket open and keeps reading (and discarding) from it, so that engine has to detect
//! the dead link on its own heartbeat timeout, not from a TCP close. The pair ends when:
//!
//! - the other side closes too;
//! - the test calls [`Proxy::heal`], which clears the blackhole and so passes the withheld close on;
//! - or the `Proxy` is dropped.
//!
//! [`ProxyEvent::Ended`] reports each side's close and whether it was passed on, and
//! [`ProxyEvent::Disconnected`] the end of the pair.

use std::collections::VecDeque;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::Instant;

use crate::mailbox::{Mailbox, Missing};
use crate::{EVENT_TIMEOUT, Role};

const SOH: u8 = 0x01;
/// The trailer `10=xxx<SOH>`.
const TRAILER_LEN: usize = 7;
/// The longest frame held back whole: Turbojet's own limit on BodyLength (64 KiB), plus room for
/// the header and trailer. A frame claiming to be longer is forwarded as unframed bytes.
const MAX_FRAME: usize = 64 * 1024 + 64;
/// Where the header `8=FIXT.1.1<SOH>9=nnnnn<SOH>` must have ended; 32 leaves room for both.
const MAX_HEADER: usize = 32;
/// How much is read from a socket at a time. With [`MAX_FRAME`], it bounds a direction's buffer.
const READ_CHUNK: usize = 16 * 1024;
/// One-shot faults waiting in one direction. A scenario sets one or two.
const MAX_PENDING: usize = 4;
/// Frames (or unframed chunks) queued between a direction's reader and its writer.
const QUEUE_PIECES: usize = 64;
/// The most a hold may keep back in one direction. A hold covers a latency spike of a second or
/// two of session traffic, a few KiB; more means a scenario is holding a bulk transfer.
const MAX_HELD: usize = 1024 * 1024;
/// How many steps a second of bandwidth pacing takes. Each step writes 1/50 of the rate (400 bytes
/// at 20 KB/s), which keeps the pacing smooth without a timer per byte.
const PACE_STEPS: u32 = 50;

/// The direction of travel, by the engine that receives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Turbojet to QuickFIX/J.
    ToPeer,
    /// QuickFIX/J to Turbojet.
    ToTj,
}

impl Dir {
    fn index(self) -> usize {
        match self {
            Self::ToPeer => 0,
            Self::ToTj => 1,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Self::ToPeer => Self::ToTj,
            Self::ToTj => Self::ToPeer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    Blackhole,
    Drop,
    Garble,
    Cut,
    /// Reported once per frame held back.
    Hold,
    Delay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyEvent {
    /// The proxy accepted a connection and dialed the acceptor: a new pair, with no faults.
    Connected,
    /// A fault applied to a frame, shown with `|` for SOH (as it was before garbling or cutting).
    /// A blackhole reports every frame it swallows.
    Applied { dir: Dir, fault: Fault, msg_type: String, frame: String },
    /// The sender in `dir` closed its connection, and the proxy passed the close on to the
    /// receiver, or withheld it because of a blackhole.
    Ended { dir: Dir, passed_on: bool },
    /// Both sides are closed; the next connection starts afresh.
    Disconnected,
}

/// The faults set on the current connection pair.
#[derive(Debug, Default)]
struct Faults {
    /// By [`Dir::index`].
    blackhole: [bool; 2],
    /// One-shot faults, oldest first, by [`Dir::index`].
    pending: [Vec<OneShot>; 2],
    hold: [bool; 2],
    /// Bytes per second.
    bandwidth: [Option<u32>; 2],
    stall: [bool; 2],
}

/// A fault on the next frame of a MsgType.
#[derive(Debug)]
struct OneShot {
    fault: Fault,
    msg_type: String,
    /// For [`Fault::Delay`].
    delay: Duration,
}

impl Faults {
    fn blackholed(&self) -> bool {
        self.blackhole.iter().any(|&b| b)
    }
}

/// The proxy, listening on 127.0.0.1. Dropping it stops it and closes its connections.
pub struct Proxy {
    addr: SocketAddr,
    /// Changes wake a connection pair that is withholding a close, to check the blackhole again.
    faults: Arc<watch::Sender<Faults>>,
    events: Mailbox<ProxyEvent>,
    task: JoinHandle<()>,
}

impl Proxy {
    /// Listens on a free port, and for each connection dials `upstream`, the acceptor. `role` says
    /// which end is Turbojet, so the proxy knows which way [`Dir::ToPeer`] runs.
    pub async fn start(upstream: SocketAddr, role: Role) -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        // The initiator connects to the proxy; the acceptor is upstream.
        let downstream_to_upstream = match role {
            Role::TjInitiator => Dir::ToPeer,
            Role::TjAcceptor => Dir::ToTj,
        };
        let faults = Arc::new(watch::Sender::new(Faults::default()));
        let (events, rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(serve(listener, upstream, downstream_to_upstream, faults.clone(), events));
        Self { addr, faults, events: Mailbox::new(rx), task }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    /// Discards everything in `dir` and withholds closes, until the pair ends or [`Proxy::heal`].
    pub fn blackhole(&self, dir: Dir) {
        self.faults.send_modify(|f| f.blackhole[dir.index()] = true);
    }

    /// Swallows the next `msg_type` frame in `dir`.
    pub fn drop_next(&self, dir: Dir, msg_type: &str) {
        self.once(dir, Fault::Drop, msg_type, Duration::ZERO);
    }

    /// Corrupts the CheckSum of the next `msg_type` frame in `dir`, keeping its length.
    pub fn garble_next(&self, dir: Dir, msg_type: &str) {
        self.once(dir, Fault::Garble, msg_type, Duration::ZERO);
    }

    /// Forwards the first half of the next `msg_type` frame in `dir`, then closes both sides.
    pub fn cut_mid(&self, dir: Dir, msg_type: &str) {
        self.once(dir, Fault::Cut, msg_type, Duration::ZERO);
    }

    /// Holds the next `msg_type` frame in `dir` for `delay` from when it arrives. What follows it
    /// queues behind it.
    pub fn delay_next(&self, dir: Dir, msg_type: &str, delay: Duration) {
        self.once(dir, Fault::Delay, msg_type, delay);
    }

    fn once(&self, dir: Dir, fault: Fault, msg_type: &str, delay: Duration) {
        self.faults.send_modify(|f| {
            let pending = &mut f.pending[dir.index()];
            assert!(pending.len() < MAX_PENDING, "more than {MAX_PENDING} one-shot faults pending {dir:?}");
            pending.push(OneShot { fault, msg_type: msg_type.to_string(), delay });
        });
    }

    /// Holds back everything in `dir`, still reading it, until [`Proxy::release`].
    pub fn hold(&self, dir: Dir) {
        self.faults.send_modify(|f| f.hold[dir.index()] = true);
    }

    /// Forwards what `dir` held, in order, and stops holding.
    pub fn release(&self, dir: Dir) {
        self.faults.send_modify(|f| f.hold[dir.index()] = false);
    }

    /// Paces forwarding in `dir` to `bytes_per_sec`.
    pub fn bandwidth(&self, dir: Dir, bytes_per_sec: u32) {
        assert!(bytes_per_sec >= PACE_STEPS, "a rate under {PACE_STEPS} bytes/s can't be paced in steps");
        self.faults.send_modify(|f| f.bandwidth[dir.index()] = Some(bytes_per_sec));
    }

    /// Stops reading from the sender in `dir`, once the read in progress completes.
    pub fn stall(&self, dir: Dir) {
        self.faults.send_modify(|f| f.stall[dir.index()] = true);
    }

    /// Reads from the sender in `dir` again.
    pub fn unstall(&self, dir: Dir) {
        self.faults.send_modify(|f| f.stall[dir.index()] = false);
    }

    /// Clears every fault on the current pair. A close withheld by a blackhole is passed on.
    pub fn heal(&self) {
        self.faults.send_replace(Faults::default());
    }

    /// The first event matching `pred`, buffered or yet to come; others stay buffered.
    pub async fn expect(&mut self, what: &str, pred: impl FnMut(&ProxyEvent) -> bool) -> ProxyEvent {
        match self.events.expect(Instant::now() + EVENT_TIMEOUT, pred).await {
            Ok(event) => event,
            Err(Missing::Closed) => panic!("the proxy stopped while waiting for {what}"),
            Err(Missing::TimedOut) => panic!("timed out waiting for proxy {what}"),
        }
    }

    /// Fails if an event matching `pred` is buffered or arrives `within`.
    pub async fn expect_none(&mut self, what: &str, pred: impl FnMut(&ProxyEvent) -> bool, within: Duration) {
        if let Some(event) = self.events.find_within(within, pred).await {
            panic!("expected no proxy {what}, got {event:?}");
        }
    }

    pub async fn connected(&mut self) {
        self.expect("connection", |e| matches!(e, ProxyEvent::Connected)).await;
    }

    pub async fn disconnected(&mut self) {
        self.expect("disconnection", |e| matches!(e, ProxyEvent::Disconnected)).await;
    }

    /// Waits for `fault` to apply to a `msg_type` frame in `dir`, and returns the frame, `|` for SOH.
    pub async fn applied(&mut self, dir: Dir, fault: Fault, msg_type: &str) -> String {
        let what = format!("{fault:?} of 35={msg_type} {dir:?}");
        let pred = |e: &ProxyEvent| {
            matches!(e, ProxyEvent::Applied { dir: d, fault: f, msg_type: t, .. }
                if *d == dir && *f == fault && t == msg_type)
        };
        match self.expect(&what, pred).await {
            ProxyEvent::Applied { frame, .. } => frame,
            _ => unreachable!(),
        }
    }

    /// Waits for the sender in `dir` to close; returns whether the close was passed on.
    pub async fn ended(&mut self, dir: Dir) -> bool {
        let what = format!("close {dir:?}");
        match self.expect(&what, |e| matches!(e, ProxyEvent::Ended { dir: d, .. } if *d == dir)).await {
            ProxyEvent::Ended { passed_on, .. } => passed_on,
            _ => unreachable!(),
        }
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        // The pair runs on this task, so this closes its sockets too.
        self.task.abort();
        if std::thread::panicking() {
            eprintln!("---- unconsumed proxy events ----");
            for event in self.events.drain() {
                eprintln!("{event:?}");
            }
        }
    }
}

/// Accepts connections one at a time and links each to a new connection to `upstream`.
async fn serve(
    listener: TcpListener,
    upstream: SocketAddr,
    downstream_to_upstream: Dir,
    faults: Arc<watch::Sender<Faults>>,
    events: mpsc::UnboundedSender<ProxyEvent>,
) {
    loop {
        let downstream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(e) => {
                eprintln!("proxy: accept failed, stopping: {e}");
                return;
            }
        };
        let upstream = match TcpStream::connect(upstream).await {
            Ok(stream) => stream,
            Err(e) => {
                // Dropping `downstream` closes it, as if the acceptor had refused it.
                eprintln!("proxy: could not reach {upstream}: {e}");
                continue;
            }
        };
        for stream in [&downstream, &upstream] {
            stream.set_nodelay(true).unwrap();
        }
        let _ = events.send(ProxyEvent::Connected);
        link(downstream, upstream, downstream_to_upstream, &faults, &events).await;
        faults.send_replace(Faults::default());
        let _ = events.send(ProxyEvent::Disconnected);
    }
}

/// How a direction stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum End {
    /// Its sender closed.
    Closed,
    /// A [`Fault::Cut`] applied.
    Cut,
}

/// Forwards both ways until the pair ends (see the module docs), then closes both sockets.
async fn link(
    mut downstream: TcpStream,
    mut upstream: TcpStream,
    downstream_to_upstream: Dir,
    faults: &watch::Sender<Faults>,
    events: &mpsc::UnboundedSender<ProxyEvent>,
) {
    // Borrowed halves: a direction that stops must not shut down the other socket's write side,
    // which would pass on a close that a blackhole withholds.
    let (mut down_read, mut down_write) = downstream.split();
    let (mut up_read, mut up_write) = upstream.split();
    let forward = pump(downstream_to_upstream, &mut down_read, &mut up_write, faults, events);
    let backward = pump(downstream_to_upstream.reverse(), &mut up_read, &mut down_write, faults, events);
    tokio::pin!(forward, backward);
    let mut changes = faults.subscribe();
    let mut ended = [false; 2];
    loop {
        let (dir, end) = tokio::select! {
            end = &mut forward, if !ended[downstream_to_upstream.index()] => (downstream_to_upstream, end),
            end = &mut backward, if !ended[downstream_to_upstream.reverse().index()] => {
                (downstream_to_upstream.reverse(), end)
            }
            Ok(()) = changes.changed() => {
                // A withheld close goes through once nothing is blackholed.
                if ended.iter().any(|&e| e) && !changes.borrow_and_update().blackholed() {
                    return;
                }
                continue;
            }
        };
        if end == End::Cut {
            return;
        }
        ended[dir.index()] = true;
        let passed_on = ended.iter().all(|&e| e) || !faults.borrow().blackholed();
        let _ = events.send(ProxyEvent::Ended { dir, passed_on });
        if passed_on {
            return;
        }
    }
}

/// Forwards frames from `src` to `dst` in `dir`, applying faults, until `src` closes and what it
/// sent has been forwarded, or a cut.
async fn pump(
    dir: Dir,
    src: &mut ReadHalf<'_>,
    dst: &mut WriteHalf<'_>,
    faults: &watch::Sender<Faults>,
    events: &mpsc::UnboundedSender<ProxyEvent>,
) -> End {
    let (tx, rx) = mpsc::channel(QUEUE_PIECES);
    let reader = read_pieces(dir, src, tx, faults, events);
    let writer = write_pieces(dir, dst, rx, faults, events);
    tokio::pin!(reader, writer);
    let mut reading = true;
    loop {
        tokio::select! {
            () = &mut reader, if reading => reading = false,
            end = &mut writer => return end,
        }
    }
}

/// What a direction's reader hands its writer: a frame, or bytes that aren't one.
#[derive(Debug)]
struct Piece {
    bytes: Vec<u8>,
    framed: bool,
    /// Not to be forwarded before this, for [`Fault::Delay`].
    not_before: Option<Instant>,
    /// Close both sides once this is forwarded.
    cut: bool,
}

/// Reads and frames what the sender in `dir` sends, applies the faults on single frames, and
/// queues the rest for the writer, until `src` closes (which closes the queue) or a cut.
async fn read_pieces(
    dir: Dir,
    src: &mut ReadHalf<'_>,
    tx: mpsc::Sender<Piece>,
    faults: &watch::Sender<Faults>,
    events: &mpsc::UnboundedSender<ProxyEvent>,
) {
    let mut changes = faults.subscribe();
    let mut buffer = Vec::with_capacity(MAX_FRAME + READ_CHUNK);
    let mut chunk = vec![0; READ_CHUNK];
    loop {
        while changes.borrow_and_update().stall[dir.index()] {
            if changes.changed().await.is_err() {
                return;
            }
        }
        let size = match src.read(&mut chunk).await {
            Ok(0) | Err(_) => 0,
            Ok(size) => size,
        };
        let closed = size == 0;
        buffer.extend_from_slice(&chunk[..size]);
        debug_assert!(buffer.len() <= MAX_FRAME + READ_CHUNK);
        loop {
            let (len, framed) = match parse(&buffer) {
                Parse::Frame(len) => (len, true),
                Parse::Unframed(len) => (len, false),
                // A partial frame at the close goes out as it is.
                Parse::Incomplete if closed && !buffer.is_empty() => (buffer.len(), false),
                Parse::Incomplete => break,
            };
            let piece = apply(dir, &buffer[..len], framed, faults, events);
            buffer.drain(..len);
            let Some(piece) = piece else { continue };
            let cut = piece.cut;
            if tx.send(piece).await.is_err() || cut {
                return;
            }
        }
        debug_assert!(buffer.len() < MAX_FRAME);
        if closed {
            return;
        }
    }
}

/// The piece to forward for `bytes`, after the faults on single frames; `None` if it is lost.
fn apply(
    dir: Dir,
    bytes: &[u8],
    framed: bool,
    faults: &watch::Sender<Faults>,
    events: &mpsc::UnboundedSender<ProxyEvent>,
) -> Option<Piece> {
    let one_shot = if framed { take_fault(faults, dir, bytes) } else { unframed_fault(faults, dir) };
    if let Some((fault, _)) = one_shot.as_ref().filter(|_| framed) {
        let _ = events.send(applied(dir, *fault, bytes));
    }
    let mut piece = Piece { bytes: Vec::new(), framed, not_before: None, cut: false };
    match one_shot {
        None => piece.bytes = bytes.to_vec(),
        Some((Fault::Blackhole | Fault::Drop, _)) => return None,
        Some((Fault::Garble, _)) => piece.bytes = garble(bytes),
        Some((Fault::Cut, _)) => {
            piece.bytes = bytes[..bytes.len() / 2].to_vec();
            piece.cut = true;
        }
        Some((Fault::Delay, delay)) => {
            piece.bytes = bytes.to_vec();
            piece.not_before = Some(Instant::now() + delay);
        }
        Some((Fault::Hold, _)) => unreachable!("a hold isn't a one-shot fault"),
    }
    Some(piece)
}

fn applied(dir: Dir, fault: Fault, frame: &[u8]) -> ProxyEvent {
    let msg_type = String::from_utf8_lossy(msg_type(frame)).into_owned();
    let frame = String::from_utf8_lossy(frame).replace('\u{1}', "|");
    ProxyEvent::Applied { dir, fault, msg_type, frame }
}

/// Forwards the reader's pieces to `dst`, applying holds, delays and pacing, until the queue
/// closes and is empty (`End::Closed`) or a cut. Once `dst` fails, what arrives is discarded: its
/// engine has gone, and the other direction will see the close.
async fn write_pieces(
    dir: Dir,
    dst: &mut WriteHalf<'_>,
    mut rx: mpsc::Receiver<Piece>,
    faults: &watch::Sender<Faults>,
    events: &mpsc::UnboundedSender<ProxyEvent>,
) -> End {
    let mut changes = faults.subscribe();
    let mut held: VecDeque<Piece> = VecDeque::new();
    let mut held_len = 0;
    let mut queue_open = true;
    let mut dst_open = true;
    loop {
        let hold = changes.borrow_and_update().hold[dir.index()];
        if !hold && let Some(piece) = held.pop_front() {
            held_len -= piece.bytes.len();
            if forward(dir, dst, &piece, &mut dst_open, faults).await == Some(End::Cut) {
                return End::Cut;
            }
            continue;
        }
        if !queue_open && held.is_empty() {
            return End::Closed;
        }
        tokio::select! {
            piece = rx.recv(), if queue_open => match piece {
                None => queue_open = false,
                Some(piece) if hold => {
                    if piece.framed {
                        let _ = events.send(applied(dir, Fault::Hold, &piece.bytes));
                    }
                    held_len += piece.bytes.len();
                    assert!(held_len <= MAX_HELD, "held more than {MAX_HELD} bytes {dir:?}");
                    held.push_back(piece);
                }
                Some(piece) => {
                    if forward(dir, dst, &piece, &mut dst_open, faults).await == Some(End::Cut) {
                        return End::Cut;
                    }
                }
            },
            Ok(()) = changes.changed() => {}
        }
    }
}

/// Writes `piece` to `dst` once its delay is over, at the bandwidth set for `dir`. Returns
/// `Some(End::Cut)` if it ends the pair.
async fn forward(
    dir: Dir,
    dst: &mut WriteHalf<'_>,
    piece: &Piece,
    dst_open: &mut bool,
    faults: &watch::Sender<Faults>,
) -> Option<End> {
    if let Some(not_before) = piece.not_before {
        tokio::time::sleep_until(not_before).await;
    }
    let mut rest = &piece.bytes[..];
    while !rest.is_empty() {
        let rate = faults.borrow().bandwidth[dir.index()];
        let step = rate.map_or(rest.len(), |rate| rest.len().min(usize::try_from(rate / PACE_STEPS).unwrap()));
        if *dst_open && dst.write_all(&rest[..step]).await.is_err() {
            *dst_open = false;
        }
        if let Some(rate) = rate {
            let step = u64::try_from(step).unwrap();
            tokio::time::sleep(Duration::from_micros(step * 1_000_000 / u64::from(rate))).await;
        }
        rest = &rest[step..];
    }
    piece.cut.then_some(End::Cut)
}

/// The one-shot fault that applies to `frame` in `dir`, if any, and its delay. Takes it out of the
/// pending ones; a blackhole comes first and leaves them pending.
fn take_fault(faults: &watch::Sender<Faults>, dir: Dir, frame: &[u8]) -> Option<(Fault, Duration)> {
    let msg_type = msg_type(frame);
    let mut fault = None;
    // Not a change anyone waits for, so no notification.
    faults.send_if_modified(|f| {
        if f.blackhole[dir.index()] {
            fault = Some((Fault::Blackhole, Duration::ZERO));
        } else {
            let pending = &mut f.pending[dir.index()];
            if let Some(i) = pending.iter().position(|o| o.msg_type.as_bytes() == msg_type) {
                let one_shot = pending.remove(i);
                fault = Some((one_shot.fault, one_shot.delay));
            }
        }
        false
    });
    fault
}

/// Bytes that aren't a frame are only ever blackholed.
fn unframed_fault(faults: &watch::Sender<Faults>, dir: Dir) -> Option<(Fault, Duration)> {
    faults.borrow().blackhole[dir.index()].then_some((Fault::Blackhole, Duration::ZERO))
}

#[derive(Debug, PartialEq, Eq)]
enum Parse {
    /// A frame of this length starts the buffer.
    Frame(usize),
    /// This many bytes at the start aren't a frame.
    Unframed(usize),
    /// The start of a frame, or nothing.
    Incomplete,
}

/// What starts `buffer`. A frame is `8=<x><SOH>9=<digits><SOH>`, BodyLength bytes, then
/// `10=<3 bytes><SOH>`; the CheckSum's value isn't checked, so a garbled frame is still a frame.
fn parse(buffer: &[u8]) -> Parse {
    let unframed = || Parse::Unframed(next_begin(buffer));
    let begin_string_end = match buffer.iter().position(|&b| b == SOH) {
        Some(i) => i,
        None if buffer.len() < MAX_HEADER && could_start(buffer, b"8=FIX") => return Parse::Incomplete,
        None => return unframed(),
    };
    if !buffer.starts_with(b"8=") {
        return unframed();
    }
    let rest = &buffer[begin_string_end + 1..];
    let Some(digits_len) = rest.iter().position(|&b| b == SOH) else {
        return if buffer.len() < MAX_HEADER && could_start(rest, b"9=") { Parse::Incomplete } else { unframed() };
    };
    let Some(digits) = rest[..digits_len].strip_prefix(b"9=") else { return unframed() };
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return unframed();
    }
    let body_len = match std::str::from_utf8(digits).ok().and_then(|d| d.parse::<usize>().ok()) {
        Some(len) if len <= MAX_FRAME => len,
        _ => return unframed(),
    };
    let body_start = begin_string_end + 1 + digits_len + 1;
    let end = body_start + body_len + TRAILER_LEN;
    if body_start > MAX_HEADER || end > MAX_FRAME {
        return unframed();
    }
    if buffer.len() < end {
        return Parse::Incomplete;
    }
    let trailer = &buffer[end - TRAILER_LEN..end];
    if !trailer.starts_with(b"10=") || trailer[TRAILER_LEN - 1] != SOH {
        return unframed();
    }
    Parse::Frame(end)
}

/// Whether `bytes` could grow into something starting with `start`.
fn could_start(bytes: &[u8], start: &[u8]) -> bool {
    if bytes.len() <= start.len() { start.starts_with(bytes) } else { bytes.starts_with(start) }
}

/// The length of the unframed bytes at the start of `buffer`: up to the next `8=FIX` after the
/// first byte, or all of it.
fn next_begin(buffer: &[u8]) -> usize {
    let len = buffer.windows(5).skip(1).position(|w| w == b"8=FIX").map_or(buffer.len(), |i| i + 1);
    assert!(len > 0, "unframed bytes are never empty");
    len
}

/// The MsgType of a frame: the first `35=` field. Empty if there is none.
fn msg_type(frame: &[u8]) -> &[u8] {
    let Some(start) = frame.windows(4).position(|w| w == b"\x0135=").map(|i| i + 4) else { return &[] };
    let len = frame[start..].iter().position(|&b| b == SOH).unwrap_or(frame.len() - start);
    &frame[start..start + len]
}

/// `frame` with a CheckSum one more than the right one, so its length is unchanged.
fn garble(frame: &[u8]) -> Vec<u8> {
    let body_end = frame.len() - TRAILER_LEN;
    let sum = frame[..body_end].iter().fold(0u8, |sum, &b| sum.wrapping_add(b));
    let mut garbled = frame.to_vec();
    garbled[body_end + 3..body_end + 6].copy_from_slice(format!("{:03}", sum.wrapping_add(1)).as_bytes());
    debug_assert_eq!(garbled.len(), frame.len());
    garbled
}

#[cfg(test)]
mod tests {
    use tokio::time::timeout;

    use super::*;

    /// Long enough for the proxy to have forwarded anything it was going to.
    const QUIET: Duration = Duration::from_millis(200);

    /// A FIX 4.4 frame with a correct CheckSum.
    fn frame(msg_type: &str, seq: u32) -> Vec<u8> {
        let body = format!("35={msg_type}\x0134={seq}\x0149=TJ\x0156=QFJ\x01");
        let head = format!("8=FIX.4.4\x019={}\x01{body}", body.len());
        let sum = head.bytes().fold(0u8, |sum, b| sum.wrapping_add(b));
        format!("{head}10={sum:03}\x01").into_bytes()
    }

    /// A proxy whose downstream end plays Turbojet as initiator, so [`Dir::ToPeer`] runs from the
    /// connecting client to the listening server.
    async fn start() -> (Proxy, TcpListener) {
        let server = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let proxy = Proxy::start(server.local_addr().unwrap(), Role::TjInitiator).await;
        (proxy, server)
    }

    /// Turbojet's end and QuickFIX/J's end of a new connection pair.
    async fn connect(proxy: &mut Proxy, server: &TcpListener) -> (TcpStream, TcpStream) {
        let tj = TcpStream::connect(proxy.addr()).await.unwrap();
        let (peer, _) = server.accept().await.unwrap();
        proxy.connected().await;
        (tj, peer)
    }

    async fn read_exactly(stream: &mut TcpStream, len: usize) -> Vec<u8> {
        let mut bytes = vec![0; len];
        timeout(EVENT_TIMEOUT, stream.read_exact(&mut bytes)).await.unwrap().unwrap();
        bytes
    }

    async fn assert_closed(stream: &mut TcpStream) {
        let mut byte = [0];
        let read = timeout(EVENT_TIMEOUT, stream.read(&mut byte)).await.unwrap();
        assert!(matches!(read, Ok(0) | Err(_)), "expected a close, read {read:?}");
    }

    /// Nothing arrives, and the connection stays open.
    async fn assert_silent(stream: &mut TcpStream) {
        let mut byte = [0];
        if let Ok(read) = timeout(QUIET, stream.read(&mut byte)).await {
            panic!("expected silence, read {read:?}");
        }
    }

    #[test]
    fn parses_frames_and_garbage() {
        let order = frame("D", 2);
        assert_eq!(parse(&order), Parse::Frame(order.len()));
        assert_eq!(parse(&order[..order.len() - 1]), Parse::Incomplete);
        assert_eq!(parse(&order[..5]), Parse::Incomplete);
        assert_eq!(parse(b""), Parse::Incomplete);
        let garbage = [&b"junk"[..], &order].concat();
        assert_eq!(parse(&garbage), Parse::Unframed(4));
        let bad_length = b"8=FIX.4.4\x019=x\x01";
        assert_eq!(parse(bad_length), Parse::Unframed(bad_length.len()));
        // A BodyLength past the limit isn't waited for.
        let too_long = b"8=FIX.4.4\x019=999999\x01";
        assert_eq!(parse(too_long), Parse::Unframed(too_long.len()));
        // A header that never ends isn't waited for either.
        assert_eq!(parse(&[b'8'; MAX_HEADER]), Parse::Unframed(MAX_HEADER));
        assert_eq!(msg_type(&order), b"D");
    }

    #[test]
    fn garbling_keeps_the_length_and_breaks_the_checksum() {
        let order = frame("D", 2);
        let garbled = garble(&order);
        assert_eq!(garbled.len(), order.len());
        assert_eq!(garbled[..order.len() - 4], order[..order.len() - 4]);
        assert_ne!(garbled, order);
        assert_eq!(parse(&garbled), Parse::Frame(order.len()));
    }

    /// Frames split across writes, several in one write, and unframed bytes, both ways.
    #[tokio::test]
    async fn forwards_frames_intact_both_ways() {
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        let logon = frame("A", 1);
        let (first, second) = logon.split_at(13);
        tj.write_all(first).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        tj.write_all(second).await.unwrap();
        assert_eq!(read_exactly(&mut peer, logon.len()).await, logon);

        let several = [frame("0", 2), b"not fix".to_vec(), frame("D", 3), frame("D", 4)].concat();
        peer.write_all(&several).await.unwrap();
        assert_eq!(read_exactly(&mut tj, several.len()).await, several);
        tj.write_all(&several).await.unwrap();
        assert_eq!(read_exactly(&mut peer, several.len()).await, several);

        // A close passes on, after what came before it.
        let partial = b"8=FIX.4.4\x019=5";
        tj.write_all(partial).await.unwrap();
        drop(tj);
        assert_eq!(read_exactly(&mut peer, partial.len()).await, partial);
        assert_closed(&mut peer).await;
        assert!(proxy.ended(Dir::ToPeer).await);
        proxy.disconnected().await;
    }

    /// Each one-shot fault takes the first frame of its type in its direction, and only that one.
    #[tokio::test]
    async fn drop_and_garble_apply_once() {
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.drop_next(Dir::ToPeer, "D");
        proxy.garble_next(Dir::ToTj, "8");

        let (heartbeat, order, order2) = (frame("0", 2), frame("D", 3), frame("D", 4));
        tj.write_all(&[heartbeat.clone(), order.clone(), order2.clone()].concat()).await.unwrap();
        assert_eq!(read_exactly(&mut peer, heartbeat.len() + order2.len()).await, [heartbeat, order2].concat());
        let dropped = proxy.applied(Dir::ToPeer, Fault::Drop, "D").await;
        assert!(dropped.contains("|34=3|"), "{dropped}");

        // An order the other way isn't dropped; the first ExecutionReport is garbled.
        let (order, report, report2) = (frame("D", 2), frame("8", 3), frame("8", 4));
        peer.write_all(&[order.clone(), report.clone(), report2.clone()].concat()).await.unwrap();
        let received = read_exactly(&mut tj, order.len() + report.len() + report2.len()).await;
        assert_eq!(received[..order.len()], order);
        assert_eq!(received[order.len()..order.len() + report.len()], garble(&report));
        assert_eq!(received[order.len() + report.len()..], report2);
        let garbled = proxy.applied(Dir::ToTj, Fault::Garble, "8").await;
        assert!(garbled.contains("|34=3|"), "{garbled}");
        assert!(proxy.events.drain().iter().all(|e| !matches!(e, ProxyEvent::Applied { .. })));
    }

    /// The receiver gets the first half of the frame, then both sides are closed.
    #[tokio::test]
    async fn cut_mid_closes_both_sides() {
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.cut_mid(Dir::ToTj, "D");
        let (heartbeat, order) = (frame("0", 2), frame("D", 3));
        // An order the other way isn't cut.
        tj.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut peer, order.len()).await, order);
        peer.write_all(&[heartbeat.clone(), order.clone()].concat()).await.unwrap();
        let half = &order[..order.len() / 2];
        assert_eq!(read_exactly(&mut tj, heartbeat.len() + half.len()).await, [&heartbeat[..], half].concat());
        assert_closed(&mut tj).await;
        assert_closed(&mut peer).await;
        proxy.applied(Dir::ToTj, Fault::Cut, "D").await;
        proxy.disconnected().await;
    }

    /// A blackhole swallows its direction and withholds a close from either side until healed.
    #[tokio::test]
    async fn blackhole_swallows_and_withholds_a_close() {
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.blackhole(Dir::ToTj);
        peer.write_all(&frame("1", 2)).await.unwrap();
        assert_silent(&mut tj).await;
        proxy.applied(Dir::ToTj, Fault::Blackhole, "1").await;
        // The other direction still flows.
        let order = frame("D", 2);
        tj.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut peer, order.len()).await, order);

        drop(tj);
        assert!(!proxy.ended(Dir::ToPeer).await);
        assert_silent(&mut peer).await;
        // Its writes don't block or fail, as on a dead link.
        for seq in 3..100 {
            peer.write_all(&frame("0", seq)).await.unwrap();
        }
        proxy.heal();
        assert_closed(&mut peer).await;
        proxy.disconnected().await;

        // Withheld the other way too, until the other side closes as well.
        let (mut tj, peer) = connect(&mut proxy, &server).await;
        proxy.blackhole(Dir::ToPeer);
        drop(peer);
        assert!(!proxy.ended(Dir::ToTj).await);
        assert_silent(&mut tj).await;
        drop(tj);
        assert!(proxy.ended(Dir::ToPeer).await);
        proxy.disconnected().await;
    }

    /// Faults clear with the pair: a reconnect gets a clean link.
    #[tokio::test]
    async fn a_new_connection_is_fault_free() {
        let (mut proxy, server) = start().await;
        proxy.drop_next(Dir::ToPeer, "D");
        proxy.blackhole(Dir::ToTj);
        let (tj, peer) = connect(&mut proxy, &server).await;
        drop(tj);
        assert!(!proxy.ended(Dir::ToPeer).await);
        drop(peer);
        assert!(proxy.ended(Dir::ToTj).await);
        proxy.disconnected().await;

        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        let order = frame("D", 2);
        tj.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut peer, order.len()).await, order);
        peer.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut tj, order.len()).await, order);
        drop(peer);
        assert_closed(&mut tj).await;
        assert!(proxy.ended(Dir::ToTj).await);
    }

    /// A hold keeps reading, so the sender isn't blocked, and a release forwards it all in order.
    #[tokio::test]
    async fn hold_then_release_forwards_in_order() {
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.hold(Dir::ToPeer);
        let held = [frame("0", 2), frame("D", 3), frame("1", 4)].concat();
        tj.write_all(&held).await.unwrap();
        for msg_type in ["0", "D", "1"] {
            proxy.applied(Dir::ToPeer, Fault::Hold, msg_type).await;
        }
        assert_silent(&mut peer).await;
        // The other direction isn't held.
        let order = frame("D", 2);
        peer.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut tj, order.len()).await, order);

        proxy.release(Dir::ToPeer);
        assert_eq!(read_exactly(&mut peer, held.len()).await, held);
        let after = frame("D", 5);
        tj.write_all(&after).await.unwrap();
        assert_eq!(read_exactly(&mut peer, after.len()).await, after);
        proxy.expect_none("more holds", |e| matches!(e, ProxyEvent::Applied { .. }), QUIET).await;
    }

    /// The delayed frame waits its time, what follows waits behind it, and what comes before
    /// doesn't.
    #[tokio::test]
    async fn delay_next_holds_one_frame_and_what_follows() {
        const DELAY: Duration = Duration::from_millis(400);
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.delay_next(Dir::ToPeer, "D", DELAY);
        let (before, order, after) = (frame("0", 2), frame("D", 3), frame("0", 4));
        let sent_at = Instant::now();
        tj.write_all(&[before.clone(), order.clone(), after.clone()].concat()).await.unwrap();
        assert_eq!(read_exactly(&mut peer, before.len()).await, before);
        assert!(sent_at.elapsed() < DELAY / 2, "the frame before was held {:?}", sent_at.elapsed());
        assert_eq!(read_exactly(&mut peer, order.len() + after.len()).await, [order, after].concat());
        assert!(sent_at.elapsed() >= DELAY, "delayed only {:?}", sent_at.elapsed());
        proxy.applied(Dir::ToPeer, Fault::Delay, "D").await;

        // Once only.
        let order = frame("D", 5);
        let sent_at = Instant::now();
        tj.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut peer, order.len()).await, order);
        assert!(sent_at.elapsed() < DELAY / 2, "delayed again {:?}", sent_at.elapsed());
    }

    /// 10 KB at 20 KB/s takes about half a second, and arrives intact.
    #[tokio::test]
    async fn bandwidth_paces_forwarding() {
        const RATE: u32 = 20_000;
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.bandwidth(Dir::ToPeer, RATE);
        let frames: Vec<u8> = (2..260).flat_map(|seq| frame("0", seq)).collect();
        assert!(frames.len() >= 10_000, "{}", frames.len());
        let sent_at = Instant::now();
        tj.write_all(&frames).await.unwrap();
        assert_eq!(read_exactly(&mut peer, frames.len()).await, frames);
        let expected = Duration::from_secs(1) * u32::try_from(frames.len()).unwrap() / RATE;
        let elapsed = sent_at.elapsed();
        assert!(elapsed >= expected * 9 / 10, "{} bytes in {elapsed:?}, expected {expected:?}", frames.len());
        assert!(elapsed <= expected * 2, "{} bytes in {elapsed:?}, expected {expected:?}", frames.len());
        // The other direction isn't paced.
        let sent_at = Instant::now();
        peer.write_all(&frames).await.unwrap();
        assert_eq!(read_exactly(&mut tj, frames.len()).await, frames);
        assert!(sent_at.elapsed() < expected / 2, "{:?}", sent_at.elapsed());
    }

    /// While the proxy doesn't read from it, the sender's writes back up and block, and the other
    /// direction still flows. After the stall everything arrives, in order.
    #[tokio::test]
    async fn stall_backs_up_the_sender() {
        /// More than any loopback socket buffers; reaching it means the stall didn't hold.
        const LIMIT: usize = 64 * 1024 * 1024;
        let (mut proxy, server) = start().await;
        let (mut tj, mut peer) = connect(&mut proxy, &server).await;
        proxy.stall(Dir::ToPeer);
        let block = vec![b'x'; 64 * 1024];
        let mut written = 0;
        while let Ok(size) = timeout(QUIET, tj.write(&block)).await {
            written += size.unwrap();
            assert!(written < LIMIT, "wrote {written} bytes to a stalled proxy");
        }
        let order = frame("D", 2);
        peer.write_all(&order).await.unwrap();
        assert_eq!(read_exactly(&mut tj, order.len()).await, order);

        proxy.unstall(Dir::ToPeer);
        assert_eq!(read_exactly(&mut peer, written).await, vec![b'x'; written]);
        let after = frame("D", 3);
        tj.write_all(&after).await.unwrap();
        assert_eq!(read_exactly(&mut peer, after.len()).await, after);
        eprintln!("a stalled reader backed up the sender after {written} bytes");
    }
}
