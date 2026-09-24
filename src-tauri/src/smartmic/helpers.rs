use std::net::Ipv4Addr;

pub fn parse_unicast_bind_v4(address: &str) -> Result<Ipv4Addr, String> {
    let ip: Ipv4Addr = address
        .parse()
        .map_err(|e| format!("Failed to parse bind address '{}': {}", address, e))?;
    if ip.is_unspecified() || ip.is_broadcast() || ip.is_multicast() {
        return Err(
            "Bind address cannot be 0.0.0.0, broadcast, or multicast. Enable relay mode to listen on all interfaces."
                .to_string(),
        );
    }
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::parse_unicast_bind_v4;
    use std::net::Ipv4Addr;

    #[test]
    fn accepts_a_lan_address() {
        assert_eq!(
            parse_unicast_bind_v4("192.168.1.20").unwrap(),
            Ipv4Addr::new(192, 168, 1, 20)
        );
    }

    #[test]
    fn rejects_unspecified() {
        let err = parse_unicast_bind_v4("0.0.0.0").unwrap_err();
        assert!(err.contains("0.0.0.0"));
    }

    #[test]
    fn rejects_broadcast() {
        assert!(parse_unicast_bind_v4("255.255.255.255").is_err());
    }
}
