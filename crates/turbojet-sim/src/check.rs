//! The invariants, checked against what each side writes and what each application receives.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use turbojet::codec::{Decoded, decode};
use turbojet::fields::SessionRejectReason;
use turbojet::message::tags;
use turbojet::{CancelOnDisconnect, CancelTrigger, Disconnect, Message, MsgType};

use crate::Side;
use crate::app::{Delivery, Lifecycle, id_of};
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
#[derive(Clone)]
struct Sent {
    /// The MsgSeqNum the store must record next.
    next_recorded: u64,
    /// The next incoming MsgSeqNum the store last recorded.
    incoming: u64,
    /// Every MsgSeqNum recorded: an application message with the message, a session one `None`,
    /// as is one the store has since evicted.
    recorded: BTreeMap<u64, Option<Message>>,
    /// Application messages the store evicted, for checking deliveries of them made before.
    evicted: BTreeMap<u64, Message>,
    /// The last new (not PossDup) MsgSeqNum written.
    last_new: u64,
    /// Ledger entries already checked.
    ledger_seen: usize,
    /// Changes that may or may not have taken effect (a commit a power loss tore, or changes never
    /// committed), until the store's next open shows what became of them.
    uncertain: Vec<Stored>,
    /// The operators' skips under way when those changes were made, which they may include.
    uncertain_skips: BTreeSet<u64>,
    /// What was recorded in the epoch before this one, for deliveries checked only after a reset
    /// (a delivery and the reset that follows it can come in one step).
    previous: BTreeMap<u64, Option<Message>>,
    /// Operators are moving the next outgoing number forward to these: the store may record the
    /// number before one as used, skipping those between. Several can be under way at once.
    skip_to: BTreeSet<u64>,
    /// MsgSeqNums the counterparty rejected for SendingTime accuracy: delayed past its
    /// `max_latency` (a long stall), they're rejected, not delivered, as the spec says.
    rejected_late: BTreeSet<u64>,
}

impl Sent {
    /// Rule 2 on one change the store recorded: each MsgSeqNum once, in order.
    fn apply(&mut self, side: Side, change: &Stored) -> Result<(), Violation> {
        match change {
            // An operator's skip under way may still land after a reset.
            Stored::Reset => {
                let skip_to = std::mem::take(&mut self.skip_to);
                let previous = std::mem::take(&mut self.recorded);
                *self = Sent { ledger_seen: self.ledger_seen, skip_to, previous, ..Sent::default() };
            }
            Stored::Incoming { seq } => self.incoming = *seq,
            // Its messages need no longer be resent or delivered.
            Stored::Evicted { through } => {
                for (seq, msg) in self.recorded.range_mut(..=*through) {
                    if let Some(msg) = msg.take() {
                        self.evicted.insert(*seq, msg);
                    }
                }
            }
            Stored::Sent { seq, bytes } => {
                // An operator's skip: the numbers before `to` are used, without messages.
                if bytes.is_none() && self.skip_to.contains(&(seq + 1)) && *seq >= self.next_recorded {
                    for skipped in self.next_recorded..=*seq {
                        self.recorded.insert(skipped, None);
                    }
                    self.next_recorded = seq + 1;
                    return Ok(());
                }
                if *seq != self.next_recorded {
                    return Err(violation(
                        "2 sequence",
                        format!("{side:?} stored {seq}, expected {}", self.next_recorded),
                    ));
                }
                let msg = match bytes {
                    Some(bytes) => Some(
                        parse(bytes)
                            .ok_or_else(|| violation("1 valid output", format!("{side:?} stored garbage as {seq}")))?,
                    ),
                    None => None,
                };
                self.recorded.insert(*seq, msg);
                self.next_recorded = seq + 1;
            }
            Stored::Opened { .. } | Stored::OpenFailed(_) | Stored::Uncertain(_) | Stored::Log(_) => {
                unreachable!("handled by stored")
            }
        }
        Ok(())
    }
}

impl Sent {
    /// What of `changes` took effect, as a store that opens with `opened` (next outgoing, next
    /// incoming) shows, if that's possible: the messages before the outgoing number it opens with,
    /// in order, and an incoming number it had before them or one of them recorded. A disk store
    /// writes a commit's messages before its numbers, and recovers the outgoing number from the
    /// messages; a memory store keeps every change.
    fn settle(&self, side: Side, changes: &[Stored], skips: &BTreeSet<u64>, opened: (u64, u64)) -> Option<Sent> {
        let mut after = self.clone();
        after.skip_to.extend(skips.iter().copied());
        for change in changes {
            match change {
                Stored::Sent { seq, .. } if *seq >= opened.0 => break,
                Stored::Sent { .. } => after.apply(side, change).ok()?,
                Stored::Incoming { .. } => {}
                // A reset is written at once, and the ledger has it as certain.
                _ => return None,
            }
        }
        let incoming_recorded = changes.iter().any(|c| matches!(c, Stored::Incoming { seq } if *seq == opened.1));
        if after.next_recorded != opened.0 || (opened.1 != self.incoming && !incoming_recorded) {
            return None;
        }
        after.incoming = opened.1;
        after.skip_to.clone_from(&self.skip_to);
        Some(after)
    }
}

impl Default for Sent {
    fn default() -> Self {
        Self {
            next_recorded: 1,
            incoming: 1,
            recorded: BTreeMap::new(),
            last_new: 0,
            ledger_seen: 0,
            uncertain: Vec::new(),
            evicted: BTreeMap::new(),
            uncertain_skips: BTreeSet::new(),
            skip_to: BTreeSet::new(),
            rejected_late: BTreeSet::new(),
            previous: BTreeMap::new(),
        }
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

/// One side's cancel on disconnect, as its application saw it.
#[derive(Default)]
struct Cancels {
    config: Option<CancelOnDisconnect>,
    /// Lifecycle events already checked.
    seen: usize,
    /// The countdown under way: when it ends, and the ending that started it.
    pending: Option<(SimTime, Disconnect)>,
    /// An ending with no grace period: the session cancels as it ends, so the next event is the
    /// cancel.
    cancel_now: bool,
}

impl Cancels {
    /// Rule 8 on one lifecycle event at `at`.
    fn apply(&mut self, side: Side, at: SimTime, event: Lifecycle) -> Result<(), Violation> {
        if self.cancel_now && !matches!(event, Lifecycle::Cancel(_)) {
            return Err(violation("8 cancel", format!("{side:?} ended with no grace period, then {event:?} at {at}")));
        }
        match event {
            Lifecycle::LoggedOn => {
                // A logon at the very end of the grace period may beat the cancel or not.
                if let Some((deadline, ended)) = self.pending.take()
                    && deadline < at
                {
                    return Err(violation(
                        "8 cancel",
                        format!("{side:?} logged on at {at} with no cancel for {ended:?}, due at {deadline}"),
                    ));
                }
            }
            Lifecycle::LoggedOut(ended) => {
                // Only a session that logged on ends, and its logon stopped any countdown before.
                if let Some((deadline, first)) = self.pending {
                    return Err(violation(
                        "8 cancel",
                        format!(
                            "{side:?} ended with {ended:?} at {at} without a logon since it ended with {first:?}, \
                             its cancel due at {deadline}"
                        ),
                    ));
                }
                if let Some(CancelOnDisconnect { trigger, grace, .. }) = self.config
                    && counts(trigger, ended)
                {
                    self.pending = Some((at.after(grace), ended));
                    self.cancel_now = grace.is_zero();
                }
            }
            Lifecycle::Cancel(ended) => {
                self.cancel_now = false;
                match self.pending.take() {
                    Some(expected) if expected == (at, ended) => {}
                    Some((deadline, expected)) => {
                        return Err(violation(
                            "8 cancel",
                            format!(
                                "{side:?} cancelled at {at} for {ended:?}; expected at {deadline} for {expected:?}"
                            ),
                        ));
                    }
                    None => {
                        return Err(violation(
                            "8 cancel",
                            format!("{side:?} cancelled at {at} for {ended:?} with no countdown under way"),
                        ));
                    }
                }
            }
            // The countdown went with the process's registry.
            Lifecycle::Crashed => self.pending = None,
        }
        Ok(())
    }
}

/// Which endings `trigger` counts: the checker's own table, not the engine's.
fn counts(trigger: CancelTrigger, ended: Disconnect) -> bool {
    match ended {
        Disconnect::ConnectionLost | Disconnect::HeartbeatTimeout | Disconnect::Error => true,
        Disconnect::Logout | Disconnect::CounterpartyLogout => trigger == CancelTrigger::DisconnectOrLogout,
        // Shutdown, and any ending added later: a cancel for one is a violation, to look into.
        _ => false,
    }
}

#[derive(Default)]
pub struct Checker {
    sent: [Sent; 2],
    received: [Received; 2],
    cancels: [Cancels; 2],
    lossy: bool,
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
    /// A checker for sides with these cancel-on-disconnect settings.
    pub fn new(cancels: [Option<CancelOnDisconnect>; 2]) -> Self {
        Self { cancels: cancels.map(|config| Cancels { config, ..Cancels::default() }), ..Self::default() }
    }

    /// Rule 8, on `side`'s lifecycle events since the last call, at `now`: a cancel on disconnect
    /// for each ending its trigger counts, unless the session logs on within the grace period,
    /// at the end of it; none otherwise. The first ending's countdown runs on until a logon, and
    /// a crash takes it.
    pub fn lifecycle(&mut self, side: Side, events: &[(SimTime, Lifecycle)], now: SimTime) -> Result<(), Violation> {
        let cancels = &mut self.cancels[side.index()];
        for (at, event) in &events[cancels.seen..] {
            cancels.apply(side, *at, *event)?;
            cancels.seen += 1;
        }
        // A session with no grace period cancels in the same call that ends it.
        if cancels.cancel_now {
            return Err(violation(
                "8 cancel",
                format!("{side:?} ended with no grace period, and didn't cancel at once"),
            ));
        }
        match cancels.pending {
            Some((deadline, ended)) if deadline < now => {
                Err(violation("8 cancel", format!("{side:?} had no cancel for {ended:?} by {now}, due at {deadline}")))
            }
            _ => Ok(()),
        }
    }

    /// Rule 8 as a run ends: every countdown has run out, with a cancel or a logon.
    pub fn no_countdowns(&self) -> Result<(), Violation> {
        for side in [Side::Initiator, Side::Acceptor] {
            if let Some((deadline, ended)) = self.cancels[side.index()].pending {
                return Err(violation(
                    "8 cancel",
                    format!("{side:?}'s countdown for {ended:?}, due at {deadline}, still under way as the run ends"),
                ));
            }
        }
        Ok(())
    }

    /// The MsgSeqNum `side` stored application message `id` as, in this epoch.
    pub fn seq_of(&self, side: Side, id: &str) -> Option<u64> {
        self.sent[side.index()]
            .recorded
            .iter()
            .find(|(_, m)| m.as_ref().and_then(id_of) == Some(id))
            .map(|(seq, _)| *seq)
    }

    /// Rule 7: a receipt tells the truth. Stored as `seq` means `side`'s store recorded message
    /// `id` as `seq` (in this epoch or the one before); dropped means it never recorded it.
    pub fn receipt(&self, side: Side, id: &str, outcome: &Result<u64, turbojet::Dropped>) -> Result<(), Violation> {
        if self.lossy {
            return Ok(());
        }
        let sent = &self.sent[side.index()];
        let epochs = [&sent.recorded, &sent.previous];
        // Stored, then evicted by the time the receipt is checked, counts as stored.
        let evicted_as = |seq: u64| sent.evicted.get(&seq).and_then(id_of) == Some(id);
        let stored_as = |seq: u64| {
            evicted_as(seq) || epochs.iter().any(|r| r.get(&seq).and_then(Option::as_ref).and_then(id_of) == Some(id))
        };
        // A scan of everything stored: only for a send said to be dropped, which is rare.
        let ever_stored = || {
            sent.evicted.values().any(|m| id_of(m) == Some(id))
                || epochs.iter().any(|r| r.values().flatten().any(|m| id_of(m) == Some(id)))
        };
        match outcome {
            Ok(seq) if stored_as(*seq) => Ok(()),
            Ok(seq) => {
                Err(violation("7 receipt", format!("{side:?}'s receipt says {id} was stored as {seq}; it wasn't")))
            }
            // A failed store call may have taken effect: the receipt says so.
            Err(turbojet::Dropped::Storage) => Ok(()),
            Err(_) if !ever_stored() => Ok(()),
            Err(dropped) => Err(violation(
                "7 receipt",
                format!("{side:?}'s receipt says {id} was dropped ({dropped}), but it was stored"),
            )),
        }
    }

    /// Whether the other side rejected `side`'s `seq`, of this epoch, for SendingTime accuracy.
    pub fn rejected_late(&self, side: Side, seq: u64) -> bool {
        self.sent[side.index()].rejected_late.contains(&seq)
    }

    /// Application messages `side` has committed to sending, by id.
    pub fn committed(&self, side: Side) -> impl Iterator<Item = &str> {
        self.sent[side.index()].recorded.values().flatten().filter_map(id_of)
    }

    /// Rules 2 and 6 on what `side`'s store recorded since the last call: each MsgSeqNum once, in
    /// order, and a store that opens with the numbers it last recorded.
    pub fn stored(&mut self, side: Side, ledger: &[Stored]) -> Result<(), Violation> {
        let lossy = self.lossy;
        let sent = &mut self.sent[side.index()];
        for entry in &ledger[sent.ledger_seen..] {
            match entry {
                Stored::Uncertain(changes) => {
                    sent.uncertain.extend(changes.iter().cloned());
                    sent.uncertain_skips.extend(sent.skip_to.iter().copied());
                }
                Stored::OpenFailed(e) => return Err(violation("6 store", format!("{side:?}'s store won't open: {e}"))),
                // A FIX node keeps one session's log.
                Stored::Log(_) => {}
                Stored::Opened { next_outgoing, next_incoming } => {
                    let opened = (*next_outgoing, *next_incoming);
                    let uncertain = std::mem::take(&mut sent.uncertain);
                    let uncertain_skips = std::mem::take(&mut sent.uncertain_skips);
                    if lossy {
                        // What was lost is lost: carry on from what the store has.
                        sent.recorded.retain(|seq, _| *seq < opened.0);
                        (sent.next_recorded, sent.incoming) = opened;
                    } else if opened != (sent.next_recorded, sent.incoming) {
                        // Rule 6: a store reopens with what it recorded, crash or not; changes it
                        // wasn't sure of took effect or didn't.
                        let applied = sent.settle(side, &uncertain, &uncertain_skips, opened);
                        let Some(applied) = applied else {
                            return Err(violation(
                                "6 store",
                                format!(
                                    "{side:?}'s store opened at {} out, {} in; it recorded {} out, {} in",
                                    opened.0, opened.1, sent.next_recorded, sent.incoming
                                ),
                            ));
                        };
                        *sent = applied;
                    }
                }
                change => {
                    if matches!(change, Stored::Reset) {
                        // Both numbers start again at 1: so do the deliveries this side receives.
                        self.received[side.index()].last_seq = 0;
                    }
                    sent.apply(side, change).or_else(|e| if lossy { Ok(()) } else { Err(e) })?;
                }
            }
            sent.ledger_seen += 1;
        }
        Ok(())
    }

    /// An operator is about to move `side`'s next outgoing number forward to `to`.
    pub fn expect_skip(&mut self, side: Side, to: u64) {
        self.sent[side.index()].skip_to.insert(to);
    }

    /// An operator's skip to `to` has been answered, made or not.
    pub fn skip_done(&mut self, side: Side, to: u64) {
        self.sent[side.index()].skip_to.remove(&to);
    }

    /// A power loss on a store without fsync has lost what the OS hadn't written back: from now
    /// on only rule 1 holds, and the sessions needn't settle.
    pub fn lose(&mut self) {
        self.lossy = true;
    }

    pub fn is_lossy(&self) -> bool {
        self.lossy
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
            if !self.lossy {
                self.sent_message(side, &msg)?;
            }
        }
        Ok(())
    }

    fn sent_message(&mut self, side: Side, msg: &Message) -> Result<(), Violation> {
        if msg.msg_type() == MsgType::Reject
            && msg.get(tags::SESSION_REJECT_REASON) == Some(SessionRejectReason::SendingTimeAccuracyProblem.code())
        {
            self.sent[side.other().index()].rejected_late.insert(number(msg, tags::REF_SEQ_NUM));
        }
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
        // One the store has since evicted may have been read for this resend before it went.
        let Some(first) = recorded.as_ref().or_else(|| sent.evicted.get(&seq)) else {
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
        if self.lossy {
            self.received[side.index()].seen = deliveries.len();
            return Ok(());
        }
        let sender = &self.sent[side.other().index()];
        let received = &mut self.received[side.index()];
        for delivery in &deliveries[received.seen..] {
            let in_epoch = |recorded: &BTreeMap<u64, Option<Message>>| {
                recorded.get(&delivery.seq).and_then(Option::as_ref).and_then(id_of).map(str::to_string)
            };
            let mut expected = in_epoch(&sender.recorded);
            if expected.is_none() {
                // Delivered before the store evicted it.
                expected = sender.evicted.get(&delivery.seq).and_then(id_of).map(str::to_string);
            }
            if expected.as_deref() != Some(delivery.id.as_str())
                && in_epoch(&sender.previous).as_deref() == Some(&delivery.id)
            {
                // Sent before a reset the checker has already seen, and delivered just before it.
                expected = in_epoch(&sender.previous);
            }
            if expected.as_deref() != Some(delivery.id.as_str()) {
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
            // passes a stored message means it's lost, unless it was rejected for arriving late.
            if !delivery.redelivered
                && let Some((lost, _)) = sender.recorded.range(received.last_seq + 1..delivery.seq).find(|(seq, m)| {
                    !sender.rejected_late.contains(seq)
                        && m.as_ref().and_then(id_of).is_some_and(|id| !received.ids.contains_key(id))
                })
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
    fn a_delivery_past_a_message_rejected_for_arriving_late_is_fine() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        h.send(order("b"), 2).unwrap();
        let late = Message::new(MsgType::Reject)
            .with(tags::REF_SEQ_NUM, 1u64)
            .with(tags::SESSION_REJECT_REASON, SessionRejectReason::SendingTimeAccuracyProblem.code());
        h.checker.stored(Side::Acceptor, &[Stored::Sent { seq: 1, bytes: None }]).unwrap();
        h.checker.written(Side::Acceptor, &framed(late, 1, &[]), SimTime(0)).unwrap();
        let b = Delivery { id: "b".into(), seq: 2, redelivered: false };
        assert_eq!(h.checker.delivered(Side::Acceptor, &[b]).map_err(|e| e.rule), Ok(()));
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
    fn a_receipt_that_misstates_what_was_stored_breaks_rule_7() {
        let mut h = Harness::default();
        h.send(order("a"), 1).unwrap();
        let check = |outcome| h.checker.receipt(Side::Initiator, "a", &outcome).map_err(|e| e.rule);
        assert_eq!(check(Ok(1)), Ok(()));
        assert_eq!(check(Ok(2)), Err("7 receipt"), "stored as 1, not 2");
        assert_eq!(check(Err(turbojet::Dropped::Disconnected)), Err("7 receipt"), "it was stored");
        assert_eq!(check(Err(turbojet::Dropped::Storage)), Ok(()), "a failed store may have stored it");
        let check_b = |outcome| h.checker.receipt(Side::Initiator, "b", &outcome).map_err(|e| e.rule);
        assert_eq!(check_b(Err(turbojet::Dropped::LoggingOut)), Ok(()), "never stored");
    }

    /// Runs `events` (seconds, event) past a checker whose acceptor cancels on `trigger` after
    /// `grace` seconds, then checks at `now` seconds.
    fn cancels(trigger: CancelTrigger, grace: u64, events: &[(u64, Lifecycle)], now: u64) -> Result<(), &'static str> {
        let config = CancelOnDisconnect::new(trigger, std::time::Duration::from_secs(grace));
        let mut checker = Checker::new([None, Some(config)]);
        let at = |secs: u64| SimTime::from_duration(std::time::Duration::from_secs(secs));
        let events: Vec<_> = events.iter().map(|(secs, event)| (at(*secs), *event)).collect();
        checker.lifecycle(Side::Acceptor, &events, at(now)).map_err(|e| e.rule)
    }

    #[test]
    fn a_cancel_comes_at_the_end_of_the_grace_period_unless_the_session_logs_on_breaking_rule_8() {
        use CancelTrigger::{Disconnect as OnDisconnect, DisconnectOrLogout};
        use Disconnect::{ConnectionLost, CounterpartyLogout, Error, HeartbeatTimeout, Logout, Shutdown};
        use Lifecycle::{Cancel, Crashed, LoggedOn, LoggedOut};
        let check = |trigger, events: &[(u64, Lifecycle)], now| cancels(trigger, 5, events, now);
        let lost = [(0, LoggedOn), (10, LoggedOut(ConnectionLost))];
        assert_eq!(check(OnDisconnect, &[lost[0], lost[1], (15, Cancel(ConnectionLost))], 20), Ok(()));
        assert_eq!(check(OnDisconnect, &lost, 15), Ok(()), "due now");
        assert_eq!(check(OnDisconnect, &lost, 16), Err("8 cancel"), "missed");
        assert_eq!(check(OnDisconnect, &[lost[0], lost[1], (16, Cancel(ConnectionLost))], 16), Err("8 cancel"));
        assert_eq!(check(OnDisconnect, &[lost[0], lost[1], (15, LoggedOn)], 30), Ok(()), "logged on in time");
        assert_eq!(check(OnDisconnect, &[lost[0], lost[1], (16, LoggedOn)], 16), Err("8 cancel"));
        assert_eq!(check(OnDisconnect, &[lost[0], lost[1], (12, Crashed)], 30), Ok(()), "lost in a crash");
        let again = [lost[0], lost[1], (12, LoggedOut(ConnectionLost))];
        assert_eq!(check(OnDisconnect, &again, 12), Err("8 cancel"), "ended twice without a logon");
        for ended in [HeartbeatTimeout, Error] {
            assert_eq!(check(OnDisconnect, &[(0, LoggedOn), (10, LoggedOut(ended)), (15, Cancel(ended))], 20), Ok(()));
            assert_eq!(check(OnDisconnect, &[(0, LoggedOn), (10, LoggedOut(ended))], 20), Err("8 cancel"));
        }
        let logout = [(0, LoggedOn), (10, LoggedOut(Logout))];
        assert_eq!(check(OnDisconnect, &logout, 30), Ok(()), "a logout doesn't count");
        assert_eq!(check(OnDisconnect, &[logout[0], logout[1], (15, Cancel(Logout))], 30), Err("8 cancel"));
        assert_eq!(check(DisconnectOrLogout, &logout, 30), Err("8 cancel"), "it does now");
        assert_eq!(
            check(DisconnectOrLogout, &[logout[0], logout[1], (15, Cancel(ConnectionLost))], 30),
            Err("8 cancel")
        );
        let theirs = [(0, LoggedOn), (10, LoggedOut(CounterpartyLogout)), (15, Cancel(CounterpartyLogout))];
        assert_eq!(check(DisconnectOrLogout, &theirs, 30), Ok(()));
        assert_eq!(check(OnDisconnect, &theirs, 30), Err("8 cancel"));
        assert_eq!(check(DisconnectOrLogout, &[(0, LoggedOn), (10, LoggedOut(Shutdown))], 30), Ok(()), "ours");
    }

    /// With no grace period, the cancel is the very next thing the application hears.
    #[test]
    fn with_no_grace_period_the_cancel_comes_at_once_or_rule_8_breaks() {
        use Disconnect::ConnectionLost;
        use Lifecycle::{Cancel, LoggedOn, LoggedOut};
        let check = |events: &[(u64, Lifecycle)], now| cancels(CancelTrigger::Disconnect, 0, events, now);
        let lost = [(0, LoggedOn), (10, LoggedOut(ConnectionLost))];
        assert_eq!(check(&[lost[0], lost[1], (10, Cancel(ConnectionLost)), (10, LoggedOn)], 10), Ok(()));
        assert_eq!(check(&lost, 10), Err("8 cancel"), "not at once");
        assert_eq!(check(&[lost[0], lost[1], (10, LoggedOn)], 10), Err("8 cancel"), "a logon first");
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
