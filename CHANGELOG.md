# Changelog

Notable changes to the published crates.

## Unreleased

### `turbojet`

- **Changed:** reading and replaying message logs is in the public `message_log` module, no longer
  at the crate root: `message_log::{Direction, LogFile, LogRecord, LogRecords, Replayed, replay,
  mask_secrets, DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FILE_BYTES}`. What a session is configured
  with stays at the root as well: `MessageLog`, `FileMessageLog`, `FileLogOptions`, `FinishedHook`
  and `LogFilter`.
- **Changed:** the `sbe` functions that generated codecs call (`take`, `length`, `count`, `chars`,
  `reserve`, `get_le`/`get_be`/`put_le`/`put_be`, `Primitive`, `Group::scan`) are hidden from the
  docs: they aren't an application's API, and may change with the generator.
- **Changed:** `message::utc_timestamp()` is gone: `UtcTimestamp::now().to_fix()` gives the same
  string. `SessionId::escape`, which only `DiskStorage` uses, is no longer public.
- **Changed:** the `application`, `cancel` and `session` modules are private: everything in them
  is at the crate root, and was already, but for `application::Reply`, which is now
  `turbojet::Reply`.
- **Changed:** `UtcTimestamp` and `UtcTimeOnly` no longer deref to chrono's `DateTime<Utc>` and
  `NaiveTime`, so chrono's methods aren't part of their API: take the chrono value with
  `UtcTimestamp::datetime` (before, `time`) or `UtcTimeOnly::time`, and call them on that.
  `datetime` is named as `TzTimestamp::datetime` is.
- **Changed:** `Context::send` returns `Result<(), SendRejected>`: a reply the session would never
  send (a session-level message type, an ApplVerID it doesn't support, a value containing SOH)
  is refused at once, where before it was dropped after `on_message` returned, and only logged.
  `ctx.send(reply)?` turns the refusal into a BusinessMessageReject (ApplicationNotAvailable) of
  the message it answers, as a panic in `on_message` does.
- **Changed:** `Context::session_id` is gone: use `ctx.session().id()`.
- **Changed:** `MsgType`, `Code` and `CodeRef` compare and hash by their FIX code, so a code
  that has a variant is equal to it however it's held: `MsgType::Other("D".into())` is
  `MsgType::NewOrderSingle`, and `Code::Unknown("0".into())` is `Code::Known(EncryptMethod::None)`.
  Before, they were unequal, so a map keyed by one missed the other. A `MsgType` constant can no
  longer be a `match` pattern (`NewOrderSingle::MSG_TYPE =>`); match its variant, or compare with
  `==`.
- **Changed:** `Code::Unknown` holds a `CompactString`, not a `String`, so parsing an unknown code
  doesn't allocate. `Code::Unknown("Z".into())` builds one as before.
- `AcceptorConfig::require_tls` makes `Acceptor::serve`, the plain TCP one, fail at once with
  `InvalidInput`, so an acceptor whose counterparties expect TLS can't be served without it by
  mistake. False by default.

### `turbojet-codegen`

- **Changed:** generated SBE enums compare and hash by their value on the wire, so `Unknown`
  holding a value the schema lists is that value's variant. The FIXP messages in `turbojet` are
  regenerated with it.

### `turbojet-config`

- **Fixed:** an acceptor from a sessions file with `[acceptor.tls]` served with `serve` rather
  than `serve_tls` accepted counterparties in plain TCP, without a word. It now requires TLS, so
  `serve` fails instead.

## 0.7.0 (2026-10-29)

### `turbojet`

- `MemoryStorage` and `DiskStorage` index the messages they keep in a ring buffer in sequence
  order rather than a `BTreeMap`: storing a message no longer allocates an index node, and takes
  about a third of the time in memory.
- `SessionStorage::sessions` lists every session a store holds state for (the built-in stores,
  `StorageByCounterparty` and `turbojet-sql`'s do; the default is `Unsupported`), with
  `conformance::check_sessions`. `SessionStorage` is implemented for `Arc<T>`, so one store can
  be shared. `Job::join` runs two jobs as one, and `Job::map` and `Job::map_result` map a job's
  result, for stores wrapping others.
- `Marks::maybe_handled` (`MaybeHandled::BelowLogon` or `Below(seq)`, and
  `Marks::with_maybe_handled`): a session taken over from an instance that may have handled
  incoming messages its copy doesn't count as received marks the resends of everything the
  counterparty sent before its next Logon `maybe_redelivered`. The stores keep it in their marks'
  text form (`maybe_handled=logon` or `=SEQ`), which an earlier build refuses to read.
- **Changed:** an application's own `impl SessionStorage for Arc<MyStore>` now conflicts with the
  one Turbojet provides, which forwards to `MyStore`: remove it.

### `turbojet-replica`

- New: copies each session's committed state to a hot standby that can take the sessions over
  with no sequence gap and no resend storm. `ReplicatedStorage` wraps the active's store and
  sends each commit to a `Replica` over a `TcpLink` (`serve_standby` on the standby), over TLS
  with the feature `tls` (`TcpLink::with_tls`, `serve_standby_tls`), and a commit waits for both.
  A `Lease` under a rising epoch fences off a replaced active. A commit the standby can't take
  fails the session, which catches the standby up when it opens again; made `degrading`, the
  store commits alone instead, once the lease records the standby out of sync
  (`Lease::set_in_sync`).
- Taking over: acquire the lease, read `Lease::in_sync` under its epoch (only the holder may),
  then call `Replica::promote`. A standby promotes only once it has been checked since it
  started: `ReplicatedStorage::catch_up`, run every few seconds on every replicated store,
  compares each session with the standby's copy, sends whole those that differ (applied on the
  standby only once all of it has arrived), and tells the standby (`Replica::checked`).
  `Replica::promote_unchecked` takes over anyway, for a forced failover.
- Asynchronous replication: `ReplicatedStorage::asynchronous(n)` commits without waiting for the
  standby until a session has more than `n` outgoing sequence numbers unacknowledged (or 64
  batches); the standby must accept the lag (`Replica::accepting_lag(n)` or more), or opening a
  session fails with `InvalidInput`. Promotion skips each session's next outgoing number ahead
  by the lag, so a failover can lose up to `n` of a session's outgoing messages, gap-filled, and
  marks each session (`Marks::maybe_handled`), so the counterparty's resends of messages the old
  active may have handed over arrive `maybe_redelivered`. Promoting with a lag needs a store that
  lists its sessions (`SessionStorage::sessions`). Synchronous stays the default.

### `turbojet-sql`

- `SqlLease` (feature `replica`): a lease on a whole instance in `turbojet_leases`, which
  `migrate` creates, for `turbojet-replica`, recording whether the holder's standby is in sync.

## 0.6.0 (2026-10-25)

### `turbojet`

- **Changed:** FAST is now in `turbojet`, as `turbojet::fast` behind the `fast` feature, rather
  than the `turbojet-fast` crate: depend on `turbojet` with `features = ["fast"]` and change
  `turbojet_fast::` to `turbojet::fast::`. The API is the same. `turbojet-fast` stays at 0.5.0
  on crates.io.
- Early closes in holiday calendars: a line `YYYY-MM-DD HH:MM` closes the session at that time
  on that date. It ends the daily period in session then (a later period that day, an evening
  reopen, runs as usual), and closes a weekly period until midnight, in the same period.
  `HolidayCalendar::with_early_closes` and `early_close`; `len`, `is_empty` and `last` count
  them too.
- A guide, `docs/guide/`, also in the API documentation as `turbojet::guide`: eleven chapters
  from a first session to storage, recovery, schedules and operations, with every example
  compiled and run as a test. The readme's reference sections moved into it, and the readme is
  now an overview, the quick start and the benchmarks.
- Examples in the API documentation of the main types: the acceptor and initiator and their
  configurations, `SessionHandle`, `SessionRegistry`, `Application`, `Context`, `Hold`,
  `Message`, the stores, schedules, rate limits, the sans-IO `Session`, FIXP, TLS and the
  message log, each compiled, and most run, as a test.
- An initiator (FIX or FIXP) that fails to connect while no `tracing` subscriber is installed
  says why once on stderr, rather than nothing, and suggests installing one.
- The `quickstart` example: both roles in one program, logging on and placing an order. The
  readme's first program is now this example, complete, rather than a sketch that didn't compile.
- A hold released after its connection ended moves the stored in-flight marker on, so the next
  connection no longer asks for the message again; before, holds ended with their connection,
  and a released message was redelivered after any logout.
- `AcceptorConfig::proxy_protocol` (and `FixpAcceptorConfig`'s): connections from the listed
  load balancers or proxies must start with a PROXY protocol header, version 1 or 2, and are
  taken as coming from the client it names, for the allowed addresses, the per-address limit and
  `ConnectionInfo`.
- A `MessageLog` gets an acceptor's Logon (a FIXP server's first `Negotiate` or `Establish`)
  with the session's `SessionId`, in the log a `Counterparty` sets if it sets one, rather than
  with none in the acceptor's own: it's logged just after the session handles it. A refused
  Logon is logged with the session it named. `Session::message_log_id` gives a driver of one's
  own the ID to log with.
- `CounterpartyMap::with_session` and `StorageByCounterparty::with_session`: settings and a store
  for one session, told apart from a counterparty's others by its SubIDs or LocationIDs, over
  its CompID's entry.
- **Changed:** `SessionHandle::pause` and `resume` are async and return `io::Result<()>`: they
  store the change, so a pause survives a restart, and return the store's error (the change
  applies anyway, until a restart).
- A session logged on with cancel on disconnect is marked in its store, so after a crash or any
  stop without a shutdown its countdown starts again from the restart, whether or not the
  counterparty comes back. Before, the countdowns were lost with the process, and a crash, which
  drops every connection at once, usually had none to lose.
- `store::Marks`, kept by a store that overrides `StoredSession::marks` and `set_marks` and
  `SessionStorage::marked`; the defaults keep none, so stores of one's own work unchanged.
  `DiskStorage` keeps them in a `<stem>.marks` file, `MemoryStorage` for as long as the process.
  `conformance::check_marks` checks a store that keeps them, and `SessionId::from_key` reads back
  a `key_suffix`.
- `SessionRegistry::restore` and `restore_cancels` read the marks before the first logon, for a
  driver of one's own; acceptors and initiators call them. `Acceptor::serve` returns an error if
  the store can't be read then.
- `Command::Paused`, which a driver of one's own passes to the session like the other commands.
- `ValidationOptions::allow_unknown_tags`: accepts undefined tags below 5000, and defined ones in a
  message they don't belong to, leaving tags from 5000 up to `allow_user_defined_tags` (QuickFIX's
  AllowUnknownMsgFields=Y). Before, accepting them below 5000 meant accepting them from 5000 up
  too.
- An operator change to a disconnected session (`SessionHandle::set_next_outgoing` and the
  like) no longer holds up the other tasks on its runtime worker while a store's blocking commit
  runs: on a multi-threaded runtime they move to another thread for the fsync.
- A weekly schedule is closed for the day on a holiday after its start day: the session logs
  out at midnight, logons are refused, and an initiator waits for the midnight after. It's the
  same period afterwards, so sequence numbers don't reset. Before, only a holiday on the start
  day mattered. `SessionSchedule::next_start` gives the reopening.
- A schedule time the clocks skip is taken as the moment they skip to, the end of the gap, rather
  than the same time on the old offset: on the night clocks go forward in New York, a 02:30 start
  is 03:00, not 03:30. A clock change of three hours or more (Samoa skipping 2011-12-30) was
  resolved with the wrong offset; that day now has no period, and `next_start` passes over a
  period a clock change leaves empty.

### `turbojet-sql`

- `SqlStorage` keeps sessions' marks, in a `marks` column of `turbojet_sessions` that `migrate`
  adds to a database made before it.

### Python bindings (`crates/turbojet-python`, not published)

- An `AppMessage` dropped without `done()` warns with a `RuntimeWarning`, since it stays held and
  holds its session up 256 messages on.
- **Changed:** `Session.pause` and `resume` are awaitable, returning once the change is stored.

### `turbojet-config`

- `proxy_protocol` in `[acceptor]`: the proxies whose connections start with a PROXY header.
- A `[counterparty.NAME]` section with `target_comp_id` and `sender_sub_id`,
  `sender_location_id`, `target_sub_id` or `target_location_id` is for that one session, over
  the CompID's section. An initiator is refused only for a session the acceptor would serve for
  a listed section, so one to a counterparty listed only by whole sessions may log on with other
  SubIDs.
- **Changed:** `SessionsFile::counterparty_config` takes the session's `SessionId`, not a CompID,
  and `counterparties()` and reload `Changes` name sections, which are CompIDs unless they set
  `target_comp_id`.
- The `validation` table takes `allow_unknown_tags`, and the QuickFIX settings converter maps
  AllowUnknownMsgFields=Y to it, so it no longer refuses AllowUnknownMsgFields=Y with
  ValidateUserDefinedFields=Y.

## 0.5.0 (2026-10-25)

Most of this release comes from a review of the public API, and much of it breaks code written
for 0.4: each such change is marked **Changed**. What an acceptor or initiator is made with is
in its configuration (`AcceptorConfig`, `InitiatorConfig`), generated messages take their
required fields by name, and several names changed. A sessions file using a renamed key is
refused with the key's new name.

### `turbojet-dictionary`

- **Changed:** `FieldType::parse` is `From<&str>`: it can't fail.
- `Value::deprecated`: whether the FIX standard deprecates a code, read from Orchestra's
  `deprecated` attribute on each code, as `Field::deprecated` already was on fields.

### `turbojet-codegen` and `turbojet-fix`

- **Changed:** a generated message or group with two or more required fields takes them by name:
  `ExecutionReport::new(ExecutionReportRequired { order_id: "O1".into(), .. })`. Positional
  arguments of one type (the IDs; LeavesQty, CumQty and AvgPx) could be swapped and still
  compile. One required field is still taken as it is.
- **Changed:** an ID ending an acronym is a word of its own in generated names: IOIID(23) is the
  field `ioi_id` and the tag `IOI_ID`, not `ioiid` and `IOIID`.
- A deprecated code's enum variant says so in its docs, as a deprecated field's does: OrdStatus
  Replaced in FIX 4.4, say, and the pegged ExecInst codes in FIX 5.0.

### `turbojet`

- **Changed:** a FIXT session gives each application version its dictionary with
  `SessionConfig::with_appl_version(ApplVersion::new(id).with_dictionary(&d))`.
  `SessionConfig::with_dictionary` and `with_validator` are for FIX 4.x sessions, and refused on
  FIXT whatever the order of calls; before, they attached to the last version added, so the
  order of calls decided which version a dictionary checked.
- **Changed:** renamed, for names that say what things are and match one another: `ApplVerId`
  is `ApplVerID` (FIX's spelling, as the other enums use); `Acceptor::session(comp_id)` is
  `handle_for`; `SessionConfig::send_queue` and `FixpConfig::send_queue` are `max_queued_sends`;
  `Counterparty::DEFAULT_HEARTBEAT` is `MAX_HEARTBEAT_RANGE`; `FileLogOptions::file_bytes_max`
  and `buffer_bytes_max` are `max_file_bytes` and `max_buffer_bytes`, and their defaults
  `DEFAULT_MAX_FILE_BYTES` and `DEFAULT_MAX_BUFFER_BYTES`; `fixp::Ended::TerminatedByPeer` is
  `TerminatedByCounterparty`; `ClientLogin::session_id` is `uuid`; `SbeMessage::bytes` is
  `as_bytes` (with `AsRef<[u8]>`); `CommandReceiver::next(sends)` is `next()` and
  `next_control()`.
- **Changed:** `DiskStorage::new(dir)` syncs every commit; `with_fsync(false)` doesn't. It took
  the choice as a bare bool.
- **Changed:** the `acceptor`, `initiator`, `peer` and `counterparty` modules are private: their
  items are at the crate root (`turbojet::ConnectionInfo`, not `turbojet::peer::ConnectionInfo`).
- **Changed:** `MsgType`, `Precision` and `MessageReject`'s variants are `#[non_exhaustive]`;
  build a reject with `MessageReject::session` or `MessageReject::business`, or the helpers.
- **Changed:** `registry::command_queues`, `CommandSender` and `Session::is_committing` are no
  longer public, and the functions `fix_message!` expands to are in a hidden `message::__private`.
- **Changed:** `SessionLog` is `StoredSession`, so that "log" means only the message log.
- **Changed:** a store must implement what it's for: `SessionStorage::open` and `is_durable`, and
  `StoredSession::sent_messages`, `in_flight`, `set_in_flight`, `created_at` and
  `set_created_at`, have no defaults. Before, a store could leave them out and compile, then
  refuse every Logon, claim to be durable, or lose the at-least-once marker and scheduled resets.
  `begin_open` and `fetch` keep their defaults; a store that overrides them to wait for a
  network implements `open` and `sent_messages` by running their jobs.
- **Changed:** parsing a configuration value returns `ConfigError`, a `std::error::Error`, not a
  `String`: `FromStr` for `RateLimit`, `Proxy`, `ScheduleTimeZone`, `HolidayCalendar` and
  `SessionSchedule`, and `AllowedIps::new`. `ConfigError` no longer converts from any `String`.
- **Changed:** the sans-IO `Session::acceptor`, `Session::initiator`, `FixpSession::client` and
  `FixpSession::server` return a `ConfigError` for an invalid configuration rather than
  panicking, as `Acceptor::new` and the other constructors do. `replay` returns it as an
  `InvalidInput` error.
- `ValueError` and `validation::Invalid` implement `Display` and `Error`, and
  `SequenceError::Storage`'s `io::Error` is its `source()`.
- **Changed:** what was set on an `Acceptor` or `Initiator` after making it is now in its
  configuration, so clones can't disagree about it. `AcceptorConfig` wraps the sessions'
  `SessionConfig` with `max_connections`, `max_connections_per_ip`, `allowed_ips` and
  `counterparties`; `Acceptor::new` takes it, or a `SessionConfig` as before.
  `InitiatorConfig` has `endpoints` (`with_endpoint`) and, with feature `tls`, `tls`
  (`InitiatorTls`); `Initiator::new(config, storage, app)` and `reconfigure(config)` take it
  whole, and `check` refuses no endpoints or an invalid TLS server name. `Initiator::with_registry`
  is a constructor taking the registry in place of storage, refusing one on another clock.
  Removed: `Acceptor::with_max_connections`, `with_max_connections_per_ip`, `with_allowed_ips`,
  `with_counterparties`, `Initiator::with_failover`, `with_tls`. The default limits are
  `AcceptorConfig::DEFAULT_MAX_CONNECTIONS` and `DEFAULT_MAX_CONNECTIONS_PER_IP`.
- **Changed:** FIXP likewise: `FixpConfig` no longer holds the role; `FixpAcceptorConfig` adds
  the server's identity and connection limits, and `FixpInitiatorConfig` the client's identity,
  endpoints, connect timeout, proxy, reconnect policy and TLS. `FixpSession::new` is
  `FixpSession::client` and `FixpSession::server`; `FixpAcceptor::session` is `handle`. Removed:
  `fixp::Role`, `FixpAcceptor::with_max_connections`, `with_allowed_ips`,
  `FixpInitiator::with_failover`, `with_tls`, `with_proxy`, `with_reconnect`.
- `Clock::system()` returns the same clock each time, so configurations and registries left on
  the default agree; `SessionRegistry::clock` returns a registry's.
- **Changed:** `MsgType::Other` holds a `CompactString`, not a `Cow<'static, str>`. Reading the
  type of a message without a variant of its own (MarketDataRequest, say) allocated a string each
  time, and the engine reads it several times per message each way; it now allocates nothing.
- **Changed:** `MessageReject::Business` has a `ref_id`, set with `MessageReject::with_ref_id`
  and sent as BusinessRejectRefID(379), which FIX 4.2 and later require whenever the rejected
  message has a business ID (its ClOrdID, say). The admin `BusinessMessageReject` has
  `business_reject_ref_id`, and Python's `Session.reject` takes `ref_id`.
- FIXP application callbacks are guarded against panics, as FIX ones are: a panic in `on_message`
  drops the replies it queued, one in `verify` refuses the client, and the rest are logged. Before,
  a panic ended the connection.
- FIXP `ClientConfig`, `ServerConfig` and `ClientLogin`, and the generated SBE decoders, print
  credentials as `***` in `Debug` output; before, `{:?}` of a FIXP config printed them.

- While input waits for a held message's release, the connection reads on, up to 1 MiB, so a
  counterparty that closes the connection ends it and one that goes quiet is sent TestRequests and
  disconnected; before, it read nothing and noticed neither. `Session::watch_while_held` and
  `Session::on_read_while_held` do this for a sans-IO driver of one's own.
- `MessageLog::garbled`: input ignored as garbled that framed as a FIX message (a CheckSum that
  doesn't match, say), with why, as QuickFIX logs it. It does nothing unless implemented;
  `FileMessageLog` writes it as a `garbled` record, read back as `LogRecord::Garbled`, which
  `LogFilter` keeps as inbound and `replay` counts as skipped.
- **Changed:** with a dictionary set, a code it doesn't list in an application message no longer
  gets the message rejected, since a rejected message is never resent: an ExecutionReport carrying
  a fill was lost over a venue's own code. `ValidationOptions::application_values` rejects them,
  as QuickFIX and the FIX session test case 14e do; `values` still checks session-level messages.
- A CRL past its next update is logged as a warning when a `Trust` holding it is put in use and
  at each handshake it covers; it's still used. `Trust::refuse_expired_crls` refuses one instead:
  putting the trust in use is an `InvalidData` error, and one that expires in use fails each
  handshake.
- `store::conformance::check_model`: random sequences of what a session does to its log
  (storing, skipping ahead, the in-flight marker, commits, resets, reopening), checked after
  every step against a model of the log. `conformance::check` runs 20 of them; a store with
  limits of its own can run more with the limits small, to reach eviction and its other edges.

### `turbojet-config`

- **Changed:** keys renamed to the engine's names: `connect` is `endpoints`, `local_address`
  `local_addr`, `send_queue` `max_queued_sends`, `allow` `allowed_ips`, and `refuse_expired_crl`
  and `refuse_expired_client_crl` are `refuse_expired_crls` and `refuse_expired_client_crls`. A
  file using an old key is refused with its new name.
- **Changed:** `SessionsFile::server_tls` returns `Error`, as the crate's other methods do, not
  `io::Error`.
- `validation` takes `application_values`.
- `refuse_expired_crl = true` in an initiator's `tls`, and `refuse_expired_client_crl = true` in
  `[acceptor.tls]`, refuse a CRL past its next update, failing the load or reload.

### `turbojet-sql`

- `SqlStorage::open` and its sessions' `sent_messages` work, running the database round trip on
  the calling thread (which needs a multi-threaded tokio runtime current); before, they returned
  `Unsupported`.

### `turbojet-fast`

- **Changed:** `Encoder::encode` is `encode_into`, as `Decoder::decode_into` is.

### `turbojet-log`

- Prints a message ignored as garbled with why, whatever MsgType is asked for.

### Examples

- The gateway's order book holds at most a million orders and two million ClOrdIDs, in B-trees
  rather than hash maps, so it no longer stalls every session each time it grows; past either
  limit, it rejects orders (and past the second, cancels and replaces) until it restarts.
- An `ack` example: a FIX 4.2 acceptor that answers each NewOrderSingle with an ExecutionReport
  and keeps nothing, for `load` to measure the engine against rather than the gateway's order
  book.

### Python bindings (`crates/turbojet-python`, not published)

- FIX initiators from Python's asyncio, loaded from a `turbojet-config` sessions file: events for
  logons, logouts and messages, application messages finished with `done()` so that delivery is at
  least once, sending, rejects, sequence-number controls, pausing, reloading, and untyped messages
  with repeating groups and tag names. Built with maturin.

## 0.4.0 (2026-10-17)

The typed messages are one crate now, `turbojet-fix`, with a feature per FIX version, and it adds
FIX 4.0 and 4.1. It replaces `turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44` and
`turbojet-fix50sp2`, which get no further releases.

### `turbojet`

- Generated messages build much faster in release when they're large: an owned parse keeps its
  slots in one struct, so an early return drops one value rather than each slot. FIX Latest's
  ExecutionReport (619 fields) builds in 36 s rather than 111 s, FIX 5.0 SP2 in 96 s rather than
  113 s. Parsing is as fast as before.
- FIX 4.0 and 4.1 sessions send what those versions define: a Reject with only RefSeqNum(45) and
  Text(58), a session Reject in place of BusinessMessageReject, EndSeqNo(16)=999999 for a resend
  to the end, and SendingTime in whole seconds. `InitiatorConfig::check` refuses `reset_on_logon`
  on FIX 4.0, which has no ResetSeqNumFlag(141).
- `Message::with_reverse_route` doesn't copy OnBehalfOfLocationID(144) or DeliverToLocationID(145)
  from a FIX 4.0 message, which has no such fields.

### `turbojet-codegen`

- `--message Name` (`Generator::only_message`) generates only the messages named, with the groups
  and enums they use, for a few messages of a large dictionary such as FIX Latest's.
- Groups and messages are spread over up to 16 private modules, re-exported, so a release build
  optimises them on every core: FIX 5.0 SP2 builds in 48 s rather than 95 s, FIX Latest in 207 s
  rather than 6.5 minutes. Paths are unchanged.

### `turbojet-fix`

- A new crate replacing the four version crates: each version is a module behind a feature of its
  name (`fix40`, `fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1`, `fix50sp2`), none on
  by default. To move, depend on `turbojet-fix = { version = "0.4", features = ["fix44"] }` in place of
  `turbojet-fix44`, and change `turbojet_fix44::` to `turbojet_fix::fix44::`. The messages are
  unchanged.
- FIX 4.0 and 4.1: every application message, group and enum, generated from the FIX Unified
  Repository, with its names (`OrderSingle` for NewOrderSingle(D), `OrderList`).
- FIX 5.0 and 5.0 SP1, over FIXT 1.1 as FIX 5.0 SP2 is (`ApplVerId::Fix50`, `Fix50Sp1`), generated
  from the FIX Unified Repository.

## 0.3.1 (2026-10-15)

A new crate: `turbojet-fast`, FAST market data decoded into Turbojet messages.

### `turbojet`

- Messages parse from text: `"8=FIX.4.4|9=20|35=0|…".parse::<Message>()`, with SOH or `|`
  between fields, as logs and `Display` show them. BodyLength(9) and CheckSum(10) are checked
  when present and added when not; `codec::decode_text` takes a venue's own data fields.
- `Message::clear` is public: it empties a message, MsgType(35) included, keeping its
  allocations, for building one field by field into a reused message without copying a
  `MsgType` that has no variant of its own.
- Decimal fields parse faster: up to 19 digits are read in one pass rather than checked and then
  parsed again. A FIX 4.2 NewOrderSingle parses about 7% faster borrowed, an ExecutionReport about
  9%. Values parse exactly as before.
- `Activity::resets` counts the store's resets since the session bound, so an application can
  tell its sequence numbers started again between two reads of `status()`: a counterparty's
  ResetSeqNumFlag(141) mid-session leaves no other sign, and `next_outgoing` may since have
  passed where it was.

### `turbojet-fast`

- A new crate: FAST (FIX Adapted for STreaming) 1.1. `Templates::from_xml` loads a feed's templates;
  a `Decoder` decodes its messages into `Message`s, each field's `id` its FIX tag, keeping the
  stream's previous values; an `Encoder` encodes messages back. Every field type, operator and
  dictionary, sequences (as repeating groups), groups, and static and dynamic template references;
  errors carry the specification's codes.

## 0.3.0 (2026-10-14)

A new crate: `turbojet-log`, a command that prints message log files. The other crates have
breaking changes, marked below.

### `turbojet`

- `FileMessageLog::read_from` reads a log file from a `LogRecords::position`, where an earlier
  read stopped, without reading from the start again: a `.log` file seeks to it, a `.log.gz` file
  is decompressed up to it. Positions count the uncompressed bytes, so one taken from a `.log`
  file holds after it's compressed. A position past the end is an `UnexpectedEof` error.
- `LogFilter` chooses a `FileMessageLog`'s records by session, direction, MsgType, field values
  (each tag matching any of several values) and a UTC time range, as `turbojet-log` does;
  `covers_day` says whether a log file's day can hold a match, so it can be skipped unread.
- `SessionStorage::is_durable` says whether a store's sessions survive the process ending: false
  for `MemoryStorage`, true by default, and for `StorageByCounterparty` only if every store it
  routes to is.
- `Context::hold` keeps a message in flight after `on_message` returns, until the `Hold` is
  released or dropped, for an application that finishes it elsewhere (a gateway forwarding an
  order, say): if the process stops first, the next connection asks for it and everything after it
  again, marked `maybe_redelivered`. A session hands over at most 256 messages past the oldest
  held, then waits for its release. A release reaches the session as `Command::HoldReleased`, and
  `Session::is_held_back` says when input waits for one, which is logged once per connection and
  shown by the `turbojet_held_back` gauge (feature `metrics`). Replays give holds that do nothing.
  Not available on FIXP sessions.
- The in-flight marker is now the first incoming message that may not be finished (the next
  incoming number, unless one is held), and a session bound with a marker below its next incoming
  number expects messages again from it. A marker an earlier version left below the number (after
  a restart, or mid-batch in a store that writes each change as it's made) has up to 255 messages
  redelivered once, all marked.
- `SessionLog::set_next_incoming_in_flight` records the next incoming number and the marker
  together. The default makes the two calls; a store that makes each change durable as it's made
  should make it one change, as `MemoryStorage` does.
- `turbojet::replay` feeds the messages a session received, as a `FileMessageLog` recorded them,
  to an `Application` again, through a session at the recorded times, to reproduce a problem away
  from production. `Replayed` says how many connections and messages it replayed, skipped, and
  the log dropped.
- A `load` example opens many sessions to any acceptor, sends orders at a set rate, and reports
  latency percentiles measured from when each order was due.
- `tls::Identity::from_pkcs12` and `from_pkcs12_file` (feature `pkcs12`) read a certificate
  chain and private key from a PKCS#12 bundle (`.p12`, `.pfx`), in the AES form or the legacy
  3DES and RC2 one.
- `Acceptor::with_allowed_ips` and `FixpAcceptor::with_allowed_ips` accept connections only
  from the addresses and CIDR ranges given (`turbojet::AllowedIps`), closing others on accept;
  `set_allowed_ips` replaces the list while serving. Refused ones are counted in
  `turbojet_connections_refused_total` with reason `not_allowed`.
- `InitiatorConfig::proxy` and `FixpInitiator::with_proxy` connect through an HTTP (`CONNECT`)
  or SOCKS5 proxy, with an optional username and password (`turbojet::Proxy`), keeping failover,
  reconnecting and TLS to the counterparty.
- `FileMessageLog` writes passwords and credentials as `*`s (FIX Logons and UserRequests, FIXP
  Negotiates, NegotiationResponses and Establishes) unless `FileLogOptions::mask_secrets` is
  off. `turbojet::mask_secrets` does the same to a frame, for a `MessageLog` of one's own.
- `FileLogOptions::compress` (the new `gzip` feature) compresses each finished `FileMessageLog`
  file to `.log.gz`, and `FileLogOptions::on_finished` is called with each finished file, to
  archive it. `FileMessageLog::files` and `read` list and read compressed files
  (`LogFile::compressed`). The example gateway takes `--message-log-gzip`.
- `DiskStorage` reads a resend step's messages with one read of each segment rather than one
  read per message: 256 messages take 15 µs rather than 160 µs.
- For a management layer: `SessionRegistry::statuses`, `Acceptor::statuses` and
  `SessionHandle::status` list connected sessions with when and over what connection they logged
  on, whether they're paused, and an `Activity` (state, whether recovering a gap, sequence
  numbers, last message each way), for FIX and FIXP sessions alike; `FixpAcceptor::statuses`
  too. `subscribe` on the registry, an acceptor or an
  initiator (FIX or FIXP) broadcasts `SessionEvent`s. `SessionHandle::pause` and `resume` stop and start a
  session from either side, and `request_resend` asks the counterparty for processed messages
  again. `FileMessageLog::files` and `read` read the message log back.
- Breaking: `SequenceCommand` has `RequestResend` and is `#[non_exhaustive]`.
- TLS revocation checks, off unless asked for: `Trust::with_crls_pem`, `with_crls_pem_files` and
  `with_crls_der` add certificate revocation lists, and a certificate they revoke, or that none
  of them covers, is refused at the handshake. A `Trust` with newer CRLs, given to
  `ServerTls::set_client_trust` or `ClientTls::set_trust`, applies from the next handshake. The
  example gateway takes `--tls-client-crl FILE`.
- `FileMessageLog` is a `MessageLog` that writes every message to files in a directory, a new
  one each UTC day and at a size limit, deleting files past an optional retention period, on a
  thread of its own so sessions don't wait for the disk. `FileLogOptions` sets its limits. The
  example gateway writes one with `--message-log DIR` and `--message-log-days N`.
- `MessageLog` sees every message a session receives and sends, framed as on the wire, for an
  audit trail: set it in `SessionConfig::message_log` or `FixpConfig::message_log`.
  `Session::message_log` and `FixpSession::message_log` give it to a driver of one's own, and
  `FixpSession::session_id` gives a FIXP session's ID, as `Session::session_id` does.
- Breaking: `SessionId` has `sender_sub_id`, `sender_location_id`, `target_sub_id`,
  `target_location_id` and `qualifier`, with `with_*` builders, and displays them as QuickFIX
  does (`FIX.4.4:GATEWAY/DESK->CLIENT/TRADER7:qualifier`). An acceptor takes them from each Logon,
  so a counterparty's sessions with different SubIDs or LocationIDs are separate sessions, each
  with its own store and sequence numbers; `Acceptor::handle` reaches one by its full ID, and
  `Acceptor::session` the one without them. Every message a session sends carries its SubIDs and
  LocationIDs unless the application set its own.
- `InitiatorConfig` has the four IDs and `qualifier`, and `InitiatorConfig::session_id` gives the
  full ID; `Initiator::reconfigure` refuses to change any of it.
- `SessionConfig::max_sessions_per_counterparty`, 16 by default: how many sessions one
  counterparty may have connected to an acceptor at once.
- `SessionId::key_suffix` is what a store keys a session on beyond its BeginString and CompIDs,
  empty for one without the new fields, so existing stores open as before; `DiskStorage` adds it
  to the file name. The store conformance suite checks that sessions differing only in those
  fields are independent.
- Breaking: every `Application` callback is given the session's `&SessionHandle` in place of
  its `&SessionId`, and `on_logon` a `&SessionHandle` rather than an owned one (clone it to
  keep it). `handle.id()` gives the ID, and a callback can act on the session, say logging it
  out after a Reject. `Context::session` gives `on_message` the handle, and `Context::new` takes
  one; `SessionHandle::disconnected` makes one for unit tests, and `SessionHandle` displays as
  its ID. `SessionRegistry::run_due_cancels` and `run_all_cancels` take `self: &Arc<Self>`.
- `MessageReject` implements `Display` (as the reject it sends) and `std::error::Error`.
- Breaking: `Acceptor::new` and `Initiator::new` return `Result<Self, ConfigError>` rather than
  panicking on an invalid configuration.
- Breaking: the configuration checks (`SessionConfig::check`, `InitiatorConfig::check`,
  `ReconnectPolicy::check`, `Counterparty::check`) and `Initiator::reconfigure` return a
  `ConfigError`, which implements `std::error::Error`, rather than a `String`.
- `SessionConfig::check` refuses a BeginString that can't be sent: empty, or with SOH or `=` in
  it. Any other is accepted, a venue's own version included.
- Breaking: the public config structs (`SessionConfig`, `InitiatorConfig`, `ReconnectPolicy`,
  `ValidationOptions`, `RateLimit`, `CancelOnDisconnect`), `Endpoint` and `SessionId` are
  `#[non_exhaustive]`, so adding a field to one no longer breaks code that uses it. Build them
  with their constructors (new: `SessionId::new`, `CancelOnDisconnect::new`) and set fields
  afterwards, rather than with struct literals. The error enums, `MessageReject`,
  `InboundLimit` and `CancelTrigger` are `#[non_exhaustive]` too: a match on one from outside
  needs a wildcard arm. So are `FileLogOptions` and `Counterparty` (build them with `default()`
  and `new`), `ScheduleTimeZone`, whose `Named` variant comes and goes with the `tz` feature,
  and `validation::Invalid`.
- Every public type implements `Debug`, among them `Acceptor`, `Initiator`, `Session`,
  `SessionRegistry` and the stores.
- The builder methods that return the value they change are `#[must_use]`, so
  `msg.with(tag, value);` without using the result warns.
- `Application::on_admin_message` shows the application each inbound Heartbeat, TestRequest,
  ResendRequest, Reject, SequenceReset and Logout that passes the session's header and sequence
  checks, once, before the session acts on it. Until now a counterparty's session-level Reject
  of one of our messages was only logged, and a Logout's Text never reached the application.
  It has a default that does nothing.
- `InitiatorConfig::local_addr` connects an initiator from a chosen local address (and port, or 0
  for any), trying only its endpoints' addresses of the same family. Breaking for an
  `InitiatorConfig` built as a struct literal: it has the new field.
- `Application::should_resend` is asked about each stored application message a ResendRequest
  covers; one it declines is gap-filled instead. The default resends everything.
- `SessionConfig::resend_request_chunk` asks for a gap at most that many messages at a time, for
  counterparties that cap ResendRequests. Breaking for a `SessionConfig` built as a struct
  literal: it has the new field.
- Fixed: a resend over a long run of numbers the store doesn't hold wrote nothing until the run
  ended, so if reading through it took longer than the counterparty's resend timeout, the
  counterparty logged out, and did again on every reconnect. Each step of a resend now gap-fills
  what it doesn't resend.
- Fixed: each message the session logs out over restarted the logout timeout, so a counterparty
  that kept sending them (stale ones after a long stall, say) could put a logout off
  indefinitely. The timeout now counts from the first Logout.
- Cancel on disconnect: `SessionConfig::cancel_on_disconnect` takes a `CancelOnDisconnect`, a
  `CancelTrigger` (`Disconnect`, endings without a Logout, or `DisconnectOrLogout`) and a grace
  period of up to `MAX_CANCEL_GRACE` (an hour), set per counterparty through `Counterparties`.
  When a session that logged on ends in a way the trigger counts and the counterparty doesn't
  log back on within the grace period, the new `Application::on_cancel_on_disconnect` is called
  once, and the application cancels the session's orders. Our own shutdown and the schedule's end
  never count. A logon of the session waits for a cancel in progress, so the cancel never comes
  after that logon's `on_logon`. Acceptors and initiators run the countdowns on a task per
  registry, and fire their pending ones at once when they shut down; a process that stops
  without shutting down loses them. Custom drivers run them with
  `SessionRegistry::next_cancel_deadline`, `run_due_cancels` and `run_all_cancels`. With the
  `metrics` feature, `turbojet_cancel_on_disconnect_total` (by `trigger`) counts the calls and
  `turbojet_cancels_pending` the countdowns under way. The example gateway cancels a session's
  open orders 5 seconds after it drops without a Logout. Breaking for a `SessionConfig` built as
  a struct literal: it has the new field.
- Breaking: `Application::on_logout` takes a `Disconnect` saying how the session ended: a Logout
  from us (`Logout`) or the counterparty (`CounterpartyLogout`), `ConnectionLost`,
  `HeartbeatTimeout`, `Error` or `Shutdown` (ours, or the schedule's end). `Disconnect` is
  `#[non_exhaustive]`.
- `Session::on_disconnect` tells a session driven directly that its transport ended, and when,
  so a cancel-on-disconnect countdown starts from then.
- A busy-polling driver: `connection::run_spinning`, `Acceptor::accept_spinning` and
  `Initiator::run_spinning` drive a session on the calling thread without waiting, over a
  `connection::SpinningStream` (a non-blocking TCP socket read and written on each poll). It
  keeps a core busy; store jobs and cancel-on-disconnect still run on the tokio runtime given.
- Decoding is faster: a frame's `=` and SOH delimiters are found 64 bytes at a time and tags
  parsed in one pass. A FIX 4.2 NewOrderSingle decodes into a reused message in about 164 ns
  against 192 ns on an Apple M3.
- Encoding is faster: fields that lie back to back in a message's buffer are copied in one go
  rather than one at a time. An ExecutionReport encodes in about 54 ns against 122 ns on an
  Apple M3, and an order to its acknowledgement, wire to wire, takes about 964 ns against 1.04 µs.
- Integer and decimal fields are written up to three digits at a time, so building a typed
  ExecutionReport takes about 156 ns against 164 ns on an Apple M3.
- `turbojet::sbe`: what codecs generated from SBE message schemas share (see `turbojet-codegen`),
  including `Encode`, which generated messages implement.
- `turbojet::fixp`: FIXP 1.0 sessions (the FIX Performance Session Layer) over TCP, as client
  (`FixpInitiator`) or server (`FixpAcceptor`), or over any stream (`fixp::run`), with recoverable,
  idempotent, unsequenced and one-way flows. Messages on a sequenced flow arrive in order, each
  once, and at least once across a crash or a lost connection, a possible repeat marked
  `Received::maybe_redelivered` (an in-flight marker, as FIX sessions keep). Sessions keep their
  state in any store, `DiskStorage` included (which stores each FIXP frame after a header with
  its sequence number, as frames carry none). `FixpHandle::finish` finishes sending, finalizing
  the logical session once the counterparty has everything. `FixpAcceptor::serve_tls` and
  `FixpInitiator::with_tls` run sessions over TLS (feature `tls`), and `ClientLogin::connection`
  gives `FixpApplication::verify` the client's address and the certificate it presented.
  `FixpInitiator::with_failover` adds backup endpoints, and FIXP sessions record the per-session
  metrics (feature `metrics`), with `FixpConfig::latency_metrics` for the latency histograms.
- Breaking: `Command` is `#[non_exhaustive]`, and has a `Finish` variant (FIXP's finish sending,
  which FIX sessions ignore): a match on it from outside needs a wildcard arm.
- `Command`, `CommandSender`, `CommandReceiver`, `SessionRegistry`, `SessionHandle` and `SendError`
  take the type of what's sent, defaulting to `Message`, so existing code is unchanged;
  `SessionRegistry::with_storage` builds a registry of any kind. Breaking: `command_queues` takes
  it as a type parameter, which a caller names (`command_queues::<Message>`) where it can't be
  inferred.
- `ValidationOptions::reject_invalid` (on by default): off, a message that fails validation is
  logged and delivered rather than rejected, as QuickFIX's RejectInvalidMessage=N.
- `SessionConfig::check_empty_values` (on by default): off, a field without a value is delivered
  as an empty value rather than rejected (4), as QuickFIX's ValidateFieldsHaveValues=N.
- `ValidationOptions::allow_user_defined_tags` also accepts a defined tag from 5000 up in a
  message the dictionary doesn't list it for, as QuickFIX's ValidateUserDefinedFields=N does; it
  accepted only undefined ones. `Validator::options` gives a validator's options.
- The validator accepts any code in a field a QuickFIX dictionary marks `allowOtherValues="Y"`.
- `Code<E>` converts from a `CodeRef<E>` with `.into()`.

### `turbojet-codegen`

- `turbojet-codegen sbe schema.xml --out codec.rs` (and `SbeGenerator`, for a `build.rs`)
  generates a codec for an SBE (Simple Binary Encoding) message schema, as venues publish them: a
  borrowed decoder and a struct to encode per message, an enum, set or composite struct per type,
  and `decode`, which picks the decoder by template ID. SBE 1.0, either byte order, with schema
  versions (`sinceVersion`), nested groups, variable-length data and included type files.
  Generated messages implement `turbojet::sbe::Encode`, and byte arrays (UUIDs, say) are copied
  whole.
- Breaking: enumerated fields are generated as `turbojet::fields::Code<E>`, which keeps a code
  the dictionary doesn't list as `Code::Unknown` rather than failing the parse, as QuickFIX's
  typed fields do. `--strict-enums` (`Generator::strict_enums`) generates `E`, as before, and
  replaces `--lenient-enums` (`Generator::lenient_enums`); `--lenient-enum Field` keeps one field
  a `Code` among strict ones, as does a dictionary's `allowOtherValues="Y"`.
- Breaking: `Modules` is `#[non_exhaustive]`.

### `turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44` and `turbojet-fix50sp2`

- Breaking: enumerated fields are `turbojet::fields::Code<E>`, so a message with a code the
  dictionary doesn't list, in any enumerated field, parses rather than failing, keeping it as
  `Code::Unknown`. Compare a field with a value directly (`order.side == Side::Buy`), set one with
  `.into()`, and get the enum with `.known()`; constructors take either. To reject unknown codes,
  check messages against a dictionary (`SessionConfig::with_dictionary`, feature `validation`).
  Parsing and building typed messages take about 16 ns longer, the fields being bigger.

### `turbojet-dictionary`

- `Field::allow_other_values`: a QuickFIX dictionary's `allowOtherValues="Y"`, which a merged
  venue dictionary can set.
- Breaking: `Error` is `#[non_exhaustive]`.

### `turbojet-config`

- `Initiators::subscribe` gives the events of every initiator's session, those a reload starts
  later included.
- `SessionsFile::acceptor_store` is the store the acceptor keeps its sessions in, durable only
  while every store a counterparty can log on to is.
- `SessionsFile::counterparty_config` gives the `SessionConfig` the acceptor would give a
  counterparty, by CompID: a listed one's settings over `[defaults]`, the defaults for an
  unlisted one under `unknown = "admit"`, or `None` if it would be refused. It reads the file as
  last loaded or reloaded.
- `pkcs12` and `pkcs12_password_env` in `[acceptor.tls]` and an initiator's `tls` (feature
  `pkcs12`): the certificate and key from a PKCS#12 bundle, and the environment variable holding
  its password. The QuickFIX converter's hint for key stores mentions them.
- Builds without its `tls` feature again (broken since initiator proxies, unreleased).
- `allow` in `[acceptor]`: the IP addresses and CIDR ranges connections are accepted from,
  reloaded with the rest of the file.
- `proxy` and `proxy_password_env` in an initiator's section: the proxy to connect through, and
  the environment variable holding its password. The QuickFIX converter reads QuickFIX/J's
  ProxyType, ProxyHost, ProxyPort and ProxyUser into it.
- `quickfix::convert` turns a QuickFIX settings file (`.cfg`) into a sessions file, converting
  the keys that map and refusing, by name, those that would behave differently. The example
  gateway runs it as `gateway convert-cfg FILE`.
- `client_crl` in `[acceptor.tls]` and `crl` in an initiator's `tls`: a PEM file of CRLs to check
  the other side's certificate against.
- `local_address` in an initiator's section: the IP address to connect from, with or without a
  port.
- `resend_request_chunk`, in `[defaults]`, a counterparty's section or an initiator's.
- `cancel_on_disconnect` (`"off"`, the default, `"disconnect"` or `"disconnect_or_logout"`, under
  which Logouts from either side count too) and `cancel_grace` (`0s`, the default, up to `1h`), in
  `[defaults]`, a counterparty's section or an initiator's. Stopping an initiator, removed or
  restarted by a reload, fires its countdown at once.
- `sender_sub_id`, `sender_location_id`, `target_sub_id`, `target_location_id` and `qualifier` in
  an initiator's section; two initiators may log on to one counterparty if these differ.
- `max_sessions_per_counterparty`, in `[defaults]` or a counterparty's section.
- A reload logs out every session of a counterparty no longer listed, whatever its SubIDs; it
  reached only the one without them.
- `SessionsFileBuilder::with_message_log` gives every session in the file a `MessageLog`.
- `SessionsFile::initiator_stores` gives each initiator's session and its store, to read what a
  session stored before its initiator starts (a gateway looking for requests it sent before a
  crash, say).
- `validation = { values = false, reject_invalid = false, ... }`, keyed as `ValidationOptions`'
  fields, in `[defaults]`, a counterparty's section or an initiator's: the checks the section's
  dictionaries make (feature `validation`). `check_empty_values` beside `check_header_order`.
- The QuickFIX converter maps RequiresOrigSendingTime, ValidateFieldsOutOfOrder,
  ValidateFieldsHaveValues, RejectInvalidMessage, ValidateUserDefinedFields,
  AllowUnknownMsgFields and ValidateIncomingMessage, and accepts ValidateSequenceNumbers,
  ValidateChecksum and RejectGarbledMessage at their defaults.
- Breaking: `ConvertError` is `#[non_exhaustive]`.

### `turbojet-sql`

- Sessions are keyed on their SubIDs, LocationIDs and qualifier too, in a new `extra` column.
  `SqlStorage::migrate` adds it to a database made by 0.2 (PostgreSQL alters the table, SQLite
  rebuilds it), keeping existing sessions as they were.
- Breaking: `SqlConfig` is `#[non_exhaustive]`: build it with `default()` and set fields afterwards.

### `turbojet-log`

- A new crate: the `turbojet-log` command prints `FileMessageLog` files with FIX field and value
  names, filtered by session, direction, MsgType, a field's value or a time range.

## 0.2.0 (2026-10-04)

Two new crates: `turbojet-config`, session configuration files, and `turbojet-sql`, session
storage in SQLite or PostgreSQL. The other crates have breaking changes, marked below.

### `turbojet`

- `Initiator::reconfigure` replaces an initiator's configuration and endpoints from its next
  connection attempt (and its reconnect policy from the next wait), here and in clones; a session
  connected carries on. It refuses another session's identity or clock.
  `Initiator::with_registry` keeps the session in a shared `SessionRegistry`, so handles from it
  outlive the initiator. Breaking: `Initiator::endpoints` returns a `Vec<Endpoint>`, since the
  endpoints can change.
- `Counterparty::check` is public, for resolvers that load their settings ahead of time.
- The example gateway reads its sessions from a `turbojet-config` file with `--config FILE`,
  reloading it on SIGHUP.
- Per-counterparty settings on an acceptor: `Acceptor::with_counterparties` takes a
  `Counterparties` resolver, asked at each Logon whether that counterparty may log on and with
  which `Counterparty` settings: a `SessionConfig` derived from the acceptor's (schedule, rate
  limits, validation and the rest), the HeartBtInt range it may ask for, and whether it must
  present a TLS client certificate. `CounterpartyMap` gives them by CompID, refusing or admitting
  others. `store::StorageByCounterparty` keeps chosen counterparties' sessions in stores of their
  own. `Session::set_counterparties` does the same for a session driven directly. The example
  gateway's `--allow` is now a `CounterpartyMap`.
- A typed message that repeats a group's NumInGroup field skips the repeat whole, as it skips any
  repeated field. Before, it skipped only the NumInGroup field, so the repeat's entries were read
  as the message's own fields: a field the message also declares at the top level, such as Text,
  took an entry's value. A repeat that's malformed, so that its end can't be found, now fails the
  parse.
- Breaking: text fields in typed messages are `CompactString`s rather than `String`s (in the
  generated crates, and Text, TestReqID and Username in the admin messages), re-exported with
  `format_compact!` and `ToCompactString` from `turbojet::fields`. Values of up to 24 bytes are
  kept inline, so typical IDs and symbols are parsed and sent without allocating: an owned FIX 4.2
  NewOrderSingle parses about 25% faster, and the example acknowledgement allocates nothing. Build
  one with `.into()` from a `&str` or `String`, or `format_compact!`; code that needs a `String`
  converts with `.to_string()`. `Secret`, `Code::Unknown` and `String` itself are unchanged, so
  custom messages may keep `String` fields.
- Owned typed messages (`FixMessage::from_message`, `Message::parse::<NewOrderSingle>()`) are
  parsed straight into their owned form, rather than borrowed and then made owned, so each group
  entry is parsed once: a FIX 4.2 NewOrderSingle about 11% faster (18% with three allocated
  strings), a FIX 4.4 one with nested groups about 34%. They fail exactly as the borrowed forms do.
  The generated crates hold a second parser per message, so they take about twice as long to
  build in release (`turbojet-fix50sp2` 39 s to 79 s on an M3) and half as long again in debug.
- `DiskStorage` commits with one `fsync` instead of two. A commit that stores messages appends
  them and a checksummed record of the session's state to the newest segment, in one write, as a
  journal; one that stores none writes the record to the `.seqnums` file's slots, as before, so
  records don't fill the byte budget in place of messages. Opening takes the record with the
  highest generation from either file; a commit torn by a power loss reads as before it, its
  messages kept as far as they reached. Stores written by 0.1 open as they were; 0.1 can't open a
  store once a segment holds a journal record, and reports the segment as corrupt.
- Latency histograms, opt-in per session with `SessionConfig::latency_metrics` (feature
  `metrics`): `turbojet_inbound_message_seconds` (decoding, the session and `on_message`, per
  inbound message), `turbojet_commit_seconds` (store commits run off the connection's task) and
  `turbojet_read_to_write_seconds` (from reading input to everything it caused being committed and
  ready to write), labelled by `session`. They cost a clock read per inbound message, and a few per
  batch. The example gateway records them with `--latency-metrics`. Breaking for a `SessionConfig`
  built as a struct literal with the `metrics` feature: it has the new field.
- Application replies are built in messages the session reuses: the session keeps the list
  `Context::send` adds to, and writes a typed reply into a spare message (at most 8 kept per
  session, none over 64 KiB) with the new `FixMessage::write_into`, which `fix_message!` implements
  in place. That's 3 fewer allocations per order → ack, and about 7% less time. `Message::reset`
  and `reset_with_capacity` empty a message for reuse, keeping its allocations. Breaking:
  `Context::send` takes a `Message` or a typed `FixMessage` (the sealed `application::Reply`
  trait) rather than any `impl Into<Message>`.
- Message-rate limits: `SessionConfig::outbound_limit` and `inbound_limit` take a
  `throttle::RateLimit` (at most N application messages in any sliding window W, N up to 100,000
  and W up to a day; it parses `100/1s` or `50/200ms`), inbound with `throttle::InboundLimit::Delay` or
  `Reject`. Sends beyond the outbound limit wait in the send queue, so back-pressure reaches
  `send` and `send_when_ready`; replies from `on_message` go out at once but count. Admin
  messages, resends and the session's own BusinessMessageRejects don't count. Inbound, `Delay`
  stops reading while the window is full, holding admin messages too, and `Reject` answers a
  message over the limit with a BusinessMessageReject (reason Other, "throttle limit exceeded")
  instead of delivering it. Custom drivers pace themselves with `Session::can_send`,
  `send_free_at` and `input_free_at`, and give `Session::on_command` sends only after logon when
  there's an outbound limit. `turbojet_throttled_total` (by `direction`) counts sends that
  waited, holds and rejects. The example gateway limits counterparties with
  `--inbound-limit N/W` and `--over-limit delay|reject`. Breaking for a `SessionConfig` built as
  a struct literal: it has the two new fields.
- Session schedules can skip holidays: a `HolidayCalendar`, set with
  `SessionSchedule::with_holidays`, lists dates (in the schedule's time zone) on which no period
  starts. It parses one `YYYY-MM-DD` per line. A refused logon or an initiator's wait names the
  holiday. The example gateway reads one with `--holidays FILE`.
- TLS certificates can be given from memory as well as files, and replaced while running:
  `tls::ServerTls` (an acceptor's certificate and the CAs it trusts for client certificates) and
  `tls::ClientTls` (an initiator's trusted CAs and client certificate), built from
  `tls::Identity` and `tls::Trust`. A change applies from the next handshake; connected sessions
  carry on. These handshakes are never resumed, so each is checked against the certificates
  current then. `tls::acceptor` and `tls::connector` are built on them, so their handshakes
  aren't resumed either. The example gateway reloads
  its certificates on SIGHUP.
- `DiskStorage` keeps each session's messages in segments (`<id>.body`, `<id>.body.1`, ...) of
  64 MiB, and at most 1 GiB of them (`with_segment_bytes`, `with_max_session_bytes`): past it the
  oldest segments are deleted, and a resend gap-fills their messages, logs a warning and counts it
  in `turbojet_resend_requests_evicted_total`. A store from before opens as it was, its `.body`
  file the first segment. Opening a session's store scans what's kept rather than everything
  since the last sequence reset.
- Breaking: `InitiatorConfig::reconnect_interval` is replaced by `reconnect`, a
  `ReconnectPolicy`. By default an initiator waits 1 s before reconnecting, doubling while
  attempts keep failing up to 60 s, each wait a random half to all of that, and 1 s again once a
  session has logged on; it used to wait 5 s every time. `ReconnectPolicy::fixed` keeps a fixed
  wait.
- An acceptor keeps at most 1,024 connections open at once, and 16 from one IP address
  (`Acceptor::with_max_connections`, `with_max_connections_per_ip`); one past either is closed as
  soon as it's accepted, before any TLS handshake, and counted in the new
  `turbojet_connections_refused_total`.
- Breaking: stores commit once per batch of work (everything from one read, one batch of sends,
  one step of a resend), and nothing the batch sends is written before its commit is done.
  `SessionLog::commit` (by default, nothing to do) lets a store buffer its changes and make them
  durable together, at once or through a returned `Commit` that the connection driver runs on a
  blocking thread, so a store waiting for its device never blocks the async runtime.
  `DiskStorage` buffers: a batch's messages go in one write and its sequence numbers in one
  record, and with fsync both fsyncs run off the runtime. With fsync, 100 orders in flight take
  11 ms rather than 1.6 s, and one at a time 8.1 ms rather than 16.1 ms. Drivers of a `Session`
  other than the connection driver must call `take_commit` after each call into it (running any
  `Commit`, then `on_committed`), ask `ready_for_input` before feeding it a message, and find in
  `output` only what's committed; `commit_blocking` does it all on the calling thread. Code that
  changes a `SessionLog` outside a session must commit too.
- Breaking: the in-flight marker covers a window. Each commit also records the next incoming
  number as in flight, so up to 256 messages from it can be handed over without another commit,
  and a session that ends cleanly clears it. After a crash, a message in the window that comes
  again (flagged PossDupFlag=Y, or answering our ResendRequest) is marked `maybe_redelivered`, so
  some resends the application never saw may be marked too; new messages aren't.
  `SessionLog::set_in_flight` now means "from this message on".
- A `Receipt` resolves once its message is committed, and reads `Dropped::Storage` if the commit
  fails or the connection ends before it finishes. An operator hears of a sequence number change
  once it's committed.
- Breaking: `SessionHandle` commands go on bounded queues. Application sends wait in a queue of
  `SessionConfig::send_queue` messages (10,000 by default): `send` hands the message back in
  `SendError::Full` when it's full, and the new `send_when_ready` waits for room. Logout and
  operator commands have a queue of their own (`CONTROL_QUEUE`, 64), so a full send queue never
  holds them up; a Logout still follows the sends queued before it. Sends now wait in their queue
  until logon completes, and while the connection's output is backed up.
- Breaking: `send` and `send_when_ready` return a `Receipt`, a future resolving to the message's
  MsgSeqNum once the session has stored it, or to `Dropped` saying why it wasn't: the connection
  ended first, the session was logging out, the message was rejected, or the store failed (when
  it may have been stored all the same). `send` returns `Result<Receipt, SendError>`, `logout`
  `Result<(), CommandError>`; `NotConnected` is gone. `Command::Send` carries an optional receipt
  (`Command::send` builds one without), and `CommandSender` and `CommandReceiver` are structs over
  the two queues.

- `MemoryStorage` is bounded. Each session keeps its newest sent messages up to a byte budget
  (`with_max_session_bytes`, 64 MiB by default), evicting the oldest; a resend gap-fills what was
  evicted, logs a warning, and counts it in the new `turbojet_resend_requests_evicted_total`. The
  store keeps at most `with_max_sessions` sessions (1,024 by default): opening a new one past that
  fails with `io::ErrorKind::QuotaExceeded`, so its Logon is refused, and known sessions are never
  forgotten. `SessionLog::evicted_through` (default `None`) tells the session what a store has
  evicted.
- A long resend goes out in steps of 256 sequence numbers, each written before the next is read
  from the store, rather than all at once: a ResendRequest for everything a session has sent no
  longer holds all of it in memory. Until the resend ends, the connection sends nothing new,
  Heartbeats included, and what the counterparty sends is read but not processed, so the order on
  the wire is unchanged. Logging out (or shutting down) during a resend stops it.
- Raising a logged-on session's next outgoing number (`SessionHandle::set_next_outgoing`) tells
  the counterparty with a SequenceReset in gap-fill mode rather than reset mode. Reset mode made a
  counterparty still filling an earlier gap abandon it, losing messages already sent; in gap-fill
  mode it waits its turn.
- `DiskStorage` keeps its sequence numbers in two slots, written alternately, each with a
  generation and a checksum, so a write torn by a power loss falls back to the record before it.
  A torn record used to leave the store unable to open, or read as numbers never recorded (a torn
  19 to 20 could read as 29). Files from before still read; the first write after one leaves it
  intact.
- The connection driver reads while its output waits to be written. It used to write all of a
  wake-up's output before reading again, so once both ends' send buffers were full, each waited
  for the other to read and the connection hung, its timers unable to fire. Handle commands now
  wait while 256 KiB of output is unwritten, and a counterparty that has stopped reading is
  disconnected once 16 MiB of output, or of input waiting for a resend to end, has built up.
- `Session::is_resending` and `Session::on_resume`, for drivers of a `Session` other than
  `connection::run`: while a session is resending, write its output and call `on_resume` for the
  next step. `Session::next_deadline` is `None` meanwhile.

- A `Message` that grows past 4 GiB panics. Its field offsets used to wrap silently, which
  would have corrupted the message.
- Debug assertions check framing, sequence numbers and the gap queue: whatever is encoded,
  framed for sending or given to a store must frame back as exactly one message. They run in
  tests and fuzzing only.
- Data fields carry any bytes, including SOH and bytes that aren't UTF-8: RawData, SecureData,
  XmlData, the Encoded* fields and the rest FIX pairs with a Length field (`DataFields`), and a
  venue's own (`SessionConfig::with_data_field`, or from the session's dictionary). Each is decoded
  by the length its Length field gives, stored and resent intact, and read with
  `Message::get_bytes` or `Message::fields_bytes`; `Message::set_data` writes one with its length.
- A data field whose Length field doesn't give its length is rejected (SessionRejectReason 6, or 5
  from validation), and an outbound message with one isn't sent, as with SOH in any other field.
- Breaking: `Message::get` and `Message::fields` leave out a data field whose value isn't UTF-8.
- Breaking: `fix_message!` and `fix_group!` declare data fields as `data` or `opt_data`, with both
  tags (`LEN => DATA`), holding `Vec<u8>`; `GroupSpec` has a `lengths` field for them.
  `SessionConfig` has a `data_fields` field.
- Logs show binary values by their length, and redact RawData, SecureData, EncryptedPassword and
  EncryptedNewPassword.
- Breaking: `SessionLog::record_outgoing` takes the message as sent, encoded (`&[u8]`), and
  `SessionLog::sent_messages` returns those bytes: stores no longer encode or decode messages.
  `DiskStorage` checks only each stored message's framing, when it opens and when it reads one
  back; the session parses a message when it resends it. On a FIXT session supporting more than
  one application version, a message in the default version is stored with ApplVerID(1128) stated.
- `DiskStorage` opens a session that has sent a message over 64 KiB, and the session resends it:
  the 64 KiB limit on BodyLength(9) applies only to what the counterparty sends. It used to refuse
  to open the session.
- Outbound messages are logged as they were encoded, BodyLength(9) and CheckSum(10) included, as
  inbound ones are.
- Typed dates and times: `UtcTimeOnly`, `NaiveDate` (UTCDateOnly, UTCDate and LocalMktDate),
  `MonthYear`, `TzTimeOnly` and `TzTimestamp`; `char`; and space-separated lists as `Vec<T>`.
- Timestamps and times keep their precision (seconds, milliseconds, microseconds or nanoseconds):
  a received value is written back as it came, and new ones are milliseconds unless given another.
  `SessionConfig::timestamp_precision` sets how SendingTime is written.
- Breaking: `UtcTimestamp` is a struct rather than an alias of chrono's `DateTime<Utc>`, which it
  dereferences to; `DateTime<Utc>` no longer converts to or from a field value. Use
  `UtcTimestamp::now()` or `UtcTimestamp::from(time)`. `SessionConfig` has a
  `timestamp_precision` field.
- Validation checks the format of every date and time type, and of MonthYear: a malformed one is
  rejected (SessionRejectReason 6) where it used to pass.
- Decoding is about 45% faster, and parsing a typed message with timestamps about 40% faster.
- Breaking: `Session` writes what it sends, encoded, to one buffer it reuses: `Session::output`
  and `Session::clear_output` replace the `Vec<Action>` its methods returned (they now return
  nothing), and `Session::is_closed` replaces `Action::Disconnect`. `Action` is gone.
- Messages are encoded once, straight from the body into the session's output, and stored as
  those bytes: `DiskStorage` no longer encodes each one a second time, and acknowledging an order
  allocates about 4 fewer times with `MemoryStorage`. `DiskStorage` also writes its sequence
  numbers without allocating, 3 times per order. Taking an order to its acknowledgement, wire to
  wire, is about 13% faster.
- `SessionConfig::clock` also stamps outbound SendingTime: the session reads it once per call,
  for checking inbound SendingTime and stamping outbound. With the default (system) clock nothing
  changes; a replaced clock's time now goes on the wire.
- Typed messages write decimals and integers without `core::fmt`: building an ExecutionReport is
  about 39% faster. With fewer passes over each message's fields and one clock read per message,
  taking an order to its acknowledgement is about 15% faster again.
- Each inbound message is decoded into one `Message` reused for the connection, so decoding no
  longer allocates once it has grown to the size of the messages received. `codec::decode_into`
  decodes into a message the caller reuses. Decoding a NewOrderSingle into a reused message is
  about 15% faster than into a new one, and taking an order to its acknowledgement, wire to wire,
  about 2.5% faster.
- Breaking: `Session::on_message` takes the message by reference.
- Typed messages and groups come in a borrowed form as well as an owned one: `NewOrderSingleRef<'a>`
  beside `NewOrderSingle`, with strings as `&str`, data fields as `&[u8]`, lists as `List`, groups
  as `Group` (iterated without collecting their entries), and secrets and lenient codes as
  `SecretRef` and `CodeRef`. `Message::parse` and `Message::parse_strict` give either form, every
  field still checked up front, and `into_owned()` makes a borrowed one owned. Parsing the borrowed
  form allocates nothing, groups included, and is about 30% faster than parsing the owned form was.
  `fields::FieldRef` is each field type's borrowed form, and `FixMessageRef` and `FixGroupRef` are
  implemented by the borrowed types.
- Breaking: `fix_message!` and `fix_group!` name both forms (`NewOrderSingle / NewOrderSingleRef =
  "D" { .. }`, `Party / PartyRef { .. }`), and a message needs at least one field. Derives and
  most other attributes stay on the owned form; `#[deprecated]`, `#[doc(hidden)]` and lint levels
  apply to the borrowed form too.
- Breaking: a custom field type used in `fix_message!` or `fix_group!` needs a borrowed form: for a
  `FromFix + Copy` type, `impl_field_ref!(MyType)` makes it its own.
- Breaking: `FixMessage` and `FixGroup` have a `Ref` associated type, the borrowed form.
- Parsing the owned form is the borrowed parse followed by `into_owned()`, and is slower
  than it was: about 15-35% for messages with flat or no groups and about 60% for a FIX 4.4
  NewOrderSingle with nested groups.
- `#[cfg]` on a `fix_message!` or `fix_group!` applies to everything it generates: a message
  configured out used to fail to compile.
- Stores can hand the session's driver a future as well as a blocking job: `store::Job<T>` is
  either, and `Commit` is now `Job<()>` (`Commit::blocking` and `run` work as before). A store
  can also open a session's log with one (`SessionStorage::begin_open`, which by default calls
  `open`; `open` itself now has a default that refuses, for stores that only open with
  `begin_open`) and read a resend step with one (`SessionLog::fetch`, which by default calls
  `sent_messages`). The session waits for them as for a commit: custom drivers call
  `Session::take_fetch` / `on_fetched` and `take_open` / `on_opened` after `take_commit`, and
  hold input while `is_waiting_on_store`. `Session::commit_blocking` runs them too.
- Operator changes to a disconnected session await a store's opening and commit.
- The store conformance suite is public with the `conformance` feature:
  `store::conformance::check` (async) and `check_blocking` run a store through everything a
  session relies on.

### `turbojet-config`

- New: session configuration files. `SessionsFile` reads an acceptor (`[acceptor]`), named stores
  (`[store.NAME]`, memory or disk, or registered in code), `[defaults]` and a
  `[counterparty.COMPID]` section per counterparty from TOML, and makes the `Acceptor` it
  describes. Unknown keys are errors and every value is checked on loading, with errors naming
  the section and key. `SessionsFile::reload` reads it again while running: a file that doesn't
  load, or that changes what's fixed until a restart, leaves the one in use; otherwise changed
  settings apply from each counterparty's next Logon, TLS certificates are replaced, and under
  `unknown = "refuse"` connected counterparties no longer listed are logged out. Features `tls`,
  `tz`, `validation`, `metrics`.
- `[initiator.NAME]` sections: an initiator's counterparty, addresses, credentials (the password
  from an environment variable), reconnect policy, TLS and session settings over `[defaults]`.
  `SessionsFile::initiators` starts each on its own task as `Initiators`, whose handles survive
  reloads. `SessionsFile::reload_all` reloads acceptor and initiators together: added initiators
  start, removed ones are logged out and stopped, and changed ones apply their settings from
  their next connection. `[acceptor]` is optional, so `acceptor`, `listen` and `base` return
  `Option`s.

### `turbojet-sql`

- New: session storage in SQLite or PostgreSQL through sqlx, `SqlStorage` with `SqlConfig`.
  Each session's state is a row of `turbojet_sessions` and its messages rows of
  `turbojet_messages` (`SqlStorage::migrate` creates them); a commit is one transaction. Opening,
  committing and resend reads run as futures the connection's driver awaits. Each session keeps
  its newest messages up to `max_session_bytes` (1 GiB by default). Gateways sharing a database
  hold a lease on each session they run (`SqlConfig::lease`, two minutes by default), renewed by
  every commit: another gateway can't open the session while it's held, and a gateway whose lease
  expired and was taken can't commit. Features `sqlite` (default), `postgres` and `tls`.

### `turbojet-dictionary`

- `Dictionary::data_fields` pairs each data field with the Length field listed before it.

### `turbojet-codegen` and the version crates

- Breaking: data fields are generated as `Vec<u8>` (`data` or `opt_data`), and their Length fields
  are no longer fields of their own.
- Breaking: dates, times, MonthYear and char fields are generated with their types rather than as
  `String`, and multi-value fields as lists: `Vec<ExecInst>` where the field has codes (with
  `Code` when lenient), `Vec<String>` where it doesn't.
- Each message and group is generated with its borrowed form, named with `Ref`. A group whose name,
  or its borrowed form's, would clash with another type's gets `Entry`, as before, and an enum
  named like a message's borrowed form gets `Code`; a message named like another's borrowed form,
  or one with no body fields, is an error. No generated name in the version crates changed.
- Debug builds of the version crates take about a third longer; a release build of
  `turbojet-fix44` takes about a fifth less time.
- Breaking: text fields are generated as `CompactString` (lists of them as `Vec<CompactString>`);
  required ones are still taken by `new` as `impl Into<…>`. See `turbojet` above.

## `turbojet` 0.1.1 (2026-09-29)

- The README's links and logo work on crates.io, and it shows the crates.io badge.

## 0.1.0 (2026-09-29)

The first release.

### `turbojet`

- FIX session layer for FIX 4.2 and later, and FIXT.1.1 with FIX 5.0 SP2: logon (credentials,
  NextExpectedMsgSeqNum), sequencing, gap detection with messages ahead of a gap kept until it's
  filled, resends and gap fills, heartbeats and TestRequests, logout, and a counterparty's sequence
  reset while logged on.
- FIXT.1.1: DefaultApplVerID negotiated at logon, per-message ApplVerID in either direction,
  CstmApplVerID and ApplExtID passed through, and messages resent in the version they were sent in.
- At-least-once delivery to the application: a message counts as received only once it's handled,
  and the one in flight at a crash is marked as possibly handled when it's resent.
- Checks from the FIX session test cases: SendingTime accuracy, OrigSendingTime on resends, header
  field order, required header fields, and optional validation against a data dictionary (feature
  `validation`). Routing fields kept in the header, and rejects routed back.
- `Acceptor` and `Initiator` over TCP, TLS with optional mutual authentication (feature `tls`),
  initiator failover, and graceful shutdown.
- Memory and disk session storage, session schedules (named time zones with feature `tz`), and
  operator control of sequence numbers.
- Typed messages, groups and enums through the `fix_message!`, `fix_group!` and `fix_enum!` macros,
  with strict parsing on request (`Message::parse_strict`).
- Structured logging, and Prometheus-compatible metrics (feature `metrics`).

### `turbojet-dictionary`

- Loads FIX Orchestra and QuickFIX-format data dictionaries, and merges a venue's QuickFIX-format
  additions onto either.

### `turbojet-codegen`

- Generates `fix_enum!`, `fix_group!` and `fix_message!` invocations from a dictionary, from an
  application's `build.rs` or as a command.

### `turbojet-fix42`, `turbojet-fix43`, `turbojet-fix44`, `turbojet-fix50sp2`

- Every application message, group and enum of FIX 4.2, 4.3, 4.4 and 5.0 SP2, generated from the
  FIX Trading Community's official data.
