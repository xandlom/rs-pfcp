//! RTP Header Extension Additional Information IE (IE Type 349).
//!
//! Per 3GPP TS 29.244, contains additional information about an RTP header extension,
//! including the FI (Format Indicator) and PSSAI (PDU Set Secondary Action Indicator) flags,
//! and an optional format byte when FI is set.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtpHeaderExtensionAdditionalInformation {
    /// FI: Format Indicator — when set, the format field is significant.
    pub fi: bool,
    /// PSSAI: PDU Set Secondary Action Indicator.
    pub pssai: bool,
    /// Format byte (significant when FI = 1).
    pub format: Option<u8>,
}

impl RtpHeaderExtensionAdditionalInformation {
    pub fn new(fi: bool, pssai: bool, format: Option<u8>) -> Self {
        Self { fi, pssai, format }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let flags = (if self.fi { 0x01 } else { 0x00 }) | (if self.pssai { 0x02 } else { 0x00 });
        vec![flags, self.format.unwrap_or(0)]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "RTP Header Extension Additional Information",
                IeType::RtpHeaderExtensionAdditionalInformation,
                2,
                data.len(),
            ));
        }
        let fi = data[0] & 0x01 != 0;
        let pssai = data[0] & 0x02 != 0;
        let format = if fi { Some(data[1]) } else { None };
        Ok(Self { fi, pssai, format })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::RtpHeaderExtensionAdditionalInformation,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_fi_and_pssai() {
        let original = RtpHeaderExtensionAdditionalInformation::new(true, true, Some(0x42));
        let parsed =
            RtpHeaderExtensionAdditionalInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_fi_only() {
        let original = RtpHeaderExtensionAdditionalInformation::new(true, false, Some(0x10));
        let parsed =
            RtpHeaderExtensionAdditionalInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_pssai_only() {
        let original = RtpHeaderExtensionAdditionalInformation::new(false, true, None);
        let parsed =
            RtpHeaderExtensionAdditionalInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_neither() {
        let original = RtpHeaderExtensionAdditionalInformation::new(false, false, None);
        let parsed =
            RtpHeaderExtensionAdditionalInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            RtpHeaderExtensionAdditionalInformation::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
        assert!(matches!(
            RtpHeaderExtensionAdditionalInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            RtpHeaderExtensionAdditionalInformation::new(false, false, None)
                .to_ie()
                .ie_type,
            IeType::RtpHeaderExtensionAdditionalInformation
        );
    }
}
