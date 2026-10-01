# Working on Turbojet

Turbojet is a FIX engine in Rust: a sans-IO session state machine with an injected clock, tokio
transport, and typed messages generated from the FIX Orchestra files. Read [STYLE.md](STYLE.md)
before writing code; it's the house style and isn't repeated here. [ROADMAP.md](ROADMAP.md) says
what's done, what's next and the known caveats.

## Checking a change

`scripts/check.sh` runs the per-push CI checks (fmt, clippy and tests for each feature combination,
rustdoc with `-D warnings`, the fuzz crate's lints, the codegen diff). Run it before calling work
done or committing it; it takes about 3 minutes warm. `--interop` adds the QuickFIX/J tests and
`--msrv` the 1.89 tests. While iterating, narrower runs are fine:

```sh
cargo test -p turbojet --all-features                       # the engine
cargo test -p turbojet --test allocations -- --nocapture    # the allocation budget, by stage
cargo test -p turbojet-acceptance                           # the QuickFIX scenarios, under a second
scripts/interop.sh gap_fill_from_peer                       # one interop test (JDK 21+)
cargo bench -p turbojet --all-features -- --quick           # benchmark smoke run
scripts/fuzz.sh 60 session                                  # fuzz one target (nightly, cargo-fuzz)
```

Features: `tls`, `metrics`, `tz`, `validation`. Feature-gated code only builds when its feature is
on, so check with `--all-features` and `--no-default-features` at least.

## Things that bite

- **Generated crates.** `crates/turbojet-fix{42,43,44,50sp2}/src/generated` come from
  `turbojet-codegen` and `dictionaries/orchestra`. Don't edit them by hand: change the generator or
  the `scripts/codegen.sh` arguments, rerun the script and commit its output. CI fails on a diff.
- **The fuzz crate** (`crates/turbojet/fuzz`) is outside the workspace with its own `Cargo.lock`.
  Workspace commands don't see it. A version bump or API change must also update it, or the
  `fuzz-build` job fails.
- **The allocation budget** (`crates/turbojet/tests/allocations.rs`) is exact. It fails if any
  stage's count changes, up or down. When a change moves it on purpose, update the budget and the
  README table, and say why in the commit.
- **Acceptance known failures.** `crates/turbojet-acceptance/known_failures.txt` lists scenarios
  expected to fail, with a reason. A listed scenario that passes fails its test, so remove it from
  the list when it's fixed.
- **Interop tests** are skipped unless `TURBOJET_INTEROP=1` (`scripts/interop.sh` sets it), not
  behind a feature, because CI runs `--all-features`.
- **When Turbojet and a peer or a script disagree** (interop, acceptance), decide from the FIX spec
  which side is wrong. Don't just make the test pass. Fix Turbojet bugs in their own commits.
- **Versions.** `turbojet` has its own version in `crates/turbojet/Cargo.toml`; the other crates
  use the workspace version.

## How work is done here

- Plans and design notes go in `docs/plans/` (gitignored). Feature branches are git worktrees under
  `.worktrees/` (gitignored). Leave the main checkout on `main`: several sessions may share it, and
  one that switches branch there makes another's merge land on the wrong branch. Before merging,
  check which branch the checkout is on.
- Commits are small and each says why. Performance commits give before and after numbers (criterion
  medians on an idle machine). Running benchmarks needs no permission.
- A feature ends with a commit that records it in README.md, ROADMAP.md and CHANGELOG.md
  (`## Unreleased`), e.g. "Record X in the readme, roadmap and changelog".
- Public documents (README, SECURITY.md, the changelog, crate docs) make no promises: no
  timelines, support commitments or fix commitments unless the maintainer says so.
- Expensive checks that aren't regressions (long fuzzing) stay off the per-push CI path. They run
  in scheduled or manual workflows (`fuzz.yml`, nightly).
