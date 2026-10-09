# QuickFIX/n's data dictionaries

`FIX42.xml`, `FIX43.xml`, `FIX44.xml`, `FIX50SP2.xml` and `FIXT11.xml` from QuickFIX/n's
`spec/fix` directory at the commit tagged v1.14.1 (`44c111acdb253335fd3c035589795468ff60c400`), the
version the peer runs (`../peer-net.csproj`): the QuickFIXn.Core package doesn't include them. The
build copies them next to the peer, where it reads them. `fetch.sh` downloads them again; bump its
`COMMIT` with the package.

They're © quickfixengine.org and distributed under the QuickFIX Software License, Version 1.0
(`LICENSE`): "This product includes software developed by quickfixengine.org
(http://www.quickfixengine.org/)."
