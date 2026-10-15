//! The specification's own template (section 1, an incremental refresh), decoded from bytes
//! encoded by hand and then read as a typed FIX 4.4 message.

use std::sync::Arc;

use turbojet::Message;
use turbojet::fields::Code;
use turbojet_fast::{Decoder, Templates};
use turbojet_fix44::{MDEntryType, MDUpdateAction, MarketDataIncrementalRefresh};

/// Section 1's example, with an `id` so the stream can name it.
const TEMPLATES: &str = r#"
<templates xmlns="http://www.fixprotocol.org/ns/template-definition"
    templateNs="http://www.fixprotocol.org/ns/templates/sample" ns="http://www.fixprotocol.org/ns/fix">
  <template name="MDRefreshSample" id="1">
    <typeRef name="MarketDataIncrementalRefresh"/>
    <string name="BeginString" id="8"> <constant value="FIX4.4"/> </string>
    <string name="MessageType" id="35"> <constant value="X"/> </string>
    <string name="SenderCompID" id="49"> <copy/> </string>
    <uInt32 name="MsgSeqNum" id="34"> <increment/> </uInt32>
    <sequence name="MDEntries">
      <length name="NoMDEntries" id="268"/>
      <uInt32 name="MDUpdateAction" id="279"> <copy/> </uInt32>
      <string name="MDEntryType" id="269"> <copy/> </string>
      <string name="Symbol" id="55"> <copy/> </string>
      <string name="SecurityType" id="167"> <copy/> </string>
      <decimal name="MDEntryPx" id="270"> <delta/> </decimal>
      <decimal name="MDEntrySize" id="271"> <delta/> </decimal>
      <uInt32 name="NumberOfOrders" id="346"> <delta/> </uInt32>
      <string name="QuoteCondition" id="276"> <copy/> </string>
      <string name="TradeCondition" id="277"> <copy/> </string>
    </sequence>
  </template>
</templates>"#;

/// Two bids for ESZ6, the second copying most of the first and moving its price and size.
fn message() -> Vec<u8> {
    let header = [
        &[0xf0][..],         // Presence map: template identifier, SenderCompID, MsgSeqNum.
        &[0x81],             // Template 1.
        &[0x43, 0x4d, 0xc5], // SenderCompID CME.
        &[0x81],             // MsgSeqNum 1.
        &[0x82],             // Two entries.
    ];
    let first = [
        &[0xfe][..],               // Every copied field present.
        &[0x80],                   // MDUpdateAction 0 (New).
        &[0xb0],                   // MDEntryType 0 (Bid).
        &[0x45, 0x53, 0x5a, 0xb6], // Symbol ESZ6.
        &[0x46, 0x55, 0xd4],       // SecurityType FUT.
        &[0xfe, 0x39, 0x45, 0xa3], // MDEntryPx: 942755 * 10^-2, from 0.
        &[0x80, 0x8a],             // MDEntrySize: 10, from 0.
        &[0x83],                   // NumberOfOrders: 3, from 0.
        &[0xc1],                   // QuoteCondition A.
        &[0xc2],                   // TradeCondition B.
    ];
    let second = [
        &[0xa0][..],   // Only MDEntryType present.
        &[0xb1],       // MDEntryType 1 (Offer).
        &[0x80, 0x81], // MDEntryPx up 0.01.
        &[0x80, 0xfb], // MDEntrySize down 5.
        &[0xfe],       // NumberOfOrders down 2.
    ];
    [&header[..], &first, &second].concat().concat()
}

#[test]
fn the_specifications_template_decodes_into_a_typed_refresh() {
    let mut decoder = Decoder::new(Arc::new(Templates::from_xml(TEMPLATES).expect("valid templates")));
    let mut msg = Message::default();
    let input = message();
    let decoded = decoder.decode_into(&input, &mut msg).expect("decodes");
    assert!(decoded.rest.is_empty());
    assert_eq!(msg.get(49), Some("CME"));
    assert_eq!(msg.get(34), Some("1"));

    let refresh: MarketDataIncrementalRefresh = msg.parse().expect("a FIX 4.4 refresh");
    let [bid, offer] = refresh.md_entries.as_slice() else { panic!("two entries") };
    assert_eq!(bid.md_update_action, Code::Known(MDUpdateAction::New));
    assert_eq!(bid.md_entry_type, Some(Code::Known(MDEntryType::Bid)));
    assert_eq!(offer.md_entry_type, Some(Code::Known(MDEntryType::Offer)));
    for entry in [bid, offer] {
        assert_eq!(entry.symbol.as_deref(), Some("ESZ6"));
    }
    assert_eq!(bid.md_entry_px.map(|px| px.to_string()).as_deref(), Some("9427.55"));
    assert_eq!(offer.md_entry_px.map(|px| px.to_string()).as_deref(), Some("9427.56"));
    assert_eq!(bid.md_entry_size.map(|size| size.to_string()).as_deref(), Some("10"));
    assert_eq!(offer.md_entry_size.map(|size| size.to_string()).as_deref(), Some("5"));
    assert_eq!((bid.number_of_orders, offer.number_of_orders), (Some(3), Some(1)));
}

#[test]
fn encoding_the_decoded_refresh_gives_the_same_bytes() {
    let templates = Arc::new(Templates::from_xml(TEMPLATES).expect("valid templates"));
    let mut decoder = Decoder::new(Arc::clone(&templates));
    let mut msg = Message::default();
    let input = message();
    let decoded = decoder.decode_into(&input, &mut msg).expect("decodes");
    let mut encoder = turbojet_fast::Encoder::new(templates);
    let mut out = Vec::new();
    encoder.encode(decoded.template_id, &msg, &mut out).expect("encodes");
    assert_eq!(out, input);
}
