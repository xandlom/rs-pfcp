//! Usage Report Within Session Report Request IE implementation.
//!
//! Per 3GPP TS 29.244 Section 8.2.80, the Usage Report Within Session
//! Report Request IE is used to report usage information within a
//! PFCP Session Report Request message.
//!
//! This IE has the same structure as Usage Report (IE 144) but uses IE type 80
//! to indicate it appears within a Session Report Request message.

use crate::error::PfcpError;
use crate::ie::usage_report::UsageReport;
use crate::ie::{Ie, IeType};

/// Usage Report Within Session Report Request Information Element.
///
/// This is a context-specific wrapper around the core UsageReport structure.
/// It contains the same fields and behavior as UsageReport, but with IE type 80
/// to indicate it appears in a Session Report Request message.
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
/// use rs_pfcp::ie::usage_report_srr::UsageReportSrr;
/// use rs_pfcp::ie::usage_report::UsageReportBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
/// use rs_pfcp::ie::sequence_number::SequenceNumber;
///
/// let usage_report = UsageReportBuilder::periodic_usage_report(
///     UrrId::new(1),
///     SequenceNumber::new(42)
/// ).build().unwrap();
///
/// let srr_report = UsageReportSrr::new(usage_report);
/// let ie = srr_report.to_ie();
/// assert_eq!(ie.ie_type, rs_pfcp::ie::IeType::UsageReportWithinSessionReportRequest);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportSrr {
    /// The underlying usage report data
    pub report: UsageReport,
}

impl UsageReportSrr {
    /// Creates a new Usage Report Within Session Report Request.
    ///
    /// # Arguments
    /// * `report` - The underlying UsageReport containing measurement data
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::usage_report_srr::UsageReportSrr;
    /// use rs_pfcp::ie::usage_report::UsageReport;
    /// use rs_pfcp::ie::urr_id::UrrId;
    /// use rs_pfcp::ie::sequence_number::SequenceNumber;
    /// use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;
    ///
    /// let report = UsageReport::new(
    ///     UrrId::new(1),
    ///     SequenceNumber::new(42),
    ///     UsageReportTrigger::PERIO
    /// );
    /// let srr_report = UsageReportSrr::new(report);
    /// ```
    pub fn new(report: UsageReport) -> Self {
        UsageReportSrr { report }
    }

    /// Marshals the Usage Report Within Session Report Request to bytes.
    ///
    /// # Returns
    /// A vector containing the marshaled usage report payload (child IEs).
    pub fn marshal(&self) -> Vec<u8> {
        self.report.marshal()
    }

    /// Unmarshals a Usage Report Within Session Report Request from bytes.
    ///
    /// # Arguments
    /// * `data` - The byte slice containing the usage report payload
    ///
    /// # Returns
    /// A `UsageReportSrr` instance or an error if unmarshaling fails.
    ///
    /// # Errors
    /// Returns `PfcpError` if:
    /// - Required fields (URR ID, UR-SEQN, Usage Report Trigger) are missing
    /// - Any child IE cannot be parsed
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        Ok(UsageReportSrr {
            report: UsageReport::unmarshal(data)?,
        })
    }

    /// Wraps the Usage Report in a generic IE with type UsageReportWithinSessionReportRequest.
    ///
    /// # Returns
    /// An `Ie` with type 80 and the marshaled payload.
    ///
    /// # Note
    /// This is equivalent to calling `UsageReport::to_ie()` directly, as the default
    /// `to_ie()` method on UsageReport uses IE type 80.
    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::UsageReportWithinSessionReportRequest,
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
    fn test_usage_report_srr_new() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(42),
            UsageReportTrigger::PERIO,
        );
        let srr_report = UsageReportSrr::new(report.clone());
        assert_eq!(srr_report.report, report);
    }

    #[test]
    fn test_usage_report_srr_marshal_unmarshal() {
        let report =
            UsageReportBuilder::periodic_usage_report(UrrId::new(10), SequenceNumber::new(100))
                .build()
                .unwrap();
        let srr_report = UsageReportSrr::new(report);

        let marshaled = srr_report.marshal();
        let unmarshaled = UsageReportSrr::unmarshal(&marshaled).unwrap();

        assert_eq!(srr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_srr_round_trip() {
        let report =
            UsageReportBuilder::quota_exhausted_report(UrrId::new(5), SequenceNumber::new(50))
                .build()
                .unwrap();
        let original = UsageReportSrr::new(report);

        let marshaled = original.marshal();
        let unmarshaled = UsageReportSrr::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_usage_report_srr_to_ie() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(1),
            UsageReportTrigger::PERIO,
        );
        let srr_report = UsageReportSrr::new(report);
        let ie = srr_report.to_ie();

        assert_eq!(ie.ie_type, IeType::UsageReportWithinSessionReportRequest);
        assert_eq!(ie.payload, srr_report.marshal());
    }

    #[test]
    fn test_usage_report_srr_ie_round_trip() {
        let report =
            UsageReportBuilder::volume_threshold_report(UrrId::new(7), SequenceNumber::new(77))
                .build()
                .unwrap();
        let srr_report = UsageReportSrr::new(report);

        let ie = srr_report.to_ie();
        let unmarshaled = UsageReportSrr::unmarshal(&ie.payload).unwrap();

        assert_eq!(srr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_srr_edge_cases() {
        // Test with minimal report
        let minimal_report = UsageReport::new(
            UrrId::new(0),
            SequenceNumber::new(0),
            UsageReportTrigger::new(0),
        );
        let srr = UsageReportSrr::new(minimal_report);
        let marshaled = srr.marshal();
        let unmarshaled = UsageReportSrr::unmarshal(&marshaled).unwrap();
        assert_eq!(srr, unmarshaled);

        // Test with maximum URR ID
        let max_report = UsageReport::new(
            UrrId::new(u32::MAX),
            SequenceNumber::new(u32::MAX),
            UsageReportTrigger::new(u8::MAX),
        );
        let srr = UsageReportSrr::new(max_report);
        let marshaled = srr.marshal();
        let unmarshaled = UsageReportSrr::unmarshal(&marshaled).unwrap();
        assert_eq!(srr, unmarshaled);
    }

    #[test]
    fn test_usage_report_srr_unmarshal_error_cases() {
        // Empty payload
        let result = UsageReportSrr::unmarshal(&[]);
        assert!(result.is_err());

        // Too short for even URR ID
        let result = UsageReportSrr::unmarshal(&[0x00, 0x01]);
        assert!(result.is_err());
    }

    #[test]
    fn test_usage_report_srr_with_builder() {
        let report = UsageReportBuilder::new(UrrId::new(99))
            .sequence_number(SequenceNumber::new(255))
            .periodic_report()
            .with_volume_data(5000000, 3000000, 2000000)
            .with_duration(3600)
            .build()
            .unwrap();

        let srr = UsageReportSrr::new(report);

        // Verify marshal/unmarshal preserves all data
        let marshaled = srr.marshal();
        let unmarshaled = UsageReportSrr::unmarshal(&marshaled).unwrap();
        assert_eq!(srr, unmarshaled);

        // Verify IE type is correct
        let ie = srr.to_ie();
        assert_eq!(ie.ie_type, IeType::UsageReportWithinSessionReportRequest);
    }

    #[test]
    fn test_usage_report_srr_equivalent_to_usage_report_to_ie() {
        // Verify that UsageReportSrr::to_ie() produces the same result as UsageReport::to_ie()
        let report =
            UsageReportBuilder::periodic_usage_report(UrrId::new(42), SequenceNumber::new(123))
                .build()
                .unwrap();

        let srr = UsageReportSrr::new(report.clone());
        let srr_ie = srr.to_ie();
        let report_ie = report.to_ie();

        assert_eq!(srr_ie.ie_type, report_ie.ie_type);
        assert_eq!(srr_ie.payload, report_ie.payload);
    }
}
