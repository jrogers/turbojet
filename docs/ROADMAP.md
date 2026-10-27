# Roadmap

What's left to do on Turbojet, in rough priority order. Each item says why it matters and roughly
how big it is: **S** (a day or less), **M** (a few days), **L** (a week or more). What's built, and
how, is in [DESIGN.md](DESIGN.md); behaviour worth knowing about is in [CAVEATS.md](CAVEATS.md). A
gateway's own work (routing, risk, high availability and the rest) is on [Turbojet
Gateway](https://github.com/jrogers/turbojet-gateway)'s roadmap.

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
  vendored as a feature. Whole, it builds in 98 s debug and 207 s release, peaking at 9 GB
  (clean, Apple M3, 2026-10-17), against 25 s and 48 s for FIX 5.0 SP2; the workspace builds
  every version, and all eight already take 173 s and 10 GB in release, which CI's runners
  strain. An application can generate just the messages it uses meanwhile
  (`turbojet-codegen --message`). QuickFIX has no acceptance scripts for it.
- **More of FIX Orchestra** (M). Loading covers what the standard FIX 4.2 and 4.4 files use.
  Still to do:
  - scenarios (per-workflow variants of a message) and `constant` and `forbidden` presence,
    which the loader rejects with an error for now;
  - venue overlays in Orchestra format (`merge_xml` takes only QuickFIX format).

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
  2,768, mostly empty `Option`s. That's about half of a typed parse: declaring only the 10
  fields a FIX 4.2 order carries takes its parse from 100.5 ns to 59.5 ns borrowed and 124.0 ns
  to 62.8 ns owned (Apple M3, 2026-10-26; see DESIGN.md). Each option changes what's generated;
  cheapest first:
  - A codegen option to generate only chosen fields per message, opt-in, as `--message` chooses
    messages: the whole gain for those who use it, with the default crate unchanged. Strict
    parsing should then skip a dictionary field left out rather than reject it as undefined.
  - Move rarely used fields into a sub-struct per message: boxed in the owned form, parsed on
    access in the borrowed one, which can't allocate. Changes the default API, needs a rule, or
    per-venue configuration, for which fields are common, and wins only part of the gain.
  - A borrowed view that keeps the message and a table from tag to field, and parses each field
    as it's read: about one pass over the tags to parse, but the largest change.
- **Sending on the caller's thread** (M, research). `SessionHandle::send` queues each message
  for the connection's task, and the hop to it is most of what separates a round trip sent one
  at a time through a handle (31.2 µs over localhost TCP, M4 Pro) from one replying from
  `on_message`, already on that task (11.2 µs; README, DESIGN.md Latency).
  QuickFIX numbers, stores and encodes on the sender's thread under the session's lock, and in
  its threaded mode writes there too. A handle could do the same when the connection is idle:
  take a lock on the session, encode, commit and write without blocking, and fall back to the
  queue when the connection is busy or the write would block. The reading side would then take
  that lock too, and a commit that waits for the device (fsync) would hold up the sender. Wants a
  prototype measured one at a time and with 1,000 in flight before the sans-IO session is shared.
- **Generated code's build time** (S, research). Release builds of the generated versions take
  48 s for FIX 5.0 SP2 alone, 173 s and 10 GB for all eight, and 207 s for FIX Latest (clean, M3,
  2026-10-17), after the fixes in DESIGN.md. No single function dominates an LLVM time trace any
  more. Table-driven enums were tried and made no difference in release while costing 18% in
  debug. `opt-level = 1` saves 13% and costs 6-16% in parse speed, so it isn't worth it. About
  20 s of FIX 5.0 SP2's build is the front end (macro expansion and type checking take 5 s each),
  which no optimisation level touches: shrinking what the macros expand to is what's left.
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
- **Padding per-session data** (S). Padding what each session writes from another core
  (`CommandSender`'s queued count, `LiveStatus`, `MemoryStorage`'s per-session state with its
  lock inline) to 128 bytes gained 2 to 4% from 4 sessions up on an M4 Pro (DESIGN.md, Many
  sessions). Small, but it costs a few hundred bytes a session; it wants a Linux machine with
  64-byte lines measured too, and a padding type sized per target.

## 5. Operations and deployment

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
- **Replication, next steps** (`turbojet-replica`; S to M each). Still to do:
  - batching across sessions (S, measured, parked). Many sessions committing at once over one
    link cost about 2 µs a commit in all with memory stores, sync or async, however many share
    it: some 450,000 a second, which writing every queued frame in one write, and the standby's
    answers likewise, would raise. With fsync, replicated commits cost one sync on each side in
    turn, and the store alone already caps them at a few hundred a second (DESIGN.md), so a group
    commit on the standby would at best halve that. See "Group commit in `DiskStorage`" below.
  - an application's own records (the gateway's book) committed with a session's batch, which the
    wire form has room for.
- **Group commit in `DiskStorage`** (M to L, research). Each session's commit syncs its own
  file, and concurrent syncs barely overlap on macOS (`F_FULLFSYNC` flushes the whole drive
  cache each time): 64 sessions committing at once take 3.4 ms a commit in all (DESIGN.md).
  Sessions committing together could share one flush, replicated or not, the standby included.
  On macOS that's a plain `fsync` per file and one `F_FULLFSYNC` a group, which std can't express
  (its `sync_data` is the full flush; `rustix` can). On Linux there's no flush covering other
  files, and the filesystem's journal already merges concurrent `fdatasync`s, so a group commit
  there needs every session's records in one shared file per store: a new format and recovery.
  Wants measuring on Linux first, where gateways run.
- **More TLS revocation sources** (S each). Revocation is checked against CRLs the application
  gives; OCSP (stapled, or fetched) and fetching CRLs from a certificate's distribution points
  aren't supported.

## 6. Other languages

- **Python bindings, next steps** (S to M each). `crates/turbojet-python` runs initiators from
  asyncio (see DESIGN.md). Still to do:
  - wheels for Linux, macOS and Windows built in CI, if it's to be published to PyPI;
  - several events taken at once, under one hold of the GIL, for loops that work in batches;
  - logging into Python's `logging`, once `pyo3-log` supports PyO3 0.30;
  - acceptors, FIXP sessions, and typed message classes generated from the dictionaries, if
    they're wanted.

## 7. Usability and outreach

- **Announcement posts** (S). Posts introducing Turbojet where Rust and trading-systems
  developers read, pointing to the website and guide.
