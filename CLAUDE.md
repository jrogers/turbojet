# Working on Turbojet

Turbojet is a FIX engine in Rust: a sans-IO session state machine with an injected clock, tokio
transport, and typed messages generated from the FIX Orchestra files. Read [STYLE.md](docs/STYLE.md)
before writing code; it's the house style and isn't repeated here. [ARCHITECTURE.md](docs/ARCHITECTURE.md)
is the overview of how the pieces fit, [DESIGN.md](docs/DESIGN.md) says how it's built and what's been measured, [ROADMAP.md](docs/ROADMAP.md) what's still to do (open work only),
and [CAVEATS.md](docs/CAVEATS.md) the behaviour worth knowing about.

## Checking a change

`scripts/check.sh` runs the per-push CI checks (fmt, clippy and tests for each feature combination,
rustdoc with `-D warnings`, the fuzz crate's lints and regression inputs, the codegen diff). Run it before calling work
done or committing it; it takes about 3 minutes warm. `--interop` adds the QuickFIX/J and QuickFIX/n tests and
`--msrv` the 1.89 tests. While iterating, narrower runs are fine:

```sh
cargo test -p turbojet --all-features                       # the engine
cargo test -p turbojet --test allocations -- --nocapture    # the allocation budget, by stage
cargo test -p turbojet-acceptance                           # the QuickFIX scenarios, under a second
scripts/interop.sh gap_fill_from_peer                       # one interop scenario (JDK 21+, .NET 10)
cargo bench -p turbojet --all-features -- --quick           # benchmark smoke run
scripts/fuzz.sh 60 session                                  # fuzz one target (nightly, cargo-fuzz)
scripts/python.sh                                           # the Python bindings (uv)
scripts/nightly.sh                                          # last night's runs, and how to replay a failure
scripts/site.sh [--serve]                                   # the website, into target/site (mdbook, mdbook-mermaid)
```

Features: `tls`, `pkcs12`, `metrics`, `tz`, `validation`, `gzip`, `fast`. Feature-gated code only builds when its feature is
on, so check with `--all-features` and `--no-default-features` at least.

## Things that bite

- **Generated code.** `crates/turbojet-fix/src/fix*/` come from
  `turbojet-codegen` and `dictionaries/orchestra`. Don't edit them by hand: change the generator or
  the `scripts/codegen.sh` arguments, rerun the script and commit its output. CI fails on a diff.
- **The fuzz crate** (`crates/turbojet/fuzz`) is outside the workspace with its own `Cargo.lock`.
  Workspace commands don't see it. A version bump or API change must also update it, or the
  `fuzz-build` job fails.
- **The Python bindings** (`crates/turbojet-python`) are outside the workspace too, with their own
  `Cargo.lock` and `uv.lock`: `cargo test` can't link libpython, so they're built with maturin and
  tested with pytest (`scripts/python.sh`, `check.sh --python`). An API change must update them.
- **Workspace builds unify features.** `turbojet`'s dev-dependency on `turbojet-config` (for the
  gateway) turns `tls` on everywhere, so `--no-default-features` across the workspace doesn't
  build `turbojet-config` without it. Check that crate alone: `cargo test -p turbojet-config
  --no-default-features`. Likewise `turbojet` turns on every FIX version of `turbojet-fix`, so
  check its code for one version alone with `-p turbojet-fix --no-default-features --features
  fix42`.
- **The allocation budget** (`crates/turbojet/tests/allocations.rs`) is exact. It fails if any
  stage's count changes, up or down. When a change moves it on purpose, update the budget and the
  README table, say why in the commit, and add the old numbers to DESIGN.md's measurement history.
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

- Plans and working notes go in `docs/plans/` (gitignored). Feature branches are git worktrees under
  `.worktrees/` (gitignored). Leave the main checkout on `main`: several sessions may share it, and
  one that switches branch there makes another's merge land on the wrong branch. Before merging,
  check which branch the checkout is on.
- Commits are small and each says why. Performance commits give before and after numbers (criterion
  medians on an idle machine). Running benchmarks needs no permission.
- A feature ends with a commit that records it: how to use it in the guide's chapter for it
  (`docs/guide/`, for users: no measurement history or internals; its examples are doctests, run
  from `crates/turbojet/src/guide.rs`), and in README.md only if it changes the overview or the
  quick start; how it works and its numbers in DESIGN.md, its entry taken
  off ROADMAP.md (not marked done), any caveat in CAVEATS.md, and CHANGELOG.md
  (`## Unreleased`), e.g. "Record X in the guide, design notes and changelog".
- Public documents (README, SECURITY.md, the changelog, crate docs) make no promises: no
  timelines, support commitments or fix commitments unless the maintainer says so.
- Expensive checks that aren't regressions (long fuzzing) stay off the per-push CI path. They run
  in scheduled or manual workflows (`fuzz.yml`, nightly).

## Releasing

1. Bump `[workspace.package] version`, `turbojet`'s own in `crates/turbojet/Cargo.toml`, and the
   `turbojet` and `turbojet-dictionary` versions in `[workspace.dependencies]`; `cargo update -w`,
   and `cargo update -p turbojet` in `crates/turbojet/fuzz` and `crates/turbojet-python`.
2. Date the changelog's `## Unreleased` heading: `## 0.6.0 (2026-11-02)`.
3. `scripts/check.sh --msrv --semver --python`, then `cargo publish --workspace --dry-run`.
4. Commit ("Release 0.6.0") and push. The maintainer runs `cargo publish --workspace` (it needs
   their crates.io token); log it to a file rather than a pipe, since a filter that exits early
   kills the publish.
5. `scripts/github-release.sh 0.6.0`: tags the release commit `v0.6.0` and makes a GitHub release
   of it, with the changelog's section as its notes. Pushing the tag also deploys the website
   (`.github/workflows/site.yml`), built from the tag's docs.
6. Once 0.6.0 is on crates.io, delete `assets/logo.svg` (and this step): the READMEs published
   before it load the logo from there, and 0.6's from `docs/assets/`.
