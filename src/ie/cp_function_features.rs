// src/ie/cp_function_features.rs

//! CP Function Features Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct CPFunctionFeatures: u8 {
        const LOAD = 1 << 0; // Bit 1
        const OVRL = 1 << 1; // Bit 2
        const EPCO = 1 << 2; // Bit 3
        const DDEX = 1 << 3; // Bit 4
        const PFDL = 1 << 4; // Bit 5
        const APDP = 1 << 5; // Bit 6
        const PFDC = 1 << 6; // Bit 7
    }
}

impl CPFunctionFeatures {
    pub fn new(features: u8) -> Self {
        CPFunctionFeatures::from_bits_truncate(features)
    }

    pub fn marshal(&self) -> [u8; 1] {
        self.bits().to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "CP Function Features",
                IeType::CpFunctionFeatures,
                1,
                0,
            ));
        }
        Ok(CPFunctionFeatures::from_bits_truncate(data[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp_function_features_marshal_unmarshal() {
        let features = CPFunctionFeatures::LOAD | CPFunctionFeatures::EPCO;
        let marshaled = features.marshal();
        let unmarshaled = CPFunctionFeatures::unmarshal(&marshaled).unwrap();
        assert_eq!(features, unmarshaled);
    }

    #[test]
    fn test_cp_function_features_unmarshal_empty() {
        let result = CPFunctionFeatures::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("CP Function Features"));
    }
}
