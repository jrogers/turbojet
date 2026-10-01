//! The invariants, checked against what each side writes and what each application receives.

use std::collections::BTreeMap;
use std::fmt;

use turbojet::codec::{Decoded, decode};
use turbojet::message::tags;
use turbojet::{Message, MsgType};

use crate::Side;
use crate::app::{Delivery, id_of};
use crate::time::SimTime;

/// A broken invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub rule: &'static str,
    pub detail: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.rule, self.detail)
    }
}

fn violation(rule: &'static str, detail: impl Into<String>) -> Violation {
    Violation { rule, detail: detail.into() }
}

/// Fields a resend may change: framing, MsgSeqNum (kept, but checked separately), and the resend
/// markers and times.
const RESEND_MAY_CHANGE: &[u32] = &[
    tags::BEGIN_STRING,
    tags::BODY_LENGTH,
    tags::CHECK_SUM,
    tags::MSG_SEQ_NUM,
    tags::POSS_DUP_FLAG,
    tags::SENDING_TIME,
    tags::POSS_RESEND,
    tags::ORIG_SENDING_TIME,
];

/// What one side has sent in the current sequence epoch.
#[derive(Default)]
struct Sent {
    /// The MsgSeqNum the next new message must have.
    next_new: u64,
    /// Application messages as first sent, by MsgSeqNum.
    first: BTreeMap<u64, Message>,
}

/// What one side's application has received.
#[derive(Default)]
struct Received {
    /// Deliveries already checked.
    seen: usize,
    last_seq: u64,
    ids: BTreeMap<String, u64>,
}

pub struct Checker {
    sent: [Sent; 2],
    received: [Received; 2],
}

impl Checker {
    pub fn new() -> Self {
        let fresh = || Sent { next_new: 1, first: BTreeMap::new() };
        Self { sent: [fresh(), fresh()], received: Default::default() }
    }

    /// Application messages `side` has sent with a MsgSeqNum, by id.
    pub fn committed(&self, side: Side) -> impl Iterator<Item = &str> {
        self.sent[side.index()].first.values().filter_map(id_of)
    }

    /// Rules 1-3, on what `side` wrote.
    pub fn written(&mut self, side: Side, mut bytes: &[u8], _now: SimTime) -> Result<(), Violation> {
        while !bytes.is_empty() {
            // Rule 1: everything written is whole, valid messages.
            let (msg, len) = match decode(bytes) {
                Decoded::Message(msg, len) => (msg, len),
                other => return Err(violation("1 valid output", format!("{side:?} wrote {other:?}"))),
            };
            bytes = &bytes[len..];
            self.sent_message(side, msg)?;
        }
        Ok(())
    }

    fn sent_message(&mut self, side: Side, msg: Message) -> Result<(), Violation> {
        let sent = &mut self.sent[side.index()];
        let seq: u64 = msg.get(tags::MSG_SEQ_NUM).and_then(|s| s.parse().ok()).unwrap_or(0);
        let poss_dup = msg.get(tags::POSS_DUP_FLAG) == Some("Y");
        // A Logon resetting sequence numbers starts a new epoch.
        if msg.msg_type() == MsgType::Logon && msg.get(tags::RESET_SEQ_NUM_FLAG) == Some("Y") && seq == 1 {
            *sent = Sent { next_new: 1, first: BTreeMap::new() };
        }
        if !poss_dup {
            // Rule 2: new messages take consecutive numbers.
            if seq != sent.next_new {
                return Err(violation(
                    "2 sequence",
                    format!("{side:?} sent new {seq}, expected {}: {msg}", sent.next_new),
                ));
            }
            sent.next_new += 1;
            if !msg.msg_type().is_admin() {
                sent.first.insert(seq, msg);
            }
            return Ok(());
        }
        if seq >= sent.next_new {
            return Err(violation("2 sequence", format!("{side:?} resent {seq}, never sent: {msg}")));
        }
        // Rule 3: a resend is the original, and a gap fill covers no application message.
        if msg.msg_type() == MsgType::SequenceReset {
            let new_seq_no: u64 = msg.get(tags::NEW_SEQ_NO).and_then(|s| s.parse().ok()).unwrap_or(0);
            if let Some((covered, _)) = sent.first.range(seq..new_seq_no).next() {
                return Err(violation("3 resend", format!("{side:?} gap-filled {seq}..{new_seq_no} over {covered}")));
            }
            return Ok(());
        }
        let Some(first) = sent.first.get(&seq) else {
            return Err(violation(
                "3 resend",
                format!("{side:?} resent {seq}, not an application message it sent: {msg}"),
            ));
        };
        let body = |m: &Message| -> Vec<(u32, String)> {
            m.fields().filter(|(tag, _)| !RESEND_MAY_CHANGE.contains(tag)).map(|(t, v)| (t, v.to_string())).collect()
        };
        if body(first) != body(&msg) {
            return Err(violation("3 resend", format!("{side:?} resent {seq} changed: was {first}, now {msg}")));
        }
        if msg.get(tags::ORIG_SENDING_TIME) != first.get(tags::SENDING_TIME) {
            return Err(violation(
                "3 resend",
                format!("{side:?} resent {seq} with OrigSendingTime not its SendingTime: {msg}"),
            ));
        }
        Ok(())
    }

    /// Rule 4, on what `side`'s application has received since the last call.
    pub fn delivered(&mut self, side: Side, deliveries: &[Delivery]) -> Result<(), Violation> {
        let sender = &self.sent[side.other().index()];
        let received = &mut self.received[side.index()];
        for delivery in &deliveries[received.seen..] {
            let expected = sender.first.get(&delivery.seq).and_then(id_of);
            if expected != Some(delivery.id.as_str()) {
                return Err(violation(
                    "4 delivery",
                    format!(
                        "{side:?} got {} as {}, but {expected:?} was sent as {}",
                        delivery.id, delivery.seq, delivery.seq
                    ),
                ));
            }
            match received.ids.get(&delivery.id) {
                Some(_) if delivery.redelivered => {}
                Some(seq) => {
                    return Err(violation(
                        "4 delivery",
                        format!("{side:?} got {} again (first as {seq})", delivery.id),
                    ));
                }
                None if delivery.seq <= received.last_seq => {
                    return Err(violation(
                        "4 delivery",
                        format!("{side:?} got {} as {} after {}", delivery.id, delivery.seq, received.last_seq),
                    ));
                }
                None => {}
            }
            received.ids.insert(delivery.id.clone(), delivery.seq);
            received.last_seq = received.last_seq.max(delivery.seq);
        }
        received.seen = deliveries.len();
        Ok(())
    }

    /// Ids `side`'s application has received.
    pub fn received_ids(&self, side: Side) -> impl Iterator<Item = &str> {
        self.received[side.index()].ids.keys().map(String::as_str)
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use turbojet::codec::encode;

    use super::*;
    use crate::app::order;

    fn framed(msg: Message, seq: u64, extra: &[(u32, &str)]) -> Vec<u8> {
        let mut out = Message::default()
            .with(tags::BEGIN_STRING, "FIX.4.4")
            .with(tags::MSG_TYPE, msg.msg_type())
            .with(tags::SENDER_COMP_ID, "A")
            .with(tags::TARGET_COMP_ID, "B")
            .with(tags::MSG_SEQ_NUM, seq)
            .with(tags::SENDING_TIME, "20260105-09:00:00.000");
        for (tag, value) in extra {
            out = out.with(*tag, *value);
        }
        for (tag, value) in msg.fields().filter(|(t, _)| *t != tags::MSG_TYPE) {
            out = out.with(tag, value);
        }
        encode(&out).unwrap()
    }

    fn resend(msg: Message, seq: u64) -> Vec<u8> {
        framed(msg, seq, &[(tags::POSS_DUP_FLAG, "Y"), (tags::ORIG_SENDING_TIME, "20260105-09:00:00.000")])
    }

    #[test]
    fn garbage_breaks_rule_1() {
        let err = Checker::new().written(Side::Initiator, b"8=FIX.4.4\x019=5\x01junk", SimTime(0)).unwrap_err();
        assert_eq!(err.rule, "1 valid output");
    }

    #[test]
    fn a_skipped_or_unsent_number_breaks_rule_2() {
        let mut checker = Checker::new();
        checker.written(Side::Initiator, &framed(order("a"), 1, &[]), SimTime(0)).unwrap();
        assert_eq!(
            checker.written(Side::Initiator, &framed(order("b"), 3, &[]), SimTime(0)).unwrap_err().rule,
            "2 sequence"
        );
        let mut checker = Checker::new();
        assert_eq!(
            checker.written(Side::Initiator, &resend(order("a"), 1), SimTime(0)).unwrap_err().rule,
            "2 sequence"
        );
    }

    #[test]
    fn a_changed_resend_or_a_gap_fill_over_an_order_breaks_rule_3() {
        let mut checker = Checker::new();
        checker.written(Side::Initiator, &framed(order("a"), 1, &[]), SimTime(0)).unwrap();
        checker.written(Side::Initiator, &resend(order("a"), 1), SimTime(0)).unwrap();
        assert_eq!(checker.written(Side::Initiator, &resend(order("z"), 1), SimTime(0)).unwrap_err().rule, "3 resend");
        let gap_fill = Message::new(MsgType::SequenceReset).with(tags::GAP_FILL_FLAG, "Y").with(tags::NEW_SEQ_NO, 2u64);
        assert_eq!(checker.written(Side::Initiator, &resend(gap_fill, 1), SimTime(0)).unwrap_err().rule, "3 resend");
    }

    /// A checker that has seen the initiator send orders "a" as 1 and "b" as 2.
    fn sent_a_and_b() -> Checker {
        let mut checker = Checker::new();
        for (seq, id) in [(1, "a"), (2, "b")] {
            checker.written(Side::Initiator, &framed(order(id), seq, &[]), SimTime(0)).unwrap();
        }
        checker
    }

    fn deliver(list: &[(&str, u64, bool)]) -> Result<(), &'static str> {
        let deliveries: Vec<_> = list
            .iter()
            .map(|(id, seq, redelivered)| Delivery { id: (*id).into(), seq: *seq, redelivered: *redelivered })
            .collect();
        sent_a_and_b().delivered(Side::Acceptor, &deliveries).map_err(|e| e.rule)
    }

    #[test]
    fn deliveries_must_match_what_was_sent_in_order_once() {
        assert_eq!(deliver(&[("a", 1, false), ("b", 2, false)]), Ok(()));
        assert_eq!(deliver(&[("a", 1, false), ("a", 1, true)]), Ok(()), "a marked redelivery");
        assert_eq!(deliver(&[("x", 1, false)]), Err("4 delivery"), "not what was sent as 1");
        assert_eq!(deliver(&[("a", 1, false), ("a", 1, false)]), Err("4 delivery"), "repeated unmarked");
        assert_eq!(deliver(&[("b", 2, false), ("a", 1, false)]), Err("4 delivery"), "out of order");
    }
}
