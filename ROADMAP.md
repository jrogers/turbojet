# Roadmap

What's left to do on Turbojet and the example gateway, in rough priority order. Each item says
why it matters and roughly how big it is: **S** (a day or less), **M** (a few days), **L** (a week
or more). What's built, and how, is in [DESIGN.md](DESIGN.md); behaviour worth knowing about is in
[CAVEATS.md](CAVEATS.md).

Turbojet has been tested against two other FIX engines (QuickFIX/J and quickfix-go) and QuickFIX's
scripted session scenarios, but not yet with any venue or real counterparty.

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

What's left is more FIX versions and the rest of Orchestra.

- **Generate more FIX versions** (M). FIX 5.0 and 5.0 SP1 have no bundled source; they could be
  converted from the Unified Repository like FIX 4.3 and 5.0 SP2 (`convert-unified.sh`). Extension
  packs after FIX 5.0 SP2 are in FIX Latest (EP312), which generates and compiles cleanly (172
  messages, 578 groups, 1,959 enums, about 192,000 lines; 9 of its groups start with a nested group,
  whose NumInGroup delimits their entries) but isn't vendored or checked in yet (see "More of FIX
  Orchestra"). It's big: built alone (Apple M3, Rust 1.98.1), about 44 s debug and 155 s release,
  against 6 s and 16 s for FIX 4.4. Before checking in a `turbojet-fixlatest`, consider splitting it
  by message category behind features, or generating only chosen messages (`skip_message`, an
  include list), so users pay only for what they use.
- **More of FIX Orchestra** (M). Loading covers what the standard FIX 4.2 and 4.4 files use.
  Still to do:
  - scenarios (per-workflow variants of a message) and `constant` and `forbidden` presence,
    which the loader rejects with an error for now;
  - venue overlays in Orchestra format (`merge_xml` takes only QuickFIX format);
  - FIX Latest: vendor its file, generate it (with the nested-group rule above), and use the
    FIXT file as its transport;
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

- **Inbound without the copy** (M). Decoding still copies each frame, once, out of the read
  buffer into the reused message. A view borrowing the read buffer would avoid that, at the cost
  of a second message type through `Fields`, the typed-message macros and the generated crates;
  worth it only if a benchmark shows the copy matters.
- **Large generated messages** (M). Generated messages carry every field in the dictionary, so
  a FIX 4.2 NewOrderSingle is 1,008 bytes against 200 for the old hand-written one (when it was
  1,144 bytes, parsing it took about 237 ns against 202 ns). Lookup and conversion cost the same;
  the gap is initialising, copying and dropping the larger struct, and it grows with FIX 4.4 and
  5.0 (1,976 bytes for FIX 4.4's). The borrowed forms are about 15% smaller (840 and 1,680 bytes)
  and allocate nothing: a FIX 4.2 NewOrderSingle parses in about 98 ns borrowed against 148 ns
  owned, and a FIX 4.4 one with nested groups in 448 ns against 692 ns. They are still mostly
  empty `Option`s. Options, cheapest first:
  - Build the struct in place (`MaybeUninit`) instead of copying it into the return slot. No API
    change; an estimated 15-35 ns, not yet measured. Needs `unsafe` in an exported macro, with
    care over drops on the error path and when a conversion panics.
  - Move rarely used fields into an `Option<Box<...>>` sub-struct per message, which brings
    parsing and dropping close to the old numbers (about 35-55 ns saved). Changes the generated
    API, and needs a rule, or per-venue configuration, for which fields are common.
  - A codegen option to generate only chosen fields per message, as the hand-written module
    did. Matches the old numbers, but the generated API then depends on configuration.
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
- **Memory-mapped disk storage** (M, research). `DiskStorage` writes each commit with a system
  call (and, with sync, an `fsync`). Writing its segments through a memory map instead would
  replace the write calls with stores to mapped memory, the kernel writing pages back (or
  `msync` making a commit durable). Measure it against `DiskStorage`, with and without sync, on
  the store and round-trip benchmarks. What it would cost: `unsafe` (a mapping crate such as
  `memmap2`, so a crate of its own or a reviewed exception), files sized ahead of use, and a
  process killed with `SIGBUS` if a mapped file is truncated or the disk fills; the power-loss
  tests (`turbojet-sim`) would have to hold for it as they do for `DiskStorage`.

- **Cache-aligned data** (S each, research). Data shared between threads, such as the session
  registry, the command queues, metrics counters and `MemoryStorage`'s per-session state, can
  share a cache line with unrelated data that another core writes (false sharing), and a
  session's per-message fields are spread across a large struct. Pad what's contended to a
  cache line (`#[repr(align(64))]`, or 128 bytes on Apple silicon) and group the fields the hot
  path touches. It needs a benchmark with many sessions on several cores, which doesn't exist
  yet: the round-trip benchmarks run one session.

## 5. Operations and deployment

- **QuickFIX settings files** (S). `turbojet-config` reads TOML. An importer for the QuickFIX
  `.cfg` keys that map (BeginString, SenderCompID, TargetCompID, SocketConnectHost/Port,
  StartTime/EndTime, HeartBtInt, ResetOnLogon and the like) would ease moving from QuickFIX,
  QuickFIX/J or QuickFIX/n.
- **Management API and console** (L). The operator API works on connected sessions in the
  library, but nothing exposes it remotely, and the gateway only has the offline `seqnums`
  tool. Provide an optional management endpoint (HTTP, or a local socket) that lists sessions
  and their state; starts, stops, logs out and resets them; triggers resends; and browses and
  searches the message log live. A web console on top is what commercial engines sell on. The
  gateway should use it.
- **Message log compression and archiving** (S). `FileMessageLog` rotates files and deletes them
  after a retention period, but leaves compressing and archiving finished files to the operator.
  Compressing them as they're finished would need a compression crate, behind a feature; a hook
  called with each finished file would let one archive it.
- **Alternative storage backends**. Today there are `MemoryStorage`, `DiskStorage` and
  `turbojet-sql`'s `SqlStorage`. Each backend should live behind its own feature (or in its own
  crate) so its dependencies stay optional, record `created_at` so session schedules work, pass
  the conformance suite (`turbojet::store::conformance`), and be measured against `DiskStorage`.
  - **Embedded database** (M). A `SessionStorage` over [redb](https://docs.rs/redb) (or a
    similar embedded store) could be a better default than the hand-rolled files: sequence
    numbers and message bodies update in one transaction, so there's no torn-write recovery, and
    space is reclaimed without the rotation `DiskStorage` needs. Benchmark it against
    `DiskStorage` with and without fsync. If it matches or beats it, make it the recommended
    durable store.
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
- **TLS certificate revocation** (S). Client and server certificates are checked against their
  CA but not for revocation. Check CRLs (and optionally OCSP), and accept PKCS#12 bundles as well
  as PEM files.
- **Interop over TLS** (M). QuickFIX/J and quickfix-go are tested directly and through a
  fault-injecting proxy (lost, garbled, cut and delayed messages, a silent counterparty, a slow
  link and a stalled reader), but only over plain TCP.

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

- **Other encodings**. FIXML and FAST.
- **Routing and translation between counterparties**. A hub that routes messages between
  sessions and maps one counterparty's dialect to another's with rules (like FIX Antenna's
  FIXEdge). Routing fields are in scope; the hub is an application.
- **Ready-made venue dictionaries**. Pre-built, certified definitions for particular venues
  (CME iLink, ICE, LSE, Nasdaq). Dictionary support makes them straightforward to generate, but
  maintaining and certifying them is a separate effort.
- **Bindings for other languages**. A C API or Python bindings. Turbojet is a Rust library.
