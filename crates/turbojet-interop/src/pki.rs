//! Certificates for TLS scenarios: a CA that issues both sides' certificates, and a second CA
//! that issues none, for a side configured to trust the wrong one. PEM files in a directory that
//! lasts as long as the [`Pki`].

use std::path::{Path, PathBuf};

use rcgen::{BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair};
use tempfile::TempDir;

/// The certificates for one pair: `ca.pem` and `other-ca.pem`, and `tj` and `peer`, each a
/// certificate (`.pem`) and key (`.key`) issued by `ca`, for `localhost` and `127.0.0.1`, usable by
/// a server or a client.
pub struct Pki {
    dir: TempDir,
}

impl Pki {
    pub fn new() -> Self {
        let pki = Self { dir: TempDir::with_prefix("turbojet-interop-pki-").unwrap() };
        let ca = pki.ca("ca");
        pki.ca("other-ca");
        pki.issue("tj", &ca);
        pki.issue("peer", &ca);
        pki
    }

    /// The CA that issued both sides' certificates, or, if not `trusted`, the one that issued
    /// neither.
    pub fn ca_path(&self, trusted: bool) -> PathBuf {
        self.path(if trusted { "ca.pem" } else { "other-ca.pem" })
    }

    /// `side`'s certificate and key: `tj` or `peer`.
    pub fn identity_paths(&self, side: &str) -> (PathBuf, PathBuf) {
        (self.path(&format!("{side}.pem")), self.path(&format!("{side}.key")))
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    fn ca(&self, name: &str) -> Issuer<'static, KeyPair> {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.distinguished_name.push(DnType::CommonName, name);
        let cert = params.self_signed(&key).unwrap();
        write(&self.path(&format!("{name}.pem")), &cert.pem());
        Issuer::new(params, key)
    }

    fn issue(&self, name: &str, issuer: &Issuer<'_, KeyPair>) {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(vec!["localhost".to_string(), "127.0.0.1".to_string()]).unwrap();
        params.distinguished_name.push(DnType::CommonName, name);
        params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth, ExtendedKeyUsagePurpose::ClientAuth];
        let cert = params.signed_by(&key, issuer).unwrap();
        write(&self.path(&format!("{name}.pem")), &cert.pem());
        write(&self.path(&format!("{name}.key")), &key.serialize_pem());
    }
}

impl Default for Pki {
    fn default() -> Self {
        Self::new()
    }
}

fn write(path: &Path, contents: &str) {
    std::fs::write(path, contents).unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
}
