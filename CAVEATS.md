# Known caveats

Behaviour that is deliberate or documented, but worth knowing about before you rely on Turbojet,
and worth revisiting. Planned work is in [ROADMAP.md](ROADMAP.md).

- `SessionHandle::send` returning `Ok` means queued, not sent: its `Receipt` says whether the
  message was stored (with its MsgSeqNum) or dropped, and why (the connection ending first,
  logging out, a store failure).
- Custom stores that don't record creation times never reset on a session schedule.
- Pausing a session is kept in memory: a restart forgets it. Session events are dropped for a
  subscriber that falls more than `EVENT_QUEUE` behind (it's told how many), and only Logons the
  registry refuses (paused, already connected, too many, store failing) are sent as `Refused`;
  earlier refusals are logged. A FIXP session's
  `activity` is brought up to date once per batch of work, not at each message.
- TLS revocation is checked only against CRLs the application gives, and only at the handshake:
  a session already connected carries on after its certificate is revoked. With CRLs, a
  certificate in the chain that none of them covers is refused. A CRL past its next update is
  still used.
- A PKCS#12 bundle gives an identity only: trusted CAs come as PEM, and a bundle holding more
  than one private key is refused.
- Allowed addresses and connection limits see the address a connection comes from: behind a load
  balancer or a TCP proxy, that's the device's, not the counterparty's (the PROXY protocol isn't
  read), so list the device. Changing the list doesn't close connections already open.
- Proxies: HTTP with `CONNECT` and Basic authentication, and SOCKS5 with no authentication or a
  username and password. Not SOCKS4, NTLM or Kerberos proxy authentication, or TLS to the proxy
  itself (an `https://` proxy). Through a proxy, a session's `ConnectionInfo` has the proxy's
  address, not the counterparty's.
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
  a huge outbound message can reach it.
- The generated `turbojet-fix*` crates' enumerated fields are strict: parsing a typed message
  fails on a code its dictionary doesn't list, in any enumerated field, even one the application
  never reads (a venue's own OrdRejReason, a LastLiquidityInd or TargetStrategy of its own). An
  application that rejects such a message, as `?` on `parse` does, loses it for good: a
  counterparty doesn't resend a message we rejected, so an ExecutionReport carrying a fill is
  gone. For messages that must not be lost, read the fields needed from the `Message` itself
  (`field`, `opt_field`), or generate a crate with `--lenient-enums` (`turbojet-codegen`).
- Counterparty settings (`CounterpartyMap`, the sessions file) and stores
  (`StorageByCounterparty`) are chosen by CompID, so a counterparty's sessions with different
  SubIDs share them. A `Counterparties` resolver of one's own sees the full `SessionId`.
- The sessions file refuses an initiator to a CompID the acceptor serves as a listed counterparty,
  even with different SubIDs that would make it another session.
- A `MessageLog` sees an acceptor's Logon (a FIXP server's first `Negotiate` or `Establish`)
  with no `SessionId`, and the acceptor's own log sees it even when a `Counterparty` sets
  another for the rest. Anything sent on a connection whose Logon is refused, or whose FIXP
  handshake is refused before the session's log is open, is logged with no `SessionId` too. An outbound message is logged as it's queued to be written, so one may be
  logged that a failing connection never writes, and the replies to a batch of input are logged
  after the whole batch. Garbled input the codec skips isn't logged. The bytes are raw,
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
  they're logged. A file can pass `file_bytes_max` by up to one batch. Two logs must not share a
  directory. Dropping the log waits for its threads to write what it holds and to compress and
  hand over the last file, a second or so for a full one. After a crash, the `on_finished` hook
  can be called again for a file it was called for before (with `compress`, which redoes what the
  crash cut short); without `compress`, the file a crash left isn't handed to it at all. The hook
  runs on the thread that compresses files, so a slow hook delays the files after it, and one
  that panics stops compression and the hook for that log.
- A few helpers are public only because the exported macros call them (`#[doc(hidden)]`); they
  aren't a stable API.
