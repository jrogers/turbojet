# The Turbojet guide

This guide goes from a first initiator and acceptor to what running sessions in production asks
for: storage and recovery, schedules, operations and logging. Each chapter's examples are
compiled and run as tests, so they work as shown. The API reference has the details of each type.

1. [A first session][first-session]: an acceptor and an initiator in one program, the
   `Application` callbacks, sending, and how the engine is layered.
2. [Messages][messages]: typed messages, groups and enums, your own with `fix_message!`, and
   repeating groups.
3. [Session behaviour][sessions]: session IDs, logon credentials, the checks on inbound
   messages, and validating them against a data dictionary.
4. [Acceptors][acceptors]: settings and stores per counterparty, connection limits, allowed
   addresses, load balancers, and configuration files.
5. [Initiators][initiators]: failover, reconnecting, source addresses and proxies.
6. [TLS][tls]: certificates from files or memory, replaced while running, PKCS#12 and
   revocation lists.
7. [Storage and recovery][storage]: what is stored when, delivery at least once, holding a
   message in flight, the stores, and a hot standby that takes the sessions over.
8. [Schedules and rate limits][schedules]: trading hours, holidays and throttling.
9. [Operations][operations]: sequence numbers, managing sessions, cancel on disconnect, shutting
   down, logging, message logs and metrics.
10. [Performance][performance]: the lowest-latency ways to send, and spinning connections.
11. [Binary encodings][binary]: SBE codecs, FIXP sessions and FAST.

[first-session]: first-session.md
[messages]: messages.md
[sessions]: sessions.md
[acceptors]: acceptors.md
[initiators]: initiators.md
[tls]: tls.md
[storage]: storage.md
[schedules]: schedules.md
[operations]: operations.md
[performance]: performance.md
[binary]: binary.md
