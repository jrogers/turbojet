# turbojet-interop

Tests Turbojet's session layer against two other FIX engines,
[QuickFIX/J](https://github.com/quickfix-j/quickfixj) 3.0.2 and
[QuickFIX/n](https://github.com/connamara/quickfixn) 1.14.1. Each scenario runs in 16 cells:
against each engine (`qfj`, `qfn` in the test names), with Turbojet as initiator and as
acceptor, on FIX 4.2, 4.3 and 4.4 and on FIXT.1.1 with FIX 5.0 SP2. There are 42 scenarios (672
tests), plus checks on the harness itself:

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

Each engine's side is a small program that runs one session and is driven over stdin and stdout:
Java for QuickFIX/J (`peer/`), C# for QuickFIX/n (`peer-net/`).
Both speak the same protocol, so a scenario is written once. The crate isn't published.

## Running

```sh
scripts/interop.sh                      # all of them
scripts/interop.sh gap_fill_from_peer   # arguments go to cargo test
scripts/interop.sh qfn                  # one engine's cells
```

This needs a JDK, 21 or later, and the .NET 10 SDK: the script builds the peer jar with the
Gradle wrapper in `peer/` and the .NET peer in `peer-net/`, then runs
`cargo test -p turbojet-interop` with `TURBOJET_INTEROP=1`. Without that variable the tests pass
without doing anything, so `cargo test --workspace` doesn't need Java or .NET.
`INTEROP_PEER_JAR` and `INTEROP_NET_PEER` (the `peer-net.dll` to run with `dotnet`) point the
tests at peers built elsewhere. QuickFIX/n's data dictionaries aren't in its NuGet package, so they're in `peer-net/spec/`, from its v1.14.1 tag.

## Adding a scenario

A scenario is an `async fn` taking a `Setup`; `matrix!` turns each one into its sixteen tests.
`setup.engine` says which engine it's running against, for the few places they differ.

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
  the peer. `orders::tj_order` and `orders::peer_order` build an order valid in every version.
- `pair.peer.expect` waits for a peer event and `pair.tj_received` for a message Turbojet
  delivered; events that don't match stay buffered for later expects.
- `received`/`sent` (the `from_*`/`to_*` events) are the engine's re-serialization of messages it
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
`peer/src/main/java/dev/turbojet/interop/Peer.java`. The .NET peer takes the same arguments, and
emits `qfn_event` where the Java peer emits `qfj_error`, since QuickFIX/n's log mixes errors and
other events; `PeerEvent::logged` gives either engine's text. QuickFIX/n reads `FileLogPath` and `ReconnectInterval`
only from the default settings, and can't report the port an acceptor bound, so the peer takes a
free port first. Its sends write to the socket before returning, so a send into a stalled reader
blocks the peer until the reader reads again: start such a command with `pair.peer.start_cmd` and
wait for it with `cmd_done`.

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

Against QuickFIX/n, Turbojet needed no changes. It departs from the spec in two places,
both checked for:

- As acceptor, it answers a Logon with ResetSeqNumFlag=Y with a Logon that leaves the flag out,
  though it resets; the spec, and QuickFIX's own `SessionReset` acceptance scenario, have the
  reply carry it (`logon_with_reset`).
- As initiator, it disconnects on a garbled message (a wrong CheckSum) where the session layer
  should ignore it: its initiator's read loop doesn't catch the error its session throws, which its
  acceptor's does. It recovers the message by resend once logged on again
  (`garbled_order_to_peer`).

## TLS

`tests/tls.rs` runs without the proxy, which can't read encrypted frames: orders over TLS with the server's
certificate alone, and with mutual TLS through a reconnect; and handshakes that must fail, for an
untrusted server, a client without a certificate and a client with an untrusted one. With
`Options { tls: Some(Tls { .. }), .. }`, `src/pki.rs` makes a CA that issues both sides'
certificates (for `localhost` and `127.0.0.1`) and a second CA that issues none, for a side set to
trust the wrong one. The Java peer writes QuickFIX/J's PKCS#12 key and trust stores from those PEM
files (its key store empty when it presents no certificate, so QuickFIX/J doesn't fall back to its
bundled one), and as initiator checks the server's name (`EndpointIdentificationAlgorithm=HTTPS`).

QuickFIX/n takes its certificate as a PKCS#12 bundle, so `src/pki.rs` also writes each side's
certificate, key and CA as a `.pfx`, and in `qfn` cells Turbojet loads its own from its bundle
too (`Identity::from_pkcs12_file`). The .NET peer checks the server's name (`SSLServerName`).
Two QuickFIX/n settings need care: it checks revocation online by default, which certificates
without a CRL distribution point fail, so the peer turns that off; and an acceptor that validates
certificates refuses a client without one whatever `SSLRequireClientCertificate` says, so it
validates only when it requires one.

A refused handshake must leave both sides logged off while the initiator keeps trying. Each
refusal was checked to fail for its reason on both sides. As initiator, QuickFIX/J and QuickFIX/n
log each failed handshake. When it's QuickFIX/n's certificate that's refused, TLS 1.3 tells it only
after its side of the handshake: on Linux, where .NET uses TLS 1.3, it logs the alert that ends its
first read; on macOS, .NET negotiates TLS 1.2 and learns of the refusal in the handshake.

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
