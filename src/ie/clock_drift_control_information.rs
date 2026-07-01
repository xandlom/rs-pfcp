//! Clock Drift Control Information IE (Grouped).
//!
//! Per 3GPP TS 29.244 Section 8.2.162, contains control parameters for
//! clock drift monitoring in TSN (Time Sensitive Networking).

use crate::error::PfcpError;
use crate::ie::cumulative_rate_ratio_threshold::CumulativeRateRatioThreshold;
use crate::ie::requested_clock_drift_information::RequestedClockDriftInformation;
use crate::ie::time_offset_threshold::TimeOffsetThreshold;
use crate::ie::tsn_time_domain_number::TsnTimeDomainNumber;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDriftControlInformation {
    pub requested_clock_drift_information: RequestedClockDriftInformation,
    pub tsn_time_domain_number: TsnTimeDomainNumber,
    pub time_offset_threshold: Option<TimeOffsetThreshold>,
    pub cumulative_rate_ratio_threshold: Option<CumulativeRateRatioThreshold>,
}

impl ClockDriftControlInformation {
    pub fn new(
        requested_clock_drift_information: RequestedClockDriftInformation,
        tsn_time_domain_number: TsnTimeDomainNumber,
        time_offset_threshold: Option<TimeOffsetThreshold>,
        cumulative_rate_ratio_threshold: Option<CumulativeRateRatioThreshold>,
    ) -> Self {
        Self {
            requested_clock_drift_information,
            tsn_time_domain_number,
            time_offset_threshold,
            cumulative_rate_ratio_threshold,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.requested_clock_drift_information.to_ie(),
            self.tsn_time_domain_number.to_ie(),
        ];
        if let Some(tot) = &self.time_offset_threshold {
            ies.push(tot.to_ie());
        }
        if let Some(crrt) = &self.cumulative_rate_ratio_threshold {
            ies.push(crrt.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut requested_clock_drift_information = None;
        let mut tsn_time_domain_number = None;
        let mut time_offset_threshold = None;
        let mut cumulative_rate_ratio_threshold = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::RequestedClockDriftInformation => {
                    requested_clock_drift_information =
                        Some(RequestedClockDriftInformation::unmarshal(&ie.payload)?);
                }
                IeType::TsnTimeDomainNumber => {
                    tsn_time_domain_number = Some(TsnTimeDomainNumber::unmarshal(&ie.payload)?);
                }
                IeType::TimeOffsetThreshold => {
                    time_offset_threshold = Some(TimeOffsetThreshold::unmarshal(&ie.payload)?);
                }
                IeType::CumulativeRateRatioThreshold => {
                    cumulative_rate_ratio_threshold =
                        Some(CumulativeRateRatioThreshold::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(Self {
            requested_clock_drift_information: requested_clock_drift_information.ok_or_else(
                || {
                    PfcpError::missing_ie_in_grouped(
                        IeType::RequestedClockDriftInformation,
                        IeType::ClockDriftControlInformation,
                    )
                },
            )?,
            tsn_time_domain_number: tsn_time_domain_number.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::TsnTimeDomainNumber,
                    IeType::ClockDriftControlInformation,
                )
            })?,
            time_offset_threshold,
            cumulative_rate_ratio_threshold,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ClockDriftControlInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_minimal() {
        let original = ClockDriftControlInformation::new(
            RequestedClockDriftInformation::RRTO,
            TsnTimeDomainNumber::new(1),
            None,
            None,
        );
        let marshaled = original.marshal();
        let parsed = ClockDriftControlInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_marshal_unmarshal_full() {
        let original = ClockDriftControlInformation::new(
            RequestedClockDriftInformation::RRTO | RequestedClockDriftInformation::RRCR,
            TsnTimeDomainNumber::new(2),
            Some(TimeOffsetThreshold::new(1_000_000)),
            Some(CumulativeRateRatioThreshold::new(500)),
        );
        let marshaled = original.marshal();
        let parsed = ClockDriftControlInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_to_ie() {
        let cdci = ClockDriftControlInformation::new(
            RequestedClockDriftInformation::RRCR,
            TsnTimeDomainNumber::new(0),
            None,
            None,
        );
        assert_eq!(cdci.to_ie().ie_type, IeType::ClockDriftControlInformation);
    }

    #[test]
    fn test_missing_mandatory_ie() {
        let empty: &[u8] = &[];
        assert!(matches!(
            ClockDriftControlInformation::unmarshal(empty),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }
}
