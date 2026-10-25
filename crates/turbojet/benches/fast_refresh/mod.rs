//! An incremental refresh like a feed's, shared by the benchmarks and the allocation test: the
//! specification's section 1 template, and messages of it with a given number of entries.

use std::sync::Arc;

use turbojet::Message;
use turbojet::fast::{Encoder, Templates};

pub const TEMPLATES: &str = r#"
<templates xmlns="http://www.fixprotocol.org/ns/fast/td/1.1">
  <template name="MDRefreshSample" id="1">
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

pub fn templates() -> Arc<Templates> {
    Arc::new(Templates::from_xml(TEMPLATES).expect("valid templates"))
}

/// A refresh of `entries` price levels, bids then offers, a tick apart.
pub fn refresh(seq: u32, entries: usize) -> Message {
    let mut msg = Message::default();
    msg.push(8, "FIX4.4");
    msg.push(49, "CME");
    msg.push(34, seq);
    msg.push(268, u32::try_from(entries).expect("a few"));
    for i in 0..entries {
        let offer = i >= entries / 2;
        msg.push(279, 1u32);
        msg.push(269, if offer { "1" } else { "0" });
        msg.push(55, "ESZ6");
        msg.push(167, "FUT");
        let ticks = u32::try_from(i).expect("a few");
        msg.push(270, format!("9427.{:02}", 50 + ticks).as_str());
        msg.push(271, 10 + ticks);
        msg.push(346, 1 + ticks % 3);
        msg.push(276, "A");
        msg.push(277, "0");
    }
    msg
}

/// Two consecutive refreshes of `entries` levels, encoded: the first, every value explicit; the
/// second, as a feed sends most of them, mostly copied and small deltas from the first.
pub fn encoded(entries: usize) -> (Vec<u8>, Vec<u8>) {
    let mut encoder = Encoder::new(templates());
    let (mut first, mut second) = (Vec::new(), Vec::new());
    encoder.encode_into(1, &refresh(1, entries), &mut first).expect("encodes");
    encoder.encode_into(1, &refresh(2, entries), &mut second).expect("encodes");
    (first, second)
}
