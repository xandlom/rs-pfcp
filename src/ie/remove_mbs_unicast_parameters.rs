//! Remove MBS Unicast Parameters IE (IE Type 304).
//!
//! Per 3GPP TS 29.244 Table 7.5.4.3-4, used in Update FAR to remove
//! MBS unicast forwarding to a GTP-U peer.

use crate::error::PfcpError;
use crate::ie::mbs_unicast_parameters_id::MbsUnicastParametersId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Remove MBS Unicast Parameters grouped IE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMbsUnicastParameters {
    pub mbs_unicast_parameters_id: MbsUnicastParametersId, // M
}

impl RemoveMbsUnicastParameters {
    pub fn new(mbs_unicast_parameters_id: MbsUnicastParametersId) -> Self {
        Self {
            mbs_unicast_parameters_id,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let ies = vec![self.mbs_unicast_parameters_id.to_ie()];
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mbs_unicast_parameters_id = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::MbsUnicastParametersId {
                mbs_unicast_parameters_id = Some(MbsUnicastParametersId::unmarshal(&ie.payload)?);
            }
        }

        let mbs_unicast_parameters_id = mbs_unicast_parameters_id.ok_or_else(|| {
            PfcpError::missing_ie_in_grouped(
                IeType::MbsUnicastParametersId,
                IeType::RemoveMbsUnicastParameters,
            )
        })?;

        Ok(Self {
            mbs_unicast_parameters_id,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::RemoveMbsUnicastParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = RemoveMbsUnicastParameters::new(MbsUnicastParametersId::new(42));
        let ie = original.to_ie();
        let parsed = RemoveMbsUnicastParameters::unmarshal(&ie.payload).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let result = RemoveMbsUnicastParameters::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::MissingMandatoryIe { .. })));
    }
}
