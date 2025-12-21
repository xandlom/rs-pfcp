//! Ethernet Filter ID Information Element
//!
//! The Ethernet Filter ID IE uniquely identifies an Ethernet packet filter.
//! Per 3GPP TS 29.244 Section 8.2.98, this IE is used to reference specific Ethernet filters.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Ethernet Filter ID
///
/// Uniquely identifies an Ethernet packet filter within a PDR.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.98
///
/// # Structure
/// - 4 octets: Filter ID value in network byte order
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
///
/// // Create Ethernet Filter ID
/// let filter_id = EthernetFilterId::new(1);
/// assert_eq!(filter_id.value(), 1);
///
/// // Marshal and unmarshal
/// let bytes = filter_id.marshal();
/// let parsed = EthernetFilterId::unmarshal(&bytes).unwrap();
/// assert_eq!(filter_id, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EthernetFilterId {
    /// Filter ID value (32-bit)
    value: u32,
}

impl EthernetFilterId {
    /// Create a new Ethernet Filter ID
    ///
    /// # Arguments
    /// * `value` - Filter ID value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    ///
    /// let filter_id = EthernetFilterId::new(42);
    /// assert_eq!(filter_id.value(), 42);
    /// ```
    pub fn new(value: u32) -> Self {
        EthernetFilterId { value }
    }

    /// Get the filter ID value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    ///
    /// let filter_id = EthernetFilterId::new(100);
    /// assert_eq!(filter_id.value(), 100);
    /// ```
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Marshal Ethernet Filter ID to bytes
    ///
    /// # Returns
    /// 4-byte array with filter ID in network byte order
    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    /// Unmarshal Ethernet Filter ID from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing filter ID data (must be at least 4 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    ///
    /// let filter_id = EthernetFilterId::new(999);
    /// let bytes = filter_id.marshal();
    /// let parsed = EthernetFilterId::unmarshal(&bytes).unwrap();
    /// assert_eq!(filter_id, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Ethernet Filter ID",
                IeType::EthernetFilterId,
                4,
                data.len(),
            ));
        }

        let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok(EthernetFilterId { value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let filter_id = EthernetFilterId::new(5);
    /// let ie = filter_id.to_ie();
    /// assert_eq!(ie.ie_type, IeType::EthernetFilterId);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetFilterId, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_filter_id_new() {
        let filter_id = EthernetFilterId::new(1);
        assert_eq!(filter_id.value(), 1);

        let filter_id2 = EthernetFilterId::new(0);
        assert_eq!(filter_id2.value(), 0);

        let filter_id3 = EthernetFilterId::new(u32::MAX);
        assert_eq!(filter_id3.value(), u32::MAX);
    }

    #[test]
    fn test_ethernet_filter_id_marshal() {
        let filter_id = EthernetFilterId::new(1);
        let bytes = filter_id.marshal();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x01]);

        let filter_id2 = EthernetFilterId::new(0x12345678);
        let bytes2 = filter_id2.marshal();
        assert_eq!(bytes2, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_ethernet_filter_id_unmarshal_valid() {
        let data = [0x00, 0x00, 0x00, 0x01];
        let filter_id = EthernetFilterId::unmarshal(&data).unwrap();
        assert_eq!(filter_id.value(), 1);

        let data2 = [0x12, 0x34, 0x56, 0x78];
        let filter_id2 = EthernetFilterId::unmarshal(&data2).unwrap();
        assert_eq!(filter_id2.value(), 0x12345678);
    }

    #[test]
    fn test_ethernet_filter_id_unmarshal_short() {
        let data = [0x00, 0x00, 0x01];
        let result = EthernetFilterId::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ethernet_filter_id_unmarshal_empty() {
        let result = EthernetFilterId::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        assert!(err.to_string().contains("Ethernet Filter ID"));
        assert!(err.to_string().contains("4"));
        assert!(err.to_string().contains("0"));
    }

    #[test]
    fn test_ethernet_filter_id_round_trip() {
        let test_cases = vec![0, 1, 100, 1000, 65535, 0x12345678, u32::MAX];

        for value in test_cases {
            let original = EthernetFilterId::new(value);
            let marshaled = original.marshal();
            let unmarshaled = EthernetFilterId::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for filter ID {}", value);
        }
    }

    #[test]
    fn test_ethernet_filter_id_to_ie() {
        let filter_id = EthernetFilterId::new(42);
        let ie = filter_id.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetFilterId);
        assert_eq!(ie.payload.len(), 4);

        // Verify IE can be unmarshaled
        let parsed = EthernetFilterId::unmarshal(&ie.payload).unwrap();
        assert_eq!(filter_id, parsed);
    }

    #[test]
    fn test_ethernet_filter_id_scenarios() {
        // Scenario 1: First filter in PDR
        let filter1 = EthernetFilterId::new(1);
        assert_eq!(filter1.value(), 1);

        // Scenario 2: Multiple filters in sequence
        let filter2 = EthernetFilterId::new(2);
        let filter3 = EthernetFilterId::new(3);
        assert!(filter1 < filter2);
        assert!(filter2 < filter3);

        // Scenario 3: High-numbered filter
        let high_filter = EthernetFilterId::new(10000);
        assert_eq!(high_filter.value(), 10000);

        // Scenario 4: Maximum filter ID
        let max_filter = EthernetFilterId::new(u32::MAX);
        assert_eq!(max_filter.value(), u32::MAX);
    }

    #[test]
    fn test_ethernet_filter_id_ordering() {
        let id1 = EthernetFilterId::new(1);
        let id2 = EthernetFilterId::new(2);
        let id3 = EthernetFilterId::new(100);

        assert!(id1 < id2);
        assert!(id2 < id3);
        assert!(id1 < id3);
        assert_eq!(id1, EthernetFilterId::new(1));
    }

    #[test]
    fn test_ethernet_filter_id_clone_copy() {
        let id1 = EthernetFilterId::new(42);
        let id2 = id1;
        assert_eq!(id1, id2);

        let id3 = id1;
        assert_eq!(id1, id3);
    }
}
