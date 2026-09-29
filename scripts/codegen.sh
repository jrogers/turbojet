#!/bin/sh
# Regenerates the checked-in FIX version crates from the official FIX Orchestra files in
# dictionaries/orchestra. CI fails if the output differs from what's committed.
set -eu
cd "$(dirname "$0")/.."
# FIX 4.2 corrections (see crates/turbojet-fix42/README.md): QuoteCancel's NoQuoteEntries is
# required in the file, but a cancel-all request has no entries; FIX 4.4 made it optional.
cargo run --quiet --locked -p turbojet-codegen -- dictionaries/orchestra/OrchestraFIX42.xml \
    --optional QuoteCancel.NoQuoteEntries \
    --out crates/turbojet-fix42/src/generated
cargo run --quiet --locked -p turbojet-codegen -- dictionaries/orchestra/OrchestraFIX43.xml --out crates/turbojet-fix43/src/generated
cargo run --quiet --locked -p turbojet-codegen -- dictionaries/orchestra/OrchestraFIX44.xml --out crates/turbojet-fix44/src/generated
# FIX 5.0 SP2 runs over FIXT 1.1, which supplies the header and trailer.
cargo run --quiet --locked -p turbojet-codegen -- dictionaries/orchestra/OrchestraFIX50SP2.xml \
    --transport dictionaries/orchestra/FIXTSession.xml \
    --out crates/turbojet-fix50sp2/src/generated
