//! `turbojet.Engine`: the initiators a sessions file lists, run on the engine's tokio runtime, and
//! the events they raise, drained from asyncio.

use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyStopAsyncIteration};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::{future_into_py, get_runtime};
use tokio::sync::{mpsc, watch};
use turbojet::{Application, CommandError, SendError, SessionHandle, SessionId};
use turbojet_config::{Initiators, SessionsFile};

use crate::message::PyMessage;

create_exception!(turbojet, Error, PyException, "Every error the engine raises.");
create_exception!(turbojet, ConfigError, Error, "The sessions file can't be used.");
create_exception!(turbojet, NotConnected, Error, "No connection has the session.");
create_exception!(turbojet, QueueFull, Error, "The session's send or control queue is full.");
create_exception!(turbojet, Dropped, Error, "A queued message was never stored or sent.");

/// What a callback tells Python, on its way through the channel. Python objects are made only
/// when Python takes it, so a session's task never needs the GIL.
enum Event {
    Logon(SessionHandle),
    Logout(SessionHandle, turbojet::Disconnect),
}

impl Event {
    fn into_py(self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(match self {
            Event::Logon(session) => Py::new(py, Logon { session: Py::new(py, Session(session))? })?.into_any(),
            Event::Logout(session, ended) => {
                Py::new(py, Logout { session: Py::new(py, Session(session))?, ended: ended.into() })?.into_any()
            }
        })
    }
}

/// The application the engine calls: it only passes events on. A closed channel means Python has
/// stopped listening, and the event goes nowhere.
struct Bridge(mpsc::UnboundedSender<Event>);

impl Application for Bridge {
    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.0.send(Event::Logon(session.clone()));
    }

    fn on_logout(&self, session: &SessionHandle, ended: turbojet::Disconnect) {
        let _ = self.0.send(Event::Logout(session.clone(), ended));
    }
}

/// Loads the sessions file at `path`: its initiators, run when the engine is entered with
/// `async with`.
#[pyfunction]
pub fn load(path: PathBuf) -> PyResult<Engine> {
    let file = SessionsFile::load(path).map_err(|e| ConfigError::new_err(e.to_string()))?;
    if file.listen().is_some() {
        return Err(ConfigError::new_err("the Python bindings run initiators only: remove [acceptor]"));
    }
    let (sender, events) = mpsc::unbounded_channel();
    Ok(Engine {
        file,
        sender: Mutex::new(Some(sender)),
        initiators: Mutex::new(None),
        events: Arc::new(tokio::sync::Mutex::new(events)),
        closed: watch::Sender::new(false),
    })
}

/// The initiators a sessions file lists. `async with` starts them and, on the way out, logs them
/// out and stops them; `events()` gives what they raise meanwhile.
#[pyclass(frozen, module = "turbojet")]
pub struct Engine {
    file: SessionsFile,
    /// The bridge's end of the channel, until the engine starts.
    sender: Mutex<Option<mpsc::UnboundedSender<Event>>>,
    initiators: Mutex<Option<Arc<Initiators>>>,
    events: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Event>>>,
    /// Set once the engine has shut down: events then end when the channel is empty.
    closed: watch::Sender<bool>,
}

#[pymethods]
impl Engine {
    fn __aenter__<'py>(slf: Bound<'py, Self>) -> PyResult<Bound<'py, PyAny>> {
        let this = slf.get();
        let sender = lock(&this.sender).take().ok_or_else(|| Error::new_err("the engine has already started"))?;
        let initiators = {
            let _runtime = get_runtime().enter();
            this.file.initiators(Arc::new(Bridge(sender))).map_err(|e| ConfigError::new_err(e.to_string()))?
        };
        *lock(&this.initiators) = Some(Arc::new(initiators));
        let engine = slf.clone().unbind();
        future_into_py(slf.py(), async move { Ok(engine) })
    }

    #[pyo3(signature = (*_exc))]
    fn __aexit__<'py>(&self, py: Python<'py>, _exc: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let initiators = lock(&self.initiators).take();
        let closed = self.closed.clone();
        future_into_py(py, async move {
            if let Some(initiators) = initiators {
                initiators.shutdown(None).await;
            }
            closed.send_replace(true);
            Ok(false)
        })
    }

    /// The initiators' events, in the order they happened, until the engine shuts down.
    fn events(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let events = self.events.clone();
        let mut closed = self.closed.subscribe();
        future_into_py(py, async move {
            let mut events = events.lock().await;
            let event = tokio::select! {
                biased;
                event = events.recv() => event,
                _ = closed.wait_for(|closed| *closed) => events.try_recv().ok(),
            };
            let Some(event) = event else { return Err(PyStopAsyncIteration::new_err(())) };
            Python::attach(|py| event.into_py(py))
        })
    }

    /// The initiators the file lists, by section name.
    fn sessions(&self) -> Vec<String> {
        self.file.initiator_names()
    }

    /// The session initiator `name` logs on to, once the engine has started.
    fn session(&self, name: &str) -> PyResult<Option<Session>> {
        let initiators = lock(&self.initiators);
        let initiators = initiators.as_ref().ok_or_else(|| Error::new_err("the engine isn't running"))?;
        Ok(initiators.handle(name).map(Session))
    }
}

/// One FIX session: to send on it and control it. It stays valid across reconnects.
#[pyclass(frozen, module = "turbojet")]
pub struct Session(SessionHandle);

#[pymethods]
impl Session {
    #[getter]
    fn id(&self) -> PySessionId {
        PySessionId(self.0.id().clone())
    }

    /// Whether a connection has the session.
    #[getter]
    fn connected(&self) -> bool {
        self.0.is_connected()
    }

    /// Sends `msg`, waiting for room in the send queue, and returns the MsgSeqNum it was stored
    /// with.
    fn send<'py>(&self, py: Python<'py>, msg: PyRef<'py, PyMessage>) -> PyResult<Bound<'py, PyAny>> {
        let (session, msg) = (self.0.clone(), msg.0.clone());
        future_into_py(py, async move {
            let receipt = session.send_when_ready(msg).await.map_err(send_error)?;
            receipt.await.map_err(|dropped| Dropped::new_err(dropped.to_string()))
        })
    }

    /// Queues `msg` without waiting: `QueueFull` if there's no room.
    fn send_nowait(&self, msg: PyRef<'_, PyMessage>) -> PyResult<()> {
        self.0.send(msg.0.clone()).map(drop).map_err(send_error)
    }

    /// Starts an orderly logout, once the messages already queued have gone.
    #[pyo3(signature = (text = None))]
    fn logout(&self, text: Option<&str>) -> PyResult<()> {
        self.0.logout(text).map_err(|e| match e {
            CommandError::NotConnected => NotConnected::new_err(e.to_string()),
            CommandError::Full => QueueFull::new_err(e.to_string()),
            _ => Error::new_err(e.to_string()),
        })
    }

    fn __repr__(&self) -> String {
        format!("Session({})", self.0.id())
    }
}

fn send_error(e: SendError) -> PyErr {
    match e {
        SendError::NotConnected(_) => NotConnected::new_err(e.to_string()),
        SendError::Full(_) => QueueFull::new_err(e.to_string()),
        _ => Error::new_err(e.to_string()),
    }
}

/// Identifies a session: BeginString, the CompIDs, and the optional SubIDs, LocationIDs and
/// qualifier.
#[pyclass(frozen, eq, hash, str, name = "SessionId", module = "turbojet")]
#[derive(PartialEq, Eq, Hash)]
pub struct PySessionId(SessionId);

impl fmt::Display for PySessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[pymethods]
impl PySessionId {
    #[getter]
    fn begin_string(&self) -> &str {
        &self.0.begin_string
    }
    #[getter]
    fn sender_comp_id(&self) -> &str {
        &self.0.sender_comp_id
    }
    #[getter]
    fn sender_sub_id(&self) -> Option<&str> {
        self.0.sender_sub_id.as_deref()
    }
    #[getter]
    fn sender_location_id(&self) -> Option<&str> {
        self.0.sender_location_id.as_deref()
    }
    #[getter]
    fn target_comp_id(&self) -> &str {
        &self.0.target_comp_id
    }
    #[getter]
    fn target_sub_id(&self) -> Option<&str> {
        self.0.target_sub_id.as_deref()
    }
    #[getter]
    fn target_location_id(&self) -> Option<&str> {
        self.0.target_location_id.as_deref()
    }
    #[getter]
    fn qualifier(&self) -> Option<&str> {
        self.0.qualifier.as_deref()
    }

    fn __repr__(&self) -> String {
        format!("SessionId({})", self.0)
    }
}

/// How a logged-on session ended.
#[pyclass(frozen, eq, eq_int, name = "Disconnect", module = "turbojet")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PyDisconnect {
    /// We logged out.
    Logout,
    /// The counterparty logged out.
    CounterpartyLogout,
    /// The connection ended without a Logout.
    ConnectionLost,
    /// The counterparty didn't answer a TestRequest.
    HeartbeatTimeout,
    /// We ended the session over an error.
    Error,
    /// Our side shut down, or the session's schedule ended.
    Shutdown,
}

impl From<turbojet::Disconnect> for PyDisconnect {
    fn from(ended: turbojet::Disconnect) -> Self {
        use turbojet::Disconnect as D;
        match ended {
            D::Logout => Self::Logout,
            D::CounterpartyLogout => Self::CounterpartyLogout,
            D::ConnectionLost => Self::ConnectionLost,
            D::HeartbeatTimeout => Self::HeartbeatTimeout,
            D::Shutdown => Self::Shutdown,
            // Error, and any added since: mirror a new one above.
            _ => Self::Error,
        }
    }
}

/// A session logged on.
#[pyclass(frozen, get_all, module = "turbojet")]
pub struct Logon {
    session: Py<Session>,
}

#[pymethods]
impl Logon {
    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("session",)
    }

    fn __repr__(&self) -> String {
        format!("Logon({})", self.session.get().0.id())
    }
}

/// A logged-on session ended, as `ended` says.
#[pyclass(frozen, get_all, module = "turbojet")]
pub struct Logout {
    session: Py<Session>,
    ended: PyDisconnect,
}

#[pymethods]
impl Logout {
    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("session", "ended")
    }

    fn __repr__(&self) -> String {
        format!("Logout({}, Disconnect.{:?})", self.session.get().0.id(), self.ended)
    }
}

/// Locks `mutex`, through a panic elsewhere: what it guards stays consistent.
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
