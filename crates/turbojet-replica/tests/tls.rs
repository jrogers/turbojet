//! The link over TLS: replicating with each side's certificate checked, and refusing an active
//! without one.

use std::sync::{Arc, Mutex};

use rcgen::{BasicConstraints, CertificateParams, DnType, IsCa, Issuer, KeyPair};
use turbojet::store::SessionStorage;
use turbojet::store::conformance::{app_message, id};
use turbojet::tls::{ClientTls, ClientTrust, Identity, ServerTls, Trust};
use turbojet::{InitiatorTls, MemoryStorage};
use turbojet_replica::{Replica, ReplicatedStorage, TcpLink, serve_standby_tls};

/// A CA, and the standby's and the active's certificates from it.
struct Pki {
    ca: String,
    standby: Identity,
    active: Identity,
}

impl Pki {
    fn new() -> Self {
        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.distinguished_name.push(DnType::CommonName, "ca");
        let ca = params.self_signed(&key).unwrap().pem();
        let issuer = Issuer::new(params, key);
        let issue = |name: &str| {
            let key = KeyPair::generate().unwrap();
            let cert = CertificateParams::new(vec![name.into()]).unwrap().signed_by(&key, &issuer).unwrap();
            Identity::from_pem(cert.pem().as_bytes(), key.serialize_pem().as_bytes()).unwrap()
        };
        Self { standby: issue("localhost"), active: issue("active.example"), ca }
    }

    fn trust(&self) -> Trust {
        Trust::from_pem(self.ca.as_bytes()).unwrap()
    }
}

/// A standby requiring a client certificate from the CA, serving from a thread; its address and
/// store.
fn standby(pki: &Pki) -> (String, Arc<MemoryStorage>) {
    let tls = ServerTls::new(pki.standby.clone(), ClientTrust::Required(pki.trust())).unwrap().acceptor();
    let store = Arc::new(MemoryStorage::new());
    let replica = Arc::new(Mutex::new(Replica::new(store.clone())));
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    listener.set_nonblocking(true).unwrap();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(async {
            serve_standby_tls(tokio::net::TcpListener::from_std(listener).unwrap(), tls, replica).await
        })
    });
    (addr, store)
}

fn active(addr: &str, tls: &ClientTls) -> ReplicatedStorage<MemoryStorage> {
    let link = TcpLink::new(addr, 1, "active").with_tls(InitiatorTls::new(tls.connector(), "localhost"));
    ReplicatedStorage::new(MemoryStorage::new(), Arc::new(link), 1)
}

#[test]
fn replicates_over_tls() {
    let pki = Pki::new();
    let (addr, standby) = standby(&pki);
    let active = active(&addr, &ClientTls::new(pki.trust(), Some(pki.active.clone())).unwrap());
    let mut log = active.open(&id("A")).unwrap();
    log.record_outgoing(1, Some(&app_message(1))).unwrap();
    log.commit().unwrap().unwrap().run().unwrap();
    drop(log);
    assert_eq!(standby.open(&id("A")).unwrap().next_outgoing(), 2);
}

#[test]
fn an_active_without_a_certificate_is_refused() {
    let pki = Pki::new();
    let (addr, _) = standby(&pki);
    let active = active(&addr, &ClientTls::new(pki.trust(), None).unwrap());
    let refused = active.open(&id("A")).err().expect("refused");
    assert!(refused.to_string().contains("CertificateRequired"), "{refused}");
}
