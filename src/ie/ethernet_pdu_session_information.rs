//! Ethernet PDU Session Information Element
//!
//! The Ethernet PDU Session Information IE indicates whether the Ethernet PDU session
//! is carrying Ethernet frames or untagged Ethernet frames. Per 3GPP TS 29.244 Section 8.2.102,
//! this IE provides information about the Ethernet encapsulation used in the session.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Ethernet PDU Session Information
///
/// Indicates the Ethernet PDU session encapsulation type.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.102
///
/// # Structure
/// - Octet 5: Flags (currently only ETHI flag defined)
///
/// ```text
/// Bit 1: ETHI (Ethernet Header Indication)
///        0 = Ethernet header is present
///        1 = Ethernet header is not present (untagged)
/// Bits 2-8: Spare (set to 0)
/// ```
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
///
/// // Create with Ethernet header present
/// let with_header = EthernetPduSessionInformation::with_ethernet_header();
/// assert!(!with_header.is_untagged());
///
/// // Create without Ethernet header (untagged)
/// let untagged = EthernetPduSessionInformation::untagged();
/// assert!(untagged.is_untagged());
///
/// // Marshal and unmarshal
/// let bytes = with_header.marshal();
/// let parsed = EthernetPduSessionInformation::unmarshal(&bytes).unwrap();
/// assert_eq!(with_header, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthernetPduSessionInformation {
    /// ETHI flag - Ethernet Header Indication
    /// false = Ethernet header present, true = no Ethernet header (untagged)
    ethi: bool,
}

impl EthernetPduSessionInformation {
    /// ETHI flag bit mask
    const ETHI_FLAG: u8 = 0x01;

    /// Create new Ethernet PDU Session Information
    ///
    /// # Arguments
    /// * `untagged` - True if Ethernet header is not present (untagged), false otherwise
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let info = EthernetPduSessionInformation::new(false);
    /// assert!(!info.is_untagged());
    /// ```
    pub fn new(untagged: bool) -> Self {
        EthernetPduSessionInformation { ethi: untagged }
    }

    /// Create Ethernet PDU session with Ethernet header present
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let info = EthernetPduSessionInformation::with_ethernet_header();
    /// assert!(!info.is_untagged());
    /// assert!(info.has_ethernet_header());
    /// ```
    pub fn with_ethernet_header() -> Self {
        EthernetPduSessionInformation::new(false)
    }

    /// Create untagged Ethernet PDU session (no Ethernet header)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let info = EthernetPduSessionInformation::untagged();
    /// assert!(info.is_untagged());
    /// assert!(!info.has_ethernet_header());
    /// ```
    pub fn untagged() -> Self {
        EthernetPduSessionInformation::new(true)
    }

    /// Check if Ethernet header is not present (untagged)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let untagged = EthernetPduSessionInformation::untagged();
    /// assert!(untagged.is_untagged());
    /// ```
    pub fn is_untagged(&self) -> bool {
        self.ethi
    }

    /// Check if Ethernet header is present
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let with_header = EthernetPduSessionInformation::with_ethernet_header();
    /// assert!(with_header.has_ethernet_header());
    /// ```
    pub fn has_ethernet_header(&self) -> bool {
        !self.ethi
    }

    /// Marshal Ethernet PDU Session Information to bytes
    ///
    /// # Returns
    /// 1-byte array with flags
    pub fn marshal(&self) -> [u8; 1] {
        let mut flags = 0u8;
        if self.ethi {
            flags |= Self::ETHI_FLAG;
        }
        [flags]
    }

    /// Unmarshal Ethernet PDU Session Information from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing session info data (must be at least 1 byte)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    ///
    /// let info = EthernetPduSessionInformation::with_ethernet_header();
    /// let bytes = info.marshal();
    /// let parsed = EthernetPduSessionInformation::unmarshal(&bytes).unwrap();
    /// assert_eq!(info, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Ethernet PDU Session Information",
                IeType::EthernetPduSessionInformation,
                1,
                0,
            ));
        }

        let ethi = (data[0] & Self::ETHI_FLAG) != 0;

        Ok(EthernetPduSessionInformation { ethi })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_pdu_session_information::EthernetPduSessionInformation;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let info = EthernetPduSessionInformation::with_ethernet_header();
    /// let ie = info.to_ie();
    /// assert_eq!(ie.ie_type, IeType::EthernetPduSessionInformation);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::EthernetPduSessionInformation,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_pdu_session_information_new() {
        let with_header = EthernetPduSessionInformation::new(false);
        assert!(!with_header.is_untagged());
        assert!(with_header.has_ethernet_header());

        let untagged = EthernetPduSessionInformation::new(true);
        assert!(untagged.is_untagged());
        assert!(!untagged.has_ethernet_header());
    }

    #[test]
    fn test_ethernet_pdu_session_information_with_header() {
        let info = EthernetPduSessionInformation::with_ethernet_header();
        assert!(!info.is_untagged());
        assert!(info.has_ethernet_header());
    }

    #[test]
    fn test_ethernet_pdu_session_information_untagged() {
        let info = EthernetPduSessionInformation::untagged();
        assert!(info.is_untagged());
        assert!(!info.has_ethernet_header());
    }

    #[test]
    fn test_ethernet_pdu_session_information_marshal() {
        let with_header = EthernetPduSessionInformation::with_ethernet_header();
        let bytes = with_header.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x00); // No flags set

        let untagged = EthernetPduSessionInformation::untagged();
        let bytes_untagged = untagged.marshal();
        assert_eq!(bytes_untagged.len(), 1);
        assert_eq!(bytes_untagged[0], 0x01); // ETHI flag set
    }

    #[test]
    fn test_ethernet_pdu_session_information_unmarshal_with_header() {
        let data = [0x00]; // No flags set
        let info = EthernetPduSessionInformation::unmarshal(&data).unwrap();
        assert!(!info.is_untagged());
        assert!(info.has_ethernet_header());
    }

    #[test]
    fn test_ethernet_pdu_session_information_unmarshal_untagged() {
        let data = [0x01]; // ETHI flag set
        let info = EthernetPduSessionInformation::unmarshal(&data).unwrap();
        assert!(info.is_untagged());
        assert!(!info.has_ethernet_header());
    }

    #[test]
    fn test_ethernet_pdu_session_information_unmarshal_with_spare_bits() {
        // Test with spare bits set (should be ignored)
        let data = [0xFF]; // All bits set
        let info = EthernetPduSessionInformation::unmarshal(&data).unwrap();
        assert!(info.is_untagged()); // ETHI bit is set
    }

    #[test]
    fn test_ethernet_pdu_session_information_unmarshal_empty() {
        let result = EthernetPduSessionInformation::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ethernet_pdu_session_information_round_trip() {
        let test_cases = vec![true, false];

        for untagged in test_cases {
            let original = EthernetPduSessionInformation::new(untagged);
            let marshaled = original.marshal();
            let unmarshaled = EthernetPduSessionInformation::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled, "Failed for untagged={}", untagged);
        }
    }

    #[test]
    fn test_ethernet_pdu_session_information_to_ie() {
        let info = EthernetPduSessionInformation::with_ethernet_header();
        let ie = info.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetPduSessionInformation);
        assert_eq!(ie.payload.len(), 1);
        assert_eq!(ie.payload[0], 0x00);

        // Verify IE can be unmarshaled
        let parsed = EthernetPduSessionInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_ethernet_pdu_session_information_scenarios() {
        // Scenario 1: Standard Ethernet PDU session with 802.3 header
        let standard = EthernetPduSessionInformation::with_ethernet_header();
        assert!(standard.has_ethernet_header());
        assert!(!standard.is_untagged());

        // Scenario 2: Untagged Ethernet PDU session (raw payload)
        let raw = EthernetPduSessionInformation::untagged();
        assert!(!raw.has_ethernet_header());
        assert!(raw.is_untagged());

        // Scenario 3: VLAN-tagged Ethernet frames
        let vlan_tagged = EthernetPduSessionInformation::with_ethernet_header();
        assert!(vlan_tagged.has_ethernet_header());

        // Scenario 4: Layer 3 encapsulation (no Ethernet header)
        let l3_only = EthernetPduSessionInformation::untagged();
        assert!(l3_only.is_untagged());
    }

    #[test]
    fn test_ethernet_pdu_session_information_clone_copy() {
        let info1 = EthernetPduSessionInformation::with_ethernet_header();
        let info2 = info1;
        assert_eq!(info1, info2);

        let info3 = info1;
        assert_eq!(info1, info3);
    }
}
