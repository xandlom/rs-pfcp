// src/ie/activate_predefined_rules.rs

//! Activate Predefined Rules Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatePredefinedRules {
    pub rule_name: String,
}

impl ActivatePredefinedRules {
    pub fn new(rule_name: &str) -> Self {
        ActivatePredefinedRules {
            rule_name: rule_name.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.rule_name.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let rule_name = String::from_utf8(data.to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "Activate Predefined Rules",
                IeType::ActivatePredefinedRules,
                e.utf8_error(),
            )
        })?;
        Ok(ActivatePredefinedRules { rule_name })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ActivatePredefinedRules, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activate_predefined_rules_marshal_unmarshal() {
        let apr = ActivatePredefinedRules::new("rule1");
        let marshaled = apr.marshal();
        let unmarshaled = ActivatePredefinedRules::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, apr);
    }

    #[test]
    fn test_activate_predefined_rules_invalid_utf8() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let result = ActivatePredefinedRules::unmarshal(&invalid_utf8);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::EncodingError { .. }));
        assert!(err.to_string().contains("Activate Predefined Rules"));
    }
}
