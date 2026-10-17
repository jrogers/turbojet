//! `turbojet.Engine`: the initiators a sessions file lists, run on the engine's tokio runtime, and
//! the events they raise, drained from asyncio.

use std::fmt;
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyStopAsyncIteration, PyStopIteration, PyValueError};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::{future_into_py, get_runtime};
use tokio::sync::{mpsc, watch};
use turbojet::peer::ConnectionInfo;
use turbojet::{
    Application, CommandError, Context, Hold, Message, MessageReject, MsgType, SendError, SessionHandle, SessionId,
};
use turbojet_config::{Initiators, SessionsFile};

use crate::message::PyMessage;

create_exception!(turbojet, Error, PyException, "Every error the engine raises.");
create_exception!(turbojet, ConfigError, Error, "The sessions file can't be used.");
create_exception!(turbojet, NotConnected, Error, "No connection has the session.");
create_exception!(turbojet, QueueFull, Error, "The session's send or control queue is full.");
create_exception!(turbojet, Dropped, Error, "A queued message was never stored or sent.");
create_exception!(turbojet, SequenceError, Error, "A change to sequence numbers was refused.");

/// What a callback tells Python, on its way through the channel. Python objects are made only
/// when Python takes it, so a session's task never needs the GIL.
enum Event {
    Logon(SessionHandle),
    Logout(SessionHandle, turbojet::Disconnect),
    Admin(SessionHandle, Message),
    App { session: SessionHandle, msg: Message, maybe_redelivered: bool, hold: Hold },
    CancelOnDisconnect(SessionHandle, turbojet::Disconnect),
}

impl Event {
    fn into_py(self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let session = |s| Py::new(py, Session(s));
        let msg = |m| Py::new(py, PyMessage(m));
        Ok(match self {
            Event::Logon(s) => Py::new(py, Logon { session: session(s)? })?.into_any(),
            Event::Logout(s, ended) => Py::new(py, Logout { session: session(s)?, ended: ended.into() })?.into_any(),
            Event::Admin(s, m) => Py::new(py, AdminMessage { session: session(s)?, msg: msg(m)? })?.into_any(),
            Event::App { session: s, msg: m, maybe_redelivered, hold } => {
                let event =
                    AppMessage { session: session(s)?, msg: msg(m)?, maybe_redelivered, hold: Mutex::new(Some(hold)) };
                Py::new(py, event)?.into_any()
            }
            Event::CancelOnDisconnect(s, ended) => {
                Py::new(py, CancelOnDisconnect { session: session(s)?, ended: ended.into() })?.into_any()
            }
        })
    }
}

/// The application the engine calls: it passes events on, and calls the sync hooks Python gave.
/// A closed channel means Python has stopped listening, and the event goes nowhere.
struct Bridge {
    events: mpsc::UnboundedSender<Event>,
    verify_logon: Option<Py<PyAny>>,
    to_admin: Option<Py<PyAny>>,
    should_resend: Option<Py<PyAny>>,
}

impl Application for Bridge {
    fn verify_logon(
        &self,
        session: &SessionHandle,
        logon: &Message,
        connection: &ConnectionInfo,
    ) -> Result<(), String> {
        let Some(hook) = &self.verify_logon else { return Ok(()) };
        Python::attach(|py| {
            let args = (Session(session.clone()), PyMessage(logon.clone()), connection.addr.map(|a| a.to_string()));
            match hook.call1(py, args).and_then(|refused| refused.extract::<Option<String>>(py)) {
                Ok(None) => Ok(()),
                Ok(Some(reason)) => Err(reason),
                Err(e) => {
                    e.display(py);
                    Err(format!("verify_logon raised {e}"))
                }
            }
        })
    }

    /// The hook changes a copy, which then replaces `msg`. An exception panics, which the engine
    /// answers by disconnecting rather than send a message the hook may have half changed.
    fn to_admin(&self, session: &SessionHandle, msg: &mut Message) {
        let Some(hook) = &self.to_admin else { return };
        Python::attach(|py| {
            let changed = Py::new(py, PyMessage(msg.clone())).and_then(|changed| {
                hook.call1(py, (Session(session.clone()), changed.clone_ref(py)))?;
                Ok(changed)
            });
            match changed {
                Ok(changed) => *msg = std::mem::take(&mut changed.borrow_mut(py).0),
                Err(e) => {
                    e.display(py);
                    panic!("to_admin raised {e}");
                }
            }
        });
    }

    /// An exception resends the message, as a panic would.
    fn should_resend(&self, session: &SessionHandle, msg: &Message) -> bool {
        let Some(hook) = &self.should_resend else { return true };
        Python::attach(|py| {
            match hook.call1(py, (Session(session.clone()), PyMessage(msg.clone()))).and_then(|r| r.is_truthy(py)) {
                Ok(resend) => resend,
                Err(e) => {
                    e.display(py);
                    true
                }
            }
        })
    }

    fn on_admin_message(&self, session: &SessionHandle, msg: &Message) {
        let _ = self.events.send(Event::Admin(session.clone(), msg.clone()));
    }

    fn on_logon(&self, session: &SessionHandle) {
        let _ = self.events.send(Event::Logon(session.clone()));
    }

    fn on_logout(&self, session: &SessionHandle, ended: turbojet::Disconnect) {
        let _ = self.events.send(Event::Logout(session.clone(), ended));
    }

    fn on_cancel_on_disconnect(&self, session: &SessionHandle, ended: turbojet::Disconnect) {
        let _ = self.events.send(Event::CancelOnDisconnect(session.clone(), ended));
    }

    /// Holds the message until Python calls `done()`, so it counts as received only then.
    fn on_message(&self, ctx: &mut Context<'_>, msg: &Message) -> Result<(), MessageReject> {
        let event = Event::App {
            session: ctx.session().clone(),
            msg: msg.clone(),
            maybe_redelivered: ctx.maybe_redelivered(),
            hold: ctx.hold(),
        };
        let _ = self.events.send(event);
        Ok(())
    }
}

/// Loads the sessions file at `path`: its initiators, run when the engine is entered with
/// `async with`. The hooks, if given, are called on the session's task with the GIL, so they must
/// be quick: `verify_logon(session, logon, addr)` returns a reason to refuse the counterparty's
/// Logon, or `None`; `to_admin(session, msg)` changes an outbound session-level message, such as
/// adding credentials to Logon; `should_resend(session, msg)` returns whether to resend a message
/// the counterparty asked for again, or gap-fill it.
#[pyfunction]
#[pyo3(signature = (path, *, verify_logon = None, to_admin = None, should_resend = None))]
pub fn load(
    path: PathBuf,
    verify_logon: Option<Py<PyAny>>,
    to_admin: Option<Py<PyAny>>,
    should_resend: Option<Py<PyAny>>,
) -> PyResult<Engine> {
    let file = SessionsFile::load(path).map_err(|e| ConfigError::new_err(e.to_string()))?;
    if file.listen().is_some() {
        return Err(ConfigError::new_err("the Python bindings run initiators only: remove [acceptor]"));
    }
    let (sender, events) = mpsc::unbounded_channel();
    Ok(Engine {
        file,
        bridge: Mutex::new(Some(Bridge { events: sender, verify_logon, to_admin, should_resend })),
        initiators: Mutex::new(None),
        events: Arc::new(tokio::sync::Mutex::new(events)),
        closed: watch::Sender::new(false),
        ready: AtomicU32::new(0),
    })
}

/// The initiators a sessions file lists. `async with` starts them and, on the way out, logs them
/// out and stops them; `events()` gives what they raise meanwhile.
#[pyclass(frozen, module = "turbojet")]
pub struct Engine {
    file: SessionsFile,
    /// The application the engine calls, until it starts.
    bridge: Mutex<Option<Bridge>>,
    initiators: Mutex<Option<Arc<Initiators>>>,
    events: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Event>>>,
    /// Set once the engine has shut down: events then end when the channel is empty.
    closed: watch::Sender<bool>,
    /// Events returned ready, counted to yield to the event loop every `READY_YIELD`.
    ready: AtomicU32,
}

/// How many events in a row are returned ready before one yields to the event loop first, so a
/// loop over events that keep coming still lets other tasks run.
const READY_YIELD: u32 = 32;

#[pymethods]
impl Engine {
    fn __aenter__<'py>(slf: Bound<'py, Self>) -> PyResult<Bound<'py, PyAny>> {
        let this = slf.get();
        let bridge = lock(&this.bridge).take().ok_or_else(|| Error::new_err("the engine has already started"))?;
        let initiators = {
            let _runtime = get_runtime().enter();
            this.file.initiators(Arc::new(bridge)).map_err(|e| ConfigError::new_err(e.to_string()))?
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

    /// An event already waiting is made here, on the event loop's thread, and returned ready:
    /// going through the runtime would cost a task, an asyncio future and a wake-up for each.
    /// Every `READY_YIELD`th yields to the event loop once first, as `asyncio.sleep(0)` does.
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(mut events) = self.events.try_lock()
            && let Ok(event) = events.try_recv()
        {
            let yields = self.ready.fetch_add(1, Ordering::Relaxed) % READY_YIELD == READY_YIELD - 1;
            let ready = Ready { event: Mutex::new(Some(event.into_py(py)?)), yields: AtomicBool::new(yields) };
            return Ok(Bound::new(py, ready)?.into_any());
        }
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

    /// Reads the sessions file again and puts it in use: see turbojet-config's Reloading.
    /// `ConfigError` if it doesn't load, or changes what's fixed until a restart; the file in use
    /// stays in use.
    fn reload(&self) -> PyResult<()> {
        let initiators = lock(&self.initiators);
        let initiators = initiators.as_ref().ok_or_else(|| Error::new_err("the engine isn't running"))?;
        let _runtime = get_runtime().enter();
        self.file.reload_all(None, initiators).map(drop).map_err(|e| ConfigError::new_err(e.to_string()))
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

/// An awaitable that's already finished, with `event`: awaiting it returns at once, or after
/// yielding to the event loop once if `yields`.
#[pyclass(frozen, module = "turbojet")]
struct Ready {
    event: Mutex<Option<Py<PyAny>>>,
    yields: AtomicBool,
}

#[pymethods]
impl Ready {
    fn __await__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    /// A bare yield (`None`), which asyncio answers by running the task again on the loop's next
    /// turn; then the event, as a coroutine returning it would.
    fn __next__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        if self.yields.swap(false, Ordering::Relaxed) {
            return Ok(py.None());
        }
        Err(PyStopIteration::new_err((lock(&self.event).take(),)))
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
        send(py, self.0.clone(), msg.0.clone())
    }

    /// Sends a BusinessMessageReject for `msg`, a message received on this session, with
    /// BusinessRejectReason `reason` and Text `text`, as `send` does. Then call the event's
    /// `done()`, as for any message.
    #[pyo3(signature = (msg, reason, text = None))]
    fn reject<'py>(
        &self,
        py: Python<'py>,
        msg: PyRef<'py, PyMessage>,
        reason: u32,
        text: Option<&str>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let id = self.0.id();
        if id.begin_string == "FIX.4.0" || id.begin_string == "FIX.4.1" {
            return Err(Error::new_err(format!("{} has no BusinessMessageReject", id.begin_string)));
        }
        let received = &msg.0;
        let seq_num =
            received.get(34).ok_or_else(|| PyValueError::new_err("not a received message: no MsgSeqNum(34)"))?;
        let mut reply = Message::new(MsgType::from_code("j")).with_reverse_route(received);
        reply.push(45, seq_num);
        reply.push(372, received.msg_type());
        reply.push(380, reason);
        if let Some(text) = text {
            reply.push(58, text);
        }
        send(py, self.0.clone(), reply)
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

    /// Logs the session out and keeps it out until `resume()`, with `text` as the Logout's Text.
    #[pyo3(signature = (text = None))]
    fn pause(&self, text: Option<&str>) {
        self.0.pause(text);
    }

    /// Lets a paused session log on again.
    fn resume(&self) {
        self.0.resume();
    }

    /// Whether the session is paused.
    #[getter]
    fn paused(&self) -> bool {
        self.0.is_paused()
    }

    /// The session's sequence numbers.
    fn sequence_numbers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let session = self.0.clone();
        sequence(py, async move { session.sequence_numbers().await })
    }

    /// Sets the MsgSeqNum expected next from the counterparty.
    fn set_next_incoming<'py>(&self, py: Python<'py>, seq: u64) -> PyResult<Bound<'py, PyAny>> {
        let session = self.0.clone();
        sequence(py, async move { session.set_next_incoming(seq).await })
    }

    /// Sets the MsgSeqNum of the next message sent.
    fn set_next_outgoing<'py>(&self, py: Python<'py>, seq: u64) -> PyResult<Bound<'py, PyAny>> {
        let session = self.0.clone();
        sequence(py, async move { session.set_next_outgoing(seq).await })
    }

    /// Asks the counterparty to send its messages again from MsgSeqNum `begin`.
    fn request_resend<'py>(&self, py: Python<'py>, begin: u64) -> PyResult<Bound<'py, PyAny>> {
        let session = self.0.clone();
        sequence(py, async move { session.request_resend(begin).await })
    }

    /// Resets both sequence numbers to 1.
    fn reset_sequence_numbers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let session = self.0.clone();
        sequence(py, async move { session.reset_sequence_numbers().await })
    }

    fn __repr__(&self) -> String {
        format!("Session({})", self.0.id())
    }
}

/// Sends `msg` on `session`, waiting for room in the send queue, and returns the MsgSeqNum it was
/// stored with.
fn send(py: Python<'_>, session: SessionHandle, msg: Message) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        let receipt = session.send_when_ready(msg).await.map_err(send_error)?;
        receipt.await.map_err(|dropped| Dropped::new_err(dropped.to_string()))
    })
}

/// Awaits an operator's change to sequence numbers, `change`.
fn sequence<F>(py: Python<'_>, change: F) -> PyResult<Bound<'_, PyAny>>
where
    F: Future<Output = Result<turbojet::SequenceNumbers, turbojet::SequenceError>> + Send + 'static,
{
    future_into_py(py, async move {
        let numbers = change.await.map_err(|e| SequenceError::new_err(e.to_string()))?;
        Ok(SequenceNumbers { next_incoming: numbers.next_incoming, next_outgoing: numbers.next_outgoing })
    })
}

/// A session's sequence numbers.
#[pyclass(frozen, eq, get_all, module = "turbojet")]
#[derive(PartialEq)]
pub struct SequenceNumbers {
    /// The MsgSeqNum expected on the next message from the counterparty.
    next_incoming: u64,
    /// The MsgSeqNum of the next message sent.
    next_outgoing: u64,
}

#[pymethods]
impl SequenceNumbers {
    #[new]
    fn new(next_incoming: u64, next_outgoing: u64) -> Self {
        Self { next_incoming, next_outgoing }
    }

    fn __repr__(&self) -> String {
        format!("SequenceNumbers(next_incoming={}, next_outgoing={})", self.next_incoming, self.next_outgoing)
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

/// A session-level message from the counterparty, other than a Logon: a Heartbeat, TestRequest,
/// ResendRequest, Reject, SequenceReset or Logout. The session has handled it; this is to see it.
#[pyclass(frozen, get_all, module = "turbojet")]
pub struct AdminMessage {
    session: Py<Session>,
    msg: Py<PyMessage>,
}

#[pymethods]
impl AdminMessage {
    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("session", "msg")
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("AdminMessage({}, {})", self.session.get().0.id(), self.msg.borrow(py).0.msg_type())
    }
}

/// An application message from the counterparty, in sequence. It counts as received only once
/// `done()` is called: until then, a restart has the counterparty send it again, with
/// `maybe_redelivered` set. One never finished holds its session up: 256 messages later, the
/// session reads nothing more until it's finished. Dropping the event doesn't finish it.
#[pyclass(frozen, module = "turbojet")]
pub struct AppMessage {
    #[pyo3(get)]
    session: Py<Session>,
    #[pyo3(get)]
    msg: Py<PyMessage>,
    /// Whether this message may have been handled already, before a restart or a lost connection:
    /// check (by ClOrdID, say) before acting on it again.
    #[pyo3(get)]
    maybe_redelivered: bool,
    hold: Mutex<Option<Hold>>,
}

#[pymethods]
impl AppMessage {
    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("session", "msg")
    }

    /// The message is finished: it counts as received. Calling it again does nothing.
    fn done(&self) {
        lock(&self.hold).take();
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("AppMessage({}, {})", self.session.get().0.id(), self.msg.borrow(py).0.msg_type())
    }
}

impl Drop for AppMessage {
    /// A hold dropped is released, but an event dropped unfinished, by an exception say, isn't
    /// finished: keep the message held, so it's sent again after a restart.
    fn drop(&mut self) {
        let hold = self.hold.get_mut().unwrap_or_else(PoisonError::into_inner).take();
        std::mem::forget(hold);
    }
}

/// A session with cancel on disconnect ended, as `ended` says, and didn't log back on in time:
/// cancel its resting orders.
#[pyclass(frozen, get_all, module = "turbojet")]
pub struct CancelOnDisconnect {
    session: Py<Session>,
    ended: PyDisconnect,
}

#[pymethods]
impl CancelOnDisconnect {
    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("session", "ended")
    }

    fn __repr__(&self) -> String {
        format!("CancelOnDisconnect({}, Disconnect.{:?})", self.session.get().0.id(), self.ended)
    }
}

/// Locks `mutex`, through a panic elsewhere: what it guards stays consistent.
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
