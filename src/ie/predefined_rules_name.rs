//! Predefined Rules Name IE (Variable Length, IE Type 299).
//!
//! Per 3GPP TS 29.244 Clause 8.2.205, contains the name referring to one or
//! more predefined rules activated in the UP function. Encoded as OctetString.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredefinedRulesName {
    pub name: Vec<u8>,
}

impl PredefinedRulesName {
    pub fn new(name: impl Into<Vec<u8>>) -> Self {
        Self { name: name.into() }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.name.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PredefinedRulesName",
                IeType::PredefinedRulesName,
                1,
                0,
            ));
        }
        Ok(Self {
            name: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PredefinedRulesName, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = PredefinedRulesName::new(b"rule-set-1".to_vec());
        let parsed = PredefinedRulesName::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_empty_rejected() {
        let result = PredefinedRulesName::unmarshal(&[]);
        assert!(matches!(result, Err(PfcpError::InvalidLength { .. })));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = PredefinedRulesName::new(b"r".to_vec()).to_ie();
        assert_eq!(ie.ie_type, IeType::PredefinedRulesName);
    }
}
