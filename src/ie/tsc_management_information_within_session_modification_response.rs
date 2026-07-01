//! TSC Management Information within Session Modification Response IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.5.11-2, carries TSN port management
//! information for Time-Sensitive Communication configuration in the response
//! to a session modification request.

use crate::error::PfcpError;
use crate::ie::created_bridge_info_for_tsc::CreatedBridgeInfoForTsc;
use crate::ie::dstt_port_number::DsttPortNumber;
use crate::ie::nwtt_port_number::NwttPortNumber;
use crate::ie::port_management_information_container::PortManagementInformationContainer;
use crate::ie::tsn_bridge_id::TsnBridgeId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TscManagementInformationWithinSessionModificationResponse {
    pub port_management_information_container: PortManagementInformationContainer,
    pub nwtt_port_number: Option<NwttPortNumber>,
    pub dstt_port_number: Option<DsttPortNumber>,
    pub tsn_bridge_id: Option<TsnBridgeId>,
    pub created_bridge_info_for_tsc: Option<CreatedBridgeInfoForTsc>,
}

impl TscManagementInformationWithinSessionModificationResponse {
    pub fn new(
        port_management_information_container: PortManagementInformationContainer,
        nwtt_port_number: Option<NwttPortNumber>,
        dstt_port_number: Option<DsttPortNumber>,
        tsn_bridge_id: Option<TsnBridgeId>,
        created_bridge_info_for_tsc: Option<CreatedBridgeInfoForTsc>,
    ) -> Self {
        Self {
            port_management_information_container,
            nwtt_port_number,
            dstt_port_number,
            tsn_bridge_id,
            created_bridge_info_for_tsc,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.port_management_information_container.to_ie()];
        if let Some(n) = &self.nwtt_port_number {
            ies.push(n.to_ie());
        }
        if let Some(d) = &self.dstt_port_number {
            ies.push(d.to_ie());
        }
        if let Some(b) = &self.tsn_bridge_id {
            ies.push(b.to_ie());
        }
        if let Some(c) = &self.created_bridge_info_for_tsc {
            ies.push(c.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut port_management_information_container = None;
        let mut nwtt_port_number = None;
        let mut dstt_port_number = None;
        let mut tsn_bridge_id = None;
        let mut created_bridge_info_for_tsc = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PortManagementInformationContainer => {
                    port_management_information_container =
                        Some(PortManagementInformationContainer::unmarshal(&ie.payload)?);
                }
                IeType::NwttPortNumber => {
                    nwtt_port_number = Some(NwttPortNumber::unmarshal(&ie.payload)?);
                }
                IeType::DsttPortNumber => {
                    dstt_port_number = Some(DsttPortNumber::unmarshal(&ie.payload)?);
                }
                IeType::TsnBridgeId => {
                    tsn_bridge_id = Some(TsnBridgeId::unmarshal(&ie.payload)?);
                }
                IeType::CreatedBridgeInfoForTsc => {
                    created_bridge_info_for_tsc =
                        Some(CreatedBridgeInfoForTsc::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            port_management_information_container: port_management_information_container
                .ok_or_else(|| {
                    PfcpError::missing_ie_in_grouped(
                        IeType::PortManagementInformationContainer,
                        IeType::TscManagementInformationWithinSessionModificationResponse,
                    )
                })?,
            nwtt_port_number,
            dstt_port_number,
            tsn_bridge_id,
            created_bridge_info_for_tsc,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::TscManagementInformationWithinSessionModificationResponse,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pmic() -> PortManagementInformationContainer {
        PortManagementInformationContainer::new(vec![0xDE, 0xAD, 0xBE, 0xEF])
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = TscManagementInformationWithinSessionModificationResponse::new(
            make_pmic(),
            None,
            None,
            None,
            None,
        );
        let parsed = TscManagementInformationWithinSessionModificationResponse::unmarshal(
            &original.marshal(),
        )
        .unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = TscManagementInformationWithinSessionModificationResponse::new(
            make_pmic(),
            Some(NwttPortNumber::new(3000)),
            Some(DsttPortNumber::new(4000)),
            Some(TsnBridgeId::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66])),
            None,
        );
        let parsed = TscManagementInformationWithinSessionModificationResponse::unmarshal(
            &original.marshal(),
        )
        .unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = TscManagementInformationWithinSessionModificationResponse::new(
            make_pmic(),
            None,
            None,
            None,
            None,
        )
        .to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::TscManagementInformationWithinSessionModificationResponse
        );
    }

    #[test]
    fn test_missing_port_management_info() {
        assert!(matches!(
            TscManagementInformationWithinSessionModificationResponse::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
