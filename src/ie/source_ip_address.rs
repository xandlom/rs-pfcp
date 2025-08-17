//! Source IP Address IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a Source IP Address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceIpAddress {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl SourceIpAddress {
    /// Creates a new Source IP Address.
    pub fn new(ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Self {
        SourceIpAddress { ipv4, ipv6 }
    }

    /// Creates a new Source IP Address from IPv4 only.
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        SourceIpAddress {
            ipv4: Some(addr),
            ipv6: None,
        }
    }

    /// Creates a new Source IP Address from IPv6 only.
    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        SourceIpAddress {
            ipv4: None,
            ipv6: Some(addr),
        }
    }

    /// Creates a new Source IP Address from both IPv4 and IPv6.
    pub fn new_dual(ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        SourceIpAddress {
            ipv4: Some(ipv4),
            ipv6: Some(ipv6),
        }
    }

    /// Marshals the Source IP Address into a byte vector, which is the payload of the IE.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        if let Some(ipv4) = self.ipv4 {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = self.ipv6 {
            data.extend_from_slice(&ipv6.octets());
        }
        data
    }

    /// Unmarshals a byte slice into a Source IP Address.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut ipv4 = None;
        let mut ipv6 = None;
        let mut offset = 0;

        // Try to parse IPv4 (4 bytes)
        if payload.len() >= offset + 4 {
            let mut octets = [0; 4];
            octets.copy_from_slice(&payload[offset..offset + 4]);
            ipv4 = Some(Ipv4Addr::from(octets));
            offset += 4;
        }

        // Try to parse IPv6 (16 bytes)
        if payload.len() >= offset + 16 {
            let mut octets = [0; 16];
            octets.copy_from_slice(&payload[offset..offset + 16]);
            ipv6 = Some(Ipv6Addr::from(octets));
        }

        Ok(SourceIpAddress { ipv4, ipv6 })
    }

    /// Wraps the Source IP Address in a SourceIPAddress IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SourceIPAddress, self.marshal())
    }
}
