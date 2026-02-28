//! Created Bridge Info for TSC Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.3.1-2, provides created TSN bridge
//! information including bridge ID and port management information.

use crate::error::PfcpError;
use crate::ie::port_management_information_container::PortManagementInformationContainer;
use crate::ie::tsn_bridge_id::TsnBridgeId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Created Bridge Info for TSC per 3GPP TS 29.244 §7.5.3.1-2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedBridgeInfoForTsc {
    pub tsn_bridge_id: Option<TsnBridgeId>,
    pub port_management_info_container: Option<PortManagementInformationContainer>,
}

impl CreatedBridgeInfoForTsc {
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(bid) = &self.tsn_bridge_id {
            ies.push(bid.to_ie());
        }
        if let Some(pmic) = &self.port_management_info_container {
            ies.push(pmic.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut tsn_bridge_id = None;
        let mut port_management_info_container = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::TsnBridgeId => {
                    tsn_bridge_id = Some(TsnBridgeId::unmarshal(&ie.payload)?);
                }
                IeType::PortManagementInformationContainer => {
                    port_management_info_container =
                        Some(PortManagementInformationContainer::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(CreatedBridgeInfoForTsc {
            tsn_bridge_id,
            port_management_info_container,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreatedBridgeInfoForTsc, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all_fields() {
        let ie = CreatedBridgeInfoForTsc {
            tsn_bridge_id: Some(TsnBridgeId::new([0x01, 0x02, 0x03, 0x04, 0x05, 0x06])),
            port_management_info_container: Some(PortManagementInformationContainer::new(vec![
                0xAB, 0xCD,
            ])),
        };
        let parsed = CreatedBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let ie = CreatedBridgeInfoForTsc {
            tsn_bridge_id: None,
            port_management_info_container: None,
        };
        let parsed = CreatedBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_bridge_id_only() {
        let ie = CreatedBridgeInfoForTsc {
            tsn_bridge_id: Some(TsnBridgeId::new([0xFF; 6])),
            port_management_info_container: None,
        };
        let parsed = CreatedBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_to_ie() {
        let ie = CreatedBridgeInfoForTsc {
            tsn_bridge_id: Some(TsnBridgeId::new([0x01; 6])),
            port_management_info_container: None,
        }
        .to_ie();
        assert_eq!(ie.ie_type, IeType::CreatedBridgeInfoForTsc);
        assert!(!ie.payload.is_empty());
    }
}
