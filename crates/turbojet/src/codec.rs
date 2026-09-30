//! Tag=value wire encoding: framing via BodyLength(9) and validation via CheckSum(10).

use crate::message::{DataFields, FieldError, FieldErrorKind, Message, SOH, tags};

/// Upper bound on BodyLength(9), to stop a bad length from buffering unbounded input.
const MAX_BODY_LENGTH: usize = 64 * 1024;
/// Longest BeginString or BodyLength field we wait for before declaring the input garbled.
const MAX_HEADER_FIELD_LEN: usize = 32;
/// `10=NNN<SOH>`
const TRAILER_LEN: usize = 7;

/// The result of [`decode`].
#[derive(Debug, PartialEq)]
pub enum Decoded {
    /// A complete, valid message and the number of bytes it occupied.
    Message(Message, usize),
    /// More bytes are needed.
    Incomplete,
    /// The input at the front of the buffer can't be trusted as a message: its framing, checksum
    /// or header is invalid. Per the FIX spec, garbled messages are ignored: the caller should drop
    /// `skip` bytes and try again. A malformed body field doesn't garble a message: it decodes,
    /// with the defect recorded for the session to reject.
    Garbled {
        /// Bytes to drop from the front of the buffer.
        skip: usize,
        /// What was wrong, for logging.
        reason: String,
    },
}

/// Attempts to decode one message from the front of `buf`, knowing the standard data fields
/// ([`DataFields::standard`]).
pub fn decode(buf: &[u8]) -> Decoded {
    thread_local! {
        static STANDARD: DataFields = DataFields::standard();
    }
    STANDARD.with(|data| decode_with(buf, data))
}

/// [`decode`], knowing the data fields in `data`: each value is as long as its Length field says,
/// and may contain SOH.
pub fn decode_with(buf: &[u8], data: &DataFields) -> Decoded {
    let total = match frame(buf) {
        Ok(total) => total,
        Err(d) => return d,
    };
    match Message::from_frame(&buf[..total], data) {
        Ok(msg) => Decoded::Message(msg, total),
        Err(reason) => garbled(buf, reason),
    }
}

/// The length of the message at the front of `buf`, if its framing is valid: BeginString,
/// BodyLength and MsgType first, and the CheckSum where BodyLength puts it. Otherwise
/// [`Decoded::Incomplete`] or [`Decoded::Garbled`]. The body isn't parsed.
pub(crate) fn frame(buf: &[u8]) -> Result<usize, Decoded> {
    if buf.is_empty() {
        return Err(Decoded::Incomplete);
    }
    let (_, len_start) = header_field(buf, 0, b"8=")?;
    let (len_soh, body_start) = header_field(buf, len_start, b"9=")?;
    let body_len = match parse_digits(&buf[len_start + 2..len_soh]) {
        Some(n) if n <= MAX_BODY_LENGTH => n,
        _ => return Err(garbled(buf, "invalid BodyLength(9)".into())),
    };

    let trailer_start = body_start + body_len;
    let total = trailer_start + TRAILER_LEN;
    if buf.len() < total {
        return Err(Decoded::Incomplete);
    }
    let trailer = &buf[trailer_start..total];
    if &trailer[..3] != b"10=" || trailer[6] != SOH {
        return Err(garbled(buf, "BodyLength(9) does not match message".into()));
    }
    let Some(received) = parse_digits(&trailer[3..6]) else {
        return Err(garbled(buf, "invalid CheckSum(10)".into()));
    };
    let computed = checksum(&buf[..trailer_start]);
    if received != computed as usize {
        return Err(garbled(buf, format!("CheckSum mismatch: received {received}, computed {computed}")));
    }

    // BeginString, BodyLength and MsgType must be the first three fields.
    if !buf[body_start..].starts_with(b"35=") {
        return Err(garbled(buf, "MsgType(35) is not the third field".into()));
    }
    Ok(total)
}

/// Encodes a message, computing BodyLength(9) and CheckSum(10). BeginString(8) is taken from the
/// message, and a message without one is refused as missing it; any BodyLength or CheckSum fields
/// already present are ignored.
pub fn encode(msg: &Message) -> Result<Vec<u8>, FieldError> {
    let mut out = Vec::new();
    encode_into(msg, &mut out)?;
    Ok(out)
}

/// [`encode`], appending to `out` so a buffer can be reused across messages. On error, `out` is
/// unchanged.
pub fn encode_into(msg: &Message, out: &mut Vec<u8>) -> Result<(), FieldError> {
    let begin_string =
        msg.get(tags::BEGIN_STRING).ok_or(FieldError { tag: tags::BEGIN_STRING, kind: FieldErrorKind::Missing })?;
    let in_body = |tag| !matches!(tag, tags::BEGIN_STRING | tags::BODY_LENGTH | tags::CHECK_SUM);
    let body_len = msg.segments_len(in_body);
    let start = out.len();
    out.reserve(begin_string.len() + body_len + 24);
    out.extend_from_slice(b"8=");
    out.extend_from_slice(begin_string.as_bytes());
    out.extend_from_slice(b"\x019=");
    push_digits(out, body_len);
    out.push(SOH);
    msg.write_segments(out, in_body);
    push_trailer(out, start);
    Ok(())
}

/// Appends the CheckSum(10) field for the message that starts at `out[start]`.
pub(crate) fn push_trailer(out: &mut Vec<u8>, start: usize) {
    let sum = checksum(&out[start..]);
    out.extend_from_slice(&[b'1', b'0', b'=', b'0' + sum / 100, b'0' + sum / 10 % 10, b'0' + sum % 10, SOH]);
}

/// Appends the decimal digits of `n` without allocating.
pub(crate) fn push_digits(out: &mut Vec<u8>, mut n: usize) {
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
    out.extend_from_slice(&digits[start..]);
}

/// The CheckSum(10) of `bytes`: their sum modulo 256.
pub fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |acc, b| acc.wrapping_add(*b))
}

/// Checks that the field at `start` begins with `prefix` and returns (index of its SOH, index
/// just past its SOH).
fn header_field(buf: &[u8], start: usize, prefix: &[u8]) -> Result<(usize, usize), Decoded> {
    let avail = &buf[start..];
    let n = avail.len().min(prefix.len());
    if avail[..n] != prefix[..n] {
        let tag = String::from_utf8_lossy(&prefix[..prefix.len() - 1]).into_owned();
        return Err(garbled(buf, format!("expected tag {tag}")));
    }
    match avail.iter().position(|&b| b == SOH) {
        Some(p) => Ok((start + p, start + p + 1)),
        None if avail.len() > MAX_HEADER_FIELD_LEN => Err(garbled(buf, "header field too long".into())),
        None => Err(Decoded::Incomplete),
    }
}

fn garbled(buf: &[u8], reason: String) -> Decoded {
    // Resynchronise on the next plausible BeginString.
    let skip = buf.windows(5).skip(1).position(|w| w == b"8=FIX").map(|p| p + 1).unwrap_or_else(|| {
        // Keep a trailing partial "8=FIX" that may be completed by the next read.
        let keep = (1..5).rev().find(|&k| buf.ends_with(&b"8=FIX"[..k])).unwrap_or(0);
        buf.len() - keep
    });
    Decoded::Garbled { skip: skip.max(1), reason }
}

fn parse_digits(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() || bytes.len() > 9 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(bytes).ok()?.parse().ok()
}

/// `msg` framed for the wire with `raw` (e.g. `b"x5=A"`) appended as an extra body field, with
/// correct BodyLength and CheckSum, so the decoder sees exactly those bytes.
#[cfg(test)]
pub(crate) fn frame_with_raw_field(msg: &Message, raw: &[u8]) -> Vec<u8> {
    let wire = encode(msg).unwrap();
    // Body: after "9=<len><SOH>", up to the "10=NNN<SOH>" trailer.
    let len_start = wire.iter().position(|&b| b == SOH).unwrap() + 1;
    let body_start = len_start + wire[len_start..].iter().position(|&b| b == SOH).unwrap() + 1;
    let mut body = wire[body_start..wire.len() - TRAILER_LEN].to_vec();
    body.extend_from_slice(raw);
    body.push(SOH);
    let mut out = wire[..len_start].to_vec();
    out.extend_from_slice(format!("9={}", body.len()).as_bytes());
    out.push(SOH);
    out.extend_from_slice(&body);
    let sum = checksum(&out);
    out.extend_from_slice(format!("10={sum:03}").as_bytes());
    out.push(SOH);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{MsgType, SessionRejectReason};

    fn sample() -> Message {
        Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.4")
            .with(tags::MSG_TYPE, MsgType::Heartbeat)
            .with(tags::SENDER_COMP_ID, "A")
            .with(tags::TARGET_COMP_ID, "B")
            .with(tags::MSG_SEQ_NUM, "1")
    }

    #[test]
    fn encodes_known_checksum() {
        let text = String::from_utf8(encode(&sample()).unwrap()).unwrap().replace('\x01', "|");
        assert_eq!(text, "8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=125|");
    }

    #[test]
    fn refuses_a_message_without_begin_string() {
        let msg = Message::new(MsgType::Heartbeat).with(tags::SENDER_COMP_ID, "A");
        let missing = FieldError { tag: tags::BEGIN_STRING, kind: FieldErrorKind::Missing };
        assert_eq!(encode(&msg), Err(missing.clone()));
        let mut out = b"kept".to_vec();
        assert_eq!(encode_into(&msg, &mut out), Err(missing));
        assert_eq!(out, b"kept");
    }

    #[test]
    fn round_trips() {
        let wire = encode(&sample()).unwrap();
        match decode(&wire) {
            Decoded::Message(msg, len) => {
                assert_eq!(len, wire.len());
                assert_eq!(msg.get(tags::SENDER_COMP_ID), Some("A"));
                assert_eq!(msg.msg_type(), MsgType::Heartbeat);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn partial_input_is_incomplete() {
        let wire = encode(&sample()).unwrap();
        for split in 0..wire.len() {
            assert_eq!(decode(&wire[..split]), Decoded::Incomplete, "split at {split}");
        }
    }

    #[test]
    fn decodes_back_to_back_messages() {
        let mut wire = encode(&sample()).unwrap();
        wire.extend(encode(&sample().with(tags::MSG_SEQ_NUM, "2")).unwrap());
        let Decoded::Message(_, len) = decode(&wire) else { panic!() };
        let Decoded::Message(second, _) = decode(&wire[len..]) else { panic!() };
        assert_eq!(second.get(tags::MSG_SEQ_NUM), Some("2"));
    }

    #[test]
    fn bad_checksum_is_skipped_and_next_message_decodes() {
        let mut wire = encode(&sample()).unwrap();
        let n = wire.len();
        wire[n - 2] = if wire[n - 2] == b'9' { b'0' } else { b'9' };
        wire.extend(encode(&sample().with(tags::MSG_SEQ_NUM, "2")).unwrap());
        let Decoded::Garbled { skip, reason } = decode(&wire) else { panic!() };
        assert!(reason.contains("CheckSum"), "{reason}");
        assert_eq!(skip, n);
        let Decoded::Message(msg, _) = decode(&wire[skip..]) else { panic!() };
        assert_eq!(msg.get(tags::MSG_SEQ_NUM), Some("2"));
    }

    #[test]
    fn leading_garbage_is_skipped() {
        let mut wire = b"garbage".to_vec();
        wire.extend(encode(&sample()).unwrap());
        let Decoded::Garbled { skip, .. } = decode(&wire) else { panic!() };
        assert_eq!(skip, 7);
        assert!(matches!(decode(&wire[skip..]), Decoded::Message(..)));
    }

    #[test]
    fn wrong_body_length_is_garbled() {
        let wire = b"8=FIX.4.4\x019=5\x0135=0\x0149=A\x0110=000\x01";
        assert!(matches!(decode(wire), Decoded::Garbled { .. }));
    }

    #[test]
    fn msg_type_out_of_place_is_garbled() {
        let body = b"34=1\x0135=0\x0149=A\x0156=B\x01";
        let mut wire = format!("8=FIX.4.4\x019={}\x01", body.len()).into_bytes();
        wire.extend_from_slice(body);
        let sum = checksum(&wire);
        wire.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
        let Decoded::Garbled { reason, skip } = decode(&wire) else { panic!("decoded") };
        assert!(reason.contains("MsgType(35)"), "{reason}");
        assert_eq!(skip, wire.len());
    }

    #[test]
    fn a_body_defect_decodes_with_the_defect() {
        let msg = sample().with(tags::SENDING_TIME, "20260928-12:00:00");
        let wire = frame_with_raw_field(&msg, b"x5=A");
        let Decoded::Message(msg, len) = decode(&wire) else { panic!("garbled") };
        assert_eq!(len, wire.len());
        assert_eq!(msg.defect().unwrap().reason, SessionRejectReason::InvalidTagNumber);
    }

    #[test]
    fn a_header_defect_is_garbled() {
        let body = b"35=D\x0149=\xff\x0156=B\x0134=1\x0152=20260928-12:00:00\x0111=A\x01";
        let mut wire = format!("8=FIX.4.4\x019={}\x01", body.len()).into_bytes();
        wire.extend_from_slice(body);
        let sum = checksum(&wire);
        wire.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
        let Decoded::Garbled { reason, .. } = decode(&wire) else { panic!("decoded") };
        assert!(reason.contains("not UTF-8"), "{reason}");
    }

    #[test]
    fn data_fields_round_trip_with_soh_and_bytes_that_are_not_utf8() {
        let msg = sample()
            .with(tags::SENDING_TIME, "20260930-12:00:00")
            .with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"\x01\xff10=000\x01")
            .with_data(5000, 5001, b"a\x01b")
            .with(tags::TEXT, "after");
        let wire = encode(&msg).unwrap();
        let data = DataFields::standard().with(5000, 5001);
        let Decoded::Message(decoded, len) = decode_with(&wire, &data) else { panic!("did not decode") };
        assert_eq!(len, wire.len());
        assert!(decoded.defect().is_none(), "{:?}", decoded.defect());
        assert_eq!(decoded.get_bytes(tags::RAW_DATA), Some(&b"\x01\xff10=000\x01"[..]));
        assert_eq!(decoded.get(5001), Some("a\x01b"));
        assert_eq!(decoded.get(tags::TEXT), Some("after"));
        // Without the venue's pair, 5001 ends at its SOH and `b` is a malformed field.
        let Decoded::Message(decoded, _) = decode(&wire) else { panic!("did not decode") };
        assert_eq!(decoded.get(5001), Some("a"));
        assert!(decoded.defect().is_some());
    }
}
