//! Clock Drift Report IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 8.2.163, reports measured clock drift values
//! for TSN (Time Sensitive Networking) time domain synchronization.

use crate::error::PfcpError;
use crate::ie::cumulative_rate_ratio_measurement::CumulativeRateRatioMeasurement;
use crate::ie::time_offset_measurement::TimeOffsetMeasurement;
use crate::ie::tsn_time_domain_number::TsnTimeDomainNumber;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDriftReport {
    pub tsn_time_domain_number: TsnTimeDomainNumber,
    pub time_offset_measurement: Option<TimeOffsetMeasurement>,
    pub cumulative_rate_ratio_measurement: Option<CumulativeRateRatioMeasurement>,
}

impl ClockDriftReport {
    pub fn new(
        tsn_time_domain_number: TsnTimeDomainNumber,
        time_offset_measurement: Option<TimeOffsetMeasurement>,
        cumulative_rate_ratio_measurement: Option<CumulativeRateRatioMeasurement>,
    ) -> Self {
        Self {
            tsn_time_domain_number,
            time_offset_measurement,
            cumulative_rate_ratio_measurement,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.tsn_time_domain_number.to_ie()];
        if let Some(tom) = &self.time_offset_measurement {
            ies.push(tom.to_ie());
        }
        if let Some(crrm) = &self.cumulative_rate_ratio_measurement {
            ies.push(crrm.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut tsn_time_domain_number = None;
        let mut time_offset_measurement = None;
        let mut cumulative_rate_ratio_measurement = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::TsnTimeDomainNumber => {
                    tsn_time_domain_number = Some(TsnTimeDomainNumber::unmarshal(&ie.payload)?);
                }
                IeType::TimeOffsetMeasurement => {
                    time_offset_measurement = Some(TimeOffsetMeasurement::unmarshal(&ie.payload)?);
                }
                IeType::CumulativeRateRatioMeasurement => {
                    cumulative_rate_ratio_measurement =
                        Some(CumulativeRateRatioMeasurement::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            tsn_time_domain_number: tsn_time_domain_number.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::TsnTimeDomainNumber,
                    IeType::ClockDriftReport,
                )
            })?,
            time_offset_measurement,
            cumulative_rate_ratio_measurement,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ClockDriftReport, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = ClockDriftReport::new(TsnTimeDomainNumber::new(1), None, None);
        let marshaled = original.marshal();
        let parsed = ClockDriftReport::unmarshal(&marshaled).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = ClockDriftReport::new(
            TsnTimeDomainNumber::new(3),
            Some(TimeOffsetMeasurement::new(2_000_000)),
            Some(CumulativeRateRatioMeasurement::new(-100)),
        );
        let marshaled = original.marshal();
        let parsed = ClockDriftReport::unmarshal(&marshaled).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let report = ClockDriftReport::new(TsnTimeDomainNumber::new(0), None, None);
        assert_eq!(report.to_ie().ie_type, IeType::ClockDriftReport);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let empty: &[u8] = &[];
        assert!(matches!(
            ClockDriftReport::unmarshal(empty),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
