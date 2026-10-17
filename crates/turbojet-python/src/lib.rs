//! Python bindings for Turbojet: FIX initiators loaded from a sessions file, driven from asyncio.

mod message;

use pyo3::prelude::*;

/// The `turbojet._turbojet` extension module, re-exported by the `turbojet` package.
#[pymodule]
fn _turbojet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<message::PyMessage>()?;
    Ok(())
}
