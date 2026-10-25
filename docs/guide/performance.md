# Performance

The repository's README has the benchmark numbers; this chapter is about the choices that move
them.

## Send from the callback

For the lowest latency, reply from `Application::on_message`. `Context::send` there is written
with the same wake-up of the connection task, while a `SessionHandle` send from another task
wakes the connection's task to take it: over localhost, an order sent and its acknowledgement
received through another task took 31.2 µs, against 11.2 µs from `on_message`.

Turbojet runs on a current-thread tokio runtime as well as a multi-threaded one; on a
current-thread one, a `SessionHandle` send from a task on the same thread wakes no other thread,
which makes the hop cheaper.

## Spinning connections

To go lower at the cost of a core per connection, drive the session with
`Acceptor::accept_spinning` or `Initiator::run_spinning` (or `connection::run_spinning`) over a
`connection::SpinningStream`. They run on the calling thread and never wait: each time round they
read and write the non-blocking socket directly, and check the handle's commands and the timer,
rather than waiting for tokio's reactor to wake them. Store jobs and cancel on disconnect still
run on the tokio runtime passed in. Over localhost on macOS, with both ends spinning, the round
trip from `on_message` took 9.2 µs against 11.2 µs. Give each spinning connection a thread, and
on Linux a core, of its own. TLS isn't supported over it.

## Stores

A store that syncs every commit to its device costs what the device does: `DiskStorage` with
`fsync` takes about 4 ms a commit on a laptop's SSD. The session commits once per batch of work,
so under load many messages share a sync; one at a time, each waits for its own. Without `fsync`,
a commit survives a process crash but not an OS crash (see [Storage and recovery][storage]).

## Allocations

Decoding into the message a connection reuses, the session, and an application that parses its
orders borrowed (`NewOrderSingleRef`) and replies with owned messages whose text fields fit in a
`CompactString` allocate nothing per order once warmed up. A test in the repository counts them
by stage, and fails if any count changes.

[storage]: storage.md
