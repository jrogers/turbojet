# TLA+ specifications

Models of `turbojet-replica`'s protocol, checked with TLC. They describe the design at the level
of numbers and messages, not the code: what an active, a standby, a lease and a counterparty can
do in any order, crashes and lost connections included, and what must never happen.

```sh
specs/check.sh Replication                  # the design as built
specs/check.sh InFlight
specs/check.sh Replication Checks=FALSE     # a constant overridden: here, a flaw put back
INVARIANTS=NoSeqReuse2 specs/check.sh Replication Checks=FALSE   # one invariant
```

`check.sh` needs Java 11 or later; it downloads TLC (`tla2tools.jar`, 1.8.0) into `target/` on
first use and checks its SHA-256. Each `.cfg` is the design as built, at small bounds.

## `Replication.tla`

Two nodes, one active and one standby, and one FIX session's outgoing sequence numbers and the
messages stored under them. The active commits locally and sends each commit to the standby
(synchronously, or with a lag); the standby applies it, can crash, restart over its store or lose
it, and is promoted when the lease is taken. The lease expires on clocks that may be skewed, and
fences writes by holder and epoch. The counterparty is logged on to one node at a time. A second
session, committed before the run and then closed, stands for every session the active isn't
using.

Invariants:

- `NoSeqReuse`, `NoSeqReuse2`: the counterparty never gets a MsgSeqNum it has had, on either
  session: the harm a failover to a copy that lacks something does.
- `ResendsMatch`: the lease holder, asked to resend, sends what the counterparty had or a gap
  fill.
- `PromotableCovers`, `PromotableCovers2`: a standby that could be promoted (checked, and the
  lease saying in sync) holds everything the counterparty has had, less the lag.

## `InFlight.tla`

The incoming side: before handing messages to the application, a session commits the in-flight
marker; once they're handled, a commit moves the next incoming number past them. After a crash
the marker flags the resends from it `maybe_redelivered`. Each commit goes to the standby,
synchronously or with a lag.

Invariants: `NoUnflaggedRepeat` (a message handed over again is flagged) and `NoLoss` (no copy
counts a message received that the application wasn't handed).

## What the models found

Each flaw below was fixed; the settings put it back, and TLC finds it again.

| Flaw | Put back with | Fails |
|---|---|---|
| A session sent whole was applied batch by batch: its first batch's reset, then a lost link, left the standby below what was sent | `Replication HoldWhole=FALSE Chunked=TRUE LosesStore=FALSE` | `PromotableCovers` |
| A standby crashing between a reset and its commit left its copy reset (closed by the check below) | `INVARIANTS=NoSeqReuse Replication StandbyCrashes=TRUE LosesStore=FALSE Checks=FALSE MaxConn=4` | `NoSeqReuse` |
| `Lease::in_sync` read before acquiring the lease, while the active could still mark the standby out of sync | `Replication CheckFirst=TRUE Degrade=TRUE LosesStore=FALSE` | `NoSeqReuse2` |
| A standby that lost its store (a restart in memory, a disk replaced) could be promoted without it | `INVARIANTS=NoSeqReuse2 Replication Checks=FALSE` | `NoSeqReuse2` |
| A failover that promotes without the check (a planted bug) | `INVARIANTS=NoSeqReuse2 Replication PlantUnchecked=TRUE` | `NoSeqReuse2` |
| After an asynchronous failover, messages the old active had handed over came again unflagged | `InFlight OnPromote='"none"'` | `NoUnflaggedRepeat` |
| A marker set at promotion flags one window, less than the lag can be | `InFlight OnPromote='"marker"'` | `NoUnflaggedRepeat` |
| The bound taken from the new active's own next number, not the counterparty's Logon | `InFlight Resolve='"next"'` | `NoUnflaggedRepeat` |

## Results

As built, every invariant holds (checked exhaustively at the bounds in each `.cfg`):

| Spec | Bounds | Distinct states |
|---|---|---|
| `Replication` | 2 sequence numbers on each session, 2 epochs, 4 standby incarnations, 2 connections, 5 batches | 24,780,682 |
| `InFlight` | 3 messages, 2 failovers, up to 4 commits unacknowledged | 176,289 |

Synchronously, `InFlight` holds without the promotion's mark (`Async=FALSE OnPromote='"none"'`,
33,509 states): the in-flight marker reaches the standby before the hand-over.

Other checks run while finding the flaws, at their own bounds: asynchronous replication with a
lag of 1, degrading, both together, a manual lease, a skewed clock, and standby crashes with the
check (308 million distinct states) all passed.

## Not modelled

- More than two nodes: one active and one standby. With more, a standby that restarts forgets
  the epochs it has seen, which two can't exploit.
- Resets other than inside a session sent whole, the asynchronous digest's contents (only
  whether it matches), and the 64-batch cap.
- Liveness: every property is one that must never break, not one that must eventually happen
  (that a catch-up eventually checks the standby, say).
- The models are written from the code, by hand, and nothing checks the code against them.
