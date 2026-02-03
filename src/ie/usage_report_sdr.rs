//! Usage Report Within Session Deletion Response IE implementation.
//!
//! Per 3GPP TS 29.244 Section 8.2.79, the Usage Report Within Session
//! Deletion Response IE is used to report usage information in response
//! to a PFCP Session Deletion Request.
//!
//! This IE has the same structure as Usage Report (IE 144) but uses IE type 79
//! to indicate it appears within a Session Deletion Response message.

use crate::error::PfcpError;
use crate::ie::usage_report::UsageReport;
use crate::ie::{Ie, IeType};

/// Usage Report Within Session Deletion Response Information Element.
///
/// This is a context-specific wrapper around the core UsageReport structure.
/// It contains the same fields and behavior as UsageReport, but with IE type 79
/// to indicate it appears in a Session Deletion Response message.
///
/// # Structure
/// Identical to UsageReport (IE 144), containing:
/// - URR ID (mandatory)
/// - UR-SEQN (mandatory)
/// - Usage Report Trigger (mandatory)
/// - Plus optional measurement, quota, and extended IEs
///
/// # Example
/// ```
/// use rs_pfcp::ie::usage_report_sdr::UsageReportSdr;
/// use rs_pfcp::ie::usage_report::UsageReportBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
/// use rs_pfcp::ie::sequence_number::SequenceNumber;
///
/// let usage_report = UsageReportBuilder::stop_of_traffic_report(
///     UrrId::new(1),
///     SequenceNumber::new(42)
/// ).build().unwrap();
///
/// let sdr_report = UsageReportSdr::new(usage_report);
/// let ie = sdr_report.to_ie();
/// assert_eq!(ie.ie_type, rs_pfcp::ie::IeType::UsageReportWithinSessionDeletionResponse);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportSdr {
    /// The underlying usage report data
    pub report: UsageReport,
}

impl UsageReportSdr {
    /// Creates a new Usage Report Within Session Deletion Response.
    ///
    /// # Arguments
    /// * `report` - The underlying UsageReport containing measurement data
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::usage_report_sdr::UsageReportSdr;
    /// use rs_pfcp::ie::usage_report::UsageReport;
    /// use rs_pfcp::ie::urr_id::UrrId;
    /// use rs_pfcp::ie::sequence_number::SequenceNumber;
    /// use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;
    ///
    /// let report = UsageReport::new(
    ///     UrrId::new(1),
    ///     SequenceNumber::new(42),
    ///     UsageReportTrigger::STOPT
    /// );
    /// let sdr_report = UsageReportSdr::new(report);
    /// ```
    pub fn new(report: UsageReport) -> Self {
        UsageReportSdr { report }
    }

    /// Marshals the Usage Report Within Session Deletion Response to bytes.
    ///
    /// # Returns
    /// A vector containing the marshaled usage report payload (child IEs).
    pub fn marshal(&self) -> Vec<u8> {
        self.report.marshal()
    }

    /// Unmarshals a Usage Report Within Session Deletion Response from bytes.
    ///
    /// # Arguments
    /// * `data` - The byte slice containing the usage report payload
    ///
    /// # Returns
    /// A `UsageReportSdr` instance or an error if unmarshaling fails.
    ///
    /// # Errors
    /// Returns `PfcpError` if:
    /// - Required fields (URR ID, UR-SEQN, Usage Report Trigger) are missing
    /// - Any child IE cannot be parsed
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(UsageReportSdr {
            report: UsageReport::unmarshal(data)?,
        })
    }

    /// Wraps the Usage Report in a generic IE with type UsageReportWithinSessionDeletionResponse.
    ///
    /// # Returns
    /// An `Ie` with type 79 and the marshaled payload.
    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::UsageReportWithinSessionDeletionResponse,
            self.marshal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ie::sequence_number::SequenceNumber;
    use crate::ie::urr_id::UrrId;
    use crate::ie::usage_report::UsageReportBuilder;
    use crate::ie::usage_report_trigger::UsageReportTrigger;

    #[test]
    fn test_usage_report_sdr_new() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(42),
            UsageReportTrigger::STOPT,
        );
        let sdr_report = UsageReportSdr::new(report.clone());
        assert_eq!(sdr_report.report, report);
    }

    #[test]
    fn test_usage_report_sdr_marshal_unmarshal() {
        let report =
            UsageReportBuilder::stop_of_traffic_report(UrrId::new(10), SequenceNumber::new(100))
                .build()
                .unwrap();
        let sdr_report = UsageReportSdr::new(report);

        let marshaled = sdr_report.marshal();
        let unmarshaled = UsageReportSdr::unmarshal(&marshaled).unwrap();

        assert_eq!(sdr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_sdr_round_trip() {
        let report =
            UsageReportBuilder::periodic_usage_report(UrrId::new(5), SequenceNumber::new(50))
                .build()
                .unwrap();
        let original = UsageReportSdr::new(report);

        let marshaled = original.marshal();
        let unmarshaled = UsageReportSdr::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_usage_report_sdr_to_ie() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(1),
            UsageReportTrigger::STOPT,
        );
        let sdr_report = UsageReportSdr::new(report);
        let ie = sdr_report.to_ie();

        assert_eq!(ie.ie_type, IeType::UsageReportWithinSessionDeletionResponse);
        assert_eq!(ie.payload, sdr_report.marshal());
    }

    #[test]
    fn test_usage_report_sdr_ie_round_trip() {
        let report =
            UsageReportBuilder::time_threshold_report(UrrId::new(7), SequenceNumber::new(77))
                .build()
                .unwrap();
        let sdr_report = UsageReportSdr::new(report);

        let ie = sdr_report.to_ie();
        let unmarshaled = UsageReportSdr::unmarshal(&ie.payload).unwrap();

        assert_eq!(sdr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_sdr_edge_cases() {
        // Test with minimal report
        let minimal_report = UsageReport::new(
            UrrId::new(0),
            SequenceNumber::new(0),
            UsageReportTrigger::new(0),
        );
        let sdr = UsageReportSdr::new(minimal_report);
        let marshaled = sdr.marshal();
        let unmarshaled = UsageReportSdr::unmarshal(&marshaled).unwrap();
        assert_eq!(sdr, unmarshaled);

        // Test with maximum URR ID
        let max_report = UsageReport::new(
            UrrId::new(u32::MAX),
            SequenceNumber::new(u32::MAX),
            UsageReportTrigger::new(u8::MAX),
        );
        let sdr = UsageReportSdr::new(max_report);
        let marshaled = sdr.marshal();
        let unmarshaled = UsageReportSdr::unmarshal(&marshaled).unwrap();
        assert_eq!(sdr, unmarshaled);
    }

    #[test]
    fn test_usage_report_sdr_unmarshal_error_cases() {
        // Empty payload
        let result = UsageReportSdr::unmarshal(&[]);
        assert!(result.is_err());

        // Too short for even URR ID
        let result = UsageReportSdr::unmarshal(&[0x00, 0x01]);
        assert!(result.is_err());
    }

    #[test]
    fn test_usage_report_sdr_with_builder() {
        let report = UsageReportBuilder::new(UrrId::new(99))
            .sequence_number(SequenceNumber::new(255))
            .stop_of_traffic()
            .with_volume_data(5000000, 3000000, 2000000)
            .with_duration(3600)
            .build()
            .unwrap();

        let sdr = UsageReportSdr::new(report);

        // Verify marshal/unmarshal preserves all data
        let marshaled = sdr.marshal();
        let unmarshaled = UsageReportSdr::unmarshal(&marshaled).unwrap();
        assert_eq!(sdr, unmarshaled);

        // Verify IE type is correct
        let ie = sdr.to_ie();
        assert_eq!(ie.ie_type, IeType::UsageReportWithinSessionDeletionResponse);
    }
}
