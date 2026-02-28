//! Traffic Parameter Threshold Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.220, specifies the DL jitter threshold
//! for traffic parameter monitoring.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Traffic Parameter Threshold per 3GPP TS 29.244 §8.2.220.
///
/// # Wire Format
/// - Byte 0: flags (DL=0x01)
/// - If DL: u32 DL jitter threshold (ms)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrafficParameterThreshold {
    /// DL jitter threshold in milliseconds, present when DL flag is set.
    pub dl_threshold: Option<u32>,
}

impl TrafficParameterThreshold {
    pub fn new(dl_threshold: Option<u32>) -> Self {
        Self { dl_threshold }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let flags = if self.dl_threshold.is_some() {
            0x01u8
        } else {
            0x00u8
        };
        let mut data = vec![flags];
        if let Some(v) = self.dl_threshold {
            data.extend_from_slice(&v.to_be_bytes());
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Traffic Parameter Threshold",
                IeType::TrafficParameterThreshold,
                1,
                0,
            ));
        }
        let flags = data[0];
        let dl_flag = (flags & 0x01) != 0;
        if dl_flag {
            if data.len() < 5 {
                return Err(PfcpError::invalid_length(
                    "Traffic Parameter Threshold",
                    IeType::TrafficParameterThreshold,
                    5,
                    data.len(),
                ));
            }
            let v = u32::from_be_bytes(data[1..5].try_into().unwrap());
            Ok(Self {
                dl_threshold: Some(v),
            })
        } else {
            Ok(Self { dl_threshold: None })
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TrafficParameterThreshold, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_with_dl() {
        let ie = TrafficParameterThreshold::new(Some(5000));
        let parsed = TrafficParameterThreshold::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_no_dl() {
        let ie = TrafficParameterThreshold::new(None);
        let parsed = TrafficParameterThreshold::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_byte_layout() {
        let ie = TrafficParameterThreshold::new(Some(0x00000100));
        let data = ie.marshal();
        assert_eq!(data[0], 0x01);
        assert_eq!(data[1..5], [0x00, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TrafficParameterThreshold::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short_with_flag() {
        assert!(matches!(
            TrafficParameterThreshold::unmarshal(&[0x01, 0x00]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = TrafficParameterThreshold::new(Some(1000)).to_ie();
        assert_eq!(ie.ie_type, IeType::TrafficParameterThreshold);
        assert_eq!(ie.payload.len(), 5);
    }
}
