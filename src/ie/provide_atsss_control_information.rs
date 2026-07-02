//! Provide ATSSS Control Information IE (Grouped, IE Type 220).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.10-1, requests ATSSS functionality
//! in a PFCP Session Establishment Request.

use crate::error::PfcpError;
use crate::ie::atsss_ll_control_information::AtsssLlControlInformation;
use crate::ie::mpquic_control_information::MpquicControlInformation;
use crate::ie::mptcp_control_information::MptcpControlInformation;
use crate::ie::pmf_control_information::PmfControlInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvideAtsssControlInformation {
    pub mptcp_control_information: Option<MptcpControlInformation>,
    pub atsss_ll_control_information: Option<AtsssLlControlInformation>,
    pub pmf_control_information: Option<PmfControlInformation>,
    pub mpquic_control_information: Option<MpquicControlInformation>,
}

impl ProvideAtsssControlInformation {
    pub fn new() -> Self {
        Self {
            mptcp_control_information: None,
            atsss_ll_control_information: None,
            pmf_control_information: None,
            mpquic_control_information: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.mptcp_control_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.atsss_ll_control_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.pmf_control_information {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.mpquic_control_information {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mptcp = None;
        let mut atsss_ll = None;
        let mut pmf = None;
        let mut mpquic = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MptcpControlInformation => {
                    mptcp = Some(MptcpControlInformation::unmarshal(&ie.payload)?);
                }
                IeType::AtsssLlControlInformation => {
                    atsss_ll = Some(AtsssLlControlInformation::unmarshal(&ie.payload)?);
                }
                IeType::PmfControlInformation => {
                    pmf = Some(PmfControlInformation::unmarshal(&ie.payload)?);
                }
                IeType::MpquicControlInformation => {
                    mpquic = Some(MpquicControlInformation::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            mptcp_control_information: mptcp,
            atsss_ll_control_information: atsss_ll,
            pmf_control_information: pmf,
            mpquic_control_information: mpquic,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ProvideAtsssControlInformation, self.marshal())
    }
}

impl Default for ProvideAtsssControlInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_empty() {
        let original = ProvideAtsssControlInformation::new();
        let parsed = ProvideAtsssControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_all_set() {
        let original = ProvideAtsssControlInformation {
            mptcp_control_information: Some(MptcpControlInformation::TCI),
            atsss_ll_control_information: Some(AtsssLlControlInformation::LLI),
            pmf_control_information: Some(PmfControlInformation {
                pmfi: true,
                drtti: false,
                qfis: vec![],
            }),
            mpquic_control_information: Some(MpquicControlInformation::CUDP),
        };
        let parsed = ProvideAtsssControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = ProvideAtsssControlInformation::new().to_ie();
        assert_eq!(ie.ie_type, IeType::ProvideAtsssControlInformation);
    }
}
