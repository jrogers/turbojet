//! `turbojet.Message`: the untyped FIX message, fields by tag.

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PyList};
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

    /// The value of data field `tag` as bytes, or `None`.
    fn get_bytes(&self, tag: u32) -> Option<&[u8]> {
        self.0.get_bytes(tag)
    }

    /// Sets data field `data_tag` to `value`, and its Length field `length_tag` to its length.
    fn set_data(&mut self, length_tag: u32, data_tag: u32, value: &[u8]) {
        self.0.set_data(length_tag, data_tag, value);
    }

    /// The entries of the repeating group introduced by NumInGroup field `count_tag`, each a
    /// message of its fields. `members` are the group's tags, the delimiter that starts each entry
    /// first; the group ends at the first field that isn't one. For a nested group, list its tags
    /// too and call `group` on an entry. Empty if `count_tag` is absent; `ValueError` if the
    /// entries don't match the count or don't start with the delimiter.
    fn group(&self, count_tag: u32, members: Vec<u32>) -> PyResult<Vec<PyMessage>> {
        let Some(&delimiter) = members.first() else {
            return Err(PyValueError::new_err("members start with the group's delimiter"));
        };
        let fields: Vec<(u32, &str)> = self.0.fields().collect();
        let Some(start) = fields.iter().position(|&(tag, _)| tag == count_tag) else { return Ok(Vec::new()) };
        let count: usize = fields[start]
            .1
            .parse()
            .map_err(|_| PyValueError::new_err(format!("NumInGroup({count_tag}) isn't a number")))?;
        let mut entries: Vec<Message> = Vec::new();
        for &(tag, value) in fields[start + 1..].iter().take_while(|(tag, _)| members.contains(tag)) {
            if tag == delimiter {
                entries.push(Message::new(self.0.msg_type()));
            }
            let Some(entry) = entries.last_mut() else {
                return Err(PyValueError::new_err(format!(
                    "group {count_tag}'s first entry doesn't start with {delimiter}"
                )));
            };
            entry.push(tag, value);
        }
        if entries.len() != count {
            let found = entries.len();
            return Err(PyValueError::new_err(format!(
                "NumInGroup({count_tag}) is {count} but {found} entries follow"
            )));
        }
        Ok(entries.into_iter().map(PyMessage).collect())
    }

    fn __len__(&self) -> usize {
        self.0.fields().count()
    }

    /// `(tag, value)` pairs, in wire order: MsgType(35) first, and on a received message the rest of
    /// the header and the trailer too.
    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        PyList::new(py, self.0.fields())?.try_iter()
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
