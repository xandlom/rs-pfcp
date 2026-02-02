//! MAC Addresses Removed Information Element
//!
//! The MAC Addresses Removed IE contains a list of MAC address values that have been removed
//! on an Ethernet PDU session. Per 3GPP TS 29.244 Section 8.2.104, this IE contains raw
//! 6-byte MAC address values with optional C-TAG and S-TAG VLAN identifiers.

use crate::error::PfcpError;
use crate::ie::c_tag::CTag;
use crate::ie::s_tag::STag;
use crate::ie::{Ie, IeType};

/// MAC Addresses Removed
///
/// Contains a list of raw MAC address values removed on an Ethernet PDU session.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.104
///
/// # Structure (per 3GPP TS 29.244 § 8.2.104)
/// - Octet 5: Number of MAC addresses (k)
/// - Octets 6+: MAC address values (6 bytes each)
/// - Length of C-TAG field (1 byte)
/// - C-TAG field (3 bytes if length > 0)
/// - Length of S-TAG field (1 byte)
/// - S-TAG field (3 bytes if length > 0)
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
    /// Optional Customer VLAN tag (C-TAG)
    c_tag: Option<CTag>,
    /// Optional Service VLAN tag (S-TAG)
    s_tag: Option<STag>,
}

impl MacAddressesRemoved {
    /// Maximum number of MAC addresses (per spec)
    pub const MAX_ADDRESSES: usize = 255;

    /// Create new MAC Addresses Removed IE without VLAN tags
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
    pub fn new(addresses: Vec<[u8; 6]>) -> Result<Self, PfcpError> {
        Self::new_with_vlan(addresses, None, None)
    }

    /// Create new MAC Addresses Removed IE with VLAN tags
    ///
    /// Per 3GPP TS 29.244 § 8.2.104, C-TAG and S-TAG fields allow multiple instances
    /// of this IE with different VLAN configurations.
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
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    /// use rs_pfcp::ie::c_tag::CTag;
    /// use rs_pfcp::ie::s_tag::STag;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let c_tag = CTag::new(1, false, 100).unwrap();
    /// let s_tag = STag::new(2, false, 200).unwrap();
    ///
    /// let removed = MacAddressesRemoved::new_with_vlan(
    ///     vec![mac],
    ///     Some(c_tag),
    ///     Some(s_tag)
    /// ).unwrap();
    /// assert_eq!(removed.addresses().len(), 1);
    /// assert!(removed.c_tag().is_some());
    /// assert!(removed.s_tag().is_some());
    /// ```
    pub fn new_with_vlan(
        addresses: Vec<[u8; 6]>,
        c_tag: Option<CTag>,
        s_tag: Option<STag>,
    ) -> Result<Self, PfcpError> {
        if addresses.len() > Self::MAX_ADDRESSES {
            return Err(PfcpError::invalid_value(
                "MAC Addresses Removed count",
                addresses.len().to_string(),
                format!("cannot contain more than {} addresses", Self::MAX_ADDRESSES),
            ));
        }
        Ok(MacAddressesRemoved {
            addresses,
            c_tag,
            s_tag,
        })
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
            c_tag: None,
            s_tag: None,
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

    /// Get the optional C-TAG (Customer VLAN tag)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    /// use rs_pfcp::ie::c_tag::CTag;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let c_tag = CTag::new(1, false, 100).unwrap();
    /// let removed = MacAddressesRemoved::new_with_vlan(vec![mac], Some(c_tag), None).unwrap();
    /// assert!(removed.c_tag().is_some());
    /// ```
    pub fn c_tag(&self) -> Option<&CTag> {
        self.c_tag.as_ref()
    }

    /// Get the optional S-TAG (Service VLAN tag)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
    /// use rs_pfcp::ie::s_tag::STag;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let s_tag = STag::new(2, false, 200).unwrap();
    /// let removed = MacAddressesRemoved::new_with_vlan(vec![mac], None, Some(s_tag)).unwrap();
    /// assert!(removed.s_tag().is_some());
    /// ```
    pub fn s_tag(&self) -> Option<&STag> {
        self.s_tag.as_ref()
    }

    /// Marshal MAC Addresses Removed to bytes
    ///
    /// # Returns
    /// Vector containing:
    /// - Count (1 byte)
    /// - MAC addresses (6 bytes each)
    /// - C-TAG length (1 byte) + C-TAG data (3 bytes if present)
    /// - S-TAG length (1 byte) + S-TAG data (3 bytes if present)
    ///
    /// Per 3GPP TS 29.244 § 8.2.104
    pub fn marshal(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Octet 5: Number of MAC addresses
        bytes.push(self.addresses.len() as u8);

        // Octets 6+: MAC address values (6 bytes each)
        for mac in &self.addresses {
            bytes.extend_from_slice(mac);
        }

        // C-TAG length (1 byte) + C-TAG data (3 bytes if present)
        if let Some(ref ctag) = self.c_tag {
            bytes.push(3); // Length of C-TAG field
            bytes.extend_from_slice(&ctag.marshal());
        } else {
            bytes.push(0); // C-TAG field absent
        }

        // S-TAG length (1 byte) + S-TAG data (3 bytes if present)
        if let Some(ref stag) = self.s_tag {
            bytes.push(3); // Length of S-TAG field
            bytes.extend_from_slice(&stag.marshal());
        } else {
            bytes.push(0); // S-TAG field absent
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
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MAC Addresses Removed",
                IeType::MacAddressesRemoved,
                1,
                0,
            ));
        }

        let count = data[0] as usize;
        let mut offset = 1;

        // Parse MAC addresses
        let mut addresses = Vec::with_capacity(count);
        for _ in 0..count {
            if offset + 6 > data.len() {
                return Err(PfcpError::invalid_length(
                    "MAC Addresses Removed MAC address",
                    IeType::MacAddressesRemoved,
                    offset + 6,
                    data.len(),
                ));
            }

            let mut mac = [0u8; 6];
            mac.copy_from_slice(&data[offset..offset + 6]);
            addresses.push(mac);
            offset += 6;
        }

        // Parse C-TAG length and data (if present)
        let c_tag = if offset < data.len() {
            let c_tag_len = data[offset] as usize;
            offset += 1;

            if c_tag_len > 0 {
                if c_tag_len != 3 {
                    return Err(PfcpError::invalid_value(
                        "MAC Addresses Removed C-TAG length",
                        c_tag_len.to_string(),
                        "must be 0 or 3",
                    ));
                }

                if offset + c_tag_len > data.len() {
                    return Err(PfcpError::invalid_length(
                        "MAC Addresses Removed C-TAG",
                        IeType::MacAddressesRemoved,
                        offset + c_tag_len,
                        data.len(),
                    ));
                }

                let ctag = CTag::unmarshal(&data[offset..offset + c_tag_len])?;
                offset += c_tag_len;
                Some(ctag)
            } else {
                None
            }
        } else {
            None
        };

        // Parse S-TAG length and data (if present)
        let s_tag = if offset < data.len() {
            let s_tag_len = data[offset] as usize;
            offset += 1;

            if s_tag_len > 0 {
                if s_tag_len != 3 {
                    return Err(PfcpError::invalid_value(
                        "MAC Addresses Removed S-TAG length",
                        s_tag_len.to_string(),
                        "must be 0 or 3",
                    ));
                }

                if offset + s_tag_len > data.len() {
                    return Err(PfcpError::invalid_length(
                        "MAC Addresses Removed S-TAG",
                        IeType::MacAddressesRemoved,
                        offset + s_tag_len,
                        data.len(),
                    ));
                }

                let stag = STag::unmarshal(&data[offset..offset + s_tag_len])?;
                Some(stag)
            } else {
                None
            }
        } else {
            None
        };

        Ok(MacAddressesRemoved {
            addresses,
            c_tag,
            s_tag,
        })
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
        use crate::error::PfcpError;

        let addresses: Vec<[u8; 6]> = (0..=255).map(|i| [i as u8, 0, 0, 0, 0, 0]).collect();

        let result = MacAddressesRemoved::new(addresses);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_mac_addresses_removed_marshal_empty() {
        let removed = MacAddressesRemoved::empty();
        let bytes = removed.marshal();
        // Per §8.2.104: count (1) + C-TAG length (1) + S-TAG length (1) = 3 bytes
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes[0], 0); // Count = 0
        assert_eq!(bytes[1], 0); // C-TAG length = 0 (absent)
        assert_eq!(bytes[2], 0); // S-TAG length = 0 (absent)
    }

    #[test]
    fn test_mac_addresses_removed_marshal_single() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let removed = MacAddressesRemoved::new(vec![mac]).unwrap();
        let bytes = removed.marshal();

        // Per §8.2.104: count (1) + MAC (6) + C-TAG length (1) + S-TAG length (1) = 9 bytes
        assert_eq!(bytes.len(), 9);
        assert_eq!(bytes[0], 1); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(bytes[7], 0); // C-TAG length = 0 (absent)
        assert_eq!(bytes[8], 0); // S-TAG length = 0 (absent)
    }

    #[test]
    fn test_mac_addresses_removed_marshal_multiple() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let removed = MacAddressesRemoved::new(vec![mac1, mac2]).unwrap();
        let bytes = removed.marshal();

        // Per §8.2.104: count (1) + 2 MACs (12) + C-TAG length (1) + S-TAG length (1) = 15 bytes
        assert_eq!(bytes.len(), 15);
        assert_eq!(bytes[0], 2); // Count
        assert_eq!(&bytes[1..7], &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(&bytes[7..13], &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(bytes[13], 0); // C-TAG length = 0 (absent)
        assert_eq!(bytes[14], 0); // S-TAG length = 0 (absent)
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
        use crate::error::PfcpError;

        let result = MacAddressesRemoved::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
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
        assert_eq!(ie.payload.len(), 9); // Per §8.2.104: includes VLAN tag length fields

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

    #[test]
    fn test_mac_addresses_removed_with_vlan() {
        use crate::ie::c_tag::CTag;
        use crate::ie::s_tag::STag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let c_tag = CTag::new(1, false, 100).unwrap();
        let s_tag = STag::new(2, false, 200).unwrap();

        let removed =
            MacAddressesRemoved::new_with_vlan(vec![mac], Some(c_tag), Some(s_tag)).unwrap();

        assert_eq!(removed.count(), 1);
        assert!(removed.c_tag().is_some());
        assert!(removed.s_tag().is_some());
    }

    #[test]
    fn test_mac_addresses_removed_vlan_round_trip() {
        use crate::ie::c_tag::CTag;
        use crate::ie::s_tag::STag;

        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let c_tag = CTag::new(1, false, 100).unwrap();
        let s_tag = STag::new(2, false, 200).unwrap();

        let original =
            MacAddressesRemoved::new_with_vlan(vec![mac1, mac2], Some(c_tag), Some(s_tag)).unwrap();
        let marshaled = original.marshal();
        let unmarshaled = MacAddressesRemoved::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
        assert_eq!(unmarshaled.count(), 2);
        assert!(unmarshaled.c_tag().is_some());
        assert!(unmarshaled.s_tag().is_some());
    }

    #[test]
    fn test_mac_addresses_removed_c_tag_only() {
        use crate::ie::c_tag::CTag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let c_tag = CTag::new(1, false, 100).unwrap();

        let removed = MacAddressesRemoved::new_with_vlan(vec![mac], Some(c_tag), None).unwrap();

        assert_eq!(removed.count(), 1);
        assert!(removed.c_tag().is_some());
        assert!(removed.s_tag().is_none());

        // Round trip
        let marshaled = removed.marshal();
        let unmarshaled = MacAddressesRemoved::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, removed);
    }

    #[test]
    fn test_mac_addresses_removed_s_tag_only() {
        use crate::ie::s_tag::STag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let s_tag = STag::new(2, false, 200).unwrap();

        let removed = MacAddressesRemoved::new_with_vlan(vec![mac], None, Some(s_tag)).unwrap();

        assert_eq!(removed.count(), 1);
        assert!(removed.c_tag().is_none());
        assert!(removed.s_tag().is_some());

        // Round trip
        let marshaled = removed.marshal();
        let unmarshaled = MacAddressesRemoved::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, removed);
    }

    #[test]
    fn test_mac_addresses_removed_vlan_marshal_format() {
        use crate::ie::c_tag::CTag;
        use crate::ie::s_tag::STag;

        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let c_tag = CTag::new(1, false, 100).unwrap();
        let s_tag = STag::new(2, false, 200).unwrap();

        let removed =
            MacAddressesRemoved::new_with_vlan(vec![mac], Some(c_tag), Some(s_tag)).unwrap();
        let bytes = removed.marshal();

        // Per §8.2.104: count (1) + MAC (6) + C-TAG length (1) + C-TAG (3) + S-TAG length (1) + S-TAG (3) = 15 bytes
        assert_eq!(bytes.len(), 15);
        assert_eq!(bytes[0], 1); // Count
        assert_eq!(&bytes[1..7], &mac); // MAC address
        assert_eq!(bytes[7], 3); // C-TAG length = 3
                                 // bytes[8..11] = C-TAG data (3 bytes)
        assert_eq!(bytes[11], 3); // S-TAG length = 3
                                  // bytes[12..15] = S-TAG data (3 bytes)
    }
}
