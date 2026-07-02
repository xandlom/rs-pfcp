//! ATSSS Control Parameters IE (Grouped, IE Type 221).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.7-1, carries ATSSS allocation results
//! in a PFCP Session Establishment Response.

use crate::error::PfcpError;
use crate::ie::atsss_ll_parameters::AtsssLlParameters;
use crate::ie::mpquic_parameters::MpquicParameters;
use crate::ie::mptcp_parameters::MptcpParameters;
use crate::ie::pmf_parameters::PmfParameters;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsssControlParameters {
    pub mptcp_parameters: Option<MptcpParameters>,
    pub atsss_ll_parameters: Option<AtsssLlParameters>,
    /// Multiple PmfParameters may be present (one per QoS flow)
    pub pmf_parameters: Vec<PmfParameters>,
    pub mpquic_parameters: Option<MpquicParameters>,
}

impl AtsssControlParameters {
    pub fn new() -> Self {
        Self {
            mptcp_parameters: None,
            atsss_ll_parameters: None,
            pmf_parameters: Vec::new(),
            mpquic_parameters: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.mptcp_parameters {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.atsss_ll_parameters {
            ies.push(v.to_ie());
        }
        for pmf in &self.pmf_parameters {
            ies.push(pmf.to_ie());
        }
        if let Some(ref v) = self.mpquic_parameters {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut mptcp = None;
        let mut atsss_ll = None;
        let mut pmf_parameters = Vec::new();
        let mut mpquic = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MptcpParameters => {
                    mptcp = Some(MptcpParameters::unmarshal(&ie.payload)?);
                }
                IeType::AtsssLlParameters => {
                    atsss_ll = Some(AtsssLlParameters::unmarshal(&ie.payload)?);
                }
                IeType::PmfParameters => {
                    pmf_parameters.push(PmfParameters::unmarshal(&ie.payload)?);
                }
                IeType::MpquicParameters => {
                    mpquic = Some(MpquicParameters::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            mptcp_parameters: mptcp,
            atsss_ll_parameters: atsss_ll,
            pmf_parameters,
            mpquic_parameters: mpquic,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AtsssControlParameters, self.marshal())
    }
}

impl Default for AtsssControlParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::atsss_ll_information::AtsssLlInformation;
    use crate::ie::link_specific_multipath_ip_address::LinkSpecificMultipathIpAddress;
    use crate::ie::mptcp_address_information::MptcpAddressInformation;
    use crate::ie::pmf_address_information::PmfAddressInformation;
    use std::net::Ipv4Addr;

    #[test]
    fn test_marshal_unmarshal_empty() {
        let original = AtsssControlParameters::new();
        let parsed = AtsssControlParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_atsss_ll() {
        let mut original = AtsssControlParameters::new();
        original.atsss_ll_parameters = Some(AtsssLlParameters::new(AtsssLlInformation::LLI));
        let parsed = AtsssControlParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_pmf() {
        let mut original = AtsssControlParameters::new();
        let pmf_addr = PmfAddressInformation {
            ipv4_address: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6_address: None,
            port_3gpp: Some(5001),
            port_non3gpp: Some(5002),
            mac_3gpp: None,
            mac_non3gpp: None,
        };
        original
            .pmf_parameters
            .push(PmfParameters::new(pmf_addr.clone()));
        original.pmf_parameters.push(PmfParameters::new(pmf_addr));
        let parsed = AtsssControlParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_mptcp() {
        let mut original = AtsssControlParameters::new();
        let mut mptcp_addr = MptcpAddressInformation::new(1, 8080);
        mptcp_addr.ipv4_address = Some(Ipv4Addr::new(10, 0, 0, 1));
        let mut link = LinkSpecificMultipathIpAddress::new();
        link.ipv4_3gpp = Some(Ipv4Addr::new(10, 1, 0, 1));
        original.mptcp_parameters = Some(MptcpParameters::new(mptcp_addr, link));
        let parsed = AtsssControlParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = AtsssControlParameters::new().to_ie();
        assert_eq!(ie.ie_type, IeType::AtsssControlParameters);
    }
}
