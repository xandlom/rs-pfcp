// src/ie/deactivate_predefined_rules.rs

//! Deactivate Predefined Rules Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeactivatePredefinedRules {
    pub rule_name: String,
}

impl DeactivatePredefinedRules {
    pub fn new(rule_name: &str) -> Self {
        DeactivatePredefinedRules {
            rule_name: rule_name.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.rule_name.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let rule_name = String::from_utf8(data.to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "Deactivate Predefined Rules",
                IeType::DeactivatePredefinedRules,
                e.utf8_error(),
            )
        })?;
        Ok(DeactivatePredefinedRules { rule_name })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DeactivatePredefinedRules, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deactivate_predefined_rules_marshal_unmarshal() {
        let dpr = DeactivatePredefinedRules::new("rule1");
        let marshaled = dpr.marshal();
        let unmarshaled = DeactivatePredefinedRules::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, dpr);
    }

    #[test]
    fn test_deactivate_predefined_rules_invalid_utf8() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let result = DeactivatePredefinedRules::unmarshal(&invalid_utf8);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::EncodingError { .. }));
        assert!(err.to_string().contains("Deactivate Predefined Rules"));
    }
}
