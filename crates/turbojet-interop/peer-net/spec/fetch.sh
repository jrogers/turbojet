#!/bin/sh
# Downloads QuickFIX/n's data dictionaries at the commit tagged v1.14.1, the version the peer
# uses (peer-net.csproj): QuickFIXn.Core doesn't ship them. Bump COMMIT with the package.
set -eu
COMMIT=44c111acdb253335fd3c035589795468ff60c400
BASE=https://raw.githubusercontent.com/connamara/quickfixn/$COMMIT
cd "$(dirname "$0")"
for f in FIX42 FIX43 FIX44 FIX50SP2 FIXT11; do
    curl -fsSL -o "$f.xml" "$BASE/spec/fix/$f.xml"
done
curl -fsSL -o LICENSE "$BASE/LICENSE"
