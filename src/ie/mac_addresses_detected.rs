//! MAC Addresses Detected Information Element
//!
//! The MAC Addresses Detected IE contains a list of MAC addresses that have been detected
//! on an Ethernet PDU session. Per 3GPP TS 29.244 Section 8.2.103, this IE is used for
//! MAC address learning and reporting in Ethernet sessions.

use crate::ie::mac_address::MacAddress;
use crate::ie::{Ie, IeType};
use std::io;

/// MAC Addresses Detected
///
/// Contains a list of MAC addresses detected on an Ethernet PDU session.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.103
///
/// # Structure
/// - Octet 5: Number of MAC addresses
/// - Octets 6 to m: MAC address(es) (6 bytes each)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
/// use rs_pfcp::ie::mac_address::MacAddress;
///
/// // Create with single MAC address
/// let mac1 = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// let detected = MacAddressesDetected::new(vec![mac1]);
/// assert_eq!(detected.addresses().len(), 1);
///
/// // Create with multiple MAC addresses
/// let mac2 = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
/// let detected2 = MacAddressesDetected::new(vec![mac1, mac2]);
/// assert_eq!(detected2.addresses().len(), 2);
///
/// // Marshal and unmarshal
/// let bytes = detected.marshal();
/// let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
/// assert_eq!(detected, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressesDetected {
    /// List of detected MAC addresses
    addresses: Vec<MacAddress>,
}

impl MacAddressesDetected {
    /// Maximum number of MAC addresses (per spec)
    pub const MAX_ADDRESSES: usize = 255;

    /// Create new MAC Addresses Detected IE
    ///
    /// # Arguments
    /// * `addresses` - List of MAC addresses (max 255)
    ///
    /// # Errors
    /// Returns error if more than 255 addresses provided
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// let detected = MacAddressesDetected::new(vec![mac]);
    /// assert_eq!(detected.addresses().len(), 1);
    /// ```
    pub fn new(addresses: Vec<MacAddress>) -> Result<Self, io::Error> {
        if addresses.len() > Self::MAX_ADDRESSES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "MAC Addresses Detected cannot contain more than {} addresses, got {}",
                    Self::MAX_ADDRESSES,
                    addresses.len()
                ),
            ));
        }
        Ok(MacAddressesDetected { addresses })
    }

    /// Create empty MAC Addresses Detected IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    ///
    /// let detected = MacAddressesDetected::empty();
    /// assert_eq!(detected.addresses().len(), 0);
    /// ```
    pub fn empty() -> Self {
        MacAddressesDetected {
            addresses: Vec::new(),
        }
    }

    /// Get the list of detected MAC addresses
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// let detected = MacAddressesDetected::new(vec![mac]).unwrap();
    /// assert_eq!(detected.addresses()[0], mac);
    /// ```
    pub fn addresses(&self) -> &[MacAddress] {
        &self.addresses
    }

    /// Get the number of detected MAC addresses
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac1 = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// let mac2 = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// let detected = MacAddressesDetected::new(vec![mac1, mac2]).unwrap();
    /// assert_eq!(detected.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.addresses.len()
    }

    /// Marshal MAC Addresses Detected to bytes
    ///
    /// # Returns
    /// Vector with count byte followed by MAC addresses (6 bytes each)
    pub fn marshal(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.addresses.len() * 6);

        // Byte 0: Number of MAC addresses
        bytes.push(self.addresses.len() as u8);

        // Bytes 1+: MAC addresses (6 bytes each)
        for mac in &self.addresses {
            bytes.extend_from_slice(&mac.marshal());
        }

        bytes
    }

    /// Unmarshal MAC Addresses Detected from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing MAC addresses data
    ///
    /// # Errors
    /// Returns error if data is too short or malformed
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    /// let detected = MacAddressesDetected::new(vec![mac]).unwrap();
    /// let bytes = detected.marshal();
    /// let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
    /// assert_eq!(detected, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MAC Addresses Detected requires at least 1 byte for count",
            ));
        }

        let count = data[0] as usize;

        // Verify we have enough data for all MAC addresses
        let expected_len = 1 + count * 6;
        if data.len() < expected_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "MAC Addresses Detected: expected {} bytes for {} addresses, got {}",
                    expected_len,
                    count,
                    data.len()
                ),
            ));
        }

        let mut addresses = Vec::with_capacity(count);
        let mut offset = 1;

        for i in 0..count {
            if offset + 6 > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "MAC Addresses Detected: incomplete MAC address {} at offset {}",
                        i, offset
                    ),
                ));
            }

            let mac = MacAddress::unmarshal(&data[offset..offset + 6])?;
            addresses.push(mac);
            offset += 6;
        }

        Ok(MacAddressesDetected { addresses })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// let detected = MacAddressesDetected::new(vec![mac]).unwrap();
    /// let ie = detected.to_ie();
    /// assert_eq!(ie.ie_type, IeType::MacAddressesDetected);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MacAddressesDetected, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_addresses_detected_empty() {
        let detected = MacAddressesDetected::empty();
        assert_eq!(detected.count(), 0);
        assert_eq!(detected.addresses().len(), 0);
    }

    #[test]
    fn test_mac_addresses_detected_single() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        assert_eq!(detected.count(), 1);
        assert_eq!(detected.addresses()[0], mac);
    }

    #[test]
    fn test_mac_addresses_detected_multiple() {
        let mac1 = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let mac2 = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let mac3 = MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);

        let detected = MacAddressesDetected::new(vec![mac1, mac2, mac3]).unwrap();
        assert_eq!(detected.count(), 3);
        assert_eq!(detected.addresses()[0], mac1);
        assert_eq!(detected.addresses()[1], mac2);
        assert_eq!(detected.addresses()[2], mac3);
    }

    #[test]
    fn test_mac_addresses_detected_too_many() {
        let addresses: Vec<MacAddress> = (0..=255)
            .map(|i| MacAddress::new([i as u8, 0, 0, 0, 0, 0]))
            .collect();

        let result = MacAddressesDetected::new(addresses);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot contain more than 255"));
    }

    #[test]
    fn test_mac_addresses_detected_marshal_empty() {
        let detected = MacAddressesDetected::empty();
        let bytes = detected.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0);
    }

    #[test]
    fn test_mac_addresses_detected_marshal_single() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let bytes = detected.marshal();

        assert_eq!(bytes.len(), 7); // 1 byte count + 6 bytes MAC
        assert_eq!(bytes[0], 1); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_mac_addresses_detected_marshal_multiple() {
        let mac1 = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let mac2 = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        let detected = MacAddressesDetected::new(vec![mac1, mac2]).unwrap();
        let bytes = detected.marshal();

        assert_eq!(bytes.len(), 13); // 1 byte count + 12 bytes (2 MACs)
        assert_eq!(bytes[0], 2); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(&bytes[7..13], &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_mac_addresses_detected_unmarshal_empty() {
        let data = [0x00]; // Count = 0
        let detected = MacAddressesDetected::unmarshal(&data).unwrap();
        assert_eq!(detected.count(), 0);
    }

    #[test]
    fn test_mac_addresses_detected_unmarshal_single() {
        let data = [0x01, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::unmarshal(&data).unwrap();
        assert_eq!(detected.count(), 1);
        assert_eq!(
            detected.addresses()[0],
            MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
        );
    }

    #[test]
    fn test_mac_addresses_detected_unmarshal_multiple() {
        let data = [
            0x02, // Count = 2
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // MAC 1
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, // MAC 2
        ];
        let detected = MacAddressesDetected::unmarshal(&data).unwrap();
        assert_eq!(detected.count(), 2);
        assert_eq!(
            detected.addresses()[0],
            MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
        );
        assert_eq!(
            detected.addresses()[1],
            MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])
        );
    }

    #[test]
    fn test_mac_addresses_detected_unmarshal_no_data() {
        let result = MacAddressesDetected::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least 1 byte"));
    }

    #[test]
    fn test_mac_addresses_detected_unmarshal_incomplete() {
        let data = [0x02, 0x00, 0x11, 0x22]; // Says 2 MACs but only partial data
        let result = MacAddressesDetected::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_mac_addresses_detected_round_trip() {
        let test_cases = vec![
            vec![],
            vec![MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])],
            vec![
                MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
                MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            ],
            vec![
                MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
                MacAddress::new([0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC]),
                MacAddress::new([0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22]),
            ],
        ];

        for addresses in test_cases {
            let original = MacAddressesDetected::new(addresses.clone()).unwrap();
            let marshaled = original.marshal();
            let unmarshaled = MacAddressesDetected::unmarshal(&marshaled).unwrap();
            assert_eq!(
                original,
                unmarshaled,
                "Failed for {} addresses",
                addresses.len()
            );
        }
    }

    #[test]
    fn test_mac_addresses_detected_to_ie() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let ie = detected.to_ie();

        assert_eq!(ie.ie_type, IeType::MacAddressesDetected);
        assert_eq!(ie.payload.len(), 7);

        // Verify IE can be unmarshaled
        let parsed = MacAddressesDetected::unmarshal(&ie.payload).unwrap();
        assert_eq!(detected, parsed);
    }

    #[test]
    fn test_mac_addresses_detected_scenarios() {
        // Scenario 1: Single device detected
        let device1 = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        let detected1 = MacAddressesDetected::new(vec![device1]).unwrap();
        assert_eq!(detected1.count(), 1);

        // Scenario 2: Multiple devices on network segment
        let devices = vec![
            MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]),
            MacAddress::new([0x00, 0x1B, 0x44, 0x11, 0x3A, 0x2F]),
            MacAddress::new([0x00, 0x50, 0x56, 0xC0, 0x00, 0x01]),
        ];
        let detected2 = MacAddressesDetected::new(devices).unwrap();
        assert_eq!(detected2.count(), 3);

        // Scenario 3: No devices detected yet
        let detected3 = MacAddressesDetected::empty();
        assert_eq!(detected3.count(), 0);
    }
}
