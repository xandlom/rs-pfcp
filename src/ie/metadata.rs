//! Metadata Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.219, carries opaque metadata bytes
//! for service function chaining (base64-encoded in the spec but stored raw here).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Metadata IE.
///
/// Carries an opaque byte sequence representing service function chain metadata.
/// The spec defines this as a base64-encoded octet string, but the wire format
/// stores the raw bytes.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.219
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub value: Vec<u8>,
}

impl Metadata {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Metadata",
                IeType::Metadata,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Metadata, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = Metadata::new(vec![0xAB, 0xCD, 0xEF]);
        let parsed = Metadata::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            Metadata::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = Metadata::new(vec![0x01, 0x02]).to_ie();
        assert_eq!(ie.ie_type, IeType::Metadata);
        assert_eq!(ie.payload.len(), 2);
    }
}
