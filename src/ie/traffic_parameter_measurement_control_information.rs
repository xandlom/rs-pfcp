//! Traffic Parameter Measurement Control Information IE (Grouped, IE Type 323).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.9-5, requests the UPF to perform traffic
//! parameter measurement for one or more QoS flows. Multiple QFIs may be
//! listed; either MeasurementPeriod or TrafficParameterThreshold MUST be present.

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficParameterMeasurementControlInformation {
    /// QoS Flow Identifiers to measure (mandatory, multiple).
    pub qfis: Vec<Ie>,
    /// Requested traffic parameter measurement flags (mandatory).
    pub traffic_parameter_measurement_indication: Ie,
    /// Measurement period for periodic reporting (conditional).
    pub measurement_period: Option<Ie>,
    /// DL Periodicity for N6 jitter measurement (conditional).
    pub dl_periodicity: Option<Ie>,
    /// Threshold for threshold-triggered reporting (conditional).
    pub traffic_parameter_threshold: Option<Ie>,
}

impl TrafficParameterMeasurementControlInformation {
    pub fn new(traffic_parameter_measurement_indication: Ie) -> Self {
        Self {
            qfis: Vec::new(),
            traffic_parameter_measurement_indication,
            measurement_period: None,
            dl_periodicity: None,
            traffic_parameter_threshold: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        for qfi in &self.qfis {
            ies.push(qfi.clone());
        }
        ies.push(self.traffic_parameter_measurement_indication.clone());
        if let Some(ref v) = self.measurement_period {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.dl_periodicity {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.traffic_parameter_threshold {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut qfis = Vec::new();
        let mut traffic_parameter_measurement_indication = None;
        let mut measurement_period = None;
        let mut dl_periodicity = None;
        let mut traffic_parameter_threshold = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::Qfi => qfis.push(ie),
                IeType::TrafficParameterMeasurementIndication => {
                    traffic_parameter_measurement_indication = Some(ie);
                }
                IeType::MeasurementPeriod => measurement_period = Some(ie),
                IeType::DlPeriodicity => dl_periodicity = Some(ie),
                IeType::TrafficParameterThreshold => traffic_parameter_threshold = Some(ie),
                _ => (),
            }
        }

        Ok(Self {
            qfis,
            traffic_parameter_measurement_indication: traffic_parameter_measurement_indication
                .ok_or_else(|| {
                    PfcpError::missing_ie_in_grouped(
                        IeType::TrafficParameterMeasurementIndication,
                        IeType::TrafficParameterMeasurementControlInformation,
                    )
                })?,
            measurement_period,
            dl_periodicity,
            traffic_parameter_threshold,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::TrafficParameterMeasurementControlInformation,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_indication_ie() -> Ie {
        Ie::new(IeType::TrafficParameterMeasurementIndication, vec![0x07])
    }

    fn make_qfi_ie(qfi: u8) -> Ie {
        Ie::new(IeType::Qfi, vec![qfi])
    }

    #[test]
    fn test_round_trip_minimal() {
        let original = TrafficParameterMeasurementControlInformation::new(make_indication_ie());
        let parsed =
            TrafficParameterMeasurementControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_with_qfis() {
        let mut original = TrafficParameterMeasurementControlInformation::new(make_indication_ie());
        original.qfis.push(make_qfi_ie(5));
        original.qfis.push(make_qfi_ie(9));
        let parsed =
            TrafficParameterMeasurementControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let mut original = TrafficParameterMeasurementControlInformation::new(make_indication_ie());
        original.qfis.push(make_qfi_ie(3));
        original.measurement_period = Some(Ie::new(
            IeType::MeasurementPeriod,
            vec![0x00, 0x00, 0x00, 0x3C],
        ));
        original.dl_periodicity =
            Some(Ie::new(IeType::DlPeriodicity, vec![0x00, 0x00, 0x01, 0x2C]));
        original.traffic_parameter_threshold =
            Some(Ie::new(IeType::TrafficParameterThreshold, vec![0x00, 0x0A]));
        let parsed =
            TrafficParameterMeasurementControlInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_missing_indication_fails() {
        assert!(matches!(
            TrafficParameterMeasurementControlInformation::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        let ie = TrafficParameterMeasurementControlInformation::new(make_indication_ie()).to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::TrafficParameterMeasurementControlInformation
        );
    }
}
