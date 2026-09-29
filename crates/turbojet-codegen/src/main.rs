//! turbojet-codegen: writes generated modules for a dictionary, formatted with rustfmt.

use std::path::PathBuf;
use std::process::{Command, ExitCode};

use turbojet_codegen::Generator;
use turbojet_dictionary::Dictionary;

const USAGE: &str = "usage: turbojet-codegen <dictionary.xml> [--transport FIXT11.xml] [--merge venue.xml]... \
                     [--optional Owner.Member]... [--docs] [--lenient-enums] [--lenient-enum Field]... \
                     --out <dir>\n\n--docs copies the dictionary's documentation into the doc comments; \
                     the text is FIX Protocol Limited's, so check its licence first.\n--lenient-enums makes every enumerated \
                     field a Code<E>, keeping codes the dictionary doesn't list; --lenient-enum does it for one.\n\
                     --optional QuoteCancel.NoQuoteEntries makes a field, group or component optional in a \
                     message or component, correcting a dictionary.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turbojet-codegen: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (mut dictionary, mut transport, mut merges, mut out, mut docs) = (None, None, Vec::new(), None, false);
    let (mut lenient_all, mut lenient, mut optional) = (false, Vec::new(), Vec::new());
    while let Some(arg) = args.next() {
        let mut value = || args.next().map(PathBuf::from).ok_or_else(|| format!("{arg} needs a value\n{USAGE}"));
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            "--transport" => transport = Some(value()?),
            "--merge" => merges.push(value()?),
            "--out" => out = Some(value()?),
            "--docs" => docs = true,
            "--lenient-enums" => lenient_all = true,
            "--lenient-enum" => lenient.push(value()?),
            "--optional" => optional.push(value()?),
            _ if arg.starts_with('-') || dictionary.is_some() => {
                return Err(format!("unexpected {arg}\n{USAGE}").into());
            }
            _ => dictionary = Some(PathBuf::from(&arg)),
        }
    }
    let (Some(dictionary), Some(out)) = (dictionary, out) else { return Err(USAGE.into()) };

    let mut dict = Dictionary::load(&dictionary)?;
    // The transport comes first, whatever the argument order, so venues can refer to its fields
    // and a venue's header or trailer replaces the transport's rather than being replaced by it.
    if let Some(transport) = transport {
        dict = dict.with_transport(&Dictionary::load(transport)?)?;
    }
    for venue in &merges {
        dict.merge_file(venue)?;
    }
    for correction in &optional {
        let correction = correction.to_string_lossy();
        let (owner, member) =
            correction.split_once('.').ok_or_else(|| format!("--optional {correction}: expected Owner.Member"))?;
        dict.make_optional(owner, member)?;
    }
    let mut generator = Generator::new(&dict).with_docs(docs).lenient_enums(lenient_all);
    for field in &lenient {
        generator = generator.lenient_enum(&field.to_string_lossy());
    }
    let files = generator.write_modules(&out)?;
    // The style is pinned (to the repository's rustfmt.toml), so the output doesn't depend on any
    // rustfmt.toml above where it's written.
    let status = Command::new("rustfmt")
        .args(["--edition", "2024", "--config", "max_width=120,use_small_heuristics=Max"])
        .args(&files)
        .status()
        .map_err(|e| format!("running rustfmt: {e} (install it with rustup component add rustfmt)"))?;
    if !status.success() {
        return Err("rustfmt failed".into());
    }
    Ok(())
}
