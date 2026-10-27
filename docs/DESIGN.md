# Design

How Turbojet is built and why, what's been measured along the way, and what testing found.
[ARCHITECTURE.md](ARCHITECTURE.md) is the overview, with diagrams, [the guide](guide/README.md)
says how to use it, [ROADMAP.md](ROADMAP.md) what's still to do, and
[CAVEATS.md](CAVEATS.md) the behaviour worth knowing about. The house style, safety first, then
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
- **The stores' indexes.** Both built-in stores find the messages they keep by MsgSeqNum in a ring
  buffer (`VecDeque`) in sequence order, not a `BTreeMap`. The numbers stored only increase, so a
  message goes on the back and eviction takes from the front, and a resend finds its range by
  binary search. A `BTreeMap` allocated a node every six messages or so, and its insert was most
  of a memory store's: storing a message took 48 ns, now 14 ns (Apple M3, 2026-10-25, criterion
  medians of two interleaved rounds each), and 70 ns to 40 ns in the disk store at 100 per
  commit; an order → ack through the session with the memory store 813 ns to 784 ns. The cost is
  growth: the buffer doubles when full, and once eviction has wrapped it around, growing moves
  part of it in one go: 0.7 ms at a million messages kept and 2.7 ms at four million, against
  0.04 ms at worst for a `BTreeMap` insert. That happens only when a full store keeps more
  messages than it ever has, such as when they get smaller under a byte budget. Growing one
  that hasn't wrapped is cheap (0.1 ms at five million), since `realloc` of a large block remaps
  its pages.
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
  - Divergence is checked, not trusted away. A batch carries the session's commit number and its
    records must follow the standby's copy (an outgoing number not yet used, a stored message
    the next), or the standby refuses it. When a session opens, the active compares the
    standby's summary of it (numbers, in-flight marker, created time, marks) with its own and,
    if they differ, sends it whole, in batches of a megabyte of messages: `MemoryStorage` keeps
    changes never committed, so a log dropped without a commit leaves the standby behind.
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
    sends one, and marks the lease in sync once none is behind. A new holder leaves the mark as
    it is, so after a forced failover the first catch-up counts every session behind. The
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
    driving its I/O and waking it.
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
    has to be known at promotion, on the standby: each Bind carries the active's bound (wire
    version 2: a version 1 peer and this one refuse each other at hello) and a standby accepting less
    (`accepting_lag`) refuses it, failing the opening with `InvalidInput`. Promotion lists the
    standby store's sessions (`SessionStorage::sessions`) to skip each. Incoming state isn't
    bounded the same way: the new active may expect a lower incoming number than the old one
    reached, and the counterparty's resend hands the application PossDup duplicates of messages
    already handed over.
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
    the link's per-batch cost (a frame, a write and an answer each, applied in turn) caps it near
    450,000 commits a second. With fsync, replicated, 6.9 ms, 7.3 ms and 6.1 ms; `DiskStorage`
    alone, 4.03 ms, 2.36 ms and 3.43 ms, since concurrent `F_FULLFSYNC`s barely overlap. Not
    measured on Linux, where `fdatasync` is per file and journals merge concurrent ones.
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
QuickFIX-format additions onto either, keeping the official names and documentation where the venue
re-lists something. `turbojet-codegen` turns a dictionary into `fix_enum!`, `fix_group!` and
`fix_message!` invocations (copying the dictionary's documentation into doc comments only on
request), from an application's `build.rs` or as a command for checked-in crates.

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
  power loss on disk stores with and without sync, logout requests, operators skipping numbers ahead
  and resetting both sides, daily schedules, and on a third of the seeds sessions with SubIDs and a
  LocationID. After every event a checker holds both sides to what their stores recorded, what they
  wrote and what their applications received, and once the faults stop they must settle; bugs
  planted in the simulator show the checker catches what it should. 100 seeds run on every push and
  random ones nightly, and any failure replays from its seed (`scripts/sim.sh`). It found a write
  deadlock in the connection driver, torn sequence-number records in `DiskStorage`, and an
  operator's skip ahead making a counterparty abandon a gap, all since fixed. It exercises outbound
  limits and inbound `Delay`, but not `Reject`, since its checker can't cheaply model a
  BusinessMessageReject in place of a delivery, and no `MessageLog`; session and connection tests
  cover both.
- **Concurrency tests** with [shuttle](https://github.com/awslabs/shuttle), on every push. Built
  with `--cfg shuttle`, the registry's and the holds' locks and atomics are shuttle's
  (`src/sync.rs`), and the tests run each protocol threads share in thousands of interleavings:
  a hold released on another thread is either seen or wakes the session; two Logons at once bind
  the session once; a pause racing a Logon either refuses it or finds it connected, to log it
  out; a send queued before a logout goes out first. A bug planted in each shows its test
  catches it. What shuttle doesn't model, and these tests can't find: orderings weaker than
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
  (`NewOrderSingleRef`) whose strings, lists and groups point into the message, so typed parsing
  allocates nothing.
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
judged size to matter less than it does. Building it in place with `MaybeUninit`
isn't worth `unsafe`: Rust can't construct in the caller's slot, so it would only save the copy
behind an out-parameter API. Shrinking every enumerated field from 24 bytes to 16 (boxing
`Code::Unknown`'s string) saved 1 to 4%. Comparing MsgType(35)'s raw code in `Message::parse`,
rather than building a `MsgType` to compare, saved nothing: finding the field is the cost.

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
differences at 1 and 2 sessions are within the noise. The padding hasn't been adopted.

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

### Current numbers

Run with `cargo bench -p turbojet --all-features` (about 13 minutes); `cargo bench -p turbojet-sql
--all-features` for the SQL stores, with PostgreSQL at `TURBOJET_POSTGRES_URL`. The README shows a
selection of these rows.

Apple M4 Pro (8 performance and 4 efficiency cores), macOS 27, Rust 1.98.1; criterion medians.
Benchmarks build with one codegen unit and fat LTO (`[profile.bench]`), so results don't shift with
how the compiler happens to partition the crate.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 163 ns / 134 ns | 987 / 1,203 MiB/s |
| Encode NewOrderSingle (169 B) | 39 ns | 4.0 GiB/s |
| Encode ExecutionReport (209 B) | 45 ns | 4.3 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 85 ns / 137 ns | |
| Typed parse NewOrderSingle, owned (no groups / with 3 allocations) | 112 ns / 174 ns | |
| Typed parse FIX 4.4 NewOrderSingle with nested groups (363 B): borrowed / reading every entry / owned | 383 ns / 614 ns / 482 ns | |
| Typed build ExecutionReport | 137 ns | |
| Format a timestamp (same second / new second) | 8.9 ns / 27 ns | |
| Session: order → ack, no I/O, encoded reply (memory store) | 716 ns | 1.40M msg/s |
| Session: order → ack, wire to wire (decode + session, which encodes) | 864 ns | 1.16M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 43 ns / 1.5 µs / 3.9 ms | |
| Store a sent message, 100 per commit: disk / disk + fsync | 60 ns / 39 µs | |
| Store a sent message and commit it: SQLite / SQLite synced / PostgreSQL | 66 µs / 4.0 ms / 140 µs | |
| Store a sent message, 100 per commit: SQLite / SQLite synced / PostgreSQL | 2.5 µs / 49 µs / 7.1 µs | |
| Read a resend step of 256 back: disk / SQLite / PostgreSQL | 11.6 µs / 179 µs / 146 µs | |
| SBE: encode a B3 NewOrderSingle (113 B) / tag=value NewOrderSingle (169 B), reused buffer | 6.7 ns / 30 ns | |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 4.7 ns / 219 ns | |
| FAST: decode a market data refresh of 1 / 10 / 50 entries into a reused message | 200 ns / 1.32 µs / 6.20 µs | |
| FAST: encode the same refreshes | 297 ns / 1.90 µs / 8.28 µs | |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 16.2 µs | 3.20M msg/s |
| Round trip over localhost TCP, one at a time | 31.2 µs | 32.1k/s |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 11.2 µs | 89.3k/s |
| The same, both ends spinning (`run_spinning`) | 9.2 µs | 109k/s |
| Round trip over localhost TCP, 1,000 in flight | | 1.03M msg/s |
| Round trip over localhost TLS, one at a time | 31.8 µs | 31.4k/s |
| Round trip over localhost TLS, one at a time, replying from `on_message` | 21.3 µs | 46.9k/s |
| Round trip over localhost TLS, 1,000 in flight | | 962k msg/s |
| Round trip, acceptor storing to disk: one at a time / 1,000 in flight | 33.4 µs | 885k msg/s |
| Round trip, acceptor storing to disk + fsync: one at a time / 100 in flight | 3.9 ms | 18.3k msg/s |

Round trips are initiator → acceptor application → initiator application, using a store that
discards messages (storage is measured separately), except the disk rows, where the acceptor
stores to `DiskStorage`. Most send each order from the benchmark's task through a `SessionHandle`;
the "replying from `on_message`" rows send the next order from the initiator's application, with
no hop between tasks (see [Performance](guide/performance.md)). The SBE rows use B3's Binary
Entrypoint schema; the FIXP rows carry its orders under FIXP 1.0 with recoverable flows, each end
storing in memory. The FAST rows use the FAST specification's own incremental refresh template,
each message following the one before. The SQL rows used PostgreSQL 18 on the same machine over
TCP, and SQLite synced is `synchronous = FULL` with `fullfsync`, as `DiskStorage`'s fsync is. Rows
were measured 2026-10-25;
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
| Store: memory / disk | 1 / 0 | 0.004 | 278 / 46 |
| Engine (all but the application): memory / disk | 1 / 0 | 0.004 | 278 / 46 |

Means of 1,000 orders after 100 warm-up, with each store (the disk store without fsync). The
memory store's allocation is its copy of each message it keeps; the reallocs are the stores'
indexes doubling as they grow, 4 in 1,000 orders. Debug and release builds count the same. The
application parses the order borrowed, without allocating, and copies the strings it needs into
its owned ExecutionReport, whose `CompactString`s keep them inline. `Context::send` writes the
ExecutionReport into a message the session reuses, which allocates nothing once warmed up.

The same test counts a FIXP server (`turbojet::fixp`) answering each B3 NewOrderSingle with an
order, recoverable flows both ways, as the FIXP benchmark does:

| Stage | Allocations | Reallocs | Bytes |
|---|---|---|---|
| Session (framing, the session's own messages, encoding the answer) | 0 | 0 | 0 |
| Application (B3 decode and answer) | 0 | 0 | 0 |
| Store: memory / disk | 1 / 0 | 0.004 | 180 / 46 |

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

[Current numbers](#current-numbers) has the latest. Every row was re-measured 2026-10-25 on the
same machine, the SQL rows against PostgreSQL 18.6. A round trip replying from `on_message` varies
more from run to run than the table's other rows (9.4 to 11.6 µs over TCP in seven runs), so it and
the spinning, one-at-a-time TLS and pipelined TCP and FIXP rows are medians of three to seven runs. On 2026-10-18,
in the table's order: decoding 162 / 133 ns, encoding 40 and 46 ns, typed parsing 87 / 137 ns
borrowed and 111 / 174 ns owned, nested groups 398 / 616 / 484 ns, building 134 ns, timestamps
9.2 / 27 ns; the session 739 ns and 866 ns wire to wire; storing 43 ns / 1.4 µs / 4.0 ms one per
commit and 59 ns / 40 µs at 100, SQLite / synced / PostgreSQL 64 µs / 4.0 ms / 138 µs and
2.5 / 48 / 6.4 µs, resend steps 11.7 / 179 / 148 µs; SBE 6.7 / 30 ns and 4.8 / 222 ns; FAST
decoding 216 ns / 1.34 µs / 6.27 µs and encoding 301 ns / 2.01 µs / 8.65 µs; FIXP 16.2 µs and
3.33M msg/s; TCP round trips 31.1 µs, 10.7 µs from `on_message`, 9.0 µs spinning and 1.04M msg/s;
TLS 31.3 µs, 21.2 µs and 958k msg/s; the disk store 32.8 µs and 913k msg/s, and with fsync 4.0 ms
and 16.7k msg/s. Most rows are within noise of then. FAST is 2-7% faster, the session's order → ack
3%, and the disk store's pipelined round trip 3% slower (885k msg/s in each of three runs); with
fsync, pipelining managed 10% more, which is the device.

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
