//! A message body (everything between BodyLength and CheckSum), framed correctly so it reaches
//! the message parser, then read the ways the session and applications read it.

#![no_main]

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode};
use turbojet::fields::{FromFix, MonthYear, NaiveDate, ToFix, TzTimeOnly, TzTimestamp, UtcTimeOnly, UtcTimestamp};
use turbojet::message::tags;
use turbojet_fuzz::{frame, round_trip};

fuzz_target!(|body: &[u8]| {
    let frame = frame("FIX.4.4", body);
    let msg = match decode(&frame) {
        Decoded::Message(msg, len) => {
            assert_eq!(len, frame.len());
            msg
        }
        // A malformed header field the session relies on (MsgType, the CompIDs, MsgSeqNum,
        // SendingTime), or a frame over the BodyLength limit.
        Decoded::Garbled { .. } => return,
        Decoded::Incomplete => panic!("a complete frame decoded as incomplete"),
    };
    let _ = msg.msg_type();
    let body = msg.body();
    for (tag, _) in msg.fields_bytes() {
        assert_eq!(body.get(tag), msg.get(tag), "tag {tag}");
        assert_eq!(body.get_bytes(tag), msg.get_bytes(tag), "tag {tag}");
    }
    let _ = msg.field::<u64>(tags::MSG_SEQ_NUM);
    let _ = msg.opt_field::<UtcTimestamp>(tags::SENDING_TIME);
    let _ = msg.opt_field::<UtcTimestamp>(tags::ORIG_SENDING_TIME);
    let _ = msg.opt_field::<bool>(tags::POSS_DUP_FLAG);
    let _ = msg.opt_field::<i64>(tags::HEART_BT_INT);
    // Any value that parses as one of the field types writes out as something that parses back
    // to the same value, and writes the same way again.
    for (_, value) in msg.fields() {
        stable::<UtcTimestamp>(value);
        stable::<UtcTimeOnly>(value);
        stable::<NaiveDate>(value);
        stable::<MonthYear>(value);
        stable::<TzTimeOnly>(value);
        stable::<TzTimestamp>(value);
        stable::<char>(value);
        stable::<Vec<String>>(value);
    }
    let _ = msg.to_string();
    let _ = msg.redacted().to_string();
    round_trip(&msg);
});

fn stable<T: FromFix + ToFix + PartialEq + std::fmt::Debug>(value: &str) {
    let Ok(parsed) = T::from_fix(value) else { return };
    let written = parsed.to_fix();
    let again = T::from_fix(&written).unwrap_or_else(|e| panic!("{value:?} wrote {written:?}, which fails: {e:?}"));
    assert_eq!(again, parsed, "{value:?} wrote {written:?}");
    assert_eq!(again.to_fix(), written, "{value:?}");
}
