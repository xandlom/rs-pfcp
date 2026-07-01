//! Provide RDS Configuration Information IE (Grouped, IE Type 261).
//!
//! Per 3GPP TS 29.244 Section 7.5.2.11, instructs the UP function to apply
//! the Reliable Data Service (RDS) mechanism for a PFCP session.

use crate::error::PfcpError;
use crate::ie::rds_configuration_information::RdsConfigurationInformation;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvideRdsConfigurationInformation {
    pub rds_configuration_information: Option<RdsConfigurationInformation>,
}

impl ProvideRdsConfigurationInformation {
    pub fn new(rds_configuration_information: Option<RdsConfigurationInformation>) -> Self {
        Self {
            rds_configuration_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies: Vec<Ie> = Vec::new();
        if let Some(ref v) = self.rds_configuration_information {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut rds_configuration_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            if ie.ie_type == IeType::RdsConfigurationInformation {
                rds_configuration_information =
                    Some(RdsConfigurationInformation::unmarshal(&ie.payload)?);
            }
        }

        Ok(Self {
            rds_configuration_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ProvideRdsConfigurationInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_empty() {
        let original = ProvideRdsConfigurationInformation::new(None);
        let parsed = ProvideRdsConfigurationInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_rds() {
        let original =
            ProvideRdsConfigurationInformation::new(Some(RdsConfigurationInformation::new(true)));
        let parsed = ProvideRdsConfigurationInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = ProvideRdsConfigurationInformation::new(None).to_ie();
        assert_eq!(ie.ie_type, IeType::ProvideRdsConfigurationInformation);
    }
}
