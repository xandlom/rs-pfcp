//! 3GPP Interface Type IE.
//!
//! Identifies the type of interface in the 5G network architecture (N3, N6, N9, etc.).
//! Used in ForwardingParameters to specify the interface type for packet forwarding.

use crate::error::messages;
use crate::ie::{Ie, IeType};
use std::io;

/// 3GPP Interface Type values
///
/// Specifies the interface type in 5G network architecture as defined
/// in 3GPP TS 29.244 Section 8.2.149.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeGppInterfaceType {
    S1U = 0,                  // S1-U (4G)
    S5S8U = 1,                // S5/S8-U (4G)
    S4U = 2,                  // S4-U (3G/4G interworking)
    S11U = 3,                 // S11-U (4G)
    S12U = 4,                 // S12-U (4G)
    Gn = 5,                   // Gn/Gp (3G)
    S2aU = 6,                 // S2a-U (untrusted non-3GPP)
    S2bU = 7,                 // S2b-U (trusted non-3GPP)
    ENodeBCpFunctionGtpU = 8, // eNodeB GTP-U
    ENodeBUpFunctionGtpU = 9, // eNodeB GTP-U UP
    SgmbU = 10,               // SGmb-U (eMBMS)
    N3 = 11,                  // N3 (5G - RAN to UPF)
    N6 = 12,                  // N6 (5G - UPF to Data Network)
    N9 = 13,                  // N9 (5G - UPF to UPF)
    N4U = 14,                 // N4-U (5G)
    N19 = 15,                 // N19 (5G - UPF to UPF for roaming)
}

impl ThreeGppInterfaceType {
    /// Creates from u8 value
    pub fn from_u8(value: u8) -> Result<Self, io::Error> {
        match value {
            0 => Ok(ThreeGppInterfaceType::S1U),
            1 => Ok(ThreeGppInterfaceType::S5S8U),
            2 => Ok(ThreeGppInterfaceType::S4U),
            3 => Ok(ThreeGppInterfaceType::S11U),
            4 => Ok(ThreeGppInterfaceType::S12U),
            5 => Ok(ThreeGppInterfaceType::Gn),
            6 => Ok(ThreeGppInterfaceType::S2aU),
            7 => Ok(ThreeGppInterfaceType::S2bU),
            8 => Ok(ThreeGppInterfaceType::ENodeBCpFunctionGtpU),
            9 => Ok(ThreeGppInterfaceType::ENodeBUpFunctionGtpU),
            10 => Ok(ThreeGppInterfaceType::SgmbU),
            11 => Ok(ThreeGppInterfaceType::N3),
            12 => Ok(ThreeGppInterfaceType::N6),
            13 => Ok(ThreeGppInterfaceType::N9),
            14 => Ok(ThreeGppInterfaceType::N4U),
            15 => Ok(ThreeGppInterfaceType::N19),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid 3GPP Interface Type value: {}", value),
            )),
        }
    }

    /// Converts to u8 value
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Returns true if this is a 5G interface (N3, N6, N9, N4U, N19)
    pub fn is_5g_interface(self) -> bool {
        matches!(
            self,
            ThreeGppInterfaceType::N3
                | ThreeGppInterfaceType::N6
                | ThreeGppInterfaceType::N9
                | ThreeGppInterfaceType::N4U
                | ThreeGppInterfaceType::N19
        )
    }

    /// Returns true if this is a 4G interface
    pub fn is_4g_interface(self) -> bool {
        matches!(
            self,
            ThreeGppInterfaceType::S1U
                | ThreeGppInterfaceType::S5S8U
                | ThreeGppInterfaceType::S4U
                | ThreeGppInterfaceType::S11U
                | ThreeGppInterfaceType::S12U
                | ThreeGppInterfaceType::SgmbU
        )
    }
}

/// 3GPP Interface Type IE wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreeGppInterfaceTypeIe {
    pub interface_type: ThreeGppInterfaceType,
}

impl ThreeGppInterfaceTypeIe {
    /// Creates a new 3GPP Interface Type IE
    pub fn new(interface_type: ThreeGppInterfaceType) -> Self {
        ThreeGppInterfaceTypeIe { interface_type }
    }

    /// Marshals the IE into bytes
    pub fn marshal(&self) -> Vec<u8> {
        // 1 byte for interface type value + 5 spare bytes (total 6 bytes)
        vec![self.interface_type.to_u8(), 0, 0, 0, 0, 0]
    }

    /// Unmarshals bytes into the IE
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                messages::payload_too_short("3GPP Interface Type"),
            ));
        }

        let interface_type = ThreeGppInterfaceType::from_u8(payload[0])?;
        Ok(ThreeGppInterfaceTypeIe { interface_type })
    }

    /// Wraps in an IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TgppInterfaceType, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3gpp_interface_type_n3() {
        let ie = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N3);

        assert_eq!(ie.interface_type, ThreeGppInterfaceType::N3);
        assert!(ie.interface_type.is_5g_interface());
        assert!(!ie.interface_type.is_4g_interface());

        let marshaled = ie.marshal();
        let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&marshaled).unwrap();

        assert_eq!(ie, unmarshaled);
    }

    #[test]
    fn test_3gpp_interface_type_n6() {
        let ie = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N6);

        assert_eq!(ie.interface_type, ThreeGppInterfaceType::N6);
        assert!(ie.interface_type.is_5g_interface());

        let marshaled = ie.marshal();
        let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&marshaled).unwrap();

        assert_eq!(ie, unmarshaled);
    }

    #[test]
    fn test_3gpp_interface_type_s1u() {
        let ie = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::S1U);

        assert_eq!(ie.interface_type, ThreeGppInterfaceType::S1U);
        assert!(!ie.interface_type.is_5g_interface());
        assert!(ie.interface_type.is_4g_interface());

        let marshaled = ie.marshal();
        let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&marshaled).unwrap();

        assert_eq!(ie, unmarshaled);
    }

    #[test]
    fn test_3gpp_interface_type_to_ie() {
        let ie = ThreeGppInterfaceTypeIe::new(ThreeGppInterfaceType::N9);
        let wrapped = ie.to_ie();

        assert_eq!(wrapped.ie_type, IeType::TgppInterfaceType);

        let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&wrapped.payload).unwrap();
        assert_eq!(ie, unmarshaled);
    }

    #[test]
    fn test_3gpp_interface_type_all_values() {
        let types = vec![
            ThreeGppInterfaceType::S1U,
            ThreeGppInterfaceType::S5S8U,
            ThreeGppInterfaceType::S4U,
            ThreeGppInterfaceType::S11U,
            ThreeGppInterfaceType::S12U,
            ThreeGppInterfaceType::Gn,
            ThreeGppInterfaceType::S2aU,
            ThreeGppInterfaceType::S2bU,
            ThreeGppInterfaceType::ENodeBCpFunctionGtpU,
            ThreeGppInterfaceType::ENodeBUpFunctionGtpU,
            ThreeGppInterfaceType::SgmbU,
            ThreeGppInterfaceType::N3,
            ThreeGppInterfaceType::N6,
            ThreeGppInterfaceType::N9,
            ThreeGppInterfaceType::N4U,
            ThreeGppInterfaceType::N19,
        ];

        for interface_type in types {
            let ie = ThreeGppInterfaceTypeIe::new(interface_type);
            let marshaled = ie.marshal();
            let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&marshaled).unwrap();
            assert_eq!(ie, unmarshaled);
        }
    }

    #[test]
    fn test_3gpp_interface_type_from_u8_invalid() {
        let result = ThreeGppInterfaceType::from_u8(255);
        assert!(result.is_err());
    }

    #[test]
    fn test_3gpp_interface_type_unmarshal_empty() {
        let result = ThreeGppInterfaceTypeIe::unmarshal(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_5g_interface_classification() {
        assert!(ThreeGppInterfaceType::N3.is_5g_interface());
        assert!(ThreeGppInterfaceType::N6.is_5g_interface());
        assert!(ThreeGppInterfaceType::N9.is_5g_interface());
        assert!(ThreeGppInterfaceType::N4U.is_5g_interface());
        assert!(ThreeGppInterfaceType::N19.is_5g_interface());

        assert!(!ThreeGppInterfaceType::S1U.is_5g_interface());
        assert!(!ThreeGppInterfaceType::S5S8U.is_5g_interface());
    }

    #[test]
    fn test_4g_interface_classification() {
        assert!(ThreeGppInterfaceType::S1U.is_4g_interface());
        assert!(ThreeGppInterfaceType::S5S8U.is_4g_interface());
        assert!(ThreeGppInterfaceType::S4U.is_4g_interface());
        assert!(ThreeGppInterfaceType::S11U.is_4g_interface());

        assert!(!ThreeGppInterfaceType::N3.is_4g_interface());
        assert!(!ThreeGppInterfaceType::N6.is_4g_interface());
    }
}
