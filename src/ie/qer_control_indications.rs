//! QER Control Indications Information Element
//!
//! The QER Control Indications IE contains control flags for QoS Enforcement Rules.
//! Per 3GPP TS 29.244 Section 8.2.174.

use crate::ie::{Ie, IeType};
use std::io;

/// QER Control Indications
///
/// Contains one-byte bitflags for QER control indications.
/// Used in PFCP Session modification/deletion to indicate QER-specific control operations.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.174
///
/// # Structure
/// - 1 byte: Control flags
///   - Bit 1 (GCSIR): GCS IP Range handling
///   - Bit 2 (Reserved): Reserved for future use
///   - Bit 3 (Reserved): Reserved for future use
///   - Bit 4 (Reserved): Reserved for future use
///   - Bit 5 (Reserved): Reserved for future use
///   - Bit 6 (Reserved): Reserved for future use
///   - Bit 7 (Reserved): Reserved for future use
///   - Bit 8 (Reserved): Reserved for future use
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::qer_control_indications::QerControlIndications;
///
/// // Create QER Control Indications with GCSIR flag set
/// let qci = QerControlIndications::new(0x01);
/// assert_eq!(qci.flags(), 0x01);
///
/// // Marshal and unmarshal
/// let bytes = qci.marshal()?;
/// let parsed = QerControlIndications::unmarshal(&bytes)?;
/// assert_eq!(qci, parsed);
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QerControlIndications {
    /// Control flags (1 byte)
    flags: u8,
}

impl QerControlIndications {
    /// GCSIR flag - GCS IP Range handling (bit 1)
    pub const GCSIR: u8 = 0x01;

    /// Create a new QER Control Indications
    ///
    /// # Arguments
    /// * `flags` - Control flags (1 byte)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qer_control_indications::QerControlIndications;
    ///
    /// let qci = QerControlIndications::new(0x01);
    /// assert_eq!(qci.flags(), 0x01);
    /// ```
    pub fn new(flags: u8) -> Self {
        QerControlIndications { flags }
    }

    /// Get the control flags
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qer_control_indications::QerControlIndications;
    ///
    /// let qci = QerControlIndications::new(0x03);
    /// assert_eq!(qci.flags(), 0x03);
    /// ```
    pub fn flags(&self) -> u8 {
        self.flags
    }

    /// Check if GCSIR flag is set
    pub fn gcsir(&self) -> bool {
        self.flags & Self::GCSIR != 0
    }

    /// Set GCSIR flag
    pub fn set_gcsir(&mut self) {
        self.flags |= Self::GCSIR;
    }

    /// Clear GCSIR flag
    pub fn clear_gcsir(&mut self) {
        self.flags &= !Self::GCSIR;
    }

    /// Marshal QER Control Indications to bytes
    ///
    /// # Returns
    /// 1-byte vector containing control flags
    ///
    /// # Errors
    /// Returns error if serialization fails
    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        Ok(vec![self.flags])
    }

    /// Unmarshal QER Control Indications from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing control flags (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qer_control_indications::QerControlIndications;
    ///
    /// let qci = QerControlIndications::new(0x01);
    /// let bytes = qci.marshal()?;
    /// let parsed = QerControlIndications::unmarshal(&bytes)?;
    /// assert_eq!(qci, parsed);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "QER Control Indications requires 1 byte",
            ));
        }

        Ok(QerControlIndications { flags: data[0] })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qer_control_indications::QerControlIndications;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let qci = QerControlIndications::new(0x01);
    /// let ie = qci.to_ie()?;
    /// assert_eq!(ie.ie_type, IeType::QerControlIndications);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::QerControlIndications, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qer_control_indications_new() {
        let qci = QerControlIndications::new(0x01);
        assert_eq!(qci.flags(), 0x01);
    }

    #[test]
    fn test_qer_control_indications_gcsir_flag() {
        let qci = QerControlIndications::new(0x01);
        assert!(qci.gcsir());
    }

    #[test]
    fn test_qer_control_indications_gcsir_flag_not_set() {
        let qci = QerControlIndications::new(0x00);
        assert!(!qci.gcsir());
    }

    #[test]
    fn test_qer_control_indications_set_gcsir() {
        let mut qci = QerControlIndications::new(0x00);
        assert!(!qci.gcsir());
        qci.set_gcsir();
        assert!(qci.gcsir());
        assert_eq!(qci.flags(), 0x01);
    }

    #[test]
    fn test_qer_control_indications_clear_gcsir() {
        let mut qci = QerControlIndications::new(0x01);
        assert!(qci.gcsir());
        qci.clear_gcsir();
        assert!(!qci.gcsir());
        assert_eq!(qci.flags(), 0x00);
    }

    #[test]
    fn test_qer_control_indications_marshal_unmarshal() {
        let original = QerControlIndications::new(0x01);
        let bytes = original.marshal().unwrap();
        assert_eq!(bytes.len(), 1);

        let parsed = QerControlIndications::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.flags(), 0x01);
    }

    #[test]
    fn test_qer_control_indications_marshal_zero() {
        let qci = QerControlIndications::new(0x00);
        let bytes = qci.marshal().unwrap();
        let parsed = QerControlIndications::unmarshal(&bytes).unwrap();

        assert_eq!(qci, parsed);
        assert_eq!(parsed.flags(), 0x00);
    }

    #[test]
    fn test_qer_control_indications_marshal_all_flags() {
        let qci = QerControlIndications::new(0xFF);
        let bytes = qci.marshal().unwrap();
        let parsed = QerControlIndications::unmarshal(&bytes).unwrap();

        assert_eq!(qci, parsed);
        assert_eq!(parsed.flags(), 0xFF);
    }

    #[test]
    fn test_qer_control_indications_unmarshal_empty() {
        let data = vec![];
        let result = QerControlIndications::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_qer_control_indications_to_ie() {
        let qci = QerControlIndications::new(0x01);
        let ie = qci.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::QerControlIndications);
        assert_eq!(ie.payload.len(), 1);

        // Verify IE can be unmarshaled
        let parsed = QerControlIndications::unmarshal(&ie.payload).unwrap();
        assert_eq!(qci, parsed);
    }

    #[test]
    fn test_qer_control_indications_round_trip_various() {
        let values = vec![0x00, 0x01, 0x03, 0x55, 0xAA, 0xFF];
        for flags_val in values {
            let original = QerControlIndications::new(flags_val);
            let bytes = original.marshal().unwrap();
            let parsed = QerControlIndications::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for flags 0x{:02x}", flags_val);
        }
    }

    #[test]
    fn test_qer_control_indications_5g_qer_enforcement() {
        // Scenario: QER enforcement with GCSIR
        let qci = QerControlIndications::new(QerControlIndications::GCSIR);
        let bytes = qci.marshal().unwrap();
        let parsed = QerControlIndications::unmarshal(&bytes).unwrap();

        assert!(parsed.gcsir());
        assert_eq!(qci, parsed);
    }
}
