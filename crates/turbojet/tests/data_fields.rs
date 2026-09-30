//! The standard data fields are exactly those the vendored dictionaries pair with a Length field.
#![cfg(feature = "validation")]

use std::collections::BTreeSet;

use turbojet::message::DataFields;
use turbojet_dictionary::Dictionary;

#[test]
fn standard_data_fields_match_the_dictionaries() {
    let mut found = BTreeSet::new();
    for file in ["OrchestraFIX42", "OrchestraFIX43", "OrchestraFIX44", "OrchestraFIX50SP2", "FIXTSession"] {
        let path = format!("{}/../../dictionaries/orchestra/{file}.xml", env!("CARGO_MANIFEST_DIR"));
        found.extend(Dictionary::load(&path).unwrap_or_else(|e| panic!("{e}")).data_fields());
    }
    let standard: BTreeSet<_> = DataFields::standard().pairs().collect();
    assert_eq!(standard, found);
}
