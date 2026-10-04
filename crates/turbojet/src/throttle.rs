//! Message-rate limits: at most N messages in any window of length W.

use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, Instant};

/// Most messages a limit may allow per window: the window keeps one time per message
/// (16 bytes), so this caps it at 1.6 MB.
pub const MAX_LIMIT_MESSAGES: u32 = 100_000;

/// Longest window a limit may have. Venues count per second or per minute; a day is far beyond
/// that, and keeps a message's time plus the window well inside what an `Instant` can hold.
pub const MAX_LIMIT_PER: Duration = Duration::from_secs(24 * 60 * 60);

/// At most `messages` in any window of `per`. Parses from `N/W`, e.g. `100/1s` or `50/200ms`
/// (units `ms`, `s` and `m`, and `ns` for a window finer than a millisecond; a bare unit means one
/// of it), and displays in the largest whole unit.
///
/// The window slides: whenever a message would make `messages + 1` within `per`, it waits (or,
/// inbound, is rejected). That is exactly a venue's "N per second", which a token bucket only
/// approximates. Windows are half-open: a message at `t` no longer counts at `t + per`, so with
/// `1/1s`, messages at `t` and `t + 1s` are both allowed.
///
/// Each connection keeps a ring of the last `messages` times for each limited direction, 16 bytes
/// a message, allocated when the connection's session is made.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct RateLimit {
    /// How many messages a window allows, `1..=MAX_LIMIT_MESSAGES`.
    pub messages: u32,
    /// The window's length, more than zero and at most [`MAX_LIMIT_PER`].
    pub per: Duration,
}

impl RateLimit {
    /// At most `messages` per `per`. Panics outside the bounds above.
    pub fn new(messages: u32, per: Duration) -> Self {
        let limit = Self { messages, per };
        if let Err(error) = limit.check() {
            panic!("a valid rate limit: {error}");
        }
        limit
    }

    /// Whether the limit is within its bounds, and if not, which one it breaks. The fields are
    /// public, so a session's configuration checks them when an `Acceptor` or `Initiator` is
    /// built, and again when a session builds a [`Window`].
    pub(crate) fn check(&self) -> Result<(), String> {
        if self.messages == 0 {
            return Err("a rate limit must allow at least 1 message per window, not 0".into());
        }
        if self.messages > MAX_LIMIT_MESSAGES {
            return Err(format!(
                "a rate limit may allow at most {MAX_LIMIT_MESSAGES} messages per window, not {}",
                self.messages
            ));
        }
        if self.per.is_zero() {
            return Err("a rate limit's window must be longer than zero".into());
        }
        if self.per > MAX_LIMIT_PER {
            return Err(format!(
                "a rate limit's window may be no longer than {}, not {}",
                Per(MAX_LIMIT_PER),
                Per(self.per)
            ));
        }
        Ok(())
    }

    /// `messages` as a length, for the window's ring.
    fn messages_len(&self) -> usize {
        usize::try_from(self.messages).expect("a u32 fits in usize")
    }
}

impl fmt::Display for RateLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.messages, Per(self.per))
    }
}

impl FromStr for RateLimit {
    type Err = String;

    /// Parses `N/W`: a message count, then a window such as `1s`, `200ms`, `1m` or `s`.
    fn from_str(s: &str) -> Result<Self, String> {
        let (messages, per) =
            s.split_once('/').ok_or_else(|| format!("expected a rate limit N/W, e.g. 100/1s, not '{s}'"))?;
        let messages = messages.trim();
        // `u32::from_str` takes a leading `+`; a count is plain digits.
        if !messages.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("invalid message count '{messages}' in '{s}'"));
        }
        let messages = messages.parse().map_err(|_| format!("invalid message count '{messages}' in '{s}'"))?;
        let limit = Self { messages, per: parse_per(per.trim())? };
        limit.check()?;
        Ok(limit)
    }
}

/// A window: an optional whole number, then a unit. A bare unit means one of it.
fn parse_per(s: &str) -> Result<Duration, String> {
    let invalid = || format!("invalid window '{s}' (expected e.g. 1s, 200ms or 1m)");
    let too_long = || format!("window '{s}' is too long");
    let digits_end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (count, unit) = s.split_at(digits_end);
    // All digits, so parsing fails only on overflow.
    let count: u64 = if count.is_empty() { 1 } else { count.parse().map_err(|_| too_long())? };
    match unit {
        "ns" => Ok(Duration::from_nanos(count)),
        "ms" => Ok(Duration::from_millis(count)),
        "s" => Ok(Duration::from_secs(count)),
        "m" => count.checked_mul(60).map(Duration::from_secs).ok_or_else(too_long),
        _ => Err(invalid()),
    }
}

/// Displays a window in the largest unit that holds it whole, which [`parse_per`] reads back.
struct Per(Duration);

impl fmt::Display for Per {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NANOS_PER_MILLI: u32 = 1_000_000;
        let (secs, nanos) = (self.0.as_secs(), self.0.subsec_nanos());
        if nanos == 0 && secs > 0 && secs % 60 == 0 {
            write!(f, "{}m", secs / 60)
        } else if nanos == 0 {
            write!(f, "{secs}s")
        } else if nanos % NANOS_PER_MILLI == 0 {
            write!(f, "{}ms", self.0.as_millis())
        } else {
            write!(f, "{}ns", self.0.as_nanos())
        }
    }
}

/// What happens to an inbound application message over the limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InboundLimit {
    /// Stop reading until the window allows more: TCP slows the sender. Every application
    /// message read counts, resends included, and admin messages wait too, since input stays in
    /// order. Holds against a hostile counterparty; see
    /// [`SessionConfig::inbound_limit`](crate::SessionConfig::inbound_limit).
    Delay(RateLimit),
    /// Answer it with a BusinessMessageReject instead of delivering it; rejected messages don't
    /// count. Recovery we asked for counts but is never rejected, so a hostile counterparty can
    /// push one gap's worth through: this protects against fast, well-behaved counterparties.
    Reject(RateLimit),
}

/// An inbound limit as a connection keeps it: the window of application messages received, and
/// what happens to one over the limit.
#[derive(Debug)]
pub(crate) struct Inbound {
    pub(crate) window: Window,
    pub(crate) over: Over,
}

/// What [`Inbound`] does with a message over the limit; [`InboundLimit`] without the limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Over {
    Delay,
    Reject,
}

impl Inbound {
    /// An empty window under `limit`. Panics if the limit is out of bounds, as [`Window::new`].
    pub(crate) fn new(limit: InboundLimit) -> Self {
        let (limit, over) = match limit {
            InboundLimit::Delay(limit) => (limit, Over::Delay),
            InboundLimit::Reject(limit) => (limit, Over::Reject),
        };
        Self { window: Window::new(limit), over }
    }
}

/// The times of the last messages under a limit, oldest first: a ring of at most
/// `limit.messages`, allocated when the window is made, so recording never allocates.
#[derive(Debug)]
pub(crate) struct Window {
    limit: RateLimit,
    times: VecDeque<Instant>,
}

impl Window {
    /// An empty window. Panics if the limit is out of bounds; it's built once per connection.
    pub(crate) fn new(limit: RateLimit) -> Self {
        if let Err(error) = limit.check() {
            panic!("a valid rate limit: {error}");
        }
        Self { limit, times: VecDeque::with_capacity(limit.messages_len()) }
    }

    /// The limit the window keeps.
    pub(crate) fn limit(&self) -> RateLimit {
        self.limit
    }

    /// When another message may go, if not at `now`: a whole window after the oldest message,
    /// once the window holds `limit.messages`.
    pub(crate) fn free_at(&self, now: Instant) -> Option<Instant> {
        self.free_at_or_none().filter(|at| *at > now)
    }

    /// When the window frees up if it's full, whether or not that has passed. It needs no `now`.
    /// [`record`](Self::record) expires the window first, so it's always later than the last
    /// message recorded.
    pub(crate) fn free_at_or_none(&self) -> Option<Instant> {
        if self.times.len() < self.limit.messages_len() {
            return None;
        }
        Some(self.times[0] + self.limit.per)
    }

    /// Records a message at `now`. Past full (a reply that couldn't wait) the oldest goes: it
    /// expires before the rest, so the window still frees up at the right time.
    pub(crate) fn record(&mut self, now: Instant) {
        debug_assert!(self.times.back().is_none_or(|last| *last <= now), "times are recorded in order");
        // Otherwise, after a long gap, a reply past full would leave the window freeing up at a
        // time already past.
        self.expire(now);
        if self.times.len() == self.limit.messages_len() {
            self.times.pop_front();
        }
        self.times.push_back(now);
        debug_assert!(self.times.len() <= self.limit.messages_len());
        debug_assert!(self.free_at_or_none().is_none_or(|at| at > now), "a window frees up after its last message");
    }

    /// Forgets the messages a whole window old at `now`, which no longer count. At most
    /// `limit.messages` steps.
    fn expire(&mut self, now: Instant) {
        while self.times.front().is_some_and(|at| *at + self.limit.per <= now) {
            self.times.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limit(messages: u32, per: Duration) -> RateLimit {
        RateLimit { messages, per }
    }

    #[test]
    fn parses_counts_per_window_in_each_unit() {
        assert_eq!("100/1s".parse(), Ok(limit(100, Duration::from_secs(1))));
        assert_eq!("50/200ms".parse(), Ok(limit(50, Duration::from_millis(200))));
        assert_eq!("10/1m".parse(), Ok(limit(10, Duration::from_secs(60))));
        assert_eq!(" 7 / 2s ".parse(), Ok(limit(7, Duration::from_secs(2))));
    }

    #[test]
    fn a_bare_unit_means_one_of_it() {
        assert_eq!("5/s".parse(), Ok(limit(5, Duration::from_secs(1))));
        assert_eq!("5/ms".parse(), Ok(limit(5, Duration::from_millis(1))));
        assert_eq!("5/m".parse(), Ok(limit(5, Duration::from_secs(60))));
    }

    #[test]
    fn displays_in_the_largest_whole_unit_and_round_trips() {
        for (text, canonical) in [
            ("100/1s", "100/1s"),
            ("50/200ms", "50/200ms"),
            ("10/1m", "10/1m"),
            ("5/s", "5/1s"),
            ("5/1000ms", "5/1s"),
            ("5/60s", "5/1m"),
            ("5/90s", "5/90s"),
            ("5/1500ms", "5/1500ms"),
        ] {
            let parsed: RateLimit = text.parse().unwrap();
            assert_eq!(parsed.to_string(), canonical, "{text}");
            assert_eq!(canonical.parse(), Ok(parsed), "{canonical}");
        }
    }

    #[test]
    fn displays_a_window_finer_than_a_millisecond_in_nanoseconds() {
        // Not something `FromStr` produces, but `new` allows it, and it must round-trip.
        let fine = RateLimit::new(3, Duration::from_nanos(1_500));
        assert_eq!(fine.to_string(), "3/1500ns");
        assert_eq!("3/1500ns".parse(), Ok(fine));
    }

    #[test]
    fn rejects_bad_limits_saying_what_is_wrong() {
        let error = |s: &str| s.parse::<RateLimit>().unwrap_err();
        assert!(error("0/1s").contains("at least 1"), "{}", error("0/1s"));
        assert!(error("100/0s").contains("longer than zero"), "{}", error("100/0s"));
        assert!(error("100").contains("N/W"), "{}", error("100"));
        assert!(error("x/1s").contains("'x'"), "{}", error("x/1s"));
        assert!(error("-1/1s").contains("'-1'"), "{}", error("-1/1s"));
        assert!(error("+5/1s").contains("'+5'"), "{}", error("+5/1s"));
        assert!(error("100/1h2").contains("'1h2'"), "{}", error("100/1h2"));
        assert!(error("100/").contains("''"), "{}", error("100/"));
        assert!(error("100/1").contains("'1'"), "{}", error("100/1"));
        let too_many = format!("{}/1s", MAX_LIMIT_MESSAGES + 1);
        assert!(error(&too_many).contains("at most 100000"), "{}", error(&too_many));
        assert!(format!("{MAX_LIMIT_MESSAGES}/1s").parse::<RateLimit>().is_ok());
        let overflow = format!("1/{}m", u64::MAX);
        assert!(error(&overflow).contains("too long"), "{}", error(&overflow));
        assert!(error("1/1441m").contains("no longer than 1440m"), "{}", error("1/1441m"));
        assert!("1/1440m".parse::<RateLimit>().is_ok());
    }

    #[test]
    #[should_panic(expected = "a valid rate limit")]
    fn new_panics_on_zero_messages() {
        let _ = RateLimit::new(0, Duration::from_secs(1));
    }

    #[test]
    #[should_panic(expected = "a valid rate limit")]
    fn new_panics_on_an_empty_window() {
        let _ = RateLimit::new(1, Duration::ZERO);
    }

    #[test]
    fn a_window_frees_exactly_when_its_oldest_message_expires() {
        let t0 = Instant::now();
        let ms = Duration::from_millis;
        let mut window = Window::new(RateLimit::new(3, Duration::from_secs(1)));
        assert_eq!(window.free_at(t0), None);
        window.record(t0);
        window.record(t0 + ms(100));
        assert_eq!(window.free_at(t0 + ms(100)), None, "two of three");
        assert_eq!(window.free_at_or_none(), None);
        window.record(t0 + ms(200));
        assert_eq!(window.free_at(t0 + ms(999)), Some(t0 + ms(1000)));
        assert_eq!(window.free_at(t0 + ms(1000) - Duration::from_nanos(1)), Some(t0 + ms(1000)));
        assert_eq!(window.free_at(t0 + ms(1000)), None, "the first message is a whole window old");
        assert_eq!(window.free_at_or_none(), Some(t0 + ms(1000)), "regardless of now");
        // Sending again at t0+1s makes the second message the oldest.
        window.record(t0 + ms(1000));
        assert_eq!(window.free_at(t0 + ms(1000)), Some(t0 + ms(1100)));
        assert_eq!(window.free_at_or_none(), Some(t0 + ms(1100)));
    }

    #[test]
    fn recording_past_full_keeps_the_window_exact() {
        // A reply that couldn't wait goes over the limit; the window still frees a whole `per`
        // after the oldest message it holds, and never holds more than `messages`.
        let t0 = Instant::now();
        let per = Duration::from_secs(1);
        let mut window = Window::new(RateLimit::new(3, per));
        for _ in 0..4 {
            window.record(t0);
        }
        assert_eq!(window.times.len(), 3);
        assert_eq!(window.free_at(t0 + per - Duration::from_nanos(1)), Some(t0 + per));
        assert_eq!(window.free_at(t0 + per), None);
    }

    #[test]
    fn expiring_forgets_only_messages_a_whole_window_old() {
        let t0 = Instant::now();
        let ms = Duration::from_millis;
        let mut window = Window::new(RateLimit::new(2, Duration::from_secs(1)));
        window.record(t0);
        window.record(t0 + ms(300));
        window.expire(t0 + ms(999));
        assert_eq!(window.free_at_or_none(), Some(t0 + ms(1000)), "nothing is a whole window old yet");
        window.expire(t0 + ms(1000));
        assert_eq!(window.free_at_or_none(), None, "the first message no longer counts");
        assert_eq!(window.times.len(), 1);
        window.expire(t0 + ms(5000));
        assert!(window.times.is_empty());
    }

    #[test]
    fn recording_after_a_long_gap_frees_the_window_after_the_new_message() {
        // Three sends fill the window; long after it frees, a reply would push out only the
        // oldest and leave the window "full" until 1.1s, already past.
        let t0 = Instant::now();
        let ms = Duration::from_millis;
        let mut window = Window::new(RateLimit::new(3, Duration::from_secs(1)));
        for at in [0, 100, 200] {
            window.record(t0 + ms(at));
        }
        window.expire(t0 + ms(1000));
        window.record(t0 + ms(5000));
        assert_eq!(window.free_at_or_none(), None, "only the reply is in the window");
        assert_eq!(window.free_at(t0 + ms(5000)), None);
        assert_eq!(window.times.len(), 1);
    }

    #[test]
    fn a_window_allocates_its_ring_up_front() {
        let window = Window::new(RateLimit::new(1_000, Duration::from_secs(1)));
        assert!(window.times.capacity() >= 1_000);
    }
}
