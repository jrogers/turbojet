//! Decoding a stream's messages: presence maps, template identifiers, field operators and the
//! dictionaries of previous values (sections 6.3 and 10).

use std::sync::Arc;

use turbojet::Message;
use turbojet::fields::ToFix;

use crate::convert::EXPONENT_RANGE;
use crate::dictionary::{Got, Slot, State};
use crate::templates::{Field, Group, IntegerType, Kind, MAX_NESTING, Operator, OperatorKind, Template, Type};
use crate::wire::{PresenceMap, Reader, push_ascii};
use crate::{Code, Error, Templates};

/// MsgType(35), which [`Message::reset`] writes from the template.
const MSG_TYPE: u32 = 35;
/// SOH, which ends a FIX field and so can't be in a value.
const SOH: u8 = 0x01;

/// Decodes one FAST stream's messages into [`Message`]s, keeping the previous values its
/// operators refer to. Use one decoder per stream: dictionaries belong to the stream.
#[derive(Debug)]
pub struct Decoder {
    templates: Arc<Templates>,
    slots: Vec<Slot>,
    /// The previous template identifier: the global dictionary's, under a key of its own.
    template_id: Option<u32>,
    /// Where a string or byte vector is decoded before it's written into the message.
    scratch: Vec<u8>,
}

/// What [`Decoder::decode_into`] decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Decoded<'a> {
    /// The template the message was decoded with: a feed may give a template, such as a reset,
    /// a meaning of its own.
    pub template_id: u32,
    /// The input after the message.
    pub rest: &'a [u8],
}

impl Decoder {
    /// A decoder with every previous value undefined.
    pub fn new(templates: Arc<Templates>) -> Self {
        let slots = (0..templates.slots).map(|_| Slot::default()).collect();
        Self { templates, slots, template_id: None, scratch: Vec::new() }
    }

    /// Makes every previous value undefined again (section 6.3.1), as the feed's own rules say
    /// when: at the start of each packet, say, or on a reset message.
    pub fn reset(&mut self) {
        for slot in &mut self.slots {
            slot.state = State::Undefined;
        }
        self.template_id = None;
    }

    /// Decodes the message at the start of `input` into `msg`, replacing what it held but keeping
    /// its allocations.
    ///
    /// # Errors
    ///
    /// A dynamic or reportable error (D or R) from the specification, with the template and
    /// field; [`ErrorKind::Truncated`](crate::ErrorKind::Truncated) if `input` ends inside the
    /// message; [`ErrorKind::InvalidValue`](crate::ErrorKind::InvalidValue) for a string that
    /// isn't UTF-8 or holds SOH. After an error, `msg` holds part of the message, and the
    /// previous values may no longer match the encoder's.
    pub fn decode_into<'a>(&mut self, input: &'a [u8], msg: &mut Message) -> Result<Decoded<'a>, Error> {
        let mut decoding = Decoding {
            reader: Reader::new(input),
            presence: PresenceMap::empty(),
            templates: &self.templates,
            template_id: &mut self.template_id,
            slots: &mut self.slots,
            scratch: &mut self.scratch,
            depth: 0,
            entries: 0,
        };
        decoding.presence = decoding.reader.presence_map()?;
        let (template_id, template) = decoding.identified_template()?;
        msg.reset(template.msg_type.clone());
        decoding.fields(&template.body.fields, msg).map_err(|e| e.with_template(&template.name))?;
        Ok(Decoded { template_id, rest: decoding.reader.rest() })
    }
}

/// The most sequence entries one message may have, nested ones included. A market data message
/// has tens; the limit bounds the work a short message can ask for, since an entry may take no
/// bytes at all.
const MAX_ENTRIES: usize = 100_000;

/// [ERR R8]: more presence map bytes than a segment's fields could use.
fn check_presence_map_length(presence: &PresenceMap<'_>, bits_needed: usize) -> Result<(), Error> {
    let bytes_needed = bits_needed.div_ceil(7).max(1);
    if presence.len_bits() / 7 > bytes_needed {
        return Err(Error::spec(Code::R8));
    }
    Ok(())
}

/// What decoding one message works on.
struct Decoding<'r, 'a, 't> {
    reader: Reader<'a>,
    /// The presence map of the segment being decoded.
    presence: PresenceMap<'a>,
    templates: &'t Templates,
    template_id: &'r mut Option<u32>,
    slots: &'r mut [Slot],
    scratch: &'r mut Vec<u8>,
    /// How deeply groups, sequences and template references nest at this point.
    depth: usize,
    /// The sequence entries decoded so far.
    entries: usize,
}

impl<'a, 't> Decoding<'_, 'a, 't> {
    /// The template a segment's identifier names (section 10.3): read if its presence map bit is
    /// set, else the previous one, as if by a copy operator.
    fn identified_template(&mut self) -> Result<(u32, &'t Template), Error> {
        let id = if self.presence.next_bit() {
            let id = self.reader.unsigned(false)?.expect("a mandatory integer isn't NULL");
            let id = u32::try_from(id).map_err(|_| Error::spec(Code::D2))?;
            *self.template_id = Some(id);
            id
        } else {
            self.template_id.ok_or_else(|| Error::spec(Code::D5).with_detail("no template identifier yet"))?
        };
        let template =
            self.templates.get(id).ok_or_else(|| Error::spec(Code::D9).with_detail(format!("template id {id}")))?;
        check_presence_map_length(&self.presence, 1 + template.body.presence_bits)
            .map_err(|e| e.with_template(&template.name))?;
        Ok((id, template))
    }

    fn fields(&mut self, fields: &'t [Field], msg: &mut Message) -> Result<(), Error> {
        for field in fields {
            self.field(field, msg).map_err(|e| e.with_field(&field.name))?;
        }
        Ok(())
    }

    /// Runs `decode` one level deeper [up to MAX_NESTING].
    fn nested(&mut self, decode: impl FnOnce(&mut Self) -> Result<(), Error>) -> Result<(), Error> {
        if self.depth >= MAX_NESTING {
            return Err(Error::limit(format!("nesting deeper than {MAX_NESTING}")));
        }
        self.depth += 1;
        let result = decode(self);
        self.depth -= 1;
        result
    }

    /// A group's or sequence element's fields, under a presence map of their own if they use one.
    fn segment(&mut self, group: &'t Group, msg: &mut Message) -> Result<(), Error> {
        self.nested(|this| {
            if group.presence_bits == 0 {
                return this.fields(&group.fields, msg);
            }
            let presence = this.reader.presence_map()?;
            check_presence_map_length(&presence, group.presence_bits)?;
            let outer = std::mem::replace(&mut this.presence, presence);
            let result = this.fields(&group.fields, msg);
            this.presence = outer;
            result
        })
    }

    /// A sequence: its length, written as the NumInGroup field, then each element.
    fn sequence(
        &mut self,
        field: &Field,
        length: &Operator,
        length_tag: u32,
        element: &'t Group,
        msg: &mut Message,
    ) -> Result<(), Error> {
        let uint32 = Type::Integer(IntegerType::UInt32);
        let count = self.operate(length, field.optional, uint32, |reader, _| {
            integer_value(reader, IntegerType::UInt32, field.optional)
        })?;
        let Some(Got::Integer(count)) = count else { return Ok(()) };
        let count = u32::try_from(count).expect("a uInt32 is in range");
        self.entries = self.entries.saturating_add(usize::try_from(count).unwrap_or(usize::MAX));
        if self.entries > MAX_ENTRIES {
            return Err(Error::limit(format!("more than {MAX_ENTRIES} sequence entries")));
        }
        msg.push(length_tag, count);
        for _ in 0..count {
            self.segment(element, msg)?;
        }
        Ok(())
    }

    /// A dynamic template reference (section 6.4): a segment of the template it identifies.
    fn dynamic_reference(&mut self, msg: &mut Message) -> Result<(), Error> {
        self.nested(|this| {
            let presence = this.reader.presence_map()?;
            let outer = std::mem::replace(&mut this.presence, presence);
            let result = this.identified_template().and_then(|(_, template)| {
                this.fields(&template.body.fields, msg).map_err(|e| e.with_template(&template.name))
            });
            this.presence = outer;
            result
        })
    }

    /// Decodes one field and writes it into `msg`, unless it's absent or has no tag.
    fn field(&mut self, field: &'t Field, msg: &mut Message) -> Result<(), Error> {
        let optional = field.optional;
        let value = match &field.kind {
            Kind::Group(group) => {
                if optional && !self.presence.next_bit() {
                    return Ok(());
                }
                return self.segment(group, msg);
            }
            Kind::Sequence { length, length_tag, element } => {
                return self.sequence(field, length, *length_tag, element, msg);
            }
            Kind::TemplateRef => return self.dynamic_reference(msg),
            Kind::Integer(integer, operator) => {
                let ty = Type::Integer(*integer);
                self.operate(operator, optional, ty, |reader, _| integer_value(reader, *integer, optional))?
            }
            Kind::Decimal(operator) => {
                self.operate(operator, optional, Type::Decimal, |reader, _| decimal(reader, optional))?
            }
            Kind::DecimalParts { exponent, mantissa } => self.decimal_parts(exponent, mantissa, optional)?,
            Kind::Ascii(operator) => self.operate(operator, optional, Type::Ascii, |reader, scratch| {
                let Some(chars) = reader.ascii(optional)? else { return Ok(None) };
                scratch.clear();
                push_ascii(chars, scratch);
                Ok(Some(Got::Bytes(scratch)))
            })?,
            Kind::Unicode(operator) | Kind::Bytes { operator, .. } => {
                let ty = if matches!(field.kind, Kind::Unicode(_)) { Type::Unicode } else { Type::Bytes };
                self.operate(operator, optional, ty, |reader, _| Ok(reader.bytes(optional)?.map(Got::Bytes)))?
            }
        };
        let (Some(value), Some(tag)) = (value, field.tag) else { return Ok(()) };
        if tag != MSG_TYPE {
            write(msg, tag, &field.kind, value)?;
        }
        Ok(())
    }

    fn decimal_parts(
        &mut self,
        exponent: &Operator,
        mantissa: &Operator,
        optional: bool,
    ) -> Result<Option<Got<'_>>, Error> {
        let int32 = Type::Integer(IntegerType::Int32);
        let exponent =
            self.operate(exponent, optional, int32, |reader, _| integer_value(reader, IntegerType::Int32, optional))?;
        // The mantissa, its presence map bit included, is there only if the exponent is.
        let Some(Got::Integer(exponent)) = exponent else { return Ok(None) };
        let int64 = Type::Integer(IntegerType::Int64);
        let mantissa =
            self.operate(mantissa, false, int64, |reader, _| integer_value(reader, IntegerType::Int64, false))?;
        let Some(Got::Integer(mantissa)) = mantissa else { unreachable!("a mandatory field is present or an error") };
        let exponent = i32::try_from(exponent).expect("an int32 field's value is in range");
        let mantissa = i64::try_from(mantissa).expect("an int64 field's value is in range");
        if !EXPONENT_RANGE.contains(&exponent) {
            return Err(Error::spec(Code::R1));
        }
        Ok(Some(Got::Decimal { exponent, mantissa }))
    }

    /// Applies `operator` (sections 6.3 and 10.5.1), reading the value from the stream with `read`
    /// where the operator says it's there. `None` is an absent value.
    fn operate<'s>(
        &'s mut self,
        operator: &'s Operator,
        optional: bool,
        ty: Type,
        read: impl FnOnce(&mut Reader<'a>, &'s mut Vec<u8>) -> Result<Option<Got<'s>>, Error>,
    ) -> Result<Option<Got<'s>>, Error> {
        let initial = operator.initial.as_ref().map(Got::from);
        match operator.kind {
            OperatorKind::None => read(&mut self.reader, self.scratch),
            OperatorKind::Constant if optional && !self.presence.next_bit() => Ok(None),
            OperatorKind::Constant => Ok(initial),
            OperatorKind::Default if self.presence.next_bit() => read(&mut self.reader, self.scratch),
            OperatorKind::Default => Ok(initial),
            OperatorKind::Delta => self.delta(operator, optional, ty),
            OperatorKind::Copy | OperatorKind::Increment | OperatorKind::Tail => {
                let slot = &mut self.slots[operator.slot.expect("the operator keeps a previous value")];
                if !self.presence.next_bit() {
                    return slot.unchanged(operator.kind, ty, initial, optional);
                }
                let Some(value) = read(&mut self.reader, self.scratch)? else {
                    slot.state = State::Empty;
                    return Ok(None);
                };
                match (operator.kind, value) {
                    (OperatorKind::Tail, Got::Bytes(tail)) => {
                        slot.base(ty, initial, false)?;
                        slot.tail(tail);
                        slot.check_utf8(ty)?;
                    }
                    _ => slot.assign(ty, value),
                }
                Ok(Some(slot.value()))
            }
        }
    }

    /// The delta operator (section 6.3.7): a difference from the previous value in the stream,
    /// always, with no presence map bit.
    fn delta<'s>(&'s mut self, operator: &'s Operator, optional: bool, ty: Type) -> Result<Option<Got<'s>>, Error> {
        let initial = operator.initial.as_ref().map(Got::from);
        let slot = &mut self.slots[operator.slot.expect("a delta keeps a previous value")];
        // A NULL delta leaves the previous value as it was.
        let Some(delta) = self.reader.signed_wide(optional)? else { return Ok(None) };
        match ty {
            Type::Integer(integer) => {
                slot.base(ty, initial, true)?;
                let (min, max) = integer.range();
                let combined = slot.number + delta;
                if !(min..=max).contains(&combined) {
                    return Err(Error::spec(Code::R4));
                }
                slot.number = combined;
            }
            Type::Decimal => {
                let mantissa = self.reader.signed_wide(false)?.expect("a mandatory integer isn't NULL");
                slot.base(ty, initial, true)?;
                let exponent =
                    i32::try_from(i128::from(slot.exponent) + delta).ok().filter(|e| EXPONENT_RANGE.contains(e));
                let mantissa = slot.number + mantissa;
                let (Some(exponent), true) = (exponent, i64::try_from(mantissa).is_ok()) else {
                    return Err(Error::spec(Code::R1));
                };
                slot.exponent = exponent;
                slot.number = mantissa;
            }
            Type::Ascii | Type::Unicode | Type::Bytes => {
                let subtraction = i32::try_from(delta).map_err(|_| Error::spec(Code::D7))?;
                let part = if ty == Type::Ascii {
                    let chars = self.reader.ascii(false)?.expect("a mandatory string isn't NULL");
                    self.scratch.clear();
                    push_ascii(chars, self.scratch);
                    self.scratch.as_slice()
                } else {
                    self.reader.bytes(false)?.expect("a mandatory byte vector isn't NULL")
                };
                slot.base(ty, initial, true)?;
                slot.splice(subtraction, part)?;
                slot.check_utf8(ty)?;
            }
        }
        Ok(Some(slot.value()))
    }
}

/// An integer field's value from the stream, checked against its type's range [ERR D2].
fn integer_value<'b>(reader: &mut Reader<'_>, integer: IntegerType, nullable: bool) -> Result<Option<Got<'b>>, Error> {
    let value = if integer.is_signed() {
        reader.signed(nullable)?.map(i128::from)
    } else {
        reader.unsigned(nullable)?.map(i128::from)
    };
    let Some(value) = value else { return Ok(None) };
    let (min, max) = integer.range();
    if !(min..=max).contains(&value) {
        return Err(Error::spec(Code::D2));
    }
    Ok(Some(Got::Integer(value)))
}

/// A scaled number (section 10.6.2): an exponent, NULL if the field is absent, then a mantissa.
fn decimal<'b>(reader: &mut Reader<'_>, nullable: bool) -> Result<Option<Got<'b>>, Error> {
    let Some(Got::Integer(exponent)) = integer_value(reader, IntegerType::Int32, nullable)? else { return Ok(None) };
    let Some(Got::Integer(mantissa)) = integer_value(reader, IntegerType::Int64, false)? else {
        unreachable!("a mandatory integer isn't NULL")
    };
    let exponent = i32::try_from(exponent).expect("an int32 is in range");
    if !EXPONENT_RANGE.contains(&exponent) {
        return Err(Error::spec(Code::R1));
    }
    let mantissa = i64::try_from(mantissa).expect("an int64 is in range");
    Ok(Some(Got::Decimal { exponent, mantissa }))
}

/// Writes a field's value into the message as FIX text.
fn write(msg: &mut Message, tag: u32, kind: &Kind, value: Got<'_>) -> Result<(), Error> {
    match (kind, value) {
        (Kind::Integer(integer, _), Got::Integer(n)) if integer.is_signed() => {
            msg.push(tag, i64::try_from(n).expect("checked against the field's range"));
        }
        (Kind::Integer(..), Got::Integer(n)) => {
            msg.push(tag, u64::try_from(n).expect("checked against the field's range"))
        }
        (Kind::Decimal(_) | Kind::DecimalParts { .. }, Got::Decimal { exponent, mantissa }) => {
            msg.push(tag, Scaled { exponent, mantissa });
        }
        (Kind::Ascii(_) | Kind::Unicode(_), Got::Bytes(bytes)) => {
            let text = std::str::from_utf8(bytes).map_err(|_| Error::invalid_value("not UTF-8"))?;
            if bytes.contains(&SOH) {
                return Err(Error::invalid_value("holds SOH"));
            }
            msg.push(tag, text);
        }
        (Kind::Bytes { length_tag: Some(length_tag), .. }, Got::Bytes(bytes)) => msg.push_data(*length_tag, tag, bytes),
        (Kind::Bytes { length_tag: None, .. }, Got::Bytes(bytes)) => msg.push(tag, Hex(bytes)),
        _ => unreachable!("a field's value has the field's type"),
    }
    Ok(())
}

/// A scaled number written as a FIX decimal: `mantissa * 10^exponent`.
struct Scaled {
    exponent: i32,
    mantissa: i64,
}

impl ToFix for Scaled {
    fn write_fix(&self, out: &mut String) {
        if self.mantissa < 0 {
            out.push('-');
        }
        let digits = self.mantissa.unsigned_abs();
        let length = usize::try_from(digits.checked_ilog10().map_or(1, |log| log + 1)).expect("at most 20 digits");
        let zeros = |out: &mut String, n: usize| out.extend(std::iter::repeat_n('0', n));
        if self.exponent >= 0 {
            digits.write_fix(out);
            if digits == 0 {
                // Zero, however it's scaled: not "000".
                return;
            }
            zeros(out, usize::try_from(self.exponent).expect("non-negative"));
            return;
        }
        let fraction = usize::try_from(self.exponent.unsigned_abs()).expect("at most 63");
        if length > fraction {
            let start = out.len();
            digits.write_fix(out);
            out.insert(start + length - fraction, '.');
        } else {
            out.push_str("0.");
            zeros(out, fraction - length);
            digits.write_fix(out);
        }
    }
}

/// A byte vector written as lower-case hexadecimal (section 8.4).
struct Hex<'b>(&'b [u8]);

impl ToFix for Hex<'_> {
    fn write_fix(&self, out: &mut String) {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        for b in self.0 {
            out.push(char::from(DIGITS[usize::from(b >> 4)]));
            out.push(char::from(DIGITS[usize::from(b & 0x0f)]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorKind;

    /// A decoder for template 1 holding `fields`.
    fn decoder(fields: &str) -> Decoder {
        let xml = format!(r#"<templates><template name="T" id="1">{fields}</template></templates>"#);
        Decoder::new(Arc::new(Templates::from_xml(&xml).expect("valid templates")))
    }

    /// A message of template 1: its presence map (the template identifier's bit, then `bits`),
    /// the identifier, then `body`.
    fn message(bits: &[bool], body: &[u8]) -> Vec<u8> {
        let all: Vec<bool> = [true].iter().chain(bits).copied().collect();
        let mut map: Vec<u8> = all
            .chunks(7)
            .map(|chunk| chunk.iter().enumerate().fold(0, |byte, (i, &bit)| byte | (u8::from(bit) << (6 - i))))
            .collect();
        while map.len() > 1 && map.last() == Some(&0) {
            map.pop();
        }
        *map.last_mut().expect("at least one byte") |= 0x80;
        [map.as_slice(), &[0x81], body].concat()
    }

    /// Decodes one message, which must use all of `input`, and returns tag 1's value.
    fn value(decoder: &mut Decoder, input: &[u8]) -> Result<Option<String>, Error> {
        let mut msg = Message::default();
        let decoded = decoder.decode_into(input, &mut msg)?;
        assert!(decoded.rest.is_empty(), "the whole message is read");
        Ok(msg.get(1).map(str::to_string))
    }

    fn ok(s: &str) -> Result<Option<String>, Error> {
        Ok(Some(s.to_string()))
    }

    #[test]
    fn decimals_as_in_appendix_3_1_5() {
        let mut mandatory = decoder(r#"<decimal id="1" name="Value"/>"#);
        assert_eq!(value(&mut mandatory, &message(&[], &[0x82, 0x39, 0x45, 0xa3])), ok("94275500"));
        assert_eq!(value(&mut mandatory, &message(&[], &[0x81, 0x04, 0x3f, 0x34, 0xde])), ok("94275500"));
        assert_eq!(value(&mut mandatory, &message(&[], &[0xfe, 0x39, 0x45, 0xa3])), ok("9427.55"));
        let mut optional = decoder(r#"<decimal id="1" presence="optional" name="Value"/>"#);
        assert_eq!(value(&mut optional, &message(&[], &[0x83, 0x39, 0x45, 0xa3])), ok("94275500"));
        assert_eq!(value(&mut optional, &message(&[], &[0xfe, 0x46, 0x3a, 0xdd])), ok("-9427.55"));
        // The specification has 0x73 for 0x7f here too, as in its integer example.
        assert_eq!(value(&mut optional, &message(&[], &[0xfd, 0x7f, 0x3f, 0xff])), ok("-8.193"));
        assert_eq!(value(&mut optional, &message(&[], &[0x80])), Ok(None));
        let mut copied = decoder(r#"<decimal id="1" presence="optional" name="Value"><copy/></decimal>"#);
        assert_eq!(value(&mut copied, &message(&[true], &[0xfe, 0x39, 0x45, 0xa3])), ok("9427.55"));
    }

    #[test]
    fn decimal_parts_with_copies_as_in_appendix_3_2_6() {
        let mut decoder = decoder(
            r#"<decimal id="1" presence="optional" name="Value">
                 <exponent><copy/></exponent><mantissa><copy/></mantissa>
               </decimal>"#,
        );
        // The specification shows the first mantissa as 0x39 0x45 0xa4, a typo: the mantissa is
        // mandatory, so 942755 is 0x39 0x45 0xa3, as 942760 is 0x39 0x45 0xa8 in the next row.
        assert_eq!(value(&mut decoder, &message(&[true, true], &[0xfe, 0x39, 0x45, 0xa3])), ok("9427.55"));
        assert_eq!(value(&mut decoder, &message(&[false, true], &[0x39, 0x45, 0xa8])), ok("9427.60"));
        // A NULL exponent: the mantissa, and its bit, aren't there.
        assert_eq!(value(&mut decoder, &message(&[true], &[0x80])), Ok(None));
    }

    #[test]
    fn constants_as_in_appendix_3_2_1() {
        let mut mandatory = decoder(r#"<uInt32 id="1" name="Flag"><constant value="0"/></uInt32>"#);
        assert_eq!(value(&mut mandatory, &message(&[], &[])), ok("0"));
        let mut optional = decoder(r#"<uInt32 id="1" presence="optional" name="Flag"><constant value="0"/></uInt32>"#);
        assert_eq!(value(&mut optional, &message(&[true], &[])), ok("0"));
        assert_eq!(value(&mut optional, &message(&[false], &[])), Ok(None));
    }

    #[test]
    fn defaults_as_in_appendix_3_2_2() {
        let mut mandatory = decoder(r#"<uInt32 id="1" name="Flag"><default value="0"/></uInt32>"#);
        assert_eq!(value(&mut mandatory, &message(&[false], &[])), ok("0"));
        assert_eq!(value(&mut mandatory, &message(&[true], &[0x81])), ok("1"));
        let mut optional = decoder(r#"<uInt32 id="1" presence="optional" name="Flag"><default/></uInt32>"#);
        assert_eq!(value(&mut optional, &message(&[false], &[])), Ok(None));
    }

    #[test]
    fn copies_as_in_appendix_3_2_3() {
        let mut mandatory = decoder(r#"<string id="1" name="Flag"><copy/></string>"#);
        assert_eq!(value(&mut mandatory, &message(&[true], &[0x43, 0x4d, 0xc5])), ok("CME"));
        assert_eq!(value(&mut mandatory, &message(&[false], &[])), ok("CME"));
        assert_eq!(value(&mut mandatory, &message(&[true], &[0x49, 0x53, 0xc5])), ok("ISE"));
        let mut optional = decoder(r#"<string id="1" presence="optional" name="Flag"><copy/></string>"#);
        assert_eq!(value(&mut optional, &message(&[true], &[0x80])), Ok(None));
        assert_eq!(value(&mut optional, &message(&[false], &[])), Ok(None));
        assert_eq!(value(&mut optional, &message(&[true], &[0x43, 0x4d, 0xc5])), ok("CME"));
    }

    #[test]
    fn increments_as_in_appendix_3_2_4() {
        let mut decoder = decoder(r#"<uInt32 id="1" name="Flag"><increment value="1"/></uInt32>"#);
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("1"));
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("2"));
        assert_eq!(value(&mut decoder, &message(&[true], &[0x84])), ok("4"));
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("5"));
    }

    #[test]
    fn increments_wrap_at_the_types_maximum() {
        let mut decoder = decoder(r#"<int32 id="1" name="A"><increment value="2147483647"/></int32>"#);
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("2147483647"));
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("-2147483648"));
    }

    #[test]
    fn integer_deltas_as_in_appendix_3_2_5() {
        let mut decoder = decoder(r#"<int32 id="1" name="Price"><delta/></int32>"#);
        assert_eq!(value(&mut decoder, &message(&[], &[0x39, 0x45, 0xa3])), ok("942755"));
        assert_eq!(value(&mut decoder, &message(&[], &[0xfb])), ok("942750"));
        assert_eq!(value(&mut decoder, &message(&[], &[0xfb])), ok("942745"));
        assert_eq!(value(&mut decoder, &message(&[], &[0x80])), ok("942745"));
    }

    #[test]
    fn decimal_deltas_as_in_appendix_3_2_5() {
        let mut plain = decoder(r#"<decimal id="1" name="Price"><delta/></decimal>"#);
        assert_eq!(value(&mut plain, &message(&[], &[0xfe, 0x39, 0x45, 0xa3])), ok("9427.55"));
        assert_eq!(value(&mut plain, &message(&[], &[0x80, 0xfc])), ok("9427.51"));
        assert_eq!(value(&mut plain, &message(&[], &[0x80, 0xfb])), ok("9427.46"));
        // From an initial value, normalised to 12 * 10^3.
        let mut with_initial = decoder(r#"<decimal id="1" name="Price"><delta value="12000"/></decimal>"#);
        assert_eq!(value(&mut with_initial, &message(&[], &[0xfe, 0x09, 0xae])), ok("12100"));
        assert_eq!(value(&mut with_initial, &message(&[], &[0x80, 0x85])), ok("12150"));
        assert_eq!(value(&mut with_initial, &message(&[], &[0x80, 0x85])), ok("12200"));
    }

    #[test]
    fn string_deltas_as_in_appendix_3_2_5() {
        let mut decoder = decoder(r#"<string id="1" name="Security"><delta/></string>"#);
        assert_eq!(value(&mut decoder, &message(&[], &[0x80, 0x47, 0x45, 0x48, 0xb6])), ok("GEH6"));
        assert_eq!(value(&mut decoder, &message(&[], &[0x82, 0x4d, 0xb6])), ok("GEM6"));
        // Negative lengths remove from the front, less one: -3 removes two, -1 (-0) none.
        assert_eq!(value(&mut decoder, &message(&[], &[0xfd, 0x45, 0xd3])), ok("ESM6"));
        assert_eq!(value(&mut decoder, &message(&[], &[0xff, 0x52, 0xd3])), ok("RSESM6"));
    }

    #[test]
    fn an_optional_delta_strings_part_is_not_nullable() {
        // Section 10.7.3: only the subtraction length is nullable. 0x82 (remove one, nullable)
        // then 0x80, an empty string; 0x00 0x80 would be "\0".
        let mut decoder = decoder(r#"<string id="1" presence="optional" name="A"><delta value="AB"/></string>"#);
        assert_eq!(value(&mut decoder, &message(&[], &[0x82, 0x80])), ok("A"));
        assert_eq!(value(&mut decoder, &message(&[], &[0x81, 0x00, 0x80])), ok("A\0"));
    }

    #[test]
    fn integer_deltas_can_be_wider_than_their_field() {
        // Section 6.3.7.1's note: a uInt64 going from 0 to its maximum and back needs 65 bits.
        let mut decoder = decoder(r#"<uInt64 id="1" name="A"><delta/></uInt64>"#);
        let up = [&[0x01][..], &[0x7f; 8], &[0xff]].concat();
        assert_eq!(value(&mut decoder, &message(&[], &up)), ok("18446744073709551615"));
        let down = [&[0x7e][..], &[0x00; 8], &[0x81]].concat();
        assert_eq!(value(&mut decoder, &message(&[], &down)), ok("0"));
    }

    #[test]
    fn delta_errors() {
        let mut d = decoder(r#"<string id="1" name="A"><delta value="AB"/></string>"#);
        let code = |d: &mut Decoder, body: &[u8]| value(d, &message(&[], body)).unwrap_err().code();
        // Removing three from a base of two.
        assert_eq!(code(&mut d, &[0x83, 0x80]), Some(Code::D7));
        let mut d = decoder(r#"<uInt32 id="1" name="A"><delta/></uInt32>"#);
        assert_eq!(code(&mut d, &[0xff]), Some(Code::R4));
        let mut d = decoder(r#"<decimal id="1" name="A"><delta/></decimal>"#);
        assert_eq!(code(&mut d, &[0x00, 0xc0, 0x81]), Some(Code::R1));
        let mut d = decoder(r#"<string id="1" name="A" charset="unicode"><delta/></string>"#);
        assert_eq!(code(&mut d, &[0x80, 0x81, 0xc3]), Some(Code::R2));
    }

    #[test]
    fn a_null_delta_is_absent_and_leaves_the_previous_value() {
        let mut decoder = decoder(r#"<uInt32 id="1" presence="optional" name="A"><delta/></uInt32>"#);
        assert_eq!(value(&mut decoder, &message(&[], &[0x86])), ok("5"));
        assert_eq!(value(&mut decoder, &message(&[], &[0x80])), Ok(None));
        assert_eq!(value(&mut decoder, &message(&[], &[0x82])), ok("6"));
    }

    #[test]
    fn a_delta_from_an_empty_previous_value_is_an_error() {
        let mut decoder = decoder(
            r#"<uInt32 id="1" presence="optional" name="A"><copy key="k"/></uInt32>
               <uInt32 id="2" name="B"><delta key="k"/></uInt32>"#,
        );
        let error = value(&mut decoder, &message(&[true], &[0x80, 0x81])).unwrap_err();
        assert_eq!(error.code(), Some(Code::D6));
    }

    #[test]
    fn tails_replace_the_end_of_the_previous_value() {
        let mut decoder = decoder(r#"<string id="1" presence="optional" name="A"><tail value="ABCD"/></string>"#);
        // From the initial value, then from each previous one.
        assert_eq!(value(&mut decoder, &message(&[true], &[0x58, 0xd9])), ok("ABXY"));
        assert_eq!(value(&mut decoder, &message(&[false], &[])), ok("ABXY"));
        assert_eq!(value(&mut decoder, &message(&[true], &[0xda])), ok("ABXZ"));
        // A tail longer than the base replaces it.
        assert_eq!(value(&mut decoder, &message(&[true], &[0x31, 0x32, 0x33, 0x34, 0xb5])), ok("12345"));
        // NULL: absent, and the previous value empty, so the next tail starts from the initial value.
        assert_eq!(value(&mut decoder, &message(&[true], &[0x80])), Ok(None));
        assert_eq!(value(&mut decoder, &message(&[false], &[])), Ok(None));
        assert_eq!(value(&mut decoder, &message(&[true], &[0xda])), ok("ABCZ"));
    }

    #[test]
    fn mandatory_tails_without_a_previous_value_are_errors() {
        let mut decoder = decoder(r#"<byteVector id="1" name="A"><tail/></byteVector>"#);
        let error = value(&mut decoder, &message(&[false], &[])).unwrap_err();
        assert_eq!(error.code(), Some(Code::D5));
        assert_eq!(value(&mut decoder, &message(&[true], &[0x81, 0xab])), ok("ab"));
    }

    /// Decodes `input` against `xml`, returning every field written, as text.
    fn decode_all(xml: &str, input: &[u8]) -> Result<Vec<(u32, String)>, Error> {
        let mut decoder = Decoder::new(Arc::new(Templates::from_xml(xml).expect("valid templates")));
        let mut msg = Message::default();
        let decoded = decoder.decode_into(input, &mut msg)?;
        assert!(decoded.rest.is_empty(), "the whole message is read");
        Ok(msg.fields().map(|(tag, value)| (tag, value.to_string())).collect())
    }

    fn fields(pairs: &[(u32, &str)]) -> Vec<(u32, String)> {
        pairs.iter().map(|&(tag, value)| (tag, value.to_string())).collect()
    }

    #[test]
    fn optional_groups_take_a_bit_and_bring_their_own_presence_map() {
        let xml = r#"<template name="T" id="1">
                       <group name="G" presence="optional"><uInt32 id="2" name="A"><copy/></uInt32></group>
                       <uInt32 id="3" name="B"/>
                     </template>"#;
        // Present: the group's bit, then its own map with A's bit.
        assert_eq!(decode_all(xml, &[0xe0, 0x81, 0xc0, 0x85, 0x86]), Ok(fields(&[(35, ""), (2, "5"), (3, "6")])));
        // Absent: no map of its own.
        assert_eq!(decode_all(xml, &[0xc0, 0x81, 0x86]), Ok(fields(&[(35, ""), (3, "6")])));
    }

    #[test]
    fn sequences_write_a_repeating_group() {
        let xml = r#"<template name="T" id="1">
                       <sequence name="S" presence="optional">
                         <length name="N" id="9"/>
                         <uInt32 id="2" name="A"><copy/></uInt32>
                         <string id="3" name="B"/>
                       </sequence>
                     </template>"#;
        // Two entries, each with its own map: A sent, then copied.
        let input = [0xc0, 0x81, 0x83, 0xc0, 0x85, 0xd8, 0x80, 0xd9];
        let expected = fields(&[(35, ""), (9, "2"), (2, "5"), (3, "X"), (2, "5"), (3, "Y")]);
        assert_eq!(decode_all(xml, &input), Ok(expected));
        // A NULL length: no group at all.
        assert_eq!(decode_all(xml, &[0xc0, 0x81, 0x80]), Ok(fields(&[(35, "")])));
    }

    #[test]
    fn dynamic_references_decode_the_template_they_name() {
        let xml = r#"<templates>
                       <template name="Outer" id="1"><uInt32 id="2" name="A"/><templateRef/></template>
                       <template name="Inner" id="2"><uInt32 id="3" name="B"><copy/></uInt32></template>
                     </templates>"#;
        // Outer, A = 1, then a segment: map (identifier, B), identifier 2, B = 7.
        let input = [0xc0, 0x81, 0x81, 0xe0, 0x82, 0x87];
        assert_eq!(decode_all(xml, &input), Ok(fields(&[(35, ""), (2, "1"), (3, "7")])));
    }

    #[test]
    fn work_a_message_can_ask_for_is_bounded() {
        // Entries that take no bytes, as many as a length can say.
        let xml = r#"<template name="T" id="1">
                       <sequence name="S"><length name="N" id="9"/><uInt32 id="2" name="A"><constant value="1"/></uInt32></sequence>
                     </template>"#;
        let error = decode_all(xml, &[0xc0, 0x81, 0x0f, 0x7f, 0x7f, 0x7f, 0xff]).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        // A template that refers to itself, again and again.
        let xml = r#"<template name="T" id="1"><templateRef/></template>"#;
        let input: Vec<u8> = std::iter::repeat_n([0xc0, 0x81], MAX_NESTING + 2).flatten().collect();
        assert_eq!(decode_all(xml, &input).unwrap_err().kind(), ErrorKind::Limit);
    }

    #[test]
    fn copies_start_from_the_initial_value_and_reset_to_undefined() {
        let mut with_initial = decoder(r#"<uInt32 id="1" name="A"><copy value="7"/></uInt32>"#);
        assert_eq!(value(&mut with_initial, &message(&[false], &[])), ok("7"));
        let mut without = decoder(r#"<uInt32 id="1" name="A"><copy/></uInt32>"#);
        assert_eq!(value(&mut without, &message(&[false], &[])).map_err(|e| e.code()), Err(Some(Code::D5)));
        assert_eq!(value(&mut without, &message(&[true], &[0x85])), ok("5"));
        assert_eq!(value(&mut without, &message(&[false], &[])), ok("5"));
        without.reset();
        assert_eq!(value(&mut without, &message(&[false], &[])).map_err(|e| e.code()), Err(Some(Code::D5)));
    }

    #[test]
    fn previous_values_are_shared_by_key_and_checked_for_type() {
        // An optional field leaves the shared entry empty; a mandatory one then can't copy it.
        let mut empty = decoder(
            r#"<uInt32 id="1" presence="optional" name="A"><copy key="k"/></uInt32>
               <uInt32 id="2" name="B"><copy key="k"/></uInt32>"#,
        );
        let error = value(&mut empty, &message(&[true, false], &[0x80])).unwrap_err();
        assert_eq!((error.code(), error.field()), (Some(Code::D6), Some("B")));
        let mut mismatch = decoder(
            r#"<uInt32 id="1" name="A"><copy key="k"/></uInt32>
               <string id="2" name="B"><copy key="k"/></string>"#,
        );
        let error = value(&mut mismatch, &message(&[true, false], &[0x85])).unwrap_err();
        assert_eq!(error.code(), Some(Code::D4));
    }

    #[test]
    fn template_identifiers_are_copied_and_must_be_known() {
        let mut decoder = decoder(r#"<uInt32 id="1" name="A"/>"#);
        let error = decoder.decode_into(&[0x80, 0x81], &mut Message::default()).unwrap_err();
        assert_eq!(error.code(), Some(Code::D5));
        let error = decoder.decode_into(&[0xc0, 0x82, 0x81], &mut Message::default()).unwrap_err();
        assert_eq!(error.code(), Some(Code::D9));
        assert_eq!(value(&mut decoder, &message(&[], &[0x81])), ok("1"));
        // The identifier is now copied.
        let mut msg = Message::default();
        let decoded = decoder.decode_into(&[0x80, 0x82], &mut msg).expect("decodes");
        assert_eq!((decoded.template_id, msg.get(1)), (1, Some("2")));
    }

    #[test]
    fn a_presence_map_longer_than_the_template_needs_is_reported() {
        let mut decoder = decoder(r#"<uInt32 id="1" name="A"><copy/></uInt32>"#);
        let error = decoder.decode_into(&[0x60, 0x81, 0x81, 0x85], &mut Message::default()).unwrap_err();
        assert_eq!(error.code(), Some(Code::R8));
    }

    #[test]
    fn values_are_written_as_fix_text() {
        let mut decoder = decoder(
            r#"<string name="MsgType" id="35"><constant value="X"/></string>
               <int32 id="2" name="Signed"/>
               <byteVector id="3" name="Hex"/>
               <byteVector id="96" name="RawData"><length name="RawDataLength" id="95"/></byteVector>
               <string id="5" name="Text" charset="unicode"/>
               <uInt32 name="NoTag"/>"#,
        );
        let mut msg = Message::default();
        let body = [&[0xff][..], &[0x82, 0xab, 0x01], &[0x82, 0x00, 0xff], &[0x82], "é".as_bytes(), &[0x81]].concat();
        decoder.decode_into(&message(&[], &body), &mut msg).expect("decodes");
        assert_eq!(msg.msg_type().code(), "X");
        let fields: Vec<(u32, &[u8])> = msg.fields_bytes().collect();
        let expected: [(u32, &[u8]); 6] =
            [(35, b"X"), (2, b"-1"), (3, b"ab01"), (95, b"2"), (96, &[0x00, 0xff]), (5, "é".as_bytes())];
        assert_eq!(fields, expected);
    }

    #[test]
    fn decimals_are_written_with_their_scale() {
        let cases =
            [(0, 0, "0"), (3, 12, "12000"), (-2, 5, "0.05"), (-2, -12345, "-123.45"), (-3, 0, "0.000"), (37, 0, "0")];
        for (exponent, mantissa, text) in cases {
            assert_eq!(Scaled { exponent, mantissa }.to_fix(), text);
        }
    }

    #[test]
    fn strings_that_cant_be_fix_values_are_refused() {
        let mut ascii = decoder(r#"<string id="1" name="A"/>"#);
        let error = value(&mut ascii, &message(&[], &[0x41, 0x81])).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
        let mut unicode = decoder(r#"<string id="1" name="A" charset="unicode"/>"#);
        let error = value(&mut unicode, &message(&[], &[0x81, 0xff])).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn a_truncated_message_is_an_error() {
        let mut decoder = decoder(r#"<uInt32 id="1" name="A"/><string id="2" name="B"/>"#);
        let error = decoder.decode_into(&message(&[], &[0x81, 0x41]), &mut Message::default()).unwrap_err();
        assert_eq!((error.kind(), error.field()), (ErrorKind::Truncated, Some("B")));
    }

    #[test]
    fn messages_follow_one_another_and_reuse_the_message() {
        let mut decoder = decoder(r#"<uInt32 id="1" presence="optional" name="A"/>"#);
        let input = [message(&[], &[0x82]), message(&[], &[0x80])].concat();
        let mut msg = Message::default();
        let decoded = decoder.decode_into(&input, &mut msg).expect("first");
        assert_eq!(msg.get(1), Some("1"));
        decoder.decode_into(decoded.rest, &mut msg).expect("second");
        assert_eq!(msg.get(1), None);
    }
}
