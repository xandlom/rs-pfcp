//! TSC Management Information within PFCP Session Report Request IE (Grouped, IE Type 201).
//!
//! Per 3GPP TS 29.244 Table 7.5.8.5-1, carries TSN port and/or user plane node
//! management information containers for reporting to the CP function.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TscManagementInformationWithinSessionReportRequest {
    /// Port Management Information Container (optional).
    pub port_management_information_container: Option<Ie>,
    /// User Plane Node Management Information Container (optional, IE type 266).
    pub user_plane_node_management_information_container: Option<Ie>,
    /// NW-TT Port Number (conditional — present when PMIC is present).
    pub nwtt_port_number: Option<Ie>,
}

impl TscManagementInformationWithinSessionReportRequest {
    pub fn new() -> Self {
        Self {
            port_management_information_container: None,
            user_plane_node_management_information_container: None,
            nwtt_port_number: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.port_management_information_container {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.user_plane_node_management_information_container {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.nwtt_port_number {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut port_management_information_container = None;
        let mut user_plane_node_management_information_container = None;
        let mut nwtt_port_number = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::PortManagementInformationContainer => {
                    port_management_information_container = Some(ie);
                }
                IeType::BridgeManagementInformationContainer => {
                    user_plane_node_management_information_container = Some(ie);
                }
                IeType::NwttPortNumber => {
                    nwtt_port_number = Some(ie);
                }
                _ => (),
            }
        }

        Ok(Self {
            port_management_information_container,
            user_plane_node_management_information_container,
            nwtt_port_number,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::TscManagementInformationWithinSessionReportRequest,
            self.marshal(),
        )
    }
}

impl Default for TscManagementInformationWithinSessionReportRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pmic_ie() -> Ie {
        Ie::new(IeType::PortManagementInformationContainer, vec![0xDE, 0xAD])
    }

    fn make_umic_ie() -> Ie {
        Ie::new(
            IeType::BridgeManagementInformationContainer,
            vec![0xBE, 0xEF],
        )
    }

    fn make_nwtt_port_ie() -> Ie {
        Ie::new(IeType::NwttPortNumber, vec![0x00, 0x00, 0x03, 0xE8])
    }

    #[test]
    fn test_round_trip_empty() {
        let original = TscManagementInformationWithinSessionReportRequest::new();
        let parsed =
            TscManagementInformationWithinSessionReportRequest::unmarshal(&original.marshal())
                .unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_pmic_only() {
        let mut original = TscManagementInformationWithinSessionReportRequest::new();
        original.port_management_information_container = Some(make_pmic_ie());
        let parsed =
            TscManagementInformationWithinSessionReportRequest::unmarshal(&original.marshal())
                .unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = TscManagementInformationWithinSessionReportRequest {
            port_management_information_container: Some(make_pmic_ie()),
            user_plane_node_management_information_container: Some(make_umic_ie()),
            nwtt_port_number: Some(make_nwtt_port_ie()),
        };
        let parsed =
            TscManagementInformationWithinSessionReportRequest::unmarshal(&original.marshal())
                .unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = TscManagementInformationWithinSessionReportRequest::new().to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::TscManagementInformationWithinSessionReportRequest
        );
    }
}
