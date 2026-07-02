//! Created L2TP Session IE (IE Type 279).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.1-2, a grouped IE sent in Session
//! Establishment Response with addresses allocated by the UPF for the L2TP
//! session.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Created L2TP Session grouped IE.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CreatedL2tpSession {
    pub dns_server_addresses: Vec<Ie>,  // O (multiple) - IE 285
    pub nbns_server_addresses: Vec<Ie>, // O (multiple) - IE 286
    pub lns_address: Option<Ie>,        // O - IE 280
}

impl CreatedL2tpSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dns_server_address(mut self, ie: Ie) -> Self {
        self.dns_server_addresses.push(ie);
        self
    }

    pub fn add_nbns_server_address(mut self, ie: Ie) -> Self {
        self.nbns_server_addresses.push(ie);
        self
    }

    pub fn with_lns_address(mut self, ie: Ie) -> Self {
        self.lns_address = Some(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        for ie in &self.dns_server_addresses {
            ies.push(ie.clone());
        }
        for ie in &self.nbns_server_addresses {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.lns_address {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut dns_server_addresses = Vec::new();
        let mut nbns_server_addresses = Vec::new();
        let mut lns_address = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::DnsServerAddress {
                dns_server_addresses.push(ie);
            } else if ie.ie_type == IeType::NbnsServerAddress {
                nbns_server_addresses.push(ie);
            } else if ie.ie_type == IeType::LnsAddress {
                lns_address = Some(ie);
            }
        }

        Ok(Self {
            dns_server_addresses,
            nbns_server_addresses,
            lns_address,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatedL2tpSession, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::dns_server_address::DnsServerAddress;
    use crate::ie::lns_address::LnsAddress;
    use crate::ie::nbns_server_address::NbnsServerAddress;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_empty_round_trip() {
        let original = CreatedL2tpSession::new();
        let ie = original.to_ie();
        let parsed = CreatedL2tpSession::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_full_round_trip() {
        let original = CreatedL2tpSession::new()
            .add_dns_server_address(DnsServerAddress::new(Ipv4Addr::new(8, 8, 8, 8)).to_ie())
            .add_dns_server_address(DnsServerAddress::new(Ipv4Addr::new(8, 8, 4, 4)).to_ie())
            .add_nbns_server_address(NbnsServerAddress::new(Ipv4Addr::new(192, 168, 1, 1)).to_ie())
            .with_lns_address(
                LnsAddress::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)).to_ie(),
            );
        let ie = original.to_ie();
        let parsed = CreatedL2tpSession::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_multiple_dns_preserved() {
        let original = CreatedL2tpSession::new()
            .add_dns_server_address(DnsServerAddress::new(Ipv4Addr::new(1, 1, 1, 1)).to_ie())
            .add_dns_server_address(DnsServerAddress::new(Ipv4Addr::new(1, 0, 0, 1)).to_ie());
        let ie = original.to_ie();
        let parsed = CreatedL2tpSession::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed.dns_server_addresses.len(), 2);
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            CreatedL2tpSession::new().to_ie().ie_type,
            IeType::CreatedL2tpSession
        );
    }
}
