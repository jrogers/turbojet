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
//! Holidays ([`HolidayCalendar`], set with [`SessionSchedule::with_holidays`]) are dates, in the
//! schedule's time zone, on which no period starts. A period that starts the day before a holiday
//! still runs into it; a weekly schedule skips only a week that starts on a holiday.
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

/// Most candidate periods a search looks through: over a year of daily periods (weekly schedules:
/// over seven years of weeks), enough to cross any run of holidays, while a calendar that closes
/// every day still ends the search.
const MAX_CANDIDATES: usize = 400;
// A daily search covers over a year after its 2-day look-back.
const _: () = assert!(MAX_CANDIDATES > 366 + 2);

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

    /// Whether `other` is this clock, or a clone of it.
    pub(crate) fn same_as(&self, other: &Clock) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
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
        let utc = match self {
            Self::Utc => local.and_utc(),
            Self::Fixed(offset) => resolve(&offset, local),
            #[cfg(feature = "tz")]
            Self::Named(tz) => resolve(&tz, local),
        };
        // Back to local time, it's the time asked for, or just after the gap it falls in.
        debug_assert!(self.to_local(utc) >= local, "{local} resolves to {utc}");
        debug_assert!(!self.exists(local) || self.to_local(utc) == local, "{local} resolves to {utc}");
        utc
    }

    /// Whether `local` occurs in the zone, at least once: not in a gap where clocks go forward.
    fn exists(self, local: NaiveDateTime) -> bool {
        match self {
            Self::Utc | Self::Fixed(_) => true,
            #[cfg(feature = "tz")]
            Self::Named(tz) => !matches!(tz.from_local_datetime(&local), LocalResult::None),
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
///
/// Holidays aren't part of the text form: [`Display`](fmt::Display) leaves them out, and parsing
/// gives a schedule without any. Add them with [`SessionSchedule::with_holidays`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSchedule {
    kind: Kind,
    time_zone: ScheduleTimeZone,
    holidays: HolidayCalendar,
}

impl SessionSchedule {
    /// A period every day from `start` to `end` (the next day if `end <= start`), in UTC.
    pub fn daily(start: NaiveTime, end: NaiveTime) -> Self {
        Self {
            kind: Kind::Daily { start, end, days: [true; 7] },
            time_zone: ScheduleTimeZone::Utc,
            holidays: HolidayCalendar::default(),
        }
    }

    /// A period each week from `start_day start` to the next `end_day end`, in UTC.
    pub fn weekly(start_day: Weekday, start: NaiveTime, end_day: Weekday, end: NaiveTime) -> Self {
        Self {
            kind: Kind::Weekly { start_day, start, end_day, end },
            time_zone: ScheduleTimeZone::Utc,
            holidays: HolidayCalendar::default(),
        }
    }

    /// Restricts a daily schedule to periods starting on `days`. Has no effect on weekly
    /// schedules.
    #[must_use]
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
    #[must_use]
    pub fn in_time_zone(mut self, time_zone: ScheduleTimeZone) -> Self {
        self.time_zone = time_zone;
        self
    }

    /// Starts no period on these dates, read in the schedule's time zone. For a weekly schedule,
    /// only a holiday on its start day matters: it skips that week's period.
    #[must_use]
    pub fn with_holidays(mut self, holidays: HolidayCalendar) -> Self {
        self.holidays = holidays;
        self
    }

    /// The dates on which no period starts.
    pub fn holidays(&self) -> &HolidayCalendar {
        &self.holidays
    }

    /// The period containing `time`, if any.
    pub fn period_at(&self, time: DateTime<Utc>) -> Option<Period> {
        self.periods_around(time).take_while(|p| p.start <= time).filter(|p| p.contains(time)).last()
    }

    /// Whether `time` falls inside a period.
    pub fn is_active(&self, time: DateTime<Utc>) -> bool {
        self.period_at(time).is_some()
    }

    /// The start of the first period beginning after `time`, if one begins within about 400 days
    /// (weekly schedules: weeks).
    pub fn next_start(&self, time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.periods_around(time).map(|p| p.start).find(|start| *start > time)
    }

    /// Why the schedule doesn't allow a session at `time`, naming the holiday if one is the reason;
    /// `None` inside a period.
    pub(crate) fn closed_reason(&self, time: DateTime<Utc>) -> Option<String> {
        if self.is_active(time) {
            return None;
        }
        let holiday = self.holiday_at(time).map(|date| format!("; {date} is a holiday")).unwrap_or_default();
        Some(match self.next_start(time) {
            Some(next) => format!("outside session time (schedule '{self}'{holiday}); next session starts {next}"),
            None => format!("outside session time (schedule '{self}'{holiday})"),
        })
    }

    /// The holiday that removed the period `time` would otherwise be in, if any. The date comes
    /// from the calendar, so it is the period's start date even where a start in a DST gap
    /// resolves to the next local date.
    fn holiday_at(&self, time: DateTime<Utc>) -> Option<NaiveDate> {
        let today = self.time_zone.to_local(time).date();
        self.holidays
            .dates
            .range(today - chrono::Duration::days(self.look_back())..=today)
            .rev()
            .copied()
            .filter(|date| self.regular_start(*date))
            .find(|date| self.period_starting(*date).contains(time))
    }

    /// Periods in start order, from early enough that the first can still contain `time` (a daily
    /// period lasts at most a day, a weekly one at most a week).
    fn periods_around(&self, time: DateTime<Utc>) -> impl Iterator<Item = Period> + '_ {
        let today = self.time_zone.to_local(time).date();
        self.periods_from(today - chrono::Duration::days(self.look_back()))
    }

    /// How many days before `time`'s local date a period containing it can start: a period's own
    /// length (a day, or a week), plus a day's margin.
    fn look_back(&self) -> i64 {
        match self.kind {
            Kind::Daily { .. } => 2,
            Kind::Weekly { .. } => 8,
        }
    }

    /// Periods in start order, starting on `first` (a local date) or later, at most
    /// `MAX_CANDIDATES` days (weekly: weeks) of them.
    fn periods_from(&self, first: NaiveDate) -> impl Iterator<Item = Period> + '_ {
        let (first, step) = match &self.kind {
            Kind::Daily { .. } => (first, chrono::Duration::days(1)),
            Kind::Weekly { start_day, .. } => {
                let ahead = (7 + start_day.num_days_from_monday() - first.weekday().num_days_from_monday()) % 7;
                (first + chrono::Duration::days(i64::from(ahead)), chrono::Duration::weeks(1))
            }
        };
        std::iter::successors(Some(first), move |date| Some(*date + step))
            .take(MAX_CANDIDATES)
            .filter(|date| self.starts_on(*date))
            .map(|date| self.period_starting(date))
            // `period_at` stops at the first period starting after its time, and `next_start`
            // takes the first one: both rely on this order.
            .scan(None, |previous: &mut Option<DateTime<Utc>>, period| {
                debug_assert!(previous.is_none_or(|start| start < period.start), "periods come in start order");
                *previous = Some(period.start);
                Some(period)
            })
    }

    /// Whether a period starts on `date`, a local date: never on a holiday, otherwise as
    /// [`regular_start`](Self::regular_start) says.
    fn starts_on(&self, date: NaiveDate) -> bool {
        !self.holidays.contains(date) && self.regular_start(date)
    }

    /// Whether a period would start on `date`, a local date, ignoring holidays: for a daily
    /// schedule one of its days, for a weekly one its start day.
    fn regular_start(&self, date: NaiveDate) -> bool {
        match &self.kind {
            Kind::Daily { days, .. } => days[date.weekday().num_days_from_monday() as usize],
            Kind::Weekly { start_day, .. } => date.weekday() == *start_day,
        }
    }

    /// The period starting on `date`, a local date the schedule starts a period on.
    fn period_starting(&self, date: NaiveDate) -> Period {
        let period = self.period_on(date);
        // `periods_around` looks back this far for a period containing a time. A period can end
        // before it starts, where its start falls in a DST gap: it's then never in session.
        debug_assert!(period.end - period.start < chrono::Duration::days(self.look_back()), "{period:?} is too long");
        period
    }

    fn period_on(&self, date: NaiveDate) -> Period {
        let local = |date: NaiveDate, at: NaiveTime| self.time_zone.to_utc(date.and_time(at));
        match &self.kind {
            Kind::Daily { start, end, .. } => {
                let end_date = if end > start { date } else { date + chrono::Duration::days(1) };
                Period { start: local(date, *start), end: local(end_date, *end) }
            }
            Kind::Weekly { start_day, start, end_day, end } => {
                debug_assert_eq!(date.weekday(), *start_day);
                let span = (7 + end_day.num_days_from_monday() - start_day.num_days_from_monday()) % 7;
                let mut end_date = date + chrono::Duration::days(i64::from(span));
                if end_date.and_time(*end) <= date.and_time(*start) {
                    end_date += chrono::Duration::weeks(1);
                }
                Period { start: local(date, *start), end: local(end_date, *end) }
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
        let holidays = closed("daily 08:00-17:00 mon-fri", &["2026-12-25"]);
        assert_eq!(holidays.to_string(), "daily 08:00:00-17:00:00 mon,tue,wed,thu,fri UTC", "Display omits holidays");
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

    fn closed(s: &str, dates: &[&str]) -> SessionSchedule {
        schedule(s).with_holidays(dates.iter().map(|d| date(d)).collect())
    }

    // 2026-12-25 is a Friday.

    #[test]
    fn no_period_starts_on_a_holiday() {
        let s = closed("daily 08:00-17:00 mon-fri", &["2026-12-25"]);
        assert!(s.is_active(at("2026-12-24 10:00")));
        assert!(!s.is_active(at("2026-12-25 10:00")));
        assert_eq!(s.next_start(at("2026-12-24 18:00")), Some(at("2026-12-28 08:00")), "over the holiday and weekend");
    }

    #[test]
    fn next_start_crosses_a_long_run_of_holidays() {
        let days: Vec<String> = (21..=31).map(|d| format!("2026-12-{d}")).collect();
        let days: Vec<&str> = days.iter().map(String::as_str).collect();
        let s = closed("daily 08:00-17:00 mon-fri", &days);
        assert_eq!(s.next_start(at("2026-12-18 18:00")), Some(at("2027-01-01 08:00")));
    }

    #[test]
    fn a_period_running_into_a_holiday_still_happens() {
        let s = closed("daily 22:00-06:00", &["2026-12-25"]);
        assert!(s.is_active(at("2026-12-25 05:00")), "started on the 24th");
        assert!(!s.is_active(at("2026-12-25 23:00")), "would have started on the 25th");
        assert_eq!(s.next_start(at("2026-12-25 06:00")), Some(at("2026-12-26 22:00")));
    }

    #[test]
    fn weekly_schedules_skip_only_a_holiday_on_their_start_day() {
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-20", "2026-12-25"]);
        assert!(!s.is_active(at("2026-12-22 12:00")), "the week starting on the 20th is skipped");
        assert_eq!(s.next_start(at("2026-12-22 12:00")), Some(at("2026-12-27 17:00")));
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-23"]);
        assert!(s.is_active(at("2026-12-23 12:00")), "a mid-week holiday changes nothing");
    }

    #[test]
    fn closed_reasons_name_the_holiday() {
        let s = closed("daily 08:00-17:00 mon-fri", &["2026-12-25"]);
        assert_eq!(s.closed_reason(at("2026-12-24 10:00")), None);
        assert_eq!(
            s.closed_reason(at("2026-12-25 10:00")).unwrap(),
            "outside session time (schedule 'daily 08:00:00-17:00:00 mon,tue,wed,thu,fri UTC'; \
             2026-12-25 is a holiday); next session starts 2026-12-28 08:00:00 UTC"
        );
        // Evening of the holiday: outside hours anyway, so no holiday is named.
        assert_eq!(
            s.closed_reason(at("2026-12-25 18:00")).unwrap(),
            "outside session time (schedule 'daily 08:00:00-17:00:00 mon,tue,wed,thu,fri UTC'); \
             next session starts 2026-12-28 08:00:00 UTC"
        );
        let weekly = closed("weekly sun 17:00-fri 17:00", &["2026-12-20"]);
        let reason = weekly.closed_reason(at("2026-12-22 12:00")).unwrap();
        assert!(reason.contains("; 2026-12-20 is a holiday)"), "{reason}");
    }

    #[cfg(feature = "tz")]
    #[test]
    fn closed_reasons_name_the_start_date_of_a_period_in_a_gap_over_midnight() {
        // Nuuk springs forward from 23:00 on Saturday 2026-03-28 to 00:00 on the Sunday, so the
        // 28th's 23:30 start doesn't exist and is taken as 00:30 on the 29th (01:30 UTC).
        let s = closed("daily 23:30-23:45 America/Nuuk", &["2026-03-28"]);
        let reason = s.closed_reason(at("2026-03-29 01:40")).unwrap();
        assert!(reason.contains("2026-03-28 is a holiday"), "{reason}");
    }

    #[test]
    fn a_calendar_closing_every_day_has_no_next_start() {
        let first = date("2026-12-01");
        let every_day = (0..800).map(|d| first + chrono::Duration::days(d)).collect();
        let s = schedule("daily 08:00-17:00").with_holidays(every_day);
        assert_eq!(s.period_at(at("2026-12-02 10:00")), None);
        assert_eq!(s.next_start(at("2026-12-02 10:00")), None);
    }

    #[cfg(feature = "tz")]
    #[test]
    fn holidays_are_dates_in_the_schedules_time_zone() {
        // 04:00 UTC on the 25th is 23:00 on the 24th in New York, inside the 24th's period;
        // 04:00 UTC on the 26th is 23:00 on the 25th, when the 25th's period would have run.
        let s = closed("daily 22:00-06:00 America/New_York", &["2026-12-25"]);
        assert!(s.is_active(at("2026-12-25 04:00")), "the 24th's period, until 06:00 New York");
        assert!(!s.is_active(at("2026-12-26 04:00")));
    }
}
