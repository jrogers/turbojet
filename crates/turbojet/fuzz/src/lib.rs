//! Helpers shared by the fuzz targets.

use turbojet::Message;
use turbojet::codec::{Decoded, checksum, decode, encode};
use turbojet::message::tags;

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
    let bytes = encode(msg).expect("accepted and sent messages have a BeginString");
    let again = match decode(&bytes) {
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
fn body(msg: &Message) -> Vec<(u32, &str)> {
    msg.fields().filter(|(tag, _)| !matches!(*tag, tags::BEGIN_STRING | tags::BODY_LENGTH | tags::CHECK_SUM)).collect()
}
