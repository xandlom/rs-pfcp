//! MAR ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.137, identifies a MAR (Multi-Access Rule).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarId {
    pub value: u16,
}

impl MarId {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "MAR ID",
                IeType::MarId,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MarId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let id = MarId::new(42);
        let parsed = MarId::unmarshal(&id.marshal()).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            MarId::unmarshal(&[0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(MarId::new(1).to_ie().ie_type, IeType::MarId);
    }
}
