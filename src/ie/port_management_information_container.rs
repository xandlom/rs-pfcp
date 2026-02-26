//! Port Management Information Container Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.144, carries raw port management message bytes
//! as defined in IEEE 802.1Qcc and TS 24.539.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Port Management Information Container IE.
///
/// Carries an opaque byte sequence representing a port management information
/// message (e.g. an IEEE 802.1Qcc frame) for TSN (Time-Sensitive Networking)
/// port configuration.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.144
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortManagementInformationContainer {
    pub value: Vec<u8>,
}

impl PortManagementInformationContainer {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Port Management Information Container",
                IeType::PortManagementInformationContainer,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PortManagementInformationContainer, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = PortManagementInformationContainer::new(vec![0x01, 0x02, 0x03]);
        let parsed = PortManagementInformationContainer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PortManagementInformationContainer::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = PortManagementInformationContainer::new(vec![0xAB]).to_ie();
        assert_eq!(ie.ie_type, IeType::PortManagementInformationContainer);
        assert_eq!(ie.payload, vec![0xAB]);
    }

    #[test]
    fn test_round_trip_various() {
        for data in [vec![0x00], vec![0xFF; 10], vec![0x12, 0x34, 0x56, 0x78]] {
            let original = PortManagementInformationContainer::new(data.clone());
            let parsed =
                PortManagementInformationContainer::unmarshal(&original.marshal()).unwrap();
            assert_eq!(original, parsed);
        }
    }
}
