//! [`AllowedIps`]: the addresses an acceptor accepts connections from.

use std::fmt;
use std::net::IpAddr;

/// The IP addresses an acceptor accepts connections from: single addresses and CIDR ranges, IPv4
/// or IPv6. A connection from anywhere else is closed as soon as it's accepted, before a TLS
/// handshake or a Logon. See [`Acceptor::with_allowed_ips`](crate::Acceptor::with_allowed_ips).
///
/// An IPv4 address that reaches an IPv6 listener as `::ffff:a.b.c.d` is matched as IPv4.
///
/// ```
/// use turbojet::AllowedIps;
///
/// let allowed = AllowedIps::new(["10.1.0.0/16", "203.0.113.5", "2001:db8::/32"])?;
/// assert!(allowed.contains("10.1.2.3".parse().unwrap()));
/// assert!(!allowed.contains("10.2.0.1".parse().unwrap()));
/// # Ok::<(), String>(())
/// ```
#[derive(Clone, PartialEq, Eq, Default)]
pub struct AllowedIps(Vec<(IpAddr, u8)>);

impl AllowedIps {
    /// The addresses and ranges in `entries`: each an address (`203.0.113.5`, `2001:db8::1`) or
    /// a CIDR range (`10.1.0.0/16`, `2001:db8::/32`).
    ///
    /// # Errors
    ///
    /// The first entry that isn't one, or a range with bits set past its prefix length
    /// (`10.1.2.0/16`), which is more likely a mistake than meant.
    pub fn new(entries: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Self, String> {
        entries.into_iter().map(|entry| parse(entry.as_ref())).collect::<Result<_, _>>().map(Self)
    }

    /// Whether a connection from `ip` is accepted.
    pub fn contains(&self, ip: IpAddr) -> bool {
        let ip = ip.to_canonical();
        self.0.iter().any(|&(network, prefix)| within(ip, network, prefix))
    }
}

/// `address` or `address/prefix`.
fn parse(entry: &str) -> Result<(IpAddr, u8), String> {
    let (address, prefix) = match entry.split_once('/') {
        Some((address, prefix)) => (address, Some(prefix)),
        None => (entry, None),
    };
    let address: IpAddr = address.parse().map_err(|_| format!("'{entry}' isn't an IP address or CIDR range"))?;
    let bits = if address.is_ipv4() { 32 } else { 128 };
    let prefix = match prefix {
        None => bits,
        Some(prefix) => prefix
            .parse()
            .ok()
            .filter(|&prefix| prefix <= bits)
            .ok_or_else(|| format!("'{entry}': the prefix length must be 0 to {bits}"))?,
    };
    if mask(address, prefix) != bits_of(address) {
        return Err(format!(
            "'{entry}' has bits set past its prefix: did you mean {}/{prefix}?",
            network(address, prefix)
        ));
    }
    Ok((address, prefix))
}

/// Whether `ip` is in the range `network`/`prefix`.
fn within(ip: IpAddr, network: IpAddr, prefix: u8) -> bool {
    ip.is_ipv4() == network.is_ipv4() && mask(ip, prefix) == mask(network, prefix)
}

/// `ip`'s bits, the first `prefix` of them kept.
fn mask(ip: IpAddr, prefix: u8) -> u128 {
    let bits = if ip.is_ipv4() { 32 } else { 128 };
    let kept = if prefix == 0 { 0 } else { u128::MAX << (bits - u32::from(prefix)) };
    bits_of(ip) & kept & (u128::MAX >> (128 - bits))
}

fn bits_of(ip: IpAddr) -> u128 {
    match ip {
        IpAddr::V4(ip) => u128::from(u32::from(ip)),
        IpAddr::V6(ip) => u128::from(ip),
    }
}

/// `ip` with the bits past `prefix` cleared.
fn network(ip: IpAddr, prefix: u8) -> IpAddr {
    match ip {
        IpAddr::V4(_) => IpAddr::V4(u32::try_from(mask(ip, prefix)).expect("32 bits").into()),
        IpAddr::V6(_) => IpAddr::V6(mask(ip, prefix).into()),
    }
}

/// As it was given: `10.1.0.0/16, 203.0.113.5`.
impl fmt::Debug for AllowedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full = |ip: &IpAddr| if ip.is_ipv4() { 32 } else { 128 };
        let entries: Vec<String> = self
            .0
            .iter()
            .map(|(ip, prefix)| if *prefix == full(ip) { ip.to_string() } else { format!("{ip}/{prefix}") })
            .collect();
        f.debug_list().entries(entries).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(text: &str) -> IpAddr {
        text.parse().unwrap()
    }

    #[test]
    fn addresses_and_ranges_of_both_families_match() {
        let allowed = AllowedIps::new(["10.1.0.0/16", "203.0.113.5", "2001:db8::/32"]).unwrap();
        assert!(allowed.contains(ip("10.1.255.255")) && allowed.contains(ip("10.1.0.0")));
        assert!(!allowed.contains(ip("10.0.255.255")) && !allowed.contains(ip("10.2.0.0")));
        assert!(allowed.contains(ip("203.0.113.5")) && !allowed.contains(ip("203.0.113.6")));
        assert!(allowed.contains(ip("2001:db8:ffff::1")) && !allowed.contains(ip("2001:db9::1")));
        assert!(allowed.contains(ip("::ffff:10.1.2.3")), "IPv4 by way of an IPv6 listener");
        assert!(!allowed.contains(ip("::a01:203")), "an IPv6 address isn't matched against IPv4 ranges");
        assert!(AllowedIps::new(["0.0.0.0/0"]).unwrap().contains(ip("192.0.2.1")));
        assert!(!AllowedIps::default().contains(ip("127.0.0.1")), "none listed, none allowed");
        assert_eq!(format!("{allowed:?}"), r#"["10.1.0.0/16", "203.0.113.5", "2001:db8::/32"]"#);
    }

    #[test]
    fn mistakes_are_refused() {
        assert!(AllowedIps::new(["10.1.2.0/16"]).unwrap_err().contains("did you mean 10.1.0.0/16?"));
        assert!(AllowedIps::new(["10.0.0.0/33"]).unwrap_err().contains("0 to 32"));
        assert!(AllowedIps::new(["2001:db8::/129"]).unwrap_err().contains("0 to 128"));
        assert!(AllowedIps::new(["venue.example"]).unwrap_err().contains("isn't an IP address"));
    }
}
