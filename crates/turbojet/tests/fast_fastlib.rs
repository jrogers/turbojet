//! Decoding checked against another implementation: random messages over templates that use
//! every operator, encoded by `fastlib`, decoded by both, and compared field by field.
#![cfg(feature = "fast")]

use std::sync::Arc;

use fastlib::{MessageFactory, TextMessageVisitor, Value};
use turbojet::Message;
use turbojet::fast::{Decoder, Templates};

const TEMPLATES: &str = r#"
<templates xmlns="http://www.fixprotocol.org/ns/fast/td/1.1">
  <template name="Header">
    <uInt32 name="MsgSeqNum" id="34"><increment/></uInt32>
  </template>
  <template name="Quote" id="1">
    <templateRef name="Header"/>
    <string name="MsgType" id="35"><constant value="X"/></string>
    <uInt64 name="SendingTime" id="52"><delta/></uInt64>
    <string name="Symbol" id="55"><copy/></string>
    <string name="Text" id="58" presence="optional"><default/></string>
    <int32 name="I32" id="1001" presence="optional"><copy/></int32>
    <int64 name="I64" id="1002"><delta/></int64>
    <uInt32 name="U32" id="1003" presence="optional"><increment/></uInt32>
    <decimal name="Px" id="1004"><delta/></decimal>
    <decimal name="Qty" id="1005" presence="optional"><copy/></decimal>
    <decimal name="Parts" id="1006" presence="optional">
      <exponent><copy value="-2"/></exponent><mantissa><delta/></mantissa>
    </decimal>
    <string name="Account" id="1007"><tail value="ACCT0000"/></string>
    <string name="Desc" id="1008" charset="unicode" presence="optional"><copy/></string>
    <string name="Venue" id="1009"><delta/></string>
    <group name="Extra" presence="optional">
      <uInt32 name="E1" id="1010"><copy/></uInt32>
      <string name="E2" id="1011"/>
    </group>
    <sequence name="Entries" id="268">
      <length name="NoMDEntries" id="268"/>
      <uInt32 name="Action" id="279"><copy/></uInt32>
      <decimal name="EntryPx" id="270"><delta/></decimal>
      <int32 name="Level" id="1023" presence="optional"><default value="1"/></int32>
      <!-- Mandatory: for an optional one, fastlib writes an empty delta string as a nullable
           string (0x00 0x80), where section 10.7.3 makes only the subtraction length nullable. -->
      <string name="Cond" id="276"><delta/></string>
    </sequence>
  </template>
  <template name="Status" id="2">
    <uInt32 name="MsgSeqNum" id="34"><increment/></uInt32>
    <string name="Status" id="326"><copy dictionary="template"/></string>
  </template>
</templates>"#;

/// The decimal fields, compared by value rather than by text.
const DECIMALS: [u32; 4] = [270, 1004, 1005, 1006];

/// A small deterministic generator (xorshift64*), so a failure replays from its seed.
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[usize::try_from(self.below(items.len() as u64)).expect("small")]
    }
}

/// What a stream's messages carry from one to the next, so operators have something to do.
struct Stream {
    random: Random,
    seq: u64,
    time: u64,
    i64_value: i64,
    u32_value: u64,
    px: i64,
}

impl Stream {
    fn new(seed: u64) -> Self {
        Self { random: Random(seed | 1), seq: 0, time: 1_700_000_000_000, i64_value: 0, u32_value: 0, px: 942_755 }
    }

    /// The next message, in fastlib's text form.
    fn next_message(&mut self) -> String {
        let r = &mut self.random;
        self.seq += if r.chance(90) { 1 } else { r.below(1000) };
        if r.chance(10) {
            return format!("Status=<MsgSeqNum={}|Status={}>", self.seq, r.pick(&["OPEN", "HALT", "CLOSE"]));
        }
        let mut fields = vec![format!("MsgSeqNum={}", self.seq), "MsgType=X".to_string()];
        self.time += r.below(5000);
        fields.push(format!("SendingTime={}", self.time));
        fields.push(format!("Symbol={}", r.pick(&["ESZ6", "ESH7", "NQZ6"])));
        if r.chance(20) {
            fields.push(format!("Text={}", r.pick(&["late", "cancelled", ""])));
        }
        if r.chance(80) {
            fields.push(format!("I32={}", r.pick(&["0", "-1", "7", "2147483647", "-2147483648"])));
        }
        // fastlib's encoder computes deltas in 64 bits and increments without wrapping, where the
        // specification allows wider deltas and wraps (sections 6.3.6 and 6.3.7.1), so values stay
        // where it can encode them; the unit tests cover the rest.
        self.i64_value = match r.below(10) {
            0 => i64::MAX / 4,
            1 => i64::MIN / 4,
            _ => self.i64_value + i64::try_from(r.below(2001)).expect("small") - 1000,
        };
        fields.push(format!("I64={}", self.i64_value));
        if r.chance(80) {
            self.u32_value = if r.chance(70) { (self.u32_value + 1) % u64::from(u32::MAX) } else { r.below(1 << 31) };
            fields.push(format!("U32={}", self.u32_value));
        }
        self.px += i64::try_from(r.below(21)).expect("small") - 10;
        fields.push(format!("Px={}", decimal(self.px, 2)));
        if r.chance(70) {
            fields.push(format!("Qty={}", r.pick(&["1", "10", "0.5", "1000", "250.25"])));
        }
        if r.chance(80) {
            let scale = if r.chance(80) { 2 } else { 3 };
            fields.push(format!("Parts={}", decimal(i64::try_from(r.below(100_000)).expect("small") + 1, scale)));
        }
        fields.push(format!("Account=ACCT{:04}", r.below(10_000)));
        if r.chance(60) {
            fields.push(format!("Desc={}", r.pick(&["café", "東京", "plain", "naïve façade"])));
        }
        fields.push(format!("Venue={}", r.pick(&["XCME", "XCBT", "CME", "XCMEX", "GLBX"])));
        if r.chance(30) {
            fields.push(format!("Extra=<E1={}|E2={}>", r.below(5), r.pick(&["a", "bb", "ccc"])));
        }
        let entries: Vec<String> = (0..1 + r.below(4))
            .map(|_| {
                let mut entry = vec![format!("Action={}", r.below(3)), format!("EntryPx={}", decimal(self.px, 2))];
                if r.chance(50) {
                    entry.push(format!("Level={}", r.below(5)));
                }
                entry.push(format!("Cond={}", r.pick(&["A", "AB", "BA", "ABC", "CAB"])));
                format!("<{}>", entry.join("|"))
            })
            .collect();
        fields.push(format!("Entries={}", entries.concat()));
        format!("Quote=<{}>", fields.join("|"))
    }
}

/// `mantissa` with `scale` digits after the point.
fn decimal(mantissa: i64, scale: u32) -> String {
    let divisor = 10i64.pow(scale);
    let sign = if mantissa < 0 { "-" } else { "" };
    let (whole, fraction) = (mantissa.abs() / divisor, mantissa.abs() % divisor);
    format!("{sign}{whole}.{fraction:0width$}", width = usize::try_from(scale).expect("small"))
}

/// A decimal in normal form (section 6.3.2): no trailing zero in the mantissa.
fn normal(exponent: i32, mantissa: i64) -> (i32, i64) {
    let (mut exponent, mut mantissa) = (exponent, mantissa);
    if mantissa == 0 {
        return (0, 0);
    }
    while mantissa % 10 == 0 {
        mantissa /= 10;
        exponent += 1;
    }
    (exponent, mantissa)
}

/// A FIX decimal's text in normal form.
fn normal_text(text: &str) -> (i32, i64) {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    let mantissa: i64 = format!("{whole}{fraction}").parse().expect("a decimal");
    normal(-i32::try_from(fraction.len()).expect("short"), mantissa)
}

/// The fields fastlib decodes, as tag and text: sequences as their NumInGroup count.
#[derive(Default)]
struct Fields(Vec<(u32, String)>);

impl MessageFactory for Fields {
    fn start_template(&mut self, _id: u32, _name: &str) {
        self.0.clear();
    }
    fn stop_template(&mut self) {}
    fn set_value(&mut self, id: u32, _name: &str, value: Option<Value>) {
        let Some(value) = value else { return };
        let text = match value {
            Value::UInt32(v) => v.to_string(),
            Value::Int32(v) => v.to_string(),
            Value::UInt64(v) => v.to_string(),
            Value::Int64(v) => v.to_string(),
            Value::Decimal(d) => format!("{:?}", normal(d.exponent, d.mantissa)),
            Value::ASCIIString(s) | Value::UnicodeString(s) => s,
            Value::Bytes(_) => unreachable!("no byte vectors in the templates"),
        };
        self.0.push((id, text));
    }
    fn start_sequence(&mut self, id: u32, _name: &str, length: u32) {
        self.0.push((id, length.to_string()));
    }
    fn start_sequence_item(&mut self, _index: u32) {}
    fn stop_sequence_item(&mut self) {}
    fn stop_sequence(&mut self) {}
    fn start_group(&mut self, _name: &str) {}
    fn stop_group(&mut self) {}
    fn start_template_ref(&mut self, _name: &str, _dynamic: bool) {}
    fn stop_template_ref(&mut self) {}
}

/// Our message's fields as tag and text, decimals in normal form, MsgType left out.
fn ours(msg: &Message) -> Vec<(u32, String)> {
    msg.fields()
        .filter(|(tag, _)| *tag != 35)
        .map(|(tag, value)| {
            let text = if DECIMALS.contains(&tag) { format!("{:?}", normal_text(value)) } else { value.to_string() };
            (tag, text)
        })
        .collect()
}

#[test]
fn decodes_what_fastlib_encodes_as_fastlib_decodes_it() {
    const MESSAGES: usize = 1000;
    for seed in 1..=20 {
        let mut stream = Stream::new(seed);
        let mut encoder = fastlib::Encoder::new_from_xml(TEMPLATES).expect("fastlib loads the templates");
        let mut bytes = Vec::new();
        let mut texts = Vec::new();
        for _ in 0..MESSAGES {
            let text = stream.next_message();
            let mut visitor = TextMessageVisitor::from_text(&text).expect("valid text");
            bytes.extend(encoder.encode_vec(&mut visitor).unwrap_or_else(|e| panic!("encodes {text}: {e}")));
            texts.push(text);
        }

        let mut theirs = fastlib::Decoder::new_from_xml(TEMPLATES).expect("fastlib loads the templates");
        let mut decoder = Decoder::new(Arc::new(Templates::from_xml(TEMPLATES).expect("valid templates")));
        let mut msg = Message::default();
        let mut rest = bytes.as_slice();
        for text in &texts {
            let decoded = decoder.decode_into(rest, &mut msg).unwrap_or_else(|e| panic!("seed {seed}, {text}: {e}"));
            let length = rest.len() - decoded.rest.len();
            let mut expected = Fields::default();
            theirs.decode_slice(&rest[..length], &mut expected).unwrap_or_else(|e| panic!("fastlib, {text}: {e}"));
            let expected: Vec<_> = expected.0.into_iter().filter(|(tag, _)| *tag != 35).collect();
            assert_eq!(ours(&msg), expected, "seed {seed}: {text}");
            rest = decoded.rest;
        }
        assert!(rest.is_empty(), "seed {seed}: every message decoded");
    }
}

#[test]
fn fastlib_decodes_what_we_encode() {
    const MESSAGES: usize = 1000;
    for seed in 1..=20 {
        // Messages to encode: fastlib's stream, decoded by us.
        let mut stream = Stream::new(seed);
        let mut theirs = fastlib::Encoder::new_from_xml(TEMPLATES).expect("fastlib loads the templates");
        let mut bytes = Vec::new();
        for _ in 0..MESSAGES {
            let mut visitor = TextMessageVisitor::from_text(&stream.next_message()).expect("valid text");
            bytes.extend(theirs.encode_vec(&mut visitor).expect("encodes"));
        }
        let templates = Arc::new(Templates::from_xml(TEMPLATES).expect("valid templates"));
        let mut decoder = Decoder::new(Arc::clone(&templates));
        let mut encoder = turbojet::fast::Encoder::new(templates);
        let mut ours_encoded = Vec::new();
        let mut expected = Vec::new();
        let mut msg = Message::default();
        let mut rest = bytes.as_slice();
        while !rest.is_empty() {
            let decoded = decoder.decode_into(rest, &mut msg).expect("decodes");
            encoder
                .encode_into(decoded.template_id, &msg, &mut ours_encoded)
                .unwrap_or_else(|e| panic!("seed {seed}: {e}"));
            expected.push(ours(&msg));
            rest = decoded.rest;
        }

        let mut fastlib_decoder = fastlib::Decoder::new_from_xml(TEMPLATES).expect("fastlib loads the templates");
        let mut rest = ours_encoded.as_slice();
        for (i, expected) in expected.iter().enumerate() {
            let mut got = Fields::default();
            let read = fastlib_decoder
                .decode_buffer(rest, &mut got)
                .unwrap_or_else(|e| panic!("seed {seed}, message {i}: {e}"));
            let got: Vec<_> = got.0.into_iter().filter(|(tag, _)| *tag != 35).collect();
            assert_eq!(&got, expected, "seed {seed}, message {i}");
            rest = &rest[usize::try_from(read).expect("a length")..];
        }
        assert!(rest.is_empty(), "seed {seed}: fastlib read every message");
    }
}
