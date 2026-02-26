//! MBS Unicast Parameters ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.210, identifies a set of MBS unicast
//! parameters. Encoded as a 16-bit unsigned integer.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// MBS Unicast Parameters ID IE.
///
/// Identifies a set of MBS (Multicast/Broadcast Service) unicast parameters
/// within an N4 session.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.210
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MbsUnicastParametersId {
    pub value: u16,
}

impl MbsUnicastParametersId {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "MBS Unicast Parameters ID",
                IeType::MbsUnicastParametersId,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes([data[0], data[1]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsUnicastParametersId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = MbsUnicastParametersId::new(0x0001);
        let parsed = MbsUnicastParametersId::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0u16, 1, 0xFF, 0xFFFF] {
            let ie = MbsUnicastParametersId::new(v);
            let parsed = MbsUnicastParametersId::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            MbsUnicastParametersId::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MbsUnicastParametersId::new(42).to_ie();
        assert_eq!(ie.ie_type, IeType::MbsUnicastParametersId);
        assert_eq!(ie.payload.len(), 2);
    }
}
