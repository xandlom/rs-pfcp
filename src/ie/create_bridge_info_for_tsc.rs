//! Create Bridge Info for TSC Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.1-4, provides TSN bridge configuration
//! information for Time-Sensitive Communication including bridge ID, time
//! domain, and management information containers.

use crate::error::PfcpError;
use crate::ie::bridge_management_information_container::BridgeManagementInformationContainer;
use crate::ie::port_management_information_container::PortManagementInformationContainer;
use crate::ie::tsn_bridge_id::TsnBridgeId;
use crate::ie::tsn_time_domain_number::TsnTimeDomainNumber;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Create Bridge Info for TSC per 3GPP TS 29.244 §7.5.2.1-4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateBridgeInfoForTsc {
    pub tsn_bridge_id: Option<TsnBridgeId>,
    pub tsn_time_domain_number: Option<TsnTimeDomainNumber>,
    pub port_management_info_container: Option<PortManagementInformationContainer>,
    pub bridge_management_info_container: Option<BridgeManagementInformationContainer>,
}

impl CreateBridgeInfoForTsc {
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(bid) = &self.tsn_bridge_id {
            ies.push(bid.to_ie());
        }
        if let Some(tdn) = &self.tsn_time_domain_number {
            ies.push(tdn.to_ie());
        }
        if let Some(pmic) = &self.port_management_info_container {
            ies.push(pmic.to_ie());
        }
        if let Some(bmic) = &self.bridge_management_info_container {
            ies.push(bmic.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut tsn_bridge_id = None;
        let mut tsn_time_domain_number = None;
        let mut port_management_info_container = None;
        let mut bridge_management_info_container = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::TsnBridgeId => {
                    tsn_bridge_id = Some(TsnBridgeId::unmarshal(&ie.payload)?);
                }
                IeType::TsnTimeDomainNumber => {
                    tsn_time_domain_number = Some(TsnTimeDomainNumber::unmarshal(&ie.payload)?);
                }
                IeType::PortManagementInformationContainer => {
                    port_management_info_container =
                        Some(PortManagementInformationContainer::unmarshal(&ie.payload)?);
                }
                IeType::BridgeManagementInformationContainer => {
                    bridge_management_info_container = Some(
                        BridgeManagementInformationContainer::unmarshal(&ie.payload)?,
                    );
                }
                _ => (),
            }
        }

        Ok(CreateBridgeInfoForTsc {
            tsn_bridge_id,
            tsn_time_domain_number,
            port_management_info_container,
            bridge_management_info_container,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateBridgeInfoForTsc, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_all_fields() -> CreateBridgeInfoForTsc {
        CreateBridgeInfoForTsc {
            tsn_bridge_id: Some(TsnBridgeId::new([0x01, 0x02, 0x03, 0x04, 0x05, 0x06])),
            tsn_time_domain_number: Some(TsnTimeDomainNumber::new(1)),
            port_management_info_container: Some(PortManagementInformationContainer::new(vec![
                0xDE, 0xAD,
            ])),
            bridge_management_info_container: Some(BridgeManagementInformationContainer::new(
                vec![0xBE, 0xEF],
            )),
        }
    }

    #[test]
    fn test_marshal_unmarshal_all_fields() {
        let ie = make_all_fields();
        let parsed = CreateBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let ie = CreateBridgeInfoForTsc {
            tsn_bridge_id: None,
            tsn_time_domain_number: None,
            port_management_info_container: None,
            bridge_management_info_container: None,
        };
        let parsed = CreateBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_bridge_id_only() {
        let ie = CreateBridgeInfoForTsc {
            tsn_bridge_id: Some(TsnBridgeId::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])),
            tsn_time_domain_number: None,
            port_management_info_container: None,
            bridge_management_info_container: None,
        };
        let parsed = CreateBridgeInfoForTsc::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_to_ie() {
        let ie = make_all_fields().to_ie();
        assert_eq!(ie.ie_type, IeType::CreateBridgeInfoForTsc);
        assert!(!ie.payload.is_empty());
    }
}
