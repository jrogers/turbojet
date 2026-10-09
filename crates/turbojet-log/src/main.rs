//! Prints the records of Turbojet's message log files (`FileMessageLog`), FIX fields named from
//! a dictionary, filtered by session, direction, MsgType, a field's value or a time range.

use std::collections::HashMap;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use chrono::{DateTime, NaiveTime, Utc};
use turbojet::codec::{Decoded, decode};
use turbojet::message::tags;
use turbojet::{Direction, FileMessageLog, LogRecord, Message};
use turbojet_dictionary::Dictionary;

const USAGE: &str = "\
usage: turbojet-log [OPTIONS] <DIR or FILE>...

Prints a FileMessageLog's records, oldest first: every log file in a directory, or the files given.

  --session ID     only this session, as logged (FIX.4.4:US->THEM); repeatable
  --in, --out      only messages received, or sent
  --type T,...     only these MsgTypes (35)
  --tag T=V        only messages with field T equal to V; repeatable, all must match
  --since TIME     from TIME: RFC 3339, or HH:MM[:SS] in UTC on each record's day
  --until TIME     before TIME, as --since
  --raw            each message on one line, as on the wire (FIX with | for SOH, FIXP in hex)
  --dictionary F   name fields from F (Orchestra or QuickFIX XML) instead of the standard
                   FIX 4.2, 4.3, 4.4 and 5.0 SP2 dictionaries";

/// The standard dictionaries, by BeginString. FIXT.1.1 sessions are named from FIX 5.0 SP2.
const BUILT_IN: [(&str, &str); 5] = [
    ("FIX.4.2", include_str!("../dictionaries/OrchestraFIX42.xml")),
    ("FIX.4.3", include_str!("../dictionaries/OrchestraFIX43.xml")),
    ("FIX.4.4", include_str!("../dictionaries/OrchestraFIX44.xml")),
    ("FIX.5.0SP2", include_str!("../dictionaries/OrchestraFIX50SP2.xml")),
    ("FIXT.1.1", include_str!("../dictionaries/OrchestraFIX50SP2.xml")),
];

fn main() -> ExitCode {
    let options = match Options::parse(std::env::args().skip(1)) {
        Ok(Some(options)) => options,
        Ok(None) => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("turbojet-log: {e}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let mut names = match Names::new(options.dictionary.as_ref()) {
        Ok(names) => names,
        Err(e) => {
            eprintln!("turbojet-log: {e}");
            return ExitCode::from(2);
        }
    };
    let mut out = BufWriter::new(io::stdout().lock());
    match print(&options, &mut names, &mut out).and_then(|ok| out.flush().map(|()| ok)) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        // Piped into `head`, say.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turbojet-log: {e}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Default)]
struct Options {
    sessions: Vec<String>,
    direction: Option<Direction>,
    types: Vec<String>,
    tags: Vec<(u32, String)>,
    since: Option<Time>,
    until: Option<Time>,
    raw: bool,
    dictionary: Option<PathBuf>,
    paths: Vec<PathBuf>,
}

/// A bound of the time range.
#[derive(Clone, Copy)]
enum Time {
    At(DateTime<Utc>),
    /// A time of day, on each record's own day.
    OfDay(NaiveTime),
}

impl Time {
    fn parse(s: &str) -> Result<Self, String> {
        if let Ok(at) = DateTime::parse_from_rfc3339(s) {
            return Ok(Time::At(at.with_timezone(&Utc)));
        }
        NaiveTime::parse_from_str(s, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
            .map(Time::OfDay)
            .map_err(|_| format!("{s:?} is neither RFC 3339 nor HH:MM[:SS]"))
    }

    /// `time` as a value comparable with this bound.
    fn cmp_with(self, time: DateTime<Utc>) -> std::cmp::Ordering {
        match self {
            Time::At(at) => time.cmp(&at),
            Time::OfDay(of_day) => time.time().cmp(&of_day),
        }
    }
}

impl Options {
    /// The options in `args`; `None` if they ask for help.
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Option<Self>, String> {
        let mut options = Options::default();
        while let Some(arg) = args.next() {
            let mut value = || args.next().ok_or_else(|| format!("{arg} needs a value"));
            match arg.as_str() {
                "--session" => options.sessions.push(value()?),
                "--in" => options.direction = Some(Direction::Inbound),
                "--out" => options.direction = Some(Direction::Outbound),
                "--type" => options.types.extend(value()?.split(',').map(str::to_owned)),
                "--tag" => {
                    let tag = value()?;
                    let (t, v) = tag.split_once('=').ok_or_else(|| format!("--tag {tag:?} isn't TAG=VALUE"))?;
                    let t = t.parse().map_err(|_| format!("--tag {tag:?}: {t:?} isn't a tag number"))?;
                    options.tags.push((t, v.to_owned()));
                }
                "--since" => options.since = Some(Time::parse(&value()?)?),
                "--until" => options.until = Some(Time::parse(&value()?)?),
                "--raw" => options.raw = true,
                "--dictionary" => options.dictionary = Some(value()?.into()),
                "-h" | "--help" => return Ok(None),
                _ if arg.starts_with('-') => return Err(format!("unknown option {arg}")),
                _ => options.paths.push(arg.into()),
            }
        }
        if options.paths.is_empty() {
            return Err("no log directory or file given".into());
        }
        Ok(Some(options))
    }

    /// Whether a record at `time` is in the time range.
    fn in_range(&self, time: DateTime<Utc>) -> bool {
        self.since.is_none_or(|since| since.cmp_with(time).is_ge())
            && self.until.is_none_or(|until| until.cmp_with(time).is_lt())
    }

    /// Whether a message passes the filters. FIXP frames, and FIX ones that don't decode, have
    /// no MsgType or fields to match.
    fn matches(&self, direction: Direction, session: Option<&str>, msg: Option<&Message>) -> bool {
        if self.direction.is_some_and(|d| d != direction)
            || (!self.sessions.is_empty() && !session.is_some_and(|s| self.sessions.iter().any(|want| want == s)))
        {
            return false;
        }
        if self.types.is_empty() && self.tags.is_empty() {
            return true;
        }
        let Some(msg) = msg else { return false };
        let msg_type = msg.get(tags::MSG_TYPE).unwrap_or_default();
        (self.types.is_empty() || self.types.iter().any(|t| t == msg_type))
            && self.tags.iter().all(|(tag, value)| msg.fields().any(|(t, v)| t == *tag && v == value))
    }
}

/// Prints every file's records that pass the filters. False if a file couldn't be read whole.
fn print(options: &Options, names: &mut Names, out: &mut impl Write) -> io::Result<bool> {
    let mut ok = true;
    for path in &options.paths {
        let files = if path.is_dir() {
            match FileMessageLog::files(path) {
                Ok(files) => files.into_iter().map(|file| file.path).collect(),
                Err(e) => {
                    eprintln!("turbojet-log: {}: {e}", path.display());
                    ok = false;
                    continue;
                }
            }
        } else {
            vec![path.clone()]
        };
        for file in files {
            let records = FileMessageLog::read(&file).and_then(|records| {
                for record in records {
                    print_record(options, names, &record?, out)?;
                }
                Ok(())
            });
            if let Err(e) = records {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    return Err(e);
                }
                eprintln!("turbojet-log: {}: {e}", file.display());
                ok = false;
            }
        }
    }
    Ok(ok)
}

fn print_record(options: &Options, names: &mut Names, record: &LogRecord, out: &mut impl Write) -> io::Result<()> {
    let (time, direction, session, frame) = match record {
        LogRecord::Message { time, direction, session, frame } => (time, *direction, session.as_deref(), frame),
        // Always shown, so a gap in what's printed is explained.
        LogRecord::Dropped { time, count } => return writeln!(out, "{} dropped {count} messages", stamp(*time)),
        _ => return Ok(()),
    };
    if !options.in_range(*time) {
        return Ok(());
    }
    let fix = frame.starts_with(b"8=");
    let decoded = if fix { Some(decode(frame)) } else { None };
    let msg = match &decoded {
        Some(Decoded::Message(msg, _)) => Some(msg),
        _ => None,
    };
    if !options.matches(direction, session, msg) {
        return Ok(());
    }
    let arrow = if direction == Direction::Inbound { "in " } else { "out" };
    write!(out, "{} {arrow} {}", stamp(*time), session.unwrap_or("-"))?;
    if options.raw {
        return if fix {
            writeln!(out, " {}", frame.escape_ascii().to_string().replace("\\x01", "|"))
        } else {
            writeln!(out, " {}", frame.iter().map(|b| format!("{b:02x}")).collect::<String>())
        };
    }
    match (&decoded, msg) {
        (_, Some(msg)) => {
            let dictionary = names.for_message(msg);
            let msg_type = msg.get(tags::MSG_TYPE).unwrap_or_default();
            let name = dictionary.and_then(|d| value_name(d, tags::MSG_TYPE, msg_type)).unwrap_or("");
            writeln!(out, " {msg_type} {name}")?;
            for (tag, value) in msg.fields_bytes() {
                let value = String::from_utf8(value.to_vec()).unwrap_or_else(|_| value.escape_ascii().to_string());
                let field = dictionary.and_then(|d| d.field_by_tag(tag));
                write!(out, "  {tag} {}={value}", field.map_or("", |f| f.name.as_str()))?;
                match dictionary.and_then(|d| value_name(d, tag, &value)) {
                    Some(name) => writeln!(out, " ({name})")?,
                    None => writeln!(out)?,
                }
            }
            Ok(())
        }
        (Some(Decoded::Garbled { reason, .. }), None) => {
            writeln!(out, " garbled: {reason}")?;
            writeln!(out, "  {}", frame.escape_ascii().to_string().replace("\\x01", "|"))
        }
        _ if fix => writeln!(out, " incomplete: {}", frame.escape_ascii().to_string().replace("\\x01", "|")),
        _ => writeln!(out, " FIXP, {} bytes", frame.len()),
    }
}

fn stamp(time: DateTime<Utc>) -> String {
    time.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// The name of an enumerated field's value, if the dictionary has one.
fn value_name<'d>(dictionary: &'d Dictionary, tag: u32, code: &str) -> Option<&'d str> {
    let value = dictionary.field_by_tag(tag)?.values.iter().find(|v| v.code == code)?;
    value.name.as_deref().or(Some(value.description.as_str()).filter(|d| !d.is_empty()))
}

/// The dictionaries fields are named from: the one given, or the standard ones, each parsed
/// when a message of its version first needs it.
struct Names {
    given: Option<Dictionary>,
    built_in: HashMap<&'static str, Dictionary>,
}

impl Names {
    fn new(given: Option<&PathBuf>) -> Result<Self, String> {
        let given = given.map(|path| Dictionary::load(path).map_err(|e| e.to_string())).transpose()?;
        Ok(Names { given, built_in: HashMap::new() })
    }

    fn for_message(&mut self, msg: &Message) -> Option<&Dictionary> {
        if self.given.is_some() {
            return self.given.as_ref();
        }
        let begin_string = msg.get(tags::BEGIN_STRING)?;
        let &(version, xml) = BUILT_IN.iter().find(|(version, _)| *version == begin_string)?;
        Some(
            self.built_in
                .entry(version)
                .or_insert_with(|| Dictionary::from_orchestra(xml).expect("the standard dictionaries load")),
        )
    }
}
