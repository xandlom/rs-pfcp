//! Bridge Management Information Container Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.190, carries raw bridge management information
//! bytes for TSN bridge configuration.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Bridge Management Information Container IE.
///
/// Carries an opaque byte sequence representing bridge management information
/// for TSN (Time-Sensitive Networking) bridge configuration.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.190
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeManagementInformationContainer {
    pub value: Vec<u8>,
}

impl BridgeManagementInformationContainer {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Bridge Management Information Container",
                IeType::BridgeManagementInformationContainer,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::BridgeManagementInformationContainer, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = BridgeManagementInformationContainer::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let parsed = BridgeManagementInformationContainer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            BridgeManagementInformationContainer::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = BridgeManagementInformationContainer::new(vec![0x01]).to_ie();
        assert_eq!(ie.ie_type, IeType::BridgeManagementInformationContainer);
    }
}
