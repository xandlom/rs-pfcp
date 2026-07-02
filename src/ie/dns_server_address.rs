//! DNS Server Address IE (IE Type 285).
//!
//! Per 3GPP TS 29.244 Section 8.2.193, contains an IPv4 DNS server address
//! allocated by the LNS for the L2TP session.

use std::net::Ipv4Addr;

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsServerAddress {
    pub address: Ipv4Addr,
}

impl DnsServerAddress {
    pub fn new(address: Ipv4Addr) -> Self {
        Self { address }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.address.octets().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "DNS Server Address",
                IeType::DnsServerAddress,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            address: Ipv4Addr::new(data[0], data[1], data[2], data[3]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DnsServerAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = DnsServerAddress::new(Ipv4Addr::new(8, 8, 8, 8));
        let parsed = DnsServerAddress::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            DnsServerAddress::unmarshal(&[8, 8, 8]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            DnsServerAddress::new(Ipv4Addr::new(8, 8, 8, 8))
                .to_ie()
                .ie_type,
            IeType::DnsServerAddress
        );
    }
}
