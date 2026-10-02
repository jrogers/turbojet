//! Session schedules: when a FIX session may be logged on, and when its sequence numbers reset.
//!
//! A schedule is a series of *session periods*, each with a start and an end, in a time zone:
//!
//! - **Daily**: `daily 08:00-17:00 mon-fri America/New_York`. An end at or before the start
//!   means the period ends the next day (`22:00-06:00`); `00:00-00:00` is a continuous 24-hour
//!   session that starts a new period every midnight. Days, if given, are the days a period
//!   *starts* on.
//! - **Weekly**: `weekly sun 17:00-fri 17:00 America/New_York`.
//!
//! With a schedule configured, an acceptor refuses logons outside a period, an initiator waits
//! for the next period before connecting, a logged-on session logs out when its period ends, and
//! the first logon of a new period resets sequence numbers to 1.
//!
//! Time zones are `UTC`, fixed offsets (`+05:30`, `-04:00`), or, with the `tz` feature, IANA names
//! (`America/New_York`) that follow daylight saving. A local start or end time that doesn't exist
//! (skipped when clocks go forward) is taken as the equivalent time on the old offset, i.e. just
//! after the gap; one that occurs twice (when clocks go back) is taken as the first occurrence.

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use chrono::{
    DateTime, Datelike, FixedOffset, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc, Weekday,
};

const WEEK: [Weekday; 7] =
    [Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat, Weekday::Sun];

/// A source of wall-clock time. [`Clock::system`] in production; tests can supply their own with
/// [`Clock::from_fn`] to exercise schedules without waiting.
#[derive(Clone)]
pub struct Clock(Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>);

impl Clock {
    /// The system clock.
    pub fn system() -> Self {
        Self(Arc::new(Utc::now))
    }

    /// A clock that reads the time from `now`.
    pub fn from_fn(now: impl Fn() -> DateTime<Utc> + Send + Sync + 'static) -> Self {
        Self(Arc::new(now))
    }

    /// The current time.
    pub fn now(&self) -> DateTime<Utc> {
        (self.0)()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::system()
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock({})", self.now())
    }
}

/// The time zone a schedule's times are in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleTimeZone {
    /// UTC.
    Utc,
    /// A fixed offset from UTC, with no daylight saving.
    Fixed(FixedOffset),
    /// An IANA zone, following its daylight-saving rules.
    #[cfg(feature = "tz")]
    Named(chrono_tz::Tz),
}

impl ScheduleTimeZone {
    fn to_local(self, time: DateTime<Utc>) -> NaiveDateTime {
        match self {
            Self::Utc => time.naive_utc(),
            Self::Fixed(offset) => time.with_timezone(&offset).naive_local(),
            #[cfg(feature = "tz")]
            Self::Named(tz) => time.with_timezone(&tz).naive_local(),
        }
    }

    fn to_utc(self, local: NaiveDateTime) -> DateTime<Utc> {
        match self {
            Self::Utc => local.and_utc(),
            Self::Fixed(offset) => resolve(&offset, local),
            #[cfg(feature = "tz")]
            Self::Named(tz) => resolve(&tz, local),
        }
    }
}

/// Converts local time to UTC, taking the first occurrence of an ambiguous time and resolving a
/// non-existent one with the offset in force just before the gap.
fn resolve<Tz: TimeZone>(tz: &Tz, local: NaiveDateTime) -> DateTime<Utc> {
    match tz.from_local_datetime(&local) {
        LocalResult::Single(time) | LocalResult::Ambiguous(time, _) => time.with_timezone(&Utc),
        LocalResult::None => {
            let before = tz
                .from_local_datetime(&(local - chrono::Duration::hours(3)))
                .earliest()
                .map(|t| t.offset().fix())
                .unwrap_or_else(|| FixedOffset::east_opt(0).expect("zero offset"));
            (local - chrono::Duration::seconds(i64::from(before.local_minus_utc()))).and_utc()
        }
    }
}

impl fmt::Display for ScheduleTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utc => f.write_str("UTC"),
            Self::Fixed(offset) => write!(f, "{offset}"),
            #[cfg(feature = "tz")]
            Self::Named(tz) => f.write_str(tz.name()),
        }
    }
}

impl FromStr for ScheduleTimeZone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s.eq_ignore_ascii_case("UTC") || s.eq_ignore_ascii_case("Z") {
            return Ok(Self::Utc);
        }
        if s.starts_with('+') || s.starts_with('-') {
            let offset = parse_offset(s).ok_or_else(|| format!("invalid UTC offset '{s}' (expected e.g. +05:30)"))?;
            return Ok(Self::Fixed(offset));
        }
        #[cfg(feature = "tz")]
        {
            s.parse::<chrono_tz::Tz>().map(Self::Named).map_err(|_| format!("unknown time zone '{s}'"))
        }
        #[cfg(not(feature = "tz"))]
        {
            Err(format!("named time zones like '{s}' need turbojet's `tz` feature; use UTC or an offset"))
        }
    }
}

/// `+HH:MM`, `-HH:MM` or `+HHMM`.
fn parse_offset(s: &str) -> Option<FixedOffset> {
    let sign = if s.starts_with('-') { -1 } else { 1 };
    let digits: String = s[1..].chars().filter(|c| *c != ':').collect();
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let hours: i32 = digits[..2].parse().ok()?;
    let minutes: i32 = digits[2..].parse().ok()?;
    if hours > 23 || minutes > 59 {
        return None;
    }
    FixedOffset::east_opt(sign * (hours * 3600 + minutes * 60))
}

/// One session period, in UTC. `start <= t < end` is inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    /// When the period begins, inclusive.
    pub start: DateTime<Utc>,
    /// When the period ends, exclusive.
    pub end: DateTime<Utc>,
}

impl Period {
    /// Whether `time` is inside the period.
    pub fn contains(&self, time: DateTime<Utc>) -> bool {
        self.start <= time && time < self.end
    }
}

/// Dates on which no session period starts, in the schedule's time zone. A period that starts the
/// day before a holiday and runs into it is unaffected.
///
/// Parses from text: one `YYYY-MM-DD` per line, with blank lines and `#` comments allowed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HolidayCalendar {
    dates: BTreeSet<NaiveDate>,
}

impl HolidayCalendar {
    /// A calendar of `dates`.
    pub fn new(dates: impl IntoIterator<Item = NaiveDate>) -> Self {
        Self { dates: dates.into_iter().collect() }
    }

    /// Whether `date` is a holiday.
    pub fn contains(&self, date: NaiveDate) -> bool {
        self.dates.contains(&date)
    }

    /// The latest holiday. After it, the calendar has no effect.
    pub fn last(&self) -> Option<NaiveDate> {
        self.dates.last().copied()
    }

    /// The number of holidays.
    pub fn len(&self) -> usize {
        self.dates.len()
    }

    /// Whether there are no holidays.
    pub fn is_empty(&self) -> bool {
        self.dates.is_empty()
    }
}

impl FromIterator<NaiveDate> for HolidayCalendar {
    fn from_iter<I: IntoIterator<Item = NaiveDate>>(dates: I) -> Self {
        Self::new(dates)
    }
}

impl FromStr for HolidayCalendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut dates = BTreeSet::new();
        for (index, line) in s.lines().enumerate() {
            let text = line.split_once('#').map_or(line, |(before, _)| before).trim();
            if text.is_empty() {
                continue;
            }
            let date = NaiveDate::parse_from_str(text, "%Y-%m-%d")
                .map_err(|_| format!("line {}: invalid date '{text}' (expected YYYY-MM-DD)", index + 1))?;
            dates.insert(date);
        }
        Ok(Self { dates })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Daily {
        start: NaiveTime,
        end: NaiveTime,
        /// Days a period may start on, indexed from Monday.
        days: [bool; 7],
    },
    Weekly {
        start_day: Weekday,
        start: NaiveTime,
        end_day: Weekday,
        end: NaiveTime,
    },
}

/// When a session may be logged on. Parse one from a string (see the [module docs](self)) or build
/// it with [`SessionSchedule::daily`] or [`SessionSchedule::weekly`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSchedule {
    kind: Kind,
    time_zone: ScheduleTimeZone,
}

impl SessionSchedule {
    /// A period every day from `start` to `end` (the next day if `end <= start`), in UTC.
    pub fn daily(start: NaiveTime, end: NaiveTime) -> Self {
        Self { kind: Kind::Daily { start, end, days: [true; 7] }, time_zone: ScheduleTimeZone::Utc }
    }

    /// A period each week from `start_day start` to the next `end_day end`, in UTC.
    pub fn weekly(start_day: Weekday, start: NaiveTime, end_day: Weekday, end: NaiveTime) -> Self {
        Self { kind: Kind::Weekly { start_day, start, end_day, end }, time_zone: ScheduleTimeZone::Utc }
    }

    /// Restricts a daily schedule to periods starting on `days`. Has no effect on weekly
    /// schedules.
    pub fn on_days(mut self, days: impl IntoIterator<Item = Weekday>) -> Self {
        if let Kind::Daily { days: allowed, .. } = &mut self.kind {
            *allowed = [false; 7];
            for day in days {
                allowed[day.num_days_from_monday() as usize] = true;
            }
        }
        self
    }

    /// Reads the schedule's days and times in `time_zone` instead of UTC.
    pub fn in_time_zone(mut self, time_zone: ScheduleTimeZone) -> Self {
        self.time_zone = time_zone;
        self
    }

    /// The period containing `time`, if any.
    pub fn period_at(&self, time: DateTime<Utc>) -> Option<Period> {
        self.periods_near(time).into_iter().filter(|p| p.contains(time)).max_by_key(|p| p.start)
    }

    /// Whether `time` falls inside a period.
    pub fn is_active(&self, time: DateTime<Utc>) -> bool {
        self.period_at(time).is_some()
    }

    /// The start of the first period beginning after `time`.
    pub fn next_start(&self, time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.periods_near(time).into_iter().map(|p| p.start).filter(|start| *start > time).min()
    }

    /// Periods starting from a little before `time` to more than a week after it: enough to find
    /// the period containing `time` and the next one to start.
    fn periods_near(&self, time: DateTime<Utc>) -> Vec<Period> {
        let today = self.time_zone.to_local(time).date();
        let local = |date: NaiveDate, at: NaiveTime| self.time_zone.to_utc(date.and_time(at));
        match &self.kind {
            Kind::Daily { start, end, days } => (-2..=8)
                .map(|offset| today + chrono::Duration::days(offset))
                .filter(|date| days[date.weekday().num_days_from_monday() as usize])
                .map(|date| {
                    let end_date = if end > start { date } else { date + chrono::Duration::days(1) };
                    Period { start: local(date, *start), end: local(end_date, *end) }
                })
                .collect(),
            Kind::Weekly { start_day, start, end_day, end } => {
                let since_start_day =
                    (7 + today.weekday().num_days_from_monday() - start_day.num_days_from_monday()) % 7;
                let anchor = today - chrono::Duration::days(i64::from(since_start_day));
                let span = (7 + end_day.num_days_from_monday() - start_day.num_days_from_monday()) % 7;
                (-1..=2)
                    .map(|week| anchor + chrono::Duration::weeks(week))
                    .map(|start_date| {
                        let mut end_date = start_date + chrono::Duration::days(i64::from(span));
                        if end_date.and_time(*end) <= start_date.and_time(*start) {
                            end_date += chrono::Duration::weeks(1);
                        }
                        Period { start: local(start_date, *start), end: local(end_date, *end) }
                    })
                    .collect()
            }
        }
    }
}

impl fmt::Display for SessionSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = |t: &NaiveTime| t.format("%H:%M:%S").to_string();
        let day = |d: &Weekday| d.to_string().to_lowercase();
        match &self.kind {
            Kind::Daily { start, end, days } => {
                write!(f, "daily {}-{}", time(start), time(end))?;
                if days != &[true; 7] {
                    let names: Vec<String> =
                        WEEK.iter().filter(|d| days[d.num_days_from_monday() as usize]).map(day).collect();
                    write!(f, " {}", names.join(","))?;
                }
            }
            Kind::Weekly { start_day, start, end_day, end } => {
                write!(f, "weekly {} {}-{} {}", day(start_day), time(start), day(end_day), time(end))?
            }
        }
        write!(f, " {}", self.time_zone)
    }
}

impl FromStr for SessionSchedule {
    type Err = String;

    /// Parses `daily HH:MM[:SS]-HH:MM[:SS] [DAYS] [ZONE]` or
    /// `weekly DAY HH:MM[:SS]-DAY HH:MM[:SS] [ZONE]`. DAYS is a comma-separated list of days or
    /// ranges (`mon-fri`, `mon,wed,fri`, `sun-thu`); ZONE defaults to UTC.
    fn from_str(s: &str) -> Result<Self, String> {
        let normalized = s.trim().replace(" - ", "-");
        let mut words = normalized.split_whitespace();
        let kind = words.next().ok_or("empty schedule")?.to_ascii_lowercase();
        let rest: Vec<&str> = words.collect();
        let (schedule, zone) = match kind.as_str() {
            "daily" => {
                let (range, rest) = rest.split_first().ok_or("daily schedule needs HH:MM-HH:MM")?;
                let (start, end) =
                    range.split_once('-').ok_or_else(|| format!("expected HH:MM-HH:MM, got '{range}'"))?;
                let mut schedule = Self::daily(parse_time(start)?, parse_time(end)?);
                let mut rest = rest;
                if let Some((first, after)) = rest.split_first()
                    && let Ok(days) = parse_days(first)
                {
                    schedule = schedule.on_days(days);
                    rest = after;
                }
                (schedule, rest)
            }
            "weekly" => {
                if rest.len() < 3 {
                    return Err("weekly schedule needs DAY HH:MM-DAY HH:MM".into());
                }
                let (start, end) = rest[1].split_once('-').ok_or("weekly schedule needs DAY HH:MM-DAY HH:MM")?;
                let schedule =
                    Self::weekly(parse_day(rest[0])?, parse_time(start)?, parse_day(end)?, parse_time(rest[2])?);
                (schedule, &rest[3..])
            }
            other => return Err(format!("schedule must start with 'daily' or 'weekly', not '{other}'")),
        };
        match zone {
            [] => Ok(schedule),
            [zone] => Ok(schedule.in_time_zone(zone.parse()?)),
            [_, extra, ..] => Err(format!("unexpected '{extra}' in schedule")),
        }
    }
}

fn parse_time(s: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
        .map_err(|_| format!("invalid time '{s}' (expected HH:MM or HH:MM:SS)"))
}

fn parse_day(s: &str) -> Result<Weekday, String> {
    s.parse::<Weekday>().map_err(|_| format!("invalid day '{s}'"))
}

/// `mon-fri`, `mon,wed,fri`, `fri-mon` (wrapping), or a mix.
fn parse_days(s: &str) -> Result<Vec<Weekday>, String> {
    let mut days = Vec::new();
    for item in s.split(',') {
        match item.split_once('-') {
            Some((from, to)) => {
                let (mut day, to) = (parse_day(from)?, parse_day(to)?);
                loop {
                    days.push(day);
                    if day == to {
                        break;
                    }
                    day = day.succ();
                }
            }
            None => days.push(parse_day(item)?),
        }
    }
    Ok(days)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> DateTime<Utc> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").unwrap().and_utc()
    }

    fn schedule(s: &str) -> SessionSchedule {
        s.parse().unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    // 2026-09-28 is a Monday.

    #[test]
    fn daily_period_within_a_day() {
        let s = schedule("daily 08:00-17:00");
        assert!(!s.is_active(at("2026-09-28 07:59")));
        assert_eq!(
            s.period_at(at("2026-09-28 08:00")),
            Some(Period { start: at("2026-09-28 08:00"), end: at("2026-09-28 17:00") })
        );
        assert!(!s.is_active(at("2026-09-28 17:00")), "end is exclusive");
        assert_eq!(s.next_start(at("2026-09-28 17:00")), Some(at("2026-09-29 08:00")));
        assert_eq!(s.next_start(at("2026-09-28 07:00")), Some(at("2026-09-28 08:00")));
    }

    #[test]
    fn daily_period_across_midnight() {
        let s = schedule("daily 22:00-06:00");
        assert_eq!(s.period_at(at("2026-09-29 03:00")).unwrap().start, at("2026-09-28 22:00"));
        assert_eq!(s.period_at(at("2026-09-28 23:00")).unwrap().end, at("2026-09-29 06:00"));
        assert!(!s.is_active(at("2026-09-29 12:00")));
    }

    #[test]
    fn continuous_daily_session_starts_a_new_period_each_day() {
        let s = schedule("daily 00:00-00:00");
        let before = s.period_at(at("2026-09-28 23:59")).unwrap();
        let after = s.period_at(at("2026-09-29 00:00")).unwrap();
        assert!(s.is_active(at("2026-09-28 23:59")) && s.is_active(at("2026-09-29 00:00")));
        assert_ne!(before, after);
        assert_eq!(after.start, at("2026-09-29 00:00"));
    }

    #[test]
    fn daily_days_are_the_days_a_period_starts_on() {
        let s = schedule("daily 08:00-17:00 mon-fri");
        assert!(s.is_active(at("2026-10-02 10:00")), "Friday");
        assert!(!s.is_active(at("2026-10-03 10:00")), "Saturday");
        assert_eq!(s.next_start(at("2026-10-02 18:00")), Some(at("2026-10-05 08:00")), "Friday evening to Monday");
        // An overnight period starting Friday runs into Saturday.
        let overnight = schedule("daily 22:00-06:00 mon-fri");
        assert!(overnight.is_active(at("2026-10-03 05:00")));
        assert!(!overnight.is_active(at("2026-10-04 05:00")), "no period starts on Saturday");
    }

    #[test]
    fn weekly_period_across_the_weekend_boundary() {
        let s = schedule("weekly sun 17:00-fri 17:00");
        let period = s.period_at(at("2026-09-30 12:00")).unwrap(); // Wednesday
        assert_eq!(period, Period { start: at("2026-09-27 17:00"), end: at("2026-10-02 17:00") });
        assert!(!s.is_active(at("2026-10-03 12:00")), "Saturday");
        assert_eq!(s.next_start(at("2026-10-03 12:00")), Some(at("2026-10-04 17:00")));
        assert!(s.is_active(at("2026-10-04 17:00")));
        // Same start and end: a continuous week, restarting each Sunday 17:00.
        let continuous = schedule("weekly sun 17:00-sun 17:00");
        assert_eq!(continuous.period_at(at("2026-10-04 16:59")).unwrap().end, at("2026-10-04 17:00"));
    }

    #[test]
    fn fixed_offsets_shift_the_period() {
        let s = schedule("daily 08:00-17:00 -05:00");
        assert_eq!(s.period_at(at("2026-09-28 14:00")).unwrap().start, at("2026-09-28 13:00"));
        let s = schedule("daily 09:15-15:30 +05:30");
        assert_eq!(s.period_at(at("2026-09-28 04:00")).unwrap().start, at("2026-09-28 03:45"));
    }

    #[cfg(feature = "tz")]
    #[test]
    fn named_zones_follow_daylight_saving() {
        let s = schedule("daily 09:30-16:00 America/New_York");
        // EDT (UTC-4) in September, EST (UTC-5) in December.
        assert_eq!(s.period_at(at("2026-09-28 14:00")).unwrap().start, at("2026-09-28 13:30"));
        assert_eq!(s.period_at(at("2026-12-01 15:00")).unwrap().start, at("2026-12-01 14:30"));
        // 02:30 doesn't exist on 2026-03-08 in New York: taken as just after the gap (03:30 EDT).
        let gap = schedule("daily 02:30-04:00 America/New_York");
        assert_eq!(gap.period_at(at("2026-03-08 07:40")).unwrap().start, at("2026-03-08 07:30"));
        // 01:30 happens twice on 2026-11-01: the first (EDT) is used.
        let repeated = schedule("daily 01:30-03:00 America/New_York");
        assert_eq!(repeated.period_at(at("2026-11-01 06:00")).unwrap().start, at("2026-11-01 05:30"));
    }

    #[test]
    fn parses_and_displays_round_trip() {
        for text in [
            "daily 08:00:00-17:00:00 UTC",
            "daily 22:00:00-06:00:00 mon,tue,wed,thu,fri +05:30",
            "weekly sun 17:00:00-fri 17:00:00 -05:00",
        ] {
            assert_eq!(schedule(text).to_string(), text);
            assert_eq!(schedule(&schedule(text).to_string()), schedule(text));
        }
        assert_eq!(schedule("weekly sun 17:00 - fri 17:00"), schedule("weekly sun 17:00-fri 17:00 UTC"));
        assert_eq!(schedule("daily 08:00-17:00 fri-mon"), schedule("daily 08:00-17:00 fri,sat,sun,mon"));
        assert_eq!(schedule("DAILY 8:00-17:00 Mon-Fri utc"), schedule("daily 08:00-17:00 mon-fri"));
    }

    #[test]
    fn rejects_malformed_schedules() {
        for (text, expected) in [
            ("", "empty"),
            ("hourly 08:00-17:00", "'daily' or 'weekly'"),
            ("daily 08:00", "HH:MM-HH:MM"),
            ("daily 25:00-17:00", "invalid time"),
            ("weekly sun 17:00", "DAY HH:MM-DAY HH:MM"),
            ("weekly xyz 17:00-fri 17:00", "invalid day"),
            ("daily 08:00-17:00 +25:00", "invalid UTC offset"),
            ("daily 08:00-17:00 mon-fri UTC extra", "unexpected 'extra'"),
        ] {
            let err = text.parse::<SessionSchedule>().unwrap_err();
            assert!(err.contains(expected), "{text}: {err}");
        }
        #[cfg(not(feature = "tz"))]
        assert!(schedule_err("daily 08:00-17:00 Europe/London").contains("`tz` feature"));
    }

    #[cfg(not(feature = "tz"))]
    fn schedule_err(s: &str) -> String {
        s.parse::<SessionSchedule>().unwrap_err()
    }

    fn date(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn holiday_calendars_parse_one_date_per_line() {
        let calendar: HolidayCalendar =
            "# NYSE 2026\n2026-12-25\n\n  2026-11-26  # Thanksgiving\n2026-12-25\n".parse().unwrap();
        assert!(calendar.contains(date("2026-12-25")));
        assert!(calendar.contains(date("2026-11-26")));
        assert!(!calendar.contains(date("2026-12-24")));
        assert_eq!(calendar.last(), Some(date("2026-12-25")));
        assert_eq!(calendar.len(), 2, "duplicates collapse");
        assert_eq!(calendar, HolidayCalendar::new([date("2026-11-26"), date("2026-12-25")]));
        assert!("".parse::<HolidayCalendar>().unwrap().is_empty());
    }

    #[test]
    fn holiday_calendars_reject_bad_dates_with_their_line() {
        let err = "2026-12-25\n2026-13-01\n".parse::<HolidayCalendar>().unwrap_err();
        assert!(err.contains("line 2"), "{err}");
        assert!(err.contains("'2026-13-01'"), "{err}");
        let err = "2026-12-25 2026-12-26".parse::<HolidayCalendar>().unwrap_err();
        assert!(err.contains("line 1"), "{err}");
    }
}
