# Acceptors

An `Acceptor` serves every counterparty that logs on to its listener, each session with its own
sequence numbers and store. `Acceptor::new` takes a `SessionConfig`, which every counterparty
then shares; an `AcceptorConfig` adds settings per counterparty and limits on connections.

## Settings per counterparty

With `AcceptorConfig::counterparties`, a `Counterparties` resolver decides at each Logon, once
SenderCompID is known, whether that counterparty may log on and with which settings: its own
schedule, rate limits, validation, SendingTime tolerance and so on, the HeartBtInt range it may
ask for, and whether it must present a TLS client certificate. `CounterpartyMap` covers the usual
case, by CompID:

```rust
# use std::sync::Arc;
# use std::time::Duration;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let app = Arc::new(App);
# let dir = tempfile::tempdir()?;
use turbojet::store::StorageByCounterparty;
use turbojet::CounterpartyMap;

let counterparties = CounterpartyMap::new()
    .with("BROKER", |c| c.config.schedule = Some("daily 08:00-17:00 UTC".parse().unwrap()))
    .with("FUND", |c| {
        c.require_client_certificate = true;
        c.heartbeat = Duration::from_secs(10)..=Duration::from_secs(60);
    })
    // One of BROKER's sessions, told apart by its SubID, with settings of its own.
    .with_session(SessionId::new("FIX.4.4", "VENUE", "BROKER").with_target_sub_id("DESK2"), |c| {
        c.config.max_latency = None;
    })
    .refuse_unknown(); // or admit others with the acceptor's own settings
let storage = StorageByCounterparty::new(Arc::new(MemoryStorage::new()))
    .with("FUND", Arc::new(DiskStorage::new(dir.path())?));
let mut config = AcceptorConfig::new(SessionConfig::new("FIX.4.4", "VENUE"));
config.counterparties = Some(Arc::new(counterparties));
let acceptor = Acceptor::new(config, Arc::new(storage), app)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Each counterparty starts from the acceptor's configuration. What's in use before Logon stays the
acceptor's, and a counterparty's settings that change it are refused at Logon: its identity
(BeginString, SenderCompID), clock, logon timeout and send queue. The Logon itself is checked
against the acceptor's configuration too, its SendingTime against `max_latency` for one. The
resolver runs before `Application::verify_logon`; a refusal, or a panic, closes the connection
as a `verify_logon` refusal does.

Stores are chosen apart, by `StorageByCounterparty`, because an operator's change to a session
that isn't connected opens its store with no Logon to decide from.

## Connection limits and allowed addresses

An acceptor keeps at most 1,024 connections open at once, and 16 from one IP address
(`AcceptorConfig::max_connections`, `max_connections_per_ip`), counting every connection until it
closes, logged on or not. One past either is closed as soon as it's accepted, before any TLS
handshake, and counted in `turbojet_connections_refused_total`. Counterparties behind one address
(a NAT, or a hub serving several firms) share its limit.

`AcceptorConfig::allowed_ips` (and `FixpAcceptorConfig`'s) accepts connections only from the
addresses and CIDR ranges given, closing any other as soon as it's accepted, before a TLS
handshake or a Logon. `set_allowed_ips` replaces the list while serving; connections already open
stay. In a sessions file it's `allowed_ips` in `[acceptor]`, reloaded with the rest.

```rust
# use std::sync::Arc;
# use turbojet::*;
# struct App;
# impl Application for App {
#     fn on_message(&self, _: &mut Context<'_>, _: &Message) -> Result<(), MessageReject> { Ok(()) }
# }
# let (storage, app) = (Arc::new(MemoryStorage::new()), Arc::new(App));
use turbojet::AllowedIps;

let mut config = AcceptorConfig::new(SessionConfig::new("FIX.4.4", "VENUE"));
config.allowed_ips = Some(AllowedIps::new(["10.1.0.0/16", "203.0.113.5"])?);
config.proxy_protocol = Some(AllowedIps::new(["10.9.0.10", "10.9.0.11"])?); // the load balancers
let acceptor = Acceptor::new(config, storage, app)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Behind a load balancer

Behind a load balancer or TCP proxy, every connection comes from the device. With
`AcceptorConfig::proxy_protocol` listing the devices, as above, a connection from one must start
with a PROXY protocol header (version 1 or 2, within `PROXY_HEADER_TIMEOUT`), and is taken as
coming from the client the header names: for the allowed addresses, the per-address limit, and the
`ConnectionInfo` that `verify_logon` sees. Connections from other addresses are taken as they
come, so a client reaching the listener directly can't claim another's address. In a sessions
file it's `proxy_protocol` in `[acceptor]`, fixed until a restart.

## Configuration files

`turbojet-config` reads all of this from a TOML file: an `[acceptor]`, `[defaults]` and a
`[counterparty.COMPID]` section each (or `[counterparty.NAME]` with `target_comp_id` and SubIDs
or LocationIDs, for one of a counterparty's sessions), named stores, and `[initiator.NAME]`
sections. It reloads the file while running: changed settings apply from each counterparty's next
Logon, counterparties no longer listed are logged out, and a file that doesn't load leaves the one
in use in place. Its own documentation lists every key. The example gateway takes one with
`--config FILE` and reloads it on SIGHUP.

**Coming from QuickFIX.** `turbojet_config::quickfix::convert` turns a QuickFIX, QuickFIX/J or
quickfix-go settings file into a sessions file to review, and the gateway does it from the
command line:

```sh
gateway convert-cfg quickfix.cfg > sessions.toml
```

Acceptor sessions become the `[acceptor]` and a `[counterparty.X]` each (so they must share one
port and SenderCompID), and initiator sessions an `[initiator.X]` each. The session's identity,
addresses, schedule (`StartTime`, `EndTime`, `StartDay`, `EndDay`, `Weekdays`, `TimeZone`,
`NonStopSession`), `HeartBtInt`, `ReconnectInterval`, `ResetOnLogon` (an initiator's), timeouts,
`MaxLatency`, timestamp precision, file stores and data dictionaries are converted. Logging and
socket options are listed in a comment, since Turbojet sets them in code. Anything that would
behave differently, such as an acceptor's `ResetOnLogon=Y`, `ResetOnLogout=Y` or Java key stores,
is refused, every one named. Paths are copied as written, but a sessions file reads them from its
own directory, where QuickFIX reads them from where it runs.
