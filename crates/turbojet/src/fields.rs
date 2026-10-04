//! Typed FIX field values: conversions for the FIX data types, and the enumerated fields used by
//! the session layer. Enumerations whose value sets differ by FIX version live in the generated
//! version crates, e.g. `turbojet-fix42`.

use std::fmt;
use std::str::FromStr;

use std::borrow::Cow;

/// The type of string fields in typed messages: values up to 24 bytes are kept inline, so most
/// IDs and symbols are parsed and built without allocating. Build one with `.into()` from a
/// `&str` or `String`, or [`format_compact!`] in place of `format!`.
pub use compact_str::{CompactString, ToCompactString, format_compact};
pub use rust_decimal::Decimal;

mod borrowed;
mod time;

pub use borrowed::{CodeRef, FieldRef, List, ListIter, SecretRef};
pub use time::{
    DayOrWeek, MonthYear, NaiveDate, NaiveTime, Precision, TzTimeOnly, TzTimestamp, UtcTimeOnly, UtcTimestamp,
};

/// Why a raw field value could not be converted to its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    ///
    /// # Errors
    ///
    /// [`ValueError`] if `s` isn't in the type's format, or isn't one of its permitted values.
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

impl ToFix for CompactString {
    fn write_fix(&self, out: &mut String) {
        out.push_str(self)
    }
}

/// Appends `ascii`, which the caller has just written as ASCII (digits, punctuation), without
/// the UTF-8 check `str::from_utf8` would make. Faster for the short runs of an integer or a
/// decimal; for a timestamp's 20 or so bytes, the check and one copy are faster (measured).
fn push_ascii(out: &mut String, ascii: &[u8]) {
    debug_assert!(ascii.is_ascii());
    out.extend(ascii.iter().map(|&b| char::from(b)));
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
    push_ascii(out, &digits[start..]);
}

impl FromFix for String {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(s.to_string())
    }
}

impl FromFix for CompactString {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        Ok(Self::from(s))
    }
}

/// char: exactly one character.
impl FromFix for char {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(ValueError::Format),
        }
    }
}

impl ToFix for char {
    fn write_fix(&self, out: &mut String) {
        out.push(*self)
    }
}

/// MultipleCharValue, MultipleStringValue and MultipleValueString: values separated by single
/// spaces, e.g. ExecInst(18) `1 G`. An empty value, or a doubled space, is a format error.
impl<T: FromFix> FromFix for Vec<T> {
    fn from_fix(s: &str) -> Result<Self, ValueError> {
        s.split(' ').map(|value| if value.is_empty() { Err(ValueError::Format) } else { T::from_fix(value) }).collect()
    }
}

impl<T: ToFix> ToFix for Vec<T> {
    fn write_fix(&self, out: &mut String) {
        for (i, value) in self.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            value.write_fix(out);
        }
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
    /// As `Display` writes it, without the formatting machinery: the mantissa's digits, with the
    /// point `scale` digits from the right and at least one digit before it.
    fn write_fix(&self, out: &mut String) {
        if self.is_sign_negative() {
            out.push('-');
        }
        // A 96-bit mantissa has at most 29 digits; the scale is at most 28.
        let mut digits = [b'0'; 40];
        let mut start = digits.len();
        let mantissa = self.mantissa().unsigned_abs();
        if let Ok(mut n) = u64::try_from(mantissa) {
            loop {
                start -= 1;
                digits[start] = b'0' + (n % 10) as u8;
                n /= 10;
                if n == 0 {
                    break;
                }
            }
        } else {
            let mut n = mantissa;
            while n > 0 {
                start -= 1;
                digits[start] = b'0' + (n % 10) as u8;
                n /= 10;
            }
        }
        let scale = self.scale() as usize;
        // Leading zeros (already in the buffer) up to one digit before the point.
        start = start.min(digits.len() - scale - 1);
        let point = digits.len() - scale;
        push_ascii(out, &digits[start..point]);
        if scale > 0 {
            out.push('.');
            push_ascii(out, &digits[point..]);
        }
    }
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

    /// Decimals are written as `Display` writes them: every scale, sign and size of mantissa.
    #[test]
    fn decimals_are_written_as_display_writes_them() {
        let mut state: u64 = 0x2545_f491_4f6c_dd1d;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut values = vec![Decimal::ZERO, Decimal::from_parts(0, 0, 0, true, 2), Decimal::MAX, Decimal::MIN];
        for scale in 0..=28 {
            for _ in 0..200 {
                #[expect(clippy::cast_possible_truncation, reason = "the low 32 random bits")]
                let (lo, mid, hi) = (next() as u32, next() as u32, next() as u32);
                // Mantissas of 32, 64 and 96 bits, and small ones that need leading zeros.
                let (lo, mid, hi) = match next() % 4 {
                    0 => (lo, 0, 0),
                    1 => (lo, mid, 0),
                    2 => (lo, mid, hi),
                    _ => (lo % 1000, 0, 0),
                };
                values.push(Decimal::from_parts(lo, mid, hi, next() % 2 == 0, scale));
            }
        }
        for d in values {
            assert_eq!(d.to_fix(), d.to_string(), "{d:?}");
        }
    }

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
    fn integers_and_booleans() {
        assert_eq!(u64::from_fix("42"), Ok(42));
        assert_eq!(u64::from_fix("-1"), Err(ValueError::Format));
        assert_eq!(u64::from_fix("4x"), Err(ValueError::Format));
        assert_eq!(bool::from_fix("Y"), Ok(true));
        assert_eq!(bool::from_fix("yes"), Err(ValueError::Incorrect));
    }

    #[test]
    fn chars_are_one_character() {
        assert_eq!(char::from_fix("A"), Ok('A'));
        assert_eq!('A'.to_fix(), "A");
        for bad in ["", "AB"] {
            assert_eq!(char::from_fix(bad), Err(ValueError::Format), "{bad:?}");
        }
    }

    #[test]
    fn lists_are_separated_by_single_spaces() {
        let list: Vec<Code<EncryptMethod>> = Vec::from_fix("0 Z").unwrap();
        assert_eq!(list, [Code::Known(EncryptMethod::None), Code::Unknown("Z".into())]);
        assert_eq!(list.to_fix(), "0 Z");
        assert_eq!(Vec::<String>::from_fix("A"), Ok(vec!["A".to_string()]));
        for bad in ["", " 0", "0 ", "0  1"] {
            assert_eq!(Vec::<String>::from_fix(bad), Err(ValueError::Format), "{bad:?}");
        }
        assert_eq!(Vec::<EncryptMethod>::from_fix("0 Z"), Err(ValueError::Incorrect), "an unknown code");
    }
}
