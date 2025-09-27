//! UE IP Address IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a UE IP Address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeIpAddress {
    pub v4: bool,
    pub v6: bool,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
}

impl UeIpAddress {
    /// Creates a new UE IP Address.
    pub fn new(ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) -> Self {
        UeIpAddress {
            v4: ipv4_address.is_some(),
            v6: ipv6_address.is_some(),
            ipv4_address,
            ipv6_address,
        }
    }

    /// Marshals the UE IP Address into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.v6 {
            flags |= 1;  // Bit 0: V6 (IPv6)
        }
        if self.v4 {
            flags |= 2;  // Bit 1: V4 (IPv4)
        }
        data.push(flags);
        if let Some(addr) = self.ipv4_address {
            data.extend_from_slice(&addr.octets());
        }
        if let Some(addr) = self.ipv6_address {
            data.extend_from_slice(&addr.octets());
        }
        data
    }

    /// Unmarshals a byte slice into a UE IP Address.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "UE IP Address payload too short",
            ));
        }
        let flags = payload[0];
        let v6 = flags & 1 != 0;  // Bit 0: V6 (IPv6)
        let v4 = flags & 2 != 0;  // Bit 1: V4 (IPv4)
        let mut offset = 1;
        let ipv4_address = if v4 {
            if payload.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "UE IP Address payload too short for IPv4",
                ));
            }
            let addr = Ipv4Addr::new(
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            );
            offset += 4;
            Some(addr)
        } else {
            None
        };
        let ipv6_address = if v6 {
            if payload.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "UE IP Address payload too short for IPv6",
                ));
            }
            let mut octets = [0; 16];
            octets.copy_from_slice(&payload[offset..offset + 16]);
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };
        Ok(UeIpAddress {
            v4,
            v6,
            ipv4_address,
            ipv6_address,
        })
    }

    /// Wraps the UE IP Address in a UeIpAddress IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UeIpAddress, self.marshal())
    }
}
