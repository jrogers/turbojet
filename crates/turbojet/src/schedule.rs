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
//! schedule's time zone, on which no period starts. A daily period that starts the day before a
//! holiday still runs into it. A weekly schedule skips a week that starts on a holiday, and is
//! closed for the day on a holiday later in the week: it reopens at midnight after it, in the
//! same period, so sequence numbers don't reset.
//!
//! A calendar's early closes are a date and a time: the daily period in session then ends then,
//! while one that starts later that day (an evening reopen) runs as usual; a weekly period is
//! closed from then until midnight, in the same period, as for a holiday.
//!
//! With a schedule configured, an acceptor refuses logons outside a period, an initiator waits
//! for the next period before connecting, a logged-on session logs out when its period ends, and
//! the first logon of a new period resets sequence numbers to 1.
//!
//! Time zones are `UTC`, fixed offsets (`+05:30`, `-04:00`), or, with the `tz` feature, IANA names
//! (`America/New_York`) that follow daylight saving. A local start or end time that doesn't exist
//! (skipped when clocks go forward) is taken as the moment the clocks skip to, the end of the gap,
//! however long the gap (Samoa skipped all of 2011-12-30); one that occurs twice (when clocks go
//! back) is taken as the first occurrence. So a period that lies inside the gap doesn't happen
//! that day: `daily 02:30-03:00 America/New_York` starts as it ends on the night clocks go forward.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use crate::session::ConfigError;
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday};

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
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use chrono::{TimeZone, Utc};
/// use turbojet::{Clock, SessionConfig};
///
/// // A clock a test moves by hand, to see a schedule's end without waiting for it.
/// let now = Arc::new(Mutex::new(Utc.with_ymd_and_hms(2026, 10, 26, 16, 59, 0).unwrap()));
/// let mut config = SessionConfig::new("FIX.4.4", "VENUE");
/// config.clock = Clock::from_fn({
///     let now = now.clone();
///     move || *now.lock().unwrap()
/// });
/// *now.lock().unwrap() += chrono::Duration::minutes(2);
/// ```
#[derive(Clone)]
pub struct Clock(Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>);

impl Clock {
    /// The system clock. Every call gives the same clock, so configurations and registries
    /// left on the default agree on it.
    pub fn system() -> Self {
        static SYSTEM: std::sync::LazyLock<Clock> = std::sync::LazyLock::new(|| Clock(Arc::new(Utc::now)));
        SYSTEM.clone()
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
#[non_exhaustive]
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
    #[cfg_attr(not(feature = "tz"), expect(unused_variables, reason = "only named zones have gaps"))]
    fn exists(self, local: NaiveDateTime) -> bool {
        match self {
            Self::Utc | Self::Fixed(_) => true,
            #[cfg(feature = "tz")]
            Self::Named(tz) => !matches!(tz.from_local_datetime(&local), chrono::LocalResult::None),
        }
    }
}

/// Converts local time to UTC, taking the first occurrence of an ambiguous time, and for a
/// non-existent one the moment the clocks skip to: the first local second after the gap.
fn resolve<Tz: TimeZone>(tz: &Tz, local: NaiveDateTime) -> DateTime<Utc> {
    let exists = |local: &NaiveDateTime| tz.from_local_datetime(local).earliest();
    if let Some(time) = exists(&local) {
        return time.with_timezone(&Utc);
    }
    // No gap is longer than a day (Samoa's was one), so one ends within two: bisect, in whole
    // seconds after `local`, for its end.
    let at = |seconds: i64| local + chrono::Duration::seconds(seconds);
    let (mut skipped, mut after) = (0, 2 * 24 * 3600);
    while after - skipped > 1 {
        let middle = skipped + (after - skipped) / 2;
        if exists(&at(middle)).is_some() {
            after = middle;
        } else {
            skipped = middle;
        }
    }
    exists(&at(after)).expect("a gap ends within two days").with_timezone(&Utc)
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
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, ConfigError> {
        parse_time_zone(s).map_err(ConfigError::new)
    }
}

/// [`ScheduleTimeZone::from_str`], its error as text.
fn parse_time_zone(s: &str) -> Result<ScheduleTimeZone, String> {
    if s.eq_ignore_ascii_case("UTC") || s.eq_ignore_ascii_case("Z") {
        return Ok(ScheduleTimeZone::Utc);
    }
    if s.starts_with('+') || s.starts_with('-') {
        let offset = parse_offset(s).ok_or_else(|| format!("invalid UTC offset '{s}' (expected e.g. +05:30)"))?;
        return Ok(ScheduleTimeZone::Fixed(offset));
    }
    #[cfg(feature = "tz")]
    {
        s.parse::<chrono_tz::Tz>().map(ScheduleTimeZone::Named).map_err(|_| format!("unknown time zone '{s}'"))
    }
    #[cfg(not(feature = "tz"))]
    {
        Err(format!("named time zones like '{s}' need turbojet's `tz` feature; use UTC or an offset"))
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

/// Dates on which no session period starts, and dates the session closes early, in the
/// schedule's time zone. A daily period that starts the day before a holiday and runs into it is
/// unaffected; a weekly period is closed for the day. An early close ends the daily period in
/// session at that time, and closes a weekly period from then until midnight.
///
/// Parses from text: one `YYYY-MM-DD` per line for a holiday, or `YYYY-MM-DD HH:MM[:SS]` for an
/// early close, with blank lines and `#` comments allowed.
///
/// ```
/// use chrono::{NaiveDate, NaiveTime};
/// use turbojet::HolidayCalendar;
///
/// let from_text: HolidayCalendar = "# New York\n2026-11-26\n2026-11-27 13:00\n2026-12-25\n".parse()?;
/// let day = |m, d| NaiveDate::from_ymd_opt(2026, m, d).unwrap();
/// let from_dates = HolidayCalendar::new([day(11, 26), day(12, 25)])
///     .with_early_closes([(day(11, 27), NaiveTime::from_hms_opt(13, 0, 0).unwrap())]);
/// assert_eq!(from_text, from_dates);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HolidayCalendar {
    dates: BTreeSet<NaiveDate>,
    /// When the session closes on each of these dates, not otherwise holidays.
    early_closes: BTreeMap<NaiveDate, NaiveTime>,
}

impl HolidayCalendar {
    /// A calendar of holidays on `dates`.
    pub fn new(dates: impl IntoIterator<Item = NaiveDate>) -> Self {
        Self { dates: dates.into_iter().collect(), early_closes: BTreeMap::new() }
    }

    /// Adds early closes: on each date, the session closes at that time. A date that's also a
    /// holiday stays a holiday, closed all day.
    #[must_use]
    pub fn with_early_closes(mut self, closes: impl IntoIterator<Item = (NaiveDate, NaiveTime)>) -> Self {
        self.early_closes.extend(closes);
        self.early_closes.retain(|date, _| !self.dates.contains(date));
        self
    }

    /// Whether `date` is a holiday, closed all day.
    pub fn contains(&self, date: NaiveDate) -> bool {
        self.dates.contains(&date)
    }

    /// When the session closes on `date`, if it closes early then.
    pub fn early_close(&self, date: NaiveDate) -> Option<NaiveTime> {
        self.early_closes.get(&date).copied()
    }

    /// The latest holiday or early close. After it, the calendar has no effect.
    pub fn last(&self) -> Option<NaiveDate> {
        self.dates.last().copied().max(self.early_closes.keys().next_back().copied())
    }

    /// The number of holidays and early closes.
    pub fn len(&self) -> usize {
        self.dates.len() + self.early_closes.len()
    }

    /// Whether there are no holidays or early closes.
    pub fn is_empty(&self) -> bool {
        self.dates.is_empty() && self.early_closes.is_empty()
    }

    /// Dates from `first` to `last` closed for all or part of the day, in order.
    fn closed_dates(&self, first: NaiveDate, last: NaiveDate) -> impl Iterator<Item = NaiveDate> + '_ {
        let mut dates: Vec<NaiveDate> = self
            .dates
            .range(first..=last)
            .chain(self.early_closes.range(first..=last).map(|(date, _)| date))
            .copied()
            .collect();
        dates.sort_unstable();
        dates.into_iter()
    }
}

impl FromIterator<NaiveDate> for HolidayCalendar {
    fn from_iter<I: IntoIterator<Item = NaiveDate>>(dates: I) -> Self {
        Self::new(dates)
    }
}

impl FromStr for HolidayCalendar {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, ConfigError> {
        parse_calendar(s).map_err(ConfigError::new)
    }
}

/// [`HolidayCalendar::from_str`], its error as text.
fn parse_calendar(s: &str) -> Result<HolidayCalendar, String> {
    let mut calendar = HolidayCalendar::default();
    for (index, line) in s.lines().enumerate() {
        let text = line.split_once('#').map_or(line, |(before, _)| before).trim();
        let line = index + 1;
        let mut words = text.split_whitespace();
        let Some(day) = words.next() else { continue };
        let date = NaiveDate::parse_from_str(day, "%Y-%m-%d")
            .map_err(|_| format!("line {line}: invalid date '{day}' (expected YYYY-MM-DD)"))?;
        let close = words.next().map(|time| parse_time(time).map_err(|e| format!("line {line}: {e}"))).transpose()?;
        if let Some(extra) = words.next() {
            return Err(format!("line {line}: unexpected '{extra}' (expected YYYY-MM-DD [HH:MM])"));
        }
        let earlier = (calendar.dates.contains(&date), calendar.early_closes.get(&date));
        match (close, earlier) {
            (None, (_, None)) => {
                calendar.dates.insert(date);
            }
            (Some(close), (false, None)) => {
                calendar.early_closes.insert(date, close);
            }
            (Some(close), (false, Some(earlier))) if *earlier == close => {}
            _ => return Err(format!("line {line}: {date} is already a holiday or an early close at another time")),
        }
    }
    Ok(calendar)
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
///
/// ```
/// use turbojet::{HolidayCalendar, SessionConfig, SessionSchedule};
///
/// let schedule: SessionSchedule = "daily 08:00-17:00 mon-fri UTC".parse()?;
/// let weekly: SessionSchedule = "weekly sun 17:00-fri 17:00 UTC".parse()?;
/// let mut config = SessionConfig::new("FIX.4.4", "VENUE");
/// config.schedule = Some(schedule.with_holidays("2026-12-25\n2027-01-01".parse::<HolidayCalendar>()?));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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

    /// Starts no period on these dates, read in the schedule's time zone, and closes early on the
    /// calendar's early closes. A weekly schedule skips a week that starts on a holiday, and is
    /// closed for the day on one later in the week, or from an early close until midnight: the
    /// period goes on after midnight, so sequence numbers don't reset.
    #[must_use]
    pub fn with_holidays(mut self, holidays: HolidayCalendar) -> Self {
        self.holidays = holidays;
        self
    }

    /// The dates on which no period starts.
    pub fn holidays(&self) -> &HolidayCalendar {
        &self.holidays
    }

    /// The period containing `time`, if any: none while a holiday or an early close closes a
    /// weekly period.
    pub fn period_at(&self, time: DateTime<Utc>) -> Option<Period> {
        let period = self.scheduled_period(time)?;
        (!self.closed_mid_period(time)).then_some(period)
    }

    /// The period containing `time`, even on a holiday that closes it for the day.
    fn scheduled_period(&self, time: DateTime<Utc>) -> Option<Period> {
        self.periods_around(time).take_while(|p| p.start <= time).filter(|p| p.contains(time)).last()
    }

    /// Whether a weekly period under way is closed at `time`: on a holiday, or after an early
    /// close until midnight. A daily period that runs into a holiday is left alone, and one an
    /// early close ends simply ends then (see [`close_early`](Self::close_early)).
    fn closed_mid_period(&self, time: DateTime<Utc>) -> bool {
        if !matches!(self.kind, Kind::Weekly { .. }) {
            return false;
        }
        let local = self.time_zone.to_local(time);
        self.holidays.contains(local.date())
            || self
                .holidays
                .early_close(local.date())
                .is_some_and(|close| self.time_zone.to_utc(local.date().and_time(close)) <= time)
    }

    /// The first early close inside a daily `period`, as its date, local time and instant.
    fn early_close_in(&self, period: Period) -> Option<(NaiveDate, NaiveTime, DateTime<Utc>)> {
        if !matches!(self.kind, Kind::Daily { .. }) {
            return None;
        }
        let (first, last) = (self.time_zone.to_local(period.start).date(), self.time_zone.to_local(period.end).date());
        self.holidays
            .early_closes
            .range(first..=last)
            .map(|(date, close)| (*date, *close, self.time_zone.to_utc(date.and_time(*close))))
            .find(|(_, _, at)| period.contains(*at))
    }

    /// `period` as an early close inside it leaves it: ending then.
    fn close_early(&self, period: Period) -> Period {
        match self.early_close_in(period) {
            Some((_, _, at)) => Period { end: at, ..period },
            None => period,
        }
    }

    /// Whether `time` falls inside a period.
    pub fn is_active(&self, time: DateTime<Utc>) -> bool {
        self.period_at(time).is_some()
    }

    /// The first time after `time` that a session can start: a period beginning, if one begins
    /// within about 400 days (weekly schedules: weeks), or a weekly period reopening after a
    /// holiday, whichever comes first.
    pub fn next_start(&self, time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        // A weekly period that an early close shuts before it starts opens at the reopening.
        let start =
            self.periods_around(time).map(|p| p.start).find(|start| *start > time && !self.closed_mid_period(*start));
        match (start, self.next_reopening(time, start)) {
            (Some(start), Some(reopening)) => Some(start.min(reopening)),
            (start, reopening) => start.or(reopening),
        }
    }

    /// The first midnight after `time`, up to `until`'s date, that ends a holiday or an early close
    /// closing a weekly period under way.
    fn next_reopening(&self, time: DateTime<Utc>, until: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        if !matches!(self.kind, Kind::Weekly { .. }) {
            return None;
        }
        let today = self.time_zone.to_local(time).date();
        let horizon = today + chrono::Duration::weeks(i64::try_from(MAX_CANDIDATES).expect("a small constant"));
        let last = until.map_or(horizon, |until| self.time_zone.to_local(until).date());
        self.holidays
            .closed_dates(today, last)
            .map(|closed| self.time_zone.to_utc((closed + chrono::Duration::days(1)).and_time(NaiveTime::MIN)))
            .filter(|reopening| *reopening > time && !self.closed_mid_period(*reopening))
            // Not a holiday before the week's period starts (a Saturday's), or in a week skipped.
            .find(|reopening| self.scheduled_period(*reopening).is_some())
    }

    /// Why the schedule doesn't allow a session at `time`, naming the holiday if one is the reason;
    /// `None` inside a period.
    pub(crate) fn closed_reason(&self, time: DateTime<Utc>) -> Option<String> {
        if self.is_active(time) {
            return None;
        }
        let holiday = self.closure_at(time).map(|closure| format!("; {closure}")).unwrap_or_default();
        Some(match self.next_start(time) {
            Some(next) => format!("outside session time (schedule '{self}'{holiday}); next session starts {next}"),
            None => format!("outside session time (schedule '{self}'{holiday})"),
        })
    }

    /// What removed the period `time` would otherwise be in, if anything: a holiday ("2026-12-25 is
    /// a holiday") or an early close ("2026-11-27 closes early at 13:00:00"). A holiday's date
    /// comes from the calendar, so it is the period's start date even where a start in a DST gap
    /// resolves to the next local date.
    fn closure_at(&self, time: DateTime<Utc>) -> Option<String> {
        let local = self.time_zone.to_local(time);
        let today = local.date();
        let holiday = |date: NaiveDate| format!("{date} is a holiday");
        let early = |date: NaiveDate, close: NaiveTime| format!("{date} closes early at {close}");
        if self.closed_mid_period(time) && self.scheduled_period(time).is_some() {
            return Some(match self.holidays.early_close(today) {
                Some(close) => early(today, close),
                None => holiday(today),
            });
        }
        let first = today - chrono::Duration::days(self.look_back());
        if let Some(date) = self
            .holidays
            .dates
            .range(first..=today)
            .rev()
            .copied()
            .filter(|date| self.regular_start(*date))
            .find(|date| self.period_starting(*date).contains(time))
        {
            return Some(holiday(date));
        }
        // A daily period an early close ended: one starting in the look-back that would contain
        // `time`, but for the close.
        std::iter::successors(Some(first), |date| Some(*date + chrono::Duration::days(1)))
            .take_while(|date| *date <= today)
            .filter(|date| self.starts_on(*date))
            .map(|date| self.period_starting(date))
            .filter(|period| period.contains(time))
            .find_map(|period| self.early_close_in(period).filter(|(_, _, at)| *at <= time))
            .map(|(date, close, _)| early(date, close))
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
            .map(|date| self.close_early(self.period_starting(date)))
            // Swallowed by a clock change or closed early as it starts: it never happens, so it's no
            // start to wait for.
            .filter(|period| period.start < period.end)
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
    type Err = ConfigError;

    /// Parses `daily HH:MM[:SS]-HH:MM[:SS] [DAYS] [ZONE]` or
    /// `weekly DAY HH:MM[:SS]-DAY HH:MM[:SS] [ZONE]`. DAYS is a comma-separated list of days or
    /// ranges (`mon-fri`, `mon,wed,fri`, `sun-thu`); ZONE defaults to UTC.
    fn from_str(s: &str) -> Result<Self, ConfigError> {
        parse_schedule(s).map_err(ConfigError::new)
    }
}

/// [`SessionSchedule::from_str`], its error as text.
fn parse_schedule(s: &str) -> Result<SessionSchedule, String> {
    let normalized = s.trim().replace(" - ", "-");
    let mut words = normalized.split_whitespace();
    let kind = words.next().ok_or("empty schedule")?.to_ascii_lowercase();
    let rest: Vec<&str> = words.collect();
    let (schedule, zone) = match kind.as_str() {
        "daily" => {
            let (range, rest) = rest.split_first().ok_or("daily schedule needs HH:MM-HH:MM")?;
            let (start, end) = range.split_once('-').ok_or_else(|| format!("expected HH:MM-HH:MM, got '{range}'"))?;
            let mut schedule = SessionSchedule::daily(parse_time(start)?, parse_time(end)?);
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
                SessionSchedule::weekly(parse_day(rest[0])?, parse_time(start)?, parse_day(end)?, parse_time(rest[2])?);
            (schedule, &rest[3..])
        }
        other => return Err(format!("schedule must start with 'daily' or 'weekly', not '{other}'")),
    };
    match zone {
        [] => Ok(schedule),
        [zone] => Ok(schedule.in_time_zone(parse_time_zone(zone)?)),
        [_, extra, ..] => Err(format!("unexpected '{extra}' in schedule")),
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
        // 02:30 doesn't exist on 2026-03-08 in New York: taken as the end of the gap (03:00 EDT).
        let gap = schedule("daily 02:30-04:00 America/New_York");
        assert_eq!(gap.period_at(at("2026-03-08 07:40")).unwrap().start, at("2026-03-08 07:00"));
        // A period inside the gap doesn't happen.
        let inside = schedule("daily 02:15-02:45 America/New_York");
        assert_eq!(inside.next_start(at("2026-03-08 05:00")), Some(at("2026-03-09 06:15")));
        // 01:30 happens twice on 2026-11-01: the first (EDT) is used.
        let repeated = schedule("daily 01:30-03:00 America/New_York");
        assert_eq!(repeated.period_at(at("2026-11-01 06:00")).unwrap().start, at("2026-11-01 05:30"));
    }

    /// Samoa skipped 2011-12-30, going from UTC-10 to UTC+14 at midnight: that day's period
    /// doesn't happen, and the days either side keep their own offsets.
    #[cfg(feature = "tz")]
    #[test]
    fn a_day_skipped_by_a_clock_change_has_no_period() {
        let s = schedule("daily 08:00-17:00 Pacific/Apia");
        assert_eq!(s.period_at(at("2011-12-29 19:00")).unwrap().start, at("2011-12-29 18:00"), "the 29th, UTC-10");
        assert_eq!(s.next_start(at("2011-12-30 04:00")), Some(at("2011-12-30 18:00")), "the 31st, UTC+14");
        assert_eq!(s.period_at(at("2011-12-30 19:00")).unwrap().end, at("2011-12-31 03:00"));
        let continuous = schedule("daily 00:00-00:00 Pacific/Apia");
        assert_eq!(
            continuous.period_at(at("2011-12-30 09:59")),
            Some(Period { start: at("2011-12-29 10:00"), end: at("2011-12-30 10:00") }),
            "the 29th runs until the clocks skip"
        );
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
            let err = text.parse::<SessionSchedule>().unwrap_err().to_string();
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

    fn time(s: &str) -> NaiveTime {
        parse_time(s).unwrap()
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
        let err = "2026-12-25\n2026-13-01\n".parse::<HolidayCalendar>().unwrap_err().to_string();
        assert!(err.contains("line 2"), "{err}");
        assert!(err.contains("'2026-13-01'"), "{err}");
        let err = "2026-12-25 2026-12-26".parse::<HolidayCalendar>().unwrap_err().to_string();
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
    fn weekly_schedules_skip_a_week_starting_on_a_holiday() {
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-20", "2026-12-25"]);
        assert!(!s.is_active(at("2026-12-22 12:00")), "the week starting on the 20th is skipped");
        assert_eq!(s.next_start(at("2026-12-22 12:00")), Some(at("2026-12-27 17:00")));
    }

    #[test]
    fn a_mid_week_holiday_closes_a_weekly_period_for_the_day_without_ending_it() {
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-23", "2026-12-24"]);
        let week = s.period_at(at("2026-12-22 12:00")).unwrap();
        assert!(!s.is_active(at("2026-12-23 00:00")));
        assert!(!s.is_active(at("2026-12-24 23:59")));
        assert_eq!(s.next_start(at("2026-12-22 12:00")), Some(at("2026-12-25 00:00")), "after both days");
        assert_eq!(s.next_start(at("2026-12-23 12:00")), Some(at("2026-12-25 00:00")));
        // The same period, so the session doesn't reset its sequence numbers.
        assert_eq!(s.period_at(at("2026-12-25 00:00")), Some(week));
        // Holidays outside a week's period don't reopen anything: a Saturday's, or the end day's
        // after the period ends.
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-26"]);
        assert_eq!(s.next_start(at("2026-12-26 12:00")), Some(at("2026-12-27 17:00")));
        let s = closed("weekly sun 17:00-fri 17:00", &["2026-12-25"]);
        assert!(!s.is_active(at("2026-12-25 10:00")), "Friday, closed until the period ends");
        assert_eq!(s.next_start(at("2026-12-25 10:00")), Some(at("2026-12-27 17:00")));
    }

    #[test]
    fn calendars_read_early_closes_as_a_date_and_a_time() {
        let calendar: HolidayCalendar =
            "2026-11-26  # Thanksgiving\n2026-11-27 13:00  # closes early\n2026-12-24 13:00:30\n2026-11-27 13:00\n"
                .parse()
                .unwrap();
        assert!(calendar.contains(date("2026-11-26")));
        assert!(!calendar.contains(date("2026-11-27")), "an early close isn't a whole-day holiday");
        assert_eq!(calendar.early_close(date("2026-11-27")), NaiveTime::from_hms_opt(13, 0, 0));
        assert_eq!(calendar.early_close(date("2026-12-24")), NaiveTime::from_hms_opt(13, 0, 30));
        assert_eq!(calendar.early_close(date("2026-11-26")), None);
        assert_eq!(calendar.len(), 3, "a repeated early close collapses");
        let built = HolidayCalendar::new([date("2026-11-26")])
            .with_early_closes([(date("2026-11-27"), time("13:00")), (date("2026-12-24"), time("13:00:30"))]);
        assert_eq!(calendar, built);
    }

    #[test]
    fn calendars_refuse_a_date_closed_two_ways() {
        let err = "2026-11-27\n2026-11-27 13:00\n".parse::<HolidayCalendar>().unwrap_err().to_string();
        assert!(err.contains("line 2") && err.contains("2026-11-27"), "{err}");
        let err = "2026-11-27 13:00\n2026-11-27 12:00\n".parse::<HolidayCalendar>().unwrap_err().to_string();
        assert!(err.contains("line 2") && err.contains("2026-11-27"), "{err}");
        let err = "2026-11-27 25:00".parse::<HolidayCalendar>().unwrap_err().to_string();
        assert!(err.contains("line 1") && err.contains("'25:00'"), "{err}");
    }

    fn closing_early(s: &str, closes: &[(&str, &str)]) -> SessionSchedule {
        let closes = closes.iter().map(|(d, t)| (date(d), time(t)));
        schedule(s).with_holidays(HolidayCalendar::default().with_early_closes(closes))
    }

    // 2026-11-27 is a Friday.

    #[test]
    fn an_early_close_ends_the_daily_period_in_session_then() {
        let s = closing_early("daily 08:00-17:00 mon-fri", &[("2026-11-27", "13:00")]);
        assert_eq!(
            s.period_at(at("2026-11-27 12:59")),
            Some(Period { start: at("2026-11-27 08:00"), end: at("2026-11-27 13:00") })
        );
        assert!(!s.is_active(at("2026-11-27 13:00")));
        assert_eq!(s.next_start(at("2026-11-27 13:00")), Some(at("2026-11-30 08:00")), "the next period, as usual");
        assert!(s.is_active(at("2026-11-26 16:00")), "other days are untouched");
    }

    #[test]
    fn an_early_close_leaves_periods_it_isnt_inside_alone() {
        // A close before the day's period starts, or after it ends, changes nothing.
        let s = closing_early("daily 08:00-17:00", &[("2026-11-27", "07:00"), ("2026-11-28", "18:00")]);
        assert!(s.is_active(at("2026-11-27 16:00")));
        assert!(s.is_active(at("2026-11-28 16:00")));
        // An overnight period closed early; the evening's period still opens, as a market reopens.
        let s = closing_early("daily 17:00-16:00", &[("2026-11-27", "12:00")]);
        assert_eq!(s.period_at(at("2026-11-27 11:00")).map(|p| p.end), Some(at("2026-11-27 12:00")));
        assert!(!s.is_active(at("2026-11-27 12:00")));
        assert_eq!(s.next_start(at("2026-11-27 12:00")), Some(at("2026-11-27 17:00")));
        // A close at a period's very start removes it.
        let s = closing_early("daily 08:00-17:00", &[("2026-11-27", "08:00")]);
        assert!(!s.is_active(at("2026-11-27 10:00")));
        assert_eq!(s.next_start(at("2026-11-27 07:00")), Some(at("2026-11-28 08:00")));
    }

    #[test]
    fn an_early_close_closes_a_weekly_period_until_midnight_without_ending_it() {
        let s = closing_early("weekly sun 17:00-fri 17:00", &[("2026-11-26", "13:00")]);
        let week = s.period_at(at("2026-11-26 12:59")).unwrap();
        assert!(!s.is_active(at("2026-11-26 13:00")));
        assert!(!s.is_active(at("2026-11-26 23:59")));
        assert_eq!(s.next_start(at("2026-11-26 13:00")), Some(at("2026-11-27 00:00")));
        assert_eq!(s.next_start(at("2026-11-26 10:00")), Some(at("2026-11-27 00:00")), "the reopening after it");
        // The same period, so the session doesn't reset its sequence numbers.
        assert_eq!(s.period_at(at("2026-11-27 00:00")), Some(week));
        // On the start day before the period starts: no session that day, and none until midnight.
        let s = closing_early("weekly sun 17:00-fri 17:00", &[("2026-11-29", "12:00")]);
        assert!(!s.is_active(at("2026-11-29 18:00")));
        assert_eq!(s.next_start(at("2026-11-28 12:00")), Some(at("2026-11-30 00:00")));
        // After the week ends: nothing to close or reopen.
        let s = closing_early("weekly sun 17:00-fri 17:00", &[("2026-11-27", "18:00")]);
        assert!(s.is_active(at("2026-11-27 16:00")));
        assert_eq!(s.next_start(at("2026-11-27 17:00")), Some(at("2026-11-29 17:00")));
    }

    #[test]
    fn closed_reasons_name_the_early_close() {
        let s = closing_early("daily 08:00-17:00 mon-fri", &[("2026-11-27", "13:00")]);
        assert_eq!(s.closed_reason(at("2026-11-27 12:00")), None);
        assert_eq!(
            s.closed_reason(at("2026-11-27 14:00")).unwrap(),
            "outside session time (schedule 'daily 08:00:00-17:00:00 mon,tue,wed,thu,fri UTC'; \
             2026-11-27 closes early at 13:00:00); next session starts 2026-11-30 08:00:00 UTC"
        );
        assert!(!s.closed_reason(at("2026-11-27 18:00")).unwrap().contains("closes early"), "outside hours anyway");
        let weekly = closing_early("weekly sun 17:00-fri 17:00", &[("2026-11-26", "13:00")]);
        let reason = weekly.closed_reason(at("2026-11-26 14:00")).unwrap();
        assert!(
            reason.contains("; 2026-11-26 closes early at 13:00:00); next session starts 2026-11-27 00:00:00 UTC"),
            "{reason}"
        );
    }

    #[cfg(feature = "tz")]
    #[test]
    fn early_closes_are_times_in_the_schedules_time_zone() {
        // 13:00 in New York on 2026-11-27 is 18:00 UTC.
        let s = closing_early("daily 09:30-16:00 America/New_York", &[("2026-11-27", "13:00")]);
        assert!(s.is_active(at("2026-11-27 17:59")));
        assert!(!s.is_active(at("2026-11-27 18:00")));
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
        let weekly = closed("weekly sun 17:00-fri 17:00", &["2026-12-20", "2026-12-23"]);
        let reason = weekly.closed_reason(at("2026-12-22 12:00")).unwrap();
        assert!(reason.contains("; 2026-12-20 is a holiday)"), "{reason}");
        let weekly = closed("weekly sun 17:00-fri 17:00", &["2026-12-23"]);
        let reason = weekly.closed_reason(at("2026-12-23 12:00")).unwrap();
        assert!(reason.contains("; 2026-12-23 is a holiday); next session starts 2026-12-24 00:00:00 UTC"), "{reason}");
    }

    #[cfg(feature = "tz")]
    #[test]
    fn closed_reasons_name_the_start_date_of_a_period_in_a_gap_over_midnight() {
        // Nuuk springs forward from 23:00 on Saturday 2026-03-28 to 00:00 on the Sunday, so the
        // 28th's 23:30 start doesn't exist and is taken as 00:00 on the 29th (01:00 UTC).
        let s = closed("daily 23:30-01:00 America/Nuuk", &["2026-03-28"]);
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
