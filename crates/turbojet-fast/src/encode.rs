//! Encoding [`Message`]s as FAST: the decoder's work in reverse, keeping the same previous values
//! so a decoder reading the stream ends up where the encoder is.

use std::sync::Arc;

use turbojet::Message;

use crate::convert::{decimal_as_written, hex, integer_value};
use crate::dictionary::{Got, Slot, State};
use crate::templates::{Field, Group, IntegerType, Kind, Operator, OperatorKind, Type};
use crate::wire::{write_ascii, write_bytes, write_presence_map, write_signed, write_unsigned};
use crate::{Code, Error, Templates};

/// MsgType(35), which a template sets with a constant rather than a field in the stream.
const MSG_TYPE: u32 = 35;

/// Encodes [`Message`]s as one FAST stream, keeping the previous values its operators refer to.
/// Use one encoder per stream, and reset it where the stream's decoders reset.
#[derive(Debug)]
pub struct Encoder {
    templates: Arc<Templates>,
    slots: Vec<Slot>,
    /// The previous template identifier, as the decoder keeps it.
    template_id: Option<u32>,
}

impl Encoder {
    /// An encoder with every previous value undefined.
    pub fn new(templates: Arc<Templates>) -> Self {
        let slots = (0..templates.slots).map(|_| Slot::default()).collect();
        Self { templates, slots, template_id: None }
    }

    /// Makes every previous value undefined again, as [`Decoder::reset`](crate::Decoder::reset)
    /// does.
    pub fn reset(&mut self) {
        for slot in &mut self.slots {
            slot.state = State::Undefined;
        }
        self.template_id = None;
    }

    /// Appends `msg` to `out`, encoded with template `template_id`. The message's fields must be
    /// in the template's order, as [`Decoder::decode_into`](crate::Decoder::decode_into) writes
    /// them: a sequence as its NumInGroup field, then each entry's fields; a group's fields in its
    /// place. MsgType(35) may be anywhere: it comes from the template.
    ///
    /// # Errors
    ///
    /// D9 for an unknown template; [`ErrorKind::InvalidValue`](crate::ErrorKind::InvalidValue)
    /// for a mandatory field that's missing, a field the template doesn't have where it's found,
    /// a value that differs from the template's constant, or one a tail can't make; D11 for a
    /// value that doesn't parse as its type, and R1, R3 or R4 for one out of its type's range;
    /// [`ErrorKind::Unsupported`](crate::ErrorKind::Unsupported) for a dynamic template
    /// reference. Nothing is appended then, but the previous values may have changed: reset.
    pub fn encode_into(&mut self, template_id: u32, msg: &Message, out: &mut Vec<u8>) -> Result<(), Error> {
        let template = self
            .templates
            .get(template_id)
            .ok_or_else(|| Error::spec(Code::D9).with_detail(format!("template id {template_id}")))?;
        let fields: Vec<(u32, &[u8])> = msg.fields_bytes().filter(|(tag, _)| *tag != MSG_TYPE).collect();
        let mut encoding = Encoding { input: &fields, next: 0, slots: &mut self.slots };
        let start = out.len();
        let result = encoding
            .segment(&template.body, Some((template_id, &mut self.template_id)), out)
            .and_then(|()| encoding.finished())
            .map_err(|e| e.with_template(&template.name));
        if result.is_err() {
            out.truncate(start);
        }
        result
    }
}

/// What encoding one message works on: its fields, read in order.
struct Encoding<'m, 'r> {
    input: &'r [(u32, &'m [u8])],
    next: usize,
    slots: &'r mut [Slot],
}

impl<'m> Encoding<'m, '_> {
    /// The next field's value, if it's `tag`.
    fn take(&mut self, tag: Option<u32>) -> Option<&'m [u8]> {
        let (next_tag, value) = *self.input.get(self.next)?;
        (Some(next_tag) == tag).then(|| {
            self.next += 1;
            value
        })
    }

    fn finished(&self) -> Result<(), Error> {
        match self.input.get(self.next) {
            None => Ok(()),
            Some((tag, _)) => Err(Error::invalid_value(&format!("tag {tag} isn't where the template has a field"))),
        }
    }

    /// A segment: its presence map, a template identifier if it has one, then its fields.
    fn segment(
        &mut self,
        group: &Group,
        template_id: Option<(u32, &mut Option<u32>)>,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let mut bits = Vec::new();
        let mut body = Vec::new();
        if let Some((id, previous)) = template_id {
            // As if by a copy operator (section 10.3).
            let same = *previous == Some(id);
            bits.push(!same);
            if !same {
                write_unsigned(&mut body, Some(u128::from(id)), false);
                *previous = Some(id);
            }
        }
        self.fields(&group.fields, &mut bits, &mut body)?;
        write_presence_map(out, &bits);
        out.extend_from_slice(&body);
        Ok(())
    }

    fn fields(&mut self, fields: &[Field], bits: &mut Vec<bool>, body: &mut Vec<u8>) -> Result<(), Error> {
        for field in fields {
            self.field(field, bits, body).map_err(|e| e.with_field(&field.name))?;
        }
        Ok(())
    }

    /// A group's or sequence element's fields: a segment of their own if they use presence bits.
    fn group(&mut self, group: &Group, bits: &mut Vec<bool>, body: &mut Vec<u8>) -> Result<(), Error> {
        if group.presence_bits == 0 {
            return self.fields(&group.fields, bits, body);
        }
        self.segment(group, None, body)
    }

    /// A group, given a bit if it's optional: there if the next field is one of its own.
    fn optional_group(
        &mut self,
        group: &Group,
        optional: bool,
        bits: &mut Vec<bool>,
        body: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let present = self.input.get(self.next).is_some_and(|(tag, _)| has_tag(&group.fields, *tag));
        if optional {
            bits.push(present);
            if !present {
                return Ok(());
            }
        }
        self.group(group, bits, body)
    }

    /// A sequence: its NumInGroup field as the length, then each entry.
    fn sequence(
        &mut self,
        length: &Operator,
        length_tag: u32,
        element: &Group,
        optional: bool,
        bits: &mut Vec<bool>,
        body: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let count = self.take(Some(length_tag)).map(|raw| integer(raw, IntegerType::UInt32)).transpose()?;
        self.operate(length, optional, Type::Integer(IntegerType::UInt32), count.map(Got::Integer), bits, body)?;
        for _ in 0..count.unwrap_or(0) {
            self.group(element, bits, body)?;
        }
        Ok(())
    }

    fn field(&mut self, field: &Field, bits: &mut Vec<bool>, body: &mut Vec<u8>) -> Result<(), Error> {
        let optional = field.optional;
        match &field.kind {
            Kind::Group(group) => self.optional_group(group, optional, bits, body),
            Kind::Sequence { length, length_tag, element } => {
                self.sequence(length, *length_tag, element, optional, bits, body)
            }
            Kind::TemplateRef => Err(Error::unsupported("encoding a dynamic template reference")),
            Kind::Integer(integer_type, operator) => {
                let value = self.take(field.tag).map(|raw| integer(raw, *integer_type)).transpose()?;
                self.operate(operator, optional, Type::Integer(*integer_type), value.map(Got::Integer), bits, body)
            }
            Kind::Decimal(operator) => {
                let value = self.take(field.tag).map(decimal).transpose()?;
                let value = value.map(|(exponent, mantissa)| Got::Decimal { exponent, mantissa });
                self.operate(operator, optional, Type::Decimal, value, bits, body)
            }
            Kind::DecimalParts { exponent, mantissa } => {
                let value = self.take(field.tag).map(decimal).transpose()?;
                let int32 = Type::Integer(IntegerType::Int32);
                let part = value.map(|(e, _)| Got::Integer(i128::from(e)));
                self.operate(exponent, optional, int32, part, bits, body)?;
                // The mantissa is there only if the exponent is.
                let Some((_, m)) = value else { return Ok(()) };
                self.operate(
                    mantissa,
                    false,
                    Type::Integer(IntegerType::Int64),
                    Some(Got::Integer(i128::from(m))),
                    bits,
                    body,
                )
            }
            Kind::Ascii(operator) => {
                let value = self.take(field.tag);
                if value.is_some_and(|chars| !chars.is_ascii()) {
                    return Err(Error::spec(Code::R3));
                }
                self.operate(operator, optional, Type::Ascii, value.map(Got::Bytes), bits, body)
            }
            Kind::Unicode(operator) => {
                let value = self.take(field.tag);
                self.operate(operator, optional, Type::Unicode, value.map(Got::Bytes), bits, body)
            }
            Kind::Bytes { operator, length_tag: Some(length_tag) } => {
                // A FIX data field: its Length field, then the data.
                let value = self.take(Some(*length_tag)).and_then(|_| self.take(field.tag));
                self.operate(operator, optional, Type::Bytes, value.map(Got::Bytes), bits, body)
            }
            Kind::Bytes { operator, length_tag: None } => {
                let hex_text = self.take(field.tag);
                let bytes = hex_text
                    .map(|raw| std::str::from_utf8(raw).ok().and_then(hex).ok_or_else(|| Error::spec(Code::D11)))
                    .transpose()?;
                self.operate(operator, optional, Type::Bytes, bytes.as_deref().map(Got::Bytes), bits, body)
            }
        }
    }

    /// Applies `operator` in reverse: sets the presence map bit, writes what the stream needs, and
    /// changes the previous value as the decoder will.
    fn operate(
        &mut self,
        operator: &Operator,
        optional: bool,
        ty: Type,
        value: Option<Got<'_>>,
        bits: &mut Vec<bool>,
        body: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let initial = operator.initial.as_ref().map(Got::from);
        match operator.kind {
            OperatorKind::None => write_value(body, ty, value, optional),
            OperatorKind::Constant => {
                if value.is_some() && value != initial {
                    return Err(Error::invalid_value("not the template's constant"));
                }
                if optional {
                    bits.push(value.is_some());
                }
                Ok(())
            }
            OperatorKind::Default => {
                let present = value != initial;
                bits.push(present);
                if present { write_value(body, ty, value, optional) } else { Ok(()) }
            }
            OperatorKind::Delta => {
                let slot = &mut self.slots[operator.slot.expect("a delta keeps a previous value")];
                delta(slot, ty, initial, value, optional, body)
            }
            OperatorKind::Copy | OperatorKind::Increment | OperatorKind::Tail => {
                let slot = &mut self.slots[operator.slot.expect("the operator keeps a previous value")];
                if slot.preview(operator.kind, ty, initial, optional) == Some(value) {
                    bits.push(false);
                    slot.unchanged(operator.kind, ty, initial, optional)?;
                    return Ok(());
                }
                bits.push(true);
                let Some(value) = value else {
                    write_value(body, ty, None, optional)?;
                    slot.state = State::Empty;
                    return Ok(());
                };
                match (operator.kind, value) {
                    (OperatorKind::Tail, Got::Bytes(bytes)) => {
                        slot.base(ty, initial, false)?;
                        let tail = tail(&slot.bytes, bytes)?;
                        write_value(body, ty, Some(Got::Bytes(tail)), optional)?;
                        slot.tail(tail);
                        debug_assert_eq!(slot.bytes, bytes, "the decoder's tail gives the value");
                    }
                    _ => {
                        write_value(body, ty, Some(value), optional)?;
                        slot.assign(ty, value);
                    }
                }
                Ok(())
            }
        }
    }
}

/// The delta operator in reverse (section 6.3.7): the difference from the base value.
fn delta(
    slot: &mut Slot,
    ty: Type,
    initial: Option<Got<'_>>,
    value: Option<Got<'_>>,
    optional: bool,
    body: &mut Vec<u8>,
) -> Result<(), Error> {
    let Some(value) = value else {
        if !optional {
            return Err(missing());
        }
        // NULL: absent, the previous value left as it is.
        write_signed(body, None, true);
        return Ok(());
    };
    slot.base(ty, initial, true)?;
    match value {
        Got::Integer(n) => {
            write_signed(body, Some(n - slot.number), optional);
            slot.number = n;
        }
        Got::Decimal { exponent, mantissa } => {
            write_signed(body, Some(i128::from(exponent) - i128::from(slot.exponent)), optional);
            write_signed(body, Some(i128::from(mantissa) - slot.number), false);
            slot.exponent = exponent;
            slot.number = i128::from(mantissa);
        }
        Got::Bytes(bytes) => {
            let (subtraction, part) = string_delta(&slot.bytes, bytes)?;
            write_signed(body, Some(i128::from(subtraction)), optional);
            if ty == Type::Ascii {
                write_ascii(body, Some(part), false);
            } else {
                write_bytes(body, Some(part), false);
            }
            slot.splice(subtraction, part)?;
            debug_assert_eq!(slot.bytes, bytes, "the decoder's delta gives the value");
        }
    }
    Ok(())
}

/// The shorter of the two ways to turn `base` into `value` (section 6.3.7.3): keep the common
/// prefix and replace the back, or keep the common suffix and replace the front. A front removal
/// of `n` is written `-n - 1`, so that removing none from the front is `-1`.
fn string_delta<'v>(base: &[u8], value: &'v [u8]) -> Result<(i32, &'v [u8]), Error> {
    let prefix = base.iter().zip(value).take_while(|(a, b)| a == b).count();
    let suffix = base.iter().rev().zip(value.iter().rev()).take_while(|(a, b)| a == b).count();
    let length = |n: usize| i32::try_from(n).map_err(|_| Error::spec(Code::D7));
    if value.len() - prefix <= value.len() - suffix {
        Ok((length(base.len() - prefix)?, &value[prefix..]))
    } else {
        Ok((-length(base.len() - suffix)? - 1, &value[..value.len() - suffix]))
    }
}

/// The tail that turns `base` into `value` (section 6.3.8): the bytes after their common prefix
/// if they're as long, or all of `value` if it's longer. A shorter value can't be made.
fn tail<'v>(base: &[u8], value: &'v [u8]) -> Result<&'v [u8], Error> {
    if value.len() > base.len() {
        return Ok(value);
    }
    if value.len() < base.len() {
        return Err(Error::invalid_value("a tail can't make a value shorter than the previous one"));
    }
    let prefix = base.iter().zip(value).take_while(|(a, b)| a == b).count();
    Ok(&value[prefix..])
}

/// Writes a value, or NULL for an absent one if the field is nullable.
fn write_value(body: &mut Vec<u8>, ty: Type, value: Option<Got<'_>>, nullable: bool) -> Result<(), Error> {
    if value.is_none() && !nullable {
        return Err(missing());
    }
    match (ty, value) {
        (Type::Integer(integer), value) if integer.is_signed() => {
            write_signed(body, value.map(|v| integer_of(v)), nullable);
        }
        (Type::Integer(_), value) => {
            let value = value.map(|v| u128::try_from(integer_of(v)).expect("checked against the range"));
            write_unsigned(body, value, nullable);
        }
        (Type::Decimal, None) => write_signed(body, None, true),
        (Type::Decimal, Some(Got::Decimal { exponent, mantissa })) => {
            write_signed(body, Some(i128::from(exponent)), nullable);
            write_signed(body, Some(i128::from(mantissa)), false);
        }
        (Type::Ascii, value) => write_ascii(body, value.map(bytes_of), nullable),
        (Type::Unicode | Type::Bytes, value) => write_bytes(body, value.map(bytes_of), nullable),
        (Type::Decimal, Some(_)) => unreachable!("a decimal field's value is a decimal"),
    }
    Ok(())
}

fn integer_of(value: Got<'_>) -> i128 {
    let Got::Integer(n) = value else { unreachable!("an integer field's value is an integer") };
    n
}

fn bytes_of(value: Got<'_>) -> &[u8] {
    let Got::Bytes(bytes) = value else { unreachable!("a string or byte vector field's value is bytes") };
    bytes
}

fn missing() -> Error {
    Error::invalid_value("a mandatory field is missing")
}

/// A field's text as an integer of its type [ERR D11 if it isn't one, R4 if out of range].
fn integer(raw: &[u8], integer_type: IntegerType) -> Result<i128, Error> {
    let text = std::str::from_utf8(raw).map_err(|_| Error::spec(Code::D11))?;
    if let Some(value) = integer_value(text, integer_type) {
        return Ok(value);
    }
    let is_number = text.strip_prefix('-').unwrap_or(text).bytes().all(|b| b.is_ascii_digit()) && !text.is_empty();
    Err(Error::spec(if is_number { Code::R4 } else { Code::D11 }))
}

/// A field's text as a decimal, at the scale it's written with [ERR D11, or R1 if it can't be
/// a FAST decimal].
fn decimal(raw: &[u8]) -> Result<(i32, i64), Error> {
    let text = std::str::from_utf8(raw).map_err(|_| Error::spec(Code::D11))?;
    decimal_as_written(text).ok_or_else(|| {
        let well_formed = text.strip_prefix('-').unwrap_or(text).bytes().all(|b| b.is_ascii_digit() || b == b'.');
        Error::spec(if well_formed { Code::R1 } else { Code::D11 })
    })
}

/// Whether `tag` is one of `fields`' tags, nested ones included: how the encoder tells whether
/// an optional group is in the message.
fn has_tag(fields: &[Field], tag: u32) -> bool {
    fields.iter().any(|field| {
        field.tag == Some(tag)
            || match &field.kind {
                Kind::Group(group) => has_tag(&group.fields, tag),
                Kind::Sequence { length_tag, element, .. } => *length_tag == tag || has_tag(&element.fields, tag),
                Kind::Bytes { length_tag, .. } => *length_tag == Some(tag),
                _ => false,
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Decoder, ErrorKind};

    fn templates(fields: &str) -> Arc<Templates> {
        let xml = format!(r#"<templates><template name="T" id="1">{fields}</template></templates>"#);
        Arc::new(Templates::from_xml(&xml).expect("valid templates"))
    }

    /// Encodes each message, then decodes the stream and checks it gives the messages back.
    fn round_trip(fields: &str, messages: &[Message]) -> Vec<u8> {
        let templates = templates(fields);
        let mut encoder = Encoder::new(Arc::clone(&templates));
        let mut out = Vec::new();
        for msg in messages {
            encoder.encode_into(1, msg, &mut out).expect("encodes");
        }
        let mut decoder = Decoder::new(templates);
        let mut rest = out.as_slice();
        let mut decoded = Message::default();
        for msg in messages {
            rest = decoder.decode_into(rest, &mut decoded).expect("decodes").rest;
            let without_msg_type = |m: &Message| {
                m.fields().filter(|(t, _)| *t != MSG_TYPE).map(|(t, v)| (t, v.to_string())).collect::<Vec<_>>()
            };
            assert_eq!(without_msg_type(&decoded), without_msg_type(msg));
        }
        assert!(rest.is_empty());
        out
    }

    fn msg(fields: &[(u32, &str)]) -> Message {
        Message::from_fields(fields.iter().copied())
    }

    #[test]
    fn string_deltas_as_in_appendix_3_2_5() {
        assert_eq!(string_delta(b"", b"GEH6"), Ok((0, &b"GEH6"[..])));
        assert_eq!(string_delta(b"GEH6", b"GEM6"), Ok((2, &b"M6"[..])));
        assert_eq!(string_delta(b"GEM6", b"ESM6"), Ok((-3, &b"ES"[..])));
        assert_eq!(string_delta(b"ESM6", b"RSESM6"), Ok((-1, &b"RS"[..])));
        let out = round_trip(
            r#"<string id="1" name="Security"><delta/></string>"#,
            &[msg(&[(1, "GEH6")]), msg(&[(1, "GEM6")]), msg(&[(1, "ESM6")]), msg(&[(1, "RSESM6")])],
        );
        // The specification's bytes, after each message's presence map (and the first's template).
        let expected = [
            &[0xc0, 0x81, 0x80, 0x47, 0x45, 0x48, 0xb6][..],
            &[0x80, 0x82, 0x4d, 0xb6],
            &[0x80, 0xfd, 0x45, 0xd3],
            &[0x80, 0xff, 0x52, 0xd3],
        ];
        assert_eq!(out, expected.concat());
    }

    #[test]
    fn operators_leave_out_what_the_decoder_can_work_out() {
        let fields = r#"<uInt32 id="1" name="A"><copy/></uInt32>
                        <uInt32 id="2" name="B"><increment value="1"/></uInt32>
                        <uInt32 id="3" name="C"><default value="0"/></uInt32>
                        <string id="4" name="D"><constant value="X"/></string>"#;
        let out = round_trip(
            fields,
            &[msg(&[(1, "5"), (2, "1"), (3, "0"), (4, "X")]), msg(&[(1, "5"), (2, "2"), (3, "7"), (4, "X")])],
        );
        // First: template and A sent, B the initial value, C the default. Second: only C.
        assert_eq!(out, [0xe0, 0x81, 0x85, 0x88, 0x87]);
    }

    #[test]
    fn tails_and_optional_values_round_trip() {
        let fields = r#"<string id="1" name="A" presence="optional"><tail value="ABCD"/></string>
                        <decimal id="2" name="B" presence="optional"><exponent><copy value="-2"/></exponent><mantissa><delta/></mantissa></decimal>
                        <group name="G" presence="optional"><uInt32 id="3" name="C"/></group>
                        <sequence name="S"><length name="N" id="4"/><string id="5" name="D"><copy/></string></sequence>"#;
        round_trip(
            fields,
            &[
                msg(&[(1, "ABXY"), (2, "9427.55"), (3, "1"), (4, "2"), (5, "x"), (5, "x")]),
                msg(&[(2, "-0.5"), (4, "0")]),
                msg(&[(1, "123456"), (2, "1"), (3, "2"), (4, "1"), (5, "y")]),
                msg(&[(1, "123457"), (4, "0")]),
            ],
        );
    }

    #[test]
    fn data_fields_and_hex_byte_vectors_round_trip() {
        let fields = r#"<byteVector id="96" name="RawData"><length name="RawDataLength" id="95"/></byteVector>
                        <byteVector id="3" name="Hex"><copy/></byteVector>"#;
        let mut first = Message::default();
        first.push_data(95, 96, &[0x00, 0x01, 0xff]);
        first.push(3, "ab01");
        let mut second = Message::default();
        second.push_data(95, 96, b"");
        second.push(3, "ab01");
        round_trip(fields, &[first, second]);
    }

    #[test]
    fn messages_that_dont_fit_the_template_are_refused() {
        let templates = templates(
            r#"<uInt32 id="1" name="A"/><string id="2" name="B"><constant value="X"/></string>
               <string id="3" name="C" presence="optional"><tail/></string>"#,
        );
        let mut encoder = Encoder::new(templates);
        let mut out = Vec::new();
        let error = |encoder: &mut Encoder, fields: &[(u32, &str)]| {
            encoder.encode_into(1, &msg(fields), &mut Vec::new()).unwrap_err()
        };
        assert_eq!(error(&mut encoder, &[]).kind(), ErrorKind::InvalidValue);
        assert_eq!(error(&mut encoder, &[(1, "1"), (2, "Y")]).kind(), ErrorKind::InvalidValue);
        assert_eq!(error(&mut encoder, &[(2, "X"), (1, "1")]).kind(), ErrorKind::InvalidValue);
        assert_eq!(error(&mut encoder, &[(1, "x")]).code(), Some(Code::D11));
        assert_eq!(error(&mut encoder, &[(1, "4294967296")]).code(), Some(Code::R4));
        assert_eq!(encoder.encode_into(9, &msg(&[]), &mut out).unwrap_err().code(), Some(Code::D9));
        // A tail can lengthen a value but not shorten it.
        encoder.encode_into(1, &msg(&[(1, "1"), (3, "ABC")]), &mut out).expect("encodes");
        assert_eq!(error(&mut encoder, &[(1, "1"), (3, "AB")]).kind(), ErrorKind::InvalidValue);
        assert!(!out.is_empty());
    }
}
