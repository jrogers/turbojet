# Containers and Kubernetes

Turbojet is a library, so a container runs your program, and what it needs around it is set up
there: storage that outlives the pod, time to log out on shutdown, a fixed address for
counterparties to allow, and probes. This chapter goes through each, pointing to the chapters
with the details.

## Storage

`DiskStorage` keeps each session's files in its directory (see [Storage and recovery][storage]),
which has to outlive the pod: a store lost with it starts its sessions again at sequence number 1,
which a counterparty refuses partway through a day. Give each instance a volume of its own, a
StatefulSet's `volumeClaimTemplates` on block storage that attaches to one node at a time
(`ReadWriteOnce`). Keep it off NFS and similar network file systems: `DiskStorage` locks a
session's `.seqnums` file while the session is connected, so that two processes can't run it at
once, and those file systems don't all keep locks reliably.

Every commit waits for `fsync`, and on network block storage that typically takes milliseconds
rather than the tens of microseconds of a local NVMe drive. Measure it on the volume class you
use. A local volume is faster but ties the pod to its node.

`SqlStorage` (`turbojet-sql`) needs no volume: several gateways can share a database, a lease on
each session letting one run it at a time.

## Shutting down

Kubernetes stops a pod with SIGTERM and, after `terminationGracePeriodSeconds` (30 by default),
SIGKILL. Turbojet doesn't handle signals: listen for SIGTERM and call `shutdown`, as in
[Operations][operations], which logs every session out and returns within the sessions'
`logout_timeout` (5 s by default) and a second. Keep the grace period above that and whatever
your program does after it. A process that runs as PID 1, as a container's entry point does,
ignores SIGTERM unless it installs a handler, so without one each stop waits out the grace period
and kills the sessions without a Logout.

A sessions file from a ConfigMap, with TLS files from Secrets, reloads while the program runs:
`SessionsFile::reload` reads the TLS files again too, so renewed certificates are used from the
next handshake (see [TLS][tls]). Mount them as directories: a file mounted with `subPath` isn't
updated when its ConfigMap or Secret changes. Passwords go in environment variables
(`password_env`, `proxy_password_env` and `pkcs12_password_env` name them), set from a Secret.

## A hot standby

A standby (`turbojet-replica`, see [Storage and recovery][storage]) runs in a pod of its own,
with a volume of its own, serving the replication link on a port of its own. Nothing in Turbojet
fails over by itself: something you run takes the lease, checks the standby is in sync and
promotes it, then starts its acceptor. Until then, keep the standby's pod out of the Service the
counterparties connect to: report it not ready until it's serving FIX, since a Service routes to
every ready pod. `SqlLease` keeps the lease in a database, timed by the database's clock.

## Message logs

A `FileMessageLog`'s directory needs a persistent volume too, or something that ships its files
off the pod as they're written, or the audit trail goes with the pod. See
[Operations][operations] for its files and retention.

## Networking

Counterparties usually allow known addresses only. An initiator's connections leave through the
node, so give them a fixed address on the way out, such as a NAT gateway's, or a pod network
that keeps the pod's address, with `InitiatorConfig::local_addr` choosing among a host's
addresses where it has several (see [Initiators][initiators]). Endpoints are names looked up again
at each connection, so a counterparty's DNS changes take effect at the next reconnect.

An acceptor behind a network load balancer sees every connection come from the balancer, unless it
keeps client addresses. Turn on the PROXY protocol at the balancer and list it in
`AcceptorConfig::proxy_protocol`, so that allowed addresses and the per-address limit apply to the
counterparties (see [Acceptors][acceptors]).

## Latency

For the lowest latency, give the pod the Guaranteed QoS class (requests equal to limits) with
whole CPUs, which the kubelet's static CPU manager policy gives it to itself. A CPU limit below
what the pod uses is enforced by throttling it for the rest of each scheduling period (100 ms by
default), which shows up as latency spikes rather than as a slower average. See
[Performance][performance] for the ways to send and spinning connections.

## Readiness and liveness

Turbojet serves no HTTP, so probes are your program's. Its state is in an acceptor's
`statuses()` (the sessions connected, with their state and last messages), an initiator's
`handle().status()`, and either's `subscribe()` (an event as each session connects, logs on, logs
out or is refused):

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let acceptor = Acceptor::new(SessionConfig::new("FIX.4.2", "SERVER"), Arc::new(MemoryStorage::new()), Arc::new(App))?;
let logged_on = acceptor
    .statuses()
    .iter()
    .filter(|status| status.activity.as_ref().is_some_and(|activity| activity.state == SessionState::LoggedOn))
    .count();
# let _ = logged_on;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Make an acceptor ready once it's serving, and a standby only once promoted. Keep liveness to
whether the program itself is working, not whether counterparties are logged on: a counterparty
that's down would otherwise have its pods restarted for it, and a restart logs every session out.
