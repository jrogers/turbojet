# turbojet-dictionary

FIX data dictionaries in the QuickFIX XML and FIX Orchestra formats, for
[Turbojet](https://github.com/jrogers/turbojet): fields, components, repeating groups and
messages, validated on load, with venue customisations (in the QuickFIX format) merged onto a
base dictionary, and corrections for a dictionary's mistakes (`make_optional`, for something it
marks required that counterparties leave out).

```rust,no_run
use turbojet_dictionary::Dictionary;

let mut dict = Dictionary::load("OrchestraFIX44.xml")?;
dict.merge_file("venue.xml")?;
dict.make_optional("QuoteCancel", "NoQuoteEntries")?;
let side = dict.field("Side").unwrap();
assert_eq!(side.tag, 54);
# Ok::<(), turbojet_dictionary::Error>(())
```

The official FIX Orchestra files (FIX 4.2, FIX 4.4 and FIX Latest) are published in
[FIXTradingCommunity/orchestrations](https://github.com/FIXTradingCommunity/orchestrations).
`turbojet-codegen` generates typed Turbojet messages from a dictionary.
