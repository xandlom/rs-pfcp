//! Protocol Description IE (Grouped, IE Type 334).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.2-7, describes the media transport
//! protocol, RTP header extensions, and RTP payload types for DL PDRs
//! that perform PDU-set marking or EDB marking (see clause 5.36).

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProtocolDescription {
    /// Media transport protocol (optional).
    pub media_transport_protocol: Option<Ie>,
    /// RTP header extension configuration (conditional).
    pub rtp_header_extension_information: Option<Ie>,
    /// RTP payload information entries (optional, multiple).
    pub rtp_payload_informations: Vec<Ie>,
}

impl ProtocolDescription {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.media_transport_protocol {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.rtp_header_extension_information {
            ies.push(v.clone());
        }
        for ie in &self.rtp_payload_informations {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut media_transport_protocol = None;
        let mut rtp_header_extension_information = None;
        let mut rtp_payload_informations = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MediaTransportProtocol => media_transport_protocol = Some(ie),
                IeType::RtpHeaderExtensionInformation => {
                    rtp_header_extension_information = Some(ie);
                }
                IeType::RtpPayloadInformation => rtp_payload_informations.push(ie),
                _ => (),
            }
        }

        Ok(Self {
            media_transport_protocol,
            rtp_header_extension_information,
            rtp_payload_informations,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ProtocolDescription, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_mtp_ie() -> Ie {
        Ie::new(IeType::MediaTransportProtocol, vec![0x01])
    }

    fn make_rtphei_ie() -> Ie {
        // Encode a child RtpHeaderExtensionType (342 = 0x0156) IE with 1-byte payload
        // TLV: type(2) + len(2) + value(1) = [0x01, 0x56, 0x00, 0x01, 0x01]
        Ie::new(
            IeType::RtpHeaderExtensionInformation,
            vec![0x01, 0x56, 0x00, 0x01, 0x01],
        )
    }

    fn make_rtppi_ie(pt: u8) -> Ie {
        Ie::new(IeType::RtpPayloadInformation, vec![pt])
    }

    #[test]
    fn test_round_trip_empty() {
        let original = ProtocolDescription::new();
        let parsed = ProtocolDescription::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = ProtocolDescription {
            media_transport_protocol: Some(make_mtp_ie()),
            rtp_header_extension_information: Some(make_rtphei_ie()),
            rtp_payload_informations: vec![make_rtppi_ie(96), make_rtppi_ie(97)],
        };
        let parsed = ProtocolDescription::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = ProtocolDescription::new().to_ie();
        assert_eq!(ie.ie_type, IeType::ProtocolDescription);
    }
}
