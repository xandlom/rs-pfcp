//! RTP Payload Type Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.236, carries the 7-bit RTP payload type
//! per RFC 3550 (values 0–127).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// RTP Payload Type IE.
///
/// A 7-bit RTP payload type value as defined in RFC 3550.
/// Valid values are 0–127.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.236
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtpPayloadType {
    pub value: u8,
}

impl RtpPayloadType {
    /// Maximum valid RTP payload type value (7-bit field)
    pub const MAX: u8 = 127;

    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value & 0x7F]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "RTP Payload Type",
                IeType::RtpPayloadType,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data[0] & 0x7F,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpPayloadType, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = RtpPayloadType::new(96); // Dynamic payload type
        let parsed = RtpPayloadType::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0u8, 1, 63, 96, 127] {
            let ie = RtpPayloadType::new(v);
            let parsed = RtpPayloadType::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed.value, v);
        }
    }

    #[test]
    fn test_marshal_masks_high_bit() {
        // 7-bit masking
        let ie = RtpPayloadType::new(96);
        assert_eq!(ie.marshal()[0], 96);
    }

    #[test]
    fn test_unmarshal_masks_high_bit() {
        // High bit should be stripped during unmarshal
        let parsed = RtpPayloadType::unmarshal(&[0x80 | 96]).unwrap();
        assert_eq!(parsed.value, 96);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RtpPayloadType::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RtpPayloadType::new(0).to_ie();
        assert_eq!(ie.ie_type, IeType::RtpPayloadType);
    }
}
