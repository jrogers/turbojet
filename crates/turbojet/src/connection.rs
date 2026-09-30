//! Drives a [`Session`] over any byte stream.

use std::io;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::time::Instant;
use tracing::{Instrument, debug, warn};

use crate::codec::{DecodedInto, decode_into};
use crate::message::Message;
use crate::registry::CommandReceiver;
use crate::session::Session;
use crate::shutdown::Signal;
use crate::telemetry;

/// Commands taken from a [`SessionHandle`](crate::SessionHandle) queue in one batch, so a flood of
/// sends can't starve reading from the peer.
const MAX_COMMANDS_PER_BATCH: usize = 256;

/// Initial capacity of the read buffer; a larger message still arrives, as the buffer grows.
///
/// Measured with 1,000 orders in flight: 8-16 KiB is fastest, and 64 KiB was 12-14% slower. Larger
/// reads make larger batches, so the peer waits longer for the first replies and the two ends
/// overlap less. Bigger is not better here.
const READ_BUFFER_SIZE: usize = 8 * 1024;

/// Longest the connection waits without calling `on_timer`: schedule boundaries are wall-clock
/// times, which [`Session::next_deadline`] doesn't cover.
const MAX_TIMER_SLEEP: Duration = Duration::from_secs(1);

/// Runs `session` over `stream` until either side disconnects.
///
/// Each wake-up (a read from the peer, a batch of handle commands, or a timer deadline) can produce
/// several outgoing messages; the session encodes them into one buffer, written with a single
/// write and flush.
///
/// Works with any transport (plain TCP, TLS, in-memory duplex), so custom transports can reuse
/// the engine without going through [`Acceptor`](crate::Acceptor) or
/// [`Initiator`](crate::Initiator).
pub async fn run<S>(stream: S, session: Session, commands: CommandReceiver) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    run_tracked(stream, session, commands, &mut false, None).await
}

/// [`run`], also recording in `logged_on` whether the session ever logged on, and following
/// `shutdown` (from an Acceptor or Initiator): logging out once it starts, and closing the
/// connection at once, whatever it's doing, if shutdown gives up waiting.
///
/// Runs inside a `session` span whose `id` field is filled in once the session knows its ID.
pub(crate) async fn run_tracked<S>(
    stream: S,
    session: Session,
    commands: CommandReceiver,
    logged_on: &mut bool,
    shutdown: Option<Signal>,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let span = tracing::info_span!("session", id = tracing::field::Empty);
    let mut closing = shutdown.clone();
    async {
        tokio::select! {
            result = drive(stream, session, commands, logged_on, shutdown) => result,
            // Dropping the driver drops the session, which notifies the application, and the
            // stream, which closes the connection.
            () = async { closing.as_mut().expect("guarded by is_some").closing().await }, if closing.is_some() => {
                warn!("closing the connection: shutdown timed out waiting for the logout");
                Ok(())
            }
        }
    }
    .instrument(span)
    .await
}

#[expect(clippy::too_many_lines, reason = "see ROADMAP: split long functions")]
async fn drive<S>(
    stream: S,
    mut session: Session,
    mut commands: CommandReceiver,
    logged_on: &mut bool,
    mut shutdown: Option<Signal>,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let (mut reader, mut writer) = tokio::io::split(stream);
    let mut buf = Vec::with_capacity(READ_BUFFER_SIZE);
    // Every inbound frame is decoded into this one message, which keeps its allocations.
    let mut scratch = Message::default();
    // Bytes read before the session is bound (an acceptor's Logon) are attributed once it is.
    let mut unattributed_bytes = 0;
    let timer = tokio::time::sleep(MAX_TIMER_SLEEP);
    tokio::pin!(timer);
    let mut stuck = None;

    // A connection made once shutdown has started closes without logging on.
    match shutdown.as_ref().and_then(Signal::started_now) {
        Some(text) => {
            shutdown = None;
            session.on_shutdown(text.as_deref(), Instant::now().into_std());
        }
        None => session.on_connect(Instant::now().into_std()),
    }
    loop {
        *logged_on |= session.has_logged_on();
        if let Some(metrics) = session.metrics() {
            metrics.bytes_received(std::mem::take(&mut unattributed_bytes));
            metrics.bytes_sent(session.output().len());
        }
        // Everything the session sent, a Logout before a close included, goes out in one write.
        if !session.output().is_empty() {
            writer.write_all(session.output()).await?;
            // Buffering transports (TLS in particular) may hold written data until flushed;
            // without this, replies can sit unsent while the driver waits for the peer.
            writer.flush().await?;
            session.clear_output();
        }
        if session.is_closed() {
            // Release the session (and its store) before the peer sees the close, so an
            // immediate reconnect can log on again.
            drop(session);
            let _ = writer.shutdown().await;
            return Ok(());
        }
        // Only ever bring the timer forward: most sends and receives push deadlines later, and a
        // timer that fires early just finds nothing due.
        if let Some(deadline) = session.next_deadline().map(Instant::from_std)
            && deadline < timer.deadline()
            && stuck != Some(deadline)
        {
            timer.as_mut().reset(deadline);
        }

        tokio::select! {
            read = reader.read_buf(&mut buf) => {
                let read = read?;
                if read == 0 {
                    return Ok(());
                }
                unattributed_bytes += read;
                // Decode everything this read delivered, then drop the consumed bytes once; any
                // partial message at the end stays for the next read. The messages arrived
                // together, so they share one timestamp. Each is decoded into the same `scratch`.
                let now = Instant::now().into_std();
                let mut consumed = 0;
                loop {
                    match decode_into(&buf[consumed..], session.data_fields(), &mut scratch) {
                        DecodedInto::Message(len) => {
                            consumed += len;
                            debug!(target: "turbojet::messages", direction = "in", "{}", scratch.redacted());
                            session.on_message(&scratch, now);
                        }
                        DecodedInto::Incomplete => break,
                        DecodedInto::Garbled { skip, reason } => {
                            warn!("discarding {skip} garbled bytes: {reason}");
                            telemetry::garbled_message();
                            consumed += skip;
                        }
                    }
                }
                buf.drain(..consumed);
            }
            Some(command) = commands.recv() => {
                let now = Instant::now().into_std();
                session.on_command(command, now);
                // Take whatever else is already queued, so a burst of sends becomes one write.
                for _ in 1..MAX_COMMANDS_PER_BATCH {
                    let Ok(command) = commands.try_recv() else { break };
                    session.on_command(command, now);
                }
            }
            // Once only: after that the session's logout (or its timeout) ends the connection.
            text = async { shutdown.as_mut().expect("guarded by is_some").started().await }, if shutdown.is_some() => {
                shutdown = None;
                session.on_shutdown(text.as_deref(), Instant::now().into_std());
            }
            () = &mut timer => {
                let now = Instant::now();
                timer.as_mut().reset(now + MAX_TIMER_SLEEP);
                session.on_timer(now.into_std());
                // A deadline on_timer left in the past waits for the ceiling rather than spin.
                stuck = session.next_deadline().map(Instant::from_std).filter(|deadline| *deadline <= now);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context as TaskContext, Poll};

    use tokio::io::{DuplexStream, ReadBuf, duplex};

    use super::*;
    use crate::application::{Application, Context, MessageReject};
    use crate::codec::{Decoded, encode, frame_with_raw_field};
    use crate::fields::MsgType;
    use crate::message::{tags, utc_timestamp};
    use crate::registry::SessionRegistry;
    use crate::session::SessionConfig;
    use crate::store::SessionId;

    /// A stream that counts the write calls that move data.
    struct CountingStream {
        inner: DuplexStream,
        writes: Arc<AtomicUsize>,
    }

    impl AsyncRead for CountingStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut TaskContext<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for CountingStream {
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>, data: &[u8]) -> Poll<io::Result<usize>> {
            let result = Pin::new(&mut self.inner).poll_write(cx, data);
            if let Poll::Ready(Ok(n)) = result
                && n > 0
            {
                self.writes.fetch_add(1, Ordering::SeqCst);
            }
            result
        }
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_flush(cx)
        }
        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.inner).poll_shutdown(cx)
        }
    }

    /// Acknowledges every application message with an ExecutionReport.
    struct Acker;

    impl Application for Acker {
        fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
            ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
            Ok(())
        }
    }

    fn from_peer(seq: u64, body: Message) -> Vec<u8> {
        encode(&with_peer_header(seq, body)).unwrap()
    }

    /// `body` with the peer's header, ready to encode.
    fn with_peer_header(seq: u64, body: Message) -> Message {
        let mut msg = Message::default();
        msg.push(tags::BEGIN_STRING, "FIX.4.2");
        msg.push(tags::MSG_TYPE, body.msg_type());
        msg.push(tags::SENDER_COMP_ID, "PEER");
        msg.push(tags::TARGET_COMP_ID, "US");
        msg.push(tags::MSG_SEQ_NUM, seq);
        msg.push(tags::SENDING_TIME, utc_timestamp());
        for (tag, value) in body.fields().filter(|(t, _)| *t != tags::MSG_TYPE) {
            msg.push(tag, value);
        }
        msg
    }

    /// Reads from `peer` until `count` complete messages have arrived.
    async fn receive(peer: &mut DuplexStream, buf: &mut Vec<u8>, count: usize) -> Vec<Message> {
        let mut messages = Vec::new();
        while messages.len() < count {
            match crate::codec::decode(buf) {
                Decoded::Message(msg, len) => {
                    buf.drain(..len);
                    messages.push(msg);
                }
                Decoded::Incomplete => {
                    let read = tokio::time::timeout(Duration::from_secs(5), peer.read_buf(buf)).await;
                    assert!(read.expect("timed out").expect("read failed") > 0, "connection closed");
                }
                Decoded::Garbled { reason, .. } => panic!("garbled output: {reason}"),
            }
        }
        messages
    }

    /// Runs an acceptor and logs the peer on with HeartBtInt=1. Returns the peer's end and read
    /// buffer once the Logon reply has arrived.
    async fn logged_on_with_one_second_heartbeats() -> (DuplexStream, Vec<u8>) {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 1u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let mut buf = Vec::new();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);
        (peer, buf)
    }

    /// A Heartbeat falls due HeartBtInt after the last send, not at the next whole-second tick.
    #[tokio::test(start_paused = true)]
    async fn heartbeat_goes_out_when_due_not_on_the_next_tick() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        tokio::time::advance(Duration::from_millis(300)).await;
        peer.write_all(&from_peer(2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A"))).await.unwrap();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::ExecutionReport);
        let acked = tokio::time::Instant::now();

        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(acked.elapsed(), Duration::from_secs(1));
    }

    #[tokio::test(start_paused = true)]
    async fn silent_counterparty_is_probed_then_dropped() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        let start = tokio::time::Instant::now();

        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(start.elapsed(), Duration::from_secs(1));
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::TestRequest);
        assert_eq!(start.elapsed(), Duration::from_millis(1200));

        // Unanswered: disconnected HeartBtInt later, with nothing else sent.
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
        assert!(buf.is_empty() && rest.is_empty(), "{:?}", String::from_utf8_lossy(&rest));
        assert_eq!(start.elapsed(), Duration::from_millis(2200));
    }

    /// Traffic keeps pushing the deadlines later, and the timer sends nothing meanwhile.
    #[tokio::test(start_paused = true)]
    async fn nothing_extra_is_sent_while_traffic_keeps_the_link_up() {
        let (mut peer, mut buf) = logged_on_with_one_second_heartbeats().await;
        for seq in 2..12 {
            // Off the whole second, so a once-a-second tick would send the last Heartbeat late.
            // Sleep rather than advance, which would fire the timer late and shift its phase.
            tokio::time::sleep(Duration::from_millis(650)).await;
            let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{seq}"));
            peer.write_all(&from_peer(seq, order)).await.unwrap();
            let reply = receive(&mut peer, &mut buf, 1).await;
            assert_eq!(reply[0].msg_type(), MsgType::ExecutionReport, "at order {seq}");
        }
        let acked = tokio::time::Instant::now();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(acked.elapsed(), Duration::from_secs(1));
    }

    /// A deadline already past when it's learned goes out at once, even if the timer has fired
    /// since that deadline.
    #[tokio::test(start_paused = true)]
    async fn an_overdue_heartbeat_goes_out_as_soon_as_logon_completes() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let mut config = crate::InitiatorConfig::new(SessionConfig::new("FIX.4.2", "US"), "PEER");
        config.heartbeat_interval = Duration::from_secs(1);
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::initiator(&config, registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));
        let mut buf = Vec::new();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Logon);

        // Sleep rather than advance, so the timer fires at 1 s, before the reply: our Heartbeat
        // is then overdue.
        tokio::time::sleep(Duration::from_millis(1500)).await;
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 1u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();
        let replied = tokio::time::Instant::now();
        assert_eq!(receive(&mut peer, &mut buf, 1).await[0].msg_type(), MsgType::Heartbeat);
        assert_eq!(replied.elapsed(), Duration::ZERO);
    }

    /// Spoils every outgoing Heartbeat with an SOH, so the session drops it and its Heartbeat stays
    /// due; counts the attempts.
    struct SpoilsHeartbeats {
        attempts: AtomicUsize,
        spinning: tokio::sync::Notify,
    }

    impl Application for SpoilsHeartbeats {
        fn to_admin(&self, _session: &SessionId, msg: &mut Message) {
            if msg.msg_type() == MsgType::Heartbeat {
                msg.set(tags::TEXT, "a\x01b");
                if self.attempts.fetch_add(1, Ordering::SeqCst) == 100 {
                    self.spinning.notify_one();
                }
            }
        }
        fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
            Ok(())
        }
    }

    /// A timer that fires and finds its deadline still due waits for the next second rather than
    /// firing again at once.
    #[tokio::test(start_paused = true)]
    async fn a_deadline_that_does_not_move_is_retried_once_a_second() {
        let (ours, mut peer) = duplex(1 << 20);
        let app = Arc::new(SpoilsHeartbeats { attempts: AtomicUsize::new(0), spinning: tokio::sync::Notify::new() });
        let registry = Arc::new(SessionRegistry::default());
        let now = tokio::time::Instant::now().into_std();
        let (session, commands) = Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, app.clone(), now);
        tokio::spawn(run(ours, session, commands));
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 1u64);
        peer.write_all(&from_peer(1, logon)).await.unwrap();

        // Keep the link up for 5 s, so no TestRequest goes out. A spinning loop never lets paused
        // time advance, so it is caught by the Notify rather than a timeout.
        let keep_alive = async {
            for seq in 2..12 {
                tokio::time::sleep(Duration::from_millis(500)).await;
                peer.write_all(&from_peer(seq, Message::new(MsgType::Heartbeat))).await.unwrap();
            }
        };
        tokio::select! {
            () = keep_alive => {}
            () = app.spinning.notified() => panic!("the timer is spinning"),
        }
        let attempts = app.attempts.load(Ordering::SeqCst);
        assert!((4..=6).contains(&attempts), "{attempts} Heartbeat attempts in 5 s");
    }

    #[tokio::test]
    async fn messages_split_across_reads_and_garbage_between_them_are_handled() {
        const ORDERS: u64 = 30;
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        for i in 0..ORDERS {
            if i == ORDERS / 2 {
                stream.extend_from_slice(b"garbage between messages");
            }
            stream
                .extend(from_peer(i + 2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}"))));
        }
        // Deliver in 13-byte pieces, so messages (and the garbage) straddle reads.
        for piece in stream.chunks(13) {
            peer.write_all(piece).await.unwrap();
            tokio::task::yield_now().await;
        }

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, usize::try_from(1 + ORDERS).unwrap()).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let acked: Vec<_> = replies[1..].iter().map(|m| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect();
        let expected: Vec<_> = (0..ORDERS).map(|i| format!("O{i}")).collect();
        assert_eq!(acked, expected, "every order acknowledged, in order, with no resend requested");
    }

    #[tokio::test]
    async fn message_with_an_empty_value_is_rejected_and_the_session_continues() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        stream.extend(from_peer(
            2,
            Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A").with(tags::TEXT, ""),
        ));
        stream.extend(from_peer(3, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "B")));
        peer.write_all(&stream).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, 3).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let reject = &replies[1];
        assert_eq!(reject.msg_type(), MsgType::Reject, "not ignored as garbled");
        assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(reject.get(tags::REF_TAG_ID), Some("58"));
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("4"), "tag specified without a value");
        assert_eq!(replies[2].msg_type(), MsgType::ExecutionReport, "no resend requested");
        assert_eq!(replies[2].get(tags::CL_ORD_ID), Some("B"));
    }

    #[tokio::test]
    async fn message_with_a_malformed_field_is_rejected_and_the_session_continues() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry,
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut stream = from_peer(1, logon);
        let order = with_peer_header(2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A"));
        stream.extend(frame_with_raw_field(&order, b"x5=1"));
        stream.extend(from_peer(3, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "B")));
        peer.write_all(&stream).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, 3).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        let reject = &replies[1];
        assert_eq!(reject.msg_type(), MsgType::Reject, "not ignored as garbled");
        assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(reject.get(tags::REF_TAG_ID), None);
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("0"), "invalid tag number");
        assert_eq!(reject.get(tags::TEXT), Some("Invalid tag 'x5'"));
        assert_eq!(replies[2].msg_type(), MsgType::ExecutionReport, "no resend requested");
        assert_eq!(replies[2].get(tags::CL_ORD_ID), Some("B"));
    }

    #[tokio::test]
    async fn each_batch_is_written_with_one_write() {
        const ORDERS: u64 = 20;
        let (ours, mut peer) = duplex(1 << 20);
        let writes = Arc::new(AtomicUsize::new(0));
        let registry = Arc::new(SessionRegistry::default());
        let (session, commands) = Session::acceptor(
            SessionConfig::new("FIX.4.2", "US"),
            registry.clone(),
            Arc::new(Acker),
            Instant::now().into_std(),
        );
        tokio::spawn(run(CountingStream { inner: ours, writes: writes.clone() }, session, commands));

        // The peer logs on and pipelines orders, all arriving in one read.
        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut burst = from_peer(1, logon);
        for i in 0..ORDERS {
            burst
                .extend(from_peer(i + 2, Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, format!("O{i}"))));
        }
        peer.write_all(&burst).await.unwrap();

        let mut buf = Vec::new();
        let replies = receive(&mut peer, &mut buf, usize::try_from(1 + ORDERS).unwrap()).await;
        assert_eq!(replies[0].msg_type(), MsgType::Logon);
        assert_eq!(replies.last().unwrap().get(tags::CL_ORD_ID), Some("O19"));
        assert_eq!(writes.load(Ordering::SeqCst), 1, "Logon reply and {ORDERS} acks should go out in one write");

        // A burst of sends through the handle is also written at once.
        let handle = registry.handle(SessionId {
            begin_string: "FIX.4.2".into(),
            sender_comp_id: "US".into(),
            target_comp_id: "PEER".into(),
        });
        for i in 0..50 {
            handle.send(Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, format!("E{i}"))).unwrap();
        }
        let sent = receive(&mut peer, &mut buf, 50).await;
        assert_eq!(sent.last().unwrap().get(tags::EXEC_ID), Some("E49"), "sent in order");
        assert_eq!(writes.load(Ordering::SeqCst), 2, "50 queued sends should go out in one write");

        // Logout is still written before the disconnect closes the stream.
        handle.logout(Some("bye")).unwrap();
        let logout = receive(&mut peer, &mut buf, 1).await;
        assert_eq!(logout[0].msg_type(), MsgType::Logout);
        peer.write_all(&from_peer(2 + ORDERS, Message::new(MsgType::Logout))).await.unwrap();
        let mut rest = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut rest)).await;
        assert!(closed.expect("did not close").is_ok());
    }

    /// The Logout that answers the peer's Logout is queued in the same call that closes the
    /// session; it still reaches the peer before the connection closes.
    #[tokio::test]
    async fn a_logout_sent_while_closing_reaches_the_peer() {
        let (ours, mut peer) = duplex(1 << 20);
        let registry = Arc::new(SessionRegistry::default());
        let now = Instant::now().into_std();
        let (session, commands) =
            Session::acceptor(SessionConfig::new("FIX.4.2", "US"), registry, Arc::new(Acker), now);
        tokio::spawn(run(ours, session, commands));

        let logon = Message::new(MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, 30u64);
        let mut burst = from_peer(1, logon);
        burst.extend(from_peer(2, Message::new(MsgType::Logout)));
        peer.write_all(&burst).await.unwrap();

        let mut received = Vec::new();
        let closed = tokio::time::timeout(Duration::from_secs(5), peer.read_to_end(&mut received)).await;
        closed.expect("did not close").unwrap();
        let mut sent = Vec::new();
        let mut rest = received.as_slice();
        while let Decoded::Message(msg, len) = crate::codec::decode(rest) {
            sent.push(msg.msg_type());
            rest = &rest[len..];
        }
        assert_eq!(sent, [MsgType::Logon, MsgType::Logout], "{}", String::from_utf8_lossy(&received));
        assert!(rest.is_empty(), "trailing bytes: {}", String::from_utf8_lossy(rest));
    }
}
