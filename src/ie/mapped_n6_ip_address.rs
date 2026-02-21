//! Mapped N6 IP Address Information Element.
//!
//! Per 3GPP TS 29.244, contains a mapped N6 IP address (IPv4 and/or IPv6).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedN6IpAddress {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl MappedN6IpAddress {
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        Self {
            ipv4: Some(addr),
            ipv6: None,
        }
    }

    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        Self {
            ipv4: None,
            ipv6: Some(addr),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        if let Some(ipv4) = &self.ipv4 {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = &self.ipv6 {
            data.extend_from_slice(&ipv6.octets());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        match data.len() {
            4 => {
                let mut octets = [0u8; 4];
                octets.copy_from_slice(data);
                Ok(Self {
                    ipv4: Some(Ipv4Addr::from(octets)),
                    ipv6: None,
                })
            }
            16 => {
                let mut octets = [0u8; 16];
                octets.copy_from_slice(data);
                Ok(Self {
                    ipv4: None,
                    ipv6: Some(Ipv6Addr::from(octets)),
                })
            }
            20 => {
                let mut v4 = [0u8; 4];
                v4.copy_from_slice(&data[0..4]);
                let mut v6 = [0u8; 16];
                v6.copy_from_slice(&data[4..20]);
                Ok(Self {
                    ipv4: Some(Ipv4Addr::from(v4)),
                    ipv6: Some(Ipv6Addr::from(v6)),
                })
            }
            len => Err(PfcpError::invalid_value(
                "Mapped N6 IP Address",
                len.to_string(),
                "must be 4 (IPv4), 16 (IPv6), or 20 (both) bytes",
            )),
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MappedN6IpAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4() {
        let addr = MappedN6IpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let parsed = MappedN6IpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_marshal_unmarshal_ipv6() {
        let addr = MappedN6IpAddress::new_ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let parsed = MappedN6IpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_unmarshal_invalid_length() {
        assert!(matches!(
            MappedN6IpAddress::unmarshal(&[0x01, 0x02]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            MappedN6IpAddress::new_ipv4(Ipv4Addr::LOCALHOST)
                .to_ie()
                .ie_type,
            IeType::MappedN6IpAddress
        );
    }
}
