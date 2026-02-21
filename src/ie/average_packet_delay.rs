//! Average Packet Delay Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.163, contains the average packet delay
//! in microseconds for QoS monitoring.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AveragePacketDelay {
    pub delay_us: u32,
}

impl AveragePacketDelay {
    pub fn new(delay_us: u32) -> Self {
        Self { delay_us }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.delay_us.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Average Packet Delay",
                IeType::AveragePacketDelay,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            delay_us: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AveragePacketDelay, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = AveragePacketDelay::new(5000);
        let parsed = AveragePacketDelay::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 1000, 50000, u32::MAX] {
            let ie = AveragePacketDelay::new(v);
            let parsed = AveragePacketDelay::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            AveragePacketDelay::unmarshal(&[0; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            AveragePacketDelay::new(100).to_ie().ie_type,
            IeType::AveragePacketDelay
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(
            AveragePacketDelay::new(0x12345678).marshal(),
            [0x12, 0x34, 0x56, 0x78]
        );
    }
}
