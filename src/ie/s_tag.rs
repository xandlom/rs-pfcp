//! S-TAG (Service VLAN Tag) Information Element
//!
//! The S-TAG IE contains service provider VLAN tagging information for Ethernet packet filtering.
//! Per 3GPP TS 29.244 Section 8.2.95, this IE is used in Ethernet packet filtering scenarios.
//!
//! S-TAG is used in provider bridging (IEEE 802.1ad) for service provider VLAN tagging.

use crate::ie::{Ie, IeType};
use std::io;

/// S-TAG (Service VLAN Tag)
///
/// Represents service provider VLAN tagging information per IEEE 802.1ad.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.95
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
/// use rs_pfcp::ie::s_tag::STag;
///
/// // Create S-TAG with priority 3, no DEI, VLAN ID 100
/// let stag = STag::new(3, false, 100).unwrap();
/// assert_eq!(stag.priority(), 3);
/// assert_eq!(stag.dei(), false);
/// assert_eq!(stag.vid(), 100);
///
/// // Marshal and unmarshal
/// let bytes = stag.marshal();
/// let parsed = STag::unmarshal(&bytes).unwrap();
/// assert_eq!(stag, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct STag {
    /// Priority Code Point (0-7, 3 bits)
    pcp: u8,
    /// Drop Eligible Indicator (boolean)
    dei: bool,
    /// VLAN ID (0-4095, 12 bits)
    vid: u16,
}

impl STag {
    /// Maximum valid priority (3 bits = 0-7)
    pub const MAX_PRIORITY: u8 = 7;
    /// Maximum valid VID (12 bits = 0-4095)
    pub const MAX_VID: u16 = 4095;

    /// Create a new S-TAG
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
    /// use rs_pfcp::ie::s_tag::STag;
    ///
    /// let stag = STag::new(5, true, 1000).unwrap();
    /// assert_eq!(stag.priority(), 5);
    /// assert_eq!(stag.dei(), true);
    /// assert_eq!(stag.vid(), 1000);
    /// ```
    pub fn new(pcp: u8, dei: bool, vid: u16) -> Result<Self, io::Error> {
        if pcp > Self::MAX_PRIORITY {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("S-TAG PCP {} exceeds maximum {}", pcp, Self::MAX_PRIORITY),
            ));
        }
        if vid > Self::MAX_VID {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("S-TAG VID {} exceeds maximum {}", vid, Self::MAX_VID),
            ));
        }
        Ok(STag { pcp, dei, vid })
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

    /// Marshal S-TAG to bytes
    ///
    /// # Returns
    /// 3-byte array with S-TAG encoded
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

    /// Unmarshal S-TAG from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing S-TAG data (must be at least 3 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::s_tag::STag;
    ///
    /// let stag = STag::new(7, false, 4095).unwrap();
    /// let bytes = stag.marshal();
    /// let parsed = STag::unmarshal(&bytes).unwrap();
    /// assert_eq!(stag, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("S-TAG requires 3 bytes, got {}", data.len()),
            ));
        }

        // Extract PCP (bits 7-5 of byte 0)
        let pcp = (data[0] >> 5) & 0x07;

        // Extract DEI (bit 4 of byte 0)
        let dei = (data[0] & 0x10) != 0;

        // Extract VID (bits 3-0 of byte 0 + byte 1)
        let vid = (((data[0] & 0x0F) as u16) << 8) | (data[1] as u16);

        Ok(STag { pcp, dei, vid })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::s_tag::STag;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let stag = STag::new(2, false, 500).unwrap();
    /// let ie = stag.to_ie();
    /// assert_eq!(ie.ie_type, IeType::STag);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::STag, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stag_new_valid() {
        let stag = STag::new(0, false, 0).unwrap();
        assert_eq!(stag.priority(), 0);
        assert!(!stag.dei());
        assert_eq!(stag.vid(), 0);

        let stag2 = STag::new(7, true, 4095).unwrap();
        assert_eq!(stag2.priority(), 7);
        assert!(stag2.dei());
        assert_eq!(stag2.vid(), 4095);
    }

    #[test]
    fn test_stag_new_invalid_pcp() {
        let result = STag::new(8, false, 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PCP"));
    }

    #[test]
    fn test_stag_new_invalid_vid() {
        let result = STag::new(3, false, 4096);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("VID"));
    }

    #[test]
    fn test_stag_marshal() {
        let stag = STag::new(5, true, 1000).unwrap();
        let bytes = stag.marshal();
        assert_eq!(bytes.len(), 3);

        // Expected byte 0: 101 1 0011 = 0xB3
        assert_eq!(bytes[0], 0xB3);

        // Verify VID low byte = 0xE8
        assert_eq!(bytes[1], 0xE8);

        // Spare byte
        assert_eq!(bytes[2], 0x00);
    }

    #[test]
    fn test_stag_unmarshal_valid() {
        let data = [0xB3, 0xE8, 0x00]; // PCP=5, DEI=1, VID=1000
        let stag = STag::unmarshal(&data).unwrap();
        assert_eq!(stag.priority(), 5);
        assert!(stag.dei());
        assert_eq!(stag.vid(), 1000);
    }

    #[test]
    fn test_stag_unmarshal_short() {
        let data = [0xB3, 0xE8];
        let result = STag::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_stag_unmarshal_empty() {
        let result = STag::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 3 bytes"));
    }

    #[test]
    fn test_stag_round_trip() {
        let test_cases = vec![
            (0, false, 0),
            (7, true, 4095),
            (3, false, 100),
            (5, true, 1000),
            (1, false, 1),
        ];

        for (pcp, dei, vid) in test_cases {
            let original = STag::new(pcp, dei, vid).unwrap();
            let marshaled = original.marshal();
            let unmarshaled = STag::unmarshal(&marshaled).unwrap();
            assert_eq!(
                original, unmarshaled,
                "Failed for PCP={}, DEI={}, VID={}",
                pcp, dei, vid
            );
        }
    }

    #[test]
    fn test_stag_to_ie() {
        let stag = STag::new(4, false, 200).unwrap();
        let ie = stag.to_ie();
        assert_eq!(ie.ie_type, IeType::STag);
        assert_eq!(ie.payload.len(), 3);

        // Verify IE can be unmarshaled
        let parsed = STag::unmarshal(&ie.payload).unwrap();
        assert_eq!(stag, parsed);
    }

    #[test]
    fn test_stag_scenarios() {
        // Scenario 1: Service provider backbone VLAN
        let backbone = STag::new(7, false, 100).unwrap();
        assert_eq!(backbone.vid(), 100);
        assert_eq!(backbone.priority(), 7);

        // Scenario 2: Customer service VLAN with DEI
        let customer_service = STag::new(5, true, 500).unwrap();
        assert_eq!(customer_service.priority(), 5);
        assert!(customer_service.dei());

        // Scenario 3: Provider Edge VLAN
        let pe_vlan = STag::new(6, false, 1000).unwrap();
        assert_eq!(pe_vlan.vid(), 1000);

        // Scenario 4: QinQ double tagging scenario
        let outer_tag = STag::new(4, false, 2000).unwrap();
        assert_eq!(outer_tag.vid(), 2000);

        // Scenario 5: Maximum values for provider network
        let max_stag = STag::new(7, true, 4095).unwrap();
        assert_eq!(max_stag.priority(), 7);
        assert!(max_stag.dei());
        assert_eq!(max_stag.vid(), 4095);
    }

    #[test]
    fn test_stag_boundary_values() {
        // Minimum values
        assert!(STag::new(0, false, 0).is_ok());

        // Maximum values
        assert!(STag::new(7, true, 4095).is_ok());

        // Out of range
        assert!(STag::new(8, false, 0).is_err());
        assert!(STag::new(0, false, 4096).is_err());
    }
}
