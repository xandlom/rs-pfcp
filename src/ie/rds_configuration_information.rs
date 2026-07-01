//! RDS Configuration Information IE (IE Type 262).
//!
//! Per 3GPP TS 29.244 Section 8.2.180, indicates whether the Reliable Data
//! Service (RDS) is requested to be applied (in request) or applied (in response).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// RDS Configuration Information per 3GPP TS 29.244 §8.2.180.
///
/// # Wire Format
/// - Byte 0: Spare (bits 7-1) | RDS (bit 0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RdsConfigurationInformation {
    /// RDS (Reliable Data Service): when true, RDS is requested/applied.
    pub rds: bool,
}

impl RdsConfigurationInformation {
    pub fn new(rds: bool) -> Self {
        Self { rds }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [if self.rds { 0x01 } else { 0x00 }]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "RDS Configuration Information",
                IeType::RdsConfigurationInformation,
                1,
                0,
            ));
        }
        Ok(Self {
            rds: data[0] & 0x01 != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RdsConfigurationInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_rds_true() {
        let original = RdsConfigurationInformation::new(true);
        let parsed = RdsConfigurationInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_rds_false() {
        let original = RdsConfigurationInformation::new(false);
        let parsed = RdsConfigurationInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_spare_bits_ignored() {
        let parsed = RdsConfigurationInformation::unmarshal(&[0xFE]).unwrap();
        assert!(!parsed.rds);
    }

    #[test]
    fn test_to_ie() {
        let ie = RdsConfigurationInformation::new(true).to_ie();
        assert_eq!(ie.ie_type, IeType::RdsConfigurationInformation);
    }

    #[test]
    fn test_empty_buffer() {
        assert!(matches!(
            RdsConfigurationInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }
}
