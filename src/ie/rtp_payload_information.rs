//! RTP Payload Information IE (Grouped, IE Type 341).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.2-9, describes RTP payload type(s) and
//! optional format information for a PDR that performs PDU-set marking.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtpPayloadInformation {
    /// RTP payload types to match (conditional, multiple).
    pub rtp_payload_types: Vec<Ie>,
    /// RTP payload format (optional).
    pub rtp_payload_format: Option<Ie>,
}

impl RtpPayloadInformation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        for pt in &self.rtp_payload_types {
            ies.push(pt.clone());
        }
        if let Some(ref v) = self.rtp_payload_format {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut rtp_payload_types = Vec::new();
        let mut rtp_payload_format = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::RtpPayloadType => rtp_payload_types.push(ie),
                IeType::RtpPayloadFormat => rtp_payload_format = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            rtp_payload_types,
            rtp_payload_format,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RtpPayloadInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_payload_type_ie(pt: u8) -> Ie {
        Ie::new(IeType::RtpPayloadType, vec![pt])
    }

    fn make_payload_format_ie() -> Ie {
        Ie::new(IeType::RtpPayloadFormat, vec![0x60])
    }

    #[test]
    fn test_round_trip_empty() {
        let original = RtpPayloadInformation::new();
        let parsed = RtpPayloadInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_multiple_types() {
        let mut original = RtpPayloadInformation::new();
        original.rtp_payload_types.push(make_payload_type_ie(96));
        original.rtp_payload_types.push(make_payload_type_ie(97));
        let parsed = RtpPayloadInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = RtpPayloadInformation {
            rtp_payload_types: vec![make_payload_type_ie(96)],
            rtp_payload_format: Some(make_payload_format_ie()),
        };
        let parsed = RtpPayloadInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = RtpPayloadInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::RtpPayloadInformation);
    }
}
