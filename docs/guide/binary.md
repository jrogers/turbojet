# Binary encodings

## SBE codecs

SBE (Simple Binary Encoding) is the FIX Trading Community's binary encoding: fixed-size fields at
fixed offsets, then repeating groups and variable-length data. `turbojet-codegen sbe` generates a
codec from a venue's SBE message schema: for each message, a struct to encode (`NewOrderSingle`)
and a decoder that reads it where it lies (`NewOrderSingleRef`), and `decode`, which reads the
message header and picks the decoder by template ID. Here the schema is B3's Binary Entrypoint,
the order entry of Brazil's exchange:

```sh
turbojet-codegen sbe b3-entrypoint.xml --out src/b3.rs
```

```rust
# #[allow(dead_code, clippy::all)]
# mod b3 { include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sbe/b3.rs")); }
use b3::*;
use turbojet::sbe::pad;

let order = NewOrderSingle {
    cl_ord_id: 123_456_789,
    security_id: 4001,
    price: PriceOptional { mantissa: Some(254_500) }, // 25.45: a decimal with exponent -4
    order_qty: 100,
    account: Some(12345),
    market_segment_id: 1,
    side: Side::Buy,
    ord_type: OrdType::Limit,
    time_in_force: TimeInForce::Day,
    ord_tag_id: None,
    mm_protection_reset: None,
    routing_instruction: None,
    self_trade_prevention_instruction: None,
    stop_px: PriceOptional { mantissa: None },
    min_qty: None,
    max_floor: None,
    investor_id: None,
    custodian_info: CustodianInfo { custodian: None, custody_account: None, custody_allocation_type: None },
    expire_date: None,
    sender_location: pad(b"DMA"),
    entering_trader: *b"TRADR",
};
let mut out = Vec::new();
order.encode_into(&mut out)?; // header first, appended

let (Decoded::NewOrderSingle(read), len) = decode(&out)? else { unreachable!("an order") };
assert_eq!((read.cl_ord_id(), read.side(), read.order_qty()), (123_456_789, Side::Buy, 100));
assert_eq!(len, out.len());
# Ok::<(), turbojet::sbe::SbeError>(())
```

A decoder checks the whole message when it's wrapped, groups and data included, so reading a field
afterwards can't fail and copies nothing. Fields newer than the message's schema version read as
absent, and a longer block from a newer version is read as far as the schema knows it. Enums keep a
value the schema doesn't list as `Unknown`. SBE 1.0 schemas are supported, in either byte order.
The generated code is checked against real-logic's SBE codecs, decoding what they encode and
encoding the same bytes, for SBE's example schema and B3's Binary Entrypoint.

## FIXP sessions

FIXP (the FIX Performance Session Layer, version 1.0) is the session protocol under SBE order
entry, as FIX's session layer is under tag=value. `turbojet::fixp` runs it point to point over
TCP, in either role, through the same connection driver as FIX sessions: a client *negotiates* a
session (a UUID) and the type of flow each way, then *establishes* it on each connection. Sequence
numbers are implicit, set by `Sequence` messages. Each direction's flow is recoverable (gaps
retransmitted), idempotent (gaps reported as not applied), unsequenced or none, configured on
both ends.

```rust,no_run
# #[allow(dead_code, clippy::all)]
# mod b3 { include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sbe/b3.rs")); }
use std::sync::Arc;
use turbojet::MemoryStorage;
use turbojet::fixp::{ClientConfig, FixpApplication, FixpContext, FixpInitiator, FixpInitiatorConfig, Received, SbeMessage};

/// Reads each message the venue sends with its codec.
struct Desk;

impl FixpApplication for Desk {
    fn on_message(&self, _ctx: &mut FixpContext<'_>, msg: Received<'_>) {
        if let Ok((b3::Decoded::ExecutionReportNew(report), _)) = b3::decode(msg.bytes) {
            println!("{:?}: order {:?} accepted", msg.seq, report.cl_ord_id());
        }
    }
}

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# use b3::*;
# use turbojet::sbe::pad;
# let order = NewOrderSingle {
#     cl_ord_id: 123_456_789,
#     security_id: 4001,
#     price: PriceOptional { mantissa: Some(254_500) }, // 25.45: a decimal with exponent -4
#     order_qty: 100,
#     account: Some(12345),
#     market_segment_id: 1,
#     side: Side::Buy,
#     ord_type: OrdType::Limit,
#     time_in_force: TimeInForce::Day,
#     ord_tag_id: None,
#     mm_protection_reset: None,
#     routing_instruction: None,
#     self_trade_prevention_instruction: None,
#     stop_px: PriceOptional { mantissa: None },
#     min_qty: None,
#     max_floor: None,
#     investor_id: None,
#     custodian_info: CustodianInfo { custodian: None, custody_account: None, custody_allocation_type: None },
#     expire_date: None,
#     sender_location: pad(b"DMA"),
#     entering_trader: *b"TRADR",
# };
let config = FixpInitiatorConfig::new(ClientConfig::new("CLIENT", "EXCHANGE")).with_endpoint("exchange:9000");
let initiator = FixpInitiator::new(config, Arc::new(MemoryStorage::new()), Arc::new(Desk))?;
tokio::spawn(initiator.clone().run());
// Once established: any message from a generated SBE codec.
initiator.handle().send(SbeMessage::encode(&order)?)?;
# Ok(())
# }
```

A `FixpAcceptor` serves clients, configured by a `FixpAcceptorConfig` (or just its
`ServerConfig`). The application sees each message as its SBE bytes, decoded with the venue's
codec, with its sequence number and whether it was retransmitted; it's told what the counterparty
didn't apply on an idempotent flow. A client derives its session ID from when it negotiated, so
it re-establishes the same session after a restart; a server keeps a log per session. Messages
sent on a recoverable flow are stored for retransmission.

`FixpAcceptor::serve_tls` and `FixpInitiatorConfig::tls` run sessions over TLS (feature `tls`),
and a server asking for client certificates sees the one a client presented in
`FixpApplication::verify`. `FixpHandle::finish` finishes sending, finalizing the session once the
counterparty has everything.

It's validated against itself, not a venue: a client and server in memory, each rule of the
specification a test (every flow, gaps both ways, each reject), over TCP and TLS through
reconnects, fuzzed, and run through the deterministic simulator. A client fails over to backup
endpoints (`FixpInitiatorConfig::endpoints`), and sessions record the same metrics as FIX ones.
Not supported: multiplexed or multicast sessions, throttling, and venues' own dialects (B3's,
CME's iLink 3).

## FAST

FAST (FIX Adapted for STreaming) is the compressed encoding some venues use for market data.
`turbojet-fast` loads a feed's templates from their XML and decodes each message into a `Message`,
each field's `id` its FIX tag, so typed messages read it like any other:

```rust,no_run
# let packet: &[u8] = &[];
use std::sync::Arc;
use turbojet::Message;
use turbojet_fast::{Decoder, Templates};
use turbojet_fix::fix44::MarketDataIncrementalRefresh;

let templates = Arc::new(Templates::from_xml(&std::fs::read_to_string("templates.xml")?)?);
let mut decoder = Decoder::new(templates);
let mut msg = Message::default();
let mut rest = packet;
while !rest.is_empty() {
    let decoded = decoder.decode_into(rest, &mut msg)?;
    let refresh: MarketDataIncrementalRefresh = msg.parse()?;
    rest = decoded.rest;
}
decoder.reset(); // where the feed resets its dictionaries
# Ok::<(), Box<dyn std::error::Error>>(())
```

A decoder keeps the stream's previous values, which the copy, increment, delta and tail
operators work from, so use one per stream and call `reset` where the feed says its dictionaries
reset: each packet, say, or on a reset message (`Decoded::template_id` says which template a
message used). A sequence becomes a repeating group, its `<length>`'s `id` the NumInGroup tag; a
group's fields, and a referenced template's, are written in its place; MsgType(35) is the
template's constant field with `id` 35. A decoder that has decoded a message once allocates
nothing for the next. Errors carry the FAST specification's error codes.

`Encoder` does the reverse, from a message whose fields are in the template's order, as a decoder
writes them: `encoder.encode_into(template_id, &msg, &mut out)`.

It's a codec only: receiving the feed (multicast, line arbitration, gap recovery from a snapshot or
replay channel) is the application's. FAST 1.1 is supported, not 1.2's extensions. Decoding is
checked against the specification's examples and against another implementation, and fuzzed.
