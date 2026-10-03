#!/usr/bin/env bash
# Runs the per-push CI checks (.github/workflows/ci.yml) locally, stopping at the first failure:
# formatting, clippy and tests for each feature combination CI uses, rustdoc with warnings as
# errors, the fuzz crate's lints, and generated crates matching the generator. Flags add the slower
# jobs:
#   scripts/check.sh            # what most changes need
#   scripts/check.sh --interop  # also the QuickFIX/J interop tests (needs a JDK, 21 or later)
#   scripts/check.sh --msrv     # also the tests on the minimum supported Rust (rustup toolchain 1.89)
#   scripts/check.sh --postgres # also the SQL stores on PostgreSQL, at TURBOJET_POSTGRES_URL (a
#                               # database the tests may clear)
#   scripts/check.sh --all      # interop and msrv
# cargo-deny and benchmark compilation are left to CI.
set -euo pipefail
cd "$(dirname "$0")/.."

interop=false
msrv=false
postgres=false
for arg in "$@"; do
    case $arg in
        --interop) interop=true ;;
        --msrv) msrv=true ;;
        --postgres) postgres=true ;;
        --all) interop=true; msrv=true ;;
        *) echo "unknown argument: $arg" >&2; exit 2 ;;
    esac
done

step() { echo "== $*"; }

step fmt
cargo fmt --all --check
cargo fmt --manifest-path crates/turbojet/fuzz/Cargo.toml --check

# Keep in step with the test matrix in ci.yml.
for features in --no-default-features "--features tls" "--features metrics" "--features tz" --all-features; do
    step "clippy and test ($features)"
    # shellcheck disable=SC2086 # $features is one or two words on purpose.
    cargo clippy --workspace --all-targets $features --locked --quiet -- -D warnings
    # shellcheck disable=SC2086
    cargo test --workspace $features --locked --quiet
done

step docs
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked --quiet

step fuzz clippy
cargo clippy --manifest-path crates/turbojet/fuzz/Cargo.toml --all-targets --locked --quiet -- -D warnings

step codegen
# Compare against the working tree as it was, so uncommitted generator changes are checked too.
before=$(git status --porcelain -- crates/turbojet-fix*/src/generated; git diff -- crates/turbojet-fix*/src/generated)
scripts/codegen.sh
after=$(git status --porcelain -- crates/turbojet-fix*/src/generated; git diff -- crates/turbojet-fix*/src/generated)
if [ "$before" != "$after" ]; then
    echo "codegen changed the generated crates; commit the output of scripts/codegen.sh" >&2
    exit 1
fi

if $interop; then
    step interop
    scripts/interop.sh
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
