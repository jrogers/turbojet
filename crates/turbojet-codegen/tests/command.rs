//! The turbojet-codegen command.

use std::path::Path;
use std::process::Command;

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_turbojet-codegen"))
}

fn workspace() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
}

#[test]
fn help_prints_usage() {
    for flag in ["-h", "--help"] {
        let out = command().arg(flag).output().unwrap();
        assert!(out.status.success(), "{flag}");
        assert!(String::from_utf8(out.stdout).unwrap().starts_with("usage: turbojet-codegen"), "{flag}");
    }
}

#[test]
fn output_is_the_same_wherever_it_is_written() {
    // Under the workspace's target/, where the repository's rustfmt.toml would apply, and in a
    // temporary directory outside it. Comparing the two rather than the checked-in files keeps
    // this independent of the rustfmt version; CI checks the checked-in files.
    let inside = Path::new(env!("CARGO_TARGET_TMPDIR")).join("output_is_the_same_wherever_it_is_written");
    let outside = tempfile::tempdir().unwrap();
    for out in [inside.as_path(), outside.path()] {
        let status = command()
            .arg(workspace().join("dictionaries/orchestra/OrchestraFIX42.xml"))
            .arg("--out")
            .arg(out)
            .status()
            .unwrap();
        assert!(status.success());
    }
    for file in ["mod.rs", "tags.rs", "enums.rs", "groups.rs", "messages.rs"] {
        let read = |dir: &Path| std::fs::read_to_string(dir.join(file)).unwrap();
        assert!(read(&inside) == read(outside.path()), "{file} differs");
    }
}

#[test]
fn a_missing_rustfmt_says_how_to_install_it() {
    let out = tempfile::tempdir().unwrap();
    let output = command()
        .arg(workspace().join("dictionaries/orchestra/OrchestraFIX42.xml"))
        .arg("--out")
        .arg(out.path())
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("turbojet-codegen: running rustfmt: "), "{stderr}");
    assert!(stderr.contains("(install it with rustup component add rustfmt)"), "{stderr}");
}

/// Runs the command with `args`, writing to a new temporary directory, and returns it.
fn generate(args: &[&std::ffi::OsStr]) -> tempfile::TempDir {
    let out = tempfile::tempdir().unwrap();
    let output = command().args(args).arg("--out").arg(out.path()).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    out
}

fn read(dir: &tempfile::TempDir, file: &str) -> String {
    std::fs::read_to_string(dir.path().join(file)).unwrap()
}

/// Writes `xml` to a temporary file.
fn overlay(xml: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut file, xml.as_bytes()).unwrap();
    file
}

#[test]
fn merge_applies_a_venue_dictionary() {
    // A value added to Side, and NewOrderSingle re-listed with its NoAllocs group, which is the
    // same as Orchestra's PreAllocGrp and so keeps that name.
    let venue = overlay(
        "<fix>
          <fields>
           <field number='54' name='Side' type='CHAR'>
            <value enum='1' description='BUY'/><value enum='2' description='SELL'/>
            <value enum='Z' description='VENUE_SPECIAL'/>
           </field>
          </fields>
          <messages>
           <message name='NewOrderSingle' msgtype='D' msgcat='app'>
            <field name='ClOrdID' required='Y'/>
            <group name='NoAllocs' required='N'>
             <field name='AllocAccount' required='N'/><field name='AllocShares' required='N'/>
            </group>
           </message>
          </messages>
         </fix>",
    );
    let base = workspace().join("dictionaries/orchestra/OrchestraFIX42.xml");
    let out = generate(&[base.as_os_str(), "--merge".as_ref(), venue.path().as_os_str()]);
    let groups = read(&out, "groups.rs");
    assert!(groups.contains("    PreAllocGrp / PreAllocGrpRef {"), "{groups}");
    assert!(!groups.contains("Alloc {"), "{groups}");
    assert!(read(&out, "enums.rs").contains("VenueSpecial = \"Z\","));
}

#[test]
fn optional_corrects_the_dictionary() {
    let base = workspace().join("dictionaries/orchestra/OrchestraFIX42.xml");
    let strict = read(&generate(&[base.as_os_str()]), "messages.rs");
    assert!(strict.contains("quote_entries: req_group QuotCxlEntriesGrp = NO_QUOTE_ENTRIES,"), "{strict}");
    let out = generate(&[base.as_os_str(), "--optional".as_ref(), "QuoteCancel.NoQuoteEntries".as_ref()]);
    let corrected = read(&out, "messages.rs");
    assert!(corrected.contains("quote_entries: group QuotCxlEntriesGrp = NO_QUOTE_ENTRIES,"), "{corrected}");

    let bad = command().arg(&base).args(["--optional", "QuoteCancel", "--out", "x"]).output().unwrap();
    assert!(!bad.status.success());
    assert!(String::from_utf8(bad.stderr).unwrap().contains("expected Owner.Member"));
}

#[test]
fn lenient_enum_flags() {
    let base = workspace().join("dictionaries/orchestra/OrchestraFIX42.xml");
    let one = generate(&[base.as_os_str(), "--lenient-enum".as_ref(), "Side".as_ref()]);
    let messages = read(&one, "messages.rs");
    assert!(messages.contains("side: req Code<Side> = SIDE,"), "{messages}");
    assert!(messages.contains("ord_type: req OrdType = ORD_TYPE,"), "{messages}");
    let all = generate(&[base.as_os_str(), "--lenient-enums".as_ref()]);
    assert!(read(&all, "messages.rs").contains("ord_type: req Code<OrdType> = ORD_TYPE,"));
}

#[test]
fn docs_are_copied_only_when_asked() {
    let base = workspace().join("dictionaries/orchestra/OrchestraFIX42.xml");
    let synopsis = "Unique identifier for Order as assigned by institution";
    let without = generate(&[base.as_os_str()]);
    assert!(!read(&without, "messages.rs").contains(synopsis));
    let with = generate(&[base.as_os_str(), "--docs".as_ref()]);
    assert!(read(&with, "messages.rs").contains(synopsis));
}

fn fixtures() -> std::path::PathBuf {
    workspace().join("crates/turbojet-dictionary/tests/fixtures")
}

#[test]
fn transport_adds_the_fixt_fields() {
    let (app, transport) = (fixtures().join("fix50.xml"), fixtures().join("fixt11.xml"));
    let out = generate(&[app.as_os_str(), "--transport".as_ref(), transport.as_os_str()]);
    assert!(read(&out, "messages.rs").contains("NewOrderSingle / NewOrderSingleRef = \"D\" {"));
    // ApplExtID is only in the transport.
    assert!(read(&out, "tags.rs").contains("pub const APPL_EXT_ID: u32 = 1156;"));
}

#[test]
fn transport_comes_before_merges_whatever_the_argument_order() {
    // The venue refers to ApplExtID, which only the transport defines.
    let venue = overlay(
        "<fix><messages>
          <message name='NewOrderSingle' msgtype='D' msgcat='app'>
           <field name='ClOrdID' required='Y'/><field name='ApplExtID' required='N'/>
          </message>
         </messages></fix>",
    );
    let (app, transport) = (fixtures().join("fix50.xml"), fixtures().join("fixt11.xml"));
    let out = generate(&[
        app.as_os_str(),
        "--merge".as_ref(),
        venue.path().as_os_str(),
        "--transport".as_ref(),
        transport.as_os_str(),
    ]);
    assert!(read(&out, "messages.rs").contains("appl_ext_id: opt i64 = APPL_EXT_ID,"));
}

#[test]
fn argument_errors_print_usage() {
    let dictionary = workspace().join("dictionaries/orchestra/OrchestraFIX42.xml");
    // Missing values, an unknown option, and no --out: none of them writes anything.
    for args in [&["--merge"][..], &["--out", "out", "--transport"], &["--bogus"], &[]] {
        let output = command().arg(&dictionary).args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(1), "{args:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.starts_with("turbojet-codegen: "), "{args:?}: {stderr}");
        assert!(stderr.contains("usage: turbojet-codegen"), "{args:?}: {stderr}");
    }
}
