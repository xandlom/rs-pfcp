//! SRR ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.165, identifies a Session Report Rule.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrrId {
    pub value: u8,
}

impl SrrId {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length("SRR ID", IeType::SrrId, 1, 0));
        }
        Ok(Self { value: data[0] })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SrrId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let id = SrrId::new(5);
        let parsed = SrrId::unmarshal(&id.marshal()).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            SrrId::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(SrrId::new(1).to_ie().ie_type, IeType::SrrId);
    }
}
