#!/usr/bin/env bash
# Model-checks a TLA+ spec in this directory with TLC, its .cfg's constants overridden by any
# KEY=VALUE given, and its invariants by INVARIANTS if set:
#   specs/check.sh Replication                         # the design as built
#   specs/check.sh Replication Checks=FALSE MaxConn=4  # without the standby check
#   INVARIANTS=NoSeqReuse2 specs/check.sh Replication LosesStore=TRUE Checks=FALSE
# Needs Java 11 or later. TLC is downloaded into target/ on first use, and its checksum checked.
set -euo pipefail
cd "$(dirname "$0")"
spec=${1:?usage: specs/check.sh SPEC [KEY=VALUE ...]}
shift
version=1.8.0
sha256=fc6878c603f0fdc18b9fe3e5bf51860cb90ab5b1154e3c371660c036d6bdd63e
out=../target/specs
mkdir -p "$out"
jar=$out/tla2tools-$version.jar
if [ ! -f "$jar" ]; then
    curl -sSfL -o "$jar.part" "https://github.com/tlaplus/tlaplus/releases/download/v$version/tla2tools.jar"
    echo "$sha256  $jar.part" | shasum -a 256 -c - >/dev/null
    mv "$jar.part" "$jar"
fi
cfg=$out/$spec.cfg
cp "$spec.cfg" "$cfg"
for kv in "$@"; do
    key=${kv%%=*}
    grep -q "^  $key = " "$cfg" || { echo "no constant $key in $spec.cfg" >&2; exit 2; }
    sed -i.bak "s|^  $key = .*|  $key = ${kv#*=}|" "$cfg"
done
if [ -n "${INVARIANTS:-}" ]; then
    sed -i.bak "s|^INVARIANTS .*|INVARIANTS $INVARIANTS|" "$cfg"
fi
exec java -XX:+UseParallelGC -cp "$jar" tlc2.TLC -workers auto -deadlock \
    -config "$cfg" -metadir "$out/$spec-states" "$spec.tla"
