//! What a reload may change, what it changed, and who it logs out.

use std::collections::BTreeSet;

use tracing::{info, warn};
use turbojet::Acceptor;

use crate::Error;
use crate::load::Loaded;
use crate::raw::{RawAcceptor, Unknown};

/// What a [reload](crate::SessionsFile::reload) changed, by counterparty CompID.
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
}

impl Changes {
    /// Whether nothing changed for any listed counterparty.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.changed.is_empty() && self.removed.is_empty() && self.logged_out.is_empty()
    }
}

/// Why `new` can't replace `old` while the acceptor runs, if it can't.
pub(crate) fn check(old: &Loaded, new: &Loaded) -> Result<(), Error> {
    check_acceptor(&old.raw.acceptor, &new.raw.acceptor)?;
    for (name, store) in &old.raw.store {
        match new.raw.store.get(name) {
            Some(same) if same == store => {}
            Some(_) => return Err(Error::at(&format!("store {name}"), "kind", "can't change until a restart")),
            None => return Err(Error::at(&format!("store {name}"), "name", "can't be removed until a restart")),
        }
    }
    let comp_ids: BTreeSet<&String> = old.raw.counterparty.keys().chain(new.raw.counterparty.keys()).collect();
    for comp_id in comp_ids {
        let section = format!("counterparty {comp_id}");
        if let (Some(before), Some(after)) = (store_name(old, comp_id), store_name(new, comp_id))
            && before != after
        {
            return Err(Error::at(&section, "store", moved(before, after)));
        }
    }
    if let (Unknown::Admit, Unknown::Admit) = (old.raw.acceptor.unknown, new.raw.acceptor.unknown)
        && old.unlisted.store != new.unlisted.store
    {
        return Err(Error::at("defaults", "store", moved(&old.unlisted.store, &new.unlisted.store)));
    }
    Ok(())
}

fn moved(before: &str, after: &str) -> String {
    format!("can't change from '{before}' to '{after}' until a restart: its sessions are in '{before}'")
}

/// The store counterparty `comp_id`'s sessions are kept in, if it may log on.
fn store_name<'a>(loaded: &'a Loaded, comp_id: &str) -> Option<&'a str> {
    match loaded.listed.get(comp_id) {
        Some(resolved) => Some(&resolved.store),
        None if loaded.raw.acceptor.unknown == Unknown::Admit => Some(&loaded.unlisted.store),
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

/// Logs out `acceptor`'s connected counterparties that `loaded` doesn't admit, returning their
/// CompIDs.
pub(crate) fn log_out_removed(loaded: &Loaded, acceptor: &Acceptor) -> Vec<String> {
    if loaded.raw.acceptor.unknown == Unknown::Admit {
        return Vec::new();
    }
    let mut logged_out = Vec::new();
    for id in acceptor.sessions() {
        let comp_id = &id.target_comp_id;
        if loaded.listed.contains_key(comp_id) {
            continue;
        }
        match acceptor.session(comp_id).logout(Some("no longer configured")) {
            Ok(()) => {
                info!(session = %id, "logging out a counterparty no longer configured");
                logged_out.push(comp_id.clone());
            }
            // It disconnected meanwhile: nothing to do.
            Err(e) => warn!(session = %id, "cannot log out a counterparty no longer configured: {e}"),
        }
    }
    logged_out.sort();
    logged_out
}
