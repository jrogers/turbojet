//! Reading the PROXY protocol header a load balancer sends ahead of a connection's own bytes,
//! saying where the connection came from: version 1 (text) or 2 (binary), as
//! [HAProxy's specification](https://www.haproxy.org/download/2.9/doc/proxy-protocol.txt) has
//! them.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use tokio::io::{AsyncRead, AsyncReadExt};

/// The longest version 1 header, its CRLF included, as the specification bounds it.
const MAX_V1_BYTES: usize = 107;
/// The most a version 2 header may say follows its first 16 bytes: the addresses, and room for
/// a few TLVs (which are skipped). The specification suggests 536 bytes for the whole header.
const MAX_V2_BODY: usize = 536 - 16;
/// What starts a version 2 header.
const V2_SIGNATURE: &[u8; 12] = b"\r\n\r\n\0\r\nQUIT\n";
const _: () = assert!(MAX_V1_BYTES < 536);

/// Reads the header at the start of `stream`, and no more: the connection's own bytes stay
/// unread. Returns the address the connection came from, or `None` for one the proxy made itself
/// (a health check, `LOCAL` or `UNKNOWN`), which keeps its own address.
///
/// # Errors
///
/// The stream's, or `InvalidData` for anything that isn't a header this reads.
pub(crate) async fn read<S: AsyncRead + Unpin>(stream: &mut S) -> std::io::Result<Option<SocketAddr>> {
    // Both versions' starts are 6 bytes, and the shortest header (`PROXY UNKNOWN\r\n`) is 15, so
    // reading 6 can't take any of what follows.
    let mut start = [0; 6];
    stream.read_exact(&mut start).await?;
    if &start == b"PROXY " {
        read_v1(stream).await
    } else if start == V2_SIGNATURE[..6] {
        read_v2(stream).await
    } else {
        Err(invalid("no PROXY protocol header"))
    }
}

/// The rest of a version 1 header, after `PROXY `: read a byte at a time, as it ends only at its
/// CRLF.
async fn read_v1<S: AsyncRead + Unpin>(stream: &mut S) -> std::io::Result<Option<SocketAddr>> {
    let mut line = Vec::with_capacity(MAX_V1_BYTES);
    while !line.ends_with(b"\r\n") {
        if line.len() + 6 >= MAX_V1_BYTES {
            return Err(invalid("PROXY header longer than 107 bytes"));
        }
        line.push(stream.read_u8().await?);
    }
    let line = std::str::from_utf8(&line[..line.len() - 2]).map_err(|_| invalid("PROXY header isn't ASCII"))?;
    parse_v1(line)
}

/// A version 1 header's fields after `PROXY `: `TCP4 src dst sport dport`, `TCP6 ...`, or
/// `UNKNOWN` with anything after it.
fn parse_v1(line: &str) -> std::io::Result<Option<SocketAddr>> {
    let mut fields = line.split(' ');
    let family = fields.next().unwrap_or_default();
    if family == "UNKNOWN" {
        return Ok(None);
    }
    let [Some(source), Some(_), Some(port), Some(_), None] = std::array::from_fn(|_| fields.next()) else {
        return Err(invalid("PROXY header needs source and destination addresses and ports"));
    };
    let ip: IpAddr = source.parse().map_err(|_| invalid("PROXY header's source address isn't an IP address"))?;
    let port: u16 = port.parse().map_err(|_| invalid("PROXY header's source port isn't a port"))?;
    match (family, ip) {
        ("TCP4", IpAddr::V4(_)) | ("TCP6", IpAddr::V6(_)) => Ok(Some(SocketAddr::new(ip, port))),
        _ => Err(invalid("PROXY header's protocol isn't TCP4 or TCP6 for its address")),
    }
}

/// The rest of a version 2 header, after its signature's first 6 bytes.
async fn read_v2<S: AsyncRead + Unpin>(stream: &mut S) -> std::io::Result<Option<SocketAddr>> {
    let mut rest = [0; 10];
    stream.read_exact(&mut rest).await?;
    if rest[..6] != V2_SIGNATURE[6..] {
        return Err(invalid("no PROXY protocol header"));
    }
    let (version_command, family) = (rest[6], rest[7]);
    let len = usize::from(u16::from_be_bytes([rest[8], rest[9]]));
    if version_command >> 4 != 2 {
        return Err(invalid("PROXY header isn't version 2"));
    }
    if len > MAX_V2_BODY {
        return Err(invalid("PROXY header longer than 536 bytes"));
    }
    let mut body = vec![0; len];
    stream.read_exact(&mut body).await?;
    match version_command & 0x0F {
        0 => Ok(None),
        1 => parse_v2(family, &body),
        _ => Err(invalid("PROXY header's command isn't LOCAL or PROXY")),
    }
}

/// A version 2 PROXY command's source address, for TCP over IPv4 or IPv6; `None` for other
/// families, as the specification has the receiver keep the connection's own then.
fn parse_v2(family: u8, body: &[u8]) -> std::io::Result<Option<SocketAddr>> {
    let short = || invalid("PROXY header shorter than its addresses");
    match family {
        0x11 => {
            let fields: &[u8; 12] = body.get(..12).and_then(|b| b.try_into().ok()).ok_or_else(short)?;
            let ip = Ipv4Addr::from([fields[0], fields[1], fields[2], fields[3]]);
            Ok(Some(SocketAddr::new(ip.into(), u16::from_be_bytes([fields[8], fields[9]]))))
        }
        0x21 => {
            let fields: &[u8; 36] = body.get(..36).and_then(|b| b.try_into().ok()).ok_or_else(short)?;
            let ip: [u8; 16] = fields[..16].try_into().expect("16 of 36 bytes");
            Ok(Some(SocketAddr::new(Ipv6Addr::from(ip).into(), u16::from_be_bytes([fields[32], fields[33]]))))
        }
        _ => Ok(None),
    }
}

fn invalid(why: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, why)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads a header from `bytes`; returns the address and what's left unread.
    async fn header(bytes: &[u8]) -> (std::io::Result<Option<SocketAddr>>, Vec<u8>) {
        let mut stream = bytes;
        let read = read(&mut stream).await;
        (read, stream.to_vec())
    }

    fn addr(s: &str) -> Option<SocketAddr> {
        Some(s.parse().unwrap())
    }

    fn v2(command: u8, family: u8, body: &[u8]) -> Vec<u8> {
        let len = u16::try_from(body.len()).unwrap().to_be_bytes();
        [&V2_SIGNATURE[..], &[0x20 | command, family], &len, body, b"8=FIX"].concat()
    }

    #[tokio::test]
    async fn version_1_gives_the_source_and_leaves_what_follows() {
        let (read, rest) = header(b"PROXY TCP4 192.0.2.7 10.0.0.1 51234 9876\r\n8=FIX").await;
        assert_eq!(read.unwrap(), addr("192.0.2.7:51234"));
        assert_eq!(rest, b"8=FIX");
        let (read, _) = header(b"PROXY TCP6 2001:db8::7 2001:db8::1 51234 9876\r\n").await;
        assert_eq!(read.unwrap(), addr("[2001:db8::7]:51234"));
        let (read, rest) = header(b"PROXY UNKNOWN\r\n8=FIX").await;
        assert_eq!((read.unwrap(), rest.as_slice()), (None, &b"8=FIX"[..]));
        assert_eq!(header(b"PROXY UNKNOWN ffff::1 ffff::2 1 2\r\n").await.0.unwrap(), None);
    }

    #[tokio::test]
    async fn version_2_gives_the_source_and_leaves_what_follows() {
        let ipv4 = [192, 0, 2, 7, 10, 0, 0, 1, 0xC8, 0x22, 0x26, 0x94];
        let (read, rest) = header(&v2(1, 0x11, &ipv4)).await;
        assert_eq!(read.unwrap(), addr("192.0.2.7:51234"));
        assert_eq!(rest, b"8=FIX");
        let mut ipv6 = vec![0x20, 0x01, 0x0d, 0xb8];
        ipv6.extend([0; 11]);
        ipv6.push(7);
        ipv6.extend([0; 16]);
        ipv6.extend([0xC8, 0x22, 0x26, 0x94]);
        ipv6.extend([0x04, 0x00, 0x01, 0xFF]); // a TLV, skipped
        assert_eq!(header(&v2(1, 0x21, &ipv6)).await.0.unwrap(), addr("[2001:db8::7]:51234"));
        assert_eq!(header(&v2(0, 0x11, &ipv4)).await.0.unwrap(), None, "LOCAL: the proxy's own");
        assert_eq!(header(&v2(1, 0x31, &[0; 216])).await.0.unwrap(), None, "a family it doesn't carry");
    }

    #[tokio::test]
    async fn anything_else_is_refused() {
        let long_v1 = format!("PROXY TCP4 {} 1 2 3\r\n", "1".repeat(100));
        let refused: [&[u8]; 9] = [
            b"8=FIX.4.4\x019=5\x01",
            b"PROXY TCP4 192.0.2.7 10.0.0.1 51234\r\n",
            b"PROXY TCP4 2001:db8::7 10.0.0.1 1 2\r\n",
            b"PROXY TCP4 192.0.2.7 10.0.0.1 99999 2\r\n",
            b"PROXY UDP4 192.0.2.7 10.0.0.1 1 2\r\n",
            long_v1.as_bytes(),
            b"PROXY TCP4",
            &v2(1, 0x11, &[192, 0, 2])[..],
            b"\r\n\r\n\0\rXXXXXXXXXX",
        ];
        for bytes in refused {
            assert!(header(bytes).await.0.is_err(), "{:?}", String::from_utf8_lossy(bytes));
        }
        let mut too_long = v2(1, 0x11, &[0; 12]);
        too_long[14..16].copy_from_slice(&600u16.to_be_bytes());
        assert!(header(&too_long).await.0.is_err());
        let mut version_3 = v2(1, 0x11, &[0; 12]);
        version_3[12] = 0x31;
        assert!(header(&version_3).await.0.is_err());
    }
}
