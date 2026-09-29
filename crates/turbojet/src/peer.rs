//! What the engine knows about the other end of a connection, for authenticating logons.

use std::fmt;
use std::net::SocketAddr;

/// The transport a session is running over, passed to
/// [`Application::verify_logon`](crate::Application::verify_logon).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConnectionInfo {
    /// The remote socket address, when the transport has one.
    pub addr: Option<SocketAddr>,
    /// The certificate chain the peer presented over TLS, leaf first. Empty without TLS, when
    /// the acceptor doesn't request client certificates, or when a client didn't send one under
    /// `tls::ClientAuth::Optional` (feature `tls`). Any certificates here have already been
    /// verified against the configured CAs.
    pub peer_certificates: Vec<PeerCertificate>,
}

impl ConnectionInfo {
    /// For custom transports: describe the connection handed to
    /// [`Acceptor::accept_stream`](crate::Acceptor::accept_stream) or
    /// [`Initiator::run_stream`](crate::Initiator::run_stream).
    pub fn new(addr: Option<SocketAddr>, peer_certificates: Vec<PeerCertificate>) -> Self {
        Self { addr, peer_certificates }
    }

    /// The peer's own (leaf) certificate, if it presented one.
    pub fn peer_certificate(&self) -> Option<&PeerCertificate> {
        self.peer_certificates.first()
    }
}

/// A DER-encoded X.509 certificate presented by the peer.
#[derive(Clone, PartialEq, Eq)]
pub struct PeerCertificate {
    der: Vec<u8>,
}

impl PeerCertificate {
    /// Wraps a certificate's DER encoding.
    pub fn from_der(der: impl Into<Vec<u8>>) -> Self {
        Self { der: der.into() }
    }

    /// The certificate's DER encoding.
    pub fn der(&self) -> &[u8] {
        &self.der
    }

    /// The first CommonName (CN) in the certificate's subject.
    #[cfg(feature = "tls")]
    pub fn subject_common_name(&self) -> Option<String> {
        let (_, cert) = x509_parser::parse_x509_certificate(&self.der).ok()?;
        let cn = cert.subject().iter_common_name().next()?;
        cn.as_str().ok().map(String::from)
    }

    /// The DNS names in the certificate's Subject Alternative Name extension.
    #[cfg(feature = "tls")]
    pub fn dns_names(&self) -> Vec<String> {
        let Ok((_, cert)) = x509_parser::parse_x509_certificate(&self.der) else { return Vec::new() };
        let Ok(Some(san)) = cert.subject_alternative_name() else { return Vec::new() };
        san.value
            .general_names
            .iter()
            .filter_map(|name| match name {
                x509_parser::extensions::GeneralName::DNSName(dns) => Some(dns.to_string()),
                _ => None,
            })
            .collect()
    }
}

impl fmt::Debug for PeerCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("PeerCertificate");
        #[cfg(feature = "tls")]
        debug.field("subject_common_name", &self.subject_common_name());
        debug.field("der_len", &self.der.len()).finish()
    }
}
