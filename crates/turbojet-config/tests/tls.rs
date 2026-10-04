//! `[acceptor.tls]`: read and checked when the file loads, and replaced by a reload.
#![cfg(feature = "tls")]

use std::path::Path;
use std::sync::Arc;

use rcgen::{CertificateParams, KeyPair};
use turbojet_config::SessionsFile;

struct App;
impl turbojet::Application for App {}

const ACCEPTOR: &str = r#"
[acceptor]
begin_string = "FIX.4.4"
sender_comp_id = "VENUE"
listen = "127.0.0.1:0"
"#;

/// Writes a self-signed certificate `NAME.pem` and its key `NAME.key` into `dir`.
fn certificate(dir: &Path, name: &str) {
    let key = KeyPair::generate().unwrap();
    let cert = CertificateParams::new(vec!["localhost".to_string()]).unwrap().self_signed(&key).unwrap();
    std::fs::write(dir.join(format!("{name}.pem")), cert.pem()).unwrap();
    std::fs::write(dir.join(format!("{name}.key")), key.serialize_pem()).unwrap();
}

fn write(dir: &Path, tls: &str) -> std::path::PathBuf {
    let path = dir.join("sessions.toml");
    std::fs::write(&path, format!("{ACCEPTOR}tls = {tls}\n")).unwrap();
    path
}

#[test]
fn certificates_are_checked_when_the_file_loads() {
    let dir = tempfile::tempdir().unwrap();
    certificate(dir.path(), "server");
    certificate(dir.path(), "other");
    let path = write(dir.path(), r#"{ cert = "server.pem", key = "server.key" }"#);
    let sessions = SessionsFile::load(&path).unwrap();
    assert!(sessions.server_tls().unwrap().is_some());
    for (tls, error) in [
        (r#"{ cert = "missing.pem", key = "server.key" }"#, "acceptor: tls: "),
        (r#"{ cert = "server.pem", key = "other.key" }"#, "acceptor: tls: "),
        (r#"{ cert = "server.pem", key = "server.key", client_certificate = "required" }"#, "needs a client_ca"),
    ] {
        let path = write(dir.path(), tls);
        let loaded = SessionsFile::load(&path).unwrap_err().to_string();
        assert!(loaded.contains(error), "{tls}: {loaded}");
    }
}

#[test]
fn a_reload_replaces_the_certificates_but_not_whether_tls_is_served() {
    let dir = tempfile::tempdir().unwrap();
    certificate(dir.path(), "server");
    certificate(dir.path(), "renewed");
    let path = write(dir.path(), r#"{ cert = "server.pem", key = "server.key" }"#);
    let sessions = SessionsFile::load(&path).unwrap();
    let acceptor = sessions.acceptor(Arc::new(App));
    let server = sessions.server_tls().unwrap().unwrap();
    write(dir.path(), r#"{ cert = "renewed.pem", key = "renewed.key" }"#);
    assert!(sessions.reload(&acceptor).unwrap().is_empty());
    assert!(sessions.server_tls().unwrap().is_some(), "the same server, its certificate replaced");
    drop(server);
    std::fs::write(&path, ACCEPTOR).unwrap();
    let error = sessions.reload(&acceptor).unwrap_err().to_string();
    assert!(error.contains("acceptor: tls: can't change until a restart"), "{error}");
}
