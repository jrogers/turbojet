//! The FIXP invariants, checked against what each side's store records, what each side writes
//! and what each application receives. See the crate README for the rules.

use std::collections::{BTreeMap, BTreeSet};

use turbojet::Dropped;
use turbojet::fixp::{Ended, FlowType};

use super::app::Delivery;
use super::wire::{self, Frame};
use crate::Side;
use crate::check::Violation;
use crate::net::ConnId;
use crate::store::Stored;

fn violation(rule: &'static str, detail: impl Into<String>) -> Violation {
    Violation { rule, detail: detail.into() }
}

/// The reason a session gives in the `Terminate` it sends when the counterparty goes quiet: the
/// one `UnspecifiedError` that isn't a protocol error.
const KEEPALIVE_LAPSED: &[u8] = b"keepalive interval lapsed";
/// EstablishmentRejectCode Unnegotiated.
const UNNEGOTIATED: u8 = 0;

fn sequenced(flow: FlowType) -> bool {
    matches!(flow, FlowType::Recoverable | FlowType::Idempotent)
}

/// What one side has committed to sending on its flow, from its store, and what of it has been on
/// the wire.
#[derive(Clone, Default)]
struct Sent {
    /// The store has opened at least once: its numbers are known.
    opened: bool,
    /// The sequence number the store must record next, and the next incoming it last recorded.
    next_recorded: u64,
    incoming: u64,
    /// Every number recorded, with the frame stored for it (a recoverable flow).
    recorded: BTreeMap<u64, Option<Vec<u8>>>,
    /// The order each number carries, as stored or as first written.
    ids: BTreeMap<u64, u64>,
    /// Numbers the session took for its own Applied and NotApplied.
    session_owned: BTreeSet<u64>,
    /// Receipts' numbers, for orders not yet seen with them.
    receipts: BTreeMap<u64, u64>,
    /// The last number written live.
    last_live: u64,
    /// Orders written on an unsequenced flow, in order.
    unsequenced: Vec<u64>,
    /// NotApplied written, about the other side's flow: first and count.
    not_applied: Vec<(u64, u64)>,
    ledger_seen: usize,
    /// Changes that may or may not have taken effect, until the store's next open shows.
    uncertain: Vec<Stored>,
}

impl Sent {
    /// Rule 4 on one change the store recorded: each number once, in order.
    fn apply(&mut self, side: Side, change: &Stored) -> Result<(), Violation> {
        match change {
            // A new session: both numbers start again at 1.
            Stored::Reset => {
                *self = Sent {
                    ledger_seen: self.ledger_seen,
                    opened: true,
                    next_recorded: 1,
                    incoming: 1,
                    ..Sent::default()
                };
            }
            Stored::Incoming { seq } => self.incoming = *seq,
            Stored::Evicted { .. } => {}
            Stored::Sent { seq, bytes } => {
                if *seq != self.next_recorded {
                    return Err(violation(
                        "4 sequence",
                        format!("{side:?} stored {seq}, expected {}", self.next_recorded),
                    ));
                }
                if let Some(frame) = bytes {
                    match wire::decode(frame) {
                        Ok(Frame::Order(id)) => self.learn(side, *seq, id)?,
                        Ok(f) if f.is_application() => {
                            self.session_owned.insert(*seq);
                        }
                        other => return Err(violation("1 frames", format!("{side:?} stored {other:?} as {seq}"))),
                    }
                }
                self.recorded.insert(*seq, bytes.clone());
                self.next_recorded = seq + 1;
            }
            Stored::Opened { .. } | Stored::OpenFailed(_) | Stored::Uncertain(_) => unreachable!("handled by stored"),
        }
        Ok(())
    }

    /// Order `id` goes with number `seq`: agreeing with what was stored, written or receipted.
    fn learn(&mut self, side: Side, seq: u64, id: u64) -> Result<(), Violation> {
        let known = self.ids.get(&seq).or_else(|| self.receipts.get(&seq));
        if let Some(&other) = known.filter(|&&other| other != id) {
            return Err(violation(
                "4 sequence",
                format!("{side:?} sent order {id} as {seq}, which is order {other}'s"),
            ));
        }
        self.ids.insert(seq, id);
        Ok(())
    }

    /// What of `changes` took effect, as a store that opens with `opened` (next outgoing, next
    /// incoming) shows, if that's possible: the numbers before the outgoing one it opens with, in
    /// order, and an incoming number it had before them or one of them recorded.
    fn settle(&self, side: Side, changes: &[Stored], opened: (u64, u64)) -> Option<Sent> {
        let mut after = self.clone();
        let mut incoming = vec![self.incoming];
        for change in changes {
            match change {
                Stored::Sent { seq, .. } if *seq >= opened.0 => break,
                Stored::Incoming { seq } => incoming.push(*seq),
                _ => {}
            }
            after.apply(side, change).ok()?;
        }
        if after.next_recorded != opened.0 || !incoming.contains(&opened.1) {
            return None;
        }
        after.incoming = opened.1;
        Some(after)
    }
}

/// What one side's application has received from the other's flow.
#[derive(Default)]
struct Received {
    seen: usize,
    /// Numbers delivered, whether retransmitted, and the last delivered.
    delivered: BTreeMap<u64, bool>,
    last: u64,
    /// Unsequenced: the index in the sender's `unsequenced` after the last delivered.
    unsequenced_next: usize,
}

/// One side's writing on one connection: the number its next live application message takes,
/// and a retransmission under way (the next number, and how many are left).
#[derive(Default)]
struct Wire {
    next: Option<u64>,
    replay: Option<(u64, u64)>,
}

pub struct Checker {
    /// Each side's own flow.
    flows: [FlowType; 2],
    sent: [Sent; 2],
    received: [Received; 2],
    wires: BTreeMap<(Side, ConnId), Wire>,
    /// Retransmissions each side has asked for on a connection, not yet answered: first and count.
    asked: BTreeMap<(Side, ConnId), Vec<(u64, u64)>>,
    /// Connections the server has written an EstablishmentAck on.
    established: BTreeSet<ConnId>,
    /// The session the client negotiated, and whether it may negotiate another (its log reset).
    client_session: Option<[u8; 16]>,
    may_negotiate: bool,
    /// The server has answered the client's Negotiate.
    negotiated: bool,
    /// The client has negotiated a new session: the server's next open is of that session's log.
    server_log_new: bool,
    ended_seen: [usize; 2],
    /// Retransmitted application messages written, for the report.
    pub resent: u64,
}

impl Checker {
    /// Checks a client and server whose flows are `flows` (client's, server's).
    pub fn new(flows: [FlowType; 2]) -> Self {
        Self {
            flows,
            sent: Default::default(),
            received: Default::default(),
            wires: BTreeMap::new(),
            asked: BTreeMap::new(),
            established: BTreeSet::new(),
            client_session: None,
            may_negotiate: true,
            negotiated: false,
            server_log_new: false,
            ended_seen: [0; 2],
            resent: 0,
        }
    }

    /// Rules 4 and 9 on what `side`'s store recorded since the last call.
    pub fn stored(&mut self, side: Side, ledger: &[Stored]) -> Result<(), Violation> {
        let sent = &mut self.sent[side.index()];
        for entry in &ledger[sent.ledger_seen..] {
            match entry {
                Stored::Uncertain(changes) => sent.uncertain.extend(changes.iter().cloned()),
                Stored::OpenFailed(e) => return Err(violation("9 store", format!("{side:?}'s store won't open: {e}"))),
                Stored::Opened { next_outgoing, next_incoming } => {
                    let opened = (*next_outgoing, *next_incoming);
                    let uncertain = std::mem::take(&mut sent.uncertain);
                    if side == Side::Acceptor && std::mem::take(&mut self.server_log_new) {
                        // Another session's log: both directions start again.
                        *sent = Sent { ledger_seen: sent.ledger_seen, ..Sent::default() };
                        let seen = self.received[side.index()].seen;
                        self.received[side.index()] = Received { seen, ..Received::default() };
                    }
                    if !sent.opened {
                        sent.opened = true;
                        (sent.next_recorded, sent.incoming) = opened;
                    } else if opened != (sent.next_recorded, sent.incoming) {
                        let Some(applied) = sent.settle(side, &uncertain, opened) else {
                            return Err(violation(
                                "9 store",
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
                        if side == Side::Initiator {
                            self.may_negotiate = true;
                        }
                        // A new session: what this side receives starts again too.
                        self.received[side.index()] =
                            Received { seen: self.received[side.index()].seen, ..Received::default() };
                    }
                    sent.apply(side, change)?;
                }
            }
            sent.ledger_seen += 1;
        }
        Ok(())
    }

    /// Rules 1-4, on what `side` wrote on `conn`, after what its store recorded.
    pub fn written(&mut self, side: Side, conn: ConnId, bytes: &[u8]) -> Result<(), Violation> {
        let frames = wire::frames(bytes).map_err(|e| violation("1 frames", format!("{side:?} wrote {e}")))?;
        for frame in frames {
            let decoded = wire::decode(frame).map_err(|e| violation("1 frames", format!("{side:?} wrote {e}")))?;
            if decoded.is_application() {
                self.application(side, conn, frame, &decoded)?;
            } else {
                self.session_message(side, conn, decoded)?;
            }
        }
        Ok(())
    }

    /// Rules 2 and 3, and the numbering a session message sets.
    fn session_message(&mut self, side: Side, conn: ConnId, frame: Frame) -> Result<(), Violation> {
        match frame {
            Frame::Negotiate { session_id } => {
                if !self.may_negotiate {
                    return Err(violation("2 handshake", "the client negotiated again, its session not finished"));
                }
                self.may_negotiate = false;
                self.negotiated = false;
                self.server_log_new = true;
                self.client_session = Some(session_id);
            }
            Frame::NegotiationResponse => self.negotiated = true,
            // A connection lost before the server committed the client's Negotiate: the client
            // establishes, is told it never negotiated, and negotiates another session.
            Frame::EstablishmentReject { code: UNNEGOTIATED } if !self.negotiated => {}
            Frame::Establish { session_id, .. } if self.client_session != Some(session_id) => {
                return Err(violation("2 handshake", "the client established a session it never negotiated"));
            }
            Frame::EstablishmentAck { .. } => {
                self.established.insert(conn);
            }
            Frame::NegotiationReject | Frame::EstablishmentReject { .. } | Frame::RetransmitReject => {
                return Err(violation("3 protocol", format!("{side:?} wrote {frame:?}")));
            }
            // The answer to a Terminate echoes its code, without a reason.
            Frame::Terminate { code, ref reason }
                if code >= 2 || code == 1 && !reason.is_empty() && reason != KEEPALIVE_LAPSED =>
            {
                let reason = String::from_utf8_lossy(reason);
                return Err(violation("3 protocol", format!("{side:?} terminated with code {code}: {reason}")));
            }
            Frame::Sequence { next } => {
                let sent = &self.sent[side.index()];
                if next > sent.next_recorded {
                    return Err(violation(
                        "4 sequence",
                        format!("{side:?} wrote Sequence {next}, but only recorded up to {}", sent.next_recorded),
                    ));
                }
                let wire = self.wires.entry((side, conn)).or_default();
                wire.next = Some(next);
                wire.replay = None;
            }
            Frame::RetransmitRequest { from, count } => {
                self.asked.entry((side, conn)).or_default().push((from, u64::from(count)));
            }
            Frame::Retransmission { next, count } => {
                let count = u64::from(count);
                let asked = self.asked.entry((side.other(), conn)).or_default();
                let within = asked.iter().position(|&(from, n)| next >= from && next + count <= from + n);
                let Some(i) = within else {
                    return Err(violation(
                        "4 sequence",
                        format!("{side:?} retransmitted {next} +{count}, never asked for"),
                    ));
                };
                asked.remove(i);
                self.wires.entry((side, conn)).or_default().replay = (count > 0).then_some((next, count));
            }
            _ => {}
        }
        Ok(())
    }

    /// Rules 2 and 4 on an application message `side` wrote.
    fn application(&mut self, side: Side, conn: ConnId, frame: &[u8], decoded: &Frame) -> Result<(), Violation> {
        if !self.established.contains(&conn) {
            return Err(violation(
                "2 handshake",
                format!("{side:?} wrote {decoded:?} before the session was established"),
            ));
        }
        let flow = self.flows[side.index()];
        let id = match decoded {
            Frame::Order(id) => Some(*id),
            _ => None,
        };
        if let Frame::NotApplied { from, count } = decoded {
            self.sent[side.index()].not_applied.push((*from, u64::from(*count)));
        }
        if !sequenced(flow) {
            return match (flow, id) {
                (FlowType::Unsequenced, Some(id)) => {
                    self.sent[side.index()].unsequenced.push(id);
                    Ok(())
                }
                // The session's own Applied and NotApplied go on whatever flow it has.
                (FlowType::Unsequenced, None) => Ok(()),
                _ => Err(violation("4 sequence", format!("{side:?} wrote {decoded:?} on a {flow:?} flow"))),
            };
        }
        let wire = self.wires.entry((side, conn)).or_default();
        let sent = &mut self.sent[side.index()];
        let (seq, replayed) = if let Some((next, left)) = wire.replay {
            wire.replay = (left > 1).then_some((next + 1, left - 1));
            (next, true)
        } else {
            let Some(next) = wire.next else {
                return Err(violation("4 sequence", format!("{side:?} wrote {decoded:?} before a Sequence")));
            };
            wire.next = Some(next + 1);
            (next, false)
        };
        let Some(stored) = sent.recorded.get(&seq) else {
            return Err(violation("4 sequence", format!("{side:?} wrote {seq}, which its store never recorded")));
        };
        if stored.as_ref().is_some_and(|stored| stored != frame) {
            return Err(violation("4 sequence", format!("{side:?} wrote {seq} unlike it stored it")));
        }
        if replayed {
            if flow != FlowType::Recoverable || stored.is_none() {
                return Err(violation("4 sequence", format!("{side:?} retransmitted {seq}, which it never stored")));
            }
            self.resent += 1;
            return Ok(());
        }
        if seq <= sent.last_live {
            return Err(violation("4 sequence", format!("{side:?} wrote {seq} live after {}", sent.last_live)));
        }
        sent.last_live = seq;
        match id {
            Some(id) => sent.learn(side, seq, id),
            None => {
                sent.session_owned.insert(seq);
                Ok(())
            }
        }
    }

    /// Rules 5 and 7, on what `side`'s application has received since the last call.
    pub fn delivered(&mut self, side: Side, deliveries: &[Delivery]) -> Result<(), Violation> {
        let flow = self.flows[side.other().index()];
        let sender = &self.sent[side.other().index()];
        let received = &mut self.received[side.index()];
        for d in &deliveries[received.seen..] {
            let Some(seq) = d.seq else {
                if flow != FlowType::Unsequenced {
                    return Err(violation(
                        "5 delivery",
                        format!("{side:?} got {} unnumbered on a {flow:?} flow", d.id),
                    ));
                }
                let from = received.unsequenced_next;
                let Some(i) = sender.unsequenced[from..].iter().position(|&id| id == d.id) else {
                    return Err(violation(
                        "7 unsequenced",
                        format!("{side:?} got {} out of order, again or never sent", d.id),
                    ));
                };
                received.unsequenced_next = from + i + 1;
                continue;
            };
            if sender.ids.get(&seq) != Some(&d.id) {
                let sent = sender.ids.get(&seq);
                return Err(violation(
                    "5 delivery",
                    format!("{side:?} got {} as {seq}, but {sent:?} was sent as {seq}", d.id),
                ));
            }
            if let Some(first) = received.delivered.insert(seq, d.retransmitted) {
                let how = |retransmitted| if retransmitted { "retransmitted" } else { "live" };
                return Err(violation(
                    "5 delivery",
                    format!("{side:?} got {} as {seq} again: {} after {}", d.id, how(d.retransmitted), how(first)),
                ));
            }
            if d.retransmitted && flow != FlowType::Recoverable {
                return Err(violation("5 delivery", format!("{side:?} got {seq} retransmitted on a {flow:?} flow")));
            }
            // In order: on a recoverable flow every number, on an idempotent one with gaps the
            // sender hears of.
            if seq <= received.last {
                return Err(violation("5 delivery", format!("{side:?} got {seq} after {}", received.last)));
            }
            // The session takes its own Applied and NotApplied: they never reach the application.
            let skipped = (received.last + 1..seq).find(|n| !sender.session_owned.contains(n));
            if flow == FlowType::Recoverable && skipped.is_some() {
                return Err(violation(
                    "6 lost",
                    format!("{side:?} got {seq} after {}, skipping a recoverable message", received.last),
                ));
            }
            received.last = seq;
        }
        received.seen = deliveries.len();
        Ok(())
    }

    /// Rule 8 on a receipt `side`'s application got for order `id`.
    pub fn receipt(&mut self, side: Side, id: u64, outcome: &Result<u64, Dropped>) -> Result<(), Violation> {
        let flow = self.flows[side.index()];
        let seq = match outcome {
            Ok(seq) => *seq,
            Err(Dropped::Rejected(why)) => {
                return Err(violation("8 receipt", format!("{side:?}'s order {id} was rejected: {why}")));
            }
            Err(_) => return Ok(()),
        };
        let sent = &mut self.sent[side.index()];
        if !sequenced(flow) {
            return if seq == 0 {
                Ok(())
            } else {
                Err(violation("8 receipt", format!("{side:?}'s order {id} got {seq} on a {flow:?} flow")))
            };
        }
        if !sent.recorded.contains_key(&seq) {
            return Err(violation(
                "8 receipt",
                format!("{side:?}'s order {id} got {seq}, which its store never recorded"),
            ));
        }
        if let Some(&other) = sent.ids.get(&seq).filter(|&&other| other != id) {
            return Err(violation("8 receipt", format!("{side:?}'s order {id} got {seq}, order {other}'s")));
        }
        sent.receipts.insert(seq, id);
        Ok(())
    }

    /// Rule 3 on how `side`'s established connections ended.
    pub fn ended(&mut self, side: Side, ended: &[Ended]) -> Result<(), Violation> {
        for how in &ended[self.ended_seen[side.index()]..] {
            if matches!(how, Ended::Error | Ended::NegotiationRejected(_) | Ended::EstablishmentRejected(_)) {
                return Err(violation("3 protocol", format!("{side:?}'s connection ended {how:?}")));
            }
        }
        self.ended_seen[side.index()] = ended.len();
        Ok(())
    }

    /// For settling: what `side` sent and the other side still lacks, given the numbers `side`'s
    /// application heard weren't applied. `None` once everything is accounted for. A NotApplied
    /// on a flow that isn't recoverable may itself be lost: then it's enough that it was written.
    pub fn missing(&self, side: Side, not_applied: &[(u64, u64)]) -> Option<String> {
        let flow = self.flows[side.index()];
        let sent = &self.sent[side.index()];
        let received = &self.received[side.other().index()];
        let covers = |ranges: &[(u64, u64)], seq: u64| ranges.iter().any(|&(from, n)| seq >= from && seq < from + n);
        let written = &self.sent[side.other().index()].not_applied;
        let lossy_reports = self.flows[side.other().index()] != FlowType::Recoverable;
        // With no flow back, a gap can't be reported at all: at most once is all that's promised.
        let silent = self.flows[side.other().index()] == FlowType::None;
        let reported = |seq: u64| silent || covers(not_applied, seq) || lossy_reports && covers(written, seq);
        let lacking = sent.recorded.keys().find(|&&seq| match flow {
            FlowType::Recoverable => sent.ids.contains_key(&seq) && !received.delivered.contains_key(&seq),
            FlowType::Idempotent => {
                !received.delivered.contains_key(&seq) && !sent.session_owned.contains(&seq) && !reported(seq)
            }
            _ => false,
        });
        if let Some(seq) = lacking {
            return Some(format!("{side:?} sent {seq} on its {flow:?} flow, never delivered or reported"));
        }
        if sequenced(flow) && sent.next_recorded != self.sent[side.other().index()].incoming {
            let theirs = self.sent[side.other().index()].incoming;
            return Some(format!("{side:?} sends {} next, but the other side expects {theirs}", sent.next_recorded));
        }
        None
    }

    /// Rule 6 on an idempotent flow: `side`'s application only hears of numbers it sent.
    pub fn not_applied(&self, side: Side, not_applied: &[(u64, u64)]) -> Result<(), Violation> {
        let next = self.sent[side.index()].next_recorded;
        match not_applied.iter().find(|&&(from, n)| from == 0 || n == 0 || from + n > next) {
            Some((from, n)) => Err(violation(
                "6 lost",
                format!("{side:?} heard {n} from {from} weren't applied; it sent up to {next}"),
            )),
            None => Ok(()),
        }
    }

    /// Application messages `side` sent with a number.
    pub fn committed(&self, side: Side) -> usize {
        self.sent[side.index()].ids.len()
    }
}
