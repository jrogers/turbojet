//! FAST's transfer encoding (section 10 of the specification): stop-bit encoded integers,
//! strings and presence maps, and length-prefixed byte vectors.

use crate::{Code, Error};

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
        let value = match (nullable, value) {
            (true, 0) => return Ok(None),
            (true, value) if value > 0 => value - 1,
            (_, value) => value,
        };
        i64::try_from(value).map(Some).map_err(|_| Error::spec(Code::D2))
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
    fn an_entity_without_a_stop_bit_is_truncated() {
        assert_eq!(unsigned(&[0x01, 0x02], false), Err(Error::truncated()));
        assert_eq!(Reader::new(&[0x41]).ascii(false).err(), Some(Error::truncated()));
    }
}
