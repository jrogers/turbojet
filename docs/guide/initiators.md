# Initiators

An `Initiator` connects out to one counterparty and logs on, and `Initiator::run` keeps it
connected: it reconnects when the session ends and fails over between endpoints.

## Failover

An initiator has a primary endpoint and any number of backups, `InitiatorConfig::endpoints`,
tried in order:

```rust,no_run
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let (storage, app) = (Arc::new(MemoryStorage::new()), Arc::new(App));
let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER")
    .with_endpoint("primary.example.com:9876")
    .with_endpoint("dr.example.com:9876")
    .with_endpoint(Endpoint::new("10.1.2.3:9876").with_tls_server_name("dr2.example.com"));
// The default TLS name; endpoints may override it.
let client = turbojet::tls::ClientTls::new(turbojet::tls::Trust::from_pem_files("ca.pem".as_ref())?, None)?;
config.tls = Some(InitiatorTls::new(client.connector(), "primary.example.com"));
let initiator = Initiator::new(config, storage, app)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Each connection attempt starts at the primary. An endpoint fails over to the next when the TCP
connect fails or exceeds `connect_timeout` (10 s by default), the TLS handshake fails, or the
connection ends before logon (the acceptor refuses it, say). Once a session has logged on, it
stays on that endpoint until it ends; the next attempt then starts at the primary again, so the
initiator fails back as soon as the primary is reachable.

All endpoints are treated as the same FIX session (a counterparty's primary and DR sites, say), so
sequence numbers carry over and the `SessionHandle` stays valid across failovers.

## Reconnecting

If every endpoint fails, or a session ends, `run()` waits and starts over, as
`InitiatorConfig::reconnect` says: by default 1 s, doubling while attempts keep failing up to
60 s, each wait a random half to all of that so initiators that lost a venue together don't all
come back together, and 1 s again once a session has logged on. `ReconnectPolicy::fixed` waits
the same every time:

```rust
use std::time::Duration;
use turbojet::{InitiatorConfig, ReconnectPolicy, SessionConfig};

let mut config = InitiatorConfig::new(SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
config.reconnect = ReconnectPolicy::fixed(Duration::from_secs(5));
```

Each failure is logged through `tracing`, as a warning. `Initiator::connect_once` makes a single
attempt instead, for a program that decides for itself when to try again.

## Source addresses and proxies

`InitiatorConfig::local_addr` makes every connection come from one local address, for a host with
several networks or a counterparty that only accepts known source addresses: an IP address, and
a port or 0 to let the system choose one. Only endpoint addresses of the same family (IPv4 or
IPv6) are tried. In configuration files it's an initiator's `local_addr`.

`InitiatorConfig::proxy` reaches every endpoint through an HTTP proxy (with `CONNECT`) or a SOCKS5
one, for a network that only connects out that way; `FixpInitiatorConfig::proxy` does the same
for FIXP. The proxy is asked for a tunnel to each endpoint by name, within the connect timeout,
and TLS runs through the tunnel to the counterparty. A username and password are optional. In
configuration files it's an initiator's `proxy`, with the password in the environment variable
`proxy_password_env` names.

```rust
# let mut config = turbojet::InitiatorConfig::new(turbojet::SessionConfig::new("FIX.4.4", "CLIENT"), "SERVER");
# let password = "secret";
use turbojet::Proxy;

config.local_addr = Some("10.0.0.5:0".parse()?);
config.proxy = Some(Proxy::socks5("proxy.internal:1080").with_username("firm").with_password(password));
// or "http://proxy.internal:3128".parse()?, or in a sessions file:
//   proxy = "socks5://firm@proxy.internal:1080"
//   proxy_password_env = "PROXY_PASSWORD"
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Changing an initiator while it runs

`Initiator::reconfigure` replaces an initiator's configuration, endpoints and TLS included, from
its next connection attempt, leaving a session that's connected alone, and
`Initiator::with_registry` lets initiators share a `SessionRegistry`, so handles from it outlive
an initiator replaced by another for the same session. `turbojet-config` uses both to run
`[initiator.NAME]` sections from a file and apply a reload to them.
