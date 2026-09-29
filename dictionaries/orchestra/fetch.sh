#!/bin/sh
# Downloads the FIX Orchestra files at a pinned commit. Bump COMMIT to update them.
set -eu
COMMIT=cd24169a2abd8daba7c360987c7a46ca11873a12
BASE=https://raw.githubusercontent.com/FIXTradingCommunity/orchestrations/$COMMIT
cd "$(dirname "$0")"
for f in OrchestraFIX42 OrchestraFIX44 FIXTSession; do
    curl -fsSL -o "$f.xml" "$BASE/FIX%20Standard/$f.xml"
done
curl -fsSL -o LICENSE "$BASE/LICENSE"
