//! CP PFCP Entity IP Address Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.152, contains the CP PFCP entity IP address.
//! Same encoding as CP IP Address: flags + v4/v6.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpPfcpEntityIpAddress {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl CpPfcpEntityIpAddress {
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

    pub fn new_dual_stack(ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        Self {
            ipv4: Some(ipv4),
            ipv6: Some(ipv6),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0u8;
        if self.ipv6.is_some() {
            flags |= 0x01;
        }
        if self.ipv4.is_some() {
            flags |= 0x02;
        }
        data.push(flags);
        if let Some(ipv4) = &self.ipv4 {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = &self.ipv6 {
            data.extend_from_slice(&ipv6.octets());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "CP PFCP Entity IP Address",
                IeType::CpPfcpEntityIpAddress,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v6 = (flags & 0x01) != 0;
        let v4 = (flags & 0x02) != 0;

        if !v4 && !v6 {
            return Err(PfcpError::invalid_value(
                "CP PFCP Entity IP Address flags",
                format!("0x{:02X}", flags),
                "at least one IP address must be present",
            ));
        }

        let mut offset = 1;
        let mut ipv4 = None;
        let mut ipv6 = None;

        if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "CP PFCP Entity IP Address (IPv4)",
                    IeType::CpPfcpEntityIpAddress,
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
                    "CP PFCP Entity IP Address (IPv6)",
                    IeType::CpPfcpEntityIpAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            ipv6 = Some(Ipv6Addr::from(octets));
        }

        Ok(Self { ipv4, ipv6 })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CpPfcpEntityIpAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4() {
        let addr = CpPfcpEntityIpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let parsed = CpPfcpEntityIpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_marshal_unmarshal_ipv6() {
        let addr = CpPfcpEntityIpAddress::new_ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let parsed = CpPfcpEntityIpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_marshal_unmarshal_dual() {
        let addr = CpPfcpEntityIpAddress::new_dual_stack(
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
        );
        let parsed = CpPfcpEntityIpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            CpPfcpEntityIpAddress::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_no_flags() {
        assert!(matches!(
            CpPfcpEntityIpAddress::unmarshal(&[0x00]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            CpPfcpEntityIpAddress::new_ipv4(Ipv4Addr::LOCALHOST)
                .to_ie()
                .ie_type,
            IeType::CpPfcpEntityIpAddress
        );
    }
}
