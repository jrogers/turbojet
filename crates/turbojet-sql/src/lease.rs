//! A lease on a whole instance in the database, for replication: which of an active and its
//! standby runs the sessions.

use std::io;
use std::sync::Arc;
use std::time::Duration;

use sqlx::Row;
use turbojet::store::Job;
use turbojet_replica::Lease;

use crate::Shared;
use crate::log::{database, millis, now_millis};

/// A [`Lease`] in `turbojet_leases`, under a name the instances sharing it agree on: whoever
/// holds it is the active. Taking it raises its epoch; renewing and giving it up are fenced on
/// the holder and the epoch, so an instance it was taken from can't renew it. Times come from
/// each instance's clock, as for the session leases.
#[derive(Clone)]
pub struct SqlLease {
    shared: Arc<Shared>,
    name: String,
    duration: Duration,
}

impl std::fmt::Debug for SqlLease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlLease").field("name", &self.name).field("duration", &self.duration).finish_non_exhaustive()
    }
}

impl SqlLease {
    pub(crate) fn new(shared: Arc<Shared>, name: &str) -> Self {
        let duration = shared.config.lease;
        Self { shared, name: name.into(), duration }
    }
}

impl Lease for SqlLease {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn acquire(&self) -> Job<u64> {
        let lease = self.clone();
        Job::future(async move {
            let Self { shared, name, duration } = &lease;
            let holder = &shared.config.holder;
            let now = now_millis();
            let mut tx = shared.pool.begin().await.map_err(database("taking a lease"))?;
            sqlx::query("INSERT INTO turbojet_leases (name) VALUES ($1) ON CONFLICT (name) DO NOTHING")
                .bind(name)
                .execute(&mut *tx)
                .await
                .map_err(database("creating a lease"))?;
            let taken = sqlx::query(
                "UPDATE turbojet_leases SET holder = $2, epoch = epoch + 1, lease_until = $3
                 WHERE name = $1 AND (holder IS NULL OR lease_until < $4 OR holder = $2)",
            )
            .bind(name)
            .bind(holder)
            .bind(now.saturating_add(millis(*duration)))
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(database("taking a lease"))?;
            let row = sqlx::query("SELECT holder, epoch, lease_until FROM turbojet_leases WHERE name = $1")
                .bind(name)
                .fetch_one(&mut *tx)
                .await
                .map_err(database("reading a lease"))?;
            if taken.rows_affected() == 0 {
                let holder: Option<String> = row.try_get("holder").map_err(database("reading a lease"))?;
                let until: Option<i64> = row.try_get("lease_until").map_err(database("reading a lease"))?;
                let left = Duration::from_millis(until.unwrap_or(now).saturating_sub(now).try_into().unwrap_or(0));
                return Err(io::Error::new(
                    io::ErrorKind::ResourceBusy,
                    format!("lease {name} is held by {} for another {left:?}", holder.as_deref().unwrap_or("another")),
                ));
            }
            let epoch: i64 = row.try_get("epoch").map_err(database("reading a lease"))?;
            tx.commit().await.map_err(database("committing a lease"))?;
            u64::try_from(epoch).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "a negative epoch"))
        })
    }

    fn renew(&self, epoch: u64) -> Job<()> {
        let lease = self.clone();
        Job::future(async move {
            let Self { shared, name, duration } = &lease;
            let renewed = sqlx::query(
                "UPDATE turbojet_leases SET lease_until = $3 WHERE name = $1 AND holder = $2 AND epoch = $4",
            )
            .bind(name)
            .bind(&shared.config.holder)
            .bind(now_millis().saturating_add(millis(*duration)))
            .bind(i64::try_from(epoch).unwrap_or(i64::MAX))
            .execute(&shared.pool)
            .await
            .map_err(database("renewing a lease"))?;
            if renewed.rows_affected() == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("lease {name} (epoch {epoch}) has been taken by another instance"),
                ));
            }
            Ok(())
        })
    }

    fn set_in_sync(&self, epoch: u64, in_sync: bool) -> Job<()> {
        let lease = self.clone();
        Job::future(async move {
            let Self { shared, name, .. } = &lease;
            let set =
                sqlx::query("UPDATE turbojet_leases SET in_sync = $3 WHERE name = $1 AND holder = $2 AND epoch = $4")
                    .bind(name)
                    .bind(&shared.config.holder)
                    .bind(i64::from(in_sync))
                    .bind(i64::try_from(epoch).unwrap_or(i64::MAX))
                    .execute(&shared.pool)
                    .await
                    .map_err(database("recording whether the standby is in sync"))?;
            if set.rows_affected() == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("lease {name} (epoch {epoch}) has been taken by another instance"),
                ));
            }
            Ok(())
        })
    }

    fn in_sync(&self) -> Job<bool> {
        let lease = self.clone();
        Job::future(async move {
            let Self { shared, name, .. } = &lease;
            let row = sqlx::query("SELECT in_sync FROM turbojet_leases WHERE name = $1")
                .bind(name)
                .fetch_optional(&shared.pool)
                .await
                .map_err(database("reading a lease"))?;
            let Some(row) = row else { return Ok(true) };
            let in_sync: i64 = row.try_get("in_sync").map_err(database("reading a lease"))?;
            Ok(in_sync != 0)
        })
    }

    fn release(&self, epoch: u64) -> Job<()> {
        let lease = self.clone();
        Job::future(async move {
            let Self { shared, name, .. } = &lease;
            sqlx::query(
                "UPDATE turbojet_leases SET holder = NULL, lease_until = NULL
                 WHERE name = $1 AND holder = $2 AND epoch = $3",
            )
            .bind(name)
            .bind(&shared.config.holder)
            .bind(i64::try_from(epoch).unwrap_or(i64::MAX))
            .execute(&shared.pool)
            .await
            .map_err(database("giving a lease up"))?;
            Ok(())
        })
    }
}
