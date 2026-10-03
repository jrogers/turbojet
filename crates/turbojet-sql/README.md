# turbojet-sql

SQL session storage for [Turbojet](https://github.com/jrogers/turbojet): each FIX session's
sequence numbers and sent messages in SQLite or PostgreSQL, through
[sqlx](https://docs.rs/sqlx), for deployments that can't rely on local disk or want session
state next to their other data.

```rust,no_run
use std::sync::Arc;
use turbojet_sql::{SqlConfig, SqlStorage};

# async fn example() -> std::io::Result<()> {
let storage = SqlStorage::connect("postgres://fix@db/sessions", SqlConfig::default()).await?;
storage.migrate().await?;
let registry = turbojet::SessionRegistry::new(Arc::new(storage));
# Ok(())
# }
```

- Opening a session, committing and reading messages for a resend all run off the connection's
  task, so a database round trip never blocks the runtime. Each commit is one transaction.
- Gateways sharing a database hold a lease on each session they run. A gateway whose lease
  another has taken can't commit, so it disconnects rather than send under numbers the other is
  using. The lease must be longer than the longest heartbeat interval.
- Each session keeps up to a byte budget of messages (1 GiB by default); a resend that reaches
  back past it gap-fills what went.

Features: `sqlite` (the default, bundled), `postgres`, and `tls` for TLS to PostgreSQL.
