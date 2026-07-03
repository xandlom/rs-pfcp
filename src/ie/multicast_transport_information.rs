//! Multicast Transport Information IE (IE Type 306).
//!
//! Per 3GPP TS 29.244 Section 8.2.207, carries the Common GTP-U TEID, IP
//! Multicast Distribution Address, and IP Source Address for MBS multicast.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulticastTransportInformation {
    /// IP multicast distribution (destination) address.
    pub multicast_address: IpAddr,
    /// IP source address for the SSM group.
    pub source_address: IpAddr,
    /// Common GTP-U Tunnel Endpoint Identifier (C-TEID).
    pub common_teid: u32,
}

impl MulticastTransportInformation {
    pub fn new(multicast_address: IpAddr, source_address: IpAddr, common_teid: u32) -> Self {
        Self {
            multicast_address,
            source_address,
            common_teid,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        // Spare(1) + TEID(4) + addr1(1+4..16) + addr2(1+4..16)
        let mut bytes = Vec::with_capacity(15);
        bytes.push(0x00); // spare
        bytes.extend_from_slice(&self.common_teid.to_be_bytes());
        encode_addr(&mut bytes, self.multicast_address);
        encode_addr(&mut bytes, self.source_address);
        bytes
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        // Minimum: 1 spare + 4 TEID + (1+4) mcast + (1+4) src = 15 bytes
        if data.len() < 15 {
            return Err(PfcpError::invalid_length(
                "MulticastTransportInformation",
                IeType::MulticastTransportInformation,
                15,
                data.len(),
            ));
        }
        let common_teid = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
        let (multicast_address, rest) = decode_addr(&data[5..])?;
        let (source_address, _) = decode_addr(rest)?;
        Ok(Self {
            multicast_address,
            source_address,
            common_teid,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MulticastTransportInformation, self.marshal())
    }
}

fn encode_addr(buf: &mut Vec<u8>, addr: IpAddr) {
    match addr {
        IpAddr::V4(v4) => {
            buf.push(0x04); // type=0 (IPv4, bits 7-6), len=4 (bits 5-0)
            buf.extend_from_slice(&v4.octets());
        }
        IpAddr::V6(v6) => {
            buf.push(0x50); // type=1 (IPv6, bits 7-6), len=16 (bits 5-0)
            buf.extend_from_slice(&v6.octets());
        }
    }
}

fn decode_addr(data: &[u8]) -> Result<(IpAddr, &[u8]), PfcpError> {
    if data.is_empty() {
        return Err(PfcpError::invalid_length(
            "MulticastTransportInformation address",
            IeType::MulticastTransportInformation,
            1,
            0,
        ));
    }
    let type_len = data[0];
    let addr_type = (type_len >> 6) & 0x03;
    let addr_len = (type_len & 0x3F) as usize;
    if data.len() < 1 + addr_len {
        return Err(PfcpError::invalid_length(
            "MulticastTransportInformation address",
            IeType::MulticastTransportInformation,
            1 + addr_len,
            data.len(),
        ));
    }
    let addr_bytes = &data[1..1 + addr_len];
    let addr = match addr_type {
        0 => {
            if addr_len != 4 {
                return Err(PfcpError::invalid_value(
                    "MulticastTransportInformation address length",
                    format!("{addr_len}"),
                    "expected 4 bytes for IPv4 address type",
                ));
            }
            IpAddr::V4(Ipv4Addr::new(
                addr_bytes[0],
                addr_bytes[1],
                addr_bytes[2],
                addr_bytes[3],
            ))
        }
        1 => {
            if addr_len != 16 {
                return Err(PfcpError::invalid_value(
                    "MulticastTransportInformation address length",
                    format!("{addr_len}"),
                    "expected 16 bytes for IPv6 address type",
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(addr_bytes);
            IpAddr::V6(Ipv6Addr::from(octets))
        }
        _ => {
            return Err(PfcpError::invalid_value(
                "MulticastTransportInformation address type",
                format!("{addr_type}"),
                "expected 0 (IPv4) or 1 (IPv6)",
            ));
        }
    };
    Ok((addr, &data[1 + addr_len..]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn ipv4_mcast() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(239, 1, 2, 3))
    }
    fn ipv4_src() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))
    }
    fn ipv6_mcast() -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 1))
    }
    fn ipv6_src() -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(0x2001, 0x0DB8, 0, 0, 0, 0, 0, 1))
    }

    #[test]
    fn test_round_trip_ipv4() {
        let original = MulticastTransportInformation::new(ipv4_mcast(), ipv4_src(), 0xDEADBEEF);
        let parsed = MulticastTransportInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_ipv6() {
        let original = MulticastTransportInformation::new(ipv6_mcast(), ipv6_src(), 0x12345678);
        let parsed = MulticastTransportInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_mixed() {
        let original = MulticastTransportInformation::new(ipv4_mcast(), ipv6_src(), 0x00000001);
        let parsed = MulticastTransportInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer_fails() {
        assert!(matches!(
            MulticastTransportInformation::unmarshal(&[0u8; 14]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_empty_fails() {
        assert!(matches!(
            MulticastTransportInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = MulticastTransportInformation::new(ipv4_mcast(), ipv4_src(), 1).to_ie();
        assert_eq!(ie.ie_type, IeType::MulticastTransportInformation);
    }
}
