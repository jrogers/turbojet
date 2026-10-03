<p align="center"><img src="https://raw.githubusercontent.com/jrogers/turbojet/main/assets/logo.svg" width="120" alt=""></p>

<h1 align="center">turbojet</h1>

<p align="center">
  <a href="https://crates.io/crates/turbojet"><img src="https://img.shields.io/crates/v/turbojet" alt="crates.io"></a>
  <a href="https://docs.rs/turbojet"><img src="https://img.shields.io/docsrs/turbojet" alt="docs.rs"></a>
  <a href="https://github.com/jrogers/turbojet/actions/workflows/ci.yml"><img src="https://github.com/jrogers/turbojet/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-Apache--2.0-blue" alt="License: Apache-2.0"></a>
</p>

> [!WARNING]
> **Pre-1.0, and not yet proven in production.** Turbojet is tested against one other FIX engine,
> QuickFIX/J, in CI; passes 221 of QuickFIX's 235 scripted session acceptance scenarios (the rest
> are listed with their reasons); and has its parsers and session state machine fuzzed. It hasn't
> been certified with any venue or run against a real counterparty, its APIs will change before
> 1.0, and it has known gaps (see [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/ROADMAP.md)). Don't use it to trade real money or to
> connect to real counterparties without testing it thoroughly yourself. It is provided as is,
> without warranty of any kind; see the licenses below.

**Turbojet** ([`crates/turbojet`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet)) is a FIX engine in Rust (tokio): codec,
session layer, storage, `Acceptor` and `Initiator`, with typed messages for FIX 4.2, 4.3, 4.4 and
5.0 SP2. Its examples are an order-entry gateway (an `Application` served by an `Acceptor`, in
[`examples/gateway`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet/examples/gateway)) and a client for it (an `Initiator`).
The gateway needs the `tls`, `metrics` and `tz` features, the client `tls`.

Typed application messages are generated from FIX data dictionaries:
[`turbojet-dictionary`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-dictionary) loads FIX Orchestra and QuickFIX-format XML
and merges venue customisations onto it, [`turbojet-codegen`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-codegen) generates
messages, groups and enums from it (from a `build.rs`, or as a command), and
[`turbojet-fix42`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix42), [`turbojet-fix43`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix43),
[`turbojet-fix44`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix44) and [`turbojet-fix50sp2`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix50sp2) are
every FIX 4.2, 4.3, 4.4 and 5.0 SP2 application message, generated from the FIX Trading
Community's official data and checked in.

CI runs Turbojet's sessions against QuickFIX/J, in both roles, on FIX 4.2, 4.3 and 4.4 and on
FIXT.1.1 with FIX 5.0 SP2 ([`turbojet-interop`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-interop)), directly and through a proxy that
loses, garbles and cuts messages and silences either side, and through QuickFIX's
235 scripted session acceptance scenarios ([`turbojet-acceptance`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-acceptance)).

```sh
cargo run --example gateway --all-features --release -- --listen 0.0.0.0:9876 --comp-id GATEWAY \
    --allow CLIENT1,CLIENT2 [--store-dir ./store [--fsync]] \
    [--tls-cert server.pem --tls-key server.key \
      [--tls-client-ca ca.pem [--tls-client-auth optional] [--tls-match-comp-id]]] \
    [--schedule "daily 08:00-17:00 mon-fri America/New_York" [--holidays holidays.txt]] \
    [--inbound-limit 100/1s [--over-limit reject]] \
    [--metrics-listen 127.0.0.1:9000] [--log-format json]
cargo run --example client --features tls     # an Initiator: logon → order → cancel → logout
cargo run --example client --features tls -- --tls-ca ca.pem [--tls-cert client.pem --tls-key client.key]
cargo run --example client --features tls -- primary:9876 --failover backup:9876
RUST_LOG=info,turbojet::messages=debug cargo run --example gateway --all-features -- --allow CLIENT1  # log messages
cargo run --example gateway --all-features -- seqnums --store-dir ./store --session CLIENT1 [--reset] \
    [--set-next-incoming N] [--set-next-outgoing N]   # operator: a disconnected session's seqnums
cargo test --all-features     # includes the gateway's tests
```

## Motivation

Turbojet aims to be a FIX engine that a trading or order execution system can build on. A bug in
one can send a duplicate order or lose a fill, so it puts **safety first, then performance, then
developer experience**, after TigerBeetle's
[Tiger Style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md);
[STYLE.md](https://github.com/jrogers/turbojet/blob/main/STYLE.md) says what that means in
practice, and where Turbojet differs.

- **Safe.** What a counterparty sends is bounded (message size, header fields, messages held
  ahead of a gap, how much of a resend is held at once, output it won't read), and what still
  grows without a limit is listed in the roadmap. Invariants are asserted, paired where they can
  be (a message is checked as it's stored and again as it's read back), and the parsers and
  session state machine are fuzzed with those assertions on. Two sessions are also run against
  each other in a deterministic simulator (`crates/turbojet-sim`), over a network that delays,
  stalls, resets and black-holes connections, through crashes, store failures and power loss,
  with every run replayable from its seed. The session is tested against QuickFIX/J and
  QuickFIX's scripted acceptance scenarios (see the warning above for what that does and doesn't
  cover).
- **Pure Rust.** No bindings to a C++ or Java engine, and no `unsafe` code in the engine; lints
  deny it. The one exception is optional: the `tls` feature uses rustls with the `ring` crypto
  provider, which includes C and assembly.
- **Fast.** Performance is part of the design, not an afterthought: outbound messages are
  batched into one buffer, and allocations per message are counted by stage against an exact
  budget on every test run. Encoding, decoding, the session layer and storage have benchmarks (see
  [Benchmarks](#benchmarks)); taking an order to its acknowledgement costs about 1.1 µs, wire
  to wire.
- **Embeddable.** Turbojet is a library, not a server: your code owns the process, the runtime and
  the business logic. Every layer is public, from the codec up through the sans-IO session state
  machine to the TCP transports, so you can use only the layers you need. Storage, clocks and
  transports are traits or plain streams you can replace.
- **Tested.** CI tests every feature combination and the minimum supported Rust version, runs
  clippy with warnings as errors, and checks every dependency's licence and known advisories.
  Planned work is in [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/ROADMAP.md). Until a 1.0 release, APIs may still change (see the warning above).

## Using the engine

Implement `Application` for your business logic; the engine owns the session layer (logon,
sequence numbers, gap detection and resend, heartbeats/TestRequest, logout, persistence).
Typed application messages come from a generated crate per FIX version: add `turbojet-fix42`
(FIX 4.2), `turbojet-fix43` (FIX 4.3), `turbojet-fix44` (FIX 4.4) or `turbojet-fix50sp2`
(FIX 5.0 SP2) alongside `turbojet`. FIX 5.0 SP2 runs over the FIXT 1.1 session protocol: give the
session BeginString `FIXT.1.1` and its application version,
`SessionConfig::new("FIXT.1.1", "CLIENT").with_appl_ver_id(ApplVerId::Fix50Sp2)`, and the engine
negotiates DefaultApplVerID(1137) at logon (an acceptor may support several), and reports the
agreed version through `SessionHandle::appl_ver_id()`. With more than one `with_appl_ver_id`, a
message in either direction may name another supported version in ApplVerID(1128), and is
validated with that version's dictionary; a message naming an unsupported one is rejected (or, if
the application sends it, dropped). CstmApplVerID(1129) and ApplExtID(1156) pass through, rejects
name the version of the message they answer (RefApplVerID and the like), and a message is resent
in the version it was first sent in.

A sketch of both roles follows; the `turbojet` crate documentation has a complete program,
compiled and run as a test.

```rust
use turbojet::*;
use turbojet::fields::Decimal;
use turbojet_fix44::{ExecType, ExecutionReport, NewOrderSingleRef, OrdStatus};

struct MyApp;

impl Application for MyApp {
    // Optional hooks: verify_logon, to_admin, on_logon, on_logout.
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        match msg.msg_type() {
            MsgType::NewOrderSingle => {
                // A missing or malformed field becomes a Reject(3) naming the tag.
                let order: NewOrderSingleRef = msg.parse()?;
                let qty = order.order_qty.unwrap_or_default();
                let mut ack = ExecutionReport::new(
                    "O1", "E1", ExecType::New, OrdStatus::New, order.side, qty, Decimal::ZERO, Decimal::ZERO,
                );
                ack.cl_ord_id = Some(order.cl_ord_id.into());
                ctx.send(ack);
                Ok(())
            }
            _ => Err(MessageReject::unsupported_message_type()),
        }
    }
}

// Acceptor: many counterparties log on to us.
let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), storage.clone(), Arc::new(MyApp));
tokio::spawn(acceptor.clone().serve(listener));
acceptor.session("CLIENT").send(msg)?;          // queue for a connected counterparty

// Initiator: we log on to one counterparty and reconnect as needed.
let config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
let initiator = Initiator::new("host:9876", config, storage, Arc::new(MyApp));
tokio::spawn(initiator.clone().run());
let seq = initiator.handle().send(msg)?.await?; // its MsgSeqNum, once stored
```

`send` queues the message and returns a `Receipt`, a future that resolves to the message's
MsgSeqNum once the session has stored it (from then on it's resent if the counterparty misses it),
or to why it was dropped, such as the connection ending first. Ignore it for fire-and-forget. The
queue holds `SessionConfig::send_queue` messages (10,000 by default): past that, `send` hands the
message back (`SendError::Full`) and `send_when_ready` waits for room. Logout and operator
commands have a queue of their own, so a full send queue doesn't hold them up.

Layers, from the bottom up — each is public, so you can stop at any level:

- `codec` – framing, BodyLength/CheckSum validation, resync after garbled input
- `Message` – the raw tag/value message, with typed access (`msg.field::<Decimal>(tags::PRICE)?`)
- `fields` – `FromFix`/`ToFix` for FIX data types (strict `Decimal` prices/quantities, UTC
  timestamps, `Y`/`N`, integers) and session-level enums (`MsgType`, `SessionRejectReason`,
  `ApplVerId`, …)
- `admin` – typed session messages (Logon, Heartbeat, ResendRequest, Reject, …)
- `turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44`, `turbojet-fix50sp2` (separate crates) –
  every FIX 4.2, 4.3, 4.4 or 5.0 SP2 application message, group and enum, generated from the
  official FIX data
- `fix_message!`, `fix_group!`, `fix_enum!` – define your own typed messages, groups and enums
- `store` – `SessionStorage` / `SessionLog` traits; `MemoryStorage`, `DiskStorage`
- `session::Session` – sans-IO state machine for either role; feed it messages and commands,
  call its timer when its next deadline falls due, write the encoded messages it leaves in
  `output`, and close the connection once it `is_closed`. While it `is_resending`, write its
  output and call `on_resume` for the next step, rather than feed it more
- `connection::run` – drives a `Session` over any `AsyncRead + AsyncWrite`;
  `Acceptor::accept_stream` / `Initiator::run_stream` take such streams too
- `tls` (feature `tls`) – TLS transport, see below
- `validation` (feature `validation`) – checks inbound messages against a data dictionary, see
  below
- `Acceptor`, `Initiator` – TCP transports; `SessionHandle` sends on a live session from anywhere

`Application` callbacks run on the session's connection task and must not block; hand slow work
to another task and send the result with a `SessionHandle`.

### Typed messages

Messages, groups and enums are generated from their FIX definitions by three exported macros,
which Turbojet uses for its own types and applications can use for theirs, including custom tags
and venue-specific message types:

```rust
use turbojet::{fix_enum, fix_group, fix_message};

mod venue_tags {
    pub use turbojet::message::tags::*;
    pub const PRIORITY: u32 = 5001;
    pub const NO_LEGS: u32 = 5002;
    pub const LEG_SYMBOL: u32 = 5003;
}
use venue_tags::*;

fix_enum! { Priority { Normal = "N", Urgent = "U", } }
fix_group! { Leg / LegRef { symbol: req String = LEG_SYMBOL } }
fix_message! {
    /// A venue-specific spread order.
    SpreadOrder / SpreadOrderRef = "U1" { // or a MsgType variant, e.g. = NewOrderSingle
        cl_ord_id: req String = CL_ORD_ID,
        priority: opt Priority = PRIORITY,
        legs: req_group Leg = NO_LEGS,    // at least one entry; `group` allows none
    }
}
```

Each message and group comes in two forms. `SpreadOrder` owns its values: build one to send, or
keep one. `SpreadOrderRef<'a>` borrows its strings, lists and groups from the message it was
parsed from, so parsing it allocates nothing; read messages with it, and call `into_owned()` on
one worth keeping:

```rust
let order: SpreadOrderRef = msg.parse()?; // borrows from msg
for leg in &order.legs {
    println!("{}", leg.symbol);
}
let kept: SpreadOrder = order.into_owned(); // or msg.parse::<SpreadOrder>()?
```

Tags are paths to `u32` constants and are matched as patterns, so a misspelt tag is a compile
error. Parsing is strict: an unknown enum code is *value incorrect* (373=5), a malformed number or
timestamp is *incorrect data format* (373=6), and a missing required field is *required tag
missing* (373=1). Tags the message doesn't define are ignored; `msg.parse_strict()` refuses them
instead, as *tag not defined for this message type* (373=2), for counterparties whose extra fields
should be rejected. The raw `Message` stays available for anything not modelled. For
counterparties that send values outside the spec, `turbojet-codegen --lenient-enums` generates
enumerated fields as `fields::Code<E>`, which keeps an unknown code instead of failing.

## Repeating groups

Groups are parsed from a `GroupSpec`: the group's fields in order, the first being the delimiter
that starts each entry, with nested groups by their NumInGroup tag. Typed messages declare them
with `fix_group!`, as a `Vec` in the owned form and a `Group` in the borrowed one, which reads
each entry as it's iterated; FIX 4.2 NewOrderSingle has `allocs` (NoAllocs 78) and
`trading_sessions` (NoTradingSessions 386), and ExecutionReport has `contra_brokers`
(NoContraBrokers 382).

```rust
let order: NewOrderSingleRef = msg.parse()?;
for alloc in &order.allocs {
    println!("{} gets {:?}", alloc.alloc_account, alloc.alloc_shares);
}

// Raw access, without a typed definition: one zero-copy view per entry.
const PARTIES: GroupSpec = GroupSpec { fields: &[(448, None), (447, None), (452, None)] };
for party in msg.group(453, &PARTIES)? {
    println!("{:?} role {:?}", party.get(448), party.get(452));
}
```

Typed parsing is a single pass: each tag is matched to its field, and a group's fields are
consumed where its count appears, so a tag used both inside a group and outside it is never
confused. A count that disagrees with the entries, an entry not starting with its delimiter, or a
member repeated within an entry is a `FieldError` (SessionRejectReason 16 or 15; these codes are
FIX 4.3+, as FIX 4.2 has none for groups).

## Operating on sequence numbers

Every `SessionHandle` (e.g. `acceptor.session("CLIENT1")` or `initiator.handle()`) can inspect and
correct its session's sequence numbers, connected or not:

```rust
let session = acceptor.session("CLIENT1");
let SequenceNumbers { next_incoming, next_outgoing } = session.sequence_numbers().await?;
session.set_next_incoming(42).await?;   // either direction
session.set_next_outgoing(100).await?;  // forward only
session.reset_sequence_numbers().await?; // both to 1; disconnected sessions only
```

- A connected session applies the change on its connection task, so its state stays consistent.
  Raising the outgoing number while logged on also sends the counterparty a SequenceReset in
  gap-fill mode, so it expects the new number instead of detecting a gap; a counterparty still
  filling an earlier gap applies it once that's filled, so nothing sent before is lost.
- A disconnected session has its stored state changed directly; it is claimed in the registry
  meanwhile, so it can't log on half-way through.
- Outgoing numbers never move backwards (sent numbers must not be reused); resetting to 1 needs
  the session disconnected, or a Logon with ResetSeqNumFlag for a reset agreed with the
  counterparty. A counterparty may also reset while logged on, with a Logon carrying
  ResetSeqNumFlag=Y at MsgSeqNum 1: both sides start again at 1, Turbojet replying with a Logon
  of its own, and anything outstanding from the old numbering is dropped.

The gateway's `seqnums` subcommand does the same for a session in its `--store-dir` that isn't
connected, even while the gateway runs; a connected session's store is locked, and the tool says
so rather than changing it.

## Logon credentials and NextExpectedMsgSeqNum

An initiator sends Username(553) and Password(554) from `InitiatorConfig::username` and
`password`; an acceptor checks them in `Application::verify_logon`, where
`logon.parse::<turbojet::admin::LogonRef>()` gives them typed. Passwords are `fields::Secret`
(`SecretRef` when borrowed), which `Debug` and `Display` show as `***` (`.expose()` gives the
text); the message log masks them too, and generated messages type Password(554) and
NewPassword(925) fields as `Secret` as well.

With `InitiatorConfig::next_expected_msg_seq_num`, the initiator's Logon carries
NextExpectedMsgSeqNum(789), the FIX 4.4 way to recover a gap at logon: each side resends what the
other missed straight after logon, with no ResendRequest. An acceptor honours 789 whenever a
counterparty sends it and answers with its own; a 789 beyond what was ever sent is answered with a
Logout.

## Session checks

Besides framing, sequencing and the header's CompIDs, every inbound message is checked as the FIX
specification's session test cases expect, and each check can be turned off on `SessionConfig`
for a counterparty that needs it:

- `max_latency` (120 s by default, as QuickFIX's MaxLatency): a SendingTime further than this
  from the session's clock, either way, is rejected (SessionRejectReason 10) and the session logs
  out; a Logon is refused. A message kept behind a gap is checked when it arrives.
- `check_orig_sending_time` (on): a message with PossDupFlag=Y needs an OrigSendingTime, no later
  than its SendingTime. A missing one is rejected (1); a later one is rejected (10) and the
  session logs out. Gap fills are exempt.
- `check_header_order` (on): a header field after a body field is rejected (14).

On FIX 4.2, which defines reasons only up to 11, a Reject for a later one leaves the reason out
and says what's wrong in its Text.

Routing fields (OnBehalfOf and DeliverTo CompID, SubID and LocationID) an application sets go out
in the header. The session's own Rejects and BusinessMessageRejects are routed back to where the
message came from; `Message::with_reverse_route(&received)` does the same for an application's
replies through a hub.

## Validating against a dictionary

With the `validation` feature, a session can check every inbound message against a data
dictionary before the application or the session acts on it:

```rust,ignore
let dict = turbojet_dictionary::Dictionary::load("dictionaries/orchestra/OrchestraFIX44.xml")?;
let config = SessionConfig::new("FIX.4.4", "GATEWAY").with_dictionary(&dict);
```

A message that fails gets a session Reject (35=3) naming the tag and the reason, and isn't
delivered; it still counts as received. The checks, each of which can be turned off with
`validation::ValidationOptions` (counterparties vary in how closely they follow the spec):
MsgType not in the dictionary (373=11), undefined tag (0), tag not defined for the message (2),
required tag missing, including in group entries (1), value not one of the field's codes (5),
data format (6), a tag repeated outside a group (13), and group counts and order (16, 15).
`allow_user_defined_tags` accepts undefined tags from 5000 up, which venues often add. With a
FIX 4.2 dictionary, reasons added after 4.2 are left out and only the text says what's wrong.
Session-level messages are checked where the dictionary defines them (`admin_messages` turns this
off), except Logon, which the engine checks itself, and Logout, which always completes. Correct a
dictionary's mistakes before building the session (`Dictionary::make_optional`). Validation costs
about 0.36 µs for a FIX 4.2 NewOrderSingle.

## Session schedules

Set `SessionConfig::schedule` (or the gateway's `--schedule`) to confine a session to trading
hours:

```rust
config.schedule = Some("daily 08:00-17:00 mon-fri America/New_York".parse()?);
// or: "weekly sun 17:00-fri 17:00 America/New_York", "daily 22:00-06:00 UTC",
//     "daily 00:00-00:00 +05:30" (continuous, new period each midnight)
```

- Outside a period an acceptor refuses logons and an initiator waits for the next period
  (`connect_once` fails immediately).
- When a period ends, a logged-on session sends Logout ("End of session") and disconnects,
  including at the boundary between back-to-back periods.
- The first logon of a new period resets sequence numbers to 1 and clears the resend store,
  based on when the store's state was created (`SessionLog::created_at`). State from before
  creation times were recorded is kept and resets from the next period; custom stores that don't
  record creation times never reset on schedule (a warning says so).

Holidays are dates, in the schedule's time zone, on which no period starts. Give them as a
`HolidayCalendar`, which parses one `YYYY-MM-DD` per line with `#` comments (the gateway reads one
with `--holidays FILE`):

```rust
let holidays: HolidayCalendar = std::fs::read_to_string("holidays.txt")?.parse()?;
config.schedule = Some(schedule.with_holidays(holidays));
```

A period that starts the day before a holiday still runs into it, and the period after one resets
sequence numbers as any new period does. A weekly schedule skips only a week that starts on a
holiday. A refused logon, or an initiator waiting, names the holiday.

Times are in UTC, a fixed offset, or, with turbojet's `tz` feature, an IANA zone that follows
daylight saving. `SessionConfig::clock` supplies wall-clock time; replace it with
`Clock::from_fn` to test schedules without waiting.

## Throttling

Venues limit how fast a counterparty may send. `SessionConfig::outbound_limit` keeps a session
within such a limit, and `inbound_limit` holds a counterparty to one:

```rust
config.outbound_limit = Some("100/1s".parse()?); // RateLimit::new(100, Duration::from_secs(1))
config.inbound_limit = Some(InboundLimit::Delay("50/200ms".parse()?)); // or InboundLimit::Reject
```

A `RateLimit` of N per W allows at most N application messages in any window of length W, sliding
and half-open, so it's exactly a venue's "N per second". N is at most 100,000 and W at most a day.
Each connection starts with an empty window.

- **Outbound.** `SessionHandle::send`s wait in the send queue while the window is full, so a full
  queue hands them back as usual. Replies from `Application::on_message` go out at once but count,
  so they can take a window past N, and while they alone fill it, queued sends wait. Admin
  messages, resends and the session's own BusinessMessageRejects neither count nor wait. A message
  counts when it's framed, not when it's written. `SessionHandle::logout` waits for the sends
  queued before it, at the limit's rate; a shutdown, or a logout the session starts itself (a
  schedule's end, the counterparty's Logout), drops them.
- **Inbound, `Delay`.** Every application message counts as it arrives, resends included. While
  the window is full the connection stops reading, and TCP slows the counterparty down. Admin
  messages wait in order behind the rest, so a Logout or ResendRequest may wait up to W, and a
  counterparty that has died may be noticed up to W late; the time held doesn't count as its
  silence.
- **Inbound, `Reject`.** A message over the limit takes its sequence number but is answered with
  a BusinessMessageReject (reason Other, "throttle limit exceeded") instead of being delivered.
  Rejections don't count. Recovery we asked for counts but is never rejected, so a hostile
  counterparty, which controls its own gaps, can push one gap's worth through. `Reject` suits a
  fast but well-behaved counterparty, `Delay` holds against a hostile one, and under a flood
  `Delay` is also the cheaper.

`turbojet_throttled_total` counts sends that waited, holds and rejects, and the first time a limit
is reached on a connection it logs a warning. Custom drivers pace themselves with
`Session::can_send`, `send_free_at` and `input_free_at`; the outbound warning and count come from
Turbojet's own connection driver. The gateway limits each counterparty with
`--inbound-limit N/W`, and `--over-limit delay` (the default) or `reject`. Limits that are never
reached cost about 10 ns an order, within noise.

## Initiator failover

An initiator has a primary endpoint and any number of backups, tried in order:

```rust
let initiator = Initiator::new("primary.example.com:9876", config, storage, app)
    .with_failover("dr.example.com:9876")
    .with_failover(Endpoint::new("10.1.2.3:9876").with_tls_server_name("dr2.example.com"))
    .with_tls(connector, "primary.example.com")?; // default TLS name; endpoints may override
```

Each connection attempt starts at the primary. An endpoint fails over to the next when the TCP
connect fails or exceeds `connect_timeout` (default 10s), the TLS handshake fails, or the
connection ends before logon (e.g. the acceptor refuses it). Once a session has logged on,
it stays on that endpoint until it ends; the next attempt then starts at the primary again,
so the initiator fails back as soon as the primary is reachable. If every endpoint fails, or a
session ends, `run()` waits and starts over, as `InitiatorConfig::reconnect` says: by default
1 s, doubling while attempts keep failing up to 60 s, each wait a random half to all of that so
initiators that lost a venue together don't all come back together, and 1 s again once a session
has logged on. `ReconnectPolicy::fixed` waits the same every time.

An acceptor keeps at most 1,024 connections open at once, and 16 from one IP address
(`Acceptor::with_max_connections`, `with_max_connections_per_ip`), counting every connection
until it closes, logged on or not. One past either is closed as soon as it's accepted, before any
TLS handshake, and counted in `turbojet_connections_refused_total`. Counterparties behind one
address (a NAT, or a hub serving several firms) share its limit.

All endpoints are treated as the same FIX session (e.g. a counterparty's primary and DR
sites), so sequence numbers carry over and the `SessionHandle` stays valid across failovers.

## Shutting down

`Acceptor::shutdown` and `Initiator::shutdown`, on any clone, log every session out (with an
optional Text), disconnect connections that haven't logged on, abandon TLS handshakes, and stop
accepting or reconnecting. They return once every connection has closed, and `serve` or `run`
returns too. A counterparty that doesn't answer the Logout is disconnected after the session's
`logout_timeout` (5 s by default), and a second later anything still open is closed regardless: a
connection blocked writing to a counterparty that has stopped reading, say. Shutdown is permanent.

```rust
tokio::signal::ctrl_c().await?;
acceptor.shutdown(Some("end of day")).await;
```

The gateway does this on SIGINT or SIGTERM; a second signal exits at once.

## TLS

Enable the `tls` feature of `turbojet` (rustls with the `ring` provider; nothing TLS-related is
compiled without it). The gateway enables it.

```rust
// Acceptor: server certificate, and client authentication (mutual TLS):
//   ClientAuth::None           - no client certificates
//   ClientAuth::Optional(ca)   - request one; admit clients without, refuse invalid ones
//   ClientAuth::Required(ca)   - refuse clients without a valid certificate
let tls = turbojet::tls::acceptor("server.pem".as_ref(), "server.key".as_ref(), ClientAuth::Required("ca.pem".as_ref()))?;
tokio::spawn(acceptor.serve_tls(listener, tls));

// Initiator: trust a CA, verify the server name, optionally present a client certificate.
let tls = turbojet::tls::connector("ca.pem".as_ref(), Some(("client.pem".as_ref(), "client.key".as_ref())))?;
let initiator = initiator.with_tls(tls, "gateway.example.com")?;
```

Certificates can also come from memory (a secrets manager, say) and be replaced while running.
`ServerTls` and `ClientTls` hold them: a change applies from the next handshake, and sessions
already connected carry on. Every handshake is a full one, checked against the certificates
current then (sessions aren't resumed).

```rust
use turbojet::tls::{ClientTrust, Identity, ServerTls, Trust};

let server = ServerTls::new(Identity::from_pem(&cert_pem, &key_pem)?, ClientTrust::Required(Trust::from_pem(&ca_pem)?))?;
tokio::spawn(acceptor.serve_tls(listener, server.acceptor()));
// Later, when the certificate is renewed:
server.set_identity(Identity::from_pem(&new_cert_pem, &new_key_pem)?);
```

`ClientTls` does the same for an initiator (`set_trust`, `set_identity`). An `Identity` is
checked when it's built (the key must be the certificate's), so a bad renewal is refused and
the old one kept. The gateway reloads its certificate, key and client CAs from their files on
SIGHUP.

For other setups (system roots, custom verifiers, different protocol versions), build a
`rustls::ServerConfig`/`ClientConfig` yourself and wrap it with `TlsAcceptor::from` /
`TlsConnector::from`. Handshakes run on each connection's own task and are bounded by the logon
timeout.

`Application::verify_logon` receives a `ConnectionInfo` with the remote address and the peer's
verified certificate chain (the client's on an acceptor, the server's on an initiator). With
the `tls` feature, `PeerCertificate::subject_common_name()` and `dns_names()` make it easy to tie
a certificate to a CompID. The gateway does this with `--tls-match-comp-id`: a client that
presents a certificate may only log on as the SenderCompID named by its CN or a DNS name.

## Session storage

Sequence numbers and sent application messages are persisted before each message is written to
the socket; if the store fails, the session disconnects. The store commits them once per batch of
work (everything that arrived in one read, one batch of application sends, one step of a resend),
and nothing the batch sends is written until that commit is done. A store that waits for its
device (`DiskStorage` with `fsync`) hands the commit to the connection driver, which runs it on a
blocking thread, so the async runtime never waits for a disk; meanwhile the connection reads but
processes nothing. A `Receipt` resolves, and an operator hears of a sequence number change, once
the change is committed.

Delivery to the application is at least once. An inbound message counts as received only after
`on_message` has returned and anything it sent in reply is stored, so if the process stops before
then, the counterparty resends it. Each commit also records a marker saying the messages from the
next incoming one on (up to 256) may be handed over, so a batch needs no commit of its own before
its first message; a session that ends cleanly clears it. After a crash, a message in that window
that comes again (flagged PossDupFlag=Y, or answering our ResendRequest) has
`Context::maybe_redelivered()` set: it may have been handled before. Look it up (by ClOrdID, say)
before acting on it again. Resends the application never saw may be marked too; new messages
aren't.

- `MemoryStorage`: survives reconnects, not restarts. Each session keeps its newest sent messages
  up to 64 MiB (`with_max_session_bytes`); a resend gap-fills older ones, logs a warning and counts
  it in `turbojet_resend_requests_evicted_total`. It keeps at most 1,024 sessions
  (`with_max_sessions`) and never forgets one: a new session's Logon past that is refused.
- `DiskStorage`: per session, `<id>.seqnums` (both sequence numbers and the message in flight,
  as a fixed-width record rewritten in place and locked while the session is connected),
  `<id>.body`, `<id>.body.1` and so on (sent messages appended in wire format, in segments of
  64 MiB, indexed on open and read back for resends) and, once recorded, `<id>.created` (when the
  state was created or last reset, for session schedules). A commit appends the batch's messages
  to the newest segment in one write, then rewrites the record once. Each session keeps its
  newest 1 GiB of messages (`with_max_session_bytes`, `with_segment_bytes`): past it the oldest
  segments are deleted, and a resend gap-fills their messages, as with `MemoryStorage`. On open it
  truncates a torn trailing write and advances the outgoing sequence number past the last stored
  message. Without `fsync`, commits are written in the call and survive a process
  crash but not an OS crash; with it, a batch costs one `fsync` of each file.

## The gateway

Speaks FIX 4.2. Handles NewOrderSingle `D` (→ ExecutionReport New/Rejected, or Reject `3` for
malformed fields), OrderCancelRequest `F` (→ ExecutionReport Canceled or OrderCancelReject `9`),
OrderCancelReplaceRequest `G` (→ ExecutionReport Replaced, or OrderCancelReject; quantity, price
and time in force can change, symbol/side/type can't) and OrderStatusRequest `H` (→ status
ExecutionReport, found by any ClOrdID the order has had); other message types get a
BusinessMessageReject `j`. Market and Limit orders, TimeInForce Day/GTC, Side
Buy/Sell/SellShort/SellShortExempt. Business rejections without a specific FIX 4.2 reason code
use the spec's "Broker option", named BrokerCredit (OrdRejReason 0, CxlRejReason 2), with an
explanatory Text. The typed FIX 4.2 messages type every field in the dictionary, so an
out-of-spec value is rejected even in a field the gateway doesn't use (e.g. Rule80A 47 or
OpenClose 77) rather than ignored.

## Logging and metrics

**Logging** uses [`tracing`](https://docs.rs/tracing). Each connection runs in a `session` span
whose `id` (e.g. `FIX.4.2:GATEWAY->CLIENT1`) is filled in once known, so engine events and
`Application` callbacks carry it. Inbound and outbound messages are logged at `DEBUG` under their
own target, `turbojet::messages`, with `direction` = `in`/`out`, so the FIX message log can be
enabled or routed separately: `RUST_LOG=info,turbojet::messages=debug`; passwords in it are shown
as `***`. The gateway can emit JSON logs (`--log-format json`).

**Metrics** use the [`metrics`](https://docs.rs/metrics) facade, behind turbojet's optional
`metrics` feature. Install a recorder (e.g. `metrics-exporter-prometheus`) before starting
acceptors or initiators, then optionally call `turbojet::describe_metrics()` for help text. Every
metric is labelled with `session`:

| Metric | Type |
|---|---|
| `turbojet_messages_received_total`, `turbojet_messages_sent_total` | counter |
| `turbojet_bytes_received_total`, `turbojet_bytes_sent_total` | counter |
| `turbojet_logons_total`, `turbojet_disconnects_total` | counter |
| `turbojet_rejects_sent_total` (`type` = `session`/`business`) | counter |
| `turbojet_sequence_gaps_total`, `turbojet_resend_requests_received_total` | counter |
| `turbojet_resend_requests_evicted_total` (reaching messages the store evicted) | counter |
| `turbojet_throttled_total` (`direction` = `inbound`/`outbound`; see Throttling) | counter |
| `turbojet_session_logged_on`, `turbojet_next_incoming_seq`, `turbojet_next_outgoing_seq` | gauge |

plus an unlabelled `turbojet_garbled_messages_total`, `turbojet_connections_refused_total` by
`reason` (`total` or `per_ip`), and `turbojet_application_panics_total` by
`callback` (a panicking callback is caught; see the `Application` docs). Handles are created once
per session, so recording is a counter increment: measured A/B, order → ack costs nothing extra
with no recorder installed, and about 2% (≈25 ns) with a Prometheus recorder. The gateway serves
them, along with `gateway_orders_total` and `gateway_cancels_total` by `result`, at
`--metrics-listen ADDR` (`/metrics`).

## Benchmarks

```sh
cargo bench -p turbojet --all-features            # everything (~3 minutes)
cargo bench -p turbojet --bench codec             # one group: codec, session or roundtrip
cargo bench -p turbojet --all-features -- --quick # fast smoke run
```

Snapshot (Apple M3, macOS 27, Rust 1.98.1, 2026-09-27; medians). Benchmarks build with one
codegen unit and fat LTO (`[profile.bench]`), so results don't shift with how the compiler happens
to partition the crate.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one¹ | 221 ns / 186 ns | 729 / 866 MiB/s |
| Encode ExecutionReport (209 B) | 118 ns | 1.6 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations)² | 98 ns / 173 ns | |
| Typed parse NewOrderSingle, owned (no groups / with 3 allocations)² | 163 ns / 334 ns | |
| Typed parse FIX 4.4 NewOrderSingle with nested groups (363 B): borrowed / reading every entry / owned² | 448 ns / 722 ns / 1.03 µs | |
| Typed build ExecutionReport¹ | 158 ns | |
| Format a timestamp (same second / new second)¹ | 11 ns / 33 ns | |
| Session: order → ack, no I/O, encoded reply (memory store)² | 883 ns | 1.13M msg/s |
| Session: order → ack, wire to wire (decode + session, which encodes)² | 1.09 µs | 921k msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync³ | 49 ns / 3.3 µs / 8.2 ms | |
| Store a sent message, 100 per commit: disk / disk + fsync³ | 88 ns / 82 µs | |
| Round trip over localhost TCP, one at a time | 27.7 µs | 36.1k/s |
| Round trip over localhost TCP, 1,000 in flight | | 532k msg/s |
| Round trip over localhost TLS, one at a time | 27.8 µs | 36.0k/s |
| Round trip over localhost TLS, 1,000 in flight | | 508k msg/s |
| Round trip, acceptor storing to disk: one at a time / 1,000 in flight³ | 32.9 µs | 695k msg/s |
| Round trip, acceptor storing to disk + fsync: one at a time / 100 in flight³ | 8.1 ms | 9.1k msg/s |

Round trips are initiator → acceptor application → initiator application, using a store that
discards messages (storage is measured separately), except the disk rows, where the acceptor
stores to `DiskStorage`. Session benchmarks restart the session every
10,000 messages, untimed, to keep the in-memory resend store from growing without bound.
¹ Re-measured 2026-09-30 on the same machine, after the session started encoding what it sends
straight into its output, decimals and integers were written without `core::fmt`, and each inbound
message was decoded into one reused for the connection. ² Re-measured 2026-10-01, after typed
messages gained borrowed forms (`NewOrderSingleRef`); the owned form is now parsed as the borrowed
one and then made owned, so it costs more than it did. ³ Measured 2026-10-02, with group commit:
before it, storing 100 messages with fsync took 837 ms, and the disk + fsync round trip managed 62
messages a second with 100 in flight; and with each commit recording the next in-flight window
(before that, 12.2 ms one at a time with fsync). The other rows are the 2026-09-27 snapshot.
The FIX 4.4 order has three parties with two sub-IDs each.

A test counts heap allocations per order → ack, wire to wire, by stage, and fails if any stage's
count changes, up or down, so both regressions and improvements show up in CI:

```sh
cargo test -p turbojet --test allocations -- --nocapture   # prints the table
```

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Decode (into one message reused per connection) | 0 | 0 | 0 |
| Session (including encoding the ack) | 0 | 0 | 0 |
| Application (typed parse and ack)³ | 8 | 2 | 3,138 |
| Store: memory / disk | 1.2 / 0.2 | 0 | 280 / 49 |
| Engine (all but the application): memory / disk | 1.2 / 0.2 | 0 | 280 / 49 |

Means of 1,000 orders after 100 warm-up, 2026-09-30, with each store (the disk store without
fsync). Store allocations are fractional because the stores' maps allocate a node every few
messages; the memory store also copies each message it keeps. Debug and release builds count the
same. ³ Includes one of the engine's: the reply list that `Context::send` pushes onto. The
application parses the order borrowed, without allocating, and copies the strings it needs into
its owned ExecutionReport.

Another test counts typed parsing alone, per FIX 4.2 NewOrderSingle:

| Parse | Allocations | Reallocs | Bytes |
|---|---|---|---|
| `NewOrderSingleRef`: no groups / 3 allocations, every entry read | 0 / 0 | 0 | 0 / 0 |
| `NewOrderSingle`: no groups / 3 allocations | 3 / 7 | 0 | 16 / 226 |

The owned form allocates a `String` for each text field it holds (ClOrdID, Symbol and Account
here) and a `Vec` for each group, with its entries' strings.

## Limitations

See [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/ROADMAP.md) for the planned work. In brief:

- Gateway orders live in memory; a restart keeps session state but forgets orders, and orders
  are acknowledged but not routed or matched.
- Resends read the disk store on the connection task, and opening a session's store scans its
  segments (up to 1 GiB by default) there.
- Typed messages come for FIX 4.2, 4.3, 4.4 and 5.0 SP2; other versions need `turbojet-codegen`,
  or `fix_message!` for messages defined by hand.

## Releases and security

Releases and what changed in each are in [CHANGELOG.md](https://github.com/jrogers/turbojet/blob/main/CHANGELOG.md). Report security problems
privately, as [SECURITY.md](https://github.com/jrogers/turbojet/blob/main/SECURITY.md) describes, not in a public issue. CI checks every
dependency's licence and known advisories with `cargo deny` ([deny.toml](https://github.com/jrogers/turbojet/blob/main/deny.toml)).

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](https://github.com/jrogers/turbojet/blob/main/LICENSE) or
<http://www.apache.org/licenses/LICENSE-2.0>).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.

### Third-party data

The FIX Orchestra files in [`dictionaries/orchestra`](https://github.com/jrogers/turbojet/tree/main/dictionaries/orchestra) are © FIX Protocol
Limited and licensed under the Apache License, Version 2.0 (see the `LICENSE` and `NOTICE` there);
`OrchestraFIX43.xml` and `OrchestraFIX50SP2.xml` are converted from the FIX Unified Repository,
as the `NOTICE` there explains. The generated crates take their names and structure from them, but
don't reproduce their documentation; see their `NOTICE` files.

The session acceptance scripts in
[`crates/turbojet-acceptance/definitions`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-acceptance/definitions) are QuickFIX's,
under the QuickFIX Software License (the `LICENSE` there). This product includes software developed
by quickfixengine.org (http://www.quickfixengine.org/).
