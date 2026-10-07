# Design

How Turbojet is built and why, what's been measured along the way, and what testing found. The
[README](README.md) says how to use it, [ROADMAP.md](ROADMAP.md) what's still to do, and
[CAVEATS.md](CAVEATS.md) the behaviour worth knowing about. The house style, safety first, then
performance, then developer experience, is in [STYLE.md](STYLE.md).

A standing goal behind the design: the engine's hot path should get as close as possible to zero
copies and zero allocations per message (see [The hot path](#the-hot-path)).

## The session layer

A complete FIX session layer (logon, including credentials and FIX 4.4 NextExpectedMsgSeqNum;
sequencing, gap detection and resend, heartbeats, logout) for FIX 4.2 and later, as a sans-IO state
machine with an injected clock. It has:

- FIXT.1.1 sessions for FIX 5.0 SP2: DefaultApplVerID(1137) configured and negotiated at logon (an
  acceptor may support several versions and echoes the counterparty's), per-message
  ApplVerID(1128) naming any supported version in either direction, CstmApplVerID and ApplExtID
  passed through, rejects naming the version of the message they answer, messages resent in the
  version they were sent in, and dictionary validation chosen by application version;
- every inbound session-level message but the Logon shown to the application
  (`Application::on_admin_message`), so it learns of a counterparty's Reject of one of its
  messages, or a Logout's reason;
- a malformed body field (no `=`, an invalid tag, non-UTF-8 data) answered with a Reject rather
  than discarded, and an unanswered ResendRequest re-sent once and then ended with a Logout;
- a long resend sent in steps of 256 sequence numbers, each written before the next is read from
  the store, so a ResendRequest for everything never holds the whole range in memory; nothing
  new goes out, and what the counterparty sends is read but not processed, until it ends; each
  step gap-fills what it doesn't resend, so a long run of numbers not stored shows progress;
- the application asked about each message before it's resent (`Application::should_resend`),
  gap-filling those it declines, and ResendRequests in chunks of a configured size for
  counterparties that cap them (`SessionConfig::resend_request_chunk`);
- SendingTime accuracy (within 120 s by default), OrigSendingTime on resends, and header field
  order checked on every message, each of which can be turned off;
- messages that arrive ahead of a gap kept until it's filled, then processed in order, and a
  Logout that arrives during one answered at once;
- a counterparty's sequence reset while logged on (a Logon with ResetSeqNumFlag=Y at 1) accepted;
- routing fields (OnBehalfOf and DeliverTo) kept in the header, and session-level rejects routed
  back to where the message came from;
- data fields (RawData, XmlData, the Encoded* fields, a venue's own) carrying any bytes, SOH
  included: decoded by the length their Length field gives, stored and resent intact, checked on
  the way out, and typed as `Vec<u8>`;
- typed dates, times (UTC and with a zone), MonthYear, `char` and multi-value lists, with
  timestamps written at the precision they arrived with (seconds to nanoseconds), and SendingTime
  at a configured one;
- cancel on disconnect, per counterparty: when a logged-on session ends without a Logout (or,
  if chosen, with one either side sent) and the counterparty doesn't log back on within a grace
  period, the application is told to cancel its orders; `on_logout` says how every session ended
  (a Logout from either side, the connection lost, a heartbeat timeout, an error, or our own
  shutdown or schedule). A venue that negotiates cancel on disconnect at logon, in Logon fields of
  its own, is asked through `Application::to_admin`;
- optional inbound and outbound message-rate limits (N per sliding window): sends beyond the
  outbound limit wait in the send queue, and inbound messages beyond it are delayed (input isn't
  read, so TCP slows the counterparty) or answered with a BusinessMessageReject;
- session schedules with holiday calendars, and operator control of sequence numbers;
- sessions identified as QuickFIX identifies them, by SubIDs, LocationIDs and a local qualifier as
  well as CompIDs: an acceptor takes them from each Logon, so one counterparty can run a session
  per desk, each with its own store, and at most `max_sessions_per_counterparty` (16) at once,
  checked as the session binds. The IDs are part of the session's identity at logon only, as in
  QuickFIX/J: what the counterparty sends afterwards isn't checked, and what we send carries the
  session's IDs unless the application set its own. Stores key a session on a suffix of the extra
  fields, empty without them, so existing stores kept their keys. Stamping cost nothing measurable
  on sessions without them (order to ack 771 ns before, 758 ns after).

## Transports and the connection driver

`connection::run` drives a session over any `AsyncRead + AsyncWrite`; `Acceptor` and `Initiator`
add TCP, TLS with optional mutual authentication (certificates given from memory or files, and
replaced while running), initiator failover from a chosen local address, and graceful shutdown
(every session logged out, bounded by the logout timeout). Initiators reconnect with a
configurable backoff (by default 1 s to 60 s, jittered), and acceptors limit connections overall
and per IP address, and give each counterparty its own settings, decided at Logon, and its own
store.

- **Full duplex.** The driver reads while its output waits to be written, so two ends writing to
  each other at once never each wait for the other to read, and it disconnects a counterparty that
  has stopped reading altogether (16 MiB of output waiting for it). The simulator found the write
  deadlock this replaced.
- **Bounded commands.** Application sends wait in a queue of `SessionConfig::send_queue` messages
  (a full one hands the message back, or `send_when_ready` waits for room), logout and operator
  commands in a small one of their own that a full send queue never holds up, and each send
  returns a `Receipt` saying whether the message was stored, with its MsgSeqNum, or dropped and
  why.
- **Spinning.** `run_spinning` (with `Acceptor::accept_spinning` and `Initiator::run_spinning`)
  drives a session on its own thread without waiting, over a `SpinningStream` that reads and
  writes the non-blocking socket on every poll, since tokio's `try_read` answers from readiness
  its reactor caches. TLS isn't supported over it: its handshake would need driving by polling
  too.
- **The message log.** A `MessageLog` is called by the driver, not the session: `inbound` with
  each frame as it's cut from the read buffer, before the session handles it, and `outbound` with
  each frame of the session's output as it's moved to be written (`Driver::stage_output`). Only
  there do the bytes exist as they are on the wire: a decoded `Message` doesn't keep its frame,
  and a session's output isn't final until its store commits, since a failed commit cuts it back
  and messages held behind a resend are added after the resend. So the log sees what goes out in
  wire order, and within one wake-up the replies to a batch of input after the whole batch, as
  they cross the wire. For FIXP, `FixpSession::feed` makes the inbound call itself. The
  allocation stages set a log that does nothing and call it as the driver does, with every count
  unchanged. Measured 2026-10-07 on an M3 under a load average of 2.5 to 4.5, not idle, as
  criterion point estimates over three alternating runs: with no log the round trips are within
  noise of before (one at a time 26.0 to 26.6 µs before, 26.0 to 26.2 µs after); with a log at
  both ends (the "roundtrip tcp, message log" group) one at a time is about 0.3 µs (1%) slower,
  at the edge of the noise, and 1,000 pipelined orders aren't slower.

## Storage

- **Group commit.** The store commits what each batch of work did (one read, one batch of sends,
  one step of a resend) once, before any of it is written, and a store that waits for its device
  hands the commit to the driver, which runs it on a blocking thread. `DiskStorage` buffers until
  then, so a batch costs one write and, with fsync, one fsync: 100 orders in flight over a disk
  store with fsync take 11 ms rather than 1.6 s. Receipts and operator replies wait for the
  commit.
- **At-least-once delivery.** An inbound message counts as received only once the application has
  handled it, and the messages that may have been in flight at a crash (a window of up to 256,
  committed before the first is handed over) are marked as possibly handled when they're resent
  (`Context::maybe_redelivered`).
- **The disk store** keeps its sequence numbers in two checksummed slots, so a write torn by a
  power loss falls back to the record before it, and writes each commit as one journal record and
  one `fsync`. Both built-in stores keep each session's newest messages up to a byte budget
  (gap-filling older ones on a resend): the disk store in segments, deleting the oldest, and the
  memory store a capped number of sessions.
- **SQL storage** (`turbojet-sql`), in SQLite or PostgreSQL through sqlx: one transaction per
  commit, the same byte budget, and a lease per session, taken on opening and renewed by each
  commit, so gateways sharing a database can't run one session at once and one whose lease was
  taken can't commit. Stores can open sessions, commit and read resend steps with futures the
  driver awaits (`SessionStorage::begin_open`, `SessionLog::fetch`, `Job`), so a networked store
  never blocks the runtime; the simulator runs half its seeds over stores that do. The store
  conformance suite is public (`conformance` feature) for other stores to check themselves.
  Sessions are keyed on an `extra` column as well as BeginString and CompIDs; `migrate` adds it to
  a database made before it, PostgreSQL altering the table under a lock and SQLite rebuilding it
  with foreign keys off, so a second gateway migrating at once waits and finds nothing to do.

## Configuration and operations

- **Session files** (`turbojet-config`): an acceptor, its counterparties, initiators and their
  stores from TOML, every value checked on loading and errors naming the section and key, and
  reloaded while running (changes apply from each counterparty's next Logon and each initiator's
  next connection, counterparties no longer listed are logged out, initiators are started and
  stopped as they're added and removed, and a file that doesn't load leaves the one in use).
- **Observability**: structured logging and Prometheus-compatible metrics, with opt-in latency
  histograms (handling each inbound message, store commits, and reading input to its replies
  being ready to write), and an optional `MessageLog` that sees every message's bytes.

## Dictionaries and code generation

Typed messages come from data dictionaries. `turbojet-dictionary` loads and validates the FIX
Trading Community's official FIX Orchestra files, which give groups and enum values their official
names (`PreAllocGrp`, `OrderCancelRequest`) and document messages, fields and values, as well as
dictionaries in the QuickFIX XML format, the one venues ship their specs in. It merges a venue's
QuickFIX-format additions onto either, keeping the official names and documentation where the venue
re-lists something. `turbojet-codegen` turns a dictionary into `fix_enum!`, `fix_group!` and
`fix_message!` invocations (copying the dictionary's documentation into doc comments only on
request), from an application's `build.rs` or as a command for checked-in crates.

The macros convert and write each field through out-of-line helpers, one copy per field type
rather than one per field, which keeps whole versions quick to build: built alone (Apple M3, Rust
1.98.1, 2026-09-28), `turbojet-fix44` (84 messages) takes about 6 s debug and 16 s release, down
from 75 s, and `turbojet-fix42` about 1.4 s and 2.6 s, for about 3% on the session benchmarks.
Parsing straight into the owned form, rather than through the borrowed one, doubled the generated
crates' build time; that was accepted for the faster owned parse.

The Orchestra files for FIX 4.2, 4.4 and FIXT 1.1 are vendored in `dictionaries/orchestra`
(Apache-2.0, © FIX Protocol Limited), with FIX 4.3 and FIX 5.0 SP2 converted from the FIX Unified
Repository by the FIX Trading Community's own stylesheet (`convert-unified.sh`); each version is
generated into its own crate (`turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44`,
`turbojet-fix50sp2`, the last over FIXT 1.1's header), and CI checks they match. A dictionary's
mistakes can be corrected before generating (`Dictionary::make_optional`, `--optional`;
`turbojet-fix42` makes QuoteCancel's NoQuoteEntries optional, as FIX 4.4 did), and enumerated
fields can accept unknown codes (`--lenient-enums`, `fields::Code`). Sessions can also check
inbound messages against a dictionary (feature `validation`, `SessionConfig::with_dictionary`).

## SBE and FIXP

Low-latency venues increasingly use binary encodings rather than tag=value: SBE (Simple Binary
Encoding) for messages, under FIXP (the FIX Performance Session Layer) for the session.

SBE codecs are generated from SBE message schemas as venues publish them (`turbojet-codegen sbe`),
checked against real-logic's own codecs on SBE's example schema and B3's Binary Entrypoint, and
fuzzed. Schemas aren't derived from the FIX dictionaries: generic FIX messages, mostly optional
strings and nested groups, make poor SBE, and no venue speaks such a schema.

FIXP 1.0 sessions (`turbojet::fixp`) run over TCP or TLS, in both roles, with every flow type,
through the FIX connection driver, made generic over the session it runs (the registry and handles
generic over what they send). They're validated against our own client and server and fuzzed
rather than against a venue. A client fails over to backup endpoints, and sessions record the FIX
sessions' metrics.

FIXP sessions are simulated as FIX ones are (`turbojet-sim --fixp`), without a hostile middlebox:
FIXP numbers messages by their place in the stream and relies on TCP, so a frame dropped or
repeated breaks every rule without an engine bug. The simulation found ten bugs, since fixed: a
client on disk couldn't re-establish or reopen its log; messages past a gap could be delivered
twice after a reconnect; messages in flight after our Terminate were taken for a protocol error; a
crash could lose a message or repeat it unmarked (now at least once, with an in-flight marker as
FIX sessions keep); after a store failure a session kept recording, delivering and committing; and
at finalization a session could send after answering FinishedReceiving, or ignore what came before
the answer while terminating.

## Safety and testing

Following [STYLE.md](STYLE.md) (after TigerBeetle's Tiger Style): limits on everything, asserted
invariants, and tests that look for bugs rather than confirm what already works.

- **Lints and assertions.** The workspace lints deny `unsafe`, lossy casts, `unwrap` in library
  code and functions much over 70 lines. Long functions are split into a parent that decides and
  helpers that compute; the few still marked `#[expect(clippy::too_many_lines)]` are a match arm
  per event or option, or a benchmark group. Each module asserts what its readers rely on, paired
  where it can be: a stored message is checked as stored and as read back; BodyLength as computed
  and as written; a store job as it starts and as it ends; a rate-limit window never outgrows its
  limit or allocates; a connection releases only its own registration; a schedule's periods come
  in start order, each shorter than the look-back that finds it; a compiled dictionary's members
  are all known fields, and checking a group always moves on. Further checks go where they state
  something a reader relies on, not to reach a count.
- **CI** on every push: clippy and tests for each feature combination, the 1.89 minimum Rust
  version, `cargo fmt --check`, rustdoc with warnings as errors, a compile check of the
  benchmarks, a check that the generated crates match their dictionaries, and a check that the
  fuzz targets build and pass the inputs that once found bugs.
- **Fuzzing**: cargo-fuzz targets for the codec, message parsing, every generated message type,
  the SBE codecs and the session state machines (`crates/turbojet/fuzz`, `scripts/fuzz.sh`),
  fuzzed nightly in CI. A failing input, once fixed, is kept in `fuzz/regressions/` and replayed on
  every push (`scripts/fuzz-regressions.sh`).
- **Interop with QuickFIX/J** (`turbojet-interop`), with Turbojet as initiator and as acceptor, on
  FIX 4.2, 4.3 and 4.4 and on FIXT.1.1 with FIX 5.0 SP2: logon and logout, reconnection, heartbeats
  and TestRequests, application messages (one with XmlData containing SOH), gap fills and resends in
  each direction, SequenceResets in both modes, MsgSeqNum too low and sessions identified by SubIDs
  and a LocationID, directly and through a proxy that loses, garbles, cuts and delays messages,
  slows the link and silences or stalls either side. They found Heartbeats going out a second late,
  since fixed.
- **QuickFIX's 235 scripted acceptance scenarios** (`turbojet-acceptance`), which cover the FIX
  specification's session test cases, on every build. They found four deviations from the spec's
  test cases, since fixed; 221 pass, and the 14 that fail are listed with their reasons in
  `known_failures.txt`: two deliberate differences (keeping a valid message that follows a
  too-long BodyLength, and no RefTagID for a negative tag) and scripts that rely on QuickFIX's own
  behaviour or dictionaries.
- **Deterministic simulation** (`crates/turbojet-sim`): an initiator and an acceptor run against
  each other in one thread from a seed, over a TCP-like network that splits, delays, stalls, resets
  and black-holes connections (on some seeds behind a middlebox that drops, duplicates, reorders and
  corrupts messages), with process crashes between events and inside store calls, store errors,
  power loss on disk stores with and without sync, logout requests, operators skipping numbers ahead
  and resetting both sides, daily schedules, and on a third of the seeds sessions with SubIDs and a
  LocationID. After every event a checker holds both sides to what their stores recorded, what they
  wrote and what their applications received, and once the faults stop they must settle; bugs
  planted in the simulator show the checker catches what it should. 100 seeds run on every push and
  random ones nightly, and any failure replays from its seed (`scripts/sim.sh`). It found a write
  deadlock in the connection driver, torn sequence-number records in `DiskStorage`, and an
  operator's skip ahead making a counterparty abandon a gap, all since fixed. It exercises outbound
  limits and inbound `Delay`, but not `Reject` (see [CAVEATS.md](CAVEATS.md)).

## Performance

### The hot path

The long-term goal is that a message in steady state, from the read buffer through the session
and application and back out to the socket, is neither copied nor allocated beyond what the
application itself asks for. Where it stands:
- A `Message` keeps all its fields in one buffer with an offset index (two allocations, not one per
  field), and each inbound frame is decoded into one `Message` reused for the connection, so
  decoding doesn't allocate once it has grown. It still copies each frame, once, out of the read
  buffer (see the roadmap).
- Raw group access is zero-copy, and typed messages parse into borrowed forms
  (`NewOrderSingleRef`) whose strings, lists and groups point into the message, so typed parsing
  allocates nothing.
- The session encodes what it sends once, straight into one reused output buffer, and the stores
  keep those bytes. The application's replies go into a list the session keeps, and a typed reply
  is written into a message the session reuses (`FixMessage::write_into`).
- Text fields in generated messages are `CompactString`s, kept inline up to 24 bytes, so the
  strings an acknowledgement copies from its order don't allocate. Longer values, and groups'
  entry lists, still do.

As of 2026-10-03 the engine makes about 1 allocation per order with the memory store and 0.2 with
the disk store (the store's copy or index node), and the example application none.
`tests/allocations.rs` counts them by stage, and the build fails if a count changes.

### Parsing and encoding

Profiling the decode of a FIX 4.2 NewOrderSingle into a reused message (2026-10-05) showed the
per-field searches for SOH and `=` took about 38% of it and parsing tags about 20%; the CheckSum,
summed by a loop the compiler already vectorises, about 2%. Each 64-byte chunk of the body is now
compared whole into a bit mask of its delimiters, a loop with no early exit that the compiler
vectorises without `unsafe` or `std::simd`, and tags are checked and summed in one pass without a
branch per byte: 192 ns to 164 ns on an Apple M3. `memchr2_iter` was slower (249 ns), starting a
new search for each delimiter a few bytes on. What's left is mostly per-message work: the UTF-8
check (about 7%), framing (about 8%) and recording each field.

Encoding had no search to speed up: 39% of it was copying each field, a few bytes at a time.
Fields that lie back to back in the message's buffer are now copied in one go, so an
ExecutionReport encodes in 54 ns against 122 ns. Building a typed message spent about a third of
its time appending digits a character at a time; integers and decimals are now appended up to
three digits at a time from a table, 164 ns to 156 ns for an ExecutionReport.

### Latency

Researched 2026-10-04 on an Apple M3. A one-at-a-time round trip over localhost takes 26.5 µs when
the benchmark's task sends each order through a `SessionHandle` and receives each acknowledgement
back, and 16.3 µs when the initiator's application sends the next order from `on_message`. The
10 µs between them is the hop between tasks each way, waking another thread. Of the 16.3 µs, about
13 µs is the operating system (the same sizes between two threads over blocking sockets: 12.9 µs),
about 3 µs tokio's reactor, and about 1 µs each end's processing. Running everything on a
current-thread runtime helps only by keeping hops on one thread; with each end on its own thread,
it was no faster than the default.

Spinning takes out the reactor: with both ends spinning, the no-hop round trip fell from 16.45 µs
to 10.34 µs; raw sockets with both ends spinning take 6.8 µs. What's left is mostly the kernel.

### Measurement history

The README's benchmark table gives current numbers. When each row was last measured, and what it
was before:

- Decoding into a reused message, building an ExecutionReport and formatting timestamps were
  re-measured 2026-09-30, after the session started encoding what it sends straight into its
  output, decimals and integers were written without `core::fmt`, and each inbound message was
  decoded into one reused for the connection.
- Borrowed typed parsing was measured 2026-10-01, when typed messages gained borrowed forms.
- Owned typed parsing was re-measured 2026-10-03, after owned messages were parsed straight into
  their owned form: 166 ns, 335 ns and 1.04 µs just before, on the same day.
- Session order → ack was re-measured 2026-10-03, after the application's replies were built in
  messages the session reuses: 945 ns and 1.20 µs just before.
- Disk stores were measured 2026-10-02 with group commit: before it, storing 100 messages with
  fsync took 837 ms, and the disk + fsync round trip managed 62 messages a second with 100 in
  flight. With each commit recording the next in-flight window it took 12.2 ms one at a time with
  fsync. They were re-measured 2026-10-03, after a commit took one write and one `fsync` rather
  than one of each file; just before, on the same day: 3.2 µs and 8.0 ms one per commit, 84 ns and
  81 µs at 100 per commit, and round trips of 33.2 µs, 689k msg/s, 8.1 ms and 9.5k msg/s.
- The SQL stores were measured 2026-10-03, PostgreSQL 14 on the same machine over TCP.
- Round trips replying from `on_message` were measured 2026-10-04; through the benchmark's task
  they took 26.5 µs (TCP) and 28.3 µs (TLS) the same day. Spinning was measured the same day,
  16.45 µs without spinning in the same run.
- SBE was measured 2026-10-05 and FIXP 2026-10-06; the FIX round trips in the FIXP run took
  16.2 µs and 0.81M msg/s.
- The allocation counts were taken 2026-09-30; the application's were re-counted 2026-10-03,
  before which the reply list and message cost 3 allocations more, and `String` fields in place
  of `CompactString`s cost 5.
- The other rows are the 2026-09-27 snapshot.
