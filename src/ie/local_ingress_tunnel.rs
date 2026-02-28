//! Local Ingress Tunnel Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.209, indicates the local ingress tunnel
//! endpoint for an RTP or MPQUIC session.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Local Ingress Tunnel per 3GPP TS 29.244 §8.2.209.
///
/// # Wire Format
/// - Byte 0: flags (V4=0x01, V6=0x02, CH=0x04)
/// - If CH set: no further fields (UPF chooses the endpoint)
/// - If CH not set:
///   - 2 bytes UDP port
///   - If V4: 4 bytes IPv4 address
///   - If V6: 16 bytes IPv6 address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalIngressTunnel {
    /// When set, the UPF shall choose the local ingress tunnel endpoint.
    pub choose: bool,
    /// UDP port number (present when `choose` is false).
    pub port: Option<u16>,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl LocalIngressTunnel {
    /// Create a CHOOSE endpoint (UPF selects the address and port).
    pub fn choose() -> Self {
        Self {
            choose: true,
            port: None,
            ipv4: None,
            ipv6: None,
        }
    }

    /// Create an explicit IPv4 endpoint.
    pub fn new_ipv4(port: u16, addr: Ipv4Addr) -> Self {
        Self {
            choose: false,
            port: Some(port),
            ipv4: Some(addr),
            ipv6: None,
        }
    }

    /// Create an explicit IPv6 endpoint.
    pub fn new_ipv6(port: u16, addr: Ipv6Addr) -> Self {
        Self {
            choose: false,
            port: Some(port),
            ipv4: None,
            ipv6: Some(addr),
        }
    }

    /// Create an explicit dual-stack endpoint.
    pub fn new_dual_stack(port: u16, ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        Self {
            choose: false,
            port: Some(port),
            ipv4: Some(ipv4),
            ipv6: Some(ipv6),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut flags = 0u8;
        if self.ipv4.is_some() {
            flags |= 0x01;
        }
        if self.ipv6.is_some() {
            flags |= 0x02;
        }
        if self.choose {
            flags |= 0x04;
        }
        let mut data = vec![flags];
        if !self.choose {
            if let Some(port) = self.port {
                data.extend_from_slice(&port.to_be_bytes());
            }
            if let Some(ip) = self.ipv4 {
                data.extend_from_slice(&ip.octets());
            }
            if let Some(ip) = self.ipv6 {
                data.extend_from_slice(&ip.octets());
            }
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Local Ingress Tunnel",
                IeType::LocalIngressTunnel,
                1,
                0,
            ));
        }
        let flags = data[0];
        let v4 = (flags & 0x01) != 0;
        let v6 = (flags & 0x02) != 0;
        let ch = (flags & 0x04) != 0;

        if ch {
            return Ok(Self {
                choose: true,
                port: None,
                ipv4: None,
                ipv6: None,
            });
        }

        // Without CH: 2-byte port + optional IPv4/IPv6
        let expected = 1 + 2 + (v4 as usize * 4) + (v6 as usize * 16);
        if data.len() < expected {
            return Err(PfcpError::invalid_length(
                "Local Ingress Tunnel",
                IeType::LocalIngressTunnel,
                expected,
                data.len(),
            ));
        }

        let port = u16::from_be_bytes(data[1..3].try_into().unwrap());
        let mut offset = 3;

        let ipv4 = if v4 {
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            Some(Ipv4Addr::from(octets))
        } else {
            None
        };
        let ipv6 = if v6 {
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        Ok(Self {
            choose: false,
            port: Some(port),
            ipv4,
            ipv6,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::LocalIngressTunnel, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_choose() {
        let ie = LocalIngressTunnel::choose();
        let parsed = LocalIngressTunnel::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
        assert!(parsed.choose);
    }

    #[test]
    fn test_marshal_unmarshal_ipv4() {
        let ie = LocalIngressTunnel::new_ipv4(5004, Ipv4Addr::new(10, 0, 0, 1));
        let parsed = LocalIngressTunnel::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_ipv6() {
        let ie = LocalIngressTunnel::new_ipv6(5004, Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let parsed = LocalIngressTunnel::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_dual_stack() {
        let ie = LocalIngressTunnel::new_dual_stack(
            8080,
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
        );
        let parsed = LocalIngressTunnel::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_choose_flag_byte() {
        let ie = LocalIngressTunnel::choose();
        assert_eq!(ie.marshal(), [0x04]); // CH flag only
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            LocalIngressTunnel::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_explicit() {
        // No CH, V4 flag, but not enough bytes
        assert!(matches!(
            LocalIngressTunnel::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = LocalIngressTunnel::new_ipv4(1234, Ipv4Addr::LOCALHOST).to_ie();
        assert_eq!(ie.ie_type, IeType::LocalIngressTunnel);
        assert_eq!(ie.payload.len(), 7); // 1 flags + 2 port + 4 IPv4
    }
}
