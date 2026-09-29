//! A message body (everything between BodyLength and CheckSum), framed correctly so it reaches
//! the message parser, then read the ways the session and applications read it.

#![no_main]

use libfuzzer_sys::fuzz_target;
use turbojet::codec::{Decoded, decode};
use turbojet::fields::UtcTimestamp;
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
    for (tag, _) in msg.fields() {
        assert_eq!(body.get(tag), msg.get(tag), "tag {tag}");
    }
    let _ = msg.field::<u64>(tags::MSG_SEQ_NUM);
    let _ = msg.opt_field::<UtcTimestamp>(tags::SENDING_TIME);
    let _ = msg.opt_field::<UtcTimestamp>(tags::ORIG_SENDING_TIME);
    let _ = msg.opt_field::<bool>(tags::POSS_DUP_FLAG);
    let _ = msg.opt_field::<i64>(tags::HEART_BT_INT);
    let _ = msg.to_string();
    let _ = msg.redacted().to_string();
    round_trip(&msg);
});
