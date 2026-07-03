//! QoS Monitoring Report IE (Grouped, IE Type 247).
//!
//! Per 3GPP TS 29.244 Table 7.5.8.6-3, reports QoS monitoring results for a
//! specific QFI, including the measurement, timestamp, and optional start time.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QosMonitoringReport {
    /// QFI identifying the monitored QoS flow (mandatory).
    pub qfi: Ie,
    /// QoS Monitoring Measurement containing packet delay(s) (mandatory).
    pub qos_monitoring_measurement: Ie,
    /// Timestamp when the collection was generated (mandatory, IE type 156 = Time Stamp).
    pub time_stamp: Ie,
    /// Timestamp when collection started (optional).
    pub start_time: Option<Ie>,
}

impl QosMonitoringReport {
    pub fn new(qfi: Ie, qos_monitoring_measurement: Ie, time_stamp: Ie) -> Self {
        Self {
            qfi,
            qos_monitoring_measurement,
            time_stamp,
            start_time: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.qfi.clone(),
            self.qos_monitoring_measurement.clone(),
            self.time_stamp.clone(),
        ];
        if let Some(ref st) = self.start_time {
            ies.push(st.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qfi = None;
        let mut qos_monitoring_measurement = None;
        let mut time_stamp = None;
        let mut start_time = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Qfi => qfi = Some(ie),
                IeType::QosMonitoringMeasurement => qos_monitoring_measurement = Some(ie),
                IeType::EventTimeStamp => time_stamp = Some(ie),
                IeType::StartTime => start_time = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            qfi: qfi.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(IeType::Qfi, IeType::QosMonitoringReport)
            })?,
            qos_monitoring_measurement: qos_monitoring_measurement.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::QosMonitoringMeasurement,
                    IeType::QosMonitoringReport,
                )
            })?,
            time_stamp: time_stamp.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::EventTimeStamp,
                    IeType::QosMonitoringReport,
                )
            })?,
            start_time,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QosMonitoringReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_qfi_ie() -> Ie {
        Ie::new(IeType::Qfi, vec![0x01])
    }

    fn make_qmm_ie() -> Ie {
        Ie::new(
            IeType::QosMonitoringMeasurement,
            vec![0x01, 0x00, 0x00, 0x00, 0x05],
        )
    }

    fn make_timestamp_ie() -> Ie {
        Ie::new(IeType::EventTimeStamp, vec![0xEB, 0x24, 0x00, 0x00])
    }

    fn make_start_time_ie() -> Ie {
        Ie::new(IeType::StartTime, vec![0xEB, 0x23, 0x00, 0x00])
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = QosMonitoringReport::new(make_qfi_ie(), make_qmm_ie(), make_timestamp_ie());
        let parsed = QosMonitoringReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let mut original =
            QosMonitoringReport::new(make_qfi_ie(), make_qmm_ie(), make_timestamp_ie());
        original.start_time = Some(make_start_time_ie());
        let parsed = QosMonitoringReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_qfi_fails() {
        assert!(matches!(
            QosMonitoringReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie =
            QosMonitoringReport::new(make_qfi_ie(), make_qmm_ie(), make_timestamp_ie()).to_ie();
        assert_eq!(ie.ie_type, IeType::QosMonitoringReport);
    }
}
