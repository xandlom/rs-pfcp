//! IP Multicast Address Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.158, contains IP multicast addressing information.
//! Flags + optional source v4/v6 + any-source v4/v6.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpMulticastAddress {
    pub any_source_ipv4: Option<Ipv4Addr>,
    pub any_source_ipv6: Option<Ipv6Addr>,
    pub source_ipv4: Option<Ipv4Addr>,
    pub source_ipv6: Option<Ipv6Addr>,
}

impl IpMulticastAddress {
    pub fn any_source_v4(multicast: Ipv4Addr) -> Self {
        Self {
            any_source_ipv4: Some(multicast),
            any_source_ipv6: None,
            source_ipv4: None,
            source_ipv6: None,
        }
    }

    pub fn any_source_v6(multicast: Ipv6Addr) -> Self {
        Self {
            any_source_ipv4: None,
            any_source_ipv6: Some(multicast),
            source_ipv4: None,
            source_ipv6: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.any_source_ipv6.is_some() || self.source_ipv6.is_some() {
            flags |= 0x01; // V6
        }
        if self.any_source_ipv4.is_some() || self.source_ipv4.is_some() {
            flags |= 0x02; // V4
        }
        // ANY bit (bit 4) - indicates any-source multicast
        if self.source_ipv4.is_none() && self.source_ipv6.is_none() {
            flags |= 0x08; // ANY
        }

        let mut data = vec![flags];
        if let Some(addr) = &self.any_source_ipv4 {
            data.extend_from_slice(&addr.octets());
        } else if let Some(addr) = &self.source_ipv4 {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = &self.any_source_ipv6 {
            data.extend_from_slice(&addr.octets());
        } else if let Some(addr) = &self.source_ipv6 {
            data.extend_from_slice(&addr.octets());
        }
        // For source-specific multicast, also include multicast address
        // (simplified: for now we encode the source address fields)
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "IP Multicast Address",
                IeType::IpMulticastAddress,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v6 = (flags & 0x01) != 0;
        let v4 = (flags & 0x02) != 0;
        let any = (flags & 0x08) != 0;

        let mut offset = 1;
        let mut ipv4 = None;
        let mut ipv6 = None;

        if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "IP Multicast Address (IPv4)",
                    IeType::IpMulticastAddress,
                    offset + 4,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            ipv4 = Some(Ipv4Addr::from(octets));
            offset += 4;
        }
        if v6 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "IP Multicast Address (IPv6)",
                    IeType::IpMulticastAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            ipv6 = Some(Ipv6Addr::from(octets));
        }

        if any {
            Ok(Self {
                any_source_ipv4: ipv4,
                any_source_ipv6: ipv6,
                source_ipv4: None,
                source_ipv6: None,
            })
        } else {
            Ok(Self {
                any_source_ipv4: None,
                any_source_ipv6: None,
                source_ipv4: ipv4,
                source_ipv6: ipv6,
            })
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::IpMulticastAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_any_v4() {
        let addr = IpMulticastAddress::any_source_v4(Ipv4Addr::new(239, 1, 1, 1));
        let parsed = IpMulticastAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_marshal_unmarshal_any_v6() {
        let addr = IpMulticastAddress::any_source_v6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1));
        let parsed = IpMulticastAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            IpMulticastAddress::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            IpMulticastAddress::any_source_v4(Ipv4Addr::new(239, 0, 0, 1))
                .to_ie()
                .ie_type,
            IeType::IpMulticastAddress
        );
    }
}
