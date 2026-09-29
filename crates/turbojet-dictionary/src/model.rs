use std::collections::HashMap;
use std::fmt;

/// FIX (application and, before 5.0, session) or FIXT (the 5.0+ session layer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Protocol {
    /// `FIX`.
    Fix,
    /// `FIXT`.
    Fixt,
}

/// A dictionary's protocol version: in QuickFIX, from the root element's `type`, `major`, `minor`
/// and `servicepack` attributes; in Orchestra, from the repository's `name` and `version`
/// (`FIX.4.2_EP310`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Version {
    /// FIX or FIXT.
    pub protocol: Protocol,
    /// A numbered version, or FIX Latest.
    pub release: Release,
    /// The extension pack, e.g. 310 in `FIX.4.2_EP310`; `None` if not given (QuickFIX
    /// dictionaries don't say).
    pub extension_pack: Option<u32>,
}

/// Which release of a protocol a dictionary describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Release {
    /// A numbered version, e.g. 4.2, or 5.0 SP2 (`service_pack` 2; 0 if none).
    Numbered {
        /// The major version, e.g. 4 in FIX 4.2.
        major: u32,
        /// The minor version, e.g. 2 in FIX 4.2.
        minor: u32,
        /// The service pack, 0 if none.
        service_pack: u32,
    },
    /// FIX Latest: the continuously updated successor to FIX 5.0 SP2, identified by its
    /// extension pack rather than a number.
    Latest,
}

/// `FIX 4.4`, `FIX 5.0 SP2`, `FIXT 1.1`, `FIX 4.2 EP310`, `FIX Latest EP312`.
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let protocol = match self.protocol {
            Protocol::Fix => "FIX",
            Protocol::Fixt => "FIXT",
        };
        match self.release {
            Release::Latest => write!(f, "{protocol} Latest")?,
            Release::Numbered { major, minor, service_pack } => {
                write!(f, "{protocol} {major}.{minor}")?;
                if service_pack > 0 {
                    write!(f, " SP{service_pack}")?;
                }
            }
        }
        if let Some(ep) = self.extension_pack {
            write!(f, " EP{ep}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn versions_display() {
        let numbered = |protocol, major, minor, service_pack, extension_pack| Version {
            protocol,
            release: Release::Numbered { major, minor, service_pack },
            extension_pack,
        };
        assert_eq!(numbered(Protocol::Fix, 4, 4, 0, None).to_string(), "FIX 4.4");
        assert_eq!(numbered(Protocol::Fix, 5, 0, 2, None).to_string(), "FIX 5.0 SP2");
        assert_eq!(numbered(Protocol::Fixt, 1, 1, 0, None).to_string(), "FIXT 1.1");
        assert_eq!(numbered(Protocol::Fix, 4, 2, 0, Some(310)).to_string(), "FIX 4.2 EP310");
        let latest = Version { protocol: Protocol::Fix, release: Release::Latest, extension_pack: Some(312) };
        assert_eq!(latest.to_string(), "FIX Latest EP312");
    }
}

/// A field's data type. Types this crate doesn't know are kept as `Other`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FieldType {
    /// `INT`.
    Int,
    /// `LENGTH`.
    Length,
    /// `TAGNUM`.
    TagNum,
    /// `SEQNUM`.
    SeqNum,
    /// `NUMINGROUP`.
    NumInGroup,
    /// `DAYOFMONTH`.
    DayOfMonth,
    /// `FLOAT`.
    Float,
    /// `QTY`.
    Qty,
    /// `PRICE`.
    Price,
    /// `PRICEOFFSET`.
    PriceOffset,
    /// `AMT`.
    Amt,
    /// `PERCENTAGE`.
    Percentage,
    /// `CHAR`.
    Char,
    /// `BOOLEAN`.
    Boolean,
    /// `STRING`.
    String,
    /// `MULTIPLECHARVALUE`.
    MultipleCharValue,
    /// `MULTIPLESTRINGVALUE`.
    MultipleStringValue,
    /// `MULTIPLEVALUESTRING`.
    MultipleValueString,
    /// `COUNTRY`.
    Country,
    /// `CURRENCY`.
    Currency,
    /// `EXCHANGE`.
    Exchange,
    /// `LANGUAGE`.
    Language,
    /// `MONTHYEAR`.
    MonthYear,
    /// `UTCTIMESTAMP`.
    UtcTimestamp,
    /// `UTCTIMEONLY`.
    UtcTimeOnly,
    /// `UTCDATEONLY`.
    UtcDateOnly,
    /// `UTCDATE`.
    UtcDate,
    /// `LOCALMKTDATE`.
    LocalMktDate,
    /// `LOCALMKTTIME`.
    LocalMktTime,
    /// `TZTIMEONLY`.
    TzTimeOnly,
    /// `TZTIMESTAMP`.
    TzTimestamp,
    /// `DATA`.
    Data,
    /// `XMLDATA`.
    XmlData,
    /// `XID`.
    Xid,
    /// `XIDREF`.
    XidRef,
    /// Any other type, by its name in the dictionary.
    Other(String),
}

impl FieldType {
    /// From the dictionary's `type` attribute, e.g. `UTCTIMESTAMP`.
    pub fn parse(name: &str) -> Self {
        match name {
            "INT" => Self::Int,
            "LENGTH" => Self::Length,
            "TAGNUM" => Self::TagNum,
            "SEQNUM" => Self::SeqNum,
            "NUMINGROUP" => Self::NumInGroup,
            "DAYOFMONTH" => Self::DayOfMonth,
            "FLOAT" => Self::Float,
            "QTY" => Self::Qty,
            "PRICE" => Self::Price,
            "PRICEOFFSET" => Self::PriceOffset,
            "AMT" => Self::Amt,
            "PERCENTAGE" => Self::Percentage,
            "CHAR" => Self::Char,
            "BOOLEAN" => Self::Boolean,
            "STRING" => Self::String,
            "MULTIPLECHARVALUE" => Self::MultipleCharValue,
            "MULTIPLESTRINGVALUE" => Self::MultipleStringValue,
            "MULTIPLEVALUESTRING" => Self::MultipleValueString,
            "COUNTRY" => Self::Country,
            "CURRENCY" => Self::Currency,
            "EXCHANGE" => Self::Exchange,
            "LANGUAGE" => Self::Language,
            "MONTHYEAR" => Self::MonthYear,
            "UTCTIMESTAMP" => Self::UtcTimestamp,
            "UTCTIMEONLY" => Self::UtcTimeOnly,
            "UTCDATEONLY" => Self::UtcDateOnly,
            "UTCDATE" => Self::UtcDate,
            "LOCALMKTDATE" => Self::LocalMktDate,
            "LOCALMKTTIME" => Self::LocalMktTime,
            "TZTIMEONLY" => Self::TzTimeOnly,
            "TZTIMESTAMP" => Self::TzTimestamp,
            "DATA" => Self::Data,
            "XMLDATA" => Self::XmlData,
            "XID" => Self::Xid,
            "XIDREF" => Self::XidRef,
            other => Self::Other(other.to_string()),
        }
    }
}

/// A field definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Field {
    /// The tag number.
    pub tag: u32,
    /// The name, e.g. `ClOrdID`.
    pub name: String,
    /// The data type.
    pub ty: FieldType,
    /// The permitted values, for enumerated fields; empty otherwise.
    pub values: Vec<Value>,
    /// The documentation (Orchestra's synopsis); `None` if there is none, as in QuickFIX.
    pub doc: Option<String>,
    /// Whether the FIX standard deprecates the field. It's still valid on the wire.
    pub deprecated: bool,
}

/// A permitted value of an enumerated field.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Value {
    /// The value as sent, e.g. `1`.
    pub code: String,
    /// The official symbolic name, e.g. `Buy`, from Orchestra; `None` in QuickFIX.
    pub name: Option<String>,
    /// QuickFIX's name for it, e.g. `BUY`; empty in Orchestra, and may be empty in QuickFIX.
    pub description: String,
    /// The documentation (Orchestra's synopsis); `None` if there is none, as in QuickFIX.
    pub doc: Option<String>,
}

/// An entry in a message, component, group, header or trailer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Member {
    /// A field.
    Field {
        /// The field's name.
        name: String,
        /// Whether it must be present.
        required: bool,
    },
    /// A repeating group, named by its NumInGroup field. The first member is the delimiter.
    Group {
        /// The NumInGroup field's name.
        name: String,
        /// The group's official name, e.g. `PreAllocGrp`, from Orchestra; `None` in QuickFIX.
        official_name: Option<String>,
        /// Whether it must be present.
        required: bool,
        /// Each entry's members, in order.
        members: Vec<Member>,
    },
    /// A component, whose members are included in place.
    Component {
        /// The component's name.
        name: String,
        /// Whether it must be present.
        required: bool,
    },
}

impl Member {
    /// The field, group or component name.
    pub fn name(&self) -> &str {
        match self {
            Self::Field { name, .. } | Self::Group { name, .. } | Self::Component { name, .. } => name,
        }
    }

    /// Whether the member must be present.
    pub fn required(&self) -> bool {
        match self {
            Self::Field { required, .. } | Self::Group { required, .. } | Self::Component { required, .. } => *required,
        }
    }
}

/// A named, reusable list of members.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Component {
    /// The name, e.g. `Instrument`.
    pub name: String,
    /// The members, in order.
    pub members: Vec<Member>,
}

/// Whether a message belongs to the session or application layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Session-level (`msgcat='admin'`).
    Admin,
    /// Application-level (any other `msgcat`).
    App,
}

/// A message definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Message {
    /// The name, e.g. `NewOrderSingle`.
    pub name: String,
    /// The MsgType(35) value, e.g. `D`.
    pub msg_type: String,
    /// Session or application.
    pub category: Category,
    /// The body's members, in order (the header and trailer are the dictionary's).
    pub members: Vec<Member>,
    /// The documentation (Orchestra's synopsis); `None` if there is none, as in QuickFIX.
    pub doc: Option<String>,
}

/// A validated dictionary: every field, component and group a message, header or trailer names
/// is defined, and no component includes itself.
#[derive(Debug, Clone)]
pub struct Dictionary {
    /// The protocol version.
    pub version: Version,
    pub(crate) header: Vec<Member>,
    pub(crate) trailer: Vec<Member>,
    pub(crate) fields: Vec<Field>,
    pub(crate) components: Vec<Component>,
    pub(crate) messages: Vec<Message>,
    field_names: HashMap<String, usize>,
    field_tags: HashMap<u32, usize>,
    component_names: HashMap<String, usize>,
    message_names: HashMap<String, usize>,
}

impl Dictionary {
    pub(crate) fn new(
        version: Version,
        header: Vec<Member>,
        trailer: Vec<Member>,
        fields: Vec<Field>,
        components: Vec<Component>,
        messages: Vec<Message>,
    ) -> Self {
        let mut dict = Self {
            version,
            header,
            trailer,
            fields,
            components,
            messages,
            field_names: HashMap::new(),
            field_tags: HashMap::new(),
            component_names: HashMap::new(),
            message_names: HashMap::new(),
        };
        dict.reindex();
        dict
    }

    pub(crate) fn reindex(&mut self) {
        self.field_names = self.fields.iter().enumerate().map(|(i, f)| (f.name.clone(), i)).collect();
        self.field_tags = self.fields.iter().enumerate().map(|(i, f)| (f.tag, i)).collect();
        self.component_names = self.components.iter().enumerate().map(|(i, c)| (c.name.clone(), i)).collect();
        self.message_names = self.messages.iter().enumerate().map(|(i, m)| (m.name.clone(), i)).collect();
    }

    /// The standard header's members, in order; empty in FIX 5.0+ application dictionaries,
    /// whose header is the transport's.
    pub fn header(&self) -> &[Member] {
        &self.header
    }

    /// The standard trailer's members, in order; empty like the header in FIX 5.0+.
    pub fn trailer(&self) -> &[Member] {
        &self.trailer
    }

    /// Fields in dictionary order.
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// The field with this name.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.field_names.get(name).map(|&i| &self.fields[i])
    }

    /// The field with this tag number.
    pub fn field_by_tag(&self, tag: u32) -> Option<&Field> {
        self.field_tags.get(&tag).map(|&i| &self.fields[i])
    }

    /// Components in dictionary order.
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// The component with this name.
    pub fn component(&self, name: &str) -> Option<&Component> {
        self.component_names.get(name).map(|&i| &self.components[i])
    }

    /// Messages in dictionary order.
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// The message with this name, e.g. `NewOrderSingle`.
    pub fn message(&self, name: &str) -> Option<&Message> {
        self.message_names.get(name).map(|&i| &self.messages[i])
    }
}
