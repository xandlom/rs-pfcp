//! Outer Header Creation IE.
//!
//! This IE specifies the creation of GTP-U, UDP/IPv4, or UDP/IPv6 outer headers
//! for forwarding packets. Used in ForwardingParameters to configure tunnel endpoints.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Outer Header Creation Description flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OuterHeaderCreationFlags {
    pub gtpu_udp_ipv4: bool,
    pub gtpu_udp_ipv6: bool,
    pub udp_ipv4: bool,
    pub udp_ipv6: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub ctag: bool, // C-TAG
    pub stag: bool, // S-TAG
}

impl OuterHeaderCreationFlags {
    pub fn new() -> Self {
        OuterHeaderCreationFlags {
            gtpu_udp_ipv4: false,
            gtpu_udp_ipv6: false,
            udp_ipv4: false,
            udp_ipv6: false,
            ipv4: false,
            ipv6: false,
            ctag: false,
            stag: false,
        }
    }

    /// GTP-U/UDP/IPv4 tunnel
    pub fn gtpu_ipv4() -> Self {
        OuterHeaderCreationFlags {
            gtpu_udp_ipv4: true,
            ..Self::new()
        }
    }

    /// GTP-U/UDP/IPv6 tunnel
    pub fn gtpu_ipv6() -> Self {
        OuterHeaderCreationFlags {
            gtpu_udp_ipv6: true,
            ..Self::new()
        }
    }

    /// UDP/IPv4 encapsulation (no GTP-U)
    pub fn udp_ipv4() -> Self {
        OuterHeaderCreationFlags {
            udp_ipv4: true,
            ..Self::new()
        }
    }

    /// UDP/IPv6 encapsulation (no GTP-U)
    pub fn udp_ipv6() -> Self {
        OuterHeaderCreationFlags {
            udp_ipv6: true,
            ..Self::new()
        }
    }

    fn to_u16(self) -> u16 {
        let mut flags = 0u16;
        if self.gtpu_udp_ipv4 {
            flags |= 0x0100; // Bit 8
        }
        if self.gtpu_udp_ipv6 {
            flags |= 0x0200; // Bit 9
        }
        if self.udp_ipv4 {
            flags |= 0x0400; // Bit 10
        }
        if self.udp_ipv6 {
            flags |= 0x0800; // Bit 11
        }
        if self.ipv4 {
            flags |= 0x1000; // Bit 12
        }
        if self.ipv6 {
            flags |= 0x2000; // Bit 13
        }
        if self.ctag {
            flags |= 0x4000; // Bit 14
        }
        if self.stag {
            flags |= 0x8000; // Bit 15
        }
        flags
    }

    fn from_u16(value: u16) -> Self {
        OuterHeaderCreationFlags {
            gtpu_udp_ipv4: (value & 0x0100) != 0,
            gtpu_udp_ipv6: (value & 0x0200) != 0,
            udp_ipv4: (value & 0x0400) != 0,
            udp_ipv6: (value & 0x0800) != 0,
            ipv4: (value & 0x1000) != 0,
            ipv6: (value & 0x2000) != 0,
            ctag: (value & 0x4000) != 0,
            stag: (value & 0x8000) != 0,
        }
    }
}

impl Default for OuterHeaderCreationFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Outer Header Creation IE
///
/// Specifies how to create outer headers for packet forwarding.
/// Commonly used for GTP-U tunnel setup in 5G networks.
///
/// # 3GPP TS 29.244 Reference
/// - Section 8.2.56: Outer Header Creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OuterHeaderCreation {
    pub description: OuterHeaderCreationFlags,
    pub teid: Option<u32>, // GTP-U TEID
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub port_number: Option<u16>, // UDP port
    pub ctag: Option<u32>,        // C-TAG (3 bytes)
    pub stag: Option<u32>,        // S-TAG (3 bytes)
}

impl OuterHeaderCreation {
    /// Creates a new Outer Header Creation IE
    pub fn new(description: OuterHeaderCreationFlags) -> Self {
        OuterHeaderCreation {
            description,
            teid: None,
            ipv4_address: None,
            ipv6_address: None,
            port_number: None,
            ctag: None,
            stag: None,
        }
    }

    /// Creates GTP-U/UDP/IPv4 tunnel configuration
    pub fn gtpu_ipv4(teid: u32, ipv4: Ipv4Addr) -> Self {
        OuterHeaderCreation {
            description: OuterHeaderCreationFlags::gtpu_ipv4(),
            teid: Some(teid),
            ipv4_address: Some(ipv4),
            ipv6_address: None,
            port_number: None,
            ctag: None,
            stag: None,
        }
    }

    /// Creates GTP-U/UDP/IPv6 tunnel configuration
    pub fn gtpu_ipv6(teid: u32, ipv6: Ipv6Addr) -> Self {
        OuterHeaderCreation {
            description: OuterHeaderCreationFlags::gtpu_ipv6(),
            teid: Some(teid),
            ipv4_address: None,
            ipv6_address: Some(ipv6),
            port_number: None,
            ctag: None,
            stag: None,
        }
    }

    /// Creates UDP/IPv4 encapsulation
    pub fn udp_ipv4(ipv4: Ipv4Addr, port: u16) -> Self {
        OuterHeaderCreation {
            description: OuterHeaderCreationFlags::udp_ipv4(),
            teid: None,
            ipv4_address: Some(ipv4),
            ipv6_address: None,
            port_number: Some(port),
            ctag: None,
            stag: None,
        }
    }

    /// Adds C-TAG (Customer VLAN tag)
    pub fn with_ctag(mut self, ctag: u32) -> Self {
        self.description.ctag = true;
        self.ctag = Some(ctag & 0xFFFFFF); // 3 bytes
        self
    }

    /// Adds S-TAG (Service VLAN tag)
    pub fn with_stag(mut self, stag: u32) -> Self {
        self.description.stag = true;
        self.stag = Some(stag & 0xFFFFFF); // 3 bytes
        self
    }

    /// Marshals the Outer Header Creation IE into bytes
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Description (2 bytes, big-endian)
        let desc_value = self.description.to_u16();
        data.extend_from_slice(&desc_value.to_be_bytes());

        // TEID (4 bytes) - included if any GTP-U flag is set
        if self.description.gtpu_udp_ipv4 || self.description.gtpu_udp_ipv6 {
            let teid = self.teid.unwrap_or(0);
            data.extend_from_slice(&teid.to_be_bytes());
        }

        // IPv4 Address (4 bytes)
        if self.description.gtpu_udp_ipv4 || self.description.udp_ipv4 || self.description.ipv4 {
            if let Some(ipv4) = self.ipv4_address {
                data.extend_from_slice(&ipv4.octets());
            }
        }

        // IPv6 Address (16 bytes)
        if self.description.gtpu_udp_ipv6 || self.description.udp_ipv6 || self.description.ipv6 {
            if let Some(ipv6) = self.ipv6_address {
                data.extend_from_slice(&ipv6.octets());
            }
        }

        // Port Number (2 bytes) - for UDP encapsulation
        if self.description.udp_ipv4 || self.description.udp_ipv6 {
            let port = self.port_number.unwrap_or(0);
            data.extend_from_slice(&port.to_be_bytes());
        }

        // C-TAG (3 bytes)
        if self.description.ctag {
            if let Some(ctag) = self.ctag {
                data.push(((ctag >> 16) & 0xFF) as u8);
                data.push(((ctag >> 8) & 0xFF) as u8);
                data.push((ctag & 0xFF) as u8);
            }
        }

        // S-TAG (3 bytes)
        if self.description.stag {
            if let Some(stag) = self.stag {
                data.push(((stag >> 16) & 0xFF) as u8);
                data.push(((stag >> 8) & 0xFF) as u8);
                data.push((stag & 0xFF) as u8);
            }
        }

        data
    }

    /// Unmarshals bytes into an Outer Header Creation IE
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Outer Header Creation payload too short",
            ));
        }

        let mut offset = 0;

        // Description (2 bytes)
        let desc_value = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
        let description = OuterHeaderCreationFlags::from_u16(desc_value);
        offset += 2;

        // TEID (4 bytes) - if GTP-U
        let teid = if description.gtpu_udp_ipv4 || description.gtpu_udp_ipv6 {
            if offset + 4 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing TEID in Outer Header Creation",
                ));
            }
            let teid_val = u32::from_be_bytes([
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            ]);
            offset += 4;
            Some(teid_val)
        } else {
            None
        };

        // IPv4 Address (4 bytes)
        let ipv4_address = if description.gtpu_udp_ipv4 || description.udp_ipv4 || description.ipv4
        {
            if offset + 4 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing IPv4 address in Outer Header Creation",
                ));
            }
            let ipv4 = Ipv4Addr::new(
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            );
            offset += 4;
            Some(ipv4)
        } else {
            None
        };

        // IPv6 Address (16 bytes)
        let ipv6_address = if description.gtpu_udp_ipv6 || description.udp_ipv6 || description.ipv6
        {
            if offset + 16 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing IPv6 address in Outer Header Creation",
                ));
            }
            let mut ipv6_bytes = [0u8; 16];
            ipv6_bytes.copy_from_slice(&payload[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(ipv6_bytes))
        } else {
            None
        };

        // Port Number (2 bytes) - for UDP
        let port_number = if description.udp_ipv4 || description.udp_ipv6 {
            if offset + 2 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing port number in Outer Header Creation",
                ));
            }
            let port = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
            offset += 2;
            Some(port)
        } else {
            None
        };

        // C-TAG (3 bytes)
        let ctag = if description.ctag {
            if offset + 3 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing C-TAG in Outer Header Creation",
                ));
            }
            let ctag_val = ((payload[offset] as u32) << 16)
                | ((payload[offset + 1] as u32) << 8)
                | (payload[offset + 2] as u32);
            offset += 3;
            Some(ctag_val)
        } else {
            None
        };

        // S-TAG (3 bytes)
        let stag = if description.stag {
            if offset + 3 > payload.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing S-TAG in Outer Header Creation",
                ));
            }
            let stag_val = ((payload[offset] as u32) << 16)
                | ((payload[offset + 1] as u32) << 8)
                | (payload[offset + 2] as u32);
            Some(stag_val)
        } else {
            None
        };

        Ok(OuterHeaderCreation {
            description,
            teid,
            ipv4_address,
            ipv6_address,
            port_number,
            ctag,
            stag,
        })
    }

    /// Wraps the Outer Header Creation in an IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::OuterHeaderCreation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer_header_creation_gtpu_ipv4() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0x12345678, "192.168.1.1".parse().unwrap());

        assert!(ohc.description.gtpu_udp_ipv4);
        assert_eq!(ohc.teid, Some(0x12345678));
        assert_eq!(ohc.ipv4_address, Some("192.168.1.1".parse().unwrap()));

        let marshaled = ohc.marshal();
        let unmarshaled = OuterHeaderCreation::unmarshal(&marshaled).unwrap();

        assert_eq!(ohc, unmarshaled);
    }

    #[test]
    fn test_outer_header_creation_gtpu_ipv6() {
        let ohc = OuterHeaderCreation::gtpu_ipv6(0xABCDEF01, "2001:db8::1".parse().unwrap());

        assert!(ohc.description.gtpu_udp_ipv6);
        assert_eq!(ohc.teid, Some(0xABCDEF01));
        assert_eq!(ohc.ipv6_address, Some("2001:db8::1".parse().unwrap()));

        let marshaled = ohc.marshal();
        let unmarshaled = OuterHeaderCreation::unmarshal(&marshaled).unwrap();

        assert_eq!(ohc, unmarshaled);
    }

    #[test]
    fn test_outer_header_creation_udp_ipv4() {
        let ohc = OuterHeaderCreation::udp_ipv4("10.0.0.1".parse().unwrap(), 2152);

        assert!(ohc.description.udp_ipv4);
        assert_eq!(ohc.ipv4_address, Some("10.0.0.1".parse().unwrap()));
        assert_eq!(ohc.port_number, Some(2152));
        assert_eq!(ohc.teid, None);

        let marshaled = ohc.marshal();
        let unmarshaled = OuterHeaderCreation::unmarshal(&marshaled).unwrap();

        assert_eq!(ohc, unmarshaled);
    }

    #[test]
    fn test_outer_header_creation_with_ctag() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0x1000, "192.168.1.1".parse().unwrap())
            .with_ctag(0x123456);

        assert!(ohc.description.ctag);
        assert_eq!(ohc.ctag, Some(0x123456));

        let marshaled = ohc.marshal();
        let unmarshaled = OuterHeaderCreation::unmarshal(&marshaled).unwrap();

        assert_eq!(ohc, unmarshaled);
    }

    #[test]
    fn test_outer_header_creation_to_ie() {
        let ohc = OuterHeaderCreation::gtpu_ipv4(0x12345678, "192.168.1.1".parse().unwrap());
        let ie = ohc.to_ie();

        assert_eq!(ie.ie_type, IeType::OuterHeaderCreation);

        let unmarshaled = OuterHeaderCreation::unmarshal(&ie.payload).unwrap();
        assert_eq!(ohc, unmarshaled);
    }

    #[test]
    fn test_flags_round_trip() {
        let flags = OuterHeaderCreationFlags {
            gtpu_udp_ipv4: true,
            udp_ipv6: true,
            ctag: true,
            ..OuterHeaderCreationFlags::new()
        };

        let value = flags.to_u16();
        let recovered = OuterHeaderCreationFlags::from_u16(value);

        assert_eq!(flags, recovered);
    }
}
