//! N6 Jitter Measurement Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.222, contains downlink jitter measurement data.
//! Structure: flags byte + conditional DL periodicity (u32 ms) + lower jitter (i32 ms) + higher jitter (i32 ms).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// N6 Jitter Measurement per 3GPP TS 29.244 Section 8.2.222.
///
/// When the DL flag is set, three measurement fields are present:
/// - DL Periodicity (milliseconds)
/// - Lower DL Jitter Measurement (signed, milliseconds)
/// - Higher DL Jitter Measurement (signed, milliseconds)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N6JitterMeasurement {
    /// DL periodicity in milliseconds, present when DL flag is set.
    pub dl_periodicity: Option<u32>,
    /// Lower DL jitter measurement in milliseconds (signed).
    pub lower_dl_jitter: Option<i32>,
    /// Higher DL jitter measurement in milliseconds (signed).
    pub higher_dl_jitter: Option<i32>,
}

impl N6JitterMeasurement {
    /// Create with DL measurements.
    pub fn new(dl_periodicity: u32, lower_dl_jitter: i32, higher_dl_jitter: i32) -> Self {
        Self {
            dl_periodicity: Some(dl_periodicity),
            lower_dl_jitter: Some(lower_dl_jitter),
            higher_dl_jitter: Some(higher_dl_jitter),
        }
    }

    /// Create with no DL measurements (DL flag not set).
    pub fn empty() -> Self {
        Self {
            dl_periodicity: None,
            lower_dl_jitter: None,
            higher_dl_jitter: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        if let (Some(periodicity), Some(lower), Some(higher)) = (
            self.dl_periodicity,
            self.lower_dl_jitter,
            self.higher_dl_jitter,
        ) {
            data.push(0x01); // DL flag
            data.extend_from_slice(&periodicity.to_be_bytes());
            data.extend_from_slice(&lower.to_be_bytes());
            data.extend_from_slice(&higher.to_be_bytes());
        } else {
            data.push(0x00); // No DL flag
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "N6 Jitter Measurement",
                IeType::N6JitterMeasurement,
                1,
                0,
            ));
        }

        let flags = data[0];
        if flags & 0x01 != 0 {
            // DL flag set - need 1 + 4 + 4 + 4 = 13 bytes
            if data.len() < 13 {
                return Err(PfcpError::invalid_length(
                    "N6 Jitter Measurement",
                    IeType::N6JitterMeasurement,
                    13,
                    data.len(),
                ));
            }
            let dl_periodicity = u32::from_be_bytes(data[1..5].try_into().unwrap());
            let lower_dl_jitter = i32::from_be_bytes(data[5..9].try_into().unwrap());
            let higher_dl_jitter = i32::from_be_bytes(data[9..13].try_into().unwrap());
            Ok(Self {
                dl_periodicity: Some(dl_periodicity),
                lower_dl_jitter: Some(lower_dl_jitter),
                higher_dl_jitter: Some(higher_dl_jitter),
            })
        } else {
            Ok(Self::empty())
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::N6JitterMeasurement, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_with_dl() {
        let jitter = N6JitterMeasurement::new(1000, -50, 100);
        let parsed = N6JitterMeasurement::unmarshal(&jitter.marshal()).unwrap();
        assert_eq!(parsed, jitter);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let jitter = N6JitterMeasurement::empty();
        let parsed = N6JitterMeasurement::unmarshal(&jitter.marshal()).unwrap();
        assert_eq!(parsed, jitter);
    }

    #[test]
    fn test_marshal_dl_flag_set() {
        let jitter = N6JitterMeasurement::new(500, -10, 20);
        let data = jitter.marshal();
        assert_eq!(data[0] & 0x01, 0x01);
        assert_eq!(data.len(), 13);
    }

    #[test]
    fn test_marshal_no_dl_flag() {
        let jitter = N6JitterMeasurement::empty();
        let data = jitter.marshal();
        assert_eq!(data[0], 0x00);
        assert_eq!(data.len(), 1);
    }

    #[test]
    fn test_negative_jitter_values() {
        let jitter = N6JitterMeasurement::new(2000, -1000, -500);
        let parsed = N6JitterMeasurement::unmarshal(&jitter.marshal()).unwrap();
        assert_eq!(parsed.lower_dl_jitter, Some(-1000));
        assert_eq!(parsed.higher_dl_jitter, Some(-500));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            N6JitterMeasurement::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_dl_flag() {
        assert!(matches!(
            N6JitterMeasurement::unmarshal(&[0x01, 0x00]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            N6JitterMeasurement::new(100, 0, 0).to_ie().ie_type,
            IeType::N6JitterMeasurement
        );
    }
}
