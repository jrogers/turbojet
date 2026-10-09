#!/usr/bin/env bash
# Builds the QuickFIX/J, quickfix-go and QuickFIX/n peers and runs the interop tests against them.
# Arguments go to cargo test, e.g. scripts/interop.sh gap_fill_from_peer, or scripts/interop.sh qfn
# for one engine's cells. Needs a JDK, 21 or later (the Gradle wrapper fetches Gradle), Go, and the
# .NET 10 SDK.
set -euo pipefail
cd "$(dirname "$0")/.."

crates/turbojet-interop/peer/gradlew -q -p crates/turbojet-interop/peer shadowJar
go build -C crates/turbojet-interop/peer-go -o peer .
dotnet build crates/turbojet-interop/peer-net -c Release --nologo -v quiet
TURBOJET_INTEROP=1 cargo test -p turbojet-interop --locked "$@"
