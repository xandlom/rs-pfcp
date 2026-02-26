//! Minimum Wait Time Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.170, carries the minimum wait time in
//! seconds between two consecutive QoS monitoring reports.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Minimum Wait Time IE.
///
/// Specifies the minimum number of seconds that must elapse between two
/// consecutive QoS monitoring reports.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.170
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinimumWaitTime {
    /// Wait time in seconds
    pub seconds: u32,
}

impl MinimumWaitTime {
    pub fn new(seconds: u32) -> Self {
        Self { seconds }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.seconds.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Minimum Wait Time",
                IeType::MinimumWaitTime,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            seconds: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MinimumWaitTime, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = MinimumWaitTime::new(3600);
        let parsed = MinimumWaitTime::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0u32, 1, 60, 3600, u32::MAX] {
            let ie = MinimumWaitTime::new(v);
            let parsed = MinimumWaitTime::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            MinimumWaitTime::unmarshal(&[0x00; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MinimumWaitTime::new(10).to_ie();
        assert_eq!(ie.ie_type, IeType::MinimumWaitTime);
        assert_eq!(ie.payload.len(), 4);
    }
}
