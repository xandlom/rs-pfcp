//! Path Failure Report IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents the Path Failure Report Information Element.
/// Used to report path failures in multi-path scenarios for network resilience.
/// Defined in 3GPP TS 29.244 Section 8.2.105.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathFailureReport {
    pub remote_peer_addresses: Vec<RemotePeerAddress>,
}

/// Represents a remote peer address that has failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemotePeerAddress {
    pub address_type: AddressType,
    pub address: Vec<u8>,
}

/// Address type for remote peer addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressType {
    /// IPv4 address (4 bytes)
    Ipv4 = 0,
    /// IPv6 address (16 bytes)
    Ipv6 = 1,
    /// FQDN (Fully Qualified Domain Name)
    Fqdn = 2,
    /// Unknown/reserved address type
    Unknown(u8),
}

impl From<u8> for AddressType {
    fn from(value: u8) -> Self {
        match value {
            0 => AddressType::Ipv4,
            1 => AddressType::Ipv6,
            2 => AddressType::Fqdn,
            _ => AddressType::Unknown(value),
        }
    }
}

impl From<AddressType> for u8 {
    fn from(addr_type: AddressType) -> u8 {
        match addr_type {
            AddressType::Ipv4 => 0,
            AddressType::Ipv6 => 1,
            AddressType::Fqdn => 2,
            AddressType::Unknown(value) => value,
        }
    }
}

impl RemotePeerAddress {
    /// Creates a new remote peer address.
    pub fn new(address_type: AddressType, address: Vec<u8>) -> Self {
        RemotePeerAddress {
            address_type,
            address,
        }
    }

    /// Creates a remote peer address with IPv4.
    pub fn ipv4(ipv4: std::net::Ipv4Addr) -> Self {
        RemotePeerAddress::new(AddressType::Ipv4, ipv4.octets().to_vec())
    }

    /// Creates a remote peer address with IPv6.
    pub fn ipv6(ipv6: std::net::Ipv6Addr) -> Self {
        RemotePeerAddress::new(AddressType::Ipv6, ipv6.octets().to_vec())
    }

    /// Creates a remote peer address with FQDN.
    pub fn fqdn(fqdn: String) -> Self {
        RemotePeerAddress::new(AddressType::Fqdn, fqdn.into_bytes())
    }

    /// Gets the address as IPv4 if possible.
    pub fn as_ipv4(&self) -> Option<std::net::Ipv4Addr> {
        if self.address_type == AddressType::Ipv4 && self.address.len() == 4 {
            Some(std::net::Ipv4Addr::new(
                self.address[0],
                self.address[1],
                self.address[2],
                self.address[3],
            ))
        } else {
            None
        }
    }

    /// Gets the address as IPv6 if possible.
    pub fn as_ipv6(&self) -> Option<std::net::Ipv6Addr> {
        if self.address_type == AddressType::Ipv6 && self.address.len() == 16 {
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&self.address);
            Some(std::net::Ipv6Addr::from(octets))
        } else {
            None
        }
    }

    /// Gets the address as FQDN string if possible.
    pub fn as_fqdn(&self) -> Option<String> {
        if self.address_type == AddressType::Fqdn {
            String::from_utf8(self.address.clone()).ok()
        } else {
            None
        }
    }

    /// Marshals the remote peer address.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(u8::from(self.address_type.clone()));
        data.push(self.address.len() as u8);
        data.extend_from_slice(&self.address);
        data
    }

    /// Unmarshals a remote peer address.
    pub fn unmarshal(payload: &[u8], offset: &mut usize) -> Result<Self, PfcpError> {
        if *offset + 2 > payload.len() {
            return Err(PfcpError::invalid_length(
                "Remote peer address",
                IeType::UserPlanePathFailureReport,
                *offset + 2,
                payload.len(),
            ));
        }

        let address_type = AddressType::from(payload[*offset]);
        *offset += 1;

        let address_len = payload[*offset] as usize;
        *offset += 1;

        if *offset + address_len > payload.len() {
            return Err(PfcpError::invalid_length(
                "Remote peer address data",
                IeType::UserPlanePathFailureReport,
                *offset + address_len,
                payload.len(),
            ));
        }

        let address = payload[*offset..*offset + address_len].to_vec();
        *offset += address_len;

        Ok(RemotePeerAddress {
            address_type,
            address,
        })
    }
}

impl PathFailureReport {
    /// Creates a new Path Failure Report IE.
    pub fn new(remote_peer_addresses: Vec<RemotePeerAddress>) -> Self {
        PathFailureReport {
            remote_peer_addresses,
        }
    }

    /// Creates an empty Path Failure Report.
    pub fn empty() -> Self {
        PathFailureReport::new(Vec::new())
    }

    /// Adds a remote peer address to the failure report.
    pub fn add_peer_address(mut self, peer_address: RemotePeerAddress) -> Self {
        self.remote_peer_addresses.push(peer_address);
        self
    }

    /// Adds an IPv4 peer address to the failure report.
    pub fn add_ipv4_peer(mut self, ipv4: std::net::Ipv4Addr) -> Self {
        self.remote_peer_addresses
            .push(RemotePeerAddress::ipv4(ipv4));
        self
    }

    /// Adds an IPv6 peer address to the failure report.
    pub fn add_ipv6_peer(mut self, ipv6: std::net::Ipv6Addr) -> Self {
        self.remote_peer_addresses
            .push(RemotePeerAddress::ipv6(ipv6));
        self
    }

    /// Adds an FQDN peer address to the failure report.
    pub fn add_fqdn_peer(mut self, fqdn: String) -> Self {
        self.remote_peer_addresses
            .push(RemotePeerAddress::fqdn(fqdn));
        self
    }

    /// Gets the number of failed peer addresses.
    pub fn peer_count(&self) -> usize {
        self.remote_peer_addresses.len()
    }

    /// Checks if the failure report is empty.
    pub fn is_empty(&self) -> bool {
        self.remote_peer_addresses.is_empty()
    }

    /// Gets the length of the marshaled Path Failure Report.
    pub fn len(&self) -> usize {
        1 + self
            .remote_peer_addresses
            .iter()
            .map(|addr| 1 + 1 + addr.address.len()) // type + len + address
            .sum::<usize>()
    }

    /// Marshals the Path Failure Report into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Number of remote peer addresses
        data.push(self.remote_peer_addresses.len() as u8);

        // Each remote peer address
        for peer_addr in &self.remote_peer_addresses {
            data.extend_from_slice(&peer_addr.marshal());
        }

        data
    }

    /// Unmarshals a byte slice into a Path Failure Report IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "Path Failure Report",
                IeType::UserPlanePathFailureReport,
                1,
                0,
            ));
        }

        let mut offset = 0;
        let peer_count = payload[offset] as usize;
        offset += 1;

        let mut remote_peer_addresses = Vec::new();
        for _ in 0..peer_count {
            let peer_addr = RemotePeerAddress::unmarshal(payload, &mut offset)?;
            remote_peer_addresses.push(peer_addr);
        }

        Ok(PathFailureReport {
            remote_peer_addresses,
        })
    }

    /// Wraps the Path Failure Report in a Path Failure Report IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UserPlanePathFailureReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_remote_peer_address_ipv4() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let peer_addr = RemotePeerAddress::ipv4(ipv4);

        assert_eq!(peer_addr.address_type, AddressType::Ipv4);
        assert_eq!(peer_addr.address, vec![192, 168, 1, 100]);
        assert_eq!(peer_addr.as_ipv4(), Some(ipv4));
        assert_eq!(peer_addr.as_ipv6(), None);
        assert_eq!(peer_addr.as_fqdn(), None);
    }

    #[test]
    fn test_remote_peer_address_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0, 0, 0x8a2e, 0x370, 0x7334);
        let peer_addr = RemotePeerAddress::ipv6(ipv6);

        assert_eq!(peer_addr.address_type, AddressType::Ipv6);
        assert_eq!(peer_addr.address, ipv6.octets().to_vec());
        assert_eq!(peer_addr.as_ipv6(), Some(ipv6));
        assert_eq!(peer_addr.as_ipv4(), None);
        assert_eq!(peer_addr.as_fqdn(), None);
    }

    #[test]
    fn test_remote_peer_address_fqdn() {
        let fqdn = "peer.example.com".to_string();
        let peer_addr = RemotePeerAddress::fqdn(fqdn.clone());

        assert_eq!(peer_addr.address_type, AddressType::Fqdn);
        assert_eq!(peer_addr.address, fqdn.as_bytes());
        assert_eq!(peer_addr.as_fqdn(), Some(fqdn));
        assert_eq!(peer_addr.as_ipv4(), None);
        assert_eq!(peer_addr.as_ipv6(), None);
    }

    #[test]
    fn test_path_failure_report_marshal_unmarshal_empty() {
        let report = PathFailureReport::empty();
        let marshaled = report.marshal();
        let unmarshaled = PathFailureReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert!(report.is_empty());
        assert_eq!(report.peer_count(), 0);
        assert_eq!(marshaled, vec![0]); // Zero peer addresses
        assert_eq!(report.len(), 1);
    }

    #[test]
    fn test_path_failure_report_marshal_unmarshal_single_ipv4() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let report = PathFailureReport::empty().add_ipv4_peer(ipv4);
        let marshaled = report.marshal();
        let unmarshaled = PathFailureReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert!(!report.is_empty());
        assert_eq!(report.peer_count(), 1);
        assert_eq!(unmarshaled.remote_peer_addresses[0].as_ipv4(), Some(ipv4));

        // Check marshaling format: count + type + len + address
        let expected = vec![
            1, // 1 peer address
            0, // IPv4 type
            4, // 4 bytes length
            10, 0, 0, 1, // IPv4 address
        ];
        assert_eq!(marshaled, expected);
    }

    #[test]
    fn test_path_failure_report_marshal_unmarshal_multiple_peers() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0, 0, 0x8a2e, 0x370, 0x7334);
        let fqdn = "backup.server.com".to_string();

        let report = PathFailureReport::empty()
            .add_ipv4_peer(ipv4)
            .add_ipv6_peer(ipv6)
            .add_fqdn_peer(fqdn.clone());

        let marshaled = report.marshal();
        let unmarshaled = PathFailureReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(report.peer_count(), 3);
        assert_eq!(unmarshaled.remote_peer_addresses[0].as_ipv4(), Some(ipv4));
        assert_eq!(unmarshaled.remote_peer_addresses[1].as_ipv6(), Some(ipv6));
        assert_eq!(unmarshaled.remote_peer_addresses[2].as_fqdn(), Some(fqdn));
    }

    #[test]
    fn test_path_failure_report_builder_pattern() {
        let report = PathFailureReport::new(vec![])
            .add_ipv4_peer(Ipv4Addr::new(10, 0, 0, 1))
            .add_fqdn_peer("primary.server.com".to_string())
            .add_ipv6_peer(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));

        assert_eq!(report.peer_count(), 3);
        assert!(!report.is_empty());
    }

    #[test]
    fn test_path_failure_report_to_ie() {
        let report = PathFailureReport::empty().add_ipv4_peer(Ipv4Addr::new(172, 16, 1, 1));
        let ie = report.to_ie();

        assert_eq!(ie.ie_type, IeType::UserPlanePathFailureReport);

        let unmarshaled = PathFailureReport::unmarshal(&ie.payload).unwrap();
        assert_eq!(report, unmarshaled);
    }

    #[test]
    fn test_remote_peer_address_marshal_unmarshal_ipv4() {
        let peer_addr = RemotePeerAddress::ipv4(Ipv4Addr::new(203, 0, 113, 1));
        let marshaled = peer_addr.marshal();

        let mut offset = 0;
        let unmarshaled = RemotePeerAddress::unmarshal(&marshaled, &mut offset).unwrap();

        assert_eq!(peer_addr, unmarshaled);
        assert_eq!(offset, marshaled.len());
    }

    #[test]
    fn test_remote_peer_address_marshal_unmarshal_fqdn() {
        let fqdn = "test.example.org".to_string();
        let peer_addr = RemotePeerAddress::fqdn(fqdn.clone());
        let marshaled = peer_addr.marshal();

        let mut offset = 0;
        let unmarshaled = RemotePeerAddress::unmarshal(&marshaled, &mut offset).unwrap();

        assert_eq!(peer_addr, unmarshaled);
        assert_eq!(unmarshaled.as_fqdn(), Some(fqdn));
        assert_eq!(offset, marshaled.len());
    }

    #[test]
    fn test_path_failure_report_unmarshal_empty_payload() {
        let result = PathFailureReport::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_remote_peer_address_unmarshal_short_header() {
        let mut offset = 0;
        let result = RemotePeerAddress::unmarshal(&[0], &mut offset);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_remote_peer_address_unmarshal_short_data() {
        let mut offset = 0;
        // Type=0, Length=4, but only 2 bytes of data
        let result = RemotePeerAddress::unmarshal(&[0, 4, 0x01, 0x02], &mut offset);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_address_type_conversions() {
        // Test u8 to AddressType conversion
        assert_eq!(AddressType::from(0), AddressType::Ipv4);
        assert_eq!(AddressType::from(1), AddressType::Ipv6);
        assert_eq!(AddressType::from(2), AddressType::Fqdn);
        assert_eq!(AddressType::from(99), AddressType::Unknown(99));

        // Test AddressType to u8 conversion
        assert_eq!(u8::from(AddressType::Ipv4), 0);
        assert_eq!(u8::from(AddressType::Ipv6), 1);
        assert_eq!(u8::from(AddressType::Fqdn), 2);
        assert_eq!(u8::from(AddressType::Unknown(99)), 99);
    }

    #[test]
    fn test_path_failure_report_len() {
        let empty_report = PathFailureReport::empty();
        assert_eq!(empty_report.len(), 1); // Just the count byte

        let ipv4_report = PathFailureReport::empty().add_ipv4_peer(Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(ipv4_report.len(), 1 + 1 + 1 + 4); // count + type + len + 4 bytes

        let fqdn_report = PathFailureReport::empty().add_fqdn_peer("test.com".to_string());
        assert_eq!(fqdn_report.len(), 1 + 1 + 1 + 8); // count + type + len + 8 bytes
    }

    #[test]
    fn test_path_failure_report_round_trip_complex() {
        let report = PathFailureReport::new(vec![
            RemotePeerAddress::ipv4(Ipv4Addr::new(10, 1, 2, 3)),
            RemotePeerAddress::ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 42)),
            RemotePeerAddress::fqdn("failed.peer.example.com".to_string()),
            RemotePeerAddress::new(AddressType::Unknown(99), vec![0xAA, 0xBB, 0xCC]),
        ]);

        let marshaled = report.marshal();
        let unmarshaled = PathFailureReport::unmarshal(&marshaled).unwrap();

        assert_eq!(report, unmarshaled);
        assert_eq!(unmarshaled.peer_count(), 4);
    }
}
