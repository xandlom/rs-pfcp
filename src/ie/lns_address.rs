//! LNS Address IE (IE Type 280).
//!
//! Per 3GPP TS 29.244 Section 8.2.188, contains the IP address of the L2TP
//! Network Server (LNS). IPv4 encodes as 4 bytes; IPv6 as 16 bytes.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LnsAddress {
    pub address: IpAddr,
}

impl LnsAddress {
    pub fn new(address: impl Into<IpAddr>) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        match self.address {
            IpAddr::V4(v4) => v4.octets().to_vec(),
            IpAddr::V6(v6) => v6.octets().to_vec(),
        }
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let address = match data.len() {
            4 => IpAddr::V4(Ipv4Addr::new(data[0], data[1], data[2], data[3])),
            16 => IpAddr::V6(Ipv6Addr::from(<[u8; 16]>::try_from(data).unwrap())),
            n => {
                return Err(PfcpError::invalid_length(
                    "LNS Address",
                    IeType::LnsAddress,
                    4,
                    n,
                ))
            }
        };
        Ok(Self { address })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::LnsAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_round_trip() {
        let original = LnsAddress::new(Ipv4Addr::new(10, 0, 0, 1));
        let parsed = LnsAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_ipv6_round_trip() {
        let original = LnsAddress::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let parsed = LnsAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_invalid_length() {
        assert!(matches!(
            LnsAddress::unmarshal(&[1, 2, 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = LnsAddress::new(Ipv4Addr::new(1, 2, 3, 4)).to_ie();
        assert_eq!(ie.ie_type, IeType::LnsAddress);
    }
}
