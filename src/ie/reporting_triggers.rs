//! ReportingTriggers IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Represents Reporting Triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportingTriggers {
    pub periodic: bool,
    pub volume_threshold: bool,
    pub time_threshold: bool,
    pub quota_exhausted: bool,
    pub start_of_traffic: bool,
    pub stop_of_traffic: bool,
    pub dropped_dl_traffic: bool,
    pub linked_urr: bool,
    pub event_threshold: bool,
}

impl ReportingTriggers {
    /// Creates a new Reporting Triggers.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_periodic(mut self, periodic: bool) -> Self {
        self.periodic = periodic;
        self
    }

    pub fn with_volume_threshold(mut self, volume_threshold: bool) -> Self {
        self.volume_threshold = volume_threshold;
        self
    }

    pub fn with_time_threshold(mut self, time_threshold: bool) -> Self {
        self.time_threshold = time_threshold;
        self
    }

    pub fn with_quota_exhausted(mut self, quota_exhausted: bool) -> Self {
        self.quota_exhausted = quota_exhausted;
        self
    }

    pub fn with_start_of_traffic(mut self, start_of_traffic: bool) -> Self {
        self.start_of_traffic = start_of_traffic;
        self
    }

    pub fn with_stop_of_traffic(mut self, stop_of_traffic: bool) -> Self {
        self.stop_of_traffic = stop_of_traffic;
        self
    }

    pub fn with_dropped_dl_traffic(mut self, dropped_dl_traffic: bool) -> Self {
        self.dropped_dl_traffic = dropped_dl_traffic;
        self
    }

    pub fn with_linked_urr(mut self, linked_urr: bool) -> Self {
        self.linked_urr = linked_urr;
        self
    }

    pub fn with_event_threshold(mut self, event_threshold: bool) -> Self {
        self.event_threshold = event_threshold;
        self
    }

    /// Marshals the Reporting Triggers into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut b = [0; 2];
        if self.periodic {
            b[0] |= 1;
        }
        if self.volume_threshold {
            b[0] |= 2;
        }
        if self.time_threshold {
            b[0] |= 4;
        }
        if self.quota_exhausted {
            b[0] |= 8;
        }
        if self.start_of_traffic {
            b[0] |= 16;
        }
        if self.stop_of_traffic {
            b[0] |= 32;
        }
        if self.dropped_dl_traffic {
            b[0] |= 64;
        }
        if self.linked_urr {
            b[0] |= 128;
        }
        if self.event_threshold {
            b[1] |= 1;
        }
        b.to_vec()
    }

    /// Unmarshals a byte slice into a Reporting Triggers.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        if payload.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Reporting Triggers",
                IeType::ReportingTriggers,
                2,
                payload.len(),
            ));
        }
        Ok(ReportingTriggers {
            periodic: (payload[0] & 1) != 0,
            volume_threshold: (payload[0] & 2) != 0,
            time_threshold: (payload[0] & 4) != 0,
            quota_exhausted: (payload[0] & 8) != 0,
            start_of_traffic: (payload[0] & 16) != 0,
            stop_of_traffic: (payload[0] & 32) != 0,
            dropped_dl_traffic: (payload[0] & 64) != 0,
            linked_urr: (payload[0] & 128) != 0,
            event_threshold: (payload[1] & 1) != 0,
        })
    }

    /// Wraps the Reporting Triggers in a ReportingTriggers IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingTriggers, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporting_triggers_new() {
        let triggers = ReportingTriggers::new();
        assert!(!triggers.periodic);
        assert!(!triggers.volume_threshold);
        assert!(!triggers.time_threshold);
        assert!(!triggers.quota_exhausted);
        assert!(!triggers.start_of_traffic);
        assert!(!triggers.stop_of_traffic);
        assert!(!triggers.dropped_dl_traffic);
        assert!(!triggers.linked_urr);
        assert!(!triggers.event_threshold);
    }

    #[test]
    fn test_reporting_triggers_single_periodic() {
        let triggers = ReportingTriggers::new().with_periodic(true);
        assert!(triggers.periodic);
        assert!(!triggers.volume_threshold);
        assert!(!triggers.time_threshold);
    }

    #[test]
    fn test_reporting_triggers_single_volume_threshold() {
        let triggers = ReportingTriggers::new().with_volume_threshold(true);
        assert!(!triggers.periodic);
        assert!(triggers.volume_threshold);
        assert!(!triggers.time_threshold);
    }

    #[test]
    fn test_reporting_triggers_single_time_threshold() {
        let triggers = ReportingTriggers::new().with_time_threshold(true);
        assert!(!triggers.periodic);
        assert!(!triggers.volume_threshold);
        assert!(triggers.time_threshold);
    }

    #[test]
    fn test_reporting_triggers_single_quota_exhausted() {
        let triggers = ReportingTriggers::new().with_quota_exhausted(true);
        assert!(triggers.quota_exhausted);
        assert!(!triggers.volume_threshold);
    }

    #[test]
    fn test_reporting_triggers_single_start_of_traffic() {
        let triggers = ReportingTriggers::new().with_start_of_traffic(true);
        assert!(triggers.start_of_traffic);
        assert!(!triggers.stop_of_traffic);
    }

    #[test]
    fn test_reporting_triggers_single_stop_of_traffic() {
        let triggers = ReportingTriggers::new().with_stop_of_traffic(true);
        assert!(!triggers.start_of_traffic);
        assert!(triggers.stop_of_traffic);
    }

    #[test]
    fn test_reporting_triggers_single_dropped_dl_traffic() {
        let triggers = ReportingTriggers::new().with_dropped_dl_traffic(true);
        assert!(triggers.dropped_dl_traffic);
        assert!(!triggers.start_of_traffic);
    }

    #[test]
    fn test_reporting_triggers_single_linked_urr() {
        let triggers = ReportingTriggers::new().with_linked_urr(true);
        assert!(triggers.linked_urr);
        assert!(!triggers.dropped_dl_traffic);
    }

    #[test]
    fn test_reporting_triggers_single_event_threshold() {
        let triggers = ReportingTriggers::new().with_event_threshold(true);
        assert!(triggers.event_threshold);
        assert!(!triggers.periodic);
    }

    #[test]
    fn test_reporting_triggers_volume_and_time() {
        let triggers = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true);
        assert!(triggers.volume_threshold);
        assert!(triggers.time_threshold);
        assert!(!triggers.periodic);
    }

    #[test]
    fn test_reporting_triggers_all_threshold_types() {
        let triggers = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_event_threshold(true);
        assert!(triggers.volume_threshold);
        assert!(triggers.time_threshold);
        assert!(triggers.event_threshold);
        assert!(!triggers.quota_exhausted);
    }

    #[test]
    fn test_reporting_triggers_traffic_events() {
        let triggers = ReportingTriggers::new()
            .with_start_of_traffic(true)
            .with_stop_of_traffic(true)
            .with_dropped_dl_traffic(true);
        assert!(triggers.start_of_traffic);
        assert!(triggers.stop_of_traffic);
        assert!(triggers.dropped_dl_traffic);
    }

    #[test]
    fn test_reporting_triggers_all_flags_set() {
        let triggers = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_quota_exhausted(true)
            .with_start_of_traffic(true)
            .with_stop_of_traffic(true)
            .with_dropped_dl_traffic(true)
            .with_linked_urr(true)
            .with_event_threshold(true);

        assert!(triggers.periodic);
        assert!(triggers.volume_threshold);
        assert!(triggers.time_threshold);
        assert!(triggers.quota_exhausted);
        assert!(triggers.start_of_traffic);
        assert!(triggers.stop_of_traffic);
        assert!(triggers.dropped_dl_traffic);
        assert!(triggers.linked_urr);
        assert!(triggers.event_threshold);
    }

    #[test]
    fn test_reporting_triggers_marshal_empty() {
        let triggers = ReportingTriggers::new();
        let marshaled = triggers.marshal();
        assert_eq!(marshaled, vec![0, 0]);
    }

    #[test]
    fn test_reporting_triggers_marshal_periodic() {
        let triggers = ReportingTriggers::new().with_periodic(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 1, 1);
        assert_eq!(marshaled[1], 0);
    }

    #[test]
    fn test_reporting_triggers_marshal_volume_threshold() {
        let triggers = ReportingTriggers::new().with_volume_threshold(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 2, 2);
    }

    #[test]
    fn test_reporting_triggers_marshal_time_threshold() {
        let triggers = ReportingTriggers::new().with_time_threshold(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 4, 4);
    }

    #[test]
    fn test_reporting_triggers_marshal_quota_exhausted() {
        let triggers = ReportingTriggers::new().with_quota_exhausted(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 8, 8);
    }

    #[test]
    fn test_reporting_triggers_marshal_start_of_traffic() {
        let triggers = ReportingTriggers::new().with_start_of_traffic(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 16, 16);
    }

    #[test]
    fn test_reporting_triggers_marshal_stop_of_traffic() {
        let triggers = ReportingTriggers::new().with_stop_of_traffic(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 32, 32);
    }

    #[test]
    fn test_reporting_triggers_marshal_dropped_dl_traffic() {
        let triggers = ReportingTriggers::new().with_dropped_dl_traffic(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 64, 64);
    }

    #[test]
    fn test_reporting_triggers_marshal_linked_urr() {
        let triggers = ReportingTriggers::new().with_linked_urr(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0] & 128, 128);
    }

    #[test]
    fn test_reporting_triggers_marshal_event_threshold() {
        let triggers = ReportingTriggers::new().with_event_threshold(true);
        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0], 0);
        assert_eq!(marshaled[1] & 1, 1);
    }

    #[test]
    fn test_reporting_triggers_marshal_all_flags() {
        let triggers = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_quota_exhausted(true)
            .with_start_of_traffic(true)
            .with_stop_of_traffic(true)
            .with_dropped_dl_traffic(true)
            .with_linked_urr(true)
            .with_event_threshold(true);

        let marshaled = triggers.marshal();
        assert_eq!(marshaled[0], 0xFF); // All 8 bits set
        assert_eq!(marshaled[1], 0x01); // Event threshold bit set
    }

    #[test]
    fn test_reporting_triggers_unmarshal_empty() {
        let data = vec![0, 0];
        let triggers = ReportingTriggers::unmarshal(&data).unwrap();
        assert!(!triggers.periodic);
        assert!(!triggers.volume_threshold);
        assert!(!triggers.event_threshold);
    }

    #[test]
    fn test_reporting_triggers_unmarshal_periodic() {
        let data = vec![0x01, 0x00];
        let triggers = ReportingTriggers::unmarshal(&data).unwrap();
        assert!(triggers.periodic);
        assert!(!triggers.volume_threshold);
    }

    #[test]
    fn test_reporting_triggers_unmarshal_all_flags() {
        let data = vec![0xFF, 0x01];
        let triggers = ReportingTriggers::unmarshal(&data).unwrap();
        assert!(triggers.periodic);
        assert!(triggers.volume_threshold);
        assert!(triggers.time_threshold);
        assert!(triggers.quota_exhausted);
        assert!(triggers.start_of_traffic);
        assert!(triggers.stop_of_traffic);
        assert!(triggers.dropped_dl_traffic);
        assert!(triggers.linked_urr);
        assert!(triggers.event_threshold);
    }

    #[test]
    fn test_reporting_triggers_unmarshal_short_buffer() {
        let data = vec![0x01];
        let result = ReportingTriggers::unmarshal(&data);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_reporting_triggers_unmarshal_empty_buffer() {
        let data = vec![];
        let result = ReportingTriggers::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_reporting_triggers_round_trip_empty() {
        let original = ReportingTriggers::new();
        let marshaled = original.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_round_trip_single_flag() {
        let original = ReportingTriggers::new().with_periodic(true);
        let marshaled = original.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_round_trip_multiple_flags() {
        let original = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_quota_exhausted(true);

        let marshaled = original.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_round_trip_all_flags() {
        let original = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_quota_exhausted(true)
            .with_start_of_traffic(true)
            .with_stop_of_traffic(true)
            .with_dropped_dl_traffic(true)
            .with_linked_urr(true)
            .with_event_threshold(true);

        let marshaled = original.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_to_ie() {
        let triggers = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(true);

        let ie = triggers.to_ie();
        assert_eq!(ie.ie_type, IeType::ReportingTriggers);
        assert_eq!(ie.payload.len(), 2);
    }

    #[test]
    fn test_reporting_triggers_real_world_volume_time_quota() {
        // Common scenario: Report on volume, time, or quota exhaustion
        let triggers = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true)
            .with_quota_exhausted(true);

        let marshaled = triggers.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(triggers, unmarshaled);
        assert!(unmarshaled.volume_threshold);
        assert!(unmarshaled.time_threshold);
        assert!(unmarshaled.quota_exhausted);
    }

    #[test]
    fn test_reporting_triggers_real_world_event_based() {
        // Event-based reporting: Start/stop of traffic
        let triggers = ReportingTriggers::new()
            .with_start_of_traffic(true)
            .with_stop_of_traffic(true);

        let marshaled = triggers.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(triggers, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_real_world_linked_urr() {
        // Linked URR scenario
        let triggers = ReportingTriggers::new()
            .with_linked_urr(true)
            .with_volume_threshold(true);

        let marshaled = triggers.marshal();
        let unmarshaled = ReportingTriggers::unmarshal(&marshaled).unwrap();
        assert_eq!(triggers, unmarshaled);
    }

    #[test]
    fn test_reporting_triggers_default_trait() {
        let triggers: ReportingTriggers = Default::default();
        assert!(!triggers.periodic);
        assert!(!triggers.volume_threshold);
        assert!(!triggers.event_threshold);
    }

    #[test]
    fn test_reporting_triggers_builder_chaining() {
        let triggers = ReportingTriggers::new()
            .with_periodic(false)
            .with_volume_threshold(true)
            .with_periodic(true) // Should override previous false
            .with_time_threshold(true);

        assert!(triggers.periodic);
        assert!(triggers.volume_threshold);
        assert!(triggers.time_threshold);
    }
}
