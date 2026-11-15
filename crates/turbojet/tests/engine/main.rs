//! The engine's integration tests in one test binary, rather than one per file, so turbojet and its
//! test dependencies link once. Tests that depend on something for the whole process stay
//! binaries of their own: an allocator (allocations, fast_allocations), a metrics recorder
//! (metrics, fixp_metrics, wire_to_read), or captured logs (logging, tls), as tracing decides
//! once for the process whether each log statement is enabled, and other tests running alongside
//! with no subscriber can turn it off for the one capturing. So do the tests in tests/repo, which
//! the package leaves out.

mod cancel_on_disconnect;
mod counterparties;
mod custom_messages;
mod engine;
mod failover;
mod fast_fastlib;
mod fast_refresh;
mod operator;
mod readme;
mod replay;
mod schedule;
mod shutdown;
