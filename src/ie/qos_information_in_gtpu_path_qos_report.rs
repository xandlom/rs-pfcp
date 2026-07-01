//! QoS Information in GTP-U Path QoS Report IE (Grouped, IE Type 240).
//!
//! Per 3GPP TS 29.244 Section 7.4.5.1.7, contains QoS measurements for a
//! GTP-U path as part of GTP-U path QoS monitoring reporting.

use crate::error::PfcpError;
use crate::ie::average_packet_delay::AveragePacketDelay;
use crate::ie::gtpu_path_interface_type::GtpuPathInterfaceType;
use crate::ie::maximum_packet_delay::MaximumPacketDelay;
use crate::ie::minimum_packet_delay::MinimumPacketDelay;
use crate::ie::qos_monitoring_measurement::QosMonitoringMeasurement;
use crate::ie::transport_level_marking::TransportLevelMarking;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QosInformationInGtpuPathQosReport {
    pub average_packet_delay: Option<AveragePacketDelay>,
    pub minimum_packet_delay: Option<MinimumPacketDelay>,
    pub maximum_packet_delay: Option<MaximumPacketDelay>,
    pub qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
    pub transport_level_marking: Option<TransportLevelMarking>,
    pub gtpu_path_interface_type: Option<GtpuPathInterfaceType>,
}

impl QosInformationInGtpuPathQosReport {
    pub fn new(
        average_packet_delay: Option<AveragePacketDelay>,
        minimum_packet_delay: Option<MinimumPacketDelay>,
        maximum_packet_delay: Option<MaximumPacketDelay>,
        qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
        transport_level_marking: Option<TransportLevelMarking>,
        gtpu_path_interface_type: Option<GtpuPathInterfaceType>,
    ) -> Self {
        Self {
            average_packet_delay,
            minimum_packet_delay,
            maximum_packet_delay,
            qos_monitoring_measurement,
            transport_level_marking,
            gtpu_path_interface_type,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies: Vec<Ie> = Vec::new();
        if let Some(ref v) = self.average_packet_delay {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.minimum_packet_delay {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.maximum_packet_delay {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.qos_monitoring_measurement {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.transport_level_marking {
            ies.push(v.to_ie());
        }
        if let Some(ref v) = self.gtpu_path_interface_type {
            ies.push(v.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut average_packet_delay = None;
        let mut minimum_packet_delay = None;
        let mut maximum_packet_delay = None;
        let mut qos_monitoring_measurement = None;
        let mut transport_level_marking = None;
        let mut gtpu_path_interface_type = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::AveragePacketDelay => {
                    average_packet_delay = Some(AveragePacketDelay::unmarshal(&ie.payload)?);
                }
                IeType::MinimumPacketDelay => {
                    minimum_packet_delay = Some(MinimumPacketDelay::unmarshal(&ie.payload)?);
                }
                IeType::MaximumPacketDelay => {
                    maximum_packet_delay = Some(MaximumPacketDelay::unmarshal(&ie.payload)?);
                }
                IeType::QosMonitoringMeasurement => {
                    qos_monitoring_measurement =
                        Some(QosMonitoringMeasurement::unmarshal(&ie.payload)?);
                }
                IeType::TransportLevelMarking => {
                    transport_level_marking = Some(TransportLevelMarking::unmarshal(&ie.payload)?);
                }
                IeType::GtpuPathInterfaceType => {
                    gtpu_path_interface_type = Some(GtpuPathInterfaceType::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            average_packet_delay,
            minimum_packet_delay,
            maximum_packet_delay,
            qos_monitoring_measurement,
            transport_level_marking,
            gtpu_path_interface_type,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QosInformationInGtpuPathQosReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::transport_level_marking::TransportLevelMarking;

    fn make_minimal() -> QosInformationInGtpuPathQosReport {
        QosInformationInGtpuPathQosReport::new(None, None, None, None, None, None)
    }

    fn make_full() -> QosInformationInGtpuPathQosReport {
        QosInformationInGtpuPathQosReport::new(
            Some(AveragePacketDelay::new(1000)),
            Some(MinimumPacketDelay::new(500)),
            Some(MaximumPacketDelay::new(2000)),
            None,
            Some(TransportLevelMarking::new(0x28)),
            Some(GtpuPathInterfaceType::from_bits_truncate(0x01)),
        )
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = make_minimal();
        let parsed = QosInformationInGtpuPathQosReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = make_full();
        let parsed = QosInformationInGtpuPathQosReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = make_full().to_ie();
        assert_eq!(ie.ie_type, IeType::QosInformationInGtpuPathQosReport);
    }
}
