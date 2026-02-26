//! Time Offset Measurement Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.149, carries a signed 64-bit nanosecond
//! measured time offset for TSN clock drift reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Time Offset Measurement IE.
///
/// Signed 64-bit value in nanoseconds representing the measured TSN
/// time offset per IEEE 802.1AS.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.149
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOffsetMeasurement {
    /// Measured time offset in nanoseconds (signed)
    pub nanoseconds: i64,
}

impl TimeOffsetMeasurement {
    pub fn new(nanoseconds: i64) -> Self {
        Self { nanoseconds }
    }

    pub fn marshal(&self) -> [u8; 8] {
        self.nanoseconds.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 8 {
            return Err(PfcpError::invalid_length(
                "Time Offset Measurement",
                IeType::TimeOffsetMeasurement,
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
        Ie::new(IeType::TimeOffsetMeasurement, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = TimeOffsetMeasurement::new(-500_000_000);
        let parsed = TimeOffsetMeasurement::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0i64, 1, -1, i64::MAX, i64::MIN] {
            let ie = TimeOffsetMeasurement::new(v);
            let parsed = TimeOffsetMeasurement::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            TimeOffsetMeasurement::unmarshal(&[0x00; 7]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = TimeOffsetMeasurement::new(0).to_ie();
        assert_eq!(ie.ie_type, IeType::TimeOffsetMeasurement);
        assert_eq!(ie.payload.len(), 8);
    }
}
