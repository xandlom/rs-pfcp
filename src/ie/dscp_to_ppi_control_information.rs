//! DSCP to PPI Control Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.1-6, groups one or more DSCP-to-PPI
//! mapping information entries with optional QFI associations.

use crate::error::PfcpError;
use crate::ie::dscp_to_ppi_mapping_information::DscpToPpiMappingInformation;
use crate::ie::qfi::Qfi;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// DSCP to PPI Control Information per 3GPP TS 29.244 §7.5.2.1-6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DscpToPpiControlInformation {
    /// DSCP-to-PPI mapping entries (mandatory, at least one).
    pub dscp_to_ppi_mapping_informations: Vec<DscpToPpiMappingInformation>,
    /// Associated QFIs (optional, zero or more).
    pub qfis: Vec<Qfi>,
}

impl DscpToPpiControlInformation {
    pub fn new(dscp_to_ppi_mapping_informations: Vec<DscpToPpiMappingInformation>) -> Self {
        DscpToPpiControlInformation {
            dscp_to_ppi_mapping_informations,
            qfis: Vec::new(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies: Vec<Ie> = self
            .dscp_to_ppi_mapping_informations
            .iter()
            .map(|m| m.to_ie())
            .collect();
        for qfi in &self.qfis {
            ies.push(qfi.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut dscp_to_ppi_mapping_informations = Vec::new();
        let mut qfis = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::DscpToPpiMappingInformation => {
                    dscp_to_ppi_mapping_informations
                        .push(DscpToPpiMappingInformation::unmarshal(&ie.payload)?);
                }
                IeType::Qfi => {
                    qfis.push(Qfi::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        if dscp_to_ppi_mapping_informations.is_empty() {
            return Err(PfcpError::missing_ie_in_grouped(
                IeType::DscpToPpiMappingInformation,
                IeType::DscpToPpiControlInformation,
            ));
        }

        Ok(DscpToPpiControlInformation {
            dscp_to_ppi_mapping_informations,
            qfis,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DscpToPpiControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_mapping() -> DscpToPpiMappingInformation {
        DscpToPpiMappingInformation {
            ppi: 3,
            dscp_values: vec![0x10, 0x18],
        }
    }

    #[test]
    fn test_marshal_unmarshal_mapping_only() {
        let ie = DscpToPpiControlInformation::new(vec![make_mapping()]);
        let parsed = DscpToPpiControlInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_qfis() {
        let mut ie = DscpToPpiControlInformation::new(vec![make_mapping()]);
        ie.qfis = vec![Qfi::new(1).unwrap(), Qfi::new(2).unwrap()];
        let parsed = DscpToPpiControlInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_multiple_mappings() {
        let ie = DscpToPpiControlInformation::new(vec![make_mapping(), make_mapping()]);
        let parsed = DscpToPpiControlInformation::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_mapping_fails() {
        assert!(matches!(
            DscpToPpiControlInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = DscpToPpiControlInformation::new(vec![make_mapping()]).to_ie();
        assert_eq!(ie.ie_type, IeType::DscpToPpiControlInformation);
        assert!(!ie.payload.is_empty());
    }
}
