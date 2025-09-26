// src/ie/usage_report.rs

use crate::ie::sequence_number::SequenceNumber;
use crate::ie::urr_id::UrrId;
use crate::ie::usage_report_trigger::UsageReportTrigger;
use crate::ie::volume_measurement::VolumeMeasurement;
use crate::ie::duration_measurement::DurationMeasurement;
use crate::ie::time_of_first_packet::TimeOfFirstPacket;
use crate::ie::time_of_last_packet::TimeOfLastPacket;
use crate::ie::usage_information::UsageInformation;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReport {
    pub urr_id: UrrId,
    pub ur_seqn: SequenceNumber,
    pub usage_report_trigger: UsageReportTrigger,

    // Phase 1: Core measurement IEs
    pub volume_measurement: Option<VolumeMeasurement>,
    pub duration_measurement: Option<DurationMeasurement>,
    pub time_of_first_packet: Option<TimeOfFirstPacket>,
    pub time_of_last_packet: Option<TimeOfLastPacket>,
    pub usage_information: Option<UsageInformation>,

    // Future phases:
    // pub volume_quota: Option<VolumeQuota>, // IE Type 73
    // pub time_quota: Option<TimeQuota>, // IE Type 76
    // pub quota_holding_time: Option<QuotaHoldingTime>, // IE Type 71
    // pub start_time: Option<StartTime>, // IE Type 77
    // pub end_time: Option<EndTime>, // IE Type 78
    // pub query_urr_reference: Option<QueryUrrReference>, // IE Type 125
    // pub application_detection_information: Option<ApplicationDetectionInformation>, // IE Type 68
    // pub additional_usage_reports_information: Option<AdditionalUsageReportsInformation>, // IE Type 126
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
            volume_measurement: None,
            duration_measurement: None,
            time_of_first_packet: None,
            time_of_last_packet: None,
            usage_information: None,
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

        // Marshal Phase 1 measurement IEs
        if let Some(ref vm) = self.volume_measurement {
            if let Ok(ie) = vm.to_ie() {
                buffer.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ref dm) = self.duration_measurement {
            if let Ok(ie) = dm.to_ie() {
                buffer.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ref tofp) = self.time_of_first_packet {
            if let Ok(ie) = tofp.to_ie() {
                buffer.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ref tolp) = self.time_of_last_packet {
            if let Ok(ie) = tolp.to_ie() {
                buffer.extend_from_slice(&ie.marshal());
            }
        }
        if let Some(ref ui) = self.usage_information {
            if let Ok(ie) = ui.to_ie() {
                buffer.extend_from_slice(&ie.marshal());
            }
        }

        buffer
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = 0;
        let mut urr_id = None;
        let mut ur_seqn = None;
        let mut usage_report_trigger = None;
        let mut volume_measurement = None;
        let mut duration_measurement = None;
        let mut time_of_first_packet = None;
        let mut time_of_last_packet = None;
        let mut usage_information = None;

        while cursor < data.len() {
            let ie = Ie::unmarshal(&data[cursor..])?;
            match ie.ie_type {
                IeType::UrrId => urr_id = Some(UrrId::unmarshal(&ie.payload)?),
                IeType::SequenceNumber => ur_seqn = Some(SequenceNumber::unmarshal(&ie.payload)?),
                IeType::UsageReportTrigger => {
                    usage_report_trigger = Some(UsageReportTrigger::unmarshal(&ie.payload)?)
                }
                IeType::VolumeMeasurement => {
                    volume_measurement = Some(VolumeMeasurement::unmarshal(&ie.payload)?)
                }
                IeType::DurationMeasurement => {
                    duration_measurement = Some(DurationMeasurement::unmarshal(&ie.payload)?)
                }
                IeType::TimeOfFirstPacket => {
                    time_of_first_packet = Some(TimeOfFirstPacket::unmarshal(&ie.payload)?)
                }
                IeType::TimeOfLastPacket => {
                    time_of_last_packet = Some(TimeOfLastPacket::unmarshal(&ie.payload)?)
                }
                IeType::UsageInformation => {
                    usage_information = Some(UsageInformation::unmarshal(&ie.payload)?)
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
            volume_measurement,
            duration_measurement,
            time_of_first_packet,
            time_of_last_packet,
            usage_information,
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
    volume_measurement: Option<VolumeMeasurement>,
    duration_measurement: Option<DurationMeasurement>,
    time_of_first_packet: Option<TimeOfFirstPacket>,
    time_of_last_packet: Option<TimeOfLastPacket>,
    usage_information: Option<UsageInformation>,
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
            volume_measurement: None,
            duration_measurement: None,
            time_of_first_packet: None,
            time_of_last_packet: None,
            usage_information: None,
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
            volume_measurement: self.volume_measurement,
            duration_measurement: self.duration_measurement,
            time_of_first_packet: self.time_of_first_packet,
            time_of_last_packet: self.time_of_last_packet,
            usage_information: self.usage_information,
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

    // Phase 1: Core measurement IE setters

    /// Sets volume measurement data for the usage report.
    ///
    /// Volume measurement contains traffic volume statistics including
    /// total, uplink, and downlink volumes and packet counts.
    ///
    /// # Arguments
    ///
    /// * `volume_measurement` - The volume measurement data
    pub fn volume_measurement(mut self, volume_measurement: VolumeMeasurement) -> Self {
        self.volume_measurement = Some(volume_measurement);
        self
    }

    /// Sets duration measurement for the usage report.
    ///
    /// Duration measurement contains the session duration in seconds.
    ///
    /// # Arguments
    ///
    /// * `duration_measurement` - The duration measurement data
    pub fn duration_measurement(mut self, duration_measurement: DurationMeasurement) -> Self {
        self.duration_measurement = Some(duration_measurement);
        self
    }

    /// Sets time of first packet for the usage report.
    ///
    /// Time of first packet contains the 3GPP NTP timestamp of the first packet.
    ///
    /// # Arguments
    ///
    /// * `time_of_first_packet` - The time of first packet data
    pub fn time_of_first_packet(mut self, time_of_first_packet: TimeOfFirstPacket) -> Self {
        self.time_of_first_packet = Some(time_of_first_packet);
        self
    }

    /// Sets time of last packet for the usage report.
    ///
    /// Time of last packet contains the 3GPP NTP timestamp of the last packet.
    ///
    /// # Arguments
    ///
    /// * `time_of_last_packet` - The time of last packet data
    pub fn time_of_last_packet(mut self, time_of_last_packet: TimeOfLastPacket) -> Self {
        self.time_of_last_packet = Some(time_of_last_packet);
        self
    }

    /// Sets usage information flags for the usage report.
    ///
    /// Usage information contains flags indicating before/after enforcement
    /// and other usage reporting context.
    ///
    /// # Arguments
    ///
    /// * `usage_information` - The usage information flags
    pub fn usage_information(mut self, usage_information: UsageInformation) -> Self {
        self.usage_information = Some(usage_information);
        self
    }

    // Convenience methods for creating measurement data

    /// Convenience method to set volume data with total, uplink, and downlink volumes.
    ///
    /// # Arguments
    ///
    /// * `total` - Total volume in bytes
    /// * `uplink` - Uplink volume in bytes
    /// * `downlink` - Downlink volume in bytes
    pub fn with_volume_data(mut self, total: u64, uplink: u64, downlink: u64) -> Self {
        let volume_measurement = VolumeMeasurement::new(
            0x07, // TOVOL | ULVOL | DLVOL flags
            Some(total),
            Some(uplink),
            Some(downlink),
            None,
            None,
            None,
        );
        self.volume_measurement = Some(volume_measurement);
        self
    }

    /// Convenience method to set packet count data.
    ///
    /// # Arguments
    ///
    /// * `total` - Total packet count
    /// * `uplink` - Uplink packet count
    /// * `downlink` - Downlink packet count
    pub fn with_packet_data(mut self, total: u64, uplink: u64, downlink: u64) -> Self {
        let volume_measurement = VolumeMeasurement::new(
            0x38, // TONOP | ULNOP | DLNOP flags
            None,
            None,
            None,
            Some(total),
            Some(uplink),
            Some(downlink),
        );
        self.volume_measurement = Some(volume_measurement);
        self
    }

    /// Convenience method to set session duration in seconds.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Duration in seconds
    pub fn with_duration(mut self, seconds: u32) -> Self {
        self.duration_measurement = Some(DurationMeasurement::new(seconds));
        self
    }

    /// Convenience method to set packet timing information.
    ///
    /// # Arguments
    ///
    /// * `first_timestamp` - 3GPP NTP timestamp of first packet
    /// * `last_timestamp` - 3GPP NTP timestamp of last packet
    pub fn with_packet_times(mut self, first_timestamp: u32, last_timestamp: u32) -> Self {
        self.time_of_first_packet = Some(TimeOfFirstPacket::new(first_timestamp));
        self.time_of_last_packet = Some(TimeOfLastPacket::new(last_timestamp));
        self
    }

    /// Convenience method to set usage information with flags.
    ///
    /// # Arguments
    ///
    /// * `bef` - Before enforcement flag
    /// * `aft` - After enforcement flag
    /// * `uae` - Usage after enforcement flag
    /// * `ube` - Usage before enforcement flag
    pub fn with_usage_flags(mut self, bef: bool, aft: bool, uae: bool, ube: bool) -> Self {
        self.usage_information = Some(UsageInformation::new_with_flags(bef, aft, uae, ube));
        self
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

    // Phase 1 Tests - Core measurement IEs

    #[test]
    fn test_usage_report_with_volume_measurement() {
        let urr_id = UrrId::new(1);
        let ur_seqn = SequenceNumber::new(42);
        let volume_measurement = VolumeMeasurement::new(
            0x07, // TOVOL | ULVOL | DLVOL
            Some(1000000),
            Some(600000),
            Some(400000),
            None,
            None,
            None,
        );

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .volume_threshold_triggered()
            .volume_measurement(volume_measurement.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.urr_id, urr_id);
        assert_eq!(usage_report.ur_seqn, ur_seqn);
        assert_eq!(usage_report.volume_measurement, Some(volume_measurement));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_with_duration_measurement() {
        let urr_id = UrrId::new(2);
        let ur_seqn = SequenceNumber::new(43);
        let duration_measurement = DurationMeasurement::new(3600); // 1 hour

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .time_threshold_triggered()
            .duration_measurement(duration_measurement.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.duration_measurement, Some(duration_measurement));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_with_time_of_first_packet() {
        let urr_id = UrrId::new(3);
        let ur_seqn = SequenceNumber::new(44);
        let time_of_first_packet = TimeOfFirstPacket::new(0x12345678);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .start_of_traffic()
            .time_of_first_packet(time_of_first_packet.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.time_of_first_packet, Some(time_of_first_packet));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_with_time_of_last_packet() {
        let urr_id = UrrId::new(4);
        let ur_seqn = SequenceNumber::new(45);
        let time_of_last_packet = TimeOfLastPacket::new(0x87654321);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .stop_of_traffic()
            .time_of_last_packet(time_of_last_packet.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.time_of_last_packet, Some(time_of_last_packet));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_with_usage_information() {
        let urr_id = UrrId::new(5);
        let ur_seqn = SequenceNumber::new(46);
        let usage_information = UsageInformation::new_with_flags(true, false, true, false);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .periodic_report()
            .usage_information(usage_information.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.usage_information, Some(usage_information));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_with_all_measurements() {
        let urr_id = UrrId::new(6);
        let ur_seqn = SequenceNumber::new(47);
        let volume_measurement = VolumeMeasurement::new(
            0x3F, // All flags
            Some(2000000),
            Some(1200000),
            Some(800000),
            Some(2000),
            Some(1200),
            Some(800),
        );
        let duration_measurement = DurationMeasurement::new(7200); // 2 hours
        let time_of_first_packet = TimeOfFirstPacket::new(0x11111111);
        let time_of_last_packet = TimeOfLastPacket::new(0x22222222);
        let usage_information = UsageInformation::new_with_flags(true, true, false, false);

        let usage_report = UsageReportBuilder::new(urr_id.clone())
            .sequence_number(ur_seqn.clone())
            .quota_exhausted()
            .volume_measurement(volume_measurement.clone())
            .duration_measurement(duration_measurement.clone())
            .time_of_first_packet(time_of_first_packet.clone())
            .time_of_last_packet(time_of_last_packet.clone())
            .usage_information(usage_information.clone())
            .build()
            .unwrap();

        assert_eq!(usage_report.volume_measurement, Some(volume_measurement));
        assert_eq!(usage_report.duration_measurement, Some(duration_measurement));
        assert_eq!(usage_report.time_of_first_packet, Some(time_of_first_packet));
        assert_eq!(usage_report.time_of_last_packet, Some(time_of_last_packet));
        assert_eq!(usage_report.usage_information, Some(usage_information));

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_builder_convenience_methods_phase1() {
        // Test with_volume_data convenience method
        let volume_report = UsageReportBuilder::new(UrrId::new(1))
            .sequence_number(SequenceNumber::new(100))
            .volume_threshold_triggered()
            .with_volume_data(5000000, 3000000, 2000000)
            .build()
            .unwrap();

        let vm = volume_report.volume_measurement.unwrap();
        assert!(vm.has_total_volume());
        assert!(vm.has_uplink_volume());
        assert!(vm.has_downlink_volume());
        assert_eq!(vm.total_volume, Some(5000000));
        assert_eq!(vm.uplink_volume, Some(3000000));
        assert_eq!(vm.downlink_volume, Some(2000000));

        // Test with_packet_data convenience method
        let packet_report = UsageReportBuilder::new(UrrId::new(2))
            .sequence_number(SequenceNumber::new(101))
            .volume_threshold_triggered()
            .with_packet_data(10000, 6000, 4000)
            .build()
            .unwrap();

        let vm = packet_report.volume_measurement.unwrap();
        assert!(vm.has_total_packets());
        assert!(vm.has_uplink_packets());
        assert!(vm.has_downlink_packets());
        assert_eq!(vm.total_packets, Some(10000));
        assert_eq!(vm.uplink_packets, Some(6000));
        assert_eq!(vm.downlink_packets, Some(4000));

        // Test with_duration convenience method
        let duration_report = UsageReportBuilder::new(UrrId::new(3))
            .sequence_number(SequenceNumber::new(102))
            .time_threshold_triggered()
            .with_duration(1800)
            .build()
            .unwrap();

        let dm = duration_report.duration_measurement.unwrap();
        assert_eq!(dm.duration_seconds, 1800);

        // Test with_packet_times convenience method
        let timing_report = UsageReportBuilder::new(UrrId::new(4))
            .sequence_number(SequenceNumber::new(103))
            .start_of_traffic()
            .with_packet_times(0x12345678, 0x87654321)
            .build()
            .unwrap();

        let tofp = timing_report.time_of_first_packet.unwrap();
        let tolp = timing_report.time_of_last_packet.unwrap();
        assert_eq!(tofp.timestamp, 0x12345678);
        assert_eq!(tolp.timestamp, 0x87654321);

        // Test with_usage_flags convenience method
        let usage_report = UsageReportBuilder::new(UrrId::new(5))
            .sequence_number(SequenceNumber::new(104))
            .periodic_report()
            .with_usage_flags(true, false, true, false)
            .build()
            .unwrap();

        let ui = usage_report.usage_information.unwrap();
        assert!(ui.has_bef());
        assert!(!ui.has_aft());
        assert!(ui.has_uae());
        assert!(!ui.has_ube());
    }

    #[test]
    fn test_usage_report_comprehensive_measurement_scenario() {
        // Simulate a realistic quota exhaustion scenario with all measurements
        let usage_report = UsageReportBuilder::quota_exhausted_report(
            UrrId::new(99),
            SequenceNumber::new(255)
        )
        .with_volume_data(5000000000, 3000000000, 2000000000) // 5GB total, 3GB up, 2GB down
        .with_packet_data(5000000, 3000000, 2000000) // 5M packets total
        .with_duration(3600) // 1 hour session
        .with_packet_times(0x60000000, 0x60000E10) // Session timestamps
        .with_usage_flags(false, true, false, true) // After enforcement, before enforcement
        .build()
        .unwrap();

        // Verify trigger
        assert_eq!(
            usage_report.usage_report_trigger,
            UsageReportTrigger::VOLTH | UsageReportTrigger::TIMTH
        );

        // Verify all measurements are present
        assert!(usage_report.volume_measurement.is_some());
        assert!(usage_report.duration_measurement.is_some());
        assert!(usage_report.time_of_first_packet.is_some());
        assert!(usage_report.time_of_last_packet.is_some());
        assert!(usage_report.usage_information.is_some());

        // Test marshal/unmarshal round trip
        let marshaled = usage_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(usage_report, unmarshaled);

        // Test IE conversion
        let ie = usage_report.to_ie();
        assert_eq!(ie.ie_type, IeType::UsageReport);
    }

    #[test]
    fn test_usage_report_phase1_marshal_unmarshal_edge_cases() {
        // Test with zero values
        let zero_report = UsageReportBuilder::new(UrrId::new(1))
            .sequence_number(SequenceNumber::new(1))
            .periodic_report()
            .with_volume_data(0, 0, 0)
            .with_duration(0)
            .with_packet_times(0, 0)
            .with_usage_flags(false, false, false, false)
            .build()
            .unwrap();

        let marshaled = zero_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(zero_report, unmarshaled);

        // Test with maximum values
        let max_report = UsageReportBuilder::new(UrrId::new(2))
            .sequence_number(SequenceNumber::new(2))
            .periodic_report()
            .with_volume_data(u64::MAX, u64::MAX, u64::MAX)
            .with_duration(u32::MAX)
            .with_packet_times(u32::MAX, u32::MAX)
            .with_usage_flags(true, true, true, true)
            .build()
            .unwrap();

        let marshaled = max_report.marshal();
        let unmarshaled = UsageReport::unmarshal(&marshaled).unwrap();
        assert_eq!(max_report, unmarshaled);
    }
}
