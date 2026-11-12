# Performance

This chapter is about the choices that move latency and throughput.

## Send from the callback

For the lowest latency, reply from `Application::on_message`. `Context::send` there is written
with the same wake-up of the connection task, while a `SessionHandle` send from another task
wakes the connection's task to take it: over localhost on `c9gd` (see [Results](#results)), an
order sent and its acknowledgement received through another task took 21.7 µs, against 8.7 µs
from `on_message`.

Turbojet runs on a current-thread tokio runtime as well as a multi-threaded one; on a
current-thread one, a `SessionHandle` send from a task on the same thread wakes no other thread,
which makes the hop cheaper.

## Spinning connections

To go lower at the cost of a core per connection, drive the session with
`Acceptor::accept_spinning` or `Initiator::run_spinning` (or `connection::run_spinning`) over a
`connection::SpinningStream`. They run on the calling thread and never wait: each time round they
read and write the non-blocking socket directly, and check the handle's commands and the timer,
rather than waiting for tokio's reactor to wake them. Store jobs and cancel on disconnect still
run on the tokio runtime passed in. Over localhost with both ends spinning, the round trip from
`on_message` took 7.4 µs against 8.7 µs on `c9gd`, and 8.7 µs against 9.6 µs on a Mac. Give each
spinning connection a thread, and on Linux a core, of its own. TLS isn't supported over it.

## Stores

A store that syncs every commit to its device costs what the device does: `DiskStorage` with
`fsync` takes about 4 ms a commit on a laptop's SSD. The session commits once per batch of work,
so under load many messages share a sync; one at a time, each waits for its own. Without `fsync`,
a commit survives a process crash but not an OS crash (see [Storage and recovery][storage]).

## Typed parsing

A generated message declares every field the dictionary gives it, mostly empty `Option`s, and its
size is about half the cost of parsing it: a FIX 4.2 NewOrderSingle declares 68 fields, and one
declaring only the 10 an order carries parses in about 60% of the time borrowed and half owned.
To generate messages with only the fields your counterparty sends, re-list them in a QuickFIX-format
file given to `turbojet-codegen --merge` (see the `turbojet-codegen` README). A field that arrives
but isn't declared is ignored by `parse`, and rejected by `parse_strict`.

The borrowed form (`NewOrderSingleRef`; see [Messages][messages]) converts only the fields read,
so its cost follows what the application reads more than what the message declares.

## Allocations

Decoding into the message a connection reuses, the session, and an application that parses its
orders borrowed (`NewOrderSingleRef`) and replies with owned messages whose text fields fit in a
`CompactString` allocate nothing per order once warmed up. A test in the repository counts them
by stage, and fails if any count changes.

## Interrupt moderation

On Linux, a network card waits a few microseconds after a packet before it interrupts, to take
more packets per interrupt. Under light load that wait is added to each message. AWS's ENA
waits adaptively, up to 20 µs on receive and 64 µs on send, and between two `c9gd` machines it
took an order's round trip from 37 µs to 220 µs at 10,000 orders a second, and to 345 µs at
100,000. Turning it off (`ethtool -C <dev> adaptive-rx off rx-usecs 0 tx-usecs 0`) costs CPU
time in interrupts: measure it with your load.

## Results

Measure with your own load: these are for comparing machines and configurations. The benchmarks
are in the repository: `cargo bench -p turbojet --all-features` runs each in one process, over
localhost, and `examples/ack.rs` and `examples/load.rs` are an acceptor and a load generator for
two machines.

### On Linux servers

Two AWS machines of 8 vCPUs with local NVMe instance storage, in one cluster placement group in
us-east-1, Linux 7.0, Rust 1.99.1, at commit `b2ddc50`, November 2026, the NICs' interrupt
moderation off (see above), `c9gd` built with `-C target-feature=+lse,+crc`, with
`scripts/aws-bench.sh`. One run of each.

The `ack` acceptor on one machine, answering each NewOrderSingle with an ExecutionReport, and
`load` on the other, sending orders at a set rate and timing each from when it was due, 10 s
after 2 s of warm-up:

| sessions × orders a second | `c9gd` p50 | p99 | p99.99 | `c8id` p50 | p99 | p99.99 |
|---|---:|---:|---:|---:|---:|---:|
| 1 × 1,000 | 36 µs | 40 µs | 63 µs | 126 µs | 350 µs | 600 µs |
| 1 × 10,000 | 35 µs | 38 µs | 63 µs | 144 µs | 585 µs | 904 µs |
| 1 × 100,000 | 36 µs | 44 µs | 101 µs | 102 µs | 157 µs | 492 µs |
| 8 × 200,000 | 37 µs | 55 µs | 159 µs | 107 µs | 316 µs | 3.56 ms |

On `c9gd` the p50 was 33 to 37 µs at every rate in all three runs, each on a placement of its
own. `c8id`'s round trips are about three times as long, with a longer tail.

On AWS c9gd.2xlarge (Graviton5), Linux 7.0: the benchmarks in one process, criterion medians,
temporary files on the instance storage.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 210 ns / 186 ns | 767 / 868 MiB/s |
| Encode ExecutionReport (209 B) | 61 ns | 3.2 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 36 ns / 57 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.21 µs | 830k msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 33 ns / 592 ns / 51 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.7 µs | 115k/s |
| The same, both ends spinning (`run_spinning`) | 7.4 µs | 135k/s |
| Round trip over localhost TCP, 1,000 in flight | | 498k msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 474k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 473k / 2.53M / 2.55M msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 6.3 µs | 1.60M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 7.9 ns / 268 ns | |

The same on c8id.2xlarge (Xeon 6975P, Granite Rapids):

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 286 ns / 266 ns | 564 / 605 MiB/s |
| Encode ExecutionReport (209 B) | 56 ns | 3.5 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 42 ns / 65 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.43 µs | 701k msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 26 ns / 505 ns / 133 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.3 µs | 121k/s |
| The same, both ends spinning (`run_spinning`) | 9.0 µs | 111k/s |
| Round trip over localhost TCP, 1,000 in flight | | 284k msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 271k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 380k / 1.42M / 1.44M msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 5.8 µs | 1.27M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 5.5 ns / 360 ns | |

A synced commit costs what the device does: 51 µs on `c9gd`'s instance storage and 133 µs on
`c8id`'s, against about 4 ms on a laptop's SSD.

### On a Mac

Apple M4 Pro (8 performance and 4 efficiency cores), macOS 27, Rust 1.98.1, criterion medians,
October 2026. Over localhost only.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 163 ns / 136 ns | 991 / 1,185 MiB/s |
| Encode ExecutionReport (209 B) | 45 ns | 4.3 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 26 ns / 41 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 825 ns | 1.21M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 12 ns / 1.4 µs / 3.9 ms | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 9.6 µs | 104k/s |
| The same, both ends spinning (`run_spinning`) | 8.7 µs | 115k/s |
| Round trip over localhost TCP, 1,000 in flight | | 1.06M msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 950k msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 16.1 µs | 3.97M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 4.8 ns / 181 ns | |

[messages]: messages.md
[storage]: storage.md
