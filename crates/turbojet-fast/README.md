# turbojet-fast

FAST (FIX Adapted for STreaming) 1.1 for [Turbojet](https://github.com/jrogers/turbojet): a feed's
templates loaded from their XML at runtime, and its messages decoded into Turbojet `Message`s, each
field's `id` its FIX tag, so typed FIX messages read them like any other. An `Encoder` encodes
messages back.

```rust
use std::sync::Arc;
use turbojet::Message;
use turbojet_fast::{Decoder, Templates};

let templates = Arc::new(Templates::from_xml(&std::fs::read_to_string("templates.xml")?)?);
let mut decoder = Decoder::new(templates);
let mut msg = Message::default();
let decoded = decoder.decode_into(packet, &mut msg)?;
```

Every field type, operator and dictionary, sequences (as repeating groups), groups, and static and
dynamic template references are supported. It's a codec only: receiving the feed (multicast, line
arbitration, gap recovery) is the application's. See the
[README](https://github.com/jrogers/turbojet#fast) for more.

Licensed under Apache-2.0.
