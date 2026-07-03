//! QoS Monitoring per QoS flow Control Information IE (Grouped, IE Type 242).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.9-3, requests per-QFI QoS monitoring in an
//! SRR. Multiple QFIs may be listed; both RequestedQosMonitoring and
//! ReportingFrequency are mandatory.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QosMonitoringPerQosFlowControlInformation {
    /// QoS Flow Identifiers to monitor (mandatory, multiple).
    pub qfis: Vec<Ie>,
    /// Requested QoS parameter(s) to measure (mandatory).
    pub requested_qos_monitoring: Ie,
    /// Reporting type: event-triggered or periodic (mandatory).
    pub reporting_frequency: Ie,
    /// Packet delay threshold for event-triggered reporting (conditional).
    pub packet_delay_thresholds: Option<Ie>,
    /// Minimum wait time between consecutive reports (conditional).
    pub minimum_wait_time: Option<Ie>,
    /// Measurement / reporting period (conditional).
    pub measurement_period: Option<Ie>,
    /// Delay-tolerant reporting suggestion (conditional).
    pub reporting_suggestion_info: Option<Ie>,
    /// Measurement flags, e.g. DQFI (conditional).
    pub measurement_indication: Option<Ie>,
    /// Reporting thresholds for congestion/data-rate monitoring (conditional).
    pub reporting_thresholds: Option<Ie>,
}

impl QosMonitoringPerQosFlowControlInformation {
    pub fn new(requested_qos_monitoring: Ie, reporting_frequency: Ie) -> Self {
        Self {
            qfis: Vec::new(),
            requested_qos_monitoring,
            reporting_frequency,
            packet_delay_thresholds: None,
            minimum_wait_time: None,
            measurement_period: None,
            reporting_suggestion_info: None,
            measurement_indication: None,
            reporting_thresholds: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        for qfi in &self.qfis {
            ies.push(qfi.clone());
        }
        ies.push(self.requested_qos_monitoring.clone());
        ies.push(self.reporting_frequency.clone());
        if let Some(ref v) = self.packet_delay_thresholds {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.minimum_wait_time {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.measurement_period {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.reporting_suggestion_info {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.measurement_indication {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.reporting_thresholds {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qfis = Vec::new();
        let mut requested_qos_monitoring = None;
        let mut reporting_frequency = None;
        let mut packet_delay_thresholds = None;
        let mut minimum_wait_time = None;
        let mut measurement_period = None;
        let mut reporting_suggestion_info = None;
        let mut measurement_indication = None;
        let mut reporting_thresholds = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Qfi => qfis.push(ie),
                IeType::RequestedQosMonitoring => requested_qos_monitoring = Some(ie),
                IeType::ReportingFrequency => reporting_frequency = Some(ie),
                IeType::PacketDelayThresholds => packet_delay_thresholds = Some(ie),
                IeType::MinimumWaitTime => minimum_wait_time = Some(ie),
                IeType::MeasurementPeriod => measurement_period = Some(ie),
                IeType::ReportingSuggestionInfo => reporting_suggestion_info = Some(ie),
                IeType::MeasurementIndication => measurement_indication = Some(ie),
                IeType::ReportingThresholds => reporting_thresholds = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            qfis,
            requested_qos_monitoring: requested_qos_monitoring.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::RequestedQosMonitoring,
                    IeType::QosMonitoringPerQosFlowControlInformation,
                )
            })?,
            reporting_frequency: reporting_frequency.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::ReportingFrequency,
                    IeType::QosMonitoringPerQosFlowControlInformation,
                )
            })?,
            packet_delay_thresholds,
            minimum_wait_time,
            measurement_period,
            reporting_suggestion_info,
            measurement_indication,
            reporting_thresholds,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::QosMonitoringPerQosFlowControlInformation,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_requested_ie() -> Ie {
        Ie::new(IeType::RequestedQosMonitoring, vec![0x07])
    }

    fn make_freq_ie() -> Ie {
        Ie::new(IeType::ReportingFrequency, vec![0x01])
    }

    fn make_qfi_ie(qfi: u8) -> Ie {
        Ie::new(IeType::Qfi, vec![qfi])
    }

    #[test]
    fn test_round_trip_minimal() {
        let original =
            QosMonitoringPerQosFlowControlInformation::new(make_requested_ie(), make_freq_ie());
        let parsed =
            QosMonitoringPerQosFlowControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_with_qfis() {
        let mut original =
            QosMonitoringPerQosFlowControlInformation::new(make_requested_ie(), make_freq_ie());
        original.qfis.push(make_qfi_ie(5));
        original.qfis.push(make_qfi_ie(9));
        let parsed =
            QosMonitoringPerQosFlowControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let mut original =
            QosMonitoringPerQosFlowControlInformation::new(make_requested_ie(), make_freq_ie());
        original.qfis.push(make_qfi_ie(1));
        original.packet_delay_thresholds = Some(Ie::new(
            IeType::PacketDelayThresholds,
            vec![0x00, 0x00, 0x00, 0x0A],
        ));
        original.minimum_wait_time = Some(Ie::new(
            IeType::MinimumWaitTime,
            vec![0x00, 0x00, 0x00, 0x0A],
        ));
        original.measurement_period = Some(Ie::new(
            IeType::MeasurementPeriod,
            vec![0x00, 0x00, 0x00, 0x3C],
        ));
        original.reporting_suggestion_info =
            Some(Ie::new(IeType::ReportingSuggestionInfo, vec![0x01]));
        original.measurement_indication = Some(Ie::new(IeType::MeasurementIndication, vec![0x01]));
        original.reporting_thresholds =
            Some(Ie::new(IeType::ReportingThresholds, vec![0x00, 0x00, 0x01]));
        let parsed =
            QosMonitoringPerQosFlowControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_requested_qos_monitoring_fails() {
        let ie = Ie::new(IeType::ReportingFrequency, vec![0x01]);
        let bytes = marshal_ies(&[ie]);
        assert!(matches!(
            QosMonitoringPerQosFlowControlInformation::unmarshal(&bytes),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_missing_reporting_frequency_fails() {
        let ie = Ie::new(IeType::RequestedQosMonitoring, vec![0x07]);
        let bytes = marshal_ies(&[ie]);
        assert!(matches!(
            QosMonitoringPerQosFlowControlInformation::unmarshal(&bytes),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie =
            QosMonitoringPerQosFlowControlInformation::new(make_requested_ie(), make_freq_ie())
                .to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::QosMonitoringPerQosFlowControlInformation
        );
    }
}
