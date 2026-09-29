#!/usr/bin/env bash
# Fuzzes Turbojet's parsers (crates/turbojet/fuzz) with cargo-fuzz: each target for SECONDS
# (default 60), or just TARGET. Needs a nightly toolchain and `cargo install cargo-fuzz`.
#   scripts/fuzz.sh              # every target, a minute each
#   scripts/fuzz.sh 600 session  # the session target, ten minutes
# A crash is saved under crates/turbojet/fuzz/artifacts/<target>/; replay it with
#   cargo +nightly fuzz run <target> <file>   (from crates/turbojet)
set -euo pipefail
cd "$(dirname "$0")/../crates/turbojet"

seconds=${1:-60}
targets=${2:-$(cargo +nightly fuzz list)}
# cargo-fuzz builds for the platform it was itself built for, which for a prebuilt Linux binary is
# musl, where the sanitizer doesn't work; build for the toolchain's host instead.
host=$(rustc +nightly -vV | sed -n 's/^host: //p')
for target in $targets; do
    echo "== $target (${seconds}s)"
    cargo +nightly fuzz run --target "$host" "$target" -- -dict=fuzz/fix.dict -max_total_time="$seconds"
done
