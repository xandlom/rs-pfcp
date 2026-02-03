//! Ethernet Filter Properties Information Element
//!
//! The Ethernet Filter Properties IE contains properties for Ethernet packet filtering.
//! Per 3GPP TS 29.244 Section 8.2.99, this IE specifies bidirectional filtering behavior.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Ethernet Filter Properties
///
/// Specifies properties for Ethernet packet filtering.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.99
///
/// # Structure
/// - Octet 5: Flags (currently only BIDE flag defined)
///
/// ```text
/// Bit 1: BIDE (Bidirectional) - Indicates bidirectional filtering
/// Bits 2-8: Spare (set to 0)
/// ```
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
///
/// // Create bidirectional filter
/// let props = EthernetFilterProperties::new(true);
/// assert!(props.is_bidirectional());
///
/// // Create unidirectional filter
/// let props2 = EthernetFilterProperties::new(false);
/// assert!(!props2.is_bidirectional());
///
/// // Marshal and unmarshal
/// let bytes = props.marshal();
/// let parsed = EthernetFilterProperties::unmarshal(&bytes).unwrap();
/// assert_eq!(props, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthernetFilterProperties {
    /// BIDE flag - Bidirectional filtering
    bide: bool,
}

impl EthernetFilterProperties {
    /// BIDE flag bit mask
    const BIDE_FLAG: u8 = 0x01;

    /// Create new Ethernet Filter Properties
    ///
    /// # Arguments
    /// * `bidirectional` - True for bidirectional filtering, false for unidirectional
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    ///
    /// let props = EthernetFilterProperties::new(true);
    /// assert!(props.is_bidirectional());
    /// ```
    pub fn new(bidirectional: bool) -> Self {
        EthernetFilterProperties {
            bide: bidirectional,
        }
    }

    /// Create bidirectional filter properties
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    ///
    /// let props = EthernetFilterProperties::bidirectional();
    /// assert!(props.is_bidirectional());
    /// ```
    pub fn bidirectional() -> Self {
        EthernetFilterProperties::new(true)
    }

    /// Create unidirectional filter properties
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    ///
    /// let props = EthernetFilterProperties::unidirectional();
    /// assert!(!props.is_bidirectional());
    /// ```
    pub fn unidirectional() -> Self {
        EthernetFilterProperties::new(false)
    }

    /// Check if bidirectional filtering is enabled
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    ///
    /// let props = EthernetFilterProperties::new(true);
    /// assert!(props.is_bidirectional());
    /// ```
    pub fn is_bidirectional(&self) -> bool {
        self.bide
    }

    /// Marshal Ethernet Filter Properties to bytes
    ///
    /// # Returns
    /// 1-byte array with flags
    pub fn marshal(&self) -> [u8; 1] {
        let mut flags = 0u8;
        if self.bide {
            flags |= Self::BIDE_FLAG;
        }
        [flags]
    }

    /// Unmarshal Ethernet Filter Properties from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing properties data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    ///
    /// let props = EthernetFilterProperties::bidirectional();
    /// let bytes = props.marshal();
    /// let parsed = EthernetFilterProperties::unmarshal(&bytes).unwrap();
    /// assert_eq!(props, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Ethernet Filter Properties",
                IeType::EthernetFilterProperties,
                1,
                0,
            ));
        }

        let bide = (data[0] & Self::BIDE_FLAG) != 0;

        Ok(EthernetFilterProperties { bide })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let props = EthernetFilterProperties::bidirectional();
    /// let ie = props.to_ie();
    /// assert_eq!(ie.ie_type, IeType::EthernetFilterProperties);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetFilterProperties, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_filter_properties_new() {
        let props = EthernetFilterProperties::new(true);
        assert!(props.is_bidirectional());

        let props2 = EthernetFilterProperties::new(false);
        assert!(!props2.is_bidirectional());
    }

    #[test]
    fn test_ethernet_filter_properties_bidirectional() {
        let props = EthernetFilterProperties::bidirectional();
        assert!(props.is_bidirectional());
    }

    #[test]
    fn test_ethernet_filter_properties_unidirectional() {
        let props = EthernetFilterProperties::unidirectional();
        assert!(!props.is_bidirectional());
    }

    #[test]
    fn test_ethernet_filter_properties_marshal() {
        let props_bidi = EthernetFilterProperties::bidirectional();
        let bytes_bidi = props_bidi.marshal();
        assert_eq!(bytes_bidi.len(), 1);
        assert_eq!(bytes_bidi[0], 0x01);

        let props_uni = EthernetFilterProperties::unidirectional();
        let bytes_uni = props_uni.marshal();
        assert_eq!(bytes_uni.len(), 1);
        assert_eq!(bytes_uni[0], 0x00);
    }

    #[test]
    fn test_ethernet_filter_properties_unmarshal_valid() {
        let data_bidi = [0x01];
        let props = EthernetFilterProperties::unmarshal(&data_bidi).unwrap();
        assert!(props.is_bidirectional());

        let data_uni = [0x00];
        let props2 = EthernetFilterProperties::unmarshal(&data_uni).unwrap();
        assert!(!props2.is_bidirectional());

        // Test with spare bits set (should be ignored)
        let data_spare = [0xFF];
        let props3 = EthernetFilterProperties::unmarshal(&data_spare).unwrap();
        assert!(props3.is_bidirectional()); // BIDE bit is set
    }

    #[test]
    fn test_ethernet_filter_properties_unmarshal_empty() {
        let result = EthernetFilterProperties::unmarshal(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::InvalidLength { .. }
        ));
    }

    #[test]
    fn test_ethernet_filter_properties_round_trip() {
        let test_cases = vec![true, false];

        for bide in test_cases {
            let original = EthernetFilterProperties::new(bide);
            let marshaled = original.marshal();
            let unmarshaled = EthernetFilterProperties::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for BIDE={}", bide);
        }
    }

    #[test]
    fn test_ethernet_filter_properties_to_ie() {
        let props = EthernetFilterProperties::bidirectional();
        let ie = props.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetFilterProperties);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 0x01);

        // Verify IE can be unmarshaled
        let parsed = EthernetFilterProperties::unmarshal(&ie.payload).unwrap();
        assert_eq!(props, parsed);
    }

    #[test]
    fn test_ethernet_filter_properties_scenarios() {
        // Scenario 1: Symmetric traffic filtering (bidirectional)
        let symmetric = EthernetFilterProperties::bidirectional();
        assert!(symmetric.is_bidirectional());

        // Scenario 2: Asymmetric traffic filtering (unidirectional)
        let asymmetric = EthernetFilterProperties::unidirectional();
        assert!(!asymmetric.is_bidirectional());

        // Scenario 3: Default behavior (unidirectional)
        let default = EthernetFilterProperties::new(false);
        assert!(!default.is_bidirectional());
    }

    #[test]
    fn test_ethernet_filter_properties_clone_copy() {
        let props1 = EthernetFilterProperties::bidirectional();
        let props2 = props1;
        assert_eq!(props1, props2);

        let props3 = props1;
        assert_eq!(props1, props3);
    }
}
