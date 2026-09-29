# FIX Orchestra files

The official machine-readable FIX 4.2, FIX 4.4 and FIXT 1.1 repositories (FIX Orchestra format) from the
`FIX Standard` directory of
[FIXTradingCommunity/orchestrations](https://github.com/FIXTradingCommunity/orchestrations),
downloaded by `fetch.sh` at the commit pinned there
(`cd24169a2abd8daba7c360987c7a46ca11873a12`). `OrchestraFIX42.xml` is FIX 4.2 as of Extension
Pack 310, `OrchestraFIX44.xml` is FIX 4.4 as of Extension Pack 311, and `FIXTSession.xml` is the
FIXT 1.1 session protocol (as of Extension Pack 247), which FIX 5.0 SP2 runs over.

The orchestrations repository has no FIX 4.3 or FIX 5.0 SP2 file, so `OrchestraFIX43.xml` and
`OrchestraFIX50SP2.xml` (the base SP2 specification; later extension packs are in FIX Latest) are
converted from the FIX Unified Repository, 2010 Edition (a zip from
[fixtrading.org](https://www.fixtrading.org/standards/fix-repository/), sign-in needed), by
`convert-unified.sh`: the FIX Trading Community's own `unified2orchestra.xslt` (fix-orchestra
v1.6), run with an empty phrases file so it carries no documentation text, then
`fix-inline-groups.py`, which defines the four groups FIX 4.3 declares inline in components (the
stylesheet doesn't handle them) under their FIX 4.4 names, and drops the conversion time.
Regenerate them with `./convert-unified.sh path/to/fix_repository_2010_edition_20200402.zip
FIX.4.3` (or `FIX.5.0SP2`); it needs Java and Python 3.

They are © FIX Protocol Limited and distributed under the Apache License, Version 2.0 (see
`LICENSE` and `NOTICE`).

To update them, set `COMMIT` in `fetch.sh` to the new commit, run `./fetch.sh`, and update the
commit in this README, in `NOTICE`, and in `crates/turbojet-fix42/NOTICE` and
`crates/turbojet-fix44/NOTICE`.
