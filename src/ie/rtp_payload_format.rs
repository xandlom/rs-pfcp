//! RTP Payload Format Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.237, indicates the video codec format
//! used for RTP payload.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// RTP payload format values per 3GPP TS 29.244 Section 8.2.237.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtpPayloadFormat {
    /// H.264 (AVC) video codec
    H264,
    /// H.265 (HEVC) video codec
    H265,
    /// Unknown/reserved value
    Unknown(u8),
}

impl RtpPayloadFormat {
    fn to_byte(self) -> u8 {
        match self {
            RtpPayloadFormat::H264 => 1,
            RtpPayloadFormat::H265 => 2,
            RtpPayloadFormat::Unknown(v) => v,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.to_byte()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "RTP Payload Format",
                IeType::RtpPayloadFormat,
                1,
                0,
            ));
        }
        Ok(match data[0] {
            1 => RtpPayloadFormat::H264,
            2 => RtpPayloadFormat::H265,
            v => RtpPayloadFormat::Unknown(v),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpPayloadFormat, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for fmt in [RtpPayloadFormat::H264, RtpPayloadFormat::H265] {
            let parsed = RtpPayloadFormat::unmarshal(&fmt.marshal()).unwrap();
            assert_eq!(parsed, fmt);
        }
    }

    #[test]
    fn test_unknown_value() {
        let parsed = RtpPayloadFormat::unmarshal(&[0x0F]).unwrap();
        assert_eq!(parsed, RtpPayloadFormat::Unknown(0x0F));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RtpPayloadFormat::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = RtpPayloadFormat::H264.to_ie();
        assert_eq!(ie.ie_type, IeType::RtpPayloadFormat);
        assert_eq!(ie.payload, vec![0x01]);
    }

    #[test]
    fn test_h265_bytes() {
        let ie = RtpPayloadFormat::H265.to_ie();
        assert_eq!(ie.payload, vec![0x02]);
    }
}
