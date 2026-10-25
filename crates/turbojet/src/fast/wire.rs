//! FAST's transfer encoding (section 10 of the specification): stop-bit encoded integers,
//! strings and presence maps, and length-prefixed byte vectors.

use crate::fast::{Code, Error};

/// The most bytes a stop-bit integer takes: 64 bits, plus one for the sign or for a nullable
/// value's increment, in seven-bit groups. Longer is an overlong encoding or out of range.
const MAX_INTEGER_BYTES: usize = 10;

/// The stop bit, set on an entity's last byte.
const STOP: u8 = 0x80;

/// A cursor over the bytes of one message.
#[derive(Debug)]
pub(crate) struct Reader<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self { input, position: 0 }
    }

    /// The bytes not read yet.
    pub(crate) fn rest(&self) -> &'a [u8] {
        &self.input[self.position..]
    }

    /// The bytes of one stop-bit encoded entity, stop bit included, at most `limit` of them.
    fn entity(&mut self, limit: usize) -> Result<&'a [u8], Error> {
        let rest = self.rest();
        let Some(end) = rest.iter().position(|b| b & STOP != 0) else {
            return Err(Error::truncated());
        };
        if end >= limit {
            return Err(Error::spec(Code::D2));
        }
        self.position += end + 1;
        Ok(&rest[..=end])
    }

    /// An unsigned integer, or `None` for a nullable integer's NULL. Range checks are the caller's:
    /// a nullable `uInt64` can need 65 bits before its increment is taken off.
    pub(crate) fn unsigned(&mut self, nullable: bool) -> Result<Option<u64>, Error> {
        let bytes = self.entity(MAX_INTEGER_BYTES)?;
        if bytes.len() > 1 && bytes[0] == 0 {
            return Err(Error::spec(Code::R6));
        }
        let value = bytes.iter().fold(0u128, |value, b| (value << 7) | u128::from(b & !STOP));
        let value = if nullable {
            match value.checked_sub(1) {
                Some(value) => value,
                None => return Ok(None),
            }
        } else {
            value
        };
        u64::try_from(value).map(Some).map_err(|_| Error::spec(Code::D2))
    }

    /// A signed integer, or `None` for a nullable integer's NULL.
    pub(crate) fn signed(&mut self, nullable: bool) -> Result<Option<i64>, Error> {
        let value = self.signed_wide(nullable)?;
        value.map(i64::try_from).transpose().map_err(|_| Error::spec(Code::D2))
    }

    /// A signed integer of up to 70 bits, or `None` for NULL: a delta can need more bits than
    /// its field's type (section 6.3.7.1's note).
    pub(crate) fn signed_wide(&mut self, nullable: bool) -> Result<Option<i128>, Error> {
        let bytes = self.entity(MAX_INTEGER_BYTES)?;
        // Overlong if the first byte only repeats the sign the next one already carries.
        if let [first, second, ..] = bytes {
            let sign_follows = second & 0x40 != 0;
            if (*first == 0 && !sign_follows) || (*first == 0x7f && sign_follows) {
                return Err(Error::spec(Code::R6));
            }
        }
        let negative = bytes[0] & 0x40 != 0;
        let start: i128 = if negative { -1 } else { 0 };
        let value = bytes.iter().fold(start, |value, b| (value << 7) | i128::from(b & !STOP));
        Ok(match (nullable, value) {
            (true, 0) => None,
            (true, value) if value > 0 => Some(value - 1),
            (_, value) => Some(value),
        })
    }

    /// An ASCII string's characters, or `None` for a nullable string's NULL, with the zero
    /// preambles of section 10.6.3 taken off. Each byte keeps its stop bit, to be masked off.
    pub(crate) fn ascii(&mut self, nullable: bool) -> Result<Option<&'a [u8]>, Error> {
        let bytes = self.entity(usize::MAX)?;
        let mut chars = bytes;
        if nullable && chars[0] & !STOP == 0 {
            if chars.len() == 1 {
                return Ok(None);
            }
            if chars[1] & !STOP != 0 {
                return Err(Error::spec(Code::R9));
            }
            chars = &chars[1..];
        }
        if chars[0] & !STOP == 0 {
            if chars.len() > 1 && chars[1] & !STOP != 0 {
                return Err(Error::spec(Code::R9));
            }
            chars = &chars[1..];
        }
        Ok(Some(chars))
    }

    /// A byte vector, or `None` for a nullable one's NULL.
    pub(crate) fn bytes(&mut self, nullable: bool) -> Result<Option<&'a [u8]>, Error> {
        let Some(length) = self.unsigned(nullable)? else { return Ok(None) };
        let length = u32::try_from(length).map_err(|_| Error::spec(Code::D2))?;
        let length = usize::try_from(length).map_err(|_| Error::truncated())?;
        let rest = self.rest();
        if length > rest.len() {
            return Err(Error::truncated());
        }
        self.position += length;
        Ok(Some(&rest[..length]))
    }

    /// A presence map: its bytes, stop bit included, each holding seven bits.
    pub(crate) fn presence_map(&mut self) -> Result<PresenceMap<'a>, Error> {
        let bytes = self.entity(usize::MAX)?;
        if bytes.len() > 1 && bytes[bytes.len() - 1] & !STOP == 0 {
            return Err(Error::spec(Code::R7));
        }
        Ok(PresenceMap { bytes, next: 0 })
    }
}

/// The ASCII characters of a string [`Reader::ascii`] returned, stop bit masked off, appended to
/// `out`.
pub(crate) fn push_ascii(chars: &[u8], out: &mut Vec<u8>) {
    out.extend(chars.iter().map(|b| b & !STOP));
}

/// A segment's presence map, read a bit at a time. Past its last byte every bit is zero.
#[derive(Debug)]
pub(crate) struct PresenceMap<'a> {
    bytes: &'a [u8],
    /// The index of the next bit.
    next: usize,
}

impl PresenceMap<'_> {
    /// A map of no bits, all of them zero.
    pub(crate) fn empty() -> Self {
        Self { bytes: &[], next: 0 }
    }

    pub(crate) fn next_bit(&mut self) -> bool {
        let (byte, bit) = (self.next / 7, self.next % 7);
        self.next += 1;
        self.bytes.get(byte).is_some_and(|b| b & (0x40 >> bit) != 0)
    }

    /// The number of bits the map carries.
    pub(crate) fn len_bits(&self) -> usize {
        self.bytes.len() * 7
    }
}

/// Appends an unsigned integer, nullable or not: a nullable one is one more, leaving 0 for NULL
/// (section 10.6.1). `None` is NULL.
pub(crate) fn write_unsigned(out: &mut Vec<u8>, value: Option<u128>, nullable: bool) {
    debug_assert!(nullable || value.is_some(), "only a nullable integer can be NULL");
    let value = match value {
        None => 0,
        Some(value) => value + u128::from(nullable),
    };
    let start = out.len();
    let mut rest = value;
    loop {
        out.push(u8::try_from(rest & 0x7f).expect("seven bits"));
        rest >>= 7;
        if rest == 0 {
            break;
        }
    }
    finish(out, start);
}

/// Appends a signed integer in as few seven-bit groups as keep its sign (section 10.6.1.1). A
/// nullable one is one more if it isn't negative; `None` is NULL.
pub(crate) fn write_signed(out: &mut Vec<u8>, value: Option<i128>, nullable: bool) {
    debug_assert!(nullable || value.is_some(), "only a nullable integer can be NULL");
    let value = match value {
        None => 0,
        Some(value) if nullable && value >= 0 => value + 1,
        Some(value) => value,
    };
    let start = out.len();
    let mut rest = value;
    loop {
        let group = u8::try_from(rest & 0x7f).expect("seven bits");
        out.push(group);
        rest >>= 7;
        let sign_kept = if rest == 0 { group & 0x40 == 0 } else { rest == -1 && group & 0x40 != 0 };
        if sign_kept {
            break;
        }
    }
    finish(out, start);
}

/// Puts the groups pushed since `start`, least significant first, in order, with the stop bit.
fn finish(out: &mut [u8], start: usize) {
    out[start..].reverse();
    *out.last_mut().expect("at least one group") |= STOP;
}

/// Appends an ASCII string, with the zero preambles of section 10.6.3; `None` is NULL.
pub(crate) fn write_ascii(out: &mut Vec<u8>, value: Option<&[u8]>, nullable: bool) {
    debug_assert!(nullable || value.is_some(), "only a nullable string can be NULL");
    debug_assert!(value.is_none_or(|chars| chars.iter().all(u8::is_ascii)), "ASCII characters only");
    let Some(chars) = value else {
        out.push(STOP);
        return;
    };
    if nullable && (chars.is_empty() || chars[0] == 0) {
        out.push(0);
    }
    if chars.is_empty() {
        out.push(STOP);
        return;
    }
    if chars[0] == 0 {
        out.push(0);
    }
    out.extend_from_slice(chars);
    *out.last_mut().expect("not empty") |= STOP;
}

/// Appends a byte vector: its length, then its bytes; `None` is NULL.
pub(crate) fn write_bytes(out: &mut Vec<u8>, value: Option<&[u8]>, nullable: bool) {
    let length = value.map(|bytes| u128::try_from(bytes.len()).expect("a length fits"));
    write_unsigned(out, length, nullable);
    out.extend_from_slice(value.unwrap_or_default());
}

/// Appends a presence map of `bits`, without the trailing zero bytes that would make it overlong.
pub(crate) fn write_presence_map(out: &mut Vec<u8>, bits: &[bool]) {
    let start = out.len();
    for chunk in bits.chunks(7) {
        out.push(chunk.iter().enumerate().fold(0, |byte, (i, &bit)| byte | (u8::from(bit) << (6 - i))));
    }
    while out.len() > start + 1 && out.last() == Some(&0) {
        out.pop();
    }
    if out.len() == start {
        out.push(0);
    }
    *out.last_mut().expect("at least one byte") |= STOP;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unsigned(bytes: &[u8], nullable: bool) -> Result<Option<u64>, Error> {
        let mut reader = Reader::new(bytes);
        let value = reader.unsigned(nullable)?;
        assert!(reader.rest().is_empty(), "the whole entity is read");
        Ok(value)
    }

    fn signed(bytes: &[u8], nullable: bool) -> Result<Option<i64>, Error> {
        let mut reader = Reader::new(bytes);
        let value = reader.signed(nullable)?;
        assert!(reader.rest().is_empty(), "the whole entity is read");
        Ok(value)
    }

    fn ascii(bytes: &[u8], nullable: bool) -> Result<Option<Vec<u8>>, Error> {
        let mut reader = Reader::new(bytes);
        let chars = reader.ascii(nullable)?;
        assert!(reader.rest().is_empty(), "the whole entity is read");
        Ok(chars.map(|chars| {
            let mut out = Vec::new();
            push_ascii(chars, &mut out);
            out
        }))
    }

    #[test]
    fn signed_integers_as_in_appendix_3_1_1() {
        assert_eq!(signed(&[0x39, 0x45, 0xa4], true), Ok(Some(942_755)));
        assert_eq!(signed(&[0x39, 0x45, 0xa3], false), Ok(Some(942_755)));
        assert_eq!(signed(&[0x46, 0x3a, 0xdd], true), Ok(Some(-942_755)));
        assert_eq!(signed(&[0x7c, 0x1b, 0x1b, 0x9d], false), Ok(Some(-7_942_755)));
        // The sign needs a byte of its own.
        assert_eq!(signed(&[0x00, 0x40, 0x81], false), Ok(Some(8193)));
        // The specification's hex column says 0x73, a typo: its binary column, 01111111, is 0x7f,
        // and 0x73 0x3f 0xff would be -204801.
        assert_eq!(signed(&[0x7f, 0x3f, 0xff], false), Ok(Some(-8193)));
    }

    #[test]
    fn unsigned_integers_as_in_appendix_3_1_2() {
        assert_eq!(unsigned(&[0x80], true), Ok(None));
        assert_eq!(unsigned(&[0x81], true), Ok(Some(0)));
        assert_eq!(unsigned(&[0x82], true), Ok(Some(1)));
        assert_eq!(unsigned(&[0x39, 0x45, 0xa4], true), Ok(Some(942_755)));
        assert_eq!(unsigned(&[0x80], false), Ok(Some(0)));
        assert_eq!(unsigned(&[0x81], false), Ok(Some(1)));
        assert_eq!(unsigned(&[0x39, 0x45, 0xa3], false), Ok(Some(942_755)));
    }

    #[test]
    fn integers_at_their_limits() {
        // The nullable form of u64::MAX needs 65 bits (section 10.6.1's note, for 64 bits).
        let max_nullable = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        assert_eq!(unsigned(&max_nullable, true), Ok(Some(u64::MAX)));
        assert_eq!(unsigned(&max_nullable, false), Err(Error::spec(Code::D2)));
        let min = [0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        assert_eq!(signed(&min, false), Ok(Some(i64::MIN)));
        let too_long = [0x01; 10].iter().copied().chain([0x80]).collect::<Vec<_>>();
        assert_eq!(unsigned(&too_long, false), Err(Error::spec(Code::D2)));
    }

    #[test]
    fn overlong_integers_are_reported() {
        assert_eq!(unsigned(&[0x00, 0x81], false), Err(Error::spec(Code::R6)));
        // 1 with a redundant positive sign byte, and -1 with a redundant negative one.
        assert_eq!(signed(&[0x00, 0x81], false), Err(Error::spec(Code::R6)));
        assert_eq!(signed(&[0x7f, 0xff], false), Err(Error::spec(Code::R6)));
    }

    #[test]
    fn strings_as_in_appendix_3_1_3_and_section_10_6_3() {
        assert_eq!(ascii(&[0x80], true), Ok(None));
        assert_eq!(ascii(&[0x41, 0x42, 0xc3], true), Ok(Some(b"ABC".to_vec())));
        assert_eq!(ascii(&[0x00, 0x80], true), Ok(Some(Vec::new())));
        assert_eq!(ascii(&[0x41, 0x42, 0xc3], false), Ok(Some(b"ABC".to_vec())));
        assert_eq!(ascii(&[0x80], false), Ok(Some(Vec::new())));
        // The zero-preamble table.
        assert_eq!(ascii(&[0x00, 0x80], false), Ok(Some(vec![0])));
        assert_eq!(ascii(&[0x00, 0xc1], false), Err(Error::spec(Code::R9)));
        assert_eq!(ascii(&[0x00, 0x00, 0x80], true), Ok(Some(vec![0])));
        assert_eq!(ascii(&[0x00, 0xc1], true), Err(Error::spec(Code::R9)));
        assert_eq!(ascii(&[0x00, 0x00, 0xc1], true), Err(Error::spec(Code::R9)));
    }

    #[test]
    fn byte_vectors_as_in_appendix_3_1_4() {
        let bytes = |input: &[u8], nullable| Reader::new(input).bytes(nullable).map(|b| b.map(<[u8]>::to_vec));
        assert_eq!(bytes(&[0x80], true), Ok(None));
        assert_eq!(bytes(&[0x84, 0x41, 0x42, 0x43], true), Ok(Some(b"ABC".to_vec())));
        assert_eq!(bytes(&[0x81], true), Ok(Some(Vec::new())));
        assert_eq!(bytes(&[0x83, 0x41, 0x42, 0x43], false), Ok(Some(b"ABC".to_vec())));
        assert_eq!(bytes(&[0x80], false), Ok(Some(Vec::new())));
        assert_eq!(bytes(&[0x84, 0x41], false), Err(Error::truncated()));
    }

    #[test]
    fn presence_maps_read_bits_in_order_then_zeros() {
        let mut reader = Reader::new(&[0x50, 0x81]);
        let mut map = reader.presence_map().expect("well formed");
        let bits: Vec<bool> = (0..16).map(|_| map.next_bit()).collect();
        let mut expected = [false; 16];
        expected[0] = true; // 0x50 = 1010000: bits 0 and 2.
        expected[2] = true;
        expected[13] = true; // 0x81: the last bit of the second byte.
        assert_eq!(bits, expected);
        assert_eq!(map.len_bits(), 14);
        assert_eq!(Reader::new(&[0x40, 0x80]).presence_map().err(), Some(Error::spec(Code::R7)));
    }

    #[test]
    fn integers_written_are_read_back_in_the_fewest_bytes() {
        // The specification's examples, written.
        let mut out = Vec::new();
        write_signed(&mut out, Some(8193), false);
        assert_eq!(out, [0x00, 0x40, 0x81]);
        out.clear();
        write_signed(&mut out, Some(-8193), false);
        assert_eq!(out, [0x7f, 0x3f, 0xff]);
        out.clear();
        write_unsigned(&mut out, Some(942_755), true);
        assert_eq!(out, [0x39, 0x45, 0xa4]);
        for nullable in [false, true] {
            for value in
                [0, 1, 63, 64, 127, 128, 8191, 8192, -1, -64, -65, -8193, i128::from(i64::MIN), i128::from(i64::MAX)]
            {
                let mut out = Vec::new();
                write_signed(&mut out, Some(value), nullable);
                assert_eq!(Reader::new(&out).signed_wide(nullable), Ok(Some(value)), "{value} {nullable}");
            }
            for value in [0, 1, 127, 128, u128::from(u32::MAX), u128::from(u64::MAX)] {
                let mut out = Vec::new();
                write_unsigned(&mut out, Some(value), nullable);
                let read = Reader::new(&out).unsigned(nullable).map(|v| v.map(u128::from));
                assert_eq!(read, Ok(Some(value)), "{value} {nullable}");
            }
        }
    }

    #[test]
    fn strings_written_are_read_back() {
        for nullable in [false, true] {
            for value in [&b""[..], b"A", b"ABC", b"\0", b"\0\0", b"\0A"] {
                let mut out = Vec::new();
                write_ascii(&mut out, Some(value), nullable);
                assert_eq!(ascii(&out, nullable), Ok(Some(value.to_vec())), "{value:?} {nullable}");
            }
        }
        let mut out = Vec::new();
        write_ascii(&mut out, None, true);
        assert_eq!(ascii(&out, true), Ok(None));
    }

    #[test]
    fn presence_maps_written_are_read_back_without_trailing_zeros() {
        let mut out = Vec::new();
        write_presence_map(&mut out, &[true, false, true, false, false, false, false, false, false]);
        assert_eq!(out, [0xd0]);
        out.clear();
        write_presence_map(&mut out, &[]);
        assert_eq!(out, [0x80]);
        out.clear();
        let bits = [false, false, false, false, false, false, false, true];
        write_presence_map(&mut out, &bits);
        let mut map = Reader::new(&out).presence_map().expect("well formed");
        assert_eq!((0..8).map(|_| map.next_bit()).collect::<Vec<_>>(), bits);
    }

    #[test]
    fn an_entity_without_a_stop_bit_is_truncated() {
        assert_eq!(unsigned(&[0x01, 0x02], false), Err(Error::truncated()));
        assert_eq!(Reader::new(&[0x41]).ascii(false).err(), Some(Error::truncated()));
    }
}
