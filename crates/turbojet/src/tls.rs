//! TLS transport (feature `tls`), using rustls with the `ring` crypto provider.
//!
//! Build a [`TlsAcceptor`] for [`Acceptor::serve_tls`](crate::Acceptor::serve_tls) or a
//! [`TlsConnector`] for an initiator's [`InitiatorTls`](crate::InitiatorTls):
//!
//! - with [`ServerTls`] and [`ClientTls`], from certificates given in memory or read from files
//!   ([`Identity`], [`Trust`]), as PEM or (feature `pkcs12`) a PKCS#12 bundle, which can be
//!   replaced while running: a renewed certificate, or a
//!   new CA, is used from the next handshake, and sessions already connected carry on;
//! - or from your own [`rustls`] configuration via `TlsAcceptor::from(Arc<rustls::ServerConfig>)`
//!   / `TlsConnector::from(Arc<rustls::ClientConfig>)`.

use std::fmt;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};

use tokio_rustls::rustls::client::WebPkiServerVerifier;
use tokio_rustls::rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use tokio_rustls::rustls::client::{ResolvesClientCert, Resumption};
use tokio_rustls::rustls::crypto::{CryptoProvider, ring, verify_tls12_signature, verify_tls13_signature};
#[cfg(feature = "pkcs12")]
use tokio_rustls::rustls::pki_types::PrivatePkcs8KeyDer;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, CertificateRevocationListDer, PrivateKeyDer, UnixTime};
use tokio_rustls::rustls::server::danger::{ClientCertVerified, ClientCertVerifier};
use tokio_rustls::rustls::server::{ClientHello, NoServerSessionStorage, ResolvesServerCert, WebPkiClientVerifier};
use tokio_rustls::rustls::sign::CertifiedKey;
use tokio_rustls::rustls::{
    ClientConfig, DigitallySignedStruct, DistinguishedName, RootCertStore, ServerConfig, SignatureScheme,
};

use tracing::warn;

use crate::peer::PeerCertificate;

pub use tokio_rustls::rustls;
pub use tokio_rustls::rustls::pki_types::ServerName;
pub use tokio_rustls::{TlsAcceptor, TlsConnector};

/// A certificate chain (leaf first) and its private key: what one side presents. Built from
/// certificates in memory or read from files, and checked: the key must be the leaf
/// certificate's.
///
/// ```no_run
/// use turbojet::tls::Identity;
///
/// let pem = Identity::from_pem_files("server.pem".as_ref(), "server.key".as_ref())?;
/// let bundle = Identity::from_pkcs12_file("server.p12".as_ref(), "password")?; // feature pkcs12
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Clone)]
pub struct Identity(Arc<CertifiedKey>);

impl Identity {
    /// From DER: `chain` leaf first, and its private `key`.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if `chain` is empty, or `key` isn't a key for
    /// its leaf certificate.
    pub fn from_der(chain: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>) -> io::Result<Self> {
        if chain.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no certificates"));
        }
        let key = CertifiedKey::from_der(chain, key, &provider()).map_err(invalid_data)?;
        Ok(Self(Arc::new(key)))
    }

    /// From PEM text in memory: the certificate chain (leaf first), and the private key.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if the PEM doesn't parse, or as
    /// [`from_der`](Self::from_der).
    pub fn from_pem(chain: &[u8], key: &[u8]) -> io::Result<Self> {
        let chain = CertificateDer::pem_slice_iter(chain).collect::<Result<Vec<_>, _>>().map_err(invalid_data)?;
        Self::from_der(chain, PrivateKeyDer::from_pem_slice(key).map_err(invalid_data)?)
    }

    /// From PEM files: the certificate chain (leaf first), and the private key.
    ///
    /// # Errors
    ///
    /// Either file's error if it can't be read, or as [`from_pem`](Self::from_pem), naming the
    /// file.
    pub fn from_pem_files(chain: &Path, key: &Path) -> io::Result<Self> {
        Self::from_der(certificates(chain)?, private_key(key)?).map_err(|e| in_file(chain, e))
    }
}

#[cfg(feature = "pkcs12")]
impl Identity {
    /// From a PKCS#12 bundle (`.p12`, `.pfx`) in memory (feature `pkcs12`): its private key and
    /// that key's certificate chain, leaf first, decrypted with `password` (empty if it has none).
    /// Bundles encrypted with AES (OpenSSL 3's default) and the older 3DES and RC2 (older OpenSSL,
    /// Windows and Java exports) are read.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if `password` is wrong, the bundle doesn't
    /// parse, it holds no private key or more than one, the key has no certificate, or as
    /// [`from_der`](Self::from_der).
    pub fn from_pkcs12(bundle: &[u8], password: &str) -> io::Result<Self> {
        use p12_keystore::{KeyStore, KeyStoreEntry, Pkcs12ImportPolicy, error::Error};
        // Relaxed keeps a key that has no certificate, so it's refused as that rather than missed.
        let store = KeyStore::from_pkcs12(bundle, password, Pkcs12ImportPolicy::Relaxed).map_err(|e| match e {
            Error::MacError(_) => invalid_data("wrong password, or the bundle is damaged"),
            e => invalid_data(format!("not a PKCS#12 bundle that can be read: {e}")),
        })?;
        let mut keys = store.entries().filter_map(|(_, entry)| match entry {
            KeyStoreEntry::PrivateKeyChain(chain) => Some(chain),
            _ => None,
        });
        let chain = match (keys.next(), keys.next()) {
            (Some(chain), None) => chain,
            (None, _) => return Err(invalid_data("the bundle holds no private key")),
            (Some(_), Some(_)) => return Err(invalid_data("the bundle holds more than one private key")),
        };
        if chain.certs().is_empty() {
            return Err(invalid_data("the bundle's private key has no certificate"));
        }
        let certs = chain.certs().iter().map(|cert| CertificateDer::from(cert.as_der().to_vec())).collect();
        Self::from_der(certs, PrivatePkcs8KeyDer::from(chain.key().as_der().to_vec()).into())
    }

    /// [`from_pkcs12`](Self::from_pkcs12) from a file.
    ///
    /// # Errors
    ///
    /// The file's error if it can't be read, or as [`from_pkcs12`](Self::from_pkcs12), naming the
    /// file.
    pub fn from_pkcs12_file(path: &Path, password: &str) -> io::Result<Self> {
        let bundle = std::fs::read(path).map_err(|e| in_file(path, e))?;
        Self::from_pkcs12(&bundle, password).map_err(|e| in_file(path, e))
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity").field("certificates", &self.0.cert.len()).finish_non_exhaustive()
    }
}

/// The CAs trusted to have issued the other side's certificate, and optionally the certificate
/// revocation lists (CRLs) to check it against. Built from certificates in memory or read from
/// files; it holds at least one CA.
///
/// Revocation isn't checked unless CRLs are given ([`with_crls_pem`](Self::with_crls_pem) and the
/// like). With them, every certificate in the chain up to the trusted CA is checked, and one that
/// no CRL covers is refused, since its status is unknown: give a CRL from each CA that issues in
/// the chain, intermediates included.
///
/// A CRL past its next update is still used, so that a CRL refreshed late doesn't lock out every
/// counterparty it covers, but it's logged as a warning: when the trust is put in use
/// ([`ServerTls::new`], [`ClientTls::new`] and their `set_` methods), and at each handshake that
/// relies on it. A certificate revoked since then is let in. [`refuse_expired_crls`](Self::refuse_expired_crls)
/// refuses them instead.
///
/// Nothing is fetched: CRLs are only those given, so to take in a new one, build a `Trust` with
/// it and give that to [`ServerTls::set_client_trust`] or [`ClientTls::set_trust`], from the next
/// handshake on.
///
/// ```no_run
/// use turbojet::tls::Trust;
///
/// let cas = Trust::from_pem_files("ca.pem".as_ref())?;
/// let checked = Trust::from_pem_files("ca.pem".as_ref())?.with_crls_pem_files("ca.crl.pem".as_ref())?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct Trust {
    roots: Arc<RootCertStore>,
    crls: Vec<CertificateRevocationListDer<'static>>,
    /// The CRL whose next update comes first, if any has one.
    first_expiry: Option<CrlExpiry>,
    refuse_expired_crls: bool,
}

/// When a CRL's next update is due, to tell when it goes stale.
#[derive(Debug, Clone)]
struct CrlExpiry {
    issuer: String,
    /// Seconds since the Unix epoch.
    next_update: i64,
}

impl CrlExpiry {
    fn has_passed(&self, now: UnixTime) -> bool {
        i64::try_from(now.as_secs()).unwrap_or(i64::MAX) > self.next_update
    }

    /// The next update, readably.
    fn next_update(&self) -> String {
        x509_parser::time::ASN1Time::from_timestamp(self.next_update)
            .map_or_else(|_| self.next_update.to_string(), |time| time.to_string())
    }
}

impl Trust {
    /// From DER CA certificates.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if there are none, or one isn't a usable CA
    /// certificate.
    pub fn from_der(cas: impl IntoIterator<Item = CertificateDer<'static>>) -> io::Result<Self> {
        let mut roots = RootCertStore::empty();
        for ca in cas {
            roots.add(ca).map_err(invalid_data)?;
        }
        if roots.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no CA certificates"));
        }
        Ok(Self { roots: Arc::new(roots), crls: Vec::new(), first_expiry: None, refuse_expired_crls: false })
    }

    /// From PEM text in memory holding one or more CA certificates.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if the PEM doesn't parse, or as
    /// [`from_der`](Self::from_der).
    pub fn from_pem(cas: &[u8]) -> io::Result<Self> {
        Self::from_der(CertificateDer::pem_slice_iter(cas).collect::<Result<Vec<_>, _>>().map_err(invalid_data)?)
    }

    /// From a PEM file holding one or more CA certificates.
    ///
    /// # Errors
    ///
    /// The file's error if it can't be read, or as [`from_pem`](Self::from_pem), naming the file.
    pub fn from_pem_files(cas: &Path) -> io::Result<Self> {
        Self::from_der(certificates(cas)?).map_err(|e| in_file(cas, e))
    }

    /// Also refuses certificates these DER CRLs revoke, as well as those they don't cover (see
    /// [`Trust`]). Adds to the CRLs already given.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if there are none, or one doesn't parse.
    pub fn with_crls_der(
        mut self,
        crls: impl IntoIterator<Item = CertificateRevocationListDer<'static>>,
    ) -> io::Result<Self> {
        let count = self.crls.len();
        self.crls.extend(crls);
        if self.crls.len() == count {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no CRLs"));
        }
        // Built once here so a bad CRL is refused now, not at the first handshake.
        WebPkiServerVerifier::builder_with_provider(self.roots.clone(), provider())
            .with_crls(self.crls.iter().cloned())
            .build()
            .map_err(invalid_data)?;
        for crl in &self.crls[count..] {
            let (_, parsed) = x509_parser::parse_x509_crl(crl.as_ref()).map_err(invalid_data)?;
            let Some(next_update) = parsed.next_update() else { continue };
            if self.first_expiry.as_ref().is_none_or(|first| next_update.timestamp() < first.next_update) {
                self.first_expiry =
                    Some(CrlExpiry { issuer: parsed.issuer().to_string(), next_update: next_update.timestamp() });
            }
        }
        Ok(self)
    }

    /// As [`with_crls_der`](Self::with_crls_der), from PEM text in memory holding one or more
    /// CRLs.
    ///
    /// # Errors
    ///
    /// [`InvalidData`](io::ErrorKind::InvalidData) if the PEM doesn't parse, or as
    /// [`with_crls_der`](Self::with_crls_der).
    pub fn with_crls_pem(self, crls: &[u8]) -> io::Result<Self> {
        let crls =
            CertificateRevocationListDer::pem_slice_iter(crls).collect::<Result<Vec<_>, _>>().map_err(invalid_data)?;
        self.with_crls_der(crls)
    }

    /// As [`with_crls_der`](Self::with_crls_der), from a PEM file holding one or more CRLs.
    ///
    /// # Errors
    ///
    /// The file's error if it can't be read, or as [`with_crls_pem`](Self::with_crls_pem), naming
    /// the file.
    pub fn with_crls_pem_files(self, path: &Path) -> io::Result<Self> {
        let crls = CertificateRevocationListDer::pem_file_iter(path)
            .and_then(|iter| iter.collect::<Result<Vec<_>, _>>())
            .map_err(|e| pem_error(path, e))?;
        self.with_crls_der(crls).map_err(|e| in_file(path, e))
    }

    /// Refuses CRLs past their next update rather than using them: putting the trust in use is
    /// then an error while one has expired, and a CRL that expires while in use fails each
    /// handshake from then on ("certificate revocation list expired"), until a current one is
    /// given. Safer, since a certificate revoked since can't get in, but a CRL refreshed late
    /// locks out every counterparty it covers.
    #[must_use]
    pub fn refuse_expired_crls(mut self) -> Self {
        self.refuse_expired_crls = true;
        self
    }

    /// Checks the CRLs as the trust is put in use: one past its next update is refused, or
    /// logged. Returns the expiry to watch at each handshake, if it's to be logged.
    fn check_crls(&self) -> io::Result<Option<CrlExpiry>> {
        let Some(first) = &self.first_expiry else { return Ok(None) };
        let expired = first.has_passed(UnixTime::now());
        if expired && self.refuse_expired_crls {
            let message =
                format!("the CRL from {} expired at {}: give a current one", first.issuer, first.next_update());
            return Err(io::Error::new(io::ErrorKind::InvalidData, message));
        }
        if expired {
            warn!(issuer = %first.issuer, next_update = %first.next_update(), "{STALE_CRL}");
        }
        Ok((!self.refuse_expired_crls).then(|| first.clone()))
    }
}

/// What's logged for a CRL past its next update.
const STALE_CRL: &str = "a CRL past its next update is in use: certificates its CA has revoked since are let in";

/// Whether an acceptor asks clients for a certificate, and which CAs it trusts to issue one.
#[derive(Debug, Clone)]
pub enum ClientTrust {
    /// Don't request client certificates.
    None,
    /// Request a client certificate but admit clients that don't send one. A certificate that is
    /// sent must be issued by one of these CAs.
    Optional(Trust),
    /// Require a client certificate issued by one of these CAs.
    Required(Trust),
}

/// TLS for an [`Acceptor`](crate::Acceptor) whose certificate and trusted client CAs can be
/// replaced while it runs: [`acceptor`](Self::acceptor) is for
/// [`serve_tls`](crate::Acceptor::serve_tls), and a change applies from the next handshake.
/// Sessions already connected carry on. Every handshake is a full one, checked against the
/// certificates current then: sessions aren't resumed (a FIX connection lasts hours, so resuming
/// would save little). Cheap to clone; clones share the certificates.
///
/// With mutual TLS, no CA names are suggested to clients in the handshake (TLS makes the
/// suggestion optional): a client with one certificate sends it regardless.
///
/// ```no_run
/// use turbojet::tls::{ClientTrust, Identity, ServerTls, Trust};
///
/// let identity = Identity::from_pem_files("server.pem".as_ref(), "server.key".as_ref())?;
/// let server = ServerTls::new(identity, ClientTrust::Required(Trust::from_pem_files("clients-ca.pem".as_ref())?))?;
/// let tls = server.acceptor(); // for Acceptor::serve_tls
/// // On renewal: handshakes from now on use the new certificate.
/// server.set_identity(Identity::from_pem_files("renewed.pem".as_ref(), "renewed.key".as_ref())?);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Clone)]
pub struct ServerTls {
    identity: Arc<CurrentIdentity>,
    client_trust: Arc<CurrentClientTrust>,
    acceptor: TlsAcceptor,
}

impl ServerTls {
    /// Presents `identity`, authenticating clients as `client_trust` says.
    ///
    /// # Errors
    ///
    /// [`InvalidInput`](io::ErrorKind::InvalidInput) if no client verifier can be built from
    /// `client_trust`; [`InvalidData`](io::ErrorKind::InvalidData) if one of its CRLs has expired
    /// and it [refuses expired CRLs](Trust::refuse_expired_crls).
    pub fn new(identity: Identity, client_trust: ClientTrust) -> io::Result<Self> {
        let provider = provider();
        let identity = Arc::new(CurrentIdentity(RwLock::new(Some(identity.0))));
        let client_trust = Arc::new(CurrentClientTrust {
            verifier: RwLock::new(client_verifier(client_trust, &provider)?),
            provider: provider.clone(),
        });
        let mut config = ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .map_err(invalid_input)?
            .with_client_cert_verifier(client_trust.clone())
            .with_cert_resolver(identity.clone());
        // No resumption, so every handshake checks the current certificates and trust: a client
        // whose CA was removed can't resume a session from before.
        config.session_storage = Arc::new(NoServerSessionStorage {});
        config.send_tls13_tickets = 0;
        Ok(Self { identity, client_trust, acceptor: TlsAcceptor::from(Arc::new(config)) })
    }

    /// The acceptor to serve with, presenting whatever identity is current at each handshake.
    pub fn acceptor(&self) -> TlsAcceptor {
        self.acceptor.clone()
    }

    /// Presents `identity` from the next handshake on: a renewed certificate, say.
    pub fn set_identity(&self, identity: Identity) {
        *self.identity.0.write().expect("TLS identity lock poisoned") = Some(identity.0);
    }

    /// Authenticates clients as `client_trust` says from the next handshake on.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new); the trust in use is kept.
    pub fn set_client_trust(&self, client_trust: ClientTrust) -> io::Result<()> {
        let verifier = client_verifier(client_trust, &self.client_trust.provider)?;
        *self.client_trust.verifier.write().expect("TLS trust lock poisoned") = verifier;
        Ok(())
    }
}

impl fmt::Debug for ServerTls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerTls").finish_non_exhaustive()
    }
}

/// TLS for an [`Initiator`](crate::Initiator) whose trusted CAs and client certificate can be
/// replaced while it runs: [`connector`](Self::connector) is for
/// [`InitiatorTls`](crate::InitiatorTls), and a change applies from the next handshake, which
/// is always a full one (sessions aren't resumed). Cheap to clone; clones share the certificates.
///
/// ```no_run
/// use turbojet::InitiatorTls;
/// use turbojet::tls::{ClientTls, Identity, Trust};
///
/// let client = ClientTls::new(
///     Trust::from_pem_files("venue-ca.pem".as_ref())?,
///     Some(Identity::from_pem_files("client.pem".as_ref(), "client.key".as_ref())?),
/// )?;
/// let tls = InitiatorTls::new(client.connector(), "venue.example.com"); // for InitiatorConfig::tls
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Clone)]
pub struct ClientTls {
    trust: Arc<CurrentServerTrust>,
    identity: Arc<CurrentIdentity>,
    connector: TlsConnector,
}

impl ClientTls {
    /// Trusts servers whose certificates chain to `trust`, presenting `identity` if a server asks
    /// for a client certificate (mutual TLS).
    ///
    /// # Errors
    ///
    /// [`InvalidInput`](io::ErrorKind::InvalidInput) if no server verifier can be built from
    /// `trust`; [`InvalidData`](io::ErrorKind::InvalidData) if one of its CRLs has expired and it
    /// [refuses expired CRLs](Trust::refuse_expired_crls).
    pub fn new(trust: Trust, identity: Option<Identity>) -> io::Result<Self> {
        let provider = provider();
        let trust = Arc::new(CurrentServerTrust {
            verifier: RwLock::new(server_verifier(&trust, &provider)?),
            provider: provider.clone(),
        });
        let identity = Arc::new(CurrentIdentity(RwLock::new(identity.map(|identity| identity.0))));
        let mut config = ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .map_err(invalid_input)?
            .dangerous()
            .with_custom_certificate_verifier(trust.clone())
            .with_client_cert_resolver(identity.clone());
        // No resumption, so every handshake checks the current trust.
        config.resumption = Resumption::disabled();
        Ok(Self { trust, identity, connector: TlsConnector::from(Arc::new(config)) })
    }

    /// The connector to connect with, trusting and presenting whatever is current at each
    /// handshake.
    pub fn connector(&self) -> TlsConnector {
        self.connector.clone()
    }

    /// Trusts servers whose certificates chain to `trust` from the next handshake on.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new); the trust in use is kept.
    pub fn set_trust(&self, trust: Trust) -> io::Result<()> {
        let verifier = server_verifier(&trust, &self.trust.provider)?;
        *self.trust.verifier.write().expect("TLS trust lock poisoned") = verifier;
        Ok(())
    }

    /// Presents `identity` (or none) from the next handshake on.
    pub fn set_identity(&self, identity: Option<Identity>) {
        *self.identity.0.write().expect("TLS identity lock poisoned") = identity.map(|identity| identity.0);
    }
}

impl fmt::Debug for ClientTls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientTls").finish_non_exhaustive()
    }
}

/// The identity a side presents now, read at each handshake.
#[derive(Debug)]
struct CurrentIdentity(RwLock<Option<Arc<CertifiedKey>>>);

impl CurrentIdentity {
    fn get(&self) -> Option<Arc<CertifiedKey>> {
        self.0.read().expect("TLS identity lock poisoned").clone()
    }
}

impl ResolvesServerCert for CurrentIdentity {
    fn resolve(&self, _client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        self.get()
    }
}

impl ResolvesClientCert for CurrentIdentity {
    fn resolve(&self, _root_hint_subjects: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<Arc<CertifiedKey>> {
        self.get()
    }

    fn has_certs(&self) -> bool {
        self.0.read().expect("TLS identity lock poisoned").is_some()
    }
}

/// How an acceptor authenticates clients now, read at each handshake: `None` asks for no
/// certificate.
struct CurrentClientTrust {
    /// The verifier, and the expiry of a CRL to warn about at each handshake once it's passed.
    verifier: RwLock<ClientVerifier>,
    provider: Arc<CryptoProvider>,
}

impl CurrentClientTrust {
    fn get(&self) -> Option<Arc<dyn ClientCertVerifier>> {
        self.verifier.read().expect("TLS trust lock poisoned").0.clone()
    }
}

impl fmt::Debug for CurrentClientTrust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CurrentClientTrust").finish_non_exhaustive()
    }
}

impl ClientCertVerifier for CurrentClientTrust {
    fn offer_client_auth(&self) -> bool {
        self.get().is_some_and(|verifier| verifier.offer_client_auth())
    }

    fn client_auth_mandatory(&self) -> bool {
        self.get().is_some_and(|verifier| verifier.client_auth_mandatory())
    }

    /// None: the trusted CAs may be replaced, so there's nothing to lend from, and suggesting
    /// them is optional.
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: UnixTime,
    ) -> Result<ClientCertVerified, rustls::Error> {
        let (verifier, expiry) = self.verifier.read().expect("TLS trust lock poisoned").clone();
        let Some(verifier) = verifier else {
            return Err(rustls::Error::General("client certificates aren't accepted".into()));
        };
        let verified = verifier.verify_client_cert(end_entity, intermediates, now)?;
        warn_if_stale(expiry.as_ref(), now);
        Ok(verified)
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider.signature_verification_algorithms.supported_schemes()
    }
}

/// The CAs an initiator trusts now, read at each handshake.
struct CurrentServerTrust {
    /// The verifier, and the expiry of a CRL to warn about at each handshake once it's passed.
    verifier: RwLock<(Arc<WebPkiServerVerifier>, Option<CrlExpiry>)>,
    provider: Arc<CryptoProvider>,
}

impl fmt::Debug for CurrentServerTrust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CurrentServerTrust").finish_non_exhaustive()
    }
}

impl ServerCertVerifier for CurrentServerTrust {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let (verifier, expiry) = self.verifier.read().expect("TLS trust lock poisoned").clone();
        let verified = verifier.verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)?;
        warn_if_stale(expiry.as_ref(), now);
        Ok(verified)
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider.signature_verification_algorithms.supported_schemes()
    }
}

/// An acceptor's client verifier (`None` asks for no certificate), and the expiry of a CRL to
/// warn about at each handshake once it's passed.
type ClientVerifier = (Option<Arc<dyn ClientCertVerifier>>, Option<CrlExpiry>);

/// A certificate was verified against CRLs that may have gone stale: say so if they have.
fn warn_if_stale(expiry: Option<&CrlExpiry>, now: UnixTime) {
    if let Some(expiry) = expiry
        && expiry.has_passed(now)
    {
        warn!(issuer = %expiry.issuer, next_update = %expiry.next_update(), "{STALE_CRL}");
    }
}

fn client_verifier(client_trust: ClientTrust, provider: &Arc<CryptoProvider>) -> io::Result<ClientVerifier> {
    let (trust, optional) = match client_trust {
        ClientTrust::None => return Ok((None, None)),
        ClientTrust::Optional(trust) => (trust, true),
        ClientTrust::Required(trust) => (trust, false),
    };
    let expiry = trust.check_crls()?;
    let builder = WebPkiClientVerifier::builder_with_provider(trust.roots, provider.clone()).with_crls(trust.crls);
    let builder = if optional { builder.allow_unauthenticated() } else { builder };
    let builder = if trust.refuse_expired_crls { builder.enforce_revocation_expiration() } else { builder };
    Ok((Some(builder.build().map_err(invalid_input)?), expiry))
}

fn server_verifier(
    trust: &Trust,
    provider: &Arc<CryptoProvider>,
) -> io::Result<(Arc<WebPkiServerVerifier>, Option<CrlExpiry>)> {
    let expiry = trust.check_crls()?;
    let builder = WebPkiServerVerifier::builder_with_provider(trust.roots.clone(), provider.clone())
        .with_crls(trust.crls.iter().cloned());
    let builder = if trust.refuse_expired_crls { builder.enforce_revocation_expiration() } else { builder };
    Ok((builder.build().map_err(invalid_input)?, expiry))
}

/// Converts a completed handshake's peer certificates for [`crate::ConnectionInfo`].
pub(crate) fn peer_certificates(certs: Option<&[CertificateDer<'_>]>) -> Vec<PeerCertificate> {
    certs.unwrap_or_default().iter().map(|cert| PeerCertificate::from_der(cert.as_ref())).collect()
}

fn provider() -> Arc<CryptoProvider> {
    Arc::new(ring::default_provider())
}

fn certificates(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    let certs = CertificateDer::pem_file_iter(path)
        .and_then(|iter| iter.collect::<Result<Vec<_>, _>>())
        .map_err(|e| pem_error(path, e))?;
    if certs.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}: no certificates found", path.display())));
    }
    Ok(certs)
}

fn private_key(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    PrivateKeyDer::from_pem_file(path).map_err(|e| pem_error(path, e))
}

/// `e`, saying it's about `path`.
fn in_file(path: &Path, e: io::Error) -> io::Error {
    io::Error::new(e.kind(), format!("{}: {e}", path.display()))
}

fn invalid_data(e: impl fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
}

fn pem_error(path: &Path, e: rustls::pki_types::pem::Error) -> io::Error {
    match e {
        rustls::pki_types::pem::Error::Io(e) => io::Error::new(e.kind(), format!("{}: {e}", path.display())),
        e => io::Error::new(io::ErrorKind::InvalidData, format!("{}: {e}", path.display())),
    }
}

fn invalid_input(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
}
