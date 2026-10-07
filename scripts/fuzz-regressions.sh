#!/usr/bin/env bash
# Runs each input in crates/turbojet/fuzz/regressions/<target>/ through its fuzz target once,
# failing if any fails. They once found a bug, in Turbojet or in the target, and are kept so it
# stays fixed. Runs on stable without cargo-fuzz: a target built by plain cargo is a libFuzzer
# binary that runs the files it's given. CI runs it on every push.
#   scripts/fuzz-regressions.sh
# Once a failing input is fixed, copy it from fuzz/artifacts/<target>/ (or the Fuzz workflow's
# fuzz-artifacts) to regressions/<target>/, named for what it found.
set -euo pipefail
cd "$(dirname "$0")/../crates/turbojet/fuzz"

cargo build --bins --locked --quiet
for dir in regressions/*/; do
    target=$(basename "$dir")
    echo "== $target"
    if ! out=$("${CARGO_TARGET_DIR:-target}/debug/$target" "$dir"* 2>&1); then
        echo "$out"
        exit 1
    fi
done
