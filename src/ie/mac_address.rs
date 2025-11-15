//! MAC Address Information Element
//!
//! The MAC Address IE contains a 48-bit (6-byte) MAC address for Ethernet packet filtering.
//! Per 3GPP TS 29.244 Section 8.2.93, this IE is used in Ethernet packet filtering scenarios.
//!
//! MAC addresses are used to identify source or destination devices in Ethernet networks,
//! enabling MAC-based traffic filtering and forwarding in 5G UPF.

use crate::ie::{Ie, IeType};
use std::fmt;
use std::io;

/// MAC Address (48-bit / 6-byte address)
///
/// Represents a MAC address for Ethernet packet filtering.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.93
///
/// # Structure (Per 3GPP TS 29.244 Section 8.2.93)
/// - Octet 5: Flags (SOUR, DEST, USOU, UDES, spare bits)
/// - Optional fields based on flags:
///   - Source MAC address value (6 bytes) if SOUR=1
///   - Destination MAC address value (6 bytes) if DEST=1
///   - Upper Source MAC address value (6 bytes) if USOU=1
///   - Upper Destination MAC address value (6 bytes) if UDES=1
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::mac_address::MacAddress;
///
/// // Create source MAC
/// let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// assert_eq!(mac.source_mac, Some([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]));
///
/// // Create source and destination MAC
/// let mac2 = MacAddress::source_and_dest(
///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
/// );
/// assert!(mac2.source_mac.is_some());
/// assert!(mac2.destination_mac.is_some());
///
/// // Marshal and unmarshal
/// let bytes = mac.marshal();
/// let parsed = MacAddress::unmarshal(&bytes).unwrap();
/// assert_eq!(mac, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddress {
    /// Source MAC address (if SOUR flag is set)
    pub source_mac: Option<[u8; 6]>,
    /// Destination MAC address (if DEST flag is set)
    pub destination_mac: Option<[u8; 6]>,
    /// Upper Source MAC address for range (if USOU flag is set)
    pub upper_source_mac: Option<[u8; 6]>,
    /// Upper Destination MAC address for range (if UDES flag is set)
    pub upper_destination_mac: Option<[u8; 6]>,
}

impl Default for MacAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl MacAddress {
    /// Create a new MAC address (use the builder methods instead)
    ///
    /// Prefer using `source()`, `destination()`, or `source_and_dest()` constructors
    pub fn new() -> Self {
        MacAddress {
            source_mac: None,
            destination_mac: None,
            upper_source_mac: None,
            upper_destination_mac: None,
        }
    }

    /// Create MAC address with source MAC only
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// ```
    pub fn source(octets: [u8; 6]) -> Self {
        MacAddress {
            source_mac: Some(octets),
            destination_mac: None,
            upper_source_mac: None,
            upper_destination_mac: None,
        }
    }

    /// Create MAC address with destination MAC only
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::destination([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// ```
    pub fn destination(octets: [u8; 6]) -> Self {
        MacAddress {
            source_mac: None,
            destination_mac: Some(octets),
            upper_source_mac: None,
            upper_destination_mac: None,
        }
    }

    /// Create MAC address with both source and destination
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::source_and_dest(
    ///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
    ///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
    /// );
    /// ```
    pub fn source_and_dest(source: [u8; 6], dest: [u8; 6]) -> Self {
        MacAddress {
            source_mac: Some(source),
            destination_mac: Some(dest),
            upper_source_mac: None,
            upper_destination_mac: None,
        }
    }

    /// Create MAC address with source range
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::source_range(
    ///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x00],
    ///     [0x00, 0x11, 0x22, 0x33, 0x44, 0xFF]
    /// );
    /// ```
    pub fn source_range(lower: [u8; 6], upper: [u8; 6]) -> Self {
        MacAddress {
            source_mac: Some(lower),
            destination_mac: None,
            upper_source_mac: Some(upper),
            upper_destination_mac: None,
        }
    }

    /// Create MAC address with destination range
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::destination_range(
    ///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x00],
    ///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
    /// );
    /// ```
    pub fn destination_range(lower: [u8; 6], upper: [u8; 6]) -> Self {
        MacAddress {
            source_mac: None,
            destination_mac: Some(lower),
            upper_source_mac: None,
            upper_destination_mac: Some(upper),
        }
    }

    /// Marshal MAC address to bytes per 3GPP TS 29.244 Section 8.2.93
    ///
    /// # Returns
    /// Byte vector with: 1 byte flags + MAC address values
    ///
    /// # Format
    /// - Octet 5: Flags (SOUR, DEST, USOU, UDES)
    /// - Optional fields based on flags
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Octet 5: Flags
        let mut flags = 0u8;
        if self.source_mac.is_some() {
            flags |= 0x01; // SOUR bit
        }
        if self.destination_mac.is_some() {
            flags |= 0x02; // DEST bit
        }
        if self.upper_source_mac.is_some() {
            flags |= 0x04; // USOU bit
        }
        if self.upper_destination_mac.is_some() {
            flags |= 0x08; // UDES bit
        }
        data.push(flags);

        // Add MAC address values based on flags
        if let Some(ref mac) = self.source_mac {
            data.extend_from_slice(mac);
        }
        if let Some(ref mac) = self.destination_mac {
            data.extend_from_slice(mac);
        }
        if let Some(ref mac) = self.upper_source_mac {
            data.extend_from_slice(mac);
        }
        if let Some(ref mac) = self.upper_destination_mac {
            data.extend_from_slice(mac);
        }

        data
    }

    /// Unmarshal MAC address from bytes per 3GPP TS 29.244 Section 8.2.93
    ///
    /// # Arguments
    /// * `data` - Byte slice containing flags + MAC address values
    ///
    /// # Errors
    /// Returns error if data is too short based on flags
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::source([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    /// let bytes = mac.marshal();
    /// let parsed = MacAddress::unmarshal(&bytes).unwrap();
    /// assert_eq!(mac, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MAC Address IE requires at least 1 byte for flags",
            ));
        }

        // Parse flags from Octet 5
        let flags = data[0];
        let sour = (flags & 0x01) != 0;
        let dest = (flags & 0x02) != 0;
        let usou = (flags & 0x04) != 0;
        let udes = (flags & 0x08) != 0;

        // Check spare bits (bits 5-8 should be 0)
        if (flags & 0xF0) != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("MAC Address IE has invalid spare bits: 0x{:02X}", flags),
            ));
        }

        let mut offset = 1;
        let mut source_mac = None;
        let mut destination_mac = None;
        let mut upper_source_mac = None;
        let mut upper_destination_mac = None;

        // Parse Source MAC address
        if sour {
            if data.len() < offset + 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MAC Address IE too short for source MAC",
                ));
            }
            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            source_mac = Some(mac);
            offset += 6;
        }

        // Parse Destination MAC address
        if dest {
            if data.len() < offset + 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MAC Address IE too short for destination MAC",
                ));
            }
            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            destination_mac = Some(mac);
            offset += 6;
        }

        // Parse Upper Source MAC address (for range)
        if usou {
            if data.len() < offset + 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MAC Address IE too short for upper source MAC",
                ));
            }
            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            upper_source_mac = Some(mac);
            offset += 6;
        }

        // Parse Upper Destination MAC address (for range)
        if udes {
            if data.len() < offset + 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MAC Address IE too short for upper destination MAC",
                ));
            }
            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            upper_destination_mac = Some(mac);
        }

        Ok(MacAddress {
            source_mac,
            destination_mac,
            upper_source_mac,
            upper_destination_mac,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// let ie = mac.to_ie();
    /// assert_eq!(ie.ie_type, IeType::MacAddress);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MacAddress, self.marshal())
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(src) = self.source_mac {
            write!(
                f,
                "SRC:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                src[0], src[1], src[2], src[3], src[4], src[5]
            )?;
        }
        if let Some(dst) = self.destination_mac {
            if self.source_mac.is_some() {
                write!(f, " ")?;
            }
            write!(
                f,
                "DST:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                dst[0], dst[1], dst[2], dst[3], dst[4], dst[5]
            )?;
        }
        if let Some(upper_src) = self.upper_source_mac {
            write!(
                f,
                " UPPER_SRC:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                upper_src[0], upper_src[1], upper_src[2], upper_src[3], upper_src[4], upper_src[5]
            )?;
        }
        if let Some(upper_dst) = self.upper_destination_mac {
            write!(
                f,
                " UPPER_DST:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                upper_dst[0], upper_dst[1], upper_dst[2], upper_dst[3], upper_dst[4], upper_dst[5]
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_new() {
        let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.source_mac, Some([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]));
        assert!(mac.destination_mac.is_none());
        assert!(mac.upper_source_mac.is_none());
        assert!(mac.upper_destination_mac.is_none());
    }

    #[test]
    fn test_mac_address_marshal() {
        let mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let bytes = mac.marshal();
        // Flags byte (0x01 for SOUR) + 6 bytes MAC = 7 bytes
        assert_eq!(bytes.len(), 7);
        assert_eq!(bytes[0], 0x01); // SOUR flag
        assert_eq!(bytes[1..], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_mac_address_unmarshal_valid() {
        // Flags byte 0x01 (SOUR) + 6 bytes MAC
        let data = [0x01, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
        let mac = MacAddress::unmarshal(&data).unwrap();
        assert_eq!(mac.source_mac, Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]));
        assert!(mac.destination_mac.is_none());
    }

    #[test]
    fn test_mac_address_unmarshal_short() {
        // Flags byte 0x01 but missing MAC address bytes
        let data = [0x01, 0x11, 0x22];
        let result = MacAddress::unmarshal(&data);
        assert!(result.is_err());
        assert!(result.is_err()); // Error type changed to PfcpError
    }

    #[test]
    fn test_mac_address_unmarshal_empty() {
        let result = MacAddress::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err
            .to_string()
            .contains("requires at least 1 byte for flags"));
    }

    #[test]
    fn test_mac_address_round_trip() {
        let original = MacAddress::source([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
        let marshaled = original.marshal();
        let unmarshaled = MacAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_mac_address_constructors() {
        // Test source constructor
        let src = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(src.source_mac.is_some());
        assert!(src.destination_mac.is_none());

        // Test destination constructor
        let dst = MacAddress::destination([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert!(dst.source_mac.is_none());
        assert!(dst.destination_mac.is_some());

        // Test source_and_dest constructor
        let both = MacAddress::source_and_dest(
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        );
        assert!(both.source_mac.is_some());
        assert!(both.destination_mac.is_some());
    }

    #[test]
    fn test_mac_address_to_ie() {
        let mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ie = mac.to_ie();
        assert_eq!(ie.ie_type, IeType::MacAddress);
        // Flags byte (1) + MAC address (6) = 7 bytes
        assert_eq!(ie.payload.len(), 7);
        assert_eq!(ie.payload[0], 0x01); // SOUR flag
        assert_eq!(ie.payload[1..], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        // Verify IE can be unmarshaled
        let parsed = MacAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(mac, parsed);
    }

    #[test]
    fn test_mac_address_display() {
        let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.to_string(), "SRC:00:11:22:33:44:55");

        let mac2 = MacAddress::destination([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(mac2.to_string(), "DST:AA:BB:CC:DD:EE:FF");

        let both = MacAddress::source_and_dest(
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        );
        assert_eq!(
            both.to_string(),
            "SRC:11:22:33:44:55:66 DST:AA:BB:CC:DD:EE:FF"
        );
    }

    #[test]
    fn test_mac_address_scenarios() {
        // Scenario 1: Source MAC only
        let src_only = MacAddress::source([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert!(src_only.source_mac.is_some());
        assert!(src_only.destination_mac.is_none());

        // Scenario 2: Destination MAC only
        let dst_only = MacAddress::destination([0x01, 0x00, 0x5E, 0x01, 0x02, 0x03]);
        assert!(dst_only.source_mac.is_none());
        assert!(dst_only.destination_mac.is_some());

        // Scenario 3: Both source and destination
        let both = MacAddress::source_and_dest(
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        );
        assert!(both.source_mac.is_some());
        assert!(both.destination_mac.is_some());

        // Scenario 4: Source range
        let src_range = MacAddress::source_range(
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x00],
            [0x00, 0x11, 0x22, 0x33, 0x44, 0xFF],
        );
        assert!(src_range.source_mac.is_some());
        assert!(src_range.upper_source_mac.is_some());
    }

    #[test]
    fn test_mac_address_clone_copy() {
        let mac1 = MacAddress::source([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        let mac2 = mac1;
        assert_eq!(mac1, mac2);

        let mac3 = mac1;
        assert_eq!(mac1, mac3);
    }
}
