//! Direct Reporting Information IE (Grouped, IE Type 295).
//!
//! Per 3GPP TS 29.244 Table 7.5.2.9-4, provides information for the UPF to
//! report QoS monitoring events directly to a local NEF or AF (DRQOS feature).

use crate::error::PfcpError;
use crate::ie::{marshal_ies, Ie, IeIterator, IeType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DirectReportingInformation {
    /// Event Notification URI (M): URI for sending UPF event notifications.
    pub event_notification_uri: Option<Ie>,
    /// Notification Correlation ID (C): correlation ID in notifications.
    pub notification_correlation_id: Option<Ie>,
    /// Reporting Flags (C): present when at least one flag is set to 1.
    pub reporting_flags: Option<Ie>,
}

impl DirectReportingInformation {
    pub fn new(event_notification_uri: Ie) -> Self {
        Self {
            event_notification_uri: Some(event_notification_uri),
            notification_correlation_id: None,
            reporting_flags: None,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();
        if let Some(ref v) = self.event_notification_uri {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.notification_correlation_id {
            ies.push(v.clone());
        }
        if let Some(ref v) = self.reporting_flags {
            ies.push(v.clone());
        }
        marshal_ies(&ies)
    }

    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut event_notification_uri = None;
        let mut notification_correlation_id = None;
        let mut reporting_flags = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
            match ie.ie_type {
                IeType::EventNotificationUri => event_notification_uri = Some(ie),
                IeType::NotificationCorrelationId => {
                    notification_correlation_id = Some(ie);
                }
                IeType::ReportingFlags => reporting_flags = Some(ie),
                _ => (),
            }
        }
        Ok(Self {
            event_notification_uri,
            notification_correlation_id,
            reporting_flags,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DirectReportingInformation, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_uri_ie() -> Ie {
        Ie::new(
            IeType::EventNotificationUri,
            b"https://nef.local/qos".to_vec(),
        )
    }

    fn make_corr_ie() -> Ie {
        Ie::new(IeType::NotificationCorrelationId, vec![0x01, 0x02])
    }

    fn make_flags_ie() -> Ie {
        Ie::new(IeType::ReportingFlags, vec![0x01]) // DUPL=1
    }

    #[test]
    fn test_round_trip_uri_only() {
        let original = DirectReportingInformation::new(make_uri_ie());
        let parsed = DirectReportingInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_full() {
        let original = DirectReportingInformation {
            event_notification_uri: Some(make_uri_ie()),
            notification_correlation_id: Some(make_corr_ie()),
            reporting_flags: Some(make_flags_ie()),
        };
        let parsed = DirectReportingInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_round_trip_empty() {
        let original = DirectReportingInformation::default();
        let parsed = DirectReportingInformation::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_ie_type() {
        let ie = DirectReportingInformation::new(make_uri_ie()).to_ie();
        assert_eq!(ie.ie_type, IeType::DirectReportingInformation);
    }
}
