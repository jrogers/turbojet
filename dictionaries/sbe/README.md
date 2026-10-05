# SBE message schemas

Schemas that `scripts/codegen.sh` generates SBE codecs from, for turbojet's tests, benchmarks and
fuzz target (`crates/turbojet/tests/sbe`). They aren't published with any crate.

- `example-schema.xml` and `common-types.xml`: SBE's example schema (the `Car` message), from
  [real-logic/simple-binary-encoding](https://github.com/real-logic/simple-binary-encoding)
  `sbe-samples/src/main/resources`, at commits `04d7b0f316835bbfbf3308bab669b9580fc69174` and
  `d77b7318744309f1b6c688d01486842773417b6c` respectively. Apache License, Version 2.0.
- `b3-entrypoint.xml`: B3's Binary Entrypoint schema, version 5.6 (FIXP session messages and B3's
  order entry), as `binary_entrypoint.xml` in
  [artiofix/artio](https://github.com/artiofix/artio)
  `artio-binary-entrypoint-codecs/src/main/resources/uk/co/real_logic/artio/entrypoint`, at commit
  `25ad83cee34772db3e5bc59103c649a280910409`. The schema is B3's; the Artio repository is under the
  Apache License, Version 2.0, and the file carries no licence of its own.

`scripts/sbe-golden.sh` encodes messages with real-logic's own codecs for these schemas, which the
generated Rust codecs must read and match byte for byte.
