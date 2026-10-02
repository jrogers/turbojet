# Changelog

Notable changes to the published crates.

## Unreleased

### `turbojet`

- A long resend goes out in steps of 256 sequence numbers, each written before the next is read
  from the store, rather than all at once: a ResendRequest for everything a session has sent no
  longer holds all of it in memory. Until the resend ends, the connection sends nothing new,
  Heartbeats included, and what the counterparty sends is read but not processed, so the order on
  the wire is unchanged. Logging out (or shutting down) during a resend stops it.
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
