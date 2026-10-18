//! Tag=value wire encoding: framing via BodyLength(9) and validation via CheckSum(10).

use crate::message::{DataFields, FieldError, FieldErrorKind, Message, SOH, digits, tags};

/// Upper bound on BodyLength(9) from the counterparty, to stop a bad length from buffering
/// unbounded input. Messages Turbojet stored itself aren't held to it (see [`frame_stored`]).
const MAX_BODY_LENGTH: usize = 64 * 1024;
/// Longest BeginString or BodyLength field we wait for before declaring the input garbled.
const MAX_HEADER_FIELD_LEN: usize = 32;
/// `10=NNN<SOH>`
const TRAILER_LEN: usize = 7;

const _: () = assert!(TRAILER_LEN == b"10=000\x01".len());
const _: () = assert!(MAX_HEADER_FIELD_LEN < MAX_BODY_LENGTH);

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
    parse_framed(buf, total, data)
}

/// A message [`decode_text`] couldn't read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMessageError {
    /// The field at fault, when its tag is known.
    pub tag: Option<u32>,
    /// What was wrong.
    pub reason: String,
}

impl std::fmt::Display for ParseMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for ParseMessageError {}

/// Parses a message from text as logs and tools show it, such as
/// `8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=125|`: fields separated by SOH or `|`, a separator after
/// the last field optional. A data field in `data` that follows its Length field is read by that
/// length, so its value may contain either separator. BodyLength(9) and CheckSum(10) are checked
/// when they're there, counting each separator as the SOH it stands for, and added when they
/// aren't, so the message is the one [`decode_with`] gives for the same message on the wire; a
/// message's [`Display`](std::fmt::Display) form parses back to it unless it has a data value
/// that isn't UTF-8. Unlike decoding, a malformed field is an error, not a defect for the session
/// to reject.
///
/// [`Message`]'s `FromStr` is this with [`DataFields::standard`]: `text.parse::<Message>()`.
///
/// # Errors
///
/// A field that isn't `tag=value`, BeginString(8) not first or MsgType(35) not after it (and
/// BodyLength(9), if that's there), a BodyLength(9) or CheckSum(10) that doesn't match the message,
/// or a Length field that doesn't give its data field's length.
pub fn decode_text(text: &str, data: &DataFields) -> Result<Message, ParseMessageError> {
    let fields = split_text(text.as_bytes(), data)?;
    let Some(((tags::BEGIN_STRING, begin_string), rest)) = fields.split_first() else {
        return Err(text_error(Some(tags::BEGIN_STRING), "BeginString(8) is not the first field".into()));
    };
    let (body_len_given, rest) = match rest {
        [(tags::BODY_LENGTH, value), rest @ ..] => (Some(*value), rest),
        _ => (None, rest),
    };
    let (checksum_given, body) = match rest {
        [body @ .., (tags::CHECK_SUM, value)] => (Some(*value), body),
        _ => (None, rest),
    };
    if !matches!(body.first(), Some((tags::MSG_TYPE, _))) {
        return Err(text_error(Some(tags::MSG_TYPE), "MsgType(35) does not follow BeginString(8)".into()));
    }

    let mut frame = b"8=".to_vec();
    frame.extend_from_slice(begin_string);
    frame.extend_from_slice(b"\x019=");
    let body_len = body.iter().map(|(tag, value)| digits(*tag) + value.len() + 2).sum();
    check_given(tags::BODY_LENGTH, body_len_given, body_len)?;
    push_digits(&mut frame, body_len);
    frame.push(SOH);
    for (tag, value) in body {
        push_digits(&mut frame, usize::try_from(*tag).expect("a u32 fits a usize"));
        frame.push(b'=');
        frame.extend_from_slice(value);
        frame.push(SOH);
    }
    check_given(tags::CHECK_SUM, checksum_given, usize::from(checksum(&frame)))?;
    push_trailer(&mut frame, 0);
    debug_assert_eq!(frame_stored(&frame), Ok(frame.len()), "the text becomes a frame the codec accepts");

    let msg = Message::from_frame(&frame, data).map_err(|reason| text_error(None, reason))?;
    match msg.defect() {
        Some(defect) => Err(text_error(defect.tag, defect.text.clone())),
        None => Ok(msg),
    }
}

/// Parses a message from text with the standard data fields; see [`decode_text`].
impl std::str::FromStr for Message {
    type Err = ParseMessageError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        decode_text(text, &DataFields::standard())
    }
}

/// `text`'s fields as `(tag, value)`, split at SOH or `|`, data fields read by their lengths.
fn split_text<'a>(text: &'a [u8], data: &DataFields) -> Result<Vec<(u32, &'a [u8])>, ParseMessageError> {
    let is_separator = |b: &u8| *b == SOH || *b == b'|';
    let mut fields: Vec<(u32, &[u8])> = Vec::new();
    let mut start = 0;
    // Each field moves `start` on by at least one byte, so this ends within the text.
    while start < text.len() {
        let segment_end = text[start..].iter().position(is_separator).map_or(text.len(), |p| start + p);
        let segment = &text[start..segment_end];
        let eq = segment.iter().position(|&b| b == b'=');
        let tag = eq.and_then(|eq| parse_digits(&segment[..eq])).and_then(|tag| u32::try_from(tag).ok());
        let (Some(eq), Some(tag @ 1..)) = (eq, tag) else {
            let field = String::from_utf8_lossy(segment);
            return Err(text_error(None, format!("'{field}' is not a tag=value field")));
        };
        let value_start = start + eq + 1;
        let end = match fields.last() {
            Some(&(length_tag, length)) if data.length_tag(tag) == Some(length_tag) => {
                let end = parse_digits(length).map(|length| value_start + length);
                match end.filter(|&end| end == text.len() || text.get(end).is_some_and(is_separator)) {
                    Some(end) => end,
                    None => {
                        let reason = format!("Tag {length_tag} does not give the length of data field {tag}");
                        return Err(text_error(Some(length_tag), reason));
                    }
                }
            }
            _ => segment_end,
        };
        debug_assert!(value_start <= end && end <= text.len());
        fields.push((tag, &text[value_start..end]));
        start = end + 1;
    }
    Ok(fields)
}

/// Checks that a BodyLength(9) or CheckSum(10) `given` in text is the `computed` one.
fn check_given(tag: u32, given: Option<&[u8]>, computed: usize) -> Result<(), ParseMessageError> {
    match given {
        Some(value) if parse_digits(value) != Some(computed) => {
            let value = String::from_utf8_lossy(value);
            Err(text_error(Some(tag), format!("Tag {tag} is {value}, but the message gives {computed}")))
        }
        _ => Ok(()),
    }
}

fn text_error(tag: Option<u32>, reason: String) -> ParseMessageError {
    ParseMessageError { tag, reason }
}

/// The result of [`decode_into`]: [`Decoded`] without the message, which is in the one passed in.
#[derive(Debug, PartialEq)]
pub enum DecodedInto {
    /// A complete, valid message, decoded into the message passed in, and the number of bytes it
    /// occupied.
    Message(usize),
    /// More bytes are needed.
    Incomplete,
    /// As [`Decoded::Garbled`].
    Garbled {
        /// Bytes to drop from the front of the buffer.
        skip: usize,
        /// What was wrong, for logging.
        reason: String,
    },
}

/// [`decode_with`], decoding into `msg` and replacing what it held. Unless the result is
/// [`DecodedInto::Message`], `msg` may be unchanged or partly overwritten: don't use it. Reusing
/// one message for each frame, as the connection's read loop does, avoids allocating once it has
/// grown to the size of the messages received.
pub fn decode_into(buf: &[u8], data: &DataFields, msg: &mut Message) -> DecodedInto {
    let total = match frame(buf) {
        Ok(total) => total,
        Err(d) => return without_message(d),
    };
    debug_assert!(total > 0, "a frame has bytes");
    debug_assert!(total <= buf.len(), "a frame lies within the buffer");
    match msg.read_frame(&buf[..total], data) {
        Ok(()) => DecodedInto::Message(total),
        Err(reason) => without_message(garbled(buf, reason)),
    }
}

/// `d`, which isn't [`Decoded::Message`], as a [`DecodedInto`].
fn without_message(d: Decoded) -> DecodedInto {
    match d {
        Decoded::Incomplete => DecodedInto::Incomplete,
        Decoded::Garbled { skip, reason } => DecodedInto::Garbled { skip, reason },
        Decoded::Message(..) => unreachable!("framing doesn't decode a message"),
    }
}

/// [`decode_with`] for a message Turbojet encoded and stored itself: trusted to be as long as it
/// says, since nothing limits what an application sends.
pub(crate) fn decode_stored(buf: &[u8], data: &DataFields) -> Decoded {
    let total = match frame_stored(buf) {
        Ok(total) => total,
        Err(d) => return d,
    };
    parse_framed(buf, total, data)
}

fn parse_framed(buf: &[u8], total: usize, data: &DataFields) -> Decoded {
    match Message::from_frame(&buf[..total], data) {
        Ok(msg) => Decoded::Message(msg, total),
        Err(reason) => garbled(buf, reason),
    }
}

/// The length of the message at the front of `buf`, if its framing is valid: BeginString,
/// BodyLength and MsgType first, and the CheckSum where BodyLength puts it. Otherwise
/// [`Decoded::Incomplete`] or [`Decoded::Garbled`]. The body isn't parsed.
pub(crate) fn frame(buf: &[u8]) -> Result<usize, Decoded> {
    frame_within(buf, MAX_BODY_LENGTH)
}

/// [`frame`] for a message Turbojet encoded and stored itself, whatever its BodyLength(9).
pub(crate) fn frame_stored(buf: &[u8]) -> Result<usize, Decoded> {
    frame_within(buf, usize::MAX)
}

fn frame_within(buf: &[u8], max_body_len: usize) -> Result<usize, Decoded> {
    if buf.is_empty() {
        return Err(Decoded::Incomplete);
    }
    let (_, len_start) = header_field(buf, 0, b"8=")?;
    let (len_soh, body_start) = header_field(buf, len_start, b"9=")?;
    let body_len = match parse_digits(&buf[len_start + 2..len_soh]) {
        Some(n) if n <= max_body_len => n,
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
    debug_assert!(total <= buf.len());
    debug_assert_eq!(buf[total - 1], SOH);
    Ok(total)
}

/// Encodes a message, computing BodyLength(9) and CheckSum(10). BeginString(8) is taken from the
/// message, and a message without one is refused as missing it; any BodyLength or CheckSum fields
/// already present are ignored.
///
/// # Errors
///
/// [`FieldErrorKind::Missing`] on BeginString(8) if the
/// message has none.
pub fn encode(msg: &Message) -> Result<Vec<u8>, FieldError> {
    let mut out = Vec::new();
    encode_into(msg, &mut out)?;
    Ok(out)
}

/// [`encode`], appending to `out` so a buffer can be reused across messages. On error, `out` is
/// unchanged.
///
/// # Errors
///
/// As [`encode`].
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
    let body_start = out.len();
    msg.write_segments(out, in_body);
    debug_assert_eq!(out.len() - body_start, body_len, "the body is as long as BodyLength(9) says");
    push_trailer(out, start);
    // What we encode, we can frame: the pair of the checks decoding makes.
    debug_assert_eq!(frame_stored(&out[start..]), Ok(out.len() - start));
    Ok(())
}

/// Appends the CheckSum(10) field for the message that starts at `out[start]`.
pub(crate) fn push_trailer(out: &mut Vec<u8>, start: usize) {
    debug_assert!(start < out.len(), "a trailer follows a message");
    let sum = checksum(&out[start..]);
    out.extend_from_slice(&[b'1', b'0', b'=', b'0' + sum / 100, b'0' + sum / 10 % 10, b'0' + sum % 10, SOH]);
    debug_assert!(out[out.len() - TRAILER_LEN..].starts_with(b"10="));
}

/// Appends the decimal digits of `n` without allocating.
pub(crate) fn push_digits(out: &mut Vec<u8>, mut n: usize) {
    let mut digits = [0u8; 20];
    let mut start = digits.len();
    loop {
        start -= 1;
        #[expect(clippy::cast_possible_truncation, reason = "a digit, under 10")]
        let digit = (n % 10) as u8;
        digits[start] = b'0' + digit;
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
/// The length of the message at the front of `buf` if it frames as one (BeginString, BodyLength,
/// and a CheckSum field where BodyLength puts it) whatever else is wrong with it: garbled input
/// worth logging as the message it was meant to be, as QuickFIX logs what it frames before
/// checking it. `None` for input that doesn't frame.
pub(crate) fn framed_len(buf: &[u8]) -> Option<usize> {
    let (_, len_start) = header_field(buf, 0, b"8=").ok()?;
    let (len_soh, body_start) = header_field(buf, len_start, b"9=").ok()?;
    let body_len = parse_digits(&buf[len_start + 2..len_soh]).filter(|&n| n <= MAX_BODY_LENGTH)?;
    let total = body_start + body_len + TRAILER_LEN;
    let trailer = buf.get(total - TRAILER_LEN..total)?;
    (&trailer[..3] == b"10=" && trailer[6] == SOH).then_some(total)
}

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
    debug_assert!(!buf.is_empty(), "garbled input has bytes");
    // Resynchronise on the next plausible BeginString.
    let skip = buf.windows(5).skip(1).position(|w| w == b"8=FIX").map(|p| p + 1).unwrap_or_else(|| {
        // Keep a trailing partial "8=FIX" that may be completed by the next read.
        let keep = (1..5).rev().find(|&k| buf.ends_with(&b"8=FIX"[..k])).unwrap_or(0);
        buf.len() - keep
    });
    let skip = skip.max(1);
    // The reader drops `skip` bytes: at least one, so it can't spin on the same input, and no
    // more than it has.
    debug_assert!(skip <= buf.len(), "skip {skip} past the {} bytes read", buf.len());
    Decoded::Garbled { skip, reason }
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

    /// Input that frames as a message, but fails a check after, gives its length; input that
    /// doesn't frame gives none.
    #[test]
    fn framed_len_is_for_input_that_frames() {
        let good = encode(&Message::new(MsgType::Heartbeat).with(tags::BEGIN_STRING, "FIX.4.2")).unwrap();
        assert_eq!(framed_len(&good), Some(good.len()));
        let mut bad_sum = good.clone();
        let last_digit = bad_sum.len() - 2;
        bad_sum[last_digit] = if bad_sum[last_digit] == b'0' { b'1' } else { b'0' };
        let mut with_more = bad_sum.clone();
        with_more.extend_from_slice(&good);
        assert!(matches!(decode(&bad_sum), Decoded::Garbled { .. }));
        assert_eq!(framed_len(&with_more), Some(good.len()), "the first message only");

        assert_eq!(framed_len(b"junk"), None);
        assert_eq!(framed_len(b"8=FIX.4.2\x019=x\x0135=0\x0110=000\x01"), None, "BodyLength not a number");
        let short = String::from_utf8(good.clone()).unwrap().replacen("9=5", "9=4", 1);
        assert_eq!(framed_len(short.as_bytes()), None, "no CheckSum where BodyLength puts it");
        assert_eq!(framed_len(&good[..good.len() - 1]), None, "incomplete");
    }
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
    fn decode_into_reuses_one_message_across_a_stream() {
        let first = encode(&sample()).unwrap();
        let second = encode(&sample().with(tags::TEXT, "second")).unwrap();
        let mut stream = first.clone();
        stream.extend_from_slice(b"garbage");
        stream.extend_from_slice(&second);
        stream.extend_from_slice(&second[..10]);

        let data = DataFields::standard();
        let mut msg = Message::default();
        let mut rest = &stream[..];
        let mut seen = Vec::new();
        let mut garbled = 0;
        loop {
            match decode_into(rest, &data, &mut msg) {
                DecodedInto::Message(len) => {
                    seen.push(msg.clone());
                    rest = &rest[len..];
                }
                DecodedInto::Garbled { skip, .. } => {
                    garbled += 1;
                    rest = &rest[skip..];
                }
                DecodedInto::Incomplete => break,
            }
        }
        let Decoded::Message(expected_first, _) = decode(&first) else { panic!() };
        let Decoded::Message(expected_second, _) = decode(&second) else { panic!() };
        assert_eq!(seen, [expected_first, expected_second]);
        assert_eq!(garbled, 1, "the garbage is skipped in one go");
        assert_eq!(rest, &second[..10], "the partial message is left for the next read");
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
    fn decode_into_garbles_a_frame_it_refuses_after_framing() {
        let body = b"35=D\x0149=\xff\x0156=B\x0134=1\x0152=20260928-12:00:00\x0111=A\x01";
        let mut stream = format!("8=FIX.4.4\x019={}\x01", body.len()).into_bytes();
        stream.extend_from_slice(body);
        let sum = checksum(&stream);
        stream.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
        let good = encode(&sample()).unwrap();
        stream.extend_from_slice(&good);

        let mut msg = Message::default();
        let data = DataFields::standard();
        let Decoded::Garbled { skip, reason } = decode(&stream) else { panic!("decoded") };
        assert_eq!(decode_into(&stream, &data, &mut msg), DecodedInto::Garbled { skip, reason });
        let rest = &stream[skip..];
        assert_eq!(decode_into(rest, &data, &mut msg), DecodedInto::Message(good.len()));
        let Decoded::Message(expected, _) = decode(rest) else { panic!() };
        assert_eq!(msg, expected);
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

    fn decoded(msg: &Message) -> Message {
        let Decoded::Message(msg, _) = decode(&encode(msg).unwrap()) else { panic!("did not decode") };
        msg
    }

    #[test]
    fn text_parses_to_the_message_decoding_gives() {
        let expected = decoded(&sample());
        // As written, without its trailing separator, with SOH, and without BodyLength and CheckSum.
        for text in [
            "8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=125|",
            "8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=125",
            "8=FIX.4.4\x019=20\x0135=0|49=A|56=B|34=1|10=125\x01",
            "8=FIX.4.4|35=0|49=A|56=B|34=1|",
        ] {
            assert_eq!(text.parse::<Message>(), Ok(expected.clone()), "{text}");
        }
    }

    #[test]
    fn display_parses_back_with_separators_inside_data_values() {
        let msg =
            decoded(&sample().with_data(tags::RAW_DATA_LENGTH, tags::RAW_DATA, b"a|b\x01c|").with(tags::TEXT, "x"));
        assert_eq!(msg.to_string().parse::<Message>(), Ok(msg));
    }

    #[test]
    fn text_errors_name_the_field() {
        let cases: [(&str, Option<u32>); 8] = [
            ("8=FIX.4.4|9=21|35=0|49=A|56=B|34=1|10=125|", Some(9)),
            ("8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=126|", Some(10)),
            ("8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=12x|", Some(10)),
            ("35=0|8=FIX.4.4|49=A|", Some(8)),
            ("8=FIX.4.4|49=A|35=0|", Some(35)),
            ("8=FIX.4.4|35=0|x5=A|", None),
            ("8=FIX.4.4|35=0||49=A", None),
            ("8=FIX.4.4|35=0|95=5|96=abc|58=x|", Some(95)),
        ];
        for (text, tag) in cases {
            let error = text.parse::<Message>().expect_err(text);
            assert_eq!(error.tag, tag, "{text}: {error}");
        }
        assert_eq!("".parse::<Message>().map_err(|e| e.tag), Err(Some(8)));
    }
}
