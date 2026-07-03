//! MT-EDT Control Information IE (IE Type 249).
//!
//! Per 3GPP TS 29.244 Section 8.2.172, a 1-octet flags field requesting
//! the UP function to report the sum of DL data packets size.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtedtControlInformation {
    /// RDSI: Reporting DL data packets Size Indication.
    /// Set to 1 to request the UP function to report the sum of DL data packets size.
    pub rdsi: bool,
}

impl MtedtControlInformation {
    pub fn new(rdsi: bool) -> Self {
        Self { rdsi }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![if self.rdsi { 0x01 } else { 0x00 }]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MT-EDT Control Information",
                IeType::MtedtControlInformation,
                1,
                0,
            ));
        }
        Ok(Self {
            rdsi: data[0] & 0x01 != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MtedtControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_rdsi_set() {
        let original = MtedtControlInformation::new(true);
        let parsed = MtedtControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_rdsi_clear() {
        let original = MtedtControlInformation::new(false);
        let parsed = MtedtControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_rdsi_bit_mask() {
        // Spare bits ignored on unmarshal
        let parsed = MtedtControlInformation::unmarshal(&[0xFF]).unwrap();
        assert!(parsed.rdsi);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            MtedtControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            MtedtControlInformation::new(true).to_ie().ie_type,
            IeType::MtedtControlInformation
        );
    }
}
