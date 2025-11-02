//! MAC Addresses Detected Information Element
//!
//! The MAC Addresses Detected IE contains a list of MAC address values that have been detected
//! on an Ethernet PDU session. Per 3GPP TS 29.244 Section 8.2.103, this IE contains raw
//! 6-byte MAC address values plus optional VLAN tags.

use crate::ie::c_tag::CTag;
use crate::ie::s_tag::STag;
use crate::ie::{Ie, IeType};
use std::io;

/// MAC Addresses Detected
///
/// Contains a list of raw MAC address values detected on an Ethernet PDU session,
/// optionally associated with VLAN tags.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.103
///
/// # Structure (per 3GPP TS 29.244 §8.2.103)
/// - Octet 5: Number of MAC addresses (k)
/// - Octets 6 to 11: MAC address value 1 (6 bytes)
/// - Octets (o) to (o+5): MAC address value k (6 bytes)
/// - Length of C-TAG field (1 byte)
/// - C-TAG field (3 bytes if length > 0)
/// - Length of S-TAG field (1 byte)
/// - S-TAG field (3 bytes if length > 0)
///
/// Per spec: "Several IEs with the same IE type may be present to provision
/// multiple lists of MAC addresses (e.g. with different V-LAN tags)."
///
/// Note: This IE contains raw 6-byte MAC address values, not MAC Address IEs.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
/// use rs_pfcp::ie::c_tag::CTag;
///
/// // Create with single MAC address (no VLAN tags)
/// let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
/// let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
/// assert_eq!(detected.addresses().len(), 1);
///
/// // Create with multiple MAC addresses and VLAN tag
/// let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
/// let ctag = CTag::new(3, false, 100).unwrap();
/// let detected_vlan = MacAddressesDetected::new_with_vlan(
///     vec![mac1, mac2],
///     Some(ctag),
///     None
/// ).unwrap();
/// assert_eq!(detected_vlan.addresses().len(), 2);
/// assert!(detected_vlan.c_tag().is_some());
///
/// // Marshal and unmarshal
/// let bytes = detected.marshal();
/// let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
/// assert_eq!(detected, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressesDetected {
    /// List of detected MAC address values (6 bytes each)
    addresses: Vec<[u8; 6]>,
    /// Optional C-TAG (Customer VLAN Tag) - per 3GPP TS 29.244 §8.2.103
    c_tag: Option<CTag>,
    /// Optional S-TAG (Service VLAN Tag) - per 3GPP TS 29.244 §8.2.103
    s_tag: Option<STag>,
}

impl MacAddressesDetected {
    /// Maximum number of MAC addresses (per spec)
    pub const MAX_ADDRESSES: usize = 255;

    /// Create new MAC Addresses Detected IE without VLAN tags
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
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let detected = MacAddressesDetected::new(vec![mac]).unwrap();
    /// assert_eq!(detected.addresses().len(), 1);
    /// ```
    pub fn new(addresses: Vec<[u8; 6]>) -> Result<Self, io::Error> {
        Self::new_with_vlan(addresses, None, None)
    }

    /// Create new MAC Addresses Detected IE with optional VLAN tags
    ///
    /// # Arguments
    /// * `addresses` - List of MAC addresses (max 255)
    /// * `c_tag` - Optional Customer VLAN tag
    /// * `s_tag` - Optional Service VLAN tag
    ///
    /// # Errors
    /// Returns error if more than 255 addresses provided
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::c_tag::CTag;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let ctag = CTag::new(3, false, 100).unwrap();
    /// let detected = MacAddressesDetected::new_with_vlan(
    ///     vec![mac],
    ///     Some(ctag),
    ///     None
    /// ).unwrap();
    /// assert!(detected.c_tag().is_some());
    /// ```
    pub fn new_with_vlan(
        addresses: Vec<[u8; 6]>,
        c_tag: Option<CTag>,
        s_tag: Option<STag>,
    ) -> Result<Self, io::Error> {
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
        Ok(MacAddressesDetected {
            addresses,
            c_tag,
            s_tag,
        })
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
            c_tag: None,
            s_tag: None,
        }
    }

    /// Get the list of detected MAC address values
    pub fn addresses(&self) -> &[[u8; 6]] {
        &self.addresses
    }

    /// Get the optional C-TAG (Customer VLAN tag)
    pub fn c_tag(&self) -> Option<&CTag> {
        self.c_tag.as_ref()
    }

    /// Get the optional S-TAG (Service VLAN tag)
    pub fn s_tag(&self) -> Option<&STag> {
        self.s_tag.as_ref()
    }

    /// Get the number of detected MAC addresses
    pub fn count(&self) -> usize {
        self.addresses.len()
    }

    /// Marshal MAC Addresses Detected to bytes per 3GPP TS 29.244 §8.2.103
    ///
    /// # Returns
    /// Vector with:
    /// - Count byte
    /// - MAC address values (6 bytes each)
    /// - C-TAG length (1 byte) + C-TAG data (3 bytes if present)
    /// - S-TAG length (1 byte) + S-TAG data (3 bytes if present)
    pub fn marshal(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.addresses.len() * 6 + 2 + 6);

        // Octet 5: Number of MAC addresses
        bytes.push(self.addresses.len() as u8);

        // Octets 6+: MAC address values (6 bytes each)
        for mac in &self.addresses {
            bytes.extend_from_slice(mac);
        }

        // Length of C-TAG field (1 byte)
        if let Some(ref ctag) = self.c_tag {
            bytes.push(3); // C-TAG is 3 bytes
            bytes.extend_from_slice(&ctag.marshal());
        } else {
            bytes.push(0); // No C-TAG
        }

        // Length of S-TAG field (1 byte)
        if let Some(ref stag) = self.s_tag {
            bytes.push(3); // S-TAG is 3 bytes
            bytes.extend_from_slice(&stag.marshal());
        } else {
            bytes.push(0); // No S-TAG
        }

        bytes
    }

    /// Unmarshal MAC Addresses Detected from bytes per 3GPP TS 29.244 §8.2.103
    ///
    /// # Arguments
    /// * `data` - Byte slice containing MAC addresses data and optional VLAN tags
    ///
    /// # Errors
    /// Returns error if data is too short or malformed
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    ///
    /// let mac = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
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
        let mut offset = 1;

        // Parse MAC addresses
        let mut addresses = Vec::with_capacity(count);
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

            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            addresses.push(mac);
            offset += 6;
        }

        // Parse C-TAG length and value
        let c_tag = if offset < data.len() {
            let ctag_len = data[offset] as usize;
            offset += 1;

            if ctag_len > 0 {
                if ctag_len != 3 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("C-TAG length must be 0 or 3, got {}", ctag_len),
                    ));
                }
                if offset + 3 > data.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "MAC Addresses Detected: incomplete C-TAG data",
                    ));
                }
                let ctag = CTag::unmarshal(&data[offset..offset + 3])?;
                offset += 3;
                Some(ctag)
            } else {
                None
            }
        } else {
            None
        };

        // Parse S-TAG length and value
        let s_tag = if offset < data.len() {
            let stag_len = data[offset] as usize;
            offset += 1;

            if stag_len > 0 {
                if stag_len != 3 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("S-TAG length must be 0 or 3, got {}", stag_len),
                    ));
                }
                if offset + 3 > data.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "MAC Addresses Detected: incomplete S-TAG data",
                    ));
                }
                let stag = STag::unmarshal(&data[offset..offset + 3])?;
                Some(stag)
            } else {
                None
            }
        } else {
            None
        };

        Ok(MacAddressesDetected {
            addresses,
            c_tag,
            s_tag,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
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
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        assert_eq!(detected.count(), 1);
        assert_eq!(detected.addresses()[0], mac);
    }

    #[test]
    fn test_mac_addresses_detected_multiple() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let mac3 = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        let detected = MacAddressesDetected::new(vec![mac1, mac2, mac3]).unwrap();
        assert_eq!(detected.count(), 3);
        assert_eq!(detected.addresses()[0], mac1);
        assert_eq!(detected.addresses()[1], mac2);
        assert_eq!(detected.addresses()[2], mac3);
    }

    #[test]
    fn test_mac_addresses_detected_too_many() {
        let addresses: Vec<[u8; 6]> = (0..=255).map(|i| [i as u8, 0, 0, 0, 0, 0]).collect();

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
        assert_eq!(bytes.len(), 3); // count(1) + ctag_len(1) + stag_len(1)
        assert_eq!(bytes[0], 0); // count
        assert_eq!(bytes[1], 0); // C-TAG length
        assert_eq!(bytes[2], 0); // S-TAG length
    }

    #[test]
    fn test_mac_addresses_detected_marshal_single() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let bytes = detected.marshal();

        // count(1) + MAC(6) + ctag_len(1) + stag_len(1) = 9 bytes
        assert_eq!(bytes.len(), 9);
        assert_eq!(bytes[0], 1); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(bytes[7], 0); // C-TAG length
        assert_eq!(bytes[8], 0); // S-TAG length
    }

    #[test]
    fn test_mac_addresses_detected_marshal_multiple() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let detected = MacAddressesDetected::new(vec![mac1, mac2]).unwrap();
        let bytes = detected.marshal();

        // count(1) + 2*MAC(12) + ctag_len(1) + stag_len(1) = 15 bytes
        assert_eq!(bytes.len(), 15);
        assert_eq!(bytes[0], 2); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(&bytes[7..13], &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(bytes[13], 0); // C-TAG length
        assert_eq!(bytes[14], 0); // S-TAG length
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
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
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
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
        );
        assert_eq!(
            detected.addresses()[1],
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
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
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let ie = detected.to_ie();

        assert_eq!(ie.ie_type, IeType::MacAddressesDetected);
        // count(1) + MAC(6) + ctag_len(1) + stag_len(1) = 9 bytes
        assert_eq!(ie.payload.len(), 9);

        // Verify IE can be unmarshaled
        let parsed = MacAddressesDetected::unmarshal(&ie.payload).unwrap();
        assert_eq!(detected, parsed);
    }

    #[test]
    fn test_mac_addresses_detected_with_ctag() {
        use crate::ie::c_tag::CTag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let ctag = CTag::new(3, false, 100).unwrap();
        let detected = MacAddressesDetected::new_with_vlan(vec![mac], Some(ctag), None).unwrap();

        assert_eq!(detected.count(), 1);
        assert!(detected.c_tag().is_some());
        assert!(detected.s_tag().is_none());
        assert_eq!(detected.c_tag().unwrap().vid(), 100);

        // Test marshal/unmarshal round trip
        let bytes = detected.marshal();
        let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
        assert_eq!(detected, parsed);
    }

    #[test]
    fn test_mac_addresses_detected_with_stag() {
        use crate::ie::s_tag::STag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let stag = STag::new(5, true, 200).unwrap();
        let detected = MacAddressesDetected::new_with_vlan(vec![mac], None, Some(stag)).unwrap();

        assert_eq!(detected.count(), 1);
        assert!(detected.c_tag().is_none());
        assert!(detected.s_tag().is_some());
        assert_eq!(detected.s_tag().unwrap().vid(), 200);

        // Test marshal/unmarshal round trip
        let bytes = detected.marshal();
        let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
        assert_eq!(detected, parsed);
    }

    #[test]
    fn test_mac_addresses_detected_with_both_tags() {
        use crate::ie::c_tag::CTag;
        use crate::ie::s_tag::STag;

        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let ctag = CTag::new(3, false, 100).unwrap();
        let stag = STag::new(5, false, 200).unwrap();

        let detected =
            MacAddressesDetected::new_with_vlan(vec![mac1, mac2], Some(ctag), Some(stag)).unwrap();

        assert_eq!(detected.count(), 2);
        assert!(detected.c_tag().is_some());
        assert!(detected.s_tag().is_some());
        assert_eq!(detected.c_tag().unwrap().vid(), 100);
        assert_eq!(detected.s_tag().unwrap().vid(), 200);

        // Test marshal/unmarshal round trip
        let bytes = detected.marshal();
        let parsed = MacAddressesDetected::unmarshal(&bytes).unwrap();
        assert_eq!(detected, parsed);
    }

    #[test]
    fn test_mac_addresses_detected_vlan_marshal_format() {
        use crate::ie::c_tag::CTag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let ctag = CTag::new(3, false, 100).unwrap(); // PCP=3, DEI=0, VID=100
        let detected = MacAddressesDetected::new_with_vlan(vec![mac], Some(ctag), None).unwrap();

        let bytes = detected.marshal();

        // Expected structure:
        // count(1) + MAC(6) + ctag_len(1) + ctag_data(3) + stag_len(1) = 12 bytes
        assert_eq!(bytes.len(), 12);
        assert_eq!(bytes[0], 1); // Count = 1
        assert_eq!(&bytes[1..7], &mac); // MAC address
        assert_eq!(bytes[7], 3); // C-TAG length = 3
                                 // bytes[8..11] are C-TAG data
        assert_eq!(bytes[11], 0); // S-TAG length = 0
    }

    #[test]
    fn test_mac_addresses_detected_scenarios() {
        // Scenario 1: Single device detected
        let device1 = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let detected1 = MacAddressesDetected::new(vec![device1]).unwrap();
        assert_eq!(detected1.count(), 1);

        // Scenario 2: Multiple devices on network segment
        let devices = vec![
            [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
            [0x00, 0x1B, 0x44, 0x11, 0x3A, 0x2F],
            [0x00, 0x50, 0x56, 0xC0, 0x00, 0x01],
        ];
        let detected2 = MacAddressesDetected::new(devices).unwrap();
        assert_eq!(detected2.count(), 3);

        // Scenario 3: No devices detected yet
        let detected3 = MacAddressesDetected::empty();
        assert_eq!(detected3.count(), 0);
    }
}
