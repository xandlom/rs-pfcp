// src/ie/update_urr.rs
//! UpdateURR IE and its sub-IEs.

use crate::ie::{
    inactivity_detection_time::InactivityDetectionTime, measurement_method::MeasurementMethod,
    monitoring_time::MonitoringTime, reporting_triggers::ReportingTriggers,
    subsequent_time_threshold::SubsequentTimeThreshold,
    subsequent_volume_threshold::SubsequentVolumeThreshold, time_threshold::TimeThreshold,
    urr_id::UrrId, volume_threshold::VolumeThreshold, Ie, IeType,
};
use std::io;

/// Represents the Update URR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUrr {
    pub urr_id: UrrId,
    pub measurement_method: Option<MeasurementMethod>,
    pub reporting_triggers: Option<ReportingTriggers>,
    pub monitoring_time: Option<MonitoringTime>,
    pub volume_threshold: Option<VolumeThreshold>,
    pub time_threshold: Option<TimeThreshold>,
    pub subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
    pub subsequent_time_threshold: Option<SubsequentTimeThreshold>,
    pub inactivity_detection_time: Option<InactivityDetectionTime>,
}

impl UpdateUrr {
    /// Creates a new Update URR IE.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        urr_id: UrrId,
        measurement_method: Option<MeasurementMethod>,
        reporting_triggers: Option<ReportingTriggers>,
        monitoring_time: Option<MonitoringTime>,
        volume_threshold: Option<VolumeThreshold>,
        time_threshold: Option<TimeThreshold>,
        subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
        subsequent_time_threshold: Option<SubsequentTimeThreshold>,
        inactivity_detection_time: Option<InactivityDetectionTime>,
    ) -> Self {
        UpdateUrr {
            urr_id,
            measurement_method,
            reporting_triggers,
            monitoring_time,
            volume_threshold,
            time_threshold,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            inactivity_detection_time,
        }
    }

    /// Returns a builder for constructing an Update URR IE.
    pub fn builder(urr_id: UrrId) -> UpdateUrrBuilder {
        UpdateUrrBuilder::new(urr_id)
    }

    /// Marshals the Update URR into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.urr_id.to_ie()];

        if let Some(mm) = &self.measurement_method {
            ies.push(mm.to_ie());
        }
        if let Some(rt) = &self.reporting_triggers {
            ies.push(rt.to_ie());
        }
        if let Some(mt) = &self.monitoring_time {
            ies.push(Ie::new(IeType::MonitoringTime, mt.marshal().to_vec()));
        }
        if let Some(vt) = &self.volume_threshold {
            ies.push(Ie::new(IeType::VolumeThreshold, vt.marshal()));
        }
        if let Some(tt) = &self.time_threshold {
            ies.push(Ie::new(IeType::TimeThreshold, tt.marshal().to_vec()));
        }
        if let Some(svt) = &self.subsequent_volume_threshold {
            ies.push(Ie::new(IeType::SubsequentVolumeThreshold, svt.marshal()));
        }
        if let Some(stt) = &self.subsequent_time_threshold {
            ies.push(Ie::new(
                IeType::SubsequentTimeThreshold,
                stt.marshal().to_vec(),
            ));
        }
        if let Some(idt) = &self.inactivity_detection_time {
            ies.push(Ie::new(
                IeType::InactivityDetectionTime,
                idt.marshal().to_vec(),
            ));
        }

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshals a byte slice into a Update Urr IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut urr_id = None;
        let mut measurement_method = None;
        let mut reporting_triggers = None;
        let mut monitoring_time = None;
        let mut volume_threshold = None;
        let mut time_threshold = None;
        let mut subsequent_volume_threshold = None;
        let mut subsequent_time_threshold = None;
        let mut inactivity_detection_time = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::UrrId => {
                    urr_id = Some(UrrId::unmarshal(&ie.payload)?);
                }
                IeType::MeasurementMethod => {
                    measurement_method = Some(MeasurementMethod::unmarshal(&ie.payload)?);
                }
                IeType::ReportingTriggers => {
                    reporting_triggers = Some(ReportingTriggers::unmarshal(&ie.payload)?);
                }
                IeType::MonitoringTime => {
                    monitoring_time = Some(MonitoringTime::unmarshal(&ie.payload)?);
                }
                IeType::VolumeThreshold => {
                    volume_threshold = Some(VolumeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::TimeThreshold => {
                    time_threshold = Some(TimeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentVolumeThreshold => {
                    subsequent_volume_threshold =
                        Some(SubsequentVolumeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::SubsequentTimeThreshold => {
                    subsequent_time_threshold =
                        Some(SubsequentTimeThreshold::unmarshal(&ie.payload)?);
                }
                IeType::InactivityDetectionTime => {
                    inactivity_detection_time =
                        Some(InactivityDetectionTime::unmarshal(&ie.payload)?);
                }
                _ => (),
            }
            offset += ie.len() as usize;
        }

        Ok(UpdateUrr {
            urr_id: urr_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing mandatory URR ID IE")
            })?,
            measurement_method,
            reporting_triggers,
            monitoring_time,
            volume_threshold,
            time_threshold,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            inactivity_detection_time,
        })
    }

    /// Wraps the Update URR in a UpdateUrr IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UpdateUrr, self.marshal())
    }
}

/// Builder for constructing Update URR IEs with a fluent API.
///
/// The builder pattern provides an ergonomic way to construct Update URR IEs
/// for modifying existing URRs with proper validation and convenient methods.
///
/// # Required Fields
/// - `urr_id`: URR ID (set in `new()`)
///
/// # Optional Fields (at least one should be set for a meaningful update)
/// - `measurement_method`: How to measure usage
/// - `reporting_triggers`: When to report
/// - `monitoring_time`: When to start/stop monitoring
/// - `volume_threshold`: Volume limit before reporting
/// - `time_threshold`: Time limit before reporting
/// - `subsequent_volume_threshold`: Volume limit after first report
/// - `subsequent_time_threshold`: Time limit after first report
/// - `inactivity_detection_time`: Detect inactive sessions
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::update_urr::UpdateUrrBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
///
/// // Update volume threshold only
/// let urr = UpdateUrrBuilder::new(UrrId::new(1))
///     .volume_threshold_bytes(2_000_000_000) // Increase to 2GB
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct UpdateUrrBuilder {
    urr_id: Option<UrrId>,
    measurement_method: Option<MeasurementMethod>,
    reporting_triggers: Option<ReportingTriggers>,
    monitoring_time: Option<MonitoringTime>,
    volume_threshold: Option<VolumeThreshold>,
    time_threshold: Option<TimeThreshold>,
    subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
    subsequent_time_threshold: Option<SubsequentTimeThreshold>,
    inactivity_detection_time: Option<InactivityDetectionTime>,
}

impl UpdateUrrBuilder {
    /// Creates a new Update URR builder with the specified URR ID.
    ///
    /// URR ID is mandatory as it identifies which URR to update.
    pub fn new(urr_id: UrrId) -> Self {
        UpdateUrrBuilder {
            urr_id: Some(urr_id),
            ..Default::default()
        }
    }

    /// Sets the measurement method.
    ///
    /// Updates how to measure usage (volume, duration, event).
    pub fn measurement_method(mut self, method: MeasurementMethod) -> Self {
        self.measurement_method = Some(method);
        self
    }

    /// Sets the reporting triggers.
    ///
    /// Updates when to generate usage reports.
    pub fn reporting_triggers(mut self, triggers: ReportingTriggers) -> Self {
        self.reporting_triggers = Some(triggers);
        self
    }

    /// Sets the monitoring time.
    ///
    /// Updates when to start or stop monitoring.
    pub fn monitoring_time(mut self, time: MonitoringTime) -> Self {
        self.monitoring_time = Some(time);
        self
    }

    /// Sets the volume threshold.
    ///
    /// Updates the threshold for when to report based on traffic volume.
    pub fn volume_threshold(mut self, threshold: VolumeThreshold) -> Self {
        self.volume_threshold = Some(threshold);
        self
    }

    /// Convenience method to set volume threshold in bytes (total traffic).
    ///
    /// Creates a volume threshold for total traffic (uplink + downlink).
    pub fn volume_threshold_bytes(mut self, bytes: u64) -> Self {
        self.volume_threshold = Some(VolumeThreshold::new(
            true,  // total volume
            false, // not uplink only
            false, // not downlink only
            Some(bytes),
            None,
            None,
        ));
        self
    }

    /// Convenience method to set volume threshold for uplink and downlink separately.
    pub fn volume_threshold_uplink_downlink(mut self, uplink: u64, downlink: u64) -> Self {
        self.volume_threshold = Some(VolumeThreshold::new(
            false, // not total
            true,  // uplink
            true,  // downlink
            None,
            Some(uplink),
            Some(downlink),
        ));
        self
    }

    /// Sets the time threshold in seconds.
    ///
    /// Updates the threshold for when to report based on monitoring duration.
    pub fn time_threshold(mut self, threshold: TimeThreshold) -> Self {
        self.time_threshold = Some(threshold);
        self
    }

    /// Convenience method to set time threshold from seconds.
    pub fn time_threshold_seconds(mut self, seconds: u32) -> Self {
        self.time_threshold = Some(TimeThreshold::new(seconds));
        self
    }

    /// Sets the subsequent volume threshold.
    ///
    /// Volume threshold to use after the first report.
    pub fn subsequent_volume_threshold(mut self, threshold: SubsequentVolumeThreshold) -> Self {
        self.subsequent_volume_threshold = Some(threshold);
        self
    }

    /// Convenience method to set subsequent volume threshold in bytes.
    pub fn subsequent_volume_threshold_bytes(mut self, bytes: u64) -> Self {
        self.subsequent_volume_threshold = Some(SubsequentVolumeThreshold::new(
            true,  // total volume
            false, // not uplink only
            false, // not downlink only
            Some(bytes),
            None,
            None,
        ));
        self
    }

    /// Sets the subsequent time threshold.
    ///
    /// Time threshold to use after the first report.
    pub fn subsequent_time_threshold(mut self, threshold: SubsequentTimeThreshold) -> Self {
        self.subsequent_time_threshold = Some(threshold);
        self
    }

    /// Convenience method to set subsequent time threshold from seconds.
    pub fn subsequent_time_threshold_seconds(mut self, seconds: u32) -> Self {
        self.subsequent_time_threshold = Some(SubsequentTimeThreshold::new(seconds));
        self
    }

    /// Sets the inactivity detection time.
    ///
    /// Detect when a session becomes inactive after this duration.
    pub fn inactivity_detection_time(mut self, time: InactivityDetectionTime) -> Self {
        self.inactivity_detection_time = Some(time);
        self
    }

    /// Convenience method to set inactivity detection time from seconds.
    pub fn inactivity_detection_time_seconds(mut self, seconds: u32) -> Self {
        self.inactivity_detection_time = Some(InactivityDetectionTime::new(seconds));
        self
    }

    /// Builds the Update URR IE with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - URR ID is missing (required field)
    /// - Measurement method and threshold combinations are inconsistent when both are set
    ///
    /// # Notes
    ///
    /// Unlike CreateUrr, UpdateUrr allows all fields except urr_id to be optional,
    /// as you may want to update only specific fields of an existing URR.
    pub fn build(self) -> Result<UpdateUrr, io::Error> {
        // Validate required field first (without consuming)
        self.urr_id
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "URR ID is required"))?;

        // If measurement method is being updated, validate consistency with thresholds
        if let Some(ref measurement_method) = self.measurement_method {
            self.validate_measurement_thresholds(measurement_method)?;
        }

        // Now consume the values after validation
        Ok(UpdateUrr {
            urr_id: self.urr_id.unwrap(), // Safe because we validated above
            measurement_method: self.measurement_method,
            reporting_triggers: self.reporting_triggers,
            monitoring_time: self.monitoring_time,
            volume_threshold: self.volume_threshold,
            time_threshold: self.time_threshold,
            subsequent_volume_threshold: self.subsequent_volume_threshold,
            subsequent_time_threshold: self.subsequent_time_threshold,
            inactivity_detection_time: self.inactivity_detection_time,
        })
    }

    /// Validates that measurement method matches the configured thresholds.
    ///
    /// This validation only applies when measurement_method is being set in the update.
    fn validate_measurement_thresholds(
        &self,
        measurement_method: &MeasurementMethod,
    ) -> Result<(), io::Error> {
        // Validate volume measurement consistency
        if measurement_method.volume {
            if self.volume_threshold.is_some() || self.subsequent_volume_threshold.is_some() {
                // Volume thresholds are being set, which is consistent with volume measurement
                // No error
            }
        } else {
            // Volume measurement disabled - warn if volume thresholds are being set
            if self.volume_threshold.is_some() || self.subsequent_volume_threshold.is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Volume threshold configured but volume measurement is disabled",
                ));
            }
        }

        // Validate duration measurement consistency
        if measurement_method.duration {
            if self.time_threshold.is_some() || self.subsequent_time_threshold.is_some() {
                // Time thresholds are being set, which is consistent with duration measurement
                // No error
            }
        } else {
            // Duration measurement disabled - warn if time thresholds are being set
            if self.time_threshold.is_some() || self.subsequent_time_threshold.is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Time threshold configured but duration measurement is disabled",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let urr = UpdateUrrBuilder::new(UrrId::new(1))
            .volume_threshold_bytes(1_000_000)
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(1));
        assert!(urr.volume_threshold.is_some());
        assert!(urr.measurement_method.is_none());
    }

    #[test]
    fn test_builder_comprehensive() {
        let measurement = MeasurementMethod::new(true, true, false); // duration=true, volume=true
        let triggers = ReportingTriggers::new();

        let urr = UpdateUrrBuilder::new(UrrId::new(2))
            .measurement_method(measurement)
            .reporting_triggers(triggers)
            .volume_threshold_bytes(2_000_000_000)
            .time_threshold_seconds(3600)
            .subsequent_volume_threshold_bytes(500_000_000)
            .inactivity_detection_time_seconds(300)
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(2));
        assert_eq!(urr.measurement_method, Some(measurement));
        assert_eq!(urr.reporting_triggers, Some(triggers));
        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.inactivity_detection_time.is_some());
    }

    #[test]
    fn test_builder_method() {
        let urr = UpdateUrr::builder(UrrId::new(5))
            .volume_threshold_bytes(500_000)
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(5));
        assert!(urr.volume_threshold.is_some());
    }

    #[test]
    fn test_builder_missing_urr_id() {
        let result = UpdateUrrBuilder::default().build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "URR ID is required");
    }

    #[test]
    fn test_builder_uplink_downlink_volume() {
        let urr = UpdateUrrBuilder::new(UrrId::new(3))
            .volume_threshold_uplink_downlink(1_000_000, 5_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.uplink_volume, Some(1_000_000));
        assert_eq!(vt.downlink_volume, Some(5_000_000));
    }

    #[test]
    fn test_builder_validation_volume_without_measurement() {
        let measurement = MeasurementMethod::new(false, false, false); // No volume measurement

        let result = UpdateUrrBuilder::new(UrrId::new(1))
            .measurement_method(measurement)
            .volume_threshold_bytes(1_000_000)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("volume measurement is disabled"));
    }

    #[test]
    fn test_builder_validation_time_without_measurement() {
        let measurement = MeasurementMethod::new(false, false, false); // No duration measurement

        let result = UpdateUrrBuilder::new(UrrId::new(1))
            .measurement_method(measurement)
            .time_threshold_seconds(3600)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("duration measurement is disabled"));
    }

    #[test]
    fn test_builder_round_trip_marshal() {
        let original = UpdateUrrBuilder::new(UrrId::new(10))
            .volume_threshold_bytes(1_000_000_000)
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = UpdateUrr::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_ie_round_trip() {
        let original = UpdateUrrBuilder::new(UrrId::new(7))
            .volume_threshold_bytes(500_000_000)
            .build()
            .unwrap();

        let ie = original.to_ie();
        let unmarshaled = UpdateUrr::unmarshal(&ie.payload).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_builder_only_thresholds_update() {
        // Valid case: updating only thresholds without changing measurement method
        let urr = UpdateUrrBuilder::new(UrrId::new(8))
            .volume_threshold_bytes(2_000_000_000)
            .subsequent_volume_threshold_bytes(1_000_000_000)
            .build()
            .unwrap();

        assert!(urr.measurement_method.is_none());
        assert!(urr.volume_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
    }

    #[test]
    fn test_builder_convenience_methods() {
        let urr = UpdateUrrBuilder::new(UrrId::new(9))
            .volume_threshold_bytes(1_000_000)
            .time_threshold_seconds(300)
            .subsequent_volume_threshold_bytes(500_000)
            .subsequent_time_threshold_seconds(150)
            .inactivity_detection_time_seconds(60)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.subsequent_time_threshold.is_some());
        assert!(urr.inactivity_detection_time.is_some());
    }

    #[test]
    fn test_update_urr_only_measurement_method() {
        let measurement = MeasurementMethod::new(true, false, false);
        let urr = UpdateUrrBuilder::new(UrrId::new(10))
            .measurement_method(measurement)
            .build()
            .unwrap();

        assert_eq!(urr.measurement_method, Some(measurement));
        assert!(urr.volume_threshold.is_none());
        assert!(urr.time_threshold.is_none());
    }

    #[test]
    fn test_update_urr_only_reporting_triggers() {
        let triggers = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(true);

        let urr = UpdateUrrBuilder::new(UrrId::new(11))
            .reporting_triggers(triggers)
            .build()
            .unwrap();

        assert_eq!(urr.reporting_triggers, Some(triggers));
        assert!(urr.measurement_method.is_none());
    }

    #[test]
    fn test_update_urr_only_monitoring_time() {
        use std::time::SystemTime;
        let monitoring = MonitoringTime::new(SystemTime::now());
        let urr = UpdateUrrBuilder::new(UrrId::new(12))
            .monitoring_time(monitoring)
            .build()
            .unwrap();

        assert_eq!(urr.monitoring_time, Some(monitoring));
        assert!(urr.volume_threshold.is_none());
    }

    #[test]
    fn test_update_urr_only_volume_threshold() {
        let urr = UpdateUrrBuilder::new(UrrId::new(13))
            .volume_threshold_bytes(1_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_none());
        assert!(urr.measurement_method.is_none());
    }

    #[test]
    fn test_update_urr_only_time_threshold() {
        let urr = UpdateUrrBuilder::new(UrrId::new(14))
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        assert!(urr.time_threshold.is_some());
        assert!(urr.volume_threshold.is_none());
    }

    #[test]
    fn test_update_urr_only_subsequent_volume_threshold() {
        let urr = UpdateUrrBuilder::new(UrrId::new(15))
            .subsequent_volume_threshold_bytes(500_000)
            .build()
            .unwrap();

        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.volume_threshold.is_none());
    }

    #[test]
    fn test_update_urr_only_subsequent_time_threshold() {
        let urr = UpdateUrrBuilder::new(UrrId::new(16))
            .subsequent_time_threshold_seconds(1800)
            .build()
            .unwrap();

        assert!(urr.subsequent_time_threshold.is_some());
        assert!(urr.time_threshold.is_none());
    }

    #[test]
    fn test_update_urr_only_inactivity_detection_time() {
        let urr = UpdateUrrBuilder::new(UrrId::new(17))
            .inactivity_detection_time_seconds(600)
            .build()
            .unwrap();

        assert!(urr.inactivity_detection_time.is_some());
        assert!(urr.volume_threshold.is_none());
    }

    #[test]
    fn test_update_urr_all_optional_fields() {
        use std::time::SystemTime;
        let measurement = MeasurementMethod::new(true, true, true);
        let triggers = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true);
        let monitoring = MonitoringTime::new(SystemTime::now());

        let urr = UpdateUrrBuilder::new(UrrId::new(18))
            .measurement_method(measurement)
            .reporting_triggers(triggers)
            .monitoring_time(monitoring)
            .volume_threshold_bytes(5_000_000_000)
            .time_threshold_seconds(7200)
            .subsequent_volume_threshold_bytes(2_000_000_000)
            .subsequent_time_threshold_seconds(3600)
            .inactivity_detection_time_seconds(900)
            .build()
            .unwrap();

        assert_eq!(urr.measurement_method, Some(measurement));
        assert_eq!(urr.reporting_triggers, Some(triggers));
        assert_eq!(urr.monitoring_time, Some(monitoring));
        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.subsequent_time_threshold.is_some());
        assert!(urr.inactivity_detection_time.is_some());
    }

    #[test]
    fn test_update_urr_volume_uplink_downlink_asymmetric() {
        // Test different uplink/downlink values
        let urr = UpdateUrrBuilder::new(UrrId::new(19))
            .volume_threshold_uplink_downlink(2_000_000, 5_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.uplink_volume, Some(2_000_000));
        assert_eq!(vt.downlink_volume, Some(5_000_000));
        assert!(vt.total_volume.is_none());
    }

    #[test]
    fn test_update_urr_volume_uplink_downlink_equal() {
        // Test equal uplink/downlink values
        let urr = UpdateUrrBuilder::new(UrrId::new(20))
            .volume_threshold_uplink_downlink(3_000_000, 3_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.uplink_volume, Some(3_000_000));
        assert_eq!(vt.downlink_volume, Some(3_000_000));
    }

    #[test]
    fn test_update_urr_subsequent_volume_bytes() {
        let urr = UpdateUrrBuilder::new(UrrId::new(21))
            .subsequent_volume_threshold_bytes(1_500_000)
            .build()
            .unwrap();

        assert!(urr.subsequent_volume_threshold.is_some());
        let svt = urr.subsequent_volume_threshold.unwrap();
        assert_eq!(svt.total_volume, Some(1_500_000));
    }

    #[test]
    fn test_update_urr_both_volume_and_subsequent() {
        // Test setting both primary and subsequent thresholds
        let urr = UpdateUrrBuilder::new(UrrId::new(22))
            .volume_threshold_uplink_downlink(5_000_000, 10_000_000)
            .subsequent_volume_threshold_bytes(3_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        let svt = urr.subsequent_volume_threshold.unwrap();
        assert_eq!(vt.uplink_volume, Some(5_000_000));
        assert_eq!(svt.total_volume, Some(3_000_000));
    }

    #[test]
    fn test_update_urr_real_world_quota_increase() {
        // Common scenario: Increase quota after user payment
        let urr = UpdateUrrBuilder::new(UrrId::new(23))
            .volume_threshold_bytes(10_000_000_000) // Increase to 10GB
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(23));
        assert!(urr.volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.total_volume, Some(10_000_000_000));
    }

    #[test]
    fn test_update_urr_real_world_change_reporting() {
        // Change reporting to periodic instead of threshold-based
        let triggers = ReportingTriggers::new()
            .with_periodic(true)
            .with_volume_threshold(false);

        let urr = UpdateUrrBuilder::new(UrrId::new(24))
            .reporting_triggers(triggers)
            .build()
            .unwrap();

        assert_eq!(urr.reporting_triggers, Some(triggers));
    }

    #[test]
    fn test_update_urr_real_world_add_inactivity_detection() {
        // Add inactivity detection to existing URR
        let urr = UpdateUrrBuilder::new(UrrId::new(25))
            .inactivity_detection_time_seconds(300) // 5 minutes
            .build()
            .unwrap();

        assert!(urr.inactivity_detection_time.is_some());
    }

    #[test]
    fn test_update_urr_marshal_minimal() {
        let urr = UpdateUrrBuilder::new(UrrId::new(26))
            .volume_threshold_bytes(1_000_000)
            .build()
            .unwrap();

        let marshaled = urr.marshal();
        assert!(!marshaled.is_empty());
        // Should contain at least URR ID and Volume Threshold IEs
        assert!(marshaled.len() > 8); // At least 2 IEs with headers
    }

    #[test]
    fn test_update_urr_marshal_comprehensive() {
        let measurement = MeasurementMethod::new(true, true, false);
        let triggers = ReportingTriggers::new().with_periodic(true);

        let urr = UpdateUrrBuilder::new(UrrId::new(27))
            .measurement_method(measurement)
            .reporting_triggers(triggers)
            .volume_threshold_bytes(5_000_000_000)
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        let marshaled = urr.marshal();
        assert!(!marshaled.is_empty());
        // Should contain multiple IEs
        assert!(marshaled.len() > 20);
    }

    #[test]
    fn test_update_urr_round_trip_all_fields() {
        use std::time::SystemTime;
        let measurement = MeasurementMethod::new(true, true, true);
        let triggers = ReportingTriggers::new()
            .with_volume_threshold(true)
            .with_time_threshold(true);
        let monitoring = MonitoringTime::new(SystemTime::now());

        let original = UpdateUrrBuilder::new(UrrId::new(28))
            .measurement_method(measurement)
            .reporting_triggers(triggers)
            .monitoring_time(monitoring)
            .volume_threshold_bytes(4_000_000_000)
            .time_threshold_seconds(5400)
            .subsequent_volume_threshold_bytes(1_500_000_000)
            .subsequent_time_threshold_seconds(2700)
            .inactivity_detection_time_seconds(600)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = UpdateUrr::unmarshal(&marshaled).unwrap();

        // Verify all fields (monitoring_time loses nanosecond precision)
        assert_eq!(unmarshaled.urr_id, original.urr_id);
        assert_eq!(unmarshaled.measurement_method, original.measurement_method);
        assert_eq!(unmarshaled.reporting_triggers, original.reporting_triggers);
        assert!(unmarshaled.monitoring_time.is_some());
        assert_eq!(unmarshaled.volume_threshold, original.volume_threshold);
        assert_eq!(unmarshaled.time_threshold, original.time_threshold);
        assert_eq!(
            unmarshaled.subsequent_volume_threshold,
            original.subsequent_volume_threshold
        );
        assert_eq!(
            unmarshaled.subsequent_time_threshold,
            original.subsequent_time_threshold
        );
        assert_eq!(
            unmarshaled.inactivity_detection_time,
            original.inactivity_detection_time
        );
    }

    #[test]
    fn test_update_urr_unmarshal_missing_urr_id() {
        // Create invalid data without URR ID
        let empty_data = vec![];
        let result = UpdateUrr::unmarshal(&empty_data);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("URR ID"));
    }

    #[test]
    fn test_update_urr_to_ie() {
        let urr = UpdateUrrBuilder::new(UrrId::new(29))
            .volume_threshold_bytes(2_000_000_000)
            .build()
            .unwrap();

        let ie = urr.to_ie();
        assert_eq!(ie.ie_type, IeType::UpdateUrr);
        assert!(!ie.payload.is_empty());
    }

    #[test]
    fn test_builder_validation_subsequent_without_primary() {
        // It's valid to set subsequent thresholds without primary thresholds
        // (updating subsequent only)
        let urr = UpdateUrrBuilder::new(UrrId::new(30))
            .subsequent_volume_threshold_bytes(500_000_000)
            .subsequent_time_threshold_seconds(1800)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_none());
        assert!(urr.time_threshold.is_none());
        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.subsequent_time_threshold.is_some());
    }

    #[test]
    fn test_update_urr_builder_method_chaining() {
        let urr = UpdateUrr::builder(UrrId::new(31))
            .volume_threshold_bytes(1_000_000)
            .time_threshold_seconds(300)
            .subsequent_volume_threshold_bytes(500_000)
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(31));
        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
    }
}
