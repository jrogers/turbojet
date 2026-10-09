# turbojet-log

Prints the message log files that [Turbojet](https://github.com/jrogers/turbojet)'s
`FileMessageLog` writes, oldest first, with FIX field and value names, filtered by session,
direction, MsgType, a field's value or a time range.

```sh
cargo install turbojet-log
turbojet-log /var/log/fix --session 'FIX.4.4:US->THEM' --type D,8 --tag 11=ORD1 --since 12:00
```

Names come from the standard FIX 4.2, 4.3, 4.4 and 5.0 SP2 dictionaries (FIX Orchestra, by the
FIX Trading Community, Apache-2.0; see `dictionaries/NOTICE`), chosen by BeginString, or from
`--dictionary FILE` (Orchestra or QuickFIX XML). `turbojet-log --help` lists every option.
