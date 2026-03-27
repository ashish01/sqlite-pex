use presto_core::{FunctionRegistry, PrestoResult};

pub mod url {
    use presto_core::{PrestoError, PrestoResult};

    fn parse(input: &str) -> PrestoResult<url::Url> {
        url::Url::parse(input).map_err(|_| PrestoError::InvalidArgument("invalid URL"))
    }

    pub fn url_encode(value: &str) -> String {
        url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
    }

    pub fn url_decode(value: &str) -> PrestoResult<String> {
        let decoded = url::form_urlencoded::parse(value.as_bytes())
            .map(|(k, v)| {
                if v.is_empty() {
                    k.into_owned()
                } else {
                    format!("{k}={v}")
                }
            })
            .collect::<Vec<_>>();

        if decoded.is_empty() {
            Ok(percent_encoding::percent_decode_str(value)
                .decode_utf8()
                .map_err(|_| PrestoError::InvalidArgument("invalid URL encoding"))?
                .into_owned())
        } else {
            Ok(decoded.join("&"))
        }
    }

    pub fn url_extract_fragment(input: &str) -> PrestoResult<Option<String>> {
        Ok(parse(input)?.fragment().map(ToOwned::to_owned))
    }

    pub fn url_extract_host(input: &str) -> PrestoResult<Option<String>> {
        Ok(parse(input)?.host_str().map(ToOwned::to_owned))
    }

    pub fn url_extract_parameter(input: &str, name: &str) -> PrestoResult<Option<String>> {
        Ok(parse(input)?
            .query_pairs()
            .find_map(|(k, v)| (k == name).then(|| v.into_owned())))
    }

    pub fn url_extract_path(input: &str) -> PrestoResult<Option<String>> {
        let path = parse(input)?.path().to_string();
        Ok((!path.is_empty()).then_some(path))
    }

    pub fn url_extract_port(input: &str) -> PrestoResult<Option<i64>> {
        Ok(parse(input)?.port().map(i64::from))
    }

    pub fn url_extract_protocol(input: &str) -> PrestoResult<Option<String>> {
        let scheme = parse(input)?.scheme().to_string();
        Ok((!scheme.is_empty()).then_some(scheme))
    }

    pub fn url_extract_query(input: &str) -> PrestoResult<Option<String>> {
        Ok(parse(input)?.query().map(ToOwned::to_owned))
    }
}

pub mod ip {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use ipnet::IpNet;
    use presto_core::{PrestoError, PrestoResult};

    fn parse_ip(value: &str) -> PrestoResult<IpAddr> {
        value
            .parse::<IpAddr>()
            .map_err(|_| PrestoError::InvalidArgument("invalid IP address"))
    }

    fn parse_prefix(value: &str) -> PrestoResult<IpNet> {
        value
            .parse::<IpNet>()
            .map_err(|_| PrestoError::InvalidArgument("invalid ip_prefix"))
    }

    fn ip_sort_key(ip: IpAddr) -> (u8, u128) {
        match ip {
            IpAddr::V4(v4) => (4, u32::from(v4) as u128),
            IpAddr::V6(v6) => (6, u128::from(v6)),
        }
    }

    pub fn ip_prefix(ip_address: &str, prefix_bits: u8) -> PrestoResult<String> {
        let ip = parse_ip(ip_address)?;
        let net = IpNet::new(ip, prefix_bits)
            .map_err(|_| PrestoError::InvalidArgument("invalid prefix bits for IP family"))?;
        Ok(net.trunc().to_string())
    }

    pub fn ip_prefix_collapse(prefixes: &[String]) -> PrestoResult<Vec<String>> {
        let mut nets = prefixes
            .iter()
            .map(|p| parse_prefix(p))
            .collect::<PrestoResult<Vec<_>>>()?;

        nets.sort_by(|a, b| {
            let ka = ip_sort_key(a.network());
            let kb = ip_sort_key(b.network());
            ka.cmp(&kb)
                .then_with(|| a.prefix_len().cmp(&b.prefix_len()))
        });

        // Conservative collapse: remove exact duplicates and strict-contained subnets.
        // Since nets is sorted by network address, only the last pushed entry can cover the next.
        let mut out: Vec<IpNet> = Vec::new();
        for net in nets {
            if out.last().map_or(false, |existing| {
                existing.contains(&net.network()) && existing.contains(&net.broadcast())
            }) {
                continue;
            }
            out.push(net);
        }

        Ok(out.into_iter().map(|n| n.to_string()).collect())
    }

    pub fn ip_prefix_subnets(prefix: &str, new_prefix_len: u8) -> PrestoResult<Vec<String>> {
        let net = parse_prefix(prefix)?;
        if new_prefix_len < net.prefix_len() {
            return Err(PrestoError::InvalidArgument(
                "new prefix length must be >= existing prefix length",
            ));
        }

        if new_prefix_len == net.prefix_len() {
            return Ok(vec![net.to_string()]);
        }

        let mut out = Vec::new();
        match net {
            IpNet::V4(v4) => {
                let base = u32::from(v4.network());
                let step: u32 = 1u32 << (32 - new_prefix_len as u32);
                let total: u32 = 1u32 << (new_prefix_len as u32 - v4.prefix_len() as u32);
                for i in 0..total {
                    let next = Ipv4Addr::from(base.wrapping_add(i.wrapping_mul(step)));
                    out.push(
                        ipnet::Ipv4Net::new(next, new_prefix_len)
                            .map_err(|_| PrestoError::Internal("failed to create subnet".into()))?
                            .to_string(),
                    );
                }
            }
            IpNet::V6(v6) => {
                let base = u128::from(v6.network());
                let step: u128 = 1u128 << (128 - new_prefix_len as u32);
                let count_bits = new_prefix_len as u32 - v6.prefix_len() as u32;
                if count_bits > 16 {
                    return Err(PrestoError::InvalidArgument(
                        "too many IPv6 subnets requested for MVP",
                    ));
                }
                let total = 1u128 << count_bits;
                for i in 0..total {
                    let next = Ipv6Addr::from(base + i * step);
                    out.push(
                        ipnet::Ipv6Net::new(next, new_prefix_len)
                            .map_err(|_| PrestoError::Internal("failed to create subnet".into()))?
                            .to_string(),
                    );
                }
            }
        }
        Ok(out)
    }

    pub fn ip_subnet_min(prefix: &str) -> PrestoResult<String> {
        let net = parse_prefix(prefix)?;
        Ok(net.network().to_string())
    }

    pub fn ip_subnet_max(prefix: &str) -> PrestoResult<String> {
        let net = parse_prefix(prefix)?;
        Ok(net.broadcast().to_string())
    }

    pub fn ip_subnet_range(prefix: &str) -> PrestoResult<Vec<String>> {
        let net = parse_prefix(prefix)?;

        match (net.network(), net.broadcast()) {
            (IpAddr::V4(s), IpAddr::V4(e)) => {
                let s = u32::from(s);
                let e = u32::from(e);
                if e - s > 65_536 {
                    return Err(PrestoError::InvalidArgument(
                        "subnet too large to materialize as array in MVP",
                    ));
                }
                Ok((s..=e)
                    .map(|v| Ipv4Addr::from(v).to_string())
                    .collect::<Vec<_>>())
            }
            (IpAddr::V6(s), IpAddr::V6(e)) => {
                let s = u128::from(s);
                let e = u128::from(e);
                if e - s > 4_096 {
                    return Err(PrestoError::InvalidArgument(
                        "IPv6 subnet too large to materialize as array in MVP",
                    ));
                }
                Ok((s..=e)
                    .map(|v| Ipv6Addr::from(v).to_string())
                    .collect::<Vec<_>>())
            }
            _ => Err(PrestoError::InvalidArgument("mixed address family range")),
        }
    }

    pub fn is_private_ip(ip_address: &str) -> PrestoResult<bool> {
        let ip = parse_ip(ip_address)?;
        Ok(match ip {
            IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
            IpAddr::V6(v6) => {
                v6.is_loopback()
                    || v6.is_unspecified()
                    || v6.is_unique_local()
                    || v6.is_unicast_link_local()
            }
        })
    }

    pub fn is_subnet_of_prefix_ip(prefix: &str, ip_address: &str) -> PrestoResult<bool> {
        let net = parse_prefix(prefix)?;
        let ip = parse_ip(ip_address)?;
        Ok(net.contains(&ip))
    }

    pub fn is_subnet_of_prefix_prefix(prefix1: &str, prefix2: &str) -> PrestoResult<bool> {
        let p1 = parse_prefix(prefix1)?;
        let p2 = parse_prefix(prefix2)?;
        if p1.network().is_ipv4() != p2.network().is_ipv4() {
            return Ok(false);
        }
        Ok(p1.contains(&p2.network()) && p1.contains(&p2.broadcast()))
    }
}

pub mod uuid {
    use presto_core::{PrestoError, PrestoResult};

    pub fn uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn parse_uuid(value: &str) -> PrestoResult<String> {
        let parsed = uuid::Uuid::parse_str(value)
            .map_err(|_| PrestoError::InvalidArgument("invalid UUID"))?;
        Ok(parsed.to_string())
    }
}

pub fn register(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    let scalar_functions = [
        "url_decode",
        "url_encode",
        "url_extract_fragment",
        "url_extract_host",
        "url_extract_parameter",
        "url_extract_path",
        "url_extract_port",
        "url_extract_protocol",
        "url_extract_query",
        "ip_prefix",
        "ip_prefix_collapse",
        "ip_prefix_subnets",
        "ip_subnet_max",
        "ip_subnet_min",
        "ip_subnet_range",
        "is_private_ip",
        "is_subnet_of",
        "uuid",
    ];
    for name in scalar_functions {
        registry.register_scalar(name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ip, url, uuid};

    #[test]
    fn url_extractors_work() {
        let input = "https://example.com:8443/path/a?q=1&z=2#frag";
        assert_eq!(
            url::url_extract_host(input).unwrap().as_deref(),
            Some("example.com")
        );
        assert_eq!(url::url_extract_port(input).unwrap(), Some(8443));
        assert_eq!(
            url::url_extract_fragment(input).unwrap().as_deref(),
            Some("frag")
        );
        assert_eq!(
            url::url_extract_parameter(input, "q").unwrap().as_deref(),
            Some("1")
        );
    }

    #[test]
    fn ip_helpers_work() {
        assert_eq!(ip::ip_prefix("10.1.2.3", 16).unwrap(), "10.1.0.0/16");
        assert!(ip::is_private_ip("10.1.2.3").unwrap());
        assert!(ip::is_subnet_of_prefix_ip("10.1.0.0/16", "10.1.2.3").unwrap());
        assert_eq!(ip::ip_subnet_min("10.1.0.0/30").unwrap(), "10.1.0.0");
        assert_eq!(ip::ip_subnet_max("10.1.0.0/30").unwrap(), "10.1.0.3");
        assert_eq!(ip::ip_subnet_range("10.1.0.0/30").unwrap().len(), 4);
    }

    #[test]
    fn uuid_parse_round_trip() {
        let value = uuid::uuid();
        assert_eq!(uuid::parse_uuid(&value).unwrap(), value);
    }
}
