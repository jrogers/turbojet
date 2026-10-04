use std::sync::Mutex;

use super::*;
use crate::cancel::{CancelOnDisconnect, CancelTrigger, MAX_CANCEL_GRACE};
use crate::codec::{decode, decode_stored, encode};
use crate::fields::{ApplVerId, FromFix};
use crate::message::utc_timestamp;
use crate::peer::{ConnectionInfo, PeerCertificate};
use crate::registry::{SendError, SessionHandle};
use crate::store::deferring::DeferringStorage;
use crate::store::{MemoryStorage, SessionStorage};

/// Records callbacks. Accepts `D` messages that carry Symbol(55), replying with an
/// ExecutionReport; rejects everything else as unsupported.
#[derive(Default)]
struct TestApp {
    received: Mutex<Vec<Message>>,
    events: Mutex<Vec<String>>,
    handles: Mutex<Vec<SessionHandle>>,
    /// The ConnectionInfo seen by each verify_logon call.
    connections: Mutex<Vec<ConnectionInfo>>,
    refuse_logon: bool,
    /// The callback that panics; `on_message` panics only for ClOrdID "PANIC".
    panic_in: Option<&'static str>,
    /// Leave outgoing Logons alone in `to_admin`, rather than setting Username(553).
    keep_logon: bool,
    /// `Context::maybe_redelivered` for each message `on_message` saw.
    redelivered: Mutex<Vec<bool>>,
    /// The messages `on_admin_message` saw.
    admin: Mutex<Vec<Message>>,
    /// ClOrdIDs whose ExecutionReports `should_resend` declines.
    skip_resend: Vec<&'static str>,
    /// The ClOrdIDs of the messages `should_resend` was asked about.
    resend_asked: Mutex<Vec<String>>,
}

impl TestApp {
    fn received(&self) -> usize {
        self.received.lock().unwrap().len()
    }

    fn events(&self) -> Vec<String> {
        self.events.lock().unwrap().clone()
    }
}

impl Application for TestApp {
    fn verify_logon(&self, _session: &SessionId, _logon: &Message, connection: &ConnectionInfo) -> Result<(), String> {
        self.connections.lock().unwrap().push(connection.clone());
        if self.panic_in == Some("verify_logon") {
            panic!("verify_logon panicked");
        }
        if self.refuse_logon { Err("refused by test".into()) } else { Ok(()) }
    }

    fn to_admin(&self, _session: &SessionId, msg: &mut Message) {
        if self.panic_in == Some("to_admin") && msg.msg_type() == MsgType::Heartbeat {
            panic!("to_admin panicked");
        }
        if msg.msg_type() == MsgType::Logon && !self.keep_logon {
            msg.set(tags::USERNAME, "user");
        }
    }

    fn on_admin_message(&self, _session: &SessionId, msg: &Message) {
        self.admin.lock().unwrap().push(msg.clone());
        if self.panic_in == Some("on_admin_message") {
            panic!("on_admin_message panicked");
        }
    }

    fn should_resend(&self, _session: &SessionId, msg: &Message) -> bool {
        let id = msg.get(tags::CL_ORD_ID).unwrap_or_default();
        self.resend_asked.lock().unwrap().push(id.to_string());
        if self.panic_in == Some("should_resend") {
            panic!("should_resend panicked");
        }
        !self.skip_resend.contains(&id)
    }

    fn on_logon(&self, session: SessionHandle) {
        self.events.lock().unwrap().push(format!("logon {}", session.id().target_comp_id));
        self.handles.lock().unwrap().push(session);
        if self.panic_in == Some("on_logon") {
            panic!("on_logon panicked");
        }
    }

    fn on_logout(&self, session: &SessionId, ended: Disconnect) {
        self.events.lock().unwrap().push(format!("logout {} {ended:?}", session.target_comp_id));
        if self.panic_in == Some("on_logout") {
            panic!("on_logout panicked");
        }
    }

    fn on_cancel_on_disconnect(&self, session: &SessionId, ended: Disconnect) {
        self.events.lock().unwrap().push(format!("cancel {} {ended:?}", session.target_comp_id));
    }

    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        self.redelivered.lock().unwrap().push(ctx.maybe_redelivered());
        if msg.msg_type() != MsgType::NewOrderSingle {
            return Err(MessageReject::unsupported_message_type());
        }
        msg.get(tags::SYMBOL).ok_or_else(|| MessageReject::required_tag_missing(tags::SYMBOL))?;
        if self.panic_in == Some("on_message") && msg.get(tags::CL_ORD_ID) == Some("PANIC") {
            ctx.send(Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "half-done"));
            panic!("on_message panicked");
        }
        let mut received = self.received.lock().unwrap();
        received.push(msg.clone());
        ctx.send(
            Message::new(MsgType::ExecutionReport)
                .with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID))
                .with(tags::EXEC_ID, format!("E{}", received.len())),
        );
        Ok(())
    }
}

struct Harness {
    config: SessionConfig,
    registry: Arc<SessionRegistry>,
    app: Arc<TestApp>,
    t0: Instant,
}

impl Harness {
    fn new() -> Self {
        Self::with_storage(Arc::new(MemoryStorage::new()))
    }

    fn with_storage(storage: Arc<dyn SessionStorage>) -> Self {
        Self {
            config: SessionConfig::new("FIX.4.4", "GATEWAY"),
            registry: Arc::new(SessionRegistry::new(storage)),
            app: Arc::default(),
            t0: Instant::now(),
        }
    }

    fn session(&self) -> Session {
        Session::acceptor(self.config.clone(), self.registry.clone(), self.app.clone(), self.t0).0
    }

    /// A logged-on acceptor session whose counterparty has sent MsgSeqNum 1.
    fn logged_on(&self) -> Session {
        let mut s = self.session();
        let out = s.recv(logon(1), self.t0);
        assert_eq!(sent(&out)[0].msg_type(), MsgType::Logon);
        s
    }

    fn initiator(&self, reset: bool) -> Session {
        self.initiator_with(|config| config.reset_on_logon = reset)
    }

    /// An initiator with its config adjusted by `adjust`.
    fn initiator_with(&self, adjust: impl FnOnce(&mut InitiatorConfig)) -> Session {
        let mut config = InitiatorConfig::new(self.config.clone(), "CLIENT");
        config.heartbeat_interval = Duration::from_secs(20);
        adjust(&mut config);
        Session::initiator(&config, self.registry.clone(), self.app.clone(), self.t0).0
    }

    fn at(&self, secs: u64) -> Instant {
        self.t0 + Duration::from_secs(secs)
    }
}

/// A message from the counterparty "CLIENT" to us, "GATEWAY".
fn client(seq: u64, mtype: MsgType) -> Message {
    Message::default()
        .with(tags::BEGIN_STRING, "FIX.4.4")
        .with(tags::MSG_TYPE, mtype)
        .with(tags::SENDER_COMP_ID, "CLIENT")
        .with(tags::TARGET_COMP_ID, "GATEWAY")
        .with(tags::MSG_SEQ_NUM, seq)
        .with(tags::SENDING_TIME, utc_timestamp())
}

fn logon(seq: u64) -> Message {
    client(seq, MsgType::Logon).with(tags::ENCRYPT_METHOD, "0").with(tags::HEART_BT_INT, "30")
}

fn order(seq: u64, cl_ord_id: &str) -> Message {
    client(seq, MsgType::NewOrderSingle)
        .with(tags::CL_ORD_ID, cl_ord_id)
        .with(tags::SYMBOL, "MSFT")
        .with(tags::SIDE, "2")
        .with(tags::ORDER_QTY, "10")
        .with(tags::ORD_TYPE, "1")
}

/// What a session asked for, decoded from its output: each message sent, then DISCONNECT if
/// the call closed it.
#[derive(Debug)]
enum Action {
    Send(Message),
    Disconnect,
}

/// Session calls returning what each call sent, for assertions.
trait Drive {
    fn connect(&mut self, now: Instant) -> Vec<Action>;
    fn recv(&mut self, msg: Message, now: Instant) -> Vec<Action>;
    fn command(&mut self, command: Command, now: Instant) -> Vec<Action>;
    fn shutdown(&mut self, text: Option<&str>, now: Instant) -> Vec<Action>;
    fn timer(&mut self, now: Instant) -> Vec<Action>;
    fn resume(&mut self, now: Instant) -> Vec<Action>;
}

impl Drive for Session {
    fn connect(&mut self, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_connect(now);
        self.commit_blocking(now);
        taken(self, was)
    }

    fn recv(&mut self, msg: Message, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_message(&msg, now);
        self.commit_blocking(now);
        taken(self, was)
    }

    fn command(&mut self, command: Command, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_command(command, now);
        self.commit_blocking(now);
        taken(self, was)
    }

    fn shutdown(&mut self, text: Option<&str>, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_shutdown(text, now);
        self.commit_blocking(now);
        taken(self, was)
    }

    fn timer(&mut self, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_timer(now);
        self.commit_blocking(now);
        taken(self, was)
    }

    fn resume(&mut self, now: Instant) -> Vec<Action> {
        let was = self.is_closed();
        self.on_resume(now);
        self.commit_blocking(now);
        taken(self, was)
    }
}

/// Takes the session's output, decoded, adding DISCONNECT if it closed since `was_closed`.
fn taken(s: &mut Session, was_closed: bool) -> Vec<Action> {
    let mut actions = Vec::new();
    let mut rest = s.output();
    while !rest.is_empty() {
        match decode_stored(rest, s.data_fields()) {
            Decoded::Message(msg, len) => {
                actions.push(Action::Send(msg));
                rest = &rest[len..];
            }
            _ => panic!("the session sent something that doesn't decode: {:?}", String::from_utf8_lossy(rest)),
        }
    }
    s.clear_output();
    if !was_closed && s.is_closed() {
        actions.push(Action::Disconnect);
    }
    actions
}

fn sent(actions: &[Action]) -> Vec<&Message> {
    actions
        .iter()
        .filter_map(|a| match a {
            Action::Send(m) => Some(m),
            Action::Disconnect => None,
        })
        .collect()
}

/// The first header field after a body field, as the in-sequence check finds it.
fn misplaced_header_field(msg: &Message) -> Option<u32> {
    misplaced_header_and_empty_field(msg, true).0
}

/// Message type names (e.g. "Logout") and "DISCONNECT", in order.
fn types(actions: &[Action]) -> Vec<String> {
    actions
        .iter()
        .map(|a| match a {
            Action::Send(m) => format!("{:?}", m.msg_type()),
            Action::Disconnect => "DISCONNECT".into(),
        })
        .collect()
}

// ---- Acceptor logon ----

#[test]
fn logon_reply_has_header_heartbeat_and_to_admin_fields() {
    let h = Harness::new();
    let mut s = h.session();
    let out = s.recv(logon(1), h.t0);
    let reply = sent(&out)[0];
    assert_eq!(reply.get(tags::SENDER_COMP_ID), Some("GATEWAY"));
    assert_eq!(reply.get(tags::TARGET_COMP_ID), Some("CLIENT"));
    assert_eq!(reply.get(tags::MSG_SEQ_NUM), Some("1"));
    assert_eq!(reply.get(tags::HEART_BT_INT), Some("30"));
    assert_eq!(reply.get(tags::USERNAME), Some("user"));
    assert!(s.is_logged_on());
    assert_eq!(h.app.events(), ["logon CLIENT"]);
}

#[test]
fn first_message_must_be_logon() {
    let h = Harness::new();
    let mut s = h.session();
    assert_eq!(types(&s.recv(client(1, MsgType::Heartbeat), h.t0)), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

#[test]
fn logon_with_wrong_target_is_refused() {
    let h = Harness::new();
    let mut s = h.session();
    let out = s.recv(logon(1).with(tags::TARGET_COMP_ID, "OTHER"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
}

#[test]
fn application_can_refuse_logon() {
    let mut h = Harness::new();
    h.app = Arc::new(TestApp { refuse_logon: true, ..TestApp::default() });
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
    assert!(h.registry.sessions().is_empty());
}

#[test]
fn logon_with_an_empty_value_is_refused() {
    let h = Harness::new();
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1).with(tags::TEXT, ""), h.t0)), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

#[test]
fn logon_timeout_disconnects() {
    let h = Harness::new();
    let mut s = h.session();
    assert!(s.timer(h.at(9)).is_empty());
    assert_eq!(types(&s.timer(h.at(10))), ["DISCONNECT"]);
}

#[test]
fn concurrent_logon_refused_and_sequence_survives_reconnect() {
    let h = Harness::new();
    let mut first = h.logged_on();
    first.recv(client(2, MsgType::Heartbeat), h.t0);

    let mut second = h.session();
    assert_eq!(types(&second.recv(logon(3), h.t0)), ["DISCONNECT"]);
    drop(second);
    drop(first);

    // Reconnect without reset: sequence numbers continue.
    let mut third = h.session();
    let out = third.recv(logon(3), h.t0);
    assert_eq!(types(&out), ["Logon"]);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("2"));
    drop(third);

    // Reset on logon starts again from 1.
    let mut fourth = h.session();
    let out = fourth.recv(logon(1).with(tags::RESET_SEQ_NUM_FLAG, "Y"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("1"));
    assert_eq!(sent(&out)[0].get(tags::RESET_SEQ_NUM_FLAG), Some("Y"));
}

#[test]
fn logon_with_seq_too_low_is_logged_out() {
    let h = Harness::new();
    let mut first = h.logged_on();
    first.recv(client(2, MsgType::Heartbeat), h.t0);
    drop(first);
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logout", "DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost"], "second session never logged on");
}

// ---- Initiator logon ----

#[test]
fn initiator_sends_logon_on_connect() {
    let h = Harness::new();
    let mut s = h.initiator(true);
    let out = s.connect(h.t0);
    let logon = sent(&out)[0];
    assert_eq!(logon.msg_type(), MsgType::Logon);
    assert_eq!(logon.get(tags::SENDER_COMP_ID), Some("GATEWAY"));
    assert_eq!(logon.get(tags::TARGET_COMP_ID), Some("CLIENT"));
    assert_eq!(logon.get(tags::MSG_SEQ_NUM), Some("1"));
    assert_eq!(logon.get(tags::HEART_BT_INT), Some("20"));
    assert_eq!(logon.get(tags::RESET_SEQ_NUM_FLAG), Some("Y"));
    assert_eq!(logon.get(tags::USERNAME), Some("user"));
    assert!(!s.is_logged_on());
}

#[test]
fn initiator_logs_on_after_reply_and_then_behaves_like_any_session() {
    let h = Harness::new();
    let mut s = h.initiator(true);
    s.connect(h.t0);
    assert!(s.recv(logon(1), h.t0).is_empty());
    assert!(s.is_logged_on());
    assert_eq!(h.app.events(), ["logon CLIENT"]);

    // The heartbeat interval is the one we requested.
    assert_eq!(types(&s.timer(h.at(20))), ["Heartbeat"]);
    let out = s.recv(order(2, "A"), h.t0);
    assert_eq!(types(&out), ["ExecutionReport"]);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("3"));
}

#[test]
fn initiator_refuses_reply_from_wrong_counterparty() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    let out = s.recv(logon(1).with(tags::SENDER_COMP_ID, "SOMEONE"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

#[test]
fn initiator_refuses_reply_with_an_empty_value() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    let out = s.recv(logon(1).with(tags::TEXT, ""), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

#[test]
fn initiator_disconnects_when_logon_is_answered_with_logout() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    let out = s.recv(client(1, MsgType::Logout).with(tags::TEXT, "bad password"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
}

#[test]
fn initiator_times_out_waiting_for_reply() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.at(5));
    assert!(s.timer(h.at(14)).is_empty());
    assert_eq!(types(&s.timer(h.at(15))), ["DISCONNECT"]);
}

#[test]
fn initiator_without_reset_continues_sequence() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.recv(logon(1), h.t0);
    drop(s);
    let mut s = h.initiator(false);
    let out = s.connect(h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("2"));
    assert_eq!(sent(&out)[0].get(tags::RESET_SEQ_NUM_FLAG), None);
}

// ---- Credentials ----

#[test]
fn initiator_sends_its_credentials() {
    let mut h = Harness::new();
    h.app = Arc::new(TestApp { keep_logon: true, ..TestApp::default() });
    let mut s = h.initiator_with(|config| {
        config.username = Some("trader".into());
        config.password = Some("secret".into());
    });
    let out = s.connect(h.t0);
    let logon = sent(&out)[0];
    assert_eq!(logon.get(tags::USERNAME), Some("trader"));
    assert_eq!(logon.get(tags::PASSWORD), Some("secret"));

    // Without them, neither tag is sent.
    let mut s = Harness { app: h.app.clone(), ..Harness::new() }.initiator(false);
    let out = s.connect(h.t0);
    assert_eq!((sent(&out)[0].get(tags::USERNAME), sent(&out)[0].get(tags::PASSWORD)), (None, None));
}

#[test]
fn typed_logon_carries_credentials_to_verify_logon() {
    let request = logon(1).with(tags::USERNAME, "trader").with(tags::PASSWORD, "secret");
    let typed: Logon = request.parse().unwrap();
    let password = typed.password.as_ref().map(Secret::expose);
    assert_eq!((typed.username.as_deref(), password), (Some("trader"), Some("secret")));
    let debug = format!("{typed:?}");
    assert!(debug.contains("password: Some(***)") && !debug.contains("secret"), "{debug}");
}

// ---- NextExpectedMsgSeqNum(789) ----

#[test]
fn initiator_sends_next_expected_only_when_configured() {
    let h = Harness::new();
    let out = h.initiator(true).connect(h.t0);
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), None);
    drop(out);

    let h = Harness::new();
    let mut s = h.initiator_with(|config| config.next_expected_msg_seq_num = true);
    let out = s.connect(h.t0);
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), Some("1"));
}

#[test]
fn acceptor_answers_next_expected_with_its_own() {
    let h = Harness::new();
    let mut s = h.session();
    let out = s.recv(logon(1).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "1"), h.t0);
    assert_eq!(types(&out), ["Logon"]);
    // Their Logon (1) is processed, so we next expect 2.
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), Some("2"));

    // A counterparty that doesn't send it doesn't get it.
    let h = Harness::new();
    let out = h.session().recv(logon(1), h.t0);
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), None);
}

#[test]
fn acceptor_resends_what_the_counterparty_missed() {
    let h = Harness::new();
    let mut first = h.logged_on(); // our 1: Logon
    first.recv(order(2, "A"), h.t0); // our 2: ExecutionReport
    first.recv(order(3, "B"), h.t0); // our 3: ExecutionReport
    drop(first);

    // They reconnect having received only our 1.
    let mut s = h.session();
    let out = s.recv(logon(4).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "2"), h.t0);
    let msgs = sent(&out);
    let seen: Vec<_> =
        msgs.iter().map(|m| (m.msg_type(), m.get(tags::MSG_SEQ_NUM), m.get(tags::POSS_DUP_FLAG))).collect();
    assert_eq!(
        seen,
        [
            (MsgType::Logon, Some("4"), None),
            (MsgType::ExecutionReport, Some("2"), Some("Y")),
            (MsgType::ExecutionReport, Some("3"), Some("Y")),
        ]
    );
    assert_eq!(msgs[1].get(tags::CL_ORD_ID), Some("A"));
    assert!(s.is_logged_on());
}

#[test]
fn next_expected_beyond_what_was_sent_logs_out() {
    let h = Harness::new();
    let mut s = h.session();
    // Our reply would be our first message, so they can't expect 5.
    let out = s.recv(logon(1).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "5"), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert!(sent(&out)[0].get(tags::TEXT).unwrap().contains("NextExpectedMsgSeqNum(789) too high"));
    assert!(h.app.events().is_empty(), "never logged on");
}

#[test]
fn a_gap_at_logon_waits_for_the_counterparty_to_resend_when_both_use_next_expected() {
    let h = Harness::new();
    let mut s = h.session();
    // We expect their 1; their Logon is 3.
    let out = s.recv(logon(3).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "1"), h.t0);
    assert_eq!(types(&out), ["Logon"], "no ResendRequest: they resend from our 789");
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), Some("1"));

    // Their resend fills the gap.
    let fill = gap_fill(1, 3);
    assert!(s.recv(fill, h.t0).is_empty());
    // The Logon (3) wasn't consumed as a normal message, so 3 is next.
    assert_eq!(types(&s.recv(order(3, "A"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn a_counterparty_that_does_not_resend_still_gets_a_resend_request() {
    let h = Harness::new();
    let mut s = h.session();
    s.recv(logon(3).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "1"), h.t0);
    // They carry on without resending: the next message out of sequence asks for it.
    let out = s.recv(order(4, "A"), h.t0);
    assert_eq!(types(&out), ["ResendRequest"]);
    assert_eq!(sent(&out)[0].get(tags::BEGIN_SEQ_NO), Some("1"));
}

#[test]
fn initiator_resends_what_the_acceptor_missed() {
    let h = Harness::new();
    let configure = |config: &mut InitiatorConfig| config.next_expected_msg_seq_num = true;
    let mut first = h.initiator_with(configure);
    first.connect(h.t0); // our 1: Logon
    first.recv(logon(1), h.t0);
    first.command(send_command("A"), h.t0); // our 2
    first.command(send_command("B"), h.t0); // our 3
    drop(first);

    let mut s = h.initiator_with(configure);
    let out = s.connect(h.t0); // our 4: Logon
    assert_eq!(sent(&out)[0].get(tags::NEXT_EXPECTED_MSG_SEQ_NUM), Some("2"));
    // The acceptor got our 1 and our Logon, 4, but not 2 or 3.
    let out = s.recv(logon(2).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "2"), h.t0);
    let seen: Vec<_> = sent(&out).iter().map(|m| (m.get(tags::CL_ORD_ID), m.get(tags::MSG_SEQ_NUM))).collect();
    assert_eq!(seen, [(Some("A"), Some("2")), (Some("B"), Some("3"))]);
    assert!(s.is_logged_on());
}

#[test]
fn initiator_logs_out_when_the_acceptor_expects_too_much() {
    let h = Harness::new();
    let mut s = h.initiator_with(|config| {
        config.reset_on_logon = true;
        config.next_expected_msg_seq_num = true;
    });
    s.connect(h.t0); // our 1: Logon
    let out = s.recv(logon(1).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "3"), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
}

// ---- Heartbeats ----

#[test]
fn heartbeat_then_test_request_then_disconnect() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert!(s.timer(h.at(29)).is_empty());
    assert_eq!(types(&s.timer(h.at(30))), ["Heartbeat"]);
    // 30s heartbeat + 20% grace without hearing from the counterparty.
    let out = s.timer(h.at(36));
    assert_eq!(types(&out), ["TestRequest"]);
    assert_eq!(sent(&out)[0].get(tags::TEST_REQ_ID), Some("TEST1"));
    assert_eq!(types(&s.timer(h.at(66))), ["DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT HeartbeatTimeout"]);
}

#[test]
fn heartbeat_is_due_exactly_at_the_interval() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert!(s.timer(h.at(30) - Duration::from_millis(1)).is_empty());
    assert_eq!(types(&s.timer(h.at(30))), ["Heartbeat"]);
}

#[test]
fn inbound_message_satisfies_test_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.timer(h.at(36));
    s.recv(client(2, MsgType::Heartbeat).with(tags::TEST_REQ_ID, "TEST1"), h.at(40));
    assert_eq!(types(&s.timer(h.at(66))), ["Heartbeat"]);
}

#[test]
fn answers_test_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "abc"), h.t0);
    let hb = sent(&out)[0];
    assert_eq!(hb.msg_type(), MsgType::Heartbeat);
    assert_eq!(hb.get(tags::TEST_REQ_ID), Some("abc"));
}

// ---- Sequencing and resends ----

#[test]
fn sequence_gap_triggers_single_resend_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(order(4, "A"), h.t0);
    let req = sent(&out)[0];
    assert_eq!(req.msg_type(), MsgType::ResendRequest);
    assert_eq!(req.get(tags::BEGIN_SEQ_NO), Some("2"));
    assert_eq!(req.get(tags::END_SEQ_NO), Some("0"));
    assert!(s.recv(order(5, "B"), h.t0).is_empty(), "no duplicate ResendRequest");
    assert_eq!(h.app.received(), 0, "out-of-sequence messages are not delivered yet");

    // Counterparty gap-fills 2..3: the queued 4 and 5 follow at once, as sent.
    let fill = gap_fill(2, 4);
    assert_eq!(types(&s.recv(fill, h.t0)), ["ExecutionReport", "ExecutionReport"]);
    assert!(s.resend.is_none());
    let delivered = h.app.received.lock().unwrap().clone();
    assert_eq!(delivered.iter().map(|m| m.get(tags::CL_ORD_ID).unwrap()).collect::<Vec<_>>(), ["A", "B"]);
    assert!(delivered.iter().all(|m| m.get(tags::POSS_DUP_FLAG).is_none()), "the originals");
    // Their resends (the request was open-ended) are duplicates now.
    assert!(s.recv(resend_of(order(4, "A")), h.t0).is_empty());
    assert!(s.recv(resend_of(order(5, "B")), h.t0).is_empty());
    assert_eq!(h.app.received(), 2);
}

/// `msg` with `extra` added to its header, after SendingTime, where a counterparty puts them;
/// `with` would append them after the body.
fn with_header(msg: Message, extra: &[(u32, &str)]) -> Message {
    let mut out = Message::default();
    for (tag, value) in msg.fields() {
        out.push(tag, value);
        if tag == tags::SENDING_TIME {
            for (tag, value) in extra {
                out.push(*tag, *value);
            }
        }
    }
    out
}

/// `msg` as the counterparty resends it: PossDupFlag and OrigSendingTime (its SendingTime) added.
fn resend_of(msg: Message) -> Message {
    let sent = msg.get(tags::SENDING_TIME).unwrap().to_string();
    with_header(msg, &[(tags::POSS_DUP_FLAG, "Y"), (tags::ORIG_SENDING_TIME, &sent)])
}

// ---- Stricter checks ----

fn at_offset(msg: Message, secs: i64) -> Message {
    let time = UtcTimestamp::now() + chrono::TimeDelta::seconds(secs);
    msg.with(tags::SENDING_TIME, time)
}

/// Session test case 2o: a SendingTime more than two minutes off either way is rejected, and the
/// session logs out.
#[test]
fn inaccurate_sending_time_is_rejected_then_logged_out() {
    for offset in [-121, 121] {
        let h = Harness::new();
        let mut s = h.logged_on();
        let out = s.recv(at_offset(client(2, MsgType::Heartbeat), offset), h.t0);
        assert_eq!(types(&out), ["Reject", "Logout"], "{offset}");
        assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("10"));
        assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("52"));
        assert_eq!(sent(&out)[0].get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(s.peer().log.next_incoming(), 3, "it took its number");
    }
    // Within the limit, and with the check off.
    let h = Harness::new();
    let mut s = h.logged_on();
    assert!(s.recv(at_offset(client(2, MsgType::Heartbeat), -100), h.t0).is_empty());
    let mut h = Harness::new();
    h.config.max_latency = None;
    let mut s = h.logged_on();
    assert!(s.recv(at_offset(client(2, MsgType::Heartbeat), -3600), h.t0).is_empty());
}

/// More stale messages after the first are each rejected and logged out over, but the logout
/// timeout still counts from the first Logout, so they can't put it off.
#[test]
fn stale_messages_while_logging_out_do_not_put_off_the_logout_timeout() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(at_offset(client(2, MsgType::Heartbeat), 121), h.at(1))), ["Reject", "Logout"]);
    assert_eq!(types(&s.recv(at_offset(client(3, MsgType::Heartbeat), 121), h.at(2))), ["Reject", "Logout"]);
    assert_eq!(s.next_deadline(), Some(h.at(1) + h.config.logout_timeout));
}

/// Session test case 1d: a Logon with an inaccurate SendingTime is refused.
#[test]
fn logon_with_inaccurate_sending_time_is_refused() {
    let h = Harness::new();
    let mut s = h.session();
    let out = s.recv(logon(1).with(tags::SENDING_TIME, "20010101-00:00:00"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
}

/// Session test case 2g: a resend without OrigSendingTime is rejected naming it; a duplicate
/// doesn't take a number.
#[test]
fn poss_dup_without_orig_sending_time_is_rejected() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Heartbeat), h.t0);
    let bare = client(2, MsgType::Heartbeat).with(tags::POSS_DUP_FLAG, "Y");
    let out = s.recv(bare, h.t0);
    assert_eq!(types(&out), ["Reject"]);
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("122"));
    assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("1"));
    assert_eq!(s.peer().log.next_incoming(), 3);
    // In sequence, it's rejected like any other field, and takes its number.
    let out = s.recv(client(3, MsgType::Heartbeat).with(tags::POSS_DUP_FLAG, "Y"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("122"));
    assert_eq!(s.peer().log.next_incoming(), 4);
    // A proper resend is fine.
    assert!(s.recv(resend_of(client(2, MsgType::Heartbeat)), h.t0).is_empty());
}

/// Session test case 2f: an OrigSendingTime later than SendingTime is rejected, and the session
/// logs out.
#[test]
fn orig_sending_time_after_sending_time_logs_out() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Heartbeat), h.t0);
    let later = (UtcTimestamp::now() + chrono::TimeDelta::seconds(10)).to_fix();
    let out = s.recv(resend_of(client(2, MsgType::Heartbeat)).with(tags::ORIG_SENDING_TIME, later), h.t0);
    assert_eq!(types(&out), ["Reject", "Logout"]);
    assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("10"));
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("122"));
}

#[test]
fn orig_sending_time_can_be_left_unchecked() {
    let mut h = Harness::new();
    h.config.check_orig_sending_time = false;
    let mut s = h.logged_on();
    assert!(s.recv(client(2, MsgType::Heartbeat).with(tags::POSS_DUP_FLAG, "Y"), h.t0).is_empty());
}

/// Session test case 14g: a header field after the body is rejected, naming it.
#[test]
fn header_fields_after_the_body_are_rejected() {
    let h = Harness::new();
    let mut s = h.logged_on();
    // MsgSeqNum and SendingTime after ClOrdID.
    let mut msg = Message::default();
    for (tag, value) in [(8, "FIX.4.4"), (35, "D"), (49, "CLIENT"), (56, "GATEWAY"), (11, "A"), (55, "MSFT")] {
        msg.push(tag, value);
    }
    msg.push(tags::MSG_SEQ_NUM, 2u64);
    msg.push(tags::SENDING_TIME, utc_timestamp());
    let out = s.recv(msg.clone(), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("14"));
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("34"));
    assert_eq!(h.app.received(), 0);

    let mut h = Harness::new();
    h.config.check_header_order = false;
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(msg, h.t0)), ["ExecutionReport"]);
}

/// FIX 4.2 defines SessionRejectReason up to 11: a later reason is left out, and the Text says
/// what's wrong.
#[test]
fn fix42_rejects_leave_out_reasons_added_later() {
    let mut h = Harness::new();
    h.config = SessionConfig::new("FIX.4.2", "GATEWAY");
    let mut s = h.session();
    s.recv(logon(1).with(tags::BEGIN_STRING, "FIX.4.2"), h.t0);
    let mut msg = Message::default();
    for (tag, value) in [(8, "FIX.4.2"), (35, "D"), (49, "CLIENT"), (56, "GATEWAY"), (11, "A"), (55, "MSFT")] {
        msg.push(tag, value);
    }
    msg.push(tags::MSG_SEQ_NUM, 2u64);
    msg.push(tags::SENDING_TIME, utc_timestamp());
    let out = s.recv(msg, h.t0);
    assert_eq!(types(&out), ["Reject"]);
    let reject = sent(&out)[0];
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), None);
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("34"));
    assert!(reject.get(tags::TEXT).unwrap().contains("out of required order"));
}

// ---- Routing fields ----

/// Acceptance scenarios ReverseRoute: a session-level Reject goes back reverse-routed, skipping
/// empty routing fields.
#[test]
fn rejects_are_reverse_routed() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let routed = |seq, extra: &[(u32, &str)]| with_header(order(seq, "A").with(tags::TEXT, ""), extra);
    let out = s.recv(routed(2, &[(tags::ON_BEHALF_OF_COMP_ID, "JCD"), (tags::ON_BEHALF_OF_SUB_ID, "CS")]), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::DELIVER_TO_COMP_ID), Some("JCD"));
    assert_eq!(reject.get(tags::DELIVER_TO_SUB_ID), Some("CS"));
    assert_eq!(misplaced_header_field(reject), None, "in the header");
    let out = s.recv(routed(3, &[(tags::DELIVER_TO_COMP_ID, "JCD")]), h.t0);
    assert_eq!(sent(&out)[0].get(tags::ON_BEHALF_OF_COMP_ID), Some("JCD"));
}

#[test]
fn business_rejects_are_reverse_routed() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let msg = with_header(client(2, MsgType::from_code("U1")), &[(tags::ON_BEHALF_OF_COMP_ID, "JCD")]);
    let out = s.recv(msg, h.t0);
    assert_eq!(sent(&out)[0].msg_type(), MsgType::BusinessMessageReject);
    assert_eq!(sent(&out)[0].get(tags::DELIVER_TO_COMP_ID), Some("JCD"));
}

/// Routing fields an application sets go out in the header, not after the body.
#[test]
fn routing_fields_the_application_sets_go_in_the_header() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let report = Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "E1").with(tags::DELIVER_TO_COMP_ID, "JCD");
    let out = s.command(Command::send(report), h.t0);
    let sent = sent(&out)[0];
    assert_eq!(sent.get(tags::DELIVER_TO_COMP_ID), Some("JCD"));
    assert_eq!(misplaced_header_field(sent), None, "{sent}");
}

// ---- Intraday sequence reset ----

fn reset_logon() -> Message {
    logon(1).with(tags::RESET_SEQ_NUM_FLAG, "Y")
}

/// A Logon with ResetSeqNumFlag=Y and MsgSeqNum 1 while logged on resets both sides: our Logon
/// reply goes out at 1, and theirs counts as 1.
#[test]
fn a_reset_logon_while_logged_on_resets_both_sides() {
    let h = Harness::new();
    let mut s = h.logged_on();
    for seq in 2..=5 {
        s.recv(client(seq, MsgType::Heartbeat), h.t0);
    }
    s.recv(order(6, "A"), h.t0); // our 2: ExecutionReport
    let out = s.recv(reset_logon(), h.t0);
    assert_eq!(types(&out), ["Logon"]);
    let reply = sent(&out)[0];
    assert_eq!(reply.get(tags::MSG_SEQ_NUM), Some("1"));
    assert_eq!(reply.get(tags::RESET_SEQ_NUM_FLAG), Some("Y"));
    assert_eq!(reply.get(tags::HEART_BT_INT), Some("30"));
    assert_eq!((s.peer().log.next_incoming(), s.peer().log.next_outgoing()), (2, 2));
    assert!(s.peer_mut().log.sent_messages(1, u64::MAX).unwrap().is_empty(), "nothing old to resend");
    // Both sides carry on from 2.
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "T"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("2"));
    assert_eq!(h.app.events(), ["logon CLIENT"], "still the same logon");
}

#[test]
fn a_reset_logon_ends_a_gap() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(5, "E"), h.t0); // gap: queued, ResendRequest for 2..
    assert_eq!(types(&s.recv(reset_logon(), h.t0)), ["Logon"]);
    assert!(s.resend.is_none() && s.queued.is_empty());
    assert_eq!(types(&s.recv(order(2, "B"), h.t0)), ["ExecutionReport"]);
    assert_eq!(h.app.received(), 1, "only B: the queued order belonged to the old numbering");
}

/// A reset Logon must be MsgSeqNum 1; otherwise a Logon while logged on is refused as before.
#[test]
fn a_reset_logon_at_another_number_is_refused() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(logon(2).with(tags::RESET_SEQ_NUM_FLAG, "Y"), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    assert_eq!(s.peer().log.next_incoming(), 3);
}

// ---- At-least-once delivery ----

/// Records the order of a store's writes.
struct RecordingStorage {
    inner: MemoryStorage,
    writes: Arc<Mutex<Vec<String>>>,
}

struct RecordingLog {
    inner: Box<dyn SessionLog>,
    writes: Arc<Mutex<Vec<String>>>,
}

impl SessionStorage for RecordingStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        Ok(Box::new(RecordingLog { inner: self.inner.open(id)?, writes: self.writes.clone() }))
    }
}

impl SessionLog for RecordingLog {
    fn next_outgoing(&self) -> u64 {
        self.inner.next_outgoing()
    }
    fn next_incoming(&self) -> u64 {
        self.inner.next_incoming()
    }
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.writes.lock().unwrap().push(format!("incoming {seq}"));
        self.inner.set_next_incoming(seq)
    }
    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        self.writes.lock().unwrap().push(format!("outgoing {seq}"));
        self.inner.record_outgoing(seq, msg)
    }
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        self.inner.sent_messages(begin, end)
    }
    fn reset(&mut self) -> io::Result<()> {
        self.inner.reset()
    }
    fn in_flight(&self) -> Option<u64> {
        self.inner.in_flight()
    }
    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.writes.lock().unwrap().push(format!("in flight {seq}"));
        self.inner.set_in_flight(seq)
    }
}

/// The incoming number is saved once the application has handled the message and its replies
/// are stored, so a crash before then gets the message resent. The window it's handed over in was
/// recorded by the commit before.
#[test]
fn the_incoming_number_is_saved_after_the_application() {
    let writes = Arc::new(Mutex::new(Vec::new()));
    let h = Harness::with_storage(Arc::new(RecordingStorage { inner: MemoryStorage::new(), writes: writes.clone() }));
    let mut s = h.logged_on();
    writes.lock().unwrap().clear();
    s.recv(order(2, "A"), h.t0);
    // The window is recorded again after the number moves on, and the next one as the batch ends.
    assert_eq!(*writes.lock().unwrap(), ["outgoing 2", "incoming 3", "in flight 2", "in flight 3"]);
    writes.lock().unwrap().clear();
    s.recv(client(3, MsgType::Heartbeat), h.t0);
    assert_eq!(*writes.lock().unwrap(), ["incoming 4", "in flight 3", "in flight 4"]);
}

/// After a crash while messages from 2 on may have been with the application, those that come
/// again are marked as possibly handled already: flagged as resends, or answering our
/// ResendRequest. New messages aren't, nor resends past the window.
#[test]
fn the_message_in_flight_at_a_crash_is_marked_when_resent() {
    let storage = Arc::new(MemoryStorage::new());
    {
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GATEWAY".into(),
            target_comp_id: "CLIENT".into(),
        };
        let mut log = storage.open(&id).unwrap();
        log.record_outgoing(1, None).unwrap(); // our Logon
        log.set_next_incoming(2).unwrap();
        log.set_in_flight(2).unwrap();
    }
    let h = Harness::with_storage(storage);
    let mut s = h.session();
    // The counterparty logs on at 4, having sent 2 and 3 before the crash: we ask for them again.
    assert_eq!(types(&s.recv(logon(4), h.t0)), ["Logon", "ResendRequest"]);
    assert_eq!(types(&s.recv(resend_of(order(2, "B")), h.t0)), ["ExecutionReport"]);
    // Resent without PossDupFlag, but answering our ResendRequest.
    s.recv(order(3, "C"), h.t0);
    s.recv(gap_fill(4, 5), h.t0); // its Logon
    s.recv(order(5, "D"), h.t0);
    let past = 2 + DELIVERIES_PER_COMMIT;
    s.recv(gap_fill(6, past), h.t0);
    s.recv(resend_of(order(past, "E")), h.t0);
    let marked = h.app.redelivered.lock().unwrap().clone();
    assert_eq!(marked, [true, true, false, false], "2 and 3 might have been handled; 5 is new, {past} past the window");
}

// ---- Group commit ----

/// Runs the commit a session asks for, if any, as a driver would.
fn run_commit(s: &mut Session, now: Instant) -> bool {
    let Some(commit) = s.take_commit(now) else { return false };
    assert!(s.is_committing() && !s.ready_for_input(), "nothing is fed in meanwhile");
    s.on_committed(commit.run(), now);
    true
}

#[test]
fn output_waits_for_its_commit() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    let commit = s.take_commit(h.t0).expect("the Logon's numbers are to be committed");
    assert!(s.output().is_empty(), "nothing goes out before its commit");
    s.on_committed(commit.run(), h.t0);
    assert_eq!(types(&taken(&mut s, false)), ["Logon"]);
    assert!(s.take_commit(h.t0).is_none(), "nothing more to commit");
}

/// Each commit records the window the next batch is handed over in, so a batch costs one commit:
/// its own, after it.
#[test]
fn a_batch_needs_only_its_own_commit() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    assert!(run_commit(&mut s, h.t0));
    assert_eq!(types(&taken(&mut s, false)), ["Logon"]);
    assert_eq!(s.peer().log.in_flight(), Some(2), "the Logon's commit recorded the first window");
    storage.calls.lock().unwrap().clear();

    for _ in 0..2 {
        assert!(s.ready_for_input(), "the last commit recorded a window");
        let next = s.peer().log.next_incoming();
        s.on_message(&order(next, "A"), h.t0);
        s.on_message(&order(next + 1, "B"), h.t0);
        assert!(s.output().is_empty());
        assert!(run_commit(&mut s, h.t0), "the batch's end");
        assert_eq!(types(&taken(&mut s, false)), ["ExecutionReport", "ExecutionReport"]);
        assert!(s.take_commit(h.t0).is_none(), "one commit");
    }
    let commits = storage.calls.lock().unwrap().iter().filter(|c| *c == "commit").count();
    assert_eq!(commits, 2);
    assert_eq!(s.peer().log.in_flight(), Some(6));
}

/// A session that ends cleanly leaves nothing in flight; one whose connection drops keeps the
/// window, since the batch it was handling may not have been recorded.
#[test]
fn a_clean_end_clears_the_window_and_a_dropped_connection_keeps_it() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(s.peer().log.in_flight(), Some(2));
    assert_eq!(types(&s.recv(client(2, MsgType::Logout), h.t0)), ["Logout", "DISCONNECT"]);
    assert_eq!(s.peer().log.in_flight(), None, "logged out");

    let storage = Arc::new(MemoryStorage::new());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.t0);
    drop(s); // the connection drops
    let id =
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() };
    assert_eq!(storage.open(&id).unwrap().in_flight(), Some(3));
}

/// A step of a resend changes nothing stored, so it costs no commit.
#[test]
fn a_resend_step_costs_no_commit() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.set_resend_batch(1);
    s.on_message(&logon(1), h.t0);
    assert!(run_commit(&mut s, h.t0));
    s.on_message(&order(2, "A"), h.t0);
    assert!(run_commit(&mut s, h.t0));
    s.on_message(&resend_request(3, 1), h.t0);
    assert!(run_commit(&mut s, h.t0), "the ResendRequest's number");
    assert!(s.is_resending());
    while s.is_resending() {
        s.on_resume(h.t0);
        assert!(s.take_commit(h.t0).is_none(), "a resend step stores nothing");
    }
}

fn client_id() -> SessionId {
    SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() }
}

/// An acceptor's logon waits for the store to open the session's log: the session is claimed
/// meanwhile, takes no input and sends nothing, and answers once the log is open.
#[test]
fn a_logon_waits_for_the_store_to_open_the_log() {
    let storage = Arc::new(DeferringStorage::deferring_all());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    assert!(s.take_commit(h.t0).is_none(), "nothing to commit before the log is open");
    let open = s.take_open().expect("the store opens the log with a job");
    assert!(s.take_open().is_none(), "one opening");
    assert!(s.is_waiting_on_store());
    assert!(!s.ready_for_input());
    assert!(s.output().is_empty());
    assert_eq!(s.session_id(), None, "not bound yet");
    assert_eq!(h.registry.sessions(), [client_id()], "claimed meanwhile");

    s.on_opened(open.run(), h.t0);
    assert!(!s.is_waiting_on_store());
    s.commit_blocking(h.t0);
    assert_eq!(types(&taken(&mut s, false)), ["Logon"]);
    assert!(s.is_logged_on());
    assert_eq!(storage.calls.lock().unwrap()[0], format!("open {}", client_id()));
}

/// An initiator sends its Logon once the store has opened the session's log.
#[test]
fn an_initiator_sends_logon_once_the_log_is_open() {
    let h = Harness::with_storage(Arc::new(DeferringStorage::deferring_all()));
    let mut s = h.initiator(false);
    s.on_connect(h.t0);
    let open = s.take_open().expect("the store opens the log with a job");
    assert!(s.output().is_empty());
    s.on_opened(open.run(), h.t0);
    s.commit_blocking(h.t0);
    assert_eq!(types(&taken(&mut s, false)), ["Logon"]);
}

/// A failed opening refuses the logon and releases the session, which can log on again.
#[test]
fn a_failed_open_refuses_the_logon_and_releases_the_session() {
    let storage = Arc::new(DeferringStorage::deferring_all());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    let open = s.take_open().expect("the store opens the log with a job");
    *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("connection refused")));
    s.on_opened(open.run(), h.t0);
    assert!(s.is_closed());
    assert_eq!(types(&taken(&mut s, false)), ["DISCONNECT"]);
    assert!(h.registry.sessions().is_empty(), "released");

    *storage.job.lock().unwrap() = Arc::new(|| Ok(()));
    let mut again = h.session();
    again.on_message(&logon(1), h.t0);
    again.commit_blocking(h.t0);
    assert!(again.is_logged_on());
}

/// A session that closes, or is dropped, while its log opens releases the session: the log, once
/// open, is closed rather than bound.
#[test]
fn a_session_that_ends_while_its_log_opens_releases_it() {
    let h = Harness::with_storage(Arc::new(DeferringStorage::deferring_all()));
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    let open = s.take_open().expect("the store opens the log with a job");
    s.on_shutdown(None, h.t0);
    assert!(s.is_closed());
    s.on_opened(open.run(), h.t0);
    assert_eq!(s.session_id(), None, "never bound");
    assert!(h.registry.sessions().is_empty(), "released");
    assert!(!s.is_waiting_on_store());

    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    assert!(s.take_open().is_some());
    drop(s);
    assert!(h.registry.sessions().is_empty(), "released");
}

/// A logged-on session over a store whose resend reads are jobs, having sent two
/// ExecutionReports (our 2 and 3), and asked to resend from 1 one sequence number per step: the
/// first step's read is due.
fn resending_with_deferred_reads() -> (Arc<DeferringStorage>, Harness, Session) {
    let storage = Arc::new(DeferringStorage::deferring_all());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.logged_on();
    s.set_resend_batch(1);
    s.recv(order(2, "A"), h.t0);
    s.recv(order(3, "B"), h.t0);
    s.on_message(&resend_request(4, 1), h.t0);
    assert!(run_commit(&mut s, h.t0), "the ResendRequest's number");
    s.clear_output();
    (storage, h, s)
}

/// Each step of a resend waits for the store's read: nothing of it goes out, the session takes
/// no input, and resuming starts no second read, until the read's result is handed back.
#[test]
fn a_resend_step_waits_for_the_stores_read() {
    let (storage, h, mut s) = resending_with_deferred_reads();
    let mut resent = Vec::new();
    while s.is_resending() {
        assert!(s.take_commit(h.t0).is_none(), "a resend step stores nothing");
        let fetch = s.take_fetch().expect("each step is read by a job");
        assert!(s.is_waiting_on_store());
        assert!(s.output().is_empty(), "nothing of the step goes out before its read");
        s.on_resume(h.t0);
        assert!(s.take_fetch().is_none(), "one read at a time");
        s.on_fetched(fetch.run(), h.t0);
        assert!(!s.is_waiting_on_store());
        assert!(s.take_commit(h.t0).is_none(), "a resend step stores nothing");
        resent.extend(covered(&taken(&mut s, false)));
        s.clear_output();
        s.on_resume(h.t0);
    }
    assert_eq!(resent, [1, 2, 3], "the Logon gap-filled, both reports resent");
    let fetches: Vec<String> =
        storage.calls.lock().unwrap().iter().filter(|c| c.starts_with("fetch")).cloned().collect();
    assert_eq!(fetches, ["fetch 1..=1", "fetch 2..=2", "fetch 3..=3"]);
}

/// A read that fails is a store failure: the session drops the resend and disconnects.
#[test]
fn a_failed_read_closes_the_session() {
    let (storage, h, mut s) = resending_with_deferred_reads();
    *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("connection reset")));
    let fetch = s.take_fetch().expect("the first step's read");
    s.on_fetched(fetch.run(), h.t0);
    assert!(s.is_closed());
    assert!(!s.is_resending());
    assert_eq!(types(&taken(&mut s, false)), ["DISCONNECT"]);
}

/// A read that ends after its resend did (the session logged out meanwhile) is ignored.
#[test]
fn a_read_after_its_resend_ended_is_ignored() {
    let (_, h, mut s) = resending_with_deferred_reads();
    let fetch = s.take_fetch().expect("the first step's read");
    s.on_shutdown(None, h.t0);
    s.commit_blocking(h.t0);
    assert!(!s.is_resending());
    assert_eq!(types(&taken(&mut s, false)), ["Logout"]);
    s.clear_output();
    s.on_fetched(fetch.run(), h.t0);
    assert!(s.output().is_empty(), "nothing of the resend goes out");
    assert!(!s.is_waiting_on_store());
}

/// Regression: a store that makes each change as it's made (as `MemoryStorage` does) still has
/// the window if the process stops part-way through a batch. Recording the next incoming number
/// used to clear it, so the message being handled when it stopped came back unmarked.
#[test]
fn a_batch_cut_short_leaves_its_window_in_a_store_without_commits() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert!(s.ready_for_input());
    s.on_message(&order(2, "A"), h.t0);
    s.on_message(&order(3, "B"), h.t0);
    // The process stops before the batch's commit.
    let log = &s.peer().log;
    assert_eq!((log.next_incoming(), log.in_flight()), (4, Some(2)));
    s.commit_blocking(h.t0);
    assert_eq!(s.peer().log.in_flight(), Some(4), "the batch's end records the next window");
}

#[test]
fn a_failed_commit_sends_nothing_it_covered_and_disconnects() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("disk full")));
    assert!(run_commit(&mut s, h.t0));
    assert!(s.output().is_empty() && s.is_closed());
}

#[test]
fn messages_queued_behind_a_gap_are_handled_a_window_at_a_time() {
    let writes = Arc::new(Mutex::new(Vec::new()));
    let h = Harness::with_storage(Arc::new(RecordingStorage { inner: MemoryStorage::new(), writes: writes.clone() }));
    let mut s = h.logged_on();
    let last = 2 + DELIVERIES_PER_COMMIT + 10;
    for seq in 3..=last {
        s.recv(order(seq, &format!("O{seq}")), h.t0);
    }
    writes.lock().unwrap().clear();
    s.recv(order(2, "first"), h.t0);
    assert_eq!(h.app.received(), usize::try_from(last - 1).unwrap(), "all of them, in sequence");
    let mut windows: Vec<String> =
        writes.lock().unwrap().iter().filter(|w| w.starts_with("in flight")).cloned().collect();
    windows.dedup();
    let expected = [2, 2 + DELIVERIES_PER_COMMIT, last + 1].map(|start| format!("in flight {start}"));
    assert_eq!(windows, expected, "one exhausted mid-batch, then the next batch's");
}

#[test]
fn a_receipt_waits_for_its_message_to_be_committed() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    assert!(run_commit(&mut s, h.t0));
    let order = || Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "1");

    let (receipt, mut stored) = tokio::sync::oneshot::channel();
    s.on_command(Command::Send(order(), Some(receipt)), h.t0);
    assert!(stored.try_recv().is_err(), "stored, but not committed yet");
    assert!(run_commit(&mut s, h.t0));
    assert_eq!(stored.try_recv().unwrap(), Ok(2));

    let (receipt, mut failed) = tokio::sync::oneshot::channel();
    s.on_command(Command::Send(order(), Some(receipt)), h.t0);
    *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("disk full")));
    assert!(run_commit(&mut s, h.t0));
    assert_eq!(failed.try_recv().unwrap(), Err(Dropped::Storage), "it may have been stored");
}

/// Regression (simulator): once its store has failed, a session that's still bound (its output
/// draining) commits nothing more, so an operator's change there would never be stored; it's
/// refused rather than reported as made.
#[test]
fn an_operator_change_after_the_store_failed_is_refused() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    *storage.job.lock().unwrap() = Arc::new(|| Err(io::Error::other("disk full")));
    assert!(run_commit(&mut s, h.t0));
    assert!(s.is_closed());
    let (reply, mut answer) = tokio::sync::oneshot::channel();
    s.on_command(Command::Sequence(SequenceCommand::SetNextOutgoing(50), reply), h.t0);
    s.commit_blocking(h.t0);
    assert!(matches!(answer.try_recv().unwrap(), Err(SequenceError::Storage(_))));
}

#[test]
fn an_operator_hears_of_a_change_once_it_is_committed() {
    let storage = Arc::new(DeferringStorage::default());
    let h = Harness::with_storage(storage.clone());
    let mut s = h.session();
    s.on_message(&logon(1), h.t0);
    assert!(run_commit(&mut s, h.t0));
    let (reply, mut answer) = tokio::sync::oneshot::channel();
    s.on_command(Command::Sequence(SequenceCommand::SetNextIncoming(7), reply), h.t0);
    assert!(answer.try_recv().is_err(), "not committed yet");
    assert!(run_commit(&mut s, h.t0));
    assert_eq!(answer.try_recv().unwrap().unwrap().next_incoming, 7);
}

/// Regression (simulator): a window recovered after a restart stays recorded until messages in it
/// can no longer come back, so if the process stops again first, the next restart still marks
/// them. Closing each batch's window used to clear it.
#[test]
fn a_recovered_window_outlives_the_batches_before_its_messages_return() {
    let storage = Arc::new(MemoryStorage::new());
    {
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GATEWAY".into(),
            target_comp_id: "CLIENT".into(),
        };
        let mut log = storage.open(&id).unwrap();
        log.record_outgoing(1, None).unwrap();
        log.set_next_incoming(2).unwrap();
        log.set_in_flight(2).unwrap();
    }
    let h = Harness::with_storage(storage);
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(4), h.t0)), ["Logon", "ResendRequest"]);
    s.recv(resend_of(client(2, MsgType::Heartbeat)), h.t0);
    assert_eq!(s.peer().log.in_flight(), Some(2), "3 may still come back, and may have been handled");
}

// ---- Messages ahead of a gap ----

fn gap_fill(seq: u64, new_seq_no: u64) -> Message {
    resend_of(client(seq, MsgType::SequenceReset)).with(tags::GAP_FILL_FLAG, "Y").with(tags::NEW_SEQ_NO, new_seq_no)
}

/// Session test case 1a: a Logout that arrives during a gap is answered at once rather than
/// queued behind it (the counterparty is leaving, so it's unlikely to resend first).
#[test]
fn a_logout_during_a_gap_is_answered_at_once() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(5, "E"), h.t0)), ["ResendRequest"]);
    assert_eq!(types(&s.recv(client(6, MsgType::Logout), h.t0)), ["Logout", "DISCONNECT"]);
    assert_eq!(h.app.received(), 0);
}

#[test]
fn a_resend_request_ahead_of_a_gap_is_answered_once() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our 1: Logon
    s.recv(order(2, "A"), h.t0); // our 2: ExecutionReport
    let request = client(4, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "2").with(tags::END_SEQ_NO, "0");
    // Answered now, so both sides can recover from a mutual gap, and our own request goes out.
    assert_eq!(types(&s.recv(request, h.t0)), ["ExecutionReport", "ResendRequest"]);
    // Filling our gap reaches the queued ResendRequest: it takes its number, and isn't answered again.
    assert!(s.recv(gap_fill(3, 4), h.t0).is_empty());
    assert_eq!(s.peer().log.next_incoming(), 5);
}

/// A message lost after the gap was noticed leaves a hole once the first resend completes; it
/// gets its own ResendRequest, rather than waiting for the next message to reveal it.
#[test]
fn a_hole_behind_the_gap_gets_its_own_resend_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(4, "D"), h.t0); // ResendRequest from 2
    s.recv(order(6, "F"), h.t0); // queued; 5 is lost
    let out = s.recv(gap_fill(2, 4), h.t0);
    assert_eq!(types(&out), ["ExecutionReport", "ResendRequest"]);
    assert_eq!(sent(&out)[1].get(tags::BEGIN_SEQ_NO), Some("5"));
    assert_eq!(types(&s.recv(resend_of(order(5, "E")), h.t0)), ["ExecutionReport", "ExecutionReport"]);
    assert!(s.resend.is_none());
    assert_eq!(s.peer().log.next_incoming(), 7);
}

/// A gap fill past queued messages makes them duplicates: they're dropped.
#[test]
fn a_gap_fill_past_queued_messages_discards_them() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(4, "D"), h.t0);
    assert!(s.recv(gap_fill(2, 5), h.t0).is_empty());
    assert_eq!(h.app.received(), 0);
    assert_eq!(s.peer().log.next_incoming(), 5);
    assert!(s.queued.is_empty());
}

/// The queue is bounded; what's dropped comes back with the open-ended resend.
#[test]
fn the_queue_is_bounded() {
    let h = Harness::new();
    let mut s = h.logged_on();
    for seq in 3..3 + MAX_QUEUED as u64 + 10 {
        s.recv(client(seq, MsgType::Heartbeat), h.t0);
    }
    assert_eq!(s.queued.len(), MAX_QUEUED);
    s.recv(client(2, MsgType::Heartbeat), h.t0);
    assert_eq!(s.peer().log.next_incoming(), 3 + MAX_QUEUED as u64);
}

#[test]
fn seq_num_too_low_logs_out_unless_poss_dup() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Heartbeat), h.t0);
    assert!(s.recv(resend_of(client(2, MsgType::Heartbeat)), h.t0).is_empty());
    let out = s.recv(client(2, MsgType::Heartbeat), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert!(sent(&out)[0].get(tags::TEXT).unwrap().contains("too low"));
}

#[test]
fn resend_request_replays_app_messages_and_gap_fills_admin() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our seq 1: Logon
    s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.t0); // our 2: Heartbeat
    let er = s.recv(order(3, "A"), h.t0); // our 3: ExecutionReport
    let original = sent(&er)[0].clone();
    s.recv(client(4, MsgType::TestRequest).with(tags::TEST_REQ_ID, "y"), h.t0); // our 4: Heartbeat

    let req = client(5, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0");
    let out = s.recv(req, h.t0);
    let msgs = sent(&out);
    assert_eq!(msgs.len(), 3);

    assert_eq!(msgs[0].msg_type(), MsgType::SequenceReset);
    assert_eq!(msgs[0].get(tags::MSG_SEQ_NUM), Some("1"));
    assert_eq!(msgs[0].get(tags::NEW_SEQ_NO), Some("3"));
    assert_eq!(msgs[0].get(tags::GAP_FILL_FLAG), Some("Y"));

    assert_eq!(msgs[1].msg_type(), MsgType::ExecutionReport);
    assert_eq!(msgs[1].get(tags::MSG_SEQ_NUM), Some("3"));
    assert_eq!(msgs[1].get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(msgs[1].get(tags::ORIG_SENDING_TIME), original.get(tags::SENDING_TIME));
    assert_eq!(msgs[1].get(tags::EXEC_ID), original.get(tags::EXEC_ID));
    // PossDupFlag belongs in the header, ahead of any body field.
    let pos = |m: &Message, tag| m.fields().position(|(t, _)| t == tag).unwrap();
    assert!(pos(msgs[1], tags::POSS_DUP_FLAG) < pos(msgs[1], tags::CL_ORD_ID));

    assert_eq!(msgs[2].get(tags::MSG_SEQ_NUM), Some("4"));
    assert_eq!(msgs[2].get(tags::NEW_SEQ_NO), Some("5"));

    // Resending does not consume new sequence numbers.
    let out = s.recv(client(6, MsgType::TestRequest).with(tags::TEST_REQ_ID, "z"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("5"));
}

/// A logged-on acceptor that has sent `n` ExecutionReports after its Logon (our MsgSeqNum 2 to
/// n + 1), resending at most `batch` sequence numbers per step.
fn with_reports(h: &Harness, n: u64, batch: u64) -> Session {
    let mut s = h.logged_on();
    s.resend_batch = batch;
    for i in 0..n {
        s.recv(order(i + 2, &format!("O{i}")), h.t0);
    }
    s
}

fn resend_request(seq: u64, begin: u64) -> Message {
    client(seq, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, begin).with(tags::END_SEQ_NO, "0")
}

/// Messages the application declines to resend are gap-filled, a run of them together with what
/// wasn't stored.
#[test]
fn messages_the_application_declines_to_resend_are_gap_filled() {
    let mut h = Harness::new();
    h.app = Arc::new(TestApp { skip_resend: vec!["O1", "O2"], ..TestApp::default() });
    let mut s = with_reports(&h, 4, 100); // our 2 to 5: ExecutionReports for O0 to O3
    let out = s.recv(resend_request(6, 1), h.t0);
    let msgs = sent(&out);
    let summary: Vec<_> = msgs
        .iter()
        .map(|m| match m.msg_type() {
            MsgType::SequenceReset => {
                format!("fill {}..{}", m.get(tags::MSG_SEQ_NUM).unwrap(), m.get(tags::NEW_SEQ_NO).unwrap())
            }
            _ => format!("resend {}", m.get(tags::CL_ORD_ID).unwrap()),
        })
        .collect();
    assert_eq!(summary, ["fill 1..2", "resend O0", "fill 3..5", "resend O3"]);
    assert_eq!(*h.app.resend_asked.lock().unwrap(), ["O0", "O1", "O2", "O3"], "asked once each, not about the Logon");
}

#[test]
fn a_panic_in_should_resend_resends_the_message() {
    let h = panicking("should_resend");
    let mut s = with_reports(&h, 1, 100);
    let out = s.recv(resend_request(3, 2), h.t0);
    assert_eq!(types(&out), ["ExecutionReport"]);
}

/// The sequence numbers each message sent covers: its own, or a gap fill's whole range.
fn covered(actions: &[Action]) -> Vec<u64> {
    let seq = |m: &Message, tag| m.get(tag).unwrap().parse::<u64>().unwrap();
    sent(actions)
        .into_iter()
        .flat_map(|m| match m.msg_type() {
            MsgType::SequenceReset => seq(m, tags::MSG_SEQ_NUM)..seq(m, tags::NEW_SEQ_NO),
            _ => seq(m, tags::MSG_SEQ_NUM)..seq(m, tags::MSG_SEQ_NUM) + 1,
        })
        .collect()
}

/// A harness whose store keeps two of [`with_reports`]' ExecutionReports, which are all the same
/// length.
fn keeping_two_reports() -> Harness {
    Harness::with_storage(Arc::new(MemoryStorage::new().with_max_session_bytes(2 * report_len())))
}

/// The length of each of [`with_reports`]' ExecutionReports, which are all the same.
fn report_len() -> usize {
    let mut probe = with_reports(&Harness::new(), 4, 256);
    let lens: Vec<usize> = probe.peer_mut().log.sent_messages(2, 5).unwrap().iter().map(|(_, m)| m.len()).collect();
    assert!(lens.iter().all(|len| *len == lens[0]), "{lens:?}");
    lens[0]
}

#[test]
fn a_resend_gap_fills_messages_the_store_evicted() {
    let h = keeping_two_reports();
    let mut s = with_reports(&h, 4, 256); // our 1: Logon, 2 to 5: ExecutionReports, 2 and 3 evicted
    assert_eq!(s.peer().log.evicted_through(), Some(3));

    let out = s.recv(resend_request(6, 1), h.t0);
    assert_eq!(types(&out), ["SequenceReset", "ExecutionReport", "ExecutionReport"]);
    assert_eq!(covered(&out), [1, 2, 3, 4, 5]);
    assert_eq!(sent(&out)[1].get(tags::MSG_SEQ_NUM), Some("4"));
}

/// The same with a disk store, whose oldest segments go past its budget.
#[test]
fn a_resend_gap_fills_messages_the_disk_store_evicted() {
    let dir = tempfile::tempdir().unwrap();
    // A report and the record of the commit that stores it.
    let len = u64::try_from(report_len() + crate::store::disk::JOURNAL_RECORD).unwrap();
    let disk =
        crate::DiskStorage::new(dir.path(), false).unwrap().with_max_session_bytes(2 * len).with_segment_bytes(len);
    let h = Harness::with_storage(Arc::new(disk));
    let mut s = with_reports(&h, 4, 256); // one report a segment; 2 and 3 deleted with theirs
    assert_eq!(s.peer().log.evicted_through(), Some(3));

    let out = s.recv(resend_request(6, 1), h.t0);
    assert_eq!(types(&out), ["SequenceReset", "ExecutionReport", "ExecutionReport"]);
    assert_eq!(covered(&out), [1, 2, 3, 4, 5]);
}

#[test]
fn a_long_resend_goes_out_in_steps() {
    let h = Harness::new();
    let mut s = with_reports(&h, 10, 4); // our 1: Logon, 2 to 11: ExecutionReports

    let first = s.recv(resend_request(12, 1), h.t0);
    assert_eq!(types(&first), ["SequenceReset", "ExecutionReport", "ExecutionReport", "ExecutionReport"]);
    assert_eq!(covered(&first), [1, 2, 3, 4]);
    assert!(s.is_resending());

    let second = s.resume(h.t0);
    assert_eq!(covered(&second), [5, 6, 7, 8]);
    assert!(s.is_resending());
    let third = s.resume(h.t0);
    assert_eq!(covered(&third), [9, 10, 11]);
    assert!(!s.is_resending());
    for er in sent(&first).into_iter().skip(1).chain(sent(&second)).chain(sent(&third)) {
        assert_eq!(er.get(tags::POSS_DUP_FLAG), Some("Y"));
        assert!(er.get(tags::ORIG_SENDING_TIME).is_some());
    }

    // Nothing more to resend, and new messages carry on from 12.
    assert!(s.resume(h.t0).is_empty());
    let out = s.recv(client(13, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("12"));
}

/// A step that resends nothing gap-fills its numbers then, rather than leave them to the next
/// message resent: through a long run, the counterparty sees progress each step.
#[test]
fn each_step_gap_fills_what_it_does_not_resend() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.resend_batch = 4;
    for seq in 2..=9 {
        s.recv(client(seq, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.t0); // our 2 to 9: Heartbeats
    }
    s.recv(order(10, "A"), h.t0); // our 10: ExecutionReport

    let first = s.recv(resend_request(11, 1), h.t0);
    assert_eq!(covered(&first), [1, 2, 3, 4], "1 to 4 are all session messages");
    assert_eq!(types(&first), ["SequenceReset"]);
    assert_eq!(covered(&s.resume(h.t0)), [5, 6, 7, 8], "so are 5 to 8");
    let last = s.resume(h.t0);
    assert_eq!(types(&last), ["SequenceReset", "ExecutionReport"]);
    assert_eq!(sent(&last)[0].get(tags::NEW_SEQ_NO), Some("10"));
    assert!(!s.is_resending());
}

#[test]
fn a_resend_within_one_step_finishes_in_the_call() {
    let h = Harness::new();
    let mut s = with_reports(&h, 2, 4);
    assert_eq!(covered(&s.recv(resend_request(4, 1), h.t0)), [1, 2, 3]);
    assert!(!s.is_resending());
}

#[test]
fn each_step_sends_at_most_one_batch() {
    let h = Harness::new();
    let mut s = with_reports(&h, 100, 4);
    let mut all = covered(&s.recv(resend_request(102, 1), h.t0));
    while s.is_resending() {
        let step = s.resume(h.t0);
        assert!(sent(&step).len() <= 4, "{} messages in one step", sent(&step).len());
        all.extend(covered(&step));
    }
    assert_eq!(all, (1..=101).collect::<Vec<_>>());
}

/// Messages sent while a resend is in progress wait for it, so the counterparty sees the range it
/// asked for before any new sequence number.
#[test]
fn a_resend_request_sent_while_resending_follows_the_resend() {
    let h = Harness::new();
    let mut s = with_reports(&h, 6, 4); // our 2 to 7; their next is 8
    // Ahead of a gap: answered now, and our own ResendRequest goes out too.
    let first = s.recv(resend_request(9, 1), h.t0);
    assert_eq!(covered(&first), [1, 2, 3, 4]);
    let last = s.resume(h.t0);
    assert_eq!(types(&last), ["ExecutionReport", "ExecutionReport", "ExecutionReport", "ResendRequest"]);
    assert_eq!(sent(&last)[3].get(tags::MSG_SEQ_NUM), Some("8"));
}

#[test]
fn messages_delivered_while_resending_follow_the_resend() {
    let h = Harness::new();
    let mut s = with_reports(&h, 6, 4); // our 2 to 7; their next is 8
    assert_eq!(types(&s.recv(order(9, "Q"), h.t0)), ["ResendRequest"]); // our 8; 9 is queued
    // Their 8 fills the gap and asks for everything: the queued order is answered after it.
    let first = s.recv(resend_request(8, 1), h.t0);
    assert_eq!(covered(&first), [1, 2, 3, 4]);
    let last = s.resume(h.t0);
    assert_eq!(covered(&last), [5, 6, 7, 8, 9]);
    let ack = *sent(&last).last().unwrap();
    assert_eq!((ack.get(tags::CL_ORD_ID), ack.get(tags::POSS_DUP_FLAG)), (Some("Q"), None));
}

#[test]
fn commands_queued_during_logon_follow_a_resend_at_logon() {
    let h = Harness::new();
    let configure = |config: &mut InitiatorConfig| config.next_expected_msg_seq_num = true;
    let mut first = h.initiator_with(configure);
    first.connect(h.t0); // our 1: Logon
    first.recv(logon(1), h.t0);
    for id in ["A", "B", "C", "D", "E", "F"] {
        first.command(send_command(id), h.t0); // our 2 to 7
    }
    drop(first);

    let mut s = h.initiator_with(configure);
    s.resend_batch = 2;
    s.connect(h.t0); // our 8: Logon
    s.command(send_command("Z"), h.t0);
    let ids =
        |out: &[Action]| sent(out).iter().map(|m| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect::<Vec<_>>();
    assert_eq!(ids(&s.recv(logon(2).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "2"), h.t0)), ["A", "B"]);
    assert_eq!(ids(&s.resume(h.t0)), ["C", "D"]);
    let last = s.resume(h.t0);
    assert_eq!(ids(&last), ["E", "F", "Z"]);
    assert_eq!(sent(&last)[2].get(tags::MSG_SEQ_NUM), Some("9"));
}

#[test]
fn a_corrupt_message_in_a_later_step_disconnects() {
    let storage = Arc::new(MemoryStorage::new());
    let h = Harness::with_storage(storage.clone());
    drop(with_reports(&h, 4, 4)); // our 1: Logon, 2 to 5: ExecutionReports; their next is 6
    store_corrupt_report(&storage, 6);
    let mut s = h.session();
    s.resend_batch = 4;
    s.recv(logon(6), h.t0); // our 7: Logon
    assert_eq!(covered(&s.recv(resend_request(7, 1), h.t0)), [1, 2, 3, 4]);
    // The step with 6 in it sends none of itself, 5 included.
    assert_eq!(types(&s.resume(h.t0)), ["DISCONNECT"]);
    assert!(!s.is_resending());
}

#[test]
fn shutdown_during_a_resend_sends_what_was_held_then_logs_out() {
    let h = Harness::new();
    let mut s = with_reports(&h, 6, 4); // our 2 to 7; their next is 8
    s.recv(resend_request(9, 1), h.t0); // ahead of a gap: our ResendRequest, 8, is held
    let out = s.shutdown(Some("bye"), h.t0);
    assert_eq!(types(&out), ["ResendRequest", "Logout"]);
    assert_eq!(sent(&out)[1].get(tags::MSG_SEQ_NUM), Some("9"));
    assert!(!s.is_resending());
    assert!(s.resume(h.t0).is_empty());
}

/// Drivers feed no messages during a resend, but one that does gets a session that still keeps
/// the order: a ResendRequest replaces the resend in progress.
#[test]
fn a_resend_request_during_a_resend_replaces_it() {
    let h = Harness::new();
    let mut s = with_reports(&h, 10, 4); // our 2 to 11; their next is 12
    assert_eq!(covered(&s.recv(resend_request(13, 1), h.t0)), [1, 2, 3, 4]); // our 12 is held
    // Their 12 fills the gap and asks again from 9: our held ResendRequest isn't needed.
    let out = s.recv(resend_request(12, 9), h.t0);
    assert_eq!(covered(&out), [9, 10, 11, 12]);
    assert_eq!(types(&out), ["ExecutionReport", "ExecutionReport", "ExecutionReport", "SequenceReset"]);
    assert!(!s.is_resending());
}

#[test]
fn a_sequence_reset_during_a_resend_ends_it() {
    let h = Harness::new();
    let mut s = with_reports(&h, 6, 4); // our 2 to 7; their next is 8
    s.recv(resend_request(9, 1), h.t0); // our ResendRequest, 8, is held
    let reset = logon(1).with(tags::RESET_SEQ_NUM_FLAG, "Y");
    let out = s.recv(reset, h.t0);
    assert_eq!(types(&out), ["Logon"]);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("1"));
    assert!(!s.is_resending());
}

/// The counterparty's input waits while we resend, so its silence then isn't a sign of trouble,
/// and we're sending anyway: no Heartbeat, TestRequest or timeout until the resend ends.
#[test]
fn timers_wait_for_a_resend_to_end() {
    let h = Harness::new();
    let mut s = with_reports(&h, 10, 4); // HeartBtInt 30
    s.recv(resend_request(12, 1), h.t0);
    assert_eq!(s.next_deadline(), None);
    assert!(s.timer(h.at(100)).is_empty());

    s.resume(h.at(100));
    assert!(!s.resume(h.at(100)).is_empty() && !s.is_resending());
    // Silence counts from the end of the resend: the next thing due is a Heartbeat, 30 s on.
    assert_eq!(s.next_deadline(), Some(h.at(130)));
    assert!(s.timer(h.at(129)).is_empty());
    assert_eq!(types(&s.timer(h.at(130))), ["Heartbeat"]);
}

// ---- Application messages and rejects ----

#[test]
fn application_session_reject_names_the_field() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let bad = client(2, MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "A");
    let out = s.recv(bad, h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("55"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("1"));
    // The rejected message still consumed its sequence number.
    assert!(s.recv(client(3, MsgType::Heartbeat), h.t0).is_empty());
}

#[test]
fn empty_value_is_rejected_before_the_application_sees_it() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(order(2, "A").with(tags::TEXT, ""), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("58"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("4"));
    assert_eq!(h.app.received(), 0);
    // The rejected message still consumed its sequence number.
    assert_eq!(types(&s.recv(order(3, "B"), h.t0)), ["ExecutionReport"]);
}

fn panicking(callback: &'static str) -> Harness {
    let mut h = Harness::new();
    h.app = Arc::new(TestApp { panic_in: Some(callback), ..TestApp::default() });
    h
}

#[test]
fn application_panic_is_answered_with_a_business_reject() {
    let h = panicking("on_message");
    let mut s = h.logged_on();
    let out = s.recv(order(2, "PANIC"), h.t0);
    assert_eq!(types(&out), ["BusinessMessageReject"], "replies queued before the panic are discarded");
    let reject = sent(&out)[0];
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(reject.get(tags::REF_MSG_TYPE), Some("D"));
    assert_eq!(reject.get(tags::BUSINESS_REJECT_REASON), Some("4"), "application not available");
    // The session carries on.
    assert_eq!(types(&s.recv(order(3, "A"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn panic_in_verify_logon_refuses_the_logon() {
    let h = panicking("verify_logon");
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
    assert!(h.registry.sessions().is_empty());
}

#[test]
fn panic_in_on_logon_leaves_the_session_logged_on() {
    let h = panicking("on_logon");
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
    assert!(s.is_logged_on());
    assert_eq!(types(&s.recv(order(2, "A"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn panic_in_on_logout_still_disconnects() {
    let h = panicking("on_logout");
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::Logout), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT CounterpartyLogout"]);
}

#[test]
fn panic_in_on_admin_message_is_only_logged() {
    let h = panicking("on_admin_message");
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "T"), h.t0);
    assert_eq!(types(&out), ["Heartbeat"]);
    assert!(s.is_logged_on());
}

#[test]
fn panic_in_to_admin_disconnects() {
    let h = panicking("to_admin");
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "T"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"], "the Heartbeat isn't sent half-modified");
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Error"]);
}

#[test]
fn unsupported_message_type_gets_business_reject() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::from_code("G")), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::BusinessMessageReject);
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(reject.get(tags::REF_MSG_TYPE), Some("G"));
    assert_eq!(reject.get(tags::BUSINESS_REJECT_REASON), Some("3"));
}

#[test]
fn comp_id_mismatch_rejects_and_logs_out() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::Heartbeat).with(tags::SENDER_COMP_ID, "EVIL"), h.t0);
    assert_eq!(types(&out), ["Reject", "Logout", "DISCONNECT"]);
}

/// Session test case 2i: a wrong BeginString ends the session with a Logout, and no Reject.
#[test]
fn wrong_begin_string_logs_out() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::Heartbeat).with(tags::BEGIN_STRING, "FIX.4.2"), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert!(sent(&out)[0].get(tags::TEXT).unwrap().contains("BeginString"));
}

/// A heartbeat from the counterparty with header field `without` left out.
fn heartbeat_without(seq: u64, without: u32) -> Message {
    let full = client(seq, MsgType::Heartbeat);
    let mut msg = Message::default();
    for (tag, value) in full.fields().filter(|(tag, _)| *tag != without) {
        msg.push(tag, value);
    }
    msg
}

/// Session test case 14b: a missing required header field is rejected like any other, naming
/// it, and the session carries on.
#[test]
fn missing_header_fields_are_rejected() {
    for tag in [tags::TARGET_COMP_ID, tags::SENDER_COMP_ID, tags::SENDING_TIME] {
        let h = Harness::new();
        let mut s = h.logged_on();
        let out = s.recv(heartbeat_without(2, tag), h.t0);
        assert_eq!(types(&out), ["Reject"], "without {tag}");
        let reject = sent(&out)[0];
        assert_eq!(reject.get(tags::REF_TAG_ID), Some(tag.to_string().as_str()));
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("1"));
        assert!(s.recv(client(3, MsgType::Heartbeat), h.t0).is_empty(), "the session didn't carry on");
    }
}

/// Session test case 14d: an empty TargetCompID is a tag without a value, not a CompID problem.
#[test]
fn empty_target_comp_id_is_rejected_as_without_a_value() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::Heartbeat).with(tags::TARGET_COMP_ID, ""), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("56"));
    assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("4"));
}

/// Each way a logged-on session ends reaches `on_logout` with its own reason.
#[test]
fn on_logout_says_how_the_session_ended() {
    type End = fn(&Harness, &mut Session);
    let cases: [(&str, End, Disconnect); 8] = [
        (
            "counterparty logs out",
            |h, s| _ = s.recv(client(2, MsgType::Logout), h.at(1)),
            Disconnect::CounterpartyLogout,
        ),
        (
            "we log out, they answer",
            |h, s| {
                s.command(Command::Logout(None), h.at(1));
                s.recv(client(2, MsgType::Logout), h.at(2));
            },
            Disconnect::Logout,
        ),
        (
            "we log out, no answer",
            |h, s| {
                s.command(Command::Logout(None), h.at(1));
                s.timer(h.at(6));
            },
            Disconnect::Logout,
        ),
        (
            "we log out, they close the connection",
            |h, s| {
                s.command(Command::Logout(None), h.at(1));
                s.on_disconnect(h.at(2));
            },
            Disconnect::Logout,
        ),
        // A TestRequest after 36 s of silence (HeartBtInt 30 plus 20%), unanswered 30 s later.
        (
            "TestRequest unanswered",
            |h, s| {
                s.timer(h.at(36));
                s.timer(h.at(66));
            },
            Disconnect::HeartbeatTimeout,
        ),
        ("MsgSeqNum too low", |h, s| _ = s.recv(client(1, MsgType::Heartbeat), h.at(1)), Disconnect::Error),
        (
            "shutdown",
            |h, s| {
                s.shutdown(None, h.at(1));
                s.recv(client(2, MsgType::Logout), h.at(2));
            },
            Disconnect::Shutdown,
        ),
        ("transport ends", |h, s| s.on_disconnect(h.at(1)), Disconnect::ConnectionLost),
    ];
    for (name, end, reason) in cases {
        let h = Harness::new();
        let mut s = h.logged_on();
        end(&h, &mut s);
        assert_eq!(h.app.events(), ["logon CLIENT".to_string(), format!("logout CLIENT {reason:?}")], "{name}");
    }
}

/// A session dropped while logged on (its connection's task ended) lost its connection.
#[test]
fn a_dropped_session_lost_its_connection() {
    let h = Harness::new();
    drop(h.logged_on());
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost"]);
}

/// A session dropped while it logs out (shutdown gave up waiting) keeps the logout's reason.
#[test]
fn a_session_dropped_while_logging_out_keeps_the_reason() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.shutdown(None, h.t0);
    drop(s);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Shutdown"]);
}

/// A logout's reason gives way to an explicit close: an error during a shutdown logout is an
/// error.
#[test]
fn an_error_close_during_a_logout_reports_the_error() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.shutdown(None, h.t0);
    let out = s.recv(client(2, MsgType::Heartbeat).with(tags::BEGIN_STRING, "FIX.4.2"), h.at(1));
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Error"]);
}

/// The first logout's reason stands: a second logout over an error, while waiting for the reply
/// to ours, doesn't change how the session ended.
#[test]
fn the_first_logouts_reason_stands() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.command(Command::Logout(None), h.t0);
    // A stale SendingTime is rejected and logged out over, without closing.
    let stale = client(2, MsgType::Heartbeat).with(tags::SENDING_TIME, "20010101-00:00:00");
    assert_eq!(types(&s.recv(stale, h.at(1))), ["Reject", "Logout"]);
    assert_eq!(types(&s.recv(client(3, MsgType::Logout), h.at(2))), ["DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Logout"]);
}

/// The transport ending once the session has closed changes nothing: it was already reported.
#[test]
fn a_disconnect_after_close_is_not_reported_again() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Logout), h.t0);
    s.on_disconnect(h.at(1));
    drop(s);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT CounterpartyLogout"]);
}

#[test]
fn counterparty_logout_is_acknowledged() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::Logout), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT CounterpartyLogout"]);
}

// ---- Handle commands ----

/// A logged-on acceptor whose send queue holds `capacity` messages, its command queues, and a
/// handle on it.
fn with_send_queue(h: &mut Harness, capacity: usize) -> (Session, CommandReceiver, SessionHandle) {
    h.config.send_queue = capacity;
    let (mut s, commands) = Session::acceptor(h.config.clone(), h.registry.clone(), h.app.clone(), h.t0);
    s.recv(logon(1), h.t0);
    let handle = h.app.handles.lock().unwrap()[0].clone();
    (s, commands, handle)
}

fn report(id: &str) -> Message {
    Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, id)
}

#[test]
fn a_full_send_queue_hands_the_message_back() {
    let mut h = Harness::new();
    let (_s, mut commands, handle) = with_send_queue(&mut h, 2);
    handle.send(report("A")).unwrap();
    handle.send(report("B")).unwrap();
    let Err(SendError::Full(msg)) = handle.send(report("C")) else { panic!("queued past the limit") };
    assert_eq!(msg.get(tags::EXEC_ID), Some("C"));
    // Taking one makes room for one.
    assert!(commands.try_send().is_some());
    handle.send(msg).unwrap();
    assert!(matches!(handle.send(report("D")), Err(SendError::Full(_))));
}

/// A logout has its own queue: a full send queue doesn't hold it up, but it waits for the sends
/// queued before it, so they still go out first.
#[test]
fn a_logout_waits_for_the_sends_before_it_not_for_room() {
    let mut h = Harness::new();
    let (mut s, mut commands, handle) = with_send_queue(&mut h, 2);
    handle.send(report("A")).unwrap();
    handle.send(report("B")).unwrap();
    handle.logout(Some("done")).unwrap();
    assert!(commands.try_control().is_none(), "A and B first");
    let mut out = Vec::new();
    while let Some(command) = commands.try_send() {
        out.extend(s.command(command, h.t0));
    }
    out.extend(s.command(commands.try_control().expect("now due"), h.t0));
    assert_eq!(types(&out), ["ExecutionReport", "ExecutionReport", "Logout"]);
}

/// Operator commands aren't held up by a logout waiting for sends.
#[test]
fn operator_commands_pass_a_waiting_logout() {
    let mut h = Harness::new();
    let (mut s, mut commands, handle) = with_send_queue(&mut h, 2);
    handle.send(report("A")).unwrap();
    handle.logout(None).unwrap();
    let mut numbers = std::pin::pin!(handle.sequence_numbers());
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(numbers.as_mut().poll(&mut cx).is_pending());
    let command = commands.try_control().expect("the operator's command, ahead of the logout");
    assert!(matches!(command, Command::Sequence(SequenceCommand::Get, _)));
    s.command(command, h.t0);
    let std::task::Poll::Ready(Ok(answer)) = numbers.as_mut().poll(&mut cx) else { panic!("unanswered") };
    assert_eq!(answer.next_outgoing, 2);
    assert!(commands.try_control().is_none(), "the logout still waits for A");
}

#[test]
fn handle_commands_reach_the_session() {
    let h = Harness::new();
    let (mut s, mut commands) = Session::acceptor(h.config.clone(), h.registry.clone(), h.app.clone(), h.t0);
    s.recv(logon(1), h.t0);

    let handle = h.app.handles.lock().unwrap()[0].clone();
    assert!(handle.is_connected());
    handle.send(Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "X")).unwrap();
    let out = s.command(commands.try_send().unwrap(), h.t0);
    let msg = sent(&out)[0];
    assert_eq!(msg.get(tags::MSG_SEQ_NUM), Some("2"));
    assert_eq!(msg.get(tags::TARGET_COMP_ID), Some("CLIENT"));

    handle.logout(Some("bye")).unwrap();
    let out = s.command(commands.try_control().unwrap(), h.t0);
    assert_eq!(types(&out), ["Logout"]);
    assert_eq!(types(&s.recv(client(2, MsgType::Logout), h.t0)), ["DISCONNECT"]);

    drop(s);
    assert!(!handle.is_connected());
    assert!(handle.send(Message::new(MsgType::ExecutionReport)).is_err());
}

#[test]
fn handle_cannot_send_session_level_messages() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.command(Command::send(Message::new(MsgType::SequenceReset)), h.t0);
    assert!(out.is_empty());
}

#[test]
fn application_poss_resend_is_sent_and_kept_on_resend() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our Logon reply is seq 1
    let report = Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "E1").with(tags::POSS_RESEND, "Y");
    let out = s.command(Command::send(report), h.t0);
    let first = sent(&out)[0];
    assert_eq!(first.get(tags::POSS_RESEND), Some("Y"), "application's PossResend(97) is sent");
    assert_eq!(first.get(tags::POSS_DUP_FLAG), None);

    let req = client(2, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "2").with(tags::END_SEQ_NO, "0");
    let out = s.recv(req, h.t0);
    let resent = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ExecutionReport).expect("resent");
    assert_eq!(resent.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(resent.get(tags::POSS_RESEND), Some("Y"), "kept when resent");
}

#[test]
fn message_with_soh_inside_a_value_is_not_sent() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our Logon reply is seq 1
    let injected = Message::new(MsgType::ExecutionReport).with(tags::TEXT, "fine\x0139=8");
    assert!(s.command(Command::send(injected), h.t0).is_empty(), "would add a field on the wire");

    let out = s.command(send_command("A"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("2"), "no sequence number used");
}

#[test]
fn data_fields_may_contain_soh_and_bytes_that_are_not_utf8() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let bytes = b"\xff\x0139=8\x01";
    let report = Message::new(MsgType::ExecutionReport)
        .with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, bytes)
        .with(tags::TEXT, "after");
    let out = s.command(Command::send(report), h.t0);
    let msg = sent(&out)[0];
    assert_eq!(msg.get_bytes(tags::RAW_DATA), Some(&bytes[..]));

    // What goes on the wire decodes to the same message.
    let wire = crate::codec::encode(msg).unwrap();
    let crate::codec::Decoded::Message(decoded, _) = crate::codec::decode(&wire) else { panic!("garbled") };
    assert!(decoded.defect().is_none());
    assert_eq!(decoded.get_bytes(tags::RAW_DATA), Some(&bytes[..]));
    assert_eq!(decoded.get(tags::TEXT), Some("after"));
}

#[test]
fn a_data_field_without_its_length_is_not_sent() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our Logon reply is seq 1
    for report in [
        Message::new(MsgType::ExecutionReport).with(tags::RAW_DATA, "a\x01b"),
        Message::new(MsgType::ExecutionReport).with(tags::RAW_DATA_LENGTH, 2u64).with(tags::RAW_DATA, "a\x01b"),
        Message::new(MsgType::ExecutionReport)
            .with(tags::RAW_DATA_LENGTH, 3u64)
            .with(tags::TEXT, "x")
            .with(tags::RAW_DATA, "a\x01b"),
    ] {
        assert!(s.command(Command::send(report.clone()), h.t0).is_empty(), "{report}");
    }
    let out = s.command(send_command("A"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("2"), "no sequence number used");
}

#[test]
fn venue_data_fields_are_sent_when_configured() {
    let venue = Message::new(MsgType::ExecutionReport).with_data(5000, 5001, b"a\x01b");
    let h = Harness::new();
    let mut s = h.logged_on();
    assert!(s.command(Command::send(venue.clone()), h.t0).is_empty(), "5001 isn't a data field");

    let mut h = Harness::new();
    h.config = h.config.with_data_field(5000, 5001);
    let mut s = h.logged_on();
    let out = s.command(Command::send(venue), h.t0);
    assert_eq!(sent(&out)[0].get(5001), Some("a\x01b"));
}

#[test]
fn venue_data_fields_are_resent_intact_from_disk() {
    let dir = tempfile::tempdir().unwrap();
    let mut h = Harness::with_storage(Arc::new(crate::store::DiskStorage::new(dir.path(), false).unwrap()));
    h.config = h.config.with_data_field(5000, 5001);
    let report = Message::new(MsgType::ExecutionReport).with_data(5000, 5001, b"\xfe\x0110=000\x01");
    let mut first = h.logged_on(); // our 1: Logon
    assert_eq!(sent(&first.command(Command::send(report), h.t0)).len(), 1, "our 2");
    drop(first);

    let mut s = h.session();
    let out = s.recv(logon(2).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "2"), h.t0);
    let resent = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ExecutionReport).expect("resent");
    assert_eq!(resent.get_bytes(5001), Some(&b"\xfe\x0110=000\x01"[..]));
}

#[test]
fn sending_time_is_written_at_the_configured_precision() {
    let mut h = Harness::new();
    h.config.timestamp_precision = Precision::Micros;
    let mut s = h.logged_on(); // our 1: Logon
    let out = s.command(send_command("A"), h.t0);
    let sending_time = sent(&out)[0].get(tags::SENDING_TIME).unwrap().to_string();
    assert_eq!(sending_time.len(), "YYYYMMDD-HH:MM:SS.ffffff".len(), "{sending_time}");

    // A resend keeps the original as OrigSendingTime, and is itself sent in microseconds.
    let req = client(2, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "2").with(tags::END_SEQ_NO, "0");
    let out = s.recv(req, h.t0);
    let resent = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::NewOrderSingle).expect("resent");
    assert_eq!(resent.get(tags::ORIG_SENDING_TIME), Some(sending_time.as_str()));
    assert_eq!(resent.get(tags::SENDING_TIME).unwrap().len(), sending_time.len());
}

fn send_command(cl_ord_id: &str) -> Command {
    Command::send(Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, cl_ord_id))
}

#[test]
fn commands_during_logon_are_queued_and_sent_in_order_after_it() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0); // our Logon is seq 1
    assert!(s.command(send_command("A"), h.t0).is_empty());
    assert!(s.command(send_command("B"), h.t0).is_empty());

    let out = s.recv(logon(1), h.t0);
    let msgs = sent(&out);
    let ids: Vec<_> = msgs.iter().map(|m| (m.get(tags::CL_ORD_ID), m.get(tags::MSG_SEQ_NUM))).collect();
    assert_eq!(ids, [(Some("A"), Some("2")), (Some("B"), Some("3"))]);
    assert_eq!(h.app.events(), ["logon CLIENT"], "on_logon runs before the queue is flushed");
}

#[test]
fn queued_commands_follow_a_resend_request_triggered_by_logon() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.command(send_command("A"), h.t0);
    // The counterparty's Logon is ahead of what we expect, so we ask for a resend first.
    let out = s.recv(logon(3), h.t0);
    assert_eq!(types(&out), ["ResendRequest", "NewOrderSingle"]);
}

#[test]
fn queued_logout_applies_after_queued_sends() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.command(send_command("A"), h.t0);
    s.command(Command::Logout(Some("done".into())), h.t0);
    s.command(send_command("B"), h.t0);

    let out = s.recv(logon(1), h.t0);
    assert_eq!(types(&out), ["NewOrderSingle", "Logout"], "B was sent after the logout request");
    assert_eq!(sent(&out)[1].get(tags::TEXT), Some("done"));
}

#[test]
fn queued_commands_are_discarded_if_logon_fails() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.command(send_command("A"), h.t0);
    assert_eq!(types(&s.timer(h.at(10))), ["DISCONNECT"]);
    assert!(s.pending.is_empty());
    assert!(s.recv(logon(1), h.t0).is_empty(), "nothing is sent after close");
}

#[test]
fn sends_after_logout_has_started_are_dropped() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.command(Command::Logout(None), h.t0);
    assert!(s.command(send_command("A"), h.t0).is_empty());
}

// ---- Shutdown ----

#[test]
fn shutdown_logs_out_and_disconnects_on_the_reply() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.shutdown(Some("end of day"), h.t0);
    assert_eq!(types(&out), ["Logout"]);
    assert_eq!(sent(&out)[0].get(tags::TEXT), Some("end of day"));
    assert_eq!(types(&s.recv(client(2, MsgType::Logout), h.at(1))), ["DISCONNECT"]);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Shutdown"]);
}

#[test]
fn shutdown_disconnects_after_the_logout_timeout() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.shutdown(None, h.t0);
    let timeout = h.config.logout_timeout.as_secs();
    assert!(s.timer(h.at(timeout - 1)).is_empty());
    assert_eq!(types(&s.timer(h.at(timeout))), ["DISCONNECT"]);
}

#[test]
fn shutdown_during_a_logout_changes_nothing() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.command(Command::Logout(None), h.t0);
    assert!(s.shutdown(Some("again"), h.t0).is_empty());
}

#[test]
fn shutdown_before_logon_disconnects() {
    let h = Harness::new();
    let mut acceptor = h.session();
    assert_eq!(types(&acceptor.shutdown(None, h.t0)), ["DISCONNECT"]);

    // An initiator whose Logon hasn't been answered.
    let mut initiator = h.initiator(false);
    assert_eq!(types(&initiator.connect(h.t0)), ["Logon"]);
    assert_eq!(types(&initiator.shutdown(None, h.t0)), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

// ---- Storage failures ----

/// Storage whose logs fail every write after the first `ok_writes` (counted per log), or with
/// `once`, only the one after them, over a memory store that keeps its state across reopening.
struct FailingStorage {
    ok_writes: usize,
    once: bool,
    inner: MemoryStorage,
}

struct FailingLog {
    inner: Box<dyn SessionLog>,
    remaining: Option<usize>,
    once: bool,
}

impl FailingLog {
    fn write(&mut self) -> io::Result<()> {
        match self.remaining {
            Some(0) => {
                self.remaining = (!self.once).then_some(0);
                Err(io::Error::other("disk full"))
            }
            Some(n) => {
                self.remaining = Some(n - 1);
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl SessionStorage for FailingStorage {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        let inner = self.inner.open(id)?;
        Ok(Box::new(FailingLog { inner, remaining: Some(self.ok_writes), once: self.once }))
    }
}

impl SessionLog for FailingLog {
    fn next_outgoing(&self) -> u64 {
        self.inner.next_outgoing()
    }
    fn next_incoming(&self) -> u64 {
        self.inner.next_incoming()
    }
    fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
        self.write()?;
        self.inner.set_next_incoming(seq)
    }
    fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
        self.write()?;
        self.inner.record_outgoing(seq, msg)
    }
    fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
        self.inner.sent_messages(begin, end)
    }
    fn reset(&mut self) -> io::Result<()> {
        self.write()?;
        self.inner.reset()
    }
    fn in_flight(&self) -> Option<u64> {
        self.inner.in_flight()
    }
    fn set_in_flight(&mut self, seq: u64) -> io::Result<()> {
        self.write()?;
        self.inner.set_in_flight(seq)
    }
}

fn failing_after(ok_writes: usize) -> Harness {
    Harness::with_storage(Arc::new(FailingStorage { ok_writes, once: false, inner: MemoryStorage::new() }))
}

#[test]
fn storage_failure_during_logon_disconnects_without_reply() {
    let h = failing_after(0);
    let mut s = h.session();
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
    assert!(h.app.events().is_empty());
}

/// A session that a storage failure closes during logon sends nothing more: not its Logon reply,
/// nor the resend the counterparty's NextExpectedMsgSeqNum asks for.
#[test]
fn storage_failure_during_logon_sends_no_resend() {
    let storage = FailingStorage { ok_writes: 0, once: false, inner: MemoryStorage::new() };
    let id =
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() };
    {
        // We've sent up to 3 on an earlier connection.
        let mut log = storage.inner.open(&id).unwrap();
        for seq in 1..=3 {
            log.record_outgoing(seq, None).unwrap();
        }
    }
    let h = Harness::with_storage(Arc::new(storage));
    let mut s = h.session();
    let out = s.recv(logon(4).with(tags::NEXT_EXPECTED_MSG_SEQ_NUM, "2"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
}

/// Logon uses three writes: our Logon reply, the incoming sequence number, and the window the
/// first batch is handed over in.
const LOGON_WRITES: usize = 3;

/// The order was handed over, but its reply couldn't be stored: the session disconnects without
/// recording the order as received, so it's resent, inside the window, and marked.
#[test]
fn storage_failure_storing_a_reply_disconnects_without_recording_the_message() {
    let h = failing_after(LOGON_WRITES);
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(2, "A"), h.t0)), ["DISCONNECT"]);
    assert_eq!(h.app.received(), 1);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Error"]);
    let log = &s.peer().log;
    assert_eq!((log.next_incoming(), log.in_flight()), (2, Some(2)));
}

/// Regression (simulator): an operator's skip whose SequenceReset fails to store closes the
/// session, and the skip itself, stored after, is never committed: it's reported as failed.
#[test]
fn an_operator_skip_whose_sequence_reset_fails_is_reported_failed() {
    let storage = FailingStorage { ok_writes: LOGON_WRITES, once: true, inner: MemoryStorage::new() };
    let h = Harness::with_storage(Arc::new(storage));
    let mut s = h.logged_on();
    let (reply, mut answer) = tokio::sync::oneshot::channel();
    let out = s.command(Command::Sequence(SequenceCommand::SetNextOutgoing(50), reply), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
    assert!(matches!(answer.try_recv().unwrap(), Err(SequenceError::Storage(_))));
}

#[test]
fn storage_failure_while_sending_suppresses_the_message() {
    // The write storing the Heartbeat fails; the TestRequest's number isn't saved either.
    let h = failing_after(LOGON_WRITES);
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.t0);
    assert_eq!(types(&out), ["DISCONNECT"]);
}

/// Nothing limits what the session sends, so what it stored is resent whatever its size; the
/// 64 KiB limit on BodyLength(9) is for input from the counterparty.
#[test]
fn a_stored_message_over_64_kib_is_resent() {
    let storage = Arc::new(MemoryStorage::new());
    let text = "x".repeat(70 * 1024);
    {
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GATEWAY".into(),
            target_comp_id: "CLIENT".into(),
        };
        let report = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.4")
            .with(tags::MSG_TYPE, MsgType::ExecutionReport)
            .with(tags::SENDER_COMP_ID, "GATEWAY")
            .with(tags::TARGET_COMP_ID, "CLIENT")
            .with(tags::MSG_SEQ_NUM, 1u64)
            .with(tags::SENDING_TIME, "20260930-12:00:00.000")
            .with(tags::TEXT, text.as_str());
        storage.open(&id).unwrap().record_outgoing(1, Some(&encode(&report).unwrap())).unwrap();
    }
    let h = Harness::with_storage(storage);
    let mut s = h.logged_on(); // our 2: Logon
    let req = client(2, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0");
    let out = s.recv(req, h.t0);
    assert_eq!(types(&out), ["ExecutionReport", "SequenceReset"]);
    assert_eq!(sent(&out)[0].get(tags::TEXT), Some(text.as_str()));
}

/// The outbound log never shows bytes it can't decode, which redaction couldn't cover.
#[test]
fn outbound_log_shows_no_content_it_cannot_decode() {
    let logged = Outbound(b"8=FIX.4.4\x019=5\x01554=secret\x01", &DataFields::standard()).to_string();
    assert!(!logged.contains("secret"), "{logged}");
    assert_eq!(logged, "<25 bytes that don't decode>");
}

/// Stores check only a stored message's framing; one whose fields don't parse can't be resent,
/// and ends the session like any other storage failure.
/// Stores, as our `seq` to CLIENT, an ExecutionReport whose framing is intact but which doesn't
/// parse: its ExecID(17) isn't UTF-8.
fn store_corrupt_report(storage: &MemoryStorage, seq: u64) {
    let id =
        SessionId { begin_string: "FIX.4.4".into(), sender_comp_id: "GATEWAY".into(), target_comp_id: "CLIENT".into() };
    let report = Message::default()
        .with(tags::BEGIN_STRING, "FIX.4.4")
        .with(tags::MSG_TYPE, MsgType::ExecutionReport)
        .with(tags::SENDER_COMP_ID, "GATEWAY")
        .with(tags::TARGET_COMP_ID, "CLIENT")
        .with(tags::MSG_SEQ_NUM, seq)
        .with(tags::SENDING_TIME, "20260930-12:00:00.000")
        .with(tags::EXEC_ID, "E1");
    // A valid CheckSum, so the framing is intact.
    let mut bytes = encode(&report).unwrap();
    let at = bytes.windows(5).position(|w| w == b"\x0117=E").unwrap() + 4;
    bytes[at] = 0xff;
    let trailer = bytes.len() - 7;
    let sum = crate::codec::checksum(&bytes[..trailer]);
    bytes[trailer..].copy_from_slice(format!("10={sum:03}\x01").as_bytes());
    storage.open(&id).unwrap().record_outgoing(seq, Some(&bytes)).unwrap();
}

#[test]
fn a_stored_message_that_does_not_parse_disconnects_on_a_resend() {
    let storage = Arc::new(MemoryStorage::new());
    store_corrupt_report(&storage, 1);
    let h = Harness::with_storage(storage);
    let mut s = h.logged_on(); // our 2: Logon
    let req = client(2, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0");
    assert_eq!(types(&s.recv(req, h.t0)), ["DISCONNECT"]);
}

// ---- Typed session-message validation ----

#[test]
fn test_request_without_id_is_rejected() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(client(2, MsgType::TestRequest), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("112"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("1"));
}

#[test]
fn second_logon_is_rejected_without_a_post_4_2_reason() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(logon(2), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::Reject);
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), None);
    assert_eq!(reject.get(tags::REF_MSG_TYPE), Some("A"));
}

#[test]
fn logon_without_encrypt_method_is_refused() {
    let h = Harness::new();
    let mut s = h.session();
    let mut logon = Message::default();
    for (tag, value) in super::tests::logon(1).fields().filter(|(t, _)| *t != tags::ENCRYPT_METHOD) {
        logon.push(tag, value);
    }
    assert_eq!(types(&s.recv(logon, h.t0)), ["DISCONNECT"]);
}

#[test]
fn malformed_resend_request_is_rejected() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let req = client(2, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "x").with(tags::END_SEQ_NO, "0");
    let reject = s.recv(req, h.t0);
    assert_eq!(sent(&reject)[0].get(tags::SESSION_REJECT_REASON), Some("6"));
}

// ---- Connection info ----

fn connection_info() -> ConnectionInfo {
    ConnectionInfo::new(Some("10.0.0.7:4000".parse().unwrap()), vec![PeerCertificate::from_der(vec![0x30, 0x00])])
}

#[test]
fn acceptor_passes_connection_info_to_verify_logon() {
    let h = Harness::new();
    let mut s = h.session();
    s.set_connection_info(connection_info());
    s.recv(logon(1), h.t0);
    assert_eq!(*h.app.connections.lock().unwrap(), [connection_info()]);
}

#[test]
fn initiator_passes_connection_info_when_verifying_the_logon_reply() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.set_connection_info(connection_info());
    s.connect(h.t0);
    s.recv(logon(1), h.t0);
    assert_eq!(*h.app.connections.lock().unwrap(), [connection_info()]);
}

#[test]
fn connection_info_defaults_to_nothing_known() {
    let h = Harness::new();
    let mut s = h.session();
    s.recv(logon(1), h.t0);
    let seen = h.app.connections.lock().unwrap();
    assert_eq!(seen[0].addr, None);
    assert!(seen[0].peer_certificate().is_none());
}

// ---- Metrics ----

#[cfg(feature = "metrics")]
mod metrics_tests {
    use metrics_util::debugging::{DebugValue, DebuggingRecorder};

    use super::*;

    const SESSION: &str = "FIX.4.4:GATEWAY->CLIENT";

    /// One snapshot of the recorder. Taking a snapshot resets the recorder's counters, so read
    /// every value from the same one.
    struct Snapshot(Vec<(metrics_util::CompositeKey, DebugValue)>);

    impl Snapshot {
        fn take(recorder: &DebuggingRecorder) -> Self {
            Self(
                recorder
                    .snapshotter()
                    .snapshot()
                    .into_vec()
                    .into_iter()
                    .map(|(key, _, _, value)| (key, value))
                    .collect(),
            )
        }

        /// The value of `name` for this session with the `extra` labels.
        fn value(&self, name: &str, extra: &[(&str, &str)]) -> f64 {
            let found = self.0.iter().find(|(key, _)| {
                let key = key.key();
                key.name() == name
                    && key.labels().any(|l| l.key() == "session" && l.value() == SESSION)
                    && extra.iter().all(|(k, v)| key.labels().any(|l| l.key() == *k && l.value() == *v))
            });
            match found.map(|(_, value)| value) {
                Some(DebugValue::Counter(n)) => *n as f64,
                Some(DebugValue::Gauge(g)) => g.into_inner(),
                other => panic!("{name}: {other:?}"),
            }
        }
    }

    #[test]
    fn session_records_its_activity() {
        let recorder = DebuggingRecorder::new();
        let h = Harness::new();
        ::metrics::with_local_recorder(&recorder, || {
            let mut s = h.session();
            s.recv(logon(1), h.t0); // we send Logon (1)
            s.recv(order(2, "A"), h.t0); // ExecutionReport (2)
            s.recv(client(3, MsgType::from_code("G")), h.t0); // BusinessMessageReject (3)
            s.recv(client(4, MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "B"), h.t0); // Reject (4)
            // Resend 1..: gap fill 1, resend 2 and 3, gap fill 4.
            s.recv(client(5, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0"), h.t0);
            s.recv(client(8, MsgType::Heartbeat), h.t0); // gap: ResendRequest (5)

            let snapshot = Snapshot::take(&recorder);
            let value = |name, extra| snapshot.value(name, extra);
            assert_eq!(value("turbojet_messages_received_total", &[]), 6.0);
            assert_eq!(value("turbojet_messages_sent_total", &[]), 9.0);
            assert_eq!(value("turbojet_logons_total", &[]), 1.0);
            assert_eq!(value("turbojet_session_logged_on", &[]), 1.0);
            assert_eq!(value("turbojet_rejects_sent_total", &[("type", "session")]), 1.0);
            assert_eq!(value("turbojet_rejects_sent_total", &[("type", "business")]), 1.0);
            assert_eq!(value("turbojet_resend_requests_received_total", &[]), 1.0);
            assert_eq!(value("turbojet_sequence_gaps_total", &[]), 1.0);
            assert_eq!(value("turbojet_next_incoming_seq", &[]), 6.0, "8 is ahead of the gap");
            assert_eq!(value("turbojet_next_outgoing_seq", &[]), 6.0);
            assert_eq!(value("turbojet_disconnects_total", &[]), 0.0);

            drop(s);
            let after = Snapshot::take(&recorder);
            assert_eq!(after.value("turbojet_disconnects_total", &[]), 1.0);
            assert_eq!(after.value("turbojet_session_logged_on", &[]), 0.0);
        });
    }

    #[test]
    fn counts_resend_requests_that_reach_evicted_messages() {
        let recorder = DebuggingRecorder::new();
        let h = keeping_two_reports();
        ::metrics::with_local_recorder(&recorder, || {
            let mut s = with_reports(&h, 4, 256); // 2 and 3 evicted
            s.recv(resend_request(6, 4), h.t0);
            s.recv(resend_request(7, 3), h.t0);
            s.recv(resend_request(8, 1), h.t0);
            let snapshot = Snapshot::take(&recorder);
            assert_eq!(snapshot.value("turbojet_resend_requests_evicted_total", &[]), 2.0);
            assert_eq!(snapshot.value("turbojet_resend_requests_received_total", &[]), 3.0);
        });
    }

    #[test]
    fn counts_each_inbound_reject() {
        let recorder = DebuggingRecorder::new();
        let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
        ::metrics::with_local_recorder(&recorder, || {
            let mut s = h.logged_on();
            s.recv(order(2, "A"), h.at(1));
            s.recv(order(3, "B"), h.at(1));
            for seq_num in 4..7 {
                s.recv(order(seq_num, "X"), h.at_millis(1_500));
            }
            s.recv(order(7, "C"), h.at(2));
            let snapshot = Snapshot::take(&recorder);
            assert_eq!(snapshot.value("turbojet_throttled_total", &[("direction", "inbound")]), 3.0);
            assert_eq!(snapshot.value("turbojet_throttled_total", &[("direction", "outbound")]), 0.0);
            assert_eq!(snapshot.value("turbojet_rejects_sent_total", &[("type", "business")]), 3.0);
        });
    }

    #[test]
    fn counts_each_inbound_delay_hold() {
        let recorder = DebuggingRecorder::new();
        let h = Harness::with_inbound_delay(2, Duration::from_secs(1));
        ::metrics::with_local_recorder(&recorder, || {
            let mut s = h.logged_on();
            s.recv(order(2, "A"), h.at(1));
            assert_eq!(Snapshot::take(&recorder).value("turbojet_throttled_total", &[("direction", "inbound")]), 0.0);
            s.recv(order(3, "B"), h.at(1)); // fills the window: a hold
            s.recv(order(4, "C"), h.at(2)); // A and B have expired
            s.recv(order(5, "D"), h.at(2)); // fills it again
            s.recv(order(6, "E"), h.at_millis(3_500));
            assert_eq!(Snapshot::take(&recorder).value("turbojet_throttled_total", &[("direction", "inbound")]), 2.0);
        });
    }

    #[test]
    fn retrying_a_resend_request_is_not_a_new_gap() {
        let recorder = DebuggingRecorder::new();
        let h = Harness::new();
        ::metrics::with_local_recorder(&recorder, || {
            let mut s = h.logged_on();
            s.recv(order(5, "E"), h.t0); // gap: ResendRequest for 2..
            assert!(types(&s.timer(h.at(60))).contains(&"ResendRequest".to_string()), "retried");
            assert_eq!(Snapshot::take(&recorder).value("turbojet_sequence_gaps_total", &[]), 1.0);
        });
    }
}

// ---- Schedules ----

mod schedule_tests {
    use chrono::{DateTime, NaiveDateTime, Utc};

    use super::*;
    use crate::schedule::Clock;

    /// A clock the test moves by hand.
    #[derive(Clone)]
    struct ManualClock(Arc<Mutex<DateTime<Utc>>>);

    impl ManualClock {
        fn at(time: &str) -> Self {
            Self(Arc::new(Mutex::new(utc(time))))
        }
        fn set(&self, time: &str) {
            *self.0.lock().unwrap() = utc(time);
        }
        fn clock(&self) -> Clock {
            let now = self.0.clone();
            Clock::from_fn(move || *now.lock().unwrap())
        }
    }

    fn utc(time: &str) -> DateTime<Utc> {
        NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap().and_utc()
    }

    fn scheduled(schedule: &str, clock: &ManualClock) -> Harness {
        scheduled_with(schedule, clock, Arc::new(MemoryStorage::new()))
    }

    fn scheduled_with(schedule: &str, clock: &ManualClock, storage: Arc<dyn SessionStorage>) -> Harness {
        let mut h = Harness::with_storage(storage);
        h.config.schedule = Some(schedule.parse().unwrap());
        h.config.clock = clock.clock();
        // The test clock is set to fixed dates, and the counterparty's messages carry the real time.
        h.config.max_latency = None;
        h
    }

    /// Outbound SendingTime(52) is the session's clock, as the check on inbound uses.
    #[test]
    fn sending_time_comes_from_the_clock() {
        let clock = ManualClock::at("2026-09-28 09:00:00");
        let h = scheduled("daily 08:00-17:00", &clock);
        let mut s = h.session();
        let out = s.recv(logon(1), h.t0);
        assert_eq!(sent(&out)[0].get(tags::SENDING_TIME), Some("20260928-09:00:00.000"));
    }

    /// Our MsgSeqNum on the Logon reply.
    fn logon_reply_seq(out: &[Action]) -> Option<&str> {
        sent(out).first().filter(|m| m.msg_type() == MsgType::Logon).and_then(|m| m.get(tags::MSG_SEQ_NUM))
    }

    // 2026-09-28 is a Monday.

    #[test]
    fn acceptor_refuses_logon_outside_session_time() {
        let clock = ManualClock::at("2026-09-28 07:00:00");
        let h = scheduled("daily 08:00-17:00", &clock);
        let mut s = h.session();
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        assert!(h.registry.sessions().is_empty());
        assert!(h.app.events().is_empty());
    }

    /// A counterparty's own schedule decides whether it may log on, under the acceptor's clock.
    #[test]
    fn a_counterpartys_schedule_decides_its_logon() {
        let clock = ManualClock::at("2026-09-28 07:00:00");
        let mut h = Harness::new();
        h.config.clock = clock.clock();
        h.config.max_latency = None;
        let in_hours = adjusted(|c| c.config.schedule = Some("daily 08:00-17:00".parse().unwrap()));
        let mut s = h.resolved(in_hours);
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"], "closed at 07:00");
        assert!(h.app.events().is_empty());
        clock.set("2026-09-28 09:00:00");
        let mut s = h.resolved(adjusted(|c| c.config.schedule = Some("daily 08:00-17:00".parse().unwrap())));
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"), "open at 09:00");
    }

    #[test]
    fn logs_out_when_the_period_ends() {
        let clock = ManualClock::at("2026-09-28 16:59:00");
        let h = scheduled("daily 08:00-17:00", &clock);
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"));
        assert!(s.timer(h.at(1)).is_empty(), "still inside the period");

        clock.set("2026-09-28 17:00:00");
        let out = s.timer(h.at(2));
        assert_eq!(types(&out), ["Logout"]);
        assert_eq!(sent(&out)[0].get(tags::TEXT), Some("End of session"));
        assert_eq!(types(&s.recv(client(2, MsgType::Logout), h.at(2))), ["DISCONNECT"]);
        assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Shutdown"]);
    }

    #[test]
    fn sequence_numbers_reset_at_the_first_logon_of_a_new_period() {
        let clock = ManualClock::at("2026-09-28 10:00:00");
        let h = scheduled("daily 08:00-17:00", &clock);
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"));
        s.recv(client(2, MsgType::Heartbeat), h.t0);
        drop(s);

        // Reconnecting in the same period continues the sequence.
        clock.set("2026-09-28 11:00:00");
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(3), h.t0)), Some("2"));
        drop(s);

        // The next day's first logon starts again at 1 on both sides.
        clock.set("2026-09-29 09:00:00");
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"));
        assert!(s.is_logged_on());
    }

    #[test]
    fn continuous_session_logs_out_and_resets_at_the_daily_boundary() {
        let clock = ManualClock::at("2026-09-28 23:59:59");
        let h = scheduled("daily 00:00-00:00", &clock);
        let mut s = h.session();
        s.recv(logon(1), h.t0);
        s.recv(client(2, MsgType::Heartbeat), h.t0);

        clock.set("2026-09-29 00:00:00");
        assert_eq!(types(&s.timer(h.at(1))), ["Logout"], "a new period began");
        s.recv(client(3, MsgType::Logout), h.at(1));
        drop(s);

        clock.set("2026-09-29 00:00:05");
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"));
    }

    #[test]
    fn initiator_does_not_log_on_outside_session_time() {
        let clock = ManualClock::at("2026-10-03 10:00:00"); // Saturday
        let h = scheduled("daily 08:00-17:00 mon-fri", &clock);
        let mut s = h.initiator(false);
        assert_eq!(types(&s.connect(h.t0)), ["DISCONNECT"], "no Logon is sent");
    }

    #[test]
    fn initiator_awaiting_logon_reply_disconnects_when_the_period_ends() {
        let clock = ManualClock::at("2026-09-28 16:59:59");
        let h = scheduled("daily 08:00-17:00", &clock);
        let mut s = h.initiator(false);
        assert_eq!(types(&s.connect(h.t0)), ["Logon"]);
        clock.set("2026-09-28 17:00:00");
        assert_eq!(types(&s.timer(h.at(1))), ["DISCONNECT"]);
    }

    #[test]
    fn existing_state_without_a_creation_time_is_kept_until_the_next_period() {
        // State written before creation times were recorded: sequence numbers but no timestamp.
        let storage = Arc::new(MemoryStorage::new());
        let id = SessionId {
            begin_string: "FIX.4.4".into(),
            sender_comp_id: "GATEWAY".into(),
            target_comp_id: "CLIENT".into(),
        };
        {
            let mut log = storage.open(&id).unwrap();
            log.record_outgoing(4, None).unwrap();
            log.set_next_incoming(7).unwrap();
            assert_eq!(log.created_at(), None);
        }
        let clock = ManualClock::at("2026-09-28 10:00:00");
        let h = scheduled_with("daily 08:00-17:00", &clock, storage);
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(7), h.t0)), Some("5"), "not reset on upgrade");
        drop(s);

        clock.set("2026-09-29 09:00:00");
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(1), h.t0)), Some("1"), "reset in the next period");
    }

    /// A persistent store that, like many custom ones, doesn't record creation times: it
    /// delegates everything except `created_at`/`set_created_at`, which keep their defaults.
    struct NoCreationTimes(MemoryStorage);

    struct NoCreationTimesLog(Box<dyn SessionLog>);

    impl SessionStorage for NoCreationTimes {
        fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
            Ok(Box::new(NoCreationTimesLog(self.0.open(id)?)))
        }
    }

    impl SessionLog for NoCreationTimesLog {
        fn next_outgoing(&self) -> u64 {
            self.0.next_outgoing()
        }
        fn next_incoming(&self) -> u64 {
            self.0.next_incoming()
        }
        fn set_next_incoming(&mut self, seq: u64) -> io::Result<()> {
            self.0.set_next_incoming(seq)
        }
        fn record_outgoing(&mut self, seq: u64, msg: Option<&[u8]>) -> io::Result<()> {
            self.0.record_outgoing(seq, msg)
        }
        fn sent_messages(&mut self, begin: u64, end: u64) -> io::Result<Vec<(u64, Vec<u8>)>> {
            self.0.sent_messages(begin, end)
        }
        fn reset(&mut self) -> io::Result<()> {
            self.0.reset()
        }
    }

    #[test]
    fn stores_without_creation_times_never_reset_on_schedule() {
        let clock = ManualClock::at("2026-09-28 10:00:00");
        let h = scheduled_with("daily 08:00-17:00", &clock, Arc::new(NoCreationTimes(MemoryStorage::new())));
        let mut s = h.session();
        s.recv(logon(1), h.t0);
        s.recv(client(2, MsgType::Heartbeat), h.t0);
        drop(s);

        clock.set("2026-09-29 09:00:00");
        let mut s = h.session();
        assert_eq!(logon_reply_seq(&s.recv(logon(3), h.t0)), Some("2"), "sequence continues");
    }
}

// ---- Operator control of sequence numbers ----

mod operator_tests {
    use super::*;
    use crate::registry::{SequenceCommand, SequenceError, SequenceNumbers};

    /// Sends an operator command to the session, returning its answer and anything it sent.
    fn operate(
        s: &mut Session,
        command: SequenceCommand,
        h: &Harness,
    ) -> (Result<SequenceNumbers, SequenceError>, Vec<Action>) {
        operate_at(s, command, h.t0)
    }

    fn operate_at(
        s: &mut Session,
        command: SequenceCommand,
        now: Instant,
    ) -> (Result<SequenceNumbers, SequenceError>, Vec<Action>) {
        let (reply, mut answer) = tokio::sync::oneshot::channel();
        let out = s.command(Command::Sequence(command, reply), now);
        (answer.try_recv().expect("answered synchronously"), out)
    }

    fn numbers(next_incoming: u64, next_outgoing: u64) -> SequenceNumbers {
        SequenceNumbers { next_incoming, next_outgoing }
    }

    #[test]
    fn get_reports_the_live_numbers() {
        let h = Harness::new();
        let mut s = h.logged_on(); // they sent 1, we sent our Logon as 1
        let (result, out) = operate(&mut s, SequenceCommand::Get, &h);
        assert_eq!(result.unwrap(), numbers(2, 2));
        assert!(out.is_empty());
    }

    #[test]
    fn raising_next_outgoing_tells_the_counterparty_with_a_sequence_reset() {
        let h = Harness::new();
        let mut s = h.logged_on();
        let (result, out) = operate(&mut s, SequenceCommand::SetNextOutgoing(100), &h);
        assert_eq!(result.unwrap(), numbers(2, 100));
        let reset = sent(&out)[0];
        assert_eq!(reset.msg_type(), MsgType::SequenceReset);
        assert_eq!(reset.get(tags::NEW_SEQ_NO), Some("100"));
        // Gap-fill mode, with its own MsgSeqNum: a counterparty still filling a gap applies it only
        // once the gap is filled, rather than abandoning the gap as reset mode would make it.
        assert_eq!(reset.get(tags::GAP_FILL_FLAG), Some("Y"));
        assert_eq!(reset.get(tags::MSG_SEQ_NUM), Some("2"));
        // Our next message carries 100.
        let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.t0);
        assert_eq!(sent(&out)[0].get(tags::MSG_SEQ_NUM), Some("100"));
    }

    /// The other end of a skip: a gap-fill SequenceReset that arrives while a gap is open waits its
    /// turn, so what was sent before it is still delivered.
    #[test]
    fn a_skip_ahead_during_a_gap_is_applied_after_the_gap_fills() {
        let h = Harness::new();
        let mut s = h.logged_on(); // their next is 2
        assert_eq!(types(&s.recv(order(3, "B"), h.t0)), ["ResendRequest"]);
        let skip = client(4, MsgType::SequenceReset).with(tags::GAP_FILL_FLAG, "Y").with(tags::NEW_SEQ_NO, "100");
        assert!(s.recv(skip, h.t0).is_empty(), "queued behind the gap");
        let resent = resend_of(order(2, "A"));
        assert_eq!(types(&s.recv(resent, h.t0)), ["ExecutionReport", "ExecutionReport"]);
        assert_eq!(h.app.received(), 2, "A and B both delivered");
        assert_eq!(s.peer().log.next_incoming(), 100);
    }

    #[test]
    fn outgoing_numbers_never_move_backwards() {
        let h = Harness::new();
        let mut s = h.logged_on();
        for seq in [0, 1] {
            let (result, out) = operate(&mut s, SequenceCommand::SetNextOutgoing(seq), &h);
            assert!(matches!(result, Err(SequenceError::Invalid(_))), "{seq}: {result:?}");
            assert!(out.is_empty());
        }
        // Setting the current value is a no-op, not an error.
        assert_eq!(operate(&mut s, SequenceCommand::SetNextOutgoing(2), &h).0.unwrap(), numbers(2, 2));
    }

    #[test]
    fn next_incoming_can_be_set_either_way() {
        let h = Harness::new();
        let mut s = h.logged_on();
        assert_eq!(operate(&mut s, SequenceCommand::SetNextIncoming(10), &h).0.unwrap(), numbers(10, 2));
        assert!(s.recv(client(10, MsgType::Heartbeat), h.t0).is_empty(), "10 is now in sequence");
        assert_eq!(operate(&mut s, SequenceCommand::SetNextIncoming(5), &h).0.unwrap(), numbers(5, 2));
        assert!(s.recv(client(5, MsgType::Heartbeat), h.t0).is_empty());
    }

    fn numbers_after(s: &Session) -> SequenceNumbers {
        SequenceNumbers { next_incoming: s.peer().log.next_incoming(), next_outgoing: s.peer().log.next_outgoing() }
    }

    #[test]
    fn setting_incoming_to_a_queued_message_processes_it() {
        let h = Harness::new();
        let mut s = h.logged_on();
        s.recv(order(5, "E"), h.t0); // gap: queued, ResendRequest for 2..
        let (numbers, out) = operate(&mut s, SequenceCommand::SetNextIncoming(5), &h);
        assert_eq!(types(&out), ["ExecutionReport"]);
        // The reply gives the numbers after the queued order was processed and answered.
        assert_eq!(numbers.unwrap(), numbers_after(&s));
        assert_eq!(s.peer().log.next_incoming(), 6);
        assert!(s.resend.is_none());
    }

    #[test]
    fn skipping_incoming_past_a_gap_ends_the_resend() {
        let h = Harness::new();
        let mut s = h.logged_on();
        s.recv(client(5, MsgType::Heartbeat), h.t0); // gap: ResendRequest for 2..
        assert!(s.resend.is_some());
        operate(&mut s, SequenceCommand::SetNextIncoming(6), &h).0.unwrap();
        assert!(s.resend.is_none());
        assert!(s.recv(client(6, MsgType::Heartbeat), h.t0).is_empty());
    }

    #[test]
    fn skipping_incoming_within_a_gap_restarts_the_resend_timeout() {
        let h = Harness::new();
        let mut s = h.logged_on();
        s.recv(order(5, "E"), h.t0); // gap: ResendRequest for 2..
        operate_at(&mut s, SequenceCommand::SetNextIncoming(4), h.at(50)).0.unwrap();
        assert!(!types(&s.timer(h.at(60))).contains(&"ResendRequest".to_string()), "timed from 50 s");
        // Keep the link up after the TestRequest sent at 60 s; a duplicate is no progress.
        s.recv(resend_of(client(1, MsgType::Heartbeat)), h.at(80));
        let out = s.timer(h.at(110));
        let retry = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ResendRequest).expect("retried");
        assert_eq!(retry.get(tags::BEGIN_SEQ_NO), Some("4"));
    }

    #[test]
    fn reset_is_refused_while_connected() {
        let h = Harness::new();
        let mut s = h.logged_on();
        assert!(matches!(operate(&mut s, SequenceCommand::Reset, &h).0, Err(SequenceError::Connected)));
    }

    #[test]
    fn operator_commands_are_answered_during_logon_not_queued() {
        let h = Harness::new();
        let mut s = h.initiator(false);
        s.connect(h.t0); // awaiting the Logon reply
        let (result, out) = operate(&mut s, SequenceCommand::SetNextOutgoing(50), &h);
        assert_eq!(result.unwrap(), numbers(1, 50));
        assert!(out.is_empty(), "no SequenceReset before logon");
    }
}

// ---- Malformed messages ----

/// `msg` as the decoder delivers it with `raw` appended as a malformed body field.
fn with_raw_field(msg: Message, raw: &[u8]) -> Message {
    let wire = crate::codec::frame_with_raw_field(&msg, raw);
    match crate::codec::decode(&wire) {
        crate::codec::Decoded::Message(msg, _) => msg,
        other => panic!("not decoded: {other:?}"),
    }
}

#[test]
fn a_malformed_message_is_rejected_and_its_sequence_number_used() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(with_raw_field(order(2, "A"), b"x5=1"), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    let reject = sent(&out)[0];
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("0"));
    assert_eq!(reject.get(tags::REF_TAG_ID), None);
    assert_eq!(reject.get(tags::TEXT), Some("Invalid tag 'x5'"));
    assert_eq!(h.app.received(), 0);
    assert_eq!(types(&s.recv(order(3, "B"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn a_non_utf8_value_is_rejected_naming_its_tag() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let out = s.recv(with_raw_field(order(2, "A"), b"58=\xff"), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    assert_eq!(sent(&out)[0].get(tags::REF_TAG_ID), Some("58"));
    assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("6"));
    assert_eq!(h.app.received(), 0);
}

#[test]
fn a_resent_malformed_message_no_longer_stalls_recovery() {
    let h = Harness::new();
    let mut s = h.logged_on();
    // 2 is lost; 3 reveals the gap and is queued.
    assert_eq!(types(&s.recv(order(3, "C"), h.t0)), ["ResendRequest"]);
    // The resend of 2 is malformed: rejected, and recovery moves past it to the queued 3.
    let resent = with_raw_field(resend_of(order(2, "B")), b"x5=1");
    assert_eq!(types(&s.recv(resent, h.t0)), ["Reject", "ExecutionReport"]);
    assert!(s.recv(resend_of(order(3, "C")), h.t0).is_empty(), "a duplicate now");
    assert_eq!(types(&s.recv(order(4, "D"), h.t0)), ["ExecutionReport"]);
    assert_eq!(h.app.received(), 2, "C and D");
    // The resend is over, so a new gap is requested afresh.
    assert_eq!(types(&s.recv(order(6, "F"), h.t0)), ["ResendRequest"]);
}

#[test]
fn malformed_sequence_resets_are_rejected_not_applied() {
    let h = Harness::new();
    let mut s = h.logged_on();
    // Reset mode.
    let reset = client(2, MsgType::SequenceReset).with(tags::NEW_SEQ_NO, "10");
    assert_eq!(types(&s.recv(with_raw_field(reset, b"x5=1"), h.t0)), ["Reject"]);
    // Gap fill at the expected number: reset mode used no sequence number, so 2 is still in sequence.
    let fill = client(2, MsgType::SequenceReset).with(tags::GAP_FILL_FLAG, "Y").with(tags::NEW_SEQ_NO, "10");
    assert_eq!(types(&s.recv(with_raw_field(fill, b"x5=1"), h.t0)), ["Reject"]);
    // Neither applied: 3 is next, not 10.
    assert_eq!(types(&s.recv(order(3, "A"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn a_malformed_logon_is_refused_on_both_roles() {
    let h = Harness::new();
    let mut s = h.session();
    assert_eq!(types(&s.recv(with_raw_field(logon(1), b"x5=1"), h.t0)), ["DISCONNECT"]);
    let mut i = h.initiator(false);
    i.connect(h.t0);
    assert_eq!(types(&i.recv(with_raw_field(logon(1), b"x5=1"), h.t0)), ["DISCONNECT"]);
    assert!(h.app.connections.lock().unwrap().is_empty(), "refused before verify_logon");
}

#[test]
fn a_malformed_message_ahead_of_a_gap_triggers_a_resend() {
    let h = Harness::new();
    let mut s = h.logged_on();
    // Its content is ignored: the gap is handled as for any message, with no Reject.
    assert_eq!(types(&s.recv(with_raw_field(order(3, "C"), b"x5=1"), h.t0)), ["ResendRequest"]);
}

#[test]
fn a_malformed_resend_request_is_rejected_not_answered() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our seq 1: Logon
    s.recv(order(2, "A"), h.t0); // our 2: ExecutionReport
    let req = client(3, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0");
    assert_eq!(types(&s.recv(with_raw_field(req, b"x5=1"), h.t0)), ["Reject"]);
}

#[test]
fn a_malformed_resend_request_ahead_of_a_gap_is_not_answered() {
    let h = Harness::new();
    let mut s = h.logged_on(); // our seq 1: Logon
    s.recv(order(2, "A"), h.t0); // our 2: ExecutionReport
    let req = client(5, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, "1").with(tags::END_SEQ_NO, "0");
    // Only our own ResendRequest for the gap: no gap fill and nothing resent.
    assert_eq!(types(&s.recv(with_raw_field(req, b"x5=1"), h.t0)), ["ResendRequest"]);
}

// ---- Resend timeout ----

#[test]
fn an_unanswered_resend_request_is_retried_once_then_the_session_logs_out() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(5, "E"), h.t0)), ["ResendRequest"]);
    let out = s.timer(h.at(60));
    let retry = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ResendRequest).expect("retried");
    assert_eq!(retry.get(tags::BEGIN_SEQ_NO), Some("2"));
    assert_eq!(retry.get(tags::END_SEQ_NO), Some("0"));
    // Any message, even a duplicate, keeps the link up after the TestRequest sent at 60 s, but isn't progress.
    assert!(s.recv(resend_of(client(1, MsgType::Heartbeat)), h.at(90)).is_empty());
    let out = s.timer(h.at(120));
    let logout = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::Logout).expect("logged out");
    assert!(logout.get(tags::TEXT).unwrap().contains("ResendRequest from 2 unanswered"));
}

/// The ResendRequests among `actions`, as `begin..end`.
fn resend_requests(actions: &[Action]) -> Vec<String> {
    sent(actions)
        .into_iter()
        .filter(|m| m.msg_type() == MsgType::ResendRequest)
        .map(|m| format!("{}..{}", m.get(tags::BEGIN_SEQ_NO).unwrap(), m.get(tags::END_SEQ_NO).unwrap()))
        .collect()
}

/// A logged-on session that asks for at most `chunk` messages per ResendRequest.
fn chunking(chunk: u64) -> (Harness, Session) {
    let mut h = Harness::new();
    h.config.resend_request_chunk = Some(chunk);
    let s = h.logged_on();
    (h, s)
}

#[test]
fn resend_requests_ask_for_a_chunk_at_a_time() {
    let (h, mut s) = chunking(2);
    // 2..6 are missing: the first chunk, then the next as each arrives, the last to the end.
    assert_eq!(resend_requests(&s.recv(order(7, "G"), h.t0)), ["2..3"]);
    assert!(resend_requests(&s.recv(resend_of(order(2, "B")), h.t0)).is_empty());
    assert_eq!(resend_requests(&s.recv(resend_of(order(3, "C")), h.t0)), ["4..5"]);
    s.recv(resend_of(order(4, "D")), h.t0);
    assert_eq!(resend_requests(&s.recv(resend_of(order(5, "E")), h.t0)), ["6..0"]);
    let out = s.recv(resend_of(order(6, "F")), h.t0);
    assert!(resend_requests(&out).is_empty());
    assert_eq!(h.app.received(), 6, "2 to 7, the queued 7 included");
}

#[test]
fn a_gap_fill_past_the_chunk_asks_for_the_next_from_where_it_ends() {
    let (h, mut s) = chunking(2);
    s.recv(order(9, "I"), h.t0); // 2..3 asked for
    assert_eq!(resend_requests(&s.recv(gap_fill(2, 5), h.t0)), ["5..6"]);
}

#[test]
fn an_unanswered_chunk_is_asked_for_again() {
    let (h, mut s) = chunking(2);
    s.recv(order(7, "G"), h.t0);
    assert_eq!(resend_requests(&s.timer(h.at(60))), ["2..3"]);
}

/// Any BeginString that can be sent is accepted, a venue's own included; one that can't isn't.
#[test]
fn a_begin_string_that_cannot_be_sent_is_refused() {
    for ok in ["FIX.4.2", "FIX.4.3", "FIX.4.4.VENUE"] {
        assert_eq!(SessionConfig::new(ok, "GATEWAY").check(), Ok(()), "{ok}");
    }
    for bad in ["", "FIX.4.4\x01", "FIX=4.4"] {
        let err = SessionConfig::new(bad, "GATEWAY").check().unwrap_err();
        assert!(err.to_string().starts_with("begin_string"), "{bad:?}: {err}");
    }
}

#[test]
fn a_resend_request_chunk_of_zero_is_refused() {
    let mut config = SessionConfig::new("FIX.4.4", "GATEWAY");
    config.resend_request_chunk = Some(0);
    assert!(config.check().unwrap_err().to_string().contains("resend_request_chunk"));
}

#[test]
fn progress_restarts_the_resend_timeout() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(5, "E"), h.t0); // ResendRequest for 2..
    s.recv(resend_of(order(2, "B")), h.at(50));
    // 60 s after the request, but only 10 s after progress: no retry.
    assert!(!types(&s.timer(h.at(60))).contains(&"ResendRequest".to_string()));
    let out = s.timer(h.at(110));
    let retry = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ResendRequest).expect("retried");
    assert_eq!(retry.get(tags::BEGIN_SEQ_NO), Some("3"), "from what's still missing");
}

#[test]
fn a_completed_resend_has_no_timeout() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(3, "C"), h.t0); // ResendRequest for 2..
    s.recv(resend_of(order(2, "B")), h.t0);
    s.recv(resend_of(order(3, "C")), h.t0);
    // No inbound traffic since: a TestRequest goes out at 60 s, but nothing about the resend.
    let out = s.timer(h.at(60));
    assert!(!types(&out).iter().any(|t| t == "ResendRequest" || t == "Logout"), "{:?}", types(&out));
}

#[test]
fn progress_after_a_retry_allows_another_retry() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(5, "E"), h.t0); // ResendRequest for 2..
    assert!(types(&s.timer(h.at(60))).contains(&"ResendRequest".to_string()), "retried");
    s.recv(resend_of(order(2, "B")), h.at(90)); // progress
    let out = s.timer(h.at(150));
    assert!(!types(&out).contains(&"Logout".to_string()), "{:?}", types(&out));
    let retry = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ResendRequest).expect("retried again");
    assert_eq!(retry.get(tags::BEGIN_SEQ_NO), Some("3"));
    // Keep the link up after the TestRequest sent at 150 s; this is no progress.
    s.recv(resend_of(client(1, MsgType::Heartbeat)), h.at(180));
    let out = s.timer(h.at(210));
    let logout = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::Logout).expect("logged out");
    assert!(logout.get(tags::TEXT).unwrap().contains("ResendRequest from 3 unanswered"));
}

// ---- Timer deadlines ----

#[test]
fn next_deadline_while_awaiting_logon_is_the_logon_timeout() {
    let h = Harness::new();
    let s = h.session();
    assert_eq!(s.next_deadline(), Some(h.t0 + h.config.logon_timeout));
}

#[test]
fn next_deadline_when_idle_is_the_heartbeat() {
    let h = Harness::new();
    let s = h.logged_on();
    // The Heartbeat at 30 s comes before the TestRequest at 36 s.
    assert_eq!(s.next_deadline(), Some(h.at(30)));
}

#[test]
fn next_deadline_moves_with_sends_and_receives() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.timer(h.at(30))), ["Heartbeat"]);
    // The next Heartbeat is due at 60 s; the TestRequest is still due at 36 s.
    assert_eq!(s.next_deadline(), Some(h.at(36)));
    s.recv(client(2, MsgType::Heartbeat), h.at(35)); // TestRequest now due at 71 s
    assert_eq!(s.next_deadline(), Some(h.at(60)));
    s.command(send_command("A"), h.at(50)); // Heartbeat now due at 80 s
    assert_eq!(s.next_deadline(), Some(h.at(71)));
}

#[test]
fn next_deadline_waits_for_an_outstanding_test_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Heartbeat), h.at(29));
    assert_eq!(types(&s.timer(h.at(30))), ["Heartbeat"]);
    assert_eq!(types(&s.timer(h.at(60))), ["Heartbeat"]);
    assert_eq!(s.next_deadline(), Some(h.at(65)), "36 s after the last message received");
    // The TestRequest is a send too, so no Heartbeat goes with it.
    assert_eq!(types(&s.timer(h.at(65))), ["TestRequest"]);
    s.command(send_command("A"), h.at(80)); // Heartbeat now due at 110 s
    assert_eq!(s.next_deadline(), Some(h.at(95)), "the answer is due within HeartBtInt");
    s.recv(client(3, MsgType::Heartbeat).with(tags::TEST_REQ_ID, "TEST1"), h.at(90));
    assert_eq!(s.next_deadline(), Some(h.at(110)), "answered: the next TestRequest is due at 126 s");
}

#[test]
fn next_deadline_includes_an_unanswered_resend_request() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(5, "E"), h.t0)), ["ResendRequest"]);
    // Traffic both ways at 50 s puts the Heartbeat (80 s) and TestRequest (86 s) after the retry.
    let keep_alive = |s: &mut Session, at| {
        s.command(send_command("A"), at);
        s.recv(resend_of(client(1, MsgType::Heartbeat)), at);
    };
    keep_alive(&mut s, h.at(50));
    assert_eq!(s.next_deadline(), Some(h.at(60)));
    // Progress at 55 s restarts the timeout.
    s.recv(resend_of(order(2, "B")), h.at(55));
    keep_alive(&mut s, h.at(100));
    assert_eq!(s.next_deadline(), Some(h.at(115)));
    let out = s.timer(h.at(115));
    assert!(types(&out).contains(&"ResendRequest".to_string()), "{:?}", types(&out));
}

#[test]
fn next_deadline_after_logout_is_the_logout_timeout() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.command(Command::Logout(None), h.at(3));
    assert_eq!(s.next_deadline(), Some(h.at(3) + h.config.logout_timeout));
}

#[test]
fn closed_session_has_no_deadline() {
    let h = Harness::new();
    let mut s = h.session();
    assert_eq!(types(&s.timer(h.at(10))), ["DISCONNECT"]);
    assert_eq!(s.next_deadline(), None);
}

/// `Duration::MAX` means "never": no deadline, rather than an overflow.
#[test]
fn a_timeout_of_duration_max_has_no_deadline() {
    let mut h = Harness::new();
    h.config.logon_timeout = Duration::MAX;
    h.config.logout_timeout = Duration::MAX;
    let mut s = h.session();
    assert_eq!(s.next_deadline(), None);
    assert!(s.timer(h.at(1_000_000)).is_empty());

    s.recv(logon(1), h.t0);
    assert_eq!(s.next_deadline(), Some(h.at(30)), "the heartbeat deadlines are unaffected");
    s.command(Command::Logout(None), h.at(1));
    assert_eq!(s.next_deadline(), None);
    assert!(s.timer(h.at(1_000_000)).is_empty());
}

#[test]
fn initiator_heartbeat_interval_is_whole_seconds_from_1_to_3600() {
    let config = |interval| {
        let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "GATEWAY"), "CLIENT");
        config.heartbeat_interval = interval;
        config
    };
    for secs in [1, 30, 3600] {
        config(Duration::from_secs(secs)).check().unwrap();
    }
    for bad in [Duration::ZERO, Duration::from_millis(500), Duration::from_millis(1500), Duration::from_secs(3601)] {
        let err = config(bad).check().unwrap_err().to_string();
        assert!(err.contains("heartbeat_interval"), "{bad:?}: {err}");
    }
}

#[test]
#[should_panic(expected = "heartbeat_interval")]
fn an_initiator_with_a_zero_heartbeat_interval_panics() {
    Harness::new().initiator_with(|config| config.heartbeat_interval = Duration::ZERO);
}

/// Calls `on_timer` at each deadline in turn until the session closes, first letting `before`
/// feed it events, and returns the number of deadlines. Checks that nothing happens just before
/// each deadline, and that something does at it.
fn walk_deadlines(s: &mut Session, mut before: impl FnMut(&mut Session, Instant)) -> usize {
    let mut steps = 0;
    while let Some(deadline) = s.next_deadline() {
        before(s, deadline);
        let Some(deadline) = s.next_deadline() else { break };
        let early = s.timer(deadline - Duration::from_millis(1));
        assert!(early.is_empty(), "acted early at step {steps}: {:?}", types(&early));
        assert_eq!(s.next_deadline(), Some(deadline), "an early call moved the deadline at step {steps}");
        s.timer(deadline);
        let next = s.next_deadline();
        assert!(next.is_none_or(|next| next > deadline), "no progress at step {steps}: {deadline:?} then {next:?}");
        steps += 1;
        assert!(steps < 100, "the session never closed");
    }
    steps
}

/// Calling `on_timer` at the deadline always acts, so the next deadline is later. Otherwise a
/// driver sleeping until the deadline would spin.
#[test]
fn on_timer_at_the_deadline_always_makes_progress() {
    let h = Harness::new();

    // Waiting for a Logon that never comes.
    assert_eq!(walk_deadlines(&mut h.session(), |_, _| {}), 1);

    // A silent counterparty: Heartbeat, TestRequest, disconnect.
    assert_eq!(walk_deadlines(&mut h.logged_on(), |_, _| {}), 3);

    // A chatty counterparty that falls silent after 20 messages.
    let h = Harness::new();
    let mut s = h.logged_on();
    let mut seq = 1;
    let steps = walk_deadlines(&mut s, |s, deadline| {
        if seq <= 20 {
            seq += 1;
            s.recv(client(seq, MsgType::Heartbeat), deadline - Duration::from_secs(1));
        }
    });
    assert!(steps > 20, "{steps}");

    // An unanswered ResendRequest, over a link kept up: retry, logout, logout timeout.
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(5, "E"), h.t0);
    let steps = walk_deadlines(&mut s, |s, deadline| {
        s.recv(resend_of(client(1, MsgType::Heartbeat)), deadline - Duration::from_secs(1));
    });
    assert!(steps > 3, "{steps}");

    // Logging out, with no reply.
    let h = Harness::new();
    let mut s = h.logged_on();
    s.command(Command::Logout(None), h.t0);
    assert_eq!(walk_deadlines(&mut s, |_, _| {}), 1);

    // An initiator, from Logon to a silent counterparty.
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.recv(logon(1), h.t0);
    assert_eq!(walk_deadlines(&mut s, |_, _| {}), 3);

    // The counterparty logs out mid-walk; the session answers and closes at once.
    let h = Harness::new();
    let mut s = h.logged_on();
    let mut step = 0;
    let steps = walk_deadlines(&mut s, |s, deadline| {
        step += 1;
        let mtype = if step == 5 { MsgType::Logout } else { MsgType::Heartbeat };
        s.recv(client(step + 1, mtype), deadline - Duration::from_secs(1));
    });
    assert_eq!(steps, 4);

    // A resend that makes progress for a while, then stalls: retry, logout, logout timeout.
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(order(8, "H"), h.t0); // ResendRequest for 2..
    let mut resent = 1;
    let steps = walk_deadlines(&mut s, |s, deadline| {
        let at = deadline - Duration::from_secs(1);
        if resent < 6 {
            resent += 1;
            s.recv(resend_of(order(resent, "R")), at);
        } else {
            s.recv(resend_of(client(1, MsgType::Heartbeat)), at);
        }
    });
    assert!(steps > 7, "{steps}");
}

// ---- FIXT.1.1 ----

fn fixt_config(versions: &[ApplVerId]) -> SessionConfig {
    versions.iter().fold(SessionConfig::new("FIXT.1.1", "GATEWAY"), |c, v| c.with_appl_ver_id(*v))
}

impl Harness {
    fn fixt(versions: &[ApplVerId]) -> Self {
        let mut h = Self::new();
        h.config = fixt_config(versions);
        h
    }
}

/// A FIXT.1.1 Logon from CLIENT, with DefaultApplVerID `version` if given.
fn fixt_logon(seq: u64, version: Option<&str>) -> Message {
    let logon = logon(seq).with(tags::BEGIN_STRING, "FIXT.1.1");
    match version {
        Some(v) => logon.with(tags::DEFAULT_APPL_VER_ID, v),
        None => logon,
    }
}

#[test]
fn fixt_configs_are_checked() {
    assert_eq!(fixt_config(&[ApplVerId::Fix50Sp2]).check(), Ok(()));
    assert_eq!(fixt_config(&[ApplVerId::Fix50Sp1, ApplVerId::Fix50Sp2]).check(), Ok(()));
    let err = |c: SessionConfig| c.check().unwrap_err().to_string();
    assert!(err(fixt_config(&[])).contains("with_appl_ver_id"));
    assert!(err(fixt_config(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp2])).contains("twice"));
    assert!(err(SessionConfig::new("FIX.4.4", "GATEWAY").with_appl_ver_id(ApplVerId::Fix44)).contains("FIXT"));
    assert_eq!(SessionConfig::new("FIX.4.4", "GATEWAY").check(), Ok(()));
}

#[test]
#[should_panic(expected = "with_appl_ver_id")]
fn a_fixt_session_without_a_version_panics() {
    let h = Harness::fixt(&[]);
    h.session();
}

/// An initiator with several versions offers the first as DefaultApplVerID.
#[test]
fn a_fixt_initiator_offers_its_first_version() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp1]);
    let mut s = h.initiator(false);
    let out = s.connect(h.t0);
    assert_eq!(sent(&out)[0].get(tags::DEFAULT_APPL_VER_ID), Some("9"));
}

#[test]
fn handles_report_the_application_version_while_connected() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
    let handle = h.registry.handle(SessionId {
        begin_string: "FIXT.1.1".into(),
        sender_comp_id: "GATEWAY".into(),
        target_comp_id: "CLIENT".into(),
    });
    assert_eq!(handle.appl_ver_id(), None);
    let mut s = h.initiator(false);
    s.connect(h.t0);
    assert_eq!(handle.appl_ver_id(), Some(ApplVerId::Fix50Sp2));
    drop(s);
    assert_eq!(handle.appl_ver_id(), None);
}

#[test]
fn fix4_handles_have_no_application_version() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    let ids = h.registry.sessions();
    assert_eq!(ids.len(), 1);
    assert_eq!(h.registry.handle(ids[0].clone()).appl_ver_id(), None);
}

#[test]
fn fixt_initiator_sends_its_version_and_logs_on_when_echoed() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
    let mut s = h.initiator(false);
    let out = s.connect(h.t0);
    assert_eq!(sent(&out)[0].get(tags::DEFAULT_APPL_VER_ID), Some("9"));
    assert_eq!(sent(&out)[0].get(tags::BEGIN_STRING), Some("FIXT.1.1"));
    s.recv(fixt_logon(1, Some("9")), h.t0);
    assert!(s.is_logged_on());
}

#[test]
fn fixt_initiator_refuses_a_reply_with_another_or_no_version() {
    for version in [Some("8"), None] {
        let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
        let mut s = h.initiator(false);
        s.connect(h.t0);
        let out = s.recv(fixt_logon(1, version), h.t0);
        assert_eq!(types(&out), ["DISCONNECT"], "{version:?}");
        assert!(h.app.connections.lock().unwrap().is_empty(), "refused before verify_logon");
    }
}

#[test]
fn fixt_acceptor_echoes_a_supported_version() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp1, ApplVerId::Fix50Sp2]);
    for (comp_id, version) in [("A", "8"), ("B", "9")] {
        let mut s = h.session();
        let out = s.recv(fixt_logon(1, Some(version)).with(tags::SENDER_COMP_ID, comp_id), h.t0);
        assert_eq!(sent(&out)[0].get(tags::DEFAULT_APPL_VER_ID), Some(version));
        assert!(s.is_logged_on());
        let handle = h.registry.handle(s.session_id().unwrap().clone());
        assert_eq!(handle.appl_ver_id().map(ApplVerId::code), Some(version));
    }
}

#[test]
fn fixt_acceptor_refuses_missing_unknown_or_unsupported_versions() {
    // "42" is not an ApplVerID code, so the Logon itself fails to parse.
    for version in [None, Some("42"), Some("6")] {
        let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
        let mut s = h.session();
        let out = s.recv(fixt_logon(1, version), h.t0);
        assert_eq!(types(&out), ["DISCONNECT"], "{version:?}");
        assert!(h.app.connections.lock().unwrap().is_empty(), "refused before verify_logon");
    }
}

#[test]
fn fixt_acceptor_refusals_name_the_supported_versions() {
    let refusal = |versions: &[ApplVerId]| {
        let s = Harness::fixt(versions).session();
        let request = s.validate_logon_request(&fixt_logon(1, Some("6"))).ok().unwrap();
        s.logon_terms(request.heartbeat, request.appl_ver_id).err().unwrap()
    };
    assert_eq!(refusal(&[ApplVerId::Fix50Sp2]), "DefaultApplVerID(1137) must be '9', not '6'");
    assert_eq!(
        refusal(&[ApplVerId::Fix50Sp1, ApplVerId::Fix50Sp2]),
        "DefaultApplVerID(1137) must be one of '8', '9', not '6'"
    );
}

#[test]
fn fix4_acceptors_ignore_default_appl_ver_id() {
    let h = Harness::new();
    let mut s = h.session();
    let out = s.recv(logon(1).with(tags::DEFAULT_APPL_VER_ID, "9"), h.t0);
    assert_eq!(sent(&out)[0].get(tags::DEFAULT_APPL_VER_ID), None);
    assert!(s.is_logged_on());
}

#[test]
fn fix4_logons_carry_no_application_version() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    assert_eq!(sent(&s.connect(h.t0))[0].get(tags::DEFAULT_APPL_VER_ID), None);
}

#[test]
fn fix4_initiators_ignore_default_appl_ver_id_in_the_reply() {
    let h = Harness::new();
    let mut s = h.initiator(false);
    s.connect(h.t0);
    s.recv(logon(1).with(tags::DEFAULT_APPL_VER_ID, "9"), h.t0);
    assert!(s.is_logged_on());
}

/// A logged-on FIXT acceptor using FIX 5.0 SP2.
fn fixt_session(h: &Harness) -> Session {
    let mut s = h.session();
    s.recv(fixt_logon(1, Some("9")), h.t0);
    assert!(s.is_logged_on());
    s
}

fn fixt_order(seq: u64, cl_ord_id: &str) -> Message {
    order(seq, cl_ord_id).with(tags::BEGIN_STRING, "FIXT.1.1")
}

// ---- Per-message application versions ----

/// A message may name, with ApplVerID(1128), any version the session supports.
#[test]
fn a_message_may_name_another_supported_version() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp1]);
    let mut s = fixt_session(&h);
    let out = s.recv(with_header(fixt_order(2, "A"), &[(tags::APPL_VER_ID, "8")]), h.t0);
    assert_eq!(types(&out), ["ExecutionReport"]);
    // FIX 5.0 isn't supported: rejected, naming the version it stated.
    let out = s.recv(with_header(fixt_order(3, "B"), &[(tags::APPL_VER_ID, "7")]), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("18"));
    assert_eq!(reject.get(tags::REF_APPL_VER_ID), Some("7"));
    assert_eq!(h.app.received(), 1);
}

/// A Reject or BusinessMessageReject names the version fields of the message it answers.
#[test]
fn rejects_name_the_version_of_the_message_they_answer() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp1]);
    let mut s = fixt_session(&h);
    let versioned = |msg| {
        with_header(msg, &[(tags::APPL_VER_ID, "8"), (tags::CSTM_APPL_VER_ID, "VENUE1"), (tags::APPL_EXT_ID, "99")])
    };
    let unsupported = client(2, MsgType::from_code("U1")).with(tags::BEGIN_STRING, "FIXT.1.1");
    let out = s.recv(versioned(unsupported), h.t0);
    let reject = sent(&out)[0];
    assert_eq!(reject.msg_type(), MsgType::BusinessMessageReject);
    assert_eq!(reject.get(tags::REF_APPL_VER_ID), Some("8"));
    assert_eq!(reject.get(tags::REF_CSTM_APPL_VER_ID), Some("VENUE1"));
    assert_eq!(reject.get(tags::REF_APPL_EXT_ID), Some("99"));
    let out = s.recv(versioned(fixt_order(3, "A").with(tags::TEXT, "")), h.t0);
    assert_eq!(sent(&out)[0].msg_type(), MsgType::Reject);
    assert_eq!(sent(&out)[0].get(tags::REF_APPL_VER_ID), Some("8"));
    // Without version fields, none are named.
    let out = s.recv(fixt_order(4, "A").with(tags::TEXT, ""), h.t0);
    assert_eq!(sent(&out)[0].get(tags::REF_APPL_VER_ID), None);
}

/// The application may send in any supported version; the default goes unstated, and a version
/// the session doesn't support is dropped. CstmApplVerID and ApplExtID pass through.
#[test]
fn the_application_may_send_in_another_supported_version() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp1]);
    let mut s = fixt_session(&h);
    let report =
        |version| Message::new(MsgType::ExecutionReport).with(tags::EXEC_ID, "E").with(tags::APPL_VER_ID, version);
    let out = s.command(Command::send(report("8")), h.t0);
    assert_eq!(sent(&out)[0].get(tags::APPL_VER_ID), Some("8"));
    assert_eq!(misplaced_header_field(sent(&out)[0]), None);
    let out = s.command(Command::send(report("9")), h.t0);
    assert_eq!(sent(&out)[0].get(tags::APPL_VER_ID), None, "the default goes unstated");
    let next = s.peer().log.next_outgoing();
    assert!(s.command(Command::send(report("7")), h.t0).is_empty(), "not a supported version");
    assert_eq!(s.peer().log.next_outgoing(), next, "and no number used");
    let custom = Message::new(MsgType::ExecutionReport)
        .with(tags::EXEC_ID, "E")
        .with(tags::CSTM_APPL_VER_ID, "VENUE1")
        .with(tags::APPL_EXT_ID, "99");
    let out = s.command(Command::send(custom), h.t0);
    assert_eq!(sent(&out)[0].get(tags::CSTM_APPL_VER_ID), Some("VENUE1"));
    assert_eq!(sent(&out)[0].get(tags::APPL_EXT_ID), Some("99"));
    assert_eq!(misplaced_header_field(sent(&out)[0]), None);
}

/// A message is resent in the version it was sent in, even after the counterparty logs on again
/// with another default.
#[test]
fn resends_keep_the_version_they_were_sent_in() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50Sp1]);
    let mut s = h.session();
    s.recv(fixt_logon(1, Some("8")), h.t0); // SP1 is the default this time
    let out = s.recv(fixt_order(2, "A"), h.t0); // our 2: an ExecutionReport in SP1
    assert_eq!(sent(&out)[0].get(tags::APPL_VER_ID), None);
    drop(s);

    let mut s = h.session();
    s.recv(fixt_logon(3, Some("9")), h.t0); // now SP2
    let request = client(4, MsgType::ResendRequest)
        .with(tags::BEGIN_STRING, "FIXT.1.1")
        .with(tags::BEGIN_SEQ_NO, "2")
        .with(tags::END_SEQ_NO, "2");
    let out = s.recv(request, h.t0);
    let resent = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::ExecutionReport).expect("resent");
    assert_eq!(resent.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(resent.get(tags::APPL_VER_ID), Some("8"), "stated, as it isn't the default now");
    assert_eq!(misplaced_header_field(resent), None);
}

#[test]
fn messages_in_another_application_version_are_rejected() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
    let mut s = fixt_session(&h);
    let out = s.recv(with_header(fixt_order(2, "A"), &[(tags::APPL_VER_ID, "8")]), h.t0);
    assert_eq!(types(&out), ["Reject"]);
    let reject = sent(&out)[0];
    assert_eq!(reject.get(tags::REF_TAG_ID), Some("1128"));
    assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("18"));
    assert_eq!(h.app.received(), 0);

    // Counted as received; the session's own version, stated or not, is delivered.
    assert_eq!(types(&s.recv(with_header(fixt_order(3, "B"), &[(tags::APPL_VER_ID, "9")]), h.t0)), ["ExecutionReport"]);
    assert_eq!(types(&s.recv(fixt_order(4, "C"), h.t0)), ["ExecutionReport"]);
    assert_eq!(h.app.received(), 2);
}

#[test]
fn session_messages_are_handled_whatever_their_appl_ver_id() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
    let mut s = fixt_session(&h);
    let request = client(2, MsgType::TestRequest).with(tags::BEGIN_STRING, "FIXT.1.1").with(tags::TEST_REQ_ID, "T1");
    let request = with_header(request, &[(tags::APPL_VER_ID, "8")]);
    let out = s.recv(request, h.t0);
    let hb = sent(&out)[0];
    assert_eq!(hb.msg_type(), MsgType::Heartbeat);
    assert_eq!(hb.get(tags::TEST_REQ_ID), Some("T1"));
    // The sequence number advanced: the next message is 3.
    assert_eq!(types(&s.recv(fixt_order(3, "A"), h.t0)), ["ExecutionReport"]);
}

#[test]
fn fix4_sessions_ignore_appl_ver_id() {
    let h = Harness::new();
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(with_header(order(2, "A"), &[(tags::APPL_VER_ID, "8")]), h.t0)), ["ExecutionReport"]);
}

/// An SP2-only session doesn't send SP1: the message is dropped rather than stripped of its
/// version, which would change what it means.
#[test]
fn outbound_messages_in_an_unsupported_version_are_dropped() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2]);
    let mut s = fixt_session(&h);
    let out = s.command(Command::send(Message::new(MsgType::ExecutionReport).with(tags::APPL_VER_ID, "8")), h.t0);
    assert!(out.is_empty());
}

// ---- Framing ----

/// The bytes `frame_into` appends to a buffer already holding some.
fn framed_into(
    s: &mut Session,
    body: &Message,
    seq: u64,
    sending_time: impl ToFix,
    orig: Option<&str>,
    state_version: bool,
) -> Vec<u8> {
    let mut out = b"junk".to_vec();
    s.frame_into(body, seq, sending_time, orig, state_version, &mut out);
    assert_eq!(&out[..4], b"junk");
    out.split_off(4)
}

/// Checks that `frame_into` writes what encoding `frame`'s message gives, with SendingTime as a
/// string and as a timestamp, and as sent and resent.
fn assert_frames_alike(s: &mut Session, body: &Message) {
    let micros = UtcTimestamp::from_fix("20260930-12:00:01.123456").unwrap();
    for orig in [None, Some("20260930-12:00:00.000")] {
        let expected = encode(&s.frame(body, 12, "20260930-12:00:01.000", orig)).unwrap();
        assert_eq!(framed_into(s, body, 12, "20260930-12:00:01.000", orig, false), expected);
        let expected = encode(&s.frame(body, 12345, micros, orig)).unwrap();
        assert_eq!(framed_into(s, body, 12345, micros, orig, false), expected);
    }
}

fn app_order() -> Message {
    Message::new(MsgType::NewOrderSingle)
        .with(tags::CL_ORD_ID, "A")
        .with(tags::SYMBOL, "MSFT")
        .with(tags::SIDE, "2")
        .with(tags::ORDER_QTY, "10")
        .with(tags::ORD_TYPE, "1")
}

#[test]
fn frame_into_writes_what_frame_and_encode_do() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let bodies = [
        app_order(),
        app_order().with(tags::POSS_RESEND, "Y"),
        app_order()
            .with(tags::ON_BEHALF_OF_COMP_ID, "HUB")
            .with(tags::DELIVER_TO_COMP_ID, "DESK")
            .with(tags::SENDER_SUB_ID, "TRADER"),
        app_order().with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"a\x01b\xffc"),
        // Fields the session sets itself are replaced or dropped.
        app_order()
            .with(tags::BEGIN_STRING, "FIX.4.2")
            .with(tags::BODY_LENGTH, "999")
            .with(tags::MSG_SEQ_NUM, "77")
            .with(tags::SENDER_COMP_ID, "SOMEONE")
            .with(tags::SENDING_TIME, "20000101-00:00:00")
            .with(tags::CHECK_SUM, "000"),
    ];
    for body in &bodies {
        assert_frames_alike(&mut s, body);
    }
}

#[test]
fn frame_into_writes_what_frame_and_encode_do_on_fixt() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50]);
    let mut s = fixt_session(&h);
    let bodies = [
        app_order(),
        app_order().with(tags::APPL_VER_ID, "9"),
        app_order().with(tags::APPL_VER_ID, "7"),
        app_order().with(tags::APPL_VER_ID, "7").with(tags::CSTM_APPL_VER_ID, "CUSTOM").with(tags::APPL_EXT_ID, "2"),
    ];
    for body in &bodies {
        assert_frames_alike(&mut s, body);
    }
}

/// For the stored copy of a multi-version session's message, the version is stated even when
/// it's the default.
#[test]
fn frame_into_can_state_the_default_version() {
    let h = Harness::fixt(&[ApplVerId::Fix50Sp2, ApplVerId::Fix50]);
    let mut s = fixt_session(&h);
    let bytes = framed_into(&mut s, &app_order(), 12, "20260930-12:00:01.000", None, true);
    let Decoded::Message(msg, len) = decode(&bytes) else { panic!("didn't decode: {bytes:?}") };
    assert_eq!(len, bytes.len());
    assert_eq!(msg.get(tags::APPL_VER_ID), Some("9"));
    assert!(misplaced_header_field(&msg).is_none());
    // Otherwise, it's the message frame gives.
    let expected = s.frame(&app_order(), 12, "20260930-12:00:01.000", None);
    let fields = |m: &Message| {
        m.fields()
            .filter(|(tag, _)| !matches!(*tag, tags::APPL_VER_ID | tags::BODY_LENGTH | tags::CHECK_SUM))
            .map(|(tag, value)| (tag, value.to_string()))
            .collect::<Vec<_>>()
    };
    assert_eq!(fields(&msg), fields(&expected));
    // A stated version is kept as stated.
    let bytes = framed_into(&mut s, &app_order().with(tags::APPL_VER_ID, "7"), 12, "20260930-12:00:01.000", None, true);
    let expected = encode(&s.frame(&app_order().with(tags::APPL_VER_ID, "7"), 12, "20260930-12:00:01.000", None));
    assert_eq!(bytes, expected.unwrap());
}

// ---- Throttling ----

impl Harness {
    /// A harness whose sessions send at most `messages` application messages per `per`.
    fn with_outbound_limit(messages: u32, per: Duration) -> Self {
        let mut h = Self::new();
        h.config.outbound_limit = Some(RateLimit::new(messages, per));
        h
    }

    fn at_millis(&self, millis: u64) -> Instant {
        self.t0 + Duration::from_millis(millis)
    }
}

#[test]
fn can_send_turns_false_at_the_limit_until_a_window_after_the_first() {
    let h = Harness::with_outbound_limit(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    assert!(s.can_send(h.t0), "the Logon doesn't count");
    assert_eq!(s.send_free_at(), None);
    assert_eq!(types(&s.command(send_command("A"), h.at(1))), ["NewOrderSingle"]);
    assert!(s.can_send(h.at(1)), "one of two");
    assert_eq!(s.send_free_at(), None);
    assert_eq!(types(&s.command(send_command("B"), h.at_millis(1_100))), ["NewOrderSingle"]);
    assert!(!s.can_send(h.at_millis(1_100)));
    assert!(!s.can_send(h.at(2) - Duration::from_nanos(1)));
    assert_eq!(s.send_free_at(), Some(h.at(2)), "the window frees a second after the first send");
    assert!(s.can_send(h.at(2)), "half-open: A no longer counts a whole window later");
    // Waiting for the window is the driver's: the timer's deadline is the next Heartbeat.
    assert_eq!(s.next_deadline(), Some(h.at_millis(31_100)));
    assert_eq!(types(&s.command(send_command("C"), h.at(2))), ["NewOrderSingle"]);
    assert!(!s.can_send(h.at(2)), "B and C fill the window");
    assert_eq!(s.send_free_at(), Some(h.at_millis(2_100)));
    assert_eq!(s.next_deadline(), Some(h.at(32)));
}

#[test]
fn admin_messages_and_resends_neither_count_nor_wait() {
    let h = Harness::with_outbound_limit(1, Duration::from_secs(60));
    let mut s = h.logged_on();
    let first = s.command(send_command("A"), h.t0); // our 2
    let original = sent(&first)[0].clone();
    assert!(!s.can_send(h.t0));
    // A TestRequest is answered, and a Heartbeat falls due, with the window full.
    let out = s.recv(client(2, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.at(1)); // our 3
    assert_eq!(types(&out), ["Heartbeat"]);
    assert_eq!(types(&s.timer(h.at(31))), ["Heartbeat"]); // our 4
    // So does a resend of everything, the queued send included.
    let out = s.recv(resend_request(3, 1), h.at(32));
    assert_eq!(covered(&out), [1, 2, 3, 4]);
    let resent = sent(&out).into_iter().find(|m| m.msg_type() == MsgType::NewOrderSingle).expect("resent");
    assert_eq!(resent.get(tags::POSS_DUP_FLAG), Some("Y"));
    assert_eq!(resent.get(tags::ORIG_SENDING_TIME), original.get(tags::SENDING_TIME));
    // None of them counted: the window still holds the one send, and frees when it expires.
    assert_eq!(s.send_free_at(), Some(h.at(60)));
    assert!(!s.can_send(h.at(59)));
    assert!(s.can_send(h.at(60)));
}

#[test]
fn replies_from_on_message_go_out_at_once_but_count() {
    let h = Harness::with_outbound_limit(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(2, "A"), h.at(1))), ["ExecutionReport"]);
    assert_eq!(types(&s.recv(order(3, "B"), h.at_millis(1_100))), ["ExecutionReport"]);
    assert!(!s.can_send(h.at_millis(1_100)), "two replies fill the window");
    // A reply can't wait, so it takes the window past the limit, and a queued send then waits
    // a window after the second reply rather than the first.
    assert_eq!(types(&s.recv(order(4, "C"), h.at_millis(1_200))), ["ExecutionReport"]);
    assert!(!s.can_send(h.at(2)));
    assert!(s.can_send(h.at_millis(2_100)));
    assert_eq!(s.send_free_at(), Some(h.at_millis(2_100)));
}

#[test]
fn the_sessions_own_business_rejects_neither_count_nor_wait() {
    let h = Harness::with_outbound_limit(1, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.command(send_command("A"), h.t0);
    assert!(!s.can_send(h.t0));
    let out = s.recv(client(2, MsgType::from_code("G")), h.at_millis(100));
    assert_eq!(types(&out), ["BusinessMessageReject"]);
    assert_eq!(s.send_free_at(), Some(h.at(1)), "still the send at t0");
    assert!(s.can_send(h.at(1)));
}

#[test]
fn a_refused_send_does_not_count() {
    let h = Harness::with_outbound_limit(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.command(send_command("A"), h.t0);
    let injected = Message::new(MsgType::ExecutionReport).with(tags::TEXT, "fine\x0139=8");
    assert!(s.command(Command::send(injected), h.at_millis(100)).is_empty(), "refused");
    assert!(s.can_send(h.at_millis(100)), "one of two");
    assert_eq!(s.send_free_at(), None);
}

#[test]
fn sends_are_not_held_once_logging_out() {
    let h = Harness::with_outbound_limit(1, Duration::from_secs(60));
    let mut s = h.logged_on();
    s.command(send_command("A"), h.t0);
    assert!(!s.can_send(h.at(1)));
    assert_eq!(types(&s.shutdown(Some("bye"), h.at(1))), ["Logout"]);
    // Held now, they would only be dropped a window later.
    assert!(s.can_send(h.at(1)));
    assert_eq!(s.send_free_at(), None);
    assert!(s.command(send_command("B"), h.at(1)).is_empty(), "dropped");
}

#[test]
fn without_a_limit_sends_never_wait() {
    let h = Harness::new();
    let mut s = h.session();
    assert!(s.can_send(h.t0));
    s.recv(logon(1), h.t0);
    for i in 0..100 {
        s.command(send_command(&format!("O{i}")), h.t0);
    }
    assert!(s.can_send(h.t0));
    assert!(s.outbound.is_none());
}

impl Harness {
    /// A harness whose sessions reject application messages received over `messages` per `per`.
    fn with_inbound_reject(messages: u32, per: Duration) -> Self {
        let mut h = Self::new();
        h.config.inbound_limit = Some(InboundLimit::Reject(RateLimit::new(messages, per)));
        h
    }
}

/// The ClOrdIDs the application has been handed, in order.
fn delivered(h: &Harness) -> Vec<String> {
    h.app.received.lock().unwrap().iter().map(|m| m.get(tags::CL_ORD_ID).unwrap().to_string()).collect()
}

/// Checks that `out` is the throttle's BusinessMessageReject of `seq_num`, alone.
fn assert_throttled(out: &[Action], seq_num: u64) {
    assert_eq!(types(out), ["BusinessMessageReject"], "{seq_num}");
    let reject = sent(out)[0];
    assert_eq!(reject.get(tags::REF_SEQ_NUM), Some(seq_num.to_string().as_str()));
    assert_eq!(reject.get(tags::REF_MSG_TYPE), Some("D"));
    assert_eq!(reject.get(tags::BUSINESS_REJECT_REASON), Some("0"), "Other");
    assert_eq!(reject.get(tags::TEXT), Some("throttle limit exceeded"));
}

#[test]
fn inbound_reject_answers_a_message_over_the_limit_with_a_business_reject() {
    let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(2, "A"), h.at(1))), ["ExecutionReport"]);
    assert_eq!(types(&s.recv(order(3, "B"), h.at_millis(1_400))), ["ExecutionReport"]);
    assert_throttled(&s.recv(order(4, "C"), h.at_millis(1_900)), 4);
    assert_eq!(delivered(&h), ["A", "B"], "the application never sees C");
    assert_eq!(s.peer().log.next_incoming(), 5, "C took its number, as an application's own reject does");
    // Half-open: A no longer counts a whole window later.
    assert_eq!(types(&s.recv(order(5, "D"), h.at(2))), ["ExecutionReport"]);
    assert_eq!(delivered(&h), ["A", "B", "D"]);
    assert_eq!(s.peer().log.next_incoming(), 6);
}

#[test]
fn inbound_reject_leaves_admin_messages_alone() {
    let h = Harness::with_inbound_reject(1, Duration::from_secs(60));
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(order(2, "A"), h.at(1))), ["ExecutionReport"]);
    assert!(s.recv(client(3, MsgType::Heartbeat), h.at(2)).is_empty(), "a Heartbeat is just received");
    let out = s.recv(client(4, MsgType::TestRequest).with(tags::TEST_REQ_ID, "x"), h.at(3));
    assert_eq!(types(&out), ["Heartbeat"]);
    assert_eq!(sent(&out)[0].get(tags::TEST_REQ_ID), Some("x"));
    assert_eq!(s.peer().log.next_incoming(), 5);
    // Neither counted: the window still holds A alone, and frees a minute after it.
    assert_throttled(&s.recv(order(5, "B"), h.at(60)), 5);
    assert_eq!(types(&s.recv(order(6, "C"), h.at(61))), ["ExecutionReport"]);
    assert_eq!(delivered(&h), ["A", "C"]);
}

#[test]
fn inbound_reject_delivers_what_the_counterparty_resends_at_our_request_but_counts_it() {
    let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    s.recv(order(3, "B"), h.at_millis(1_100));
    // A gap: 6 waits behind our ResendRequest for 4 and 5, with the window full.
    assert_eq!(types(&s.recv(order(6, "E"), h.at_millis(1_200))), ["ResendRequest"]);
    assert_eq!(types(&s.recv(resend_of(order(4, "C")), h.at_millis(1_300))), ["ExecutionReport"]);
    // Answering our request, the unflagged resend of 5 and the queued 6 are recovery too.
    let out = s.recv(order(5, "D"), h.at_millis(1_400));
    assert_eq!(types(&out), ["ExecutionReport", "ExecutionReport"]);
    assert!(s.resend.is_none());
    assert_eq!(delivered(&h), ["A", "B", "C", "D", "E"]);
    // Recovery counted, past full: with the request done, the window holds D and E, so new
    // messages are over the limit until they expire, not just until A does.
    assert_throttled(&s.recv(order(7, "F"), h.at_millis(1_500)), 7);
    // PossDupFlag alone is only the sender's claim: with no request of ours open, it's throttled
    // when the window is full, and counts when it isn't.
    assert_throttled(&s.recv(resend_of(order(8, "G")), h.at(2)), 8);
    assert_eq!(types(&s.recv(resend_of(order(9, "H")), h.at_millis(2_400))), ["ExecutionReport"]);
    assert_eq!(types(&s.recv(order(10, "I"), h.at_millis(2_450))), ["ExecutionReport"]);
    assert_throttled(&s.recv(order(11, "J"), h.at_millis(2_500)), 11);
    assert_eq!(delivered(&h), ["A", "B", "C", "D", "E", "H", "I"]);
}

#[test]
fn inbound_reject_counts_a_manufactured_gap_after_delivering_it() {
    // A hostile counterparty skips to 1000, and sends 997 new orders as the "resend" we ask for.
    // They're recovery we asked for as far as we can tell, so none is rejected; but they count,
    // so new traffic after them is.
    let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    assert_eq!(types(&s.recv(order(1_000, "Z"), h.at(1))), ["ResendRequest"]);
    for seq in 3..1_000 {
        let out = s.recv(order(seq, &format!("O{seq}")), h.at(1));
        assert!(!types(&out).contains(&"BusinessMessageReject".to_string()), "{seq}");
    }
    assert!(s.resend.is_none());
    assert_eq!(h.app.received(), 999, "2 to 1000");
    assert_throttled(&s.recv(order(1_001, "N"), h.at_millis(1_500)), 1_001);
}

#[test]
fn inbound_reject_delivers_messages_released_after_our_resend_request_is_done() {
    // More messages wait behind the gap than one window of deliveries holds, so the rest are
    // handled after a commit, by when the resend is complete: they were still asked for, so
    // they're delivered, and they count.
    let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    s.recv(order(3, "B"), h.at(1));
    let last = 4 + DELIVERIES_PER_COMMIT + 10;
    for seq in 5..=last {
        s.recv(order(seq, &format!("O{seq}")), h.at(1));
    }
    s.recv(resend_of(order(4, "C")), h.at_millis(1_500));
    assert!(s.resend.is_none());
    assert_eq!(h.app.received(), usize::try_from(last - 1).unwrap(), "all of them, in sequence");
    assert_eq!(s.peer().log.next_incoming(), last + 1);
    // A and B have expired, but what was released at 1.5s still counts.
    assert_throttled(&s.recv(order(last + 1, "N"), h.at(2)), last + 1);
}

#[test]
fn inbound_rejections_do_not_count() {
    let h = Harness::with_inbound_reject(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    s.recv(order(3, "B"), h.at_millis(1_100));
    for (i, seq_num) in (4..9).enumerate() {
        let at = h.at_millis(1_200 + 100 * u64::try_from(i).unwrap());
        assert_throttled(&s.recv(order(seq_num, "X"), at), seq_num);
    }
    // The window frees when A expires, as if the rejections hadn't happened, then again when B
    // does.
    assert_eq!(types(&s.recv(order(9, "C"), h.at(2))), ["ExecutionReport"]);
    assert_throttled(&s.recv(order(10, "X"), h.at_millis(2_050)), 10);
    assert_eq!(types(&s.recv(order(11, "D"), h.at_millis(2_100))), ["ExecutionReport"]);
    assert_eq!(delivered(&h), ["A", "B", "C", "D"]);
    assert_eq!(s.peer().log.next_incoming(), 12);
}

impl Harness {
    /// A harness whose sessions hold input once `messages` application messages have been
    /// received within `per`.
    fn with_inbound_delay(messages: u32, per: Duration) -> Self {
        let mut h = Self::new();
        h.config.inbound_limit = Some(InboundLimit::Delay(RateLimit::new(messages, per)));
        h
    }

    /// A logged-on acceptor session with `heartbeat` seconds' HeartBtInt.
    fn logged_on_with_heartbeat(&self, heartbeat: u64) -> Session {
        let mut s = self.session();
        let out = s.recv(logon(1).with(tags::HEART_BT_INT, heartbeat), self.t0);
        assert_eq!(sent(&out)[0].msg_type(), MsgType::Logon);
        s
    }
}

#[test]
fn inbound_delay_holds_input_after_the_message_that_fills_the_window() {
    let h = Harness::with_inbound_delay(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    assert_eq!(s.input_free_at(), None, "the Logon doesn't count");
    assert_eq!(types(&s.recv(order(2, "A"), h.at(1))), ["ExecutionReport"]);
    assert_eq!(s.input_free_at(), None, "one of two");
    // B fills the window and is still handled; only the next message waits.
    assert_eq!(types(&s.recv(order(3, "B"), h.at(1))), ["ExecutionReport"]);
    assert_eq!(delivered(&h), ["A", "B"]);
    assert_eq!(s.input_free_at(), Some(h.at(2)), "a whole window after A");
    assert!(s.ready_for_input(), "holding needs the time, so it's input_free_at's");
    // Waking for the window is the driver's: the timer's deadline is still the next Heartbeat.
    assert_eq!(s.next_deadline(), Some(h.at(31)));
    // Once the window frees up, the input it held goes on.
    assert_eq!(types(&s.recv(order(4, "C"), h.at(2))), ["ExecutionReport"]);
    assert_eq!(delivered(&h), ["A", "B", "C"]);
    assert_eq!(s.input_free_at(), None, "A and B no longer count: C alone is in the window");
    assert_eq!(s.peer().log.next_incoming(), 5);
    // Nothing went to the counterparty about it.
    assert_eq!(s.peer().log.next_outgoing(), 5, "the Logon and three ExecutionReports");
}

#[test]
fn inbound_delay_holds_nothing_once_logging_out() {
    let h = Harness::with_inbound_delay(1, Duration::from_secs(60));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    assert_eq!(s.input_free_at(), Some(h.at(61)));
    assert_eq!(types(&s.shutdown(Some("bye"), h.at(2))), ["Logout"]);
    // Held now, the counterparty's Logout reply would wait a minute.
    assert_eq!(s.input_free_at(), None);
    assert_eq!(types(&s.recv(client(3, MsgType::Logout), h.at(3))), ["DISCONNECT"]);
}

#[test]
fn inbound_delay_counts_every_application_message_as_it_arrives() {
    let h = Harness::with_inbound_delay(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    // A gap: 5 is queued behind our ResendRequest for 3 and 4, and counts as it arrives.
    assert_eq!(types(&s.recv(order(5, "D"), h.at(1))), ["ResendRequest"]);
    assert_eq!(s.input_free_at(), Some(h.at(2)), "A and D fill the window");
    // The resend we asked for waits for the window too, and counts.
    s.recv(resend_of(order(3, "B")), h.at(2));
    assert_eq!(s.input_free_at(), None, "B alone");
    // D is handed over behind C without counting again.
    assert_eq!(types(&s.recv(order(4, "C"), h.at(2))), ["ExecutionReport", "ExecutionReport"]);
    assert!(s.resend.is_none());
    assert_eq!(delivered(&h), ["A", "B", "C", "D"]);
    assert_eq!(s.input_free_at(), Some(h.at(3)), "B and C fill the window");
}

#[test]
fn inbound_delay_counts_a_manufactured_gap_as_it_is_read() {
    // A hostile counterparty skips to 1000, then sends new orders as the "resend" we ask for:
    // each counts as it arrives, so the driver paces them as any input.
    let h = Harness::with_inbound_delay(2, Duration::from_secs(1));
    let mut s = h.logged_on();
    s.recv(order(2, "A"), h.at(1));
    assert_eq!(types(&s.recv(order(1_000, "Z"), h.at(1))), ["ResendRequest"]);
    assert_eq!(s.input_free_at(), Some(h.at(2)), "the gap's far end counts");
    s.recv(order(3, "B"), h.at(2));
    assert_eq!(s.input_free_at(), None);
    s.recv(order(4, "C"), h.at(2));
    assert_eq!(s.input_free_at(), Some(h.at(3)), "the \"resend\" fills the window like anything else");
}

#[test]
fn time_spent_holding_input_is_not_a_stalled_resend() {
    // Our ResendRequest's answer waits behind the hold: its two-interval timeout counts from the
    // end of the hold, not from when the hold began.
    let h = Harness::with_inbound_delay(2, Duration::from_secs(10));
    let mut s = h.logged_on_with_heartbeat(1);
    s.recv(order(2, "A"), h.t0);
    assert_eq!(types(&s.recv(order(5, "D"), h.t0)), ["ResendRequest"]);
    assert_eq!(s.input_free_at(), Some(h.at(10)));
    for secs in 1..=10 {
        assert_eq!(types(&s.timer(h.at(secs))), ["Heartbeat"], "{secs}s");
    }
    assert!(types(&s.timer(h.at(12))).contains(&"ResendRequest".to_string()), "retried from the end of the hold");
}

#[test]
fn time_spent_holding_input_is_not_silence() {
    // A window longer than the heartbeat interval: the counterparty's Heartbeats wait unread
    // behind the hold, so it would otherwise be probed after 1.2s and dropped after 2.2s.
    let h = Harness::with_inbound_delay(1, Duration::from_secs(5));
    let mut s = h.logged_on_with_heartbeat(1);
    assert_eq!(types(&s.recv(order(2, "A"), h.t0)), ["ExecutionReport"]);
    assert_eq!(s.input_free_at(), Some(h.at(5)));
    for secs in 1..=5 {
        assert_eq!(types(&s.timer(h.at(secs))), ["Heartbeat"], "{secs}s: our own Heartbeats go on");
        assert_ne!(s.next_deadline(), None);
    }
    assert_eq!(s.next_deadline(), Some(h.at(6)), "the next Heartbeat; the probe waits for 6.2s");
    // Silence counts from the end of the hold: a TestRequest at 1.2 intervals after it, and a
    // disconnect an interval after that.
    assert_eq!(types(&s.timer(h.at(6))), ["Heartbeat"]);
    assert!(s.timer(h.at_millis(6_199)).is_empty());
    assert_eq!(types(&s.timer(h.at_millis(6_200))), ["TestRequest"]);
    assert_eq!(types(&s.timer(h.at_millis(7_200))), ["DISCONNECT"]);
}

#[test]
fn a_test_request_outstanding_when_input_is_held_does_not_drop_the_counterparty() {
    // The message that fills the window answers any TestRequest, as any message does; the hold
    // then keeps the counterparty from being probed again, or dropped, until it ends.
    let h = Harness::with_inbound_delay(1, Duration::from_secs(5));
    let mut s = h.logged_on_with_heartbeat(1);
    assert_eq!(types(&s.timer(h.at(1))), ["Heartbeat"]);
    assert_eq!(types(&s.timer(h.at_millis(1_200))), ["TestRequest"]);
    assert_eq!(types(&s.recv(order(2, "A"), h.at_millis(1_500))), ["ExecutionReport"]);
    assert_eq!(s.input_free_at(), Some(h.at_millis(6_500)));
    // Unanswered, the TestRequest would have dropped the counterparty at 2.2s.
    assert!(s.timer(h.at_millis(2_200)).is_empty());
    for millis in [2_500, 3_500, 4_500, 5_500, 6_500, 7_500] {
        assert_eq!(types(&s.timer(h.at_millis(millis))), ["Heartbeat"], "{millis}ms");
    }
    // Silence counts from the end of the hold, at 6.5s.
    assert_eq!(types(&s.timer(h.at_millis(7_700))), ["TestRequest"]);
}

/// A log sink shared between a subscriber and the test that reads it.
#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Captured {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Runs `f` with what it logs at DEBUG and above captured on this thread, and returns the lines.
fn logged(f: impl FnOnce()) -> Vec<String> {
    let captured = Captured::default();
    let sink = captured.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .with_writer(move || sink.clone())
        .finish();
    // While only one dispatcher exists, tracing caches a new callsite's interest from the default
    // of whichever thread reaches it first: another test's thread, with none, would turn it off
    // here too. A second keeps it asking every dispatcher, this one included.
    let _second = tracing::Dispatch::new(tracing::subscriber::NoSubscriber::default());
    tracing::subscriber::with_default(subscriber, f);
    let bytes = captured.0.lock().unwrap().clone();
    String::from_utf8(bytes).unwrap().lines().map(String::from).collect()
}

/// How many of `lines` contain every one of `parts`.
fn count(lines: &[String], parts: &[&str]) -> usize {
    lines.iter().filter(|line| parts.iter().all(|part| line.contains(part))).count()
}

#[test]
fn inbound_reject_warns_once_per_connection_and_logs_each_reject_at_debug() {
    let lines = logged(|| {
        // Each with its own store, so the second logs on afresh.
        for _ in 0..2 {
            let h = Harness::with_inbound_reject(1, Duration::from_secs(60));
            let mut s = h.logged_on();
            s.recv(order(2, "A"), h.at(1));
            for seq_num in 3..6 {
                assert_throttled(&s.recv(order(seq_num, "X"), h.at(2)), seq_num);
            }
        }
    });
    let warning = ["WARN", "inbound rate limit 1/1m reached: rejecting messages over it"];
    assert_eq!(count(&lines, &warning), 2, "once on each connection: {lines:#?}");
    assert_eq!(count(&lines, &["DEBUG", "rejecting a message over the inbound rate limit"]), 6, "{lines:#?}");
    assert_eq!(count(&lines, &["WARN", "business reject"]), 0, "no warning per message: {lines:#?}");
}

#[test]
fn an_application_business_reject_still_warns() {
    let h = Harness::with_inbound_reject(10, Duration::from_secs(60));
    let lines = logged(|| {
        let mut s = h.logged_on();
        assert_eq!(types(&s.recv(client(2, MsgType::from_code("G")), h.at(1))), ["BusinessMessageReject"]);
    });
    assert_eq!(count(&lines, &["WARN", "business reject"]), 1, "{lines:#?}");
    assert_eq!(count(&lines, &["rate limit"]), 0, "{lines:#?}");
}

#[test]
fn inbound_delay_warns_once_per_connection() {
    let lines = logged(|| {
        // Each with its own store, so the second logs on afresh.
        for _ in 0..2 {
            let h = Harness::with_inbound_delay(1, Duration::from_secs(1));
            let mut s = h.logged_on();
            // Each fills the window: three holds.
            for (seq_num, at) in [(2, 1), (3, 2), (4, 3)] {
                s.recv(order(seq_num, "A"), h.at(at));
                assert_eq!(s.input_free_at(), Some(h.at(at + 1)));
            }
        }
    });
    let warning = ["WARN", "inbound rate limit 1/1s reached: delaying input"];
    assert_eq!(count(&lines, &warning), 2, "once on each connection: {lines:#?}");
}

#[test]
fn held_sends_warn_once_per_connection() {
    let lines = logged(|| {
        // Each with its own store, so the second logs on afresh.
        for _ in 0..2 {
            let h = Harness::with_outbound_limit(1, Duration::from_secs(1));
            let mut s = h.logged_on();
            for at in [1, 2, 3] {
                s.command(send_command("A"), h.at(at));
                assert!(!s.can_send(h.at(at)));
                s.on_sends_held();
            }
        }
    });
    let warning = ["WARN", "outbound rate limit 1/1s reached: sends wait"];
    assert_eq!(count(&lines, &warning), 2, "once on each connection: {lines:#?}");
}

#[test]
fn rate_limits_out_of_bounds_are_refused() {
    let config = |outbound, inbound| {
        let mut config = SessionConfig::new("FIX.4.4", "GATEWAY");
        config.outbound_limit = outbound;
        config.inbound_limit = inbound;
        config
    };
    let good = RateLimit::new(100, Duration::from_secs(1));
    let zero = RateLimit { messages: 0, per: Duration::from_secs(1) };
    let empty = RateLimit { messages: 1, per: Duration::ZERO };
    assert_eq!(config(Some(good), Some(InboundLimit::Delay(good))).check(), Ok(()));
    assert_eq!(config(Some(good), Some(InboundLimit::Reject(good))).check(), Ok(()));
    let err = |c: SessionConfig| c.check().unwrap_err().to_string();
    assert!(err(config(Some(zero), None)).starts_with("outbound_limit: "), "{}", err(config(Some(zero), None)));
    let inbound = config(None, Some(InboundLimit::Delay(empty)));
    assert!(err(inbound.clone()).starts_with("inbound_limit: "), "{}", err(inbound));
    let inbound = config(None, Some(InboundLimit::Reject(zero)));
    assert!(err(inbound.clone()).starts_with("inbound_limit: "), "{}", err(inbound));
}

#[test]
fn a_cancel_grace_over_the_maximum_is_refused() {
    let config = |grace| {
        let mut config = SessionConfig::new("FIX.4.4", "GATEWAY");
        config.cancel_on_disconnect = Some(CancelOnDisconnect { trigger: CancelTrigger::Disconnect, grace });
        config
    };
    assert_eq!(config(Duration::ZERO).check(), Ok(()));
    assert_eq!(config(MAX_CANCEL_GRACE).check(), Ok(()));
    let err = config(MAX_CANCEL_GRACE + Duration::from_millis(1)).check().unwrap_err().to_string();
    assert!(err.starts_with("cancel_on_disconnect: "), "{err}");
}

#[test]
#[should_panic(expected = "invalid session configuration: outbound_limit")]
fn an_acceptor_with_an_invalid_outbound_limit_panics_when_built() {
    let mut config = SessionConfig::new("FIX.4.4", "GATEWAY");
    config.outbound_limit = Some(RateLimit { messages: 0, per: Duration::from_secs(1) });
    let _ = crate::Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(TestApp::default()));
}

#[test]
#[should_panic(expected = "invalid initiator configuration: inbound_limit")]
fn an_initiator_with_an_invalid_inbound_limit_panics_when_built() {
    let mut session = SessionConfig::new("FIX.4.4", "CLIENT");
    session.inbound_limit = Some(InboundLimit::Reject(RateLimit { messages: 1, per: Duration::ZERO }));
    let config = InitiatorConfig::new(session, "GATEWAY");
    let _ = crate::Initiator::new("127.0.0.1:1", config, Arc::new(MemoryStorage::new()), Arc::new(TestApp::default()));
}

// ---- Dictionary validation ----

#[cfg(feature = "validation")]
mod validation {
    use super::*;

    const DICT: &str = "<fix type='FIX' major='4' minor='4'>
     <messages>
      <message name='NewOrderSingle' msgtype='D' msgcat='app'>
       <field name='ClOrdID' required='Y'/><field name='Symbol' required='Y'/>
       <field name='Side' required='Y'/><field name='OrderQty' required='N'/>
       <field name='OrdType' required='Y'/>
      </message>
     </messages>
     <fields>
      <field number='11' name='ClOrdID' type='STRING'/><field number='55' name='Symbol' type='STRING'/>
      <field number='54' name='Side' type='CHAR'><value enum='1' description='BUY'/><value enum='2' description='SELL'/></field>
      <field number='38' name='OrderQty' type='QTY'/><field number='40' name='OrdType' type='CHAR'/>
     </fields>
    </fix>";

    fn harness() -> Harness {
        let mut h = Harness::new();
        let dict = turbojet_dictionary::Dictionary::from_xml(DICT).unwrap();
        h.config = h.config.clone().with_dictionary(&dict);
        h
    }

    #[test]
    fn invalid_messages_are_rejected_and_not_delivered() {
        let h = harness();
        let mut s = h.logged_on();
        let out = s.recv(order(2, "A").with(tags::SIDE, "9"), h.t0);
        assert_eq!(types(&out), ["Reject"]);
        let reject = sent(&out)[0];
        assert_eq!(reject.get(tags::REF_SEQ_NUM), Some("2"));
        assert_eq!(reject.get(tags::REF_TAG_ID), Some("54"));
        assert_eq!(reject.get(tags::SESSION_REJECT_REASON), Some("5"));
        assert_eq!(h.app.received(), 0);

        // It counted as received: the next message is 3, and a valid one is delivered.
        assert_eq!(types(&s.recv(order(3, "B"), h.t0)), ["ExecutionReport"]);
        assert_eq!(h.app.received(), 1);
    }

    #[test]
    fn session_messages_are_left_to_the_engine() {
        let h = harness();
        let mut s = h.logged_on();
        // Heartbeat isn't in the dictionary, but it's the engine's to check.
        assert!(s.recv(client(2, MsgType::Heartbeat), h.t0).is_empty());
        // An application message type the dictionary lacks is refused.
        let out = s.recv(client(3, MsgType::from_code("U7")), h.t0);
        assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("11"));
    }

    #[test]
    fn dictionaries_bind_to_the_preceding_version() {
        let dict = turbojet_dictionary::Dictionary::from_xml(DICT).unwrap();
        let config = SessionConfig::new("FIXT.1.1", "GATEWAY")
            .with_appl_ver_id(ApplVerId::Fix50Sp2)
            .with_dictionary(&dict)
            .with_appl_ver_id(ApplVerId::Fix50Sp1);
        assert!(config.validator.is_none());
        assert!(config.appl_versions[0].validator.is_some());
        assert!(config.appl_versions[1].validator.is_none());
        // A dictionary before any version is the bare validator, which FIXT sessions refuse.
        let early =
            SessionConfig::new("FIXT.1.1", "GATEWAY").with_dictionary(&dict).with_appl_ver_id(ApplVerId::Fix50Sp2);
        assert!(early.check().unwrap_err().to_string().contains("before"));
    }

    #[test]
    fn fixt_sessions_validate_with_the_negotiated_versions_dictionary() {
        let dict = turbojet_dictionary::Dictionary::from_xml(DICT).unwrap();
        // SP1 has the dictionary, SP2 none.
        let mut h = Harness::new();
        h.config = SessionConfig::new("FIXT.1.1", "GATEWAY")
            .with_appl_ver_id(ApplVerId::Fix50Sp1)
            .with_dictionary(&dict)
            .with_appl_ver_id(ApplVerId::Fix50Sp2);
        let bad = |seq| order(seq, "A").with(tags::BEGIN_STRING, "FIXT.1.1").with(tags::SIDE, "9");

        let mut sp1 = h.session();
        sp1.recv(fixt_logon(1, Some("8")).with(tags::SENDER_COMP_ID, "SP1"), h.t0);
        assert!(sp1.is_logged_on());
        let out = sp1.recv(bad(2).with(tags::SENDER_COMP_ID, "SP1"), h.t0);
        assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("5"));

        let mut sp2 = h.session();
        sp2.recv(fixt_logon(1, Some("9")), h.t0);
        assert_eq!(types(&sp2.recv(bad(2), h.t0)), ["ExecutionReport"]);
        // A message stating SP1 on the SP2 session is checked against SP1's dictionary.
        let out = sp2.recv(with_header(bad(3), &[(tags::APPL_VER_ID, "8")]), h.t0);
        assert_eq!(sent(&out)[0].get(tags::SESSION_REJECT_REASON), Some("5"));
    }
}

// ---- Per-counterparty settings ----

type Resolve = dyn Fn(&SessionConfig, &SessionId) -> Result<Counterparty, String> + Send + Sync;

/// Counterparties from a closure.
struct Resolver(Box<Resolve>);

impl Counterparties for Resolver {
    fn resolve(
        &self,
        base: &SessionConfig,
        id: &SessionId,
        _: &Message,
        _: &ConnectionInfo,
    ) -> Result<Counterparty, String> {
        (self.0)(base, id)
    }
}

impl Harness {
    /// An acceptor session whose counterparty's settings come from `resolve`.
    fn resolved(
        &self,
        resolve: impl Fn(&SessionConfig, &SessionId) -> Result<Counterparty, String> + Send + Sync + 'static,
    ) -> Session {
        let mut s = self.session();
        s.set_counterparties(Arc::new(Resolver(Box::new(resolve))));
        s
    }
}

/// A resolver giving every counterparty the acceptor's settings changed by `adjust`.
fn adjusted(
    adjust: impl Fn(&mut Counterparty) + Send + Sync + 'static,
) -> impl Fn(&SessionConfig, &SessionId) -> Result<Counterparty, String> + Send + Sync + 'static {
    move |base, _| {
        let mut counterparty = Counterparty::new(base.clone());
        adjust(&mut counterparty);
        Ok(counterparty)
    }
}

mod counterparty_tests {
    use super::*;

    #[test]
    fn the_resolver_is_asked_about_the_counterparty_logging_on() {
        let h = Harness::new();
        let asked = Arc::new(Mutex::new(Vec::new()));
        let mut s = h.resolved({
            let asked = asked.clone();
            move |base, id| {
                asked.lock().unwrap().push(id.clone());
                Ok(Counterparty::new(base.clone()))
            }
        });
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
        assert_eq!(
            asked.lock().unwrap().iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            ["FIX.4.4:GATEWAY->CLIENT"]
        );
    }

    #[test]
    fn resolved_settings_apply_after_the_logon() {
        let h = Harness::new();
        let mut s = h.resolved(adjusted(|c| c.config.max_latency = None));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
        assert!(s.recv(at_offset(client(2, MsgType::Heartbeat), -3600), h.t0).is_empty(), "latency unchecked");
        // The Logon itself is checked against the acceptor's, before the counterparty is known.
        let mut s = h.resolved(adjusted(|c| c.config.max_latency = None));
        assert_eq!(types(&s.recv(at_offset(logon(1), -3600), h.t0)), ["DISCONNECT"]);
    }

    #[test]
    fn a_resolved_rate_limit_applies() {
        let h = Harness::new();
        let mut s =
            h.resolved(adjusted(|c| c.config.outbound_limit = Some(RateLimit::new(1, Duration::from_secs(60)))));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
        assert!(s.can_send(h.t0));
        assert_eq!(types(&s.command(send_command("A"), h.t0)), ["NewOrderSingle"]);
        assert!(!s.can_send(h.t0), "one per minute");
    }

    #[test]
    fn heartbeat_intervals_outside_the_counterpartys_range_are_refused() {
        let h = Harness::new();
        let narrow = || adjusted(|c| c.heartbeat = Duration::from_secs(5)..=Duration::from_secs(10));
        let mut s = h.resolved(narrow());
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"], "30 s is outside 5..=10");
        let mut s = h.resolved(narrow());
        let mut ten = logon(1);
        ten.set(tags::HEART_BT_INT, 10u64);
        let out = s.recv(ten, h.t0);
        assert_eq!(sent(&out)[0].get(tags::HEART_BT_INT), Some("10"));
    }

    #[test]
    fn a_required_client_certificate_must_be_presented() {
        let h = Harness::new();
        let required = || adjusted(|c| c.require_client_certificate = true);
        let mut s = h.resolved(required());
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        assert!(h.app.connections.lock().unwrap().is_empty(), "refused before verify_logon");
        let mut s = h.resolved(required());
        s.set_connection_info(connection_info());
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
    }

    #[test]
    fn a_refusal_closes_the_connection_before_verify_logon() {
        let h = Harness::new();
        let mut s = h.resolved(|_, _| Err("not today".into()));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        assert!(h.app.connections.lock().unwrap().is_empty());
        assert!(h.app.events().is_empty());
        assert!(h.registry.sessions().is_empty());
    }

    #[test]
    fn a_panicking_resolver_refuses_the_logon() {
        let h = Harness::new();
        let mut s = h.resolved(|_, _| panic!("resolver panicked"));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        assert!(h.app.events().is_empty());
    }

    #[test]
    fn settings_that_cannot_apply_refuse_the_logon() {
        let h = Harness::new();
        let mut s = h.resolved(adjusted(|c| c.config.sender_comp_id = "OTHER".into()));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        let mut s = h.resolved(adjusted(|c| c.heartbeat = Duration::ZERO..=Duration::from_secs(30)));
        assert_eq!(types(&s.recv(logon(1), h.t0)), ["DISCONNECT"]);
        assert!(h.app.events().is_empty());
    }
}

// ---- Cancel on disconnect ----

/// A harness whose sessions cancel on disconnect with `trigger` and `grace`.
fn cancelling(trigger: CancelTrigger, grace: Duration) -> Harness {
    let mut h = Harness::new();
    h.config.cancel_on_disconnect = Some(CancelOnDisconnect { trigger, grace });
    h
}

#[test]
fn a_dropped_connection_starts_the_countdown() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let mut s = h.logged_on();
    s.on_disconnect(h.at(10));
    assert_eq!(h.registry.next_cancel_deadline(), Some(h.at(15)));
    h.registry.run_due_cancels(h.at(14));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost"]);
    h.registry.run_due_cancels(h.at(15));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "cancel CLIENT ConnectionLost"]);
    drop(s);
    h.registry.run_due_cancels(h.at(30));
    assert_eq!(h.app.events().len(), 3, "dropping the closed session starts nothing more");
}

#[test]
fn logging_back_on_stops_it() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let mut first = h.logged_on();
    first.on_disconnect(h.at(10));
    drop(first);
    let mut second = h.session();
    assert_eq!(types(&second.recv(logon(2), h.at(12))), ["Logon"]);
    assert_eq!(h.registry.next_cancel_deadline(), None);
    h.registry.run_due_cancels(h.at(20));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "logon CLIENT"]);
}

/// A reconnect refused after claiming the session (MsgSeqNum too low) didn't log on, so the
/// countdown goes on.
#[test]
fn a_refused_reconnect_does_not_stop_it() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let mut first = h.logged_on();
    first.recv(client(2, MsgType::Heartbeat), h.t0);
    first.on_disconnect(h.at(10));
    drop(first);
    let mut second = h.session();
    assert_eq!(types(&second.recv(logon(1), h.at(12))), ["Logout", "DISCONNECT"]);
    drop(second);
    h.registry.run_due_cancels(h.at(15));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "cancel CLIENT ConnectionLost"]);
}

#[test]
fn a_clean_logout_does_not_count_under_disconnect() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let mut s = h.logged_on();
    assert_eq!(types(&s.recv(client(2, MsgType::Logout), h.at(10))), ["Logout", "DISCONNECT"]);
    s.on_disconnect(h.at(10));
    assert_eq!(h.registry.next_cancel_deadline(), None);
    h.registry.run_due_cancels(h.at(100));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT CounterpartyLogout"]);
}

#[test]
fn a_clean_logout_counts_under_disconnect_or_logout() {
    let h = cancelling(CancelTrigger::DisconnectOrLogout, Duration::from_secs(5));
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Logout), h.at(10));
    s.on_disconnect(h.at(10));
    h.registry.run_due_cancels(h.at(15));
    assert_eq!(
        h.app.events(),
        ["logon CLIENT", "logout CLIENT CounterpartyLogout", "cancel CLIENT CounterpartyLogout"]
    );
}

/// Our own shutdown never counts, whatever the trigger.
#[test]
fn a_shutdown_does_not_count() {
    let h = cancelling(CancelTrigger::DisconnectOrLogout, Duration::from_secs(5));
    let mut s = h.logged_on();
    s.shutdown(None, h.at(10));
    s.recv(client(2, MsgType::Logout), h.at(11));
    assert_eq!(h.registry.next_cancel_deadline(), None);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT Shutdown"]);
}

#[test]
fn a_session_that_never_logged_on_does_not_count() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let mut s = h.session();
    s.on_disconnect(h.at(10));
    drop(s);
    assert_eq!(h.registry.next_cancel_deadline(), None);
    h.registry.run_all_cancels();
    assert!(h.app.events().is_empty());
}

/// With no grace period the cancel fires on the connection's own call, with nothing to drive.
#[test]
fn no_grace_cancels_at_once() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::ZERO);
    let mut s = h.logged_on();
    s.on_disconnect(h.at(10));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "cancel CLIENT ConnectionLost"]);
    assert_eq!(h.registry.next_cancel_deadline(), None);
}

#[test]
fn off_by_default() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.on_disconnect(h.at(10));
    assert_eq!(h.registry.next_cancel_deadline(), None);
    h.registry.run_all_cancels();
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost"]);
}

/// The application hears that the session ended before it's told to cancel, for every ending
/// that counts.
#[test]
fn the_cancel_comes_after_on_logout() {
    let h = cancelling(CancelTrigger::DisconnectOrLogout, Duration::ZERO);
    let mut s = h.logged_on();
    s.timer(h.at(36));
    s.timer(h.at(66));
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT HeartbeatTimeout", "cancel CLIENT HeartbeatTimeout"]);
}

/// A session dropped without `on_disconnect` (its task aborted) starts the countdown from when
/// it's dropped, read from the system clock.
#[test]
fn a_dropped_session_starts_the_countdown_too() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let s = h.logged_on();
    let before = Instant::now();
    drop(s);
    let after = Instant::now();
    let deadline = h.registry.next_cancel_deadline().expect("a countdown started");
    assert!(deadline >= before + Duration::from_secs(5), "{deadline:?} {before:?}");
    assert!(deadline <= after + Duration::from_secs(5), "{deadline:?} {after:?}");
    h.registry.run_due_cancels(deadline);
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "cancel CLIENT ConnectionLost"]);
}

/// With no grace period, a session dropped without `on_disconnect` cancels as it's dropped.
#[test]
fn no_grace_cancels_when_dropped() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::ZERO);
    drop(h.logged_on());
    assert_eq!(h.app.events(), ["logon CLIENT", "logout CLIENT ConnectionLost", "cancel CLIENT ConnectionLost"]);
    assert_eq!(h.registry.next_cancel_deadline(), None);
}

/// Sends through another session's handle from `on_cancel_on_disconnect`, as an application
/// cancelling orders might.
struct SendsOnCancel {
    registry: Arc<SessionRegistry>,
    events: Mutex<Vec<String>>,
}

impl Application for SendsOnCancel {
    fn on_cancel_on_disconnect(&self, session: &SessionId, ended: Disconnect) {
        let other = SessionId { target_comp_id: "OTHER".into(), ..session.clone() };
        let sent = self.registry.handle(other).send(Message::new(MsgType::NewOrderSingle));
        let not_connected = matches!(sent, Err(SendError::NotConnected(_)));
        self.events.lock().unwrap().push(format!("cancel {ended:?}, not connected: {not_connected}"));
    }
}

/// A cancel callback may take the registry's session lock (here, by sending through a handle)
/// under the tracker's: that's the one order the two locks are taken in, so it doesn't deadlock.
#[test]
fn a_cancel_can_send_through_a_handle() {
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let app = Arc::new(SendsOnCancel { registry: h.registry.clone(), events: Mutex::default() });
    let mut s = Session::acceptor(h.config.clone(), h.registry.clone(), app.clone(), h.t0).0;
    assert_eq!(types(&s.recv(logon(1), h.t0)), ["Logon"]);
    s.on_disconnect(h.at(10));
    h.registry.run_due_cancels(h.at(15));
    assert_eq!(*app.events.lock().unwrap(), ["cancel ConnectionLost, not connected: true"]);
}

/// Holds `on_cancel_on_disconnect` until the test releases it.
struct GatedCancel {
    events: Mutex<Vec<String>>,
    entered: Mutex<std::sync::mpsc::Sender<()>>,
    release: Mutex<std::sync::mpsc::Receiver<()>>,
    logged_on: Mutex<std::sync::mpsc::Sender<()>>,
}

impl Application for GatedCancel {
    fn on_logon(&self, _session: SessionHandle) {
        self.events.lock().unwrap().push("logon".into());
        self.logged_on.lock().unwrap().send(()).unwrap();
    }

    fn on_cancel_on_disconnect(&self, _session: &SessionId, _ended: Disconnect) {
        self.entered.lock().unwrap().send(()).unwrap();
        self.release.lock().unwrap().recv_timeout(Duration::from_secs(10)).expect("released");
        self.events.lock().unwrap().push("cancel".into());
    }
}

/// A logon of the session while its cancel runs on another thread waits for the cancel to return:
/// `on_logon` comes after it.
#[test]
fn a_logon_waits_for_a_cancel_in_progress() {
    use std::sync::mpsc;
    let h = cancelling(CancelTrigger::Disconnect, Duration::from_secs(5));
    let (entered, cancel_entered) = mpsc::channel();
    let (release_cancel, release) = mpsc::channel();
    let (logged_on, logons) = mpsc::channel();
    let app = Arc::new(GatedCancel {
        events: Mutex::default(),
        entered: Mutex::new(entered),
        release: Mutex::new(release),
        logged_on: Mutex::new(logged_on),
    });
    let session = || Session::acceptor(h.config.clone(), h.registry.clone(), app.clone(), h.t0).0;
    let mut first = session();
    first.recv(logon(1), h.t0);
    logons.recv().unwrap();
    first.on_disconnect(h.at(10));
    drop(first);

    std::thread::scope(|scope| {
        let canceller = scope.spawn(|| h.registry.run_due_cancels(h.at(15)));
        cancel_entered.recv_timeout(Duration::from_secs(10)).expect("the cancel started");
        let reconnect = scope.spawn(|| types(&session().recv(logon(2), h.at(12))));
        // The logon claims the session before it completes; once claimed, it's on its way to
        // waiting for the cancel.
        let give_up = Instant::now() + Duration::from_secs(10);
        while h.registry.sessions().is_empty() {
            assert!(Instant::now() < give_up, "the logon never claimed the session");
            std::thread::yield_now();
        }
        // Given time to, the logon still doesn't complete while the cancel runs. This wait can't
        // fail when the logon waits as it should; it's what catches one that doesn't.
        let early = logons.recv_timeout(Duration::from_millis(200));
        assert_eq!(early, Err(mpsc::RecvTimeoutError::Timeout), "no logon while the cancel runs");
        release_cancel.send(()).unwrap();
        canceller.join().unwrap();
        assert_eq!(reconnect.join().unwrap(), ["Logon"]);
    });
    assert_eq!(*app.events.lock().unwrap(), ["logon", "cancel", "logon"]);
}

// ---- Inbound admin messages shown to the application ----

/// The MsgTypes of the messages `on_admin_message` saw.
fn admin_types(app: &TestApp) -> Vec<MsgType> {
    app.admin.lock().unwrap().iter().map(Message::msg_type).collect()
}

#[test]
fn a_counterparty_reject_reaches_on_admin_message() {
    let h = Harness::new();
    let mut s = h.logged_on();
    let reject = client(2, MsgType::Reject).with(tags::REF_SEQ_NUM, 2u64).with(tags::TEXT, "unknown symbol");
    assert!(s.recv(reject, h.t0).is_empty());
    let admin = h.app.admin.lock().unwrap();
    assert_eq!(admin.len(), 1);
    assert_eq!(admin[0].get(tags::REF_SEQ_NUM), Some("2"));
    assert_eq!(admin[0].get(tags::TEXT), Some("unknown symbol"));
}

/// Each admin message is shown once, a ResendRequest that arrived ahead of a gap when it arrives
/// (it's answered then) and not again when the gap is filled; a Logon is left to verify_logon.
#[test]
fn inbound_admin_messages_reach_on_admin_message_once_each() {
    let h = Harness::new();
    let mut s = h.logged_on();
    s.recv(client(2, MsgType::Heartbeat), h.t0);
    s.recv(client(3, MsgType::TestRequest).with(tags::TEST_REQ_ID, "T"), h.t0);
    s.recv(client(5, MsgType::ResendRequest).with(tags::BEGIN_SEQ_NO, 1u64).with(tags::END_SEQ_NO, 0u64), h.t0);
    s.recv(gap_fill(4, 5), h.t0);
    s.recv(client(6, MsgType::SequenceReset).with(tags::NEW_SEQ_NO, 10u64), h.t0);
    s.recv(client(10, MsgType::Logout).with(tags::TEXT, "end of day"), h.t0);
    assert_eq!(
        admin_types(&h.app),
        [
            MsgType::Heartbeat,
            MsgType::TestRequest,
            MsgType::ResendRequest,
            MsgType::SequenceReset,
            MsgType::SequenceReset,
            MsgType::Logout
        ]
    );
    assert_eq!(h.app.admin.lock().unwrap()[5].get(tags::TEXT), Some("end of day"));
}

#[test]
fn an_admin_message_the_session_refuses_does_not_reach_on_admin_message() {
    let h = Harness::new();
    let mut s = h.logged_on();
    // MsgSeqNum too low, without PossDupFlag: the session logs out.
    let out = s.recv(client(1, MsgType::Heartbeat), h.t0);
    assert_eq!(types(&out), ["Logout", "DISCONNECT"]);
    assert!(admin_types(&h.app).is_empty());
}
