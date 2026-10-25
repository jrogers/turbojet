# Schedules and rate limits

## Session schedules

Set `SessionConfig::schedule` (or the example gateway's `--schedule`) to confine a session to
trading hours:

```rust
# let mut config = turbojet::SessionConfig::new("FIX.4.4", "CLIENT");
config.schedule = Some("daily 08:00-17:00 mon-fri America/New_York".parse()?);
// or: "weekly sun 17:00-fri 17:00 America/New_York", "daily 22:00-06:00 UTC",
//     "daily 00:00-00:00 +05:30" (continuous, new period each midnight)
# Ok::<(), Box<dyn std::error::Error>>(())
```

- Outside a period an acceptor refuses logons and an initiator waits for the next period
  (`connect_once` fails immediately).
- When a period ends, a logged-on session sends Logout ("End of session") and disconnects,
  including at the boundary between back-to-back periods.
- The first logon of a new period resets sequence numbers to 1 and clears the resend store,
  based on when the store's state was created (`StoredSession::created_at`). State from before
  creation times were recorded is kept and resets from the next period; custom stores that don't
  record creation times never reset on schedule (a warning says so).

Times are in UTC, a fixed offset, or, with turbojet's `tz` feature, an IANA zone that follows
daylight saving. `SessionConfig::clock` supplies wall-clock time; replace it with
`Clock::from_fn` to test schedules without waiting.

## Holidays

Holidays are dates, in the schedule's time zone, on which no period starts. Give them as a
`HolidayCalendar`, which parses one `YYYY-MM-DD` per line with `#` comments (the example gateway
reads one with `--holidays FILE`):

```rust
use turbojet::{HolidayCalendar, SessionSchedule};

# let mut config = turbojet::SessionConfig::new("FIX.4.4", "CLIENT");
let holidays: HolidayCalendar = "# New York\n2026-11-26\n2026-12-25\n".parse()?; // or from a file
let schedule: SessionSchedule = "daily 09:30-16:00 mon-fri America/New_York".parse()?;
config.schedule = Some(schedule.with_holidays(holidays));
# Ok::<(), Box<dyn std::error::Error>>(())
```

A daily period that starts the day before a holiday still runs into it, and the period after one
resets sequence numbers as any new period does. A weekly schedule skips a week that starts on a
holiday, and is closed for the day on a holiday later in the week: the session logs out at
midnight and may log on again the midnight after, in the same period, so its sequence numbers
carry on. A refused logon, or an initiator waiting, names the holiday.

## Throttling

Venues limit how fast a counterparty may send. `SessionConfig::outbound_limit` keeps a session
within such a limit, and `inbound_limit` holds a counterparty to one:

```rust
# let mut config = turbojet::SessionConfig::new("FIX.4.4", "CLIENT");
use turbojet::InboundLimit;

config.outbound_limit = Some("100/1s".parse()?); // RateLimit::new(100, Duration::from_secs(1))
config.inbound_limit = Some(InboundLimit::Delay("50/200ms".parse()?)); // or InboundLimit::Reject
# Ok::<(), Box<dyn std::error::Error>>(())
```

A `RateLimit` of N per W allows at most N application messages in any window of length W, sliding
and half-open, so it's exactly a venue's "N per second". N is at most 100,000 and W at most a day.
Each connection starts with an empty window.

- **Outbound.** `SessionHandle::send`s wait in the send queue while the window is full, so a full
  queue hands them back as usual. Replies from `Application::on_message` go out at once but count,
  so they can take a window past N, and while they alone fill it, queued sends wait. Admin
  messages, resends and the session's own BusinessMessageRejects neither count nor wait. A message
  counts when it's framed, not when it's written. `SessionHandle::logout` waits for the sends
  queued before it, at the limit's rate; a shutdown, or a logout the session starts itself (a
  schedule's end, the counterparty's Logout), drops them.
- **Inbound, `Delay`.** Every application message counts as it arrives, resends included. While
  the window is full the connection stops reading, and TCP slows the counterparty down. Admin
  messages wait in order behind the rest, so a Logout or ResendRequest may wait up to W, and a
  counterparty that has died may be noticed up to W late; the time held doesn't count as its
  silence.
- **Inbound, `Reject`.** A message over the limit takes its sequence number but is answered with
  a BusinessMessageReject (reason Other, "throttle limit exceeded") instead of being delivered.
  Rejections don't count. Recovery we asked for counts but is never rejected, so a hostile
  counterparty, which controls its own gaps, can push one gap's worth through. `Reject` suits a
  fast but well-behaved counterparty, `Delay` holds against a hostile one, and under a flood
  `Delay` is also the cheaper.

`turbojet_throttled_total` counts sends that waited, holds and rejects, and the first time a limit
is reached on a connection it logs a warning. Custom drivers pace themselves with
`Session::can_send`, `send_free_at` and `input_free_at`; the outbound warning and count come from
Turbojet's own connection driver. The example gateway limits each counterparty with
`--inbound-limit N/W`, and `--over-limit delay` (the default) or `reject`. Limits that are never
reached cost about 10 ns an order, within noise.
