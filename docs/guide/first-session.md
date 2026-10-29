# A first session

Turbojet runs the FIX session layer: logon, sequence numbers, gap detection and resends,
heartbeats and TestRequests, logout and persistence. Your code is an `Application`, which sees
the application messages once the session has checked them and put them in sequence, and replies
or sends through the session.

## Dependencies

Typed application messages come from `turbojet-fix`, with a feature per FIX version: `fix40`,
`fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1` and `fix50sp2`. The engine runs on
[tokio](https://docs.rs/tokio) and reports through [tracing](https://docs.rs/tracing):

```toml
[dependencies]
turbojet = "0.7"
turbojet-fix = { version = "0.7", features = ["fix44"] }
tokio = { version = "1", features = ["full"] }
tracing-subscriber = "0.3"
```

Install a tracing subscriber, as `main` does below, to see why a session didn't connect or log
on. Without one, an initiator that fails to connect says so once on stderr and is silent after
that.

## Both roles in one program

An `Acceptor` listens for counterparties that log on to it; an `Initiator` connects out to one
counterparty and logs on, reconnecting as needed. Here an acceptor acknowledges orders as
`SERVER`, and an initiator logs on to it as `CLIENT`, sends an order and waits for the
acknowledgement. It's the `quickstart` example in the repository (`cargo run --example
quickstart`):

```rust
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use turbojet::fields::{Decimal, UtcTimestamp};
use turbojet::{
    Acceptor, Application, Context, Initiator, InitiatorConfig, MemoryStorage, Message, MessageReject, MsgType,
    SessionConfig, SessionHandle,
};
use turbojet_fix::fix44::{
    ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingle, NewOrderSingleRef, NewOrderSingleRequired,
    OrdStatus, OrdType, Side,
};

/// Acknowledges every order.
struct OrderDesk;

impl Application for OrderDesk {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        match msg.msg_type() {
            MsgType::NewOrderSingle => {
                // A missing or malformed field becomes a Reject(3) naming the tag.
                let order: NewOrderSingleRef = msg.parse()?;
                let mut ack = ExecutionReport::new(ExecutionReportRequired {
                    order_id: "O1".into(),
                    exec_id: "E1".into(),
                    exec_type: ExecType::New.into(),
                    ord_status: OrdStatus::New.into(),
                    side: order.side.into(),
                    leaves_qty: order.order_qty.unwrap_or_default(),
                    cum_qty: Decimal::ZERO,
                    avg_px: Decimal::ZERO,
                });
                ack.cl_ord_id = Some(order.cl_ord_id.into());
                ctx.send(ack)?;
                Ok(())
            }
            _ => Err(MessageReject::unsupported_message_type()),
        }
    }
}

/// Passes the session handle and inbound messages on to `main`.
struct Client {
    logons: mpsc::UnboundedSender<SessionHandle>,
    messages: mpsc::UnboundedSender<Message>,
}

impl Application for Client {
    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.logons.send(session.clone());
    }

    fn on_message(&self, _ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let _ = self.messages.send(msg.clone());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The engine reports through `tracing`: without a subscriber, its logs go nowhere.
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?.to_string();
    let config = SessionConfig::new("FIX.4.4", "SERVER");
    let server = Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(OrderDesk))?;
    tokio::spawn(server.clone().serve(listener));

    let (logons, mut logged_on) = mpsc::unbounded_channel();
    let (messages, mut received) = mpsc::unbounded_channel();
    let config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER").with_endpoint(addr);
    let app = Arc::new(Client { logons, messages });
    let client = Initiator::new(config, Arc::new(MemoryStorage::new()), app)?;
    tokio::spawn(client.clone().run());

    let session = logged_on.recv().await.ok_or("never logged on")?;
    let mut order = NewOrderSingle::new(NewOrderSingleRequired {
        cl_ord_id: "order-1".into(),
        side: Side::Buy.into(),
        transact_time: UtcTimestamp::now(),
        ord_type: OrdType::Market.into(),
    });
    order.symbol = Some("ACME".into());
    order.order_qty = Some(Decimal::from(100));
    let seq = session.send(order)?.await?;
    println!("sent order-1 as MsgSeqNum {seq}");

    let ack: ExecutionReport = received.recv().await.ok_or("no acknowledgement")?.parse()?;
    println!("acknowledged {}", ack.cl_ord_id.as_deref().unwrap_or("an order"));

    // Log out, and wait for each side's connections to close.
    client.shutdown(None).await;
    server.shutdown(None).await;
    Ok(())
}
```

The engine adds the standard header to everything sent: BeginString, CompIDs, MsgSeqNum and
SendingTime. TransactTime and other body timestamps are chrono times (`fields::UtcTimestamp`),
and prices and quantities are exact decimals (`fields::Decimal`). `MemoryStorage` keeps sequence
numbers and sent messages only while the process runs; [Storage and recovery][storage] covers
keeping them across restarts.

## The application

`Application::on_message` is the one callback you must write. Returning a `MessageReject`
answers the message with a Reject(3) or a BusinessMessageReject(j), and a field that is missing
or malformed converts into the matching Reject with `?`. The other callbacks are optional:

- `verify_logon` checks a counterparty's Logon, with its credentials and, over TLS, its
  certificate, and may refuse it.
- `to_admin` adds to the session messages we send: a venue's own Logon fields, say.
- `on_admin_message` shows each session message received other than a Logon, before the session
  acts on it: a counterparty's Reject of one of our messages (RefSeqNum and Text say which and
  why), a Logout's Text, and Heartbeats, TestRequests, ResendRequests and SequenceResets.
- `should_resend` is asked about each stored message the counterparty asks for again, as it
  was stored; returning `false` gap-fills it instead, for an order the market has moved past,
  say. A run of messages declined or not stored is covered by one SequenceReset within each step
  of the resend (256 sequence numbers).
- `on_logon` and `on_logout` say when a session logs on and how it ended.
- `on_cancel_on_disconnect` says when to cancel a counterparty's orders (see
  [Operations][operations]).

Callbacks run on the session's connection task and must not block: hand slow work to another
task, and send its result through a `SessionHandle`. Every callback is given the session's
handle (`on_message` through `Context::session`), to read its ID, send on it or log it out;
clone it to keep it. `Acceptor::handle_for` and `Initiator::handle` give one at any time, valid
across reconnects.

One application serves every session of an acceptor or initiator, through `&self`, so what it
keeps goes behind a lock or in atomics, keyed by session where it's a counterparty's own. A
session's callbacks run one at a time, on its task, while other sessions' run at once on theirs,
so a lock held only within a callback (never across an `.await`) is cheap:

```rust
use std::collections::HashMap;
use std::sync::Mutex;

use turbojet::{Application, Context, Message, MessageReject, SessionId};

/// Counts each counterparty's messages.
#[derive(Default)]
struct Counts {
    per_session: Mutex<HashMap<SessionId, u64>>,
}

impl Application for Counts {
    fn on_message(&self, ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        *self.per_session.lock().unwrap().entry(ctx.session().id().clone()).or_default() += 1;
        Ok(())
    }
}
```

State that outlives the process, such as orders, belongs in the application's own store, and
state for work that waits (a venue's reply, say) goes to a task of its own, as the first example
does with its channels.

To unit-test an application, call its callbacks yourself, with a context for a disconnected
handle, and look at what it sent:

```rust
# use turbojet::fields::Decimal;
# use turbojet::{Application, Context, Message, MessageReject, MsgType};
# use turbojet_fix::fix44::{ExecType, ExecutionReport, ExecutionReportRequired, NewOrderSingleRef, OrdStatus};
# struct OrderDesk;
# impl Application for OrderDesk {
#     fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
#         let order: NewOrderSingleRef = msg.parse()?;
#         let mut ack = ExecutionReport::new(ExecutionReportRequired {
#             order_id: "O1".into(), exec_id: "E1".into(), exec_type: ExecType::New.into(),
#             ord_status: OrdStatus::New.into(), side: order.side.into(), leaves_qty: order.order_qty.unwrap_or_default(),
#             cum_qty: Decimal::ZERO, avg_px: Decimal::ZERO,
#         });
#         ack.cl_ord_id = Some(order.cl_ord_id.into());
#         ctx.send(ack)?;
#         Ok(())
#     }
# }
use turbojet::{SessionHandle, SessionId};

let session = SessionHandle::disconnected(SessionId::new("FIX.4.4", "SERVER", "CLIENT"));
let mut ctx = Context::new(&session);
let order: Message = "8=FIX.4.4|35=D|49=CLIENT|56=SERVER|34=2|11=order-1|54=1|60=20261025-12:00:00|40=1|38=100".parse()?;
OrderDesk.on_message(&mut ctx, &order)?;
let ack: ExecutionReport = ctx.replies()[0].parse()?;
assert_eq!(ack.cl_ord_id.as_deref(), Some("order-1"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Sending

`SessionHandle::send` queues a message and returns a `Receipt`, a future that resolves to the
message's MsgSeqNum once the session has stored it (from then on it's resent if the counterparty
misses it), or to why it was dropped, such as the connection ending first. Ignore it for fire and
forget. The queue holds `SessionConfig::max_queued_sends` messages (10,000 by default): past
that, `send` hands the message back (`SendError::Full`) and `send_when_ready` waits for room.
Logout and operator commands have a queue of their own, so a full send queue doesn't hold them
up. `Context::send`, in `on_message`, sends a reply without a queue, which is the fastest way to
answer (see [Performance][performance]). It refuses at once a reply the session would never send (a
session-level message type, an ApplVerID the session doesn't support, or SOH in a value) with
`SendRejected`, which `?` turns into a BusinessMessageReject of the message it answers.

## FIX versions

FIX 4.0 to 4.4 sessions name their version as BeginString: `SessionConfig::new("FIX.4.4",
"CLIENT")`. FIX 5.0, 5.0 SP1 and 5.0 SP2 run over the FIXT 1.1 session protocol: give the session
BeginString `FIXT.1.1` and its application version, and the engine negotiates
DefaultApplVerID(1137) at logon (an acceptor may support several), and reports the agreed version
through `SessionHandle::appl_ver_id()`:

```rust
use turbojet::fields::ApplVerID;
use turbojet::SessionConfig;

let config = SessionConfig::new("FIXT.1.1", "CLIENT").with_appl_ver_id(ApplVerID::Fix50Sp2);
```

With more than one `with_appl_ver_id`, a message in either direction may name another supported
version in ApplVerID(1128), and is validated with that version's dictionary; a message naming an
unsupported one is rejected (or, if the application sends it, dropped). CstmApplVerID(1129) and
ApplExtID(1156) pass through, rejects name the version of the message they answer (RefApplVerID
and the like), and a message is resent in the version it was first sent in.

## How it's layered

`Acceptor` and `Initiator` are thin TCP transports over the same pieces, each of them public, so
you can stop at any level:

- `codec`: framing, BodyLength and CheckSum validation, and resynchronising after garbled input.
- `Message`: the raw tag/value message, with typed access (`msg.field::<Decimal>(tags::PRICE)?`).
- `fields`: `FromFix` and `ToFix` for FIX data types (exact `Decimal` prices and quantities, UTC
  timestamps, `Y`/`N`, integers) and session-level enums (`MsgType`, `SessionRejectReason`,
  `ApplVerID`, …).
- `admin`: typed session messages (Logon, Heartbeat, ResendRequest, Reject, …).
- `turbojet-fix`, a separate crate: every FIX 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1 and 5.0 SP2
  application message, group and enum, generated from the official FIX data, a module and
  feature per version.
- `fix_message!`, `fix_group!` and `fix_enum!`: your own typed messages, groups and enums (see
  [Messages][messages]).
- `store`: the `SessionStorage` and `StoredSession` traits, `MemoryStorage` and `DiskStorage`;
  `turbojet-sql`, a separate crate, has `SqlStorage`.
- `turbojet-config`, a separate crate: an acceptor and initiators from a TOML file, reloadable.
- `session::Session`: the sans-IO state machine for either role. Feed it messages and commands,
  call its timer when its next deadline falls due, write the encoded messages it leaves in
  `output`, and close the connection once it `is_closed`. While it `is_resending`, write its
  output and call `on_resume` for the next step, rather than feed it more.
- `connection::run`: drives a `Session` over any `AsyncRead + AsyncWrite`;
  `Acceptor::accept_stream` and `Initiator::run_stream` take such streams too.
- `tls` (feature `tls`) and `validation` (feature `validation`): see [TLS][tls] and
  [Session behaviour][sessions].

[storage]: storage.md
[operations]: operations.md
[performance]: performance.md
[messages]: messages.md
[tls]: tls.md
[sessions]: sessions.md
