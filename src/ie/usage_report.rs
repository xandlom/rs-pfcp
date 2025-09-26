// src/ie/usage_report.rs

use crate::ie::sequence_number::SequenceNumber;
use crate::ie::urr_id::UrrId;
use crate::ie::usage_report_trigger::UsageReportTrigger;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReport {
    pub urr_id: UrrId,
    pub ur_seqn: SequenceNumber,
    pub usage_report_trigger: UsageReportTrigger,
    // TODO: Add optional fields when individual IE types are implemented:
    // pub volume_measurement: Option<VolumeMeasurement>, // IE Type ~63
    // pub duration_measurement: Option<DurationMeasurement>, // IE Type ~64
    // pub time_of_first_packet: Option<TimeOfFirstPacket>, // IE Type ~67
    // pub time_of_last_packet: Option<TimeOfLastPacket>, // IE Type ~68
    // pub usage_information: Option<UsageInformation>, // IE Type ~90
    // pub query_urr_reference: Option<QueryUrrReference>, // IE Type ~125
    // pub ethernet_traffic_information: Option<EthernetTrafficInformation>, // IE Type ~143
}

impl UsageReport {
    pub fn new(
        urr_id: UrrId,
        ur_seqn: SequenceNumber,
        usage_report_trigger: UsageReportTrigger,
    ) -> Self {
        UsageReport {
            urr_id,
            ur_seqn,
            usage_report_trigger,
        }
    }

    /// Creates a new builder for constructing Usage Report Information Elements.
    pub fn builder(urr_id: UrrId) -> UsageReportBuilder {
        UsageReportBuilder::new(urr_id)
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend_from_slice(&self.urr_id.to_ie().marshal());
        buffer.extend_from_slice(&self.ur_seqn.to_ie().marshal());
        buffer.extend_from_slice(&self.usage_report_trigger.to_ie().marshal());
        buffer
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = 0;
        let mut urr_id = None;
        let mut ur_seqn = None;
        let mut usage_report_trigger = None;

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])?;
            match ie.ie_type {
                IeType::UrrId => urr_id = Some(UrrId::unmarshal(&ie.payload)?),
                IeType::SequenceNumber => ur_seqn = Some(SequenceNumber::unmarshal(&ie.payload)?),
                IeType::UsageReportTrigger => {
                    usage_report_trigger = Some(UsageReportTrigger::unmarshal(&ie.payload)?)
                }
                _ => (),
            }
            cursor += ie.len() as usize;
        }

        Ok(UsageReport {
            urr_id: urr_id.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "URR ID not found")
            })?,
            ur_seqn: ur_seqn.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "UR-SEQN not found")
            })?,
            usage_report_trigger: usage_report_trigger.ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Usage Report Trigger not found",
                )
            })?,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UsageReport, self.marshal())
    }
}

/// Builder for constructing Usage Report Information Elements with validation.
///
/// The Usage Report builder provides an ergonomic way to construct usage report IEs
/// that are commonly sent from UPF to SMF to report traffic usage, quota exhaustion,
/// and other monitoring events.
///
/// # Examples
///
/// ```rust
/// use rs_pfcp::ie::usage_report::UsageReportBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
/// use rs_pfcp::ie::sequence_number::SequenceNumber;
/// use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;
///
/// // Basic usage report for quota exhaustion
/// let quota_report = UsageReportBuilder::new(UrrId::new(1))
///     .sequence_number(SequenceNumber::new(42))
///     .quota_exhausted()
///     .build()
///     .unwrap();
///
/// // Periodic usage report
/// let periodic_report = UsageReportBuilder::new(UrrId::new(2))
///     .sequence_number(SequenceNumber::new(43))
///     .periodic_report()
///     .build()
///     .unwrap();
///
/// // Volume threshold triggered report
/// let volume_report = UsageReportBuilder::volume_threshold_report(
///     UrrId::new(3),
///     SequenceNumber::new(44)
/// ).build().unwrap();
///
/// // Time threshold triggered report
/// let time_report = UsageReportBuilder::time_threshold_report(
///     UrrId::new(4),
///     SequenceNumber::new(45)
/// ).build().unwrap();
/// ```
#[derive(Debug, Default)]
pub struct UsageReportBuilder {
    urr_id: Option<UrrId>,
    ur_seqn: Option<SequenceNumber>,
    usage_report_trigger: Option<UsageReportTrigger>,
}

impl UsageReportBuilder {
    /// Creates a new Usage Report builder with the specified URR ID.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID that generated this report
    pub fn new(urr_id: UrrId) -> Self {
        UsageReportBuilder {
            urr_id: Some(urr_id),
            ur_seqn: None,
            usage_report_trigger: None,
        }
    }

    /// Sets the sequence number for the usage report.
    ///
    /// The sequence number is used to correlate usage reports and ensure
    /// proper ordering and duplicate detection.
    ///
    /// # Arguments
    ///
    /// * `ur_seqn` - The usage report sequence number
    pub fn sequence_number(mut self, ur_seqn: SequenceNumber) -> Self {
        self.ur_seqn = Some(ur_seqn);
        self
    }

    /// Sets the usage report trigger flags.
    ///
    /// This indicates what event(s) triggered the usage report generation.
    ///
    /// # Arguments
    ///
    /// * `trigger` - The usage report trigger flags
    pub fn trigger(mut self, trigger: UsageReportTrigger) -> Self {
        self.usage_report_trigger = Some(trigger);
        self
    }

    /// Configures the report as a quota exhaustion report.
    ///
    /// This is commonly used when volume or time quotas are exhausted
    /// and traffic needs to be suspended or redirected.
    pub fn quota_exhausted(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::VOLTH | UsageReportTrigger::TIMTH);
        self
    }

    /// Configures the report as a periodic usage report.
    ///
    /// This is used for regular reporting intervals to track ongoing usage.
    pub fn periodic_report(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::PERIO);
        self
    }

    /// Configures the report as a volume threshold triggered report.
    ///
    /// This is used when a volume threshold has been reached.
    pub fn volume_threshold_triggered(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::VOLTH);
        self
    }

    /// Configures the report as a time threshold triggered report.
    ///
    /// This is used when a time threshold has been reached.
    pub fn time_threshold_triggered(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::TIMTH);
        self
    }

    /// Configures the report for start of traffic detection.
    ///
    /// This is used when traffic flow begins for the first time.
    pub fn start_of_traffic(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::START);
        self
    }

    /// Configures the report for stop of traffic detection.
    ///
    /// This is used when traffic flow ends or stops.
    pub fn stop_of_traffic(mut self) -> Self {
        self.usage_report_trigger = Some(UsageReportTrigger::STOPT);
        self
    }

    /// Builds the Usage Report Information Element.
    ///
    /// # Returns
    ///
    /// Returns a `Result<UsageReport, io::Error>`. The operation will fail if:
    /// - URR ID is not set (required field)
    /// - Sequence number is not set (required field)
    /// - Usage report trigger is not set (required field)
    ///
    /// # Errors
    ///
    /// Returns an error if any required fields are missing.
    pub fn build(self) -> Result<UsageReport, io::Error> {
        let urr_id = self
            .urr_id
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "URR ID is required"))?;

        let ur_seqn = self.ur_seqn.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Sequence number is required")
        })?;

        let usage_report_trigger = self.usage_report_trigger.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Usage report trigger is required",
            )
        })?;

        Ok(UsageReport {
            urr_id,
            ur_seqn,
            usage_report_trigger,
        })
    }

    /// Creates a pre-configured builder for quota exhaustion reports.
    ///
    /// This is a common pattern when volume or time quotas are exhausted.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn quota_exhausted_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .quota_exhausted()
    }

    /// Creates a pre-configured builder for periodic usage reports.
    ///
    /// This is used for regular reporting intervals.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn periodic_usage_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .periodic_report()
    }

    /// Creates a pre-configured builder for volume threshold reports.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn volume_threshold_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .volume_threshold_triggered()
    }

    /// Creates a pre-configured builder for time threshold reports.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn time_threshold_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .time_threshold_triggered()
    }

    /// Creates a pre-configured builder for start of traffic reports.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn start_of_traffic_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .start_of_traffic()
    }

    /// Creates a pre-configured builder for stop of traffic reports.
    ///
    /// # Arguments
    ///
    /// * `urr_id` - The Usage Reporting Rule ID
    /// * `ur_seqn` - The usage report sequence number
    pub fn stop_of_traffic_report(urr_id: UrrId, ur_seqn: SequenceNumber) -> Self {
        UsageReportBuilder::new(urr_id)
            .sequence_number(ur_seqn)
            .stop_of_traffic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_report_marshal_unmarshal() {
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(1);
        let usage_report_trigger = UsageReportTrigger::new(1);
        let usage_report = UsageReport::new(urr_id, ur_seqn, usage_report_trigger);

        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();

        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_builder_basic() {
        let urr_id = UrrId::new(42);
        let ur_seqn = SequenceNumber::new(123);
        let trigger = UsageReportTrigger::PERIO;

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .trigger(trigger)
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, trigger);
    }

    #[test]
    fn test_usage_report_builder_quota_exhausted() {
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(42);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .quota_exhausted()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(
            usage_report.usage_report_trigger,
            UsageReportTrigger::VOLTH | UsageReportTrigger::TIMTH
        );
    }

    #[test]
    fn test_usage_report_builder_periodic_report() {
        let urr_id = UrrId::new(2);
        let ur_seqn = SequenceNumber::new(43);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .periodic_report()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, UsageReportTrigger::PERIO);
    }

    #[test]
    fn test_usage_report_builder_volume_threshold() {
        let urr_id = UrrId::new(3);
        let ur_seqn = SequenceNumber::new(44);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .volume_threshold_triggered()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, UsageReportTrigger::VOLTH);
    }

    #[test]
    fn test_usage_report_builder_time_threshold() {
        let urr_id = UrrId::new(4);
        let ur_seqn = SequenceNumber::new(45);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .time_threshold_triggered()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, UsageReportTrigger::TIMTH);
    }

    #[test]
    fn test_usage_report_builder_start_of_traffic() {
        let urr_id = UrrId::new(5);
        let ur_seqn = SequenceNumber::new(46);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .start_of_traffic()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, UsageReportTrigger::START);
    }

    #[test]
    fn test_usage_report_builder_stop_of_traffic() {
        let urr_id = UrrId::new(6);
        let ur_seqn = SequenceNumber::new(47);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .stop_of_traffic()
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.usage_report_trigger, UsageReportTrigger::STOPT);
    }

    #[test]
    fn test_usage_report_builder_convenience_methods() {
        // Test quota exhausted convenience method
        let quota_report =
            UsageReportBuilder::quota_exhausted_report(UrrId::new(1), SequenceNumber::new(42))
                .build()
                .unwrap();
        assert_eq!(
            quota_report.usage_report_trigger,
            UsageReportTrigger::VOLTH | UsageReportTrigger::TIMTH
        );

        // Test periodic report convenience method
        let periodic_report =
            UsageReportBuilder::periodic_usage_report(UrrId::new(2), SequenceNumber::new(43))
                .build()
                .unwrap();
        assert_eq!(
            periodic_report.usage_report_trigger,
            UsageReportTrigger::PERIO
        );

        // Test volume threshold convenience method
        let volume_report =
            UsageReportBuilder::volume_threshold_report(UrrId::new(3), SequenceNumber::new(44))
                .build()
                .unwrap();
        assert_eq!(
            volume_report.usage_report_trigger,
            UsageReportTrigger::VOLTH
        );

        // Test time threshold convenience method
        let time_report =
            UsageReportBuilder::time_threshold_report(UrrId::new(4), SequenceNumber::new(45))
                .build()
                .unwrap();
        assert_eq!(time_report.usage_report_trigger, UsageReportTrigger::TIMTH);

        // Test start of traffic convenience method
        let start_report =
            UsageReportBuilder::start_of_traffic_report(UrrId::new(5), SequenceNumber::new(46))
                .build()
                .unwrap();
        assert_eq!(start_report.usage_report_trigger, UsageReportTrigger::START);

        // Test stop of traffic convenience method
        let stop_report =
            UsageReportBuilder::stop_of_traffic_report(UrrId::new(6), SequenceNumber::new(47))
                .build()
                .unwrap();
        assert_eq!(stop_report.usage_report_trigger, UsageReportTrigger::STOPT);
    }

    #[test]
    fn test_usage_report_builder_validation_errors() {
        // Test missing URR ID (should not be possible with current API, but test anyway)
        let builder = UsageReportBuilder::default();
        let result = builder.build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "URR ID is required");

        // Test missing sequence number
        let urr_id = UrrId::new(1);
        let builder = UsageReportBuilder::new(urr_id);
        let result = builder.build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Sequence number is required"
        );

        // Test missing usage report trigger
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(42);
        let builder = UsageReportBuilder::new(urr_id).sequence_number(ur_seqn);
        let result = builder.build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Usage report trigger is required"
        );
    }

    #[test]
    fn test_usage_report_builder_round_trip_marshal() {
        let urr_id = UrrId::new(99);
        let ur_seqn = SequenceNumber::new(255);

        let original = UsageReportBuilder::quota_exhausted_report(urr_id, ur_seqn)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_usage_report_builder_comprehensive() {
        // Test all trigger types with round-trip marshal/unmarshal
        let triggers = [
            UsageReportTrigger::PERIO,
            UsageReportTrigger::VOLTH,
            UsageReportTrigger::TIMTH,
            UsageReportTrigger::QUHTI,
            UsageReportTrigger::START,
            UsageReportTrigger::STOPT,
            UsageReportTrigger::DROTH,
            UsageReportTrigger::LIUSA,
            UsageReportTrigger::VOLTH | UsageReportTrigger::TIMTH, // Combined flags
        ];

        for (i, trigger) in triggers.iter().enumerate() {
            let urr_id = UrrId::new(100 + i as u32);
            let ur_seqn = SequenceNumber::new(200 + i as u32);

            let usage_report = UsageReportBuilder::new(urr_id)
                .sequence_number(ur_seqn)
                .trigger(*trigger)
                .build()
                .unwrap();

            // Test marshal/unmarshal round trip
            let marshaled = usage_report.marshal();
            let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
            assert_eq!(usage_report, unmarshaled);

            // Test IE wrapping
            let ie = usage_report.to_ie();
            assert_eq!(ie.ie_type, IeType::UsageReport);
        }
    }
}
