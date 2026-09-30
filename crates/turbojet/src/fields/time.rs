//! FIX dates and times: UTCTimestamp, UTCTimeOnly, the date types, MonthYear, TZTimeOnly and
//! TZTimestamp. Times keep the precision they arrived with, so a relayed value is written back
//! as it came; new ones are written in milliseconds unless given another precision.

use std::cell::Cell;
use std::fmt::{self, Write as _};
use std::ops::{Add, Deref, Sub};

use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, TimeDelta, Timelike, Utc};
pub use chrono::{NaiveDate, NaiveTime};

use super::{FromFix, ToFix, ValueError};

/// How many digits of a second a time is written with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Precision {
    /// Whole seconds: `HH:MM:SS`.
    Seconds,
    /// Milliseconds: `HH:MM:SS.sss`, the default.
    #[default]
    Millis,
    /// Microseconds: `HH:MM:SS.ssssss`.
    Micros,
    /// Nanoseconds: `HH:MM:SS.sssssssss`.
    Nanos,
}

impl Precision {
    /// The number of fraction digits written.
    pub const fn digits(self) -> usize {
        match self {
            Self::Seconds => 0,
            Self::Millis => 3,
            Self::Micros => 6,
            Self::Nanos => 9,
        }
    }

    /// The precision a value with `digits` fraction digits keeps: the smallest that holds them.
    fn from_digits(digits: usize) -> Self {
        match digits {
            0 => Self::Seconds,
            1..=3 => Self::Millis,
            4..=6 => Self::Micros,
            _ => Self::Nanos,
        }
    }
}

/// UTCTimestamp: an instant, and the precision it's written with.
///
/// It dereferences to chrono's [`DateTime<Utc>`], so chrono's methods work on it. Parsed values
/// keep the precision they arrived with (`.5` and `.500` are both milliseconds); new ones are
/// milliseconds. Two timestamps are equal when they're the same instant, whatever their precision.
///
/// ```
/// use turbojet::fields::{FromFix, Precision, ToFix, UtcTimestamp};
///
/// let t = UtcTimestamp::from_fix("20260930-12:00:00.123456").unwrap();
/// assert_eq!(t.precision(), Precision::Micros);
/// assert_eq!(t.to_fix(), "20260930-12:00:00.123456");
/// assert_eq!(t.with_precision(Precision::Millis).to_fix(), "20260930-12:00:00.123");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct UtcTimestamp {
    time: DateTime<Utc>,
    precision: Precision,
}

impl UtcTimestamp {
    /// `time`, written with `precision`.
    pub const fn new(time: DateTime<Utc>, precision: Precision) -> Self {
        Self { time, precision }
    }

    /// The current time, in milliseconds.
    pub fn now() -> Self {
        Utc::now().into()
    }

    /// The instant `seconds` and `nanos` after the Unix epoch, in milliseconds; `None` if out of
    /// range.
    pub fn from_timestamp(seconds: i64, nanos: u32) -> Option<Self> {
        DateTime::from_timestamp(seconds, nanos).map(Self::from)
    }

    /// The same instant, written with `precision`.
    #[must_use]
    pub const fn with_precision(self, precision: Precision) -> Self {
        Self { precision, ..self }
    }

    /// The precision it's written with.
    pub const fn precision(self) -> Precision {
        self.precision
    }

    /// The instant.
    pub const fn time(self) -> DateTime<Utc> {
        self.time
    }
}

impl Deref for UtcTimestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &DateTime<Utc> {
        &self.time
    }
}

/// In milliseconds.
impl From<DateTime<Utc>> for UtcTimestamp {
    fn from(time: DateTime<Utc>) -> Self {
        Self::new(time, Precision::Millis)
    }
}

impl From<UtcTimestamp> for DateTime<Utc> {
    fn from(t: UtcTimestamp) -> Self {
        t.time
    }
}

impl PartialEq for UtcTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for UtcTimestamp {}

impl PartialOrd for UtcTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UtcTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl std::hash::Hash for UtcTimestamp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.time.hash(state)
    }
}

impl Add<TimeDelta> for UtcTimestamp {
    type Output = Self;

    fn add(self, delta: TimeDelta) -> Self {
        Self { time: self.time + delta, ..self }
    }
}

impl Sub<TimeDelta> for UtcTimestamp {
    type Output = Self;

    fn sub(self, delta: TimeDelta) -> Self {
        Self { time: self.time - delta, ..self }
    }
}

impl fmt::Display for UtcTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_fix())
    }
}

/// `YYYYMMDD-HH:MM:SS` with an optional fraction of 1 to 9 digits, and `:60` for a leap second.
impl FromFix for UtcTimestamp {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let b = s.as_bytes();
        if b.len() < 9 || b[8] != b'-' {
            return Err(ValueError::Format);
        }
        let date = parse_date(&b[..8]).ok_or(ValueError::Format)?;
        let (time, precision) = match parse_time(&b[9..]) {
            Some((time, Some(precision), None)) => (time, precision),
            _ => return Err(ValueError::Format),
        };
        Ok(Self::new(NaiveDateTime::new(date, time).and_utc(), precision))
    }
}

impl ToFix for UtcTimestamp {
    fn write_fix(&self, out: &mut String) {
        let t = &self.time;
        // Leap seconds and years outside 0..=9999 are rare enough to leave to chrono.
        if t.nanosecond() >= 1_000_000_000 || !(0..=9999).contains(&t.year()) {
            return write_with_chrono(out, t, self.precision);
        }
        out.push_str(std::str::from_utf8(&seconds_prefix(t)).expect("ASCII"));
        push_fraction(out, t.nanosecond(), self.precision);
    }
}

/// [`UtcTimestamp::write_fix`] for a leap second or a year outside 0..=9999, by chrono. Kept out
/// of line so the common case stays small.
#[cold]
#[inline(never)]
fn write_with_chrono(out: &mut String, t: &DateTime<Utc>, precision: Precision) {
    let fraction = match precision {
        Precision::Seconds => "",
        Precision::Millis => "%.3f",
        Precision::Micros => "%.6f",
        Precision::Nanos => "%.9f",
    };
    write!(out, "{}", t.format(&format!("%Y%m%d-%H:%M:%S{fraction}"))).expect("writing to a String cannot fail");
}

thread_local! {
    /// The most recently formatted second on this thread: (Unix seconds, `YYYYMMDD-HH:MM:SS`).
    static SECONDS_PREFIX: Cell<(i64, [u8; 17])> = const { Cell::new((i64::MIN, [0; 17])) };
}

/// `YYYYMMDD-HH:MM:SS` for `time`, which must have a four-digit year. Cached per thread, since
/// a busy session formats the same second many times.
fn seconds_prefix(time: &DateTime<Utc>) -> [u8; 17] {
    debug_assert!((0..=9999).contains(&time.year()));
    let seconds = time.timestamp();
    SECONDS_PREFIX.with(|cache| {
        let (cached_seconds, prefix) = cache.get();
        if cached_seconds == seconds {
            return prefix;
        }
        let mut prefix = [0u8; 17];
        put_digits(&mut prefix[0..4], time.year().cast_unsigned());
        put_digits(&mut prefix[4..6], time.month());
        put_digits(&mut prefix[6..8], time.day());
        put_digits(&mut prefix[9..11], time.hour());
        put_digits(&mut prefix[12..14], time.minute());
        put_digits(&mut prefix[15..17], time.second());
        prefix[8] = b'-';
        prefix[11] = b':';
        prefix[14] = b':';
        cache.set((seconds, prefix));
        prefix
    })
}

/// Writes `value`'s last `out.len()` decimal digits into `out`.
fn put_digits(out: &mut [u8], mut value: u32) {
    for digit in out.iter_mut().rev() {
        *digit = b'0' + (value % 10) as u8;
        value /= 10;
    }
}

/// Appends `.` and the fraction of a second in `nanos` (under a billion) to `precision`,
/// truncated; nothing for whole seconds.
fn push_fraction(out: &mut String, nanos: u32, precision: Precision) {
    let (value, width) = match precision {
        Precision::Seconds => return,
        // Milliseconds, the usual case, written directly.
        Precision::Millis => {
            let millis = nanos / 1_000_000;
            out.push('.');
            out.push(char::from(b'0' + (millis / 100) as u8));
            out.push(char::from(b'0' + (millis / 10 % 10) as u8));
            out.push(char::from(b'0' + (millis % 10) as u8));
            return;
        }
        Precision::Micros => (nanos / 1_000, 6),
        Precision::Nanos => (nanos, 9),
    };
    let mut digits = [0u8; 10];
    digits[0] = b'.';
    put_digits(&mut digits[1..=width], value);
    out.push_str(std::str::from_utf8(&digits[..=width]).expect("ASCII"));
}

/// `HH:MM:SS` with the fraction `precision` asks for.
fn push_time(out: &mut String, time: NaiveTime, precision: Option<Precision>) {
    let mut hms = [b':'; 8];
    put_digits(&mut hms[0..2], time.hour());
    put_digits(&mut hms[3..5], time.minute());
    // A leap second is second 59 with a nanosecond count past a billion.
    let second = if time.nanosecond() >= 1_000_000_000 { 60 } else { time.second() };
    put_digits(&mut hms[6..8], second);
    match precision {
        None => out.push_str(std::str::from_utf8(&hms[..5]).expect("ASCII")),
        Some(precision) => {
            out.push_str(std::str::from_utf8(&hms).expect("ASCII"));
            // A leap second's extra second is the `60` already written.
            push_fraction(out, time.nanosecond() % 1_000_000_000, precision);
        }
    }
}

/// `YYYYMMDD`, a valid date.
fn parse_date(b: &[u8]) -> Option<NaiveDate> {
    if b.len() != 8 {
        return None;
    }
    NaiveDate::from_ymd_opt(i32::try_from(number(&b[0..4])?).ok()?, number(&b[4..6])?, number(&b[6..8])?)
}

/// Writes `date` as `YYYYMMDD`.
fn push_date(out: &mut String, date: NaiveDate) {
    if let Ok(year) = u32::try_from(date.year())
        && year <= 9999
    {
        let mut digits = [0u8; 8];
        put_digits(&mut digits[0..4], year);
        put_digits(&mut digits[4..6], date.month());
        put_digits(&mut digits[6..8], date.day());
        out.push_str(std::str::from_utf8(&digits).expect("ASCII"));
    } else {
        write!(out, "{}", date.format("%Y%m%d")).expect("writing to a String cannot fail");
    }
}

/// The value of `b`, if it's all ASCII digits.
fn number(b: &[u8]) -> Option<u32> {
    if b.is_empty() || b.len() > 9 || !b.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(b.iter().fold(0, |n, &d| n * 10 + u32::from(d - b'0')))
}

/// `n` in a narrower type, or `None` if it doesn't fit.
fn narrow<T: TryFrom<u32>>(n: u32) -> Option<T> {
    T::try_from(n).ok()
}

/// `HH:MM`, then optionally `:SS` (60 for a leap second) and a fraction of 1 to 9 digits, then
/// optionally a zone: `Z`, `±hh` or `±hh:mm`. Returns the time, its precision (`None` for `HH:MM`)
/// and the zone's offset.
fn parse_time(b: &[u8]) -> Option<(NaiveTime, Option<Precision>, Option<FixedOffset>)> {
    if b.len() < 5 || b[2] != b':' {
        return None;
    }
    let (hour, minute) = (number(&b[0..2])?, number(&b[3..5])?);
    let mut at = 5;
    let (mut second, mut nanos, mut precision) = (0, 0, None);
    if b.get(at) == Some(&b':') {
        second = number(b.get(at + 1..at + 3)?)?;
        at += 3;
        precision = Some(Precision::Seconds);
        if b.get(at) == Some(&b'.') {
            let digits = b[at + 1..].iter().take_while(|d| d.is_ascii_digit()).count();
            if !(1..=9).contains(&digits) {
                return None;
            }
            nanos = number(&b[at + 1..at + 1 + digits])? * 10u32.pow(9 - u32::try_from(digits).ok()?);
            precision = Some(Precision::from_digits(digits));
            at += 1 + digits;
        }
    }
    let time = match second {
        60 => NaiveTime::from_hms_nano_opt(hour, minute, 59, 1_000_000_000 + nanos)?,
        _ => NaiveTime::from_hms_nano_opt(hour, minute, second, nanos)?,
    };
    let offset = match &b[at..] {
        [] => None,
        [b'Z'] => Some(FixedOffset::east_opt(0)?),
        [sign @ (b'+' | b'-'), rest @ ..] => {
            let (hours, minutes) = match rest {
                [_, _] => (number(rest)?, 0),
                [_, _, b':', _, _] => (number(&rest[..2])?, number(&rest[3..])?),
                _ => return None,
            };
            if hours > 14 || minutes > 59 {
                return None;
            }
            let seconds = i32::try_from(hours * 3600 + minutes * 60).ok()?;
            Some(FixedOffset::east_opt(if *sign == b'-' { -seconds } else { seconds })?)
        }
        _ => return None,
    };
    Some((time, precision, offset))
}

/// Writes a zone: `Z` for UTC, otherwise `±hh`, or `±hh:mm` when there are minutes.
fn push_offset(out: &mut String, offset: FixedOffset) {
    let seconds = offset.local_minus_utc();
    if seconds == 0 {
        out.push('Z');
        return;
    }
    out.push(if seconds < 0 { '-' } else { '+' });
    let minutes = seconds.unsigned_abs() / 60;
    let mut digits = [b':'; 5];
    put_digits(&mut digits[0..2], minutes / 60);
    put_digits(&mut digits[3..5], minutes % 60);
    let len = if minutes.is_multiple_of(60) { 2 } else { 5 };
    out.push_str(std::str::from_utf8(&digits[..len]).expect("ASCII"));
}

/// UTCTimeOnly: a time of day in UTC, and the precision it's written with. It dereferences to
/// chrono's [`NaiveTime`]; equality ignores the precision, as for [`UtcTimestamp`].
#[derive(Debug, Clone, Copy)]
pub struct UtcTimeOnly {
    time: NaiveTime,
    precision: Precision,
}

impl UtcTimeOnly {
    /// `time`, written with `precision`.
    pub const fn new(time: NaiveTime, precision: Precision) -> Self {
        Self { time, precision }
    }

    /// The same time, written with `precision`.
    #[must_use]
    pub const fn with_precision(self, precision: Precision) -> Self {
        Self { precision, ..self }
    }

    /// The precision it's written with.
    pub const fn precision(self) -> Precision {
        self.precision
    }

    /// The time of day.
    pub const fn time(self) -> NaiveTime {
        self.time
    }
}

impl Deref for UtcTimeOnly {
    type Target = NaiveTime;

    fn deref(&self) -> &NaiveTime {
        &self.time
    }
}

/// In milliseconds.
impl From<NaiveTime> for UtcTimeOnly {
    fn from(time: NaiveTime) -> Self {
        Self::new(time, Precision::Millis)
    }
}

impl PartialEq for UtcTimeOnly {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for UtcTimeOnly {}

impl std::hash::Hash for UtcTimeOnly {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.time.hash(state)
    }
}

impl fmt::Display for UtcTimeOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_fix())
    }
}

/// `HH:MM:SS` with an optional fraction of 1 to 9 digits.
impl FromFix for UtcTimeOnly {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        match parse_time(s.as_bytes()) {
            Some((time, Some(precision), None)) => Ok(Self::new(time, precision)),
            _ => Err(ValueError::Format),
        }
    }
}

impl ToFix for UtcTimeOnly {
    fn write_fix(&self, out: &mut String) {
        push_time(out, self.time, Some(self.precision))
    }
}

/// UTCDateOnly, UTCDate and LocalMktDate: `YYYYMMDD`.
impl FromFix for NaiveDate {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        parse_date(s.as_bytes()).ok_or(ValueError::Format)
    }
}

impl ToFix for NaiveDate {
    fn write_fix(&self, out: &mut String) {
        push_date(out, *self)
    }
}

/// MonthYear: a month, optionally narrowed to a day or a week of it: `YYYYMM`, `YYYYMMDD` or
/// `YYYYMMwN`.
///
/// ```
/// use turbojet::fields::{DayOrWeek, FromFix, MonthYear, ToFix};
///
/// let expiry = MonthYear::from_fix("202612w3").unwrap();
/// assert_eq!((expiry.year(), expiry.month(), expiry.day_or_week()), (2026, 12, Some(DayOrWeek::Week(3))));
/// assert_eq!(MonthYear::new(2026, 12).unwrap().to_fix(), "202612");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonthYear {
    year: u16,
    month: u8,
    day_or_week: Option<DayOrWeek>,
}

/// The day or week a [`MonthYear`] narrows its month to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DayOrWeek {
    /// A day of the month.
    Day(u8),
    /// A week of the month, 1 to 5.
    Week(u8),
}

impl MonthYear {
    /// Month `month` (1 to 12) of `year` (0 to 9999).
    pub fn new(year: u16, month: u8) -> Option<Self> {
        (year <= 9999 && (1..=12).contains(&month)).then_some(Self { year, month, day_or_week: None })
    }

    /// The same month, narrowed to `day`, which must exist in it.
    pub fn with_day(self, day: u8) -> Option<Self> {
        NaiveDate::from_ymd_opt(self.year.into(), self.month.into(), day.into())?;
        Some(Self { day_or_week: Some(DayOrWeek::Day(day)), ..self })
    }

    /// The same month, narrowed to week `week` (1 to 5).
    pub fn with_week(self, week: u8) -> Option<Self> {
        (1..=5).contains(&week).then_some(Self { day_or_week: Some(DayOrWeek::Week(week)), ..self })
    }

    /// The year.
    pub const fn year(self) -> u16 {
        self.year
    }

    /// The month, 1 to 12.
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day or week, if narrowed to one.
    pub const fn day_or_week(self) -> Option<DayOrWeek> {
        self.day_or_week
    }
}

impl fmt::Display for MonthYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_fix())
    }
}

impl FromFix for MonthYear {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let b = s.as_bytes();
        let month = || MonthYear::new(narrow(number(b.get(0..4)?)?)?, narrow(number(b.get(4..6)?)?)?);
        let value = match b.len() {
            6 => month(),
            8 if b[6] == b'w' => month().and_then(|m| m.with_week(narrow(number(&b[7..])?)?)),
            8 => month().and_then(|m| m.with_day(narrow(number(&b[6..])?)?)),
            _ => None,
        };
        value.ok_or(ValueError::Format)
    }
}

impl ToFix for MonthYear {
    fn write_fix(&self, out: &mut String) {
        let mut digits = [0u8; 8];
        put_digits(&mut digits[0..4], self.year.into());
        put_digits(&mut digits[4..6], self.month.into());
        let len = match self.day_or_week {
            None => 6,
            Some(DayOrWeek::Day(day)) => {
                put_digits(&mut digits[6..8], day.into());
                8
            }
            Some(DayOrWeek::Week(week)) => {
                digits[6] = b'w';
                put_digits(&mut digits[7..8], week.into());
                8
            }
        };
        out.push_str(std::str::from_utf8(&digits[..len]).expect("ASCII"));
    }
}

/// TZTimeOnly: a local time of day, optionally with its offset from UTC:
/// `HH:MM[:SS[.sss]][Z | ±hh[:mm]]`.
///
/// ```
/// use turbojet::fields::{FromFix, ToFix, TzTimeOnly};
///
/// let open = TzTimeOnly::from_fix("09:30-05").unwrap();
/// assert_eq!(open.offset().map(|o| o.local_minus_utc()), Some(-5 * 3600));
/// assert_eq!(open.to_fix(), "09:30-05");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TzTimeOnly {
    time: NaiveTime,
    precision: Option<Precision>,
    offset: Option<FixedOffset>,
}

impl TzTimeOnly {
    /// `time` at `offset` from UTC, if known, written to `precision`: `None` for `HH:MM`.
    pub const fn new(time: NaiveTime, precision: Option<Precision>, offset: Option<FixedOffset>) -> Self {
        Self { time, precision, offset }
    }

    /// The local time of day.
    pub const fn time(self) -> NaiveTime {
        self.time
    }

    /// The precision it's written with; `None` for hours and minutes only.
    pub const fn precision(self) -> Option<Precision> {
        self.precision
    }

    /// The offset from UTC, if given.
    pub const fn offset(self) -> Option<FixedOffset> {
        self.offset
    }
}

impl fmt::Display for TzTimeOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_fix())
    }
}

impl FromFix for TzTimeOnly {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let (time, precision, offset) = parse_time(s.as_bytes()).ok_or(ValueError::Format)?;
        Ok(Self::new(time, precision, offset))
    }
}

impl ToFix for TzTimeOnly {
    fn write_fix(&self, out: &mut String) {
        push_time(out, self.time, self.precision);
        if let Some(offset) = self.offset {
            push_offset(out, offset);
        }
    }
}

/// TZTimestamp: a local date and time, optionally with its offset from UTC:
/// `YYYYMMDD-HH:MM[:SS[.sss]][Z | ±hh[:mm]]`.
///
/// ```
/// use turbojet::fields::{FromFix, ToFix, TzTimestamp};
///
/// let t = TzTimestamp::from_fix("20260930-07:39:00+01").unwrap();
/// assert_eq!(t.datetime().unwrap().to_rfc3339(), "2026-09-30T07:39:00+01:00");
/// assert_eq!(t.to_fix(), "20260930-07:39:00+01");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TzTimestamp {
    local: NaiveDateTime,
    precision: Option<Precision>,
    offset: Option<FixedOffset>,
}

impl TzTimestamp {
    /// `local` at `offset` from UTC, if known, written to `precision`: `None` for hours and
    /// minutes only.
    pub const fn new(local: NaiveDateTime, precision: Option<Precision>, offset: Option<FixedOffset>) -> Self {
        Self { local, precision, offset }
    }

    /// The local date and time.
    pub const fn local(self) -> NaiveDateTime {
        self.local
    }

    /// The precision it's written with; `None` for hours and minutes only.
    pub const fn precision(self) -> Option<Precision> {
        self.precision
    }

    /// The offset from UTC, if given.
    pub const fn offset(self) -> Option<FixedOffset> {
        self.offset
    }

    /// The instant, if the offset is given.
    pub fn datetime(self) -> Option<DateTime<FixedOffset>> {
        self.local.and_local_timezone(self.offset?).single()
    }
}

impl From<DateTime<FixedOffset>> for TzTimestamp {
    /// In milliseconds.
    fn from(time: DateTime<FixedOffset>) -> Self {
        Self::new(time.naive_local(), Some(Precision::Millis), Some(*time.offset()))
    }
}

impl fmt::Display for TzTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_fix())
    }
}

impl FromFix for TzTimestamp {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let b = s.as_bytes();
        if b.len() < 9 || b[8] != b'-' {
            return Err(ValueError::Format);
        }
        let date = parse_date(&b[..8]).ok_or(ValueError::Format)?;
        let (time, precision, offset) = parse_time(&b[9..]).ok_or(ValueError::Format)?;
        Ok(Self::new(NaiveDateTime::new(date, time), precision, offset))
    }
}

impl ToFix for TzTimestamp {
    fn write_fix(&self, out: &mut String) {
        push_date(out, self.local.date());
        out.push('-');
        push_time(out, self.local.time(), self.precision);
        if let Some(offset) = self.offset {
            push_offset(out, offset);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn ts(s: &str) -> UtcTimestamp {
        UtcTimestamp::from_fix(s).unwrap_or_else(|e| panic!("{s}: {e:?}"))
    }

    #[test]
    fn timestamps_keep_the_precision_they_arrive_with() {
        for (text, precision) in [
            ("20260927-03:20:48", Precision::Seconds),
            ("20260927-03:20:48.544", Precision::Millis),
            ("20260927-03:20:48.544123", Precision::Micros),
            ("20260927-03:20:48.544123789", Precision::Nanos),
        ] {
            let t = ts(text);
            assert_eq!(t.precision(), precision, "{text}");
            assert_eq!(t.to_fix(), text, "written back as it came");
        }
        // Other digit counts keep the next precision up.
        assert_eq!(ts("20260927-03:20:48.5").to_fix(), "20260927-03:20:48.500");
        assert_eq!(ts("20260927-03:20:48.54412").to_fix(), "20260927-03:20:48.544120");
        assert_eq!(ts("20260927-03:20:48.5441237").to_fix(), "20260927-03:20:48.544123700");
    }

    #[test]
    fn timestamps_are_equal_whatever_their_precision() {
        assert_eq!(ts("20260927-03:20:48"), ts("20260927-03:20:48.000000"));
        assert!(ts("20260927-03:20:48.1") < ts("20260927-03:20:48.100001"));
    }

    #[test]
    fn precision_truncates() {
        let t = ts("20260927-03:20:48.999999999");
        assert_eq!(t.with_precision(Precision::Seconds).to_fix(), "20260927-03:20:48");
        assert_eq!(t.with_precision(Precision::Millis).to_fix(), "20260927-03:20:48.999");
        assert_eq!(t.with_precision(Precision::Micros).to_fix(), "20260927-03:20:48.999999");
    }

    #[test]
    fn malformed_timestamps_are_refused() {
        for bad in [
            "20260927-3:20:48",
            "2026-09-27T03:20:48",
            "20260927-03:20:48.",
            "20260927-03:20:48.1234567890",
            "20261327-03:20:48",
            "20260927-24:00:00",
            "20260927-03:20",
            "20260927-03:20:48Z",
            "20260927-03:20:48 ",
            "",
        ] {
            assert_eq!(UtcTimestamp::from_fix(bad), Err(ValueError::Format), "{bad:?}");
        }
    }

    #[test]
    fn leap_seconds_are_kept() {
        let t = ts("20161231-23:59:60.500");
        assert_eq!(t.nanosecond(), 1_500_000_000);
        assert_eq!(t.to_fix(), "20161231-23:59:60.500");
        assert_eq!(t.with_precision(Precision::Micros).to_fix(), "20161231-23:59:60.500000");
    }

    /// What chrono produces for `precision`.
    fn chrono_format(t: &DateTime<Utc>, precision: Precision) -> String {
        let fraction = ["", "%.3f", "%.6f", "%.9f"][precision.digits() / 3];
        t.format(&format!("%Y%m%d-%H:%M:%S{fraction}")).to_string()
    }

    #[test]
    fn timestamp_formatting_matches_chrono() {
        // Deterministic pseudo-random instants from 1970 to 2199, with arbitrary nanoseconds.
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for i in 0..20_000 {
            let seconds = i64::try_from(next() % 7_258_118_400).unwrap(); // up to 2200-01-01
            let nanos = (next() % 1_000_000_000) as u32;
            let time = Utc.timestamp_opt(seconds, nanos).unwrap();
            let precision = [Precision::Seconds, Precision::Millis, Precision::Micros, Precision::Nanos][i % 4];
            let t = UtcTimestamp::new(time, precision);
            assert_eq!(t.to_fix(), chrono_format(&time, precision), "{t:?}");
            // And again, now from the cache.
            assert_eq!(t.to_fix(), chrono_format(&time, precision), "{t:?} (cached)");
        }
    }

    #[test]
    fn timestamp_edge_cases_match_chrono() {
        let at = |y, mo, d, h, mi, s, ns| {
            NaiveDate::from_ymd_opt(y, mo, d).unwrap().and_hms_nano_opt(h, mi, s, ns).unwrap().and_utc()
        };
        let cases = [
            at(1970, 1, 1, 0, 0, 0, 0),
            at(2026, 12, 31, 23, 59, 59, 999_999_999), // truncated, not rounded, to .999
            at(2024, 2, 29, 12, 0, 0, 1_000_000),
            at(9999, 12, 31, 23, 59, 59, 0),
            at(10_000, 1, 1, 0, 0, 0, 0),                // five-digit year: chrono fallback
            at(2016, 12, 31, 23, 59, 59, 1_500_000_000), // leap second: chrono fallback
            Utc.timestamp_opt(-86_400, 0).unwrap(),      // before the epoch
        ];
        for time in cases {
            assert_eq!(UtcTimestamp::from(time).to_fix(), chrono_format(&time, Precision::Millis), "{time:?}");
        }
    }

    #[test]
    fn timestamp_cache_is_per_second_and_per_thread() {
        let base = ts("20260927-03:20:48.544");
        let next_second = base + TimeDelta::milliseconds(700);
        assert_eq!(base.to_fix(), "20260927-03:20:48.544");
        assert_eq!(next_second.to_fix(), "20260927-03:20:49.244");
        assert_eq!(base.to_fix(), "20260927-03:20:48.544", "going back a second refreshes the cache");
        let other = std::thread::spawn(move || (base + TimeDelta::days(1)).to_fix()).join().unwrap();
        assert_eq!(other, "20260928-03:20:48.544");
        assert_eq!(base.to_fix(), "20260927-03:20:48.544");
    }

    #[test]
    fn new_timestamps_are_milliseconds() {
        assert_eq!(UtcTimestamp::now().precision(), Precision::Millis);
        let t = UtcTimestamp::from_timestamp(1_790_000_000, 123_456_789).unwrap();
        assert_eq!(t.to_fix(), "20260921-14:13:20.123");
        assert_eq!(DateTime::<Utc>::from(t).timestamp_subsec_nanos(), 123_456_789, "the instant is kept whole");
    }

    #[test]
    fn times_of_day() {
        for text in ["03:20:48", "03:20:48.544", "03:20:48.544123", "03:20:48.544123789", "23:59:60"] {
            assert_eq!(UtcTimeOnly::from_fix(text).unwrap().to_fix(), text);
        }
        assert_eq!(UtcTimeOnly::from_fix("03:20:48.5").unwrap().to_fix(), "03:20:48.500");
        for bad in ["03:20", "3:20:48", "03:20:48.", "24:00:00", "03:60:00", "03:20:48Z", "03:20:61"] {
            assert_eq!(UtcTimeOnly::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
        let t = UtcTimeOnly::from(NaiveTime::from_hms_micro_opt(9, 30, 0, 250_100).unwrap());
        assert_eq!(t.to_fix(), "09:30:00.250");
        assert_eq!(t.with_precision(Precision::Micros).to_fix(), "09:30:00.250100");
    }

    #[test]
    fn dates() {
        let d = NaiveDate::from_fix("20240229").unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
        assert_eq!(d.to_fix(), "20240229");
        assert_eq!(NaiveDate::from_ymd_opt(7, 1, 2).unwrap().to_fix(), "00070102");
        for bad in ["20230229", "2024022", "202402290", "2024-02-29", "20241301", "2024022a"] {
            assert_eq!(NaiveDate::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
    }

    #[test]
    fn month_years() {
        for text in ["202612", "20261231", "202612w1", "202612w5", "000101"] {
            assert_eq!(MonthYear::from_fix(text).unwrap().to_fix(), text);
        }
        let day = MonthYear::from_fix("20261231").unwrap();
        assert_eq!(day.day_or_week(), Some(DayOrWeek::Day(31)));
        for bad in ["202613", "202600", "20260231", "202612w0", "202612w6", "202612x1", "2026121", "2026"] {
            assert_eq!(MonthYear::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
        assert!(MonthYear::new(2026, 2).unwrap().with_day(30).is_none());
        assert!(MonthYear::new(10_000, 1).is_none());
    }

    #[test]
    fn tz_times_of_day() {
        for text in ["09:30", "09:30Z", "09:30:15", "09:30:15.250-05", "09:30+05:30", "23:59:60Z"] {
            assert_eq!(TzTimeOnly::from_fix(text).unwrap().to_fix(), text, "{text}");
        }
        let t = TzTimeOnly::from_fix("09:30-03:30").unwrap();
        assert_eq!(t.offset().unwrap().local_minus_utc(), -(3 * 3600 + 30 * 60));
        assert_eq!(t.precision(), None);
        assert_eq!(TzTimeOnly::from_fix("09:30+00").unwrap().to_fix(), "09:30Z", "UTC is written Z");
        assert_eq!(TzTimeOnly::from_fix("09:30+05:00").unwrap().to_fix(), "09:30+05");
        for bad in ["9:30", "09:30+5", "09:30+15", "09:30+05:60", "09:30+0530", "09:30z", "09:30:15.", "09"] {
            assert_eq!(TzTimeOnly::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
    }

    #[test]
    fn tz_timestamps() {
        for text in [
            "20060901-07:39Z",
            "20060901-02:39-05",
            "20060901-15:39+08",
            "20060901-13:09+05:30",
            "20060901-07:39:15.123456Z",
        ] {
            assert_eq!(TzTimestamp::from_fix(text).unwrap().to_fix(), text, "{text}");
        }
        // The examples from the FIX specification are the same instant.
        let instants: Vec<_> = ["20060901-07:39Z", "20060901-02:39-05", "20060901-15:39+08", "20060901-13:09+05:30"]
            .into_iter()
            .map(|text| TzTimestamp::from_fix(text).unwrap().datetime().unwrap())
            .collect();
        assert!(instants.windows(2).all(|pair| pair[0] == pair[1]), "{instants:?}");
        assert_eq!(TzTimestamp::from_fix("20060901-07:39").unwrap().datetime(), None, "no offset");
        for bad in ["20060901-07", "2006091-07:39Z", "20060931-07:39Z", "20060901 07:39Z"] {
            assert_eq!(TzTimestamp::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
    }
}
