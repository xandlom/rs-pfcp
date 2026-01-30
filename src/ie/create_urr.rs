//! CreateURR IE and its sub-IEs.

use crate::error::PfcpError;
use crate::ie::{
    inactivity_detection_time::InactivityDetectionTime, marshal_ies,
    measurement_method::MeasurementMethod, monitoring_time::MonitoringTime,
    reporting_triggers::ReportingTriggers, subsequent_time_threshold::SubsequentTimeThreshold,
    subsequent_volume_threshold::SubsequentVolumeThreshold, time_threshold::TimeThreshold,
    urr_id::UrrId, volume_threshold::VolumeThreshold, Ie, IeIterator, IeType,
};

/// Represents the Create URR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUrr {
    pub urr_id: UrrId,
    pub measurement_method: MeasurementMethod,
    pub reporting_triggers: ReportingTriggers,
    pub monitoring_time: Option<MonitoringTime>,
    pub volume_threshold: Option<VolumeThreshold>,
    pub time_threshold: Option<TimeThreshold>,
    pub subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
    pub subsequent_time_threshold: Option<SubsequentTimeThreshold>,
    pub inactivity_detection_time: Option<InactivityDetectionTime>,
}

impl CreateUrr {
    /// Creates a new Create URR IE.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        urr_id: UrrId,
        measurement_method: MeasurementMethod,
        reporting_triggers: ReportingTriggers,
        monitoring_time: Option<MonitoringTime>,
        volume_threshold: Option<VolumeThreshold>,
        time_threshold: Option<TimeThreshold>,
        subsequent_volume_threshold: Option<SubsequentVolumeThreshold>,
        subsequent_time_threshold: Option<SubsequentTimeThreshold>,
        inactivity_detection_time: Option<InactivityDetectionTime>,
    ) -> Self {
        CreateUrr {
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

    /// Returns a builder for constructing a Create URR IE.
    pub fn builder(urr_id: UrrId) -> CreateUrrBuilder {
        CreateUrrBuilder::new(urr_id)
    }

    /// Marshals the Create URR into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![
            self.urr_id.to_ie(),
            self.measurement_method.to_ie(),
            self.reporting_triggers.to_ie(),
        ];

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

        marshal_ies(&ies)
    }

    /// Unmarshals a byte slice into a Create Urr IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut urr_id = None;
        let mut measurement_method = None;
        let mut reporting_triggers = None;
        let mut monitoring_time = None;
        let mut volume_threshold = None;
        let mut time_threshold = None;
        let mut subsequent_volume_threshold = None;
        let mut subsequent_time_threshold = None;
        let mut inactivity_detection_time = None;

        for ie_result in IeIterator::new(payload) {
            let ie = ie_result?;
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
        }

        Ok(CreateUrr {
            urr_id: urr_id.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::UrrId,
                IeType::CreateUrr,
            ))?,
            measurement_method: measurement_method.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::MeasurementMethod,
                IeType::CreateUrr,
            ))?,
            reporting_triggers: reporting_triggers.ok_or(PfcpError::missing_ie_in_grouped(
                IeType::ReportingTriggers,
                IeType::CreateUrr,
            ))?,
            monitoring_time,
            volume_threshold,
            time_threshold,
            subsequent_volume_threshold,
            subsequent_time_threshold,
            inactivity_detection_time,
        })
    }

    /// Wraps the Create URR in a CreateUrr IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CreateUrr, self.marshal())
    }
}

/// Builder for constructing Create URR IEs with a fluent API.
///
/// The builder pattern provides an ergonomic way to construct Create URR IEs
/// with proper validation and convenient methods for common usage patterns.
///
/// # Required Fields
/// - `urr_id`: URR ID (set in `new()`)
/// - `measurement_method`: How to measure usage
/// - `reporting_triggers`: When to report
///
/// # Optional Fields
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
/// use rs_pfcp::ie::create_urr::CreateUrrBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
/// use rs_pfcp::ie::measurement_method::MeasurementMethod;
/// use rs_pfcp::ie::reporting_triggers::ReportingTriggers;
///
/// // Simple volume-based URR
/// let urr = CreateUrrBuilder::new(UrrId::new(1))
///     .measurement_method(MeasurementMethod::new(false, true, false)) // volume enabled
///     .reporting_triggers(ReportingTriggers::new())
///     .volume_threshold_bytes(1_000_000_000) // 1GB
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct CreateUrrBuilder {
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

impl CreateUrrBuilder {
    /// Creates a new Create URR builder with the specified URR ID.
    ///
    /// URR ID is mandatory for all URR instances.
    pub fn new(urr_id: UrrId) -> Self {
        CreateUrrBuilder {
            urr_id: Some(urr_id),
            ..Default::default()
        }
    }

    /// Sets the measurement method.
    ///
    /// This is a required field that specifies how to measure usage (volume, duration, event).
    pub fn measurement_method(mut self, method: MeasurementMethod) -> Self {
        self.measurement_method = Some(method);
        self
    }

    /// Sets the reporting triggers.
    ///
    /// This is a required field that specifies when to generate usage reports.
    pub fn reporting_triggers(mut self, triggers: ReportingTriggers) -> Self {
        self.reporting_triggers = Some(triggers);
        self
    }

    /// Sets the monitoring time.
    ///
    /// Specifies when to start or stop monitoring.
    pub fn monitoring_time(mut self, time: MonitoringTime) -> Self {
        self.monitoring_time = Some(time);
        self
    }

    /// Sets the volume threshold.
    ///
    /// Report when traffic volume exceeds this threshold.
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
    /// Report when monitoring duration exceeds this threshold.
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

    /// Builds the Create URR IE with comprehensive validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required fields are missing (URR ID, measurement method, reporting triggers)
    /// - Measurement method and threshold combinations are inconsistent:
    ///   - Volume measurement enabled but no volume threshold set
    ///   - Duration measurement enabled but no time threshold set
    ///   - Volume threshold set but volume measurement disabled
    ///   - Time threshold set but duration measurement disabled
    pub fn build(self) -> Result<CreateUrr, PfcpError> {
        // Validate required fields first (without consuming)
        self.urr_id.as_ref().ok_or(PfcpError::MissingMandatoryIe {
            ie_type: IeType::UrrId,
            message_type: None,
            parent_ie: Some(IeType::CreateUrr),
        })?;

        let measurement_method =
            self.measurement_method
                .as_ref()
                .ok_or(PfcpError::MissingMandatoryIe {
                    ie_type: IeType::MeasurementMethod,
                    message_type: None,
                    parent_ie: Some(IeType::CreateUrr),
                })?;

        self.reporting_triggers
            .as_ref()
            .ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::ReportingTriggers,
                message_type: None,
                parent_ie: Some(IeType::CreateUrr),
            })?;

        // Validate measurement method and threshold consistency
        self.validate_measurement_thresholds(measurement_method)?;

        // Now consume the values after validation
        Ok(CreateUrr {
            urr_id: self.urr_id.unwrap(),
            measurement_method: self.measurement_method.unwrap(),
            reporting_triggers: self.reporting_triggers.unwrap(),
            monitoring_time: self.monitoring_time,
            volume_threshold: self.volume_threshold,
            time_threshold: self.time_threshold,
            subsequent_volume_threshold: self.subsequent_volume_threshold,
            subsequent_time_threshold: self.subsequent_time_threshold,
            inactivity_detection_time: self.inactivity_detection_time,
        })
    }

    /// Validates that measurement method matches the configured thresholds.
    fn validate_measurement_thresholds(
        &self,
        measurement_method: &MeasurementMethod,
    ) -> Result<(), PfcpError> {
        // Validate volume measurement consistency
        if measurement_method.volume {
            if self.volume_threshold.is_none() && self.subsequent_volume_threshold.is_none() {
                return Err(PfcpError::validation_error(
                    "CreateUrrBuilder",
                    "volume_threshold",
                    "Volume measurement enabled but no volume threshold configured",
                ));
            }
        } else {
            // Volume measurement disabled but volume thresholds configured
            if self.volume_threshold.is_some() || self.subsequent_volume_threshold.is_some() {
                return Err(PfcpError::validation_error(
                    "CreateUrrBuilder",
                    "volume_threshold",
                    "Volume threshold configured but volume measurement is disabled",
                ));
            }
        }

        // Validate duration measurement consistency
        if measurement_method.duration {
            if self.time_threshold.is_none() && self.subsequent_time_threshold.is_none() {
                return Err(PfcpError::validation_error(
                    "CreateUrrBuilder",
                    "time_threshold",
                    "Duration measurement enabled but no time threshold configured",
                ));
            }
        } else {
            // Duration measurement disabled but time thresholds configured
            if self.time_threshold.is_some() || self.subsequent_time_threshold.is_some() {
                return Err(PfcpError::validation_error(
                    "CreateUrrBuilder",
                    "time_threshold",
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
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_create_urr_marshal_unmarshal_mandatory() {
        let urr_id = UrrId::new(1);
        let measurement_method = MeasurementMethod::new(true, false, true);
        let reporting_triggers = ReportingTriggers::new();

        let create_urr = CreateUrr::new(
            urr_id,
            measurement_method,
            reporting_triggers,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let marshaled = create_urr.marshal();
        let unmarshaled = CreateUrr::unmarshal(&marshaled).unwrap();

        assert_eq!(create_urr, unmarshaled);
    }

    #[test]
    fn test_create_urr_marshal_unmarshal_all() {
        let urr_id = UrrId::new(1);
        let measurement_method = MeasurementMethod::new(true, false, true);
        let reporting_triggers = ReportingTriggers::new();
        let now = SystemTime::now();
        let now_secs = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let monitoring_time =
            MonitoringTime::new(SystemTime::UNIX_EPOCH + Duration::from_secs(now_secs));
        let volume_threshold = VolumeThreshold::new(true, true, false, Some(1000), Some(500), None);
        let time_threshold = TimeThreshold::new(3600);
        let subsequent_volume_threshold =
            SubsequentVolumeThreshold::new(false, true, true, None, Some(200), Some(300));
        let subsequent_time_threshold = SubsequentTimeThreshold::new(1800);
        let inactivity_detection_time = InactivityDetectionTime::new(60);

        let create_urr = CreateUrr::new(
            urr_id,
            measurement_method,
            reporting_triggers,
            Some(monitoring_time),
            Some(volume_threshold),
            Some(time_threshold),
            Some(subsequent_volume_threshold),
            Some(subsequent_time_threshold),
            Some(inactivity_detection_time),
        );

        let marshaled = create_urr.marshal();
        let unmarshaled = CreateUrr::unmarshal(&marshaled).unwrap();

        assert_eq!(create_urr, unmarshaled);
    }

    #[test]
    fn test_builder_basic() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, false, true)) // event only
            .reporting_triggers(ReportingTriggers::new())
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(1));
        assert!(urr.volume_threshold.is_none());
        assert!(urr.time_threshold.is_none());
    }

    #[test]
    fn test_builder_with_volume_threshold() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume measurement
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000)
            .build()
            .unwrap();

        assert!(urr.volume_threshold.is_some());
        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.total_volume, Some(1_000_000_000));
    }

    #[test]
    fn test_builder_with_time_threshold() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false)) // duration measurement
            .reporting_triggers(ReportingTriggers::new())
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        assert!(urr.time_threshold.is_some());
        let tt = urr.time_threshold.unwrap();
        assert_eq!(tt.value, 3600);
    }

    #[test]
    fn test_builder_comprehensive() {
        let now = SystemTime::now();
        let now_secs = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, true, false)) // duration + volume
            .reporting_triggers(ReportingTriggers::new())
            .monitoring_time(MonitoringTime::new(
                SystemTime::UNIX_EPOCH + Duration::from_secs(now_secs),
            ))
            .volume_threshold_bytes(1_000_000_000)
            .time_threshold_seconds(3600)
            .subsequent_volume_threshold_bytes(500_000_000)
            .subsequent_time_threshold_seconds(1800)
            .inactivity_detection_time_seconds(60)
            .build()
            .unwrap();

        assert!(urr.monitoring_time.is_some());
        assert!(urr.volume_threshold.is_some());
        assert!(urr.time_threshold.is_some());
        assert!(urr.subsequent_volume_threshold.is_some());
        assert!(urr.subsequent_time_threshold.is_some());
        assert!(urr.inactivity_detection_time.is_some());
    }

    #[test]
    fn test_builder_uplink_downlink_volume() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume measurement
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_uplink_downlink(500_000_000, 1_000_000_000)
            .build()
            .unwrap();

        let vt = urr.volume_threshold.unwrap();
        assert_eq!(vt.uplink_volume, Some(500_000_000));
        assert_eq!(vt.downlink_volume, Some(1_000_000_000));
        assert_eq!(vt.total_volume, None);
    }

    #[test]
    fn test_builder_missing_measurement_method() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .reporting_triggers(ReportingTriggers::new())
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::MeasurementMethod);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_builder_missing_reporting_triggers() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::MissingMandatoryIe { ie_type, .. } => {
                assert_eq!(ie_type, IeType::ReportingTriggers);
            }
            _ => panic!("Expected MissingMandatoryIe error"),
        }
    }

    #[test]
    fn test_builder_round_trip_marshal() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, true, false)) // duration + volume
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000)
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        let marshaled = urr.marshal();
        let unmarshaled = CreateUrr::unmarshal(&marshaled).unwrap();

        assert_eq!(urr, unmarshaled);
    }

    #[test]
    fn test_builder_method() {
        let urr = CreateUrr::builder(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false)) // duration measurement
            .reporting_triggers(ReportingTriggers::new())
            .time_threshold_seconds(3600)
            .build()
            .unwrap();

        assert_eq!(urr.urr_id, UrrId::new(1));
    }

    #[test]
    fn test_builder_ie_round_trip() {
        let urr = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume measurement
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000)
            .build()
            .unwrap();

        let ie = urr.to_ie();
        let unmarshaled = CreateUrr::unmarshal(&ie.payload).unwrap();

        assert_eq!(urr, unmarshaled);
    }

    // Type safety validation tests
    #[test]
    fn test_builder_validation_volume_without_threshold() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume enabled
            .reporting_triggers(ReportingTriggers::new())
            // No volume threshold set
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::ValidationError {
                builder,
                field,
                reason,
            } => {
                assert_eq!(builder, "CreateUrrBuilder");
                assert_eq!(field, "volume_threshold");
                assert!(reason.contains("Volume measurement enabled"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_builder_validation_threshold_without_volume() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false)) // duration only, volume disabled
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000) // volume threshold but volume disabled
            .time_threshold_seconds(3600)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::ValidationError {
                builder,
                field,
                reason,
            } => {
                assert_eq!(builder, "CreateUrrBuilder");
                assert_eq!(field, "volume_threshold");
                assert!(reason.contains("Volume threshold configured"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_builder_validation_duration_without_threshold() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false)) // duration enabled
            .reporting_triggers(ReportingTriggers::new())
            // No time threshold set
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::ValidationError {
                builder,
                field,
                reason,
            } => {
                assert_eq!(builder, "CreateUrrBuilder");
                assert_eq!(field, "time_threshold");
                assert!(reason.contains("Duration measurement enabled"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_builder_validation_threshold_without_duration() {
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume only
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000)
            .time_threshold_seconds(3600) // time threshold but duration disabled
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            PfcpError::ValidationError {
                builder,
                field,
                reason,
            } => {
                assert_eq!(builder, "CreateUrrBuilder");
                assert_eq!(field, "time_threshold");
                assert!(reason.contains("Time threshold configured"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_builder_validation_subsequent_volume_counts() {
        // Subsequent volume threshold should count as volume threshold
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, true, false)) // volume enabled
            .reporting_triggers(ReportingTriggers::new())
            .subsequent_volume_threshold_bytes(500_000_000) // Only subsequent, no initial
            .build();

        // Should succeed - subsequent threshold is valid for volume measurement
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_validation_subsequent_time_counts() {
        // Subsequent time threshold should count as time threshold
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, false, false)) // duration enabled
            .reporting_triggers(ReportingTriggers::new())
            .subsequent_time_threshold_seconds(1800) // Only subsequent, no initial
            .build();

        // Should succeed - subsequent threshold is valid for duration measurement
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_validation_event_only_no_thresholds() {
        // Event measurement doesn't require thresholds
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(false, false, true)) // event only
            .reporting_triggers(ReportingTriggers::new())
            .build();

        // Should succeed - event measurement doesn't require volume/time thresholds
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_validation_combined_measurements_valid() {
        // Volume + duration measurement with both thresholds
        let result = CreateUrrBuilder::new(UrrId::new(1))
            .measurement_method(MeasurementMethod::new(true, true, false))
            .reporting_triggers(ReportingTriggers::new())
            .volume_threshold_bytes(1_000_000_000)
            .time_threshold_seconds(3600)
            .build();

        assert!(result.is_ok());
    }
}
