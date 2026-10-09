# turbojet-config

Session configuration files for [Turbojet](https://github.com/jrogers/turbojet): an acceptor, its
counterparties, initiators and their stores, read from TOML and reloaded while running.

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
cancel_on_disconnect = "disconnect" # its orders cancelled if it drops and stays away
cancel_grace = "5s"

[initiator.LSE]                     # we log on to LSE, as VENUE
target_comp_id = "LSE"
connect = ["primary.lse:9876", "dr.lse:9876"]
username = "venue"
password_env = "LSE_PASSWORD"       # passwords come from the environment, not the file
```

```rust,ignore
let sessions = SessionsFile::load("sessions.toml")?;
let acceptor = sessions.acceptor(app.clone()).expect("the file has [acceptor]");
tokio::spawn(acceptor.clone().serve(TcpListener::bind(sessions.listen().unwrap()).await?));
let initiators = sessions.initiators(app)?;      // each [initiator] on its own task
// On SIGHUP, say:
let changes = sessions.reload_all(Some(&acceptor), &initiators)?;
```

- Each counterparty's settings are its own keys over `[defaults]`: schedule and holidays, rate
  limits, SendingTime tolerance, validation against a dictionary, the HeartBtInt it may ask for,
  whether it must present a TLS client certificate, cancel on disconnect, and the store its
  sessions are kept in.
- Unknown keys are errors, and every value is checked when the file loads, so a typo can't be
  silently ignored and a file that loads has nothing left to fail at a counterparty's Logon.
  Errors name the section and key.
- An initiator takes its counterparty, addresses (the primary, then failover), credentials,
  reconnect policy and TLS, and the same session keys over `[defaults]`. A file may hold only
  initiators.
- A reload checks the whole file first and keeps the one in use if anything fails. Changed
  settings apply from each counterparty's next Logon and each initiator's next connection, so a
  working session is never dropped for an edit; under `unknown = "refuse"`, connected
  counterparties no longer listed are logged out, and removed initiators are logged out and
  stopped. What's in use until a restart can't change:
  the acceptor's identity, address and limits, a store's definition, and which store a
  counterparty's sessions are in.
- Stores of other kinds, such as `turbojet-sql`'s, are registered in code by name and named in
  the file.

The crate documentation lists every key. Features: `tls` (`[acceptor.tls]`), `pkcs12` (a
certificate from a PKCS#12 bundle), `tz` (time zones in schedules), `validation` (dictionaries),
`metrics` (`latency_metrics`).
