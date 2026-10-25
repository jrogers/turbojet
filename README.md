<p align="center"><img src="https://raw.githubusercontent.com/jrogers/turbojet/main/assets/logo.svg" width="120" alt=""></p>

<h1 align="center">turbojet</h1>

<p align="center">
  <a href="https://crates.io/crates/turbojet"><img src="https://img.shields.io/crates/v/turbojet" alt="crates.io"></a>
  <a href="https://docs.rs/turbojet"><img src="https://img.shields.io/docsrs/turbojet" alt="docs.rs"></a>
  <a href="https://github.com/jrogers/turbojet/actions/workflows/ci.yml"><img src="https://github.com/jrogers/turbojet/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-Apache--2.0-blue" alt="License: Apache-2.0"></a>
</p>

> [!WARNING]
> **Pre-1.0, and not yet proven in production.** Turbojet is tested against two other FIX engines,
> QuickFIX/J and QuickFIX/n, in CI; passes 440 of QuickFIX's 467 scripted session acceptance scenarios (the rest
> are listed with their reasons); and has its parsers and session state machine fuzzed. It hasn't
> been certified with any venue or run against a real counterparty, its APIs will change before
> 1.0, and it has known gaps (see [CAVEATS.md](https://github.com/jrogers/turbojet/blob/main/docs/CAVEATS.md) and [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/docs/ROADMAP.md)). Don't use it to trade real money or to
> connect to real counterparties without testing it thoroughly yourself. It is provided as is,
> without warranty of any kind; see the licenses below.

**Turbojet** ([`crates/turbojet`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet)) is a FIX engine in Rust (tokio): codec,
session layer, storage, `Acceptor` and `Initiator`, with typed messages for FIX 4.0, 4.1, 4.2, 4.3,
4.4, 5.0, 5.0 SP1 and 5.0 SP2. Its examples are an order-entry gateway (an `Application` served by an `Acceptor`, in
[`examples/gateway`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet/examples/gateway)) and a client for it (an `Initiator`).
The gateway needs the `tls`, `metrics`, `tz` and `gzip` features, the client `tls`.

Typed application messages are generated from FIX data dictionaries:
[`turbojet-dictionary`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-dictionary) loads FIX Orchestra and QuickFIX-format XML
and merges venue customisations onto it, [`turbojet-codegen`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-codegen) generates
messages, groups and enums from it (from a `build.rs`, or as a command), and
[`turbojet-fix`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix) has every FIX 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1 and
5.0 SP2 application message, one feature per version, generated from the FIX Trading
Community's official data and checked in. `turbojet-codegen sbe` generates codecs for SBE (Simple
Binary Encoding) message schemas, the binary encoding venues such as B3 and CME publish (see
[Binary encodings](https://github.com/jrogers/turbojet/blob/main/docs/guide/binary.md)), and `turbojet::fixp` runs FIXP sessions, the FIX Performance Session
Layer, to carry them (see [Binary encodings](https://github.com/jrogers/turbojet/blob/main/docs/guide/binary.md#fixp-sessions)).

[`turbojet-fast`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fast) decodes and
encodes FAST 1.1, the compressed encoding some venues use for market data, into and from Turbojet
messages, with the venue's templates loaded at runtime (see [Binary encodings](https://github.com/jrogers/turbojet/blob/main/docs/guide/binary.md#fast)).

[`turbojet-sql`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-sql) keeps session
state in SQLite or PostgreSQL, with leases so that gateways sharing a database each run their own
sessions.

[`turbojet-config`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-config) reads an
acceptor, its counterparties, initiators and their stores from a TOML file, and reloads it while
running.

[`turbojet-python`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-python) runs
initiators from Python's asyncio, loaded from a `turbojet-config` file, with application messages
delivered at least once. It's built with maturin and isn't on PyPI.

[`turbojet-log`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-log) is a command
that prints a message log's files with FIX field names, filtered by session, direction, MsgType, a
field's value or a time range. The same filter is `turbojet::LogFilter`, to search the logs from
code.

CI runs Turbojet's sessions against QuickFIX/J and QuickFIX/n, in both roles, on FIX 4.2, 4.3 and 4.4 and on
FIXT.1.1 with FIX 5.0 SP2 ([`turbojet-interop`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-interop)), directly and through a proxy that
loses, garbles, cuts and delays messages, slows the link and silences or stalls either side, and
over TLS and mutual TLS; and through QuickFIX's 467 scripted session acceptance scenarios ([`turbojet-acceptance`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-acceptance)).

```sh
cargo run --example gateway --all-features --release -- --listen 0.0.0.0:9876 --comp-id GATEWAY \
    --allow CLIENT1,CLIENT2 [--store-dir ./store [--fsync]] \
    [--tls-cert server.pem --tls-key server.key \
      [--tls-client-ca ca.pem [--tls-client-auth optional] [--tls-match-comp-id]]] \
    [--schedule "daily 08:00-17:00 mon-fri America/New_York" [--holidays holidays.txt]] \
    [--inbound-limit 100/1s [--over-limit reject]] \
    [--metrics-listen 127.0.0.1:9000 [--latency-metrics]] [--log-format json] \
    [--message-log ./messages [--message-log-days 2555]]
cargo run --example gateway --all-features -- --help   # every option, including --config FILE
cargo run --example client --features tls     # an Initiator: logon → order → cancel → logout
cargo run --example client --features tls -- --tls-ca ca.pem [--tls-cert client.pem --tls-key client.key]
cargo run --example client --features tls -- primary:9876 --failover backup:9876
cargo run --release --example ack     # an acceptor that answers each order and keeps nothing, to load
cargo run --release --example load -- [ADDR] --sessions 4 --rate 50000 --duration 10   # latency under load
RUST_LOG=info,turbojet::messages=debug cargo run --example gateway --all-features -- --allow CLIENT1  # log messages
cargo run --example gateway --all-features -- seqnums --store-dir ./store --session CLIENT1 [--reset] \
    [--set-next-incoming N] [--set-next-outgoing N]   # operator: a disconnected session's seqnums
cargo test --all-features     # includes the gateway's tests
```

The `load` example sizes a deployment against any acceptor: it opens `--sessions` sessions
(`LOAD1` to `LOADN`), sends NewOrderSingles at `--rate` orders a second across them, and reports
the latency of their ExecutionReports at p50 to p99.99 and the max. Latency runs from when each
order was due, not when it was sent, so a stall counts against everything queued behind it. The
report also says how far the sender fell behind its schedule: when that's large, the generator's
machine is the bottleneck, not the acceptor. The `ack` example is an acceptor to measure the
engine against: it answers each NewOrderSingle with an ExecutionReport and keeps nothing. The
gateway keeps every order, up to a million (past that it rejects new ones until it restarts), and
logs each at `info`, so against it `load` measures its order book too.

## Motivation

Turbojet aims to be a FIX engine that a trading or order execution system can build on. A bug in
one can send a duplicate order or lose a fill, so it puts **safety first, then performance, then
developer experience**, after TigerBeetle's
[Tiger Style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md);
[STYLE.md](https://github.com/jrogers/turbojet/blob/main/docs/STYLE.md) says what that means in
practice, and where Turbojet differs.

- **Safe.** What a counterparty sends is bounded (message size, header fields, messages held
  ahead of a gap, how much of a resend is held at once, output it won't read), and what still
  grows without a limit is listed in [CAVEATS.md](https://github.com/jrogers/turbojet/blob/main/docs/CAVEATS.md). Invariants are asserted, paired where they can
  be (a message is checked as it's stored and again as it's read back), and the parsers and
  session state machine are fuzzed with those assertions on. Two sessions, FIX or FIXP, are also
  run against each other in a deterministic simulator (`crates/turbojet-sim`), over a network
  that delays, stalls, resets and black-holes connections, through crashes, store failures and
  power loss, with every run replayable from its seed. The session is tested against QuickFIX/J,
  QuickFIX/n and QuickFIX's scripted acceptance scenarios (see the warning above for what that does and doesn't
  cover).
- **Pure Rust.** No bindings to a C++ or Java engine, and no `unsafe` code in the engine; lints
  deny it. The one exception is optional: the `tls` feature uses rustls with the `ring` crypto
  provider, which includes C and assembly.
- **Fast.** Performance is part of the design, not an afterthought: outbound messages are
  batched into one buffer, and allocations per message are counted by stage against an exact
  budget on every test run. Encoding, decoding, the session layer and storage have benchmarks (see
  [Benchmarks](#benchmarks)); taking an order to its acknowledgement costs about 1 µs, wire
  to wire.
- **Embeddable.** Turbojet is a library, not a server: your code owns the process, the runtime and
  the business logic. Every layer is public, from the codec up through the sans-IO session state
  machine to the TCP transports, so you can use only the layers you need. Storage, clocks and
  transports are traits or plain streams you can replace.
- **Tested.** CI tests every feature combination and the minimum supported Rust version, runs
  clippy with warnings as errors, and checks every dependency's licence and known advisories.
  Planned work is in [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/docs/ROADMAP.md), and how Turbojet is built in
  [DESIGN.md](https://github.com/jrogers/turbojet/blob/main/docs/DESIGN.md). Until a 1.0 release, APIs may still change (see the warning above).

## Using the engine

Implement `Application` for your business logic; the engine owns the session layer (logon,
sequence numbers, gap detection and resend, heartbeats and TestRequests, logout, persistence).
Typed application messages come from `turbojet-fix`, with a feature per FIX version: `fix40`,
`fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1` and `fix50sp2`.

Here both roles run in one program
([`examples/quickstart.rs`](https://github.com/jrogers/turbojet/blob/main/crates/turbojet/examples/quickstart.rs),
`cargo run --example quickstart`): an acceptor acknowledges orders as `SERVER`, and an initiator
logs on to it as `CLIENT`, sends an order and waits for the acknowledgement. It needs these
dependencies:

```toml
[dependencies]
turbojet = "0.5"
turbojet-fix = { version = "0.5", features = ["fix44"] }
tokio = { version = "1", features = ["full"] }
tracing-subscriber = "0.3"
```

The engine reports through [`tracing`](https://docs.rs/tracing), so install a subscriber, as
`main` does here, to see why a session didn't connect or log on. Without one, an initiator that
fails to connect says so once on stderr and is silent after that.

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
                ctx.send(ack);
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

## The guide

[The guide](https://github.com/jrogers/turbojet/blob/main/docs/guide/README.md) goes from this first program to running sessions in production. Its
examples are compiled and run as tests, and it's also in the API documentation, as
`turbojet::guide`.

1. [A first session](https://github.com/jrogers/turbojet/blob/main/docs/guide/first-session.md): both roles in one program, the `Application`
   callbacks, sending, FIX versions, and how the engine is layered.
2. [Messages](https://github.com/jrogers/turbojet/blob/main/docs/guide/messages.md): typed messages, groups and enums, your own with `fix_message!`,
   and repeating groups.
3. [Session behaviour](https://github.com/jrogers/turbojet/blob/main/docs/guide/sessions.md): session IDs, logon credentials, the checks on inbound
   messages, and validating them against a data dictionary.
4. [Acceptors](https://github.com/jrogers/turbojet/blob/main/docs/guide/acceptors.md): settings and stores per counterparty, connection limits, allowed
   addresses, load balancers, and configuration files, including from QuickFIX's.
5. [Initiators](https://github.com/jrogers/turbojet/blob/main/docs/guide/initiators.md): failover, reconnecting, source addresses and proxies.
6. [TLS](https://github.com/jrogers/turbojet/blob/main/docs/guide/tls.md): certificates from files or memory, replaced while running, PKCS#12 and
   revocation lists.
7. [Storage and recovery](https://github.com/jrogers/turbojet/blob/main/docs/guide/storage.md): what is stored when, delivery at least once, holding a
   message in flight, and the stores.
8. [Schedules and rate limits](https://github.com/jrogers/turbojet/blob/main/docs/guide/schedules.md): trading hours, holidays and throttling.
9. [Operations](https://github.com/jrogers/turbojet/blob/main/docs/guide/operations.md): sequence numbers, managing sessions, cancel on disconnect,
   shutting down, logging, message logs and metrics.
10. [Performance](https://github.com/jrogers/turbojet/blob/main/docs/guide/performance.md): the lowest-latency ways to send, and spinning connections.
11. [Binary encodings](https://github.com/jrogers/turbojet/blob/main/docs/guide/binary.md): SBE codecs, FIXP sessions and FAST.

## The gateway

Speaks FIX 4.2. Handles NewOrderSingle `D` (→ ExecutionReport New/Rejected, or Reject `3` for
malformed fields), OrderCancelRequest `F` (→ ExecutionReport Canceled or OrderCancelReject `9`),
OrderCancelReplaceRequest `G` (→ ExecutionReport Replaced, or OrderCancelReject; quantity, price
and time in force can change, symbol/side/type can't) and OrderStatusRequest `H` (→ status
ExecutionReport, found by any ClOrdID the order has had); other message types get a
BusinessMessageReject `j`. Market and Limit orders, TimeInForce Day/GTC, Side
Buy/Sell/SellShort/SellShortExempt. Business rejections without a specific FIX 4.2 reason code
use the spec's "Broker option", named BrokerCredit (OrdRejReason 0, CxlRejReason 2), with an
explanatory Text. An unknown Side, OrdType or TimeInForce code gets a Reject `3` (value
incorrect); one in a field the gateway doesn't use (e.g. Rule80A 47 or OpenClose 77) is ignored.

The gateway cancels on disconnect, as most venues do: when a session drops without a Logout and
the counterparty doesn't log back on within 5 seconds, its open orders are cancelled and the count
logged. No ExecutionReports are sent, since the counterparty has gone; an OrderStatusRequest after
it logs back on reports them Canceled. With `--config`, the sessions file sets it
(`cancel_on_disconnect = "disconnect"`, `cancel_grace = "5s"`).

## Benchmarks

```sh
cargo bench -p turbojet --all-features            # everything (~13 minutes)
cargo bench -p turbojet --bench codec             # one group: codec, session, roundtrip or fixp
cargo bench -p turbojet --all-features -- --quick # fast smoke run
cargo bench -p turbojet-sql --all-features        # the SQL stores (PostgreSQL at TURBOJET_POSTGRES_URL)
cargo bench -p turbojet-fast                      # FAST decoding and encoding
```

Apple M4 Pro (8 performance and 4 efficiency cores), macOS 27, Rust 1.98.1; criterion medians.
Benchmarks build with one codegen unit and fat LTO (`[profile.bench]`), so results don't shift with
how the compiler happens to partition the crate.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 162 ns / 133 ns | 995 / 1,208 MiB/s |
| Encode NewOrderSingle (169 B) | 40 ns | 3.9 GiB/s |
| Encode ExecutionReport (209 B) | 46 ns | 4.2 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 87 ns / 137 ns | |
| Typed parse NewOrderSingle, owned (no groups / with 3 allocations) | 111 ns / 174 ns | |
| Typed parse FIX 4.4 NewOrderSingle with nested groups (363 B): borrowed / reading every entry / owned | 398 ns / 616 ns / 484 ns | |
| Typed build ExecutionReport | 134 ns | |
| Format a timestamp (same second / new second) | 9.2 ns / 27 ns | |
| Session: order → ack, no I/O, encoded reply (memory store) | 739 ns | 1.35M msg/s |
| Session: order → ack, wire to wire (decode + session, which encodes) | 866 ns | 1.15M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 43 ns / 1.4 µs / 4.0 ms | |
| Store a sent message, 100 per commit: disk / disk + fsync | 59 ns / 40 µs | |
| Store a sent message and commit it: SQLite / SQLite synced / PostgreSQL | 64 µs / 4.0 ms / 138 µs | |
| Store a sent message, 100 per commit: SQLite / SQLite synced / PostgreSQL | 2.5 µs / 48 µs / 6.4 µs | |
| Read a resend step of 256 back: disk / SQLite / PostgreSQL | 11.7 µs / 179 µs / 148 µs | |
| SBE: encode a B3 NewOrderSingle (113 B) / tag=value NewOrderSingle (169 B), reused buffer | 6.7 ns / 30 ns | |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 4.8 ns / 222 ns | |
| FAST: decode a market data refresh of 1 / 10 / 50 entries into a reused message | 216 ns / 1.34 µs / 6.27 µs | |
| FAST: encode the same refreshes | 301 ns / 2.01 µs / 8.65 µs | |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 16.2 µs | 3.33M msg/s |
| Round trip over localhost TCP, one at a time | 31.1 µs | 32.2k/s |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 10.7 µs | 93.7k/s |
| The same, both ends spinning (`run_spinning`) | 9.0 µs | 112k/s |
| Round trip over localhost TCP, 1,000 in flight | | 1.04M msg/s |
| Round trip over localhost TLS, one at a time | 31.3 µs | 31.9k/s |
| Round trip over localhost TLS, one at a time, replying from `on_message` | 21.2 µs | 47.1k/s |
| Round trip over localhost TLS, 1,000 in flight | | 958k msg/s |
| Round trip, acceptor storing to disk: one at a time / 1,000 in flight | 32.8 µs | 913k msg/s |
| Round trip, acceptor storing to disk + fsync: one at a time / 100 in flight | 4.0 ms | 16.7k msg/s |

Round trips are initiator → acceptor application → initiator application, using a store that
discards messages (storage is measured separately), except the disk rows, where the acceptor
stores to `DiskStorage`. Most send each order from the benchmark's task through a `SessionHandle`;
the "replying from `on_message`" rows send the next order from the initiator's application, with
no hop between tasks (see [Performance](https://github.com/jrogers/turbojet/blob/main/docs/guide/performance.md)). The SBE rows use B3's Binary
Entrypoint schema; the FIXP rows carry its orders under FIXP 1.0 with recoverable flows, each end
storing in memory. The FAST rows use the FAST specification's own incremental refresh template,
each message following the one before. The SQL rows used PostgreSQL 18 on the same machine over
TCP, and SQLite synced is `synchronous = FULL` with `fullfsync`, as `DiskStorage`'s fsync is. Rows
were measured 2026-10-18;
[DESIGN.md](https://github.com/jrogers/turbojet/blob/main/docs/DESIGN.md#measurement-history)
says what each was before.

A test counts heap allocations per order → ack, wire to wire, by stage, and fails if any stage's
count changes, up or down, so both regressions and improvements show up in CI:

```sh
cargo test -p turbojet --test allocations -- --nocapture   # prints the table
```

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Decode (into one message reused per connection) | 0 | 0 | 0 |
| Session (including encoding the ack) | 0 | 0 | 0 |
| Application (typed parse and ack) | 0 | 0 | 0 |
| Store: memory / disk | 1.2 / 0.2 | 0 | 280 / 49 |
| Engine (all but the application): memory / disk | 1.2 / 0.2 | 0 | 280 / 49 |

Means of 1,000 orders after 100 warm-up, with each store (the disk store without fsync). Store
allocations are fractional because the stores' maps allocate a node every few messages; the
memory store also copies each message it keeps. Debug and release builds count the same. The
application parses the order borrowed, without allocating, and copies the strings it needs into
its owned ExecutionReport, whose `CompactString`s keep them inline. `Context::send` writes the
ExecutionReport into a message the session reuses, which allocates nothing once warmed up.

The same test counts a FIXP server (`turbojet::fixp`) answering each B3 NewOrderSingle with an
order, recoverable flows both ways, as the FIXP benchmark does:

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Session (framing, the session's own messages, encoding the answer) | 0 | 0 | 0 |
| Application (B3 decode and answer) | 0 | 0 | 0 |
| Store: memory / disk | 1.2 / 0.2 | 0 | 182 / 49 |

Means of 1,000 orders after 100 warm-up. The stores cost what they do for FIX: the
memory store copies each answer it keeps, and both index them.

Another test counts typed parsing alone, per FIX 4.2 NewOrderSingle:

| Parse | Allocations | Reallocs | Bytes |
|---|---|---|---|
| `NewOrderSingleRef`: no groups / 3 allocations, every entry read | 0 / 0 | 0 | 0 / 0 |
| `NewOrderSingle`: no groups / 3 allocations | 0 / 1 | 0 | 0 / 144 |

The owned form allocates a `Vec` for each group, and a text value only if it's longer than 24
bytes; ClOrdID, Symbol and Account here are kept inline.

## Limitations

[CAVEATS.md](https://github.com/jrogers/turbojet/blob/main/docs/CAVEATS.md) lists behaviour worth knowing about, and
[ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/docs/ROADMAP.md) the planned work. In brief:

- Gateway orders live in memory; a restart keeps session state but forgets orders, and orders
  are acknowledged but not routed or matched.
- Resends read the disk store on the connection task, and opening a session's store scans its
  segments (up to 1 GiB by default) there.
- Typed messages come for FIX 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1 and 5.0 SP2; other versions need `turbojet-codegen`,
  or `fix_message!` for messages defined by hand.
- FIXP sessions run standard FIXP only: not yet a venue's dialect.
- FAST is a codec only, for FAST 1.1; receiving a feed is the application's.

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
`OrchestraFIX40.xml`, `OrchestraFIX41.xml`, `OrchestraFIX43.xml` and `OrchestraFIX50SP2.xml` are
converted from the FIX Unified Repository,
as the `NOTICE` there explains. The generated crates take their names and structure from them, but
don't reproduce their documentation; see their `NOTICE` files.

The FIXP 1.0 session schema, `fixp-1.0.xml` in [`dictionaries/sbe`](https://github.com/jrogers/turbojet/tree/main/dictionaries/sbe),
is © FIX Protocol Ltd. and reproduced unmodified under the Creative Commons
Attribution-NoDerivatives 4.0 International licence (see the `NOTICE` there). `turbojet::fixp`'s
generated session messages take their names and structure from it, but don't reproduce its
documentation; see the `turbojet` crate's `NOTICE`.

The other SBE schemas there, used only by tests and benchmarks, are SBE's example schema
(real-logic, Apache License 2.0) and B3's Binary Entrypoint schema, by way of Artio; the
`README.md` there gives their sources.

The session acceptance scripts in
[`crates/turbojet-acceptance/definitions`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-acceptance/definitions) are QuickFIX's,
under the QuickFIX Software License (the `LICENSE` there). This product includes software developed
by quickfixengine.org (http://www.quickfixengine.org/).
