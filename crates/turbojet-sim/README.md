# turbojet-sim

Deterministic simulation testing for Turbojet (not published). An initiator and an acceptor, each a
`turbojet::Session` fed by a driver that mirrors `connection::run`, run against each other in one
thread on simulated time. Every choice (timings, heartbeat interval, reconnect interval, the
workload, the network's faults) comes from a seed, so a run replays exactly from its seed.

The network behaves as TCP does under trouble: writes arrive in pieces after varying delays (never
reordered), a direction can stall, send buffers fill (4 KiB to 1 MiB), holding the writer's output
until the reader reads, connections reset or fall into black holes (no one told, until TCP gives up),
and connects are refused or slow. The acceptor keeps a session per connection, so a half-open one
can linger while the initiator reconnects. On some seeds resends go out a few sequence numbers at a
time.

A wrapper on each store tells the checker every MsgSeqNum it records and the message stored with
it, whether or not it reached the wire. After every event the checker holds the two sides to these
rules:

1. Everything a side writes is whole, valid FIX messages.
2. Each store records consecutive MsgSeqNums; new messages go on the wire in order, each once and
   as stored; a resend (PossDupFlag) only numbers already stored.
3. A resent application message is the stored original, with OrigSendingTime its first
   SendingTime, and a gap fill never covers an application message.
4. Each application receives what the other side sent, in order, and once (a repeat only when
   marked `maybe_redelivered`).

When the workload and faults stop, the sessions have until the slowest recovery could take to
settle: one connection, both sides logged on over it, every application message stored delivered,
and each side's next outgoing number the other's next expected.

Stores are in memory so far; crashes, torn disk writes, operators and schedules come next (see
ROADMAP.md, "Deterministic simulation testing").

## Known failures

`known_failures.txt` lists per-push seeds expected to fail, with the rule they break and why. A
listed seed that passes, or fails some other way, fails the test, so a fix shows up as seeds to
take off the list. None are listed now. The first, a **write deadlock** (both drivers waiting to
write to each other, neither reading), was fixed by making the connection driver read while its
output waits.

## Running

```sh
cargo test -p turbojet-sim          # the fixed seeds, as on every push
scripts/sim.sh 600                  # random seeds for ten minutes, stopping at a failure
scripts/sim.sh 0 1234 -v            # replay seed 1234, printing every event
```

A failure prints its seed and the command that replays it. A seed that found a bug goes in
`regressions.txt`, with a comment, so it runs on every push from then on.
