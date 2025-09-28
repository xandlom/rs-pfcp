//! CP IP Address Information Element.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a Control Plane IP Address.
///
/// The CP IP Address IE indicates the IP address of the PFCP entity.
/// It can contain an IPv4 address, IPv6 address, or both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpIpAddress {
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
}

impl CpIpAddress {
    /// Creates a new CP IP Address with IPv4 only.
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        CpIpAddress {
            ipv4_address: Some(addr),
            ipv6_address: None,
        }
    }

    /// Creates a new CP IP Address with IPv6 only.
    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        CpIpAddress {
            ipv4_address: None,
            ipv6_address: Some(addr),
        }
    }

    /// Creates a new CP IP Address with both IPv4 and IPv6.
    pub fn new_dual_stack(ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        CpIpAddress {
            ipv4_address: Some(ipv4),
            ipv6_address: Some(ipv6),
        }
    }

    /// Returns the IPv4 address if present.
    pub fn ipv4_address(&self) -> Option<Ipv4Addr> {
        self.ipv4_address
    }

    /// Returns the IPv6 address if present.
    pub fn ipv6_address(&self) -> Option<Ipv6Addr> {
        self.ipv6_address
    }

    /// Returns true if this contains an IPv4 address.
    pub fn has_ipv4(&self) -> bool {
        self.ipv4_address.is_some()
    }

    /// Returns true if this contains an IPv6 address.
    pub fn has_ipv6(&self) -> bool {
        self.ipv6_address.is_some()
    }

    /// Marshals the CP IP Address into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Build flags byte
        let mut flags = 0u8;
        if self.ipv6_address.is_some() {
            flags |= 0x01; // V6 flag (bit 1)
        }
        if self.ipv4_address.is_some() {
            flags |= 0x02; // V4 flag (bit 2)
        }
        // Bits 3-8 are spare and remain 0

        data.push(flags);

        // Add IPv4 address if present
        if let Some(ipv4) = &self.ipv4_address {
            data.extend_from_slice(&ipv4.octets());
        }

        // Add IPv6 address if present
        if let Some(ipv6) = &self.ipv6_address {
            data.extend_from_slice(&ipv6.octets());
        }

        data
    }

    /// Unmarshals a CP IP Address from a byte slice.
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "CP IP Address data is empty",
            ));
        }

        let flags = data[0];
        let v6 = (flags & 0x01) != 0; // bit 1
        let v4 = (flags & 0x02) != 0; // bit 2

        // Check spare bits are zero
        if (flags & 0xFC) != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Spare bits in flags must be zero",
            ));
        }

        // At least one address must be present
        if !v4 && !v6 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "At least one IP address (IPv4 or IPv6) must be present",
            ));
        }

        let mut offset = 1;
        let mut ipv4_address = None;
        let mut ipv6_address = None;

        // Parse IPv4 address if present
        if v4 {
            if data.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for IPv4 address",
                ));
            }
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[offset..offset + 4]);
            ipv4_address = Some(Ipv4Addr::from(octets));
            offset += 4;
        }

        // Parse IPv6 address if present
        if v6 {
            if data.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for IPv6 address",
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            ipv6_address = Some(Ipv6Addr::from(octets));
        }

        Ok(CpIpAddress {
            ipv4_address,
            ipv6_address,
        })
    }

    /// Converts to an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CpIpAddress, self.marshal())
    }
}

impl From<Ipv4Addr> for CpIpAddress {
    fn from(addr: Ipv4Addr) -> Self {
        CpIpAddress::new_ipv4(addr)
    }
}

impl From<Ipv6Addr> for CpIpAddress {
    fn from(addr: Ipv6Addr) -> Self {
        CpIpAddress::new_ipv6(addr)
    }
}

impl From<(Ipv4Addr, Ipv6Addr)> for CpIpAddress {
    fn from((ipv4, ipv6): (Ipv4Addr, Ipv6Addr)) -> Self {
        CpIpAddress::new_dual_stack(ipv4, ipv6)
    }
}

impl std::fmt::Display for CpIpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.ipv4_address, &self.ipv6_address) {
            (Some(ipv4), Some(ipv6)) => write!(f, "{}/{}", ipv4, ipv6),
            (Some(ipv4), None) => write!(f, "{}", ipv4),
            (None, Some(ipv6)) => write!(f, "{}", ipv6),
            (None, None) => write!(f, "no addresses"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp_ip_address_ipv4_only() {
        let addr = Ipv4Addr::new(192, 168, 1, 100);
        let cp_ip = CpIpAddress::new_ipv4(addr);

        assert_eq!(cp_ip.ipv4_address(), Some(addr));
        assert_eq!(cp_ip.ipv6_address(), None);
        assert!(cp_ip.has_ipv4());
        assert!(!cp_ip.has_ipv6());

        let marshaled = cp_ip.marshal();
        let expected = vec![0x02, 192, 168, 1, 100]; // V4 flag only
        assert_eq!(marshaled, expected);

        let unmarshaled = CpIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, cp_ip);
    }

    #[test]
    fn test_cp_ip_address_ipv6_only() {
        let addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
        let cp_ip = CpIpAddress::new_ipv6(addr);

        assert_eq!(cp_ip.ipv4_address(), None);
        assert_eq!(cp_ip.ipv6_address(), Some(addr));
        assert!(!cp_ip.has_ipv4());
        assert!(cp_ip.has_ipv6());

        let marshaled = cp_ip.marshal();
        assert_eq!(marshaled[0], 0x01); // V6 flag only
        assert_eq!(marshaled.len(), 17); // 1 byte flag + 16 bytes IPv6

        let unmarshaled = CpIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, cp_ip);
    }

    #[test]
    fn test_cp_ip_address_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let cp_ip = CpIpAddress::new_dual_stack(ipv4, ipv6);

        assert_eq!(cp_ip.ipv4_address(), Some(ipv4));
        assert_eq!(cp_ip.ipv6_address(), Some(ipv6));
        assert!(cp_ip.has_ipv4());
        assert!(cp_ip.has_ipv6());

        let marshaled = cp_ip.marshal();
        assert_eq!(marshaled[0], 0x03); // Both V4 and V6 flags (0x01 | 0x02)
        assert_eq!(marshaled.len(), 21); // 1 byte flag + 4 bytes IPv4 + 16 bytes IPv6

        let unmarshaled = CpIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, cp_ip);
    }

    #[test]
    fn test_cp_ip_address_to_ie() {
        let addr = Ipv4Addr::new(203, 0, 113, 1);
        let cp_ip = CpIpAddress::new_ipv4(addr);
        let ie = cp_ip.to_ie();

        assert_eq!(ie.ie_type, IeType::CpIpAddress);
        assert_eq!(ie.payload, cp_ip.marshal());
    }

    #[test]
    fn test_cp_ip_address_unmarshal_errors() {
        // Empty data
        let result = CpIpAddress::unmarshal(&[]);
        assert!(result.is_err());

        // No address flags set
        let result = CpIpAddress::unmarshal(&[0x00]);
        assert!(result.is_err());

        // Spare bits set
        let result = CpIpAddress::unmarshal(&[0x04]); // Bit 3 set
        assert!(result.is_err());

        // IPv4 flag but insufficient data
        let result = CpIpAddress::unmarshal(&[0x02, 192, 168]);
        assert!(result.is_err());

        // IPv6 flag but insufficient data
        let result = CpIpAddress::unmarshal(&[0x01, 0x20, 0x01]);
        assert!(result.is_err());

        // Dual stack but insufficient data for IPv6
        let result = CpIpAddress::unmarshal(&[0x03, 192, 168, 1, 1, 0x20, 0x01]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cp_ip_address_from_conversions() {
        let ipv4_addr = Ipv4Addr::new(127, 0, 0, 1);
        let cp_ip: CpIpAddress = ipv4_addr.into();
        assert_eq!(cp_ip.ipv4_address(), Some(ipv4_addr));
        assert_eq!(cp_ip.ipv6_address(), None);

        let ipv6_addr = Ipv6Addr::LOCALHOST;
        let cp_ip: CpIpAddress = ipv6_addr.into();
        assert_eq!(cp_ip.ipv4_address(), None);
        assert_eq!(cp_ip.ipv6_address(), Some(ipv6_addr));

        let dual_tuple = (ipv4_addr, ipv6_addr);
        let cp_ip: CpIpAddress = dual_tuple.into();
        assert_eq!(cp_ip.ipv4_address(), Some(ipv4_addr));
        assert_eq!(cp_ip.ipv6_address(), Some(ipv6_addr));
    }

    #[test]
    fn test_cp_ip_address_display() {
        let ipv4_addr = Ipv4Addr::new(192, 168, 1, 1);
        let cp_ip = CpIpAddress::new_ipv4(ipv4_addr);
        assert_eq!(format!("{}", cp_ip), "192.168.1.1");

        let ipv6_addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
        let cp_ip = CpIpAddress::new_ipv6(ipv6_addr);
        assert_eq!(format!("{}", cp_ip), "2001:db8::1");

        let cp_ip = CpIpAddress::new_dual_stack(ipv4_addr, ipv6_addr);
        assert_eq!(format!("{}", cp_ip), "192.168.1.1/2001:db8::1");
    }

    #[test]
    fn test_cp_ip_address_round_trip() {
        let test_cases = vec![
            CpIpAddress::new_ipv4(Ipv4Addr::new(127, 0, 0, 1)),
            CpIpAddress::new_ipv6(Ipv6Addr::LOCALHOST),
            CpIpAddress::new_dual_stack(
                Ipv4Addr::new(10, 0, 0, 1),
                Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
            ),
            CpIpAddress::new_ipv4(Ipv4Addr::new(203, 0, 113, 5)),
            CpIpAddress::new_ipv6(Ipv6Addr::new(
                0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334,
            )),
        ];

        for cp_ip in test_cases {
            let marshaled = cp_ip.marshal();
            let unmarshaled = CpIpAddress::unmarshal(&marshaled).unwrap();
            assert_eq!(cp_ip, unmarshaled);
        }
    }

    #[test]
    fn test_cp_ip_address_spec_compliance() {
        // Test all flag combinations
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let ipv6 = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);

        // IPv4 only
        let cp_ip_v4 = CpIpAddress::new_ipv4(ipv4);
        let marshaled = cp_ip_v4.marshal();
        assert_eq!(marshaled[0], 0x02); // V4 flag only

        // IPv6 only
        let cp_ip_v6 = CpIpAddress::new_ipv6(ipv6);
        let marshaled = cp_ip_v6.marshal();
        assert_eq!(marshaled[0], 0x01); // V6 flag only

        // Dual stack
        let cp_ip_dual = CpIpAddress::new_dual_stack(ipv4, ipv6);
        let marshaled = cp_ip_dual.marshal();
        assert_eq!(marshaled[0], 0x03); // V4 + V6 flags
    }
}
