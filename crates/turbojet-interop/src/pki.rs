//! Certificates for TLS scenarios: a CA that issues both sides' certificates, and a second CA
//! that issues none, for a side configured to trust the wrong one. PEM files, and each side's
//! certificate and key as a PKCS#12 bundle too, in a directory that lasts as long as the [`Pki`].

use std::path::{Path, PathBuf};

use p12_keystore::{Certificate, KeyStore, KeyStoreEntry, PrivateKey, PrivateKeyChain};
use rcgen::{
    BasicConstraints, CertificateParams, CertificateRevocationListParams, DnType, ExtendedKeyUsagePurpose, IsCa,
    Issuer, KeyIdMethod, KeyPair, RevokedCertParams, SerialNumber,
};
use tempfile::TempDir;

/// The password of every bundle the [`Pki`] writes.
pub const BUNDLE_PASSWORD: &str = "turbojet";

/// The certificates for one pair: `ca.pem` and `other-ca.pem`, and `tj` and `peer`, each a
/// certificate (`.pem`) and key (`.key`) issued by `ca`, for `localhost` and `127.0.0.1`, usable by
/// a server or a client, and both in a bundle (`.pfx`) with `ca`'s certificate. And `ca`'s CRLs: `revokes-peer.crl.pem`, revoking the peer's
/// certificate, and `revokes-nothing.crl.pem`.
pub struct Pki {
    dir: TempDir,
}

impl Pki {
    pub fn new() -> Self {
        let pki = Self { dir: TempDir::with_prefix("turbojet-interop-pki-").unwrap() };
        let (ca, ca_der) = pki.ca("ca");
        pki.ca("other-ca");
        pki.issue("tj", &ca, &ca_der);
        pki.issue("peer", &ca, &ca_der);
        pki.crl("revokes-peer", &ca, &["peer"]);
        pki.crl("revokes-nothing", &ca, &[]);
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

    /// `side`'s certificate and key as a PKCS#12 bundle, under [`BUNDLE_PASSWORD`].
    pub fn bundle_path(&self, side: &str) -> PathBuf {
        self.path(&format!("{side}.pfx"))
    }

    /// A CRL from the CA that issued both sides' certificates.
    pub fn crl_path(&self, revokes_peer: bool) -> PathBuf {
        self.path(if revokes_peer { "revokes-peer.crl.pem" } else { "revokes-nothing.crl.pem" })
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    /// Writes a CA's certificate, returning it as an issuer for signing others, and in DER.
    fn ca(&self, name: &str) -> (Issuer<'static, KeyPair>, Vec<u8>) {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.distinguished_name.push(DnType::CommonName, name);
        let cert = params.self_signed(&key).unwrap();
        write(&self.path(&format!("{name}.pem")), &cert.pem());
        (Issuer::new(params, key), cert.der().to_vec())
    }

    fn issue(&self, name: &str, issuer: &Issuer<'_, KeyPair>, issuer_der: &[u8]) {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(vec!["localhost".to_string(), "127.0.0.1".to_string()]).unwrap();
        params.distinguished_name.push(DnType::CommonName, name);
        params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth, ExtendedKeyUsagePurpose::ClientAuth];
        params.serial_number = Some(serial(name));
        let cert = params.signed_by(&key, issuer).unwrap();
        write(&self.path(&format!("{name}.pem")), &cert.pem());
        write(&self.path(&format!("{name}.key")), &key.serialize_pem());
        let chain = [Certificate::from_der(cert.der()).unwrap(), Certificate::from_der(issuer_der).unwrap()];
        let chain = PrivateKeyChain::new(name, PrivateKey::from_der(&key.serialize_der()).unwrap(), chain);
        let mut bundle = KeyStore::new();
        bundle.add_entry(name, KeyStoreEntry::PrivateKeyChain(chain));
        let path = self.path(&format!("{name}.pfx"));
        std::fs::write(&path, bundle.writer(BUNDLE_PASSWORD).write().unwrap())
            .unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
    }
}

impl Pki {
    /// Writes `issuer`'s CRL revoking the certificates named in `revoked`.
    fn crl(&self, name: &str, issuer: &Issuer<'_, KeyPair>, revoked: &[&str]) {
        let revoked_certs = revoked
            .iter()
            .map(|cert| RevokedCertParams {
                serial_number: serial(cert),
                revocation_time: rcgen::date_time_ymd(2026, 1, 1),
                reason_code: None,
                invalidity_date: None,
            })
            .collect();
        let params = CertificateRevocationListParams {
            this_update: rcgen::date_time_ymd(2026, 1, 1),
            next_update: rcgen::date_time_ymd(2099, 1, 1),
            crl_number: SerialNumber::from(1u64),
            issuing_distribution_point: None,
            revoked_certs,
            key_identifier_method: KeyIdMethod::Sha256,
        };
        write(&self.path(&format!("{name}.crl.pem")), &params.signed_by(issuer).unwrap().pem().unwrap());
    }
}

/// A certificate's serial number, from its name, so a CRL can revoke it by name.
fn serial(name: &str) -> SerialNumber {
    SerialNumber::from_slice(name.as_bytes())
}

impl Default for Pki {
    fn default() -> Self {
        Self::new()
    }
}

fn write(path: &Path, contents: &str) {
    std::fs::write(path, contents).unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
}
