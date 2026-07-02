//! PMF Address Information IE (IE Type 230).
//!
//! Per 3GPP TS 29.244 Section 8.2.159, contains the address information of the
//! Performance Measure Function (PMF) in the UPF.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// PMF Address Information.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.159
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmfAddressInformation {
    /// PMF IPv4 Address (present when V4 flag set)
    pub ipv4_address: Option<Ipv4Addr>,
    /// PMF IPv6 Address (present when V6 flag set)
    pub ipv6_address: Option<Ipv6Addr>,
    /// PMF Port for 3GPP Access (present when V4 or V6 set)
    pub port_3gpp: Option<u16>,
    /// PMF Port for Non-3GPP Access (present when V4 or V6 set)
    pub port_non3gpp: Option<u16>,
    /// PMF MAC Address for 3GPP Access (present when MAC flag set)
    pub mac_3gpp: Option<[u8; 6]>,
    /// PMF MAC Address for Non-3GPP Access (present when MAC flag set)
    pub mac_non3gpp: Option<[u8; 6]>,
}

impl PmfAddressInformation {
    pub fn new() -> Self {
        Self {
            ipv4_address: None,
            ipv6_address: None,
            port_3gpp: None,
            port_non3gpp: None,
            mac_3gpp: None,
            mac_non3gpp: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let has_ip = self.ipv4_address.is_some() || self.ipv6_address.is_some();
        let has_mac = self.mac_3gpp.is_some() || self.mac_non3gpp.is_some();
        let mut flags = 0u8;
        if self.ipv4_address.is_some() {
            flags |= 0x01; // V4
        }
        if self.ipv6_address.is_some() {
            flags |= 0x02; // V6
        }
        if has_mac {
            flags |= 0x04; // MAC
        }
        let mut data = vec![flags];
        if let Some(ipv4) = self.ipv4_address {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = self.ipv6_address {
            data.extend_from_slice(&ipv6.octets());
        }
        if has_ip {
            let p3 = self.port_3gpp.unwrap_or(0);
            let pn = self.port_non3gpp.unwrap_or(0);
            data.extend_from_slice(&p3.to_be_bytes());
            data.extend_from_slice(&pn.to_be_bytes());
        }
        if let Some(mac) = &self.mac_3gpp {
            data.extend_from_slice(mac);
        }
        if let Some(mac) = &self.mac_non3gpp {
            data.extend_from_slice(mac);
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PMF Address Information",
                IeType::PmfAddressInformation,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v4 = (flags & 0x01) != 0;
        let v6 = (flags & 0x02) != 0;
        let mac = (flags & 0x04) != 0;
        let has_ip = v4 || v6;

        let mut offset = 1usize;

        let ipv4_address = if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "PMF Address Information (IPv4)",
                    IeType::PmfAddressInformation,
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

        let ipv6_address = if v6 {
            if data.len() < offset + 16 {
                return Err(PfcpError::invalid_length(
                    "PMF Address Information (IPv6)",
                    IeType::PmfAddressInformation,
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

        let (port_3gpp, port_non3gpp) = if has_ip {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "PMF Address Information (ports)",
                    IeType::PmfAddressInformation,
                    offset + 4,
                    data.len(),
                ));
            }
            let p3 = u16::from_be_bytes([data[offset], data[offset + 1]]);
            let pn = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
            offset += 4;
            (Some(p3), Some(pn))
        } else {
            (None, None)
        };

        let mac_3gpp = if mac {
            if data.len() < offset + 6 {
                return Err(PfcpError::invalid_length(
                    "PMF Address Information (MAC 3GPP)",
                    IeType::PmfAddressInformation,
                    offset + 6,
                    data.len(),
                ));
            }
            let mut m = [0u8; 6];
            m.copy_from_slice(&data[offset..offset + 6]);
            offset += 6;
            Some(m)
        } else {
            None
        };

        let mac_non3gpp = if mac {
            if data.len() < offset + 6 {
                return Err(PfcpError::invalid_length(
                    "PMF Address Information (MAC non-3GPP)",
                    IeType::PmfAddressInformation,
                    offset + 6,
                    data.len(),
                ));
            }
            let mut m = [0u8; 6];
            m.copy_from_slice(&data[offset..offset + 6]);
            Some(m)
        } else {
            None
        };

        Ok(Self {
            ipv4_address,
            ipv6_address,
            port_3gpp,
            port_non3gpp,
            mac_3gpp,
            mac_non3gpp,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PmfAddressInformation, self.marshal())
    }
}

impl Default for PmfAddressInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4_with_ports() {
        let original = PmfAddressInformation {
            ipv4_address: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6_address: None,
            port_3gpp: Some(5001),
            port_non3gpp: Some(5002),
            mac_3gpp: None,
            mac_non3gpp: None,
        };
        let parsed = PmfAddressInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_mac_only() {
        let original = PmfAddressInformation {
            ipv4_address: None,
            ipv6_address: None,
            port_3gpp: None,
            port_non3gpp: None,
            mac_3gpp: Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            mac_non3gpp: Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
        };
        let parsed = PmfAddressInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_dual_stack_with_mac() {
        let original = PmfAddressInformation {
            ipv4_address: Some(Ipv4Addr::new(192, 168, 1, 1)),
            ipv6_address: Some("2001:db8::1".parse().unwrap()),
            port_3gpp: Some(8080),
            port_non3gpp: Some(8081),
            mac_3gpp: Some([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]),
            mac_non3gpp: Some([0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C]),
        };
        let parsed = PmfAddressInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            PmfAddressInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PmfAddressInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::PmfAddressInformation);
    }
}
