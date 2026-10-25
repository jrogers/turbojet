# Storage and recovery

A session's store keeps its sequence numbers and the application messages it sent, so it can
answer a counterparty's ResendRequest, and so a restart carries on where it left off. Every
acceptor and initiator is given one, as a `SessionStorage`, which opens a store per session.

## Choosing a store

```rust,no_run
use std::sync::Arc;
use turbojet::{DiskStorage, MemoryStorage, SessionStorage};

// Lost with the process: tests, and sessions that reset every day anyway.
let memory: Arc<dyn SessionStorage> = Arc::new(MemoryStorage::new());
// Files under ./store, synced to the device at every commit.
let disk: Arc<dyn SessionStorage> = Arc::new(DiskStorage::new("./store")?);
# Ok::<(), std::io::Error>(())
```

- `MemoryStorage`: survives reconnects, not restarts (`is_durable` says so). Each session keeps
  its newest sent messages up to 64 MiB (`with_max_session_bytes`); a resend gap-fills older
  ones, logs a warning and counts it in `turbojet_resend_requests_evicted_total`. It keeps at most
  1,024 sessions (`with_max_sessions`) and never forgets one: a new session's Logon past that is
  refused.
- `DiskStorage`: per session, a journal in `<id>.body`, `<id>.body.1` and so on (segments of
  64 MiB holding sent messages in wire format, each commit's followed by a checksummed record of
  both sequence numbers and the message in flight; indexed on open and read back for resends),
  `<id>.seqnums` (two checksummed slots, written alternately, for the records of commits that
  store no messages; locked while the session is connected) and, once recorded, `<id>.created`
  (when the state was created or last reset, for session schedules). A commit appends the batch's
  messages and their record to the newest segment in one write, or, with no messages, writes the
  record to a slot. Each session keeps its newest 1 GiB of journal (`with_max_session_bytes`,
  `with_segment_bytes`): past it the oldest segments are deleted, and a resend gap-fills their
  messages, as with `MemoryStorage`. On open it truncates a torn trailing write, takes the latest
  whole record from either file, and advances the outgoing sequence number past the last stored
  message. Every commit is followed by `fsync` unless `with_fsync(false)`; without it, commits are
  written in the call and survive a process crash but not an OS crash; with it, each commit costs
  one `fsync`. Stores from Turbojet 0.1 open as they were; 0.1 can't open a store once it has a
  journal record.
- `SqlStorage` (`turbojet-sql`): a row per session in `turbojet_sessions` and its messages in
  `turbojet_messages`, in SQLite (bundled, with `synchronous` FULL or NORMAL) or PostgreSQL. A
  commit is one transaction, so sequence numbers and messages can't tear apart. Each session
  keeps its newest 1 GiB of messages (`SqlConfig::max_session_bytes`); past it the oldest go,
  down to 7/8 of it. Opening a session takes a lease on it (`SqlConfig::lease`, two minutes by
  default), which every commit renews and closing gives up: another gateway can't open the
  session meanwhile, and a gateway whose lease expired and was taken can't commit, so it
  disconnects rather than send under numbers the other uses. Heartbeats are commits, so the lease
  must be longer than the longest heartbeat interval. Leases compare the gateways' clocks.

Stores outside Turbojet can check themselves against the suite its own stores pass:
`turbojet::store::conformance::check`, with the `conformance` feature. A store with limits of
its own (a byte budget, say) can also run `conformance::check_model` with them set small.

## When things are stored

Sequence numbers and sent application messages are persisted before each message is written to
the socket; if the store fails, the session disconnects. The store commits them once per batch of
work (everything that arrived in one read, one batch of application sends, one step of a resend),
and nothing the batch sends is written until that commit is done. A store that waits for its
device (`DiskStorage` with `fsync`) hands the commit to the connection driver, which runs it on a
blocking thread, so the async runtime never waits for a disk; meanwhile the connection reads but
processes nothing. A store that waits for a network does the same with a future, and can hand
over opening a session (at logon) and reading a resend step too (`SessionStorage::begin_open`,
`StoredSession::fetch`). A `Receipt` resolves, and an operator hears of a sequence number change,
once the change is committed.

## Delivery at least once

Delivery to the application is at least once. An inbound message counts as received only after
`on_message` has returned and anything it sent in reply is stored, so if the process stops before
then, the counterparty resends it. Each commit also records a marker saying the messages from the
next incoming one on (up to 256) may be handed over, so a batch needs no commit of its own before
its first message; a session that ends cleanly clears it. After a crash, a message in that window
that comes again (flagged PossDupFlag=Y, or answering our ResendRequest) has
`Context::maybe_redelivered()` set: it may have been handled before. Look it up (by ClOrdID, say)
before acting on it again. Resends the application never saw may be marked too; new messages
aren't.

## Holding a message in flight

An application that finishes a message somewhere else, such as a gateway forwarding a client's
order to a venue's session, can keep it in flight after `on_message` returns, until that's done:

```rust
use turbojet::{Application, Context, Message, MessageReject, SessionHandle};

/// Forwards each message to a venue's session.
struct Forwarder {
    venue: SessionHandle,
}

impl Application for Forwarder {
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let hold = ctx.hold();
        let venue = self.venue.clone();
        let order = msg.clone(); // the venue's order, built from the client's
        tokio::spawn(async move {
            if let Ok(receipt) = venue.send_when_ready(order).await {
                let _ = receipt.await; // stored on the venue's session
            }
            hold.release(); // or drop it
        });
        Ok(())
    }
}
```

While a `Hold` is unreleased, the store's marker stays at that message. Later messages are still
handed over and recorded. If the process stops first, the next connection asks for the held
message and everything after it again, each marked `maybe_redelivered`. A session hands over at
most 256 messages past the oldest one held; then it hands over nothing more until that one is
released. It reads on meanwhile, up to 1 MiB, so a counterparty that closes the connection ends it
(what was read and not handed over is resent on the next one) and one that goes quiet is sent
TestRequests and disconnected; past 1 MiB it stops reading, TCP slows the counterparty, and the
wait doesn't count as its silence.

A hold released after its connection ended moves the stored marker on, so the next connection
doesn't ask for the message again; one released only once the session has connected again is
asked for again, as after a crash. Each release costs one message to the session's task. A
session waiting for a release logs a warning, once per connection, and sets the
`turbojet_held_back` gauge (feature `metrics`). `Context::new`, for unit tests, and replays
(`turbojet::replay`) give holds that do nothing. Holds aren't available on FIXP sessions.
