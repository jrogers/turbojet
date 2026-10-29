<p align="center"><img src="https://raw.githubusercontent.com/jrogers/turbojet/main/docs/assets/logo.svg" width="120" alt=""></p>

<h1 align="center">turbojet</h1>

<p align="center">
  <a href="https://crates.io/crates/turbojet"><img src="https://img.shields.io/crates/v/turbojet" alt="crates.io"></a>
  <a href="https://docs.rs/turbojet"><img src="https://img.shields.io/docsrs/turbojet" alt="docs.rs"></a>
  <a href="https://github.com/jrogers/turbojet/actions/workflows/ci.yml"><img src="https://github.com/jrogers/turbojet/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-Apache--2.0-blue" alt="License: Apache-2.0"></a>
</p>

<p align="center">
  <a href="https://jrogers.github.io/turbojet/">Website</a> ·
  <a href="https://jrogers.github.io/turbojet/guide/">Guide</a> ·
  <a href="https://docs.rs/turbojet">API</a>
</p>

> [!WARNING]
> **Pre-1.0, and not yet proven in production.** Turbojet
> passes 440 of QuickFIX's 467 scripted session acceptance scenarios, runs against QuickFIX/J and
> QuickFIX/n in CI, and is fuzzed, but it hasn't been certified with any venue or run against a
> real counterparty, and its APIs will change before 1.0. Test it thoroughly yourself before trading real money with it. It is provided as is,
> without warranty of any kind; known gaps are in [CAVEATS.md](https://github.com/jrogers/turbojet/blob/main/docs/CAVEATS.md).

FIX (Financial Information eXchange) is the messaging standard of electronic trading: brokers,
exchanges and investment firms send each other orders, quotes and executions over it in real
time.

**Turbojet** is a FIX engine in Rust, on tokio: codec, session layer, storage, `Acceptor` and
`Initiator`, with typed messages for FIX 4.0 to 5.0 SP2, FIXT.1.1 sessions, and SBE, FIXP and
FAST for binary venues. It puts **safety first, then performance, then developer experience**: a
bug in a FIX engine can send a duplicate order or lose a fill.

- **Safe.** What a counterparty sends is bounded, invariants are asserted, and your application
  gets each message at least once. The parsers and session are fuzzed, and both ends run against
  each other in a deterministic simulator through crashes, store failures, power loss and a
  hostile network. [STYLE.md](https://github.com/jrogers/turbojet/blob/main/docs/STYLE.md) says how, after TigerBeetle's
  [Tiger Style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md).
- **Interoperable.** CI runs sessions against QuickFIX/J and QuickFIX/n in both roles, on FIX
  4.2 to 5.0 SP2, over TLS and through a proxy that loses, garbles and delays messages.
- **Fast.** An order to its acknowledgement takes under 1 µs wire to wire, and decoding, the
  session and encoding allocate nothing ([Benchmarks](#benchmarks)).
- **Embeddable.** A library, not a server. Every layer is public, from the codec through the
  sans-IO session to the transports, and stores, clocks and transports can be replaced.
- **Pure Rust.** No `unsafe` in the engine. The only C and assembly are in `ring`, under the
  optional `tls` feature.

| Crate | |
|---|---|
| [`turbojet`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet) | The engine: codec, sessions, stores, acceptor and initiator, FIXP sessions, FAST (`fast` feature), message logs |
| [`turbojet-fix`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-fix) | Every FIX 4.0 to 5.0 SP2 application message, generated from the FIX Trading Community's data, a feature per version |
| [`turbojet-dictionary`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-dictionary), [`turbojet-codegen`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-codegen) | Load Orchestra or QuickFIX XML with a venue's changes, and generate typed messages or SBE codecs from it |
| [`turbojet-sql`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-sql) | Session state in SQLite or PostgreSQL, with leases so instances can share a database |
| [`turbojet-replica`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-replica) | A hot standby that takes sessions over with no sequence gap |
| [`turbojet-config`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-config) | Acceptors and initiators from a TOML file, reloaded while running; converts QuickFIX `.cfg` files |
| [`turbojet-log`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-log) | Prints and filters message logs |
| [`turbojet-python`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet-python) | Initiators from Python's asyncio (built with maturin; not on PyPI) |

How the pieces fit is in [ARCHITECTURE.md](https://github.com/jrogers/turbojet/blob/main/docs/ARCHITECTURE.md), how they're built and
measured in [DESIGN.md](https://github.com/jrogers/turbojet/blob/main/docs/DESIGN.md), and planned work in [ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/docs/ROADMAP.md).

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
turbojet = "0.7"
turbojet-fix = { version = "0.7", features = ["fix44"] }
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

## The guide

[The guide](https://jrogers.github.io/turbojet/guide/) goes from this first program to running sessions in production. Its
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

## Examples

In [`crates/turbojet/examples`](https://github.com/jrogers/turbojet/tree/main/crates/turbojet/examples); each says at its top how to run
it, and takes `--help`.

- `quickstart`: the program above.
- `gateway`: a FIX 4.2 order-entry acceptor that keeps orders in memory, with the engine's
  options (TLS, schedules, rate limits, metrics, message logs) on the command line or from a
  config file. It's a test bed for the engine, not a product.
- `client`: an initiator for the gateway: logon, order, cancel, logout.
- `ack` and `load`: an acceptor that answers each order and keeps nothing, and a load generator
  that reports round-trip latency percentiles at a fixed rate against any acceptor.

## Benchmarks

```sh
cargo bench -p turbojet --all-features -- --quick   # fast smoke run
cargo bench -p turbojet --bench codec               # one group: codec, session, roundtrip or fixp
```

Apple M4 Pro (8 performance and 4 efficiency cores), macOS 27, Rust 1.98.1; criterion medians.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 163 ns / 134 ns | 987 / 1,203 MiB/s |
| Encode ExecutionReport (209 B) | 45 ns | 4.3 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 85 ns / 137 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 864 ns | 1.16M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 43 ns / 1.5 µs / 3.9 ms | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 11.2 µs | 89.3k/s |
| The same, both ends spinning (`run_spinning`) | 9.2 µs | 109k/s |
| Round trip over localhost TCP, 1,000 in flight | | 1.03M msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 962k msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 16.2 µs | 3.20M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 4.7 ns / 219 ns | |

Round trips go initiator → acceptor application → initiator, with a store that discards
messages. [DESIGN.md](https://github.com/jrogers/turbojet/blob/main/docs/DESIGN.md#current-numbers) has every benchmark, what
each measures, and their history.

A test counts heap allocations per order → ack, wire to wire, by stage, and fails CI if any count
changes (`cargo test -p turbojet --test allocations -- --nocapture`):

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Decode (into one message reused per connection) | 0 | 0 | 0 |
| Session (including encoding the ack) | 0 | 0 | 0 |
| Application (typed parse and ack) | 0 | 0 | 0 |
| Store: memory / disk | 1 / 0 | 0.004 | 278 / 46 |
| Engine (all but the application): memory / disk | 1 / 0 | 0.004 | 278 / 46 |

The memory store copies each message it keeps; the reallocs are the stores' indexes growing.

## Limitations

[CAVEATS.md](https://github.com/jrogers/turbojet/blob/main/docs/CAVEATS.md) lists behaviour worth knowing about, and
[ROADMAP.md](https://github.com/jrogers/turbojet/blob/main/docs/ROADMAP.md) the planned work. In brief:

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
