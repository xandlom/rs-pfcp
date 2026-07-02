//! MPQUIC Address Information IE (IE Type 332).
//!
//! Per 3GPP TS 29.244 Section 8.2.226, carries address information of the
//! MPQUIC proxy in the UPF. Same structure as MPTCP Address Information (IE 228).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// MPQUIC Address Information.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.226
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpquicAddressInformation {
    /// MPQUIC Proxy Type per 3GPP TS 24.193
    pub proxy_type: u8,
    /// MPQUIC Proxy Port number
    pub proxy_port: u16,
    /// MPQUIC Proxy IPv4 Address (present when V4 flag set)
    pub ipv4_address: Option<Ipv4Addr>,
    /// MPQUIC Proxy IPv6 Address (present when V6 flag set)
    pub ipv6_address: Option<Ipv6Addr>,
}

impl MpquicAddressInformation {
    pub fn new(proxy_type: u8, proxy_port: u16) -> Self {
        Self {
            proxy_type,
            proxy_port,
            ipv4_address: None,
            ipv6_address: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.ipv4_address.is_some() {
            flags |= 0x01; // V4
        }
        if self.ipv6_address.is_some() {
            flags |= 0x02; // V6
        }
        let mut data = vec![flags, self.proxy_type];
        data.extend_from_slice(&self.proxy_port.to_be_bytes());
        if let Some(ipv4) = self.ipv4_address {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = self.ipv6_address {
            data.extend_from_slice(&ipv6.octets());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "MPQUIC Address Information",
                IeType::MpquicAddressInformation,
                4,
                data.len(),
            ));
        }
        let flags = data[0];
        let v4 = (flags & 0x01) != 0;
        let v6 = (flags & 0x02) != 0;
        let proxy_type = data[1];
        let proxy_port = u16::from_be_bytes([data[2], data[3]]);

        let mut offset = 4;
        let ipv4_address = if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "MPQUIC Address Information (IPv4)",
                    IeType::MpquicAddressInformation,
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
                    "MPQUIC Address Information (IPv6)",
                    IeType::MpquicAddressInformation,
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
            proxy_type,
            proxy_port,
            ipv4_address,
            ipv6_address,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MpquicAddressInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4_only() {
        let mut original = MpquicAddressInformation::new(1, 8080);
        original.ipv4_address = Some(Ipv4Addr::new(10, 0, 0, 1));
        let parsed = MpquicAddressInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_dual_stack() {
        let mut original = MpquicAddressInformation::new(0, 443);
        original.ipv4_address = Some(Ipv4Addr::new(192, 168, 1, 1));
        original.ipv6_address = Some("fe80::1".parse().unwrap());
        let parsed = MpquicAddressInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            MpquicAddressInformation::unmarshal(&[0x01, 0x00]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MpquicAddressInformation::new(1, 8080).to_ie();
        assert_eq!(ie.ie_type, IeType::MpquicAddressInformation);
    }
}
