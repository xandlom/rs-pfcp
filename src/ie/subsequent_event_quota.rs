//! Subsequent Event Quota Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.114, contains the subsequent event quota
//! for event-based reporting after the initial quota is exhausted.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubsequentEventQuota {
    pub value: u32,
}

impl SubsequentEventQuota {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Subsequent Event Quota",
                IeType::SubsequentEventQuota,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SubsequentEventQuota, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = SubsequentEventQuota::new(200);
        let parsed = SubsequentEventQuota::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 100, 1000, u32::MAX] {
            let ie = SubsequentEventQuota::new(v);
            let parsed = SubsequentEventQuota::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            SubsequentEventQuota::unmarshal(&[0; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            SubsequentEventQuota::new(10).to_ie().ie_type,
            IeType::SubsequentEventQuota
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(
            SubsequentEventQuota::new(0x12345678).marshal(),
            [0x12, 0x34, 0x56, 0x78]
        );
    }
}
