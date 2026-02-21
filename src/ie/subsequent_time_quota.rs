//! Subsequent Time Quota Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.61, contains the subsequent time quota
//! in seconds for usage reporting after the initial quota is exhausted.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubsequentTimeQuota {
    pub value: u32,
}

impl SubsequentTimeQuota {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Subsequent Time Quota",
                IeType::SubsequentTimeQuota,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SubsequentTimeQuota, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = SubsequentTimeQuota::new(7200);
        let data = ie.marshal();
        let parsed = SubsequentTimeQuota::unmarshal(&data).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 60, 3600, 86400, u32::MAX] {
            let ie = SubsequentTimeQuota::new(v);
            let parsed = SubsequentTimeQuota::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            SubsequentTimeQuota::unmarshal(&[0; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            SubsequentTimeQuota::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            SubsequentTimeQuota::new(60).to_ie().ie_type,
            IeType::SubsequentTimeQuota
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(
            SubsequentTimeQuota::new(0x12345678).marshal(),
            [0x12, 0x34, 0x56, 0x78]
        );
    }
}
