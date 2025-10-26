//! Report Type Information Element
//!
//! The Report Type IE identifies the type of report being sent from the User Plane
//! Function to the Control Plane Function in PFCP messages.
//! Per 3GPP TS 29.244 Section 8.2.21.

use crate::ie::{Ie, IeType};
use std::io;

/// Report Type
///
/// Identifies the type of report in PFCP Session Report Request messages.
/// Multiple report type flags can be set simultaneously to indicate combined reports.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.21
///
/// # Structure
/// - Octet 5: Report type flags (bits 1-4 + optional bits 5-8)
/// - Octet 6 (optional): Additional report type flags (when subscriber trace enabled)
///
/// # Report Types
/// - **DLDR**: Downlink Data Report
/// - **USAR**: Usage Report
/// - **ERIR**: Error Indication Report
/// - **UPIR**: User Plane Inactivity Report
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::report_type::ReportType;
///
/// // Create a usage report
/// let usage_report = ReportType::new().with_usage_report(true);
/// assert!(usage_report.is_usage_report());
///
/// // Create a combined downlink data + usage report
/// let combined = ReportType::new()
///     .with_downlink_data_report(true)
///     .with_usage_report(true);
/// assert!(combined.is_downlink_data_report());
/// assert!(combined.is_usage_report());
///
/// // Marshal and unmarshal
/// let bytes = usage_report.marshal();
/// let parsed = ReportType::unmarshal(&bytes).unwrap();
/// assert_eq!(usage_report, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportType {
    /// Downlink Data Report (bit 1 of octet 5)
    pub downlink_data_report: bool,
    /// Usage Report (bit 2 of octet 5)
    pub usage_report: bool,
    /// Error Indication Report (bit 3 of octet 5)
    pub error_indication_report: bool,
    /// User Plane Inactivity Report (bit 4 of octet 5)
    pub user_plane_inactivity_report: bool,
}

impl ReportType {
    /// Create a new Report Type with all flags disabled
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new();
    /// assert!(!report.is_usage_report());
    /// assert!(!report.is_downlink_data_report());
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the Downlink Data Report flag
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new().with_downlink_data_report(true);
    /// assert!(report.is_downlink_data_report());
    /// ```
    pub fn with_downlink_data_report(mut self, enabled: bool) -> Self {
        self.downlink_data_report = enabled;
        self
    }

    /// Set the Usage Report flag
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new().with_usage_report(true);
    /// assert!(report.is_usage_report());
    /// ```
    pub fn with_usage_report(mut self, enabled: bool) -> Self {
        self.usage_report = enabled;
        self
    }

    /// Set the Error Indication Report flag
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new().with_error_indication_report(true);
    /// assert!(report.is_error_indication_report());
    /// ```
    pub fn with_error_indication_report(mut self, enabled: bool) -> Self {
        self.error_indication_report = enabled;
        self
    }

    /// Set the User Plane Inactivity Report flag
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new().with_user_plane_inactivity_report(true);
    /// assert!(report.is_user_plane_inactivity_report());
    /// ```
    pub fn with_user_plane_inactivity_report(mut self, enabled: bool) -> Self {
        self.user_plane_inactivity_report = enabled;
        self
    }

    /// Check if Downlink Data Report is enabled
    pub fn is_downlink_data_report(&self) -> bool {
        self.downlink_data_report
    }

    /// Check if Usage Report is enabled
    pub fn is_usage_report(&self) -> bool {
        self.usage_report
    }

    /// Check if Error Indication Report is enabled
    pub fn is_error_indication_report(&self) -> bool {
        self.error_indication_report
    }

    /// Check if User Plane Inactivity Report is enabled
    pub fn is_user_plane_inactivity_report(&self) -> bool {
        self.user_plane_inactivity_report
    }

    /// Marshal Report Type to bytes
    ///
    /// # Returns
    /// 1-byte vector containing report type flags in octet 5
    pub fn marshal(&self) -> Vec<u8> {
        let mut octet5 = 0u8;

        if self.downlink_data_report {
            octet5 |= 0x01; // Bit 1
        }
        if self.usage_report {
            octet5 |= 0x02; // Bit 2
        }
        if self.error_indication_report {
            octet5 |= 0x04; // Bit 3
        }
        if self.user_plane_inactivity_report {
            octet5 |= 0x08; // Bit 4
        }

        vec![octet5]
    }

    /// Unmarshal Report Type from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing report type data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::new().with_usage_report(true);
    /// let bytes = report.marshal();
    /// let parsed = ReportType::unmarshal(&bytes).unwrap();
    /// assert_eq!(report, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Report Type payload too short: expected at least 1 byte",
            ));
        }

        let octet5 = data[0];

        Ok(ReportType {
            downlink_data_report: (octet5 & 0x01) != 0,
            usage_report: (octet5 & 0x02) != 0,
            error_indication_report: (octet5 & 0x04) != 0,
            user_plane_inactivity_report: (octet5 & 0x08) != 0,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let report = ReportType::new().with_usage_report(true);
    /// let ie = report.to_ie();
    /// assert_eq!(ie.ie_type, IeType::ReportType);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportType, self.marshal())
    }

    // Convenience constructors for common report types

    /// Create a Downlink Data Report
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::downlink_data_report();
    /// assert!(report.is_downlink_data_report());
    /// assert!(!report.is_usage_report());
    /// ```
    pub fn downlink_data_report() -> Self {
        ReportType::new().with_downlink_data_report(true)
    }

    /// Create a Usage Report
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::usage_report();
    /// assert!(report.is_usage_report());
    /// assert!(!report.is_downlink_data_report());
    /// ```
    pub fn usage_report() -> Self {
        ReportType::new().with_usage_report(true)
    }

    /// Create an Error Indication Report
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::error_indication_report();
    /// assert!(report.is_error_indication_report());
    /// ```
    pub fn error_indication_report() -> Self {
        ReportType::new().with_error_indication_report(true)
    }

    /// Create a User Plane Inactivity Report
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::report_type::ReportType;
    ///
    /// let report = ReportType::user_plane_inactivity_report();
    /// assert!(report.is_user_plane_inactivity_report());
    /// ```
    pub fn user_plane_inactivity_report() -> Self {
        ReportType::new().with_user_plane_inactivity_report(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_type_new() {
        let report = ReportType::new();
        assert!(!report.is_downlink_data_report());
        assert!(!report.is_usage_report());
        assert!(!report.is_error_indication_report());
        assert!(!report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_default() {
        let report: ReportType = Default::default();
        assert!(!report.downlink_data_report);
        assert!(!report.usage_report);
    }

    #[test]
    fn test_report_type_downlink_data_report() {
        let report = ReportType::new().with_downlink_data_report(true);
        assert!(report.is_downlink_data_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_usage_report() {
        let report = ReportType::new().with_usage_report(true);
        assert!(!report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(!report.is_error_indication_report());
    }

    #[test]
    fn test_report_type_error_indication_report() {
        let report = ReportType::new().with_error_indication_report(true);
        assert!(report.is_error_indication_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_user_plane_inactivity_report() {
        let report = ReportType::new().with_user_plane_inactivity_report(true);
        assert!(report.is_user_plane_inactivity_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_combined_downlink_usage() {
        let report = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true);
        assert!(report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(!report.is_error_indication_report());
    }

    #[test]
    fn test_report_type_all_flags() {
        let report = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true)
            .with_error_indication_report(true)
            .with_user_plane_inactivity_report(true);

        assert!(report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(report.is_error_indication_report());
        assert!(report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_marshal_empty() {
        let report = ReportType::new();
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x00);
    }

    #[test]
    fn test_report_type_marshal_downlink_data() {
        let report = ReportType::new().with_downlink_data_report(true);
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x01); // Bit 1 set
    }

    #[test]
    fn test_report_type_marshal_usage_report() {
        let report = ReportType::new().with_usage_report(true);
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x02); // Bit 2 set
    }

    #[test]
    fn test_report_type_marshal_error_indication() {
        let report = ReportType::new().with_error_indication_report(true);
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x04); // Bit 3 set
    }

    #[test]
    fn test_report_type_marshal_user_plane_inactivity() {
        let report = ReportType::new().with_user_plane_inactivity_report(true);
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x08); // Bit 4 set
    }

    #[test]
    fn test_report_type_marshal_combined() {
        let report = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true);
        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x03); // Bits 1 and 2 set
    }

    #[test]
    fn test_report_type_marshal_all_flags() {
        let report = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true)
            .with_error_indication_report(true)
            .with_user_plane_inactivity_report(true);

        let bytes = report.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x0F); // Bits 1-4 set
    }

    #[test]
    fn test_report_type_unmarshal_empty() {
        let data = vec![0x00];
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(!report.is_downlink_data_report());
        assert!(!report.is_usage_report());
        assert!(!report.is_error_indication_report());
        assert!(!report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_unmarshal_downlink_data() {
        let data = vec![0x01];
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_downlink_data_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_unmarshal_usage_report() {
        let data = vec![0x02];
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(!report.is_downlink_data_report());
        assert!(report.is_usage_report());
    }

    #[test]
    fn test_report_type_unmarshal_error_indication() {
        let data = vec![0x04];
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_error_indication_report());
    }

    #[test]
    fn test_report_type_unmarshal_user_plane_inactivity() {
        let data = vec![0x08];
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_unmarshal_combined() {
        let data = vec![0x03]; // Bits 1 and 2
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(!report.is_error_indication_report());
    }

    #[test]
    fn test_report_type_unmarshal_all_flags() {
        let data = vec![0x0F]; // Bits 1-4
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(report.is_error_indication_report());
        assert!(report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_unmarshal_with_spare_bits() {
        // Spare bits (5-8) should be ignored
        let data = vec![0xFF]; // All bits set
        let report = ReportType::unmarshal(&data).unwrap();
        assert!(report.is_downlink_data_report());
        assert!(report.is_usage_report());
        assert!(report.is_error_indication_report());
        assert!(report.is_user_plane_inactivity_report());
    }

    #[test]
    fn test_report_type_unmarshal_empty_buffer() {
        let data = vec![];
        let result = ReportType::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_report_type_round_trip_empty() {
        let original = ReportType::new();
        let bytes = original.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_report_type_round_trip_downlink_data() {
        let original = ReportType::new().with_downlink_data_report(true);
        let bytes = original.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_report_type_round_trip_usage_report() {
        let original = ReportType::new().with_usage_report(true);
        let bytes = original.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_report_type_round_trip_combined() {
        let original = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true);
        let bytes = original.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_report_type_round_trip_all_flags() {
        let original = ReportType::new()
            .with_downlink_data_report(true)
            .with_usage_report(true)
            .with_error_indication_report(true)
            .with_user_plane_inactivity_report(true);

        let bytes = original.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_report_type_to_ie() {
        let report = ReportType::new().with_usage_report(true);
        let ie = report.to_ie();
        assert_eq!(ie.ie_type, IeType::ReportType);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 0x02);

        // Verify IE can be unmarshaled
        let parsed = ReportType::unmarshal(&ie.payload).unwrap();
        assert_eq!(report, parsed);
    }

    #[test]
    fn test_report_type_convenience_downlink_data() {
        let report = ReportType::downlink_data_report();
        assert!(report.is_downlink_data_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_convenience_usage_report() {
        let report = ReportType::usage_report();
        assert!(report.is_usage_report());
        assert!(!report.is_downlink_data_report());
    }

    #[test]
    fn test_report_type_convenience_error_indication() {
        let report = ReportType::error_indication_report();
        assert!(report.is_error_indication_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_convenience_user_plane_inactivity() {
        let report = ReportType::user_plane_inactivity_report();
        assert!(report.is_user_plane_inactivity_report());
        assert!(!report.is_usage_report());
    }

    #[test]
    fn test_report_type_clone() {
        let report1 = ReportType::new().with_usage_report(true);
        let report2 = report1;
        assert_eq!(report1, report2);
    }

    #[test]
    fn test_report_type_builder_override() {
        // Test that builder can override previous values
        let report = ReportType::new()
            .with_usage_report(false)
            .with_usage_report(true)
            .with_downlink_data_report(true)
            .with_downlink_data_report(false);

        assert!(report.is_usage_report());
        assert!(!report.is_downlink_data_report());
    }

    #[test]
    fn test_report_type_5g_scenario_usage_reporting() {
        // Scenario: UPF reports usage data to SMF
        let report = ReportType::usage_report();
        let bytes = report.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.is_usage_report());
        assert_eq!(report, parsed);
    }

    #[test]
    fn test_report_type_5g_scenario_downlink_buffering() {
        // Scenario: UPF reports buffered downlink data notification
        let report = ReportType::downlink_data_report();
        let bytes = report.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.is_downlink_data_report());
        assert_eq!(report, parsed);
    }

    #[test]
    fn test_report_type_5g_scenario_inactivity() {
        // Scenario: UPF reports user plane inactivity
        let report = ReportType::user_plane_inactivity_report();
        let bytes = report.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.is_user_plane_inactivity_report());
        assert_eq!(report, parsed);
    }

    #[test]
    fn test_report_type_5g_scenario_error() {
        // Scenario: UPF reports an error condition
        let report = ReportType::error_indication_report();
        let bytes = report.marshal();
        let parsed = ReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.is_error_indication_report());
        assert_eq!(report, parsed);
    }
}
