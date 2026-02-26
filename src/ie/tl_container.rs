//! TL-Container Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.230, carries raw TL-Container bytes
//! as defined in 3GPP TS 26.510 for RTP/multimedia transport configuration.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// TL-Container IE.
///
/// Carries an opaque byte sequence representing a TL-Container as defined
/// in 3GPP TS 26.510.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.230
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlContainer {
    pub value: Vec<u8>,
}

impl TlContainer {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "TL-Container",
                IeType::TlContainer,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TlContainer, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = TlContainer::new(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        let parsed = TlContainer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TlContainer::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = TlContainer::new(vec![0xFF]).to_ie();
        assert_eq!(ie.ie_type, IeType::TlContainer);
        assert_eq!(ie.payload, vec![0xFF]);
    }
}
