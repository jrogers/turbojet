# Messages

A `Message` is a raw tag/value message: its fields in order, read with `msg.get(tag)` or typed
with `msg.field::<T>(tag)`. Most code reads and writes typed messages instead, generated from the
FIX data: `turbojet-fix` has every message of each FIX version, and the macros that generate them
are yours to use too.

## Typed messages

Messages, groups and enums are generated from their FIX definitions by three exported macros,
which Turbojet uses for its own types and applications can use for theirs, including custom tags
and venue-specific message types:

```rust
use turbojet::fields::CompactString;
use turbojet::{fix_enum, fix_group, fix_message};

mod venue_tags {
    pub use turbojet::message::tags::*;
    pub const PRIORITY: u32 = 5001;
    pub const NO_LEGS: u32 = 5002;
    pub const LEG_SYMBOL: u32 = 5003;
}
use venue_tags::*;

fix_enum! { Priority { Normal = "N", Urgent = "U", } }
fix_group! { Leg / LegRef { symbol: req CompactString = LEG_SYMBOL } }
fix_message! {
    /// A venue-specific spread order.
    SpreadOrder / SpreadOrderRef = "U1" { // or a MsgType variant, e.g. = NewOrderSingle
        cl_ord_id: req CompactString = CL_ORD_ID,
        priority: opt Priority = PRIORITY,
        legs: req_group Leg = NO_LEGS,    // at least one entry; `group` allows none
    }
}
```

Each message and group comes in two forms. `SpreadOrder` owns its values: build one to send, or
keep one. `SpreadOrderRef<'a>` borrows its strings, lists and groups from the message it was
parsed from, so parsing it allocates nothing; read messages with it, and call `into_owned()` on
one worth keeping:

```rust
# use turbojet::fields::CompactString;
# use turbojet::{fix_enum, fix_group, fix_message, Message};
# mod venue_tags {
#     pub use turbojet::message::tags::*;
#     pub const PRIORITY: u32 = 5001;
#     pub const NO_LEGS: u32 = 5002;
#     pub const LEG_SYMBOL: u32 = 5003;
# }
# use venue_tags::*;
# fix_enum! { Priority { Normal = "N", Urgent = "U", } }
# fix_group! { Leg / LegRef { symbol: req CompactString = LEG_SYMBOL } }
# fix_message! {
#     SpreadOrder / SpreadOrderRef = "U1" {
#         cl_ord_id: req CompactString = CL_ORD_ID,
#         priority: opt Priority = PRIORITY,
#         legs: req_group Leg = NO_LEGS,
#     }
# }
use turbojet::FixMessageRef; // into_owned

let msg: Message = "8=FIX.4.4|35=U1|49=A|56=B|34=2|11=spread-1|5001=U|5002=2|5003=ACME|5003=WIDGET".parse()?;
let order: SpreadOrderRef = msg.parse()?; // borrows from msg
for leg in &order.legs {
    println!("{}", leg.symbol);
}
let kept: SpreadOrder = order.into_owned(); // or msg.parse::<SpreadOrder>()?
assert_eq!(kept.legs.len(), 2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Text fields in the generated messages are `CompactString`s (re-exported as
`turbojet::fields::CompactString`), which keep values of up to 24 bytes inline, so typical IDs and
symbols are parsed and sent without allocating. Build one with `.into()` from a `&str` or `String`,
or `format_compact!` in place of `format!`. `String` works as a field type too.

Tags are paths to `u32` constants and are matched as patterns, so a misspelt tag is a compile
error. A malformed number or timestamp is *incorrect data format* (373=6), and a missing required
field is *required tag missing* (373=1). The generated crates' enumerated fields are
`fields::Code<E>`, which keeps a code the dictionary doesn't list as `Code::Unknown` rather than
failing, as QuickFIX does: compare one with a value (`order.side == Side::Buy`), set one with
`.into()`, and get the enum with `.known()`. Unknown codes are rejected by checking messages
against a data dictionary (feature `validation`, see [Session behaviour][sessions]), or by fields
typed as the enum itself (`turbojet-codegen --strict-enums`), where one is *value incorrect*
(373=5). Tags the message doesn't define are ignored; `msg.parse_strict()` refuses them instead,
as *tag not defined for this message type* (373=2), for counterparties whose extra fields should
be rejected. The raw `Message` stays available for anything not modelled.

## Messages as text

A message prints as text with `|` between fields (`println!("{msg}")`, or `msg.redacted()` with
passwords masked), and text in that form parses back into one, with SOH or `|` between fields:

```rust
use turbojet::Message;

let msg: Message = "8=FIX.4.4|9=20|35=0|49=A|56=B|34=1|10=125|".parse()?;
let msg: Message = "8=FIX.4.4|35=0|49=A|56=B|34=1".parse()?; // BodyLength and CheckSum added
# Ok::<(), Box<dyn std::error::Error>>(())
```

BodyLength(9) and CheckSum(10) are checked when they're there, data fields such as RawData(96)
are read by their Length fields, and the error names the field that's wrong. `codec::decode_text`
takes a venue's own data fields.

## Repeating groups

Groups are parsed from a `GroupSpec`: the group's fields in order, the first being the delimiter
that starts each entry, with nested groups by their NumInGroup tag. Typed messages declare them
with `fix_group!`, as a `Vec` in the owned form and a `Group` in the borrowed one, which reads
each entry as it's iterated; FIX 4.2 NewOrderSingle has `allocs` (NoAllocs 78) and
`trading_sessions` (NoTradingSessions 386), and ExecutionReport has `contra_brokers`
(NoContraBrokers 382).

```rust
use turbojet::message::GroupSpec;
use turbojet::Message;
use turbojet_fix::fix42::NewOrderSingleRef;

let msg: Message = "8=FIX.4.2|35=D|49=A|56=B|34=2|11=order-1|21=1|55=ACME|54=1|60=20261025-12:00:00|40=1|38=300|78=2|79=FUND1|80=100|79=FUND2|80=200".parse()?;
let order: NewOrderSingleRef = msg.parse()?;
for alloc in &order.allocs {
    println!("{} gets {:?}", alloc.alloc_account, alloc.alloc_shares);
}

// Raw access, without a typed definition: one zero-copy view per entry.
// Each field (the delimiter first), and for each a data field's Length tag, else 0.
const PARTIES: GroupSpec = GroupSpec::new(&[(448, None), (447, None), (452, None)], &[0; 3]);
let msg: Message = "8=FIX.4.4|35=D|49=A|56=B|34=2|453=2|448=BROKER|447=D|452=1|448=DESK|447=D|452=24".parse()?;
for party in msg.group(453, &PARTIES)? {
    println!("{:?} role {:?}", party.get(448), party.get(452));
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Typed parsing is a single pass: each tag is matched to its field, and a group's fields are
consumed where its count appears, so a tag used both inside a group and outside it is never
confused. A count that disagrees with the entries, an entry not starting with its delimiter, or a
member repeated within an entry is a `FieldError` (SessionRejectReason 16 or 15; these codes are
FIX 4.3+, as FIX 4.2 has none for groups).

[sessions]: sessions.md
