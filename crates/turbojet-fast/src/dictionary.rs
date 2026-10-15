//! The dictionaries' previous values (section 6.3.1), kept alike by the decoder and the encoder,
//! and the values the operators work on.

use crate::templates::{OperatorKind, Type, Value};
use crate::{Code, Error};

/// One previous value (section 6.3.1).
#[derive(Debug, Default)]
pub(crate) struct Slot {
    pub(crate) state: State,
    /// The type of the value last assigned.
    pub(crate) ty: Option<Type>,
    /// An integer, or a decimal's mantissa.
    pub(crate) number: i128,
    pub(crate) exponent: i32,
    /// A string or byte vector, its buffer kept from one value to the next.
    pub(crate) bytes: Vec<u8>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    #[default]
    Undefined,
    Empty,
    Assigned,
}

/// A field's value, borrowed from the input, the scratch buffer, a slot or the template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Got<'b> {
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
    pub(crate) fn assign(&mut self, ty: Type, value: Got<'_>) {
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
    pub(crate) fn unchanged<'s>(
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

    /// What [`unchanged`](Self::unchanged) would give, changing nothing: `None` if it would be
    /// an error. The encoder leaves a field out of the stream when this is its value.
    pub(crate) fn preview<'s>(
        &'s self,
        kind: OperatorKind,
        ty: Type,
        initial: Option<Got<'s>>,
        optional: bool,
    ) -> Option<Option<Got<'s>>> {
        match (self.state, initial) {
            (State::Assigned, _) if self.ty != Some(ty) => None,
            (State::Assigned, _) if kind == OperatorKind::Increment => {
                let Type::Integer(integer) = ty else { unreachable!("increment is only for integers") };
                let (min, max) = integer.range();
                Some(Some(Got::Integer(if self.number == max { min } else { self.number + 1 })))
            }
            (State::Assigned, _) => Some(Some(self.value())),
            (State::Undefined, Some(initial)) => Some(Some(initial)),
            (State::Undefined | State::Empty, _) if optional => Some(None),
            (State::Undefined | State::Empty, _) => None,
        }
    }

    /// Makes the slot hold the base value a delta or tail combines with: the previous value if
    /// assigned, else the initial value, else the type's default (zero, or empty). An empty
    /// previous value is a delta's error [ERR D6] but a tail's base like an undefined one.
    pub(crate) fn base(&mut self, ty: Type, initial: Option<Got<'_>>, empty_is_error: bool) -> Result<(), Error> {
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
    pub(crate) fn tail(&mut self, tail: &[u8]) {
        let keep = self.bytes.len().saturating_sub(tail.len());
        self.bytes.truncate(keep);
        self.bytes.extend_from_slice(tail);
    }

    /// A string or byte vector delta (sections 6.3.7.3 to 6.3.7.5): removes `subtraction` bytes
    /// from the back, or from the front if it's negative (less one: -1 removes none), and adds
    /// `part` at the same end.
    pub(crate) fn splice(&mut self, subtraction: i32, part: &[u8]) -> Result<(), Error> {
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
    pub(crate) fn check_utf8(&self, ty: Type) -> Result<(), Error> {
        if ty == Type::Unicode && std::str::from_utf8(&self.bytes).is_err() {
            return Err(Error::spec(Code::R2));
        }
        Ok(())
    }

    pub(crate) fn value(&self) -> Got<'_> {
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
