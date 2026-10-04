# Roadmap

What's left to do on Turbojet and the example gateway, in rough priority order. Each item says
why it matters and roughly how big it is: **S** (a day or less), **M** (a few days), **L** (a week
or more).

A standing goal behind all of it: the engine's hot path should get as close as possible to zero
copies and zero allocations per message (see "Zero-copy, zero-allocation hot path" under
Performance). How the work is done, safety first, then performance, then developer experience,
is in [STYLE.md](STYLE.md).

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
  and public macros for defining your own, including repeating groups: each in an owned form for
  building and keeping, and a borrowed one that parses without allocating;
- a loader for FIX Orchestra and QuickFIX-format dictionaries with venue merging
  (`turbojet-dictionary`), and a code generator that runs from an application's `build.rs` or as
  a command (`turbojet-codegen`);
- acceptor and initiator transports, TLS with optional mutual authentication (certificates
  given from memory or files, and replaced while running), initiator
  failover, and graceful shutdown (every session logged out, bounded by the logout timeout),
  which the gateway does on SIGINT and SIGTERM; initiators reconnect with a configurable backoff
  (by default 1 s to 60 s, jittered), and acceptors limit connections overall and per IP address,
  and give each counterparty its own settings, decided at Logon, and its own store;
- cancel on disconnect, per counterparty: when a logged-on session ends without a Logout (or,
  if chosen, with one either side sent) and the counterparty doesn't log back on within a grace
  period, the application is told to cancel its orders, which the gateway does; `on_logout` says
  how every session ended (a Logout from either side, the connection lost, a heartbeat timeout,
  an error, or our own shutdown or schedule);
- a malformed body field (no `=`, an invalid tag, non-UTF-8 data) answered with a Reject rather
  than discarded, and an unanswered ResendRequest re-sent once and then ended with a Logout;
- a long resend sent in steps of 256 sequence numbers, each written before the next is read from
  the store, so a ResendRequest for everything never holds the whole range in memory; nothing
  new goes out, and what the counterparty sends is read but not processed, until it ends;
- the connection driver reading while its output waits to be written, so two ends writing to
  each other at once never each wait for the other to read, and disconnecting a counterparty that
  has stopped reading altogether (16 MiB of output waiting for it);
- data fields (RawData, XmlData, the Encoded* fields, a venue's own) carrying any bytes, SOH
  included: decoded by the length their Length field gives, stored and resent intact, checked on
  the way out, and typed as `Vec<u8>`;
- typed dates, times (UTC and with a zone), MonthYear, `char` and multi-value lists, with
  timestamps written at the precision they arrived with (seconds to nanoseconds), and SendingTime
  at a configured one;
- memory and disk session storage, session schedules with holiday calendars, and operator control
  of sequence numbers;
  the disk store keeps its sequence numbers in two checksummed slots, so a write torn by a power
  loss falls back to the record before it; both stores keep each session's newest messages up
  to a byte budget (gap-filling older ones on a resend): the disk store in segments, deleting the
  oldest, and the memory store a capped number of sessions;
- session configuration files (`turbojet-config`): an acceptor, its counterparties, initiators and
  their stores from TOML, every value checked on loading and errors naming the section and key,
  and reloaded while running (changes apply from each counterparty's next Logon and each
  initiator's next connection, counterparties no longer listed are logged out, initiators are
  started and stopped as they're added and removed, and a file that doesn't load leaves the one
  in use);
- SQL session storage (`turbojet-sql`) in SQLite or PostgreSQL through sqlx: one transaction per
  commit, the same byte budget, and a lease per session, taken on opening and renewed by each
  commit, so gateways sharing a database can't run one session at once and one whose lease was
  taken can't commit. Stores can open sessions, commit and read resend steps with futures the
  driver awaits (`SessionStorage::begin_open`, `SessionLog::fetch`, `Job`), so a networked store
  never blocks the runtime; the simulator runs half its seeds over stores that do. The store
  conformance suite is public (`conformance` feature) for other stores to check themselves;
- optional inbound and outbound message-rate limits (N per sliding window): sends beyond the
  outbound limit wait in the send queue, and inbound messages beyond it are delayed (input isn't
  read, so TCP slows the counterparty) or answered with a BusinessMessageReject;
- structured logging and Prometheus-compatible metrics, with opt-in latency histograms (handling
  each inbound message, store commits, and reading input to its replies being ready to write);
- criterion benchmarks (about 0.9 µs per order → ack of session processing, and 490k msg/s
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
  and resends in each direction, SequenceResets in both modes and MsgSeqNum too low. They found Heartbeats
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
- deterministic simulation testing (`crates/turbojet-sim`): an initiator and an acceptor run
  against each other in one thread from a seed, over a TCP-like network that splits, delays,
  stalls, resets and black-holes connections (on some seeds behind a middlebox that drops,
  duplicates, reorders and corrupts messages), with process crashes between events and inside
  store calls, store errors, power loss on disk stores with and without sync, logout requests,
  operators skipping numbers ahead and resetting both sides, and daily schedules. After every
  event a checker holds both sides to what their stores recorded, what they wrote and what their
  applications received, and once the faults stop they must settle; bugs planted in the simulator
  show the checker catches what it should. 100 seeds run on every push and random ones nightly,
  and any failure replays from its seed (`scripts/sim.sh`). It found a write deadlock in the
  connection driver, torn sequence-number records in `DiskStorage`, and an operator's skip ahead
  making a counterparty abandon a gap, all since fixed;
- bounded command queues: application sends wait in a queue of `SessionConfig::send_queue`
  messages (a full one hands the message back, or `send_when_ready` waits for room), logout and
  operator commands in a small one of their own that a full send queue never holds up, and each
  send returns a `Receipt` saying whether the message was stored, with its MsgSeqNum, or dropped
  and why;
- group commit: the store commits what each batch of work did (one read, one batch of sends,
  one step of a resend) once, before any of it is written, and a store that waits for its device
  hands the commit to the driver, which runs it on a blocking thread. `DiskStorage` buffers until
  then, so a batch costs one write and, with fsync, one fsync of each file: 100 orders in flight
  over a disk store with fsync take 11 ms rather than 1.6 s. Receipts and operator replies wait
  for the commit;
- at-least-once delivery: an inbound message counts as received only once the application has
  handled it, and the messages that may have been in flight at a crash (a window of up to 256,
  committed before the first is handed over) are marked as possibly handled when they're resent
  (`Context::maybe_redelivered`).

It has been tested against one other FIX engine (QuickFIX/J) and QuickFIX's scripted session
scenarios, but not yet with any venue or real counterparty. Version 0.1.0 of all seven crates was
published on crates.io on 2026-09-29.

## 1. Following up the first release

Nothing left: the crates.io and docs.rs pages are linked and render correctly.

## 2. Protocol completeness

What the session layer covers is under *Where things stand*. A venue that negotiates cancel on
disconnect at logon, in Logon fields of its own, is asked through `Application::to_admin`.

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

## 5. Safety and assurance

Following [STYLE.md](STYLE.md) (after TigerBeetle's Tiger Style): limits on everything, asserted
invariants, and tests that look for bugs rather than confirm what already works. The workspace
lints (no `unsafe`, no lossy casts, no `unwrap` in library code, functions of about 70 lines) and
debug assertions on framing, sequence numbers and the gap queue are in place. The long functions
in the session, the decoder, the connection driver and the code generator are split into a
parent that decides and helpers that compute; the few still marked
`#[expect(clippy::too_many_lines)]` are a match arm per event or option, or a benchmark group.

- **Assertion density** (M). `session`, `codec`, `message` and the stores assert their
  transitions and pairs (2026-10-04: a stored message is checked as stored and as read back, in
  both stores and again by the session; BodyLength is checked as computed and as written; a
  resend never reaches a number not yet sent; a store never reuses a number), about one assertion
  per four functions, up from one per seven. The rest of the engine has fewer: the registry,
  the connection driver, throttling, schedules and validation. Add checks where they state
  something a reader relies on, not to reach a count.

## 6. Performance

### Zero-copy, zero-allocation hot path

The long-term goal is that a message in steady state, from the read buffer through the session and
application and back out to the socket, is neither copied nor allocated beyond what the application
itself asks for. Some of this is already done: a `Message` keeps all its fields in one buffer with
an offset index (two allocations, not one per field), outgoing messages are encoded into one reused
batch buffer, raw group access is zero-copy, and typed messages parse into borrowed forms
(`NewOrderSingleRef`) whose strings, lists and groups point into the message, so typed parsing
allocates nothing. Allocations per order → ack are counted by stage (`tests/allocations.rs`), and
the build fails if a count changes, so each step below shows up as a lower budget. Each inbound
frame is decoded into one `Message` reused for the connection, so decoding doesn't allocate once it
has grown. The session encodes what it sends once, straight into one reused output buffer, and the
stores keep those bytes. The application's replies go into a list the session keeps, and a typed
reply is written into a message the session reuses (`FixMessage::write_into`). As of 2026-10-03 the
engine makes about 1 allocation per order with the memory store and 0.2 with the disk store (the
store's copy or index node), and the example application none: typed parsing allocates nothing,
and the strings its acknowledgement copies into an owned ExecutionReport are `CompactString`s,
kept inline up to 24 bytes. Longer values, and groups' entry lists, still allocate. What remains,
per message:
- **Inbound without the copy** (M). Decoding still copies each frame, once, out of the read
  buffer into the reused message. A view borrowing the read buffer would avoid that, at the cost
  of a second message type through `Fields`, the typed-message macros and the generated crates;
  worth it only if a benchmark shows the copy matters.

This interacts with "Latency" below.

### Measured bottlenecks

From the benchmarks.

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
- **Latency** (researched 2026-10-04). A one-at-a-time round trip over localhost on an Apple M3
  takes 26.5 µs when the benchmark's task sends each order through a `SessionHandle` and receives
  each acknowledgement back, and 16.3 µs when the initiator's application sends the next order from
  `on_message`. The 10 µs between them is the hop between tasks each way, waking another thread.
  Of the 16.3 µs, about 13 µs is the operating system (the same sizes between two threads over
  blocking sockets: 12.9 µs), about 3 µs tokio's reactor, and about 1 µs each end's processing.
  Running everything on a current-thread runtime helps only by keeping hops on one thread; with
  each end on its own thread, it was no faster than the default. Turbojet already runs on one, so
  there is nothing to build; the README says to reply from `on_message`. Busy-polling and CPU
  pinning moved to the next item.
- **Kernel bypass, busy-polling and hardware timestamps** (L, research). What's left is mostly the
  kernel. Busy-polling the socket halves the operating system's share (12.9 µs to 6.8 µs with both
  ends of a plain socket spinning), but tokio can't do it: its `try_read` answers from readiness
  cached by the reactor, so it reports nothing to read until the reactor runs, and a spinning read
  needs the raw socket. Keeping a tokio runtime awake with a `yield_now` loop made the round trip
  slower, and spinning on the acceptor's raw socket alone gained nothing through Turbojet. Pinning
  can't be measured on macOS, which has no thread affinity API. Beyond that, the lowest latencies
  come from bypassing the kernel's network stack (OpenOnload or ef_vi on Solarflare cards, DPDK)
  and from network cards that timestamp packets in hardware. All of these need a transport that
  isn't a tokio `TcpStream`, and a Linux machine with isolated cores to measure them, so they
  follow the zero-copy work.
- **Aeron and SBE** (L, research). Low-latency venues and in-house systems increasingly use binary
  encodings over messaging rather than tag=value over TCP: SBE (Simple Binary Encoding, the FIX
  Trading Community's binary standard) for messages, under FIXP for the session, and Aeron (reliable
  UDP unicast and multicast, and shared memory between processes on one host) as the transport.
  Supporting them means a second encoding beside tag=value, with SBE schemas derived from the same
  dictionaries the generated crates come from, a FIXP session layer beside the FIX one, and a
  transport that isn't a byte stream. Start with SBE encoding and decoding of the generated
  messages, measured against tag=value; then FIXP; then Aeron. Aeron's Rust clients mostly wrap its
  C library, through `unsafe` code the workspace lints forbid in Turbojet's own crates, so it would
  live in a crate of its own, as `turbojet-sql` does. Also a candidate transport for replicating
  session state to a standby (section 7, high availability).

### Ideas not yet measured

Each needs a benchmark that shows the cost before the change is worth its complexity.

- **SIMD parsing** (M). Decoding a FIX 4.2 NewOrderSingle into a reused message takes about 186 ns
  (Apple M3, 2026-10-04). The scans in it are byte at a time: finding each field's SOH and `=`,
  summing the CheckSum, and checking that values are UTF-8 (which the standard library already does
  quickly). Scanning 16 or 32 bytes at once (SSE2/AVX2 on x86, NEON on Arm) could find every
  delimiter in a frame in one pass and build the field index from the result. The workspace forbids
  `unsafe`, so through a safe crate (`memchr` uses SIMD inside) or portable SIMD once `std::simd` is
  stable. Profile the decode first to see what share these scans take.
- **Cache-aligned data** (S each, research). Data shared between threads, such as the session
  registry, the command queues, metrics counters and `MemoryStorage`'s per-session state, can
  share a cache line with unrelated data that another core writes (false sharing), and a
  session's per-message fields are spread across a large struct. Pad what's contended to a
  cache line (`#[repr(align(64))]`, or 128 bytes on Apple silicon) and group the fields the hot
  path touches. It needs a benchmark with many sessions on several cores, which doesn't exist
  yet: the round-trip benchmarks run one session.

## 7. Operations and deployment

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
- **Inbound message persistence** (M). Only sent messages are stored. An optional audit store of
  received messages would help post-incident analysis beyond what the logs keep.
- **Message log retention** (M). A message log of both directions, kept apart from the resend
  store and from diagnostic logging, with rotation, compression, retention periods and archiving.
  Regulated firms must keep these for years. Pairs with inbound message persistence.
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
  gap and no resend storm. Replicate the session journal to the standby (over TCP, or something
  like Aeron), with the exclusive-session lease deciding which instance is active.
- **TLS certificate revocation** (S). Client and server certificates are checked against their
  CA but not for revocation. Check CRLs (and optionally OCSP), and accept PKCS#12 bundles as well
  as PEM files.
- **More interop tests** (M). QuickFIX/J is tested on request and through a fault-injecting
  proxy (lost, garbled, cut and delayed messages, a silent counterparty, a slow link and a
  stalled reader). Still to do:
  - QuickFIX/n as a second peer;
  - sessions over TLS.

## 8. The example gateway

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

- **Other encodings**. FIXML and FAST. (SBE and FIXP are in section 6, under "Aeron and SBE".)
- **Routing and translation between counterparties**. A hub that routes messages between
  sessions and maps one counterparty's dialect to another's with rules (like FIX Antenna's
  FIXEdge). Routing fields (section 2) are in scope; the hub is an application.
- **Ready-made venue dictionaries**. Pre-built, certified definitions for particular venues
  (CME iLink, ICE, LSE, Nasdaq). Dictionary support (section 3) makes them straightforward to
  generate, but maintaining and certifying them is a separate effort.
- **Bindings for other languages**. A C API or Python bindings. Turbojet is a Rust library.

## Known caveats

Behaviour that's deliberate or documented, but worth revisiting.

- `SessionHandle::send` returning `Ok` means queued, not sent: its `Receipt` says whether the
  message was stored (with its MsgSeqNum) or dropped, and why (the connection ending first,
  logging out, a store failure).
- Custom stores that don't record creation times never reset on a session schedule.
- Cancel-on-disconnect countdowns live in memory: a process that stops without shutting down
  loses those under way, so after a restart the application checks the orders it kept. One that
  shuts down fires them at once, as does replacing an initiator during a grace period.
- A `SqlStorage` lease must outlast the longest heartbeat interval, since heartbeats are the
  commits that renew it while a session is idle, and gateways sharing a database need clocks in
  step to well within the lease. A log closed without a tokio runtime to give its lease up on
  leaves it to expire.
- An operator change to a disconnected session runs a store's blocking commit (`DiskStorage`
  with fsync) on the calling task, as it always has: rare, but it holds a runtime worker for the
  fsync.
- A weekly schedule honours only a holiday on its start day: a mid-week holiday doesn't close it.
  Holidays don't shorten a day either (no early closes).
- A schedule time skipped by a clock change of three hours or more (Samoa skipping 2011-12-30) is
  resolved with the wrong offset, giving that day a zero-length or inverted period. Changes of an
  hour or so, as in daylight saving, are handled.
- Replies an application makes in `on_message` go out at once but count against
  `outbound_limit`, so they can take a window past its N; queued sends then wait longer, and
  while replies alone fill the window, queued sends wait until they slow.
- With an `outbound_limit`, `SessionHandle::logout` waits for the sends queued before it, which
  leave at the limit's rate: a long queue under a slow limit delays the Logout. A shutdown, or a
  logout the session starts itself, drops them instead.
- Under an inbound `Reject` limit, the BusinessMessageRejects the session sends don't count
  against its own `outbound_limit`, so a flood is answered at the counterparty's rate.
- Each connection starts with empty windows, so a counterparty that reconnects starts afresh.
  Acceptor connection limits and the cost of a logon bound this.
- An inbound `Delay` limit holds admin messages too, in order behind held input: a counterparty
  that has died, or its Logout or ResendRequest, may be noticed up to a window late.
- An inbound `Reject` limit never rejects recovery we asked for, and a counterparty controls its
  own gaps, so a hostile one can push one gap's worth of messages through. `Delay` doesn't have
  this hole.
- The simulator exercises outbound limits and inbound `Delay`, but not `Reject`: its checker
  can't cheaply model a BusinessMessageReject in place of a delivery. Session tests cover it.
- A `Message` panics if it grows past 4 GiB: its field index holds 32-bit offsets. Inbound
  messages are far below that (BodyLength is capped at 64 KiB), so only an application building
  a huge outbound message can reach it.
- A few helpers are public only because the exported macros call them (`#[doc(hidden)]`); they
  aren't a stable API.
