//! Additional Monitoring Time Information Element.
//!
//! Per 3GPP TS 29.244 Section 7.5.2.4-3, specifies an additional monitoring
//! time and associated subsequent thresholds/quotas for usage reporting.

use crate::error::PfcpError;
use crate::ie::monitoring_time::MonitoringTime;
use crate::ie::subsequent_event_quota::SubsequentEventQuota;
use crate::ie::subsequent_event_threshold::SubsequentEventThreshold;
use crate::ie::subsequent_time_quota::SubsequentTimeQuota;
use crate::ie::subsequent_time_threshold::SubsequentTimeThreshold;
use crate::ie::subsequent_volume_quota::SubsequentVolumeQuota;
use crate::ie::subsequent_volume_threshold::SubsequentVolumeThreshold;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

/// Additional Monitoring Time per 3GPP TS 29.244 §7.5.2.4-3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalMonitoringTime {
    /// Additional monitoring time (mandatory).
    pub monitoring_time: MonitoringTime,
    pub subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
    pub subsequent_time_threshold: Option<SubsequentTimeThreshold>,
    pub subsequent_volume_quota: Option<SubsequentVolumeQuota>,
    pub subsequent_time_quota: Option<SubsequentTimeQuota>,
    pub subsequent_event_threshold: Option<SubsequentEventThreshold>,
    pub subsequent_event_quota: Option<SubsequentEventQuota>,
}

impl AdditionalMonitoringTime {
    pub fn new(monitoring_time: MonitoringTime) -> Self {
        AdditionalMonitoringTime {
            monitoring_time,
            subsequent_volume_threshold: None,
            subsequent_time_threshold: None,
            subsequent_volume_quota: None,
            subsequent_time_quota: None,
            subsequent_event_threshold: None,
            subsequent_event_quota: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![Ie::new(
            IeType::MonitoringTime,
            self.monitoring_time.marshal().to_vec(),
        )];
        if let Some(svt) = &self.subsequent_volume_threshold {
            ies.push(Ie::new(IeType::SubsequentVolumeThreshold, svt.marshal()));
        }
        if let Some(stt) = &self.subsequent_time_threshold {
            ies.push(Ie::new(
                IeType::SubsequentTimeThreshold,
                stt.marshal().to_vec(),
            ));
        }
        if let Some(svq) = &self.subsequent_volume_quota {
            ies.push(svq.to_ie());
        }
        if let Some(stq) = &self.subsequent_time_quota {
            ies.push(stq.to_ie());
        }
        if let Some(set) = &self.subsequent_event_threshold {
            ies.push(set.to_ie());
        }
        if let Some(seq) = &self.subsequent_event_quota {
            ies.push(seq.to_ie());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut monitoring_time = None;
        let mut subsequent_volume_threshold = None;
        let mut subsequent_time_threshold = None;
        let mut subsequent_volume_quota = None;
        let mut subsequent_time_quota = None;
        let mut subsequent_event_threshold = None;
        let mut subsequent_event_quota = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::MonitoringTime => {
                    monitoring_time = Some(MonitoringTime::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentVolumeThreshold => {
                    subsequent_volume_threshold =
                        Some(SubsequentVolumeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentTimeThreshold => {
                    subsequent_time_threshold =
                        Some(SubsequentTimeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentVolumeQuota => {
                    subsequent_volume_quota = Some(SubsequentVolumeQuota::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentTimeQuota => {
                    subsequent_time_quota = Some(SubsequentTimeQuota::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentEventThreshold => {
                    subsequent_event_threshold =
                        Some(SubsequentEventThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentEventQuota => {
                    subsequent_event_quota = Some(SubsequentEventQuota::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
        }

        Ok(AdditionalMonitoringTime {
            monitoring_time: monitoring_time.ok_or_else(|| {
                PfcpError::missing_ie_in_grouped(
                    IeType::MonitoringTime,
                    IeType::AdditionalMonitoringTime,
                )
            })?,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            subsequent_volume_quota,
            subsequent_time_quota,
            subsequent_event_threshold,
            subsequent_event_quota,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AdditionalMonitoringTime, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    fn make_monitoring_time() -> MonitoringTime {
        MonitoringTime::new(UNIX_EPOCH + Duration::from_secs(1_700_000_000))
    }

    #[test]
    fn test_marshal_unmarshal_monitoring_time_only() {
        let ie = AdditionalMonitoringTime::new(make_monitoring_time());
        let parsed = AdditionalMonitoringTime::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_time_threshold() {
        let mut ie = AdditionalMonitoringTime::new(make_monitoring_time());
        ie.subsequent_time_threshold = Some(SubsequentTimeThreshold::new(3600));
        let parsed = AdditionalMonitoringTime::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_marshal_unmarshal_with_event_quota() {
        let mut ie = AdditionalMonitoringTime::new(make_monitoring_time());
        ie.subsequent_event_quota = Some(SubsequentEventQuota::new(100));
        let parsed = AdditionalMonitoringTime::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_missing_monitoring_time() {
        assert!(matches!(
            AdditionalMonitoringTime::unmarshal(&[]),
            Err(PfcpError::MissingMandatoryIe { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AdditionalMonitoringTime::new(make_monitoring_time()).to_ie();
        assert_eq!(ie.ie_type, IeType::AdditionalMonitoringTime);
        assert!(!ie.payload.is_empty());
    }
}
