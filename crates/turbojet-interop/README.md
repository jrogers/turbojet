# turbojet-interop

Tests Turbojet's session layer against another FIX engine,
[QuickFIX/J](https://github.com/quickfix-j/quickfixj) 3.0.2. Each scenario runs in eight cells:
Turbojet as initiator and as acceptor, on FIX 4.2, 4.3 and 4.4 and on FIXT.1.1 with FIX 5.0 SP2.
There are 33 scenarios (264 tests, plus checks on the harness itself):

- logon; Logout from either side; a dropped connection that resumes its sequence numbers, and
  one that starts again at 1 because the initiator logs on with ResetSeqNumFlag
  (`logon_logout.rs`);
- Heartbeats both ways at HeartBtInt=1, and answering a TestRequest (`heartbeat.rs`);
- a NewOrderSingle each way, field for field, and one with XmlData containing SOH (`app.rs`);
- gap fills and resends in each direction (`gap_recovery.rs`);
- a SequenceReset from either side (reset mode from QuickFIX/J; gap-fill mode from Turbojet's
  operator skipping ahead), and MsgSeqNum too low at either side (`seq_reset.rs`);
- through a fault-injecting proxy (`faults.rs`): an order lost or garbled in either direction,
  recovered by resend; a counterparty that goes silent, either side, so the other probes it with
  a TestRequest, gives up and reconnects; and a connection cut halfway through an order;
- through the proxy, slowed down (`delays.rs`): a latency spike either way that the receiver
  probes and survives; an order delayed past a short MaxLatency, rejected (SessionRejectReason 10)
  and followed by a Logout; a link capped at 20 KB/s carrying hundreds of orders each way; and a
  side whose reads stall until the other's writes block, after which every order both ways
  arrives once, in order. (What arrives *during* a stall is left unchecked: on Linux, TCP may hold
  the open direction back until it ends; see `tests/delays.rs`.)

The QuickFIX/J side is a small Java program (`peer/`) that runs one session and is driven over
stdin and stdout. The crate isn't published.

## Running

```sh
scripts/interop.sh                      # all of them
scripts/interop.sh gap_fill_from_peer   # arguments go to cargo test
```

This needs a JDK, 21 or later: the script builds the peer jar with the Gradle wrapper in `peer/`, then
runs `cargo test -p turbojet-interop` with `TURBOJET_INTEROP=1`. Without that variable the tests pass
without doing anything, so `cargo test --workspace` doesn't need Java. `INTEROP_PEER_JAR` points
the tests at a jar built elsewhere.

## Adding a scenario

A scenario is an `async fn` taking a `Setup`; `matrix!` turns each one into its eight tests.

```rust,ignore
matrix!(order_round_trip);

async fn order_round_trip(setup: Setup) {
    let mut pair = setup.start().await;     // or start_with(Options { heartbeat_secs: 1, .. })
    pair.logged_on().await;
    pair.handle.send(tj_order("ORD1")).unwrap();
    let order = pair.peer.received("D").await;
    assert_eq!(order.get(11), Some("ORD1"), "{}", order.raw());
    pair.finish().await;
}
```

- `pair.handle` is Turbojet's `SessionHandle`; `pair.peer.send` and `pair.peer.cmd` drive
  QuickFIX/J. `orders::tj_order` and `orders::peer_order` build an order valid in every version.
- `pair.peer.expect` waits for a peer event and `pair.tj_received` for a message Turbojet
  delivered; events that don't match stay buffered for later expects.
- `received`/`sent` (the `from_*`/`to_*` events) are QuickFIX/J's re-serialization of messages it
  accepted or sent: header fields reordered, and nothing it refused. To check what was actually on
  the wire, or a message QuickFIX/J ignored, use `wire_in`/`wire_out`.
- To assert that something didn't happen, first exchange a TestRequest so both sides have
  processed everything sent before (`barrier` in `gap_recovery.rs`), then call
  `expect_none`/`tj_expect_none` with `Duration::ZERO`.
- Every scenario ends with `pair.finish().await`, which fails on any reject, BusinessMessageReject
  or QuickFIX/J error the scenario didn't consume. To be sure it has seen them all, it first
  exchanges a TestRequest with QuickFIX/J or, if the session has ended logged out, stops
  QuickFIX/J and reads its output to the end. Dropping a `Pair` without it fails the test. An
  error QuickFIX/J may or may not log, depending on timing, can be let through with
  `pair.peer.tolerate_errors`; one it always logs should be consumed with `expect`.

The peer's arguments, commands and events are documented on the `Peer` class in
`peer/src/main/java/dev/turbojet/interop/Peer.java`.

## When a test fails

The test prints the last 300 lines from the peer (its events, including the bytes on the wire and
QuickFIX/J's errors), the peer's stderr (QuickFIX/J's own log), QuickFIX/J's session event log,
and the Turbojet events the scenario didn't consume. Turbojet's tracing appears in the test output
at `turbojet=debug`, which `RUST_LOG` overrides. QuickFIX/J's log directory is kept, and its path
printed. Each expect times out after 10 s and each scenario after 120 s.

## Findings

- Turbojet's Heartbeats could go out up to a second late, since its timer ran on a fixed
  once-a-second tick, and at HeartBtInt=1 QuickFIX/J then sent it a TestRequest. The timer now
  runs from the session's deadlines, so a Heartbeat goes out when it falls due.
- On a gap, Turbojet used to drop the message that revealed it and deliver the counterparty's
  PossDup resend of it, where QuickFIX/J queues it and ignores the resend. Both are within the
  spec; Turbojet now queues too, and the gap recovery scenarios check that both engines deliver
  the original.

## The fault-injecting proxy

With `Options { proxy: true, .. }`, the initiator connects through a proxy (`src/proxy.rs`) that
forwards whole FIX frames and, per direction (`Dir::ToPeer` or `Dir::ToTj`), can:

- `drop_next` or `garble_next` (a wrong CheckSum, same length) the next frame of a MsgType;
- `cut_mid` it: forward half, then close both sides;
- `blackhole` the direction, discarding everything, as a dead link would. While a direction is
  blackholed a close isn't passed on, so each side has to notice on its own heartbeat timeout;
- `hold` the direction (keep reading, forward nothing) until `release`, or `delay_next` one frame
  with what follows queued behind it;
- cap its `bandwidth` in bytes a second;
- `stall` it: stop reading from the sender, so TCP pushes back, until `unstall`.

Each new connection starts without faults. `pair.proxy()` sets them, and `applied`,
`disconnected` and the other waits say when they've happened.

## Not covered

- QuickFIX/n, and TLS.
