# Performance

This chapter is about the choices that move latency and throughput.

## Send from the callback

For the lowest latency, reply from `Application::on_message`. `Context::send` there is written
with the same wake-up of the connection task, while a `SessionHandle` send from another task
wakes the connection's task to take it: over localhost on `c9gd` (see [Results](#results)), an
order sent and its acknowledgement received through another task took 20.7 µs, against 8.7 µs
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
`on_message` took 7.7 µs against 8.7 µs on `c9gd`, and 8.7 µs against 9.6 µs on a Mac. Between
two `c9gd` machines it took an order's round trip from 34 to 36 µs down to 23 to 25 µs, within 1 to
3 µs of the kernel's own TCP round trip between them. Give each spinning connection a thread, and on
Linux a core, of its own (see [pinning](#pinning-on-linux)). TLS isn't supported over it.

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

## The allocator on Linux

On Linux, run Turbojet on jemalloc or mimalloc rather than glibc's malloc. A message sent through a
`SessionHandle` is built on the sending thread and freed on the connection's, and glibc's malloc
takes a lock for each message freed on another thread than the one that allocated it. On the same
`c9gd` machine, 1,000 orders in flight over localhost went from 461,000 a second on glibc to
727,000 on jemalloc, over TLS from 408,000 to 681,000, and 32 sessions from 2.41 million to 2.84
million. A reply from `on_message` allocates nothing, so it gains nothing. Either preload the
allocator, as the results below do:

```sh
sudo apt-get install libjemalloc2   # Debian and Ubuntu
LD_PRELOAD=/usr/lib/$(uname -m)-linux-gnu/libjemalloc.so.2 ./your-gateway
```

or make it the program's global allocator, with the `tikv-jemallocator` or `mimalloc` crates. On
macOS the system allocator doesn't have this cost.

## Interrupt moderation

On Linux, a network card waits a few microseconds after a packet before it interrupts, to take
more packets per interrupt. Under light load that wait is added to each message. AWS's ENA
waits adaptively, up to 20 µs on receive and 64 µs on send, and between two `c9gd` machines it
took an order's round trip from 37 µs to 220 µs at 10,000 orders a second, and to 345 µs at
100,000. Turning it off (`ethtool -C <dev> adaptive-rx off rx-usecs 0 tx-usecs 0`) costs CPU
time in interrupts: measure it with your load.

## Pinning on Linux

Where the scheduler puts threads, and where the network card's interrupts land, sets the tail of
the round trip. Between two `c9gd` machines with interrupt moderation off, pinning took the p99
from 56 to 76 µs down to 31 to 35 µs with spinning connections, and from 66 to 91 µs down to 42 to
47 µs with tokio's driver; the p50 stayed where it was. The layout measured:

- the card's interrupts on CPU 0, with `irqbalance` stopped so that they stay there (write `0` to
  each `/proc/irq/*/smp_affinity_list`);
- each spinning connection's thread, and any other thread that spins, on a CPU of its own
  (`taskset -pc <cpu> <tid>`: name the threads, as `std::thread::Builder::name` does, to find
  them in `/proc/<pid>/task/*/comm`);
- everything else, tokio's workers among it, on the remaining CPUs, away from both.

Taking the spinning threads' CPUs from the scheduler and the timer tick as well (`isolcpus`,
`nohz_full` and `rcu_nocbs` on the kernel command line) made no difference that showed. Nor did
the kernel's busy polling (`net.core.busy_poll` and `net.core.busy_read`) for spinning
connections, which already poll: it took 1 to 2 µs off at best. It helped tokio's driver only at
high rates: at 100,000 orders a second, the p50 went from 36 to 29 µs.

## Results

Measure with your own load: these are for comparing machines and configurations. The benchmarks
are in the repository: `cargo bench -p turbojet --all-features` runs each in one process, over
localhost, and `examples/ack.rs` and `examples/load.rs` are an acceptor and a load generator for
two machines.

### On Linux servers

Two AWS machines of 8 vCPUs with local NVMe instance storage, in one cluster placement group in
us-east-1, Linux 7.0, Rust 1.99.1, November 2026, on jemalloc (see [the allocator](#the-allocator-on-linux)),
the NICs' interrupt moderation off (see [interrupt moderation](#interrupt-moderation)), `c9gd`
built with `-C target-feature=+lse,+crc`, with `scripts/aws-bench.sh`. One run of each: the round
trips at commit `a598624`, the benchmarks in one process at `a16adab`.

The `ack` acceptor on one machine, answering each NewOrderSingle with an ExecutionReport, and
`load` on the other, sending orders at a set rate and timing each from when it was due, 10 s
after 2 s of warm-up:

| sessions × orders a second | `c9gd` p50 | p99 | p99.99 | `c8id` p50 | p99 | p99.99 |
|---|---:|---:|---:|---:|---:|---:|
| 1 × 1,000 | 35 µs | 40 µs | 57 µs | 249 µs | 438 µs | 598 µs |
| 1 × 10,000 | 33 µs | 38 µs | 88 µs | 124 µs | 646 µs | 1.00 ms |
| 1 × 100,000 | 36 µs | 43 µs | 114 µs | 102 µs | 215 µs | 604 µs |
| 8 × 200,000 | 35 µs | 50 µs | 77 µs | 105 µs | 625 µs | 4.62 ms |

On `c9gd` the p50 held flat across the rates in each of six runs, each on a placement of its own,
but the placement set its level: 32 to 37 µs in four, 39 to 42 µs and 45 to 50 µs in the others.
`c8id`'s round trips are about three times as long, with a longer tail, on glibc as on jemalloc.

The same between two `c9gd` machines at commit `93eec91`, one session, pinned as in
[pinning](#pinning-on-linux), with tokio's driver and with spinning connections (`ack --spin`,
`load --spin`). The first row is the kernel's own TCP round trip between them, without Turbojet:
`sockperf ping-pong --tcp -m 200`, one 200-byte message at a time.

| one session, pinned | p50 | p99 | p99.99 |
|---|---:|---:|---:|
| sockperf, TCP ping-pong | 22.0 µs | 31 µs | 56 µs |
| tokio's driver, 1,000 orders a second | 36 µs | 43 µs | 61 µs |
| tokio's driver, 10,000 | 34 µs | 43 µs | 74 µs |
| tokio's driver, 100,000 | 36 µs | 47 µs | 99 µs |
| spinning, 1,000 orders a second | 25 µs | 33 µs | 51 µs |
| spinning, 10,000 | 23 µs | 33 µs | 81 µs |
| spinning, 100,000 | 25 µs | 35 µs | 71 µs |

On AWS c9gd.2xlarge (Graviton5), Linux 7.0, jemalloc: the benchmarks in one process, criterion
medians, temporary files on the instance storage.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 209 ns / 189 ns | 773 / 853 MiB/s |
| Encode ExecutionReport (209 B) | 61 ns | 3.2 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 36 ns / 57 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.26 µs | 796k msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 26 ns / 603 ns / 51 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.7 µs | 114k/s |
| The same, both ends spinning (`run_spinning`) | 7.7 µs | 130k/s |
| Round trip over localhost TCP, 1,000 in flight | | 710k msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 602k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 521k / 2.71M / 2.84M msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 6.3 µs | 2.04M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 8.0 ns / 268 ns | |

The same on c8id.2xlarge (Xeon 6975P, Granite Rapids):

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 284 ns / 267 ns | 567 / 604 MiB/s |
| Encode ExecutionReport (209 B) | 57 ns | 3.4 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 42 ns / 65 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.41 µs | 710k msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 24 ns / 505 ns / 138 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.3 µs | 121k/s |
| The same, both ends spinning (`run_spinning`) | 9.0 µs | 111k/s |
| Round trip over localhost TCP, 1,000 in flight | | 542k msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 528k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 396k / 1.49M / 1.52M msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 5.9 µs | 1.56M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 5.5 ns / 367 ns | |

A synced commit costs what the device does: 51 µs on `c9gd`'s instance storage and 138 µs on
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
