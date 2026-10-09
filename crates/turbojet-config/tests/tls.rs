//! `[acceptor.tls]`: read and checked when the file loads, and replaced by a reload.
#![cfg(feature = "tls")]

use std::path::Path;
use std::sync::Arc;

use rcgen::{CertificateParams, CertificateRevocationListParams, Issuer, KeyIdMethod, KeyPair, SerialNumber};
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

/// Writes `NAME.crl.pem`, a CRL revoking nothing, signed by a key of its own: enough to load.
fn crl(dir: &Path, name: &str) {
    let issuer = Issuer::new(CertificateParams::new(Vec::<String>::new()).unwrap(), KeyPair::generate().unwrap());
    let params = CertificateRevocationListParams {
        this_update: rcgen::date_time_ymd(2026, 1, 1),
        next_update: rcgen::date_time_ymd(2099, 1, 1),
        crl_number: SerialNumber::from(1u64),
        issuing_distribution_point: None,
        revoked_certs: Vec::new(),
        key_identifier_method: KeyIdMethod::Sha256,
    };
    std::fs::write(dir.join(format!("{name}.crl.pem")), params.signed_by(&issuer).unwrap().pem().unwrap()).unwrap();
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
    let acceptor = sessions.acceptor(Arc::new(App)).unwrap();
    let server = sessions.server_tls().unwrap().unwrap();
    write(dir.path(), r#"{ cert = "renewed.pem", key = "renewed.key" }"#);
    assert!(sessions.reload(&acceptor).unwrap().is_empty());
    assert!(sessions.server_tls().unwrap().is_some(), "the same server, its certificate replaced");
    drop(server);
    std::fs::write(&path, ACCEPTOR).unwrap();
    let error = sessions.reload(&acceptor).unwrap_err().to_string();
    assert!(error.contains("acceptor: tls: can't change until a restart"), "{error}");
}

struct Nothing;
impl turbojet::Application for Nothing {}

#[tokio::test]
async fn an_initiators_tls_is_checked_when_the_file_loads() {
    let dir = tempfile::tempdir().unwrap();
    certificate(dir.path(), "ca");
    certificate(dir.path(), "firm");
    let initiator = |tls: &str| {
        format!(
            "[initiator.LSE]\nbegin_string = \"FIX.4.4\"\nsender_comp_id = \"FIRM\"\ntarget_comp_id = \"LSE\"\n\
             connect = [\"localhost:1\"]\ntls = {tls}\n"
        )
    };
    let path = dir.path().join("sessions.toml");
    for tls in [
        r#"{ ca = "ca.pem" }"#,
        r#"{ ca = "ca.pem", cert = "firm.pem", key = "firm.key", server_name = "fix.lse.com" }"#,
    ] {
        std::fs::write(&path, initiator(tls)).unwrap();
        let sessions = SessionsFile::load(&path).unwrap();
        let initiators = sessions.initiators(Arc::new(Nothing)).unwrap();
        assert_eq!(initiators.names(), ["LSE"], "{tls}");
        initiators.shutdown(None).await;
    }
    for (tls, error) in [
        (r#"{ ca = "missing.pem" }"#, "initiator LSE: tls: "),
        (r#"{ ca = "ca.pem", cert = "firm.pem" }"#, "initiator LSE: tls: cert and key go together"),
        (r#"{ ca = "ca.pem", server_name = "not a name" }"#, "invalid server_name"),
    ] {
        std::fs::write(&path, initiator(tls)).unwrap();
        let loaded = SessionsFile::load(&path).unwrap_err().to_string();
        assert!(loaded.contains(error), "{tls}: {loaded}");
    }
}

/// CRLs are optional, and when given are read and checked as the file loads.
#[test]
fn crl_files_are_checked_when_the_file_loads() {
    let dir = tempfile::tempdir().unwrap();
    certificate(dir.path(), "server");
    certificate(dir.path(), "ca");
    crl(dir.path(), "ca");
    std::fs::write(dir.path().join("garbled.crl.pem"), "-----BEGIN X509 CRL-----\nAAAA\n-----END X509 CRL-----\n")
        .unwrap();
    let path = write(
        dir.path(),
        r#"{ cert = "server.pem", key = "server.key", client_ca = "ca.pem", client_crl = "ca.crl.pem" }"#,
    );
    assert!(SessionsFile::load(&path).unwrap().server_tls().unwrap().is_some());
    for (tls, error) in [
        (
            r#"{ cert = "server.pem", key = "server.key", client_ca = "ca.pem", client_crl = "missing.crl.pem" }"#,
            "missing.crl.pem",
        ),
        (
            r#"{ cert = "server.pem", key = "server.key", client_ca = "ca.pem", client_crl = "garbled.crl.pem" }"#,
            "garbled.crl.pem",
        ),
        (r#"{ cert = "server.pem", key = "server.key", client_crl = "ca.crl.pem" }"#, "client_crl needs a client_ca"),
    ] {
        let path = write(dir.path(), tls);
        let loaded = SessionsFile::load(&path).unwrap_err().to_string();
        assert!(loaded.contains(error), "{tls}: {loaded}");
    }

    let initiator = |crl: &str| {
        format!(
            "[initiator.LSE]\nbegin_string = \"FIX.4.4\"\nsender_comp_id = \"FIRM\"\ntarget_comp_id = \"LSE\"\n\
             connect = [\"localhost:1\"]\ntls = {{ ca = \"ca.pem\", crl = \"{crl}\" }}\n"
        )
    };
    std::fs::write(&path, initiator("ca.crl.pem")).unwrap();
    assert!(SessionsFile::load(&path).is_ok());
    std::fs::write(&path, initiator("missing.crl.pem")).unwrap();
    let loaded = SessionsFile::load(&path).unwrap_err().to_string();
    assert!(loaded.contains("initiator LSE: tls: ") && loaded.contains("missing.crl.pem"), "{loaded}");
}

/// Writes `NAME.p12`, a bundle of a self-signed certificate and its key under `password`.
#[cfg(feature = "pkcs12")]
fn bundle(dir: &Path, name: &str, password: &str) {
    use p12_keystore::{Certificate, KeyStore, KeyStoreEntry, PrivateKey, PrivateKeyChain};
    let key = KeyPair::generate().unwrap();
    let cert = CertificateParams::new(vec!["localhost".to_string()]).unwrap().self_signed(&key).unwrap();
    let key = PrivateKey::from_der(&key.serialize_der()).unwrap();
    let chain = PrivateKeyChain::new(name, key, [Certificate::from_der(cert.der()).unwrap()]);
    let mut store = KeyStore::new();
    store.add_entry(name, KeyStoreEntry::PrivateKeyChain(chain));
    std::fs::write(dir.join(format!("{name}.p12")), store.writer(password).write().unwrap()).unwrap();
}

/// A bundle's password, from a variable every test process has: tests can't set one.
#[cfg(feature = "pkcs12")]
fn password() -> String {
    std::env::var("PATH").unwrap()
}

#[cfg(feature = "pkcs12")]
#[test]
fn an_acceptors_certificate_can_come_from_a_bundle() {
    let dir = tempfile::tempdir().unwrap();
    bundle(dir.path(), "server", &password());
    bundle(dir.path(), "open", "");
    certificate(dir.path(), "server");
    for tls in [r#"{ pkcs12 = "server.p12", pkcs12_password_env = "PATH" }"#, r#"{ pkcs12 = "open.p12" }"#] {
        let sessions = SessionsFile::load(write(dir.path(), tls)).unwrap();
        assert!(sessions.server_tls().unwrap().is_some(), "{tls}");
    }
    for (tls, error) in [
        (r#"{ pkcs12 = "server.p12" }"#, "wrong password"),
        (
            r#"{ pkcs12 = "server.p12", pkcs12_password_env = "TURBOJET_NO_SUCH_VARIABLE" }"#,
            "TURBOJET_NO_SUCH_VARIABLE",
        ),
        (r#"{ pkcs12 = "missing.p12" }"#, "missing.p12"),
        (
            r#"{ cert = "server.pem", key = "server.key", pkcs12 = "server.p12" }"#,
            "pkcs12 takes the place of cert and key",
        ),
        (
            r#"{ cert = "server.pem", key = "server.key", pkcs12_password_env = "PATH" }"#,
            "pkcs12_password_env needs a pkcs12",
        ),
        (r#"{ cert = "server.pem" }"#, "cert and key go together"),
        (r#"{ client_certificate = "optional" }"#, "needs cert and key, or pkcs12"),
    ] {
        let loaded = SessionsFile::load(write(dir.path(), tls)).unwrap_err().to_string();
        assert!(loaded.contains("acceptor: tls: ") && loaded.contains(error), "{tls}: {loaded}");
    }
}

#[cfg(feature = "pkcs12")]
#[test]
fn a_reload_reads_the_bundle_again() {
    let dir = tempfile::tempdir().unwrap();
    bundle(dir.path(), "server", &password());
    let path = write(dir.path(), r#"{ pkcs12 = "server.p12", pkcs12_password_env = "PATH" }"#);
    let sessions = SessionsFile::load(&path).unwrap();
    let acceptor = sessions.acceptor(Arc::new(App)).unwrap();
    bundle(dir.path(), "server", "renewed under a new password");
    let error = sessions.reload(&acceptor).unwrap_err().to_string();
    assert!(error.contains("wrong password"), "the bundle is read again: {error}");
    bundle(dir.path(), "server", &password());
    assert!(sessions.reload(&acceptor).unwrap().is_empty());
    assert!(sessions.server_tls().unwrap().is_some());
}

#[cfg(feature = "pkcs12")]
#[tokio::test]
async fn an_initiators_certificate_can_come_from_a_bundle() {
    let dir = tempfile::tempdir().unwrap();
    certificate(dir.path(), "ca");
    certificate(dir.path(), "firm");
    bundle(dir.path(), "firm", &password());
    let path = dir.path().join("sessions.toml");
    let initiator = |tls: &str| {
        format!(
            "[initiator.LSE]\nbegin_string = \"FIX.4.4\"\nsender_comp_id = \"FIRM\"\ntarget_comp_id = \"LSE\"\n\
             connect = [\"localhost:1\"]\ntls = {tls}\n"
        )
    };
    std::fs::write(&path, initiator(r#"{ ca = "ca.pem", pkcs12 = "firm.p12", pkcs12_password_env = "PATH" }"#))
        .unwrap();
    let initiators = SessionsFile::load(&path).unwrap().initiators(Arc::new(Nothing)).unwrap();
    assert_eq!(initiators.names(), ["LSE"]);
    initiators.shutdown(None).await;
    std::fs::write(&path, initiator(r#"{ ca = "ca.pem", cert = "firm.pem", pkcs12 = "firm.p12" }"#)).unwrap();
    let loaded = SessionsFile::load(&path).unwrap_err().to_string();
    assert!(loaded.contains("initiator LSE: tls: pkcs12 takes the place of cert and key"), "{loaded}");
}

#[cfg(not(feature = "pkcs12"))]
#[test]
fn bundles_need_the_pkcs12_feature() {
    let dir = tempfile::tempdir().unwrap();
    let loaded = SessionsFile::load(write(dir.path(), r#"{ pkcs12 = "server.p12" }"#)).unwrap_err().to_string();
    assert!(loaded.contains("needs turbojet-config's pkcs12 feature"), "{loaded}");
}
