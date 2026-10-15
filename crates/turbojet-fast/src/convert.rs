//! Initial values, converted from a template's `value` attributes (sections 6.3.2 and 8.1).
//! Conversion errors are static errors here [ERR S3].

use crate::templates::{IntegerType, Type, Value};
use crate::{Code, Error};

/// A decimal's exponent range (section 6.2.2).
pub(crate) const EXPONENT_RANGE: std::ops::RangeInclusive<i32> = -63..=63;

/// `value` converted to `ty`.
pub(crate) fn initial(value: &str, ty: Type) -> Result<Value, Error> {
    let invalid = || Error::spec(Code::S3).with_detail(format!("initial value {value:?}"));
    match ty {
        Type::Integer(integer) => integer_value(value.trim(), integer).map(Value::Integer).ok_or_else(invalid),
        Type::Decimal => {
            decimal(value.trim()).map(|(exponent, mantissa)| Value::Decimal { exponent, mantissa }).ok_or_else(invalid)
        }
        Type::Ascii if !value.is_ascii() => Err(invalid()),
        Type::Ascii | Type::Unicode => Ok(Value::Bytes(value.as_bytes().into())),
        Type::Bytes => hex(value).map(|bytes| Value::Bytes(bytes.into())).ok_or_else(invalid),
    }
}

/// Whether a decimal exponent's initial value is in range.
pub(crate) fn exponent_in_range(value: &Value) -> bool {
    matches!(value, Value::Integer(exponent) if i32::try_from(*exponent).is_ok_and(|e| EXPONENT_RANGE.contains(&e)))
}

/// Digits with a leading `-` if the type is signed, within the type's range.
pub(crate) fn integer_value(s: &str, integer: IntegerType) -> Option<i128> {
    let digits = if integer.is_signed() { s.strip_prefix('-').unwrap_or(s) } else { s };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let value: i128 = s.parse().ok()?;
    let (min, max) = integer.range();
    (min..=max).contains(&value).then_some(value)
}

/// An integer part, a fractional part or both, as an exponent and mantissa, normalised so the
/// mantissa has no trailing zero (zero is `0 * 10^0`).
fn decimal(s: &str) -> Option<(i32, i64)> {
    let (mut exponent, mut mantissa) = scaled(s)?;
    if mantissa == 0 {
        return Some((0, 0));
    }
    while mantissa % 10 == 0 {
        mantissa /= 10;
        exponent += 1;
    }
    let mantissa = i64::try_from(mantissa).ok()?;
    EXPONENT_RANGE.contains(&exponent).then_some((exponent, mantissa))
}

/// A FIX decimal as written, its exponent the number of digits after the point: 9427.55 is
/// `942755 * 10^-2`. For encoding, where the scale a value was given with is kept.
pub(crate) fn decimal_as_written(s: &str) -> Option<(i32, i64)> {
    let (exponent, mantissa) = scaled(s)?;
    let mantissa = i64::try_from(mantissa).ok()?;
    EXPONENT_RANGE.contains(&exponent).then_some((exponent, mantissa))
}

/// An optional `-`, digits, and at most one `.`, as an exponent and mantissa. Trailing zeros
/// stay in the mantissa as far as it fits an `i64`, and go into the exponent past that: a decoded
/// `1 * 10^63` is written with 63 zeros.
fn scaled(s: &str) -> Option<(i32, i128)> {
    let unsigned = s.strip_prefix('-').unwrap_or(s);
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let digits = || whole.bytes().chain(fraction.bytes());
    if digits().next().is_none() || !digits().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let count = whole.len() + fraction.len();
    let zeros = digits().rev().take_while(|&b| b == b'0').count();
    let mut mantissa: i128 = 0;
    for b in digits().take(count - zeros) {
        mantissa = mantissa.checked_mul(10)?.checked_add(i128::from(b - b'0'))?;
    }
    let mut exponent = -i32::try_from(fraction.len()).ok()? + i32::try_from(zeros).ok()?;
    for _ in 0..zeros {
        match mantissa.checked_mul(10).filter(|m| i64::try_from(*m).is_ok()) {
            Some(longer) => {
                mantissa = longer;
                exponent -= 1;
            }
            None => break,
        }
    }
    if unsigned.len() < s.len() {
        mantissa = -mantissa;
    }
    Some((exponent, mantissa))
}

/// Pairs of hexadecimal digits, with any whitespace between them ignored (section 8.1.3).
pub(crate) fn hex(s: &str) -> Option<Vec<u8>> {
    let digits: Vec<u8> = s.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if !digits.len().is_multiple_of(2) {
        return None;
    }
    digits
        .chunks(2)
        .map(|pair| {
            let digit = |b: u8| char::from(b).to_digit(16);
            let byte = digit(pair[0])? * 16 + digit(pair[1])?;
            u8::try_from(byte).ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimals_are_normalised() {
        // Section 6.3.2's example, and Appendix 3.2.5's initial value 12000.
        assert_eq!(decimal("100"), Some((2, 1)));
        assert_eq!(decimal("12000"), Some((3, 12)));
        assert_eq!(decimal("9427.55"), Some((-2, 942_755)));
        assert_eq!(decimal("-0.10"), Some((-1, -1)));
        assert_eq!(decimal(".1"), Some((-1, 1)));
        assert_eq!(decimal("1."), Some((0, 1)));
        assert_eq!(decimal("-0.0"), Some((0, 0)));
        for bad in ["", ".", "-", "1e3", "1.2.3", "+1", "99999999999999999999"] {
            assert_eq!(decimal(bad), None, "{bad}");
        }
        assert_eq!(decimal_as_written("9427.50"), Some((-2, 942_750)));
        assert_eq!(decimal_as_written(&format!("1{}", "0".repeat(63))), Some((45, 1_000_000_000_000_000_000)));
    }

    #[test]
    fn integers_respect_their_type() {
        assert_eq!(integer_value("4294967295", IntegerType::UInt32), Some(4_294_967_295));
        assert_eq!(integer_value("4294967296", IntegerType::UInt32), None);
        assert_eq!(integer_value("-1", IntegerType::UInt32), None);
        assert_eq!(integer_value("-2147483648", IntegerType::Int32), Some(-2_147_483_648));
        assert_eq!(initial(" 7 ", Type::Integer(IntegerType::Int64)), Ok(Value::Integer(7)));
    }

    #[test]
    fn byte_vectors_are_hex_and_strings_are_checked() {
        assert_eq!(hex("41 42\n43"), Some(b"ABC".to_vec()));
        assert_eq!(hex("4"), None);
        assert_eq!(hex("zz"), None);
        assert_eq!(initial("é", Type::Ascii).map_err(|e| e.code()), Err(Some(Code::S3)));
        assert_eq!(initial("é", Type::Unicode), Ok(Value::Bytes("é".as_bytes().into())));
    }
}
