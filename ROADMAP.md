# Roadmap

What's left to do on Turbojet and the example gateway, in rough priority order. Each item says
why it matters and roughly how big it is: **S** (a day or less), **M** (a few days), **L** (a week
or more).

A standing goal behind all of it: the engine's hot path should get as close as possible to zero
copies and zero allocations per message (see "Zero-copy, zero-allocation hot path" under
Performance).

## Where things stand

Turbojet has a complete FIX session layer (logon, including credentials and FIX 4.4
NextExpectedMsgSeqNum; sequencing, gap detection and resend, heartbeats, logout) for FIX 4.2 and
later, with:

- FIXT.1.1 sessions for FIX 5.0 SP2: DefaultApplVerID(1137) configured and negotiated at logon (an
  acceptor may support several versions and echoes the counterparty's), per-message
  ApplVerID(1128) naming any supported version in either direction, CstmApplVerID and ApplExtID
  passed through, rejects naming the version of the message they answer, messages resent in the
  version they were sent in, and dictionary validation chosen by application version;
- every FIX 4.2, 4.3, 4.4 and 5.0 SP2 application message, group and enum, generated from the
  official FIX data (`turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44`, `turbojet-fix50sp2`),
  and public macros for defining your own, including repeating groups;
- a loader for FIX Orchestra and QuickFIX-format dictionaries with venue merging
  (`turbojet-dictionary`), and a code generator that runs from an application's `build.rs` or as
  a command (`turbojet-codegen`);
- acceptor and initiator transports, TLS with optional mutual authentication, initiator
  failover, and graceful shutdown (every session logged out, bounded by the logout timeout),
  which the gateway does on SIGINT and SIGTERM;
- a malformed body field (no `=`, an invalid tag, non-UTF-8 data) answered with a Reject rather
  than discarded, and an unanswered ResendRequest re-sent once and then ended with a Logout;
- data fields (RawData, XmlData, the Encoded* fields, a venue's own) carrying any bytes, SOH
  included: decoded by the length their Length field gives, stored and resent intact, checked on
  the way out, and typed as `Vec<u8>`;
- typed dates, times (UTC and with a zone), MonthYear, `char` and multi-value lists, with
  timestamps written at the precision they arrived with (seconds to nanoseconds), and SendingTime
  at a configured one;
- memory and disk session storage, session schedules, and operator control of sequence numbers;
- structured logging and Prometheus-compatible metrics;
- criterion benchmarks (about 1.2 µs per order → ack of session processing, and 490k msg/s
  pipelined over localhost TCP);
- a count of heap allocations, reallocs and bytes per order → ack, by stage (decode, session,
  application, store, encode), checked against an exact budget on every test run
  (`tests/allocations.rs`);
- GitHub Actions CI on every push: clippy and tests for each feature combination, the 1.89
  minimum Rust version, `cargo fmt --check`, rustdoc with warnings as errors, a compile check
  of the benchmarks, a check that the generated crates match their dictionaries, and a check
  that the fuzz targets build;
- cargo-fuzz targets for the codec, message parsing, every generated message type and the
  session state machine (`crates/turbojet/fuzz`, `scripts/fuzz.sh`), fuzzed nightly in CI;
- sessions tested against QuickFIX/J in CI (`turbojet-interop`), with Turbojet as initiator and as
  acceptor, on FIX 4.2, 4.3 and 4.4 and on FIXT.1.1 with FIX 5.0 SP2: logon and logout, reconnection,
  heartbeats and TestRequests, application messages (one with XmlData containing SOH), gap fills
  and resends in each direction, SequenceReset-Reset and MsgSeqNum too low. They found Heartbeats
  going out a second late, since fixed.
- QuickFIX's 235 scripted session acceptance scenarios, which cover the FIX specification's
  session test cases, run on every build (`turbojet-acceptance`). They found four deviations from
  the spec's test cases, since fixed; 221 pass, and the 14 that fail are listed with their reasons
  in `known_failures.txt`: two deliberate differences (keeping a valid message that follows a
  too-long BodyLength, and no RefTagID for a negative tag) and scripts that rely on QuickFIX's own
  behaviour or dictionaries;
- SendingTime accuracy (within 120 s by default), OrigSendingTime on resends, and header field
  order checked on every message, each of which can be turned off;
- messages that arrive ahead of a gap kept until it's filled, then processed in order, and a
  Logout that arrives during one answered at once;
- a counterparty's sequence reset while logged on (a Logon with ResetSeqNumFlag=Y at 1) accepted;
- routing fields (OnBehalfOf and DeliverTo) kept in the header, and session-level rejects routed
  back to where the message came from;
- at-least-once delivery: an inbound message counts as received only once the application has
  handled it, and the one message in flight at a crash is marked as possibly handled when it's
  resent (`Context::maybe_redelivered`).

It has been tested against one other FIX engine (QuickFIX/J) and QuickFIX's scripted session
scenarios, but not yet with any venue or real counterparty. Version 0.1.0 of all seven crates was
published on crates.io on 2026-09-29.

## 1. Following up the first release

Nothing left: the crates.io and docs.rs pages are linked and render correctly.

## 2. Protocol completeness

Nothing left for now: what the session layer covers is under *Where things stand*.

## 3. Dictionaries and code generation

Typed messages come from data dictionaries. `turbojet-dictionary` loads and validates the FIX
Trading Community's official FIX Orchestra files, which give groups and enum values their official
names (`PreAllocGrp`, `OrderCancelRequest`) and document messages, fields and values, as well as
dictionaries in the QuickFIX XML format, the one venues ship their specs in. It merges a venue's
QuickFIX-format additions onto either, keeping the official names and documentation where the venue
re-lists something. `turbojet-codegen` turns a dictionary into `fix_enum!`, `fix_group!` and
`fix_message!` invocations (copying the dictionary's documentation into doc comments only on
request), from an application's `build.rs` or as a command for checked-in crates. The macros convert
and write each field through out-of-line helpers, one copy per field type rather than one per field,
which keeps whole versions quick to build: built alone (Apple M3, Rust 1.98.1, 2026-09-28),
`turbojet-fix44` (84 messages) takes about 6 s debug and 16 s release, down from 75 s, and
`turbojet-fix42` about 1.4 s and 2.6 s, for about 3% on the session benchmarks. The Orchestra files
for FIX 4.2, 4.4 and FIXT 1.1 are vendored in `dictionaries/orchestra` (Apache-2.0, © FIX Protocol
Limited), with FIX 4.3 and FIX 5.0 SP2 converted from the FIX Unified Repository by the FIX Trading
Community's own stylesheet (`convert-unified.sh`); each version is generated into its own crate
(`turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44`, `turbojet-fix50sp2`, the last over FIXT 1.1's
header), and CI checks they match. A dictionary's mistakes can be corrected before generating
(`Dictionary::make_optional`, `--optional`; `turbojet-fix42` makes QuoteCancel's NoQuoteEntries
optional, as FIX 4.4 did), and enumerated fields can accept unknown codes (`--lenient-enums`,
`fields::Code`). Sessions can also check inbound messages against a dictionary (feature
`validation`, `SessionConfig::with_dictionary`). What's left is more versions and the rest of
Orchestra.

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

## 4. Encoding and data types

- **Non-UTF-8 text** (S). Text in another encoding (MessageEncoding 347) belongs in the Encoded*
  data fields, which carry it intact but only as bytes: nothing decodes it. Decoding by
  MessageEncoding (Shift_JIS, say) would need an encoding crate. Outside data fields, a value
  that isn't UTF-8 is rejected with SessionRejectReason 6, and in a header field the session
  relies on (MsgType, the CompIDs, MsgSeqNum, SendingTime) the message is ignored as garbled.

## 5. Performance

### Zero-copy, zero-allocation hot path

The long-term goal is that a message in steady state, from the read buffer through the session
and application and back out to the socket, is neither copied nor allocated beyond what the
application itself asks for. Some of this is already done: a `Message` keeps all its fields in
one buffer with an offset index (two allocations, not one per field), outgoing messages are
encoded into one reused batch buffer, and raw group access is zero-copy. Allocations per order →
ack are counted by stage (`tests/allocations.rs`), and the build fails if a count changes, so each
step below shows up as a lower budget: as of 2026-09-30 the engine makes about 11 per order
(decode 2, session 6, store 2, and the reply list the application's first send grows), and the
example application 7. What remains, per message:
- **Borrowed inbound messages** (M). Decoding copies each frame out of the read buffer into an
  owned `Message`. Decode instead into a view borrowing the read buffer, with its field index in
  reused storage, and hand applications that; they copy into an owned `Message` only to keep
  one. The read loop then has to finish with a frame before compacting or refilling the buffer.
- **Borrowed typed messages** (M). Typed parsing allocates a `String` for each text field and a
  `Vec` for each group. Parse into types that borrow from the message (`&str` fields, groups as
  iterators over entries), keeping owned types for building outbound messages. This changes the
  macros, and generated code (section 3) should produce both.
- **Encode straight to the wire** (M). Sending builds a second `Message` that copies the body
  next to the header, then encodes it. Write the header, body and trailer directly into the
  output buffer in one pass (checksum computed while copying, BodyLength back-patched), and
  store those encoded bytes rather than the message: `DiskStorage` currently encodes each stored
  message a second time, and `MemoryStorage` clones it. Format SendingTime without allocating,
  reusing the date part within the same second.
- **Reuse per-call buffers** (S). Each call into the session returns a new `Vec<Action>`; drain
  a reused outbox instead. Let applications build outbound messages in pooled buffers rather
  than a fresh `Message` each time.

This interacts with "Batched disk writes", "Bounded memory store" (a store of encoded bytes can
be a fixed ring or arena) and "Latency" below.

### Measured bottlenecks

From the benchmarks.

- **Large generated messages** (M). Generated messages carry every field in the dictionary, so
  a FIX 4.2 NewOrderSingle is 1,144 bytes against 200 for the old hand-written one, and parsing
  it takes about 237 ns against 202 ns. Lookup and conversion cost the same; the gap is
  initialising, copying and dropping the larger struct, and it grows with FIX 4.4 and 5.0.
  Options, cheapest first:
  - Build the struct in place (`MaybeUninit`) instead of copying it into the return slot. No API
    change; an estimated 15-35 ns, not yet measured. Needs `unsafe` in an exported macro, with
    care over drops on the error path and when a conversion panics.
  - Move rarely used fields into an `Option<Box<...>>` sub-struct per message, which brings
    parsing and dropping close to the old numbers (about 35-55 ns saved). Changes the generated
    API, and needs a rule, or per-venue configuration, for which fields are common.
  - A codegen option to generate only chosen fields per message, as the hand-written module
    did. Matches the old numbers, but the generated API then depends on configuration.
  - Borrowed typed messages (above), which also remove the per-field `String` allocations
    (about 15-20% of parse time): the largest gain and the largest API change.
- **Batched disk writes and group commit** (M). The disk store is now the slowest part of
  processing a message: about 3.5 µs per message, against about 1.1 µs for the whole session
  path; with fsync after every message it manages about 120 messages per second. Batch writes
  per connection wake-up and fsync once per batch, while keeping the guarantee that a message
  is durable before it is sent.
- **Disk I/O off the connection task** (M). Store writes run synchronously on the async task;
  with fsync they block the runtime. Move them to a dedicated writer (or `spawn_blocking`),
  ideally together with batching.
- **Bounded memory store** (S). `MemoryStorage` keeps every sent message forever. Cap it (by
  count or age), gap-filling anything evicted.
- **Disk store rotation** (M). The `.body` file grows until a sequence reset; add rotation or
  compaction for long-running sessions.
- **Full-duplex connection I/O** (M). The driver writes a batch before reading again. If both
  ends write very large bursts at once, each can fill the other's socket buffer and wait for it
  to be read. Reading and writing concurrently removes that risk; add a test that reproduces it
  first.
- **Latency** (L, research). A one-at-a-time round trip is about 28 µs, of which Turbojet's own
  processing is only a few µs; the rest is task scheduling and system calls. Worth exploring:
  a current-thread runtime per session, avoiding channel hops, busy-polling, and CPU pinning.
- **Kernel bypass and hardware timestamps** (L, research). Beyond tuning the runtime, the lowest
  latencies come from bypassing the kernel's network stack (OpenOnload or ef_vi on Solarflare
  cards, DPDK) and from network cards that timestamp packets in hardware. It needs a transport
  that isn't a tokio `TcpStream`, so it follows the zero-copy work.
- **Latency histograms** (S). Opt-in timing metrics (for example, time to process each inbound
  message), kept separate because they cost a clock read per message.

## 6. Operations and deployment

- **Per-counterparty configuration** (M). An `Acceptor` applies one configuration to every
  counterparty. Allow per-session heartbeat limits, schedules, TLS requirements and stores.
- **Configuration files and runtime sessions** (M). Sessions are configured in code. Commercial
  engines (and QuickFIX) read session definitions from a file, reload it without a restart, and
  add or remove sessions while running, including acceptors that admit unknown CompIDs from a
  template. Builds on per-counterparty configuration.
- **Management API and console** (L). The operator API works on connected sessions in the
  library, but nothing exposes it remotely, and the gateway only has the offline `seqnums`
  tool. Provide an optional management endpoint (HTTP, or a local socket) that lists sessions
  and their state; starts, stops, logs out and resets them; triggers resends; and browses and
  searches the message log live. A web console on top is what commercial engines sell on. The
  gateway should use it.
- **Holiday calendars** (S). Schedules have no notion of exchange holidays.
- **Throttling** (M). Optional per-session inbound and outbound message-rate limits, and a way
  for applications to apply back-pressure.
- **Connection limits** (S). An acceptor starts a task for every connection, and each may wait
  the whole logon timeout before sending anything; there's no cap on concurrent connections,
  overall or per IP address. Add optional limits, refusing connections beyond them.
- **Reconnect backoff** (S). An initiator retries at a fixed `reconnect_interval`. After a venue
  outage, many initiators then retry in lockstep. Back off exponentially, with jitter and a cap.
- **Inbound message persistence** (M). Only sent messages are stored. An optional audit store of
  received messages would help post-incident analysis beyond what the logs keep.
- **Message log retention** (M). A message log of both directions, kept apart from the resend
  store and from diagnostic logging, with rotation, compression, retention periods and archiving.
  Regulated firms must keep these for years. Pairs with inbound message persistence.
- **Alternative storage backends**. Today there are only `MemoryStorage` and `DiskStorage`. Each
  backend should live behind its own feature (or in its own crate) so its dependencies stay
  optional, record `created_at` so session schedules work, pass a shared conformance suite, and
  be measured with the existing storage benchmarks.
  - **Embedded database** (M). A `SessionStorage` over [redb](https://docs.rs/redb) (or a
    similar embedded store) could be a better default than the hand-rolled files: sequence
    numbers and message bodies update in one transaction, so there's no torn-write recovery, and
    space is reclaimed without the rotation `DiskStorage` needs. Benchmark it against
    `DiskStorage` with and without fsync. If it matches or beats it, make it the recommended
    durable store and drop "Disk store rotation".
  - **SQL databases** (M). PostgreSQL (and SQLite) through `sqlx` or similar, for deployments
    that can't rely on local disk or want session state next to their other data.
  - **Async store interface** (M, prerequisite for networked stores). `SessionLog` is
    synchronous and is called on the connection task before each message is sent, which is
    fine locally but would block the runtime for a network round trip. Networked stores need
    an async trait or a dedicated writer task; do this together with "Disk I/O off the
    connection task".
  - **Exclusive sessions across processes** (S, with networked stores). `DiskStorage` locks a
    session's files while it's connected; a shared database needs the equivalent (an advisory
    lock or an expiring lease) so two gateways can't run the same session at once. This is also
    the basis for hot-standby failover.
- **High availability with replicated state** (L). A hot standby that receives every sequence
  number change and stored message as it happens, and takes over a session on failover with no
  gap and no resend storm. Replicate the session journal to the standby (over TCP, or something
  like Aeron), with the exclusive-session lease deciding which instance is active.
- **TLS certificate reload** (S). Pick up renewed certificates without restarting.
- **TLS certificate revocation** (S). Client and server certificates are checked against their
  CA but not for revocation. Check CRLs (and optionally OCSP), and accept PKCS#12 bundles as well
  as PEM files.
- **Connect-timeout test** (S). Initiator failover on a connect timeout is implemented but
  untested; simulating a host that silently drops packets needs a test harness.
- **More interop tests** (M). Against QuickFIX/J, only what it does on request is tested. Still
  to do:
  - a fault-injecting proxy between the engines (delays, dropped or garbled bytes, a
    counterparty that goes silent). Among other things it would test a TestRequest sent by
    Turbojet, which QuickFIX/J never goes quiet long enough to prompt, and could serve the
    connect-timeout test above;
  - QuickFIX/n as a second peer;
  - sessions over TLS.

## 7. The example gateway

The gateway exists to exercise Turbojet; these matter only if it becomes more than an example.

- **Matching or routing** (L). Orders are acknowledged and booked, but never filled or sent on.
- **Persistent orders** (M). Orders live in memory and are lost on restart; session state
  survives.
- **Allocation checks** (S). Allocation quantities aren't checked against the order quantity.
- **ClOrdID history** (S). After a replace, earlier ClOrdIDs still find the order; strict venues
  reject them.
- **Example client** (S). It doesn't demonstrate replace or status requests.
- **Cancel-on-disconnect and kill switch** (S). Cancel a session's resting orders when it
  disconnects, or on an operator command. Almost every venue offers this, and it would show how
  an application reacts to session events.
- **Pre-trade risk checks** (M). Order size limits, price bands against a reference price,
  credit limits and self-trade prevention, rejected before booking.

## Out of scope for now

Listed so the decisions are explicit; any of these could be built on Turbojet separately.

- **Other encodings**. FIXML, FAST, SBE and FIXP.
- **Routing and translation between counterparties**. A hub that routes messages between
  sessions and maps one counterparty's dialect to another's with rules (like FIX Antenna's
  FIXEdge). Routing fields (section 2) are in scope; the hub is an application.
- **Ready-made venue dictionaries**. Pre-built, certified definitions for particular venues
  (CME iLink, ICE, LSE, Nasdaq). Dictionary support (section 3) makes them straightforward to
  generate, but maintaining and certifying them is a separate effort.
- **Bindings for other languages**. A C API or Python bindings. Turbojet is a Rust library.

## Known caveats

Behaviour that's deliberate or documented, but worth revisiting.

- `SessionHandle::send` after a session has started logging out drops the message (with a
  warning); `send` returning `Ok` means queued, not sent.
- Custom stores that don't record creation times never reset on a session schedule.
- A few helpers are public only because the exported macros call them (`#[doc(hidden)]`); they
  aren't a stable API.
