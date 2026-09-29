//! FIX data dictionaries in the QuickFIX XML and FIX Orchestra formats: fields, components,
//! repeating groups and messages, validated on load, with venue customisations (in the QuickFIX
//! format) merged onto a base dictionary.
//!
//! ```no_run
//! use turbojet_dictionary::Dictionary;
//!
//! let mut dict = Dictionary::load("OrchestraFIX44.xml")?;
//! dict.merge_file("venue.xml")?;
//! let side = dict.field("Side").unwrap();
//! assert_eq!(side.tag, 54);
//! # Ok::<(), turbojet_dictionary::Error>(())
//! ```
//!
//! The official FIX Orchestra files (FIX 4.2, FIX 4.4 and FIX Latest) are published in
//! [FIXTradingCommunity/orchestrations](https://github.com/FIXTradingCommunity/orchestrations).

mod correct;
mod load;
mod merge;
mod model;
mod orchestra;

pub use load::Error;
pub use model::*;
