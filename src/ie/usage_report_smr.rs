//! Usage Report Within Session Modification Response IE implementation.
//!
//! Per 3GPP TS 29.244 Section 8.2.78, the Usage Report Within Session
//! Modification Response IE is used to report usage information in response
//! to a PFCP Session Modification Request.
//!
//! This IE has the same structure as Usage Report (IE 144) but uses IE type 78
//! to indicate it appears within a Session Modification Response message.

use crate::ie::usage_report::UsageReport;
use crate::ie::{Ie, IeType};
use std::io;

/// Usage Report Within Session Modification Response Information Element.
///
/// This is a context-specific wrapper around the core UsageReport structure.
/// It contains the same fields and behavior as UsageReport, but with IE type 78
/// to indicate it appears in a Session Modification Response message.
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
/// use rs_pfcp::ie::usage_report_smr::UsageReportSmr;
/// use rs_pfcp::ie::usage_report::UsageReportBuilder;
/// use rs_pfcp::ie::urr_id::UrrId;
/// use rs_pfcp::ie::sequence_number::SequenceNumber;
///
/// let usage_report = UsageReportBuilder::quota_exhausted_report(
///     UrrId::new(1),
///     SequenceNumber::new(42)
/// ).build().unwrap();
///
/// let smr_report = UsageReportSmr::new(usage_report);
/// let ie = smr_report.to_ie();
/// assert_eq!(ie.ie_type, rs_pfcp::ie::IeType::UsageReportWithinSessionModificationResponse);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportSmr {
    /// The underlying usage report data
    pub report: UsageReport,
}

impl UsageReportSmr {
    /// Creates a new Usage Report Within Session Modification Response.
    ///
    /// # Arguments
    /// * `report` - The underlying UsageReport containing measurement data
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::usage_report_smr::UsageReportSmr;
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
    /// let smr_report = UsageReportSmr::new(report);
    /// ```
    pub fn new(report: UsageReport) -> Self {
        UsageReportSmr { report }
    }

    /// Marshals the Usage Report Within Session Modification Response to bytes.
    ///
    /// # Returns
    /// A vector containing the marshaled usage report payload (child IEs).
    pub fn marshal(&self) -> Vec<u8> {
        self.report.marshal()
    }

    /// Unmarshals a Usage Report Within Session Modification Response from bytes.
    ///
    /// # Arguments
    /// * `data` - The byte slice containing the usage report payload
    ///
    /// # Returns
    /// A `UsageReportSmr` instance or an error if unmarshaling fails.
    ///
    /// # Errors
    /// Returns `io::Error` if:
    /// - Required fields (URR ID, UR-SEQN, Usage Report Trigger) are missing
    /// - Any child IE cannot be parsed
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(UsageReportSmr {
            report: UsageReport::unmarshal(data)?,
        })
    }

    /// Wraps the Usage Report in a generic IE with type UsageReportWithinSessionModificationResponse.
    ///
    /// # Returns
    /// An `Ie` with type 78 and the marshaled payload.
    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::UsageReportWithinSessionModificationResponse,
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
    fn test_usage_report_smr_new() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(42),
            UsageReportTrigger::PERIO,
        );
        let smr_report = UsageReportSmr::new(report.clone());
        assert_eq!(smr_report.report, report);
    }

    #[test]
    fn test_usage_report_smr_marshal_unmarshal() {
        let report =
            UsageReportBuilder::quota_exhausted_report(UrrId::new(10), SequenceNumber::new(100))
                .build()
                .unwrap();
        let smr_report = UsageReportSmr::new(report);

        let marshaled = smr_report.marshal();
        let unmarshaled = UsageReportSmr::unmarshal(&marshaled).unwrap();

        assert_eq!(smr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_smr_round_trip() {
        let report =
            UsageReportBuilder::periodic_usage_report(UrrId::new(5), SequenceNumber::new(50))
                .build()
                .unwrap();
        let original = UsageReportSmr::new(report);

        let marshaled = original.marshal();
        let unmarshaled = UsageReportSmr::unmarshal(&marshaled).unwrap();

        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_usage_report_smr_to_ie() {
        let report = UsageReport::new(
            UrrId::new(1),
            SequenceNumber::new(1),
            UsageReportTrigger::VOLTH,
        );
        let smr_report = UsageReportSmr::new(report);
        let ie = smr_report.to_ie();

        assert_eq!(
            ie.ie_type,
            IeType::UsageReportWithinSessionModificationResponse
        );
        assert_eq!(ie.payload, smr_report.marshal());
    }

    #[test]
    fn test_usage_report_smr_ie_round_trip() {
        let report =
            UsageReportBuilder::volume_threshold_report(UrrId::new(7), SequenceNumber::new(77))
                .build()
                .unwrap();
        let smr_report = UsageReportSmr::new(report);

        let ie = smr_report.to_ie();
        let unmarshaled = UsageReportSmr::unmarshal(&ie.payload).unwrap();

        assert_eq!(smr_report, unmarshaled);
    }

    #[test]
    fn test_usage_report_smr_edge_cases() {
        // Test with minimal report
        let minimal_report = UsageReport::new(
            UrrId::new(0),
            SequenceNumber::new(0),
            UsageReportTrigger::new(0),
        );
        let smr = UsageReportSmr::new(minimal_report);
        let marshaled = smr.marshal();
        let unmarshaled = UsageReportSmr::unmarshal(&marshaled).unwrap();
        assert_eq!(smr, unmarshaled);

        // Test with maximum URR ID
        let max_report = UsageReport::new(
            UrrId::new(u32::MAX),
            SequenceNumber::new(u32::MAX),
            UsageReportTrigger::new(u8::MAX),
        );
        let smr = UsageReportSmr::new(max_report);
        let marshaled = smr.marshal();
        let unmarshaled = UsageReportSmr::unmarshal(&marshaled).unwrap();
        assert_eq!(smr, unmarshaled);
    }

    #[test]
    fn test_usage_report_smr_unmarshal_error_cases() {
        // Empty payload
        let result = UsageReportSmr::unmarshal(&[]);
        assert!(result.is_err());

        // Too short for even URR ID
        let result = UsageReportSmr::unmarshal(&[0x00, 0x01]);
        assert!(result.is_err());
    }

    #[test]
    fn test_usage_report_smr_with_builder() {
        let report = UsageReportBuilder::new(UrrId::new(99))
            .sequence_number(SequenceNumber::new(255))
            .quota_exhausted()
            .with_volume_data(5000000, 3000000, 2000000)
            .with_duration(3600)
            .build()
            .unwrap();

        let smr = UsageReportSmr::new(report);

        // Verify marshal/unmarshal preserves all data
        let marshaled = smr.marshal();
        let unmarshaled = UsageReportSmr::unmarshal(&marshaled).unwrap();
        assert_eq!(smr, unmarshaled);

        // Verify IE type is correct
        let ie = smr.to_ie();
        assert_eq!(
            ie.ie_type,
            IeType::UsageReportWithinSessionModificationResponse
        );
    }
}
