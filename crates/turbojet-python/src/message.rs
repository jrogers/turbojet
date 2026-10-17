//! `turbojet.Message`: the untyped FIX message, fields by tag.

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use turbojet::{Message, MsgType};

/// A FIX message: its MsgType and its fields by tag, values as text. The engine adds the header
/// and trailer when it's sent.
#[pyclass(name = "Message", module = "turbojet")]
pub struct PyMessage(pub Message);

#[pymethods]
impl PyMessage {
    #[new]
    #[pyo3(signature = (msg_type, fields = Vec::new()))]
    fn new(msg_type: &str, fields: Vec<(u32, String)>) -> Self {
        let mut msg = Message::new(MsgType::from_code(msg_type));
        for (tag, value) in fields {
            msg.push(tag, value.as_str());
        }
        Self(msg)
    }

    /// MsgType(35), e.g. `"D"`.
    #[getter]
    fn msg_type(&self) -> String {
        self.0.msg_type().code().to_string()
    }

    /// The first value of `tag`, or `None`.
    fn get(&self, tag: u32) -> Option<&str> {
        self.0.get(tag)
    }

    fn __getitem__(&self, tag: u32) -> PyResult<&str> {
        self.0.get(tag).ok_or_else(|| PyKeyError::new_err(tag))
    }

    fn __setitem__(&mut self, tag: u32, value: &str) {
        self.0.set(tag, value);
    }

    fn __repr__(&self) -> String {
        let fields: Vec<String> = self.0.fields().map(|(tag, value)| format!("{tag}={value}")).collect();
        format!("Message({}, {})", self.0.msg_type().code(), fields.join("|"))
    }
}
