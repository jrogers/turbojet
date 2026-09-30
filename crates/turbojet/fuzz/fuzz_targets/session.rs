//! A session, acceptor or initiator, fed a sequence of fuzzed steps: a Logon, then inbound
//! messages with a valid header (so they get past the codec) but any MsgType, MsgSeqNum and body
//! fields, time passing, and the application or an operator acting on the session. Every message
//! the session sends must encode and decode cleanly and, apart from resends, go out in sequence.

#![no_main]

use std::sync::Arc;
use std::time::{Duration, Instant};

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tokio::sync::oneshot;
use turbojet::codec::{Decoded, decode};
use turbojet::message::{tags, utc_timestamp};
use turbojet::registry::{Command, SequenceCommand};
use turbojet::{
    ApplVerId, Application, Context, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType, Session,
    SessionConfig, SessionRegistry,
};
use turbojet_fuzz::{frame, round_trip};

/// Admin messages, then application messages.
const MSG_TYPES: &[&str] = &["0", "1", "2", "3", "4", "5", "A", "D", "8", "j", "AE", "n"];

/// Tags the session reads: sequencing, resends, logon, logout, rejects and FIXT versions, and a
/// few order fields.
const TAGS: &[u32] = &[
    7, 16, 36, 123, 43, 97, 122, 112, 45, 371, 372, 373, 58, 98, 108, 141, 789, 1128, 1129, 1137, 553, 554, 34, 49, 56,
    52, 35, 8, 9, 10, 11, 55, 54, 38, 40,
];

#[derive(Arbitrary, Debug)]
struct Input {
    fixt: bool,
    initiator: bool,
    /// The counterparty's Logon: extra fields after EncryptMethod and HeartBtInt.
    logon: Vec<Field>,
    heartbeat_secs: u8,
    steps: Vec<Step>,
}

#[derive(Arbitrary, Debug)]
enum Step {
    Inbound(Inbound),
    /// Seconds pass, then the session's timer runs.
    Elapse(u8),
    /// The application sends an order.
    Send,
    Logout,
    /// An operator changes sequence numbers.
    Sequence(Sequence),
}

#[derive(Arbitrary, Debug)]
struct Inbound {
    msg_type: u8,
    seq: Seq,
    poss_dup: bool,
    /// Header fields to leave out: bit 0 SenderCompID, 1 TargetCompID, 2 MsgSeqNum, 3 SendingTime.
    omit: u8,
    fields: Vec<Field>,
}

#[derive(Arbitrary, Debug)]
enum Sequence {
    Get,
    SetNextIncoming(Seq),
    SetNextOutgoing(u16),
    Reset,
}

#[derive(Arbitrary, Debug)]
struct Field {
    tag: Tag,
    value: Value,
}

#[derive(Arbitrary, Debug)]
enum Tag {
    Known(u8),
    Any(u32),
}

#[derive(Arbitrary, Debug)]
enum Value {
    Number(u32),
    /// Relative to the next MsgSeqNum the counterparty sends, as for NewSeqNo.
    Seq(Seq),
    /// That many before the next MsgSeqNum Turbojet sends, as for a ResendRequest's range.
    Sent(u8),
    Yes,
    No,
    Now,
    Text(String),
    Raw(Vec<u8>),
}

/// A MsgSeqNum, relative to the next one this harness expects the session to want.
#[derive(Arbitrary, Debug)]
enum Seq {
    Next,
    Ahead(u8),
    Behind(u8),
    Exactly(u32),
}

impl Seq {
    fn resolve(&self, next_in: u64) -> u64 {
        match *self {
            Seq::Next => next_in,
            Seq::Ahead(n) => next_in + u64::from(n),
            Seq::Behind(n) => next_in.saturating_sub(u64::from(n)),
            Seq::Exactly(n) => u64::from(n),
        }
    }
}

fn push_fields(body: &mut Vec<u8>, fields: &[Field], next_in: u64, next_out: u64) {
    for Field { tag, value } in fields {
        let tag = match *tag {
            Tag::Known(i) => TAGS[usize::from(i) % TAGS.len()],
            Tag::Any(tag) => tag,
        };
        body.extend_from_slice(format!("{tag}=").as_bytes());
        match value {
            Value::Number(n) => body.extend_from_slice(n.to_string().as_bytes()),
            Value::Seq(seq) => body.extend_from_slice(seq.resolve(next_in).to_string().as_bytes()),
            Value::Sent(n) => body.extend_from_slice(next_out.saturating_sub((*n).into()).to_string().as_bytes()),
            Value::Yes => body.push(b'Y'),
            Value::No => body.push(b'N'),
            Value::Now => body.extend_from_slice(utc_timestamp().as_bytes()),
            Value::Text(text) => body.extend_from_slice(text.as_bytes()),
            Value::Raw(raw) => body.extend_from_slice(raw),
        }
        body.push(1);
    }
}

/// Accepts every message but ExecutionReports, answering an order with an ExecutionReport.
struct App;

impl Application for App {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        match msg.msg_type() {
            MsgType::NewOrderSingle => {
                ctx.send(Message::new(MsgType::ExecutionReport).with_opt(tags::CL_ORD_ID, msg.get(tags::CL_ORD_ID)));
                Ok(())
            }
            MsgType::ExecutionReport => Err(MessageReject::unsupported_message_type()),
            _ => Ok(()),
        }
    }
}

/// Checks what the session sent, and takes it from its output. Returns false once it has
/// disconnected.
fn check(session: &mut Session, next_out: &mut u64) -> bool {
    let mut rest = session.output();
    while !rest.is_empty() {
        // The engine must never send garbage.
        let Decoded::Message(msg, len) = decode(rest) else {
            panic!("the session sent something that doesn't decode: {:?}", String::from_utf8_lossy(rest))
        };
        rest = &rest[len..];
        round_trip(&msg);
        let seq: u64 = msg.field(tags::MSG_SEQ_NUM).unwrap();
        // A Logon with ResetSeqNumFlag (answering an intraday reset) starts again at 1.
        if msg.msg_type() == MsgType::Logon && msg.flag(tags::RESET_SEQ_NUM_FLAG) {
            *next_out = 1;
        }
        if msg.flag(tags::POSS_DUP_FLAG) {
            assert!(seq < *next_out, "resent {seq}, but only sent up to {}: {msg}", *next_out - 1);
            continue;
        }
        assert_eq!(seq, *next_out, "{msg}");
        *next_out += 1;
        if msg.msg_type() == MsgType::SequenceReset && !msg.flag(tags::GAP_FILL_FLAG) {
            // An operator's SequenceReset-Reset: sending continues from NewSeqNo.
            *next_out = msg.field(tags::NEW_SEQ_NO).unwrap();
        }
    }
    session.clear_output();
    !session.is_closed()
}

/// A message from the counterparty, with a valid header except for the fields `omit` leaves out.
fn inbound(begin_string: &str, msg_type: &str, seq: u64, poss_dup: bool, omit: u8, fields: &[u8]) -> Option<Message> {
    let now = utc_timestamp();
    let mut body = format!("35={msg_type}\x01").into_bytes();
    let (seq, time) = (format!("34={seq}"), format!("52={now}"));
    for (bit, field) in ["49=CLIENT", "56=TJ", &seq, &time].into_iter().enumerate() {
        if omit & (1 << bit) == 0 {
            body.extend_from_slice(field.as_bytes());
            body.push(1);
        }
    }
    if poss_dup {
        body.extend_from_slice(format!("43=Y\x01122={now}\x01").as_bytes());
    }
    body.extend_from_slice(fields);
    match decode(&frame(begin_string, &body)) {
        Decoded::Message(msg, _) => Some(msg),
        _ => None,
    }
}

fuzz_target!(|input: Input| {
    let mut config = SessionConfig::new(if input.fixt { "FIXT.1.1" } else { "FIX.4.4" }, "TJ");
    if input.fixt {
        config = config.with_appl_ver_id(ApplVerId::Fix50Sp2);
    }
    let begin_string = config.begin_string.clone();
    let registry = Arc::new(SessionRegistry::new(Arc::new(MemoryStorage::new())));
    let mut now = Instant::now();
    let heartbeat = 1 + u32::from(input.heartbeat_secs % 60);
    let mut next_out = 1;
    let (mut session, _commands) = if input.initiator {
        let mut config = InitiatorConfig::new(config, "CLIENT");
        config.heartbeat_interval = Duration::from_secs(heartbeat.into());
        let (mut session, commands) = Session::initiator(&config, registry, Arc::new(App), now);
        session.on_connect(now);
        assert!(check(&mut session, &mut next_out));
        (session, commands)
    } else {
        Session::acceptor(config, registry, Arc::new(App), now)
    };

    // The counterparty's Logon, or its reply to Turbojet's.
    let mut fields = format!("98=0\x01108={heartbeat}\x01").into_bytes();
    if input.fixt {
        fields.extend_from_slice(b"1137=9\x01");
    }
    push_fields(&mut fields, &input.logon, 1, next_out);
    let Some(logon) = inbound(&begin_string, "A", 1, false, 0, &fields) else { return };
    session.on_message(&logon, now);
    if !check(&mut session, &mut next_out) {
        return;
    }

    let mut next_in: u64 = 2;
    for step in input.steps {
        match step {
            Step::Inbound(msg) => {
                let msg_type = MSG_TYPES[usize::from(msg.msg_type) % MSG_TYPES.len()];
                let seq = msg.seq.resolve(next_in);
                let mut fields = Vec::new();
                push_fields(&mut fields, &msg.fields, next_in, next_out);
                let Some(msg) = inbound(&begin_string, msg_type, seq, msg.poss_dup, msg.omit, &fields) else {
                    continue;
                };
                next_in = next_in.max(seq + 1);
                session.on_message(&msg, now);
            }
            Step::Elapse(secs) => {
                now += Duration::from_secs(secs.into());
                session.on_timer(now);
            }
            Step::Send => {
                let order = Message::new(MsgType::NewOrderSingle).with(tags::CL_ORD_ID, "X").with(tags::SYMBOL, "AAPL");
                session.on_command(Command::Send(order), now);
            }
            Step::Logout => session.on_command(Command::Logout(None), now),
            Step::Sequence(request) => {
                let request = match request {
                    Sequence::Get => SequenceCommand::Get,
                    Sequence::SetNextIncoming(seq) => SequenceCommand::SetNextIncoming(seq.resolve(next_in)),
                    Sequence::SetNextOutgoing(seq) => SequenceCommand::SetNextOutgoing(seq.into()),
                    Sequence::Reset => SequenceCommand::Reset,
                };
                let (reply, mut numbers) = oneshot::channel();
                session.on_command(Command::Sequence(request, reply), now);
                if !check(&mut session, &mut next_out) {
                    break;
                }
                // The operator may move either number anywhere, backwards included.
                if let Ok(Ok(numbers)) = numbers.try_recv() {
                    next_in = numbers.next_incoming;
                    next_out = numbers.next_outgoing;
                }
            }
        }
        if !check(&mut session, &mut next_out) {
            break;
        }
    }
});
