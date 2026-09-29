//! Orders built as raw fields, so the same message works on every version tested. All their
//! dictionaries require 11, 54, 60 and 40; FIX 4.2 and 4.3 also require HandlInst (21), which
//! later versions allow. 55 provides the Instrument component (FIX 4.2 requires Symbol
//! directly, as does FIX 4.4 within Instrument; FIX 5.0 SP2 doesn't), and 38 OrderQtyData
//! (required from FIX 4.3; optional in FIX 4.2).

use turbojet::message::tags;
use turbojet::{Message, MsgType};

/// A NewOrderSingle valid in every version tested, as the peer's `send` fields.
pub fn peer_order(id: &str) -> String {
    format!("35=D|11={id}|21=1|54=1|60=20260929-12:00:00.000|40=1|55=AAPL|38=100")
}

/// The same order, for Turbojet to send.
pub fn tj_order(id: &str) -> Message {
    Message::new(MsgType::NewOrderSingle)
        .with(tags::CL_ORD_ID, id)
        .with(tags::HANDL_INST, "1")
        .with(tags::SIDE, "1")
        .with(tags::TRANSACT_TIME, "20260929-12:00:00.000")
        .with(tags::ORD_TYPE, "1")
        .with(tags::SYMBOL, "AAPL")
        .with(tags::ORDER_QTY, "100")
}
