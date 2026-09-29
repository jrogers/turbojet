# Security

Turbojet accepts connections and parses messages from counterparties, so a flaw in it may be
reachable from the network. Please report security problems privately, not in a public issue.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting: the **Security** tab of this repository, then
**Report a vulnerability**. Include what's affected (crate, version or commit, features), how to
reproduce it, and what an attacker could do with it.

## What counts

Anything a counterparty, or someone who can reach a listening port, can do that they shouldn't:
crash or hang the process, exhaust its memory, get a message accepted that should be refused, read
or change another session's state, or get past logon, TLS or certificate checks. Denial of service
through sheer volume (more connections or messages than the host can take) is out of scope.

## Status

Turbojet is pre-1.0 and provided as is, without warranty of any kind; see the license.
