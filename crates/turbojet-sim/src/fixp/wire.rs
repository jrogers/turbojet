//! A decoder of what FIXP sessions write, of its own, so the checker doesn't take the engine's word
//! for what's on the wire: SOFH frames, FIXP 1.0's session messages (only the fields the checker
//! reads), and the workload's orders.

/// The FIXP schema's ID, from `dictionaries/sbe/fixp-1.0.xml`.
const FIXP_SCHEMA: u16 = 2748;
/// The workload's orders: a schema of their own, one u64 body field, the order's id.
pub const ORDER_SCHEMA: u16 = 1;
pub const ORDER_TEMPLATE: u16 = 102;
pub const ORDER_BLOCK: u16 = 8;

/// The SOFH framing header: a u32 big-endian length (the header included), then 0x5BE0.
const SOFH: usize = 6;
const ENCODING_TYPE: u16 = 0x5BE0;
/// SBE's message header: block length, template, schema, version.
const HEADER: usize = 8;
/// An optional `ordinal`'s null value.
const NULL_ORDINAL: u64 = u64::MAX;

/// One frame's message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Negotiate {
        session_id: [u8; 16],
    },
    NegotiationResponse,
    NegotiationReject,
    Establish {
        session_id: [u8; 16],
        next: Option<u64>,
    },
    EstablishmentAck {
        next: Option<u64>,
    },
    EstablishmentReject {
        code: u8,
    },
    Sequence {
        next: u64,
    },
    UnsequencedHeartbeat,
    RetransmitRequest {
        from: u64,
        count: u32,
    },
    Retransmission {
        next: u64,
        count: u32,
    },
    RetransmitReject,
    Terminate {
        code: u8,
        reason: Vec<u8>,
    },
    FinishedSending {
        last: Option<u64>,
    },
    FinishedReceiving,
    /// Applied and NotApplied: application messages the session itself sends, numbered in its flow.
    Applied {
        from: u64,
        count: u32,
    },
    NotApplied {
        from: u64,
        count: u32,
    },
    /// A workload order, by its id.
    Order(u64),
}

impl Frame {
    /// Whether it takes a number in the sender's flow.
    pub fn is_application(&self) -> bool {
        matches!(self, Frame::Order(_) | Frame::Applied { .. } | Frame::NotApplied { .. })
    }
}

/// Splits `bytes` into whole frames, each with its header.
pub fn frames(mut bytes: &[u8]) -> Result<Vec<&[u8]>, String> {
    let mut out = Vec::new();
    while !bytes.is_empty() {
        if bytes.len() < SOFH {
            return Err(format!("{} bytes, not a whole framing header", bytes.len()));
        }
        let len = usize::try_from(u32::from_be_bytes(bytes[..4].try_into().expect("4 bytes"))).unwrap_or(usize::MAX);
        if u16::from_be_bytes([bytes[4], bytes[5]]) != ENCODING_TYPE {
            return Err("a framing header that isn't SBE 1.0 little-endian".into());
        }
        if len < SOFH + HEADER || len > bytes.len() {
            return Err(format!("a frame of {len} bytes, with {} left", bytes.len()));
        }
        out.push(&bytes[..len]);
        bytes = &bytes[len..];
    }
    Ok(out)
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

fn uuid_at(b: &[u8], at: usize) -> Option<[u8; 16]> {
    b.get(at..at + 16)?.try_into().ok()
}

fn optional(n: u64) -> Option<u64> {
    (n != NULL_ORDINAL).then_some(n)
}

/// The message in a whole `frame`, framing header and all.
pub fn decode(frame: &[u8]) -> Result<Frame, String> {
    decode_sbe(&frame[SOFH..])
}

/// The message in `sbe`: an SBE header and body, unframed.
pub fn decode_sbe(sbe: &[u8]) -> Result<Frame, String> {
    let header = |at| u16_at(sbe, at).ok_or("a short SBE header");
    let (block, template, schema) = (header(0)?, header(2)?, header(4)?);
    let body = sbe.get(HEADER..).unwrap_or_default();
    if body.len() < usize::from(block) {
        return Err(format!("template {template}: a block of {block} with {} bytes", body.len()));
    }
    let short = || format!("template {template} too short");
    if schema == ORDER_SCHEMA && template == ORDER_TEMPLATE {
        return u64_at(body, 0).map(Frame::Order).ok_or_else(short);
    }
    if schema != FIXP_SCHEMA {
        return Err(format!("schema {schema}, template {template}: neither FIXP's nor the workload's"));
    }
    let frame = match template {
        1 => uuid_at(body, 0).map(|session_id| Frame::Negotiate { session_id }),
        2 => Some(Frame::NegotiationResponse),
        3 => Some(Frame::NegotiationReject),
        5 => uuid_at(body, 0)
            .zip(u64_at(body, 28))
            .map(|(session_id, next)| Frame::Establish { session_id, next: optional(next) }),
        6 => u64_at(body, 28).map(|next| Frame::EstablishmentAck { next: optional(next) }),
        7 => body.get(24).map(|&code| Frame::EstablishmentReject { code }),
        8 => u64_at(body, 0).map(|next| Frame::Sequence { next }),
        10 => Some(Frame::UnsequencedHeartbeat),
        11 => u64_at(body, 24).zip(u32_at(body, 32)).map(|(from, count)| Frame::RetransmitRequest { from, count }),
        12 => u64_at(body, 24).zip(u32_at(body, 32)).map(|(next, count)| Frame::Retransmission { next, count }),
        13 => Some(Frame::RetransmitReject),
        14 => body.get(16).map(|&code| {
            let block = usize::from(block);
            let len = u16_at(body, block).map_or(0, usize::from);
            let reason = body.get(block + 2..block + 2 + len).unwrap_or_default().to_vec();
            Frame::Terminate { code, reason }
        }),
        15 => u64_at(body, 16).map(|last| Frame::FinishedSending { last: optional(last) }),
        16 => Some(Frame::FinishedReceiving),
        17 => u64_at(body, 0).zip(u32_at(body, 8)).map(|(from, count)| Frame::Applied { from, count }),
        18 => u64_at(body, 0).zip(u32_at(body, 8)).map(|(from, count)| Frame::NotApplied { from, count }),
        other => return Err(format!("FIXP template {other}, which the sessions never send")),
    };
    frame.ok_or_else(short)
}

/// The id of the order in `sbe` (unframed), if it is one.
pub fn order_id_sbe(sbe: &[u8]) -> Option<u64> {
    match decode_sbe(sbe) {
        Ok(Frame::Order(id)) => Some(id),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn framed(sbe: &[u8]) -> Vec<u8> {
        let len = u32::try_from(SOFH + sbe.len()).unwrap();
        let mut out = len.to_be_bytes().to_vec();
        out.extend_from_slice(&ENCODING_TYPE.to_be_bytes());
        out.extend_from_slice(sbe);
        out
    }

    #[test]
    fn decodes_orders_and_session_messages() {
        let mut order = Vec::new();
        for n in [ORDER_BLOCK, ORDER_TEMPLATE, ORDER_SCHEMA, 0] {
            order.extend_from_slice(&n.to_le_bytes());
        }
        order.extend_from_slice(&42u64.to_le_bytes());
        let mut sequence = Vec::new();
        for n in [8u16, 8, FIXP_SCHEMA, 0] {
            sequence.extend_from_slice(&n.to_le_bytes());
        }
        sequence.extend_from_slice(&7u64.to_le_bytes());
        let bytes = [framed(&order), framed(&sequence)].concat();
        let frames = frames(&bytes).unwrap();
        assert_eq!(decode(frames[0]), Ok(Frame::Order(42)));
        assert_eq!(decode(frames[1]), Ok(Frame::Sequence { next: 7 }));
        assert!(super::frames(&bytes[..bytes.len() - 1]).is_err());
    }
}
