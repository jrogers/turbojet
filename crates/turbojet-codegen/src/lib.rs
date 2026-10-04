//! Generates Turbojet typed messages from a FIX data dictionary: an enum per enumerated field,
//! two structs per message and repeating-group entry (an owned one, and a borrowed one named with
//! `Ref`), and a tag constant per field, as `turbojet::fix_enum!`, `fix_group!` and `fix_message!`
//! invocations.
//!
//! From a `build.rs`:
//!
//! ```no_run
//! use turbojet_dictionary::Dictionary;
//! use turbojet_codegen::Generator;
//!
//! let mut dict = Dictionary::load("dictionaries/OrchestraFIX44.xml").unwrap();
//! dict.merge_file("dictionaries/venue.xml").unwrap();
//! let code = Generator::new(&dict).render().unwrap();
//! let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("venue.rs");
//! std::fs::write(out, code).unwrap();
//! ```
//!
//! and in the crate: `mod venue { include!(concat!(env!("OUT_DIR"), "/venue.rs")); }`. The
//! crate depends on `turbojet`. In a binary, which warns about what it doesn't use, put
//! `#[allow(dead_code)]` on the module. The `turbojet-codegen` command writes the same code as
//! module files instead, for checking in.
//!
//! Components are flattened into the messages that use them, and admin messages are left out
//! (Turbojet defines them), as is BusinessMessageReject.
//!
//! Each item's docs name its field or message (`ClOrdID(11).`, `NewOrderSingle(D).`), with
//! "Deprecated in the FIX standard." for fields the standard deprecates. With
//! [`Generator::with_docs`], the dictionary's documentation (Orchestra's) follows, line for line,
//! and enum variants get their values' documentation. That text is © FIX Protocol Limited, so it's
//! off by default: check its licence covers how you'll distribute the generated code first. A
//! crate shipping code generated from the official Orchestra files should ship a `NOTICE`
//! crediting them, like `turbojet-fix42`'s.
//!
//! Generated enums are strict: a code the dictionary doesn't list makes `parse` fail with an
//! incorrect-value error. To accept a venue's extra values, merge its dictionary (`--merge
//! venue.xml`, or [`Dictionary::merge_file`]) before generating. To accept whatever arrives, make
//! fields lenient ([`Generator::lenient_enums`], [`Generator::lenient_enum`], or `--lenient-enums`
//! and `--lenient-enum Field`): they're typed `turbojet::fields::Code<E>`, which keeps an unknown
//! code as `Code::Unknown` instead of failing.
//!
//! A dictionary that marks something required that counterparties leave out can be corrected
//! before generating, with [`Dictionary::make_optional`] (`--optional Owner.Member` on the
//! command).
//!
//! A group entry's struct takes the group's official name when the dictionary has one (Orchestra:
//! `PreAllocGrp`, and a venue's group that's the same as an official one, see
//! [`Dictionary::merge_xml`]), and enum variants take their values' official names
//! (`OrderCancelRequest`, `VWAP`), as they are but for the identifier rules (`3Day` → `V3Day`,
//! `type` → `r#type`, `Self` → `Self_`; a name that can't be an identifier, such as `Good-Till`,
//! is an error). Otherwise a group entry's struct is named after its NumInGroup field
//! (`NoAllocs` → `Alloc`); when such a group has different definitions in different messages, each
//! is prefixed with the first message using it (`NewOrderSingleAlloc`), so merging a venue
//! dictionary that changes or reorders messages can rename group structs. Variants without an
//! official name are camel-cased from the value's description (`SELL_SHORT` → `SellShort`).

// Library code handles every error or names the invariant that rules it out, in an `expect`
// (STYLE.md). Tests, benches and examples may unwrap.
#![warn(clippy::unwrap_used)]

mod docs;
mod naming;
mod plan;
mod render;

use std::fmt;
use std::path::{Path, PathBuf};

use turbojet_dictionary::Dictionary;

/// Why code couldn't be generated from a dictionary.
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// An I/O error, naming the path.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> Error + '_ {
    move |e| Error(format!("{}: {e}", path.display()))
}

/// The generated modules' source, unformatted.
#[derive(Debug)]
pub struct Modules {
    /// `mod.rs`: declares the others and re-exports the enums, groups and messages.
    pub root: String,
    /// `tags.rs`: a `u32` constant per dictionary field.
    pub tags: String,
    /// `enums.rs`: a `fix_enum!` per enumerated field the messages use.
    pub enums: String,
    /// `groups.rs`: a `fix_group!` per repeating-group entry.
    pub groups: String,
    /// `messages.rs`: a `fix_message!` per application message.
    pub messages: String,
}

/// Generates code for a dictionary's application messages.
#[derive(Debug)]
pub struct Generator<'a> {
    dict: &'a Dictionary,
    skip: Vec<String>,
    docs: bool,
    lenient_all: bool,
    lenient: Vec<String>,
}

impl<'a> Generator<'a> {
    /// A generator for every application message in `dict` but BusinessMessageReject.
    #[must_use]
    pub fn new(dict: &'a Dictionary) -> Self {
        Self {
            dict,
            skip: vec!["BusinessMessageReject".to_string()],
            docs: false,
            lenient_all: false,
            lenient: Vec::new(),
        }
    }

    /// Whether to copy the dictionary's documentation (Orchestra's synopses) into the doc
    /// comments; off by default. The text is FIX Protocol Limited's, so check its licence covers
    /// how you'll distribute the generated code before turning this on.
    #[must_use]
    pub fn with_docs(mut self, docs: bool) -> Self {
        self.docs = docs;
        self
    }

    /// Whether every enumerated field is lenient: typed
    /// [`Code<E>`](https://docs.rs/turbojet/latest/turbojet/fields/enum.Code.html) rather than
    /// `E`, so a code the dictionary doesn't list is kept (`Code::Unknown`) instead of failing the
    /// parse. Off by default.
    #[must_use]
    pub fn lenient_enums(mut self, all: bool) -> Self {
        self.lenient_all = all;
        self
    }

    /// Makes one enumerated field lenient, by its dictionary name (e.g. `OrdType`); see
    /// [`lenient_enums`](Self::lenient_enums). Generating fails if it isn't an enumerated field of
    /// the generated messages.
    #[must_use]
    pub fn lenient_enum(mut self, field: &str) -> Self {
        self.lenient.push(field.to_string());
        self
    }

    /// Leaves out a message, by name.
    #[must_use]
    pub fn skip_message(mut self, name: &str) -> Self {
        self.skip.push(name.to_string());
        self
    }

    /// The code as separate modules, as [`write_modules`](Self::write_modules) writes them.
    /// Fails if the dictionary can't be generated as Rust: a group that doesn't start with a
    /// field, names that clash, or an official name that can't be an identifier.
    pub fn modules(&self) -> Result<Modules, Error> {
        let options =
            plan::Options { skip: &self.skip, docs: self.docs, lenient_all: self.lenient_all, lenient: &self.lenient };
        let plan = plan::build(self.dict, &options)?;
        Ok(Modules {
            root: render::root(self.dict),
            tags: render::tags(&plan),
            enums: render::enums(&plan),
            groups: render::groups(&plan),
            messages: render::messages(&plan),
        })
    }

    /// Everything in one file, for `include!`. The re-exports allow `unused_imports`, so the code
    /// can be included into a private module (in a binary, say) without warnings for them.
    pub fn render(&self) -> Result<String, Error> {
        let m = self.modules()?;
        Ok(format!(
            "pub mod tags {{\n{}}}\n\npub mod enums {{\n{}}}\n\npub mod groups {{\n{}}}\n\npub mod messages {{\n{}}}\n\n\
             #[allow(unused_imports)]\npub use enums::*;\n#[allow(unused_imports)]\npub use groups::*;\n\
             #[allow(unused_imports)]\npub use messages::*;\n",
            m.tags, m.enums, m.groups, m.messages
        ))
    }

    /// Writes `mod.rs`, `tags.rs`, `enums.rs`, `groups.rs` and `messages.rs` into `dir`, and
    /// returns their paths.
    pub fn write_modules(&self, dir: &Path) -> Result<Vec<PathBuf>, Error> {
        let m = self.modules()?;
        std::fs::create_dir_all(dir).map_err(io_error(dir))?;
        let files = [
            ("mod.rs", m.root),
            ("tags.rs", m.tags),
            ("enums.rs", m.enums),
            ("groups.rs", m.groups),
            ("messages.rs", m.messages),
        ];
        let mut paths = Vec::new();
        for (name, text) in files {
            let path = dir.join(name);
            std::fs::write(&path, text).map_err(io_error(&path))?;
            paths.push(path);
        }
        Ok(paths)
    }
}
