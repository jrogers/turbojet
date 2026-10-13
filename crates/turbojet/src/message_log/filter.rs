//! Choosing records of a [`FileMessageLog`](super::FileMessageLog) by session, direction,
//! MsgType, field values and time.

use chrono::{DateTime, NaiveDate, Utc};

use super::read::{Direction, LogRecord};
use crate::Message;
use crate::codec::{Decoded, decode};
use crate::message::tags;

/// Which records of a [`FileMessageLog`](crate::FileMessageLog) to keep. Each condition left
/// empty (or `None`) keeps everything; the ones set must all hold. The default keeps every record.
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use turbojet::{Direction, LogFilter};
///
/// // What we received of three orders, from 08:00 UTC on 8 October.
/// let filter = LogFilter {
///     direction: Some(Direction::Inbound),
///     tags: vec![(11, vec!["A".into(), "B".into(), "C".into()])],
///     since: Some(Utc.with_ymd_and_hms(2026, 10, 8, 8, 0, 0).unwrap()),
///     ..LogFilter::default()
/// };
/// assert!(!filter.covers_day(Utc.with_ymd_and_hms(2026, 10, 7, 0, 0, 0).unwrap().date_naive()));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LogFilter {
    /// Only these sessions, as [`SessionId`](crate::SessionId) displays them
    /// (`FIX.4.4:US->THEM`). A message whose session wasn't known yet has none to match.
    pub sessions: Vec<String>,
    /// Only messages received, or only messages sent.
    pub direction: Option<Direction>,
    /// Only these MsgTypes(35).
    pub msg_types: Vec<String>,
    /// Only messages with a field of each tag equal to one of its values: `(11, ["A", "B"])` keeps
    /// messages whose ClOrdID(11) is A or B. A field in a repeating group counts.
    pub tags: Vec<(u32, Vec<String>)>,
    /// Only records logged at or after this time.
    pub since: Option<DateTime<Utc>>,
    /// Only records logged before this time.
    pub until: Option<DateTime<Utc>>,
}

impl LogFilter {
    /// Whether `record` passes: its time is in range and, for a message, it passes
    /// [`matches_message`](Self::matches_message). A FIX message is decoded only if a MsgType
    /// or tag condition is set. A [`LogRecord::Dropped`] passes if its time is in range, so a gap
    /// in what's kept is explained.
    #[must_use]
    pub fn matches(&self, record: &LogRecord) -> bool {
        match record {
            LogRecord::Message { time, direction, session, frame } => {
                if !self.in_range(*time) {
                    return false;
                }
                let decoded = if self.needs_message() && frame.starts_with(b"8=") { Some(decode(frame)) } else { None };
                let msg = match &decoded {
                    Some(Decoded::Message(msg, _)) => Some(msg),
                    _ => None,
                };
                self.matches_message(*direction, session.as_deref(), msg)
            }
            LogRecord::Dropped { time, .. } => self.in_range(*time),
        }
    }

    /// Whether a message passes the session, direction, MsgType and tag conditions (not the
    /// time), given the message decoded: `msg` is `None` for a FIXP message or a FIX one that
    /// doesn't decode, which has no MsgType or fields to match. For a caller that decodes the
    /// message anyway; `msg` is only read if [`needs_message`](Self::needs_message).
    #[must_use]
    pub fn matches_message(&self, direction: Direction, session: Option<&str>, msg: Option<&Message>) -> bool {
        if self.direction.is_some_and(|d| d != direction)
            || (!self.sessions.is_empty() && !session.is_some_and(|s| self.sessions.iter().any(|want| want == s)))
        {
            return false;
        }
        if !self.needs_message() {
            return true;
        }
        let Some(msg) = msg else { return false };
        let msg_type = msg.get(tags::MSG_TYPE).unwrap_or_default();
        (self.msg_types.is_empty() || self.msg_types.iter().any(|t| t == msg_type))
            && self
                .tags
                .iter()
                .all(|(tag, values)| msg.fields().any(|(t, v)| t == *tag && values.iter().any(|w| w == v)))
    }

    /// Whether matching needs the message decoded: a MsgType or tag condition is set.
    #[must_use]
    pub fn needs_message(&self) -> bool {
        !self.msg_types.is_empty() || !self.tags.is_empty()
    }

    /// Whether `time` is in the time range: at or after [`since`](Self::since), before
    /// [`until`](Self::until).
    #[must_use]
    pub fn in_range(&self, time: DateTime<Utc>) -> bool {
        self.since.is_none_or(|since| time >= since) && self.until.is_none_or(|until| time < until)
    }

    /// Whether a log file of the UTC `day` ([`LogFile::day`](crate::LogFile::day)) may hold
    /// records in the time range, so it can be skipped unread if not. A file may hold the end of
    /// the day before as well as its own: a record is stamped as it's logged, but its file is
    /// chosen as it's written, a moment later. So the file of day D covers D-1 00:00 up to D+1
    /// 00:00, never a later day.
    #[must_use]
    pub fn covers_day(&self, day: NaiveDate) -> bool {
        let end = (day + chrono::TimeDelta::days(1)).and_time(chrono::NaiveTime::MIN).and_utc();
        let start = end - chrono::TimeDelta::days(2);
        self.since.is_none_or(|since| since < end) && self.until.is_none_or(|until| until > start)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn at(day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, day, hour, minute, 0).unwrap()
    }

    fn record(direction: Direction, session: Option<&str>, frame: &[u8]) -> LogRecord {
        LogRecord::Message { time: at(8, 12, 0), direction, session: session.map(str::to_owned), frame: frame.to_vec() }
    }

    fn order(direction: Direction, session: Option<&str>) -> LogRecord {
        let msg = Message::from_fields([(8, "FIX.4.4"), (35, "D"), (11, "B"), (55, "IBM"), (54, "1")]);
        record(direction, session, &crate::codec::encode(&msg).unwrap())
    }

    #[test]
    fn the_default_keeps_everything() {
        let filter = LogFilter::default();
        assert!(filter.matches(&order(Direction::Inbound, None)));
        assert!(filter.matches(&record(Direction::Outbound, None, b"\x00\x01fixp")));
        assert!(filter.matches(&LogRecord::Dropped { time: at(8, 0, 0), count: 1 }));
    }

    #[test]
    fn sessions_match_any_given_and_not_an_unknown_one() {
        let filter =
            LogFilter { sessions: vec!["FIX.4.4:A->B".into(), "FIX.4.4:US->THEM".into()], ..LogFilter::default() };
        assert!(filter.matches(&order(Direction::Inbound, Some("FIX.4.4:US->THEM"))));
        assert!(!filter.matches(&order(Direction::Inbound, Some("FIX.4.4:US->OTHER"))));
        assert!(!filter.matches(&order(Direction::Inbound, None)));
    }

    #[test]
    fn direction_matches() {
        let filter = LogFilter { direction: Some(Direction::Outbound), ..LogFilter::default() };
        assert!(filter.matches(&order(Direction::Outbound, None)));
        assert!(!filter.matches(&order(Direction::Inbound, None)));
    }

    /// A MsgType condition matches any type given, and nothing that isn't a decoded FIX message.
    #[test]
    fn msg_types_match_any_given() {
        let filter = LogFilter { msg_types: vec!["8".into(), "D".into()], ..LogFilter::default() };
        assert!(filter.matches(&order(Direction::Inbound, None)));
        let other = LogFilter { msg_types: vec!["8".into()], ..LogFilter::default() };
        assert!(!other.matches(&order(Direction::Inbound, None)));
        assert!(!filter.matches(&record(Direction::Inbound, None, b"\x00\x01fixp")));
        assert!(!filter.matches(&record(Direction::Inbound, None, b"8=FIX.4.4\x01garbled")));
    }

    /// Each tag condition holds if the field equals any of its values, and all must hold.
    #[test]
    fn tags_match_any_value_and_all_conditions() {
        let tags = |tags: &[(u32, &[&str])]| LogFilter {
            tags: tags.iter().map(|(t, vs)| (*t, vs.iter().map(|v| (*v).to_owned()).collect())).collect(),
            ..LogFilter::default()
        };
        let order = order(Direction::Inbound, None);
        assert!(tags(&[(11, &["A", "B", "C"])]).matches(&order));
        assert!(!tags(&[(11, &["A", "C"])]).matches(&order));
        assert!(tags(&[(11, &["A", "B"]), (55, &["IBM"])]).matches(&order));
        assert!(!tags(&[(11, &["A", "B"]), (55, &["MSFT"])]).matches(&order));
        assert!(!tags(&[(58, &["B"])]).matches(&order));
    }

    #[test]
    fn time_range_includes_since_and_excludes_until() {
        let filter = LogFilter { since: Some(at(8, 12, 0)), until: Some(at(8, 13, 0)), ..LogFilter::default() };
        assert!(filter.in_range(at(8, 12, 0)));
        assert!(!filter.in_range(at(8, 11, 59)));
        assert!(!filter.in_range(at(8, 13, 0)));
        assert!(filter.matches(&order(Direction::Inbound, None)));
        assert!(!filter.matches(&LogRecord::Dropped { time: at(8, 13, 0), count: 1 }));
    }

    fn days_covered(since: Option<DateTime<Utc>>, until: Option<DateTime<Utc>>) -> Vec<u32> {
        let filter = LogFilter { since, until, ..LogFilter::default() };
        (6..=10).filter(|&day| filter.covers_day(at(day, 0, 0).date_naive())).collect()
    }

    /// The files a range may be in: the days it touches, and the day after each, whose file may
    /// hold the end of the day before.
    #[test]
    fn covers_the_days_a_range_touches_and_the_next() {
        // Inside one day.
        assert_eq!(days_covered(Some(at(8, 9, 0)), Some(at(8, 17, 0))), [8, 9]);
        // Across midnight.
        assert_eq!(days_covered(Some(at(7, 23, 0)), Some(at(8, 1, 0))), [7, 8, 9]);
        // Until midnight exactly: the 8th's file may still hold the end of the 7th.
        assert_eq!(days_covered(Some(at(7, 9, 0)), Some(at(8, 0, 0))), [7, 8]);
        // Open-ended, either way or both.
        assert_eq!(days_covered(Some(at(8, 9, 0)), None), [8, 9, 10]);
        assert_eq!(days_covered(None, Some(at(8, 9, 0))), [6, 7, 8, 9]);
        assert_eq!(days_covered(None, None), [6, 7, 8, 9, 10]);
    }
}
