//! UR-SEQN Information Element
//!
//! The UR-SEQN IE contains the sequence number of a usage report.
//! Per 3GPP TS 29.244 Section 8.2.71.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// UR-SEQN (Usage Report Sequence Number)
///
/// Specifies the sequence number of a usage report for correlation
/// and ordering purposes during usage reporting procedures.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.71
///
/// # Structure
/// - 4 bytes: Sequence number (u32, big-endian)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ur_seqn::UrSeqn;
///
/// // Create UR-SEQN with sequence number 12345
/// let seqn = UrSeqn::new(12345);
/// assert_eq!(seqn.sequence_number(), 12345);
///
/// // Marshal and unmarshal
/// let bytes = seqn.marshal();
/// let parsed = UrSeqn::unmarshal(&bytes)?;
/// assert_eq!(seqn, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UrSeqn {
    /// Usage Report sequence number
    sequence_number: u32,
}

impl UrSeqn {
    /// Create a new UR-SEQN
    ///
    /// # Arguments
    /// * `sequence_number` - Usage report sequence number
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ur_seqn::UrSeqn;
    ///
    /// let seqn = UrSeqn::new(100);
    /// assert_eq!(seqn.sequence_number(), 100);
    /// ```
    pub fn new(sequence_number: u32) -> Self {
        UrSeqn { sequence_number }
    }

    /// Get the sequence number value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ur_seqn::UrSeqn;
    ///
    /// let seqn = UrSeqn::new(5000);
    /// assert_eq!(seqn.sequence_number(), 5000);
    /// ```
    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    /// Marshal UR-SEQN to bytes
    ///
    /// # Returns
    /// 4-byte vector containing sequence number (big-endian)
    pub fn marshal(&self) -> Vec<u8> {
        self.sequence_number.to_be_bytes().to_vec()
    }

    /// Unmarshal UR-SEQN from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing sequence number (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ur_seqn::UrSeqn;
    ///
    /// let seqn = UrSeqn::new(54321);
    /// let bytes = seqn.marshal();
    /// let parsed = UrSeqn::unmarshal(&bytes)?;
    /// assert_eq!(seqn, parsed);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "UR-SEQN",
                IeType::UrSeqn,
                4,
                data.len(),
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(UrSeqn { sequence_number })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ur_seqn::UrSeqn;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let seqn = UrSeqn::new(1000);
    /// let ie = seqn.to_ie();
    /// assert_eq!(ie.ie_type, IeType::UrSeqn);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UrSeqn, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ur_seqn_new() {
        let seqn = UrSeqn::new(12345);
        assert_eq!(seqn.sequence_number(), 12345);
    }

    #[test]
    fn test_ur_seqn_marshal_unmarshal() {
        let original = UrSeqn::new(54321);
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 4);

        let parsed = UrSeqn::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.sequence_number(), 54321);
    }

    #[test]
    fn test_ur_seqn_marshal_zero() {
        let seqn = UrSeqn::new(0);
        let bytes = seqn.marshal();
        let parsed = UrSeqn::unmarshal(&bytes).unwrap();

        assert_eq!(seqn, parsed);
        assert_eq!(parsed.sequence_number(), 0);
    }

    #[test]
    fn test_ur_seqn_marshal_max_value() {
        let seqn = UrSeqn::new(u32::MAX);
        let bytes = seqn.marshal();
        let parsed = UrSeqn::unmarshal(&bytes).unwrap();

        assert_eq!(seqn, parsed);
        assert_eq!(parsed.sequence_number(), u32::MAX);
    }

    #[test]
    fn test_ur_seqn_unmarshal_short() {
        let data = vec![0x00, 0x00, 0x00]; // Only 3 bytes
        let result = UrSeqn::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ur_seqn_unmarshal_empty() {
        let data = vec![];
        let result = UrSeqn::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ur_seqn_to_ie() {
        let seqn = UrSeqn::new(99999);
        let ie = seqn.to_ie();
        assert_eq!(ie.ie_type, IeType::UrSeqn);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = UrSeqn::unmarshal(&ie.payload).unwrap();
        assert_eq!(seqn, parsed);
    }

    #[test]
    fn test_ur_seqn_round_trip_various() {
        let values = vec![1, 100, 1000, 100000, 1000000, 0xFFFFFFFF];
        for seqn_num in values {
            let original = UrSeqn::new(seqn_num);
            let bytes = original.marshal();
            let parsed = UrSeqn::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for sequence {}", seqn_num);
        }
    }

    #[test]
    fn test_ur_seqn_byte_order() {
        // Verify big-endian encoding
        let seqn = UrSeqn::new(0x12345678);
        let bytes = seqn.marshal();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_ur_seqn_clone() {
        let seqn1 = UrSeqn::new(99999);
        let seqn2 = seqn1;
        assert_eq!(seqn1, seqn2);
    }

    #[test]
    fn test_ur_seqn_5g_usage_report_sequence() {
        // Scenario: Track usage report sequence
        let seqn = UrSeqn::new(1000);
        let bytes = seqn.marshal();
        let parsed = UrSeqn::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.sequence_number(), 1000);
        assert_eq!(seqn, parsed);
    }

    #[test]
    fn test_ur_seqn_report_correlation() {
        // Scenario: Correlate usage reports
        let seqn = UrSeqn::new(42);
        let bytes = seqn.marshal();
        let parsed = UrSeqn::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.sequence_number(), 42);
        assert_eq!(seqn, parsed);
    }
}
