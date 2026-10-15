//! Decoding a stream's messages: presence maps, template identifiers, field operators and the
//! dictionaries of previous values (sections 6.3 and 10).

use std::sync::Arc;

use turbojet::Message;
use turbojet::fields::ToFix;

use crate::convert::EXPONENT_RANGE;
use crate::templates::{Field, IntegerType, Kind, Operator, OperatorKind, Template, Type, Value};
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
        let mut reader = Reader::new(input);
        let mut presence = reader.presence_map()?;
        let template_id = if presence.next_bit() {
            let id = reader.unsigned(false)?.expect("a mandatory integer isn't NULL");
            let id = u32::try_from(id).map_err(|_| Error::spec(Code::D2))?;
            self.template_id = Some(id);
            id
        } else {
            self.template_id.ok_or_else(|| Error::spec(Code::D5).with_detail("no template identifier yet"))?
        };
        let template = self
            .templates
            .get(template_id)
            .ok_or_else(|| Error::spec(Code::D9).with_detail(format!("template id {template_id}")))?;
        check_presence_map_length(&presence, template).map_err(|e| e.with_template(&template.name))?;
        msg.reset(template.msg_type.clone());
        let mut decoding = Decoding {
            reader: &mut reader,
            presence: &mut presence,
            slots: &mut self.slots,
            scratch: &mut self.scratch,
        };
        for field in &template.fields {
            decoding.field(field, msg).map_err(|e| e.with_template(&template.name).with_field(&field.name))?;
        }
        Ok(Decoded { template_id, rest: reader.rest() })
    }
}

/// [ERR R8]: more presence map bytes than the template's fields could use.
fn check_presence_map_length(presence: &PresenceMap<'_>, template: &Template) -> Result<(), Error> {
    let bits_needed = 1 + template.presence_bits;
    let bytes_needed = bits_needed.div_ceil(7).max(1);
    if presence.len_bits() / 7 > bytes_needed {
        return Err(Error::spec(Code::R8));
    }
    Ok(())
}

/// One previous value (section 6.3.1).
#[derive(Debug, Default)]
struct Slot {
    state: State,
    /// The type of the value last assigned.
    ty: Option<Type>,
    /// An integer, or a decimal's mantissa.
    number: i128,
    exponent: i32,
    /// A string or byte vector, its buffer kept from one value to the next.
    bytes: Vec<u8>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Undefined,
    Empty,
    Assigned,
}

/// A field's value, borrowed from the input, the scratch buffer, a slot or the template.
#[derive(Debug, Clone, Copy)]
enum Got<'b> {
    Integer(i128),
    Decimal { exponent: i32, mantissa: i64 },
    Bytes(&'b [u8]),
}

impl<'b> From<&'b Value> for Got<'b> {
    fn from(value: &'b Value) -> Self {
        match value {
            Value::Integer(n) => Self::Integer(*n),
            Value::Decimal { exponent, mantissa } => Self::Decimal { exponent: *exponent, mantissa: *mantissa },
            Value::Bytes(bytes) => Self::Bytes(bytes),
        }
    }
}

impl Slot {
    fn assign(&mut self, ty: Type, value: Got<'_>) {
        self.state = State::Assigned;
        self.ty = Some(ty);
        match value {
            Got::Integer(n) => self.number = n,
            Got::Decimal { exponent, mantissa } => {
                self.exponent = exponent;
                self.number = i128::from(mantissa);
            }
            Got::Bytes(bytes) => {
                self.bytes.clear();
                self.bytes.extend_from_slice(bytes);
            }
        }
    }

    /// The field's value when its presence map bit is clear, for copy, increment and tail
    /// (sections 6.3.5, 6.3.6 and 6.3.8).
    fn unchanged<'s>(
        &'s mut self,
        kind: OperatorKind,
        ty: Type,
        initial: Option<Got<'_>>,
        optional: bool,
    ) -> Result<Option<Got<'s>>, Error> {
        match (self.state, initial) {
            (State::Assigned, _) if self.ty != Some(ty) => return Err(Error::spec(Code::D4)),
            (State::Assigned, _) if kind == OperatorKind::Increment => {
                let Type::Integer(integer) = ty else { unreachable!("increment is only for integers") };
                let (min, max) = integer.range();
                self.number = if self.number == max { min } else { self.number + 1 };
            }
            (State::Assigned, _) => {}
            (State::Undefined, Some(initial)) => self.assign(ty, initial),
            (State::Undefined, None) if optional => {
                self.state = State::Empty;
                return Ok(None);
            }
            // The tail operator's section gives D6 and D7 for these two; Appendix 4, and copy
            // and increment, give D5 and D6.
            (State::Undefined, None) => return Err(Error::spec(Code::D5)),
            (State::Empty, _) if optional => return Ok(None),
            (State::Empty, _) => return Err(Error::spec(Code::D6)),
        }
        Ok(Some(self.value()))
    }

    /// Makes the slot hold the base value a delta or tail combines with: the previous value if
    /// assigned, else the initial value, else the type's default (zero, or empty). An empty
    /// previous value is a delta's error [ERR D6] but a tail's base like an undefined one.
    fn base(&mut self, ty: Type, initial: Option<Got<'_>>, empty_is_error: bool) -> Result<(), Error> {
        match (self.state, initial) {
            (State::Assigned, _) if self.ty != Some(ty) => Err(Error::spec(Code::D4)),
            (State::Assigned, _) => Ok(()),
            (State::Empty, _) if empty_is_error => Err(Error::spec(Code::D6)),
            (_, Some(initial)) => {
                self.assign(ty, initial);
                Ok(())
            }
            (_, None) => {
                self.state = State::Assigned;
                self.ty = Some(ty);
                self.number = 0;
                self.exponent = 0;
                self.bytes.clear();
                Ok(())
            }
        }
    }

    /// A tail (section 6.3.8): replaces as many bytes at the end as it has, or all of them.
    fn tail(&mut self, tail: &[u8]) {
        let keep = self.bytes.len().saturating_sub(tail.len());
        self.bytes.truncate(keep);
        self.bytes.extend_from_slice(tail);
    }

    /// A string or byte vector delta (sections 6.3.7.3 to 6.3.7.5): removes `subtraction` bytes
    /// from the back, or from the front if it's negative (less one: -1 removes none), and adds
    /// `part` at the same end.
    fn splice(&mut self, subtraction: i32, part: &[u8]) -> Result<(), Error> {
        let front = subtraction < 0;
        let remove = if front { (subtraction + 1).unsigned_abs() } else { subtraction.unsigned_abs() };
        let remove = usize::try_from(remove).map_err(|_| Error::spec(Code::D7))?;
        if remove > self.bytes.len() {
            return Err(Error::spec(Code::D7));
        }
        if front {
            self.bytes.splice(..remove, part.iter().copied());
        } else {
            self.bytes.truncate(self.bytes.len() - remove);
            self.bytes.extend_from_slice(part);
        }
        Ok(())
    }

    /// [ERR R2]: a Unicode string a delta or tail made isn't UTF-8.
    fn check_utf8(&self, ty: Type) -> Result<(), Error> {
        if ty == Type::Unicode && std::str::from_utf8(&self.bytes).is_err() {
            return Err(Error::spec(Code::R2));
        }
        Ok(())
    }

    fn value(&self) -> Got<'_> {
        match self.ty {
            Some(Type::Integer(_)) | None => Got::Integer(self.number),
            Some(Type::Decimal) => Got::Decimal {
                exponent: self.exponent,
                mantissa: i64::try_from(self.number).expect("a decimal's mantissa is assigned from an i64"),
            },
            Some(Type::Ascii | Type::Unicode | Type::Bytes) => Got::Bytes(&self.bytes),
        }
    }
}

/// What decoding one message's fields works on.
struct Decoding<'r, 'a> {
    reader: &'r mut Reader<'a>,
    presence: &'r mut PresenceMap<'a>,
    slots: &'r mut [Slot],
    scratch: &'r mut Vec<u8>,
}

impl<'a> Decoding<'_, 'a> {
    /// Decodes one field and writes it into `msg`, unless it's absent or has no tag.
    fn field(&mut self, field: &Field, msg: &mut Message) -> Result<(), Error> {
        let optional = field.optional;
        let value = match &field.kind {
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
            OperatorKind::None => read(self.reader, self.scratch),
            OperatorKind::Constant if optional && !self.presence.next_bit() => Ok(None),
            OperatorKind::Constant => Ok(initial),
            OperatorKind::Default if self.presence.next_bit() => read(self.reader, self.scratch),
            OperatorKind::Default => Ok(initial),
            OperatorKind::Delta => self.delta(operator, optional, ty),
            OperatorKind::Copy | OperatorKind::Increment | OperatorKind::Tail => {
                let slot = &mut self.slots[operator.slot.expect("the operator keeps a previous value")];
                if !self.presence.next_bit() {
                    return slot.unchanged(operator.kind, ty, initial, optional);
                }
                let Some(value) = read(self.reader, self.scratch)? else {
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
        let cases = [(0, 0, "0"), (3, 12, "12000"), (-2, 5, "0.05"), (-2, -12345, "-123.45"), (-3, 0, "0.000")];
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
