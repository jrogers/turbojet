# turbojet-message

The FIX message types of [Turbojet](https://github.com/jrogers/turbojet), a FIX engine in Rust: the
raw message, typed field values, the tag=value codec, and the `fix_message!`, `fix_group!` and
`fix_enum!` macros that typed messages are written in.

Applications use it through `turbojet`, which re-exports all of it (`turbojet::message`,
`turbojet::fields`, `turbojet::codec` and the macros), and the generated messages in
`turbojet-fix` build on it alone, so changes to the engine don't rebuild them. Depend on
`turbojet` rather than on this crate.
