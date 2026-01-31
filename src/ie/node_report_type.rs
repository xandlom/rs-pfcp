//! Node Report Type Information Element
//!
//! The Node Report Type IE indicates the type of node report in PFCP Node Report messages.
//! Per 3GPP TS 29.244 Section 8.2.69.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Node Report Type
///
/// Contains one-byte bitflags for node report type control.
/// Used to indicate which types of reports are being sent in PFCP Node Report Request messages.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.69
///
/// # Structure
/// - 1 byte: Report type flags
///   - Bit 1 (UPFR): User Plane Path Failure Report
///   - Bit 2 (UPRR): User Plane Path Recovery Report
///   - Bit 3 (CKDR): Clock Drift Report
///   - Bit 4 (GPQR): GTP-U Path QoS Report
///   - Bit 5 (PURR): Peer GTP-U Entity Restart Report
///   - Bit 6 (VSR): Vendor-Specific Report
///   - Bits 7-8: Spare (zeros)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::node_report_type::NodeReportType;
///
/// // Create Node Report Type with UPFR flag set
/// let nrt = NodeReportType::new(0x01);
/// assert_eq!(nrt.flags(), 0x01);
/// assert!(nrt.upfr());
///
/// // Marshal and unmarshal
/// let bytes = nrt.marshal();
/// let parsed = NodeReportType::unmarshal(&bytes)?;
/// assert_eq!(nrt, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeReportType {
    /// Report type flags (1 byte)
    flags: u8,
}

impl NodeReportType {
    /// UPFR flag - User Plane Path Failure Report (bit 1)
    pub const UPFR: u8 = 0x01;
    /// UPRR flag - User Plane Path Recovery Report (bit 2)
    pub const UPRR: u8 = 0x02;
    /// CKDR flag - Clock Drift Report (bit 3)
    pub const CKDR: u8 = 0x04;
    /// GPQR flag - GTP-U Path QoS Report (bit 4)
    pub const GPQR: u8 = 0x08;
    /// PURR flag - Peer GTP-U Entity Restart Report (bit 5)
    pub const PURR: u8 = 0x10;
    /// VSR flag - Vendor-Specific Report (bit 6)
    pub const VSR: u8 = 0x20;

    /// Create a new Node Report Type
    ///
    /// # Arguments
    /// * `flags` - Report type flags (1 byte, at least one bit must be set per spec)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::node_report_type::NodeReportType;
    ///
    /// let nrt = NodeReportType::new(0x01);
    /// assert_eq!(nrt.flags(), 0x01);
    /// ```
    pub fn new(flags: u8) -> Self {
        NodeReportType { flags }
    }

    /// Get the report type flags
    pub fn flags(&self) -> u8 {
        self.flags
    }

    /// Check if UPFR flag is set (User Plane Path Failure Report)
    pub fn upfr(&self) -> bool {
        self.flags & Self::UPFR != 0
    }

    /// Check if UPRR flag is set (User Plane Path Recovery Report)
    pub fn uprr(&self) -> bool {
        self.flags & Self::UPRR != 0
    }

    /// Check if CKDR flag is set (Clock Drift Report)
    pub fn ckdr(&self) -> bool {
        self.flags & Self::CKDR != 0
    }

    /// Check if GPQR flag is set (GTP-U Path QoS Report)
    pub fn gpqr(&self) -> bool {
        self.flags & Self::GPQR != 0
    }

    /// Check if PURR flag is set (Peer GTP-U Entity Restart Report)
    pub fn purr(&self) -> bool {
        self.flags & Self::PURR != 0
    }

    /// Check if VSR flag is set (Vendor-Specific Report)
    pub fn vsr(&self) -> bool {
        self.flags & Self::VSR != 0
    }

    /// Set UPFR flag
    pub fn set_upfr(&mut self) {
        self.flags |= Self::UPFR;
    }

    /// Clear UPFR flag
    pub fn clear_upfr(&mut self) {
        self.flags &= !Self::UPFR;
    }

    /// Set UPRR flag
    pub fn set_uprr(&mut self) {
        self.flags |= Self::UPRR;
    }

    /// Clear UPRR flag
    pub fn clear_uprr(&mut self) {
        self.flags &= !Self::UPRR;
    }

    /// Set CKDR flag
    pub fn set_ckdr(&mut self) {
        self.flags |= Self::CKDR;
    }

    /// Clear CKDR flag
    pub fn clear_ckdr(&mut self) {
        self.flags &= !Self::CKDR;
    }

    /// Set GPQR flag
    pub fn set_gpqr(&mut self) {
        self.flags |= Self::GPQR;
    }

    /// Clear GPQR flag
    pub fn clear_gpqr(&mut self) {
        self.flags &= !Self::GPQR;
    }

    /// Set PURR flag
    pub fn set_purr(&mut self) {
        self.flags |= Self::PURR;
    }

    /// Clear PURR flag
    pub fn clear_purr(&mut self) {
        self.flags &= !Self::PURR;
    }

    /// Set VSR flag
    pub fn set_vsr(&mut self) {
        self.flags |= Self::VSR;
    }

    /// Clear VSR flag
    pub fn clear_vsr(&mut self) {
        self.flags &= !Self::VSR;
    }

    /// Marshal Node Report Type to bytes
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.flags]
    }

    /// Unmarshal Node Report Type from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Node Report Type",
                IeType::NodeReportType,
                1,
                0,
            ));
        }

        Ok(NodeReportType { flags: data[0] })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NodeReportType, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_report_type_new() {
        let nrt = NodeReportType::new(0x01);
        assert_eq!(nrt.flags(), 0x01);
    }

    #[test]
    fn test_node_report_type_upfr() {
        let nrt = NodeReportType::new(NodeReportType::UPFR);
        assert!(nrt.upfr());
        assert!(!nrt.uprr());
    }

    #[test]
    fn test_node_report_type_uprr() {
        let nrt = NodeReportType::new(NodeReportType::UPRR);
        assert!(!nrt.upfr());
        assert!(nrt.uprr());
    }

    #[test]
    fn test_node_report_type_ckdr() {
        let nrt = NodeReportType::new(NodeReportType::CKDR);
        assert!(nrt.ckdr());
        assert!(!nrt.upfr());
    }

    #[test]
    fn test_node_report_type_gpqr() {
        let nrt = NodeReportType::new(NodeReportType::GPQR);
        assert!(nrt.gpqr());
    }

    #[test]
    fn test_node_report_type_purr() {
        let nrt = NodeReportType::new(NodeReportType::PURR);
        assert!(nrt.purr());
    }

    #[test]
    fn test_node_report_type_vsr() {
        let nrt = NodeReportType::new(NodeReportType::VSR);
        assert!(nrt.vsr());
    }

    #[test]
    fn test_node_report_type_all_flags() {
        let nrt = NodeReportType::new(0x3F); // All 6 flags
        assert!(nrt.upfr());
        assert!(nrt.uprr());
        assert!(nrt.ckdr());
        assert!(nrt.gpqr());
        assert!(nrt.purr());
        assert!(nrt.vsr());
    }

    #[test]
    fn test_node_report_type_set_flags() {
        let mut nrt = NodeReportType::new(0x00);
        assert!(!nrt.upfr());

        nrt.set_upfr();
        assert!(nrt.upfr());
        assert_eq!(nrt.flags(), 0x01);

        nrt.set_uprr();
        assert!(nrt.upfr());
        assert!(nrt.uprr());
        assert_eq!(nrt.flags(), 0x03);

        nrt.set_ckdr();
        assert_eq!(nrt.flags(), 0x07);
    }

    #[test]
    fn test_node_report_type_clear_flags() {
        let mut nrt = NodeReportType::new(0x3F);
        assert!(nrt.upfr());

        nrt.clear_upfr();
        assert!(!nrt.upfr());
        assert_eq!(nrt.flags(), 0x3E);

        nrt.clear_uprr();
        assert!(!nrt.uprr());
        assert_eq!(nrt.flags(), 0x3C);

        nrt.clear_vsr();
        assert!(!nrt.vsr());
    }

    #[test]
    fn test_node_report_type_marshal_unmarshal() {
        let original = NodeReportType::new(0x01);
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 1);

        let parsed = NodeReportType::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.flags(), 0x01);
    }

    #[test]
    fn test_node_report_type_marshal_all_flags() {
        let nrt = NodeReportType::new(0x3F);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert_eq!(nrt, parsed);
        assert_eq!(parsed.flags(), 0x3F);
    }

    #[test]
    fn test_node_report_type_marshal_zero() {
        let nrt = NodeReportType::new(0x00);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert_eq!(nrt, parsed);
        assert_eq!(parsed.flags(), 0x00);
    }

    #[test]
    fn test_node_report_type_unmarshal_empty() {
        let data = vec![];
        let result = NodeReportType::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_node_report_type_to_ie() {
        let nrt = NodeReportType::new(0x3F);
        let ie = nrt.to_ie();
        assert_eq!(ie.ie_type, IeType::NodeReportType);
        assert_eq!(ie.payload.len(), 1);

        let parsed = NodeReportType::unmarshal(&ie.payload).unwrap();
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_round_trip_various() {
        let values = vec![0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x3F, 0x15, 0x2A];
        for flags_val in values {
            let original = NodeReportType::new(flags_val);
            let bytes = original.marshal();
            let parsed = NodeReportType::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for flags 0x{:02x}", flags_val);
        }
    }

    #[test]
    fn test_node_report_type_5g_path_failure() {
        // Scenario: Report user plane path failure
        let nrt = NodeReportType::new(NodeReportType::UPFR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.upfr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_path_recovery() {
        // Scenario: Report user plane path recovery
        let nrt = NodeReportType::new(NodeReportType::UPRR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.uprr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_clock_drift() {
        // Scenario: Report clock drift
        let nrt = NodeReportType::new(NodeReportType::CKDR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.ckdr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_qos_report() {
        // Scenario: Report GTP-U path QoS status
        let nrt = NodeReportType::new(NodeReportType::GPQR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.gpqr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_restart_report() {
        // Scenario: Report peer GTP-U restart
        let nrt = NodeReportType::new(NodeReportType::PURR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.purr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_vendor_report() {
        // Scenario: Vendor-specific report
        let nrt = NodeReportType::new(NodeReportType::VSR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.vsr());
        assert_eq!(nrt, parsed);
    }

    #[test]
    fn test_node_report_type_5g_multiple_reports() {
        // Scenario: Multiple report types in one node report
        let nrt =
            NodeReportType::new(NodeReportType::UPFR | NodeReportType::UPRR | NodeReportType::CKDR);
        let bytes = nrt.marshal();
        let parsed = NodeReportType::unmarshal(&bytes).unwrap();

        assert!(parsed.upfr());
        assert!(parsed.uprr());
        assert!(parsed.ckdr());
        assert!(!parsed.gpqr());
        assert_eq!(nrt, parsed);
    }
}
