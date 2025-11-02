//! MAC Addresses Removed Information Element
//!
//! The MAC Addresses Removed IE contains a list of MAC address values that have been removed
//! on an Ethernet PDU session. Per 3GPP TS 29.244 Section 8.2.104, this IE contains raw
//! 6-byte MAC address values (not MAC Address IEs).

use crate::ie::{Ie, IeType};
use std::io;

/// MAC Addresses Removed
///
/// Contains a list of raw MAC address values removed on an Ethernet PDU session.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.104
///
/// # Structure
/// - Octet 5: Number of MAC addresses (k)
/// - Octets 6 to 11: MAC address value 1 (6 bytes)
/// - Octets (o) to (o+5): MAC address value 2 (6 bytes)
/// - ... MAC address value k
///
/// Note: This IE contains raw 6-byte MAC address values, not MAC Address IEs.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
///
/// // Create with single MAC address
/// let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
/// let removed = MacAddressesRemoved::new(vec![mac1]).unwrap();
/// assert_eq!(removed.addresses().len(), 1);
///
/// // Create with multiple MAC addresses
/// let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
/// let removed2 = MacAddressesRemoved::new(vec![mac1, mac2]).unwrap();
/// assert_eq!(removed2.addresses().len(), 2);
///
/// // Marshal and unmarshal
/// let bytes = removed.marshal();
/// let parsed = MacAddressesRemoved::unmarshal(&bytes).unwrap();
/// assert_eq!(removed, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressesRemoved {
    /// List of removed MAC address values (6 bytes each)
    addresses: Vec<[u8; 6]>,
}

impl MacAddressesRemoved {
    /// Maximum number of MAC addresses (per spec)
    pub const MAX_ADDRESSES: usize = 255;

    /// Create new MAC Addresses Removed IE
    ///
    /// # Arguments
    /// * `addresses` - List of MAC addresses (max 255)
    ///
    /// # Errors
    /// Returns error if more than 255 addresses provided
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
    /// assert_eq!(removed.addresses().len(), 1);
    /// ```
    pub fn new(addresses: Vec<[u8; 6]>) -> Result<Self, io::Error> {
        if addresses.len() > Self::MAX_ADDRESSES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "MAC Addresses Removed cannot contain more than {} addresses, got {}",
                    Self::MAX_ADDRESSES,
                    addresses.len()
                ),
            ));
        }
        Ok(MacAddressesRemoved { addresses })
    }

    /// Create empty MAC Addresses Removed IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    ///
    /// let removed = MacAddressesRemoved::empty();
    /// assert_eq!(removed.addresses().len(), 0);
    /// ```
    pub fn empty() -> Self {
        MacAddressesRemoved {
            addresses: Vec::new(),
        }
    }

    /// Get the list of removed MAC address values
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
    /// assert_eq!(removed.addresses()[0], mac);
    /// ```
    pub fn addresses(&self) -> &[[u8; 6]] {
        &self.addresses
    }

    /// Get the number of removed MAC addresses
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    ///
    /// let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    /// let removed = MacAddressesRemoved::new(vec![mac1, mac2]).unwrap();
    /// assert_eq!(removed.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.addresses.len()
    }

    /// Marshal MAC Addresses Removed to bytes
    ///
    /// # Returns
    /// Vector with count byte followed by raw MAC address values (6 bytes each)
    pub fn marshal(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.addresses.len() * 6);

        // Byte 0: Number of MAC addresses
        bytes.push(self.addresses.len() as u8);

        // Bytes 1+: MAC address values (6 bytes each)
        for mac in &self.addresses {
            bytes.extend_from_slice(mac);
        }

        bytes
    }

    /// Unmarshal MAC Addresses Removed from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing MAC addresses data
    ///
    /// # Errors
    /// Returns error if data is too short or malformed
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    ///
    /// let mac = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
    /// let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
    /// let bytes = removed.marshal();
    /// let parsed = MacAddressesRemoved::unmarshal(&bytes).unwrap();
    /// assert_eq!(removed, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MAC Addresses Removed requires at least 1 byte for count",
            ));
        }

        let count = data[0] as usize;

        // Verify we have enough data for all MAC addresses
        let expected_len = 1 + count * 6;
        if data.len() < expected_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "MAC Addresses Removed: expected {} bytes for {} addresses, got {}",
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
                        "MAC Addresses Removed: incomplete MAC address {} at offset {}",
                        i, offset
                    ),
                ));
            }

            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            addresses.push(mac);
            offset += 6;
        }

        Ok(MacAddressesRemoved { addresses })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
    /// let ie = removed.to_ie();
    /// assert_eq!(ie.ie_type, IeType::MacAddressesRemoved);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MacAddressesRemoved, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_addresses_removed_empty() {
        let removed = MacAddressesRemoved::empty();
        assert_eq!(removed.count(), 0);
        assert_eq!(removed.addresses().len(), 0);
    }

    #[test]
    fn test_mac_addresses_removed_single() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
        assert_eq!(removed.count(), 1);
        assert_eq!(removed.addresses()[0], mac);
    }

    #[test]
    fn test_mac_addresses_removed_multiple() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let mac3 = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        let removed = MacAddressesRemoved::new(vec![mac1, mac2, mac3]).unwrap();
        assert_eq!(removed.count(), 3);
        assert_eq!(removed.addresses()[0], mac1);
        assert_eq!(removed.addresses()[1], mac2);
        assert_eq!(removed.addresses()[2], mac3);
    }

    #[test]
    fn test_mac_addresses_removed_too_many() {
        let addresses: Vec<[u8; 6]> = (0..=255).map(|i| [i as u8, 0, 0, 0, 0, 0]).collect();

        let result = MacAddressesRemoved::new(addresses);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot contain more than 255"));
    }

    #[test]
    fn test_mac_addresses_removed_marshal_empty() {
        let removed = MacAddressesRemoved::empty();
        let bytes = removed.marshal();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0);
    }

    #[test]
    fn test_mac_addresses_removed_marshal_single() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
        let bytes = removed.marshal();

        assert_eq!(bytes.len(), 7); // 1 byte count + 6 bytes MAC
        assert_eq!(bytes[0], 1); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_mac_addresses_removed_marshal_multiple() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let removed = MacAddressesRemoved::new(vec![mac1, mac2]).unwrap();
        let bytes = removed.marshal();

        assert_eq!(bytes.len(), 13); // 1 byte count + 12 bytes (2 MACs)
        assert_eq!(bytes[0], 2); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(&bytes[7..13], &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_mac_addresses_removed_unmarshal_empty() {
        let data = [0x00]; // Count = 0
        let removed = MacAddressesRemoved::unmarshal(&data).unwrap();
        assert_eq!(removed.count(), 0);
    }

    #[test]
    fn test_mac_addresses_removed_unmarshal_single() {
        let data = [0x01, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let removed = MacAddressesRemoved::unmarshal(&data).unwrap();
        assert_eq!(removed.count(), 1);
        assert_eq!(removed.addresses()[0], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_mac_addresses_removed_unmarshal_multiple() {
        let data = [
            0x02, // Count = 2
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // MAC 1
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, // MAC 2
        ];
        let removed = MacAddressesRemoved::unmarshal(&data).unwrap();
        assert_eq!(removed.count(), 2);
        assert_eq!(removed.addresses()[0], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(removed.addresses()[1], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_mac_addresses_removed_unmarshal_no_data() {
        let result = MacAddressesRemoved::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least 1 byte"));
    }

    #[test]
    fn test_mac_addresses_removed_unmarshal_incomplete() {
        let data = [0x02, 0x00, 0x11, 0x22]; // Says 2 MACs but only partial data
        let result = MacAddressesRemoved::unmarshal(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_mac_addresses_removed_round_trip() {
        let test_cases = vec![
            vec![],
            vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]],
            vec![
                [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
                [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            ],
            vec![
                [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
                [0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC],
                [0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22],
            ],
        ];

        for addresses in test_cases {
            let original = MacAddressesRemoved::new(addresses.clone()).unwrap();
            let marshaled = original.marshal();
            let unmarshaled = MacAddressesRemoved::unmarshal(&marshaled).unwrap();
            assert_eq!(
                original,
                unmarshaled,
                "Failed for {} addresses",
                addresses.len()
            );
        }
    }

    #[test]
    fn test_mac_addresses_removed_to_ie() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
        let ie = removed.to_ie();

        assert_eq!(ie.ie_type, IeType::MacAddressesRemoved);
        assert_eq!(ie.payload.len(), 7);

        // Verify IE can be unmarshaled
        let parsed = MacAddressesRemoved::unmarshal(&ie.payload).unwrap();
        assert_eq!(removed, parsed);
    }

    #[test]
    fn test_mac_addresses_removed_scenarios() {
        // Scenario 1: Single device removed
        let device1 = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let removed1 = MacAddressesRemoved::new(vec![device1]).unwrap();
        assert_eq!(removed1.count(), 1);

        // Scenario 2: Multiple devices on network segment
        let devices = vec![
            [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
            [0x00, 0x1B, 0x44, 0x11, 0x3A, 0x2F],
            [0x00, 0x50, 0x56, 0xC0, 0x00, 0x01],
        ];
        let removed2 = MacAddressesRemoved::new(devices).unwrap();
        assert_eq!(removed2.count(), 3);

        // Scenario 3: No devices removed yet
        let removed3 = MacAddressesRemoved::empty();
        assert_eq!(removed3.count(), 0);
    }
}
