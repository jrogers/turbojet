//! [`Proxy`]: an HTTP or SOCKS5 proxy that initiators connect through.

use std::fmt;
use std::io;
use std::net::IpAddr;
use std::str::FromStr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::fields::Secret;

/// A proxy an initiator reaches its counterparty through, for a network that only connects out
/// that way: see [`InitiatorConfig::proxy`](crate::InitiatorConfig::proxy). The initiator connects
/// to the proxy and asks it for a tunnel to the endpoint, by name, so the proxy resolves it; TLS,
/// if any, then runs through the tunnel to the counterparty, and the proxy sees only ciphertext.
///
/// ```
/// use turbojet::Proxy;
///
/// let proxy: Proxy = "socks5://firm@proxy.internal:1080".parse()?;
/// let proxy = proxy.with_password("from-a-vault");
/// assert_eq!(format!("{proxy:?}"), r#"Proxy { kind: Socks5, addr: "proxy.internal:1080", username: Some("firm"), password: Some(***) }"#);
/// # Ok::<(), String>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Proxy {
    /// How to ask it for a tunnel.
    pub kind: ProxyKind,
    /// `host:port`.
    pub addr: String,
    /// The username to give it, if it asks for one.
    pub username: Option<String>,
    /// The password to give it with `username`. Shown as `***` by `Debug`.
    pub password: Option<Secret>,
}

/// How a [`Proxy`] is asked for a tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProxyKind {
    /// An HTTP proxy, with `CONNECT`, and Basic authentication if a username is given.
    Http,
    /// A SOCKS5 proxy (RFC 1928), with username and password authentication (RFC 1929) if a
    /// username is given.
    Socks5,
}

impl Proxy {
    /// An HTTP proxy at `addr` (`host:port`), with no credentials.
    pub fn http(addr: impl Into<String>) -> Self {
        Self { kind: ProxyKind::Http, addr: addr.into(), username: None, password: None }
    }

    /// A SOCKS5 proxy at `addr` (`host:port`), with no credentials.
    pub fn socks5(addr: impl Into<String>) -> Self {
        Self { kind: ProxyKind::Socks5, addr: addr.into(), username: None, password: None }
    }

    /// Sets the username to give the proxy.
    #[must_use]
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Sets the password to give the proxy with the username.
    #[must_use]
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(Secret::from(password.into()));
        self
    }
}

/// `http://host:port` or `socks5://host:port`, with an optional `username@` before the host. A
/// password isn't taken here, so that it isn't kept in a URL: see [`Proxy::with_password`].
impl FromStr for Proxy {
    type Err = String;

    fn from_str(url: &str) -> Result<Self, String> {
        let (scheme, rest) = url.split_once("://").ok_or_else(|| format!("'{url}' isn't scheme://host:port"))?;
        let rest = rest.strip_suffix('/').unwrap_or(rest);
        let (username, addr) = match rest.rsplit_once('@') {
            Some((user, _)) if user.contains(':') => {
                return Err(format!("'{url}' has a password: give it apart from the URL"));
            }
            Some((user, addr)) => (Some(user.to_string()), addr),
            None => (None, rest),
        };
        split_host_port(addr).ok_or_else(|| format!("'{url}' has no host:port"))?;
        let proxy = match scheme {
            "http" => Self::http(addr),
            "socks5" | "socks5h" => Self::socks5(addr),
            other => return Err(format!("'{url}': proxy scheme must be http or socks5, not '{other}'")),
        };
        Ok(Self { username, ..proxy })
    }
}

impl fmt::Display for Proxy {
    /// As [`FromStr`] reads it, without the password.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scheme = match self.kind {
            ProxyKind::Http => "http",
            ProxyKind::Socks5 => "socks5",
        };
        match &self.username {
            Some(user) => write!(f, "{scheme}://{user}@{}", self.addr),
            None => write!(f, "{scheme}://{}", self.addr),
        }
    }
}

/// The longest proxy reply read: an HTTP response's headers, which a proxy keeps short.
const MAX_REPLY: usize = 8192;

/// Asks `proxy`, connected as `stream`, for a tunnel to `target` (`host:port`). Once it returns,
/// what's written to `stream` reaches the target.
pub(crate) async fn tunnel(stream: &mut TcpStream, proxy: &Proxy, target: &str) -> io::Result<()> {
    let (host, port) = split_host_port(target)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("'{target}' isn't host:port")))?;
    match proxy.kind {
        ProxyKind::Http => http_connect(stream, proxy, host, port).await,
        ProxyKind::Socks5 => socks5_connect(stream, proxy, host, port).await,
    }
}

async fn http_connect(stream: &mut TcpStream, proxy: &Proxy, host: &str, port: u16) -> io::Result<()> {
    let authority = if host.contains(':') { format!("[{host}]:{port}") } else { format!("{host}:{port}") };
    let mut request = format!("CONNECT {authority} HTTP/1.1\r\nHost: {authority}\r\n");
    if let Some(user) = &proxy.username {
        let password = proxy.password.as_ref().map_or("", Secret::expose);
        request
            .push_str(&format!("Proxy-Authorization: Basic {}\r\n", base64(format!("{user}:{password}").as_bytes())));
    }
    request.push_str("\r\n");
    stream.write_all(request.as_bytes()).await?;
    // A byte at a time, so nothing of the tunnel's is read: the proxy sends nothing more until
    // we do, but reading exactly to the blank line doesn't rely on it.
    let mut reply = Vec::new();
    while !reply.ends_with(b"\r\n\r\n") {
        if reply.len() == MAX_REPLY {
            return Err(refused(proxy, "its reply to CONNECT is too long".into()));
        }
        reply.push(stream.read_u8().await?);
    }
    let status = reply.split(|&b| b == b'\r').next().unwrap_or_default();
    let status = String::from_utf8_lossy(status);
    match status.split(' ').nth(1) {
        Some(code) if code.starts_with('2') && status.starts_with("HTTP/1.") => Ok(()),
        _ => Err(refused(proxy, format!("it answered CONNECT {authority} with '{status}'"))),
    }
}

async fn socks5_connect(stream: &mut TcpStream, proxy: &Proxy, host: &str, port: u16) -> io::Result<()> {
    // Greeting: version 5, then the methods offered: none, or username and password.
    let method = if proxy.username.is_some() { 2 } else { 0 };
    stream.write_all(&[5, 1, method]).await?;
    let mut chosen = [0; 2];
    stream.read_exact(&mut chosen).await?;
    if chosen != [5, method] {
        return Err(refused(proxy, format!("it didn't accept SOCKS5 with method {method}: {chosen:?}")));
    }
    if let Some(user) = &proxy.username {
        let password = proxy.password.as_ref().map_or("", Secret::expose);
        let mut auth = vec![1];
        for part in [user.as_bytes(), password.as_bytes()] {
            let len =
                u8::try_from(part.len()).map_err(|_| refused(proxy, "a username or password over 255 bytes".into()))?;
            auth.push(len);
            auth.extend_from_slice(part);
        }
        stream.write_all(&auth).await?;
        let mut status = [0; 2];
        stream.read_exact(&mut status).await?;
        if status[1] != 0 {
            return Err(refused(proxy, "it refused the username and password".into()));
        }
    }
    // CONNECT, to an address or, for the proxy to resolve, a name.
    let mut request = vec![5, 1, 0];
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => {
            request.push(1);
            request.extend_from_slice(&ip.octets());
        }
        Ok(IpAddr::V6(ip)) => {
            request.push(4);
            request.extend_from_slice(&ip.octets());
        }
        Err(_) => {
            let len = u8::try_from(host.len()).map_err(|_| refused(proxy, format!("host name '{host}' too long")))?;
            request.extend_from_slice(&[3, len]);
            request.extend_from_slice(host.as_bytes());
        }
    }
    request.extend_from_slice(&port.to_be_bytes());
    stream.write_all(&request).await?;
    let mut reply = [0; 4];
    stream.read_exact(&mut reply).await?;
    if reply[1] != 0 {
        return Err(refused(proxy, format!("it refused CONNECT {host}:{port}: {}", socks5_reply(reply[1]))));
    }
    // The address it bound, which isn't needed, then the port.
    let bound = match reply[3] {
        1 => 4,
        4 => 16,
        3 => usize::from(stream.read_u8().await?),
        other => return Err(refused(proxy, format!("its reply has address type {other}"))),
    };
    let mut skip = vec![0; bound + 2];
    stream.read_exact(&mut skip).await?;
    Ok(())
}

/// RFC 1928's reply codes.
fn socks5_reply(code: u8) -> &'static str {
    match code {
        1 => "general failure",
        2 => "not allowed by its rules",
        3 => "network unreachable",
        4 => "host unreachable",
        5 => "connection refused",
        6 => "TTL expired",
        7 => "command not supported",
        8 => "address type not supported",
        _ => "unknown error",
    }
}

fn refused(proxy: &Proxy, why: String) -> io::Error {
    io::Error::new(io::ErrorKind::ConnectionRefused, format!("proxy {proxy}: {why}"))
}

/// `host:port`, or `[v6 address]:port`, split.
fn split_host_port(addr: &str) -> Option<(&str, u16)> {
    let (host, port) = addr.rsplit_once(':')?;
    let host = host.strip_prefix('[').and_then(|h| h.strip_suffix(']')).unwrap_or(host);
    let port = port.parse().ok()?;
    (!host.is_empty()).then_some((host, port))
}

/// Standard base64, with padding, for HTTP Basic authentication.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().enumerate().fold(0u32, |n, (i, &b)| n | u32::from(b) << (16 - 8 * i));
        for i in 0..4 {
            let c = if i <= chunk.len() { ALPHABET[(n >> (18 - 6 * i) & 63) as usize] } else { b'=' };
            out.push(char::from(c));
        }
    }
    out
}

/// An HTTP proxy that tunnels each CONNECT to the address it names, answering 502 if it can't
/// connect there: for tests of connecting through one. Its address.
#[cfg(test)]
pub(crate) async fn forwarding_proxy() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(async move {
        loop {
            let (mut client, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut request = Vec::new();
                while !request.ends_with(b"\r\n\r\n") {
                    request.push(client.read_u8().await.unwrap());
                }
                let request = String::from_utf8(request).unwrap();
                let target = request.split(' ').nth(1).unwrap();
                let Ok(mut upstream) = TcpStream::connect(target).await else {
                    return client.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await.unwrap();
                };
                client.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n").await.unwrap();
                let _ = tokio::io::copy_bidirectional(&mut client, &mut upstream).await;
            });
        }
    });
    addr
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;

    #[test]
    fn urls_parse_without_passwords() {
        let proxy: Proxy = "http://proxy:3128".parse().unwrap();
        assert_eq!(proxy, Proxy::http("proxy:3128"));
        let proxy: Proxy = "socks5://firm@[::1]:1080/".parse().unwrap();
        assert_eq!(proxy, Proxy::socks5("[::1]:1080").with_username("firm"));
        assert_eq!(proxy.to_string(), "socks5://firm@[::1]:1080");
        assert!("http://firm:pw@proxy:3128".parse::<Proxy>().unwrap_err().contains("password"));
        assert!("ftp://proxy:21".parse::<Proxy>().unwrap_err().contains("http or socks5"));
        assert!("http://proxy".parse::<Proxy>().is_err());
    }

    #[test]
    fn base64_matches_rfc_4648() {
        for (plain, encoded) in [("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foobar", "Zm9vYmFy")] {
            assert_eq!(base64(plain.as_bytes()), encoded);
        }
    }

    /// A proxy that follows `script`, reading as many bytes as each step says and then answering
    /// with its reply, and then echoes the tunnel: everything it read, for the test to check.
    async fn scripted(script: Vec<(usize, &'static [u8])>) -> (String, tokio::task::JoinHandle<Vec<u8>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        let task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut read = Vec::new();
            for (len, reply) in script {
                let mut request = vec![0; len];
                stream.read_exact(&mut request).await.unwrap();
                read.extend_from_slice(&request);
                stream.write_all(reply).await.unwrap();
            }
            let mut tunnelled = [0; 5];
            if stream.read_exact(&mut tunnelled).await.is_ok() {
                stream.write_all(&tunnelled).await.unwrap();
            }
            read
        });
        (addr, task)
    }

    async fn through(proxy: Proxy, target: &str) -> io::Result<TcpStream> {
        let mut stream = TcpStream::connect(&proxy.addr).await?;
        tunnel(&mut stream, &proxy, target).await?;
        stream.write_all(b"hello").await?;
        let mut echoed = [0; 5];
        stream.read_exact(&mut echoed).await?;
        assert_eq!(&echoed, b"hello");
        Ok(stream)
    }

    #[tokio::test]
    async fn http_connect_with_basic_authentication() {
        let request = "CONNECT fix.venue:9876 HTTP/1.1\r\nHost: fix.venue:9876\r\nProxy-Authorization: Basic ZmlybTpwdw==\r\n\r\n";
        let (addr, proxy) = scripted(vec![(request.len(), b"HTTP/1.1 200 Connection established\r\n\r\n")]).await;
        through(Proxy::http(addr).with_username("firm").with_password("pw"), "fix.venue:9876").await.unwrap();
        assert_eq!(String::from_utf8(proxy.await.unwrap()).unwrap(), request);
    }

    #[tokio::test]
    async fn http_refusal_says_why() {
        let (addr, _proxy) = scripted(vec![(30, b"HTTP/1.1 407 Proxy Authentication Required\r\n\r\n")]).await;
        let err = through(Proxy::http(&addr), "fix.venue:9876").await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::ConnectionRefused);
        assert!(err.to_string().contains("407 Proxy Authentication Required"), "{err}");
    }

    #[tokio::test]
    async fn socks5_connect_by_name_with_credentials() {
        // Greeting, authentication, CONNECT to fix.venue:9876 by name.
        let mut expected = vec![5, 1, 2, 1, 4];
        expected.extend_from_slice(b"firm");
        expected.extend_from_slice(&[2]);
        expected.extend_from_slice(b"pw");
        expected.extend_from_slice(&[5, 1, 0, 3, 9]);
        expected.extend_from_slice(b"fix.venue");
        expected.extend_from_slice(&9876u16.to_be_bytes());
        let connect_reply: &[u8] = &[5, 0, 0, 1, 10, 0, 0, 1, 0x26, 0x94];
        let (addr, proxy) = scripted(vec![(3, &[5, 2]), (9, &[1, 0]), (expected.len() - 12, connect_reply)]).await;
        through(Proxy::socks5(addr).with_username("firm").with_password("pw"), "fix.venue:9876").await.unwrap();
        assert_eq!(proxy.await.unwrap(), expected);
    }

    #[tokio::test]
    async fn socks5_to_an_address_and_a_refusal() {
        let expected = [5, 1, 0, 5, 1, 0, 1, 127, 0, 0, 1, 0x26, 0x94];
        let (addr, proxy) = scripted(vec![(3, &[5, 0]), (10, &[5, 0, 0, 1, 0, 0, 0, 0, 0, 0])]).await;
        through(Proxy::socks5(addr), "127.0.0.1:9876").await.unwrap();
        assert_eq!(proxy.await.unwrap(), expected);

        let (addr, _proxy) = scripted(vec![(3, &[5, 0]), (10, &[5, 2, 0, 1, 0, 0, 0, 0, 0, 0])]).await;
        let err = through(Proxy::socks5(addr), "127.0.0.1:9876").await.unwrap_err();
        assert!(err.to_string().contains("not allowed by its rules"), "{err}");
    }
}
