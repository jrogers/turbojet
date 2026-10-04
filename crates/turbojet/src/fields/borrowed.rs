//! Borrowed field values, for the `…Ref` types `fix_message!` generates: a message parsed into
//! them keeps its strings in the raw [`Message`](crate::Message) rather than copying them.

use std::fmt;
use std::marker::PhantomData;

use super::{
    Code, CompactString, Decimal, FixEnum, MonthYear, MsgType, NaiveDate, Secret, TzTimeOnly, TzTimestamp, UtcTimeOnly,
    UtcTimestamp, ValueError,
};

/// A field type's borrowed form, for the `…Ref` types `fix_message!` generates: what a field of
/// this type holds when parsed without allocating, and how to make the owned value from it.
pub trait FieldRef<'a>: Sized {
    /// The borrowed form: `&'a str` for `String` (and the code for `MsgType`), [`SecretRef`] and
    /// [`CodeRef`] for `Secret` and `Code`, the type itself for `Copy` scalars.
    type Ref: Copy + fmt::Debug + PartialEq + 'a;

    /// Parses a value without its tag or delimiter; fails as [`FromFix::from_fix`](super::FromFix::from_fix)
    /// would.
    ///
    /// Must be a deterministic, pure function of `s`: a [`List`] parses each value once when it's
    /// checked and again as it's iterated, and expects the same result.
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

impl<'a> FieldRef<'a> for CompactString {
    type Ref = &'a str;

    fn parse_ref(s: &'a str) -> Result<&'a str, ValueError> {
        Ok(s)
    }

    fn into_owned(value: &'a str) -> Self {
        Self::from(value)
    }
}

/// The borrowed form is the code; [`MsgType::from_code`] gives the type.
impl<'a> FieldRef<'a> for MsgType {
    type Ref = &'a str;

    fn parse_ref(s: &'a str) -> Result<&'a str, ValueError> {
        Ok(s)
    }

    fn into_owned(code: &'a str) -> Self {
        Self::from_code(code)
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

/// A multi-value field (MultipleCharValue, MultipleStringValue) parsed without allocating: its
/// values were checked when the message was parsed, and are read again as they're iterated.
///
/// ```
/// use turbojet::fields::{EncryptMethod, FieldRef};
///
/// let methods = Vec::<EncryptMethod>::parse_ref("0 1").unwrap();
/// assert_eq!(methods.len(), 2);
/// assert_eq!(methods.iter().collect::<Vec<_>>(), [EncryptMethod::None, EncryptMethod::PKCS]);
/// ```
pub struct List<'a, T> {
    raw: &'a str,
    _value: PhantomData<fn() -> T>,
}

impl<'a, T: FieldRef<'a>> List<'a, T> {
    /// The values, in order.
    pub fn iter(&self) -> ListIter<'a, T> {
        ListIter { values: self.raw.split(' '), _value: PhantomData }
    }

    /// How many values there are.
    pub fn len(&self) -> usize {
        self.raw.split(' ').count()
    }

    /// Whether there are none: never, for a parsed list, as an empty field is a format error.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// The values as they were sent, separated by spaces.
    pub fn as_str(&self) -> &'a str {
        self.raw
    }
}

impl<T> Clone for List<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for List<'_, T> {}

impl<'a, T: FieldRef<'a>> fmt::Debug for List<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Compares values, not their text.
impl<'a, T: FieldRef<'a>> PartialEq for List<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a, T: FieldRef<'a>> IntoIterator for List<'a, T> {
    type Item = T::Ref;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> ListIter<'a, T> {
        self.iter()
    }
}

impl<'a, T: FieldRef<'a>> IntoIterator for &List<'a, T> {
    type Item = T::Ref;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> ListIter<'a, T> {
        self.iter()
    }
}

/// The values of a [`List`], in order.
pub struct ListIter<'a, T> {
    values: std::str::Split<'a, char>,
    _value: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for ListIter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListIter").finish_non_exhaustive()
    }
}

impl<'a, T: FieldRef<'a>> Iterator for ListIter<'a, T> {
    type Item = T::Ref;

    fn next(&mut self) -> Option<T::Ref> {
        self.values.next().map(|value| T::parse_ref(value).expect("checked when parsed"))
    }
}

impl<'a, T: FieldRef<'a>> std::iter::FusedIterator for ListIter<'a, T> {}

/// Split as [`FromFix`](super::FromFix) splits `Vec<T>`: an empty value, or a doubled space, is a
/// format error.
impl<'a, T: FieldRef<'a> + 'a> FieldRef<'a> for Vec<T> {
    type Ref = List<'a, T>;

    fn parse_ref(s: &'a str) -> Result<List<'a, T>, ValueError> {
        for value in s.split(' ') {
            if value.is_empty() {
                return Err(ValueError::Format);
            }
            T::parse_ref(value)?;
        }
        Ok(List { raw: s, _value: PhantomData })
    }

    fn into_owned(list: List<'a, T>) -> Self {
        list.iter().map(T::into_owned).collect()
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
    fn msg_types_borrow_their_code() {
        for input in ["D", "U1"] {
            let code = MsgType::parse_ref(input).unwrap();
            assert!(std::ptr::eq(code.as_ptr(), input.as_ptr()));
            assert_eq!(MsgType::into_owned(code), MsgType::from_fix(input).unwrap(), "{input:?}");
        }
        assert_eq!(MsgType::into_owned("D"), MsgType::NewOrderSingle);
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

    /// Checks `Vec<T>::parse_ref` against `Vec<T>::from_fix`, element by element.
    fn lists_parse_as_owned<T>(values: &[&str])
    where
        T: for<'a> FieldRef<'a> + FromFix + PartialEq + fmt::Debug,
    {
        for &s in values {
            let list = Vec::<T>::parse_ref(s);
            let owned = Vec::<T>::from_fix(s);
            assert_eq!(list.map(|_| ()), owned.as_ref().map(|_| ()).map_err(|e| *e), "{s:?}");
            if let (Ok(list), Ok(owned)) = (list, owned) {
                assert_eq!(list.len(), owned.len(), "{s:?}");
                assert!(!list.is_empty(), "{s:?}");
                assert_eq!(list.iter().map(T::into_owned).collect::<Vec<_>>(), owned, "{s:?}");
                assert_eq!(Vec::<T>::into_owned(list), owned, "{s:?}");
            }
        }
    }

    #[test]
    fn lists_are_checked_when_parsed_and_read_as_iterated() {
        let list = Vec::<Side>::parse_ref("1 2 1").unwrap();
        assert_eq!(list.iter().collect::<Vec<_>>(), [Side::Buy, Side::Sell, Side::Buy]);
        assert_eq!((list.len(), list.is_empty(), list.as_str()), (3, false, "1 2 1"));
        assert_eq!(Vec::into_owned(list), Vec::<Side>::from_fix("1 2 1").unwrap());

        let edges = ["1", "1 2", "", " ", " 1", "1 ", "1  2", "1 3", "3 x", "1 2 "];
        lists_parse_as_owned::<Side>(&edges);
        lists_parse_as_owned::<String>(&edges);
        lists_parse_as_owned::<Code<Side>>(&["1 Z", "Z  1", ""]);
        lists_parse_as_owned::<char>(&["A B", "A BC", "AB A"]);
        assert_eq!(Vec::<Side>::parse_ref("1 3"), Err(ValueError::Incorrect));
        assert_eq!(Vec::<Side>::parse_ref("1  2"), Err(ValueError::Format));
    }

    #[test]
    fn lists_of_strings_borrow_the_input() {
        let input = "G 1";
        let list = Vec::<String>::parse_ref(input).unwrap();
        let values: Vec<&str> = list.iter().collect();
        assert_eq!(values, ["G", "1"]);
        assert!(std::ptr::eq(values[0].as_ptr(), input.as_ptr()));
        assert!(std::ptr::eq(list.as_str(), input));
    }

    #[test]
    fn lists_show_and_compare_their_values() {
        let list = Vec::<Code<Side>>::parse_ref("1 Z").unwrap();
        assert_eq!(format!("{list:?}"), r#"[Known(Buy), Unknown("Z")]"#);
        assert_eq!(format!("{:?}", Vec::<Secret>::parse_ref("a b").unwrap()), "[***, ***]");
        assert_eq!(list, Vec::<Code<Side>>::parse_ref("1 Z").unwrap());
        assert_ne!(list, Vec::<Code<Side>>::parse_ref("1 Y").unwrap());
        assert_ne!(list, Vec::<Code<Side>>::parse_ref("1").unwrap());
        assert_ne!(list, Vec::<Code<Side>>::parse_ref("1 Z 1").unwrap());
        let copy = list;
        assert_eq!(copy, list);
    }

    /// A list iterates by value and by reference, as `iter` does.
    #[test]
    fn lists_iterate_in_for_loops() {
        let list = Vec::<String>::parse_ref("G 1").unwrap();
        let mut by_ref = Vec::new();
        for value in &list {
            by_ref.push(value);
        }
        let by_value: Vec<&str> = list.into_iter().collect();
        assert_eq!(by_ref, ["G", "1"]);
        assert_eq!(by_value, by_ref);
    }
}
