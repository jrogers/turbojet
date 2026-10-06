//! A FIXP client and server, in memory, on clocks the tests move: each test pumps what one writes
//! into the other and checks what the applications saw, flow by flow, against FIXP 1.0.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::{DateTime, TimeZone, Utc};

use super::*;
use crate::MemoryStorage;
use crate::fixp::{ClientConfig, ServerConfig};
use crate::registry::{Receipt, SessionHandle};
use crate::schedule::Clock;

/// An application message of another schema: an order, with a body of one number.
struct Order(u64);

impl Encode for Order {
    const TEMPLATE_ID: u16 = 102;

    fn encode_into(&self, out: &mut Vec<u8>) -> Result<(), SbeError> {
        m::MessageHeader { block_length: 8, template_id: 102, schema_id: 1, version: 0 }
            .encode(crate::sbe::reserve(out, m::MessageHeader::SIZE));
        out.extend_from_slice(&self.0.to_le_bytes());
        Ok(())
    }
}

fn order_number(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes[8..16].try_into().unwrap())
}

#[derive(Default)]
struct Recorder {
    /// Each order: its number, sequence number and whether retransmitted.
    messages: Mutex<Vec<(u64, Option<u64>, bool)>>,
    established: AtomicUsize,
    not_applied: Mutex<Vec<(u64, u64)>>,
    ended: Mutex<Vec<Ended>>,
    /// Answers each order with one of its number plus 1000.
    echo: bool,
    /// Refuses every client.
    refuse: bool,
}

impl FixpApplication for Recorder {
    fn verify(&self, _client: &ClientLogin<'_>) -> bool {
        !self.refuse
    }
    fn on_established(&self, _session: &FixpHandle) {
        self.established.fetch_add(1, Ordering::Relaxed);
    }
    fn on_message(&self, ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        let number = order_number(msg.bytes);
        self.messages.lock().unwrap().push((number, msg.seq, msg.retransmitted));
        if self.echo {
            ctx.send(&Order(number + 1000)).unwrap();
        }
    }
    fn on_not_applied(&self, _session: &FixpHandle, from: u64, count: u64) {
        self.not_applied.lock().unwrap().push((from, count));
    }
    fn on_ended(&self, _session: &FixpHandle, how: Ended) {
        self.ended.lock().unwrap().push(how);
    }
}

impl Recorder {
    fn messages(&self) -> Vec<(u64, Option<u64>, bool)> {
        self.messages.lock().unwrap().clone()
    }
    fn ended(&self) -> Vec<Ended> {
        self.ended.lock().unwrap().clone()
    }
}

/// A wall clock the tests move.
#[derive(Clone)]
struct Wall(Arc<Mutex<DateTime<Utc>>>);

impl Wall {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Utc.with_ymd_and_hms(2026, 10, 6, 10, 0, 0).unwrap())))
    }
    fn clock(&self) -> Clock {
        let wall = self.0.clone();
        Clock::from_fn(move || *wall.lock().unwrap())
    }
    fn advance(&self, by: Duration) {
        *self.0.lock().unwrap() += chrono::Duration::from_std(by).unwrap();
    }
}

/// One end: what outlives a connection (its registry and store, its application) and the
/// connection's session.
struct End {
    registry: Arc<FixpRegistry>,
    app: Arc<Recorder>,
    config: FixpConfig,
    session: Option<(FixpSession, CommandReceiver<SbeMessage>)>,
}

impl End {
    fn new(role: Role, wall: &Wall, app: Recorder) -> Self {
        let mut config = FixpConfig::new(role);
        config.clock = wall.clock();
        let registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
        Self { registry, app: Arc::new(app), config, session: None }
    }

    fn connect(&mut self, now: Instant) {
        let (mut session, commands) =
            FixpSession::new(self.config.clone(), self.registry.clone(), self.app.clone(), now);
        session.on_connect(now);
        self.session = Some((session, commands));
    }

    fn session(&mut self) -> &mut FixpSession {
        &mut self.session.as_mut().unwrap().0
    }

    /// Hands queued commands to the session, as the driver does once it's established.
    fn take_commands(&mut self, now: Instant) {
        let (session, commands) = self.session.as_mut().unwrap();
        while !session.is_closed() {
            let command = match commands.try_control() {
                Some(command) => command,
                None if session.is_established() => match commands.try_send() {
                    Some(command) => command,
                    None => break,
                },
                None => break,
            };
            session.on_command(command, now);
        }
    }

    /// What it has written since last asked, committed (memory stores commit at once).
    fn written(&mut self) -> Vec<u8> {
        let session = self.session();
        assert!(session.take_commit().is_none(), "memory stores commit at once");
        let out = session.output().to_vec();
        session.clear_output();
        out
    }

    fn feed(&mut self, mut bytes: Vec<u8>, now: Instant) {
        let deferred = self.session().feed(&mut bytes, now);
        assert!(!deferred, "memory stores never make input wait");
        assert!(bytes.is_empty() || self.session().is_closed(), "every complete frame is handled");
    }

    fn feed_message(&mut self, msg: &impl Encode, now: Instant) {
        let mut bytes = Vec::new();
        framing::push(&mut bytes, msg).unwrap();
        self.feed(bytes, now);
    }

    fn disconnect(&mut self, now: Instant) {
        if let Some((mut session, _)) = self.session.take() {
            session.on_disconnect(now);
        }
    }

    fn ended(&mut self) -> Option<Ended> {
        self.session().ended
    }
}

/// The names of the messages in `bytes`, for asserting on what was written.
fn written_messages(bytes: &[u8]) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = bytes;
    while let Framed::Message(len) = framing::frame(rest) {
        let message = match m::decode(&rest[framing::HEADER..len]) {
            Ok((message, _)) => format!("{message:?}"),
            Err(_) => "application".to_string(),
        };
        found.push(message.split(['(', ' ']).next().unwrap().to_string());
        rest = &rest[len..];
    }
    found
}

struct Net {
    wall: Wall,
    now: Instant,
    client: End,
    server: End,
}

impl Net {
    fn new() -> Self {
        Self::with(Recorder::default(), Recorder { echo: true, ..Recorder::default() }, |_| {})
    }

    /// A client and server with these applications, both configured by `configure`.
    fn with(client: Recorder, server: Recorder, configure: impl Fn(&mut FixpConfig)) -> Self {
        let wall = Wall::new();
        let mut client = End::new(Role::Client(ClientConfig::new("CLIENT", "SERVER")), &wall, client);
        let mut server = End::new(Role::Server(ServerConfig::new("SERVER")), &wall, server);
        configure(&mut client.config);
        configure(&mut server.config);
        Self { client, server, wall, now: Instant::now() }
    }

    fn connect(&mut self) {
        self.server.connect(self.now);
        self.client.connect(self.now);
        self.pump();
    }

    fn pump(&mut self) {
        self.pump_losing(false, false);
    }

    /// As `pump`, losing what the client (or the server) writes, as a connection that drops it.
    fn pump_losing(&mut self, client_lost: bool, server_lost: bool) {
        for _ in 0..100 {
            let now = self.now;
            self.client.take_commands(now);
            self.server.take_commands(now);
            let from_client = self.client.written();
            let from_server = self.server.written();
            if from_client.is_empty() && from_server.is_empty() {
                return;
            }
            if !client_lost && !from_client.is_empty() && !self.server.session().is_closed() {
                self.server.feed(from_client, now);
            }
            if !server_lost && !from_server.is_empty() && !self.client.session().is_closed() {
                self.client.feed(from_server, now);
            }
        }
        panic!("the ends kept talking");
    }

    fn tick(&mut self, by: Duration) {
        self.now += by;
        self.wall.advance(by);
        let now = self.now;
        self.client.session().on_timer(now);
        self.server.session().on_timer(now);
        self.pump();
    }

    fn reconnect(&mut self) {
        self.client.disconnect(self.now);
        self.server.disconnect(self.now);
        self.connect();
    }

    fn client_handle(&self) -> SessionHandle<SbeMessage> {
        self.client.registry.handle(SessionId::new("FIXP", "CLIENT", "SERVER"))
    }

    fn server_handle(&mut self) -> SessionHandle<SbeMessage> {
        let id = self.server.session().bound.as_ref().unwrap().id.clone();
        self.server.registry.handle(id)
    }

    fn send_order(&self, number: u64) -> Receipt {
        self.client_handle().send(SbeMessage::encode(&Order(number)).unwrap()).unwrap()
    }

    fn server_sends(&mut self, number: u64) {
        drop(self.server_handle().send(SbeMessage::encode(&Order(number)).unwrap()).unwrap());
    }
}

/// The receipt's sequence number.
fn seq_of(mut receipt: Receipt) -> u64 {
    receipt.try_outcome().expect("answered").expect("sent")
}

#[test]
fn a_new_client_negotiates_then_establishes() {
    let mut net = Net::new();
    net.connect();
    assert!(net.client.session().is_established());
    assert!(net.server.session().is_established());
    assert_eq!(net.client.app.established.load(Ordering::Relaxed), 1);
    assert_eq!(net.server.app.established.load(Ordering::Relaxed), 1);
    let id = net.client.session().session_id();
    assert_eq!(id[6] >> 4, 4, "a version 4 UUID");
    assert_eq!(id[8] >> 6, 0b10, "the RFC 4122 variant");
    assert_eq!(net.server.session().session_id(), id);
    assert_eq!(net.server.session().bound.as_ref().unwrap().id.target_comp_id, uuid_text(&id));
}

#[test]
fn each_sequenced_flow_starts_with_a_sequence() {
    let mut net = Net::new();
    net.server.connect(net.now);
    net.client.connect(net.now);
    let now = net.now;
    let negotiate = net.client.written();
    net.server.feed(negotiate, now);
    let response = net.server.written();
    net.client.feed(response, now);
    let establish = net.client.written();
    net.server.feed(establish, now);
    assert_eq!(written_messages(&net.server.written()), ["EstablishmentAck", "Sequence"]);
}

#[test]
fn application_messages_flow_both_ways_with_implicit_sequence_numbers() {
    let mut net = Net::new();
    net.connect();
    let receipts = [net.send_order(1), net.send_order(2)];
    net.pump();
    assert_eq!(receipts.map(seq_of), [1, 2]);
    assert_eq!(net.server.app.messages(), [(1, Some(1), false), (2, Some(2), false)]);
    assert_eq!(net.client.app.messages(), [(1001, Some(1), false), (1002, Some(2), false)]);
}

#[test]
fn a_reconnect_establishes_the_same_session_and_carries_on_numbering() {
    let mut net = Net::new();
    net.connect();
    let id = net.client.session().session_id();
    drop(net.send_order(1));
    net.pump();
    net.wall.advance(Duration::from_secs(60));
    net.reconnect();
    assert!(net.client.session().is_established());
    assert_eq!(net.client.session().session_id(), id, "re-established, not renegotiated");
    let receipt = net.send_order(2);
    net.pump();
    assert_eq!(seq_of(receipt), 2);
    assert_eq!(net.server.app.messages().last(), Some(&(2, Some(2), false)));
    assert_eq!(net.client.app.messages().last(), Some(&(1002, Some(2), false)));
}

#[test]
fn what_the_client_missed_is_retransmitted_on_reestablishing() {
    let mut net = Net::new();
    net.connect();
    net.server_sends(7);
    net.pump();
    net.server_sends(8);
    net.server_sends(9);
    net.pump_losing(false, true);
    net.reconnect();
    assert_eq!(net.client.app.messages(), [(7, Some(1), false), (8, Some(2), true), (9, Some(3), true)]);
    net.server_sends(10);
    net.pump();
    assert_eq!(net.client.app.messages().last(), Some(&(10, Some(4), false)));
    assert_eq!(net.client.session().log().next_incoming(), 5);
}

#[test]
fn what_the_server_missed_on_a_recoverable_client_flow_is_retransmitted() {
    let mut net = Net::new();
    net.connect();
    drop(net.send_order(1));
    net.pump();
    drop(net.send_order(2));
    drop(net.send_order(3));
    net.pump_losing(true, false);
    net.reconnect();
    let orders: Vec<_> = net.server.app.messages();
    assert_eq!(orders, [(1, Some(1), false), (2, Some(2), true), (3, Some(3), true)]);
}

#[test]
fn what_the_server_missed_on_an_idempotent_client_flow_is_reported_not_applied() {
    let mut net = Net::with(Recorder::default(), Recorder::default(), |c| c.client_flow = FlowType::Idempotent);
    net.connect();
    drop(net.send_order(1));
    net.pump();
    drop(net.send_order(2));
    drop(net.send_order(3));
    net.pump_losing(true, false);
    net.reconnect();
    assert_eq!(net.client.app.not_applied.lock().unwrap().clone(), [(2, 2)]);
    // NotApplied is an application message: it took the server's first sequence number.
    assert_eq!(net.server.session().log().next_outgoing(), 2);
    drop(net.send_order(4));
    net.pump();
    assert_eq!(net.server.app.messages().last(), Some(&(4, Some(4), false)));
}

#[test]
fn a_large_gap_is_asked_for_in_parts() {
    let mut net = Net::with(Recorder::default(), Recorder::default(), |c| c.max_retransmit = 2);
    net.connect();
    for number in 1..=5 {
        net.server_sends(number);
    }
    net.pump_losing(false, true);
    net.reconnect();
    let seqs: Vec<_> = net.client.app.messages().into_iter().map(|(_, seq, re)| (seq, re)).collect();
    assert_eq!(seqs, (1..=5).map(|s| (Some(s), true)).collect::<Vec<_>>());
    assert_eq!(net.client.session().log().next_incoming(), 6);
}

#[test]
fn unsequenced_flows_carry_no_numbers_and_heartbeat_without_them() {
    let mut net = Net::with(Recorder::default(), Recorder { echo: true, ..Recorder::default() }, |c| {
        c.client_flow = FlowType::Unsequenced;
        c.server_flow = FlowType::Unsequenced;
    });
    net.connect();
    let receipt = net.send_order(1);
    net.pump();
    assert_eq!(seq_of(receipt), 0);
    assert_eq!(net.server.app.messages(), [(1, None, false)]);
    assert_eq!(net.client.app.messages(), [(1001, None, false)]);
    net.now += Duration::from_secs(1);
    let now = net.now;
    net.client.session().on_timer(now);
    assert_eq!(written_messages(&net.client.written()), ["UnsequencedHeartbeat"]);
}

#[test]
fn a_none_flow_carries_no_application_messages() {
    let mut net = Net::with(Recorder::default(), Recorder::default(), |c| c.client_flow = FlowType::None);
    net.connect();
    let mut receipt = net.send_order(1);
    net.pump();
    assert!(matches!(receipt.try_outcome(), Some(Err(Dropped::Rejected(_)))));
    // A client that sends one anyway breaks the protocol.
    let now = net.now;
    net.server.feed_message(&Order(1), now);
    assert_eq!(net.server.ended(), Some(Ended::Error));
}

#[test]
fn a_retransmit_request_on_a_flow_that_isnt_recoverable_ends_the_session() {
    let mut net = Net::with(Recorder::default(), Recorder::default(), |c| c.server_flow = FlowType::Idempotent);
    net.connect();
    let session_id = net.client.session().session_id();
    let request = m::RetransmitRequest { session_id, timestamp: 1, from_seq_no: 1, count: 1 };
    let now = net.now;
    net.server.feed_message(&request, now);
    assert_eq!(net.server.ended(), Some(Ended::Error));
}

#[test]
fn retransmit_requests_are_checked() {
    let mut net = Net::new();
    net.connect();
    for number in 1..=3 {
        net.server_sends(number);
    }
    net.pump();
    let id = net.client.session().session_id();
    let now = net.now;
    let ask = |from_seq_no, count, session_id| m::RetransmitRequest { session_id, timestamp: 1, from_seq_no, count };
    net.server.feed_message(&ask(1, 1001, id), now);
    net.server.feed_message(&ask(1, 1, [9; 16]), now);
    assert_eq!(written_messages(&net.server.written()), ["RestransmitReject", "RestransmitReject"]);
    // Asking for what was never sent is out of bounds, which ends the connection.
    net.server.feed_message(&ask(3, 2, id), now);
    assert_eq!(written_messages(&net.server.written()), ["Terminate"]);
    assert!(!net.server.session().is_established());
}

#[test]
fn a_retransmission_no_one_asked_for_ends_the_session() {
    let mut net = Net::new();
    net.connect();
    let id = net.client.session().session_id();
    let now = net.now;
    let retransmission = m::Retransmission { session_id: id, request_timestamp: 1, next_seq_no: 1, count: 0 };
    net.client.feed_message(&retransmission, now);
    assert_eq!(net.client.ended(), Some(Ended::Error));
}

#[test]
fn idle_ends_send_sequences_and_silence_ends_the_connection() {
    let mut net = Net::new();
    net.connect();
    for _ in 0..10 {
        net.tick(Duration::from_secs(1));
    }
    assert!(net.client.session().is_established(), "keepalives kept it up");
    for _ in 0..3 {
        net.now += Duration::from_secs(1);
        let now = net.now;
        net.client.session().on_timer(now);
        let _ = net.client.written();
    }
    assert!(net.client.session().is_closed());
    let code = m::TerminationCode::UnspecifiedError;
    assert_eq!(net.client.app.ended(), [Ended::TerminatedByUs(code)]);
}

#[test]
fn a_terminate_is_answered_and_both_ends_close() {
    let mut net = Net::new();
    net.connect();
    net.client_handle().logout(None).unwrap();
    net.pump();
    assert!(net.client.session().is_closed());
    assert!(net.server.session().is_closed());
    let finished = m::TerminationCode::Finished;
    assert_eq!(net.client.app.ended(), [Ended::TerminatedByUs(finished)]);
    assert_eq!(net.server.app.ended(), [Ended::TerminatedByPeer(finished)]);
}

#[test]
fn negotiation_is_refused_for_credentials_clock_or_flow() {
    let mut net = Net::with(Recorder::default(), Recorder { refuse: true, ..Recorder::default() }, |_| {});
    net.connect();
    assert_eq!(net.client.ended(), Some(Ended::NegotiationRejected(m::NegotiationRejectCode::Credentials)));
    assert_eq!(net.client.app.established.load(Ordering::Relaxed), 0);

    let mut net = Net::new();
    let skewed = Wall::new();
    skewed.advance(Duration::from_secs(600));
    net.client.config.clock = skewed.clock();
    net.connect();
    assert_eq!(net.client.ended(), Some(Ended::NegotiationRejected(m::NegotiationRejectCode::Unspecified)));

    let mut net = Net::new();
    net.client.config.client_flow = FlowType::Idempotent;
    net.connect();
    let code = m::NegotiationRejectCode::FlowTypeNotSupported;
    assert_eq!(net.client.ended(), Some(Ended::NegotiationRejected(code)));
}

#[test]
fn a_client_refuses_a_server_flow_it_didnt_agree() {
    let mut net = Net::new();
    net.client.config.server_flow = FlowType::Idempotent;
    net.connect();
    assert_eq!(net.client.ended(), Some(Ended::Error));
}

#[test]
fn a_session_id_is_negotiated_once() {
    let mut net = Net::new();
    net.connect();
    // The client lost its state but its clock didn't move: the same ID again.
    net.client.disconnect(net.now);
    net.server.disconnect(net.now);
    net.client.registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
    net.connect();
    let code = m::NegotiationRejectCode::DuplicateId;
    assert_eq!(net.client.ended(), Some(Ended::NegotiationRejected(code)));
    // The spent ID is dropped: the next connection negotiates another.
    net.wall.advance(Duration::from_secs(1));
    net.reconnect();
    assert!(net.client.session().is_established());
}

#[test]
fn a_session_the_server_doesnt_know_is_negotiated_again() {
    let mut net = Net::new();
    net.connect();
    let first = net.client.session().session_id();
    net.client.disconnect(net.now);
    net.server.disconnect(net.now);
    net.server.registry = Arc::new(FixpRegistry::with_storage(Arc::new(MemoryStorage::new())));
    net.connect();
    let code = m::EstablishmentRejectCode::Unnegotiated;
    assert_eq!(net.client.ended(), Some(Ended::EstablishmentRejected(code)));
    net.wall.advance(Duration::from_secs(5));
    net.reconnect();
    assert!(net.client.session().is_established());
    assert_ne!(net.client.session().session_id(), first, "a new session");
}

#[test]
fn a_client_on_disk_establishes_the_session_it_negotiated() {
    // The session ID comes from the log's creation time to the nanosecond, which DiskStorage
    // writes as text: it must keep every digit, or the next connection derives another ID.
    let dir = tempfile::tempdir().unwrap();
    let mut net = Net::new();
    net.wall.advance(Duration::from_nanos(123_456_789));
    let disk = crate::DiskStorage::new(dir.path(), false).unwrap();
    net.client.registry = Arc::new(FixpRegistry::with_storage(Arc::new(disk)));
    net.connect();
    let first = net.client.session().session_id();
    net.reconnect();
    assert!(net.client.session().is_established(), "{:?}", net.client.ended());
    assert_eq!(net.client.session().session_id(), first);
}

/// The server sent 1 and 2, which the client lost with the connection; on the next, the client
/// learns of the gap, and 3 arrives live before the retransmission it asks for.
fn a_gap_with_a_live_message_behind_it(net: &mut Net) {
    net.connect();
    net.server_sends(1);
    net.server_sends(2);
    net.pump_losing(false, true);
    net.client.disconnect(net.now);
    net.server.disconnect(net.now);
    let now = net.now;
    net.server.connect(now);
    net.client.connect(now);
    let establish = net.client.written();
    net.server.feed(establish, now);
    net.server_sends(3);
    net.server.take_commands(now);
    let ack_and_order = net.server.written();
    net.client.feed(ack_and_order, now);
}

#[test]
fn live_messages_behind_a_gap_are_delivered_in_order_once_it_fills() {
    let mut net = Net::new();
    a_gap_with_a_live_message_behind_it(&mut net);
    assert!(net.client.app.messages().is_empty(), "3 waits for 1 and 2");
    net.pump();
    assert_eq!(net.client.app.messages(), [(1, Some(1), true), (2, Some(2), true), (3, Some(3), false)]);
}

#[test]
fn live_messages_behind_a_gap_arent_delivered_twice_after_a_reconnect() {
    // Held, not delivered, when the connection drops: asked for again on the next, once.
    let mut net = Net::new();
    a_gap_with_a_live_message_behind_it(&mut net);
    net.reconnect();
    assert_eq!(net.client.app.messages(), [(1, Some(1), true), (2, Some(2), true), (3, Some(3), true)]);
}

#[test]
fn finished_sending_is_answered_once_everything_has_arrived_and_ends_the_session() {
    let mut net = Net::new();
    net.connect();
    net.server_sends(1);
    net.server_sends(2);
    net.pump_losing(false, true);
    let id = net.client.session().session_id();
    let now = net.now;
    // The server finishes after message 2, which the client never got: it asks, then answers.
    net.client.feed_message(&m::FinishedSending { session_id: id, last_seq_no: Some(2) }, now);
    assert_eq!(written_messages(&net.client.written()), ["RetransmitRequest"]);
    let retransmission = m::Retransmission { session_id: id, request_timestamp: 1, next_seq_no: 1, count: 2 };
    net.client.feed_message(&retransmission, now);
    net.client.feed_message(&Order(1), now);
    net.client.feed_message(&Order(2), now);
    assert_eq!(written_messages(&net.client.written()), ["FinishedReceiving"]);
    net.client.feed_message(&m::Terminate { session_id: id, code: m::TerminationCode::Finished, reason: b"" }, now);
    assert_eq!(net.client.ended(), Some(Ended::Finalized));
    // Finalized: the next connection negotiates a new session.
    net.wall.advance(Duration::from_secs(1));
    net.client.disconnect(net.now);
    net.server.disconnect(net.now);
    net.connect();
    assert_ne!(net.client.session().session_id(), id);
}

#[test]
fn application_messages_after_our_terminate_are_ignored() {
    // The counterparty sent them before our Terminate reached it: not a protocol error.
    let mut net = Net::new();
    net.connect();
    let now = net.now;
    net.server.session().on_shutdown(now);
    net.server.feed_message(&Order(1), now);
    assert!(!net.server.session().is_closed(), "{:?}", net.server.ended());
    assert!(net.server.app.messages().is_empty());
    let id = net.server.session().session_id();
    net.server.feed_message(&m::Terminate { session_id: id, code: m::TerminationCode::Finished, reason: b"" }, now);
    assert_eq!(net.server.ended(), Some(Ended::TerminatedByUs(m::TerminationCode::Finished)));
}

#[test]
fn garbled_framing_ends_the_connection() {
    let mut net = Net::new();
    net.connect();
    let now = net.now;
    net.server.feed(vec![0, 0, 0, 20, 0xCA, 0xFE, 0, 0], now);
    assert_eq!(net.server.ended(), Some(Ended::Error));
    assert_eq!(net.server.app.ended(), [Ended::Error]);
}
