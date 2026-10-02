//! A hostile middlebox: between a connection and its receiver, it drops, duplicates, swaps or
//! corrupts whole messages, as a buggy counterparty or proxy might. TCP never does this; the
//! sessions must still recover, through resends, rejects and logouts.

use std::collections::BTreeMap;

use turbojet::codec::{Decoded, decode};

use crate::Side;
use crate::net::ConnId;
use crate::rng::Rng;

pub struct Proxy {
    /// Chance in a million that a message is tampered with.
    rate: u32,
    rng: Rng,
    /// What has arrived for each receiver but isn't a whole message yet.
    partial: BTreeMap<(ConnId, Side), Vec<u8>>,
}

impl Proxy {
    pub fn new(rate: u32, rng: Rng) -> Self {
        Self { rate, rng, partial: BTreeMap::new() }
    }

    /// Bytes arriving for `to` on `conn`: what reaches it, tampered with if `active`. Only whole
    /// messages go through; a piece waits for the rest.
    pub fn pass(&mut self, conn: ConnId, to: Side, bytes: &[u8], active: bool) -> Vec<u8> {
        let partial = self.partial.entry((conn, to)).or_default();
        partial.extend_from_slice(bytes);
        let mut pieces: Vec<Vec<u8>> = Vec::new();
        let mut consumed = 0;
        loop {
            match decode(&partial[consumed..]) {
                Decoded::Message(_, len) => {
                    pieces.push(partial[consumed..consumed + len].to_vec());
                    consumed += len;
                }
                // Garbage goes through as it is: the receiver discards it.
                Decoded::Garbled { skip, .. } => {
                    pieces.push(partial[consumed..consumed + skip].to_vec());
                    consumed += skip;
                }
                Decoded::Incomplete => break,
            }
        }
        partial.drain(..consumed);
        if active {
            pieces = self.tamper(pieces);
        }
        pieces.concat()
    }

    fn tamper(&mut self, pieces: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut out = Vec::with_capacity(pieces.len());
        let mut pieces = pieces.into_iter().peekable();
        while let Some(mut piece) = pieces.next() {
            if !self.rng.chance(self.rate) {
                out.push(piece);
                continue;
            }
            match self.rng.below(4) {
                0 => {} // dropped
                1 => out.extend([piece.clone(), piece]),
                2 => match pieces.next() {
                    Some(next) => out.extend([next, piece]),
                    None => out.push(piece),
                },
                _ => {
                    let at = usize::try_from(self.rng.below(piece.len() as u64)).expect("in range");
                    piece[at] ^= u8::try_from(self.rng.between(1, 255)).expect("a byte");
                    out.push(piece);
                }
            }
        }
        out
    }

    /// `conn` has gone: what was left of it goes too.
    pub fn forget(&mut self, conn: ConnId) {
        self.partial.retain(|(c, _), _| *c != conn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(seq: u64) -> Vec<u8> {
        let body = format!("35=0\x0149=A\x0156=B\x0134={seq}\x0152=20260105-09:00:00\x01");
        let head = format!("8=FIX.4.4\x019={}\x01", body.len());
        let sum = head.bytes().chain(body.bytes()).fold(0u8, u8::wrapping_add);
        format!("{head}{body}10={sum:03}\x01").into_bytes()
    }

    #[test]
    fn whole_messages_pass_untouched_when_quiet() {
        let mut proxy = Proxy::new(1_000_000, Rng::new(1));
        let wire = [frame(1), frame(2)].concat();
        let (first, rest) = wire.split_at(30);
        assert!(proxy.pass(1, Side::Acceptor, first, false).is_empty(), "a piece waits");
        assert_eq!(proxy.pass(1, Side::Acceptor, rest, false), wire);
    }

    #[test]
    fn active_tampering_changes_what_arrives() {
        let mut proxy = Proxy::new(1_000_000, Rng::new(2));
        let wire: Vec<u8> = (1..=20).flat_map(frame).collect();
        assert_ne!(proxy.pass(1, Side::Initiator, &wire, true), wire);
    }
}
