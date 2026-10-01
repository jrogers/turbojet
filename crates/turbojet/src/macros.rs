//! Declarative macros that turn FIX dictionary definitions into Rust types. They are exported,
//! so applications can define their own messages, groups and enumerations; see [`fix_message!`].

/// Makes `FromFix + Copy` types their own borrowed form ([`FieldRef`](crate::fields::FieldRef)),
/// so they can be used as field types in [`fix_message!`](crate::fix_message):
/// `impl_field_ref!(MyType);`. The types must also be `Debug` and `PartialEq`.
#[macro_export]
macro_rules! impl_field_ref {
    ($($ty:ty),+ $(,)?) => {$(
        impl<'a> $crate::fields::FieldRef<'a> for $ty {
            type Ref = $ty;

            fn parse_ref(s: &'a str) -> ::core::result::Result<$ty, $crate::fields::ValueError> {
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
            fn from_fix(s: &str) -> ::core::result::Result<Self, $crate::fields::ValueError> {
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

/// Defines a typed message body, in two forms: `Name`, which owns its values, and its borrowed
/// twin `NameRef<'a>`, whose string, data, list and group fields borrow from the message.
///
/// ```text
/// fix_message! {
///     /// Docs.
///     Name / NameRef = MsgTypeVariant { // or  Name / NameRef = "U1" {  for a custom MsgType
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
/// A message has at least one field. Its doc comments, derives and most other attributes are
/// `Name`'s alone, since `NameRef`'s fields have other types; `#[deprecated]`, `#[doc(hidden)]`
/// and the lint levels (`allow`, `expect`, `warn`, `deny`) apply to `NameRef` too, and a `#[cfg]`
/// to everything generated.
///
/// `NameRef<'a>` is invariant in `'a`, because its fields' types are trait projections
/// ([`FieldRef::Ref`](crate::fields::FieldRef::Ref)). That only matters when mixing borrows of
/// different lifetimes: a `NameRef<'long>` doesn't shorten to a `NameRef<'short>` by itself.
///
/// - `Type` is anything implementing [`FromFix`](crate::fields::FromFix),
///   [`ToFix`](crate::fields::ToFix) and [`FieldRef`](crate::fields::FieldRef): `String`, `u32`,
///   `u64`, `i64`, `bool`, [`Decimal`](crate::fields::Decimal),
///   [`UtcTimestamp`](crate::fields::UtcTimestamp), or an enum from [`fix_enum!`]. `Entry` is
///   defined with [`fix_group!`](crate::fix_group).
/// - In `NameRef`, a field has its type's borrowed form ([`FieldRef::Ref`](crate::fields::FieldRef::Ref):
///   `&str` for a `String`), a group is a [`Group`](crate::message::Group) of `EntryRef`s, and a
///   data field is a `&[u8]`.
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
///   It builds `NameRef`, checking every field, so it allocates nothing; parsing `Name` is that
///   followed by [`into_owned`](crate::message::FixMessageRef::into_owned), and fails alike.
///   A group that is malformed, or has an entry that fails to parse, fails the parse where it is
///   found; otherwise the error is for the first field, in declaration order, that is missing or
///   has a value that doesn't convert.
///
/// Generates the struct, [`FixMessage`](crate::message::FixMessage) and `From<Name> for Message`;
/// and `NameRef`, [`FixMessageRef`](crate::message::FixMessageRef), `From<NameRef> for Name` and
/// [`FromMessage`](crate::message::FromMessage), so [`Message::parse`](crate::Message::parse)
/// gives either.
///
/// ```
/// use turbojet::message::{FixMessage, FixMessageRef};
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
///     Note / NoteRef { note: req String = NOTE }
/// }
///
/// fix_message! {
///     /// The venue's order acknowledgement, MsgType U1.
///     VenueAck / VenueAckRef = "U1" {
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
///
/// // Borrowed: the strings are the message's own.
/// let borrowed: VenueAckRef = msg.parse().unwrap();
/// assert_eq!(borrowed.cl_ord_id, "ORD1");
/// assert_eq!(borrowed.notes.iter().map(|n| n.note).collect::<Vec<_>>(), ["queued"]);
/// assert_eq!(borrowed.into_owned(), ack);
/// ```
///
/// A tag that doesn't exist fails to compile, rather than silently matching everything:
///
/// ```compile_fail
/// use turbojet::message::tags::*;
/// turbojet::fix_message! {
///     Oops / OopsRef = NewOrderSingle { cl_ord_id: req String = CL_ORD_IDD }
/// }
/// ```
#[macro_export]
macro_rules! fix_message {
    // Each field's tags travel as one token tree, `[TAG]` or, for a data field, `[LEN => DATA]`.
    // The message's attributes travel as token trees, so `@attrs` can sort them.
    (
        $(#[$($meta:tt)*])*
        $name:ident / $ref_name:ident = $msg_type:ident {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),+ $(,)?
        }
    ) => {
        $crate::fix_message!(@attrs fix_message [] [] [] [] [$(#[$($meta)*])*]
            $name, $ref_name, $crate::fields::MsgType::$msg_type,
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),+);
    };
    (
        $(#[$($meta:tt)*])*
        $name:ident / $ref_name:ident = $msg_type:literal {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),+ $(,)?
        }
    ) => {
        $crate::fix_message!(@attrs fix_message [] [] [] [] [$(#[$($meta)*])*]
            $name, $ref_name, $crate::fields::MsgType::from_static($msg_type),
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),+);
    };
    // Sorts a message's or group's attributes into its docs, which are the owned struct's alone;
    // those the borrowed struct shares (lint levels, `deprecated` and `doc(hidden)`, which mean the
    // same for both); the rest, such as derives, which are the owned struct's alone, since the
    // borrowed one's fields have other types; and its `cfg`s, which every generated item shares, so
    // a configured-out type leaves nothing behind. Then expands `$target!(@typed ..)`.
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[doc = $($value:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)* #[doc = $($value)*]] [$($shared)*] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[cfg $($predicate:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[cfg $($predicate)*]] [$($owned)*] [$($cfg)* #[cfg $($predicate)*]]
            [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[doc(hidden)] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[doc(hidden)]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[deprecated $($args:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[deprecated $($args)*]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[allow $($args:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[allow $($args)*]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[expect $($args:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[expect $($args)*]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[warn $($args:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[warn $($args)*]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[deny $($args:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)* #[deny $($args)*]] [$($owned)*] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [#[$($attr:tt)*] $($rest:tt)*] $($body:tt)*) => {
        $crate::fix_message!(@attrs $target [$($doc)*] [$($shared)*] [$($owned)* #[$($attr)*]] [$($cfg)*] [$($rest)*] $($body)*);
    };
    (@attrs $target:ident [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] [] $($body:tt)*) => {
        $crate::$target!(@typed [$($doc)*] [$($shared)*] [$($owned)*] [$($cfg)*] $($body)*);
    };
    (@typed [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] $name:ident, $ref_name:ident, $msg_type:expr,
        $( [$(#[$fmeta:meta])*] $field:ident : $presence:ident $ty:ty = $tags:tt ),+) => {
        $($doc)*
        $($shared)*
        $($owned)*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@type $presence $ty), )+
        }

        $($cfg)*
        #[allow(deprecated)]
        impl $crate::message::FixMessage for $name {
            const MSG_TYPE: $crate::fields::MsgType = $msg_type;
            type Ref<'a> = $ref_name<'a>;

            fn from_message(msg: &$crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                <$ref_name<'_> as $crate::message::FixMessageRef<'_>>::from_message(msg)
                    .map($crate::message::FixMessageRef::into_owned)
            }

            fn from_message_strict(msg: &$crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                <$ref_name<'_> as $crate::message::FixMessageRef<'_>>::from_message_strict(msg)
                    .map($crate::message::FixMessageRef::into_owned)
            }

            fn to_message(&self) -> $crate::message::Message {
                // Room for MsgType, every field, and typical value lengths.
                const FIELDS: usize = 1 $( + $crate::fix_message!(@one $field) )+;
                let mut msg = $crate::message::Message::with_capacity(Self::MSG_TYPE, 16 * FIELDS, FIELDS);
                $( $crate::fix_message!(@write $presence msg, $tags, &self.$field); )+
                msg
            }
        }

        $($cfg)*
        #[allow(deprecated)]
        impl From<$name> for $crate::message::Message {
            fn from(body: $name) -> Self {
                $crate::message::FixMessage::to_message(&body)
            }
        }

        #[doc = concat!("The borrowed form of [`", stringify!($name), "`]: parsed without allocating.")]
        $($shared)*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $ref_name<'a> {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@reftype 'a, $presence $ty), )+
        }

        $($cfg)*
        #[allow(deprecated)]
        impl<'a> $crate::message::FixMessageRef<'a> for $ref_name<'a> {
            type Owned = $name;

            fn from_message(msg: &'a $crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                $crate::fix_message!(@parse 'a, msg.body(), false, $( $field : $presence $ty = $tags ),+)
            }

            fn from_message_strict(msg: &'a $crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                $crate::fix_message!(@parse 'a, msg.body(), true, $( $field : $presence $ty = $tags ),+)
            }

            fn into_owned(self) -> $name {
                $name { $( $field: $crate::fix_message!(@into_owned 'a, $presence $ty, self.$field), )+ }
            }
        }

        $($cfg)*
        #[allow(deprecated)]
        impl<'a> $crate::message::FromMessage<'a> for $ref_name<'a> {
            const PARSED_MSG_TYPE: $crate::fields::MsgType = <Self as $crate::message::FixMessageRef<'a>>::MSG_TYPE;

            fn parse_from(msg: &'a $crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                <Self as $crate::message::FixMessageRef<'a>>::from_message(msg)
            }

            fn parse_strict_from(msg: &'a $crate::message::Message) -> ::core::result::Result<Self, $crate::message::FieldError> {
                <Self as $crate::message::FixMessageRef<'a>>::from_message_strict(msg)
            }
        }

        $($cfg)*
        #[allow(deprecated)]
        impl From<$ref_name<'_>> for $name {
            fn from(body: $ref_name<'_>) -> Self {
                $crate::message::FixMessageRef::into_owned(body)
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
    //
    // It builds the borrowed form, `Self`, whose values borrow from the message, `$lt`.
    (@parse $lt:lifetime, $fields:expr, $strict:expr, $( $field:ident : $presence:ident $ty:ty = $tags:tt ),+) => {{
        let fields: $crate::message::Fields<$lt> = $fields;
        $( let mut $field = $crate::fix_message!(@slot $lt, $presence $ty); )+
        #[allow(unused_mut)]
        let mut failed: ::std::option::Option<$crate::message::FieldError> = None;
        let mut undeclared: ::std::option::Option<u32> = None;
        // An item, so not hygienic: named to stay clear of callers' tag constants.
        const __DECLARED_TAGS: &[u32] = &[$( $crate::fix_message!(@key $tags) ),+];
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
                    $crate::fix_message!(@take $lt, $presence $ty, $field, fields, index, tag, value, failed, __DECLARED_TAGS, $tags), )+
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
            )+
            return Err(error);
        }
        let value = Self { $( $field: $crate::fix_message!(@finish $presence $ty, $field, $tags), )+ };
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
    // A field's type in the borrowed form, borrowing from the message for `$lt`.
    (@reftype $lt:lifetime, req $ty:ty) => { <$ty as $crate::fields::FieldRef<$lt>>::Ref };
    (@reftype $lt:lifetime, opt $ty:ty) => { Option<<$ty as $crate::fields::FieldRef<$lt>>::Ref> };
    (@reftype $lt:lifetime, group $ty:ty) => {
        $crate::message::Group<$lt, <$ty as $crate::message::FixGroup>::Ref<$lt>>
    };
    (@reftype $lt:lifetime, req_group $ty:ty) => { $crate::fix_message!(@reftype $lt, group $ty) };
    (@reftype $lt:lifetime, data $ty:ty) => { &$lt [u8] };
    (@reftype $lt:lifetime, opt_data $ty:ty) => { Option<&$lt [u8]> };
    // A borrowed field's owned value.
    (@into_owned $lt:lifetime, req $ty:ty, $value:expr) => { <$ty as $crate::fields::FieldRef<$lt>>::into_owned($value) };
    (@into_owned $lt:lifetime, opt $ty:ty, $value:expr) => { $value.map(<$ty as $crate::fields::FieldRef<$lt>>::into_owned) };
    (@into_owned $lt:lifetime, group $ty:ty, $value:expr) => { $value.into_owned() };
    (@into_owned $lt:lifetime, req_group $ty:ty, $value:expr) => { $value.into_owned() };
    (@into_owned $lt:lifetime, data $ty:ty, $value:expr) => { $value.to_vec() };
    (@into_owned $lt:lifetime, opt_data $ty:ty, $value:expr) => { $value.map(<[u8]>::to_vec) };
    // Each field's slot while parsing: its borrowed value, if found yet.
    (@slot $lt:lifetime, req $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, req $ty)> };
    (@slot $lt:lifetime, opt $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, req $ty)> };
    (@slot $lt:lifetime, group $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, group $ty)> };
    (@slot $lt:lifetime, req_group $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, group $ty)> };
    (@slot $lt:lifetime, data $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, data $ty)> };
    (@slot $lt:lifetime, opt_data $ty:ty) => { None::<$crate::fix_message!(@reftype $lt, data $ty)> };
    (@take $lt:lifetime, req $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take $lt, opt $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take $lt:lifetime, opt $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {{
        $crate::message::take_value_ref::<$ty>(&mut $slot, &mut $failed, $declared, $tag, $value);
        $index + 1
    }};
    (@take $lt:lifetime, req_group $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take $lt, group $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take $lt:lifetime, group $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {{
        $crate::message::take_group_ref::<<$ty as $crate::message::FixGroup>::Ref<$lt>>(&mut $slot, &$fields, $index, $tag)?
    }};
    (@take $lt:lifetime, data $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::fix_message!(@take $lt, opt_data $ty, $slot, $fields, $index, $tag, $value, $failed, $declared, $tags)
    };
    (@take $lt:lifetime, opt_data $ty:ty, $slot:ident, $fields:ident, $index:ident, $tag:ident, $value:ident, $failed:ident, $declared:ident, $tags:tt) => {
        $crate::message::take_data_ref(&mut $slot, &$fields, $index, $tag, $crate::fix_message!(@key $tags))
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
        $crate::message::required_group_ref($crate::fix_message!(@key $tags), $slot)?
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

/// Defines a repeating-group entry, `Name / NameRef { fields }`, with fields declared as in
/// [`fix_message!`], which also says what the borrowed form `NameRef<'a>` holds. The first field
/// is the group's delimiter, which starts every entry; make it `req`, or, if it's a nested group
/// (its NumInGroup is then the delimiter), `req_group`. Groups may nest.
///
/// Generates the struct and [`FixGroup`](crate::message::FixGroup), including its
/// [`GroupSpec`](crate::message::GroupSpec); and `NameRef`,
/// [`FixGroupRef`](crate::message::FixGroupRef) and `From<NameRef> for Name`.
#[macro_export]
macro_rules! fix_group {
    (
        $(#[$($meta:tt)*])*
        $name:ident / $ref_name:ident {
            $( $(#[$fmeta:meta])* $field:ident : $presence:ident $ty:ty = $tag:path $(=> $data:path)? ),+ $(,)?
        }
    ) => {
        $crate::fix_message!(@attrs fix_group [] [] [] [] [$(#[$($meta)*])*] $name, $ref_name,
            $( [$(#[$fmeta])*] $field : $presence $ty = [$tag $(=> $data)?] ),+);
    };
    (@typed [$($doc:tt)*] [$($shared:tt)*] [$($owned:tt)*] [$($cfg:tt)*] $name:ident, $ref_name:ident,
        $( [$(#[$fmeta:meta])*] $field:ident : $presence:ident $ty:ty = $tags:tt ),+) => {
        $($doc)*
        $($shared)*
        $($owned)*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@type $presence $ty), )+
        }

        $($cfg)*
        #[allow(deprecated)]
        impl $crate::message::FixGroup for $name {
            const SPEC: $crate::message::GroupSpec = $crate::message::GroupSpec {
                fields: &[ $( ($crate::fix_message!(@key $tags), $crate::fix_message!(@spec $presence $ty)) ),+ ],
                lengths: &[ $( $crate::fix_message!(@length $tags) ),+ ],
            };
            type Ref<'a> = $ref_name<'a>;

            fn from_fields(entry: $crate::message::Fields<'_>) -> ::core::result::Result<Self, $crate::message::FieldError> {
                <$ref_name<'_> as $crate::message::FixGroupRef<'_>>::from_fields(entry)
                    .map($crate::message::FixGroupRef::into_owned)
            }

            #[allow(unused_mut)]
            fn write(&self, mut msg: &mut $crate::message::Message) {
                $( $crate::fix_message!(@write $presence msg, $tags, &self.$field); )+
            }
        }

        #[doc = concat!("The borrowed form of [`", stringify!($name), "`]: parsed without allocating.")]
        $($shared)*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $ref_name<'a> {
            $( $(#[$fmeta])* pub $field: $crate::fix_message!(@reftype 'a, $presence $ty), )+
        }

        $($cfg)*
        #[allow(deprecated)]
        impl<'a> $crate::message::FixGroupRef<'a> for $ref_name<'a> {
            type Owned = $name;

            fn from_fields(entry: $crate::message::Fields<'a>) -> ::core::result::Result<Self, $crate::message::FieldError> {
                // An undeclared tag ends a group entry, so strictness is the message's to apply.
                $crate::fix_message!(@parse 'a, entry, false, $( $field : $presence $ty = $tags ),+)
            }

            fn into_owned(self) -> $name {
                $name { $( $field: $crate::fix_message!(@into_owned 'a, $presence $ty, self.$field), )+ }
            }
        }

        $($cfg)*
        #[allow(deprecated)]
        impl From<$ref_name<'_>> for $name {
            fn from(entry: $ref_name<'_>) -> Self {
                $crate::message::FixGroupRef::into_owned(entry)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::message::tags::*;
    use crate::message::{FieldError, FieldErrorKind, FixMessage, FixMessageRef, Message, tags};

    fix_group! {
        /// A nested group, keyed by existing tags for the test.
        Note / NoteRef {
            id: req String = TRADING_SESSION_ID,
        }
    }

    fix_group! {
        /// A group with a member (Text) that also appears at the top level, and a nested group.
        Leg / LegRef {
            account: req String = ALLOC_ACCOUNT,
            text: opt String = TEXT,
            notes: group Note = NO_TRADING_SESSIONS,
        }
    }

    fix_message! {
        TestOrder / TestOrderRef = NewOrderSingle {
            legs: group Leg = NO_ALLOCS,
            symbol: req String = SYMBOL,
            text: opt String = TEXT,
        }
    }

    fix_group! {
        /// An entry whose nested group is required.
        Allocation / AllocationRef {
            account: req String = ALLOC_ACCOUNT,
            sessions: req_group Note = NO_TRADING_SESSIONS,
        }
    }

    fix_message! {
        TestAllocation / TestAllocationRef = "J" {
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
        LenientOrder / LenientOrderRef = NewOrderSingle {
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
        Credential / CredentialRef {
            user: req String = USERNAME,
            secret: opt crate::fields::Secret = PASSWORD,
        }
    }

    fix_message! {
        TestLogon / TestLogonRef = Logon {
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
        Assignment / AssignmentRef {
            notes: req_group Note = NO_TRADING_SESSIONS,
            text: opt String = TEXT,
        }
    }

    fix_message! {
        TestAssignments / TestAssignmentsRef = NewOrderSingle {
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
        TestErrors / TestErrorsRef = "U9" {
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
        Attachment / AttachmentRef {
            account: req String = ALLOC_ACCOUNT,
            blob: opt_data Vec<u8> = VENUE_DATA_LEN => VENUE_DATA,
            text: opt String = TEXT,
        }
    }

    fix_message! {
        DataOrder / DataOrderRef = NewOrderSingle {
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
        TextOrder / TextOrderRef = NewOrderSingle {
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

    // ---- The borrowed form ----

    /// Parses `text` as `T` and as `T::Ref`, leniently and strictly, and checks that the two
    /// agree: the borrowed value made owned is the owned value, or both fail with the same error.
    fn assert_same_parse<T: FixMessage + PartialEq + std::fmt::Debug>(text: &str) {
        let msg = raw(text);
        let owned = T::from_message(&msg);
        let borrowed = <T::Ref<'_> as FixMessageRef<'_>>::from_message(&msg);
        assert_eq!(borrowed.map(FixMessageRef::into_owned), owned, "{text}");
        let owned = T::from_message_strict(&msg);
        let borrowed = <T::Ref<'_> as FixMessageRef<'_>>::from_message_strict(&msg);
        assert_eq!(borrowed.map(FixMessageRef::into_owned), owned, "strict: {text}");
    }

    #[test]
    fn the_borrowed_form_parses_as_the_owned_one_does() {
        let orders = [
            "35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|",
            "35=D|78=1|79=A|58=inside|55=AAPL|",
            "35=D|78=2|79=A|55=AAPL|",
            "35=D|78=1|79=A|386=1|55=AAPL|",
            "35=D|55=X|9999=extra|",
            "35=D|58=no symbol|",
        ];
        for text in orders {
            assert_same_parse::<TestOrder>(text);
        }
        let allocations = [
            "35=J|11=A1|78=2|79=X|386=1|336=S1|79=Y|386=2|336=S2|336=S3|58=hi|",
            "35=J|11=A1|58=hi|",
            "35=J|11=A1|78=0|58=hi|",
            "35=J|11=A1|78=1|79=X|",
            "35=J|11=A1|78=1|79=X|386=0|",
            "35=J|78=1|79=A|",
        ];
        for text in allocations {
            assert_same_parse::<TestAllocation>(text);
        }
        let errors = [
            "35=U9|11=A|123=x|38=y|151=z|",
            "35=U9|151=1|11=A|123=x|38=y|386=1|336=S|",
            "35=U9|38=y|",
            "35=U9|151=z|",
            "35=U9|38=y|151=1|",
            "35=U9|123=x|151=1|11=A|",
            "35=U9|123=x|151=1|11=A|386=0|",
            "35=U9|123=x|151=1|11=A|386=1|336=S|",
            "35=U9|151=z|78=2|79=A|",
            "35=U9|151=1|11=A|38=5|38=y|386=1|336=S|",
            "35=U9|151=1|11=A|38=y|38=5|386=1|336=S|",
            "35=U9|151=z|151=w|11=A|386=1|336=S|",
            "35=U9|151=1|11=A|386=1|336=S|9999=x|",
        ];
        for text in errors {
            assert_same_parse::<TestErrors>(text);
        }
        assert_same_parse::<TestAssignments>("35=D|78=2|386=2|336=A|336=B|58=first|386=1|336=C|55=X|");
        assert_same_parse::<TestAssignments>("35=D|78=1|58=lost|55=X|");
        assert_same_parse::<LenientOrder>("35=D|54=Z|40=1|");
        assert_same_parse::<DataOrder>("35=D|55=IBM|");
    }

    #[test]
    fn borrowed_fields_point_into_the_message() {
        let msg = raw("35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|");
        let order = TestOrderRef::from_message(&msg).unwrap();
        assert_eq!((order.symbol, order.text), ("AAPL", Some("outside")));
        assert_eq!(order.symbol.as_ptr(), msg.get(SYMBOL).unwrap().as_ptr(), "not a copy");
        // Copy, and convertible either way to the owned form.
        let copy = order;
        assert_eq!(copy, order);
        let owned: TestOrder = order.into();
        assert_eq!(owned, msg.parse::<TestOrder>().unwrap());
    }

    #[test]
    fn message_parse_gives_the_borrowed_form() {
        let msg = raw("35=D|78=1|79=A|55=AAPL|9999=extra|");
        let order: TestOrderRef<'_> = msg.parse().unwrap();
        assert_eq!(order.symbol.as_ptr(), msg.get(SYMBOL).unwrap().as_ptr(), "not a copy");
        assert_eq!(order.into_owned(), msg.parse::<TestOrder>().unwrap());
        let err = msg.parse_strict::<TestOrderRef<'_>>().unwrap_err();
        assert_eq!((err.tag, err.kind), (9999, crate::message::FieldErrorKind::NotDefined));
        // The MsgType is checked first, as for the owned form.
        let err = raw("35=J|78=1|79=A|55=AAPL|").parse::<TestOrderRef<'_>>().unwrap_err();
        assert_eq!((err.tag, err.kind), (MSG_TYPE, crate::message::FieldErrorKind::IncorrectValue("J".into())));
    }

    #[test]
    fn a_borrowed_nested_group_iterates_as_the_owned_one() {
        let msg = raw("35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=AAPL|58=outside|");
        let borrowed = TestOrderRef::from_message(&msg).unwrap();
        let owned: TestOrder = msg.parse().unwrap();
        let borrowed_notes: Vec<Vec<&str>> =
            borrowed.legs.iter().map(|leg| leg.notes.iter().map(|note| note.id).collect()).collect();
        let owned_notes: Vec<Vec<&str>> =
            owned.legs.iter().map(|leg| leg.notes.iter().map(|note| note.id.as_str()).collect()).collect();
        assert_eq!(borrowed_notes, owned_notes);
        assert_eq!(borrowed_notes, [vec!["S1", "S2"], vec![]]);
        assert_eq!(borrowed.legs.into_owned(), owned.legs);
        // A required group is never empty.
        let msg = raw("35=J|11=A1|78=1|79=X|386=1|336=S1|");
        let alloc = TestAllocationRef::from_message(&msg).unwrap();
        assert_eq!(alloc.allocs.iter().next().map(|a| a.sessions.len()), Some(1));
    }

    #[test]
    fn borrowed_data_fields_are_the_message_bytes() {
        let wire = crate::codec::encode(&data_order().to_message().with(BEGIN_STRING, "FIX.4.4")).unwrap();
        let data = crate::message::DataFields::standard().with(VENUE_DATA_LEN, VENUE_DATA);
        let crate::codec::Decoded::Message(msg, _) = crate::codec::decode_with(&wire, &data) else { panic!() };
        let order = DataOrderRef::from_message(&msg).unwrap();
        assert_eq!(order.raw_data, b"\xff\x01\x00", "not UTF-8");
        assert_eq!(order.raw_data.as_ptr(), msg.get_bytes(RAW_DATA).unwrap().as_ptr(), "not a copy");
        assert_eq!(order.xml_data, None);
        let blobs: Vec<Option<&[u8]>> = order.attachments.iter().map(|a| a.blob).collect();
        assert_eq!(blobs, [Some(&b"x\x01y"[..]), None]);
        assert_eq!(order.into_owned(), data_order());
        assert_eq!(DataOrderRef::from_message_strict(&msg).unwrap(), order, "length fields are declared");
    }

    #[test]
    fn the_borrowed_form_is_strict_on_request() {
        let msg = raw("35=D|55=X|9999=extra|");
        assert!(TestOrderRef::from_message(&msg).is_ok());
        let err = TestOrderRef::from_message_strict(&msg).unwrap_err();
        assert_eq!((err.tag, err.kind), (9999, crate::message::FieldErrorKind::NotDefined));
        assert_eq!(<TestOrderRef<'_> as FixMessageRef<'_>>::MSG_TYPE, TestOrder::MSG_TYPE);
    }

    #[test]
    fn borrowed_debug_masks_passwords() {
        let msg = raw("35=A|553=trader|554=hunter2|78=1|553=ops|554=hunter4|");
        let logon = TestLogonRef::from_message(&msg).unwrap();
        let debug = format!("{logon:?}");
        assert_eq!(
            debug,
            "TestLogonRef { username: Some(\"trader\"), password: Some(***), new_password: None, \
             credentials: [CredentialRef { user: \"ops\", secret: Some(***) }] }"
        );
    }

    // ---- Attributes ----

    fix_group! {
        /// Configured out, with a tag that doesn't exist: anything generated for it that's left
        /// behind fails to compile.
        #[cfg(any())]
        GoneEntry / GoneEntryRef { id: req String = NO_SUCH_TAG }
    }

    fix_message! {
        #[cfg(any())]
        GoneOrder / GoneOrderRef = "U7" { entries: group GoneEntry = NO_SUCH_TAG }
    }

    /// Would clash with a `GoneOrderRef` left behind.
    #[allow(dead_code)]
    struct GoneOrderRef;

    fix_message! {
        /// Superseded, but still parsed; its generated impls mustn't warn.
        #[deprecated(note = "for the test")]
        OldOrder / OldOrderRef = "U8" { id: req String = CL_ORD_ID }
    }

    #[test]
    #[allow(deprecated)]
    fn a_deprecated_message_still_parses_both_ways() {
        let msg = raw("35=U8|11=A|");
        let borrowed = OldOrderRef::from_message(&msg).unwrap();
        assert_eq!(borrowed.id, "A");
        assert_eq!(msg.parse::<OldOrder>().unwrap(), borrowed.into_owned());
        assert_eq!(<OldOrderRef<'_> as FixMessageRef<'_>>::MSG_TYPE, OldOrder::MSG_TYPE, "the owned one's");
    }

    /// Names only the borrowed twin: if it weren't deprecated too, the expectation would go
    /// unfulfilled, which warns, and CI's `-D warnings` fails the build.
    #[test]
    #[expect(deprecated, reason = "the borrowed twin of a deprecated message is deprecated too")]
    fn a_deprecated_message_marks_its_borrowed_twin() {
        let msg = raw("35=U8|11=A|");
        assert_eq!(msg.parse::<OldOrderRef>().unwrap().id, "A");
    }

    fix_group! {
        /// Derives on a group entry, which the message's own derives need.
        #[derive(Hash, PartialOrd)]
        HashedEntry / HashedEntryRef { id: req String = ALLOC_ACCOUNT, qty: opt u32 = ALLOC_SHARES }
    }

    fix_message! {
        /// Derives the borrowed struct couldn't take: its `Group` is neither `Hash` nor
        /// `PartialOrd`.
        #[derive(Hash, PartialOrd)]
        HashedOrder / HashedOrderRef = "U9" {
            id: req String = CL_ORD_ID,
            entries: group HashedEntry = NO_ALLOCS,
        }
    }

    /// A derive on a message or group goes on the owned struct only, so one the borrowed struct
    /// can't take, as here, still compiles.
    #[test]
    fn derives_apply_to_the_owned_struct_only() {
        use std::collections::HashSet;
        let msg = raw("35=U9|11=A|78=1|79=X|80=5|");
        let order: HashedOrder = msg.parse().unwrap();
        assert_eq!(order.entries, [HashedEntry { id: "X".into(), qty: Some(5) }]);
        assert!(order <= order.clone());
        assert_eq!(HashSet::from([order.clone(), order]).len(), 1);
    }

    // ---- Edge cases, parsed both ways ----
    //
    // Each result here is what the parser before the borrowed form gave too.

    /// `text` parsed as `T` leniently and strictly, once [`assert_same_parse`] has checked that
    /// the borrowed form gives the same.
    fn parse_both<T: FixMessage + PartialEq + std::fmt::Debug>(
        text: &str,
    ) -> (Result<T, FieldError>, Result<T, FieldError>) {
        assert_same_parse::<T>(text);
        let msg = raw(text);
        (T::from_message(&msg), T::from_message_strict(&msg))
    }

    fn error(tag: u32, kind: FieldErrorKind) -> FieldError {
        FieldError { tag, kind }
    }

    fn leg(account: &str, text: Option<&str>, notes: &[&str]) -> Leg {
        let notes = notes.iter().map(|id| Note { id: (*id).into() }).collect();
        Leg { account: account.into(), text: text.map(Into::into), notes }
    }

    fn order(legs: Vec<Leg>, text: Option<&str>) -> TestOrder {
        TestOrder { legs, symbol: "X".into(), text: text.map(Into::into) }
    }

    #[test]
    fn a_repeated_group_is_skipped_and_its_entries_read_as_top_level_fields() {
        // The second NoAllocs is skipped, like any repeated field, without being checked; its
        // entry's AllocAccount is then a top-level tag that TestOrder doesn't declare.
        let (lenient, strict) = parse_both::<TestOrder>("35=D|78=1|79=A|78=1|79=B|55=X|");
        assert_eq!(lenient, Ok(order(vec![leg("A", None, &[])], None)));
        assert_eq!(strict, Err(error(ALLOC_ACCOUNT, FieldErrorKind::NotDefined)));
        // So a member that TestOrder also declares is read from the skipped entry, as the first
        // top-level occurrence.
        let (lenient, strict) = parse_both::<TestOrder>("35=D|78=1|79=A|58=in|78=1|79=B|58=second|55=X|58=out|");
        assert_eq!(lenient, Ok(order(vec![leg("A", Some("in"), &[])], Some("second"))));
        assert_eq!(strict, Err(error(ALLOC_ACCOUNT, FieldErrorKind::NotDefined)));
        // The repeat's count isn't checked against anything.
        let (lenient, strict) = parse_both::<TestOrder>("35=D|78=1|79=A|78=2|55=X|");
        assert_eq!(lenient, Ok(order(vec![leg("A", None, &[])], None)));
        assert_eq!(strict, lenient);
    }

    #[test]
    fn strict_parsing_refuses_a_group_member_outside_its_group() {
        let (lenient, strict) = parse_both::<TestOrder>("35=D|55=X|79=stray|");
        assert_eq!(lenient, Ok(order(vec![], None)));
        assert_eq!(strict, Err(error(ALLOC_ACCOUNT, FieldErrorKind::NotDefined)));
    }

    #[test]
    fn strict_parsing_refuses_a_nested_groups_count_at_the_top_level() {
        // NoTradingSessions is declared only in Leg, a NoAllocs entry: at the top level it's
        // undeclared, whether before NoAllocs, after it, or without it.
        for (text, legs) in [
            ("35=D|55=X|386=1|336=S|", vec![]),
            ("35=D|386=1|336=S|78=1|79=A|55=X|", vec![leg("A", None, &[])]),
            ("35=D|78=1|79=A|55=X|386=1|336=S|", vec![leg("A", None, &[])]),
        ] {
            let (lenient, strict) = parse_both::<TestOrder>(text);
            assert_eq!(lenient, Ok(order(legs, None)), "{text}");
            assert_eq!(strict, Err(error(NO_TRADING_SESSIONS, FieldErrorKind::NotDefined)), "{text}");
        }
        // Inside the entry it's the nested group; a member of that after the entry ends isn't.
        let (lenient, strict) = parse_both::<TestOrder>("35=D|78=1|79=A|386=1|336=S|55=X|336=T|");
        assert_eq!(lenient, Ok(order(vec![leg("A", None, &["S"])], None)));
        assert_eq!(strict, Err(error(TRADING_SESSION_ID, FieldErrorKind::NotDefined)));
    }

    #[test]
    fn strict_parsing_accepts_a_repeated_or_orphan_length_field() {
        // A data field's Length field is declared with it, so a repeat, or one without its data
        // field, is ignored like any other repeated or unused field. These messages are built
        // without decoding, so their lengths aren't checked against the data.
        let order = DataOrder { raw_data: b"abc".to_vec(), xml_data: None, attachments: vec![], symbol: "IBM".into() };
        for text in [
            "35=D|95=3|96=abc|95=3|55=IBM|",
            "35=D|95=3|96=abc|95=3|96=xyz|55=IBM|",
            "35=D|95=3|96=abc|212=5|55=IBM|",
            "35=D|212=5|95=3|96=abc|55=IBM|",
        ] {
            let (lenient, strict) = parse_both::<DataOrder>(text);
            assert_eq!(lenient, Ok(order.clone()), "{text}");
            assert_eq!(strict, lenient, "{text}");
        }
        // A Length field alone doesn't make its data field present.
        let (lenient, strict) = parse_both::<DataOrder>("35=D|95=3|55=IBM|");
        assert_eq!(lenient, Err(error(RAW_DATA, FieldErrorKind::Missing)));
        assert_eq!(strict, lenient);
        // A group member's Length field is undeclared at the top level...
        let (lenient, strict) = parse_both::<DataOrder>("35=D|95=3|96=abc|5000=1|55=IBM|");
        assert_eq!(lenient, Ok(order));
        assert_eq!(strict, Err(error(VENUE_DATA_LEN, FieldErrorKind::NotDefined)));
        // ...and repeated in an entry, like any member.
        let (lenient, strict) = parse_both::<DataOrder>("35=D|95=3|96=abc|78=1|79=A|5000=1|5000=1|5001=x|55=IBM|");
        assert_eq!(lenient, Err(error(VENUE_DATA_LEN, FieldErrorKind::RepeatingGroupOutOfOrder)));
        assert_eq!(strict, lenient);
    }

    #[test]
    fn a_group_reads_from_any_view_of_fields_as_from_a_message() {
        use crate::message::{FixGroup, FixGroupRef};
        let msg = raw("35=D|78=2|79=A|58=inside|386=2|336=S1|336=S2|79=B|55=X|58=outside|");
        let legs = vec![leg("A", Some("inside"), &["S1", "S2"]), leg("B", None, &[])];
        let mut fields = msg.body();
        assert_eq!(Leg::read(&mut fields, NO_ALLOCS), Ok(legs.clone()));
        assert_eq!(fields.get(TEXT), Some("outside"), "the entries are hidden from the view");
        assert_eq!(TestOrder::from_message(&msg), Ok(order(legs.clone(), Some("outside"))));
        // Entry by entry, owned and borrowed alike.
        let entries = msg.body().group(NO_ALLOCS, &Leg::SPEC).unwrap();
        assert_eq!(entries.len(), 2);
        for (entry, leg) in entries.into_iter().zip(&legs) {
            assert_eq!(Leg::from_fields(entry.clone()).as_ref(), Ok(leg));
            assert_eq!(LegRef::from_fields(entry).map(FixGroupRef::into_owned).as_ref(), Ok(leg));
        }

        let read = |text: &str| Leg::read(&mut raw(text).body(), NO_ALLOCS);
        assert_eq!(read("35=D|55=X|"), Ok(vec![]), "absent");
        let mismatch = |declared, found| FieldErrorKind::IncorrectNumInGroup { declared, found };
        assert_eq!(read("35=D|78=2|79=A|55=X|"), Err(error(NO_ALLOCS, mismatch(2, 1))));
        assert_eq!(read("35=D|78=1|79=A|386=2|336=S|55=X|"), Err(error(NO_TRADING_SESSIONS, mismatch(2, 1))));
        assert_eq!(read("35=D|78=1|58=lost|"), Err(error(TEXT, FieldErrorKind::RepeatingGroupOutOfOrder)));
        assert_eq!(read("35=D|78=x|79=A|"), Err(error(NO_ALLOCS, FieldErrorKind::IncorrectFormat("x".into()))));

        // An entry that scans but doesn't parse: its required nested group is absent.
        let msg = raw("35=J|78=1|79=X|");
        let missing = error(NO_TRADING_SESSIONS, FieldErrorKind::Missing);
        assert_eq!(Allocation::read(&mut msg.body(), NO_ALLOCS), Err(missing.clone()));
        let entry = msg.body().group(NO_ALLOCS, &Allocation::SPEC).unwrap().remove(0);
        assert_eq!(Allocation::from_fields(entry.clone()), Err(missing.clone()));
        assert_eq!(AllocationRef::from_fields(entry).map(FixGroupRef::into_owned), Err(missing));
    }
}
