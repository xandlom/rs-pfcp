//! Multiplier Information Element
//!
//! The Multiplier IE is used in Usage Reporting Rules to specify a factor
//! for multiplying usage quota values.
//! Per 3GPP TS 29.244 Section 8.2.84.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Multiplier
///
/// Represents a quotient multiplier for usage quota calculation.
/// Encoded as an unsigned 32-bit integer representing a factor
/// used in usage reporting quota calculations.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.84
///
/// # Structure
/// - 4 bytes: Multiplier value (u32, big-endian)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::multiplier::Multiplier;
///
/// // Create a multiplier with value 100
/// let multiplier = Multiplier::new(100);
/// assert_eq!(multiplier.value(), 100);
///
/// // Marshal and unmarshal
/// let bytes = multiplier.marshal();
/// let parsed = Multiplier::unmarshal(&bytes)?;
/// assert_eq!(multiplier, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Multiplier {
    /// Multiplier quotient value
    value: u32,
}

impl Multiplier {
    /// Create a new Multiplier
    ///
    /// # Arguments
    /// * `value` - Multiplier quotient value (0 to u32::MAX)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::multiplier::Multiplier;
    ///
    /// let multiplier = Multiplier::new(50);
    /// assert_eq!(multiplier.value(), 50);
    /// ```
    pub fn new(value: u32) -> Self {
        Multiplier { value }
    }

    /// Get the multiplier value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::multiplier::Multiplier;
    ///
    /// let multiplier = Multiplier::new(200);
    /// assert_eq!(multiplier.value(), 200);
    /// ```
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Marshal Multiplier to bytes
    ///
    /// # Returns
    /// 4-byte vector containing multiplier value (big-endian)
    pub fn marshal(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }

    /// Unmarshal Multiplier from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing multiplier data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::multiplier::Multiplier;
    ///
    /// let multiplier = Multiplier::new(75000);
    /// let bytes = multiplier.marshal();
    /// let parsed = Multiplier::unmarshal(&bytes)?;
    /// assert_eq!(multiplier, parsed);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Multiplier",
                IeType::Multiplier,
                4,
                data.len(),
            ));
        }

        let bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let value = u32::from_be_bytes(bytes);

        Ok(Multiplier { value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::multiplier::Multiplier;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let multiplier = Multiplier::new(100);
    /// let ie = multiplier.to_ie();
    /// assert_eq!(ie.ie_type, IeType::Multiplier);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Multiplier, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier_new() {
        let multiplier = Multiplier::new(100);
        assert_eq!(multiplier.value(), 100);
    }

    #[test]
    fn test_multiplier_marshal_unmarshal() {
        let original = Multiplier::new(5000);
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 4);

        let parsed = Multiplier::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.value(), 5000);
    }

    #[test]
    fn test_multiplier_marshal_zero() {
        let multiplier = Multiplier::new(0);
        let bytes = multiplier.marshal();
        let parsed = Multiplier::unmarshal(&bytes).unwrap();

        assert_eq!(multiplier, parsed);
        assert_eq!(parsed.value(), 0);
    }

    #[test]
    fn test_multiplier_marshal_max_value() {
        let multiplier = Multiplier::new(u32::MAX);
        let bytes = multiplier.marshal();
        let parsed = Multiplier::unmarshal(&bytes).unwrap();

        assert_eq!(multiplier, parsed);
        assert_eq!(parsed.value(), u32::MAX);
    }

    #[test]
    fn test_multiplier_unmarshal_short() {
        let data = vec![0x00, 0x00, 0x00]; // Only 3 bytes
        let result = Multiplier::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_multiplier_unmarshal_empty() {
        let data = vec![];
        let result = Multiplier::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_multiplier_to_ie() {
        let multiplier = Multiplier::new(250);
        let ie = multiplier.to_ie();
        assert_eq!(ie.ie_type, IeType::Multiplier);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = Multiplier::unmarshal(&ie.payload).unwrap();
        assert_eq!(multiplier, parsed);
    }

    #[test]
    fn test_multiplier_round_trip_various() {
        let values = vec![1, 10, 100, 1000, 10000, 100000, 1000000, u32::MAX / 2];
        for value in values {
            let original = Multiplier::new(value);
            let bytes = original.marshal();
            let parsed = Multiplier::unmarshal(&bytes).unwrap();
            assert_eq!(original, parsed, "Failed for value {}", value);
        }
    }

    #[test]
    fn test_multiplier_byte_order() {
        // Verify big-endian encoding
        let multiplier = Multiplier::new(0x12345678);
        let bytes = multiplier.marshal();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_multiplier_clone() {
        let multiplier1 = Multiplier::new(5000);
        let multiplier2 = multiplier1;
        assert_eq!(multiplier1, multiplier2);
    }

    #[test]
    fn test_multiplier_5g_usage_reporting() {
        // Scenario: Apply multiplier to usage quota
        let multiplier = Multiplier::new(1000);
        let bytes = multiplier.marshal();
        let parsed = Multiplier::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.value(), 1000);
        assert_eq!(multiplier, parsed);
    }

    #[test]
    fn test_multiplier_real_world_factors() {
        // Test common multiplier values used in telecom
        let test_values = vec![
            (1, "no multiplier"),
            (10, "10x multiplier"),
            (100, "100x multiplier"),
            (1000, "1000x multiplier"),
        ];

        for (value, _desc) in test_values {
            let multiplier = Multiplier::new(value);
            let bytes = multiplier.marshal();
            let parsed = Multiplier::unmarshal(&bytes).unwrap();
            assert_eq!(parsed.value(), value);
        }
    }
}
