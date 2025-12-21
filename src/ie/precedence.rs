// src/ie/precedence.rs

//! Precedence Information Element.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precedence {
    pub value: u32,
}

impl Precedence {
    pub fn new(value: u32) -> Self {
        Precedence { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    /// Unmarshals a byte slice into a Precedence.
    ///
    /// Per 3GPP TS 29.244, Precedence requires exactly 4 bytes (Priority value).
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Precedence",
                IeType::Precedence,
                4,
                data.len(),
            ));
        }
        Ok(Precedence {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Precedence, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence_marshal_unmarshal() {
        let precedence = Precedence::new(100);
        let marshaled = precedence.marshal();
        let unmarshaled = Precedence::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, precedence);
    }

    #[test]
    fn test_precedence_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = Precedence::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Precedence"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("3"));
    }

    #[test]
    fn test_precedence_unmarshal_empty() {
        let result = Precedence::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Precedence"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("0"));
    }
}
