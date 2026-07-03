//! Traffic Parameter Measurement Report IE (Grouped, IE Type 324).
//!
//! Per 3GPP TS 29.244 Table 7.5.8.6-4, reports traffic parameter measurements
//! for a specific QFI, including optional N6 jitter and UL periodicity.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficParameterMeasurementReport {
    /// QFI identifying the measured QoS flow (mandatory).
    pub qfi: Ie,
    /// N6 Jitter Measurement (conditional — present when N6 jitter is available).
    pub n6_jitter_measurement: Option<Ie>,
    /// UL Periodicity (conditional — present when UL periodicity was requested and available).
    pub ul_periodicity: Option<Ie>,
    /// Timestamp when collection was generated (mandatory, IE type 156 = Time Stamp).
    pub time_stamp: Ie,
    /// Timestamp when collection started (optional).
    pub start_time: Option<Ie>,
}

impl TrafficParameterMeasurementReport {
    pub fn new(qfi: Ie, time_stamp: Ie) -> Self {
        Self {
            qfi,
            n6_jitter_measurement: None,
            ul_periodicity: None,
            time_stamp,
            start_time: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.qfi.clone()];
        if let Some(ref v) = self.n6_jitter_measurement {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.ul_periodicity {
            ies.push(v.clone());
        }
        ies.push(self.time_stamp.clone());
        if let Some(ref st) = self.start_time {
            ies.push(st.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qfi = None;
        let mut n6_jitter_measurement = None;
        let mut ul_periodicity = None;
        let mut time_stamp = None;
        let mut start_time = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Qfi => qfi = Some(ie),
                IeType::N6JitterMeasurement => n6_jitter_measurement = Some(ie),
                IeType::UlPeriodicity => ul_periodicity = Some(ie),
                IeType::EventTimeStamp => time_stamp = Some(ie),
                IeType::StartTime => start_time = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            qfi: qfi.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::Qfi,
                    IeType::TrafficParameterMeasurementReport,
                )
            })?,
            n6_jitter_measurement,
            ul_periodicity,
            time_stamp: time_stamp.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::EventTimeStamp,
                    IeType::TrafficParameterMeasurementReport,
                )
            })?,
            start_time,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TrafficParameterMeasurementReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_qfi_ie() -> Ie {
        Ie::new(IeType::Qfi, vec![0x05])
    }

    fn make_timestamp_ie() -> Ie {
        Ie::new(IeType::EventTimeStamp, vec![0xEB, 0x24, 0x00, 0x00])
    }

    fn make_n6_jitter_ie() -> Ie {
        Ie::new(IeType::N6JitterMeasurement, vec![0x00, 0x00, 0x00, 0x0A])
    }

    fn make_ul_periodicity_ie() -> Ie {
        Ie::new(IeType::UlPeriodicity, vec![0x00, 0x00, 0x00, 0x64])
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = TrafficParameterMeasurementReport::new(make_qfi_ie(), make_timestamp_ie());
        let parsed = TrafficParameterMeasurementReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let mut original =
            TrafficParameterMeasurementReport::new(make_qfi_ie(), make_timestamp_ie());
        original.n6_jitter_measurement = Some(make_n6_jitter_ie());
        original.ul_periodicity = Some(make_ul_periodicity_ie());
        original.start_time = Some(Ie::new(IeType::StartTime, vec![0xEB, 0x23, 0x00, 0x00]));
        let parsed = TrafficParameterMeasurementReport::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_qfi_fails() {
        assert!(matches!(
            TrafficParameterMeasurementReport::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = TrafficParameterMeasurementReport::new(make_qfi_ie(), make_timestamp_ie()).to_ie();
        assert_eq!(ie.ie_type, IeType::TrafficParameterMeasurementReport);
    }
}
