//! PMF Parameters IE (Grouped, IE Type 227).
//!
//! Per 3GPP TS 29.244 Table 7.5.3.7-4, contains PMF allocation information.

use crate::error::PfcpError;
use crate::ie::pmf_address_information::PmfAddressInformation;
use crate::ie::qfi::Qfi;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmfParameters {
    pub pmf_address_information: PmfAddressInformation,
    /// Optional QoS Flow Identifiers
    pub qfis: Vec<Qfi>,
}

impl PmfParameters {
    pub fn new(pmf_address_information: PmfAddressInformation) -> Self {
        Self {
            pmf_address_information,
            qfis: Vec::new(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.pmf_address_information.to_ie()];
        for qfi in &self.qfis {
            ies.push(qfi.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut pmf_addr = None;
        let mut qfis = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PmfAddressInformation => {
                    pmf_addr = Some(PmfAddressInformation::unmarshal(&ie.payload)?);
                }
                IeType::Qfi => {
                    qfis.push(Qfi::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            pmf_address_information: pmf_addr.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::PmfAddressInformation,
                    IeType::PmfParameters,
                )
            })?,
            qfis,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PmfParameters, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn make_pmf_addr() -> PmfAddressInformation {
        PmfAddressInformation {
            ipv4_address: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6_address: None,
            port_3gpp: Some(5001),
            port_non3gpp: Some(5002),
            mac_3gpp: None,
            mac_non3gpp: None,
        }
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = PmfParameters::new(make_pmf_addr());
        let parsed = PmfParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_qfis() {
        let mut original = PmfParameters::new(make_pmf_addr());
        original.qfis = vec![Qfi::of(5), Qfi::of(9)];
        let parsed = PmfParameters::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = PmfParameters::new(make_pmf_addr()).to_ie();
        assert_eq!(ie.ie_type, IeType::PmfParameters);
    }

    #[test]
    fn test_missing_pmf_address_information() {
        assert!(matches!(
            PmfParameters::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
