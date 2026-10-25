# Known caveats

Behaviour that is deliberate or documented, but worth knowing about before you rely on Turbojet,
and worth revisiting. Planned work is in [ROADMAP.md](ROADMAP.md).

- FIX 4.0 and 4.1 sessions always write SendingTime in whole seconds, ignoring
  `timestamp_precision`, since their UTCTimestamp has no fractions (they came in FIX 4.2); the typed crates' other timestamps are written as the application sets
  them. Two FIX 4.0 repetitions have no count field that a dictionary could make a group of, so
  validation rejects them as repeated tags and typed parsing keeps the first: News and Email's
  RelatdSym(46), which may repeat, and Allocation's delivery instructions, BrokerOfCredit(92) and
  DlvyInst(86) repeated NoDlvyInst(85) times inside each NoAllocs entry.
- FAST (`turbojet-fast`): reportable errors are always reported, so an encoder that writes an
  overlong integer or presence map is refused rather than read; a delta string's part is read as
  the specification has it, not nullable, so an encoder that writes it nullable, as `fastlib` does
  for optional fields, gives a stray NUL character. A field without an `id` is decoded but left out
  of the message. Encoding needs the message's fields
  in the template's order, and doesn't encode dynamic template references. A decoder that hits an
  error can't know the encoder's previous values any more: reset it where the feed next resets.
- Python bindings: an `AppMessage` that's never `done()` stays held, even once Python drops it
  (which warns, with a `RuntimeWarning`), so its session stops reading 256 messages later. The hooks given to `turbojet.load` run with
  the GIL on the session's task, so a slow one, or Python busy elsewhere, holds the session up.
  Initiators only: no acceptor and no FIXP sessions. One asyncio loop handles about 100,000 round
  trips a second (see DESIGN.md). `log_to_stderr` writes to stderr, not to Python's `logging`.
- `SessionHandle::send` returning `Ok` means queued, not sent: its `Receipt` says whether the
  message was stored (with its MsgSeqNum) or dropped, and why (the connection ending first,
  logging out, a store failure).
- Custom stores that don't record creation times never reset on a session schedule.
- A held message (`Context::hold`) is redelivered after a connection ends while it's held, a
  clean logout included, unless it's released before the session connects again: a release
  then moves the stored marker on, from a task of its own, which opens the session's log (and
  holds the session meanwhile, so a Logon at that moment is refused as already connected). A
  release with no tokio runtime current leaves the marker. A
  session 256 messages past an unreleased hold reads nothing more until it's released. A custom
  store that makes each change durable as it's made should implement
  `StoredSession::set_next_incoming_in_flight` as one change, or a crash between its two calls can
  lose a hold's marker. Holds aren't available on FIXP sessions.
- While a session waits for a release (256 past a hold), it hands over nothing: a Logout, a
  shutdown or a schedule's end waits out `logout_timeout` for a reply it can't handle. It reads
  on, up to 1 MiB, so a counterparty that closes or goes quiet is noticed; past that it stops
  reading, and one that disappears then isn't noticed until a write fails or TCP gives up. A
  sans-IO driver of one's own does this only if it calls `Session::watch_while_held` and
  `on_read_while_held`.
- Holds don't survive a sequence reset (a Logon with ResetSeqNumFlag, or a schedule's reset): the
  numbers they were for are gone. A counterparty that gap-fills application messages rather than
  resending them won't redeliver a held one. An operator's `set_next_incoming` past a hold is
  undone by a crash before its release (the session asks again from the hold, and marks only the
  first 256). After recovery from a hold, the next incoming number goes backwards (in the gauges and
  `SequenceNumbers`) to the held message.
- Pauses and cancel-on-disconnect countdowns survive a restart only with a store that keeps
  marks (`DiskStorage`, `turbojet-sql`); the memory store, and a store of one's own that doesn't
  implement them, forget both. A restored countdown counts its whole grace from the restart,
  however long the session had been gone. A cancel that fires clears its mark from a task of its
  own; if that fails, or the session is connecting at that moment, a restart cancels it again.
  The registry reads marks once, as the first acceptor or initiator over it starts, so
  `paused()` lists a stored pause only from then. Session events are dropped for a
  subscriber that falls more than `EVENT_QUEUE` behind (it's told how many), and only Logons the
  registry refuses (paused, already connected, too many, store failing) are sent as `Refused`;
  earlier refusals are logged. A FIXP session's
  `activity` is brought up to date once per batch of work, not at each message.
- TLS revocation is checked only against CRLs the application gives, and only at the handshake:
  a session already connected carries on after its certificate is revoked. With CRLs, a
  certificate in the chain that none of them covers is refused. A CRL past its next update is
  still used, with a warning logged, unless `refuse_expired_crls` is set: a certificate its CA has
  revoked since is let in. With `refuse_expired_crls`, a CRL refreshed late locks out every
  counterparty it covers instead.
- A PKCS#12 bundle gives an identity only: trusted CAs come as PEM, and a bundle holding more
  than one private key is refused.
- Allowed addresses and connection limits see the address a connection comes from: behind a load
  balancer or a TCP proxy that doesn't send the PROXY protocol, that's the device's, so list the
  device. A PROXY header's TLVs (TLS details, a unique ID) are skipped, and a header naming a
  family other than TCP over IPv4 or IPv6 keeps the device's address. Changing the list doesn't
  close connections already open.
- Proxies: HTTP with `CONNECT` and Basic authentication, and SOCKS5 with no authentication or a
  username and password. Not SOCKS4, NTLM or Kerberos proxy authentication, or TLS to the proxy
  itself (an `https://` proxy). Through a proxy, a session's `ConnectionInfo` has the proxy's
  address, not the counterparty's.
- A process that shuts down fires its cancel-on-disconnect countdowns at once, as does
  replacing an initiator during a grace period.
- A `SqlStorage` lease must outlast the longest heartbeat interval, since heartbeats are the
  commits that renew it while a session is idle, and gateways sharing a database need clocks in
  step to well within the lease. A log closed without a tokio runtime to give its lease up on
  leaves it to expire.
- An operator change to a disconnected session runs a store's blocking commit (`DiskStorage`
  with fsync) on the calling task. On a multi-threaded runtime the worker's other tasks move to
  another thread meanwhile; on a current-thread runtime they wait for the fsync.
- A holiday closes a whole local day, midnight to midnight: holidays don't shorten a day (no
  early closes), and a weekly schedule closed by one mid-week doesn't follow a market's own
  convention, such as closing at the previous evening's roll.
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
- Time input waits under an inbound `Delay` limit counts against `max_latency`. A counterparty
  that keeps sending faster than the limit builds a backlog, and once that's more than
  `max_latency` (two minutes by default) behind, its messages are rejected as stale and the
  session logs out. The wait can't be excused: most of it is unread, in the network.
- An inbound `Reject` limit never rejects recovery we asked for, and a counterparty controls its
  own gaps, so a hostile one can push one gap's worth of messages through. `Delay` doesn't have
  this hole.
- The simulator exercises outbound limits and inbound `Delay`, but not `Reject`: its checker
  can't cheaply model a BusinessMessageReject in place of a delivery. Session tests cover it.
- A `Message` panics if it grows past 4 GiB: its field index holds 32-bit offsets. Inbound
  messages are far below that (BodyLength is capped at 64 KiB), so only an application building
  a huge outbound message can reach it. Refusing instead would put a `Result` on every field
  added, on the hot path, for that one case.
- A rejected message is lost for good: a counterparty doesn't resend one we rejected, so an
  ExecutionReport carrying a fill is gone. The validator rejects an application message for any
  other reason that fails (a missing required tag, a bad format, a tag the dictionary doesn't
  define), and for a code it doesn't list with `ValidationOptions::application_values` on (off by
  default, unlike QuickFIX). `?` on `parse` of a message generated with `--strict-enums` rejects
  one over an unknown code in any enumerated field, even one the application never reads (a
  venue's own OrdRejReason, a LastLiquidityInd or TargetStrategy of its own). Merge the venue's
  dictionary first.
- The validator accepts a group entry's fields in any order after its delimiter, as QuickFIX's
  ValidateUnorderedGroupFields=N; QuickFIX's default rejects them.
- The sessions file refuses an initiator to a CompID the acceptor lists by CompID alone, whatever
  the initiator's SubIDs, since a counterparty may log on with any: list the counterparty's
  sessions whole instead, or give the initiator a qualifier. Under `unknown = "admit"`,
  initiators to unlisted CompIDs aren't checked.
- A `MessageLog` sees an acceptor's Logon (a FIXP server's first `Negotiate` or `Establish`)
  just after the session handles it, not before as other input. A Logon refused before it names
  a session (a wrong TargetCompID, say), and what's sent on that connection, are logged with no
  `SessionId`, as is a FIXP handshake refused before the session binds. An outbound message is
  logged as it's queued to be written, so one may be
  logged that a failing connection never writes, and the replies to a batch of input are logged
  after the whole batch. Garbled input the codec skips is logged (`MessageLog::garbled`) only
  if it framed as a message; bytes that don't frame, before a BeginString say, aren't, nor is a
  garbled FIXP frame. The bytes are raw,
  Password(554), NewPassword(925) and FIXP credentials included (`FileMessageLog` masks them).
- Masking (`mask_secrets`, and `FileMessageLog` by default) touches only FIX Logons and
  UserRequests and FIXP Negotiates, NegotiationResponses and Establishes. A password a venue
  puts in a field or message of its own, or SecureData(91) in another message's header, is
  written as it is. Data fields are read as the standard ones are defined, so a venue's own data
  field in a Logon with an SOH in it could leave a password after it unmasked.
- A driver of one's own for a sans-IO `Session` makes both `MessageLog` calls itself; for a
  `FixpSession`, only the outbound one, since `feed` makes the inbound call.
- The simulator (`turbojet-sim`) doesn't exercise a `MessageLog`; connection tests do.
- `FileMessageLog` drops messages, and writes a `dropped` line with how many, rather than block a
  session when its buffer is full (16 MiB by default) or a write fails. It doesn't `fsync`, so a
  machine's crash can lose its last records. Records reach the file about a millisecond after
  they're logged, so a file may hold the last moments of the previous UTC day (the record's time
  is when it was logged, the file's day when it was written). A file can pass `max_file_bytes` by up to one batch. Two logs must not share a
  directory. Dropping the log waits for its threads to write what it holds and to compress and
  hand over the last file, a second or so for a full one. After a crash, the `on_finished` hook
  can be called again for a file it was called for before (with `compress`, which redoes what the
  crash cut short); without `compress`, the file a crash left isn't handed to it at all. The hook
  runs on the thread that compresses files, so a slow hook delays the files after it, and one
  that panics stops compression and the hook for that log.
- A few helpers are public only because the exported macros call them (`#[doc(hidden)]`); they
  aren't a stable API.
