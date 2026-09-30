//! Helpers shared by the fuzz targets.

use turbojet::Message;
use turbojet::codec::{Decoded, checksum, decode_with, encode};
use turbojet::message::{DataFields, tags};

/// The standard data fields and a venue's, 5000 giving the length of 5001.
pub fn venue_data_fields() -> DataFields {
    DataFields::standard().with(5000, 5001)
}

/// Frames `body` as a complete message: BeginString, a BodyLength that matches and a valid
/// CheckSum, and MsgType's `35=` first (prepended unless the body starts with it), so the codec
/// passes it on to the message parser. Random bytes almost never carry a valid checksum, so
/// without this the fuzzer would rarely get past framing.
pub fn frame(begin_string: &str, body: &[u8]) -> Vec<u8> {
    let prefix: &[u8] = if body.starts_with(b"35=") { b"" } else { b"35=" };
    let mut out = format!("8={begin_string}\x019={}\x01", prefix.len() + body.len()).into_bytes();
    out.extend_from_slice(prefix);
    out.extend_from_slice(body);
    let sum = checksum(&out);
    out.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
    out
}

/// Checks that a message Turbojet accepted can be encoded (to be stored, resent or forwarded)
/// and decoded again, with the same fields.
pub fn round_trip(msg: &Message) {
    round_trip_with(msg, &DataFields::standard());
}

/// [`round_trip`] for a message decoded knowing `data`.
///
/// Encoding leaves out BeginString, BodyLength and CheckSum wherever they are. Where one of them
/// came just before a data field, that brings the field before it next to the data field, which
/// may be its Length field: the re-encoded message then says something different. The session
/// never sends such a message (a data field must follow its Length field), so it's skipped.
///
/// A malformed message is re-encoded without its malformed fields, and with non-UTF-8 values
/// replaced, so its bytes change length; where it has a data field, whose length counts bytes,
/// the fields may then be read differently. The session rejects malformed messages rather than
/// storing or forwarding them, so those are skipped too.
pub fn round_trip_with(msg: &Message, data: &DataFields) {
    let fields: Vec<(u32, &[u8])> = msg.fields_bytes().collect();
    if fields.windows(2).any(|pair| data.is_data(pair[1].0) && is_framing(pair[0].0))
        || msg.is_malformed() && fields.iter().any(|&(tag, _)| data.is_data(tag))
    {
        return;
    }
    let bytes = encode(msg).expect("accepted and sent messages have a BeginString");
    let again = match decode_with(&bytes, data) {
        Decoded::Message(again, len) => {
            assert_eq!(len, bytes.len());
            again
        }
        other => panic!("{msg:?} re-encoded as {:?}: {other:?}", String::from_utf8_lossy(&bytes)),
    };
    assert_eq!(body(msg), body(&again));
    assert_eq!(again.get(tags::BEGIN_STRING), msg.get(tags::BEGIN_STRING));
}

/// The fields apart from those encoding writes itself: BeginString, BodyLength and CheckSum.
fn body(msg: &Message) -> Vec<(u32, &[u8])> {
    msg.fields_bytes().filter(|(tag, _)| !is_framing(*tag)).collect()
}

/// The fields encoding writes itself.
fn is_framing(tag: u32) -> bool {
    matches!(tag, tags::BEGIN_STRING | tags::BODY_LENGTH | tags::CHECK_SUM)
}
