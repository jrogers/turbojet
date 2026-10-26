# Operations

## Sequence numbers

Every `SessionHandle` (`acceptor.handle_for("CLIENT1")` or `initiator.handle()`, say) can inspect
and correct its session's sequence numbers, connected or not:

```rust
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
let session = acceptor.handle_for("CLIENT1");
let SequenceNumbers { next_incoming, next_outgoing, .. } = session.sequence_numbers().await?;
session.set_next_incoming(42).await?;   // either direction
session.set_next_outgoing(100).await?;  // forward only
session.reset_sequence_numbers().await?; // both to 1; disconnected sessions only
# Ok(())
# }
```

- A connected session applies the change on its connection task, so its state stays consistent.
  Raising the outgoing number while logged on also sends the counterparty a SequenceReset in
  gap-fill mode, so it expects the new number instead of detecting a gap; a counterparty still
  filling an earlier gap applies it once that's filled, so nothing sent before is lost.
- A disconnected session has its stored state changed directly; it is claimed in the registry
  meanwhile, so it can't log on half-way through.
- Outgoing numbers never move backwards (sent numbers must not be reused); resetting to 1 needs
  the session disconnected, or a Logon with ResetSeqNumFlag for a reset agreed with the
  counterparty. A counterparty may also reset while logged on, with a Logon carrying
  ResetSeqNumFlag=Y at MsgSeqNum 1: both sides start again at 1, Turbojet replying with a Logon
  of its own, and anything outstanding from the old numbering is dropped.

The example gateway's `seqnums` subcommand does the same for a session in its `--store-dir` that
isn't connected, even while the gateway runs; a connected session's store is locked, and the tool
says so rather than changing it.

## Managing sessions

What a management console or a live view needs is in the library; serving it over HTTP or a
websocket, with authentication, is left to the application.

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
// Where each connected session, FIX or FIXP, stands: when and from where it connected, its
// state, sequence numbers and last messages each way. Never waits on a session.
for status in acceptor.statuses() {
    let activity = status.activity.as_ref();
    println!("{} since {} from {:?}: {:?}", status.id, status.since, status.connection.addr, activity.map(|a| a.state));
}

// What happens from now on, to push to a live view.
let mut events = acceptor.subscribe();
while let Ok(SessionEvent { id, kind, .. }) = events.recv().await {
    // Connected, LoggedOn, LoggingOut, ResendStarted, ResendFinished, Disconnected, Paused,
    // Resumed, Refused(reason)
    println!("{id}: {kind:?}");
}

let session = acceptor.handle_for("CLIENT1");
session.pause(Some("maintenance")).await?; // log out, and refuse its Logons until resumed
session.resume().await?;
session.request_resend(1200).await?; // have it send 1200 onwards again

// What was sent and received, from a FileMessageLog's directory.
for file in FileMessageLog::files("/var/log/fix")? {
    for record in FileMessageLog::read(&file.path)? {
        if let LogRecord::Message { time, direction, session, frame, .. } = record? { /* ... */ }
    }
}
# Ok(())
# }
```

- `statuses()` is also on the `SessionRegistry` an acceptor and initiators can share, and
  `SessionHandle::status()` gives one session's. A session's `activity` is read as it left it,
  each value on its own. Its `resets` counts the times its sequence numbers started again since
  it bound, so two reads with the same `since` tell a counterparty's ResetSeqNumFlag(141)
  mid-session, which leaves no other sign: `next_outgoing` may since have passed where it was.
  A read may show a reset's numbers before its count, so treat `next_outgoing` going down as a
  reset too. A FIXP session's negotiating and establishing count as logging on,
  established as logged on, terminating as logging out, and a RetransmitRequest as a resend;
  `FixpAcceptor` and `FixpInitiator` have `statuses` and `subscribe` too.
- Events are broadcast as they happen and never wait for a subscriber: one that falls more than
  `EVENT_QUEUE` (4,096) behind misses the oldest and is told how many, its cue to read
  `statuses()` again. Read `statuses()` after subscribing for where things stand.
- A paused acceptor session's counterparty is turned away at its Logon; a paused initiator stops
  reconnecting until resumed, then connects at once. `pause` and `resume` return once the store
  has the change, so a restart keeps a pause; `DiskStorage` and `turbojet-sql` keep it, the
  memory store only as long as the process. A store failing returns its error, and the pause or
  resume applies anyway until a restart.
- `request_resend(from)` is for an application that lost messages it had processed: the session
  expects `from` again and asks for it, so the resends arrive in sequence and are delivered, with
  PossDupFlag=Y. It needs the session logged on with no resend of its own under way.
- `FileMessageLog::read` can read a file while it's being written: the records end at the last
  whole one. `LogRecords::position()` says where the next record starts, and
  `FileMessageLog::read_from(path, position)` carries on from there, in the same file or in it
  once it's been compressed.

## Cancel on disconnect

`Application::on_logout` says how a logged-on session ended, as a `Disconnect`: a Logout from us
or from the counterparty, the connection lost, a heartbeat timeout, an error, or our own shutdown
(or the schedule's end). Most venues cancel a counterparty's resting orders when its session drops
without a Logout, so a firm that loses its connection isn't left exposed. A session opts in with
`SessionConfig::cancel_on_disconnect`, per counterparty through `Counterparties`:

```rust
use std::time::Duration;
use turbojet::{
    Application, CancelOnDisconnect, CancelTrigger, Context, Disconnect, Message, MessageReject, SessionConfig,
    SessionHandle,
};

let mut config = SessionConfig::new("FIX.4.4", "VENUE");
// Disconnect, or DisconnectOrLogout (a Logout counts too); a grace of up to MAX_CANCEL_GRACE, an hour.
config.cancel_on_disconnect = Some(CancelOnDisconnect::new(CancelTrigger::Disconnect, Duration::from_secs(5)));

struct Venue;

impl Application for Venue {
    fn on_cancel_on_disconnect(&self, session: &SessionHandle, ended: Disconnect) {
        // Cancel the orders this application keeps for `session`.
    }

    fn on_message(&self, _ctx: &mut Context<'_>, _msg: &Message) -> Result<(), MessageReject> {
        Ok(())
    }
}
```

When a session that logged on ends in a way the trigger counts and the counterparty doesn't log
back on within the grace period, `on_cancel_on_disconnect` is called once; a logon within it
stops the countdown, and a logon just after waits for the cancel, so it never follows that
logon's `on_logon`. Our own shutdown and the schedule's end never count. Acceptors and initiators
run the countdowns on a task, and fire those under way at once when they shut down. A process
that stops without shutting down, crashing say, drops its connections with it: with a store
that keeps them (`DiskStorage`, `turbojet-sql`), the sessions logged on with cancel on disconnect
then, or counting down, count down again from the restart, whether or not the counterparty
comes back. With the memory store they're lost, so after a restart check the orders you kept.
Custom drivers call `Session::on_disconnect` when the transport ends and run the countdowns with
`SessionRegistry::next_cancel_deadline` and `run_due_cancels`. In a `turbojet-config` file:
`cancel_on_disconnect = "disconnect"` (or `"disconnect_or_logout"`, or `"off"`) and
`cancel_grace = "5s"`.

A venue that negotiates cancel on disconnect at logon, in Logon fields of its own, is asked in
`Application::to_admin`.

## Shutting down

`Acceptor::shutdown` and `Initiator::shutdown`, on any clone, log every session out (with an
optional Text), disconnect connections that haven't logged on, abandon TLS handshakes, and stop
accepting or reconnecting. They return once every connection has closed, and `serve` or `run`
returns too. A counterparty that doesn't answer the Logout is disconnected after the session's
`logout_timeout` (5 s by default), and a second later anything still open is closed regardless: a
connection blocked writing to a counterparty that has stopped reading, say. Shutdown is permanent.

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.4", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
tokio::signal::ctrl_c().await?;
acceptor.shutdown(Some("end of day")).await;
# Ok(())
# }
```

The example gateway does this on SIGINT or SIGTERM; a second signal exits at once.

## Logging

Turbojet logs through [`tracing`](https://docs.rs/tracing). Each connection runs in a `session`
span whose `id` (`FIX.4.2:GATEWAY->CLIENT1`, say) is filled in once known, so engine events and
`Application` callbacks carry it. Inbound and outbound messages are logged at `DEBUG` under their
own target, `turbojet::messages`, with `direction` = `in`/`out`, so the FIX message log can be
enabled or routed separately: `RUST_LOG=info,turbojet::messages=debug`; passwords in it are shown
as `***`. The `telemetry` module lists the targets.

## Message logs

A message log of one's own sees every message a session receives and sends, framed exactly as on
the wire, for an audit trail or a store of what was received. Implement `turbojet::MessageLog`
(`inbound` and `outbound`, each given the `SessionId`, if known yet, and the frame) and set it in
`SessionConfig::message_log` or `FixpConfig::message_log` (for a sessions file,
`SessionsFileBuilder::with_message_log`). Inbound messages are shown before the session handles
them, outbound ones once the store has committed, as they're queued to be written. The calls are
made on the session's task, so they must not block: hand the bytes to a channel or a buffer that
something else writes out, as here. Entries carry no timestamp, so stamp them yourself.

```rust
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use turbojet::{MessageLog, SessionConfig, SessionId};

#[derive(Debug)]
struct Audit(Sender<(bool, Option<String>, Vec<u8>)>);

impl MessageLog for Audit {
    fn inbound(&self, session: Option<&SessionId>, frame: &[u8]) {
        let entry = (true, session.map(ToString::to_string), frame.to_vec());
        let _ = self.0.send(entry);
    }
    fn outbound(&self, session: Option<&SessionId>, frame: &[u8]) {
        let entry = (false, session.map(ToString::to_string), frame.to_vec());
        let _ = self.0.send(entry);
    }
}

let (sender, entries) = channel();
let mut config = SessionConfig::new("FIX.4.4", "VENUE");
config.message_log = Some(Arc::new(Audit(sender)));
```

The bytes are raw: Password(554), NewPassword(925) and FIXP credentials are in them, and
`turbojet::mask_secrets(&mut bytes)` overwrites them with `*`s in a copy before it's kept. An
acceptor's Logon (a FIXP server's first `Negotiate` or `Establish`) is shown once the session has
handled it, so it has the session's `SessionId` and goes to the log a `Counterparty` config sets,
if it sets one; a refused Logon goes to the acceptor's own log, with the session it named. A
message the session ignored as garbled (a CheckSum that doesn't match, say) goes to `garbled`,
with why, if it framed as a FIX message; it does nothing unless you implement it, and bytes that
don't frame aren't seen.

## Message log files

`turbojet::FileMessageLog` is a `MessageLog` that writes both directions to files in one
directory, kept apart from the store and from diagnostic logging. A new file is started each UTC
day, when the current one reaches `max_file_bytes` (256 MiB by default), and when the log is
opened; files are named `messages-YYYYMMDD-NNNNNN.log`. With `retention` set, files whose day
ended longer ago than that are deleted. Each message is a line `<time> <in|out> <length>
<session>` (the time in UTC to the microsecond, the session `-` until it's known), then the
message's bytes and a newline, so FIX messages read as they stand and binary FIXP ones by their
length. The calls copy each message into a buffer that a thread of the log's own writes out, so
they don't wait for the disk. A message that would pass the buffer's limit (`max_buffer_bytes`,
16 MiB by default) is dropped, and a line `<time> dropped <count>` shows the gap. A message
ignored as garbled is a record `<time> garbled <length> "<reason>" <session>`, read back as
`LogRecord::Garbled`. The example gateway writes one with `--message-log DIR` (and
`--message-log-days N`, `--message-log-gzip`).

```rust,no_run
# let mut config = turbojet::SessionConfig::new("FIX.4.4", "VENUE");
use std::sync::Arc;
use std::time::Duration;
use turbojet::{FileLogOptions, FileMessageLog};

let mut options = FileLogOptions::default();
options.retention = Some(Duration::from_secs(7 * 365 * 86_400));
config.message_log = Some(Arc::new(FileMessageLog::open("/var/log/fix", options)?));
# Ok::<(), std::io::Error>(())
```

```text
20261007-22:33:54.251693 in 97 -
8=FIX.4.2|9=75|35=A|49=CLIENT1|56=GATEWAY|34=1|52=20261007-22:33:54.249|98=0|108=30|141=Y|10=051|
20261007-22:33:54.252557 out 97 FIX.4.2:GATEWAY->CLIENT1
8=FIX.4.2|9=75|35=A|49=GATEWAY|56=CLIENT1|34=1|52=20261007-22:33:54.251|98=0|108=30|141=Y|10=044|
```

(SOH shown as `|`.) Passwords and credentials are written as `*`s of the same length: in FIX
Logons and UserRequests, Password(554), NewPassword(925) and the other fields `Message::redacted`
hides, and in FIXP Negotiates and Establishes, the credentials. Set `mask_secrets: false` to
keep every message exactly as on the wire. A file is finished when the next one starts and when
the log is dropped. With `compress` (turbojet's `gzip` feature), each finished file is compressed
to `….log.gz` and the original deleted; `on_finished` is called with each finished file,
compressed or not, to archive it, say, and may move or delete it. Both run on a thread of the
log's own, so logging never waits for them. `FileMessageLog::files` and `read` list and read
compressed files too (`LogFile::compressed`).

```rust
use std::path::Path;
use std::sync::Arc;
use turbojet::{FileLogOptions, FinishedHook};

let archive: FinishedHook = Arc::new(|path: &Path| {
    // Copy `path` to long-term storage, then delete it.
});
let mut options = FileLogOptions::default();
options.compress = true;
options.on_finished = Some(archive);
```

## Reading the log

`turbojet-log` (`cargo install turbojet-log`) prints a log's records, oldest first, with field and
value names from the standard FIX 4.2, 4.3, 4.4 and 5.0 SP2 dictionaries (chosen by BeginString;
FIXT.1.1 uses 5.0 SP2), or from a dictionary of one's own with `--dictionary FILE` (Orchestra or
QuickFIX XML):

```sh
$ turbojet-log /var/log/fix --session 'FIX.4.4:US->THEM' --type D,8 --tag 11=ORD1 --since 12:00
2026-10-08 12:00:00.000000 in  FIX.4.4:US->THEM D NewOrderSingle
  8 BeginString=FIX.4.4
  ...
  11 ClOrdID=ORD1
  54 Side=1 (Buy)
```

`--in` and `--out` pick a direction; `--since` and `--until` take RFC 3339 or a UTC time of day;
`--raw` prints each message on one line as on the wire. FIXP messages aren't decoded: they're
shown by length (in hex with `--raw`), and `--type` and `--tag` leave them out. A `dropped` record
is always shown, so a gap in the log is visible. `turbojet::LogFilter` is the same filter, to
search the logs from code.

## Replaying the log

`turbojet::replay` feeds the messages a session received, as the log recorded them, to an
`Application` again, to reproduce a problem away from production. They go through a session
configured as in production, at their recorded times, so the application sees what the engine
delivered: duplicates ignored, messages behind a gap held, invalid ones rejected.

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let (config, app) = (SessionConfig::new("FIX.4.4", "US"), Arc::new(App));
use std::io;

let id = SessionId::new("FIX.4.4", "US", "THEM");
let records = FileMessageLog::files("/var/log/fix")?
    .into_iter()
    .map(|file| FileMessageLog::read(&file.path))
    .collect::<io::Result<Vec<_>>>()?
    .into_iter()
    .flatten();
let replayed = turbojet::replay(records, &id, config, app)?;
println!("{} connections, {} messages", replayed.connections, replayed.messages);
# Ok::<(), io::Error>(())
```

Each inbound Logon starts a connection, expecting the Logon's MsgSeqNum (and sending from its
NextExpectedMsgSeqNum), so a log that starts mid-day replays in sequence. What the session sends
is discarded, the store is in memory, and cancel on disconnect is off. `Replayed::dropped` counts
messages the log itself dropped, which the replay can't include.

## Metrics

Metrics use the [`metrics`](https://docs.rs/metrics) facade, behind turbojet's optional `metrics`
feature. Install a recorder (`metrics-exporter-prometheus`, say) before starting acceptors or
initiators, then optionally call `turbojet::describe_metrics()` for help text. Every metric is
labelled with `session`:

| Metric | Type |
|---|---|
| `turbojet_messages_received_total`, `turbojet_messages_sent_total` | counter |
| `turbojet_bytes_received_total`, `turbojet_bytes_sent_total` | counter |
| `turbojet_logons_total`, `turbojet_disconnects_total` | counter |
| `turbojet_rejects_sent_total` (`type` = `session`/`business`) | counter |
| `turbojet_sequence_gaps_total`, `turbojet_resend_requests_received_total` | counter |
| `turbojet_resend_requests_evicted_total` (reaching messages the store evicted) | counter |
| `turbojet_throttled_total` (`direction` = `inbound`/`outbound`) | counter |
| `turbojet_session_logged_on`, `turbojet_next_incoming_seq`, `turbojet_next_outgoing_seq` | gauge |
| `turbojet_inbound_message_seconds`, `turbojet_commit_seconds`, `turbojet_read_to_write_seconds` (opt-in) | histogram |

plus an unlabelled `turbojet_garbled_messages_total`, `turbojet_connections_refused_total` by
`reason` (`total` or `per_ip`), `turbojet_application_panics_total` by `callback` (a panicking
callback is caught; see the `Application` docs), and `turbojet_cancel_on_disconnect_total` by
`trigger` with `turbojet_cancels_pending`. Handles are created once per session, so recording is
a counter increment: order → ack costs nothing extra with no recorder installed, and about 2%
(≈25 ns) with a Prometheus recorder. The example gateway serves them, along with
`gateway_orders_total` and `gateway_cancels_total` by `result`, at `--metrics-listen ADDR`
(`/metrics`).

The latency histograms are recorded only for sessions with `SessionConfig::latency_metrics` set
(the gateway's `--latency-metrics`), since they cost a clock read per inbound message. They time
handling each inbound message (decoding, the session and the application), each store commit run
off the connection's task (an fsync or a database transaction), and the time from reading input to
everything it caused being committed and ready to write. The `telemetry` module says exactly what
each covers. Over localhost TCP, with a Prometheus recorder and both ends recording, they add
about 0.35 µs to a 27.4 µs round trip, and nothing measurable to pipelined throughput.
