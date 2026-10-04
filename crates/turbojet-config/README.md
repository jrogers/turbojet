# turbojet-config

Session configuration files for [Turbojet](https://github.com/jrogers/turbojet): an acceptor, its
counterparties and their stores, read from TOML and reloaded while running.

```toml
[acceptor]
begin_string = "FIX.4.4"
sender_comp_id = "VENUE"
listen = "0.0.0.0:9876"
unknown = "refuse"                  # or "admit": unlisted counterparties get [defaults]

[store.main]
kind = "disk"
dir = "store"                       # relative to this file

[defaults]
store = "main"
schedule = "daily 08:00-17:00 America/New_York"
max_latency = "120s"

[counterparty.BROKER]
outbound_limit = "100/1s"
inbound_limit = "50/1s"
over_limit = "reject"

[counterparty.FUND]
require_client_certificate = true
heartbeat = { min = "10s", max = "60s" }
```

```rust,ignore
let sessions = SessionsFile::load("sessions.toml")?;
let acceptor = sessions.acceptor(Arc::new(MyApp));
tokio::spawn(acceptor.clone().serve(TcpListener::bind(sessions.listen()).await?));
// On SIGHUP, say:
let changes = sessions.reload(&acceptor)?;
```

- Each counterparty's settings are its own keys over `[defaults]`: schedule and holidays, rate
  limits, SendingTime tolerance, validation against a dictionary, the HeartBtInt it may ask for,
  whether it must present a TLS client certificate, and the store its sessions are kept in.
- Unknown keys are errors, and every value is checked when the file loads, so a typo can't be
  silently ignored and a file that loads has nothing left to fail at a counterparty's Logon.
  Errors name the section and key.
- A reload checks the whole file first and keeps the one in use if anything fails. Changed
  settings apply from each counterparty's next Logon; under `unknown = "refuse"`, connected
  counterparties no longer listed are logged out. What's in use until a restart can't change:
  the acceptor's identity, address and limits, a store's definition, and which store a
  counterparty's sessions are in.
- Stores of other kinds, such as `turbojet-sql`'s, are registered in code by name and named in
  the file.

The crate documentation lists every key. Features: `tls` (`[acceptor.tls]`), `tz` (time zones
in schedules), `validation` (dictionaries), `metrics` (`latency_metrics`).

Initiators are configured in code for now.
