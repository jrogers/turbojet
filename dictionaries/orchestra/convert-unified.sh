#!/bin/sh
# Builds an Orchestra file for a FIX version the orchestrations repository doesn't publish (FIX 4.3,
# FIX 5.0 SP2) from the FIX Unified Repository (2010 Edition), which the FIX Trading Community
# publishes on fixtrading.org (sign-in needed). Uses the Community's own converter,
# unified2orchestra.xslt from fix-orchestra, with an empty phrases file so no documentation text is
# carried over, then repairs groups declared inline in components, as FIX 4.3 does (see
# fix-inline-groups.py). Needs Java, Python 3 and network access.
#
# Usage: ./convert-unified.sh path/to/fix_repository_2010_edition_20200402.zip FIX.4.3|FIX.5.0SP2
# writes OrchestraFIX43.xml or OrchestraFIX50SP2.xml.
set -eu
ZIP=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
VERSION=$2
OUT=Orchestra$(echo "$VERSION" | tr -d .).xml
ORCHESTRA=v1.6        # fix-orchestra release with unified2orchestra.xslt
SAXON=10.1            # the Saxon-HE version fix-orchestra builds with
cd "$(dirname "$0")"
HERE=$(pwd)
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

unzip -q "$ZIP" -d "$WORK/repo"
REPO=$(find "$WORK/repo" -maxdepth 1 -mindepth 1 -type d)
curl -fsSL -o "$WORK/saxon.jar" "https://repo1.maven.org/maven2/net/sf/saxon/Saxon-HE/$SAXON/Saxon-HE-$SAXON.jar"
curl -fsSL -o "$WORK/unified2orchestra.xslt" \
    "https://raw.githubusercontent.com/FIXTradingCommunity/fix-orchestra/$ORCHESTRA/repository-util/src/main/resources/xsl/unified2orchestra.xslt"
printf '<?xml version="1.0" encoding="UTF-8"?>\n<phrases version="%s"/>\n' "$VERSION" > "$WORK/no-phrases.xml"

java -jar "$WORK/saxon.jar" -s:"$REPO/Unified/FixRepository.xml" -xsl:"$WORK/unified2orchestra.xslt" \
    -o:"$WORK/converted.xml" phrases-file="file://$WORK/no-phrases.xml" name="$VERSION" new-version="$VERSION"
python3 fix-inline-groups.py "$WORK/converted.xml" "$REPO/$VERSION/Base/MsgContents.xml" \
    OrchestraFIX44.xml "$HERE/$OUT"
