//! Python bindings for Turbojet: FIX initiators loaded from a sessions file, driven from asyncio.

mod engine;
mod message;

use pyo3::prelude::*;

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
    let py = m.py();
    m.add("Error", py.get_type::<engine::Error>())?;
    m.add("ConfigError", py.get_type::<engine::ConfigError>())?;
    m.add("NotConnected", py.get_type::<engine::NotConnected>())?;
    m.add("QueueFull", py.get_type::<engine::QueueFull>())?;
    m.add("Dropped", py.get_type::<engine::Dropped>())?;
    Ok(())
}
