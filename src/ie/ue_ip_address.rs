//! UE IP Address IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
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
            flags |= 1; // Bit 0: V6 (IPv6)
        }
        if self.v4 {
            flags |= 2; // Bit 1: V4 (IPv4)
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
    ///
    /// Per 3GPP TS 29.244, UE IP Address requires minimum 1 byte (flags) plus address data.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "UE IP Address",
                IeType::UeIpAddress,
                1,
                0,
            ));
        }
        let flags = payload[0];
        let v6 = flags & 1 != 0; // Bit 0: V6 (IPv6)
        let v4 = flags & 2 != 0; // Bit 1: V4 (IPv4)
        let mut offset = 1;
        let ipv4_address = if v4 {
            if payload.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "UE IP Address IPv4",
                    IeType::UeIpAddress,
                    offset + 4,
                    payload.len(),
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
                return Err(PfcpError::invalid_length(
                    "UE IP Address IPv6",
                    IeType::UeIpAddress,
                    offset + 16,
                    payload.len(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ue_ip_address_ipv4_only() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let ue_ip = UeIpAddress::new(Some(ipv4), None);

        assert!(ue_ip.v4);
        assert!(!ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, Some(ipv4));
        assert_eq!(ue_ip.ipv6_address, None);
    }

    #[test]
    fn test_ue_ip_address_ipv6_only() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let ue_ip = UeIpAddress::new(None, Some(ipv6));

        assert!(!ue_ip.v4);
        assert!(ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, None);
        assert_eq!(ue_ip.ipv6_address, Some(ipv6));
    }

    #[test]
    fn test_ue_ip_address_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let ue_ip = UeIpAddress::new(Some(ipv4), Some(ipv6));

        assert!(ue_ip.v4);
        assert!(ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, Some(ipv4));
        assert_eq!(ue_ip.ipv6_address, Some(ipv6));
    }

    #[test]
    fn test_ue_ip_address_neither() {
        let ue_ip = UeIpAddress::new(None, None);

        assert!(!ue_ip.v4);
        assert!(!ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, None);
        assert_eq!(ue_ip.ipv6_address, None);
    }

    #[test]
    fn test_ue_ip_address_marshal_ipv4() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 100);
        let ue_ip = UeIpAddress::new(Some(ipv4), None);
        let marshaled = ue_ip.marshal();

        assert_eq!(marshaled.len(), 5); // 1 flag byte + 4 IPv4 bytes
        assert_eq!(marshaled[0], 0x02); // V4 bit set (bit 1)
        assert_eq!(&marshaled[1..5], &[192, 168, 1, 100]);
    }

    #[test]
    fn test_ue_ip_address_marshal_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let ue_ip = UeIpAddress::new(None, Some(ipv6));
        let marshaled = ue_ip.marshal();

        assert_eq!(marshaled.len(), 17); // 1 flag byte + 16 IPv6 bytes
        assert_eq!(marshaled[0], 0x01); // V6 bit set (bit 0)
    }

    #[test]
    fn test_ue_ip_address_marshal_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let ue_ip = UeIpAddress::new(Some(ipv4), Some(ipv6));
        let marshaled = ue_ip.marshal();

        assert_eq!(marshaled.len(), 21); // 1 flag byte + 4 IPv4 + 16 IPv6
        assert_eq!(marshaled[0], 0x03); // Both V4 and V6 bits set
        assert_eq!(&marshaled[1..5], &[10, 0, 0, 1]); // IPv4
    }

    #[test]
    fn test_ue_ip_address_marshal_neither() {
        let ue_ip = UeIpAddress::new(None, None);
        let marshaled = ue_ip.marshal();

        assert_eq!(marshaled.len(), 1); // Only flag byte
        assert_eq!(marshaled[0], 0x00); // No bits set
    }

    #[test]
    fn test_ue_ip_address_unmarshal_ipv4() {
        let data = vec![0x02, 192, 168, 1, 100]; // V4 flag + IPv4
        let ue_ip = UeIpAddress::unmarshal(&data).unwrap();

        assert!(ue_ip.v4);
        assert!(!ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(ue_ip.ipv6_address, None);
    }

    #[test]
    fn test_ue_ip_address_unmarshal_ipv6() {
        let mut data = vec![0x01]; // V6 flag
        data.extend_from_slice(&[
            0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ]);
        let ue_ip = UeIpAddress::unmarshal(&data).unwrap();

        assert!(!ue_ip.v4);
        assert!(ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, None);
        assert_eq!(
            ue_ip.ipv6_address,
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
        );
    }

    #[test]
    fn test_ue_ip_address_unmarshal_dual_stack() {
        let mut data = vec![0x03]; // Both V4 and V6 flags
        data.extend_from_slice(&[10, 0, 0, 1]); // IPv4
        data.extend_from_slice(&[
            0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ]); // IPv6
        let ue_ip = UeIpAddress::unmarshal(&data).unwrap();

        assert!(ue_ip.v4);
        assert!(ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, Some(Ipv4Addr::new(10, 0, 0, 1)));
        assert_eq!(
            ue_ip.ipv6_address,
            Some(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1))
        );
    }

    #[test]
    fn test_ue_ip_address_unmarshal_neither() {
        let data = vec![0x00]; // No flags set
        let ue_ip = UeIpAddress::unmarshal(&data).unwrap();

        assert!(!ue_ip.v4);
        assert!(!ue_ip.v6);
        assert_eq!(ue_ip.ipv4_address, None);
        assert_eq!(ue_ip.ipv6_address, None);
    }

    #[test]
    fn test_ue_ip_address_unmarshal_empty_buffer() {
        use crate::error::PfcpError;

        let data = vec![];
        let result = UeIpAddress::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ue_ip_address_unmarshal_short_ipv4() {
        use crate::error::PfcpError;

        let data = vec![0x02, 192, 168, 1]; // V4 flag but only 3 bytes
        let result = UeIpAddress::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ue_ip_address_unmarshal_short_ipv6() {
        use crate::error::PfcpError;

        let data = vec![
            0x01, 0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]; // V6 flag but only 10 bytes
        let result = UeIpAddress::unmarshal(&data);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ue_ip_address_round_trip_ipv4() {
        let ipv4 = Ipv4Addr::new(172, 16, 0, 1);
        let original = UeIpAddress::new(Some(ipv4), None);
        let marshaled = original.marshal();
        let unmarshaled = UeIpAddress::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ue_ip_address_round_trip_ipv6() {
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0xdead, 0xbeef, 0, 0, 0, 1);
        let original = UeIpAddress::new(None, Some(ipv6));
        let marshaled = original.marshal();
        let unmarshaled = UeIpAddress::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ue_ip_address_round_trip_dual_stack() {
        let ipv4 = Ipv4Addr::new(192, 168, 100, 50);
        let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0xcafe, 0xbabe, 0, 0, 0, 1);
        let original = UeIpAddress::new(Some(ipv4), Some(ipv6));
        let marshaled = original.marshal();
        let unmarshaled = UeIpAddress::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ue_ip_address_to_ie() {
        let ipv4 = Ipv4Addr::new(10, 10, 10, 10);
        let ue_ip = UeIpAddress::new(Some(ipv4), None);
        let ie = ue_ip.to_ie();

        assert_eq!(ie.ie_type, IeType::UeIpAddress);
        assert_eq!(ie.payload.len(), 5);
    }

    #[test]
    fn test_ue_ip_address_real_world_scenarios() {
        // Scenario 1: Mobile device with IPv4
        let mobile_ipv4 = UeIpAddress::new(Some(Ipv4Addr::new(100, 64, 0, 1)), None);
        assert!(mobile_ipv4.v4);
        assert!(!mobile_ipv4.v6);

        // Scenario 2: 5G device with IPv6
        let mobile_ipv6 =
            UeIpAddress::new(None, Some(Ipv6Addr::new(0x2001, 0xdb8, 0x5, 0, 0, 0, 0, 1)));
        assert!(!mobile_ipv6.v4);
        assert!(mobile_ipv6.v6);

        // Scenario 3: Dual-stack 5G device
        let dual_stack = UeIpAddress::new(
            Some(Ipv4Addr::new(100, 64, 1, 1)),
            Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        );
        assert!(dual_stack.v4);
        assert!(dual_stack.v6);
    }

    #[test]
    fn test_ue_ip_address_private_ranges() {
        // Test various private IP ranges used in mobile networks
        let private_10 = UeIpAddress::new(Some(Ipv4Addr::new(10, 0, 0, 1)), None);
        let private_172 = UeIpAddress::new(Some(Ipv4Addr::new(172, 16, 0, 1)), None);
        let private_192 = UeIpAddress::new(Some(Ipv4Addr::new(192, 168, 1, 1)), None);
        let cgnat = UeIpAddress::new(Some(Ipv4Addr::new(100, 64, 0, 1)), None);

        assert!(private_10.v4);
        assert!(private_172.v4);
        assert!(private_192.v4);
        assert!(cgnat.v4);
    }

    #[test]
    fn test_ue_ip_address_ipv6_link_local() {
        let link_local = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let ue_ip = UeIpAddress::new(None, Some(link_local));

        let marshaled = ue_ip.marshal();
        let unmarshaled = UeIpAddress::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.ipv6_address, Some(link_local));
    }
}
