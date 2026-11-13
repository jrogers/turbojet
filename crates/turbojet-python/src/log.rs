//! The engine's events into Python's `logging`, each record carrying the fields of the spans it
//! was logged in: `record.session` says which session logged it.

use std::collections::HashMap;
use std::fmt::{self, Write as _};
use std::sync::{LazyLock, RwLock};

use pyo3::prelude::*;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber, span};
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

/// Each target's logger and its effective level, read once: an event below it is dropped without
/// taking the GIL.
static LOGGERS: LazyLock<RwLock<HashMap<String, Logger>>> = LazyLock::new(RwLock::default);

struct Logger {
    level: u8,
    logger: Py<PyAny>,
}

/// Forgets the cached loggers, so levels changed since are read again.
pub(crate) fn reset() {
    LOGGERS.write().expect("not poisoned").clear();
}

/// Hands each event to the `logging` logger named for its target (`turbojet::session` to
/// `turbojet.session`).
pub(crate) struct PythonLayer;

/// A span's fields, as text.
struct Fields(Vec<(&'static str, String)>);

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for PythonLayer {
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let mut fields = Fields(Vec::new());
        attrs.record(&mut fields);
        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(fields);
        }
    }

    fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(id)
            && let Some(fields) = span.extensions_mut().get_mut::<Fields>()
        {
            values.record(fields);
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let normalized = event.normalized_metadata();
        let meta = normalized.as_ref().unwrap_or_else(|| event.metadata());
        let level = python_level(*meta.level());
        let cached = LOGGERS.read().expect("not poisoned").get(meta.target()).map(|l| l.level);
        if cached.is_some_and(|enabled| level < enabled) {
            return;
        }
        let mut message = Message::default();
        event.record(&mut message);
        // Outermost first, so an inner span's field wins. The connection's `session` span names
        // the session `id`; the initiator's names it `session`.
        let mut extra: Vec<(&'static str, String)> = Vec::new();
        for span in ctx.event_scope(event).into_iter().flat_map(|scope| scope.from_root()) {
            let rename = span.name() == "session";
            if let Some(fields) = span.extensions().get::<Fields>() {
                for (name, value) in &fields.0 {
                    let name = if rename && *name == "id" { "session" } else { name };
                    extra.retain(|(n, _)| *n != name);
                    extra.push((name, value.clone()));
                }
            }
        }
        Python::attach(|py| {
            let result = (|| {
                let Some(logger) = logger(py, meta.target(), level)? else { return Ok(()) };
                let file = meta.file().unwrap_or("<unknown>");
                let record = logger.call_method1(
                    "makeRecord",
                    (logger.getattr("name")?, level, file, meta.line().unwrap_or(0), message.text(), (), py.None()),
                )?;
                for (name, value) in extra {
                    // Never over one of the record's own attributes.
                    if !record.hasattr(name)? {
                        record.setattr(name, value)?;
                    }
                }
                logger.call_method1("handle", (record,))?;
                PyResult::Ok(())
            })();
            if let Err(e) = result {
                e.write_unraisable(py, None);
            }
        });
    }
}

/// The logger for `target`, if it takes records at `level`, caching it and its level.
fn logger<'py>(py: Python<'py>, target: &str, level: u8) -> PyResult<Option<Bound<'py, PyAny>>> {
    if let Some(l) = LOGGERS.read().expect("not poisoned").get(target) {
        return Ok((level >= l.level).then(|| l.logger.bind(py).clone()));
    }
    let logger = py.import("logging")?.call_method1("getLogger", (target.replace("::", "."),))?;
    let enabled: u8 = logger.call_method0("getEffectiveLevel")?.extract()?;
    let cached = Logger { level: enabled, logger: logger.clone().unbind() };
    LOGGERS.write().expect("not poisoned").insert(target.to_owned(), cached);
    Ok((level >= enabled).then_some(logger))
}

fn python_level(level: Level) -> u8 {
    match level {
        Level::ERROR => 40,
        Level::WARN => 30,
        Level::INFO => 20,
        Level::DEBUG => 10,
        Level::TRACE => 5,
    }
}

impl Visit for Fields {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.set(field.name(), value.to_owned());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.set(field.name(), format!("{value:?}"));
    }
}

impl Fields {
    fn set(&mut self, name: &'static str, value: String) {
        match self.0.iter_mut().find(|(n, _)| *n == name) {
            Some(field) => field.1 = value,
            None => self.0.push((name, value)),
        }
    }
}

/// An event's message followed by its other fields, `key=value`, as `tracing`'s `log` output
/// writes them.
#[derive(Default)]
struct Message {
    message: String,
    fields: String,
}

impl Message {
    fn text(self) -> String {
        if self.fields.is_empty() { self.message } else { format!("{}{}", self.message, self.fields) }
    }

    fn write(&mut self, field: &Field, value: fmt::Arguments<'_>) {
        match field.name() {
            "message" => {
                let _ = self.message.write_fmt(value);
            }
            // tracing-log's description of a `log` record, already in the metadata.
            name if name.starts_with("log.") => {}
            name => {
                let _ = write!(self.fields, " {name}={value}");
            }
        }
    }
}

impl Visit for Message {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.write(field, format_args!("{value}"));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.write(field, format_args!("{value:?}"));
    }
}
