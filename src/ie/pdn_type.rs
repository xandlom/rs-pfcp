//! PDN Type IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents the PDN Type Information Element.
/// Used to indicate the type of PDN connection (IPv4, IPv6, IPv4v6, Non-IP, Ethernet).
/// Defined in 3GPP TS 29.244 Section 8.2.99.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdnType {
    pub pdn_type: PdnTypeValue,
}

/// PDN Type values as defined in 3GPP TS 29.244.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PdnTypeValue {
    /// IPv4 PDN type
    Ipv4 = 1,
    /// IPv6 PDN type
    Ipv6 = 2,
    /// IPv4v6 PDN type (dual stack)
    Ipv4v6 = 3,
    /// Non-IP PDN type
    NonIp = 4,
    /// Ethernet PDN type
    Ethernet = 5,
    /// Unknown/unsupported PDN type
    Unknown(u8),
}

impl From<u8> for PdnTypeValue {
    fn from(value: u8) -> Self {
        match value {
            1 => PdnTypeValue::Ipv4,
            2 => PdnTypeValue::Ipv6,
            3 => PdnTypeValue::Ipv4v6,
            4 => PdnTypeValue::NonIp,
            5 => PdnTypeValue::Ethernet,
            _ => PdnTypeValue::Unknown(value),
        }
    }
}

impl From<PdnTypeValue> for u8 {
    fn from(pdn_type: PdnTypeValue) -> u8 {
        match pdn_type {
            PdnTypeValue::Ipv4 => 1,
            PdnTypeValue::Ipv6 => 2,
            PdnTypeValue::Ipv4v6 => 3,
            PdnTypeValue::NonIp => 4,
            PdnTypeValue::Ethernet => 5,
            PdnTypeValue::Unknown(value) => value,
        }
    }
}

impl PdnType {
    /// Creates a new PDN Type IE.
    pub fn new(pdn_type: PdnTypeValue) -> Self {
        PdnType { pdn_type }
    }

    /// Creates an IPv4 PDN Type.
    pub fn ipv4() -> Self {
        PdnType::new(PdnTypeValue::Ipv4)
    }

    /// Creates an IPv6 PDN Type.
    pub fn ipv6() -> Self {
        PdnType::new(PdnTypeValue::Ipv6)
    }

    /// Creates an IPv4v6 PDN Type (dual stack).
    pub fn ipv4v6() -> Self {
        PdnType::new(PdnTypeValue::Ipv4v6)
    }

    /// Creates a Non-IP PDN Type.
    pub fn non_ip() -> Self {
        PdnType::new(PdnTypeValue::NonIp)
    }

    /// Creates an Ethernet PDN Type.
    pub fn ethernet() -> Self {
        PdnType::new(PdnTypeValue::Ethernet)
    }

    /// Checks if the PDN type supports IPv4.
    pub fn supports_ipv4(&self) -> bool {
        matches!(self.pdn_type, PdnTypeValue::Ipv4 | PdnTypeValue::Ipv4v6)
    }

    /// Checks if the PDN type supports IPv6.
    pub fn supports_ipv6(&self) -> bool {
        matches!(self.pdn_type, PdnTypeValue::Ipv6 | PdnTypeValue::Ipv4v6)
    }

    /// Checks if the PDN type is IP-based.
    pub fn is_ip_based(&self) -> bool {
        matches!(
            self.pdn_type,
            PdnTypeValue::Ipv4 | PdnTypeValue::Ipv6 | PdnTypeValue::Ipv4v6
        )
    }

    /// Marshals the PDN Type into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        vec![u8::from(self.pdn_type.clone())]
    }

    /// Unmarshals a byte slice into a PDN Type IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length("PDN Type", IeType::PdnType, 1, 0));
        }

        let pdn_type = PdnTypeValue::from(payload[0]);

        Ok(PdnType { pdn_type })
    }

    /// Wraps the PDN Type in a PDN Type IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PdnType, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdn_type_marshal_unmarshal_ipv4() {
        let pdn_type = PdnType::ipv4();
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::Ipv4);
        assert_eq!(marshaled, vec![1]);
        assert!(unmarshaled.supports_ipv4());
        assert!(!unmarshaled.supports_ipv6());
        assert!(unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_marshal_unmarshal_ipv6() {
        let pdn_type = PdnType::ipv6();
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::Ipv6);
        assert_eq!(marshaled, vec![2]);
        assert!(!unmarshaled.supports_ipv4());
        assert!(unmarshaled.supports_ipv6());
        assert!(unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_marshal_unmarshal_ipv4v6() {
        let pdn_type = PdnType::ipv4v6();
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::Ipv4v6);
        assert_eq!(marshaled, vec![3]);
        assert!(unmarshaled.supports_ipv4());
        assert!(unmarshaled.supports_ipv6());
        assert!(unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_marshal_unmarshal_non_ip() {
        let pdn_type = PdnType::non_ip();
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::NonIp);
        assert_eq!(marshaled, vec![4]);
        assert!(!unmarshaled.supports_ipv4());
        assert!(!unmarshaled.supports_ipv6());
        assert!(!unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_marshal_unmarshal_ethernet() {
        let pdn_type = PdnType::ethernet();
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::Ethernet);
        assert_eq!(marshaled, vec![5]);
        assert!(!unmarshaled.supports_ipv4());
        assert!(!unmarshaled.supports_ipv6());
        assert!(!unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_unknown() {
        let pdn_type = PdnType::new(PdnTypeValue::Unknown(99));
        let marshaled = pdn_type.marshal();
        let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();

        assert_eq!(pdn_type, unmarshaled);
        assert_eq!(unmarshaled.pdn_type, PdnTypeValue::Unknown(99));
        assert_eq!(marshaled, vec![99]);
        assert!(!unmarshaled.supports_ipv4());
        assert!(!unmarshaled.supports_ipv6());
        assert!(!unmarshaled.is_ip_based());
    }

    #[test]
    fn test_pdn_type_to_ie() {
        let pdn_type = PdnType::ipv4v6();
        let ie = pdn_type.to_ie();

        assert_eq!(ie.ie_type, IeType::PdnType);

        let unmarshaled = PdnType::unmarshal(&ie.payload).unwrap();
        assert_eq!(pdn_type, unmarshaled);
    }

    #[test]
    fn test_pdn_type_unmarshal_empty() {
        let result = PdnType::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_pdn_type_value_conversions() {
        // Test u8 to PdnTypeValue conversion
        assert_eq!(PdnTypeValue::from(1), PdnTypeValue::Ipv4);
        assert_eq!(PdnTypeValue::from(2), PdnTypeValue::Ipv6);
        assert_eq!(PdnTypeValue::from(3), PdnTypeValue::Ipv4v6);
        assert_eq!(PdnTypeValue::from(4), PdnTypeValue::NonIp);
        assert_eq!(PdnTypeValue::from(5), PdnTypeValue::Ethernet);
        assert_eq!(PdnTypeValue::from(99), PdnTypeValue::Unknown(99));

        // Test PdnTypeValue to u8 conversion
        assert_eq!(u8::from(PdnTypeValue::Ipv4), 1);
        assert_eq!(u8::from(PdnTypeValue::Ipv6), 2);
        assert_eq!(u8::from(PdnTypeValue::Ipv4v6), 3);
        assert_eq!(u8::from(PdnTypeValue::NonIp), 4);
        assert_eq!(u8::from(PdnTypeValue::Ethernet), 5);
        assert_eq!(u8::from(PdnTypeValue::Unknown(99)), 99);
    }

    #[test]
    fn test_pdn_type_round_trip_all_values() {
        let test_cases = vec![
            PdnType::ipv4(),
            PdnType::ipv6(),
            PdnType::ipv4v6(),
            PdnType::non_ip(),
            PdnType::ethernet(),
            PdnType::new(PdnTypeValue::Unknown(128)),
        ];

        for original in test_cases {
            let marshaled = original.marshal();
            let unmarshaled = PdnType::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled);
        }
    }
}
