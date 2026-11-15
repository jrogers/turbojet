//! The engine's integration tests in one test binary, rather than one per file, so turbojet and its
//! test dependencies link once. Tests that install something for the whole process (an allocator,
//! a metrics recorder) stay binaries of their own: allocations, fast_allocations, metrics,
//! fixp_metrics and wire_to_read. So do the tests in tests/repo, which the package leaves out.

mod cancel_on_disconnect;
mod counterparties;
mod custom_messages;
mod engine;
mod failover;
mod fast_fastlib;
mod fast_refresh;
mod logging;
mod operator;
mod readme;
mod replay;
mod schedule;
mod shutdown;
mod tls;
