//! RTP Header Extension Information IE (Grouped, IE Type 340).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.2-8, describes a single RTP header
//! extension to be used for PDU-set marking or timestamping. All children
//! are conditional or optional.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtpHeaderExtensionInformation {
    /// RTP header extension type (conditional).
    pub rtp_header_extension_type: Option<Ie>,
    /// RTP header extension ID / URI (conditional).
    pub rtp_header_extension_id: Option<Ie>,
    /// Additional configuration for the extension (optional).
    pub rtp_header_extension_additional_information: Option<Ie>,
}

impl RtpHeaderExtensionInformation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.rtp_header_extension_type {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.rtp_header_extension_id {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.rtp_header_extension_additional_information {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut rtp_header_extension_type = None;
        let mut rtp_header_extension_id = None;
        let mut rtp_header_extension_additional_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::RtpHeaderExtensionType => rtp_header_extension_type = Some(ie),
                IeType::RtpHeaderExtensionId => rtp_header_extension_id = Some(ie),
                IeType::RtpHeaderExtensionAdditionalInformation => {
                    rtp_header_extension_additional_information = Some(ie);
                }
                _ => (),
            }
        }

        Ok(Self {
            rtp_header_extension_type,
            rtp_header_extension_id,
            rtp_header_extension_additional_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpHeaderExtensionInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ext_type_ie() -> Ie {
        Ie::new(IeType::RtpHeaderExtensionType, vec![0x01])
    }

    fn make_ext_id_ie() -> Ie {
        // URI as UTF-8 bytes
        Ie::new(
            IeType::RtpHeaderExtensionId,
            b"urn:ietf:params:rtp-hdrext:sdes:mid".to_vec(),
        )
    }

    fn make_additional_ie() -> Ie {
        Ie::new(
            IeType::RtpHeaderExtensionAdditionalInformation,
            vec![0x01, 0x00],
        )
    }

    #[test]
    fn test_round_trip_empty() {
        let original = RtpHeaderExtensionInformation::new();
        let parsed = RtpHeaderExtensionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = RtpHeaderExtensionInformation {
            rtp_header_extension_type: Some(make_ext_type_ie()),
            rtp_header_extension_id: Some(make_ext_id_ie()),
            rtp_header_extension_additional_information: Some(make_additional_ie()),
        };
        let parsed = RtpHeaderExtensionInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = RtpHeaderExtensionInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::RtpHeaderExtensionInformation);
    }
}
