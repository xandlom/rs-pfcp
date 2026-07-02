//! Link-Specific Multipath IP Address IE (IE Type 229).
//!
//! Per 3GPP TS 29.244 Section 8.2.158, carries link-specific IP addresses
//! used for MPTCP or MPQUIC steering. Mapped to IeType::UeLinkSpecificIpAddress.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Link-Specific Multipath IP Address.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.158
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkSpecificMultipathIpAddress {
    /// Link-specific multipath IPv4 address for 3GPP access
    pub ipv4_3gpp: Option<Ipv4Addr>,
    /// Link-specific multipath IPv6 address for 3GPP access
    pub ipv6_3gpp: Option<Ipv6Addr>,
    /// Link-specific multipath IPv4 address for non-3GPP access
    pub ipv4_non3gpp: Option<Ipv4Addr>,
    /// Link-specific multipath IPv6 address for non-3GPP access
    pub ipv6_non3gpp: Option<Ipv6Addr>,
}

impl LinkSpecificMultipathIpAddress {
    pub fn new() -> Self {
        Self {
            ipv4_3gpp: None,
            ipv6_3gpp: None,
            ipv4_non3gpp: None,
            ipv6_non3gpp: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.ipv4_3gpp.is_some() {
            flags |= 0x01; // V4
        }
        if self.ipv6_3gpp.is_some() {
            flags |= 0x02; // V6
        }
        if self.ipv4_non3gpp.is_some() {
            flags |= 0x04; // NV4
        }
        if self.ipv6_non3gpp.is_some() {
            flags |= 0x08; // NV6
        }
        let mut data = vec![flags];
        if let Some(addr) = self.ipv4_3gpp {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_3gpp {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv4_non3gpp {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_non3gpp {
            data.extend_from_slice(&addr.octets());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Link-Specific Multipath IP Address",
                IeType::UeLinkSpecificIpAddress,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v4 = (flags & 0x01) != 0;
        let v6 = (flags & 0x02) != 0;
        let nv4 = (flags & 0x04) != 0;
        let nv6 = (flags & 0x08) != 0;

        let mut offset = 1usize;

        let ipv4_3gpp = if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Link-Specific Multipath IP Address (3GPP IPv4)",
                    IeType::UeLinkSpecificIpAddress,
                    offset + 4,
                    data.len(),
                ));
            }
            let addr = Ipv4Addr::new(
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };

        let ipv6_3gpp = if v6 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "Link-Specific Multipath IP Address (3GPP IPv6)",
                    IeType::UeLinkSpecificIpAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        let ipv4_non3gpp = if nv4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Link-Specific Multipath IP Address (non-3GPP IPv4)",
                    IeType::UeLinkSpecificIpAddress,
                    offset + 4,
                    data.len(),
                ));
            }
            let addr = Ipv4Addr::new(
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };

        let ipv6_non3gpp = if nv6 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "Link-Specific Multipath IP Address (non-3GPP IPv6)",
                    IeType::UeLinkSpecificIpAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        Ok(Self {
            ipv4_3gpp,
            ipv6_3gpp,
            ipv4_non3gpp,
            ipv6_non3gpp,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UeLinkSpecificIpAddress, self.marshal())
    }
}

impl Default for LinkSpecificMultipathIpAddress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_3gpp_ipv4_only() {
        let mut original = LinkSpecificMultipathIpAddress::new();
        original.ipv4_3gpp = Some(Ipv4Addr::new(10, 1, 0, 1));
        let parsed = LinkSpecificMultipathIpAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_all_addresses() {
        let original = LinkSpecificMultipathIpAddress {
            ipv4_3gpp: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6_3gpp: Some("2001:db8::1".parse().unwrap()),
            ipv4_non3gpp: Some(Ipv4Addr::new(10, 0, 0, 2)),
            ipv6_non3gpp: Some("2001:db8::2".parse().unwrap()),
        };
        let parsed = LinkSpecificMultipathIpAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_non3gpp_only() {
        let mut original = LinkSpecificMultipathIpAddress::new();
        original.ipv4_non3gpp = Some(Ipv4Addr::new(192, 168, 1, 1));
        let parsed = LinkSpecificMultipathIpAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            LinkSpecificMultipathIpAddress::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = LinkSpecificMultipathIpAddress::new().to_ie();
        assert_eq!(ie.ie_type, IeType::UeLinkSpecificIpAddress);
    }
}
