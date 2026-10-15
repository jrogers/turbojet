//! FAST (FIX Adapted for STreaming) 1.1 decoding into Turbojet [`Message`](turbojet::Message)s.
//!
//! FAST is the compressed encoding some venues use for market data. A feed publishes its
//! templates as XML; load them once with [`Templates::from_xml`], then decode the feed's messages
//! with a [`Decoder`], which keeps the stream's previous values (its dictionaries) from one
//! message to the next. Each field's `id` in the templates is its FIX tag, so a decoded message
//! reads like any other: with [`Message::get`](turbojet::Message::get), or as a typed message.
//! A sequence becomes a repeating group, its `<length>`'s `id` the NumInGroup tag; a group's
//! fields, and those of a referenced template, are written in its place.
//!
//! ```
//! use std::sync::Arc;
//! use turbojet::Message;
//! use turbojet_fast::{Decoder, Templates};
//!
//! let templates = Templates::from_xml(r#"
//!     <templates xmlns="http://www.fixprotocol.org/ns/fast/td/1.1">
//!       <template name="Trade" id="1">
//!         <string name="MsgType" id="35"><constant value="X"/></string>
//!         <string name="Symbol" id="55"><copy/></string>
//!         <decimal name="MDEntryPx" id="270"/>
//!       </template>
//!     </templates>"#)?;
//! let mut decoder = Decoder::new(Arc::new(templates));
//! let mut msg = Message::default();
//!
//! // Template 1, Symbol "ABC" and a price of 9427.55; then the same template and Symbol again.
//! let packet = [0xe0, 0x81, 0x41, 0x42, 0xc3, 0xfe, 0x39, 0x45, 0xa3, 0x80, 0xfe, 0x39, 0x45, 0xa4];
//! let decoded = decoder.decode_into(&packet, &mut msg)?;
//! assert_eq!(msg.get(55), Some("ABC"));
//! assert_eq!(msg.get(270), Some("9427.55"));
//!
//! decoder.decode_into(decoded.rest, &mut msg)?;
//! assert_eq!(msg.get(55), Some("ABC"));
//! assert_eq!(msg.get(270), Some("9427.56"));
//! # Ok::<(), turbojet_fast::Error>(())
//! ```
//!
//! When a feed resets its dictionaries (each packet, or on a reset message), call
//! [`Decoder::reset`]: the specification leaves how a reset is signalled to the feed.
//!
//! Not supported yet: encoding.

#![warn(missing_docs)]

mod convert;
mod decode;
mod templates;
mod wire;

use std::fmt;

pub use decode::{Decoded, Decoder};
pub use templates::Templates;

/// An error from the FAST specification (Appendix 4): static errors in templates (S), dynamic
/// errors in the stream (D), and reportable errors (R), which this decoder always reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs, reason = "each is described by its code in the specification")]
pub enum Code {
    S1,
    S2,
    S3,
    S4,
    S5,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
}

/// A template that couldn't be loaded, or a message that couldn't be decoded. After a decoding
/// error the decoder's previous values are no longer known to match the encoder's: call
/// [`Decoder::reset`] where the feed next resets, or start again from a snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    template: Option<Box<str>>,
    field: Option<Box<str>>,
    detail: Option<String>,
}

/// What kind of error an [`Error`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// One the specification defines.
    Spec(Code),
    /// The input ends inside a message.
    Truncated,
    /// A string that isn't valid UTF-8, or that holds SOH, which a FIX value can't.
    InvalidValue,
    /// Something this version doesn't handle yet.
    Unsupported,
    /// More nesting or sequence entries than a decoder allows, so a message can't make it do
    /// unbounded work.
    Limit,
}

impl Error {
    /// The specification's error code, if it defines this error.
    pub fn code(&self) -> Option<Code> {
        match self.kind {
            ErrorKind::Spec(code) => Some(code),
            _ => None,
        }
    }

    /// What kind of error it is.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The template the error is in, by name, if known.
    pub fn template(&self) -> Option<&str> {
        self.template.as_deref()
    }

    /// The field the error is in, by name, if known.
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    pub(crate) fn spec(code: Code) -> Self {
        Self { kind: ErrorKind::Spec(code), template: None, field: None, detail: None }
    }

    pub(crate) fn truncated() -> Self {
        Self { kind: ErrorKind::Truncated, template: None, field: None, detail: None }
    }

    pub(crate) fn invalid_value(detail: &str) -> Self {
        Self { kind: ErrorKind::InvalidValue, template: None, field: None, detail: Some(detail.to_string()) }
    }

    pub(crate) fn limit(detail: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Limit, template: None, field: None, detail: Some(detail.into()) }
    }

    pub(crate) fn unsupported(detail: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Unsupported, template: None, field: None, detail: Some(detail.into()) }
    }

    #[must_use]
    pub(crate) fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Names the template, unless an inner one already is.
    #[must_use]
    pub(crate) fn with_template(mut self, name: &str) -> Self {
        self.template.get_or_insert_with(|| name.into());
        self
    }

    /// Names the field, unless an inner one already is.
    #[must_use]
    pub(crate) fn with_field(mut self, name: &str) -> Self {
        self.field.get_or_insert_with(|| name.into());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Spec(code) => write!(f, "ERR {code:?}")?,
            ErrorKind::Truncated => f.write_str("the message is truncated")?,
            ErrorKind::InvalidValue => f.write_str("invalid value")?,
            ErrorKind::Unsupported => f.write_str("not supported")?,
            ErrorKind::Limit => f.write_str("over a limit")?,
        }
        if let Some(template) = &self.template {
            write!(f, " in template {template}")?;
        }
        if let Some(field) = &self.field {
            write!(f, ", field {field}")?;
        }
        if let Some(detail) = &self.detail {
            write!(f, ": {detail}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
