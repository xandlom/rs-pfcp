//! Time Offset Threshold Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.147, carries a signed 64-bit nanosecond
//! threshold for TSN clock drift reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Time Offset Threshold IE.
///
/// Signed 64-bit value in nanoseconds used as a threshold for TSN
/// (Time-Sensitive Networking) time offset reporting per IEEE 802.1AS.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.147
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOffsetThreshold {
    /// Threshold in nanoseconds (signed)
    pub nanoseconds: i64,
}

impl TimeOffsetThreshold {
    pub fn new(nanoseconds: i64) -> Self {
        Self { nanoseconds }
    }

    pub fn marshal(&self) -> [u8; 8] {
        self.nanoseconds.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 8 {
            return Err(PfcpError::invalid_length(
                "Time Offset Threshold",
                IeType::TimeOffsetThreshold,
                8,
                data.len(),
            ));
        }
        Ok(Self {
            nanoseconds: i64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TimeOffsetThreshold, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = TimeOffsetThreshold::new(1_000_000_000); // 1 second in ns
        let parsed = TimeOffsetThreshold::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0i64, 1, -1, i64::MAX, i64::MIN, 500_000] {
            let ie = TimeOffsetThreshold::new(v);
            let parsed = TimeOffsetThreshold::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_marshal_bytes() {
        let ie = TimeOffsetThreshold::new(0);
        assert_eq!(ie.marshal(), [0u8; 8]);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            TimeOffsetThreshold::unmarshal(&[0x00; 7]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = TimeOffsetThreshold::new(100).to_ie();
        assert_eq!(ie.ie_type, IeType::TimeOffsetThreshold);
        assert_eq!(ie.payload.len(), 8);
    }
}
