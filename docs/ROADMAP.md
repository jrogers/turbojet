# Roadmap

What's left to do on Turbojet and the example gateway, in rough priority order. Each item says
why it matters and roughly how big it is: **S** (a day or less), **M** (a few days), **L** (a week
or more). What's built, and how, is in [DESIGN.md](DESIGN.md); behaviour worth knowing about is in
[CAVEATS.md](CAVEATS.md).

Turbojet has been tested against two other FIX engines (QuickFIX/J and QuickFIX/n)
and QuickFIX's scripted session scenarios, but not yet with any venue or real counterparty.

## 1. Protocol completeness

Venues' own binary order-entry protocols, which we may want to support:

- **CME iLink 3** (L). CME's dialect of FIXP over SBE: Negotiate and Establish signed with HMAC
  over an access key, sequence numbers in each message's business header, and its own framing.
  Its specification and SBE schema are public on CME's Client Systems Wiki, but testing against
  CME needs certification credentials; a dialect beside `turbojet::fixp`'s standard FIXP,
  sharing its driver and much of its session.
- **Nasdaq OUCH** (L). Nasdaq's binary order-entry protocol, over SoupBinTCP (login, sequenced
  messages, heartbeats) rather than FIXP; its fixed-length messages would be a codec of their own.
  A session layer beside the FIX and FIXP ones, through the same connection driver.
- **Multiplexed FIXP sessions** (M). `turbojet::fixp` runs one session per connection; FIXP also
  allows several over one.

## 2. Dictionaries and code generation

What's left is FIX Latest and the rest of Orchestra.

- **FIX Latest in `turbojet-fix`** (M). Extension packs after FIX 5.0 SP2 are in FIX Latest
  (EP312, `OrchestraFIXLatest.xml` in the orchestrations commit already pinned), which generates
  and compiles cleanly with its own header (172 application messages, about 190,000 lines; 9 of
  its groups start with a nested group, whose NumInGroup delimits their entries) but isn't
  vendored as a feature. It's too slow to build whole: 104 s debug and 23 minutes release (clean,
  Apple M3, 2026-10-16), against 25 s and 118 s for FIX 5.0 SP2. An application can generate
  just the messages it uses meanwhile (`turbojet-codegen --message`): five order messages build in
  18 s debug and 279 s release. Worth doing after "Generated code's build time" below; QuickFIX
  has no acceptance scripts for it.
- **More of FIX Orchestra** (M). Loading covers what the standard FIX 4.2 and 4.4 files use.
  Still to do:
  - scenarios (per-workflow variants of a message) and `constant` and `forbidden` presence,
    which the loader rejects with an error for now;
  - venue overlays in Orchestra format (`merge_xml` takes only QuickFIX format);
  - deprecation of code set values: Orchestra marks codes deprecated as well as fields, but the
    model records it only for fields, so a deprecated value's variant isn't marked as one.

## 3. Encoding and data types

- **Non-UTF-8 text** (S). Text in another encoding (MessageEncoding 347) belongs in the Encoded*
  data fields, which carry it intact but only as bytes: nothing decodes it. Decoding by
  MessageEncoding (Shift_JIS, say) would need an encoding crate. Outside data fields, a value
  that isn't UTF-8 is rejected with SessionRejectReason 6, and in a header field the session
  relies on (MsgType, the CompIDs, MsgSeqNum, SendingTime) the message is ignored as garbled.

## 4. Performance

Each item needs a benchmark that shows the cost before the change is worth its complexity.

- **Large generated messages** (M). Generated messages carry every field in the dictionary, so
  a FIX 4.2 NewOrderSingle is 1,376 bytes owned and 1,184 borrowed, and a FIX 4.4 one 3,080 and
  2,768, mostly empty `Option`s. Measured on 2026-10-15 (see DESIGN.md), size costs less than
  it looks: copying a whole FIX 4.2 NewOrderSingle takes 15 ns of its 130 ns owned parse, and
  the largest share of a borrowed parse is setting its 68 slots empty and dispatching each field.
  Options, cheapest first:
  - Move rarely used fields into an `Option<Box<...>>` sub-struct per message: fewer slots to
    set, copy and drop. Probably 10-20 ns on a FIX 4.2 NewOrderSingle, not yet measured. Changes
    the generated API, and needs a rule, or per-venue configuration, for which fields are common.
  - A codegen option to generate only chosen fields per message, as the hand-written module
    did. Matches the old numbers, but the generated API then depends on configuration.
- **Generated code's build time** (M). Release builds of large generated messages are slow:
  FIX 5.0 SP2 takes 118 s, and FIX Latest's ExecutionReport alone (619 fields, 367 groups, 1,435
  enumerated fields) 180 s, nearly all of it in LLVM's passes. Profiled with `cargo llvm-lines`
  (2026-10-16), about 45% of that ExecutionReport's IR is per enum: `from_code`'s `match` on the
  code and the `Code<E>` helpers each enum instantiates (`take_value`, `take_value_ref`,
  `write_fix`, `write_segment`, `parse_ref`); groups' `from_fields` and `into_owned` are another
  30%. Options: table-driven enums (a static code table searched by one shared function, `code()`
  by index), helpers for `Code<E>` that do the work outside the generic part, and the same for
  groups. Each needs typed parse benchmarks before and after, since enum parsing is on the hot
  path. Meanwhile, `opt-level = 1` for the generated crate halves its build (91 s for that
  ExecutionReport) at some cost to parse speed, not measured.
- **Kernel bypass, busy-polling and hardware timestamps** (L, research). Busy-polling is done where
  it can be measured on macOS (`run_spinning`; see DESIGN.md). What remains needs a Linux machine
  with isolated cores, which hasn't been available:
  - measure the tokio driver and the spinning one under `net.core.busy_poll`/`busy_read`,
    `taskset` and isolated cores (macOS has no thread affinity API), and under OpenOnload, which
    accelerates sockets and epoll without code changes (on Solarflare/X2 cards, or its AF_XDP
    mode, to check on the cloud machine chosen);
  - only if those numbers call for it, a crate of its own (as `turbojet-sql`, since FFI needs
    `unsafe`) wrapping TCPDirect's zero-copy TCP API in a stream `run_spinning` takes; DPDK or
    AF_XDP would need a userspace TCP stack, so stay parked;
  - hardware packet timestamps (`SO_TIMESTAMPING`), feeding the latency histograms.
- **Cache-aligned data** (S each, research). Data shared between threads, such as the session
  registry, the command queues, metrics counters and `MemoryStorage`'s per-session state, can
  share a cache line with unrelated data that another core writes (false sharing), and a
  session's per-message fields are spread across a large struct. Pad what's contended to a
  cache line (`#[repr(align(64))]`, or 128 bytes on Apple silicon) and group the fields the hot
  path touches. Measure first with `benches/sessions.rs` (N sessions pipelining at once) on a
  machine with more cores than an M3's 4+4, where it levels off with the cores at 4 to 8
  sessions, and compare against a padded build; expect the allocator and the registry's lock to
  show up before false sharing.

## 5. Operations and deployment

- **Encrypting the message log** (S). `FileMessageLog` masks passwords and credentials, but the
  rest of every order and execution is in the clear. Encrypting finished files, alongside
  compressing them, would let the log be kept where fewer people may read it.
- **Alternative storage backends**. Today there are `MemoryStorage`, `DiskStorage` and
  `turbojet-sql`'s `SqlStorage`. Each backend should live behind its own feature (or in its own
  crate) so its dependencies stay optional, record `created_at` so session schedules work, pass
  the conformance suite (`turbojet::store::conformance`), and be measured against `DiskStorage`.
  - **SQL stores, next steps** (S each). Move to sqlx 0.9 once the minimum Rust reaches 1.94.
    Without sync, a SQLite commit costs about 25 times a `DiskStorage` one (75 µs against 3 µs);
    rusqlite with blocking jobs may close some of that gap. Leases compare gateways' clocks;
    the database's own clock would remove that requirement but differs between SQLite and
    PostgreSQL. A gateway learns that its lease was taken only at its next commit; renewing it
    on a timer would let an idle session notice sooner.
- **High availability with replicated state** (L). A hot standby that receives every sequence
  number change and stored message as it happens, and takes over a session on failover with no
  gap and no resend storm. Replicate the session journal to the standby, with the exclusive-session
  lease deciding which instance is active. Cluster coordination and replication would be a separate
  project; Turbojet's part is what it needs from the engine.
- **More TLS revocation sources** (S each). Revocation is checked against CRLs the application
  gives; OCSP (stapled, or fetched) and fetching CRLs from a certificate's distribution points
  aren't supported.

## 6. The example gateway

The gateway exists to exercise Turbojet; these matter only if it becomes more than an example.

- **Matching or routing** (L). Orders are acknowledged and booked, but never filled or sent on.
- **Persistent orders** (M). Orders live in memory and are lost on restart; session state
  survives.
- **Allocation checks** (S). Allocation quantities aren't checked against the order quantity.
- **ClOrdID history** (S). After a replace, earlier ClOrdIDs still find the order; strict venues
  reject them.
- **Example client** (S). It doesn't demonstrate replace or status requests.
- **Kill switch** (S). Cancel a session's resting orders, or all of them, on an operator command.
  Almost every venue offers this alongside cancel on disconnect, which the gateway does.
- **Pre-trade risk checks** (M). Order size limits, price bands against a reference price,
  credit limits and self-trade prevention, rejected before booking.

## Out of scope for now

Listed so the decisions are explicit; any of these could be built on Turbojet separately.

- **FIXML**. FIX's XML encoding.
- **Routing and translation between counterparties**. A hub that routes messages between
  sessions and maps one counterparty's dialect to another's with rules (like FIX Antenna's
  FIXEdge). Routing fields are in scope; the hub is an application.
- **Ready-made venue dictionaries**. Pre-built, certified definitions for particular venues
  (CME iLink, ICE, LSE, Nasdaq). Dictionary support makes them straightforward to generate, but
  maintaining and certifying them is a separate effort.
- **Bindings for other languages**. A C API or Python bindings. Turbojet is a Rust library.
