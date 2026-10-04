//! Session configuration files for [Turbojet](https://docs.rs/turbojet): an acceptor, its
//! counterparties and their stores, read from TOML and reloaded while running.
//!
//! ```toml
//! [acceptor]
//! begin_string = "FIX.4.4"
//! sender_comp_id = "VENUE"
//! listen = "0.0.0.0:9876"
//! unknown = "refuse"                  # or "admit": unlisted counterparties get [defaults]
//!
//! [store.main]
//! kind = "disk"
//! dir = "store"                       # relative to this file
//!
//! [defaults]
//! store = "main"
//! schedule = "daily 08:00-17:00"
//!
//! [counterparty.BROKER]
//! outbound_limit = "100/1s"
//!
//! [counterparty.FUND]
//! require_client_certificate = true
//! heartbeat = { min = "10s", max = "60s" }
//! ```
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use turbojet_config::SessionsFile;
//! # struct App;
//! # impl turbojet::Application for App {}
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let sessions = SessionsFile::load("sessions.toml")?;
//! let acceptor = sessions.acceptor(Arc::new(App));
//! let listener = tokio::net::TcpListener::bind(sessions.listen()).await?;
//! tokio::spawn(acceptor.clone().serve(listener));
//! // Later, on SIGHUP say:
//! let changes = sessions.reload(&acceptor)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Keys
//!
//! `[acceptor]`, fixed until a restart except `unknown` and the TLS files: `begin_string`,
//! `sender_comp_id`, `listen`, `unknown` (`refuse`, the default, or `admit`), `logon_timeout`,
//! `send_queue`, `max_connections`, `max_connections_per_ip`, and `tls = { cert, key, client_ca,
//! client_certificate }` (feature `tls`; `client_certificate` is `optional`, the default, or
//! `required`).
//!
//! `[store.NAME]`: `kind = "memory"`, or `kind = "disk"` with `dir` and `fsync` (on by default).
//! A store named `memory` is built in, and others can be registered in code
//! ([`SessionsFileBuilder::with_store`]), such as a `turbojet-sql` store.
//!
//! `[defaults]` and `[counterparty.COMPID]`, a counterparty's keys taking the place of the
//! defaults' (a list whole): `store` (`memory` by default), `logout_timeout`, `schedule` (as
//! [`SessionSchedule`](turbojet::SessionSchedule) parses it), `holidays` (a file of dates),
//! `max_latency` (a duration or `off`), `check_orig_sending_time`, `check_header_order`,
//! `timestamp_precision` (`seconds`, `millis`, `micros` or `nanos`), `data_fields` (`[[length,
//! data], ...]`), `outbound_limit` and `inbound_limit` (`100/1s` or `off`), `over_limit`
//! (`delay`, the default, or `reject`), `appl_versions` (ApplVerID codes, or `{ id, dictionary
//! }`), `dictionary` (feature `validation`), `latency_metrics` (feature `metrics`), `heartbeat =
//! { min, max }` (the HeartBtInt a counterparty may ask for), and `require_client_certificate`.
//! Durations are a whole number and `ms`, `s`, `m` or `h`. Paths are relative to the file.
//!
//! Unknown keys are errors, and every value is checked when the file loads, so a file that loads
//! has nothing left to fail at a counterparty's Logon. The `[defaults]` are also the acceptor's
//! own settings, which the Logon itself is checked against (its SendingTime, say) before the
//! counterparty is known: those of the file as it was when the acceptor was made.
//!
//! # Reloading
//!
//! [`reload`](SessionsFile::reload) reads the file again and checks it whole; a file that
//! doesn't load leaves the one in use in place. Changed settings apply from each counterparty's
//! next Logon; sessions logged on keep theirs. Under `unknown = "refuse"`, connected counterparties
//! the file no longer lists are logged out. What's fixed until a restart can't change: the
//! `[acceptor]` keys other than `unknown` and the TLS files, a store's definition, and which store
//! a counterparty's sessions are kept in.

mod load;
mod raw;
mod reload;

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use turbojet::store::{Opened, SessionLog};
use turbojet::{
    Acceptor, Application, Clock, ConnectionInfo, Counterparties, Counterparty, Message, SessionConfig, SessionId,
    SessionStorage,
};

use crate::load::{Context, Loaded};
pub use crate::raw::Unknown;
pub use crate::reload::Changes;

/// Why a sessions file can't be used: where in it, and what's wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    /// A problem with `key` in `section` (`acceptor`, `defaults`, `counterparty BROKER`).
    pub(crate) fn at(section: &str, key: &str, problem: impl fmt::Display) -> Self {
        Self { message: format!("{section}: {key}: {problem}") }
    }

    /// A problem with the file as a whole.
    pub(crate) fn file(problem: impl fmt::Display) -> Self {
        Self { message: problem.to_string() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// A sessions file, loaded: makes the [`Acceptor`] it describes and reloads it. Cheap to share
/// by reference; the acceptor reads the settings current at each Logon.
pub struct SessionsFile {
    path: PathBuf,
    clock: Clock,
    registered: HashMap<String, Arc<dyn SessionStorage>>,
    current: Arc<Current>,
    /// Held while reloading, so two reloads don't interleave.
    reloading: Mutex<()>,
    #[cfg(feature = "tls")]
    server_tls: Mutex<Option<turbojet::tls::ServerTls>>,
}

/// The file in use, swapped whole by a reload.
struct Current(RwLock<Arc<Loaded>>);

impl Current {
    fn get(&self) -> Arc<Loaded> {
        self.0.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    fn set(&self, loaded: Loaded) {
        *self.0.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::new(loaded);
    }
}

/// Loads a sessions file with stores registered in code, or a clock other than the system's.
pub struct SessionsFileBuilder {
    path: PathBuf,
    clock: Clock,
    registered: HashMap<String, Arc<dyn SessionStorage>>,
}

impl SessionsFileBuilder {
    /// Makes `storage` available to the file as store `name`, e.g. a `turbojet-sql` store.
    #[must_use]
    pub fn with_store(mut self, name: impl Into<String>, storage: Arc<dyn SessionStorage>) -> Self {
        self.registered.insert(name.into(), storage);
        self
    }

    /// Gives every session `clock` (the system clock by default), as
    /// [`SessionConfig::clock`] does.
    #[must_use]
    pub fn with_clock(mut self, clock: Clock) -> Self {
        self.clock = clock;
        self
    }

    /// Reads and checks the file.
    ///
    /// # Errors
    ///
    /// If it can't be read, or doesn't load: see [`Error`].
    pub fn load(self) -> Result<SessionsFile, Error> {
        let loaded = read(&self.path, &self.clock, &self.registered, None)?;
        Ok(SessionsFile {
            path: self.path,
            clock: self.clock,
            registered: self.registered,
            current: Arc::new(Current(RwLock::new(Arc::new(loaded)))),
            reloading: Mutex::new(()),
            #[cfg(feature = "tls")]
            server_tls: Mutex::new(None),
        })
    }
}

/// Reads and loads the file at `path`, keeping the stores of `previous`, if any.
fn read(
    path: &Path,
    clock: &Clock,
    registered: &HashMap<String, Arc<dyn SessionStorage>>,
    previous: Option<&Loaded>,
) -> Result<Loaded, Error> {
    let text = std::fs::read_to_string(path).map_err(|e| Error::file(format!("{}: {e}", path.display())))?;
    let context = Context {
        dir: path.parent().unwrap_or(Path::new(".")),
        clock,
        registered,
        previous: previous.map(|p| &p.stores),
        previous_defined: previous.map(|p| &p.raw.store),
    };
    load::parse(&text, &context).map_err(|e| Error::file(format!("{}: {e}", path.display())))
}

impl SessionsFile {
    /// Reads and checks the file at `path`.
    ///
    /// # Errors
    ///
    /// If it can't be read, or doesn't load: see [`Error`].
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, Error> {
        Self::builder(path).load()
    }

    /// Loads the file at `path` with stores registered in code, or another clock.
    pub fn builder(path: impl Into<PathBuf>) -> SessionsFileBuilder {
        SessionsFileBuilder { path: path.into(), clock: Clock::system(), registered: HashMap::new() }
    }

    /// The address to listen on: `[acceptor] listen`.
    pub fn listen(&self) -> String {
        self.current.get().raw.acceptor.listen.clone()
    }

    /// What happens to a counterparty the file doesn't list.
    pub fn unknown(&self) -> Unknown {
        self.current.get().raw.acceptor.unknown
    }

    /// The counterparties the file lists, by CompID, in order.
    pub fn counterparties(&self) -> Vec<String> {
        self.current.get().raw.counterparty.keys().cloned().collect()
    }

    /// The acceptor's own configuration: `[acceptor]` and `[defaults]`.
    pub fn base(&self) -> SessionConfig {
        self.current.get().base.clone()
    }

    /// An acceptor serving `app` as the file says: its counterparties' settings and stores, and
    /// its connection limits. Serve it on [`listen`](Self::listen), with
    /// [`server_tls`](Self::server_tls) if the file has `[acceptor.tls]`.
    pub fn acceptor(&self, app: Arc<dyn Application>) -> Acceptor {
        let loaded = self.current.get();
        let storage = Arc::new(Router(self.current.clone()));
        let mut acceptor = Acceptor::new(loaded.base.clone(), storage, app)
            .with_counterparties(Arc::new(Resolver(self.current.clone())));
        if let Some(connections) = loaded.raw.acceptor.max_connections {
            acceptor = acceptor.with_max_connections(connections);
        }
        if let Some(connections) = loaded.raw.acceptor.max_connections_per_ip {
            acceptor = acceptor.with_max_connections_per_ip(connections);
        }
        acceptor
    }

    /// TLS for [`Acceptor::serve_tls`] if the file has `[acceptor.tls]`; reloading the file
    /// replaces its certificates. Every call returns the same one (clones share it).
    ///
    /// # Errors
    ///
    /// If the certificates can't be used; loading the file checked them, so only if they changed
    /// since.
    #[cfg(feature = "tls")]
    pub fn server_tls(&self) -> io::Result<Option<turbojet::tls::ServerTls>> {
        let mut server = self.server_tls.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if server.is_none()
            && let Some((identity, client_trust)) = self.current.get().tls.clone()
        {
            *server = Some(turbojet::tls::ServerTls::new(identity, client_trust)?);
        }
        Ok(server.clone())
    }

    /// Reads the file again and puts it in use for `acceptor` (made by
    /// [`acceptor`](Self::acceptor)): see [Reloading](crate#reloading).
    ///
    /// # Errors
    ///
    /// If the file can't be read, doesn't load, or changes what's fixed until a restart; the
    /// file in use stays in use.
    pub fn reload(&self, acceptor: &Acceptor) -> Result<Changes, Error> {
        let _reloading = self.reloading.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let old = self.current.get();
        let new = read(&self.path, &self.clock, &self.registered, Some(&old))?;
        reload::check(&old, &new).map_err(|e| Error::file(format!("{}: {e}", self.path.display())))?;
        #[cfg(feature = "tls")]
        self.replace_certificates(&new)?;
        let mut changes = reload::changes(&old, &new);
        self.current.set(new);
        changes.logged_out = reload::log_out_removed(&self.current.get(), acceptor);
        Ok(changes)
    }

    /// Puts `new`'s TLS certificates in use, if TLS is being served.
    #[cfg(feature = "tls")]
    fn replace_certificates(&self, new: &Loaded) -> Result<(), Error> {
        let server = self.server_tls.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let (Some(server), Some((identity, client_trust))) = (server.as_ref(), new.tls.clone()) {
            server.set_client_trust(client_trust).map_err(|e| Error::at("acceptor", "tls", e))?;
            server.set_identity(identity);
        }
        Ok(())
    }
}

impl fmt::Debug for SessionsFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionsFile").field("path", &self.path).finish_non_exhaustive()
    }
}

/// Each counterparty's settings, from the file current at its Logon.
struct Resolver(Arc<Current>);

impl Counterparties for Resolver {
    fn resolve(
        &self,
        _: &SessionConfig,
        id: &SessionId,
        _: &Message,
        _: &ConnectionInfo,
    ) -> Result<Counterparty, String> {
        let loaded = self.0.get();
        let comp_id = &id.target_comp_id;
        match loaded.listed.get(comp_id) {
            Some(resolved) => Ok(resolved.counterparty.clone()),
            None if loaded.raw.acceptor.unknown == Unknown::Admit => Ok(loaded.unlisted.counterparty.clone()),
            None => Err(format!("unknown counterparty '{comp_id}'")),
        }
    }
}

/// Opens each session's log in its counterparty's store, as the file current then says.
struct Router(Arc<Current>);

impl SessionStorage for Router {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        self.0.get().store_for(&id.target_comp_id).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.0.get().store_for(&id.target_comp_id).begin_open(id)
    }
}
