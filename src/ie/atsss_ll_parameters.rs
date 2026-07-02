//! ATSSS-LL Parameters IE (Grouped, IE Type 226).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.7-3, contains ATSSS-LL allocation information.

use crate::error::PfcpError;
use crate::ie::atsss_ll_information::AtsssLlInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsssLlParameters {
    pub atsss_ll_information: AtsssLlInformation,
}

impl AtsssLlParameters {
    pub fn new(atsss_ll_information: AtsssLlInformation) -> Self {
        Self {
            atsss_ll_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        marshal_ies(&[self.atsss_ll_information.to_ie()])
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut atsss_ll_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::AtsssLlInformation {
                atsss_ll_information = Some(AtsssLlInformation::unmarshal(&ie.payload)?);
            }
        }

        Ok(Self {
            atsss_ll_information: atsss_ll_information.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::AtsssLlInformation,
                    IeType::AtsssLlParameters,
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AtsssLlParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_round_trip() {
        let original = AtsssLlParameters::new(AtsssLlInformation::LLI);
        let parsed = AtsssLlParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = AtsssLlParameters::new(AtsssLlInformation::LLI).to_ie();
        assert_eq!(ie.ie_type, IeType::AtsssLlParameters);
    }

    #[test]
    fn test_missing_atsss_ll_information() {
        assert!(matches!(
            AtsssLlParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
