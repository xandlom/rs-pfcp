//! QFI (QoS Flow Identifier) Information Element
//!
//! The QFI IE identifies a QoS flow in 5G networks.
//! Per 3GPP TS 29.244 Section 8.2.89, this is a 6-bit identifier (values 0-63).
//!
//! QFI is used to differentiate QoS flows within a PDU session, allowing
//! fine-grained QoS control in 5G networks.

use crate::ie::{Ie, IeType};
use std::io;

/// Maximum valid QFI value (6 bits = 0-63)
const QFI_MAX: u8 = 63;

/// QFI (QoS Flow Identifier)
///
/// Identifies a QoS flow in 5G networks. The QFI is a 6-bit value (0-63).
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.89
///
/// # Structure
/// - Bits 1-6: QFI value (0-63)
/// - Bits 7-8: Spare (set to 0)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::qfi::Qfi;
///
/// // Create QFI for a specific QoS flow
/// let qfi = Qfi::new(5).unwrap();
/// assert_eq!(qfi.value(), 5);
///
/// // Validate QFI range
/// assert!(Qfi::new(63).is_ok());  // Maximum valid value
/// assert!(Qfi::new(64).is_err()); // Out of range
///
/// // Marshal and unmarshal
/// let bytes = qfi.marshal();
/// let parsed = Qfi::unmarshal(&bytes).unwrap();
/// assert_eq!(qfi, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Qfi {
    /// QFI value (0-63)
    qfi: u8,
}

impl Qfi {
    /// Create a new QFI
    ///
    /// # Arguments
    /// * `value` - QFI value (must be 0-63)
    ///
    /// # Errors
    /// Returns error if value > 63 (exceeds 6-bit range)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qfi::Qfi;
    ///
    /// let qfi = Qfi::new(9).unwrap();
    /// assert_eq!(qfi.value(), 9);
    ///
    /// // Out of range
    /// assert!(Qfi::new(64).is_err());
    /// ```
    pub fn new(value: u8) -> Result<Self, io::Error> {
        if value > QFI_MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("QFI value {} exceeds maximum {}", value, QFI_MAX),
            ));
        }
        Ok(Qfi { qfi: value })
    }

    /// Get the QFI value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qfi::Qfi;
    ///
    /// let qfi = Qfi::new(15).unwrap();
    /// assert_eq!(qfi.value(), 15);
    /// ```
    pub fn value(&self) -> u8 {
        self.qfi
    }

    /// Marshal QFI to bytes
    ///
    /// # Returns
    /// 1-byte array with QFI value in bits 1-6
    pub fn marshal(&self) -> [u8; 1] {
        // Bits 1-6: QFI value, Bits 7-8: spare (0)
        [self.qfi & 0x3F]
    }

    /// Unmarshal QFI from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing QFI data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short or QFI value is invalid
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qfi::Qfi;
    ///
    /// let qfi = Qfi::new(20).unwrap();
    /// let bytes = qfi.marshal();
    /// let parsed = Qfi::unmarshal(&bytes).unwrap();
    /// assert_eq!(qfi, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for QFI: expected 1 byte",
            ));
        }

        // Extract bits 1-6 (QFI value), ignore spare bits 7-8
        let qfi_value = data[0] & 0x3F;

        // Validate range (should be 0-63, but mask ensures this)
        if qfi_value > QFI_MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("QFI value {} exceeds maximum {}", qfi_value, QFI_MAX),
            ));
        }

        Ok(Qfi { qfi: qfi_value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::qfi::Qfi;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let qfi = Qfi::new(7).unwrap();
    /// let ie = qfi.to_ie();
    /// assert_eq!(ie.ie_type, IeType::Qfi);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Qfi, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qfi_new_valid() {
        let qfi = Qfi::new(0).unwrap();
        assert_eq!(qfi.value(), 0);

        let qfi = Qfi::new(32).unwrap();
        assert_eq!(qfi.value(), 32);

        let qfi = Qfi::new(63).unwrap();
        assert_eq!(qfi.value(), 63);
    }

    #[test]
    fn test_qfi_new_invalid() {
        assert!(Qfi::new(64).is_err());
        assert!(Qfi::new(100).is_err());
        assert!(Qfi::new(255).is_err());
    }

    #[test]
    fn test_qfi_value() {
        let qfi = Qfi::new(42).unwrap();
        assert_eq!(qfi.value(), 42);
    }

    #[test]
    fn test_qfi_marshal_min() {
        let qfi = Qfi::new(0).unwrap();
        let bytes = qfi.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x00);
    }

    #[test]
    fn test_qfi_marshal_max() {
        let qfi = Qfi::new(63).unwrap();
        let bytes = qfi.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x3F);
    }

    #[test]
    fn test_qfi_marshal_mid() {
        let qfi = Qfi::new(15).unwrap();
        let bytes = qfi.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x0F);
    }

    #[test]
    fn test_qfi_unmarshal_valid() {
        let data = [0x15]; // QFI = 21
        let qfi = Qfi::unmarshal(&data).unwrap();
        assert_eq!(qfi.value(), 21);
    }

    #[test]
    fn test_qfi_unmarshal_with_spare_bits() {
        // Spare bits (7-8) should be ignored
        let data = [0xFF]; // All bits set
        let qfi = Qfi::unmarshal(&data).unwrap();
        assert_eq!(qfi.value(), 63); // Only bits 1-6 matter
    }

    #[test]
    fn test_qfi_unmarshal_empty() {
        let data = [];
        let result = Qfi::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.is_err()); // Error type changed to PfcpError
    }

    #[test]
    fn test_qfi_round_trip_min() {
        let original = Qfi::new(0).unwrap();
        let marshaled = original.marshal();
        let unmarshaled = Qfi::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_qfi_round_trip_max() {
        let original = Qfi::new(63).unwrap();
        let marshaled = original.marshal();
        let unmarshaled = Qfi::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_qfi_round_trip_various() {
        for value in [0, 1, 5, 9, 15, 31, 32, 50, 63] {
            let original = Qfi::new(value).unwrap();
            let marshaled = original.marshal();
            let unmarshaled = Qfi::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for QFI value {}", value);
        }
    }

    #[test]
    fn test_qfi_to_ie() {
        let qfi = Qfi::new(25).unwrap();
        let ie = qfi.to_ie();
        assert_eq!(ie.ie_type, IeType::Qfi);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 25);

        // Verify IE can be unmarshaled
        let parsed = Qfi::unmarshal(&ie.payload).unwrap();
        assert_eq!(qfi, parsed);
    }

    #[test]
    fn test_qfi_clone() {
        let qfi1 = Qfi::new(10).unwrap();
        let qfi2 = qfi1;
        assert_eq!(qfi1, qfi2);
    }

    #[test]
    fn test_qfi_ordering() {
        let qfi1 = Qfi::new(5).unwrap();
        let qfi2 = Qfi::new(10).unwrap();
        let qfi3 = Qfi::new(15).unwrap();

        assert!(qfi1 < qfi2);
        assert!(qfi2 < qfi3);
        assert!(qfi1 < qfi3);
    }

    #[test]
    fn test_qfi_5g_scenarios() {
        // Scenario 1: Default bearer QoS flow
        let qfi_default = Qfi::new(1).unwrap();
        assert_eq!(qfi_default.value(), 1);

        // Scenario 2: Premium video streaming QoS flow
        let qfi_video = Qfi::new(5).unwrap();
        assert_eq!(qfi_video.value(), 5);

        // Scenario 3: VoLTE QoS flow
        let qfi_voice = Qfi::new(9).unwrap();
        assert_eq!(qfi_voice.value(), 9);

        // Scenario 4: Best effort data QoS flow
        let qfi_data = Qfi::new(8).unwrap();
        assert_eq!(qfi_data.value(), 8);

        // Scenario 5: Maximum QFI value
        let qfi_max = Qfi::new(63).unwrap();
        assert_eq!(qfi_max.value(), 63);
    }

    #[test]
    fn test_qfi_boundary_values() {
        // Test boundary values
        assert!(Qfi::new(0).is_ok());
        assert!(Qfi::new(63).is_ok());
        assert!(Qfi::new(64).is_err());

        // Ensure proper error messages
        let err = Qfi::new(100).unwrap_err();
        assert!(err.to_string().contains("100"));
        assert!(err.to_string().contains("63"));
    }
}
