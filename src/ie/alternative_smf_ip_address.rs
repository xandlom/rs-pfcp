//! Alternative SMF IP Address Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents an Alternative SMF IP Address IE.
///
/// This IE indicates the alternative SMF for PFCP sessions which are associated
/// with the FQ-CSID(s) or Group ID(s), or which have their CP F-SEIDs containing the SMF IP Address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeSmfIpAddress {
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub preferred_pfcp_entity: bool,
}

impl AlternativeSmfIpAddress {
    /// Creates a new Alternative SMF IP Address with IPv4 only.
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        AlternativeSmfIpAddress {
            ipv4_address: Some(addr),
            ipv6_address: None,
            preferred_pfcp_entity: false,
        }
    }

    /// Creates a new Alternative SMF IP Address with IPv6 only.
    pub fn new_ipv6(addr: Ipv6Addr) -> Self {
        AlternativeSmfIpAddress {
            ipv4_address: None,
            ipv6_address: Some(addr),
            preferred_pfcp_entity: false,
        }
    }

    /// Creates a new Alternative SMF IP Address with both IPv4 and IPv6.
    pub fn new_dual_stack(ipv4: Ipv4Addr, ipv6: Ipv6Addr) -> Self {
        AlternativeSmfIpAddress {
            ipv4_address: Some(ipv4),
            ipv6_address: Some(ipv6),
            preferred_pfcp_entity: false,
        }
    }

    /// Sets the Preferred PFCP Entity flag.
    pub fn with_preferred_pfcp_entity(mut self, preferred: bool) -> Self {
        self.preferred_pfcp_entity = preferred;
        self
    }

    /// Marshals the Alternative SMF IP Address into a byte vector.
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
        if self.preferred_pfcp_entity {
            flags |= 0x04; // PPE flag (bit 3)
        }
        // Bits 4-8 are spare and remain 0

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

    /// Unmarshals an Alternative SMF IP Address from a byte slice.
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Alternative SMF IP Address",
                IeType::AlternativeSmfIpAddress,
                1,
                0,
            ));
        }

        let flags = data[0];
        let v6 = (flags & 0x01) != 0; // bit 1
        let v4 = (flags & 0x02) != 0; // bit 2
        let ppe = (flags & 0x04) != 0; // bit 3

        // Check spare bits are zero
        if (flags & 0xF8) != 0 {
            return Err(PfcpError::invalid_value(
                "Alternative SMF IP Address flags",
                format!("0x{:02X}", flags),
                "spare bits must be zero",
            ));
        }

        // At least one address must be present
        if !v4 && !v6 {
            return Err(PfcpError::invalid_value(
                "Alternative SMF IP Address flags",
                format!("0x{:02X}", flags),
                "at least one IP address must be present",
            ));
        }

        let mut offset = 1;
        let mut ipv4_address = None;
        let mut ipv6_address = None;

        // Parse IPv4 address if present
        if v4 {
            if data.len() < offset + 4 {
                return Err(PfcpError::invalid_length(
                    "Alternative SMF IP Address (IPv4)",
                    IeType::AlternativeSmfIpAddress,
                    offset + 4,
                    data.len(),
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
                return Err(PfcpError::invalid_length(
                    "Alternative SMF IP Address (IPv6)",
                    IeType::AlternativeSmfIpAddress,
                    offset + 16,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&data[offset..offset + 16]);
            ipv6_address = Some(Ipv6Addr::from(octets));
        }

        Ok(AlternativeSmfIpAddress {
            ipv4_address,
            ipv6_address,
            preferred_pfcp_entity: ppe,
        })
    }

    /// Converts to an IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AlternativeSmfIpAddress, self.marshal())
    }
}

impl From<Ipv4Addr> for AlternativeSmfIpAddress {
    fn from(addr: Ipv4Addr) -> Self {
        AlternativeSmfIpAddress::new_ipv4(addr)
    }
}

impl From<Ipv6Addr> for AlternativeSmfIpAddress {
    fn from(addr: Ipv6Addr) -> Self {
        AlternativeSmfIpAddress::new_ipv6(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternative_smf_ip_address_ipv4_only() {
        let addr = Ipv4Addr::new(192, 168, 1, 100);
        let alt_smf = AlternativeSmfIpAddress::new_ipv4(addr);

        assert_eq!(alt_smf.ipv4_address, Some(addr));
        assert_eq!(alt_smf.ipv6_address, None);
        assert!(!alt_smf.preferred_pfcp_entity);

        let marshaled = alt_smf.marshal();
        let expected = vec![0x02, 192, 168, 1, 100]; // V4 flag only
        assert_eq!(marshaled, expected);

        let unmarshaled = AlternativeSmfIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, alt_smf);
    }

    #[test]
    fn test_alternative_smf_ip_address_ipv6_only() {
        let addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
        let alt_smf = AlternativeSmfIpAddress::new_ipv6(addr);

        assert_eq!(alt_smf.ipv4_address, None);
        assert_eq!(alt_smf.ipv6_address, Some(addr));

        let marshaled = alt_smf.marshal();
        assert_eq!(marshaled[0], 0x01); // V6 flag only
        assert_eq!(marshaled.len(), 17); // 1 byte flag + 16 bytes IPv6

        let unmarshaled = AlternativeSmfIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, alt_smf);
    }

    #[test]
    fn test_alternative_smf_ip_address_dual_stack() {
        let ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let alt_smf = AlternativeSmfIpAddress::new_dual_stack(ipv4, ipv6);

        assert_eq!(alt_smf.ipv4_address, Some(ipv4));
        assert_eq!(alt_smf.ipv6_address, Some(ipv6));

        let marshaled = alt_smf.marshal();
        assert_eq!(marshaled[0], 0x03); // Both V4 and V6 flags (0x01 | 0x02)
        assert_eq!(marshaled.len(), 21); // 1 byte flag + 4 bytes IPv4 + 16 bytes IPv6

        let unmarshaled = AlternativeSmfIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, alt_smf);
    }

    #[test]
    fn test_alternative_smf_ip_address_with_ppe_flag() {
        let addr = Ipv4Addr::new(203, 0, 113, 1);
        let alt_smf = AlternativeSmfIpAddress::new_ipv4(addr).with_preferred_pfcp_entity(true);

        assert!(alt_smf.preferred_pfcp_entity);

        let marshaled = alt_smf.marshal();
        assert_eq!(marshaled[0], 0x06); // V4 flag (0x02) + PPE flag (0x04)

        let unmarshaled = AlternativeSmfIpAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, alt_smf);
        assert!(unmarshaled.preferred_pfcp_entity);
    }

    #[test]
    fn test_alternative_smf_ip_address_to_ie() {
        let addr = Ipv4Addr::new(203, 0, 113, 1);
        let alt_smf = AlternativeSmfIpAddress::new_ipv4(addr);
        let ie = alt_smf.to_ie();

        assert_eq!(ie.ie_type, IeType::AlternativeSmfIpAddress);
        assert_eq!(ie.payload, alt_smf.marshal());
    }

    #[test]
    fn test_alternative_smf_ip_address_unmarshal_errors() {
        // Empty data
        let result = AlternativeSmfIpAddress::unmarshal(&[]);
        assert!(result.is_err());

        // No address flags set
        let result = AlternativeSmfIpAddress::unmarshal(&[0x00]);
        assert!(result.is_err());

        // Spare bits set
        let result = AlternativeSmfIpAddress::unmarshal(&[0x08]); // Bit 4 set
        assert!(result.is_err());

        // IPv4 flag but insufficient data
        let result = AlternativeSmfIpAddress::unmarshal(&[0x02, 192, 168]);
        assert!(result.is_err());

        // IPv6 flag but insufficient data
        let result = AlternativeSmfIpAddress::unmarshal(&[0x01, 0x20, 0x01]);
        assert!(result.is_err());

        // Dual stack but insufficient data for IPv6
        let result = AlternativeSmfIpAddress::unmarshal(&[0x03, 192, 168, 1, 1, 0x20, 0x01]);
        assert!(result.is_err());
    }

    #[test]
    fn test_alternative_smf_ip_address_spec_compliance() {
        // Test all flag combinations
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        let ipv6 = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);

        // IPv4 only
        let alt_smf_v4 = AlternativeSmfIpAddress::new_ipv4(ipv4);
        let marshaled = alt_smf_v4.marshal();
        assert_eq!(marshaled[0], 0x02); // V4 flag only

        // IPv6 only
        let alt_smf_v6 = AlternativeSmfIpAddress::new_ipv6(ipv6);
        let marshaled = alt_smf_v6.marshal();
        assert_eq!(marshaled[0], 0x01); // V6 flag only

        // Dual stack
        let alt_smf_dual = AlternativeSmfIpAddress::new_dual_stack(ipv4, ipv6);
        let marshaled = alt_smf_dual.marshal();
        assert_eq!(marshaled[0], 0x03); // V4 + V6 flags

        // IPv4 with PPE
        let alt_smf_v4_ppe =
            AlternativeSmfIpAddress::new_ipv4(ipv4).with_preferred_pfcp_entity(true);
        let marshaled = alt_smf_v4_ppe.marshal();
        assert_eq!(marshaled[0], 0x06); // V4 + PPE flags

        // All flags set
        let alt_smf_all =
            AlternativeSmfIpAddress::new_dual_stack(ipv4, ipv6).with_preferred_pfcp_entity(true);
        let marshaled = alt_smf_all.marshal();
        assert_eq!(marshaled[0], 0x07); // V6 + V4 + PPE flags
    }

    #[test]
    fn test_alternative_smf_ip_address_round_trip() {
        let test_cases = vec![
            AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(127, 0, 0, 1)),
            AlternativeSmfIpAddress::new_ipv6(Ipv6Addr::LOCALHOST),
            AlternativeSmfIpAddress::new_dual_stack(
                Ipv4Addr::new(10, 0, 0, 1),
                Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
            ),
            AlternativeSmfIpAddress::new_ipv4(Ipv4Addr::new(203, 0, 113, 5))
                .with_preferred_pfcp_entity(true),
            AlternativeSmfIpAddress::new_dual_stack(
                Ipv4Addr::new(198, 51, 100, 1),
                Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334),
            )
            .with_preferred_pfcp_entity(true),
        ];

        for addr in test_cases {
            let marshaled = addr.marshal();
            let unmarshaled = AlternativeSmfIpAddress::unmarshal(&marshaled).unwrap();
            assert_eq!(addr, unmarshaled);
        }
    }
}
