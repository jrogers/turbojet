//! What a reload may change, what it changed, and who it logs out.

use std::collections::BTreeSet;

use tracing::{info, warn};
use turbojet::Acceptor;

use crate::Error;
use crate::load::Loaded;
use crate::raw::{RawAcceptor, Unknown};

/// What a reload changed: counterparties by CompID, initiators by section name.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Changes {
    /// Counterparties listed now that weren't before.
    pub added: Vec<String>,
    /// Counterparties listed before and now whose settings changed, their own or through
    /// `[defaults]`. The new settings apply from their next Logon.
    pub changed: Vec<String>,
    /// Counterparties listed before that aren't now.
    pub removed: Vec<String>,
    /// Connected counterparties logged out because the file no longer admits them.
    pub logged_out: Vec<String>,
    /// Initiators started: new ones, and those restarted for another session or TLS setting.
    pub started: Vec<String>,
    /// Initiators whose settings changed, applying from their next connection.
    pub reconfigured: Vec<String>,
    /// Initiators stopped (logging their sessions out): removed ones, and those restarted.
    pub stopped: Vec<String>,
}

impl Changes {
    /// Whether nothing changed for any listed counterparty or initiator.
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// Why `new` can't replace `old` while running, if it can't.
pub(crate) fn check(old: &Loaded, new: &Loaded) -> Result<(), Error> {
    match (&old.raw.acceptor, &new.raw.acceptor) {
        (Some(before), Some(after)) => check_acceptor(before, after)?,
        (None, None) => {}
        _ => return Err(Error::at("acceptor", "section", "can't be added or removed until a restart")),
    }
    for (name, store) in &old.raw.store {
        match new.raw.store.get(name) {
            Some(same) if same == store => {}
            Some(_) => return Err(Error::at(&format!("store {name}"), "kind", "can't change until a restart")),
            None => return Err(Error::at(&format!("store {name}"), "name", "can't be removed until a restart")),
        }
    }
    if old.acceptor.is_some() && new.acceptor.is_some() {
        check_counterparty_stores(old, new)?;
    }
    for (name, after) in &new.initiators {
        let before = old.initiators.values().find(|before| before.id() == after.id());
        if let Some(before) = before
            && before.store != after.store
        {
            return Err(Error::at(&format!("initiator {name}"), "store", moved(&before.store, &after.store)));
        }
    }
    Ok(())
}

fn check_counterparty_stores(old: &Loaded, new: &Loaded) -> Result<(), Error> {
    let comp_ids: BTreeSet<&String> = old.raw.counterparty.keys().chain(new.raw.counterparty.keys()).collect();
    for comp_id in comp_ids {
        if let (Some(before), Some(after)) = (store_name(old, comp_id), store_name(new, comp_id))
            && before != after
        {
            return Err(Error::at(&format!("counterparty {comp_id}"), "store", moved(before, after)));
        }
    }
    let (before, after) = (&old.acceptor().unlisted.store, &new.acceptor().unlisted.store);
    if (old.unknown(), new.unknown()) == (Unknown::Admit, Unknown::Admit) && before != after {
        return Err(Error::at("defaults", "store", moved(before, after)));
    }
    Ok(())
}

fn moved(before: &str, after: &str) -> String {
    format!("can't change from '{before}' to '{after}' until a restart: its sessions are in '{before}'")
}

/// The store counterparty `comp_id`'s sessions are kept in, if it may log on.
fn store_name<'a>(loaded: &'a Loaded, comp_id: &str) -> Option<&'a str> {
    let acceptor = loaded.acceptor();
    match acceptor.listed.get(comp_id) {
        Some(resolved) => Some(&resolved.store),
        None if loaded.unknown() == Unknown::Admit => Some(&acceptor.unlisted.store),
        None => None,
    }
}

fn check_acceptor(old: &RawAcceptor, new: &RawAcceptor) -> Result<(), Error> {
    let fixed = [
        ("begin_string", old.begin_string == new.begin_string),
        ("sender_comp_id", old.sender_comp_id == new.sender_comp_id),
        ("listen", old.listen == new.listen),
        ("logon_timeout", old.logon_timeout == new.logon_timeout),
        ("send_queue", old.send_queue == new.send_queue),
        ("max_connections", old.max_connections == new.max_connections),
        ("max_connections_per_ip", old.max_connections_per_ip == new.max_connections_per_ip),
        ("tls", old.tls.is_some() == new.tls.is_some()),
    ];
    match fixed.iter().find(|(_, same)| !same) {
        Some((key, _)) => Err(Error::at("acceptor", key, "can't change until a restart")),
        None => Ok(()),
    }
}

/// The counterparties added, changed and removed from `old` to `new`.
pub(crate) fn changes(old: &Loaded, new: &Loaded) -> Changes {
    let mut changes = Changes::default();
    for (comp_id, settings) in &new.raw.counterparty {
        match old.raw.counterparty.get(comp_id) {
            None => changes.added.push(comp_id.clone()),
            Some(before) if before.or(&old.raw.defaults) != settings.or(&new.raw.defaults) => {
                changes.changed.push(comp_id.clone());
            }
            Some(_) => {}
        }
    }
    changes.removed = old.raw.counterparty.keys().filter(|c| !new.raw.counterparty.contains_key(*c)).cloned().collect();
    changes
}

/// What a reload does to each initiator, by section name.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct InitiatorChanges {
    pub start: Vec<String>,
    pub reconfigure: Vec<String>,
    pub stop: Vec<String>,
}

/// Initiators to start, reconfigure and stop from `old` to `new`. One that now logs on to
/// another session, or switches TLS on or off, is stopped and started again: the session and
/// the connector are fixed when an initiator is made.
pub(crate) fn initiator_changes(old: &Loaded, new: &Loaded) -> InitiatorChanges {
    let mut changes = InitiatorChanges::default();
    for (name, after) in &new.initiators {
        let Some(before) = old.initiators.get(name) else {
            changes.start.push(name.clone());
            continue;
        };
        if before.id() != after.id() || before.uses_tls() != after.uses_tls() {
            changes.stop.push(name.clone());
            changes.start.push(name.clone());
            continue;
        }
        let (raw_before, raw_after) = (&old.raw.initiator[name], &new.raw.initiator[name]);
        let changed = raw_before.own != raw_after.own
            || raw_before.settings.or(&old.raw.defaults) != raw_after.settings.or(&new.raw.defaults);
        if changed {
            changes.reconfigure.push(name.clone());
        }
    }
    changes.stop.extend(old.initiators.keys().filter(|name| !new.initiators.contains_key(*name)).cloned());
    changes.stop.sort();
    changes
}

/// Logs out `acceptor`'s connected counterparties that `loaded` doesn't admit, returning their
/// CompIDs.
pub(crate) fn log_out_removed(loaded: &Loaded, acceptor: &Acceptor) -> Vec<String> {
    if loaded.unknown() == Unknown::Admit {
        return Vec::new();
    }
    let mut logged_out = Vec::new();
    for id in acceptor.sessions() {
        let comp_id = &id.target_comp_id;
        if loaded.acceptor().listed.contains_key(comp_id) {
            continue;
        }
        // By its full ID: a counterparty may have several sessions, one per SubID.
        match acceptor.handle(&id).logout(Some("no longer configured")) {
            Ok(()) => {
                info!(session = %id, "logging out a counterparty no longer configured");
                logged_out.push(comp_id.clone());
            }
            // It disconnected meanwhile: nothing to do.
            Err(e) => warn!(session = %id, "cannot log out a counterparty no longer configured: {e}"),
        }
    }
    logged_out.sort();
    logged_out.dedup();
    logged_out
}
