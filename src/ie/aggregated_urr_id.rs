//! Aggregated URR ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.85, identifies a URR that is aggregated
//! across multiple sessions. Encoded as a 32-bit unsigned integer.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Aggregated URR ID IE.
///
/// Identifies a URR for cross-session usage reporting aggregation.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.85
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregatedUrrId {
    pub value: u32,
}

impl AggregatedUrrId {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Aggregated URR ID",
                IeType::AggregatedUrrId,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AggregatedUrrId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = AggregatedUrrId::new(0x12345678);
        let parsed = AggregatedUrrId::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0u32, 1, 0xFFFF, 0xFFFFFFFF] {
            let ie = AggregatedUrrId::new(v);
            let parsed = AggregatedUrrId::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            AggregatedUrrId::unmarshal(&[0x01, 0x02, 0x03]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            AggregatedUrrId::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AggregatedUrrId::new(1).to_ie();
        assert_eq!(ie.ie_type, IeType::AggregatedUrrId);
        assert_eq!(ie.payload.len(), 4);
    }
}
