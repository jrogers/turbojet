//! Borrowed field values, for the `…Ref` types `fix_message!` generates: a message parsed into
//! them keeps its strings in the raw [`Message`](crate::Message) rather than copying them.

use std::fmt;

use super::{
    Code, Decimal, FixEnum, MonthYear, NaiveDate, Secret, TzTimeOnly, TzTimestamp, UtcTimeOnly, UtcTimestamp,
    ValueError,
};

/// A field type's borrowed form, for the `…Ref` types `fix_message!` generates: what a field of
/// this type holds when parsed without allocating, and how to make the owned value from it.
pub trait FieldRef<'a>: Sized {
    /// The borrowed form: `&'a str` for `String`, the type itself for `Copy` scalars.
    type Ref: Copy + fmt::Debug + PartialEq + 'a;

    /// Parses a value without its tag or delimiter; fails as [`FromFix::from_fix`](super::FromFix::from_fix)
    /// would.
    fn parse_ref(s: &'a str) -> Result<Self::Ref, ValueError>;

    /// The owned value.
    fn into_owned(value: Self::Ref) -> Self;
}

impl_field_ref!(
    u32,
    u64,
    i64,
    bool,
    char,
    Decimal,
    UtcTimestamp,
    UtcTimeOnly,
    NaiveDate,
    MonthYear,
    TzTimeOnly,
    TzTimestamp
);

impl<'a> FieldRef<'a> for String {
    type Ref = &'a str;

    fn parse_ref(s: &'a str) -> Result<&'a str, ValueError> {
        Ok(s)
    }

    fn into_owned(value: &'a str) -> Self {
        value.to_owned()
    }
}

/// A borrowed [`Secret`]: `Debug` and `Display` show `***`, so it can't leak into logs.
///
/// ```
/// use turbojet::fields::SecretRef;
///
/// let password = SecretRef::from("hunter2");
/// assert_eq!(format!("{password:?}"), "***");
/// assert_eq!(password.expose(), "hunter2");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecretRef<'a>(&'a str);

impl<'a> SecretRef<'a> {
    /// The value itself.
    pub fn expose(&self) -> &'a str {
        self.0
    }
}

impl<'a> From<&'a str> for SecretRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl fmt::Debug for SecretRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl fmt::Display for SecretRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl<'a> FieldRef<'a> for Secret {
    type Ref = SecretRef<'a>;

    fn parse_ref(s: &'a str) -> Result<SecretRef<'a>, ValueError> {
        Ok(SecretRef(s))
    }

    fn into_owned(value: SecretRef<'a>) -> Self {
        Self::from(value.0)
    }
}

/// A borrowed [`Code`]: one of the enum's values, or a code it doesn't list, borrowed.
///
/// ```
/// use turbojet::fields::{Code, CodeRef, EncryptMethod, FieldRef};
///
/// let method = Code::<EncryptMethod>::parse_ref("Z").unwrap();
/// assert_eq!(method, CodeRef::Unknown("Z"));
/// assert_eq!(Code::<EncryptMethod>::parse_ref("0"), Ok(CodeRef::Known(EncryptMethod::None)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeRef<'a, E> {
    /// One of the enum's values.
    Known(E),
    /// A code the enum doesn't list.
    Unknown(&'a str),
}

impl<'a, E: FixEnum> CodeRef<'a, E> {
    /// The value, if it's one the enum lists.
    pub fn known(&self) -> Option<E> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Unknown(_) => None,
        }
    }

    /// The FIX code, known or not.
    pub fn code(&self) -> &'a str {
        match self {
            Self::Known(value) => value.code(),
            Self::Unknown(code) => code,
        }
    }
}

impl<E> From<E> for CodeRef<'_, E> {
    fn from(value: E) -> Self {
        Self::Known(value)
    }
}

impl<E: FixEnum> fmt::Display for CodeRef<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl<'a, E: FixEnum + fmt::Debug + PartialEq + 'a> FieldRef<'a> for Code<E> {
    type Ref = CodeRef<'a, E>;

    fn parse_ref(s: &'a str) -> Result<CodeRef<'a, E>, ValueError> {
        Ok(E::from_code(s).map_or(CodeRef::Unknown(s), CodeRef::Known))
    }

    fn into_owned(value: CodeRef<'a, E>) -> Self {
        match value {
            CodeRef::Known(value) => Self::Known(value),
            CodeRef::Unknown(code) => Self::Unknown(code.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    crate::fix_enum! {
        /// A test enum.
        Side {
            /// Buy.
            Buy = "1",
            /// Sell.
            Sell = "2",
        }
    }

    /// Checks `parse_ref` against `from_fix`: the first value is good and the last bad.
    fn parses_as_owned<T>(values: &[&str])
    where
        T: for<'a> FieldRef<'a, Ref = T> + FromFix + PartialEq + fmt::Debug,
    {
        assert!(T::from_fix(values[0]).is_ok(), "{:?}", values[0]);
        assert!(T::from_fix(values[values.len() - 1]).is_err(), "{:?}", values[values.len() - 1]);
        for &s in values {
            assert_eq!(T::parse_ref(s), T::from_fix(s), "{s:?}");
            if let Ok(value) = T::parse_ref(s) {
                assert_eq!(T::into_owned(value), T::from_fix(s).expect("parsed above"), "{s:?}");
            }
        }
    }

    #[test]
    fn scalars_are_their_own_borrowed_form() {
        parses_as_owned::<u32>(&["42", "-1", "", "99999999999"]);
        parses_as_owned::<u64>(&["42", "x"]);
        parses_as_owned::<i64>(&["-7", "--7"]);
        parses_as_owned::<bool>(&["Y", "N", "yes"]);
        parses_as_owned::<char>(&["A", "AB"]);
        parses_as_owned::<Decimal>(&["150.25", "1e3"]);
        parses_as_owned::<UtcTimestamp>(&["20260930-12:00:00.123", "2026-09-30"]);
        parses_as_owned::<UtcTimeOnly>(&["12:00:00", "25:00:00"]);
        parses_as_owned::<NaiveDate>(&["20260930", "20261330"]);
        parses_as_owned::<MonthYear>(&["202609", "202609w2", "2026"]);
        parses_as_owned::<TzTimeOnly>(&["12:00Z", "12:00+05:30", "12"]);
        parses_as_owned::<TzTimestamp>(&["20260930-12:00:00Z", "20260930"]);
        parses_as_owned::<Side>(&["1", "3"]);
        parses_as_owned::<EncryptMethod>(&["0", "Z"]);
    }

    #[test]
    fn strings_borrow_the_input() {
        let input = "order-1";
        let s = String::parse_ref(input).unwrap();
        assert!(std::ptr::eq(s.as_ptr(), input.as_ptr()));
        assert_eq!(String::into_owned(s), String::from_fix(input).unwrap());
    }

    #[test]
    fn borrowed_secrets_are_never_shown() {
        let secret = Secret::parse_ref("hunter2").unwrap();
        assert_eq!((format!("{secret:?}"), secret.to_string()), ("***".to_string(), "***".to_string()));
        assert_eq!(secret.expose(), "hunter2");
        assert_eq!(Secret::into_owned(secret), Secret::from("hunter2"));
        assert_eq!(secret, SecretRef::from("hunter2"));
    }

    #[test]
    fn borrowed_codes_keep_values_the_enum_does_not_know() {
        let known = Code::<Side>::parse_ref("1").unwrap();
        assert_eq!(known, CodeRef::Known(Side::Buy));
        assert_eq!((known.known(), known.code(), known.to_string()), (Some(Side::Buy), "1", "1".to_string()));
        assert!(known == Side::Buy);
        assert!(Side::Buy == known);
        assert!(known != Side::Sell);
        assert_eq!(CodeRef::from(Side::Buy), known);
        assert_eq!(Code::into_owned(known), Code::from_fix("1").unwrap());

        let input = "Z";
        let unknown = Code::<Side>::parse_ref(input).unwrap();
        assert_eq!(unknown, CodeRef::Unknown("Z"));
        assert_eq!(unknown.known(), None);
        assert!(std::ptr::eq(unknown.code().as_ptr(), input.as_ptr()));
        assert_eq!(unknown.to_string(), "Z");
        assert!(unknown != Side::Buy);
        assert_eq!(Code::into_owned(unknown), Code::<Side>::from_fix("Z").unwrap());
    }
}
