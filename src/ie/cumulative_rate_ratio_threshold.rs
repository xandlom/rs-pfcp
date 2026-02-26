//! Cumulative Rate Ratio Threshold Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.148, carries a signed 32-bit IEEE 802.1AS
//! cumulative rate ratio threshold for TSN clock drift reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Cumulative Rate Ratio Threshold IE.
///
/// Signed 32-bit value representing an IEEE 802.1AS cumulative rate ratio
/// used as a threshold for TSN clock drift reporting.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.148
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CumulativeRateRatioThreshold {
    pub value: i32,
}

impl CumulativeRateRatioThreshold {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Cumulative Rate Ratio Threshold",
                IeType::CumulativeRateRatioThreshold,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: i32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::CumulativeRateRatioThreshold,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = CumulativeRateRatioThreshold::new(1000);
        let parsed = CumulativeRateRatioThreshold::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0i32, 1, -1, i32::MAX, i32::MIN] {
            let ie = CumulativeRateRatioThreshold::new(v);
            let parsed = CumulativeRateRatioThreshold::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            CumulativeRateRatioThreshold::unmarshal(&[0x00; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = CumulativeRateRatioThreshold::new(0).to_ie();
        assert_eq!(ie.ie_type, IeType::CumulativeRateRatioThreshold);
        assert_eq!(ie.payload.len(), 4);
    }
}
