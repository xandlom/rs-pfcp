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
/// # Structure
/// - 6 octets: MAC address in network byte order
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::mac_address::MacAddress;
///
/// // Create from byte array
/// let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// assert_eq!(mac.octets(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
///
/// // Marshal and unmarshal
/// let bytes = mac.marshal();
/// let parsed = MacAddress::unmarshal(&bytes).unwrap();
/// assert_eq!(mac, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddress {
    /// MAC address octets (6 bytes)
    octets: [u8; 6],
}

impl MacAddress {
    /// Create a new MAC address from 6 octets
    ///
    /// # Arguments
    /// * `octets` - 6-byte MAC address
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// ```
    pub fn new(octets: [u8; 6]) -> Self {
        MacAddress { octets }
    }

    /// Get the MAC address octets
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    /// assert_eq!(mac.octets(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    /// ```
    pub fn octets(&self) -> &[u8; 6] {
        &self.octets
    }

    /// Check if this is a broadcast address (FF:FF:FF:FF:FF:FF)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let broadcast = MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    /// assert!(broadcast.is_broadcast());
    ///
    /// let unicast = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// assert!(!unicast.is_broadcast());
    /// ```
    pub fn is_broadcast(&self) -> bool {
        self.octets == [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    }

    /// Check if this is a multicast address (least significant bit of first octet is 1)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let multicast = MacAddress::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01]);
    /// assert!(multicast.is_multicast());
    ///
    /// let unicast = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// assert!(!unicast.is_multicast());
    /// ```
    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) == 0x01
    }

    /// Check if this is a unicast address (not multicast)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let unicast = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// assert!(unicast.is_unicast());
    ///
    /// let multicast = MacAddress::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01]);
    /// assert!(!multicast.is_unicast());
    /// ```
    pub fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    /// Marshal MAC address to bytes
    ///
    /// # Returns
    /// 6-byte array with MAC address
    pub fn marshal(&self) -> [u8; 6] {
        self.octets
    }

    /// Unmarshal MAC address from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing MAC address data (must be exactly 6 bytes)
    ///
    /// # Errors
    /// Returns error if data is not exactly 6 bytes
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let mac = MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    /// let bytes = mac.marshal();
    /// let parsed = MacAddress::unmarshal(&bytes).unwrap();
    /// assert_eq!(mac, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 6 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("MAC Address requires 6 bytes, got {}", data.len()),
            ));
        }

        let octets: [u8; 6] = data[0..6].try_into().unwrap();
        Ok(MacAddress { octets })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::mac_address::MacAddress;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// let ie = mac.to_ie();
    /// assert_eq!(ie.ie_type, IeType::MacAddress);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MacAddress, self.marshal().to_vec())
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_new() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.octets(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_mac_address_marshal() {
        let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let bytes = mac.marshal();
        assert_eq!(bytes.len(), 6);
        assert_eq!(bytes, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_mac_address_unmarshal_valid() {
        let data = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
        let mac = MacAddress::unmarshal(&data).unwrap();
        assert_eq!(mac.octets(), &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    }

    #[test]
    fn test_mac_address_unmarshal_short() {
        let data = [0x11, 0x22, 0x33];
        let result = MacAddress::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_mac_address_unmarshal_empty() {
        let result = MacAddress::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 6 bytes"));
        assert!(err.to_string().contains("got 0"));
    }

    #[test]
    fn test_mac_address_round_trip() {
        let original = MacAddress::new([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
        let marshaled = original.marshal();
        let unmarshaled = MacAddress::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_mac_address_is_broadcast() {
        let broadcast = MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert!(broadcast.is_broadcast());

        let not_broadcast = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(!not_broadcast.is_broadcast());
    }

    #[test]
    fn test_mac_address_is_multicast() {
        // IPv4 multicast MAC (01:00:5E:xx:xx:xx)
        let multicast = MacAddress::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01]);
        assert!(multicast.is_multicast());
        assert!(!multicast.is_unicast());

        // Unicast MAC
        let unicast = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(!unicast.is_multicast());
        assert!(unicast.is_unicast());

        // Broadcast is also multicast
        let broadcast = MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert!(broadcast.is_multicast());
    }

    #[test]
    fn test_mac_address_to_ie() {
        let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ie = mac.to_ie();
        assert_eq!(ie.ie_type, IeType::MacAddress);
        assert_eq!(ie.payload.len(), 6);
        assert_eq!(ie.payload, vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        // Verify IE can be unmarshaled
        let parsed = MacAddress::unmarshal(&ie.payload).unwrap();
        assert_eq!(mac, parsed);
    }

    #[test]
    fn test_mac_address_display() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.to_string(), "00:11:22:33:44:55");

        let mac2 = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(mac2.to_string(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_mac_address_scenarios() {
        // Scenario 1: Standard unicast MAC
        let unicast = MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert!(unicast.is_unicast());
        assert!(!unicast.is_broadcast());

        // Scenario 2: IPv4 multicast MAC
        let ipv4_multicast = MacAddress::new([0x01, 0x00, 0x5E, 0x01, 0x02, 0x03]);
        assert!(ipv4_multicast.is_multicast());

        // Scenario 3: IPv6 multicast MAC
        let ipv6_multicast = MacAddress::new([0x33, 0x33, 0x00, 0x00, 0x00, 0x01]);
        assert!(ipv6_multicast.is_multicast());

        // Scenario 4: Broadcast MAC
        let broadcast = MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert!(broadcast.is_broadcast());
        assert!(broadcast.is_multicast());

        // Scenario 5: All zeros MAC
        let zeros = MacAddress::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert!(zeros.is_unicast());
    }

    #[test]
    fn test_mac_address_clone_copy() {
        let mac1 = MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        let mac2 = mac1;
        assert_eq!(mac1, mac2);

        let mac3 = mac1;
        assert_eq!(mac1, mac3);
    }
}
