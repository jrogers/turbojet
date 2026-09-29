# turbojet-acceptance

Session-level acceptance tests: the scripted scenarios QuickFIX checks its own session layer with,
run against a Turbojet acceptor. There are 235: FIX 4.2, 4.3, 4.4 and FIXT.1.1 with FIX 5.0 SP2,
each covering the FIX specification's session test cases (logon, sequencing, gaps and resends,
garbled and invalid messages, heartbeats, logout) and a few more. Each is its own test, named
after its script, e.g. `fix42_2b_msg_seq_num_too_high`. The crate isn't published.

```sh
cargo test -p turbojet-acceptance                  # all of them, in well under a second
cargo test -p turbojet-acceptance fix44_2b -- --nocapture
```

## How they run

The scripts in `definitions/` are QuickFIX's, unchanged (see below). Each line is one step:
`iCONNECT` and `iDISCONNECT` (the counterparty connects or closes), `I<message>` (it sends a
message; BodyLength and CheckSum are added unless given, and `<TIME>`, `<TIME+n>`, `<TIME-n>`
become timestamps), `E<message>` (the engine must send this next) and `eDISCONNECT` (the engine
must close the connection, sending nothing more first). A prefix such as `I2,` names a second
connection.

There are no sockets: the bytes go through Turbojet's codec into a `Session` configured as
QuickFIX's test server is (SenderCompID `ISLD`, one counterparty per version, validation against
the official dictionary, sequence numbers reset on each logon), on a virtual clock. When the
engine is expected to send something and hasn't, the clock jumps to its next deadline, as the
connection driver's timer would, for up to 65 seconds, so heartbeat and timeout scenarios take no
real time. The application echoes orders back, as QuickFIX's test application does.

A message the engine sends matches the expected one if it has the same fields with the same
values, in any order, except that:

- BodyLength(9) and CheckSum(10) aren't compared, since they follow from everything else;
- Text(58) isn't compared: each engine words its own, and the reject reason codes carry the
  meaning;
- timestamps (SendingTime, OrigSendingTime, TransactTime, OrigTime) need only look like one;
- a Reject may carry RefTagID(371) where the script has none;
- the TestReqID of a TestRequest the engine sends is its own choice.

A message sent after the engine has disconnected is lost, as a TCP write usually is when the other
end has just closed.

## Known failures

`known_failures.txt` lists the scenarios that fail, and why: checks Turbojet doesn't do yet (on
the roadmap), deviations from the spec's test cases still to fix, and scripts that rely on
QuickFIX's own behaviour. Their tests pass while they fail; a listed scenario that starts passing
fails its test until it's removed from the list. A failure prints the step, the difference and a
transcript of everything sent each way.

## Licence

The scripts in `definitions/` are from QuickFIX
([github.com/quickfix/quickfix](https://github.com/quickfix/quickfix), `test/definitions/server`,
commit `386ce46`, 2026-05-20), under the QuickFIX Software License (`definitions/LICENSE`). This
product includes software developed by quickfixengine.org (http://www.quickfixengine.org/).
