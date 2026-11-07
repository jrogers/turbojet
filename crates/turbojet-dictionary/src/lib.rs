//! FIX data dictionaries in the QuickFIX XML and FIX Orchestra formats: fields, components,
//! repeating groups and messages, validated on load, with venue customisations (in either format)
//! merged onto a base dictionary. An Orchestra customisation's refs may name the base's fields by
//! tag and, for an Orchestra base, its components and groups by id; a group it redefines changes
//! everywhere the base uses it.
//!
//! ```no_run
//! use turbojet_dictionary::Dictionary;
//!
//! let mut dict = Dictionary::from_file("OrchestraFIX42.xml")?;
//! dict.merge_file("venue.xml")?;
//! let side = dict.field("Side").unwrap();
//! assert_eq!(side.tag, 54);
//! # Ok::<(), turbojet_dictionary::Error>(())
//! ```
//!
//! The official FIX Orchestra files (FIX 4.2, FIX 4.4 and FIX Latest) are published in
//! [FIXTradingCommunity/orchestrations](https://github.com/FIXTradingCommunity/orchestrations).

// Library code handles every error or names the invariant that rules it out, in an `expect`
// (docs/STYLE.md). Tests, benches and examples may unwrap.
#![warn(clippy::unwrap_used)]

mod correct;
mod load;
mod merge;
mod model;
mod orchestra;

pub use load::Error;
pub use model::*;
