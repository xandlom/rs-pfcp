//! Steering Mode Indicator IE (IE Type 289).
//!
//! Per 3GPP TS 29.244 Section 8.2.197, a 1-octet flags field indicating
//! steering mode for a MAR rule. Bit 0 (ALBI) = Active/Last Bearer Indicator,
//! bit 1 (UEAI) = UE Assistance Information Indicator.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringModeIndicator {
    /// ALBI: Active/Last Bearer Indicator.
    pub albi: bool,
    /// UEAI: UE Assistance Information Indicator.
    pub ueai: bool,
}

impl SteeringModeIndicator {
    pub fn new(albi: bool, ueai: bool) -> Self {
        Self { albi, ueai }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let byte = (if self.albi { 0x01 } else { 0x00 }) | (if self.ueai { 0x02 } else { 0x00 });
        vec![byte]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Steering Mode Indicator",
                IeType::SteeringModeIndicator,
                1,
                0,
            ));
        }
        Ok(Self {
            albi: data[0] & 0x01 != 0,
            ueai: data[0] & 0x02 != 0,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SteeringModeIndicator, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_both_set() {
        let original = SteeringModeIndicator::new(true, true);
        let parsed = SteeringModeIndicator::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_albi_only() {
        let original = SteeringModeIndicator::new(true, false);
        let parsed = SteeringModeIndicator::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_ueai_only() {
        let original = SteeringModeIndicator::new(false, true);
        let parsed = SteeringModeIndicator::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_none() {
        let original = SteeringModeIndicator::new(false, false);
        let parsed = SteeringModeIndicator::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_spare_bits_ignored() {
        let parsed = SteeringModeIndicator::unmarshal(&[0xFF]).unwrap();
        assert!(parsed.albi);
        assert!(parsed.ueai);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            SteeringModeIndicator::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            SteeringModeIndicator::new(true, false).to_ie().ie_type,
            IeType::SteeringModeIndicator
        );
    }
}
