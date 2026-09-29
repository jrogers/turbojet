#!/usr/bin/env bash
# Builds the QuickFIX/J peer and runs the interop tests against it. Arguments go to cargo test,
# e.g. scripts/interop.sh gap_fill_from_peer. Needs a JDK, 21 or later; the Gradle wrapper fetches Gradle.
set -euo pipefail
cd "$(dirname "$0")/.."

crates/turbojet-interop/peer/gradlew -q -p crates/turbojet-interop/peer shadowJar
TURBOJET_INTEROP=1 cargo test -p turbojet-interop --locked "$@"
