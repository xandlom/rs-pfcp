//! TSC Management Information within Session Modification Request IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.4.22-2, carries TSN port management
//! information for Time-Sensitive Communication configuration during session
//! modification.

use crate::error::PfcpError;
use crate::ie::create_bridge_info_for_tsc::CreateBridgeInfoForTsc;
use crate::ie::dstt_port_number::DsttPortNumber;
use crate::ie::nwtt_port_number::NwttPortNumber;
use crate::ie::port_management_information_container::PortManagementInformationContainer;
use crate::ie::tsn_bridge_id::TsnBridgeId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TscManagementInformationWithinSessionModificationRequest {
    pub port_management_information_container: PortManagementInformationContainer,
    pub nwtt_port_number: Option<NwttPortNumber>,
    pub dstt_port_number: Option<DsttPortNumber>,
    pub tsn_bridge_id: Option<TsnBridgeId>,
    pub create_bridge_info_for_tsc: Option<CreateBridgeInfoForTsc>,
}

impl TscManagementInformationWithinSessionModificationRequest {
    pub fn new(
        port_management_information_container: PortManagementInformationContainer,
        nwtt_port_number: Option<NwttPortNumber>,
        dstt_port_number: Option<DsttPortNumber>,
        tsn_bridge_id: Option<TsnBridgeId>,
        create_bridge_info_for_tsc: Option<CreateBridgeInfoForTsc>,
    ) -> Self {
        Self {
            port_management_information_container,
            nwtt_port_number,
            dstt_port_number,
            tsn_bridge_id,
            create_bridge_info_for_tsc,
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
        if let Some(c) = &self.create_bridge_info_for_tsc {
            ies.push(c.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut port_management_information_container = None;
        let mut nwtt_port_number = None;
        let mut dstt_port_number = None;
        let mut tsn_bridge_id = None;
        let mut create_bridge_info_for_tsc = None;

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
                IeType::CreateBridgeInfoForTsc => {
                    create_bridge_info_for_tsc =
                        Some(CreateBridgeInfoForTsc::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            port_management_information_container: port_management_information_container
                .ok_or_else(|| {
                    PfcpError::missing_ie_in_grouped(
                        IeType::PortManagementInformationContainer,
                        IeType::TscManagementInformationWithinSessionModificationRequest,
                    )
                })?,
            nwtt_port_number,
            dstt_port_number,
            tsn_bridge_id,
            create_bridge_info_for_tsc,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::TscManagementInformationWithinSessionModificationRequest,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pmic() -> PortManagementInformationContainer {
        PortManagementInformationContainer::new(vec![0x01, 0x02, 0x03])
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = TscManagementInformationWithinSessionModificationRequest::new(
            make_pmic(),
            None,
            None,
            None,
            None,
        );
        let parsed = TscManagementInformationWithinSessionModificationRequest::unmarshal(
            &original.marshal(),
        )
        .unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = TscManagementInformationWithinSessionModificationRequest::new(
            make_pmic(),
            Some(NwttPortNumber::new(1000)),
            Some(DsttPortNumber::new(2000)),
            Some(TsnBridgeId::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])),
            None,
        );
        let parsed = TscManagementInformationWithinSessionModificationRequest::unmarshal(
            &original.marshal(),
        )
        .unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = TscManagementInformationWithinSessionModificationRequest::new(
            make_pmic(),
            None,
            None,
            None,
            None,
        )
        .to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::TscManagementInformationWithinSessionModificationRequest
        );
    }

    #[test]
    fn test_missing_port_management_info() {
        assert!(matches!(
            TscManagementInformationWithinSessionModificationRequest::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
