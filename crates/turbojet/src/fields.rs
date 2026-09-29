//! Typed FIX field values: conversions for the FIX data types, and the enumerated fields used by
//! the session layer. Enumerations whose value sets differ by FIX version live in the generated
//! version crates, e.g. `turbojet-fix42`.

use std::fmt::{self, Write as _};
use std::str::FromStr;

use std::borrow::Cow;
use std::cell::Cell;

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

pub use rust_decimal::Decimal;

/// UTCTimestamp.
pub type UtcTimestamp = DateTime<Utc>;

/// Why a raw field value could not be converted to its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueError {
    /// Well-formed, but not a permitted value, e.g. an unknown enum code.
    Incorrect,
    /// Not in the field's data format, e.g. letters in a Qty.
    Format,
}

/// An enumerated field's type, as defined with [`fix_enum!`](crate::fix_enum): its values and
/// their FIX codes.
pub trait FixEnum: Sized + Copy {
    /// The value's FIX code.
    fn code(self) -> &'static str;

    /// The value for a FIX code, if it's one of this enum's.
    fn from_code(code: &str) -> Option<Self>;
}

/// An enumerated field's value that may be a code the enum doesn't list: counterparties do send
/// values outside the spec. Parsing a `Code` never fails on an unknown code, where parsing the
/// enum itself does. Generated messages use it for the fields made lenient with
/// `turbojet-codegen`'s `lenient_enums`.
///
/// ```
/// use turbojet::fields::{Code, EncryptMethod, FromFix};
///
/// let method: Code<EncryptMethod> = Code::from_fix("Z").unwrap();
/// assert_eq!(method, Code::Unknown("Z".into()));
/// assert_eq!(Code::from_fix("0"), Ok(Code::Known(EncryptMethod::None)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Code<E> {
    /// One of the enum's values.
    Known(E),
    /// A code the enum doesn't list.
    Unknown(String),
}

impl<E: FixEnum> Code<E> {
    /// The value, if it's one the enum lists.
    pub fn known(&self) -> Option<E> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Unknown(_) => None,
        }
    }

    /// The FIX code, known or not.
    pub fn code(&self) -> &str {
        match self {
            Self::Known(value) => value.code(),
            Self::Unknown(code) => code,
        }
    }
}

impl<E> From<E> for Code<E> {
    fn from(value: E) -> Self {
        Self::Known(value)
    }
}

impl<E: FixEnum> FromFix for Code<E> {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(E::from_code(s).map_or_else(|| Self::Unknown(s.to_string()), Self::Known))
    }
}

impl<E: FixEnum> ToFix for Code<E> {
    fn write_fix(&self, out: &mut String) {
        out.push_str(self.code())
    }
}

impl<E: FixEnum> fmt::Display for Code<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// A value that mustn't be shown, such as a password: sent on the wire as it is, but `Debug` and
/// `Display` show `***`, so it can't leak into logs. Password(554) and NewPassword(925) fields
/// use it.
///
/// ```
/// use turbojet::fields::Secret;
///
/// let password = Secret::from("hunter2");
/// assert_eq!(format!("{password:?}"), "***");
/// assert_eq!(password.expose(), "hunter2");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Secret(String);

impl Secret {
    /// The value itself.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Secret {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl FromFix for Secret {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(Self(s.to_string()))
    }
}

impl ToFix for Secret {
    fn write_fix(&self, out: &mut String) {
        out.push_str(&self.0)
    }
}

/// Parses a field value from its wire form.
pub trait FromFix: Sized {
    /// Parses `s`, a value without its tag or delimiter.
    fn from_fix(s: &str) -> Result<Self, ValueError>;
}

/// Renders a field value in its wire form.
pub trait ToFix {
    /// Appends the wire form to `out`. Used to write values straight into a message's buffer.
    fn write_fix(&self, out: &mut String);

    /// The wire form as a new string.
    fn to_fix(&self) -> String {
        let mut out = String::new();
        self.write_fix(&mut out);
        out
    }
}

impl<T: ToFix + ?Sized> ToFix for &T {
    fn write_fix(&self, out: &mut String) {
        (**self).write_fix(out)
    }
}

impl ToFix for str {
    fn write_fix(&self, out: &mut String) {
        out.push_str(self)
    }
}

impl ToFix for String {
    fn write_fix(&self, out: &mut String) {
        out.push_str(self)
    }
}

/// Appends the decimal digits of `n` without allocating.
fn write_unsigned(out: &mut String, mut n: u64) {
    let mut digits = [0u8; 20];
    let mut start = digits.len();
    loop {
        start -= 1;
        digits[start] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.push_str(std::str::from_utf8(&digits[start..]).expect("ASCII digits"));
}

impl FromFix for String {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(s.to_string())
    }
}

macro_rules! unsigned {
    ($($ty:ty),*) => {$(
        impl FromFix for $ty {
            fn from_fix(s: &str) -> Result<Self, ValueError> {
                if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(ValueError::Format);
                }
                s.parse().map_err(|_| ValueError::Format)
            }
        }

        impl ToFix for $ty {
            fn write_fix(&self, out: &mut String) {
                write_unsigned(out, u64::from(*self))
            }
        }
    )*};
}

unsigned!(u32, u64);

/// Int and other signed types: an optional `-` and digits.
impl FromFix for i64 {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let digits = s.strip_prefix('-').unwrap_or(s);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(ValueError::Format);
        }
        s.parse().map_err(|_| ValueError::Format)
    }
}

impl ToFix for i64 {
    fn write_fix(&self, out: &mut String) {
        if *self < 0 {
            out.push('-');
        }
        write_unsigned(out, self.unsigned_abs())
    }
}

/// Boolean: `Y` or `N`.
impl FromFix for bool {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        match s {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(ValueError::Incorrect),
        }
    }
}

impl ToFix for bool {
    fn write_fix(&self, out: &mut String) {
        out.push(if *self { 'Y' } else { 'N' })
    }
}

/// Price, Qty, Amt and other float types: an optional `-`, digits and at most one `.`.
/// Exponents, `+`, and digit separators are not FIX and are refused.
impl FromFix for Decimal {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let digits = s.strip_prefix('-').unwrap_or(s);
        let well_formed = digits.bytes().any(|b| b.is_ascii_digit())
            && digits.bytes().all(|b| b.is_ascii_digit() || b == b'.')
            && digits.bytes().filter(|&b| b == b'.').count() <= 1;
        if !well_formed {
            return Err(ValueError::Format);
        }
        Decimal::from_str(s).map_err(|_| ValueError::Format)
    }
}

impl ToFix for Decimal {
    fn write_fix(&self, out: &mut String) {
        write!(out, "{self}").expect("writing to a String cannot fail")
    }
}

/// `YYYYMMDD-HH:MM:SS` with an optional fraction of 1 to 9 digits; always sent with milliseconds.
impl FromFix for UtcTimestamp {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let b = s.as_bytes();
        let digits = |range: std::ops::Range<usize>| b.get(range).is_some_and(|d| d.iter().all(u8::is_ascii_digit));
        let shape = b.len() >= 17
            && digits(0..8)
            && b[8] == b'-'
            && digits(9..11)
            && b[11] == b':'
            && digits(12..14)
            && b[14] == b':'
            && digits(15..17)
            && (b.len() == 17 || (b[17] == b'.' && (19..=27).contains(&b.len()) && digits(18..b.len())));
        if !shape {
            return Err(ValueError::Format);
        }
        NaiveDateTime::parse_from_str(s, "%Y%m%d-%H:%M:%S%.f").map(|t| t.and_utc()).map_err(|_| ValueError::Format)
    }
}

impl ToFix for UtcTimestamp {
    fn write_fix(&self, out: &mut String) {
        let millis = self.timestamp_subsec_millis();
        // Leap seconds and years outside 0..=9999 are rare enough to leave to chrono.
        if millis >= 1000 || !(0..=9999).contains(&self.year()) {
            write!(out, "{}", self.format("%Y%m%d-%H:%M:%S%.3f")).expect("writing to a String cannot fail");
            return;
        }
        out.push_str(std::str::from_utf8(&seconds_prefix(self)).expect("ASCII"));
        out.push('.');
        out.push(char::from(b'0' + (millis / 100) as u8));
        out.push(char::from(b'0' + (millis / 10 % 10) as u8));
        out.push(char::from(b'0' + (millis % 10) as u8));
    }
}

thread_local! {
    /// The most recently formatted second on this thread: (Unix seconds, `YYYYMMDD-HH:MM:SS`).
    static SECONDS_PREFIX: Cell<(i64, [u8; 17])> = const { Cell::new((i64::MIN, [0; 17])) };
}

/// `YYYYMMDD-HH:MM:SS` for `time`, which must have a four-digit year. Cached per thread, since
/// a busy session formats the same second many times.
fn seconds_prefix(time: &UtcTimestamp) -> [u8; 17] {
    let seconds = time.timestamp();
    SECONDS_PREFIX.with(|cache| {
        let (cached_seconds, prefix) = cache.get();
        if cached_seconds == seconds {
            return prefix;
        }
        let mut prefix = [0u8; 17];
        let mut put = |at: usize, value: u32, width: usize| {
            let mut value = value;
            for i in (at..at + width).rev() {
                prefix[i] = b'0' + (value % 10) as u8;
                value /= 10;
            }
        };
        put(0, time.year() as u32, 4);
        put(4, time.month(), 2);
        put(6, time.day(), 2);
        put(9, time.hour(), 2);
        put(12, time.minute(), 2);
        put(15, time.second(), 2);
        prefix[8] = b'-';
        prefix[11] = b':';
        prefix[14] = b':';
        cache.set((seconds, prefix));
        prefix
    })
}

/// Defines [`MsgType`] from one table of variants and codes.
macro_rules! msg_types {
    ($(#[$meta:meta])* $( $variant:ident = $code:literal, )+) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum MsgType {
            $( #[doc = concat!(stringify!($variant), "(", $code, ").")] $variant, )+
            /// A code without a variant of its own, such as a venue's custom message type.
            Other(Cow<'static, str>),
        }

        impl MsgType {
            /// The MsgType(35) code, e.g. `"D"`.
            pub fn code(&self) -> &str {
                match self {
                    $( Self::$variant => $code, )+
                    Self::Other(code) => code,
                }
            }

            /// The message type with MsgType(35) code `code`.
            pub fn from_code(code: &str) -> Self {
                match code {
                    $( $code => Self::$variant, )+
                    other => Self::Other(Cow::Owned(other.to_string())),
                }
            }

            /// Like [`MsgType::from_code`], usable in constants: known codes map to their
            /// variant, and anything else to `Other` without allocating.
            pub const fn from_static(code: &'static str) -> Self {
                $( if str_eq(code, $code) { return Self::$variant; } )+
                Self::Other(Cow::Borrowed(code))
            }
        }
    };
}

const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

msg_types! {
    /// MsgType(35). Types the engine models have their own variant; any other value is `Other`.
    ///
    /// For a custom or venue-specific type in a `const` (e.g. a typed message's
    /// [`MSG_TYPE`](crate::message::FixMessage::MSG_TYPE)), use [`MsgType::from_static`].
    Heartbeat = "0",
    TestRequest = "1",
    ResendRequest = "2",
    Reject = "3",
    SequenceReset = "4",
    Logout = "5",
    ExecutionReport = "8",
    OrderCancelReject = "9",
    Logon = "A",
    NewOrderSingle = "D",
    OrderCancelRequest = "F",
    OrderCancelReplaceRequest = "G",
    OrderStatusRequest = "H",
    BusinessMessageReject = "j",
}

impl MsgType {
    /// Session-level message types, which are gap-filled rather than resent.
    pub fn is_admin(&self) -> bool {
        matches!(
            self,
            Self::Heartbeat
                | Self::TestRequest
                | Self::ResendRequest
                | Self::Reject
                | Self::SequenceReset
                | Self::Logout
                | Self::Logon
        )
    }
}

impl FromFix for MsgType {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(Self::from_code(s))
    }
}

impl ToFix for MsgType {
    fn write_fix(&self, out: &mut String) {
        out.push_str(self.code())
    }
}

impl fmt::Display for MsgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

fix_enum! {
    /// EncryptMethod(98).
    EncryptMethod {
        /// None / other.
        None = "0",
        /// PKCS (proprietary).
        PKCS = "1",
        /// DES (ECB mode).
        DES = "2",
        /// PKCS/DES (proprietary).
        PKCSDES = "3",
        /// PGP/DES (defunct).
        PGPDES = "4",
        /// PGP/DES-MD5.
        PGPDESMD5 = "5",
        /// PEM/DES-MD5.
        PEMDESMD5 = "6",
    }
}

fix_enum! {
    /// ApplVerID(1128) and DefaultApplVerID(1137): the application version on a FIXT session.
    ApplVerId {
        /// FIX 2.7.
        Fix27 = "0",
        /// FIX 3.0.
        Fix30 = "1",
        /// FIX 4.0.
        Fix40 = "2",
        /// FIX 4.1.
        Fix41 = "3",
        /// FIX 4.2.
        Fix42 = "4",
        /// FIX 4.3.
        Fix43 = "5",
        /// FIX 4.4.
        Fix44 = "6",
        /// FIX 5.0.
        Fix50 = "7",
        /// FIX 5.0 SP1.
        Fix50Sp1 = "8",
        /// FIX 5.0 SP2.
        Fix50Sp2 = "9",
        /// FIXLatest: the latest extension pack.
        FixLatest = "10",
    }
}

fix_enum! {
    /// SessionRejectReason(373). Values 0-11 are FIX 4.2; the rest were added in later versions.
    SessionRejectReason {
        /// Invalid tag number.
        InvalidTagNumber = "0",
        /// Required tag missing.
        RequiredTagMissing = "1",
        /// Tag not defined for this message type.
        TagNotDefinedForMessageType = "2",
        /// Undefined tag.
        UndefinedTag = "3",
        /// Tag specified without a value.
        TagSpecifiedWithoutValue = "4",
        /// Value is incorrect (out of range) for this tag.
        ValueIsIncorrect = "5",
        /// Incorrect data format for value.
        IncorrectDataFormat = "6",
        /// Decryption problem.
        DecryptionProblem = "7",
        /// Signature problem.
        SignatureProblem = "8",
        /// CompID problem.
        CompIDProblem = "9",
        /// SendingTime accuracy problem.
        SendingTimeAccuracyProblem = "10",
        /// Invalid MsgType.
        InvalidMsgType = "11",
        /// XML validation error.
        XMLValidationError = "12",
        /// Tag appears more than once.
        TagAppearsMoreThanOnce = "13",
        /// Tag specified out of required order.
        TagSpecifiedOutOfRequiredOrder = "14",
        /// Repeating group fields out of order.
        RepeatingGroupFieldsOutOfOrder = "15",
        /// Incorrect NumInGroup count for repeating group.
        IncorrectNumInGroupCount = "16",
        /// Non-data value includes the field delimiter (SOH).
        NonDataValueIncludesFieldDelimiter = "17",
        /// Invalid or unsupported application version.
        InvalidUnsupportedApplicationVersion = "18",
        /// Other.
        Other = "99",
    }
}

fix_enum! {
    /// BusinessRejectReason(380). Values 0-5 are FIX 4.2; the rest were added in later versions.
    BusinessRejectReason {
        /// Other.
        Other = "0",
        /// Unknown ID.
        UnknownID = "1",
        /// Unknown security.
        UnknownSecurity = "2",
        /// Unsupported message type.
        UnsupportedMessageType = "3",
        /// Application not available.
        ApplicationNotAvailable = "4",
        /// Conditionally required field missing.
        ConditionallyRequiredFieldMissing = "5",
        /// Not authorized.
        NotAuthorized = "6",
        /// DeliverTo firm not available at this time.
        DeliverToFirmNotAvailable = "7",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_versions_use_fixt_codes() {
        assert_eq!(ApplVerId::Fix50Sp2.to_fix(), "9");
        assert_eq!(ApplVerId::from_fix("10"), Ok(ApplVerId::FixLatest));
        assert_eq!(ApplVerId::from_fix("11"), Err(ValueError::Incorrect));
        assert_eq!(SessionRejectReason::from_fix("18"), Ok(SessionRejectReason::InvalidUnsupportedApplicationVersion));
    }

    #[test]
    fn signed_integers_round_trip() {
        for (text, value) in [("0", 0i64), ("42", 42), ("-7", -7), ("-9223372036854775808", i64::MIN)] {
            assert_eq!(i64::from_fix(text), Ok(value));
            assert_eq!(value.to_fix(), text);
        }
        for bad in ["", "-", "+1", " 1", "1.0", "--1", "99999999999999999999"] {
            assert_eq!(i64::from_fix(bad), Err(ValueError::Format), "{bad:?}");
        }
    }

    #[test]
    fn enums_round_trip_and_refuse_unknown_codes() {
        assert_eq!(SessionRejectReason::from_fix("5"), Ok(SessionRejectReason::ValueIsIncorrect));
        assert_eq!(SessionRejectReason::CompIDProblem.to_fix(), "9");
        assert_eq!(SessionRejectReason::from_fix("99"), Ok(SessionRejectReason::Other));
        assert_eq!(EncryptMethod::from_fix("Z"), Err(ValueError::Incorrect));
    }

    #[test]
    fn secrets_are_sent_but_never_shown() {
        let secret: Secret = Secret::from_fix("hunter2").unwrap();
        assert_eq!(secret.expose(), "hunter2");
        assert_eq!(secret.to_fix(), "hunter2");
        assert_eq!((format!("{secret:?}"), secret.to_string()), ("***".to_string(), "***".to_string()));
        assert_eq!(Secret::from("hunter2"), secret);
        assert_eq!(Secret::from(String::from("hunter2")), secret);
    }

    #[test]
    fn codes_keep_values_the_enum_does_not_know() {
        let known: Code<EncryptMethod> = Code::from_fix("1").unwrap();
        assert_eq!(known, Code::Known(EncryptMethod::PKCS));
        assert_eq!(known.known(), Some(EncryptMethod::PKCS));
        assert_eq!((known.code(), known.to_fix()), ("1", "1".to_string()));

        let unknown: Code<EncryptMethod> = Code::from_fix("Z").unwrap();
        assert_eq!(unknown, Code::Unknown("Z".into()));
        assert_eq!(unknown.known(), None);
        assert_eq!((unknown.code(), unknown.to_fix(), unknown.to_string()), ("Z", "Z".to_string(), "Z".to_string()));

        assert_eq!(Code::from(EncryptMethod::DES), Code::Known(EncryptMethod::DES));
        assert_eq!(EncryptMethod::DES, Code::Known(EncryptMethod::DES));
    }

    #[test]
    fn msg_type_keeps_unknown_codes() {
        assert_eq!(MsgType::from_code("D"), MsgType::NewOrderSingle);
        assert_eq!(MsgType::from_code("AE"), MsgType::Other("AE".into()));
        const CUSTOM: MsgType = MsgType::from_static("U1");
        const KNOWN: MsgType = MsgType::from_static("D");
        assert_eq!(CUSTOM, MsgType::from_code("U1"), "borrowed and owned codes compare equal");
        assert_eq!(KNOWN, MsgType::NewOrderSingle, "known codes map to their variant");
        assert_eq!(MsgType::from_code("G"), MsgType::OrderCancelReplaceRequest);
        assert_eq!(MsgType::OrderStatusRequest.code(), "H");
        assert_eq!(MsgType::from_code("AE").code(), "AE");
        assert!(MsgType::SequenceReset.is_admin());
        assert!(!MsgType::from_code("AE").is_admin());
    }

    #[test]
    fn decimals_are_strict_and_keep_scale() {
        for ok in ["150.25", "100", "-0.10", ".5", "1."] {
            assert!(Decimal::from_fix(ok).is_ok(), "{ok}");
        }
        assert_eq!(Decimal::from_fix("150.250").unwrap().to_fix(), "150.250");
        for bad in ["1e3", "1_000", "+1", "", "-", ".", "1.2.3", "abc"] {
            assert_eq!(Decimal::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
    }

    #[test]
    fn timestamps_accept_fractions_and_render_millis() {
        let t = UtcTimestamp::from_fix("20260927-03:20:48").unwrap();
        assert_eq!(t.to_fix(), "20260927-03:20:48.000");
        let t = UtcTimestamp::from_fix("20260927-03:20:48.544123").unwrap();
        assert_eq!(t.to_fix(), "20260927-03:20:48.544");
        for bad in ["20260927-3:20:48", "2026-09-27T03:20:48", "20260927-03:20:48.", "20261327-03:20:48"] {
            assert_eq!(UtcTimestamp::from_fix(bad), Err(ValueError::Format), "{bad}");
        }
    }

    /// What chrono produced before the hand-written formatter.
    fn chrono_format(t: &UtcTimestamp) -> String {
        t.format("%Y%m%d-%H:%M:%S%.3f").to_string()
    }

    #[test]
    fn timestamp_formatting_matches_chrono() {
        use chrono::TimeZone;
        // Deterministic pseudo-random instants from 1970 to 2199, with arbitrary nanoseconds.
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..20_000 {
            let seconds = (next() % 7_258_118_400) as i64; // up to 2200-01-01
            let nanos = (next() % 1_000_000_000) as u32;
            let t = Utc.timestamp_opt(seconds, nanos).unwrap();
            assert_eq!(t.to_fix(), chrono_format(&t), "{t:?}");
            // And again, now from the cache.
            assert_eq!(t.to_fix(), chrono_format(&t), "{t:?} (cached)");
        }
    }

    #[test]
    fn timestamp_edge_cases_match_chrono() {
        use chrono::{NaiveDate, TimeZone};
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
        for t in cases {
            assert_eq!(t.to_fix(), chrono_format(&t), "{t:?}");
        }
    }

    #[test]
    fn timestamp_cache_is_per_second_and_per_thread() {
        let base = UtcTimestamp::from_fix("20260927-03:20:48.544").unwrap();
        let next_second = base + chrono::Duration::milliseconds(700);
        assert_eq!(base.to_fix(), "20260927-03:20:48.544");
        assert_eq!(next_second.to_fix(), "20260927-03:20:49.244");
        assert_eq!(base.to_fix(), "20260927-03:20:48.544", "going back a second refreshes the cache");
        let other = std::thread::spawn(move || (base + chrono::Duration::days(1)).to_fix()).join().unwrap();
        assert_eq!(other, "20260928-03:20:48.544");
        assert_eq!(base.to_fix(), "20260927-03:20:48.544");
    }

    #[test]
    fn integers_and_booleans() {
        assert_eq!(u64::from_fix("42"), Ok(42));
        assert_eq!(u64::from_fix("-1"), Err(ValueError::Format));
        assert_eq!(u64::from_fix("4x"), Err(ValueError::Format));
        assert_eq!(bool::from_fix("Y"), Ok(true));
        assert_eq!(bool::from_fix("yes"), Err(ValueError::Incorrect));
    }
}
