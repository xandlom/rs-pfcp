//! MBS Session N4 Control Information IE (IE Type 310).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.1-5, used in PFCP Session Establishment
//! Request and Session Modification Request to associate a PDU session with
//! an MBS session.

use crate::error::PfcpError;
use crate::ie::mbs_session_identifier::MbsSessionIdentifier;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// MBS Session N4 Control Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionN4ControlInformation {
    pub mbs_session_identifier: MbsSessionIdentifier, // M
    pub area_session_id: Option<Ie>,                  // C - IE 314
    pub multicast_transport_information: Option<Ie>,  // C - IE 306
}

impl MbsSessionN4ControlInformation {
    pub fn new(mbs_session_identifier: MbsSessionIdentifier) -> Self {
        Self {
            mbs_session_identifier,
            area_session_id: None,
            multicast_transport_information: None,
        }
    }

    pub fn with_area_session_id(mut self, ie: Ie) -> Self {
        self.area_session_id = Some(ie);
        self
    }

    pub fn with_multicast_transport_information(mut self, ie: Ie) -> Self {
        self.multicast_transport_information = Some(ie);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.mbs_session_identifier.to_ie()];
        if let Some(ref ie) = self.area_session_id {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.multicast_transport_information {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mbs_session_identifier_ie = None;
        let mut area_session_id = None;
        let mut multicast_transport_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MbsSessionIdentifier => mbs_session_identifier_ie = Some(ie),
                IeType::AreaSessionId => area_session_id = Some(ie),
                IeType::MulticastTransportInformation => multicast_transport_information = Some(ie),
                _ => {}
            }
        }

        let id_ie = mbs_session_identifier_ie.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::MbsSessionIdentifier,
                IeType::MbsSessionN4ControlInformation,
            )
        })?;
        let mbs_session_identifier = MbsSessionIdentifier::unmarshal(&id_ie.payload)?;

        Ok(Self {
            mbs_session_identifier,
            area_session_id,
            multicast_transport_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsSessionN4ControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MbsSessionN4ControlInformation::new(MbsSessionIdentifier::new_tmgi([
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        ]));
        let ie = original.to_ie();
        let parsed = MbsSessionN4ControlInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let result = MbsSessionN4ControlInformation::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie =
            MbsSessionN4ControlInformation::new(MbsSessionIdentifier::new_tmgi([0; 6])).to_ie();
        assert_eq!(ie.ie_type, IeType::MbsSessionN4ControlInformation);
    }
}
