//! Session configuration files for [Turbojet](https://docs.rs/turbojet): an acceptor, its
//! counterparties, initiators and their stores, read from TOML and reloaded while running.
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
//!
//! [initiator.LSE]                     # we log on to LSE, as VENUE (the acceptor's CompID)
//! target_comp_id = "LSE"
//! connect = ["primary.lse:9876", "dr.lse:9876"]
//! username = "venue"
//! password_env = "LSE_PASSWORD"
//! ```
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use turbojet_config::SessionsFile;
//! # struct App;
//! # impl turbojet::Application for App {}
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let sessions = SessionsFile::load("sessions.toml")?;
//! let acceptor = sessions.acceptor(Arc::new(App)).expect("the file has [acceptor]");
//! let listener = tokio::net::TcpListener::bind(sessions.listen().unwrap()).await?;
//! tokio::spawn(acceptor.clone().serve(listener));
//! let initiators = sessions.initiators(Arc::new(App))?;
//! // Later, on SIGHUP say:
//! let changes = sessions.reload_all(Some(&acceptor), &initiators)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Keys
//!
//! `[acceptor]`, fixed until a restart except `unknown` and the TLS files: `begin_string`,
//! `sender_comp_id`, `listen`, `unknown` (`refuse`, the default, or `admit`), `logon_timeout`,
//! `send_queue`, `max_connections`, `max_connections_per_ip`, and `tls = { cert, key, client_ca,
//! client_crl, client_certificate }` (feature `tls`; `client_certificate` is `optional`, the
//! default, or `required`; `client_crl`, a PEM file of CRLs, checks client certificates for
//! revocation).
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
//! { min, max }` (the HeartBtInt a counterparty may ask for), `require_client_certificate`,
//! `resend_request_chunk` (the most messages one ResendRequest asks for),
//! `max_sessions_per_counterparty` (how many sessions, told apart by SubIDs and LocationIDs, one
//! counterparty may have connected at once; 16 by default),
//! `cancel_on_disconnect` (`off`, the default, `disconnect`, or `disconnect_or_logout`, under
//! which Logouts from either side count too; see [`CancelOnDisconnect`](turbojet::CancelOnDisconnect)) and
//! `cancel_grace` (how long the counterparty has to log back on before the cancel: `0s`, the
//! default, up to `1h`; a counterparty's or initiator's own needs `cancel_on_disconnect` on,
//! while one in `[defaults]` serves those that turn it on). The application's
//! [`on_cancel_on_disconnect`](turbojet::Application::on_cancel_on_disconnect) does the
//! cancelling. Durations are a whole number and `ms`, `s`, `m` or `h`. Paths are relative to
//! the file.
//!
//! `[initiator.NAME]`, NAME being for logs and errors: `target_comp_id`, `connect` (addresses,
//! the primary first, then failover), `sender_sub_id`, `sender_location_id`, `target_sub_id`,
//! `target_location_id` and `qualifier` (part of the session's ID, the first four sent on every
//! message; see [`SessionId`]), `begin_string` and `sender_comp_id` (the acceptor's by default;
//! needed without one), `heartbeat_interval`, `reset_on_logon`,
//! `next_expected_msg_seq_num`, `username`, `password_env` (the environment variable holding the
//! password: a password isn't kept in the file), `connect_timeout`, `local_address` (the IP address
//! to connect from, with or without a port), `logon_timeout`, `send_queue`,
//! `reconnect = { initial, max, multiplier, jitter }`, `tls = { ca, crl, cert, key, server_name }`
//! (feature `tls`; `server_name` is the first address's host by default; `crl`, a PEM file of
//! CRLs, checks the server's certificate for revocation), and every session key
//! above but `heartbeat` and `require_client_certificate`, over `[defaults]`. No two initiators
//! may log on to one session, nor to one the acceptor serves for a listed counterparty.
//!
//! Unknown keys are errors, and every value is checked when the file loads, so a file that loads
//! has nothing left to fail at a counterparty's Logon. The `[defaults]` are also the acceptor's
//! own settings, which the Logon itself is checked against (its SendingTime, say) before the
//! counterparty is known: those of the file as it was when the acceptor was made.
//!
//! # Reloading
//!
//! [`reload_all`](SessionsFile::reload_all) (or [`reload`](SessionsFile::reload), for a file
//! without initiators) reads the file again and checks it whole; a file that doesn't load leaves
//! the one in use in place. Changed settings apply from each counterparty's next Logon, and from
//! each initiator's next connection: sessions connected keep theirs. Under `unknown = "refuse"`,
//! connected counterparties the file no longer lists are logged out. Added initiators start, and
//! removed ones are logged out and stopped; one that now logs on to another session, or turns TLS
//! on or off, is stopped and started again. Stopping an initiator, removed or restarted, fires its
//! cancel-on-disconnect countdown, if one is under way, at once rather than after its grace.
//! What's fixed until a restart can't change: whether there's an `[acceptor]`, its keys other
//! than `unknown` and the TLS files, a store's definition, and which store a counterparty's or an
//! initiator's sessions are kept in.

mod initiators;
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
    Acceptor, Application, Clock, ConnectionInfo, Counterparties, Counterparty, Message, MessageLog, SessionConfig,
    SessionId, SessionRegistry, SessionStorage,
};

pub use crate::initiators::Initiators;
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

/// A sessions file, loaded: makes the [`Acceptor`] and [`Initiators`] it describes, and reloads
/// them. The acceptor reads the settings current at each Logon.
pub struct SessionsFile {
    path: PathBuf,
    clock: Clock,
    message_log: Option<Arc<dyn MessageLog>>,
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

/// Loads a sessions file with stores registered in code, a clock other than the system's, or a
/// message log.
pub struct SessionsFileBuilder {
    path: PathBuf,
    clock: Clock,
    message_log: Option<Arc<dyn MessageLog>>,
    registered: HashMap<String, Arc<dyn SessionStorage>>,
}

impl fmt::Debug for SessionsFileBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionsFileBuilder")
            .field("path", &self.path)
            .field("stores", &self.registered.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
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

    /// Gives every session `log`, as [`SessionConfig::message_log`] does.
    #[must_use]
    pub fn with_message_log(mut self, log: Arc<dyn MessageLog>) -> Self {
        self.message_log = Some(log);
        self
    }

    /// Reads and checks the file.
    ///
    /// # Errors
    ///
    /// If it can't be read, or doesn't load: see [`Error`].
    pub fn load(self) -> Result<SessionsFile, Error> {
        let loaded = read(&self.path, &self.clock, self.message_log.as_ref(), &self.registered, None)?;
        Ok(SessionsFile {
            path: self.path,
            clock: self.clock,
            message_log: self.message_log,
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
    message_log: Option<&Arc<dyn MessageLog>>,
    registered: &HashMap<String, Arc<dyn SessionStorage>>,
    previous: Option<&Loaded>,
) -> Result<Loaded, Error> {
    let text = std::fs::read_to_string(path).map_err(|e| Error::file(format!("{}: {e}", path.display())))?;
    let context = Context {
        dir: path.parent().unwrap_or(Path::new(".")),
        clock,
        message_log,
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

    /// Loads the file at `path` with stores registered in code, another clock or a message log.
    pub fn builder(path: impl Into<PathBuf>) -> SessionsFileBuilder {
        SessionsFileBuilder { path: path.into(), clock: Clock::system(), message_log: None, registered: HashMap::new() }
    }

    /// The address to listen on, `[acceptor] listen`, if the file has an acceptor.
    pub fn listen(&self) -> Option<String> {
        self.current.get().raw.acceptor.as_ref().map(|acceptor| acceptor.listen.clone())
    }

    /// What happens to a counterparty the file doesn't list.
    pub fn unknown(&self) -> Unknown {
        self.current.get().unknown()
    }

    /// The counterparties the file lists, by CompID, in order.
    pub fn counterparties(&self) -> Vec<String> {
        self.current.get().raw.counterparty.keys().cloned().collect()
    }

    /// The initiators the file lists, by section name, in order.
    pub fn initiator_names(&self) -> Vec<String> {
        self.current.get().raw.initiator.keys().cloned().collect()
    }

    /// The acceptor's own configuration, `[acceptor]` and `[defaults]`, if the file has one.
    pub fn base(&self) -> Option<SessionConfig> {
        self.current.get().acceptor.as_ref().map(|acceptor| acceptor.base.clone())
    }

    /// An acceptor serving `app` as the file says, if it has `[acceptor]`: its counterparties'
    /// settings and stores, and its connection limits. Serve it on [`listen`](Self::listen), with
    /// [`server_tls`](Self::server_tls) if the file has `[acceptor.tls]`.
    pub fn acceptor(&self, app: Arc<dyn Application>) -> Option<Acceptor> {
        let loaded = self.current.get();
        let (raw, part) = (loaded.raw.acceptor.as_ref()?, loaded.acceptor.as_ref()?);
        let storage = Arc::new(Router(self.current.clone()));
        let mut acceptor = Acceptor::new(part.base.clone(), storage, app)
            .expect("checked when the file loaded")
            .with_counterparties(Arc::new(Resolver(self.current.clone())));
        if let Some(connections) = raw.max_connections {
            acceptor = acceptor.with_max_connections(connections);
        }
        if let Some(connections) = raw.max_connections_per_ip {
            acceptor = acceptor.with_max_connections_per_ip(connections);
        }
        Some(acceptor)
    }

    /// Starts every `[initiator.NAME]`, each on its own task, serving `app`. Call it within a
    /// tokio runtime.
    ///
    /// # Errors
    ///
    /// If an initiator's TLS can't be set up; loading the file checked it, so only if its files
    /// changed since.
    pub fn initiators(&self, app: Arc<dyn Application>) -> Result<Initiators, Error> {
        let storage = Arc::new(InitiatorStores(self.current.clone()));
        let registry = Arc::new(SessionRegistry::new(storage).with_clock(self.clock.clone()));
        let initiators = Initiators::new(app, registry);
        for (name, initiator) in &self.current.get().initiators {
            initiators.start(name, initiator)?;
        }
        Ok(initiators)
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
        let tls = self.current.get().acceptor.as_ref().and_then(|acceptor| acceptor.tls.clone());
        if server.is_none()
            && let Some((identity, client_trust)) = tls
        {
            *server = Some(turbojet::tls::ServerTls::new(identity, client_trust)?);
        }
        Ok(server.clone())
    }

    /// Reads the file again and puts it in use for `acceptor` (made by
    /// [`acceptor`](Self::acceptor)): see [Reloading](crate#reloading). For a file with
    /// initiators, use [`reload_all`](Self::reload_all).
    ///
    /// # Errors
    ///
    /// If the file can't be read, doesn't load, changes what's fixed until a restart, or has
    /// (or had) initiators; the file in use stays in use.
    pub fn reload(&self, acceptor: &Acceptor) -> Result<Changes, Error> {
        self.reload_with(Some(acceptor), None)
    }

    /// Reads the file again and puts it in use for `acceptor` (if one was made) and `initiators`
    /// (made by [`initiators`](Self::initiators)): see [Reloading](crate#reloading).
    ///
    /// # Errors
    ///
    /// If the file can't be read, doesn't load, or changes what's fixed until a restart; the file
    /// in use stays in use.
    pub fn reload_all(&self, acceptor: Option<&Acceptor>, initiators: &Initiators) -> Result<Changes, Error> {
        self.reload_with(acceptor, Some(initiators))
    }

    fn reload_with(&self, acceptor: Option<&Acceptor>, initiators: Option<&Initiators>) -> Result<Changes, Error> {
        let _reloading = self.reloading.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let in_file = |e: Error| Error::file(format!("{}: {e}", self.path.display()));
        let old = self.current.get();
        let new = read(&self.path, &self.clock, self.message_log.as_ref(), &self.registered, Some(&old))?;
        reload::check(&old, &new).map_err(in_file)?;
        if initiators.is_none() && !(old.initiators.is_empty() && new.initiators.is_empty()) {
            return Err(in_file(Error::at("initiator", "section", "reload a file with initiators with reload_all")));
        }
        #[cfg(feature = "tls")]
        self.replace_certificates(&new)?;
        let mut changes = reload::changes(&old, &new);
        let started = reload::initiator_changes(&old, &new);
        self.current.set(new);
        let new = self.current.get();
        if let (Some(acceptor), Some(_)) = (acceptor, &new.acceptor) {
            changes.logged_out = reload::log_out_removed(&new, acceptor);
        }
        if let Some(initiators) = initiators {
            initiators.apply(&started, &new, &mut changes);
        }
        Ok(changes)
    }

    /// Puts `new`'s TLS certificates in use, if TLS is being served.
    #[cfg(feature = "tls")]
    fn replace_certificates(&self, new: &Loaded) -> Result<(), Error> {
        let server = self.server_tls.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let tls = new.acceptor.as_ref().and_then(|acceptor| acceptor.tls.clone());
        if let (Some(server), Some((identity, client_trust))) = (server.as_ref(), tls) {
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
        let (acceptor, comp_id) = (loaded.acceptor(), &id.target_comp_id);
        match acceptor.listed.get(comp_id) {
            Some(resolved) => Ok(resolved.counterparty.clone()),
            None if loaded.unknown() == Unknown::Admit => Ok(acceptor.unlisted.counterparty.clone()),
            None => Err(format!("unknown counterparty '{comp_id}'")),
        }
    }
}

/// Opens each session's log in its counterparty's store, as the file current then says.
struct Router(Arc<Current>);

impl SessionStorage for Router {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        self.0.get().counterparty_store(&id.target_comp_id).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.0.get().counterparty_store(&id.target_comp_id).begin_open(id)
    }
}

/// Opens each initiator's session log in its store, as the file current then says.
struct InitiatorStores(Arc<Current>);

impl SessionStorage for InitiatorStores {
    fn open(&self, id: &SessionId) -> io::Result<Box<dyn SessionLog>> {
        self.0.get().initiator_store(id).open(id)
    }

    fn begin_open(&self, id: &SessionId) -> io::Result<Opened> {
        self.0.get().initiator_store(id).begin_open(id)
    }
}
