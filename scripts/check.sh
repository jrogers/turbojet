#!/usr/bin/env bash
# Runs the per-push CI checks (.github/workflows/ci.yml) locally, stopping at the first failure:
# formatting, clippy and tests for each feature combination CI uses, rustdoc with warnings as
# errors, the fuzz crate's lints and regression inputs, and generated crates matching the
# generator. Flags add the slower jobs:
#   scripts/check.sh            # what most changes need
#   scripts/check.sh --interop  # also the interop tests (needs a JDK 21+ and .NET 10)
#   scripts/check.sh --msrv     # also the tests on the minimum supported Rust (rustup toolchain 1.89)
#   scripts/check.sh --python   # also the Python bindings (needs uv)
#   scripts/check.sh --postgres # also the SQL stores on PostgreSQL, at TURBOJET_POSTGRES_URL (a
#                               # database the tests may clear)
#   scripts/check.sh --all      # interop, msrv and python
#   scripts/check.sh --semver   # also each published crate's API against its last release on
#                               # crates.io (cargo-semver-checks), before releasing: fails if the
#                               # version doesn't say what changed (a 0.x API break needs 0.x+1)
# cargo-deny and benchmark compilation are left to CI.
set -euo pipefail
cd "$(dirname "$0")/.."

interop=false
msrv=false
postgres=false
python=false
semver=false
for arg in "$@"; do
    case $arg in
        --interop) interop=true ;;
        --msrv) msrv=true ;;
        --postgres) postgres=true ;;
        --python) python=true ;;
        --semver) semver=true ;;
        --all) interop=true; msrv=true; python=true ;;
        *) echo "unknown argument: $arg" >&2; exit 2 ;;
    esac
done

step() { echo "== $*"; }

step fmt
cargo fmt --all --check
cargo fmt --manifest-path crates/turbojet/fuzz/Cargo.toml --check

# Keep in step with the test matrix in ci.yml. The simulator's seeds run the same whatever the
# engine's features, so they run once, with all of them.
for features in --no-default-features "--features tls" "--features metrics" "--features tz" --all-features; do
    step "clippy and test ($features)"
    # shellcheck disable=SC2086 # $features is one or two words on purpose.
    cargo clippy --workspace --all-targets $features --locked --quiet -- -D warnings
    # turbojet-interop turns pkcs12 on in workspace builds; build turbojet with TLS but without it.
    if [ "$features" = "--features tls" ]; then
        cargo clippy -p turbojet --all-targets --features tls --locked --quiet -- -D warnings
    fi
    # turbojet turns every FIX version of turbojet-fix on in workspace builds; build it without
    # them, and with one, as an application would.
    if [ "$features" = --no-default-features ]; then
        cargo clippy -p turbojet-fix --all-targets --no-default-features --locked --quiet -- -D warnings
        cargo test -p turbojet-fix --no-default-features --features fix42 --locked --quiet
    fi
    skip=--exclude=turbojet-sim
    [ "$features" = --all-features ] && skip=
    # shellcheck disable=SC2086
    cargo test --workspace $skip $features --locked --quiet
done

step shuttle
# The concurrency tests (src/sync.rs), with shuttle's locks and atomics in place of std's; nothing
# else runs under them. Their own target directory, as RUSTFLAGS rebuilds everything.
RUSTFLAGS="--cfg shuttle" CARGO_TARGET_DIR=target/shuttle cargo clippy -p turbojet --lib --tests --locked --quiet -- -D warnings
RUSTFLAGS="--cfg shuttle" CARGO_TARGET_DIR=target/shuttle cargo test -p turbojet --lib --locked --quiet shuttle_tests

step docs
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked --quiet
# Without features too: a link to feature-gated API breaks for anyone documenting without it.
RUSTDOCFLAGS="-D warnings" cargo doc -p turbojet --no-default-features --no-deps --locked --quiet

step fuzz clippy
cargo clippy --manifest-path crates/turbojet/fuzz/Cargo.toml --all-targets --locked --quiet -- -D warnings

step fuzz regressions
scripts/fuzz-regressions.sh

step codegen
# Compare against the working tree as it was, so uncommitted generator changes are checked too.
before=$(git status --porcelain -- crates/turbojet-fix/src crates/turbojet/tests/sbe/*.rs crates/turbojet/src/fixp/messages.rs; git diff -- crates/turbojet-fix/src crates/turbojet/tests/sbe/*.rs crates/turbojet/src/fixp/messages.rs crates/turbojet-python/python/turbojet/tags.py)
scripts/codegen.sh
after=$(git status --porcelain -- crates/turbojet-fix/src crates/turbojet/tests/sbe/*.rs crates/turbojet/src/fixp/messages.rs; git diff -- crates/turbojet-fix/src crates/turbojet/tests/sbe/*.rs crates/turbojet/src/fixp/messages.rs crates/turbojet-python/python/turbojet/tags.py)
if [ "$before" != "$after" ]; then
    echo "codegen changed the generated crates; commit the output of scripts/codegen.sh" >&2
    exit 1
fi

if $interop; then
    step interop
    scripts/interop.sh
fi
if $semver; then
    step semver
    # Publishable crates only (publish = false ones are skipped), with every feature's API.
    cargo semver-checks --workspace --all-features
fi
if $python; then
    step python
    scripts/python.sh
fi
if $postgres; then
    step postgres
    if [ -z "${TURBOJET_POSTGRES_URL:-}" ]; then
        echo "--postgres needs TURBOJET_POSTGRES_URL, e.g. postgres://user@localhost/turbojet_test" >&2
        exit 2
    fi
    cargo clippy -p turbojet-sql --all-targets --no-default-features --features postgres --locked --quiet -- -D warnings
    cargo test -p turbojet-sql --all-features --locked --quiet
fi
if $msrv; then
    step msrv
    cargo +1.89 test --workspace --all-features --locked --quiet
fi
step ok
