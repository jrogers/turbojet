//! TLS transport (feature `tls`), using rustls with the `ring` crypto provider.
//!
//! Build a [`TlsAcceptor`] for [`Acceptor::serve_tls`](crate::Acceptor::serve_tls) or a
//! [`TlsConnector`] for [`Initiator::with_tls`](crate::Initiator::with_tls):
//!
//! - with [`ServerTls`] and [`ClientTls`], from certificates given in memory or read from files
//!   ([`Identity`], [`Trust`]), which can be replaced while running: a renewed certificate, or a
//!   new CA, is used from the next handshake, and sessions already connected carry on;
//! - with [`acceptor`] and [`connector`], from PEM files, once;
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
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, CertificateRevocationListDer, PrivateKeyDer, UnixTime};
use tokio_rustls::rustls::server::danger::{ClientCertVerified, ClientCertVerifier};
use tokio_rustls::rustls::server::{ClientHello, NoServerSessionStorage, ResolvesServerCert, WebPkiClientVerifier};
use tokio_rustls::rustls::sign::CertifiedKey;
use tokio_rustls::rustls::{
    ClientConfig, DigitallySignedStruct, DistinguishedName, RootCertStore, ServerConfig, SignatureScheme,
};

use crate::peer::PeerCertificate;

pub use tokio_rustls::rustls;
pub use tokio_rustls::rustls::pki_types::ServerName;
pub use tokio_rustls::{TlsAcceptor, TlsConnector};

/// Whether an acceptor asks clients for a certificate (mutual TLS).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientAuth<'a> {
    /// Don't request client certificates.
    None,
    /// Request a client certificate but admit clients that don't send one. A certificate that is
    /// sent must be issued by one of the CAs in this PEM file.
    Optional(&'a Path),
    /// Require a client certificate issued by one of the CAs in this PEM file.
    Required(&'a Path),
}

/// Server side: present `cert_chain` (leaf first) with its private `key`, authenticating
/// clients according to `client_auth`. For certificates that can be replaced while running, use
/// [`ServerTls`].
///
/// # Errors
///
/// The file's error if one can't be read; [`InvalidData`](io::ErrorKind::InvalidData), naming the
/// file, if it holds no certificate or key, or one that doesn't parse, or if the key isn't the
/// certificate's. [`InvalidInput`](io::ErrorKind::InvalidInput) if the client CAs can't verify
/// clients.
pub fn acceptor(cert_chain: &Path, key: &Path, client_auth: ClientAuth<'_>) -> io::Result<TlsAcceptor> {
    let client_trust = match client_auth {
        ClientAuth::None => ClientTrust::None,
        ClientAuth::Optional(ca) => ClientTrust::Optional(Trust::from_pem_files(ca)?),
        ClientAuth::Required(ca) => ClientTrust::Required(Trust::from_pem_files(ca)?),
    };
    Ok(ServerTls::new(Identity::from_pem_files(cert_chain, key)?, client_trust)?.acceptor())
}

/// Client side: trust servers whose certificates chain to a CA in `ca`. With `identity`
/// (certificate chain, private key), present a client certificate for mutual TLS. For
/// certificates that can be replaced while running, use [`ClientTls`].
///
/// # Errors
///
/// The file's error if one can't be read; [`InvalidData`](io::ErrorKind::InvalidData), naming the
/// file, if it holds no certificate or key, or one that doesn't parse, or if the key isn't the
/// certificate's.
pub fn connector(ca: &Path, identity: Option<(&Path, &Path)>) -> io::Result<TlsConnector> {
    let identity = identity.map(|(cert_chain, key)| Identity::from_pem_files(cert_chain, key)).transpose()?;
    Ok(ClientTls::new(Trust::from_pem_files(ca)?, identity)?.connector())
}

/// A certificate chain (leaf first) and its private key: what one side presents. Built from
/// certificates in memory or read from files, and checked: the key must be the leaf
/// certificate's.
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
/// the chain, intermediates included. A CRL past its next update is still used. Nothing is fetched:
/// CRLs are only those given, so to take in a new one, build a `Trust` with it and give that to
/// [`ServerTls::set_client_trust`] or [`ClientTls::set_trust`], from the next handshake on.
#[derive(Debug, Clone)]
pub struct Trust {
    roots: Arc<RootCertStore>,
    crls: Vec<CertificateRevocationListDer<'static>>,
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
        Ok(Self { roots: Arc::new(roots), crls: Vec::new() })
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
}

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
    /// `client_trust`.
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
/// [`with_tls`](crate::Initiator::with_tls), and a change applies from the next handshake, which
/// is always a full one (sessions aren't resumed). Cheap to clone; clones share the certificates.
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
    /// `trust`.
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
    verifier: RwLock<Option<Arc<dyn ClientCertVerifier>>>,
    provider: Arc<CryptoProvider>,
}

impl CurrentClientTrust {
    fn get(&self) -> Option<Arc<dyn ClientCertVerifier>> {
        self.verifier.read().expect("TLS trust lock poisoned").clone()
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
        match self.get() {
            Some(verifier) => verifier.verify_client_cert(end_entity, intermediates, now),
            None => Err(rustls::Error::General("client certificates aren't accepted".into())),
        }
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
    verifier: RwLock<Arc<WebPkiServerVerifier>>,
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
        let verifier = self.verifier.read().expect("TLS trust lock poisoned").clone();
        verifier.verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)
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

fn client_verifier(
    client_trust: ClientTrust,
    provider: &Arc<CryptoProvider>,
) -> io::Result<Option<Arc<dyn ClientCertVerifier>>> {
    let (trust, optional) = match client_trust {
        ClientTrust::None => return Ok(None),
        ClientTrust::Optional(trust) => (trust, true),
        ClientTrust::Required(trust) => (trust, false),
    };
    let builder = WebPkiClientVerifier::builder_with_provider(trust.roots, provider.clone()).with_crls(trust.crls);
    let builder = if optional { builder.allow_unauthenticated() } else { builder };
    Ok(Some(builder.build().map_err(invalid_input)?))
}

fn server_verifier(trust: &Trust, provider: &Arc<CryptoProvider>) -> io::Result<Arc<WebPkiServerVerifier>> {
    WebPkiServerVerifier::builder_with_provider(trust.roots.clone(), provider.clone())
        .with_crls(trust.crls.iter().cloned())
        .build()
        .map_err(invalid_input)
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
