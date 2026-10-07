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

/// Sessions kept from before, on each side: enough for an old connection's last deliveries,
/// which a stall can hold back for a few finalizations.
const PAST_SESSIONS: usize = 4;

fn sequenced(flow: FlowType) -> bool {
    matches!(flow, FlowType::Recoverable | FlowType::Idempotent)
}

/// What one side has committed to sending on its flow, from its store, and what of it has been on
/// the wire.
#[derive(Clone, Default)]
struct Sent {
    /// Which logical session this is, counting from 0, and the last few before it, latest last:
    /// a connection's writes belong to the session it began in, which a finalization may have
    /// ended in the same commit as they were recorded, and an old connection may still deliver
    /// some after more sessions have come and gone.
    epoch: u64,
    past: Vec<Sent>,
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
                self.new_session();
                (self.opened, self.next_recorded, self.incoming) = (true, 1, 1);
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
            Stored::Opened { .. } | Stored::OpenFailed(_) | Stored::Uncertain(_) | Stored::Log(_) => {
                unreachable!("handled by stored")
            }
        }
        Ok(())
    }

    /// A new logical session, keeping the last for what's still written or delivered of it.
    fn new_session(&mut self) {
        let mut old = std::mem::take(self);
        let mut past = std::mem::take(&mut old.past);
        // Unsequenced orders have no numbers to start again: their order runs on across sessions.
        // The old session keeps them too, for what it still delivers over a lingering connection.
        let unsequenced = old.unsequenced.clone();
        *self = Sent { epoch: old.epoch + 1, ledger_seen: old.ledger_seen, unsequenced, ..Sent::default() };
        past.push(old);
        if past.len() > PAST_SESSIONS {
            past.remove(0);
        }
        self.past = past;
    }

    /// The order sent as `seq`, in this session or, failing that, the one before.
    fn id_of(&self, seq: u64) -> Option<u64> {
        self.ids.get(&seq).or_else(|| self.past.last()?.ids.get(&seq)).copied()
    }

    /// Session `epoch`: this one or one kept from before.
    fn session(&self, epoch: u64) -> Option<&Sent> {
        if self.epoch == epoch { Some(self) } else { self.past.iter().find(|p| p.epoch == epoch) }
    }

    fn session_mut(&mut self, epoch: u64) -> Option<&mut Sent> {
        if self.epoch == epoch { Some(self) } else { self.past.iter_mut().find(|p| p.epoch == epoch) }
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
    /// The sender's session when the connection began.
    epoch: u64,
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
    /// Sessions the server has answered a Negotiate for, and the session each connection's
    /// Establish names.
    negotiated: BTreeSet<[u8; 16]>,
    establishing: BTreeMap<ConnId, [u8; 16]>,
    /// The session the client last negotiated, whose log the server's state follows (the server
    /// has a log per session, and may open another's, say to refuse it); and whether its first
    /// open is still to come, starting the server's new session.
    server_session: Option<[u8; 16]>,
    server_fresh: bool,
    /// Whether the log each side's ledger entries are of now is the one followed.
    following: [bool; 2],
    ended_seen: [usize; 2],
    /// A store call failed on this side in the step being checked: its session ends in error.
    store_failed: [bool; 2],
    /// A power loss on a store without fsync lost what the OS hadn't written back: from then on
    /// only rule 1 and stores reopening are checked, and the sessions needn't settle.
    lossy: bool,
    /// Sides that have written FinishedSending on a connection: nothing more of theirs follows.
    finishing: BTreeSet<(Side, ConnId)>,
    /// The last number each side's FinishedSending named, and the claims of the side answering
    /// FinishedReceiving to have everything up to it, to check once its deliveries are seen.
    finished_last: [Option<u64>; 2],
    claims: Vec<Side>,
    /// Each side's not-applied reports already checked, and where its current session's begin.
    not_applied_seen: [usize; 2],
    /// A side's store began a new session: what it receives starts again after this step's
    /// deliveries.
    received_reset: [bool; 2],
    /// The server's last session, by its key, and what it received: it may still deliver what
    /// was on its way over an old connection after the next session has begun. And the key of
    /// the session the server's current received state is of.
    received_old: BTreeMap<String, Received>,
    /// The client's session that negotiated each session ID (as a log names it).
    client_epochs: BTreeMap<String, u64>,
    followed_key: Option<String>,
    not_applied_from: [usize; 2],
    /// Sessions finalized, for the report.
    pub finalized: u64,
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
            negotiated: BTreeSet::new(),
            establishing: BTreeMap::new(),
            server_session: None,
            server_fresh: false,
            following: [true; 2],
            ended_seen: [0; 2],
            store_failed: [false; 2],
            finishing: BTreeSet::new(),
            finished_last: [None; 2],
            claims: Vec::new(),
            not_applied_seen: [0; 2],
            received_reset: [false; 2],
            received_old: BTreeMap::new(),
            client_epochs: BTreeMap::new(),
            followed_key: None,
            not_applied_from: [0; 2],
            finalized: 0,
            lossy: false,
            resent: 0,
        }
    }

    /// A power loss without fsync: see `lossy`.
    pub fn lose(&mut self) {
        self.lossy = true;
    }

    pub fn is_lossy(&self) -> bool {
        self.lossy
    }

    /// Endings of `side`'s connections already checked.
    pub fn ended_seen(&self, side: Side) -> usize {
        self.ended_seen[side.index()]
    }

    /// A store call on `side` failed (a trap): the session's ending in error is expected.
    pub fn store_failed(&mut self, side: Side) {
        self.store_failed[side.index()] = true;
    }

    /// Rules 4 and 9 on what `side`'s store recorded since the last call.
    pub fn stored(&mut self, side: Side, ledger: &[Stored]) -> Result<(), Violation> {
        let i = side.index();
        while let Some(entry) = ledger.get(self.sent[i].ledger_seen) {
            self.sent[i].ledger_seen += 1;
            if let Stored::Log(id) = entry {
                // The client keeps one log; the server's followed is its negotiated session's.
                self.following[i] =
                    side == Side::Initiator || self.server_key().as_deref() == Some(&*id.target_comp_id);
                continue;
            }
            if !self.following[i] {
                continue;
            }
            match entry {
                Stored::Uncertain(changes) => self.sent[i].uncertain.extend(changes.iter().cloned()),
                Stored::OpenFailed(e) => return Err(violation("9 store", format!("{side:?}'s store won't open: {e}"))),
                Stored::Opened { next_outgoing, next_incoming } => {
                    self.opened(side, (*next_outgoing, *next_incoming))?
                }
                change => {
                    if matches!(change, Stored::Reset) {
                        self.reset(side);
                    }
                    let lossy = self.lossy;
                    self.sent[i].apply(side, change).or_else(|e| if lossy { Ok(()) } else { Err(e) })?;
                }
            }
        }
        Ok(())
    }

    /// Rule 9 on `side`'s store opening with `opened` (next outgoing, next incoming).
    fn opened(&mut self, side: Side, opened: (u64, u64)) -> Result<(), Violation> {
        if side == Side::Acceptor && std::mem::take(&mut self.server_fresh) {
            self.begin_server_session();
        }
        let lossy = self.lossy;
        let sent = &mut self.sent[side.index()];
        let uncertain = std::mem::take(&mut sent.uncertain);
        if !sent.opened {
            sent.opened = true;
            (sent.next_recorded, sent.incoming) = opened;
        } else if lossy {
            // What was lost is lost: carry on from what the store has.
            sent.recorded.retain(|seq, _| *seq < opened.0);
            sent.ids.retain(|seq, _| *seq < opened.0);
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
        Ok(())
    }

    /// The server opens the log of the session the client last negotiated: both directions start
    /// again, and what the last session still delivers is told apart by its session.
    fn begin_server_session(&mut self) {
        let i = Side::Acceptor.index();
        self.sent[i].new_session();
        self.not_applied_from[i] = self.not_applied_seen[i];
        let last = std::mem::take(&mut self.received[i]);
        self.received[i] = Received { seen: last.seen, unsequenced_next: last.unsequenced_next, ..Received::default() };
        if let Some(key) = self.followed_key.take() {
            self.received_old.insert(key, last);
            // As many as the sessions kept on the client's side.
            while self.received_old.len() > PAST_SESSIONS {
                let oldest = self.received_old.keys().next().cloned().expect("not empty");
                self.received_old.remove(&oldest);
            }
        }
        self.followed_key = self.server_key();
    }

    /// `side`'s store has reset: its session is finished with.
    fn reset(&mut self, side: Side) {
        if side == Side::Initiator {
            self.may_negotiate = true;
            // What the client receives starts again too, once the deliveries it made before
            // ending the last are checked. The server's starts again with its next session's
            // log, its last told apart by session.
            self.received_reset[side.index()] = true;
        } else {
            // Finalized: the server no longer knows the session.
            self.negotiated.clear();
        }
        self.not_applied_from[side.index()] = self.not_applied_seen[side.index()];
    }

    /// Rules 1-4, on what `side` wrote on `conn`, after what its store recorded.
    pub fn written(&mut self, side: Side, conn: ConnId, bytes: &[u8]) -> Result<(), Violation> {
        let frames = wire::frames(bytes).map_err(|e| violation("1 frames", format!("{side:?} wrote {e}")))?;
        for frame in frames {
            let decoded = wire::decode(frame).map_err(|e| violation("1 frames", format!("{side:?} wrote {e}")))?;
            if self.lossy {
                continue;
            }
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
            Frame::Negotiate { .. }
            | Frame::NegotiationResponse
            | Frame::Establish { .. }
            | Frame::EstablishmentAck { .. } => {
                self.handshake(conn, &frame)?;
            }
            // The client may name a session its Negotiate never reached the server with (lost with
            // the connection, or never sent, its store failing first): refused, it negotiates anew.
            Frame::EstablishmentReject { code: UNNEGOTIATED }
                if self.establishing.get(&conn).is_some_and(|id| !self.negotiated.contains(id)) => {}
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
                let epoch = self.wire(side, conn).epoch;
                let Some(sent) = self.sent_in(side, epoch) else { return Ok(()) };
                if next > sent.next_recorded {
                    return Err(violation(
                        "4 sequence",
                        format!("{side:?} wrote Sequence {next}, but only recorded up to {}", sent.next_recorded),
                    ));
                }
                let wire = self.wire(side, conn);
                wire.next = Some(next);
                wire.replay = None;
            }
            Frame::FinishedSending { last } => {
                self.finishing.insert((side, conn));
                self.finished_last[side.index()] = last;
            }
            Frame::FinishedReceiving => self.claims.push(side),
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
                self.wire(side, conn).replay = (count > 0).then_some((next, count));
            }
            _ => {}
        }
        Ok(())
    }

    /// Rule 2 on a handshake message written on `conn`.
    fn handshake(&mut self, conn: ConnId, frame: &Frame) -> Result<(), Violation> {
        match *frame {
            Frame::Negotiate { session_id } => {
                if !self.may_negotiate {
                    return Err(violation("2 handshake", "the client negotiated again, its session not finished"));
                }
                self.may_negotiate = false;
                let key = session_id.iter().map(|b| format!("{b:02x}")).collect();
                self.client_epochs.insert(key, self.sent[Side::Initiator.index()].epoch);
                self.server_session = Some(session_id);
                self.server_fresh = true;
                self.client_session = Some(session_id);
            }
            Frame::NegotiationResponse => self.negotiated.extend(self.client_session),
            Frame::Establish { session_id, .. } => {
                self.establishing.insert(conn, session_id);
            }
            // The server may have recorded a Negotiate whose answer a store failure kept from
            // going out: the session the client asked for, and established, is still its own.
            Frame::EstablishmentAck { .. } => {
                if self.establishing.get(&conn).copied() != self.client_session {
                    return Err(violation(
                        "2 handshake",
                        "the server established a session the client never asked for",
                    ));
                }
                self.established.insert(conn);
            }
            _ => unreachable!("not a handshake message: {frame:?}"),
        }
        Ok(())
    }

    /// The number of the application message `side` writes next on `conn`, whether it's
    /// retransmitted, and the session the connection began in.
    fn number_written(&mut self, side: Side, conn: ConnId, decoded: &Frame) -> Result<(u64, bool, u64), Violation> {
        let wire = self.wire(side, conn);
        let epoch = wire.epoch;
        if let Some((next, left)) = wire.replay {
            wire.replay = (left > 1).then_some((next + 1, left - 1));
            return Ok((next, true, epoch));
        }
        let Some(next) = wire.next else {
            return Err(violation("4 sequence", format!("{side:?} wrote {decoded:?} before a Sequence")));
        };
        wire.next = Some(next + 1);
        Ok((next, false, epoch))
    }

    /// Rules 2 and 4 on an application message `side` wrote.
    fn application(&mut self, side: Side, conn: ConnId, frame: &[u8], decoded: &Frame) -> Result<(), Violation> {
        if !self.established.contains(&conn) {
            return Err(violation(
                "2 handshake",
                format!("{side:?} wrote {decoded:?} before the session was established"),
            ));
        }
        // Retransmissions it's asked for still go out; new messages don't.
        let finished = self.finishing.contains(&(side, conn));
        let live_after_finish =
            || violation("10 finish", format!("{side:?} wrote {decoded:?} live after FinishedSending"));
        let flow = self.flows[side.index()];
        let id = match decoded {
            Frame::Order(id) => Some(*id),
            _ => None,
        };
        if let Frame::NotApplied { from, count } = decoded {
            self.sent[side.index()].not_applied.push((*from, u64::from(*count)));
        }
        if !sequenced(flow) {
            if finished {
                return Err(live_after_finish());
            }
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
        let (seq, replayed, epoch) = self.number_written(side, conn, decoded)?;
        let sent = self.sent[side.index()].session_mut(epoch);
        let Some(sent) = sent else {
            return Err(violation("4 sequence", format!("{side:?} wrote {seq} for a session two back")));
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
        if finished {
            return Err(live_after_finish());
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

    /// Rules 5 and 7, on what `side`'s application has received since the last call; then a new
    /// session's start, if its store began one meanwhile (after them: a session ends after its
    /// last delivery, and the next delivers only over a later connection).
    pub fn delivered(&mut self, side: Side, deliveries: &[Delivery]) -> Result<(), Violation> {
        self.check_deliveries(side, deliveries)?;
        self.claims_hold(side)?;
        if std::mem::take(&mut self.received_reset[side.index()]) {
            let unsequenced_next = self.received[side.index()].unsequenced_next;
            self.received[side.index()] = Received { seen: deliveries.len(), unsequenced_next, ..Received::default() };
        }
        Ok(())
    }

    fn check_deliveries(&mut self, side: Side, deliveries: &[Delivery]) -> Result<(), Violation> {
        let flow = self.flows[side.other().index()];
        let from = self.received[side.index()].seen;
        self.received[side.index()].seen = deliveries.len();
        if self.lossy {
            return Ok(());
        }
        let current = self.server_key();
        for d in &deliveries[from..] {
            // The server's last session may still deliver what was on its way over its old
            // connection: checked against that session's own records.
            let old = side == Side::Acceptor && Some(&d.session) != current.as_ref();
            if old {
                // The client's session that negotiated it, and what the server's received of it.
                let epoch = self.client_epochs.get(&d.session).copied();
                let sender = epoch.and_then(|epoch| self.sent[side.other().index()].session(epoch));
                let (Some(sender), Some(received)) = (sender, self.received_old.get_mut(&d.session)) else {
                    continue;
                };
                check_delivery(side, flow, sender, received, d)?;
                continue;
            }
            check_delivery(side, flow, &self.sent[side.other().index()], &mut self.received[side.index()], d)?;
        }
        Ok(())
    }

    /// The key a server delivery of the followed session carries: its session ID, as its log
    /// names it.
    fn server_key(&self) -> Option<String> {
        self.server_session.map(|uuid| uuid.iter().map(|b| format!("{b:02x}")).collect())
    }

    /// Rule 10 on the FinishedReceiving answers `receiver` wrote since the last call: on a
    /// recoverable flow, it has had every message up to the last the other side named. Checked
    /// with its deliveries, before a new session's start clears them.
    fn claims_hold(&mut self, receiver: Side) -> Result<(), Violation> {
        let (mine, others): (Vec<Side>, Vec<Side>) = self.claims.drain(..).partition(|&r| r == receiver);
        self.claims = others;
        for receiver in mine {
            let sender = receiver.other();
            if self.lossy || self.flows[sender.index()] != FlowType::Recoverable {
                continue;
            }
            let last = self.finished_last[sender.index()].unwrap_or(0);
            let delivered = &self.received[receiver.index()].delivered;
            let sent = &self.sent[sender.index()];
            if let Some(seq) = sent.ids.keys().take_while(|&&seq| seq <= last).find(|seq| !delivered.contains_key(seq))
            {
                return Err(violation(
                    "10 finish",
                    format!("{receiver:?} answered FinishedReceiving to {last}, never having got {seq}"),
                ));
            }
        }
        Ok(())
    }

    /// `side`'s writing on `conn`, begun in its current session if new.
    fn wire(&mut self, side: Side, conn: ConnId) -> &mut Wire {
        let epoch = self.sent[side.index()].epoch;
        self.wires.entry((side, conn)).or_insert_with(|| Wire { epoch, ..Wire::default() })
    }

    /// `side`'s session `epoch`: the current one or the one before.
    fn sent_in(&self, side: Side, epoch: u64) -> Option<&Sent> {
        let sent = &self.sent[side.index()];
        sent.session(epoch)
    }

    /// Rule 8 on a receipt `side`'s application got for order `id`.
    pub fn receipt(&mut self, side: Side, id: u64, outcome: &Result<u64, Dropped>) -> Result<(), Violation> {
        let flow = self.flows[side.index()];
        if self.lossy {
            return Ok(());
        }
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
        let excused = std::mem::take(&mut self.store_failed[side.index()]);
        for how in &ended[self.ended_seen[side.index()]..] {
            if self.lossy || excused && *how == Ended::Error {
                continue;
            }
            if *how == Ended::Finalized {
                self.finalized += 1;
            }
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
        let not_applied = &not_applied[self.not_applied_from[side.index()].min(not_applied.len())..];
        let sent = &self.sent[side.index()];
        let received = &self.received[side.other().index()];
        let covers = |ranges: &[(u64, u64)], seq: u64| ranges.iter().any(|&(from, n)| seq >= from && seq < from + n);
        let written = &self.sent[side.other().index()].not_applied;
        let lossy_reports = self.flows[side.other().index()] != FlowType::Recoverable;
        // With no flow back, a gap can't be reported at all: at most once is all that's promised.
        let silent = self.flows[side.other().index()] == FlowType::None;
        // The receiver's store moving past a number is its decision that it wasn't applied: on a
        // flow back that isn't recoverable, the report may be lost, written or not.
        let passed = |seq: u64| seq < self.sent[side.other().index()].incoming;
        let reported =
            |seq: u64| silent || covers(not_applied, seq) || lossy_reports && (covers(written, seq) || passed(seq));
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
    pub fn not_applied(&mut self, side: Side, not_applied: &[(u64, u64)]) -> Result<(), Violation> {
        let unseen = &not_applied[self.not_applied_seen[side.index()].min(not_applied.len())..];
        self.not_applied_seen[side.index()] = not_applied.len();
        if self.lossy {
            return Ok(());
        }
        let not_applied = unseen;
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

/// Rules 5-7 on one delivery `side` received from `sender` (a session's records).
fn check_delivery(
    side: Side,
    flow: FlowType,
    sender: &Sent,
    received: &mut Received,
    d: &Delivery,
) -> Result<(), Violation> {
    let Some(seq) = d.seq else {
        if flow != FlowType::Unsequenced {
            return Err(violation("5 delivery", format!("{side:?} got {} unnumbered on a {flow:?} flow", d.id)));
        }
        let from = received.unsequenced_next;
        let Some(i) = sender.unsequenced.get(from..).unwrap_or_default().iter().position(|&id| id == d.id) else {
            return Err(violation("7 unsequenced", format!("{side:?} got {} out of order, again or never sent", d.id)));
        };
        received.unsequenced_next = from + i + 1;
        return Ok(());
    };
    if sender.id_of(seq) != Some(d.id) {
        let sent = sender.id_of(seq);
        return Err(violation("5 delivery", format!("{side:?} got {d:?}, but {sent:?} was sent as {seq}")));
    }
    // A repeat is fine only when marked as possibly one, after a crash or a lost connection.
    if d.redelivered && received.delivered.contains_key(&seq) {
        return Ok(());
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
    // In order: on a recoverable flow every number, on an idempotent one with gaps the sender
    // hears of.
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
    Ok(())
}
