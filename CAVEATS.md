# Known caveats

Behaviour that is deliberate or documented, but worth knowing about before you rely on Turbojet,
and worth revisiting. Planned work is in [ROADMAP.md](ROADMAP.md).

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
- Counterparty settings (`CounterpartyMap`, the sessions file) and stores
  (`StorageByCounterparty`) are chosen by CompID, so a counterparty's sessions with different
  SubIDs share them. A `Counterparties` resolver of one's own sees the full `SessionId`.
- The sessions file refuses an initiator to a CompID the acceptor serves as a listed counterparty,
  even with different SubIDs that would make it another session.
- A `MessageLog` sees an acceptor's Logon (a FIXP server's first `Negotiate` or `Establish`)
  with no `SessionId`, and the acceptor's own log sees it even when a `Counterparty` sets
  another for the rest. An outbound message is logged as it's queued to be written, so one may be
  logged that a failing connection never writes, and the replies to a batch of input are logged
  after the whole batch. Garbled input the codec skips isn't logged. The bytes are raw,
  Password(554), NewPassword(925) and FIXP credentials included.
- A driver of one's own for a sans-IO `Session` makes both `MessageLog` calls itself; for a
  `FixpSession`, only the outbound one, since `feed` makes the inbound call.
- The simulator (`turbojet-sim`) doesn't exercise a `MessageLog`; connection tests do.
- A few helpers are public only because the exported macros call them (`#[doc(hidden)]`); they
  aren't a stable API.
