//! MBS Session N4mb Control Information IE (IE Type 300).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.1-4, used in PFCP Session Establishment
//! Request to provide N4mb control information for MBS sessions.

use crate::error::PfcpError;
use crate::ie::mbs_session_identifier::MbsSessionIdentifier;
use crate::ie::mbsn4mb_req_flags::Mbsn4mbReqFlags;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// MBS Session N4mb Control Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionN4mbControlInformation {
    pub mbs_session_identifier: MbsSessionIdentifier, // M
    pub area_session_id: Option<Ie>,                  // C - IE 314
    pub mbsn4mb_req_flags: Option<Mbsn4mbReqFlags>,   // C - IE 307
    pub multicast_transport_information: Option<Ie>,  // C - IE 306
}

impl MbsSessionN4mbControlInformation {
    pub fn new(mbs_session_identifier: MbsSessionIdentifier) -> Self {
        Self {
            mbs_session_identifier,
            area_session_id: None,
            mbsn4mb_req_flags: None,
            multicast_transport_information: None,
        }
    }

    pub fn with_area_session_id(mut self, ie: Ie) -> Self {
        self.area_session_id = Some(ie);
        self
    }

    pub fn with_mbsn4mb_req_flags(mut self, flags: Mbsn4mbReqFlags) -> Self {
        self.mbsn4mb_req_flags = Some(flags);
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
        if let Some(flags) = self.mbsn4mb_req_flags {
            ies.push(flags.to_ie());
        }
        if let Some(ref ie) = self.multicast_transport_information {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mbs_session_identifier_ie = None;
        let mut area_session_id = None;
        let mut mbsn4mb_req_flags = None;
        let mut multicast_transport_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MbsSessionIdentifier => mbs_session_identifier_ie = Some(ie),
                IeType::AreaSessionId => area_session_id = Some(ie),
                IeType::Mbsn4mbReqFlags => {
                    mbsn4mb_req_flags = Some(Mbsn4mbReqFlags::unmarshal(&ie.payload)?)
                }
                IeType::MulticastTransportInformation => multicast_transport_information = Some(ie),
                _ => {}
            }
        }

        let id_ie = mbs_session_identifier_ie.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::MbsSessionIdentifier,
                IeType::MbsSessionN4mbControlInformation,
            )
        })?;
        let mbs_session_identifier = MbsSessionIdentifier::unmarshal(&id_ie.payload)?;

        Ok(Self {
            mbs_session_identifier,
            area_session_id,
            mbsn4mb_req_flags,
            multicast_transport_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsSessionN4mbControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MbsSessionN4mbControlInformation::new(MbsSessionIdentifier::new_tmgi([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        ]))
        .with_mbsn4mb_req_flags(Mbsn4mbReqFlags::PLLSSM);

        let ie = original.to_ie();
        let parsed = MbsSessionN4mbControlInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let result = MbsSessionN4mbControlInformation::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie =
            MbsSessionN4mbControlInformation::new(MbsSessionIdentifier::new_tmgi([0; 6])).to_ie();
        assert_eq!(ie.ie_type, IeType::MbsSessionN4mbControlInformation);
    }
}
