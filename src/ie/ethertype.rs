//! Ethertype Information Element
//!
//! The Ethertype IE contains the Ethernet frame type field for packet filtering.
//! Per 3GPP TS 29.244 Section 8.2.96, this IE is used in Ethernet packet filtering scenarios.
//!
//! Common ethertypes include IPv4 (0x0800), IPv6 (0x86DD), ARP (0x0806), VLAN (0x8100).

use crate::ie::{Ie, IeType};
use std::io;

/// Ethertype
///
/// Represents the Ethernet frame type field for packet classification.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.96
///
/// # Structure
/// - 2 octets: Ethertype value in network byte order
///
/// # Common Ethertypes
/// - 0x0800: IPv4
/// - 0x0806: ARP
/// - 0x8100: VLAN-tagged frame (IEEE 802.1Q)
/// - 0x86DD: IPv6
/// - 0x8847: MPLS unicast
/// - 0x8848: MPLS multicast
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethertype::Ethertype;
///
/// // Create Ethertype for IPv4
/// let ipv4 = Ethertype::new(0x0800);
/// assert_eq!(ipv4.value(), 0x0800);
///
/// // Create Ethertype for IPv6
/// let ipv6 = Ethertype::new(0x86DD);
/// assert_eq!(ipv6.value(), 0x86DD);
///
/// // Marshal and unmarshal
/// let bytes = ipv4.marshal();
/// let parsed = Ethertype::unmarshal(&bytes).unwrap();
/// assert_eq!(ipv4, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ethertype {
    /// Ethertype value (16-bit)
    value: u16,
}

impl Ethertype {
    /// IPv4 ethertype (0x0800)
    pub const IPV4: u16 = 0x0800;
    /// ARP ethertype (0x0806)
    pub const ARP: u16 = 0x0806;
    /// VLAN-tagged frame (0x8100)
    pub const VLAN: u16 = 0x8100;
    /// IPv6 ethertype (0x86DD)
    pub const IPV6: u16 = 0x86DD;
    /// MPLS unicast (0x8847)
    pub const MPLS_UNICAST: u16 = 0x8847;
    /// MPLS multicast (0x8848)
    pub const MPLS_MULTICAST: u16 = 0x8848;

    /// Create a new Ethertype
    ///
    /// # Arguments
    /// * `value` - Ethertype value (16-bit)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let ethertype = Ethertype::new(0x0800); // IPv4
    /// assert_eq!(ethertype.value(), 0x0800);
    /// ```
    pub fn new(value: u16) -> Self {
        Ethertype { value }
    }

    /// Create IPv4 ethertype
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let ipv4 = Ethertype::ipv4();
    /// assert_eq!(ipv4.value(), 0x0800);
    /// ```
    pub fn ipv4() -> Self {
        Ethertype::new(Self::IPV4)
    }

    /// Create IPv6 ethertype
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let ipv6 = Ethertype::ipv6();
    /// assert_eq!(ipv6.value(), 0x86DD);
    /// ```
    pub fn ipv6() -> Self {
        Ethertype::new(Self::IPV6)
    }

    /// Create ARP ethertype
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let arp = Ethertype::arp();
    /// assert_eq!(arp.value(), 0x0806);
    /// ```
    pub fn arp() -> Self {
        Ethertype::new(Self::ARP)
    }

    /// Get the ethertype value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let ethertype = Ethertype::new(0x8100);
    /// assert_eq!(ethertype.value(), 0x8100);
    /// ```
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Marshal Ethertype to bytes
    ///
    /// # Returns
    /// 2-byte array with ethertype in network byte order
    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    /// Unmarshal Ethertype from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing Ethertype data (must be at least 2 bytes)
    ///
    /// # Errors
    /// Returns error if data is too short
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    ///
    /// let ethertype = Ethertype::new(0x0800);
    /// let bytes = ethertype.marshal();
    /// let parsed = Ethertype::unmarshal(&bytes).unwrap();
    /// assert_eq!(ethertype, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Ethertype requires 2 bytes, got {}", data.len()),
            ));
        }

        let value = u16::from_be_bytes([data[0], data[1]]);
        Ok(Ethertype { value })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethertype::Ethertype;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let ethertype = Ethertype::ipv4();
    /// let ie = ethertype.to_ie();
    /// assert_eq!(ie.ie_type, IeType::Ethertype);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Ethertype, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethertype_new() {
        let ethertype = Ethertype::new(0x0800);
        assert_eq!(ethertype.value(), 0x0800);
    }

    #[test]
    fn test_ethertype_ipv4() {
        let ipv4 = Ethertype::ipv4();
        assert_eq!(ipv4.value(), 0x0800);
    }

    #[test]
    fn test_ethertype_ipv6() {
        let ipv6 = Ethertype::ipv6();
        assert_eq!(ipv6.value(), 0x86DD);
    }

    #[test]
    fn test_ethertype_arp() {
        let arp = Ethertype::arp();
        assert_eq!(arp.value(), 0x0806);
    }

    #[test]
    fn test_ethertype_marshal() {
        let ethertype = Ethertype::new(0x0800);
        let bytes = ethertype.marshal();
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes, [0x08, 0x00]);

        let ethertype2 = Ethertype::new(0x86DD);
        let bytes2 = ethertype2.marshal();
        assert_eq!(bytes2, [0x86, 0xDD]);
    }

    #[test]
    fn test_ethertype_unmarshal_valid() {
        let data = [0x08, 0x00]; // IPv4
        let ethertype = Ethertype::unmarshal(&data).unwrap();
        assert_eq!(ethertype.value(), 0x0800);

        let data2 = [0x86, 0xDD]; // IPv6
        let ethertype2 = Ethertype::unmarshal(&data2).unwrap();
        assert_eq!(ethertype2.value(), 0x86DD);
    }

    #[test]
    fn test_ethertype_unmarshal_short() {
        let data = [0x08];
        let result = Ethertype::unmarshal(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_ethertype_unmarshal_empty() {
        let result = Ethertype::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 2 bytes"));
        assert!(err.to_string().contains("got 0"));
    }

    #[test]
    fn test_ethertype_round_trip() {
        let test_cases = vec![
            0x0800, // IPv4
            0x0806, // ARP
            0x8100, // VLAN
            0x86DD, // IPv6
            0x8847, // MPLS unicast
            0x8848, // MPLS multicast
            0xFFFF, // Max value
            0x0000, // Min value
        ];

        for value in test_cases {
            let original = Ethertype::new(value);
            let marshaled = original.marshal();
            let unmarshaled = Ethertype::unmarshal(&marshaled).unwrap();
            assert_eq!(
                original, unmarshaled,
                "Failed for ethertype 0x{:04X}",
                value
            );
        }
    }

    #[test]
    fn test_ethertype_to_ie() {
        let ethertype = Ethertype::ipv4();
        let ie = ethertype.to_ie();
        assert_eq!(ie.ie_type, IeType::Ethertype);
        assert_eq!(ie.payload.len(), 2);
        assert_eq!(ie.payload, vec![0x08, 0x00]);

        // Verify IE can be unmarshaled
        let parsed = Ethertype::unmarshal(&ie.payload).unwrap();
        assert_eq!(ethertype, parsed);
    }

    #[test]
    fn test_ethertype_scenarios() {
        // Scenario 1: IPv4 traffic filtering
        let ipv4_filter = Ethertype::ipv4();
        assert_eq!(ipv4_filter.value(), 0x0800);

        // Scenario 2: IPv6 traffic filtering
        let ipv6_filter = Ethertype::ipv6();
        assert_eq!(ipv6_filter.value(), 0x86DD);

        // Scenario 3: ARP filtering
        let arp_filter = Ethertype::arp();
        assert_eq!(arp_filter.value(), 0x0806);

        // Scenario 4: VLAN-tagged frames
        let vlan_filter = Ethertype::new(Ethertype::VLAN);
        assert_eq!(vlan_filter.value(), 0x8100);

        // Scenario 5: MPLS traffic
        let mpls_filter = Ethertype::new(Ethertype::MPLS_UNICAST);
        assert_eq!(mpls_filter.value(), 0x8847);
    }

    #[test]
    fn test_ethertype_constants() {
        assert_eq!(Ethertype::IPV4, 0x0800);
        assert_eq!(Ethertype::ARP, 0x0806);
        assert_eq!(Ethertype::VLAN, 0x8100);
        assert_eq!(Ethertype::IPV6, 0x86DD);
        assert_eq!(Ethertype::MPLS_UNICAST, 0x8847);
        assert_eq!(Ethertype::MPLS_MULTICAST, 0x8848);
    }

    #[test]
    fn test_ethertype_clone_copy() {
        let eth1 = Ethertype::new(0x0800);
        let eth2 = eth1;
        assert_eq!(eth1, eth2);

        let eth3 = eth1;
        assert_eq!(eth1, eth3);
    }
}
