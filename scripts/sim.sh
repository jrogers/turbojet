#!/usr/bin/env bash
# Runs the deterministic simulator (crates/turbojet-sim), release build.
#   scripts/sim.sh 600           # random seeds for ten minutes, stopping at the first failure
#   scripts/sim.sh 0 1234 -v     # replay seed 1234, printing every event
#   scripts/sim.sh 600 --fixp    # the same for FIXP sessions (crates/turbojet-sim/src/fixp)
set -euo pipefail
cd "$(dirname "$0")/.."
exec cargo run --release --quiet -p turbojet-sim -- "$@"
