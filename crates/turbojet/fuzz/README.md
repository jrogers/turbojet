# turbojet-fuzz

[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) targets for everything in Turbojet that
reads input from the network. Each one also checks a property, so a wrong answer counts as a
failure along with a panic:

- `decode`: raw bytes through the codec, decoded message after message as the connection's read
  loop does. Skipping garbled input always makes progress, and every message decoded can be
  encoded and decoded again with the same fields.
- `message`: a message body framed with a valid BodyLength and CheckSum, so it reaches the
  message parser (random bytes almost never checksum correctly), then read the ways the session
  and applications read it: fields, the view of the body, typed header fields, Display.
- `typed`: a framed message parsed into any generated message type of FIX 4.2, 4.3, 4.4 or
  5.0 SP2, repeating groups included (`build.rs` lists them from the generated sources). A
  message that parses must write out as one that parses back to the same thing.
- `session`: an acceptor or initiator session fed a Logon, then fuzzed steps: messages from the
  counterparty with a valid header but any MsgType, MsgSeqNum and fields, time passing, the
  application sending or logging out, and operator sequence-number changes. Every message the
  session sends must encode and decode cleanly and, apart from resends, go out in sequence.

The crate isn't part of the workspace: fuzzing needs nightly and cargo fuzz. CI checks on every
push that it builds (clippy, on stable), and the Fuzz workflow (`.github/workflows/fuzz.yml`)
fuzzes each target for ten minutes nightly, or for as long as asked when run by hand.

## Running

```sh
cargo install cargo-fuzz
scripts/fuzz.sh                 # every target, a minute each
scripts/fuzz.sh 600 session     # one target, ten minutes
```

`fix.dict` gives libFuzzer FIX tokens to splice in. A failing input is saved under
`artifacts/<target>/`; replay it from `crates/turbojet` with
`cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<file>`. The corpus each run builds is
kept in `corpus/`, which isn't checked in.

`cargo +nightly fuzz coverage <target>` and `llvm-cov` (from `rustup component add
llvm-tools-preview`) show what a corpus reaches; the session target's corpus should reach
resends, gap fills and the logon checks.
