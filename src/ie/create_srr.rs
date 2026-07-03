//! Create SRR IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 7.5.2.9-1, creates a Session Reporting Rule
//! for multi-access PDU session access availability monitoring.

use crate::error::PfcpError;
use crate::ie::access_availability_control_information::AccessAvailabilityControlInformation;
use crate::ie::srr_id::SrrId;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSrr {
    pub srr_id: SrrId,
    pub access_availability_control_information: Option<AccessAvailabilityControlInformation>,
    /// QoS Monitoring per QoS flow control entries (IE 242, multiple, conditional).
    pub qos_monitoring_per_qos_flow_control_informations: Vec<Ie>,
    /// Direct reporting information for DRQOS feature (IE 295, conditional).
    pub direct_reporting_information: Option<Ie>,
    /// Traffic parameter measurement control entries (IE 323, multiple, conditional).
    pub traffic_parameter_measurement_control_informations: Vec<Ie>,
    /// Reporting control information (IE 389, conditional).
    pub reporting_control_information: Option<Ie>,
}

impl CreateSrr {
    pub fn new(
        srr_id: SrrId,
        access_availability_control_information: Option<AccessAvailabilityControlInformation>,
    ) -> Self {
        Self {
            srr_id,
            access_availability_control_information,
            qos_monitoring_per_qos_flow_control_informations: Vec::new(),
            direct_reporting_information: None,
            traffic_parameter_measurement_control_informations: Vec::new(),
            reporting_control_information: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.srr_id.to_ie()];
        if let Some(aaci) = &self.access_availability_control_information {
            ies.push(aaci.to_ie());
        }
        for ie in &self.qos_monitoring_per_qos_flow_control_informations {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.direct_reporting_information {
            ies.push(ie.clone());
        }
        for ie in &self.traffic_parameter_measurement_control_informations {
            ies.push(ie.clone());
        }
        if let Some(ref ie) = self.reporting_control_information {
            ies.push(ie.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut srr_id = None;
        let mut access_availability_control_information = None;
        let mut qos_monitoring_per_qos_flow_control_informations = Vec::new();
        let mut direct_reporting_information = None;
        let mut traffic_parameter_measurement_control_informations = Vec::new();
        let mut reporting_control_information = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::SrrId => {
                    srr_id = Some(SrrId::unmarshal(&ie.payload)?);
                }
                IeType::AccessAvailabilityControlInformation => {
                    access_availability_control_information = Some(
                        AccessAvailabilityControlInformation::unmarshal(&ie.payload)?,
                    );
                }
                IeType::QosMonitoringPerQosFlowControlInformation => {
                    qos_monitoring_per_qos_flow_control_informations.push(ie);
                }
                IeType::DirectReportingInformation => {
                    direct_reporting_information = Some(ie);
                }
                IeType::TrafficParameterMeasurementControlInformation => {
                    traffic_parameter_measurement_control_informations.push(ie);
                }
                IeType::ReportingControlInformation => {
                    reporting_control_information = Some(ie);
                }
                _ => (),
            }
        }

        Ok(Self {
            srr_id: srr_id.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::SrrId, IeType::CreateSrr)
            })?,
            access_availability_control_information,
            qos_monitoring_per_qos_flow_control_informations,
            direct_reporting_information,
            traffic_parameter_measurement_control_informations,
            reporting_control_information,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateSrr, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::requested_access_availability_information::RequestedAccessAvailabilityInformation;

    fn make_aaci() -> AccessAvailabilityControlInformation {
        AccessAvailabilityControlInformation::new(
            RequestedAccessAvailabilityInformation::from_bits_truncate(0x01),
        )
    }

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = CreateSrr::new(SrrId::new(1), None);
        let parsed = CreateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = CreateSrr::new(SrrId::new(3), Some(make_aaci()));
        let parsed = CreateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_with_new_fields() {
        let mut original = CreateSrr::new(SrrId::new(2), None);
        original
            .qos_monitoring_per_qos_flow_control_informations
            .push(Ie::new(
                IeType::QosMonitoringPerQosFlowControlInformation,
                vec![0x01],
            ));
        original
            .traffic_parameter_measurement_control_informations
            .push(Ie::new(
                IeType::TrafficParameterMeasurementControlInformation,
                vec![0x02],
            ));
        original.direct_reporting_information =
            Some(Ie::new(IeType::DirectReportingInformation, vec![0x03]));
        original.reporting_control_information =
            Some(Ie::new(IeType::ReportingControlInformation, vec![0x04]));
        let parsed = CreateSrr::unmarshal(&original.marshal()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let ie = CreateSrr::new(SrrId::new(1), None).to_ie();
        assert_eq!(ie.ie_type, IeType::CreateSrr);
    }

    #[test]
    fn test_missing_srr_id() {
        assert!(matches!(
            CreateSrr::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
