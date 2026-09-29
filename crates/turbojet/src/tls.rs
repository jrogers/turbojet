//! TLS transport (feature `tls`), using rustls with the `ring` crypto provider.
//!
//! Build a [`TlsAcceptor`] for [`Acceptor::serve_tls`](crate::Acceptor::serve_tls) or a
//! [`TlsConnector`] for [`Initiator::with_tls`](crate::Initiator::with_tls) from PEM files with
//! [`acceptor`] and [`connector`], or from your own [`rustls`] configuration via
//! `TlsAcceptor::from(Arc<rustls::ServerConfig>)` / `TlsConnector::from(Arc<rustls::ClientConfig>)`.

use std::io;
use std::path::Path;
use std::sync::Arc;

use tokio_rustls::rustls::crypto::{CryptoProvider, ring};
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::rustls::server::WebPkiClientVerifier;
use tokio_rustls::rustls::{ClientConfig, RootCertStore, ServerConfig};

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
/// clients according to `client_auth`.
pub fn acceptor(cert_chain: &Path, key: &Path, client_auth: ClientAuth<'_>) -> io::Result<TlsAcceptor> {
    let provider = provider();
    let builder = ServerConfig::builder_with_provider(provider.clone())
        .with_safe_default_protocol_versions()
        .map_err(invalid_input)?;
    let builder = match client_auth {
        ClientAuth::None => builder.with_no_client_auth(),
        ClientAuth::Optional(ca) | ClientAuth::Required(ca) => {
            let verifier = WebPkiClientVerifier::builder_with_provider(Arc::new(root_store(ca)?), provider);
            let verifier = match client_auth {
                ClientAuth::Optional(_) => verifier.allow_unauthenticated(),
                _ => verifier,
            };
            builder.with_client_cert_verifier(verifier.build().map_err(invalid_input)?)
        }
    };
    let config = builder.with_single_cert(certificates(cert_chain)?, private_key(key)?).map_err(invalid_input)?;
    Ok(TlsAcceptor::from(Arc::new(config)))
}

/// Client side: trust servers whose certificates chain to a CA in `ca`. With `identity`
/// (certificate chain, private key), present a client certificate for mutual TLS.
pub fn connector(ca: &Path, identity: Option<(&Path, &Path)>) -> io::Result<TlsConnector> {
    let builder = ClientConfig::builder_with_provider(provider())
        .with_safe_default_protocol_versions()
        .map_err(invalid_input)?
        .with_root_certificates(root_store(ca)?);
    let config = match identity {
        Some((cert_chain, key)) => {
            builder.with_client_auth_cert(certificates(cert_chain)?, private_key(key)?).map_err(invalid_input)?
        }
        None => builder.with_no_client_auth(),
    };
    Ok(TlsConnector::from(Arc::new(config)))
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

fn root_store(path: &Path) -> io::Result<RootCertStore> {
    let mut roots = RootCertStore::empty();
    for cert in certificates(path)? {
        roots.add(cert).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}: {e}", path.display())))?;
    }
    Ok(roots)
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
