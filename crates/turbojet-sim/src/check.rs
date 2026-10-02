//! The invariants, checked against what each side writes and what each application receives.

use std::collections::BTreeMap;
use std::fmt;

use turbojet::codec::{Decoded, decode};
use turbojet::message::tags;
use turbojet::{Message, MsgType};

use crate::Side;
use crate::app::{Delivery, id_of};
use crate::store::Stored;
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

/// What one side has committed to sending in the current sequence epoch, from its store, and
/// what of it has been on the wire.
struct Sent {
    /// The MsgSeqNum the store must record next.
    next_recorded: u64,
    /// The next incoming MsgSeqNum the store last recorded.
    incoming: u64,
    /// Every MsgSeqNum recorded: an application message with the message, a session one `None`.
    recorded: BTreeMap<u64, Option<Message>>,
    /// The last new (not PossDup) MsgSeqNum written.
    last_new: u64,
    /// Ledger entries already checked.
    ledger_seen: usize,
}

impl Default for Sent {
    fn default() -> Self {
        Self { next_recorded: 1, incoming: 1, recorded: BTreeMap::new(), last_new: 0, ledger_seen: 0 }
    }
}

/// What one side's application has received.
#[derive(Default)]
struct Received {
    /// Deliveries already checked.
    seen: usize,
    last_seq: u64,
    ids: BTreeMap<String, u64>,
}

#[derive(Default)]
pub struct Checker {
    sent: [Sent; 2],
    received: [Received; 2],
}

fn parse(bytes: &[u8]) -> Option<Message> {
    match decode(bytes) {
        Decoded::Message(msg, len) if len == bytes.len() => Some(msg),
        _ => None,
    }
}

fn number(msg: &Message, tag: u32) -> u64 {
    msg.get(tag).and_then(|s| s.parse().ok()).unwrap_or(0)
}

/// A message's fields that a resend keeps.
fn body(msg: &Message) -> Vec<(u32, String)> {
    msg.fields().filter(|(tag, _)| !RESEND_MAY_CHANGE.contains(tag)).map(|(t, v)| (t, v.to_string())).collect()
}

impl Checker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Application messages `side` has committed to sending, by id.
    pub fn committed(&self, side: Side) -> impl Iterator<Item = &str> {
        self.sent[side.index()].recorded.values().flatten().filter_map(id_of)
    }

    /// Rules 2 and 6 on what `side`'s store recorded since the last call: each MsgSeqNum once, in
    /// order, and a store that opens with the numbers it last recorded.
    pub fn stored(&mut self, side: Side, ledger: &[Stored]) -> Result<(), Violation> {
        let sent = &mut self.sent[side.index()];
        for entry in &ledger[sent.ledger_seen..] {
            match entry {
                Stored::Reset => {
                    let seen = sent.ledger_seen;
                    *sent = Sent { ledger_seen: seen, ..Sent::default() };
                }
                Stored::Incoming { seq } => sent.incoming = *seq,
                Stored::Opened { next_outgoing, next_incoming } => {
                    // Rule 6: what a store recorded is what it reopens with, crash or not.
                    if (*next_outgoing, *next_incoming) != (sent.next_recorded, sent.incoming) {
                        return Err(violation(
                            "6 store",
                            format!(
                                "{side:?}'s store opened at {next_outgoing} out, {next_incoming} in; it recorded {} out, {} in",
                                sent.next_recorded, sent.incoming
                            ),
                        ));
                    }
                }
                Stored::Sent { seq, bytes } => {
                    if *seq != sent.next_recorded {
                        return Err(violation(
                            "2 sequence",
                            format!("{side:?} stored {seq}, expected {}", sent.next_recorded),
                        ));
                    }
                    let msg =
                        match bytes {
                            Some(bytes) => Some(parse(bytes).ok_or_else(|| {
                                violation("1 valid output", format!("{side:?} stored garbage as {seq}"))
                            })?),
                            None => None,
                        };
                    sent.recorded.insert(*seq, msg);
                    sent.next_recorded = seq + 1;
                }
            }
            sent.ledger_seen += 1;
        }
        Ok(())
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
            self.sent_message(side, &msg)?;
        }
        Ok(())
    }

    fn sent_message(&mut self, side: Side, msg: &Message) -> Result<(), Violation> {
        let sent = &mut self.sent[side.index()];
        let seq = number(msg, tags::MSG_SEQ_NUM);
        let Some(recorded) = sent.recorded.get(&seq) else {
            return Err(violation(
                "2 sequence",
                format!("{side:?} wrote {seq}, which its store never recorded: {msg}"),
            ));
        };
        if msg.get(tags::POSS_DUP_FLAG) != Some("Y") {
            // Rule 2: new messages go out in order, each once, as recorded.
            if seq <= sent.last_new {
                return Err(violation(
                    "2 sequence",
                    format!("{side:?} wrote new {seq} after {}: {msg}", sent.last_new),
                ));
            }
            sent.last_new = seq;
            return match recorded {
                Some(stored) if body(stored) != body(msg) => {
                    Err(violation("2 sequence", format!("{side:?} wrote {seq} unlike it stored it: {stored} / {msg}")))
                }
                _ => Ok(()),
            };
        }
        // Rule 3: a resend is the original, and a gap fill covers no application message.
        if msg.msg_type() == MsgType::SequenceReset {
            let new_seq_no = number(msg, tags::NEW_SEQ_NO);
            if let Some((covered, _)) = sent.recorded.range(seq..new_seq_no).find(|(_, m)| m.is_some()) {
                return Err(violation("3 resend", format!("{side:?} gap-filled {seq}..{new_seq_no} over {covered}")));
            }
            return Ok(());
        }
        let Some(first) = recorded else {
            return Err(violation("3 resend", format!("{side:?} resent {seq}, a session message: {msg}")));
        };
        if body(first) != body(msg) {
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
            let expected = sender.recorded.get(&delivery.seq).and_then(Option::as_ref).and_then(id_of);
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
            // Rule 5: nothing the sender stored is skipped. Deliveries come in order, so one that
            // passes a stored message means it's lost.
            if !delivery.redelivered
                && let Some((lost, _)) = sender
                    .recorded
                    .range(received.last_seq + 1..delivery.seq)
                    .find(|(_, m)| m.as_ref().and_then(id_of).is_some_and(|id| !received.ids.contains_key(id)))
            {
                return Err(violation(
                    "5 lost",
                    format!("{side:?} got {} as {}, but never {lost}, stored before it", delivery.id, delivery.seq),
                ));
            }
            received.ids.insert(delivery.id.clone(), delivery.seq);
            received.last_seq = received.last_seq.max(delivery.seq);
        }
        received.seen = deliveries.len();
        Ok(())
    }

    /// The next outgoing and incoming MsgSeqNums `side`'s store last recorded.
    pub fn numbers(&self, side: Side) -> (u64, u64) {
        let sent = &self.sent[side.index()];
        (sent.next_recorded, sent.incoming)
    }

    /// Ids `side`'s application has received.
    pub fn received_ids(&self, side: Side) -> impl Iterator<Item = &str> {
        self.received[side.index()].ids.keys().map(String::as_str)
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

    /// A checker and the initiator's store ledger.
    #[derive(Default)]
    struct Harness {
        checker: Checker,
        ledger: Vec<Stored>,
    }

    impl Harness {
        /// The initiator stores `msg` as `seq`.
        fn store(&mut self, msg: Message, seq: u64) -> Result<(), &'static str> {
            self.ledger.push(Stored::Sent { seq, bytes: Some(framed(msg, seq, &[])) });
            self.checker.stored(Side::Initiator, &self.ledger).map_err(|e| e.rule)
        }

        /// The initiator stores `msg` as `seq` and writes it.
        fn send(&mut self, msg: Message, seq: u64) -> Result<(), &'static str> {
            self.store(msg.clone(), seq)?;
            self.write(&framed(msg, seq, &[]))
        }

        fn write(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
            self.checker.written(Side::Initiator, bytes, SimTime(0)).map_err(|e| e.rule)
        }
    }

    #[test]
    fn garbage_breaks_rule_1() {
        assert_eq!(Harness::default().write(b"8=FIX.4.4\x019=5\x01junk"), Err("1 valid output"));
    }

    #[test]
    fn a_skipped_unstored_or_repeated_number_breaks_rule_2() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        assert_eq!(h.store(order("b"), 3), Err("2 sequence"), "stored out of order");
        let mut h = Harness::default();
        assert_eq!(h.write(&framed(order("a"), 1, &[])), Err("2 sequence"), "written, never stored");
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        assert_eq!(h.write(&framed(order("a"), 1, &[])), Err("2 sequence"), "written new twice");
    }

    #[test]
    fn a_number_stored_but_lost_before_writing_is_fine() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        h.store(order("b"), 2).unwrap();
        assert_eq!(h.send(order("c"), 3), Ok(()));
        assert_eq!(h.write(&resend(order("b"), 2)), Ok(()));
    }

    #[test]
    fn a_changed_resend_or_a_gap_fill_over_an_order_breaks_rule_3() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        assert_eq!(h.write(&resend(order("a"), 1)), Ok(()));
        assert_eq!(h.write(&resend(order("z"), 1)), Err("3 resend"));
        let gap_fill = Message::new(MsgType::SequenceReset).with(tags::GAP_FILL_FLAG, "Y").with(tags::NEW_SEQ_NO, 2u64);
        assert_eq!(h.write(&resend(gap_fill, 1)), Err("3 resend"));
    }

    fn deliver(list: &[(&str, u64, bool)]) -> Result<(), &'static str> {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        h.send(order("b"), 2).unwrap();
        let deliveries: Vec<_> = list
            .iter()
            .map(|(id, seq, redelivered)| Delivery { id: (*id).into(), seq: *seq, redelivered: *redelivered })
            .collect();
        h.checker.delivered(Side::Acceptor, &deliveries).map_err(|e| e.rule)
    }

    #[test]
    fn a_delivery_that_skips_a_stored_message_breaks_rule_5() {
        assert_eq!(deliver(&[("b", 2, false)]), Err("5 lost"));
    }

    #[test]
    fn a_store_that_reopens_with_other_numbers_breaks_rule_6() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        h.ledger.push(Stored::Opened { next_outgoing: 2, next_incoming: 1 });
        assert_eq!(h.checker.stored(Side::Initiator, &h.ledger).map_err(|e| e.rule), Ok(()));
        h.ledger.push(Stored::Opened { next_outgoing: 1, next_incoming: 1 });
        assert_eq!(h.checker.stored(Side::Initiator, &h.ledger).map_err(|e| e.rule), Err("6 store"));
    }

    #[test]
    fn deliveries_must_match_what_was_sent_in_order_once() {
        assert_eq!(deliver(&[("a", 1, false), ("b", 2, false)]), Ok(()));
        assert_eq!(deliver(&[("a", 1, false), ("a", 1, true)]), Ok(()), "a marked redelivery");
        assert_eq!(deliver(&[("x", 1, false)]), Err("4 delivery"), "not what was sent as 1");
        assert_eq!(deliver(&[("a", 1, false), ("a", 1, false)]), Err("4 delivery"), "repeated unmarked");
        // Out of order: "b" first already passes "a", so rule 5 catches it before rule 4 would.
        assert_eq!(deliver(&[("b", 2, false), ("a", 1, false)]), Err("5 lost"), "out of order");
    }
}
