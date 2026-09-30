//! Declarative macros that turn FIX dictionary definitions into Rust types. They are exported,
//! so applications can define their own messages, groups and enumerations; see [`fix_message!`].

/// Makes `FromFix + Copy` types their own borrowed form ([`FieldRef`](crate::fields::FieldRef)),
/// so they can be used as field types in [`fix_message!`](crate::fix_message):
/// `impl_field_ref!(MyType);`
#[macro_export]
macro_rules! impl_field_ref {
    ($($ty:ty),+ $(,)?) => {$(
        impl<'a> $crate::fields::FieldRef<'a> for $ty {
            type Ref = $ty;

            fn parse_ref(s: &'a str) -> Result<$ty, $crate::fields::ValueError> {
                <$ty as $crate::fields::FromFix>::from_fix(s)
            }

            fn into_owned(value: $ty) -> $ty {
                value
            }
        }
    )+};
}

/// Defines an enumerated field from its FIX codes: a `Copy` enum with `code`/`from_code`,
/// [`FromFix`](crate::fields::FromFix) (unknown codes are
/// [`ValueError::Incorrect`](crate::fields::ValueError::Incorrect)),
/// [`ToFix`](crate::fields::ToFix) and `Display` (the code), and is its own borrowed form
/// ([`FieldRef`](crate::fields::FieldRef)). Each variant's docs end with its code.
///
/// ```
/// turbojet::fix_enum! {
///     /// A venue's order-handling flag.
///     VenueFlag {
///         Normal = "N",
///         Urgent = "U",
///     }
/// }
/// assert_eq!(VenueFlag::from_code("U"), Some(VenueFlag::Urgent));
/// assert_eq!(VenueFlag::Normal.code(), "N");
/// ```
///
/// Variants are FIX's value names, which often share a prefix (`RoundUp`, `RoundDown`) or end with
/// the field's name (`NoSecurityType`), so the enum allows `clippy::enum_variant_names`.
#[macro_export]
macro_rules! fix_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident = $code:literal, )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        // FIX's value names often share a prefix or end with the field's name.
        #[allow(clippy::enum_variant_names)]
        pub enum $name {
            $(
                $(#[$vmeta])*
                #[doc = ""]
                #[doc = concat!("FIX code `", $code, "`.")]
                $variant,
            )+
        }

        impl $name {
            /// The value's FIX code.
            pub const fn code(self) -> &'static str {
                match self {
                    $( Self::$variant => $code, )+
                }
            }

            /// The value for a FIX code, if it's one of this enum's.
            pub fn from_code(code: &str) -> Option<Self> {
                match code {
                    $( $code => Some(Self::$variant), )+
                    _ => None,
                }
            }
        }

        impl $crate::fields::FromFix for $name {
            fn from_fix(s: &str) -> Result<Self, $crate::fields::ValueError> {
                Self::from_code(s).ok_or($crate::fields::ValueError::Incorrect)
            }
        }

        impl $crate::fields::FixEnum for $name {
            fn code(self) -> &'static str {
                $name::code(self)
            }

            fn from_code(code: &str) -> Option<Self> {
                $name::from_code(code)
            }
        }

        // So a lenient field compares with a value: `order.side == Side::Buy`.
        impl PartialEq<$name> for $crate::fields::Code<$name> {
            fn eq(&self, other: &$name) -> bool {
                matches!(self, $crate::fields::Code::Known(value) if value == other)
            }
        }

        impl PartialEq<$crate::fields::Code<$name>> for $name {
            fn eq(&self, other: &$crate::fields::Code<$name>) -> bool {
                other == self
            }
        }

        impl PartialEq<$name> for $crate::fields::CodeRef<'_, $name> {
            fn eq(&self, other: &$name) -> bool {
                matches!(self, $crate::fields::CodeRef::Known(value) if value == other)
            }
        }

        impl PartialEq<$crate::fields::CodeRef<'_, $name>> for $name {
            fn eq(&self, other: &$crate::fields::CodeRef<'_, $name>) -> bool {
                other == self
            }
        }

        $crate::impl_field_ref!($name);

        impl $crate::fields::ToFix for $name {
            fn write_fix(&self, out: &mut String) {
                out.push_str(self.code())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.code())
            }
        }
    };
}

/// Defines a typed message body.
///
/// ```text
/// fix_message! {
///     /// Docs.
///     Name = MsgTypeVariant {           // or  Name = "U1" {  for a custom MsgType
///         field: req Type = TAG,        // required
///         field: opt Type = TAG,        // optional: Option<Type>
///         field: group Entry = TAG,     // repeating group by its NumInGroup tag: Vec<Entry>
///         field: req_group Entry = TAG, // required repeating group: a non-empty Vec<Entry>
///         field: data Vec<u8> = LEN => TAG,     // data field and its Length field
///         field: opt_data Vec<u8> = LEN => TAG, // optional data field: Option<Vec<u8>>
///     }
/// }
/// ```
///
/// - `Type` is anything implementing [`FromFix`](crate::fields::FromFix) and
///   [`ToFix`](crate::fields::ToFix): `String`, `u32`, `u64`, `i64`, `bool`,
///   [`Decimal`](crate::fields::Decimal), [`UtcTimestamp`](crate::fields::UtcTimestamp), or an
///   enum from [`fix_enum!`]. `Entry` is defined with [`fix_group!`](crate::fix_group).
/// - `TAG` is a path to a `u32` constant: Turbojet's are in
///   [`tags`](crate::message::tags); define your own for custom tags. Tags are matched as
///   patterns, so a misspelt one is a compile error.
/// - The message type is a [`MsgType`](crate::fields::MsgType) variant name, or a string for
///   types without one.
/// - A `req_group` fails to parse as missing (on its NumInGroup tag) when it is absent or has no
///   entries; a `group` may be empty. Both write nothing when empty.
/// - A data field (`data`, `opt_data`) holds any bytes, including SOH and bytes that aren't UTF-8.
///   It's written after its Length field, `LEN`, which is set from the value's length, so the
///   struct has no field of its own for it. Decoding a data field needs the pair in the session's
///   [`DataFields`](crate::message::DataFields); the standard ones, such as RawData(96), are
///   there already.
/// - Fields are written in declaration order, after the standard header the session adds.
///   Parsing is a single pass that takes each tag's first occurrence and ignores unknown tags.
///   A group that is malformed, or has an entry that fails to parse, fails the parse where it is
///   found; otherwise the error is for the first field, in declaration order, that is missing or
///   has a value that doesn't convert.
///
/// Generates the struct, [`FixMessage`](crate::message::FixMessage) and `From<_> for Message`.
///
/// ```
/// use turbojet::message::FixMessage;
/// use turbojet::{fix_enum, fix_group, fix_message, Message, MsgType};
///
/// // A venue's custom tags, alongside the standard ones.
/// mod venue_tags {
///     pub use turbojet::message::tags::*;
///     pub const VENUE_FLAG: u32 = 5001;
///     pub const NO_NOTES: u32 = 5002;
///     pub const NOTE: u32 = 5003;
/// }
/// use venue_tags::*;
///
/// fix_enum! {
///     VenueFlag { Normal = "N", Urgent = "U", }
/// }
///
/// fix_group! {
///     Note { note: req String = NOTE }
/// }
///
/// fix_message! {
///     /// The venue's order acknowledgement, MsgType U1.
///     VenueAck = "U1" {
///         cl_ord_id: req String = CL_ORD_ID,
///         flag: opt VenueFlag = VENUE_FLAG,
///         notes: group Note = NO_NOTES,
///     }
/// }
///
/// let ack = VenueAck {
///     cl_ord_id: "ORD1".into(),
///     flag: Some(VenueFlag::Urgent),
///     notes: vec![Note { note: "queued".into() }],
/// };
/// let msg: Message = ack.clone().into();
/// assert_eq!(msg.to_string(), "35=U1|11=ORD1|5001=U|5002=1|5003=queued|");
/// assert_eq!(msg.msg_type(), MsgType::from_code("U1"));
/// assert_eq!(msg.parse::<VenueAck>().unwrap(), ack);
/// ```
///
/// A tag that doesn't exist fails to compile, rather than silently matching everything:
///
/// ```compile_fail
/// use turbojet::message::tags::*;
/// turbojet::fix_message! {
///     Oops = NewOrderSingle { cl_ord_id: req String = CL_ORD_IDD }
/// }
/// ```
#[macro_export]
macro_rules! fix_message {
    // Each field's tags travel as one token tree, `[TAG]` or, for a data field, `[LEN => DATA]`.
    (
        $(#[$meta:meta])*
        $name:ident = $msg_type:ident {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),* $(,)?
        }
    ) => {
        $crate::fix_message!(@message [$(#[$meta])*] $name, $crate::fields::MsgType::$msg_type,
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),*);
    };
    (
        $(#[$meta:meta])*
        $name:ident = $msg_type:literal {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),* $(,)?
        }
    ) => {
        $crate::fix_message!(@message [$(#[$meta])*] $name, $crate::fields::MsgType::from_static($msg_type),
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),*);
    };
    (@message [$(#[$meta:meta])*] $name:ident, $msg_type:expr,
        $( [$(#[$fmeta:meta])*] $field:ident : $presence:ident $ty:ty = $tags:tt ),*) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@type $presence $ty), )*
        }

        impl $crate::message::FixMessage for $name {
            const MSG_TYPE: $crate::fields::MsgType = $msg_type;

            fn from_message(msg: &$crate::message::Message) -> Result<Self, $crate::message::FieldError> {
                $crate::fix_message!(@parse msg.body(), false, $( $field : $presence $ty = $tags ),*)
            }

            fn from_message_strict(msg: &$crate::message::Message) -> Result<Self, $crate::message::FieldError> {
                $crate::fix_message!(@parse msg.body(), true, $( $field : $presence $ty = $tags ),*)
            }

            fn to_message(&self) -> $crate::message::Message {
                // Room for MsgType, every field, and typical value lengths.
                const FIELDS: usize = 1 $( + $crate::fix_message!(@one $field) )*;
                #[allow(unused_mut)]
                let mut msg = $crate::message::Message::with_capacity(Self::MSG_TYPE, 16 * FIELDS, FIELDS);
                $( $crate::fix_message!(@write $presence msg, $tags, &self.$field); )*
                msg
            }
        }

        impl From<$name> for $crate::message::Message {
            fn from(body: $name) -> Self {
                $crate::message::FixMessage::to_message(&body)
            }
        }
    };
    // One pass over the fields: each tag's first occurrence is converted straight into its field's
    // slot, and a group's NumInGroup field consumes the whole group there and then. Group entries
    // are skipped over, so a tag inside a group can't be mistaken for one outside it.
    //
    // Most of a large message's fields are usually absent, so an absent field must cost next to
    // nothing: its slot is already the field's final value, with no per-field work afterwards.
    // Errors are reported as if every field were converted in declaration order after the pass:
    // a group error ends the pass at once, but a conversion failure is only noted (keeping the
    // earliest-declared one), and at the end the first of it and any missing required field wins.
    // With `$strict`, the first body tag not declared here is an error, once the rest is valid.
    (@parse $fields:expr, $strict:expr, $( $field:ident : $presence:ident $ty:ty = $tags:tt ),*) => {{
        let fields: $crate::message::Fields<'_> = $fields;
        $( #[allow(unused_mut)] let mut $field = $crate::fix_message!(@slot $presence $ty); )*
        #[allow(unused_mut)]
        let mut failed: ::std::option::Option<$crate::message::FieldError> = None;
        let mut undeclared: ::std::option::Option<u32> = None;
        // An item, so not hygienic: named to stay clear of callers' tag constants.
        const __DECLARED_TAGS: &[u32] = &[$( $crate::fix_message!(@key $tags) ),*];
        let (mut index, end) = fields.bounds();
        #[allow(unreachable_patterns)]
        while index < end {
            index = fields.skip_hidden(index);
            if index >= end {
                break;
            }
            let (tag, value) = fields.at(index);
            index = match tag {
                // A data field's Length field matches too, so it counts as declared.
                $( $crate::fix_message!(@pattern $tags) =>
                    $crate::fix_message!(@take $presence $ty, $field, fields, index, tag, value, failed, __DECLARED_TAGS, $tags), )*
                _ => {
                    if $strict && undeclared.is_none() && !$crate::message::is_header_or_trailer(tag) {
                        undeclared = Some(tag);
                    }
                    index + 1
                }
            };
        }
        if let Some(error) = failed {
            let failed_at = $crate::message::declared_at(__DECLARED_TAGS, error.tag);
            let mut _at = 0usize;
            $(
                if _at < failed_at && $crate::fix_message!(@missing $presence $field) {
                    return Err($crate::message::FieldError {
                        tag: $crate::fix_message!(@key $tags),
                        kind: $crate::message::FieldErrorKind::Missing,
                    });
                }
                _at += 1;
            )*
            return Err(error);
        }
        let value = Self { $( $field: $crate::fix_message!(@finish $presence $ty, $field, $tags), )* };
        if let Some(tag) = undeclared {
            return Err($crate::message::FieldError { tag, kind: $crate::message::FieldErrorKind::NotDefined });
        }
        Ok(value)
    }};
    (@one $field:ident) => { 1 };
    // A field's own tag: a data field's is the data tag.
    (@key [$tag:path]) => { $tag };
    (@key [$length:path => $data:path]) => { $data };
    // The tags a field takes when parsing.
    (@pattern [$tag:path]) => { $tag };
    (@pattern [$length:path => $data:path]) => { $length | $data };
    // For each member of a group, the Length field before it if it's a data field, else 0.
    (@length [$tag:path]) => { 0 };
    (@length [$length:path => $data:path]) => { $length };
    (@type req $ty:ty) => { $ty };
    (@type opt $ty:ty) => { Option<$ty> };
    (@type group $ty:ty) => { Vec<$ty> };
    (@type req_group $ty:ty) => { Vec<$ty> };
    (@type data $ty:ty) => { $ty };
    (@type opt_data $ty:ty) => { Option<$ty> };
    (@slot req $ty:ty) => { None::<$ty> };
    (@slot opt $ty:ty) => { None::<$ty> };
    (@slot group $ty:ty) => { None::<Vec<$ty>> };
    (@slot req_group $ty:ty) => { None::<Vec<$ty>> };
    (@slot data $ty:ty) => { None::<$ty> };
    (@slot opt_data $ty:ty) => { None::<$ty> };
    (@take req $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take opt $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take opt $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {{
        $crate::message::take_value::<$ty>(&mut $slot, &mut $failed, $declared, $tag, $value);
        $index + 1
    }};
    (@take req_group $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take group $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take group $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {{
        $crate::message::take_group::<$ty>(&mut $slot, &$fields, $index, $tag)?
    }};
    (@take data $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take opt_data $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take opt_data $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::message::take_data(&mut $slot, &$fields, $index, $tag, $crate::fix_message!(@key $tags))
    };
    // Whether a field fails as missing, for the error path.
    (@missing req $slot:ident) => { $slot.is_none() };
    (@missing opt $slot:ident) => { false };
    (@missing group $slot:ident) => { false };
    (@missing req_group $slot:ident) => { $slot.as_ref().is_none_or(|entries| entries.is_empty()) };
    (@missing data $slot:ident) => { $slot.is_none() };
    (@missing opt_data $slot:ident) => { false };
    (@finish req $ty:ty, $slot:ident, $tags:tt) => { $crate::message::required($crate::fix_message!(@key $tags), $slot)? };
    (@finish opt $ty:ty, $slot:ident, $tags:tt) => { $slot };
    (@finish group $ty:ty, $slot:ident, $tags:tt) => { $slot.unwrap_or_default() };
    (@finish req_group $ty:ty, $slot:ident, $tags:tt) => {
        $crate::message::required_group($crate::fix_message!(@key $tags), $slot)?
    };
    (@finish data $ty:ty, $slot:ident, $tags:tt) => { $crate::fix_message!(@finish req $ty, $slot, $tags) };
    (@finish opt_data $ty:ty, $slot:ident, $tags:tt) => { $slot };
    (@spec req $ty:ty) => { None };
    (@spec opt $ty:ty) => { None };
    (@spec group $ty:ty) => { Some(&<$ty as $crate::message::FixGroup>::SPEC) };
    (@spec req_group $ty:ty) => { Some(&<$ty as $crate::message::FixGroup>::SPEC) };
    (@spec data $ty:ty) => { None };
    (@spec opt_data $ty:ty) => { None };
    // Typed messages are built fresh with one field per tag, so fields are appended directly.
    (@write req $msg:ident, [$tag:path], $value:expr) => { $crate::message::write_value(&mut $msg, $tag, $value) };
    // Only a field that's set costs a call: most of a large message's optional fields are empty.
    (@write opt $msg:ident, [$tag:path], $value:expr) => {
        if let Some(value) = $value {
            $crate::message::write_value(&mut $msg, $tag, value);
        }
    };
    (@write req_group $msg:ident, $tags:tt, $value:expr) => {
        $crate::fix_message!(@write group $msg, $tags, $value)
    };
    (@write group $msg:ident, [$tag:path], $value:expr) => { $crate::message::write_group(&mut $msg, $tag, $value) };
    (@write data $msg:ident, [$length:path => $data:path], $value:expr) => {
        $crate::message::write_data(&mut $msg, $length, $data, $value)
    };
    (@write opt_data $msg:ident, [$length:path => $data:path], $value:expr) => {
        if let Some(value) = $value {
            $crate::message::write_data(&mut $msg, $length, $data, value);
        }
    };
}

/// Defines a repeating-group entry, with fields declared as in [`fix_message!`]. The first field
/// is the group's delimiter, which starts every entry; make it `req`, or, if it's a nested group
/// (its NumInGroup is then the delimiter), `req_group`. Groups may nest.
///
/// Generates the struct and [`FixGroup`](crate::message::FixGroup), including its
/// [`GroupSpec`](crate::message::GroupSpec).
#[macro_export]
macro_rules! fix_group {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),+ $(,)?
        }
    ) => {
        $crate::fix_group!(@group [$(#[$meta])*] $name,
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),+);
    };
    (@group [$(#[$meta:meta])*] $name:ident,
        $( [$(#[$fmeta:meta])*] $field:ident : $presence:ident $ty:ty = $tags:tt ),+) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@type $presence $ty), )+
        }

        impl $crate::message::FixGroup for $name {
            const SPEC: $crate::message::GroupSpec = $crate::message::GroupSpec {
                fields: &[ $( ($crate::fix_message!(@key $tags), $crate::fix_message!(@spec $presence $ty)) ),+ ],
                lengths: &[ $( $crate::fix_message!(@length $tags) ),+ ],
            };

            fn from_fields(entry: $crate::message::Fields<'_>) -> Result<Self, $crate::message::FieldError> {
                // An undeclared tag ends a group entry, so strictness is the message's to apply.
                $crate::fix_message!(@parse entry, false, $( $field : $presence $ty = $tags ),+)
            }

            #[allow(unused_mut)]
            fn write(&self, mut msg: &mut $crate::message::Message) {
                $( $crate::fix_message!(@write $presence msg, $tags, &self.$field); )+
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::message::tags::*;
    use crate::message::{FixMessage, Message, tags};

    fix_group! {
        /// A nested group, keyed by existing tags for the test.
        Note {
            id: req String = TRADING_SESSION_ID,
        }
    }

    fix_group! {
        /// A group with a member (Text) that also appears at the top level, and a nested group.
        Leg {
            account: req String = ALLOC_ACCOUNT,
            text: opt String = TEXT,
            notes: group Note = NO_TRADING_SESSIONS,
        }
    }

    fix_message! {
        TestOrder = NewOrderSingle {
            legs: group Leg = NO_ALLOCS,
            symbol: req String = SYMBOL,
            text: opt String = TEXT,
        }
    }

    fix_group! {
        /// An entry whose nested group is required.
        Allocation {
            account: req String = ALLOC_ACCOUNT,
            sessions: req_group Note = NO_TRADING_SESSIONS,
        }
    }

    fix_message! {
        TestAllocation = "J" {
            id: req String = CL_ORD_ID,
            allocs: req_group Allocation = NO_ALLOCS,
            text: opt String = TEXT,
        }
    }

    // Private, like generated code included in a binary, and with names FIX gives its values: every
    // variant with one prefix, and one ending with the enum's name. Clippy must accept it.
    fix_enum! {
        /// Rounding direction.
        RoundingDirection {
            RoundToNearest = "0",
            RoundDown = "1",
            RoundUp = "2",
        }
    }

    fix_enum! {
        /// A value ending with the enum's name.
        SecurityType {
            Future = "FUT",
            CommonStock = "CS",
            NoSecurityType = "NONE",
        }
    }

    #[test]
    fn enums_may_share_prefixes_and_repeat_their_name() {
        assert_eq!(RoundingDirection::from_code("1"), Some(RoundingDirection::RoundDown));
        assert_eq!(SecurityType::NoSecurityType.code(), "NONE");
    }

    fix_enum! {
        /// Side(54), for a lenient field.
        TestSide { Buy = "1", Sell = "2", }
    }

    fix_message! {
        LenientOrder = NewOrderSingle {
            side: req crate::fields::Code<TestSide> = SIDE,
            ord_type: opt crate::fields::Code<TestSide> = ORD_TYPE,
        }
    }

    #[test]
    fn lenient_fields_keep_unknown_codes() {
        use crate::fields::Code;
        let order: LenientOrder = raw("35=D|54=Z|40=1|").parse().unwrap();
        assert_eq!(order.side, Code::Unknown("Z".into()));
        assert_eq!(order.ord_type, Some(Code::Known(TestSide::Buy)));
        assert!(order.ord_type.as_ref().is_some_and(|t| *t == TestSide::Buy));
        assert_eq!(order.to_message().to_string(), "35=D|54=Z|40=1|");
        assert_eq!(order.to_message().parse::<LenientOrder>().unwrap(), order);
    }

    fix_group! {
        /// A group whose entries carry a password.
        Credential {
            user: req String = USERNAME,
            secret: opt crate::fields::Secret = PASSWORD,
        }
    }

    fix_message! {
        TestLogon = Logon {
            username: opt String = USERNAME,
            password: opt crate::fields::Secret = PASSWORD,
            new_password: opt crate::fields::Secret = NEW_PASSWORD,
            credentials: group Credential = NO_ALLOCS,
        }
    }

    #[test]
    fn debug_masks_passwords() {
        let logon = TestLogon {
            username: Some("trader".into()),
            password: Some("hunter2".into()),
            new_password: Some("hunter3".into()),
            credentials: vec![Credential { user: "ops".into(), secret: Some("hunter4".into()) }],
        };
        let debug = format!("{logon:?}");
        assert_eq!(
            debug,
            "TestLogon { username: Some(\"trader\"), password: Some(***), new_password: Some(***), \
             credentials: [Credential { user: \"ops\", secret: Some(***) }] }"
        );
        // On the wire, of course, they're sent as they are.
        assert!(logon.to_message().to_string().contains("|554=hunter2|"));
        assert!(!format!("{logon:#?}").contains("hunter"), "pretty-printed too");
        // Nothing to hide still shows as absent.
        let empty = TestLogon { username: None, password: None, new_password: None, credentials: vec![] };
        assert_eq!(
            format!("{empty:?}"),
            "TestLogon { username: None, password: None, new_password: None, credentials: [] }"
        );
    }

    fix_group! {
        /// An entry that starts with a nested group, whose NumInGroup is then the delimiter.
        Assignment {
            notes: req_group Note = NO_TRADING_SESSIONS,
            text: opt String = TEXT,
        }
    }

    fix_message! {
        TestAssignments = NewOrderSingle {
            assignments: group Assignment = NO_ALLOCS,
            symbol: req String = SYMBOL,
        }
    }

    #[test]
    fn a_group_can_start_with_a_nested_group() {
        let wire = "35=D|78=2|386=2|336=A|336=B|58=first|386=1|336=C|55=X|";
        let parsed: TestAssignments = raw(wire).parse().unwrap();
        let notes = |a: &Assignment| a.notes.iter().map(|n| n.id.clone()).collect::<Vec<_>>();
        assert_eq!(parsed.assignments.len(), 2);
        assert_eq!(
            (notes(&parsed.assignments[0]), parsed.assignments[0].text.as_deref()),
            (vec!["A".into(), "B".into()], Some("first"))
        );
        assert_eq!((notes(&parsed.assignments[1]), parsed.assignments[1].text.as_deref()), (vec!["C".into()], None));
        assert_eq!(parsed.symbol, "X");
        assert_eq!(parsed.to_message().to_string(), wire);
        // An entry without its leading group can't be delimited.
        let err = raw("35=D|78=1|58=lost|55=X|").parse::<TestAssignments>().unwrap_err();
        assert_eq!(err.tag, tags::TEXT);
    }

    fn raw(text: &str) -> Message {
        Message::from_fields(text.split('|').filter(|f| !f.is_empty()).map(|f| {
            let (tag, value) = f.split_once('=').unwrap();
            (tag.parse::<u32>().unwrap(), value)
        }))
    }

    #[test]
    fn typed_groups_nest_and_keep_their_fields_to_themselves() {
        // Text appears inside the first entry and, later, at the top level.
        let msg = raw("35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|");
        let order: TestOrder = msg.parse().unwrap();
        assert_eq!(order.text.as_deref(), Some("outside"), "not the group member's Text");
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.legs.len(), 2);
        assert_eq!(order.legs[0].text.as_deref(), Some("inside"));
        let notes: Vec<_> = order.legs[0].notes.iter().map(|n| n.id.as_str()).collect();
        assert_eq!(notes, ["S1", "S2"]);
        assert_eq!((order.legs[1].account.as_str(), order.legs[1].text.as_deref()), ("B", None));

        // Top-level Text absent: the one inside the group must not be picked up.
        let order: TestOrder = raw("35=D|78=1|79=A|58=inside|55=AAPL|").parse().unwrap();
        assert_eq!(order.text, None);
    }

    #[test]
    fn typed_groups_round_trip_in_wire_order() {
        let msg = raw("35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|");
        let order: TestOrder = msg.parse().unwrap();
        let written = order.to_message();
        assert_eq!(written.to_string(), "35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|");
        assert_eq!(written.parse::<TestOrder>().unwrap(), order);
        // Empty groups write nothing, not a zero count.
        let empty = TestOrder { legs: Vec::new(), symbol: "X".into(), text: None };
        assert_eq!(empty.to_message().to_string(), "35=D|55=X|");
    }

    #[test]
    fn typed_group_errors_surface_as_field_errors() {
        let err = raw("35=D|78=2|79=A|55=AAPL|").parse::<TestOrder>().unwrap_err();
        assert_eq!(err.tag, tags::NO_ALLOCS);
        // A nested group whose count disagrees with its entries.
        let err = raw("35=D|78=1|79=A|386=1|55=AAPL|").parse::<TestOrder>().unwrap_err();
        assert_eq!(err.tag, tags::NO_TRADING_SESSIONS, "nested count mismatch");
    }

    #[test]
    fn required_groups_parse_and_round_trip() {
        let wire = "35=J|11=A1|78=2|79=X|386=1|336=S1|79=Y|386=2|336=S2|336=S3|58=hi|";
        let alloc: TestAllocation = raw(wire).parse().unwrap();
        assert_eq!(alloc.allocs.len(), 2);
        assert_eq!(alloc.allocs[1].sessions.len(), 2);
        let written = alloc.to_message();
        assert_eq!(written.to_string(), wire);
        assert_eq!(written.parse::<TestAllocation>().unwrap(), alloc);
    }

    #[test]
    fn required_groups_absent_or_empty_are_missing() {
        let missing = |text: &str| {
            let err = raw(text).parse::<TestAllocation>().unwrap_err();
            assert_eq!(err.kind, crate::message::FieldErrorKind::Missing, "{text}");
            err.tag
        };
        assert_eq!(missing("35=J|11=A1|58=hi|"), tags::NO_ALLOCS, "absent");
        assert_eq!(missing("35=J|11=A1|78=0|58=hi|"), tags::NO_ALLOCS, "zero entries");
        assert_eq!(missing("35=J|11=A1|78=1|79=X|"), tags::NO_TRADING_SESSIONS, "nested absent");
        assert_eq!(missing("35=J|11=A1|78=1|79=X|386=0|"), tags::NO_TRADING_SESSIONS, "nested zero");
    }

    fix_message! {
        /// Fields whose values can fail to convert, declared out of tag order.
        TestErrors = "U9" {
            leaves: req u32 = LEAVES_QTY,
            id: req String = CL_ORD_ID,
            qty: opt u64 = ORDER_QTY,
            legs: group Leg = NO_ALLOCS,
            sessions: req_group Note = NO_TRADING_SESSIONS,
            gap_fill: opt bool = GAP_FILL_FLAG,
        }
    }

    #[test]
    fn the_first_error_in_declaration_order_wins_whatever_the_wire_order() {
        use crate::message::FieldErrorKind::*;
        let error = |text: &str| {
            let err = raw(text).parse::<TestErrors>().unwrap_err();
            (err.tag, err.kind)
        };
        let format = |value: &str| IncorrectFormat(value.into());
        // Conversion failures against each other: the earlier-declared field, not the earlier tag.
        assert_eq!(error("35=U9|11=A|123=x|38=y|151=z|"), (LEAVES_QTY, format("z")));
        assert_eq!(error("35=U9|151=1|11=A|123=x|38=y|386=1|336=S|"), (ORDER_QTY, format("y")));
        // Against missing fields, required fields and required groups alike.
        assert_eq!(error("35=U9|38=y|"), (LEAVES_QTY, Missing));
        assert_eq!(error("35=U9|151=z|"), (LEAVES_QTY, format("z")));
        assert_eq!(error("35=U9|38=y|151=1|"), (CL_ORD_ID, Missing));
        assert_eq!(error("35=U9|123=x|151=1|11=A|"), (NO_TRADING_SESSIONS, Missing));
        assert_eq!(error("35=U9|123=x|151=1|11=A|386=0|"), (NO_TRADING_SESSIONS, Missing));
        assert_eq!(error("35=U9|123=x|151=1|11=A|386=1|336=S|"), (GAP_FILL_FLAG, IncorrectValue("x".into())));
        // A malformed group ends the parse there, before any conversion failure is reported.
        assert_eq!(error("35=U9|151=z|78=2|79=A|"), (NO_ALLOCS, IncorrectNumInGroup { declared: 2, found: 1 }));
    }

    #[test]
    fn repeated_fields_convert_only_their_first_occurrence() {
        let parse = |text: &str| raw(text).parse::<TestErrors>();
        let good_then_bad = parse("35=U9|151=1|11=A|38=5|38=y|386=1|336=S|").unwrap();
        assert_eq!((good_then_bad.leaves, good_then_bad.qty), (1, Some(5)));
        let err = parse("35=U9|151=1|11=A|38=y|38=5|386=1|336=S|").unwrap_err();
        assert_eq!((err.tag, err.kind), (ORDER_QTY, crate::message::FieldErrorKind::IncorrectFormat("y".into())));
        let err = parse("35=U9|151=z|151=1|11=A|386=1|336=S|").unwrap_err();
        assert_eq!((err.tag, err.kind), (LEAVES_QTY, crate::message::FieldErrorKind::IncorrectFormat("z".into())));
        // Every occurrence malformed: the first one's value is reported.
        let err = parse("35=U9|151=z|151=w|11=A|386=1|336=S|").unwrap_err();
        assert_eq!((err.tag, err.kind), (LEAVES_QTY, crate::message::FieldErrorKind::IncorrectFormat("z".into())));
    }

    #[test]
    fn errors_inside_group_entries_fail_at_once() {
        // The entry lacks its required nested group; that's reported although ClOrdID, declared
        // first, is missing too.
        let err = raw("35=J|78=1|79=A|").parse::<TestAllocation>().unwrap_err();
        assert_eq!((err.tag, err.kind), (tags::NO_TRADING_SESSIONS, crate::message::FieldErrorKind::Missing));
    }

    // ---- Data fields ----

    const VENUE_DATA_LEN: u32 = 5000;
    const VENUE_DATA: u32 = 5001;

    fix_group! {
        /// An entry with a venue's data field between two others.
        Attachment {
            account: req String = ALLOC_ACCOUNT,
            blob: opt_data Vec<u8> = VENUE_DATA_LEN => VENUE_DATA,
            text: opt String = TEXT,
        }
    }

    fix_message! {
        DataOrder = NewOrderSingle {
            raw_data: data Vec<u8> = RAW_DATA_LENGTH => RAW_DATA,
            xml_data: opt_data Vec<u8> = XML_DATA_LEN => XML_DATA,
            attachments: group Attachment = NO_ALLOCS,
            symbol: req String = SYMBOL,
        }
    }

    fn data_order() -> DataOrder {
        DataOrder {
            raw_data: b"\xff\x01\x00".to_vec(),
            xml_data: None,
            attachments: vec![
                Attachment { account: "A".into(), blob: Some(b"x\x01y".to_vec()), text: Some("t".into()) },
                Attachment { account: "B".into(), blob: None, text: None },
            ],
            symbol: "IBM".into(),
        }
    }

    #[test]
    fn data_fields_are_written_after_their_lengths() {
        let msg = data_order().to_message();
        let fields: Vec<(u32, &[u8])> = msg.fields_bytes().collect();
        assert_eq!(
            fields,
            [
                (MSG_TYPE, &b"D"[..]),
                (RAW_DATA_LENGTH, b"3"),
                (RAW_DATA, b"\xff\x01\x00"),
                (NO_ALLOCS, b"2"),
                (ALLOC_ACCOUNT, b"A"),
                (VENUE_DATA_LEN, b"3"),
                (VENUE_DATA, b"x\x01y"),
                (TEXT, b"t"),
                (ALLOC_ACCOUNT, b"B"),
                (SYMBOL, b"IBM"),
            ]
        );
    }

    #[test]
    fn data_fields_round_trip_through_the_wire() {
        let order = data_order();
        let wire = crate::codec::encode(&order.to_message().with(BEGIN_STRING, "FIX.4.4")).unwrap();
        let data = crate::message::DataFields::standard().with(VENUE_DATA_LEN, VENUE_DATA);
        let crate::codec::Decoded::Message(msg, _) = crate::codec::decode_with(&wire, &data) else { panic!() };
        assert_eq!(msg.parse::<DataOrder>().unwrap(), order);
        assert_eq!(msg.parse_strict::<DataOrder>().unwrap(), order, "length fields are declared");
    }

    #[test]
    fn a_required_data_field_can_be_missing() {
        let err = raw("35=D|55=IBM|").parse::<DataOrder>().unwrap_err();
        assert_eq!((err.tag, err.kind), (RAW_DATA, crate::message::FieldErrorKind::Missing));
    }

    fix_message! {
        TextOrder = NewOrderSingle {
            raw_data: opt String = RAW_DATA,
        }
    }

    #[test]
    fn binary_data_in_a_text_field_is_a_format_error() {
        let msg = Message::new(crate::fields::MsgType::NewOrderSingle).with_data(RAW_DATA_LENGTH, RAW_DATA, b"\xff");
        let err = msg.parse::<TextOrder>().unwrap_err();
        assert_eq!((err.tag, err.kind), (RAW_DATA, crate::message::FieldErrorKind::IncorrectFormat(String::new())));
        let text = Message::new(crate::fields::MsgType::NewOrderSingle).with_data(RAW_DATA_LENGTH, RAW_DATA, b"ok");
        assert_eq!(text.parse::<TextOrder>().unwrap().raw_data.as_deref(), Some("ok"));
    }
}
