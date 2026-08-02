//! 3GPP Interface Type IE.
//!
//! Identifies the type of interface in the 5G network architecture (N3, N6, N9, etc.).
//! Used in ForwardingParameters to specify the interface type for packet forwarding.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// 3GPP Interface Type values
///
/// Specifies the interface type in 5G network architecture as defined
/// in 3GPP TS 29.244 Section 8.2.118.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreeGppInterfaceType {
    S1U = 0,
    S5S8U = 1,
    S4U = 2,
    S11U = 3,
    S12U = 4,
    GnGpU = 5,
    S2aU = 6,
    S2bU = 7,
    ENodeBGtpUForDlDataForwarding = 8,
    ENodeBGtpUForUlDataForwarding = 9,
    SgwUpfGtpUForDlDataForwarding = 10,
    N3 = 11,
    N3TrustedNon3GppAccess = 12,
    N3UntrustedNon3GppAccess = 13,
    N3ForDataForwarding = 14,
    N9 = 15,
    Sgi = 16,
    N6 = 17,
    N19 = 18,
    S8U = 19,
    GpU = 20,
    N9ForRoaming = 21,
    IuU = 22,
    N9ForDataForwarding = 23,
    SxaU = 24,
    SxbU = 25,
    SxcU = 26,
    N4U = 27,
    SgwUpfGtpUForUlDataForwarding = 28,
    N6MbNmb9 = 29,
    N3Mb = 30,
    N19Mb = 31,
}

impl ThreeGppInterfaceType {
    // Compatibility aliases for the names exposed before the values were aligned with TS 29.244.
    #[allow(non_upper_case_globals)]
    pub const Gn: Self = Self::GnGpU;
    #[allow(non_upper_case_globals)]
    pub const ENodeBCpFunctionGtpU: Self = Self::ENodeBGtpUForDlDataForwarding;
    #[allow(non_upper_case_globals)]
    pub const ENodeBUpFunctionGtpU: Self = Self::ENodeBGtpUForUlDataForwarding;
    #[allow(non_upper_case_globals)]
    pub const SgmbU: Self = Self::SgwUpfGtpUForDlDataForwarding;

    /// Creates from u8 value
    pub fn from_u8(value: u8) -> Result<Self, PfcpError> {
        match value {
            0 => Ok(ThreeGppInterfaceType::S1U),
            1 => Ok(ThreeGppInterfaceType::S5S8U),
            2 => Ok(ThreeGppInterfaceType::S4U),
            3 => Ok(ThreeGppInterfaceType::S11U),
            4 => Ok(ThreeGppInterfaceType::S12U),
            5 => Ok(ThreeGppInterfaceType::GnGpU),
            6 => Ok(ThreeGppInterfaceType::S2aU),
            7 => Ok(ThreeGppInterfaceType::S2bU),
            8 => Ok(ThreeGppInterfaceType::ENodeBGtpUForDlDataForwarding),
            9 => Ok(ThreeGppInterfaceType::ENodeBGtpUForUlDataForwarding),
            10 => Ok(ThreeGppInterfaceType::SgwUpfGtpUForDlDataForwarding),
            11 => Ok(ThreeGppInterfaceType::N3),
            12 => Ok(ThreeGppInterfaceType::N3TrustedNon3GppAccess),
            13 => Ok(ThreeGppInterfaceType::N3UntrustedNon3GppAccess),
            14 => Ok(ThreeGppInterfaceType::N3ForDataForwarding),
            15 => Ok(ThreeGppInterfaceType::N9),
            16 => Ok(ThreeGppInterfaceType::Sgi),
            17 => Ok(ThreeGppInterfaceType::N6),
            18 => Ok(ThreeGppInterfaceType::N19),
            19 => Ok(ThreeGppInterfaceType::S8U),
            20 => Ok(ThreeGppInterfaceType::GpU),
            21 => Ok(ThreeGppInterfaceType::N9ForRoaming),
            22 => Ok(ThreeGppInterfaceType::IuU),
            23 => Ok(ThreeGppInterfaceType::N9ForDataForwarding),
            24 => Ok(ThreeGppInterfaceType::SxaU),
            25 => Ok(ThreeGppInterfaceType::SxbU),
            26 => Ok(ThreeGppInterfaceType::SxcU),
            27 => Ok(ThreeGppInterfaceType::N4U),
            28 => Ok(ThreeGppInterfaceType::SgwUpfGtpUForUlDataForwarding),
            29 => Ok(ThreeGppInterfaceType::N6MbNmb9),
            30 => Ok(ThreeGppInterfaceType::N3Mb),
            31 => Ok(ThreeGppInterfaceType::N19Mb),
            _ => Err(PfcpError::invalid_value(
                "3GPP Interface Type",
                value.to_string(),
                "must be 0-31",
            )),
        }
    }

    /// Converts to u8 value
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Returns true if this is a 5G interface.
    pub fn is_5g_interface(self) -> bool {
        matches!(
            self,
            ThreeGppInterfaceType::N3
                | ThreeGppInterfaceType::N3TrustedNon3GppAccess
                | ThreeGppInterfaceType::N3UntrustedNon3GppAccess
                | ThreeGppInterfaceType::N3ForDataForwarding
                | ThreeGppInterfaceType::N6
                | ThreeGppInterfaceType::N9
                | ThreeGppInterfaceType::N4U
                | ThreeGppInterfaceType::N19
                | ThreeGppInterfaceType::N9ForRoaming
                | ThreeGppInterfaceType::N9ForDataForwarding
                | ThreeGppInterfaceType::SgwUpfGtpUForUlDataForwarding
                | ThreeGppInterfaceType::N6MbNmb9
                | ThreeGppInterfaceType::N3Mb
                | ThreeGppInterfaceType::N19Mb
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
                | ThreeGppInterfaceType::S2aU
                | ThreeGppInterfaceType::S2bU
                | ThreeGppInterfaceType::ENodeBGtpUForDlDataForwarding
                | ThreeGppInterfaceType::ENodeBGtpUForUlDataForwarding
                | ThreeGppInterfaceType::SgwUpfGtpUForDlDataForwarding
                | ThreeGppInterfaceType::S8U
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
        vec![self.interface_type.to_u8()]
    }

    /// Unmarshals bytes into the IE
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.is_empty() {
            return Err(PfcpError::invalid_length(
                "3GPP Interface Type",
                IeType::TgppInterfaceType,
                1,
                0,
            ));
        }

        let interface_type = ThreeGppInterfaceType::from_u8(payload[0] & 0x3f)?;
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
        for value in 0..=31 {
            let interface_type = ThreeGppInterfaceType::from_u8(value).unwrap();
            let ie = ThreeGppInterfaceTypeIe::new(interface_type);
            let marshaled = ie.marshal();
            let unmarshaled = ThreeGppInterfaceTypeIe::unmarshal(&marshaled).unwrap();
            assert_eq!(ie, unmarshaled);
            assert_eq!(marshaled, [value]);
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
