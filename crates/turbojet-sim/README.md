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
5. Nothing stored is lost: a delivery never passes a message the sender stored.
6. A store reopens with exactly the numbers it last recorded, crash or not; a change a power loss
   tore shows up as either made or not.
7. A send's receipt says truly whether the store recorded it, and as which MsgSeqNum.
8. On seeds with cancel on disconnect (either side, either trigger, a grace period of up to 30 s),
   an application is told to cancel exactly when a logged-on session ended in a way the trigger
   counts and didn't log on again within the grace period, at its end (with no grace period, in
   the same call that ended it). A session ends once per logon, and every countdown runs out
   before a run ends; a crash takes the countdown under way with the process's registry.

When the workload and faults stop, the sessions have until the slowest recovery could take to
settle: one connection, both sides logged on over it, every application message stored delivered,
and each side's next outgoing number the other's next expected.

Half the seeds keep state in memory (standing for files that survive a process crash), the rest in
`DiskStorage`. Either node's process can crash, between events or inside a store call (which either
takes effect or not); a store call can also just fail. A crash between the application handling a
message and the session recording it is redelivered, marked `maybe_redelivered`. Disk seeds lose
power too: with sync, the call in progress is torn at a byte (on some seeds within a sector, mixing
old and new bytes); without it, the files go back to what the OS had written back, a loss
`DiskStorage` documents, after which only rule 1, "the store reopens" and rule 8 are checked. On
half the seeds the stores are slow, as a networked one is: they hand each commit, resend read and
opening of a session's log to the driver as a job, which finishes up to 5 ms later (now and then
50 ms), and the session waits for it.

Applications ask to log out now and then; operators move a side's next outgoing number ahead
(connected or not) and reset both sides (logging them out first); and a quarter of the seeds run
to a daily schedule, whose next period starts the next morning with a sequence reset. A store
reset starts a new epoch: messages left undelivered in the old one were dropped by the operator.

On a fifth of the seeds a hostile middlebox sits between each connection and its receiver, and
during the busy phase drops, duplicates, swaps or corrupts whole messages, as a buggy
counterparty or proxy might. The sessions recover through resends, rejects and logouts, so every
rule holds there too.

## Checking the checker

A checker that passes everything proves nothing. `Options::plant` plants a bug in the simulator
standing for one in the engine: the application missing a delivery or seeing one twice, a store
that keeps a message's number but not the message, a resend that arrives altered, a cancel on
disconnect missed, run late or made up. A test per
plant runs seeds until the checker catches it, by the rule expected.

## Known failures

`known_failures.txt` lists per-push seeds expected to fail, with the rule they break and why. A
listed seed that passes, or fails some other way, fails the test, so a fix shows up as seeds to
take off the list. None are listed now. The first, a **write deadlock** (both drivers waiting to
write to each other, neither reading), was fixed by making the connection driver read while its
output waits.

## Running

Every push runs 100 fixed seeds; a nightly workflow (`.github/workflows/simulate.yml`) runs random
ones for half an hour and uploads the trace of any that fails.

```sh
cargo test -p turbojet-sim          # the fixed seeds, as on every push
scripts/sim.sh 600                  # random seeds for ten minutes, stopping at a failure
scripts/sim.sh 0 1234 -v            # replay seed 1234, printing every event
```

A failure prints its seed and the command that replays it. A seed that found a bug goes in
`regressions.txt`, with a comment, so it runs on every push from then on.
