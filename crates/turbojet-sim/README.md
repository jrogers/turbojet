# turbojet-sim

Deterministic simulation testing for Turbojet (not published). An initiator and an acceptor, each a
`turbojet::Session` fed by a driver that mirrors `connection::run`, run against each other in one
thread on simulated time. Every choice (timings, heartbeat interval, reconnect interval, the
workload) comes from a seed, so a run replays exactly from its seed.

After every event a checker holds the two sides to these rules:

1. Everything a side writes is whole, valid FIX messages.
2. New messages take consecutive MsgSeqNums; a resend (PossDupFlag) only numbers already sent.
3. A resent application message is the original, with OrigSendingTime its first SendingTime, and
   a gap fill never covers an application message.
4. Each application receives what the other side sent, in order, and once (a repeat only when
   marked `maybe_redelivered`).

When the workload stops and the network is idle, both sides must be logged on, every application
message sent with a MsgSeqNum delivered, and each side's next outgoing number the other's next
expected.

So far the network is perfect and stores are in memory; faults, crashes and torn disk writes come
next (see ROADMAP.md, "Deterministic simulation testing").

## Running

```sh
cargo test -p turbojet-sim          # the fixed seeds, as on every push
scripts/sim.sh 600                  # random seeds for ten minutes, stopping at a failure
scripts/sim.sh 0 1234 -v            # replay seed 1234, printing every event
```

A failure prints its seed and the command that replays it. A seed that found a bug goes in
`regressions.txt`, with a comment, so it runs on every push from then on.
