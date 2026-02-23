//! Paging Policy Indicator Information Element
//!
//! The Paging Policy Indicator (PPI) IE is used to indicate the paging policy
//! associated with a QoS Flow.
//! Per 3GPP TS 29.244 Section 8.2.116.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Paging Policy Indicator
///
/// Specifies the paging policy for a QoS Flow (0-7).
/// Used in 5G to control paging behavior for specific flows.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.116
///
/// # Structure
/// - 1 byte: PPI value (0-7, 3 bits used)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::paging_policy_indicator::PagingPolicyIndicator;
///
/// // Create a PPI with value 5
/// let ppi = PagingPolicyIndicator::new(5).unwrap();
/// assert_eq!(ppi.value(), 5);
///
/// // Marshal and unmarshal
/// let bytes = ppi.marshal();
/// let parsed = PagingPolicyIndicator::unmarshal(&bytes)?;
/// assert_eq!(ppi, parsed);
/// # Ok::<(), rs_pfcp::error::PfcpError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PagingPolicyIndicator {
    /// PPI value (0-7)
    value: u8,
}

impl PagingPolicyIndicator {
    /// Maximum valid PPI value (3 bits = 0-7)
    pub const MAX: u8 = 7;

    /// Create a new Paging Policy Indicator
    ///
    /// # Arguments
    /// * `value` - PPI value (0-7)
    ///
    /// # Errors
    /// Returns `PfcpError::InvalidValue` if value exceeds 7 (3-bit limit)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::paging_policy_indicator::PagingPolicyIndicator;
    ///
    /// let ppi = PagingPolicyIndicator::new(3).unwrap();
    /// assert_eq!(ppi.value(), 3);
    /// ```
    pub fn new(value: u8) -> Result<Self, PfcpError> {
        if value > Self::MAX {
            return Err(PfcpError::invalid_value(
                "PagingPolicyIndicator.value",
                value.to_string(),
                format!("PPI value exceeds maximum {}", Self::MAX),
            ));
        }
        Ok(PagingPolicyIndicator { value })
    }

    /// Get the PPI value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::paging_policy_indicator::PagingPolicyIndicator;
    ///
    /// let ppi = PagingPolicyIndicator::new(6).unwrap();
    /// assert_eq!(ppi.value(), 6);
    /// ```
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Marshal Paging Policy Indicator to bytes
    ///
    /// # Returns
    /// 1-byte vector containing PPI value
    pub fn marshal(&self) -> Vec<u8> {
        vec![self.value & 0x07] // Mask to 3 bits
    }

    /// Unmarshal Paging Policy Indicator from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing PPI data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short or value exceeds 7
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::paging_policy_indicator::PagingPolicyIndicator;
    ///
    /// let ppi = PagingPolicyIndicator::new(4).unwrap();
    /// let bytes = ppi.marshal();
    /// let parsed = PagingPolicyIndicator::unmarshal(&bytes)?;
    /// assert_eq!(ppi, parsed);
    /// # Ok::<(), rs_pfcp::error::PfcpError>(())
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Paging Policy Indicator",
                IeType::PagingPolicyIndicator,
                1,
                0,
            ));
        }

        let value = data[0] & 0x07; // Extract 3-bit value

        Ok(PagingPolicyIndicator { value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::paging_policy_indicator::PagingPolicyIndicator;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let ppi = PagingPolicyIndicator::new(2).unwrap();
    /// let ie = ppi.to_ie();
    /// assert_eq!(ie.ie_type, IeType::PagingPolicyIndicator);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PagingPolicyIndicator, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppi_new() {
        let ppi = PagingPolicyIndicator::new(3).unwrap();
        assert_eq!(ppi.value(), 3);
    }

    #[test]
    fn test_ppi_new_invalid() {
        let result = PagingPolicyIndicator::new(8);
        assert!(matches!(result, Err(PfcpError::InvalidValue { .. })));
    }

    #[test]
    fn test_ppi_marshal_unmarshal() {
        let original = PagingPolicyIndicator::new(5).unwrap();
        let bytes = original.marshal();
        assert_eq!(bytes.len(), 1);

        let parsed = PagingPolicyIndicator::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.value(), 5);
    }

    #[test]
    fn test_ppi_marshal_all_values() {
        for i in 0..=7 {
            let ppi = PagingPolicyIndicator::new(i).unwrap();
            let bytes = ppi.marshal();
            let parsed = PagingPolicyIndicator::unmarshal(&bytes).unwrap();
            assert_eq!(ppi, parsed, "Failed for value {}", i);
        }
    }

    #[test]
    fn test_ppi_unmarshal_with_spare_bits() {
        // High bits should be ignored
        let data = vec![0xFF]; // All bits set
        let ppi = PagingPolicyIndicator::unmarshal(&data).unwrap();
        assert_eq!(ppi.value(), 7); // Only 3 bits used
    }

    #[test]
    fn test_ppi_unmarshal_empty() {
        let data = vec![];
        let result = PagingPolicyIndicator::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ppi_to_ie() {
        let ppi = PagingPolicyIndicator::new(1).unwrap();
        let ie = ppi.to_ie();
        assert_eq!(ie.ie_type, IeType::PagingPolicyIndicator);
        assert_eq!(ie.payload.len(), 1);

        // Verify IE can be unmarshaled
        let parsed = PagingPolicyIndicator::unmarshal(&ie.payload).unwrap();
        assert_eq!(ppi, parsed);
    }

    #[test]
    fn test_ppi_clone() {
        let ppi1 = PagingPolicyIndicator::new(6).unwrap();
        let ppi2 = ppi1;
        assert_eq!(ppi1, ppi2);
    }

    #[test]
    fn test_ppi_round_trip_zero() {
        let original = PagingPolicyIndicator::new(0).unwrap();
        let bytes = original.marshal();
        let parsed = PagingPolicyIndicator::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_ppi_round_trip_max() {
        let original = PagingPolicyIndicator::new(7).unwrap();
        let bytes = original.marshal();
        let parsed = PagingPolicyIndicator::unmarshal(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_ppi_5g_qos_flow() {
        // Scenario: Set paging policy for QoS flow
        let ppi = PagingPolicyIndicator::new(4).unwrap();
        let bytes = ppi.marshal();
        let parsed = PagingPolicyIndicator::unmarshal(&bytes).unwrap();

        assert_eq!(parsed.value(), 4);
        assert_eq!(ppi, parsed);
    }

    #[test]
    fn test_ppi_bit_masking() {
        // Verify 3-bit masking works correctly
        let data = vec![0b11111010]; // Upper bits set, but we only use 3 bits
        let ppi = PagingPolicyIndicator::unmarshal(&data).unwrap();
        assert_eq!(ppi.value(), 0b010); // Only lower 3 bits matter
    }
}
