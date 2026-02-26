//! RTP Header Extension ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.235, carries the one-byte extension ID
//! per RFC 8285 (values 1–255).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// RTP Header Extension ID IE.
///
/// A one-byte extension identifier as defined in RFC 8285.
/// Valid values are 1–255 (0 is reserved in RFC 8285).
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.235
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtpHeaderExtensionId {
    pub value: u8,
}

impl RtpHeaderExtensionId {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "RTP Header Extension ID",
                IeType::RtpHeaderExtensionId,
                1,
                0,
            ));
        }
        Ok(Self { value: data[0] })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpHeaderExtensionId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = RtpHeaderExtensionId::new(0x0A);
        let parsed = RtpHeaderExtensionId::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [1u8, 128, 255] {
            let ie = RtpHeaderExtensionId::new(v);
            let parsed = RtpHeaderExtensionId::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RtpHeaderExtensionId::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RtpHeaderExtensionId::new(5).to_ie();
        assert_eq!(ie.ie_type, IeType::RtpHeaderExtensionId);
        assert_eq!(ie.payload, vec![0x05]);
    }
}
