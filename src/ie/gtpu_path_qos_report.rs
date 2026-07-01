//! GTP-U Path QoS Report IE (Grouped, IE Type 239).
//!
//! Per 3GPP TS 29.244 Section 7.4.5.1.7 (Table 7.4.5.1.7-1), reports
//! GTP-U path QoS measurement results for a specific remote GTP-U peer.

use crate::error::PfcpError;
use crate::ie::gtpu_path_interface_type::GtpuPathInterfaceType;
use crate::ie::qos_information_in_gtpu_path_qos_report::QosInformationInGtpuPathQosReport;
use crate::ie::remote_gtpu_peer::RemoteGtpuPeer;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtpuPathQosReport {
    pub remote_gtpu_peer: RemoteGtpuPeer, // M
    pub gtpu_path_interface_type: Option<GtpuPathInterfaceType>,
    pub qos_information: Vec<QosInformationInGtpuPathQosReport>, // C - Multiple
}

impl GtpuPathQosReport {
    pub fn new(
        remote_gtpu_peer: RemoteGtpuPeer,
        gtpu_path_interface_type: Option<GtpuPathInterfaceType>,
        qos_information: Vec<QosInformationInGtpuPathQosReport>,
    ) -> Self {
        Self {
            remote_gtpu_peer,
            gtpu_path_interface_type,
            qos_information,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.remote_gtpu_peer.to_ie()];
        if let Some(ref v) = self.gtpu_path_interface_type {
            ies.push(v.to_ie());
        }
        for qi in &self.qos_information {
            ies.push(qi.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut remote_gtpu_peer = None;
        let mut gtpu_path_interface_type = None;
        let mut qos_information = Vec::new();

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::RemoteGtpuPeer => {
                    remote_gtpu_peer = Some(RemoteGtpuPeer::unmarshal(&ie.payload)?);
                }
                IeType::GtpuPathInterfaceType => {
                    gtpu_path_interface_type = Some(GtpuPathInterfaceType::unmarshal(&ie.payload)?);
                }
                IeType::QosInformationInGtpuPathQosReport => {
                    qos_information
                        .push(QosInformationInGtpuPathQosReport::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            remote_gtpu_peer: remote_gtpu_peer.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::RemoteGtpuPeer, IeType::GtpuPathQosReport)
            })?,
            gtpu_path_interface_type,
            qos_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::GtpuPathQosReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::average_packet_delay::AveragePacketDelay;
    use crate::ie::gtpu_path_interface_type::GtpuPathInterfaceType;
    use crate::ie::remote_gtpu_peer::RemoteGtpuPeer;
    use std::net::Ipv4Addr;

    fn make_peer() -> RemoteGtpuPeer {
        RemoteGtpuPeer {
            ipv4: Some(Ipv4Addr::new(10, 0, 0, 1)),
            ipv6: None,
            destination_interface: None,
            network_instance: None,
            remote_tunnel_state: None,
        }
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = GtpuPathQosReport::new(make_peer(), None, vec![]);
        let parsed = GtpuPathQosReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let qi = QosInformationInGtpuPathQosReport::new(
            Some(AveragePacketDelay::new(500)),
            None,
            None,
            None,
            None,
            None,
        );
        let original = GtpuPathQosReport::new(
            make_peer(),
            Some(GtpuPathInterfaceType::from_bits_truncate(0x01)),
            vec![qi],
        );
        let parsed = GtpuPathQosReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = GtpuPathQosReport::new(make_peer(), None, vec![]).to_ie();
        assert_eq!(ie.ie_type, IeType::GtpuPathQosReport);
    }

    #[test]
    fn test_missing_remote_gtpu_peer() {
        assert!(matches!(
            GtpuPathQosReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
