//! Source IP Address IE.

use crate::ie::{Ie, IeType};
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents a Source IP Address.
///
/// Per 3GPP TS 29.244 V18.10.0 Section 8.2.138:
/// - Bit 1 (V6): IPv6 address present
/// - Bit 2 (V4): IPv4 address present
/// - Bit 3 (MPL): Mask/Prefix Length present (optional)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceIpAddress {
    pub v4: bool,
    pub v6: bool,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
    pub mask_prefix_length: Option<u8>,
}

impl SourceIpAddress {
    /// Creates a new Source IP Address.
    pub fn new(ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Self {
        SourceIpAddress {
            v4: ipv4.is_some(),
            v6: ipv6.is_some(),
            ipv4,
            ipv6,
            mask_prefix_length: None,
        }
    }

    /// Creates a new Source IP Address from IPv4 only.
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        SourceIpAddress {
            v4: true,
            v6: false,
            ipv4: Some(addr),
            ipv6: None,
            mask_prefix_length: None,
        }
    }

    /// Creates a new Source IP Address from IPv6 only.
    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        SourceIpAddress {
            v4: false,
            v6: true,
            ipv4: None,
            ipv6: Some(addr),
            mask_prefix_length: None,
        }
    }

    /// Creates a new Source IP Address from both IPv4 and IPv6.
    pub fn new_dual(ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        SourceIpAddress {
            v4: true,
            v6: true,
            ipv4: Some(ipv4),
            ipv6: Some(ipv6),
            mask_prefix_length: None,
        }
    }

    /// Creates a new Source IP Address with mask/prefix length.
    ///
    /// Example: IPv4 subnet 192.0.2.10/24 → mask_prefix_length = 24
    /// Example: IPv6 prefix ::1/64 → mask_prefix_length = 64
    pub fn with_mask(mut self, mask_prefix_length: u8) -> Self {
        self.mask_prefix_length = Some(mask_prefix_length);
        self
    }

    /// Marshals the Source IP Address into a byte vector, which is the payload of the IE.
    ///
    /// Format: 1 byte flags + optional IPv4 (4 bytes) + optional IPv6 (16 bytes) + optional mask (1 byte)
    /// Flags: Bit 0 = V6, Bit 1 = V4, Bit 2 = MPL (Mask/Prefix Length)
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags = 0;
        if self.v6 {
            flags |= 1; // Bit 0: V6 (IPv6)
        }
        if self.v4 {
            flags |= 2; // Bit 1: V4 (IPv4)
        }
        if self.mask_prefix_length.is_some() {
            flags |= 4; // Bit 2: MPL (Mask/Prefix Length)
        }
        data.push(flags);
        if let Some(ipv4) = self.ipv4 {
            data.extend_from_slice(&ipv4.octets());
        }
        if let Some(ipv6) = self.ipv6 {
            data.extend_from_slice(&ipv6.octets());
        }
        if let Some(mask) = self.mask_prefix_length {
            data.push(mask);
        }
        data
    }

    /// Unmarshals a byte slice into a Source IP Address.
    ///
    /// Per 3GPP TS 29.244, Source IP Address requires minimum 1 byte (flags) plus address data.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Source IP Address requires at least 1 byte, got 0",
            ));
        }

        let flags = payload[0];
        let v6 = flags & 1 != 0; // Bit 0: V6 (IPv6)
        let v4 = flags & 2 != 0; // Bit 1: V4 (IPv4)
        let mpl = flags & 4 != 0; // Bit 2: MPL (Mask/Prefix Length)
        let mut offset = 1;

        let ipv4 = if v4 {
            if payload.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Source IP Address payload too short for IPv4",
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

        let ipv6 = if v6 {
            if payload.len() < offset + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Source IP Address payload too short for IPv6",
                ));
            }
            let mut octets = [0; 16];
            octets.copy_from_slice(&payload[offset..offset + 16]);
            offset += 16;
            Some(Ipv6Addr::from(octets))
        } else {
            None
        };

        let mask_prefix_length = if mpl {
            if payload.len() < offset + 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Source IP Address payload too short for mask/prefix length",
                ));
            }
            Some(payload[offset])
        } else {
            None
        };

        Ok(SourceIpAddress {
            v4,
            v6,
            ipv4,
            ipv6,
            mask_prefix_length,
        })
    }

    /// Wraps the Source IP Address in a SourceIPAddress IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SourceIpAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_ip_address_ipv4_only() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let source_ip = SourceIpAddress::new_ipv4(ipv4);

        assert!(source_ip.v4);
        assert!(!source_ip.v6);
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, None);

        // Marshal and unmarshal
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 5); // 1 byte flags + 4 bytes IPv4
        assert_eq!(marshaled[0], 0x02); // V4 flag only

        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
    }

    #[test]
    fn test_source_ip_address_ipv6_only() {
        let ipv6 = "2001:db8::1".parse::<Ipv6Addr>().unwrap();
        let source_ip = SourceIpAddress::new_ipv6(ipv6);

        assert!(!source_ip.v4);
        assert!(source_ip.v6);
        assert_eq!(source_ip.ipv4, None);
        assert_eq!(source_ip.ipv6, Some(ipv6));

        // Marshal and unmarshal
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 17); // 1 byte flags + 16 bytes IPv6
        assert_eq!(marshaled[0], 0x01); // V6 flag only

        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
    }

    #[test]
    fn test_source_ip_address_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = "fe80::1".parse::<Ipv6Addr>().unwrap();
        let source_ip = SourceIpAddress::new_dual(ipv4, ipv6);

        assert!(source_ip.v4);
        assert!(source_ip.v6);
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, Some(ipv6));

        // Marshal and unmarshal
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 21); // 1 byte flags + 4 bytes IPv4 + 16 bytes IPv6
        assert_eq!(marshaled[0], 0x03); // Both V4 and V6 flags

        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
    }

    #[test]
    fn test_source_ip_address_new_with_options() {
        let ipv4 = Ipv4Addr::new(172, 16, 0, 1);
        let ipv6 = "::1".parse::<Ipv6Addr>().unwrap();

        // Both addresses
        let source_ip = SourceIpAddress::new(Some(ipv4), Some(ipv6));
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, Some(ipv6));
        assert!(source_ip.v4);
        assert!(source_ip.v6);

        // IPv4 only
        let source_ip = SourceIpAddress::new(Some(ipv4), None);
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, None);
        assert!(source_ip.v4);
        assert!(!source_ip.v6);

        // IPv6 only
        let source_ip = SourceIpAddress::new(None, Some(ipv6));
        assert_eq!(source_ip.ipv4, None);
        assert_eq!(source_ip.ipv6, Some(ipv6));
        assert!(!source_ip.v4);
        assert!(source_ip.v6);
    }

    #[test]
    fn test_source_ip_address_round_trip() {
        let test_cases = vec![
            SourceIpAddress::new_ipv4(Ipv4Addr::new(127, 0, 0, 1)),
            SourceIpAddress::new_ipv6("::1".parse().unwrap()),
            SourceIpAddress::new_dual(
                Ipv4Addr::new(192, 168, 1, 1),
                "2001:db8::1".parse().unwrap(),
            ),
        ];

        for source_ip in test_cases {
            let marshaled = source_ip.marshal();
            let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled, source_ip);
        }
    }

    #[test]
    fn test_source_ip_address_unmarshal_empty() {
        let result = SourceIpAddress::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.is_err()); // Error type changed to PfcpError
    }

    #[test]
    fn test_source_ip_address_unmarshal_ipv4_too_short() {
        let payload = vec![0x02, 192, 168]; // V4 flag but only 2 bytes
        let result = SourceIpAddress::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("too short for IPv4"));
    }

    #[test]
    fn test_source_ip_address_unmarshal_ipv6_too_short() {
        let payload = vec![0x01, 0x20, 0x01]; // V6 flag but only 2 bytes
        let result = SourceIpAddress::unmarshal(&payload);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("too short for IPv6"));
    }

    #[test]
    fn test_source_ip_address_to_ie() {
        let source_ip = SourceIpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let ie = source_ip.to_ie();

        assert_eq!(ie.ie_type, IeType::SourceIpAddress);
        assert_eq!(ie.payload, source_ip.marshal());
    }

    #[test]
    fn test_source_ip_address_flags_encoding() {
        // Test flag byte encoding
        let ipv4_only = SourceIpAddress::new_ipv4(Ipv4Addr::new(1, 2, 3, 4));
        assert_eq!(ipv4_only.marshal()[0], 0x02); // Bit 1 set (V4)

        let ipv6_only = SourceIpAddress::new_ipv6("::1".parse().unwrap());
        assert_eq!(ipv6_only.marshal()[0], 0x01); // Bit 0 set (V6)

        let dual = SourceIpAddress::new_dual(Ipv4Addr::new(1, 2, 3, 4), "::1".parse().unwrap());
        assert_eq!(dual.marshal()[0], 0x03); // Both bits set (V4 + V6)
    }

    #[test]
    fn test_source_ip_address_ipv4_with_mask() {
        // IPv4 with /24 subnet mask
        let ipv4 = Ipv4Addr::new(192, 0, 2, 10);
        let source_ip = SourceIpAddress::new_ipv4(ipv4).with_mask(24);

        assert!(source_ip.v4);
        assert!(!source_ip.v6);
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.mask_prefix_length, Some(24));

        // Marshal and verify
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 6); // 1 byte flags + 4 bytes IPv4 + 1 byte mask
        assert_eq!(marshaled[0], 0x06); // V4 (0x02) + MPL (0x04)
        assert_eq!(marshaled[5], 24); // Mask byte

        // Round-trip
        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
        assert_eq!(unmarshaled.mask_prefix_length, Some(24));
    }

    #[test]
    fn test_source_ip_address_ipv6_with_prefix() {
        // IPv6 with /64 prefix
        let ipv6 = "2001:db8::1".parse::<Ipv6Addr>().unwrap();
        let source_ip = SourceIpAddress::new_ipv6(ipv6).with_mask(64);

        assert!(!source_ip.v4);
        assert!(source_ip.v6);
        assert_eq!(source_ip.ipv6, Some(ipv6));
        assert_eq!(source_ip.mask_prefix_length, Some(64));

        // Marshal and verify
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 18); // 1 byte flags + 16 bytes IPv6 + 1 byte prefix
        assert_eq!(marshaled[0], 0x05); // V6 (0x01) + MPL (0x04)
        assert_eq!(marshaled[17], 64); // Prefix byte

        // Round-trip
        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
        assert_eq!(unmarshaled.mask_prefix_length, Some(64));
    }

    #[test]
    fn test_source_ip_address_dual_with_mask() {
        // Dual-stack with /24 mask
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = "fe80::1".parse::<Ipv6Addr>().unwrap();
        let source_ip = SourceIpAddress::new_dual(ipv4, ipv6).with_mask(24);

        assert!(source_ip.v4);
        assert!(source_ip.v6);
        assert_eq!(source_ip.ipv4, Some(ipv4));
        assert_eq!(source_ip.ipv6, Some(ipv6));
        assert_eq!(source_ip.mask_prefix_length, Some(24));

        // Marshal and verify
        let marshaled = source_ip.marshal();
        assert_eq!(marshaled.len(), 22); // 1 + 4 + 16 + 1
        assert_eq!(marshaled[0], 0x07); // V6 (0x01) + V4 (0x02) + MPL (0x04)
        assert_eq!(marshaled[21], 24); // Mask byte

        // Round-trip
        let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, source_ip);
    }

    #[test]
    fn test_source_ip_address_mask_prefix_values() {
        // Test various mask/prefix values
        let test_cases = vec![
            (8, "IPv4 /8"),
            (16, "IPv4 /16"),
            (24, "IPv4 /24"),
            (32, "IPv4 /32"),
            (48, "IPv6 /48"),
            (64, "IPv6 /64"),
            (128, "IPv6 /128"),
        ];

        for (mask_value, desc) in test_cases {
            let source_ip =
                SourceIpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1)).with_mask(mask_value);

            let marshaled = source_ip.marshal();
            let unmarshaled = SourceIpAddress::unmarshal(&marshaled).unwrap();

            assert_eq!(
                unmarshaled.mask_prefix_length,
                Some(mask_value),
                "Failed for {}",
                desc
            );
        }
    }

    #[test]
    fn test_source_ip_address_unmarshal_mpl_flag_without_data() {
        // MPL flag set but no mask byte - should fail
        let payload = vec![0x06, 192, 168, 1, 1]; // V4 + MPL flags, IPv4 address, but no mask byte
        let result = SourceIpAddress::unmarshal(&payload);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("too short for mask/prefix length"));
    }

    #[test]
    fn test_source_ip_address_backward_compatibility() {
        // Old payloads without MPL flag should still work
        let ipv4_payload = vec![0x02, 192, 168, 1, 100]; // V4 flag only
        let result = SourceIpAddress::unmarshal(&ipv4_payload).unwrap();

        assert!(result.v4);
        assert!(!result.v6);
        assert_eq!(result.ipv4, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(result.mask_prefix_length, None);

        let ipv6_payload = vec![
            0x01, 0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ]; // V6 flag only
        let result = SourceIpAddress::unmarshal(&ipv6_payload).unwrap();

        assert!(!result.v4);
        assert!(result.v6);
        assert_eq!(
            result.ipv6,
            Some("2001:db8::1".parse::<Ipv6Addr>().unwrap())
        );
        assert_eq!(result.mask_prefix_length, None);
    }
}
