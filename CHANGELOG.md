# Changelog

Notable changes to the published crates.

## Unreleased

### `turbojet`

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
  back; the session parses a message when it resends it.
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

### `turbojet-dictionary`

- `Dictionary::data_fields` pairs each data field with the Length field listed before it.

### `turbojet-codegen` and the version crates

- Breaking: data fields are generated as `Vec<u8>` (`data` or `opt_data`), and their Length fields
  are no longer fields of their own.
- Breaking: dates, times, MonthYear and char fields are generated with their types rather than as
  `String`, and multi-value fields as lists: `Vec<ExecInst>` where the field has codes (with
  `Code` when lenient), `Vec<String>` where it doesn't.

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
