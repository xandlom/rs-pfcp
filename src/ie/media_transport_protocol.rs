//! Media Transport Protocol Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.233, indicates the media transport protocol
//! used for an RTP session.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Media transport protocol values per 3GPP TS 29.244 Section 8.2.233.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTransportProtocol {
    /// Unspecified
    Unspecified,
    /// RTP (Real-time Transport Protocol)
    Rtp,
    /// SRTP (Secure RTP)
    Srtp,
    /// Unknown/reserved value
    Unknown(u8),
}

impl MediaTransportProtocol {
    fn to_byte(self) -> u8 {
        match self {
            MediaTransportProtocol::Unspecified => 0,
            MediaTransportProtocol::Rtp => 1,
            MediaTransportProtocol::Srtp => 2,
            MediaTransportProtocol::Unknown(v) => v,
        }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.to_byte()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Media Transport Protocol",
                IeType::MediaTransportProtocol,
                1,
                0,
            ));
        }
        Ok(match data[0] {
            0 => MediaTransportProtocol::Unspecified,
            1 => MediaTransportProtocol::Rtp,
            2 => MediaTransportProtocol::Srtp,
            v => MediaTransportProtocol::Unknown(v),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MediaTransportProtocol, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for proto in [
            MediaTransportProtocol::Unspecified,
            MediaTransportProtocol::Rtp,
            MediaTransportProtocol::Srtp,
        ] {
            let parsed = MediaTransportProtocol::unmarshal(&proto.marshal()).unwrap();
            assert_eq!(parsed, proto);
        }
    }

    #[test]
    fn test_unknown_value() {
        let parsed = MediaTransportProtocol::unmarshal(&[0x0F]).unwrap();
        assert_eq!(parsed, MediaTransportProtocol::Unknown(0x0F));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MediaTransportProtocol::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MediaTransportProtocol::Rtp.to_ie();
        assert_eq!(ie.ie_type, IeType::MediaTransportProtocol);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
