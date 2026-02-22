//! Mapped N6 IP Address Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.242, contains a mapped N6 IPv4 address
//! with CHV4 (CHOOSE IPv4) and V4 flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use std::net::Ipv4Addr;

/// Mapped N6 IP Address per 3GPP TS 29.244 Section 8.2.242.
///
/// Flags byte layout:
/// - Bit 1 (0x01): CHV4 - CHOOSE IPv4, UPF assigns the address
/// - Bit 2 (0x02): V4 - IPv4 address is present
///
/// CHV4 and V4 are mutually exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappedN6IpAddress {
    /// When true, UPF shall assign an IPv4 address (CHV4 flag).
    pub choose_ipv4: bool,
    /// Specific IPv4 address (V4 flag).
    pub ipv4: Option<Ipv4Addr>,
}

impl MappedN6IpAddress {
    /// Create with a specific IPv4 address (V4 flag set).
    pub fn new_ipv4(addr: Ipv4Addr) -> Self {
        Self {
            choose_ipv4: false,
            ipv4: Some(addr),
        }
    }

    /// Create with CHV4 flag (UPF assigns the address).
    pub fn choose_v4() -> Self {
        Self {
            choose_ipv4: true,
            ipv4: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut flags: u8 = 0;
        if self.choose_ipv4 {
            flags |= 0x01;
        }
        if self.ipv4.is_some() {
            flags |= 0x02;
        }
        data.push(flags);
        if let Some(addr) = &self.ipv4 {
            data.extend_from_slice(&addr.octets());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Mapped N6 IP Address",
                IeType::MappedN6IpAddress,
                1,
                0,
            ));
        }

        let flags = data[0];
        let chv4 = flags & 0x01 != 0;
        let v4 = flags & 0x02 != 0;

        if chv4 && v4 {
            return Err(PfcpError::invalid_value(
                "Mapped N6 IP Address",
                format!("flags=0x{flags:02x}"),
                "CHV4 and V4 are mutually exclusive",
            ));
        }

        if v4 {
            if data.len() < 5 {
                return Err(PfcpError::invalid_length(
                    "Mapped N6 IP Address",
                    IeType::MappedN6IpAddress,
                    5,
                    data.len(),
                ));
            }
            let mut octets = [0u8; 4];
            octets.copy_from_slice(&data[1..5]);
            Ok(Self {
                choose_ipv4: false,
                ipv4: Some(Ipv4Addr::from(octets)),
            })
        } else if chv4 {
            Ok(Self {
                choose_ipv4: true,
                ipv4: None,
            })
        } else {
            Err(PfcpError::invalid_value(
                "Mapped N6 IP Address",
                format!("flags=0x{flags:02x}"),
                "at least CHV4 or V4 must be set",
            ))
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MappedN6IpAddress, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_ipv4() {
        let addr = MappedN6IpAddress::new_ipv4(Ipv4Addr::new(10, 0, 0, 1));
        let parsed = MappedN6IpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_marshal_unmarshal_choose_v4() {
        let addr = MappedN6IpAddress::choose_v4();
        let parsed = MappedN6IpAddress::unmarshal(&addr.marshal()).unwrap();
        assert_eq!(parsed, addr);
        assert!(parsed.choose_ipv4);
        assert!(parsed.ipv4.is_none());
    }

    #[test]
    fn test_v4_flag_layout() {
        let addr = MappedN6IpAddress::new_ipv4(Ipv4Addr::new(192, 168, 1, 1));
        let data = addr.marshal();
        assert_eq!(data[0], 0x02); // V4 flag
        assert_eq!(data.len(), 5); // flags + 4 bytes IPv4
    }

    #[test]
    fn test_chv4_flag_layout() {
        let addr = MappedN6IpAddress::choose_v4();
        let data = addr.marshal();
        assert_eq!(data[0], 0x01); // CHV4 flag
        assert_eq!(data.len(), 1); // flags only
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MappedN6IpAddress::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_both_flags_error() {
        assert!(matches!(
            MappedN6IpAddress::unmarshal(&[0x03, 10, 0, 0, 1]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_unmarshal_no_flags_error() {
        assert!(matches!(
            MappedN6IpAddress::unmarshal(&[0x00]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_unmarshal_v4_short_buffer() {
        assert!(matches!(
            MappedN6IpAddress::unmarshal(&[0x02, 10, 0]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            MappedN6IpAddress::new_ipv4(Ipv4Addr::LOCALHOST)
                .to_ie()
                .ie_type,
            IeType::MappedN6IpAddress
        );
    }
}
