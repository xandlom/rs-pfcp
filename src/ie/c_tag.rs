//! C-TAG (Customer VLAN Tag) Information Element
//!
//! The C-TAG IE contains customer VLAN tagging information for Ethernet packet filtering.
//! Per 3GPP TS 29.244 Section 8.2.94, this IE is used in Ethernet packet filtering scenarios.
//!
//! C-TAG consists of priority, DEI (Drop Eligible Indicator), and VID (VLAN ID).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// C-TAG (Customer VLAN Tag)
///
/// Represents customer VLAN tagging information per IEEE 802.1Q.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.94
///
/// # Structure
/// - Octet 5: Flags (PCP Priority, DEI)
/// - Octet 5-6: VID (VLAN ID) - 12 bits
///
/// ```text
/// Bits 8-6: PCP (Priority Code Point) - 3 bits
/// Bit 5: DEI (Drop Eligible Indicator) - 1 bit
/// Bits 4-1 (and next 8 bits): VID (VLAN ID) - 12 bits
/// ```
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::c_tag::CTag;
///
/// // Create C-TAG with priority 3, no DEI, VLAN ID 100
/// let ctag = CTag::new(3, false, 100).unwrap();
/// assert_eq!(ctag.priority(), 3);
/// assert_eq!(ctag.dei(), false);
/// assert_eq!(ctag.vid(), 100);
///
/// // Marshal and unmarshal
/// let bytes = ctag.marshal();
/// let parsed = CTag::unmarshal(&bytes).unwrap();
/// assert_eq!(ctag, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CTag {
    /// Priority Code Point (0-7, 3 bits)
    pcp: u8,
    /// Drop Eligible Indicator (boolean)
    dei: bool,
    /// VLAN ID (0-4095, 12 bits)
    vid: u16,
}

impl CTag {
    /// Maximum valid priority (3 bits = 0-7)
    pub const MAX_PRIORITY: u8 = 7;
    /// Maximum valid VID (12 bits = 0-4095)
    pub const MAX_VID: u16 = 4095;

    /// Create a new C-TAG
    ///
    /// # Arguments
    /// * `pcp` - Priority Code Point (0-7)
    /// * `dei` - Drop Eligible Indicator
    /// * `vid` - VLAN ID (0-4095)
    ///
    /// # Errors
    /// Returns error if PCP > 7 or VID > 4095
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::c_tag::CTag;
    ///
    /// let ctag = CTag::new(5, true, 1000).unwrap();
    /// assert_eq!(ctag.priority(), 5);
    /// assert_eq!(ctag.dei(), true);
    /// assert_eq!(ctag.vid(), 1000);
    /// ```
    pub fn new(pcp: u8, dei: bool, vid: u16) -> Result<Self, PfcpError> {
        if pcp > Self::MAX_PRIORITY {
            return Err(PfcpError::invalid_value(
                "C-TAG PCP",
                pcp.to_string(),
                format!("exceeds maximum {}", Self::MAX_PRIORITY),
            ));
        }
        if vid > Self::MAX_VID {
            return Err(PfcpError::invalid_value(
                "C-TAG VID",
                vid.to_string(),
                format!("exceeds maximum {}", Self::MAX_VID),
            ));
        }
        Ok(CTag { pcp, dei, vid })
    }

    /// Get the Priority Code Point
    pub fn priority(&self) -> u8 {
        self.pcp
    }

    /// Get the Drop Eligible Indicator
    pub fn dei(&self) -> bool {
        self.dei
    }

    /// Get the VLAN ID
    pub fn vid(&self) -> u16 {
        self.vid
    }

    /// Marshal C-TAG to bytes
    ///
    /// # Returns
    /// 3-byte array with C-TAG encoded as:
    /// - Byte 0: PCP (bits 7-5) | DEI (bit 4) | VID high nibble (bits 3-0)
    /// - Bytes 1-2: VID low byte
    pub fn marshal(&self) -> [u8; 3] {
        let mut bytes = [0u8; 3];

        // Byte 0: PCP (3 bits) | DEI (1 bit) | VID[11:8] (4 bits)
        bytes[0] = (self.pcp << 5) | ((self.dei as u8) << 4) | ((self.vid >> 8) as u8 & 0x0F);

        // Byte 1: VID[7:0]
        bytes[1] = (self.vid & 0xFF) as u8;

        // Byte 2: Spare (0)
        bytes[2] = 0;

        bytes
    }

    /// Unmarshal C-TAG from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing C-TAG data (must be at least 3 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::c_tag::CTag;
    ///
    /// let ctag = CTag::new(7, false, 4095).unwrap();
    /// let bytes = ctag.marshal();
    /// let parsed = CTag::unmarshal(&bytes).unwrap();
    /// assert_eq!(ctag, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "C-TAG",
                IeType::CTag,
                3,
                data.len(),
            ));
        }

        // Extract PCP (bits 7-5 of byte 0)
        let pcp = (data[0] >> 5) & 0x07;

        // Extract DEI (bit 4 of byte 0)
        let dei = (data[0] & 0x10) != 0;

        // Extract VID (bits 3-0 of byte 0 + byte 1)
        let vid = (((data[0] & 0x0F) as u16) << 8) | (data[1] as u16);

        Ok(CTag { pcp, dei, vid })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::c_tag::CTag;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let ctag = CTag::new(2, false, 500).unwrap();
    /// let ie = ctag.to_ie();
    /// assert_eq!(ie.ie_type, IeType::CTag);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::CTag, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctag_new_valid() {
        let ctag = CTag::new(0, false, 0).unwrap();
        assert_eq!(ctag.priority(), 0);
        assert!(!ctag.dei());
        assert_eq!(ctag.vid(), 0);

        let ctag2 = CTag::new(7, true, 4095).unwrap();
        assert_eq!(ctag2.priority(), 7);
        assert!(ctag2.dei());
        assert_eq!(ctag2.vid(), 4095);
    }

    #[test]
    fn test_ctag_new_invalid_pcp() {
        let result = CTag::new(8, false, 100);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_ctag_new_invalid_vid() {
        let result = CTag::new(3, false, 4096);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_ctag_marshal() {
        let ctag = CTag::new(5, true, 1000).unwrap();
        let bytes = ctag.marshal();
        assert_eq!(bytes.len(), 3);

        // Verify PCP (bits 7-5) = 5 (101)
        // Verify DEI (bit 4) = 1
        // Verify VID high nibble (bits 3-0) = 3 (from 1000 = 0x3E8)
        // Expected byte 0: 101 1 0011 = 0xB3
        assert_eq!(bytes[0], 0xB3);

        // Verify VID low byte = 0xE8
        assert_eq!(bytes[1], 0xE8);

        // Spare byte
        assert_eq!(bytes[2], 0x00);
    }

    #[test]
    fn test_ctag_unmarshal_valid() {
        let data = [0xB3, 0xE8, 0x00]; // PCP=5, DEI=1, VID=1000
        let ctag = CTag::unmarshal(&data).unwrap();
        assert_eq!(ctag.priority(), 5);
        assert!(ctag.dei());
        assert_eq!(ctag.vid(), 1000);
    }

    #[test]
    fn test_ctag_unmarshal_short() {
        let data = [0xB3, 0xE8];
        let result = CTag::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_ctag_unmarshal_empty() {
        let result = CTag::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
    }

    #[test]
    fn test_ctag_round_trip() {
        let test_cases = vec![
            (0, false, 0),
            (7, true, 4095),
            (3, false, 100),
            (5, true, 1000),
            (1, false, 1),
        ];

        for (pcp, dei, vid) in test_cases {
            let original = CTag::new(pcp, dei, vid).unwrap();
            let marshaled = original.marshal();
            let unmarshaled = CTag::unmarshal(&marshaled).unwrap();
            assert_eq!(
                original, unmarshaled,
                "Failed for PCP={}, DEI={}, VID={}",
                pcp, dei, vid
            );
        }
    }

    #[test]
    fn test_ctag_to_ie() {
        let ctag = CTag::new(4, false, 200).unwrap();
        let ie = ctag.to_ie();
        assert_eq!(ie.ie_type, IeType::CTag);
        assert_eq!(ie.payload.len(), 3);

        // Verify IE can be unmarshaled
        let parsed = CTag::unmarshal(&ie.payload).unwrap();
        assert_eq!(ctag, parsed);
    }

    #[test]
    fn test_ctag_scenarios() {
        // Scenario 1: Default VLAN (VID 1, no priority)
        let default_vlan = CTag::new(0, false, 1).unwrap();
        assert_eq!(default_vlan.vid(), 1);
        assert_eq!(default_vlan.priority(), 0);

        // Scenario 2: High priority voice traffic (PCP 6)
        let voice = CTag::new(6, false, 100).unwrap();
        assert_eq!(voice.priority(), 6);

        // Scenario 3: Video traffic with DEI set
        let video = CTag::new(5, true, 200).unwrap();
        assert_eq!(video.priority(), 5);
        assert!(video.dei());

        // Scenario 4: Management VLAN
        let mgmt = CTag::new(7, false, 10).unwrap();
        assert_eq!(mgmt.vid(), 10);
        assert_eq!(mgmt.priority(), 7);

        // Scenario 5: Maximum values
        let max_ctag = CTag::new(7, true, 4095).unwrap();
        assert_eq!(max_ctag.priority(), 7);
        assert!(max_ctag.dei());
        assert_eq!(max_ctag.vid(), 4095);
    }

    #[test]
    fn test_ctag_boundary_values() {
        // Minimum values
        assert!(CTag::new(0, false, 0).is_ok());

        // Maximum values
        assert!(CTag::new(7, true, 4095).is_ok());

        // Out of range
        assert!(CTag::new(8, false, 0).is_err());
        assert!(CTag::new(0, false, 4096).is_err());
    }
}
