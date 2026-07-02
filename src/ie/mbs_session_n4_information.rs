//! MBS Session N4 Information IE (IE Type 311).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.1-5, used in PFCP Session Establishment
//! Response to provide the allocated N19mb DL TEID and MBS response information.

use crate::error::PfcpError;
use crate::ie::mbs_session_identifier::MbsSessionIdentifier;
use crate::ie::mbsn4_resp_flags::Mbsn4RespFlags;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// MBS Session N4 Information grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbsSessionN4Information {
    pub mbs_session_identifier: MbsSessionIdentifier, // M
    pub area_session_id: Option<Ie>,                  // C - IE 314
    pub fteid: Option<Ie>,                            // C - N19mb DL Tunnel ID, IE 21
    pub mbsn4_resp_flags: Option<Mbsn4RespFlags>,     // C - IE 312
}

impl MbsSessionN4Information {
    pub fn new(mbs_session_identifier: MbsSessionIdentifier) -> Self {
        Self {
            mbs_session_identifier,
            area_session_id: None,
            fteid: None,
            mbsn4_resp_flags: None,
        }
    }

    pub fn with_area_session_id(mut self, ie: Ie) -> Self {
        self.area_session_id = Some(ie);
        self
    }

    pub fn with_fteid(mut self, ie: Ie) -> Self {
        self.fteid = Some(ie);
        self
    }

    pub fn with_mbsn4_resp_flags(mut self, flags: Mbsn4RespFlags) -> Self {
        self.mbsn4_resp_flags = Some(flags);
        self
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.mbs_session_identifier.to_ie()];
        if let Some(ref ie) = self.area_session_id {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.fteid {
            ies.push(ie.clone());
        }
        if let Some(flags) = self.mbsn4_resp_flags {
            ies.push(flags.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mbs_session_identifier_ie = None;
        let mut area_session_id = None;
        let mut fteid = None;
        let mut mbsn4_resp_flags = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MbsSessionIdentifier => mbs_session_identifier_ie = Some(ie),
                IeType::AreaSessionId => area_session_id = Some(ie),
                IeType::Fteid => fteid = Some(ie),
                IeType::Mbsn4RespFlags => {
                    mbsn4_resp_flags = Some(Mbsn4RespFlags::unmarshal(&ie.payload)?)
                }
                _ => {}
            }
        }

        let id_ie = mbs_session_identifier_ie.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::MbsSessionIdentifier,
                IeType::MbsSessionN4Information,
            )
        })?;
        let mbs_session_identifier = MbsSessionIdentifier::unmarshal(&id_ie.payload)?;

        Ok(Self {
            mbs_session_identifier,
            area_session_id,
            fteid,
            mbsn4_resp_flags,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MbsSessionN4Information, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = MbsSessionN4Information::new(MbsSessionIdentifier::new_tmgi([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        ]))
        .with_mbsn4_resp_flags(Mbsn4RespFlags::NN19DT | Mbsn4RespFlags::JMTI);

        let ie = original.to_ie();
        let parsed = MbsSessionN4Information::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let result = MbsSessionN4Information::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = MbsSessionN4Information::new(MbsSessionIdentifier::new_tmgi([0; 6])).to_ie();
        assert_eq!(ie.ie_type, IeType::MbsSessionN4Information);
    }
}
