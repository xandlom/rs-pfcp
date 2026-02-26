//! RTP Header Extension Type Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.234, identifies the type of RTP header
//! extension (e.g., PDU Set Marking per RFC 9624).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// RTP Header Extension Type IE.
///
/// Identifies the RTP header extension type. Value 1 indicates
/// PDU Set Marking as defined in RFC 9624.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.234
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtpHeaderExtensionType {
    pub value: u8,
}

impl RtpHeaderExtensionType {
    /// PDU Set Marking extension type (RFC 9624)
    pub const PDU_SET_MARKING: u8 = 1;

    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "RTP Header Extension Type",
                IeType::RtpHeaderExtensionType,
                1,
                0,
            ));
        }
        Ok(Self { value: data[0] })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpHeaderExtensionType, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = RtpHeaderExtensionType::new(RtpHeaderExtensionType::PDU_SET_MARKING);
        let parsed = RtpHeaderExtensionType::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0u8, 1, 127, 255] {
            let ie = RtpHeaderExtensionType::new(v);
            let parsed = RtpHeaderExtensionType::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RtpHeaderExtensionType::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RtpHeaderExtensionType::new(1).to_ie();
        assert_eq!(ie.ie_type, IeType::RtpHeaderExtensionType);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
