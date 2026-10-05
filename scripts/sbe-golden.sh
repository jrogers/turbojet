#!/bin/sh
# Regenerates crates/turbojet/tests/sbe/golden/*.bin: messages encoded by real-logic's SBE codecs
# (sbe-tool's Java output) from the schemas in dictionaries/sbe, which the generated Rust codecs
# must decode and match byte for byte. Needs a JDK (17 or later) and, the first time, a download of
# sbe-tool from Maven Central. Run it when the schemas or Golden.java change; commit the output.
set -eu
cd "$(dirname "$0")/.."
version=1.35.1
sha256=456384ed1db090d018b4dc15be152d371fa192e96aa4c98d362cc163bd18777e
work=target/sbe-golden
jar=$work/sbe-all-$version.jar
mkdir -p "$work"
if [ ! -f "$jar" ]; then
    curl -fsSL -o "$jar" "https://repo1.maven.org/maven2/uk/co/real-logic/sbe-all/$version/sbe-all-$version.jar"
fi
echo "$sha256  $jar" | shasum -a 256 -c - >/dev/null
rm -rf "$work/src" "$work/classes"
for schema in dictionaries/sbe/example-schema.xml dictionaries/sbe/b3-entrypoint.xml; do
    java -Dsbe.output.dir="$work/src" -Dsbe.target.language=Java -Dsbe.xinclude.aware=true -jar "$jar" "$schema"
done
javac -nowarn -cp "$jar" -d "$work/classes" $(find "$work/src" -name '*.java') crates/turbojet/tests/sbe/golden/Golden.java
# Agrona reads its buffers through jdk.internal.misc.Unsafe.
java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -cp "$jar:$work/classes" Golden crates/turbojet/tests/sbe/golden
