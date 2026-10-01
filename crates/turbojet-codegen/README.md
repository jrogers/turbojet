# turbojet-codegen

Generates [Turbojet](https://github.com/jrogers/turbojet) typed messages from a FIX data
dictionary (FIX Orchestra or QuickFIX format): an enum per enumerated field, two structs per message
and repeating-group entry (an owned one, and a borrowed one named with `Ref` that parses without
allocating), and a tag constant per field. With an Orchestra file, groups and enum values get their
official names. Each item's doc comment gives its FIX name and tag; with
`--docs` (or `Generator::with_docs`), the dictionary's documentation is copied in too. That text is
FIX Protocol Limited's, so it's off by default: check its licence covers how you'll distribute the
generated code before turning it on. A crate shipping code generated from the official Orchestra
files should ship a `NOTICE` crediting them, like
[`turbojet-fix42`'s](https://github.com/jrogers/turbojet/blob/main/crates/turbojet-fix42/NOTICE).

From a `build.rs`:

```rust,no_run
use turbojet_dictionary::Dictionary;
use turbojet_codegen::Generator;

let mut dict = Dictionary::load("dictionaries/OrchestraFIX44.xml").unwrap();
dict.merge_file("dictionaries/venue.xml").unwrap();
let code = Generator::new(&dict).render().unwrap();
let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("venue.rs");
std::fs::write(out, code).unwrap();
```

and in the crate: `mod venue { include!(concat!(env!("OUT_DIR"), "/venue.rs")); }`. The crate
depends on `turbojet`. In a binary, which warns about what it doesn't use, put
`#[allow(dead_code)]` on the module.

The `turbojet-codegen` command writes the same code as module files, formatted with rustfmt, for
checking in:

```sh
turbojet-codegen OrchestraFIX42.xml --merge venue.xml --out src/generated
```

Enumerated fields are strict: an unknown code fails the parse. `--lenient-enums` (or
`Generator::lenient_enums`) types every enumerated field as `turbojet::fields::Code<E>` instead,
which keeps a code the dictionary doesn't list as `Code::Unknown`; `--lenient-enum OrdType` does it
for one field.

`--optional Owner.Member` corrects a dictionary that marks something required that counterparties
leave out, making a field, group or component optional in a message or component (see
`Dictionary::make_optional`); `turbojet-fix42` is generated with
`--optional QuoteCancel.NoQuoteEntries`.
