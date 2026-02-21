//! Weight Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.140, represents the weight for access forwarding.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weight {
    pub value: u8,
}

impl Weight {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length("Weight", IeType::Weight, 1, 0));
        }
        Ok(Self { value: data[0] })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Weight, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let w = Weight::new(128);
        let parsed = Weight::unmarshal(&w.marshal()).unwrap();
        assert_eq!(parsed, w);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            Weight::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(Weight::new(1).to_ie().ie_type, IeType::Weight);
    }
}
