# Design

How Turbojet is built and why, what's been measured along the way, and what testing found.
[ARCHITECTURE.md](ARCHITECTURE.md) is the overview, with diagrams, [the guide](guide/README.md)
says how to use it, and [CAVEATS.md](CAVEATS.md) the behaviour worth knowing about. The house style, safety first, then
performance, then developer experience, is in [STYLE.md](STYLE.md).

A standing goal behind the design: the engine's hot path should get as close as possible to zero
copies and zero allocations per message (see [The hot path](#the-hot-path)).

## The session layer

A complete FIX session layer (logon, including credentials and FIX 4.4 NextExpectedMsgSeqNum;
sequencing, gap detection and resend, heartbeats, logout) for FIX 4.0 and later, as a sans-IO state
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
- a counterparty's sequence reset while logged on (a Logon with ResetSeqNumFlag=Y at 1) accepted,
  except one arriving before anything else from it since the numbers started again with a reset
  Logon from each side: that's a duplicate, or the reply to ours, and answering it would set the two
  sides resetting each other;
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
- session schedules with holiday calendars, early closes included: an early close cuts the daily
  period it falls inside short where periods are generated, so the session's check that its
  period has ended needs no change, and closes a weekly period until midnight as a mid-week
  holiday does; and operator control of sequence numbers;
- sessions identified as QuickFIX identifies them, by SubIDs, LocationIDs and a local qualifier as
  well as CompIDs: an acceptor takes them from each Logon, so one counterparty can run a session
  per desk, each with its own store, and at most `max_sessions_per_counterparty` (16) at once,
  checked as the session binds. The IDs are part of the session's identity at logon only, as in
  QuickFIX/J: what the counterparty sends afterwards isn't checked, and what we send carries the
  session's IDs unless the application set its own. Stores key a session on a suffix of the extra
  fields, empty without them, so existing stores kept their keys. Stamping cost nothing measurable
  on sessions without them (order to ack 771 ns before, 758 ns after). Settings and stores are
  chosen by CompID, or for a session listed whole (`with_session`, a sessions file's section
  with SubIDs or LocationIDs) by its exact ID first. The sessions file refuses an initiator only
  for a session a listed section would serve: a CompID's section serves any SubIDs, since a
  counterparty may log on with any, so only sessions listed whole free others.

## Transports and the connection driver

`connection::run` drives a session over any `AsyncRead + AsyncWrite`; `Acceptor` and `Initiator`
add TCP, TLS with optional mutual authentication (certificates given from memory or files, as
PEM or PKCS#12 bundles, and replaced while running) and optional revocation checks against CRLs the application gives
(rustls checks them; with CRLs, a certificate none covers is refused, and a CRL past its next
update is still used, so a late refresh doesn't lock every counterparty out, but logged as a
warning when it's put in use and at each handshake it covers; `Trust::refuse_expired_crls` fails
closed instead), initiator failover from a chosen local address and through an HTTP or SOCKS5 proxy, and graceful shutdown
(every session logged out, bounded by the logout timeout). Initiators reconnect with a
configurable backoff (by default 1 s to 60 s, jittered), and acceptors limit connections overall
and per IP address, accept them only from allowed addresses and ranges, and give each counterparty its own settings, decided at Logon, and its own
store.

PKCS#12 bundles are decoded by `p12-keystore`: ring, which rustls uses here, has no AES-CBC, 3DES
or RC2 to decrypt them. It reads the AES form (OpenSSL 3's default) and the legacy RC2 and 3DES
one that older OpenSSL, Windows and Java tools write, and its one key and chain go through the
same check as PEM (`Identity::from_der`). A bundle with several keys is refused rather than one
chosen by alias, since nothing in a sessions file would name it. It's a feature of its own,
`pkcs12`, so PEM users don't build its 42 crates.

- **Full duplex.** The driver reads while its output waits to be written, so two ends writing to
  each other at once never each wait for the other to read, and it disconnects a counterparty that
  has stopped reading altogether (16 MiB of output waiting for it). The simulator found the write
  deadlock this replaced.
- **Bounded commands.** Application sends wait in a queue of `SessionConfig::max_queued_sends` messages
  (a full one hands the message back, or `send_when_ready` waits for room), logout and operator
  commands in a small one of their own that a full send queue never holds up, and each send
  returns a `Receipt` saying whether the message was stored, with its MsgSeqNum, or dropped and
  why.
- **Refusing on accept.** The connection limits and the allowed addresses are checked together
  as each connection is accepted, in the accept loop the FIX and FIXP acceptors share, so one
  refused costs an `accept` and a `close`: no TLS handshake, no task. The list sits behind a
  read lock in the acceptor's limits, so a reload replaces it for the next connection. A
  connection from a listed proxy can't be checked until its PROXY header has been read, on its
  own task: it holds a place in the total meanwhile, and its header has a few seconds to come,
  then its place moves to the client's address and is checked as any other. Only listed proxies
  are read from, so a client can't name another's address. The header is read without reading
  past it (six bytes, then a byte at a time to a version 1 header's CRLF, or a version 2
  header's stated length, at most 536 bytes), so the connection's first bytes stay for the
  session or the TLS handshake; it's a few dozen lines here rather than a crate.
- **Proxies.** An initiator with a proxy connects to it, from its local address if set, asks for
  a tunnel to the endpoint by name (HTTP `CONNECT`, or SOCKS5 with a domain-name address, so the
  proxy resolves it), and carries on with the same `TcpStream`, so TLS, failover, reconnecting
  and spinning work as they do directly. The HTTP reply is read a byte at a time up to its blank
  line, so nothing of the tunnel's is consumed. The handshakes are written here rather than taken
  from a crate: each is a few dozen lines.
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
  they cross the wire. Input the codec ignores as garbled goes to `garbled` if it framed as a
  message (BeginString, BodyLength, and CheckSum where BodyLength puts it), as QuickFIX writes
  whatever it frames to its incoming log before checking it; the codec checks CheckSum and the
  MsgType's place while framing, so without this a message failing either left no trace.
  `codec::framed_len` re-reads only the framing, on that rare path, so the codec's results stay
  as they were. For FIXP, `FixpSession::feed` makes the inbound call itself. An acceptor's Logon
  is logged just after the session handles it rather than before: only then is the session
  known, and with it the log a `Counterparty` may set, so the Logon goes there with its ID; the
  reply is staged later, so the log's order is still the wire's. Before binding, and for a
  refused Logon, `Session::message_log_id` is the session the Logon named. The
  allocation stages set a log that does nothing and call it as the driver does, with every count
  unchanged. Measured 2026-10-07 on an M3 under a load average of 2.5 to 4.5, not idle, as
  criterion point estimates over three alternating runs: with no log the round trips are within
  noise of before (one at a time 26.0 to 26.6 µs before, 26.0 to 26.2 µs after); with a log at
  both ends (the "roundtrip tcp, message log" group) one at a time is about 0.3 µs (1%) slower,
  at the edge of the noise, and 1,000 pipelined orders aren't slower.
- **Message log files.** `FileMessageLog` keeps the calls off the disk: each appends a record
  (the time, direction, length and session as a text line, then the frame) to a buffer under a
  mutex, and a thread of its own swaps that buffer for an empty one and writes it. Swapping keeps
  both buffers' capacity, so a warm log allocates nothing per message. A record's time is read as
  it's appended, so it's the time the driver saw the message. The writer waits on a condition
  variable when there's nothing to write, and an append wakes it only if it's waiting; after
  waking, it sleeps a millisecond (`GATHER_INTERVAL`) to gather a batch. So a busy log costs the
  session's task no system call per message, only about one wake-up per millisecond per log.
  The buffer is bounded (`max_buffer_bytes`), and over it messages are counted and dropped rather
  than block the session, with a `dropped` line written in their place; a failed write counts
  its batch the same way, and the next write starts a new file. Files are rotated by UTC day and
  size between batches, so a file can pass `max_file_bytes` by one batch. A file is never
  appended to after reopening: the log starts the next number, so a file that a crash cut short
  stays as it was. Retention deletes only files whose names the log would have made, by the day
  in the name, so a file's last record is past retention before the file goes. Passwords and
  credentials are masked on the writer thread, over each batch before it's written, so sessions
  don't pay for it: the scan stops at MsgType unless it's a Logon or UserRequest (21 ns for a
  192-byte ExecutionReport, 111 ns for a Logon, 2026-10-09), and overwrites values in place with
  `*`s, so lengths and the file's records are unchanged. No `fsync`: the
  resend store is what must be durable, and the log's records survive a crash of the process.
- **`turbojet-log`.** A command over `FileMessageLog::files` and `read`, not a library: a log
  written by any application is read the same way. Field names come from the four vendored
  Orchestra files, embedded (symlinked into the crate so it publishes) and each parsed only when a
  message of its BeginString first needs it: printing a log that uses two versions takes about
  20 ms in all (Apple M3, release build, 2026-10-09). A FIXT session is named from FIX 5.0 SP2
  whatever its ApplVerID, as the field names rarely differ; `--dictionary` replaces them all.
- **Replaying the log.** `turbojet::replay` drives a `Session::acceptor` from a log's inbound
  records as `turbojet-acceptance` drives one from a script: `commit_blocking` after each call, any
  resend stepped through with `on_resume`, and the output discarded. Time is virtual: the
  `Instant`s and the config's `Clock` move together from the first record's time (never backwards,
  if the logging host's clock was stepped back), and between records `on_timer` runs at each
  deadline and at least once a second, as a driver calls it, so a silent counterparty times out as
  it did. The log has no connection records, so an inbound Logon ends the open connection and
  starts another. Before each, the in-memory store is reset and set to expect the Logon's
  MsgSeqNum and send from its NextExpectedMsgSeqNum(789), if any: without that, a log starting
  mid-day would open with a gap and hold every message behind it. The Logon is logged before its
  session is known, so it's matched by BeginString and CompIDs; a Logon that then identifies a
  different session (SubIDs in it that the given `SessionId` lacks, say) ends the replay with an
  error rather than replay into the wrong store.
  Compressing finished files (gzip, through flate2's pure-Rust backend) and calling the
  `on_finished` hook happen on a second thread, fed by a channel as the writer starts each new
  file and when the log is dropped, so the writer never waits for them: a full 256 MiB file of
  synthetic ExecutionReports took about 1.0 s to compress, to 19 MiB (13 times smaller), on a
  loaded M-series Mac, 2026-10-08. A file is compressed to `.log.gz.partial` and renamed, then
  the original deleted, so a crash leaves either the original or both. Opening with `compress`
  finishes every uncompressed file there (all are finished, since opening starts a new one) and
  deletes partial ones; a file a crash left both ways is compressed again over the old copy.

  Measured 2026-10-07 on an M3, under a load average of about 2.5 (not idle), criterion medians
  with a log at both ends against the no-op log: one order at a time 27.5 µs against 26.5 µs
  (26.8 µs with no log), replying from `on_message` 19.3 µs against 16.3 µs, and 1,000 pipelined
  orders 1.39 ms against 1.11 ms, about 70 ns per record. One append on its own is about 128 ns,
  36 of it reading the clock. The first version woke the writer for every message and formatted
  the time with chrono's `format`: 33.1 µs, 22.8 µs and 2.07 ms. Writing nothing to disk, or
  waking the writer every 10 ms rather than every 1, changes neither number, so what remains is
  the append on the session's task.

## Storage

- **Backends.** `MemoryStorage`, `DiskStorage` and `turbojet-sql`'s `SqlStorage`. A new one lives
  behind its own feature (or in its own crate) so its dependencies stay optional, records
  `created_at` so session schedules work, passes the conformance suite
  (`turbojet::store::conformance`), and is measured against `DiskStorage`.
- **Group commit.** The store commits what each batch of work did (one read, one batch of sends,
  one step of a resend) once, before any of it is written, and a store that waits for its device
  hands the commit to the driver, which runs it on a blocking thread. `DiskStorage` buffers until
  then, so a batch costs one write and, with fsync, one fsync: 100 orders in flight over a disk
  store with fsync take 11 ms rather than 1.6 s. Receipts and operator replies wait for the
  commit.
- **Syncing on macOS.** There `fsync` only hands a file's writes to the drive, which may keep
  them in its cache through a power loss; `F_FULLFSYNC` (std's `sync_data`) also asks the drive
  to flush its cache, all of it ("flush all buffered data to the permanent storage device", the
  `fcntl` man page), and takes about 4 ms on an Apple M3 whoever issues it, concurrent ones
  barely overlapping. So a `DiskStorage` commit there `fsync`s its own file, then waits for a
  full flush that started after its `fsync` returned, issuing it on its own file if none is
  running (`store/drive_flush.rs`): one flush covers every commit waiting, whichever file it's
  issued on. Commits are grouped by drive (device number), so a flush never stands in for
  another drive's. Elsewhere a commit `fdatasync`s its file, since Linux's journal already
  merges concurrent ones. The calls are rustix's, a dependency on Apple targets only, so
  Turbojet has no `unsafe`. A reset's write, a new segment's directory and the creation and
  marks files, written rarely, still sync alone.
- **At-least-once delivery.** An inbound message counts as received only once the application has
  handled it, and the messages that may have been in flight at a crash (a window of up to 256,
  committed before the first is handed over) are marked as possibly handled when they're resent
  (`Context::maybe_redelivered`).
- **Holding messages in flight.** The store's in-flight marker is the first incoming message that
  may not be finished: the next incoming number, or an earlier message the application holds
  (`Context::hold`), whose number has been recorded though it isn't finished. A session bound with
  a marker below its next incoming number moves the number back to it, so the counterparty is asked
  again from there, and the window marks everything from it. Without holds the marker equals the
  number at every point a crash can see, so nothing changes; recording the number and marker
  together (`StoredSession::set_next_incoming_in_flight`) keeps a store that writes each change as
  it's made from stopping between them, which the simulator found. Each window starts at the
  marker, so a session hands over at most 256 past the oldest hold; past that, input waits until a
  release. The driver reads on meanwhile, into its buffer and unfed, up to 1 MiB, so a
  counterparty that closes is noticed (the connection ends at once: what's buffered wasn't
  counted received, so it's resent) and one that goes quiet is sent TestRequests and
  disconnected; each read tells the session it heard from the counterparty
  (`on_read_while_held`), which also answers a TestRequest whose real answer waits unread. Past
  1 MiB, reading stops and silence isn't counted (`watch_while_held(false)`), as it wasn't at all
  before: a counterparty closing or vanishing then wasn't noticed until a write failed. The session keeps a weak
  reference per held message, sorted by MsgSeqNum, and the oldest live one is the marker at each
  commit. A release wakes the connection with `Command::HoldReleased` on the session's own control
  queue, at most one waiting at a time (a flag the session clears as it looks), so releases never
  crowd out logouts or operator commands. A session that ends with messages held hands their
  weak references, the marker it left and its next incoming number to what its holds share; a
  release after that works out where the marker belongs (the oldest still held, or none) and
  one task at a time moves it there through the registry, as an operator's change to a
  disconnected session is made, only if the store still has the marker the session left: a
  connection since has asked from it already, and a failed last commit left another. Releases
  while connected check one atomic flag for this and take no lock. The simulator runs no tokio
  runtime, so there a release after its connection does nothing, as before; session tests cover
  it. Nothing is allocated until
  the first hold, so the allocation budget is unchanged.
  The session clears the flag with a swap, not a store, so the relaxed loads of the holds' counts
  that follow can't run before it (on ARM64 a store can wait in a buffer) and lose a wake-up.
  A session waiting for a release warns once per connection and sets `turbojet_held_back`; replays
  give holds that do nothing, since nothing in them would release one. Measured 2026-10-10 on an
  M3, not idle (load average about 2.5), with criterion `--quick`: order to ack with the memory
  store 785 ns, with each order held until the next 836 ns, so about 50 ns per held message (an
  `Arc` taken and released, and one control message). Sessions that never hold measured 785 ns
  before and 789 ns after (memory store), 2.86 and 2.90 µs (disk store); neither difference was
  significant (p 0.10 and 0.08), so take them as unchanged within the noise of a loaded machine.
  The simulator's acceptor holds orders on half its seeds.
- **The disk store** keeps its sequence numbers in two checksummed slots, so a write torn by a
  power loss falls back to the record before it, writes each commit as one journal record and one
  `fsync`, and reads a resend step's messages with one read of each segment they're in. Both
  built-in stores keep each session's newest messages up to a byte budget (gap-filling older ones
  on a resend): the disk store in segments, deleting the oldest, and the memory store a capped
  number of sessions.
- **The stores' indexes.** Both built-in stores find the messages they keep by MsgSeqNum in ring
  buffers (`VecDeque`) in sequence order, not a `BTreeMap`: blocks of 4,096 entries, each
  allocated once at its full size, in a ring buffer of blocks. The numbers stored only increase,
  so a message goes on the back and eviction takes from the front, and a resend finds its range by
  binary search over the blocks and then within one. A `BTreeMap` allocated a node every six messages or so, and its insert was most
  of a memory store's: storing a message took 48 ns, now 14 ns (Apple M3, 2026-10-25, criterion
  medians of two interleaved rounds each), and 70 ns to 40 ns in the disk store at 100 per
  commit; an order → ack through the session with the memory store 813 ns to 784 ns. In blocks
  because one ring buffer doubles when full and copies everything it holds, tens of megabytes
  with a few million messages kept: glibc's and macOS's `realloc` remap a block that large, but
  jemalloc's copies it, and between two `c9gd` machines on jemalloc (`ack` and `load`, each
  keeping every message in a `MemoryStorage`) that stalled a session 4 ms at 1.2 million messages
  and 8 to 10 ms at 2.4 million over 8 sessions, against at most 0.4 ms on glibc. A block's
  growth moves nothing already stored; only the ring of blocks, 32 bytes for each 4,096
  messages, ever doubles.
- **SQL storage** (`turbojet-sql`), in SQLite or PostgreSQL through sqlx: one transaction per
  commit, the same byte budget, and a lease per session, taken on opening and renewed by each
  commit, so gateways sharing a database can't run one session at once and one whose lease was
  taken can't commit. Stores can open sessions, commit and read resend steps with futures the
  driver awaits (`SessionStorage::begin_open`, `StoredSession::fetch`, `Job`), so a networked store
  never blocks the runtime; the simulator runs half its seeds over stores that do. The store
  conformance suite is public (`conformance` feature) for other stores to check themselves. Its
  model check runs random sequences of what a session does to its log against a model of it,
  after every step: a store must hold exactly the model's messages above what it says it evicted,
  and reopened, what was committed or what a later step left. `DiskStorage` and `MemoryStorage`
  also run it with budgets of a few messages, so eviction, segment rotation and resets meet at
  every turn. Bugs planted in eviction, resets and the in-flight marker are caught. It found that
  `DiskStorage` records a creation time at once while it buffers everything else; that's allowed,
  since only a schedule reads it.
  Sessions are keyed on an `extra` column as well as BeginString and CompIDs; `migrate` adds it to
  a database made before it, PostgreSQL altering the table under a lock and SQLite rebuilding it
  with foreign keys off, so a second gateway migrating at once waits and finds nothing to do.
- **Replication** (`turbojet-replica`): a store that wraps the active's and copies each commit to a
  hot standby. Everything durable goes through `StoredSession`'s mutations and `commit`, and a
  session writes nothing a commit covers until it's done, so the wrapper changes nothing in the
  session or driver: it records each mutation as an op, and a commit with any is made in the
  wrapped store and sent as one batch, the commit's job waiting for both (unless asynchronous,
  below). The standby
  (`Replica`, sans-IO) applies a batch through the same trait to a store of its own and commits
  it before acknowledging. A commit one side made and the other didn't leaves that side ahead
  of the wire, as a crash between a commit and its write does, which recovery already handles.
  - The protocol is model-checked with TLA+ (`specs/`, run with `specs/check.sh`):
    `Replication.tla` covers fencing, failover, catching up and checking a standby, and
    `InFlight.tla` the incoming side's in-flight marker across a takeover. The model found four
    of the flaws below (a whole send cut short, reading `in_sync` before acquiring, a standby
    that lost its store, unmarked redelivery after an asynchronous failover); `specs/README.md`
    says which settings reproduce each, and the bounds it's checked at.
  - Divergence is checked, not trusted away. A batch carries the session's commit number and its
    records must follow the standby's copy (an outgoing number not yet used, a stored message
    the next), or the standby refuses it. When a session opens, the active compares the
    standby's summary of it (numbers, in-flight marker, created time, marks) with its own and,
    if they differ, sends it whole, in batches of a megabyte of messages: `MemoryStorage` keeps
    changes never committed, so a log dropped without a commit leaves the standby behind. The
    batches are marked as parts of a session sent whole, and the standby holds
    them, up to 4 GiB of messages, until the last, then applies them in one commit: applied as
    they came, the first one's reset destroyed a copy that held everything sent, and a send cut
    short (a lost link, an active's crash) left it partly refilled, below numbers the
    counterparty has had, with the lease still saying it's in sync. A TLA+ model of the protocol
    found that. A `DiskStorage` under the standby still writes the reset at once and the messages
    at the commit, so a standby crash between the two leaves the copy reset; the check below
    keeps such a standby from taking over until the session has been sent again.
  - A standby promotes only once it has been checked since it started. The lease records only
    what the active sees fail, so a standby that loses its store on its own (one in memory
    restarting, a disk replaced, a crash between a reset and its commit) would leave it saying in
    sync; a closed session is compared again only when it opens, so a failover would resume it
    at 1. A TLA+ model found it, and that an active checking on reconnecting isn't enough: an
    active that crashes idle never reconnects, and only the standby knows it restarted. So each
    run of a `Replica` is an incarnation, a random number its welcome carries, and binds report
    the one they compared against. `catch_up`, now for every store, lists the sessions, binds and
    compares each closed one not compared against the current incarnation (sending it whole if
    it differs), counts each such open one behind so its next commit sends it whole, and once
    all are compared and none is behind sends `Checked` naming the incarnation; the standby
    takes it only if it names its own, so a check of the run before a restart is refused, and
    `promote` refuses a standby not checked. In the model, with the check, a standby that loses
    its store or crashes mid-send never takes over without every session (exhaustive at two
    sequence numbers a session, two epochs, four incarnations: 308 million states for crashes),
    and without it both lead to numbers sent again. The simulator loses the standby's store on a
    third of its replicated seeds and runs the catch-up on all of them; a planted failover that
    promotes unchecked is caught. A shuttle test races the check against a restart, and catches
    a standby that takes a check of another incarnation.
  - Fencing is by epoch. A `Lease` is taken under an epoch raised at each change of hands;
    hello, bind and batch frames carry it, and a standby refuses anything under one it has seen
    (bind too: a deposed active's handles, numbered from 0 like its successor's, would replace
    the active's bindings). The active renews the lease in its commits once half of it has gone
    and fails them once it was taken or ran out, so it stops even if the standby can't reach it.
  - A reset is sent at once and waited for, unlike other changes, as the built-in stores write
    it at once: otherwise a deposed active could reset its own copy and not the standby's, and a
    failover find the old numbers. The simulator found it, on a seed whose schedule reset both
    sides at a day's start.
  - Degrading, a commit the standby can't take is made alone, but only once the lease records
    the standby out of sync, fenced on holder and epoch: the lease is what fences an active the
    standby can't reach, so one that can't fence (`ManualLease`) refuses, and the commit fails.
    The simulator found that: with a lease that didn't fence, a deposed acceptor committed alone
    during an outage. Sessions the standby fell behind on are counted; each is sent whole at its
    next commit (the snapshot taken after the local commit, so it holds that commit), and
    `catch_up` sends the closed ones, counting a session's open logs so an opening waits while it
    sends one, and marks the lease in sync once none is behind (and checks the standby, below). A new holder leaves the mark as
    it is, so after a forced failover the first catch-up counts every session behind. Reading
    the mark is fenced too: only the holder under its epoch may, so an instance taking over reads
    it once it holds the lease, when no one else can change it. Read before acquiring, it could
    be stale: the active, still holding the lease, could put the standby out of sync and commit
    alone between the read and the acquisition. A TLA+ model of the protocol found that; the
    simulator, which checks and takes over in one step, couldn't. The
    simulator degrades on half its replicated seeds, with standby outages, and only fails over
    once the lease says in sync; a planted failover that doesn't wait is caught.
  - Over TCP, or TLS (`turbojet::tls`, as for FIX sessions), the link's calls block: they run in
    the store's jobs, on blocking threads. The active's link does its I/O on a current-thread
    runtime of its own, on one thread, so a caller can wait on it from any thread without
    deadlocking a runtime, a current-thread one included. Sessions share one connection, each
    with several requests under way when asynchronous; the standby answers a connection's frames
    in order, so a reader task hands each answer to the oldest waiter on its handle. A batch is sent at once, by a writer task, so the standby commits while the active
    does; a session bound on a connection since lost fails its commits until it opens again and
    catches the standby up. The standby applies batches with blocking store calls, so each
    connection has a thread with a runtime of its own, doing its handshake and its I/O: the
    thread that applies a batch is the one its arrival wakes. Serving it on a blocking thread of
    the caller's runtime instead (`Handle::block_on`) cost 2.8 µs a commit, another thread
    driving its I/O and waking it. While more frames are already read, the standby holds each
    batch's commit back (`Replica::on_frame_held`), then runs those held at once, a thread each,
    before writing their answers (`Replica::flush`), so different sessions' syncs overlap as they
    do on the active. A session's next batch, any other frame, or 64 held run what was held
    first, so a session's commits stay in order; a lost connection or a promotion drops what was
    held, and its answers are never sent.
  - Asynchronously (`asynchronous(n)`), a session's log queues each batch it sends, with the
    highest outgoing number it covers, and keeps `drained_through`, the highest number covered by
    a batch some commit has waited for. A commit's job waits for the oldest batches, in order,
    only while the newest is more than `n` past `drained_through`, or more than `MAX_QUEUED` (64)
    are queued: commits that carry only incoming state add no outgoing numbers, so without the
    cap they'd queue without limit. A batch over the bound on its own waits for itself; a reset
    waits for everything queued first, so the standby doesn't refuse those batches as diverged
    after it. The bound counts outgoing numbers, not batches, because that's what a failover
    must get past: the old active may have sent up to `n` numbers the standby lacks, and the
    promoted standby skips each session's next outgoing number ahead by `n`. Sent again, a number
    would be too low, which the counterparty takes as fatal; skipped too high, the counterparty
    asks for a resend and is gap-filled, since the numbers skipped hold no messages. So the skip
    has to be known at promotion, on the standby: each Bind carries the active's bound and a standby accepting less
    (`accepting_lag`) refuses it, failing the opening with `InvalidInput`. Promotion lists the
    standby store's sessions (`SessionStorage::sessions`) to skip each. Incoming state isn't
    bounded the same way: the new active may expect a lower incoming number than the old one
    reached, and the counterparty resends messages already handed over. The in-flight marker
    can't flag them: it's committed before a hand-over, but asynchronously that commit carries no
    outgoing numbers, so nothing waits for the standby to have it, and its window is 256 where
    the lag can be 64 batches of that. A TLA+ model of the incoming side found the guide's
    promise of `maybe_redelivered` broken, and that the fix must be exact and durable: promotion
    marks each session (`Marks::maybe_handled`, `BelowLogon`), the session stores the
    MsgSeqNum of the counterparty's next Logon as the bound before handing anything over (all
    below it was sent to the old active, which is fenced), flags each resend below it, and clears
    the mark once its next incoming number passes it, or at a reset. Kept in memory instead, a
    crash before catching up lost it; a marker at promotion covered one window. Synchronous
    promotion marks too, as insurance for a forced failover. The simulator's checker, which
    used to excuse a repeat after a promotion, now requires it marked; a planted promotion that
    doesn't mark is caught. It found that the simulator's ledger store never passed marks
    through, so no session in it had seen a mark of any kind across a restart.
    A failed answer surfaces at the commit that waits for it: it fails, or degrading, marks the
    standby out of sync and commits alone. A commit that finds the session behind clears the
    queue, since it sends the session whole anyway. The batches a commit's job waits for count as
    drained when the job is made, so a guard unbinds the session if the job panics or is dropped
    unrun: the next commit then sends the session whole, rather than trust numbers the standby
    never acknowledged. `LocalLink` applies a batch only when a job waits for it, which the tests
    use as the worst case (the standby lacks every batch no commit has waited for); `TcpLink`
    writes each batch as it's committed, so the standby is usually nearer than the bound, and its
    tests check only what doesn't race.
    Equal numbers don't make equal copies when asynchronous: after a failover with the lag full,
    the old active's copy can end where the promoted one does, holding messages in the numbers the
    promotion skipped. The summary a bind answers with carries a digest (FNV-1a, stable across
    builds) of the last `max_lag` outgoing slots, where alone the copies can differ, and a copy
    whose digest differs is sent whole. Synchronous binds read nothing for it.
  - The simulator replicates the acceptor's store on a third of the seeds that keep state in
    memory, with failovers (crash, restart over the standby's copy at the next epoch) and
    deposals (the acceptor runs on while another instance takes the lease, so its commits and
    opens must fail). A planted standby that acknowledges batches it never got is caught when a
    failover finds its copy behind. A third of those replicate asynchronously, 1 to 8 numbers
    behind, where the checker accepts what a promotion's skip allows (the next outgoing number
    above all written, skipped numbers gap-filled, incoming repeated) and nothing more; a planted
    promotion that doesn't skip is caught.
  - Shuttle tests (see "Concurrency tests" below) run the races the simulator's single thread
    can't: a TCP connection's waiters, registered by two sessions' commits, answered by the
    reader and failed by a loss at any point, each get their own answer or the loss, once; and a
    degrading, asynchronous store with a session committing through an outage, another opened in
    it and again after, and the catch-up running, never deadlocks an opening against the
    catch-up, says it's in sync only while the standby holds every commit but the lag's, and ends
    in sync with matching copies. They use shuttle's random scheduler: PCT, at as many
    iterations, missed a planted catch-up that ignored a session still behind one run in three.
  - Cost, storing one message and committing it, alone and replicated over loopback (Apple M3,
    2026-10-26, criterion medians, `benches/replica.rs`): memory 14.6 ns alone, 21.2 µs
    replicated; `DiskStorage` without fsync 1.7 µs and 27.9 µs; with fsync 4.1 ms and 6.9 ms,
    both copies syncing to the one SSD. Over TLS (mutual, TLS 1.3), memory replicated took
    22.6 µs. With the link on blocking threads rather than tokio the plain-TCP figures were
    19.6 µs, 25.0 µs and 7.7 ms: the tokio link's wake-ups cost about 1.6 µs a round trip, which
    TLS needed. Sending the batch in the commit's job, after the active's own commit, took
    17.6 µs, 23.1 µs and 8.2 ms: sending it at once overlaps the two syncs, at the cost of a
    thread hop (about 1.8 µs) when nothing syncs.
    Asynchronously, 64 numbers behind (Apple M3, 2026-10-27, the same benchmark, committing back
    to back): memory 2.10 µs against 21.3 µs synchronously, `DiskStorage` without fsync 4.95 µs
    against 27.6 µs, with fsync 7.9 ms against 7.5 ms. Back to back the standby sets the pace, so a
    commit costs what the standby takes to apply a batch, not the active's own commit: a burst
    within the bound costs less. With fsync on the one SSD the two syncs contend either way, so
    asynchronous gains nothing there.
    Many sessions committing at once over one link (Apple M3, 2026-10-27, `replicated commits at
    once`, wall time per commit across all sessions): memory 22.2 µs for 1 session, 4.55 µs for
    8, 3.09 µs for 64 synchronously, and 2.05 µs, 2.09 µs and 2.36 µs asynchronously, 64 behind:
    the link's per-batch cost (a frame, a write and an answer each, applied in turn) capped it
    near 450,000 commits a second. With fsync, replicated, 6.9 ms, 7.3 ms and 6.1 ms; `DiskStorage`
    alone, 4.03 ms, 2.36 ms and 3.43 ms, since concurrent `F_FULLFSYNC`s barely overlap. On
    Linux (AWS c9gd.2xlarge, ext4 on local NVMe, 2026-11-13) the journal merges concurrent
    `fdatasync`s: `DiskStorage` alone took 49.2 µs, 15.3 µs and 12.5 µs, but replicated 59.4 µs,
    53.2 µs and 53.5 µs, since the standby committed each batch in turn. With its commits held
    and run at once, replicated took 59.8 µs, 33.5 µs and 28.2 µs (two rounds each, interleaved,
    the second within 1.5%); memory unchanged. On the Apple M3, 8 sessions went from 7.3 and 6.3
    ms to 4.6 and 4.7 ms, 64 from 6.7 and 6.2 ms to 5.0 and 4.4 ms. With a drive flush shared by
    the commits waiting at once (Apple M3, 2026-11-13, two rounds each), `DiskStorage` alone went
    from 4.02 and 4.19 ms to 4.02 and 4.02 ms for 1 session, 2.44 and 2.31 ms to 1.09 and 1.06 ms
    for 8, 3.24 and 2.92 ms to 406 and 442 µs for 64; replicated, 7.24 and 7.99 ms to 7.26 and
    7.63 ms, 4.48 and 4.29 ms to 2.16 and 2.25 ms, 4.22 and 4.10 ms to 390 and 373 µs.
    The link then wrote each frame with a system call of its own, both ways: the active each batch
    as it was queued, the standby each answer before taking the next frame, however many were
    already read. Now the active's writer takes everything queued into one write, and the standby
    answers in one write every frame it has read whole. Before and after (Apple M4 Pro, 2026-11-04,
    the same benchmark): memory 25.9 and 26.3 µs for 1 session, 6.91 and 6.36 µs for 8, 5.70 and
    3.74 µs for 64 synchronously; asynchronously, 64 behind, 2.63 and 0.79 µs, 4.11 and 1.65 µs,
    4.06 and 2.13 µs. With fsync, 8 sessions took 6.98 and 5.12 ms, and 1 and 64 no different.
    One session synchronous has one frame in flight, so nothing to join; asynchronous, the active
    queues batches faster than one write each could take them.
- **Marks** (`StoredSession::marks`): what a restart must find besides sequence numbers, an
  operator's pause and the cancel on disconnect a session logged on with. They're the session's,
  kept through resets, and written rarely (a logon, an ending, an operator), so the disk store
  writes a small `<stem>.marks` file at once, as it does the creation time, and the SQL store a
  `marks` column with its commit, in a short text form both share. A registry lists them once,
  with `SessionStorage::marked`, before its first logon: the disk store reads its directory's
  marks files (stems read back to session IDs), the SQL store selects the rows with any. All
  three methods default to keeping none, so a store of one's own needn't change. A pause goes
  through the session's connection when it has one (`Command::Paused`, answered after the
  commit, as sequence changes are) and otherwise straight to its stored state under a claim.
  Cancel on disconnect marks the session at logon, before `on_logon`, and clears the mark on an
  ending that doesn't count; one that counts keeps it, since a crash mid-countdown must count
  down again, until the cancel fires and a task clears it. Marking at logon rather than storing
  countdowns is the point: a crash drops every connection at once, so at a crash there are
  usually no countdowns, only sessions logged on. A restored countdown starts from the restart,
  since the crash time isn't known.

## Configuration and operations

- **Session files** (`turbojet-config`): an acceptor, its counterparties, initiators and their
  stores from TOML, every value checked on loading and errors naming the section and key, and
  reloaded while running (changes apply from each counterparty's next Logon and each initiator's
  next connection, counterparties no longer listed are logged out, initiators are started and
  stopped as they're added and removed, and a file that doesn't load leaves the one in use).
  QuickFIX settings files are converted to one, not read directly, so there's one format to
  load and reload, and what each key became is there to review. The converter is strict: a key
  that maps is converted, one Turbojet sets in code (logging, socket options) becomes a comment,
  and one whose behaviour would differ is refused, with every problem in the file reported at
  once. Keys with their QuickFIX default (`ResetOnLogout=N`) pass, since that's what Turbojet
  does.
- **Management**: the registry, which already binds each session to its connection, is where an
  operator's view lives. At its Logon a session gives the registry when and over what connection
  it bound, and an `Arc` of atomics (state, resend, sequence numbers, last message times as
  offsets from the bind). A FIX session stores to them on the paths that already update the
  sequence-number gauges; a FIXP session, whose state and numbers change in more places, copies
  them in once per batch, when the driver takes its commit: listing sessions takes the registry's lock but never a session's, and
  the hot path gains a few relaxed stores and no allocation. Events go out on a bounded
  `tokio::sync::broadcast`, published only for state changes and only with subscribers, so a slow
  live view loses events (and is told) rather than holding a session up. A pause is a set in the
  registry, kept in the store's marks too (see Storage), checked under the same lock that binds
  a Logon, so a pause and a Logon can't cross;
  an initiator's run loop waits on a `Notify` while paused. An operator's resend rewinds the
  expected number before asking, because resends below it would be ignored as duplicates, and
  reuses the gap machinery by treating the next new message as the one that revealed the gap.
  A count of store resets since the bind is added to in `reset_store` (a reset at logon or the
  counterparty's mid-session, not a new schedule period's as the session binds), stored `Release`
  after the reset numbers and read `Acquire` before them, so a reader that sees a new count sees
  the new numbering rather than taking the old numbers for it. Only that way round: a read may
  show the new numbers before the count. Each binding's count starts at 0.
- **Observability**: structured logging and Prometheus-compatible metrics, with opt-in latency
  histograms (handling each inbound message, store commits, and reading input to its replies
  being ready to write), and an optional `MessageLog` that sees every message's bytes.

## Dictionaries and code generation

Typed messages come from data dictionaries. `turbojet-dictionary` loads and validates the FIX
Trading Community's official FIX Orchestra files, which give groups and enum values their official
names (`PreAllocGrp`, `OrderCancelRequest`) and document messages, fields and values, as well as
dictionaries in the QuickFIX XML format, the one venues ship their specs in. It merges a venue's
additions in either format onto either, keeping the official names and documentation where the
venue re-lists something without its own.

An Orchestra overlay (2026-11-07) refers to the base by id, as the official files do among
themselves: fields by tag, components and groups by their Orchestra ids, which a dictionary
loaded from Orchestra keeps for this (a QuickFIX-format base has none, so an overlay on one can
refer to its fields only). Components are kept by name and included where used, so replacing one
changes every message that has it; groups are expanded in place, so one the overlay redefines
replaces every expansion of that official name, nested ones too, to the same effect. `turbojet-codegen` turns a dictionary into `fix_enum!`, `fix_group!` and
`fix_message!` invocations (copying the dictionary's documentation into doc comments only on
request), from an application's `build.rs` or as a command for checked-in crates.

Orchestra scenarios, a venue's variants of a message, component, group, field or code set (a
NewOrderSingle for limit orders and another for market orders, both MsgType D), load as one
definition (2026-11-06). Nothing on the wire names a message's scenario, and a dictionary keys
messages by MsgType, so neither validation nor generated code could pick one; the union accepts
every scenario's messages. Members merge in the base scenario's order (the base first, then the
others as the file has them), each other scenario's new ones after the member they follow there,
and required only where every scenario requires them. A group's scenarios must start with the
same field, since it delimits each entry. A code set's scenarios are one set with every
scenario's codes, and a field's scenarios one field with every code set they name. `forbidden`
leaves a member out (so one forbidden in only some scenarios ends up optional), `constant` makes
it required, and `ignored` optional: `Member` has no room for a constant's value without a
breaking change, so it isn't checked. Choosing a scenario per message, which would keep them
apart, was the alternative; it only fits a session that only ever sends one.

The macros convert and write each field through out-of-line helpers, one copy per field type
rather than one per field, which keeps whole versions quick to build: built alone (Apple M3, Rust
1.98.1, 2026-09-28), FIX 4.4 (84 messages) takes about 6 s debug and 16 s release, down
from 75 s, and FIX 4.2 about 1.4 s and 2.6 s, for about 3% on the session benchmarks.
Parsing straight into the owned form, rather than through the borrowed one, doubled the generated
crates' build time; that was accepted for the faster owned parse.

An owned parse keeps its slots in one struct, not a local each (2026-10-16). Each early return
from a parse (a group's error, a missing required field) has to drop every owned slot still
live, so with a local per slot the code to do it grew with fields times returns, and LLVM's
InstCombine spent most of a large message's build on it: an `-Z llvm-time-trace` of FIX
Latest's ExecutionReport (619 fields) put 84 s of InstCombine, summed over threads, in its two
owned parses, against 0.1 s for the borrowed ones, whose slots need no drop. As one struct, a
return drops one value. Clean builds on an M3, alternating before and after:

| | Before | After |
|---|---|---|
| FIX Latest's ExecutionReport alone, release | 111.5 s | 36.1 s |
| FIX Latest's ExecutionReport alone, debug | 14.8 s | 15.1 s |
| FIX 5.0 SP2, release | 112.6 s | 95.5 s |
| FIX Latest, release | 23 minutes | 6.5 minutes |

The typed parse benchmarks didn't move: over two rounds alternating before and after, each was
within 1% (FIX 4.2 NewOrderSingle 131.0 ns before, 130.8 ns after; ExecutionReport 183.3 and
183.3 ns; FIX 4.4 NewOrderSingle with nested groups 605.6 and 605.6 ns). Borrowed parses keep a local per slot:
as one struct they were 7% slower (FIX 4.2 NewOrderSingleRef 103.7 to 111.4 ns).

The generator spreads a version's groups and messages over up to 16 private modules of at least
8 items each, re-exported (2026-10-17). rustc puts all of a module's code in one codegen unit,
optimised on one thread, so with every message in one module a release build was mostly serial:
an `-Z llvm-time-trace` of FIX 5.0 SP2 showed one thread busy for 51 s, on every message's parse,
while the other 14 finished within 22 s, most within a few. Clean release builds on an M3,
alternating before and after:

| | Before | After |
|---|---|---|
| FIX 5.0 SP2 | 95.4 s | 47.8 s |
| All eight versions | 189 s | 173 s |
| FIX Latest | 6.5 minutes | 207 s |

All eight together gain less, as their eight modules already kept the cores busy; they peak at
10 GB, and FIX Latest at 9 GB. The benchmarks build with one codegen unit and fat LTO, so the split
can't show there; under the default release profile the typed parse benchmarks stayed within
1.5% (FIX 4.2 NewOrderSingle 157.0 and 156.5 ns, FIX 4.4 NewOrderSingle with nested groups 717.2
and 706.6 ns).

Building the generated crate at `opt-level = 1` was measured and rejected (2026-10-17). Clean
release builds on an M3 (Rust 1.99), alternating: FIX 5.0 SP2 took 46 and 47 s at `opt-level = 3`
against 39 and 42 s at 1, and all eight versions 181 s against 158 s. At `opt-level = 0`, SP2
still takes 21 s, most of it the front end (`-Z time-passes`: 4.9 s macro expansion, 5.4 s type
checking), and level 1 only shortens the LLVM tail (21.0 to 15.8 s waiting on codegen units). The
typed parse benchmarks under the default release profile, two rounds alternating, got 6-16%
slower:

| | `opt-level = 3` | `opt-level = 1` |
|---|---|---|
| FIX 4.2 NewOrderSingle | 158.5 ns | 176.4 ns |
| FIX 4.2 NewOrderSingleRef | 125.2 ns | 144.7 ns |
| FIX 4.2 ExecutionReport | 229.6 ns | 251.7 ns |
| Build an ExecutionReport | 203.0 ns | 225.3 ns |
| FIX 4.4 NewOrderSingle with nested groups | 717.2 ns | 772.0 ns |
| FIX 4.4 NewOrderSingleRef with nested groups | 585.3 ns | 618.6 ns |

The Orchestra files for FIX 4.2, 4.4 and FIXT 1.1 are vendored in `dictionaries/orchestra`
(Apache-2.0, © FIX Protocol Limited), with FIX 4.0, 4.1, 4.3, 5.0, 5.0 SP1 and 5.0 SP2 converted from the FIX
Unified Repository by the FIX Trading Community's own stylesheet (`convert-unified.sh`, then
`repair-unified.py` for what the stylesheet gets wrong: FIX 4.0 and 4.1's datatype names, where
`char` is a string, `time` a timestamp and `date` a date; repetition FIX 4.0 gives only in prose;
FIX 4.3's inline groups); each version is generated into its own module of `turbojet-fix` (`fix40`,
`fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1`, `fix50sp2`, the last three over FIXT
1.1's header), and CI checks they match. Each module is behind a feature of its name, none on by default, so an application
compiles only the versions it uses. They were six crates until 2026-10-16; one crate means one to
publish and document, and a new version is a feature rather than a crate. Built alone, a version
takes as long either way (clean debug builds on an 8-core M3: FIX 4.4 13.0 s as a crate and 12.9
s as a module, FIX 5.0 SP2 25.0 s and 25.1 s). All six together take longer as one crate, 47.9 s
against 29.7 s, as six crates compile in parallel and one mostly doesn't; that falls on the
workspace's own builds, which turn every version on. A dictionary's
mistakes can be corrected before generating (`Dictionary::make_optional`, `--optional`;
FIX 4.2 makes QuoteCancel's NoQuoteEntries optional, as FIX 4.4 did). Sessions can also
check inbound messages against a dictionary (feature `validation`, `SessionConfig::with_dictionary`).

The generator can also take a list of messages (`Generator::only_message`, `--message`): it plans
only those, and since groups and enums were already planned from what the generated messages
reach, it leaves out the rest with no further pruning. For FIX Latest (EP312), whose whole
dictionary builds in 104 s debug and 23 minutes release (clean, M3, 2026-10-16), five order
messages built in 18 s and 279 s before the owned-slot change above.
`turbojet-fix`'s library builds once even under `--all-targets`, `cargo test` and `cargo bench`
(`[lib] test = false, bench = false`, as it has no unit tests): compiled twice at once, as
before, every version took about 7 GB each, and CI's runners were shut down mid-build at eight.

FIX 4.0 and 4.1 sessions differ from later ones in what the session sends, chosen by BeginString
(`SessionConfig::predates_fix42`): a Reject carries only RefSeqNum(45) and Text(58), since
RefTagID, RefMsgType and SessionRejectReason came with FIX 4.2; a session Reject stands in for
BusinessMessageReject, also FIX 4.2; a ResendRequest asks for everything to the end with
EndSeqNo(16)=999999, not 0; and SendingTime is whole seconds, whatever `timestamp_precision`
says. FIX 4.0 has no ResetSeqNumFlag(141), so `reset_on_logon` is refused there, and no location
IDs (144, 145), which `with_reverse_route` doesn't echo. QuickFIX's fix40 and fix41 acceptance
scripts check this.

Enumerated fields are generated as `fields::Code<E>`, which keeps a code the dictionary doesn't
list as `Code::Unknown`. They were strict until 2026-10-11: an unknown code in any field, even one
the application never read, failed the parse, and an application that rejected the message lost it,
since a counterparty doesn't resend what we reject. QuickFIX's typed fields never fail on a code;
rejecting one is its data dictionary's job, at session level, and Turbojet now splits it the same
way. `--strict-enums` still types fields `E`. The cost is size: `Option<Code<E>>` is 24 bytes to
`Option<E>`'s 1, so typed messages are bigger and each parse or build moves more. On an M3
(criterion medians, a loaded machine, both measured the same day) owned parsing of a FIX 4.2
NewOrderSingle went from 120 ns to 136 ns, borrowed from 93 ns to 110 ns, and building an
ExecutionReport from 155 ns to 171 ns; a known code still allocates nothing.

The validator's switches follow QuickFIX/J's, with its defaults: `reject_invalid` is its
RejectInvalidMessage, `allow_user_defined_tags` its ValidateUserDefinedFields=N (both the tag
number and the in-message checks, from 5000 up), `allow_unknown_tags` its AllowUnknownMsgFields=Y
(the same checks, below 5000), and `SessionConfig::check_empty_values` and `check_header_order` its
ValidateFieldsHaveValues and ValidateFieldsOutOfOrder. A QuickFIX dictionary's
`allowOtherValues="Y"` turns off the code check for its field and keeps it a `Code<E>` under
`--strict-enums`. The QuickFIX settings converter maps each of these. One default differs:
QuickFIX rejects an unknown code in any message (ValidateFieldsOutOfRange=Y), as the FIX session
test case 14e expects, but a message rejected is never resent, so an ExecutionReport carrying a
fill would be lost over a code the venue added to a field the application may not even read.
`values` checks codes in session-level messages, and `application_values`, off, in application
messages; the acceptance harness turns it on, so 14e and the ReverseRoute scenarios still check
the rejection.

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

## FAST

FAST 1.1 is a codec, `turbojet::fast` behind the `fast` feature: a feed's template XML loaded at runtime
and each message decoded into a `Message`, so the typed FIX messages read FAST market data, rather
than codecs generated per feed. It leaves receiving a feed (multicast, line arbitration, gap
recovery) to the application, and FAST 1.2's extensions until a feed needs them.

It was a crate of its own, `turbojet-fast`, until 2026-10-25 (0.5.0 its last release, left on
crates.io): it
decodes into `turbojet::Message`, so its users built the whole engine anyway, its version only
ever moved with `turbojet`'s, and SBE's runtime support was already `turbojet::sbe`. Its one
dependency, `roxmltree`, is optional, so the feature costs nothing to anyone without it.

Loading compiles each template into a list of field instructions with its operator, initial value
and presence-map use worked out, and each previous value's dictionary and key resolved to a slot
in a flat array, so decoding never hashes a name. Static template references are expanded where
they're used, in the referenced template's dictionary context, as section 6.4 says. The decoder
reads presence maps a bit at a time, past their end as zeros, and nests a segment's map for groups,
sequence elements and dynamic references. Values go into a scratch buffer or their slot, and the
message is built with `Message::clear` and pushes, so a decoder that has decoded a message once
allocates nothing for the next (an exact test checks it). The encoder shares the slots' code, so
its previous values change exactly as a decoder's will, and leaves a field out wherever its
operator lets the decoder work it out; string deltas take the shorter of a front or back edit.

What a message can make the decoder do is bounded: nesting, in templates and through dynamic
references in the stream, stops at 16 levels, and a message holds at most 100,000 sequence
entries in all, since an entry can take no bytes. Errors carry the specification's codes (S, D and
R); reportable errors are always reported.

Correctness comes from three sides. The specification's Appendix 3 examples are tests, though three
of its examples have typos, followed by their own binary columns (-8193 is `0x7f 0x3f 0xff`, not
`0x73`, twice; a mandatory mantissa of 942755 ends `0xa3`, not `0xa4`), and where the tail
operator's section gives D6 and D7, Appendix 4 and the other operators give D5 and D6, which the
code uses. Against `fastlib`, another Rust implementation, 20 streams of 1,000 random messages
over templates using every operator decode alike in both, from either side's encoding. That found
three `fastlib` bugs, which the generator avoids: it computes integer deltas in 64 bits, where a
`uInt64`'s can need 65; its increment doesn't wrap; and for an optional field it writes a delta
string's empty part as a nullable string (`0x00 0x80`, which reads as "\0"), where section 10.7.3
makes only the subtraction length nullable. Fuzzing decodes arbitrary bytes and templates, and
re-encodes each decoded message to check a second decoder reads it back the same; its first
minute found a decimal zero with a positive exponent written as "000…".

On an M3 (criterion, a loaded machine), a refresh following the previous message decodes in
243 ns with one entry, 1.75 µs with 10 and 8.43 µs with 50: about 170 ns an entry of nine fields.
Most of that is building the `Message`: a profile of the 50-entry decode puts 27% in Turbojet's
integer formatting, for each value and each tag, and 13% in copying one to three digits at a
time; FAST's own work (operators, slots, presence maps) is about a quarter.

## Python bindings

`crates/turbojet-python` (PyO3, built with maturin) runs initiators from asyncio. It's outside
the workspace, as the fuzz crate is: a PyO3 crate's `cargo test` links libpython, which a
workspace build can't be relied on to find, so it's tested from pytest against the `gateway`
example (`scripts/python.sh`).

- **No GIL on the session's task.** The engine runs on `pyo3-async-runtimes`' tokio runtime. The
  `Application` the bindings give it only pushes Rust values (`SessionHandle`, `Message`, a
  `Hold`) into an unbounded channel; Python objects are made when asyncio takes an event from
  `engine.events()`. The channel is bounded in effect by holds: a session stops reading 256
  messages past the oldest one Python hasn't finished.
- **Events already waiting skip the runtime.** Awaiting an event through the runtime costs an
  asyncio future and its callback, two tokio tasks, the GIL taken on a worker and
  `call_soon_threadsafe` to wake the loop. Under load an event is nearly always waiting already,
  so `__anext__` takes it on the event loop's thread and returns an awaitable that's already
  finished; only an empty channel goes through the runtime. A finished awaitable doesn't suspend,
  so every 32nd yields to the event loop once first, as `asyncio.sleep(0)` does: a loop over
  events that keep coming still lets other tasks run.
- **At least once.** `on_message` holds every application message (`Context::hold`) and returns
  at once; the event's `done()` releases the hold. Dropping the event doesn't: the bindings forget
  the hold rather than release it with the object, since a release from garbage collection would
  count an unfinished message received (lost on a restart), while a hold never released shows up
  as a stalled session. Rejecting is something Python does later, so `Session.reject` builds the
  BusinessMessageReject itself and sends it like any message.
- **Sync hooks.** `verify_logon`, `to_admin` and `should_resend` answer before the session goes
  on, so they're the exception: Python callables run with the GIL on the session's task, on
  logons and resends only. An exception in one is treated as the engine treats a panic there.
- **Untyped messages.** The generated types are per FIX version and large, so Python gets
  `Message`. A repeating group is read with its member tags, as there's no version to find its
  spec by; `turbojet.tags` is generated from FIX 5.0 SP2's by `scripts/codegen.sh`.
- **Logging.** `log_to_python` sets a tracing subscriber whose layer makes each event a
  `logging` record, adding the enclosing spans' fields as attributes (`session`, from the
  initiator's span or the connection's `id`; `addr` and `role` from an endpoint's), never over
  one of the record's own. `log` records from dependencies come through tracing-log. The layer
  caches each logger and its level, so an event a level filters out costs no GIL; one logged takes
  it on the session's task. Each span's fields are kept as text, formatted when it's made or
  recorded. `log_to_stderr` sets a subscriber too, so each refuses once the other is on.

Measured 2026-10-17 on an Apple M3, not idle (load average about 3): one session, open loop
against the release `ack` example, NewOrderSingle to ExecutionReport, latency from each order's
due time (`load` from Rust, `crates/turbojet-python/bench/load.py` from Python, release builds,
run one after the other):

| Orders/s | Rust p50 | Python p50 | Rust p99 | Python p99 |
|---|---|---|---|---|
| 1,000 | 86 µs | 421 µs (272 from sending) | 120 µs | 728 µs |
| 10,000 | 30 µs | 85 µs (62 from sending) | 57 µs | 114 µs |
| 30,000 | 30 µs | 102 µs (73) | 83 µs | 224 µs |
| 60,000 | 32 µs | 111 µs (85) | 52 µs | 222 µs |
| 100,000 | 34 µs | 127 µs (97) | 57 µs | 262 µs |

Warm, Python adds 55 to 95 µs a round trip and keeps up with Rust's `load` to 100,000 a second on
one asyncio loop. Its worst orders, about 3 to 4 ms at 100,000 a second, are mostly its sender's:
`asyncio.sleep` overshoots by up to milliseconds, where `load` spins for the last stretch. At
1,000 a second, both are slower after each lull (see Latency), and Python's more: an event then
finds the channel empty and goes through the runtime. Once, on a machine loaded to 8 to 11, the
event loop stopped for 2.6 s and the session's send queue filled; it didn't recur in eight runs
after, with garbage collection on or off.

## API conventions

A review of the public API (2026-10-24, against the Rust API Guidelines) settled these, and
`scripts/check.sh --semver` (cargo-semver-checks against the last release on crates.io) checks
before a release that the version says what changed. It sees Rust items, not a sessions file's
keys, and not every change of a type: against 0.4.0 it found the removed and renamed items but
nothing in turbojet-config, whose keys and `server_tls` changed, so the changelog stays the
record of what broke.

- **Settings live in configuration structs, not on handles.** `Acceptor`, `Initiator` and the
  FIXP endpoints are cheap-clone handles with no builder methods: what they're made with is an
  `AcceptorConfig` or `InitiatorConfig` (and FIXP's twins) around the shared `SessionConfig`,
  fixed once made but for what's meant to change while running (`set_allowed_ips`,
  `reconfigure`). Builder methods on the handles had disagreed about whether a clone saw them.
- **One error type for configuration.** Parsing a value and checking a configuration return
  `ConfigError`, a `std::error::Error`; the sans-IO constructors return it too rather than
  panic. `io::Error` is for files and sockets.
- **A store implements what it's for.** The methods a store needs (durability, the in-flight
  marker, creation times) have no defaults, so leaving one out doesn't compile.
- **FIX's spelling, the crate's words.** Types and variants keep FIX's names
  (`SecurityIDSource`, `ApplVerID`, `CompIDProblem`), so they match the specification; Rust
  case only splits acronyms in snake case (`ioi_id`). The other side of a session is the
  counterparty; a limit is `max_` then what it limits.
- **Named required fields.** A generated message or group with two or more required fields
  takes them in a `NameRequired` struct, so values of one type (the IDs; LeavesQty, CumQty and
  AvgPx) can't be swapped unnoticed. It costs FIX 5.0 SP2's release build about 7%.
- **One path per item.** Modules whose every item is re-exported at the root are private; the
  layer modules (`store`, `registry`, `session`, ...) stay public. What the macros expand to is
  in a hidden `message::__private`.

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
- **Interop with QuickFIX/J and QuickFIX/n** (`turbojet-interop`), with Turbojet as initiator and as acceptor, on
  FIX 4.2, 4.3 and 4.4 and on FIXT.1.1 with FIX 5.0 SP2: logon and logout, reconnection, heartbeats
  and TestRequests, application messages (one with XmlData containing SOH), gap fills and resends in
  each direction, SequenceResets in both modes, MsgSeqNum too low and sessions identified by SubIDs
  and a LocationID, directly and through a proxy that loses, garbles, cuts and delays messages,
  slows the link and silences or stalls either side. Also over TLS: orders with
  the server's certificate alone and with mutual TLS, a reconnect with a fresh handshake, and
  handshakes refused for an untrusted server, a missing client certificate, an untrusted one and
  a revoked one,
  with each peer checking the server's name as Turbojet does. They found Heartbeats going out a
  second late, since fixed. Against QuickFIX/n (v1.14.1, on .NET 10), every scenario runs too, the TLS ones with PKCS#12 bundles on both sides: as acceptor it leaves
  ResetSeqNumFlag out of its Logon reply, and as initiator it disconnects on a garbled message,
  recovering it by resend.
- **QuickFIX's 467 scripted acceptance scenarios** (`turbojet-acceptance`), which cover the FIX
  specification's session test cases, on every build. They found five deviations from the spec's
  test cases, since fixed; 440 pass, and the 27 that fail are listed with their reasons in
  `known_failures.txt`: two deliberate differences (keeping a valid message that follows a
  too-long BodyLength, and no RefTagID for a negative tag) and scripts that rely on QuickFIX's own
  behaviour or dictionaries.
- **Deterministic simulation** (`crates/turbojet-sim`): an initiator and an acceptor run against
  each other in one thread from a seed, over a TCP-like network that splits, delays, stalls, resets
  and black-holes connections (on some seeds behind a middlebox that drops, duplicates, reorders and
  corrupts messages), with process crashes between events and inside store calls, store errors,
  power loss on disk stores with and without sync, each node's clocks its own (drifting up to
  200 ppm, its wall clock off by up to half a second and stepped), processes paused for longer than their
  counterparties wait (on replicated seeds, an acceptor resuming after another instance took its
  lease), logout requests, operators skipping numbers ahead
  and resetting both sides, daily schedules, on a third of the seeds sessions with SubIDs and a
  LocationID, and initiators logging on with NextExpectedMsgSeqNum (a third of the seeds) or a
  reset (a sixth). Each seed counts the states only some reach (`COVERAGE`), and the per-push run
  fails if fewer of its seeds reach one than about half as many as did (2026-11-07): a resend
  straight after a Logon's 789 (11 of 100), a reset at logon (8), a pause (24), a deposed acceptor
  resuming (3), a power loss with writes not written back (9) and a torn commit (5), since nothing
  else would show a change stopped reaching them. Random runs print each state's share. A resend
  mostly takes one step of the driver, so faults at random times rarely found one under way: in
  7,056 seeds a crash came during one on 0.99%, a skip on 0.23% and a failover on 0.11%. On a third
  of the seeds, a third of the resends still under way after their first step now meet one of the
  seed's faults at once, and in 49,708 seeds those came on 5.55%, 4.51% and 0.58%; the per-push run
  asserts crashes (6 of 100) and skips (3) during a resend too. A resend after a clock step (0.06%)
  and a failover during one are still too rare to assert on a hundred. After every event a checker holds both sides to what their stores recorded, what they
  wrote and what their applications received, and once the faults stop they must settle; bugs
  planted in the simulator show the checker catches what it should. 100 seeds run on every push and
  random ones nightly, and any failure replays from its seed (`scripts/sim.sh`). It found a write
  deadlock in the connection driver, torn sequence-number records in `DiskStorage`, an operator's
  skip ahead making a counterparty abandon a gap, and a duplicated reset Logon setting both sides
  resetting each other, all since fixed. Its checker keeps every earlier sequence epoch for the
  rest of the seed: an initiator resetting at each attempt to log on can leave an acceptor that still
  holds the old connection delivering from many epochs back. It exercises outbound
  limits and inbound `Delay`, but not `Reject`, since its checker can't cheaply model a
  BusinessMessageReject in place of a delivery, and no `MessageLog`; session and connection tests
  cover both.
- **A soak** (`crates/turbojet-interop/tests/soak.rs`), nightly: the real `Initiator`, `Acceptor`,
  tokio driver and TLS, which the simulator doesn't run, against each other over `DiskStorage`,
  through a proxy that cuts the connection, blackholes, stalls, holds or slows either direction,
  and with either process crashing (its runtime dropped) and restarting, over TCP and TLS. Every
  order the client stored reaches the server's application and is acknowledged, once each and in
  order but for repeats marked possibly handled. Twenty seconds of faults each way, on an Apple M3
  (2026-11-07): 37,640 orders through 43 faults over TCP, 34 deliveries marked after the crashes;
  14,071 through 18 over TLS, 191 marked.

  Replicated (`soak_replicated_*`), the server is an active and a standby, each a runtime of its
  own over its own `DiskStorage`, the link through proxies of its own (TLS with a client
  certificate in the TLS test), the lease a `SqlLease` in SQLite with a two-second term, degrading
  or not by the run's seed. Besides the faults above, either node crashes, the link takes the
  proxy's faults, and the test, as operator, takes over by the documented sequence from an active
  that crashed or from one a partition has cut off from the lease's database (a `Lease` wrapper
  whose calls fail) and from the standby, while the client can still reach it; that one's
  application must get nothing a second after the promotion. A node's crash also cuts the link
  proxies, since the standby serves each link connection on a thread outside its runtime, which a
  process's death would close. A takeover the standby can't make is given up and the old active
  restarted. Sixty seconds each, on an Apple M3 (2026-11-07), no engine bug found: over TCP, not
  degrading, 63,291 orders through 5 failovers, 2 deposings and 3 takeovers refused as the standby
  hadn't been checked since restarting; over TLS, 27,079 through 1 failover and 3 deposings;
  degrading, over TCP, 38,132 through 2 failovers, 1 deposing and 1 refused as out of sync. Cutting
  a live active off from the client alone doesn't make a takeover possible, rightly: it renews its
  lease.
- **Concurrency tests** with [shuttle](https://github.com/awslabs/shuttle), on every push. Built
  with `--cfg shuttle`, the registry's and the holds' locks and atomics are shuttle's
  (`src/sync.rs`), as are replication's state and link waiters (`turbojet-replica`'s
  `src/sync.rs`), and the tests run each protocol threads share in thousands of interleavings:
  a hold released on another thread is either seen or wakes the session; two Logons at once bind
  the session once; a pause racing a Logon either refuses it or finds it connected, to log it
  out; a send queued before a logout goes out first; replication's link and catch-up racing
  commits and outages (see Replication above). A bug planted in each shows its test catches it.
  They use shuttle's random scheduler at 5,000 interleavings a test, not PCT. Re-checked on an
  Apple M3, ten runs each: a check and insert split across two locks at logon, a pause that
  reads the sessions and marks the pause under separate locks, and a Logout that doesn't wait for
  the sends queued before it were caught 10 times in 10 under PCT (depth 3, 2,000 iterations)
  and under random. A `tidy` that clears the holds' woken flag only when it forgets a hold, so a
  release after an earlier one's wake-up was taken wakes no one, was caught 0 in 10 under PCT,
  at depths 2 to 6 and at 20,000 iterations, and 10 in 10 under random from 500 interleavings.
  PCT switches threads only at a yield or a few change points spread over the run, and this
  window is a few steps at its start. Replication's catch-up plant was caught 6 runs in 10 under
  PCT and every run under random. The four tests here take 0.36 s together, from 0.20 s.
  What shuttle doesn't model, and these tests can't find: orderings weaker than
  `SeqCst`, since it runs every atomic as `SeqCst`; and races inside tokio's `mpsc`, `oneshot`
  and `Notify`, where it never switches threads. A lost wake-up planted in waiting for a resume,
  between our lock and tokio's `Notify`, goes unfound for that reason.

## Performance

### The hot path

The long-term goal is that a message in steady state, from the read buffer through the session
and application and back out to the socket, is neither copied nor allocated beyond what the
application itself asks for. Where it stands:
- A `Message` keeps all its fields in one buffer with an offset index (two allocations, not one per
  field), and each inbound frame is decoded into one `Message` reused for the connection, so
  decoding doesn't allocate once it has grown. It still copies each frame, once, out of the read
  buffer, which costs too little to remove: without the copy, a 169-byte NewOrderSingle decoded
  in 160.9 ns against 165.9 ns (M3, 2026-10-09), 0.5% of the 939 ns a message takes through the
  session. A message type borrowing the read buffer would have to run through `Fields`, the
  typed-message macros and the generated crates.
- Raw group access is zero-copy, and typed messages parse into borrowed forms
  (`NewOrderSingleRef`) that read their strings, lists and groups from the message, so typed
  parsing allocates nothing.
- The session encodes what it sends once, straight into one reused output buffer, and the stores
  keep those bytes. The application's replies go into a list the session keeps, and a typed reply
  is written into a message the session reuses (`FixMessage::write_into`).
- Text fields in generated messages are `CompactString`s, kept inline up to 24 bytes, so the
  strings an acknowledgement copies from its order don't allocate. Longer values, and groups'
  entry lists, still do.

As of 2026-10-25 the engine makes 1 allocation per order with the memory store (its copy of the
message) and none with the disk store, and the example application none.
`tests/allocations.rs` counts them by stage, and the build fails if a count changes.

The memory store's copy stays an allocation of its own. Keeping its messages back to back in one
ring buffer instead, evicting from the front, removes it, but isn't faster (Apple M3, 2026-10-26,
criterion medians, interleaved with the store as it is): storing a message took 14.0 ns against
13.6 ns, since a warm `malloc` of a 245-byte message costs about what copying it into the ring and
keeping its place does, and with 1,000 orders in flight on each of 1 to 32 sessions
(`benches/sessions.rs`, `--quick`, means of three and four runs) it managed 0.55 / 1.14 / 1.95 /
2.30 / 2.31 / 2.33 M orders a second against 0.54 / 1.24 / 1.94 / 2.23 / 2.32 / 2.25, within
the runs' noise. The allocator lock was 1.6% of busy time at 8 sessions (see Many sessions), so
there was little to win.

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

Writing an integer's digits into a stack buffer and appending them in one go, to save the copy
per group that FAST's profile shows, is slower (2026-10-15, an M3, criterion medians, the same code
rerun within 5% of its first run). Against 172 ns to build an ExecutionReport and 1.75 µs to
decode a 10-entry FAST refresh, appending the buffer a byte at a time took 226 ns and 1.88 µs;
with one `push_str` after `str::from_utf8`, 277 ns and 2.28 µs; and one `push_str` for values
under 1000 with the buffer only for larger ones, 235 ns and 2.12 µs. Without `unsafe`, a `String`
takes digits fastest as the table's own `&str` slices.

Sends don't keep a message encoded and patch only the fields that change: it would save too little.
Measured 2026-10-09 on an M3, criterion medians for a FIX 4.2 NewOrderSingle of about 150 bytes:
the application builds the typed order in 84 ns, turning it into a `Message` for
`SessionHandle::send` takes 160 ns, and the session sends it in 196 ns (header, encoding (47 ns),
store and commit). Patching a pre-encoded order (copying the fixed bytes, writing MsgSeqNum, both
timestamps, ClOrdID, quantity and price, then BodyLength and CheckSum) takes at least 73 ns. So a
template could save about 220 ns an order, 1.5 to 2% of a round trip over localhost (10.3 µs
spinning, 16.3 µs replying from `on_message`). The session's numbering and storing stay either way.

Typed parsing was profiled on 2026-10-15, for the borrowed FIX 4.2 NewOrderSingle (110 ns then).
The generated parse loop took about 30%: setting each of the message's 68 field slots to empty,
then one jump-table dispatch per field present. Reading each field's value took 18%, much of it
the char-boundary checks of slicing a `&str`, which safe code can't skip. Decimals took 12%:
validating in three passes, then a fourth by rust_decimal's parser. They are now read in one pass
into a `u64` mantissa and a scale, up to 19 digits (any usual price or quantity), with longer
values left to rust_decimal: 110 ns to 103 ns borrowed, 201 ns to 183 ns for an ExecutionReport.
Copying the 1,184-byte result out of its `Result` in the caller took 10%; timestamps 7%.

The struct's size is about half the cost of a typed parse. A FIX 4.2 NewOrderSingle declaring
only the benchmark order's 10 fields, with `fix_message!` alone, parses in 59.5 ns borrowed and
62.8 ns owned, against 100.5 ns and 124.0 ns for the generated one's 68 (Apple M3, 2026-10-26,
criterion medians; 176 and 224 bytes against 1,184 and 1,376). Profiled, the generated borrowed
parse spent 30% in its own function, setting the slots, dispatching each field and building the
result from them, and 20% in the caller's `unwrap`, copying the 1,184-byte result out of its
`Result`; reading the values was the rest. An earlier reading (2026-10-15) took only the copy
into account, 15 ns of an owned FIX 4.2 parse and 34 ns of a FIX 4.4 one (3,080 bytes), and
judged size to matter less than it does. An application gets the whole gain by re-listing the
messages it parses, with only the fields it uses, in a `--merge` file, which replaces a message
whole; the default crate stays complete, since which fields matter is the counterparty's. Building it in place with `MaybeUninit`
isn't worth `unsafe`: Rust can't construct in the caller's slot, so it would only save the copy
behind an out-parameter API. Shrinking every enumerated field from 24 bytes to 16 (boxing
`Code::Unknown`'s string) saved 1 to 4%. Comparing MsgType(35)'s raw code in `Message::parse`,
rather than building a `MsgType` to compare, saved nothing: finding the field is the cost.

A message's borrowed form, `NameRef<'a>`, is the message and a `u32` per declared field giving
its first occurrence, with each value converted when its method is called, so the methods return
a `Result` (and `?` in `on_message` still answers a malformed value with a Reject). Parsing walks
the fields once, matching each tag to its field; it marks where a field is rather than converting
it, checks each group's structure with `check_group` and skips its entries, so a member's tag isn't
taken for the message's own, and then checks that the required fields and groups are there. A
group's entries are parsed when it's read, into the group entries' borrowed form, which converts
every field as before; so a bad entry fails the read, not the parse. Each read converts again; a
field read often is worth keeping in a local. `into_owned` parses the owned form from the message,
so it fails as that does. `Debug` reads every field, rather than printing the message, so a
`Secret` is masked.

Until 2026-10-31 the borrowed form was a struct with every field converted, like the owned one but
borrowing; the lazy form was added beside it as `NameView`, and replaced it once it beat it even
reading every field. For FIX 4.2's NewOrderSingle the lazy form is 280 bytes against the struct's
1,184, and for its ExecutionReport 368 against 1,576. On the benchmark order (10 body fields set)
and its ack (17) (`benches/lazy_view.rs`, since removed, Apple M3, 2026-10-31, criterion medians):

| | parse | parse, read 2 fields | parse, read 10 fields |
|---|---|---|---|
| NewOrderSingle, struct | 101.3 ns | 89.3 ns | 89.8 ns |
| NewOrderSingle, lazy | 36.4 ns | 36.3 ns | 80.4 ns |
| ExecutionReport, struct | 154.1 ns | 131.4 ns | 133.1 ns |
| ExecutionReport, lazy | 51.3 ns | 51.0 ns | 83.0 ns |

The struct's parse alone includes copying it into `black_box`, so it costs about 90 ns
(NewOrderSingle) or 130 ns (ExecutionReport) in use, and its empty slots only about 10 ns of that
once the fields are read: converting the values present is most of a typed parse either way. The
lazy form takes 40% of the time for code that reads a few fields of each message (routing on
ClOrdID and Symbol, say) and a quarter of the memory, which matters to code that keeps parsed
messages. The more fields a message carries, the more it saves: reading 10 of the ack's 17 takes
62% of the struct's time, against 90% for the order's 10 of 10. What it costs is errors at read
time rather than at the parse, and a conversion each read. A prototype (2026-10-29) measured two
alternatives: converting each value while indexing, to keep errors at the parse, was slower than
the struct once the fields were read (65.6 ns to parse, 102.8 ns to read all 10), since each read
converted again; and `u16` indices, which halve the table but need a fallback for a message of more
than 65,535 fields, which one built in code can be. The accessors are `#[inline]`, so the thousands
a dictionary generates aren't compiled until an application calls them: clean builds of
`turbojet-fix` alone took 27 s and 16 s (release, debug) for FIX 4.4 before the lazy form was added
beside the struct and 31 s and 16 s after, and 63 s and 34 s for FIX 5.0 SP2 before and 55 s and
29 s after, within the noise of a machine running other work. Moving rarely used fields into a boxed sub-struct was considered too,
but it changes the default API, needs a rule for which fields are rare, and wins only part of
the gain.

Text such as `8=FIX.4.4|35=0|…` (`codec::decode_text`, `Message`'s `FromStr`) is turned into a
wire frame and parsed by the same code as one received: fields are split at SOH or `|`, a data
field read by its Length field so its value may hold either, BodyLength(9) and CheckSum(10)
computed and compared with any given, counting each `|` as the SOH it replaced. A parsed string is
therefore the message `decode` gives for the same bytes. Only a malformed field differs: the
session records it as a defect to reject, while text parsing fails with it.

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

That share can be measured on Linux, per read: a spinning stream with receive timestamps
(`SpinningStream::with_receive_timestamps`) asks the kernel to stamp each packet (`SO_TIMESTAMPING`,
by the card if it can, else the kernel's own) and reads with `recvmsg`, so the wall clock at the
read less the latest packet's stamp is the time before Turbojet saw it, as
`turbojet_wire_to_read_seconds`. The stream reaches the driver only as an `AsyncRead` behind tokio's
`split`, so it leaves the time in a thread-local, which the spinning driver, running the connection
on that one thread, takes after each read. The control-message buffer is made once per stream,
so a read allocates nothing. The calls are the `nix` crate's, a dependency on Linux only, as
rustix has neither the option nor the message. Only the spinning driver: tokio's `TcpStream` has
no `recvmsg`, and its wake-up (about 11 µs between two `c9gd` machines) would dominate. On AWS the
card's stamps need a `precision-time` placement group and AWS's own ENA driver (probed
2026-11-13 on `c9gd`, issue #21's table); the kernel's ENA driver gives the PTP clock but not
packet stamps. Measured 2026-11-13 between two `c9gd.2xlarge` in a `precision-time` group (amzn-drivers
ENA 2.17.4g with `phc_enable=1`, chrony on `/dev/ptp0` at 1.1 µs RMS offset, interrupt moderation
off, `ack --spin --rx-timestamps` against `load` at 10,000 orders a second for 12 s): the kernel's
stamps gave p50 1.23 µs, p90 1.73 µs, p99 4.84 µs over 119,993 reads; after `hwstamp_ctl -r 1`
(the card's `rx_filter` was 0 until then), the card's gave 2.08 µs, 3.32 µs and 4.82 µs over
114,506 reads, its minimum 0. A Python `recvmsg` check then found both stamps on every one of 200
TCP reads. So about 0.9 µs is the card to the kernel, of a round trip of 31 µs p50 either way;
the 5% fewer samples fit reads whose card stamp was ahead of a clock synced within about 1 µs,
which aren't recorded, though fewer, larger reads would do the same.

An order sent after a lull is much slower, but a warm-up that runs the send path without sending
would win back little, so there isn't one. Measured 2026-10-09 on an M3, medians, order to
acknowledgement over localhost, split into stages by a message log:

| Spinning driver, µs | Total | Send and encode | Socket out | Acceptor | Socket back |
|---|---|---|---|---|---|
| Back to back | 14.8 | 1.2 | 8.0 | 1.1 | 4.5 |
| A real order every 500 µs | 45 | 1.3 | 24.6 | 1.0 | 17.7 |
| After 1 s idle | 197 | 21.5 | 114.6 | 15.5 | 44.7 |
| After 1 s of other threads filling the caches | 100 | 14.3 | 55.8 | 11.0 | 18.1 |

With the tokio driver it's 23 µs back to back and 599 µs after an idle second, about 120 µs of
that waking the worker. Most of the slowdown is the kernel and waking threads, which a warm-up
can't reach: even real orders every 500 µs, which warm everything a warm-up could and more, leave
the round trip at 45 µs. They bring Turbojet's own stages back to warm speed, so a warm-up could
save at most the sender's 20 µs or so, about a tenth, and only on the spinning driver, since tokio's
worker sleeps between warm-ups anyway. An idle second is slower than a second of cache filling, so
the processor's power state counts as well as cold caches. Linux with pinned cores and a locked
frequency may differ.

### Many sessions

`benches/sessions.rs` runs N sessions against one acceptor on one multi-threaded runtime, each
keeping 1,000 orders in flight, both ends storing in a shared `MemoryStorage`. On an M4 Pro (8
performance and 4 efficiency cores, 2026-10-18, `--quick`): 0.61M orders a second with 1 session,
1.22M with 2, 2.29M with 4, 3.37M with 8, then 3.44M and 3.69M with 16 and 32. Up to 4 sessions,
8 connections on 8 performance cores, each session keeps 94% of what one alone manages; it levels
off from 8, 16 connections on 12 cores. An M3 (4 + 4 cores, 2026-10-09) levelled off at 4 the
same way, at 2.12M to 2.24M.

Shared data costs little. Sampled at 8 sessions, the only cross-core waiting is in the system
allocator: 1.6% of busy time waits on its lock, of 8.2% in `malloc` and `free`. `MemoryStorage`'s
per-session lock is 3.2%, all of it uncontended (each session has its own). The rest is each
message's own work.

False sharing was measured directly, against builds that pad what each session writes from
another core: `CommandSender`'s count of queued sends (written per send), `LiveStatus` (per
message) and `MemoryStorage`'s per-session state (locked several times per message). Each is a
small allocation of its own, so two sessions' can share a cache line, which is 128 bytes on Apple
silicon. `std::sync::Mutex` keeps its `pthread_mutex_t` in a separate allocation on macOS, so
padding the state alone wouldn't move the lock: the padded build also makes the store's lock an
inline `parking_lot::Mutex`, and a third build does that alone. A fourth swaps the allocator for
mimalloc. Means of three interleaved runs (5 s each), millions of orders a second:

| Sessions | 1 | 2 | 4 | 8 | 16 | 32 |
|---|---|---|---|---|---|---|
| As is | 0.57 | 1.28 | 2.27 | 3.12 | 3.23 | 3.26 |
| Padded, lock inline | 0.57 | 1.38 | 2.36 | 3.20 | 3.32 | 3.32 |
| Lock inline only | 0.58 | 1.36 | 2.27 | 3.11 | 3.20 | 3.21 |
| mimalloc | 0.58 | 1.34 | 2.48 | 3.32 | 3.67 | 3.73 |

Padding is worth 2 to 4% from 4 sessions up, the gap between the padded build and the inline lock
alone. The inline lock's gain at 2 sessions doesn't hold from 4. The allocator is worth more than
either, 9% at 4 sessions and 14% at 16 to 32, and it's the application's choice
(`#[global_allocator]`), not the library's. Runs vary by up to 4% between rounds, so the
differences at 1 and 2 sessions are within the noise.

On Linux, where `std::sync::Mutex` is inline already, the padding was measured alone, at 128 bytes
and at 64, the line size of both machines below (2026-11-13, `scripts/aws-bench.sh` with `REVS`:
the builds in turn on one machine, three rounds each, jemalloc). Means, millions of orders a
second, unpadded, padded to 128 and to 64:

| Sessions | 1 | 2 | 4 | 8 | 16 | 32 |
|---|---|---|---|---|---|---|
| c9gd.4xlarge (Neoverse V3, 16 cores) | 0.512 / 0.520 / 0.514 | 1.03 / 1.04 / 1.03 | 2.02 / 2.05 / 2.03 | 4.00 / 4.06 / 4.02 | 4.99 / 5.00 / 4.99 | 5.21 / 5.24 / 5.22 |
| c9gd.2xlarge (Neoverse V3, 8 cores) | 0.512 / 0.519 / 0.513 | 1.03 / 1.04 / 1.03 | 2.01 / 2.05 / 2.02 | 2.70 / 2.74 / 2.72 | 2.77 / 2.81 / 2.78 | 2.83 / 2.86 / 2.83 |
| c8id.2xlarge (Xeon 6975P-C, 8 cores) | 0.395 / 0.404 / 0.404 | 0.731 / 0.741 / 0.744 | 1.24 / 1.25 / 1.25 | 1.48 / 1.49 / 1.50 | 1.51 / 1.53 / 1.53 | 1.52 / 1.54 / 1.55 |

128 bytes gains 1 to 2% on both, against round-to-round spreads of 0.2 to 1.5% (4% once, on the
8-core Graviton at 4 sessions). 64 bytes gains as much on Intel but under 0.7% on Graviton: its
cores fetch lines in pairs, as Intel's do, so a neighbour 64 bytes away still collides. The gain
from 1 session up is between the initiator's and acceptor's sides, which share the process and
the store. So the padding is 128 bytes on every target (`sync::CachePadded`, and `LiveStatus`
aligned the same), a few hundred bytes a session.

One run of the inline-lock-only build hung at 16 sessions: every thread idle, all 64 connections
open with nothing unread, a sender waiting for acknowledgements that never came. Some of its
initiators' ports were well above the rest, which suggests sessions had disconnected and
reconnected, losing the orders in flight to the reset at logon; the benchmark's application
doesn't count logouts, so it waited. Thirty runs of the unpadded build that followed didn't hang.
Nor did any on an M3 (2026-10-20): 1,000 `--quick` runs and 80 full ones, two at a time, of the
unpadded build, and 80 full ones of the inline-lock build. The benchmark now fails after 10 s
without an acknowledgement, with the session's status and its logons, logouts and admin messages.
On the M4 Pro again (2026-10-20), 100 full runs of the unpadded build and 100 of the inline lock
alone, rebuilt on it, finished too. It isn't being chased further; if it recurs, the failure says
what happened to the session.

The `load` example measures over real connections instead, against any acceptor, open loop:
order k is due at `start + k / rate`, its ClOrdID carries k, and its latency runs from its due
time to its ExecutionReport, so a stall counts against every order queued behind it (a
closed-loop tester, waiting for each answer, hides it: "coordinated omission"). One thread sends
for all sessions, sleeping until 2 ms before each order and then spinning, since a sleep
overshoots by up to a few milliseconds; the first sleep's overshoot falls in the warm-up, and the
report gives the worst lag after it. Latencies are kept and sorted for exact percentiles, bounded
at 20 M samples. It runs against the `ack` example, which answers each order with a typed
ExecutionReport and keeps nothing. Against the gateway it measures the gateway's order book too,
and its logging of every order at `info`. The book was two `HashMap`s that kept every order, so
each time one grew the copy held the book's lock and every session waited (29 ms and 92 ms p99 on
a fresh gateway's first two runs of 600,000 orders, and up to a second once it held millions).
They're `BTreeMap`s now, which grow a node at a time, holding at most a million orders and two
million ClOrdIDs, past which the gateway rejects orders, cancels and replaces: three runs of
600,000 at 60,000 a second against one gateway (`RUST_LOG=warn`, 2026-10-17, load average about
10) gave p99 51 to 53 µs and the worst order 0.19 to 0.23 ms, the book filling during the second.
Sized up front for a million orders instead, the hash maps would take hundreds of megabytes,
touched within a few hundred thousand orders since entries scatter across them. Against `ack` on
the same M3 over loopback (2026-10-17, load average about 3): a median of 30 to 34 µs from
10,000 to 100,000 orders a second on one session, p99 52 to 83 µs and the worst order 0.16 to
1.1 ms; 100,000 a second over 4 or 16 sessions, 40 µs and p99 75 µs, with the sender within
70 µs of schedule.

### On Linux

`scripts/aws-bench.sh` measures on two AWS machines in one cluster placement group: the `ack`
acceptor on one and `load` on the other, then the criterion benchmarks on the second. The guide's
performance chapter has the results, on jemalloc. Measured 2026-11-12 from `b2ddc50` to
`a598624`, on `c9gd.2xlarge` (Graviton5) and `c8id.2xlarge` (Xeon 6975P), 8 vCPUs each, one run
of each but `c9gd`'s round trips, measured in six placements:

- Interrupt moderation, not the engine, set the round trip between machines. With ENA's defaults
  (adaptive, 20 µs receive, 64 µs send), `c9gd`'s p50 was 47 µs at 1,000 orders a second but
  220 µs at 10,000 and 345 µs at 100,000: the card waited for more packets before interrupting.
  Off, it was flat across the rates, to 200,000 a second over 8 sessions: 32 to 37 µs in four
  placements, 39 to 42 and 45 to 50 µs in the others, so where the machines land moves it more
  than the load.
- `c8id`'s round trips were about three times `c9gd`'s, 102 to 144 µs p50 with moderation off,
  and its tail longer (p99.99 3.6 ms at 8 × 200,000), as turbofan's runs found on the same type.
- glibc's malloc, not the cores, held back messages in flight. A message sent through a
  `SessionHandle` is allocated on the sender's thread and freed on a worker's, and a profile of the
  round trips on `c9gd` spent 5% in the locks glibc takes for that (`_lll_lock_wait_private` from
  `free` in `apply_command`, and from `malloc` in `SessionHandle::send`, which builds each typed
  message in a new `Message` of about 1 KB). On jemalloc, on the same machine: 1,000 in flight
  461k to 727k msg/s, over TLS 408k to 681k, with the disk store 465k to 692k, 8 and 32 sessions
  11% and 18% more, a resend of 100,000 over TCP 12% faster and storing a message in memory 33 ns
  to 26 ns. Replying from `on_message` and the single-threaded rows didn't move (within 2%). On
  `c8id` 1,000 in flight went from 284k to 542k msg/s.
- jemalloc's tail between machines was milliseconds until the stores' indexes went into blocks:
  growing a `VecDeque` index copies it on jemalloc, where glibc remaps it (see [the stores'
  indexes](#storage)). jemalloc's background purging thread (`MALLOC_CONF=background_thread:true`)
  didn't change it. With the blocks, on `c9gd`, 8 × 200,000 had p99.99 77 µs and max 254 µs on
  jemalloc, against 743 µs and 1.1 ms on glibc on the same machines.
- Over localhost, single-threaded work is a little slower than on the M4 Pro (order → ack wire to
  wire 1.26 µs against 825 ns), a round trip from `on_message` a little faster (8.7 µs against
  9.6 µs), and 1,000 in flight still below it (710k msg/s against 1.06M). Many sessions level off
  at 2.84M msg/s from 8 sessions on `c9gd`, 1.52M on `c8id`: the 8 vCPUs run out, and a profile
  of them is 70% Turbojet's own work.
- A synced commit on the local NVMe takes 51 µs on `c9gd` and 138 µs on `c8id`, against 3.9 ms on
  the M4 Pro's SSD, so the fsync round trip is 81 µs instead of 4.4 ms.
- Graviton's `-C target-feature=+lse,+crc` (the script's default there) is worth keeping. Built
  without it, the median benchmark was 1% slower, within the noise, but those with many messages
  in flight 5 to 10% (1,000 in flight over TLS 9.5%, 32 sessions 5.6%): LSE puts the atomics of
  tokio's wake-ups and channels inline rather than behind a call that checks the CPU. Turbojet
  computes no CRCs; FIX's CheckSum is a byte sum. One run of each, so a single row's change of
  under 10% may be noise.

Measured 2026-11-13 at `fbb73f6` and `93eec91` (issue #19; the same engine and examples, the later
script pinning correctly), `c9gd.2xlarge`, jemalloc, moderation off, one
session at 1,000, 10,000 and 100,000 orders a second, in four runs on four placements: two
unpinned, one pinned, one pinned with isolated cores. `ack --spin` and `load --spin` put each
session on a thread named `spin-N`; the script's `PIN=1` puts interrupts on CPU 0, the threads that
spin (`spin-N`, `load`'s `send`) on CPUs 7 and 6, and the rest on 1 to 3. `RUNS=floor` is sockperf's
TCP ping-pong between the same machines.

- sockperf's round trip with blocking reads was 21.3 to 22.0 µs p50 in the three runs that had it,
  and the spinning driver's 22.7 to 25.2 µs. That isn't like for like: a blocking read pays a
  wake-up each way, which spinning doesn't. Read as the spinning driver reads, from non-blocking
  sockets in a loop (`--nonblocked`), sockperf took 17.8 to 18.2 µs (two later placements, see
  issue #46 below), so Turbojet and its benchmark add 5 to 7 µs. tokio's driver adds 11 µs on top
  of the spinning driver: the worker's wake-up on each side.
- With the kernel busy-polling (busy_read 50 µs), sockperf fell to 16.3 µs, since its blocking
  `recv` polls the NIC itself instead of sleeping until the interrupt. The spinning driver gained
  1 to 2 µs at most from the same setting (its non-blocking read polls the NIC once per call).
  Keeping the interrupt off while it polls didn't close the rest: `SO_PREFER_BUSY_POLL` on both
  ends' sockets, with the NICs' `napi_defer_hard_irqs` 2 and `gro_flush_timeout` 200 µs, measured
  in two more placements (from a `--prefer-busy-poll` flag on the examples, parked on branch
  `prefer-busy-poll`), was within the noise of busy polling alone: p50 21.1 to 24.8 µs against
  21.2 to 24.4, p99 24.7 to 29.4 against 25.5 to 29.2, sockperf's busy-polled floor 16.3 to
  16.5 µs in both. So the 5 to 8 µs between the spinning driver and sockperf isn't interrupt
  delivery; non-blocking sockperf (below) shows it isn't the way of reading either, so it's
  Turbojet's own. The socket option needs
  `CAP_NET_ADMIN` (EPERM otherwise), and the NIC settings without busy polling are much worse:
  the interrupt stays off waiting for a poller, and packets wait for the 200 µs timer (p50 235 to
  267 µs at 10,000 and 100,000 a second, in both placements). tokio's driver gained 6 or 7 µs p50 at 100,000 a second (epoll busy-polls when it would
  sleep) and nothing at the lower rates, where a 50 µs poll rarely catches the next packet.
- Pinning set the tail, not the median. Unpinned, the spinning driver's p99 was 55 to 76 µs and
  tokio's 66 to 91 µs, alike in both placements, and sockperf's own 47 µs: the kernel alone shows
  it. Pinned, 31 to 35 µs and 42 to 47 µs, and sockperf 31 µs. The guide's earlier rows (tokio,
  unpinned, at `a598624`) had p99 38 to 43 µs; why today's unpinned runs were longer isn't known.
- Isolating CPUs 4 to 7 (`isolcpus=nohz,domain`, `nohz_full`, `rcu_nocbs`) on top of pinning
  changed nothing beyond the noise, in p50, p99 or p99.9.
- OpenOnload wasn't measured: its AF_XDP mode, the only one for non-Solarflare cards, fails to
  create a stack on ENA (Xilinx-CNS/onload#337, ENA 2.17.2, Onload master of July 2026: every socket
  falls back to the kernel), and EC2 has no Solarflare cards.

Then the floor without the kernel (issue #46), 2026-11-13 at `f45e99c` to `c86928a`, pinned,
moderation off. `RUNS=dpdk` gives each machine a second ENA interface, binds it to `vfio-pci`
without an IOMMU (EC2 exposes none) and runs `scripts/dpdk-pingpong.c`, a UDP ping-pong of
200-byte datagrams polled from one core on each side, on Ubuntu's DPDK packages. ENA's
low-latency transmit queue was on in every run that logged it (`Placement policy: Low latency`).

First, sockperf on the kernel, on `c9gd.2xlarge` in five placements (µs):

| | Placements | Range of p50 | Range of p99 |
|---|---:|---:|---:|
| TCP, blocking | 5 | 20.9 to 21.9 | 25.0 to 26.1 |
| TCP, blocking, busy poll 50 µs | 5 | 15.9 to 16.7 | 19.9 to 21.0 |
| TCP, non-blocking loop | 2 | 17.8 to 18.2 | 21.7 to 22.3 |
| TCP, non-blocking loop, busy poll 50 µs | 2 | 16.2 to 16.3 | 20.3 to 21.0 |
| UDP, blocking | 5 | 20.4 to 21.6 | 23.5 to 24.7 |
| UDP, blocking, busy poll 50 µs | 5 | 15.6 to 16.4 | 18.3 to 19.2 |

- A spinning reader gets all of busy polling's gain: non-blocking sockperf went from 18 to 16 µs,
  like blocking sockperf's 21 to 16. The spinning driver's 1 to 2 µs from it is the same gain.
- TCP costs the kernel about 0.3 µs over UDP.

Then DPDK against the busy-polled kernel UDP socket in the same placement, since placements
differ by more than the drivers do (one `c9gd` placement's kernel round trip was 27.9 µs, one
`c8id` placement's 18.6 µs, others' 31 to 32). DPDK's p50 less the kernel's, µs, and DPDK's p50:

| | Placements | DPDK less the kernel | DPDK p50 |
|---|---:|---:|---:|
| `c9gd`, stock `vfio-pci` | 3 | -1.7, -3.0, -1.5 | 14.7, 13.2, 14.4 |
| `c9gd`, write-combined | 3 | -3.4, -3.0, -3.8 | 12.7, 12.7, 24.1 |
| `c8id`, stock `vfio-pci` | 2 | +3.2, +2.7 | 35.3, 33.5 |
| `c8id`, write-combined in the user's mapping only | 3 | +3.6, +3.2, +1.8 | 28.5, 34.4, 31.9 |
| `c8id`, write-combined in both | 2 | -4.0, -4.5 | 14.6, 27.4 |

- Write-combining decides it. ENA's low-latency queue writes each descriptor into device memory,
  which the stock `vfio-pci` maps uncached; the kernel's own ENA driver maps it write-combined.
  `DPDK_WC=1` (the default) rebuilds `vfio-pci` from the running kernel's source with
  `scripts/vfio-wc.py`, the change AWS's amzn-drivers enav2-vfio-patch makes, by pattern since
  its patches stop at kernel 6.8. On x86 both of `vfio-pci`'s mappings of the BAR must change:
  PAT gives a later mapping of memory the type of the first, so with only the user's mapping
  changed, the BAR stayed out of `/sys/kernel/debug/x86/pat_memtype_list`'s write-combining
  ranges and DPDK stayed slower than the kernel. Arm64 has no PAT, so on `c9gd` the user's
  mapping alone was enough (the first write-combined run changed only that).
- Write-combined, bypassing the kernel saves 3.0 to 3.8 µs on `c9gd` and 4.0 to 4.5 µs on
  `c8id` over a busy-polled kernel socket, before a user-space TCP stack costs any of it back:
  about a fifth of `c9gd`'s best round trip. DPDK's fastest round trips on `c9gd` were 11.3 to
  11.6 µs, the network's own share, which no software on the machines removes.
- Turbojet's own 5 to 7 µs above a spinning kernel socket (above) is still the larger part, and
  the cheaper one to work on; no DPDK transport is built yet.

Then where those 5 to 7 µs go (issue #47), 2026-11-13 at `f0d41f4` with `e2e1bac`'s trace run, on `c9gd.2xlarge`, pinned,
moderation off, jemalloc, one session. `RUNS=trace` runs `ack --spin` and `load --spin` under
bpftrace on both machines, timing the spinning threads' `recvfrom` calls that return bytes and
their `sendto` calls, while `load --stamps` writes when each order was due, built, handed to its
session and answered. `scripts/trace-stages.py` sets the two side by side. Two things got in the
way:

- `Instant` and bpftrace's `nsecs` differed by a steady 3 to 5 µs, on `CLOCK_MONOTONIC` and
  `CLOCK_BOOTTIME` alike, so the stamps are placed by a `getpid` call the trace also sees. Of 100
  calls, the one with its two `Instant` readings closest (about 0.26 µs apart; the first calls
  took 8 µs) places them to within 0.13 µs.
- A probe that prints costs about 1 µs. A read's time is kept and printed at the next write, so
  that the time from a read to what follows it carries no printing. The round trip still grows by
  about 3 µs under the trace, but in the socket's share, not in the stages below.

Each stage's p50, µs, in one placement (two more agreed to within 0.2 µs on `ack` and 0.4 µs on
building the order):

| Stage | 1,000/s | 20,000/s |
|---|---:|---:|
| `load` builds the order, an untyped `Message` with `format!` and a timestamp | 0.6 | 0.4 |
| `load`'s sender hands it over, `SessionHandle::send` | 0.3 | 0.4 |
| from built to its `sendto`: the hand-over, the spinning thread taking it, the session stamping and encoding it | 1.3 | 1.3 |
| `ack`, from the order's `recvfrom` to the report's `sendto` | 2.2 | 2.0 |
| `load`, from the report's `recvfrom` to `on_message` | 0.8 | 0.8 |
| Turbojet and the benchmark, together | 4.9 | 4.5 |
| Untraced round trip; sockperf's non-blocking TCP loop 18.4 to 18.8 in the same placements | 23.9 to 24.6 | 22.6 to 23.0 |

- The gap is Turbojet's and the benchmark's own time, about 4.5 to 5 µs, and at 1,000/s about
  0.5 µs more in the kernel and network, which sockperf's back-to-back messages keep warm. At
  20,000/s the socket's share matches sockperf's.
- `ack`'s 2.0 to 2.2 µs in place is 0.7 to 0.9 µs over the session's own 1.3 µs on the same
  machine (`session/order to ack, wire to wire`, criterion): the driver's work around each
  message, its two system calls' entry and exit, and caches not kept warm by a tight loop.
- Sending costs `load` more than receiving: 1.3 µs from a built order to its `sendto`, against
  0.8 µs from a report's `recvfrom` to `on_message`. The spinning thread mostly writes an order
  before the sender's `send()` has returned.
- No stage alone is most of it. What's left to cut is spread over the driver's loop around the
  session (about 0.8 µs a message on each side), the send path (about 1 µs), and the benchmark's
  untyped order (0.4 to 0.6 µs), which a real application building typed messages pays in its
  own way.

### Current numbers

Run with `cargo bench -p turbojet --all-features` (about 13 minutes); `cargo bench -p turbojet-sql
--all-features` for the SQL stores, with PostgreSQL at `TURBOJET_POSTGRES_URL`. These are the
M4 Pro's; the guide's performance chapter has Linux servers' (see [On Linux](#on-linux)), and the
README a selection of those.

Apple M4 Pro (8 performance and 4 efficiency cores), macOS 27, Rust 1.98.1; criterion medians.
Benchmarks build with one codegen unit and fat LTO (`[profile.bench]`), so results don't shift with
how the compiler happens to partition the crate.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 163 ns / 136 ns | 991 / 1,185 MiB/s |
| Encode NewOrderSingle (169 B) | 40 ns | 4.0 GiB/s |
| Encode ExecutionReport (209 B) | 45 ns | 4.3 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 26 ns / 41 ns | |
| Typed parse NewOrderSingle, owned (no groups / with 3 allocations) | 113 ns / 175 ns | |
| Typed parse FIX 4.4 NewOrderSingle with nested groups (363 B): borrowed / reading every entry / owned | 85 ns / 560 ns / 470 ns | |
| Typed build ExecutionReport | 137 ns | |
| Format a timestamp (same second / new second) | 9.0 ns / 27 ns | |
| Session: order → ack, no I/O, encoded reply (memory store) | 675 ns | 1.48M msg/s |
| Session: order → ack, wire to wire (decode + session, which encodes) | 825 ns | 1.21M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 12 ns / 1.4 µs / 3.9 ms | |
| Store a sent message, 100 per commit: disk / disk + fsync | 37 ns / 40 µs | |
| Store a sent message and commit it: SQLite / SQLite synced / PostgreSQL | 66 µs / 4.0 ms / 140 µs | |
| Store a sent message, 100 per commit: SQLite / SQLite synced / PostgreSQL | 2.5 µs / 49 µs / 7.1 µs | |
| Read a resend step of 256 back: disk / SQLite / PostgreSQL | 11.6 µs / 179 µs / 146 µs | |
| SBE: encode a B3 NewOrderSingle (113 B) / tag=value NewOrderSingle (169 B), reused buffer | 6.7 ns / 30 ns | |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 4.8 ns / 181 ns | |
| FAST: decode a market data refresh of 1 / 10 / 50 entries into a reused message | 197 ns / 1.34 µs / 6.43 µs | |
| FAST: encode the same refreshes | 278 ns / 1.88 µs / 8.21 µs | |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 16.1 µs | 3.97M msg/s |
| Round trip over localhost TCP, one at a time | 30.8 µs | 32.5k/s |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 9.6 µs | 104k/s |
| The same, both ends spinning (`run_spinning`) | 8.7 µs | 115k/s |
| Round trip over localhost TCP, 1,000 in flight | | 1.06M msg/s |
| Round trip over localhost TLS, one at a time | 31.3 µs | 31.9k/s |
| Round trip over localhost TLS, one at a time, replying from `on_message` | 21.2 µs | 47.2k/s |
| Round trip over localhost TLS, 1,000 in flight | | 950k msg/s |
| Round trip, acceptor storing to disk: one at a time / 1,000 in flight | 33.5 µs | 959k msg/s |
| Round trip, acceptor storing to disk + fsync: one at a time / 100 in flight | 4.4 ms | 13.9k msg/s |

Round trips are initiator → acceptor application → initiator application, using a store that
discards messages (storage is measured separately), except the disk rows, where the acceptor
stores to `DiskStorage`. Most send each order from the benchmark's task through a `SessionHandle`;
the "replying from `on_message`" rows send the next order from the initiator's application, with
no hop between tasks (see [Performance](guide/performance.md)). Replying from `on_message`, TLS
costs 9-12 µs more than TCP on the M4 Pro (3-4 µs on an M3), against 0.2-0.7 µs for a bare
tokio-rustls ping-pong and 1.3 µs through a `SessionHandle`: mostly the macOS scheduler waking the
thread on another core, which TCP often avoids and TLS almost never does, not rustls or extra
system calls. The SBE rows use B3's Binary
Entrypoint schema; the FIXP rows carry its orders under FIXP 1.0 with recoverable flows, each end
storing in memory. The FAST rows use the FAST specification's own incremental refresh template,
each message following the one before. The SQL rows used PostgreSQL 18 on the same machine over
TCP, and SQLite synced is `synchronous = FULL` with `fullfsync`, as `DiskStorage`'s fsync is. Rows
were measured 2026-10-31, the SQL rows and the resend steps 2026-10-25;
[Measurement history](#measurement-history) says what each was before.

`tests/allocations.rs` counts heap allocations per order → ack, wire to wire, by stage, and fails if any stage's
count changes, up or down, so both regressions and improvements show up in CI:

```sh
cargo test -p turbojet --test allocations -- --nocapture   # prints the table
```

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Decode (into one message reused per connection) | 0 | 0 | 0 |
| Session (including encoding the ack) | 0 | 0 | 0 |
| Application (typed parse and ack) | 0 | 0 | 0 |
| Store: memory / disk | 1 / 0 | 0 | 216 / 0 |
| Engine (all but the application): memory / disk | 1 / 0 | 0 | 216 / 0 |

Means of 1,000 orders after 100 warm-up, with each store (the disk store without fsync). The
memory store's allocation is its copy of each message it keeps. The stores' indexes add a block
every 4,096 messages, which these 1,100 orders don't reach: 32 bytes a message on average in
memory and 24 on disk. Debug and release builds count the same. The
application parses the order borrowed, without allocating, and copies the strings it needs into
its owned ExecutionReport, whose `CompactString`s keep them inline. `Context::send` writes the
ExecutionReport into a message the session reuses, which allocates nothing once warmed up.

The same test counts a FIXP server (`turbojet::fixp`) answering each B3 NewOrderSingle with an
order, recoverable flows both ways, as the FIXP benchmark does:

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Session (framing, the session's own messages, encoding the answer) | 0 | 0 | 0 |
| Application (B3 decode and answer) | 0 | 0 | 0 |
| Store: memory / disk | 1 / 0 | 0 | 119 / 0 |

Means of 1,000 orders after 100 warm-up. The stores cost what they do for FIX: the
memory store copies each answer it keeps.

Another test counts typed parsing alone, per FIX 4.2 NewOrderSingle:

| Parse | Allocations | Reallocs | Bytes |
|---|---|---|---|
| `NewOrderSingleRef`: no groups / 3 allocations, every entry read | 0 / 0 | 0 | 0 / 0 |
| `NewOrderSingle`: no groups / 3 allocations | 0 / 1 | 0 | 0 / 144 |

The owned form allocates a `Vec` for each group, and a text value only if it's longer than 24
bytes; ClOrdID, Symbol and Account here are kept inline.

### Measurement history

[Current numbers](#current-numbers) has the latest. Every row but the SQL rows and the resend steps
was re-measured 2026-10-31 on the same machine, on `main` at c0f3dcc. A round trip replying from
`on_message` varies more from run to run than the table's other rows (9.2 to 10.0 µs over TCP in
five runs), so it and the spinning, TLS, pipelined TCP, FIXP and disk store round trips are medians
of four to nine runs. On 2026-10-25, in the table's order: decoding 163 / 134 ns, encoding 39 and
45 ns, typed parsing 85 / 137 ns borrowed and 112 / 174 ns owned, nested groups 383 / 614 /
482 ns, building 137 ns, timestamps 8.9 / 27 ns; the session 716 ns and 864 ns wire to wire;
storing 43 ns / 1.5 µs / 3.9 ms one per commit and 60 ns / 39 µs at 100; SBE 6.7 / 30 ns and
4.7 / 219 ns; FAST decoding 200 ns / 1.32 µs / 6.20 µs and encoding 297 ns / 1.90 µs / 8.28 µs;
FIXP 16.2 µs and 3.20M msg/s; TCP round trips 31.2 µs, 11.2 µs from `on_message` (9.4 to 11.6 µs
in seven runs), 9.2 µs spinning and 1.03M msg/s; TLS 31.8 µs, 21.3 µs and 962k msg/s; the disk
store 33.4 µs and 885k msg/s, and with fsync 3.9 ms and 18.3k msg/s. The borrowed parses are the
lazy form, which reads a field only when asked; the 2026-10-25 rows measured the eager struct it
replaced, and the tag=value SBE row parses borrowed too. Storing in memory and on disk at 100 per
commit are faster for the ring-buffer index (see [the stores' indexes](#storage)), which came
after that day's measurement, and the session's order → ack 4-6%, partly with it. FIXP pipelining is 24%
faster (3.83M to 4.00M msg/s in five runs), and the disk store's pipelined round trip 8% (955k to
966k in four runs, against 885k in each of three). With fsync, a round trip one at a time took
4.4 ms and pipelining 100 managed 24% less (11.2k to 14.6k msg/s in four runs), though a commit
alone still took 3.9 ms: that's the device.

On 2026-10-18, in the table's order: decoding 162 / 133 ns, encoding 40 and 46 ns, typed parsing 87
/ 137 ns borrowed and 111 / 174 ns owned, nested groups 398 / 616 / 484 ns, building 134 ns,
timestamps 9.2 / 27 ns; the session 739 ns and 866 ns wire to wire; storing 43 ns / 1.4 µs / 4.0 ms
one per commit and 59 ns / 40 µs at 100, SQLite / synced / PostgreSQL 64 µs / 4.0 ms / 138 µs and
2.5 / 48 / 6.4 µs, resend steps 11.7 / 179 / 148 µs; SBE 6.7 / 30 ns and 4.8 / 222 ns; FAST decoding
216 ns / 1.34 µs / 6.27 µs and encoding 301 ns / 2.01 µs / 8.65 µs; FIXP 16.2 µs and 3.33M msg/s;
TCP round trips 31.1 µs, 10.7 µs from `on_message`, 9.0 µs spinning and 1.04M msg/s; TLS 31.3 µs,
21.2 µs and 958k msg/s; the disk store 32.8 µs and 913k msg/s, and with fsync 4.0 ms and 16.7k
msg/s. On 2026-10-25 most rows were within noise of those. FAST was 2-7% faster, the session's order
→ ack 3%, and the disk store's pipelined round trip was 3% slower (885k msg/s in each of three
runs); with fsync, pipelining managed 10% more, which is the device.

Before that, every row was re-measured 2026-10-18 on an Apple M4 Pro (8 performance and 4
efficiency cores), the SQL rows against PostgreSQL 18. On the M3 (4 + 4) before it, in the table's order: decoding 197 / 164 ns, encoding 47 and 54 ns, typed
parsing 103 / 162 ns borrowed and 130 / 205 ns owned, nested groups 496 / 783 / 609 ns, building
171 ns, timestamps 11 / 33 ns; the session 755 ns and 939 ns wire to wire; storing 48 ns /
1.7 µs / 4.0 ms one per commit and 70 ns / 41 µs at 100, SQLite / synced / PostgreSQL 14 75 µs /
4.5 ms / 112 µs and 3.0 / 55 / 7.1 µs, resend steps 15.2 / 196 / 149 µs; SBE 7.5 / 37 ns and
4.8 / 258 ns; FAST decoding 243 ns / 1.75 µs / 8.43 µs and encoding 377 ns / 2.40 µs / 10.4 µs;
FIXP 15.6 µs and 2.97M msg/s; TCP round trips 27.7 µs, 16.3 µs from `on_message` (about 13 µs of
it the operating system), 10.3 µs spinning and 532k msg/s; TLS 27.8 µs, 20.4 µs and 508k msg/s;
the disk store 31.3 µs and 715k msg/s, and with fsync 4.1 ms and 13.4k msg/s. The M4 Pro is
as fast or faster at everything measured on one thread and at pipelining, but round trips one at
a time through a `SessionHandle` are slower: 31.1 µs against 27.7 over TCP, 31.3 against 27.8
over TLS, 32.8 against 31.3 storing to disk. Replying from `on_message` over TCP, with no hop between tasks,
is faster (10.7 µs against 16.3), so the hop costs more here; over TLS (21.2 against 20.4) and
FIXP (16.2 against 15.6) it's a little slower. A PostgreSQL commit took 138 µs against 112, a
different version of PostgreSQL. When each row was last measured on the M3, and what it was before:

- Decoding into a reused message, building an ExecutionReport and formatting timestamps were
  re-measured 2026-09-30, after the session started encoding what it sends straight into its
  output, decimals and integers were written without `core::fmt`, and each inbound message was
  decoded into one reused for the connection.
- Borrowed typed parsing was measured 2026-10-01, when typed messages gained borrowed forms.
- Owned typed parsing was re-measured 2026-10-03, after owned messages were parsed straight into
  their owned form: 166 ns, 335 ns and 1.04 µs just before, on the same day.
- Typed parsing and building were re-measured 2026-10-11, on a loaded machine, after enumerated
  fields became `Code<E>`. With strict fields, the same day: 93 ns / 166 ns borrowed, 120 ns /
  199 ns owned, 468 ns / 769 ns / 551 ns with nested groups, and 155 ns to build. The table had
  98 / 173, 148 / 273, 448 / 722 / 692 and 156.
- Typed parsing was re-measured 2026-10-15, after decimals were parsed in one pass: 110 / 181,
  136 / 220 and 503 / 787 / 615 just before.
- Session order → ack was re-measured 2026-10-03, after the application's replies were built in
  messages the session reuses: 945 ns and 1.20 µs just before.
- Disk stores were measured 2026-10-02 with group commit: before it, storing 100 messages with
  fsync took 837 ms, and the disk + fsync round trip managed 62 messages a second with 100 in
  flight. With each commit recording the next in-flight window it took 12.2 ms one at a time with
  fsync. They were re-measured 2026-10-03, after a commit took one write and one `fsync` rather
  than one of each file; just before, on the same day: 3.2 µs and 8.0 ms one per commit, 84 ns and
  81 µs at 100 per commit, and round trips of 33.2 µs, 689k msg/s, 8.1 ms and 9.5k msg/s.
- The SQL stores were measured 2026-10-03, PostgreSQL 14 on the same machine over TCP. The disk
  store's resend step was re-measured 2026-10-08, after its messages were read in one go rather
  than one read each: 157 µs before.
- Round trips replying from `on_message` were measured 2026-10-04; through the benchmark's task
  they took 26.5 µs (TCP) and 28.3 µs (TLS) the same day. Spinning was measured the same day,
  16.45 µs without spinning in the same run.
- SBE was measured 2026-10-05 and FIXP 2026-10-06; the FIX round trips in the FIXP run took
  16.2 µs and 0.81M msg/s.
- The `load` runs moved from the gateway to the `ack` example 2026-10-17. Against the gateway
  (`RUST_LOG=warn`, 2026-10-09): a median of about 33 to 40 µs from 10,000 to 50,000 a second,
  and 100,000 over 16 sessions saturated the machine. Python's, against the gateway at `info`
  the same day, after events already waiting skipped the runtime, p50 / p99: 471 / 649 µs at
  1,000 a second, 103 / 131 µs at 10,000, 118 / 235 µs at 30,000 and 122 / 254 µs at 60,000,
  with Rust's 88 / 117, 42 / 85, 43 / 110 and 43 / 69.
- The Python round trips were re-measured 2026-10-17, after events already waiting skipped the
  runtime. Earlier that day, p50 / p99: 559 µs / 1,037 µs at 1,000 a second, 86 / 107 µs at
  10,000, 88 / 140 µs at 15,000, 117 µs / 11 ms at 20,000, and answers queued for seconds at
  30,000.
- The stores' indexes went into blocks of 4,096 2026-11-12. Before, the allocation budget had 4
  reallocs per 1,000 orders in the store (the ring buffer doubling), and bytes per order were
  278 / 46 (memory / disk) for FIX and 180 / 46 for FIXP. On an Apple M3, two interleaved rounds:
  storing a message in memory 13.4 ns to 14.0 ns, 100 per commit on disk 41.4 ns to 41.9 ns,
  order → ack with the memory store 775 ns to 779 ns, and resends of 100,000 unchanged.
- The stores' indexes became ring buffers 2026-10-25, measured on an Apple M3, not the
  table's M4 Pro: storing a message in memory 48.4 ns to 13.6 ns, 100 per commit on disk
  69.9 ns to 40.2 ns, and order → ack with the memory store 813 ns to 784 ns. The table's store
  rows (43 and 60 ns) are the M4 Pro's from before the change, until they're re-measured there.
  The commit that made the change (eddd24c) says M4 Pro by mistake. On the M3, 1,000 orders in
  flight on each of 1 to 32 sessions (`benches/sessions.rs`, `--quick`, means of two
  interleaved runs) rose about 5% at every count: 0.52 / 1.21 / 1.85 / 2.15 / 2.09 / 2.16 to
  0.55 / 1.23 / 1.95 / 2.24 / 2.23 / 2.26 M orders a second. The allocation counts changed with it: 1.17 / 0.17 allocations per order with the memory / disk
  store, 280 / 49 bytes, and no reallocs, for FIX; 1.17 / 0.17 and 182 / 49 for FIXP.
- The allocation counts were taken 2026-09-30; the application's were re-counted 2026-10-03,
  before which the reply list and message cost 3 allocations more, and `String` fields in place
  of `CompactString`s cost 5.
- The other rows are the 2026-09-27 snapshot.
