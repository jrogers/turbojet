# Performance

This chapter is about the choices that move latency and throughput.

## Send from the callback

For the lowest latency, reply from `Application::on_message`. `Context::send` there is written
with the same wake-up of the connection task, while a `SessionHandle` send from another task
wakes the connection's task to take it: over localhost on `c9gd` (see [Results](#results)), an
order sent and its acknowledgement received through another task took 19.5 µs, against 8.8 µs
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
`on_message` took 7.5 µs against 8.8 µs on `c9gd`, and 8.7 µs against 9.6 µs on a Mac. Between
two `c9gd` machines it took an order's round trip from 34 to 36 µs down to 23 to 25 µs, against 18 µs
for the kernel's own TCP round trip between them read the same way, polling non-blocking sockets.
Give each spinning connection a thread, and on
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
high rates: at 100,000 orders a second, the p50 went from 36 to 29 µs. Keeping the card's
interrupt off while a thread busy-polls (`SO_PREFER_BUSY_POLL`, which needs `CAP_NET_ADMIN`, with
the device's `napi_defer_hard_irqs` and `gro_flush_timeout`) gained nothing that showed over busy
polling alone, and those device settings without busy polling took the p50 to over 230 µs.

## Receive timestamps on Linux

A spinning connection can measure how long each read's bytes were in the host before it read
them: in the network card, the kernel and the socket's queue. Ask for receive timestamps on its
stream, and turn the session's latency metrics on (feature `metrics`); each read with a stamp is
then a sample of `turbojet_wire_to_read_seconds`:

```rust,no_run
# #[cfg(target_os = "linux")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use std::sync::Arc;
# use turbojet::*;
# use turbojet::connection::SpinningStream;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let runtime = tokio::runtime::Runtime::new()?;
let mut config = SessionConfig::new("FIX.4.4", "GATEWAY");
config.latency_metrics = true;
let acceptor = Acceptor::new(config, Arc::new(MemoryStorage::new()), Arc::new(App))?;
let listener = std::net::TcpListener::bind("0.0.0.0:9876")?;
let socket = SpinningStream::new(listener.accept()?.0)?.with_receive_timestamps()?;
acceptor.accept_spinning(socket, ConnectionInfo::default(), runtime.handle())?;
# Ok(())
# }
# #[cfg(not(target_os = "linux"))]
# fn main() {}
```

Each packet is stamped by the network card if it timestamps packets, and otherwise by the kernel
on receiving it, which leaves out the time from the card to the kernel. The card's stamps take
three things:

- a card that stamps received packets (`ethtool -T <dev>` lists `hardware-receive`): on AWS, an
  instance type with the Amazon Time Sync PTP hardware clock, launched in a `precision-time`
  placement group, with AWS's own ENA driver (amzn-drivers, built with `ENA_PHC_INCLUDE=1` and
  loaded with `phc_enable=1`); the kernel's own ENA driver gives the clock but not packet stamps;
- stamping turned on for the interface, which takes `CAP_NET_ADMIN` (`hwstamp_ctl -i <dev> -r 1`);
- the system clock synced to the card's (chrony's `refclock PHC`), since a stamp is compared
  with the wall clock at the read. A read whose stamp is later than the wall clock isn't recorded.

A read takes what has arrived, and its sample is the latest packet's time, so a read that takes
several packets times the last of them.

Between two `c9gd` machines in a `precision-time` placement group, with interrupt moderation off
and chrony on the card's clock (within about 1 µs), a spinning acceptor taking 10,000 orders a
second read each order 1.2 µs after the kernel stamped it (p50; p99 4.8 µs) and 2.1 µs after the
card did (p99 4.8 µs): about 0.9 µs from the card to the kernel, of a 31 µs round trip. With the
card's stamps about 5% fewer reads were recorded, which a clock behind by a microsecond would
explain.

## Results

Measure with your own load: these are for comparing machines and configurations. The benchmarks
are in the repository: `cargo bench -p turbojet --all-features` runs each in one process, over
localhost, and `examples/ack.rs` and `examples/load.rs` are an acceptor and a load generator for
two machines.

### On Linux servers

Two AWS machines of 8 vCPUs with local NVMe instance storage, in one cluster placement group in
us-east-1, Linux 7.0, Rust 1.100.0, November 2026, on jemalloc (see [the allocator](#the-allocator-on-linux)),
the NICs' interrupt moderation off (see [interrupt moderation](#interrupt-moderation)), `c9gd`
built with `-C target-feature=+lse,+crc`, with `scripts/aws-bench.sh`, all at commit `df3e6eb`.

Between two `c9gd` machines, one session: the `ack` acceptor on one, answering each
NewOrderSingle with an ExecutionReport, and `load` on the other, sending orders at a set rate and
timing each from when it was due, 10 s after 2 s of warm-up. Both pinned as in
[pinning](#pinning-on-linux), with tokio's driver and with spinning connections (`ack --spin`,
`load --spin`). The first two rows are the kernel's own TCP round trip between the same two
machines, without Turbojet: `sockperf ping-pong --tcp -m 200`, one 200-byte message at a time,
read blocking as tokio's driver waits, and polling non-blocking sockets as spinning connections do.

| one session, pinned | p50 | p99 | p99.99 |
|---|---:|---:|---:|
| sockperf, TCP ping-pong, blocking reads | 21.9 µs | 28 µs | 45 µs |
| sockperf, TCP ping-pong, non-blocking sockets polled | 18.5 µs | 25 µs | 40 µs |
| tokio's driver, 1,000 orders a second | 36 µs | 43 µs | 60 µs |
| tokio's driver, 10,000 | 33 µs | 38 µs | 74 µs |
| tokio's driver, 100,000 | 36 µs | 45 µs | 116 µs |
| spinning, 1,000 orders a second | 24 µs | 29 µs | 46 µs |
| spinning, 10,000 | 23 µs | 29 µs | 70 µs |
| spinning, 100,000 | 25 µs | 35 µs | 71 µs |

So Turbojet adds about 14 µs to the kernel's round trip with tokio's driver, and about 5 µs
spinning. The placement sets the kernel's round trip, and with it every row: in another placement
sockperf's was 33 µs, tokio's driver 47 µs and spinning 35 µs, Turbojet's share the same.

Without pinning, at more sessions, and on `c8id` (Xeon 6975P) as well, tokio's driver:

| sessions × orders a second | `c9gd` p50 | p99 | p99.99 | `c8id` p50 | p99 | p99.99 |
|---|---:|---:|---:|---:|---:|---:|
| 1 × 1,000 | 36 µs | 42 µs | 74 µs | 101 µs | 328 µs | 688 µs |
| 1 × 10,000 | 32 µs | 40 µs | 82 µs | 123 µs | 564 µs | 962 µs |
| 1 × 100,000 | 36 µs | 48 µs | 161 µs | 99 µs | 358 µs | 736 µs |
| 8 × 200,000 | 36 µs | 54 µs | 92 µs | 105 µs | 517 µs | 2.83 ms |

On `c9gd` the p50 held flat across the rates in each of six earlier runs, each on a placement of
its own, but the placement set its level: 32 to 37 µs in four, 39 to 42 µs and 45 to 50 µs in the
others. `c8id`'s round trips are about three times as long, with a longer tail, on glibc as on
jemalloc.

With the kernel's busy polling as well, sockperf's round trip on non-blocking sockets was 16 µs
in another placement. Bypassing the kernel, a UDP ping-pong over DPDK between the same machines was 3
to 4 µs faster than the busy-polled kernel socket, 12.7 µs at best (`scripts/dpdk-pingpong.c`), and
4 to 4.5 µs faster on `c8id`, once the device memory DPDK writes through was mapped
write-combined; mapped uncached, as the stock `vfio-pci` does, it was slower than the kernel on
`c8id`.

On AWS c9gd.2xlarge (Graviton5), Linux 7.0, jemalloc: the benchmarks in one process, criterion
medians, temporary files on the instance storage.

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 208 ns / 187 ns | 777 / 860 MiB/s |
| Encode ExecutionReport (209 B) | 61 ns | 3.2 GiB/s |
| Typed parse NewOrderSingle, borrowed (no groups / with 3 allocations) | 36 ns / 57 ns | |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.17 µs | 858k msg/s |
| Session: an initiator taking in an ExecutionReport, wire to `on_message` | 457 ns | 2.19M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 26 ns / 602 ns / 49 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.8 µs | 114k/s |
| The same, both ends spinning (`run_spinning`) | 7.5 µs | 134k/s |
| Round trip over localhost TCP, 1,000 in flight | | 682k msg/s |
| Round trip over localhost TLS, 1,000 in flight | | 647k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 543k / 2.90M / 3.02M msg/s |
| FIXP round trip over localhost TCP: one at a time, replying from `on_message` / 1,000 in flight | 6.1 µs | 2.31M msg/s |
| SBE: decode a B3 NewOrderSingle and read the order fields / tag=value, typed borrowed | 8.0 ns / 265 ns | |

The same on c8id.2xlarge (Xeon 6975P, Granite Rapids):

| Benchmark | Time | Rate |
|---|---|---|
| Decode NewOrderSingle (169 B): into a new message / a reused one | 284 ns / 268 ns | 568 / 602 MiB/s |
| Encode ExecutionReport (209 B) | 59 ns | 3.3 GiB/s |
| Session: order → ack, wire to wire (decode + session, which encodes) | 1.35 µs | 739k msg/s |
| Session: an initiator taking in an ExecutionReport, wire to `on_message` | 594 ns | 1.68M msg/s |
| Store a sent message and commit it: memory / disk / disk + fsync | 23 ns / 508 ns / 160 µs | |
| Round trip over localhost TCP, one at a time, replying from `on_message` | 8.5 µs | 118k/s |
| The same, both ends spinning (`run_spinning`) | 9.0 µs | 111k/s |
| Round trip over localhost TCP, 1,000 in flight | | 523k msg/s |
| Many sessions over localhost TCP, 1,000 in flight each: 1 / 8 / 32 sessions | | 398k / 1.54M / 1.57M msg/s |

A synced commit costs what the device does: 49 µs on `c9gd`'s instance storage and 160 µs on
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
