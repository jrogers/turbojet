//! Session configuration files for [Turbojet](https://docs.rs/turbojet): an acceptor, its
//! counterparties and their stores, read from TOML and reloaded while running.

mod load;
mod raw;

use std::fmt;

pub use raw::Unknown;

/// Why a sessions file can't be used: where in it, and what's wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    /// A problem with `key` in `section` (`acceptor`, `defaults`, `counterparty BROKER`).
    pub(crate) fn at(section: &str, key: &str, problem: impl fmt::Display) -> Self {
        Self { message: format!("{section}: {key}: {problem}") }
    }

    /// A problem with the file as a whole.
    pub(crate) fn file(problem: impl fmt::Display) -> Self {
        Self { message: problem.to_string() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}
