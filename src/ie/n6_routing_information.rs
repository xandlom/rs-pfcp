//! N6 Routing Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.243, contains source and destination
//! IP address and port information for N6 routing.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// N6 Routing Information per 3GPP TS 29.244 §8.2.243.
///
/// # Wire Format
/// - Byte 0: flags
///   - Bit 1 (SIPV4=0x01): Source IPv4 present
///   - Bit 2 (SIPV6=0x02): Source IPv6 present
///   - Bit 3 (SPO=0x04): Source Port present
///   - Bit 4 (DIPV4=0x08): Destination IPv4 present
///   - Bit 5 (DIPV6=0x10): Destination IPv6 present
///   - Bit 6 (DPO=0x20): Destination Port present
/// - If SIPV4: 4 bytes source IPv4
/// - If SIPV6: 16 bytes source IPv6
/// - If SPO: 2 bytes source port
/// - If DIPV4: 4 bytes destination IPv4
/// - If DIPV6: 16 bytes destination IPv6
/// - If DPO: 2 bytes destination port
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct N6RoutingInformation {
    pub src_ipv4: Option<Ipv4Addr>,
    pub src_ipv6: Option<Ipv6Addr>,
    pub src_port: Option<u16>,
    pub dst_ipv4: Option<Ipv4Addr>,
    pub dst_ipv6: Option<Ipv6Addr>,
    pub dst_port: Option<u16>,
}

impl N6RoutingInformation {
    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.src_ipv4.is_some() {
            flags |= 0x01;
        }
        if self.src_ipv6.is_some() {
            flags |= 0x02;
        }
        if self.src_port.is_some() {
            flags |= 0x04;
        }
        if self.dst_ipv4.is_some() {
            flags |= 0x08;
        }
        if self.dst_ipv6.is_some() {
            flags |= 0x10;
        }
        if self.dst_port.is_some() {
            flags |= 0x20;
        }
        let mut data = vec![flags];
        if let Some(ip) = self.src_ipv4 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(ip) = self.src_ipv6 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(p) = self.src_port {
            data.extend_from_slice(&p.to_be_bytes());
        }
        if let Some(ip) = self.dst_ipv4 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(ip) = self.dst_ipv6 {
            data.extend_from_slice(&ip.octets());
        }
        if let Some(p) = self.dst_port {
            data.extend_from_slice(&p.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "N6 Routing Information",
                IeType::N6RoutingInformation,
                1,
                0,
            ));
        }
        let flags = data[0];
        let sipv4 = (flags & 0x01) != 0;
        let sipv6 = (flags & 0x02) != 0;
        let spo = (flags & 0x04) != 0;
        let dipv4 = (flags & 0x08) != 0;
        let dipv6 = (flags & 0x10) != 0;
        let dpo = (flags & 0x20) != 0;
        let expected = 1
            + (sipv4 as usize * 4)
            + (sipv6 as usize * 16)
            + (spo as usize * 2)
            + (dipv4 as usize * 4)
            + (dipv6 as usize * 16)
            + (dpo as usize * 2);
        if data.len() < expected {
            return Err(PfcpError::invalid_length(
                "N6 Routing Information",
                IeType::N6RoutingInformation,
                expected,
                data.len(),
            ));
        }
        let mut offset = 1;
        let src_ipv4 = if sipv4 {
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            Some(Ipv4Addr::from(octets))
        } else {
            None
        };
        let src_ipv6 = if sipv6 {
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };
        let src_port = if spo {
            let v = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
            offset += 2;
            Some(v)
        } else {
            None
        };
        let dst_ipv4 = if dipv4 {
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            Some(Ipv4Addr::from(octets))
        } else {
            None
        };
        let dst_ipv6 = if dipv6 {
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };
        let dst_port = if dpo {
            Some(u16::from_be_bytes(
                data[offset..offset + 2].try_into().unwrap(),
            ))
        } else {
            None
        };
        Ok(Self {
            src_ipv4,
            src_ipv6,
            src_port,
            dst_ipv4,
            dst_ipv6,
            dst_port,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::N6RoutingInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4_only() {
        let ie = N6RoutingInformation {
            src_ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            src_ipv6: None,
            src_port: None,
            dst_ipv4: Some(Ipv4Addr::new(192, 168, 1, 1)),
            dst_ipv6: None,
            dst_port: None,
        };
        let parsed = N6RoutingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_ports() {
        let ie = N6RoutingInformation {
            src_ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            src_ipv6: None,
            src_port: Some(12345),
            dst_ipv4: Some(Ipv4Addr::new(192, 168, 1, 1)),
            dst_ipv6: None,
            dst_port: Some(80),
        };
        let parsed = N6RoutingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_ipv6() {
        let ie = N6RoutingInformation {
            src_ipv4: None,
            src_ipv6: Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
            src_port: Some(5000),
            dst_ipv4: None,
            dst_ipv6: Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2)),
            dst_port: Some(8080),
        };
        let parsed = N6RoutingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let ie = N6RoutingInformation {
            src_ipv4: None,
            src_ipv6: None,
            src_port: None,
            dst_ipv4: None,
            dst_ipv6: None,
            dst_port: None,
        };
        let parsed = N6RoutingInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            N6RoutingInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_flags() {
        // SIPV4 flag set but no IPv4 data
        assert!(matches!(
            N6RoutingInformation::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = N6RoutingInformation {
            src_ipv4: Some(Ipv4Addr::LOCALHOST),
            src_ipv6: None,
            src_port: None,
            dst_ipv4: None,
            dst_ipv6: None,
            dst_port: None,
        }
        .to_ie();
        assert_eq!(ie.ie_type, IeType::N6RoutingInformation);
    }
}
