#![doc = guide_links!()]
#![doc = include_str!("../guide/README.md")]

// Each chapter is a file in docs/guide/, read on GitHub (and the website) as well as here, with
// its examples run as doctests. The chapters link to each other with reference-style links whose
// definitions name the files; `guide_links!` defines the same names first, as module paths, and
// in Markdown the first definition of a name is the one used.

#[doc = guide_links!()]
#[doc = include_str!("../guide/first-session.md")]
pub mod first_session {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/messages.md")]
pub mod messages {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/sessions.md")]
pub mod sessions {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/acceptors.md")]
pub mod acceptors {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/initiators.md")]
pub mod initiators {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/tls.md")]
pub mod tls {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/storage.md")]
pub mod storage {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/schedules.md")]
pub mod schedules {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/operations.md")]
pub mod operations {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/performance.md")]
pub mod performance {}

#[doc = guide_links!()]
#[doc = include_str!("../guide/binary.md")]
pub mod binary {}
