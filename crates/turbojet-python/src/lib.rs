//! Python bindings for Turbojet: FIX initiators loaded from a sessions file, driven from asyncio.

mod engine;
mod log;
mod message;

use std::sync::OnceLock;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Logs the engine's events at `filter` and above (`"info"`, or per module as
/// `"warn,turbojet::session=debug"`) to stderr. Once per process, and not with `log_to_python`:
/// `Error` after either.
#[pyfunction]
fn log_to_stderr(filter: &str) -> PyResult<()> {
    // Checked first: the subscriber takes over from Python's logging before its own init fails.
    if PYTHON_LOG.get().is_some() {
        return Err(engine::Error::new_err("log_to_python is already on"));
    }
    let filter = tracing_subscriber::EnvFilter::try_new(filter).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).with_writer(std::io::stderr);
    subscriber.try_init().map_err(|e| engine::Error::new_err(e.to_string()))
}

/// Logs the engine's events to Python's `logging`, under loggers named for their module
/// (`turbojet.session`), at debug and above. Each record has the session's ID as
/// `record.session`, and a connection attempt's endpoint as `record.addr` and `record.role`
/// (`primary` or `backup`). Levels are read once per logger and cached: call again after changing
/// them. Takes the GIL for each event logged, so a busy session is slower than with
/// `log_to_stderr`, and the two don't mix: `Error` once the other is on.
#[pyfunction]
fn log_to_python() -> PyResult<()> {
    use tracing_subscriber::layer::{Layer as _, SubscriberExt as _};
    use tracing_subscriber::util::SubscriberInitExt as _;

    if PYTHON_LOG.get().is_some() {
        log::reset();
        return Ok(());
    }
    if tracing::dispatcher::has_been_set() {
        return Err(engine::Error::new_err("log_to_stderr is already on"));
    }
    let layer = log::PythonLayer.with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    tracing_subscriber::registry().with(layer).try_init().map_err(|e| engine::Error::new_err(e.to_string()))?;
    let _ = PYTHON_LOG.set(());
    Ok(())
}

static PYTHON_LOG: OnceLock<()> = OnceLock::new();

/// The `turbojet._turbojet` extension module, re-exported by the `turbojet` package.
#[pymodule]
fn _turbojet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<message::PyMessage>()?;
    m.add_function(wrap_pyfunction!(engine::load, m)?)?;
    m.add_class::<engine::Engine>()?;
    m.add_class::<engine::Session>()?;
    m.add_class::<engine::PySessionId>()?;
    m.add_class::<engine::PyDisconnect>()?;
    m.add_class::<engine::Logon>()?;
    m.add_class::<engine::Logout>()?;
    m.add_class::<engine::AdminMessage>()?;
    m.add_class::<engine::AppMessage>()?;
    m.add_class::<engine::CancelOnDisconnect>()?;
    m.add_class::<engine::SequenceNumbers>()?;
    m.add_function(wrap_pyfunction!(log_to_stderr, m)?)?;
    m.add_function(wrap_pyfunction!(log_to_python, m)?)?;
    let py = m.py();
    m.add("Error", py.get_type::<engine::Error>())?;
    m.add("ConfigError", py.get_type::<engine::ConfigError>())?;
    m.add("NotConnected", py.get_type::<engine::NotConnected>())?;
    m.add("QueueFull", py.get_type::<engine::QueueFull>())?;
    m.add("Dropped", py.get_type::<engine::Dropped>())?;
    m.add("SequenceError", py.get_type::<engine::SequenceError>())?;
    Ok(())
}
