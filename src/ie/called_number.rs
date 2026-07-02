//! Called Number IE (IE Type 283).
//!
//! Per 3GPP TS 29.244 Section 8.2.191, a UTF-8 string containing the called
//! station ID (destination) provided by the L2TP Access Concentrator (LAC).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalledNumber {
    pub number: String,
}

impl CalledNumber {
    pub fn new(number: impl Into<String>) -> Self {
        Self {
            number: number.into(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.number.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let number = String::from_utf8(data.to_vec()).map_err(|_| {
            PfcpError::invalid_value(
                "CalledNumber.number",
                format!("{data:?}"),
                "not valid UTF-8",
            )
        })?;
        Ok(Self { number })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CalledNumber, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = CalledNumber::new("vpn.example.com");
        let parsed = CalledNumber::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_empty() {
        let original = CalledNumber::new("");
        let parsed = CalledNumber::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_invalid_utf8() {
        assert!(matches!(
            CalledNumber::unmarshal(&[0xFF, 0xFE]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            CalledNumber::new("456").to_ie().ie_type,
            IeType::CalledNumber
        );
    }
}
