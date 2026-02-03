//! Ethernet Packet Filter Information Element
//!
//! The Ethernet Packet Filter IE is a grouped IE that contains Ethernet packet filtering
//! criteria including MAC addresses, Ethertype, VLAN tags, and filter properties.
//! Per 3GPP TS 29.244 Section 7.5.2.2 Table 7.5.2.2-3 (IE type 132), this IE is used
//! to define Ethernet-specific packet detection rules.

use crate::error::PfcpError;
use crate::ie::c_tag::CTag;
use crate::ie::ethernet_filter_id::EthernetFilterId;
use crate::ie::ethernet_filter_properties::EthernetFilterProperties;
use crate::ie::ethertype::Ethertype;
use crate::ie::mac_address::MacAddress;
use crate::ie::s_tag::STag;
use crate::ie::{Ie, IeType};

/// Ethernet Packet Filter (Grouped IE)
///
/// Defines Ethernet packet filtering criteria for a PDR.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 7.5.2.2 Table 7.5.2.2-3 (IE Type 132)
///
/// # Structure (Grouped IE containing):
/// - Ethernet Filter ID (mandatory)
/// - Ethernet Filter Properties (optional)
/// - MAC Address (optional, may appear up to 16 times per 3GPP TS 29.244 Table 7.5.2.2-3)
/// - Ethertype (optional)
/// - C-TAG (optional)
/// - S-TAG (optional)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_packet_filter::{EthernetPacketFilter, EthernetPacketFilterBuilder};
/// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
/// use rs_pfcp::ie::mac_address::MacAddress;
/// use rs_pfcp::ie::ethertype::Ethertype;
///
/// // Simple filter by filter ID only
/// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
///     .build()
///     .unwrap();
///
/// // Filter with MAC address and Ethertype
/// let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// let filter2 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(2))
///     .mac_address(mac)
///     .ethertype(Ethertype::ipv4())
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetPacketFilter {
    /// Ethernet Filter ID (mandatory)
    pub ethernet_filter_id: EthernetFilterId,
    /// Ethernet Filter Properties (optional)
    pub ethernet_filter_properties: Option<EthernetFilterProperties>,
    /// MAC Addresses for filtering (optional, up to 16 per 3GPP TS 29.244)
    pub mac_addresses: Vec<MacAddress>,
    /// Ethertype for filtering (optional)
    pub ethertype: Option<Ethertype>,
    /// C-TAG (Customer VLAN) (optional)
    pub c_tag: Option<CTag>,
    /// S-TAG (Service VLAN) (optional)
    pub s_tag: Option<STag>,
}

impl EthernetPacketFilter {
    /// Create a new Ethernet Packet Filter with mandatory fields
    ///
    /// # Arguments
    /// * `ethernet_filter_id` - Mandatory filter identifier
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilter;
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    ///
    /// let filter = EthernetPacketFilter::new(EthernetFilterId::new(1));
    /// ```
    pub fn new(ethernet_filter_id: EthernetFilterId) -> Self {
        EthernetPacketFilter {
            ethernet_filter_id,
            ethernet_filter_properties: None,
            mac_addresses: Vec::new(),
            ethertype: None,
            c_tag: None,
            s_tag: None,
        }
    }

    /// Marshal Ethernet Packet Filter to bytes (grouped IE format)
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = vec![self.ethernet_filter_id.to_ie()];

        if let Some(props) = &self.ethernet_filter_properties {
            ies.push(props.to_ie());
        }
        // Add all MAC addresses (up to 16 per 3GPP spec)
        for mac in &self.mac_addresses {
            ies.push(mac.to_ie());
        }
        if let Some(etype) = &self.ethertype {
            ies.push(etype.to_ie());
        }
        if let Some(ctag) = &self.c_tag {
            ies.push(ctag.to_ie());
        }
        if let Some(stag) = &self.s_tag {
            ies.push(stag.to_ie());
        }

        let capacity: usize = ies.iter().map(|ie| ie.len() as usize).sum();

        let mut data = Vec::with_capacity(capacity);
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshal Ethernet Packet Filter from bytes
    ///
    /// # Arguments
    /// * `payload` - Grouped IE payload containing child IEs
    ///
    /// # Errors
    /// Returns error if mandatory Ethernet Filter ID is missing
    pub fn unmarshal(payload: &[u8]) -> Result<Self, PfcpError> {
        let mut ethernet_filter_id = None;
        let mut ethernet_filter_properties = None;
        let mut mac_addresses = Vec::new();
        let mut ethertype = None;
        let mut c_tag = None;
        let mut s_tag = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::EthernetFilterId => {
                    ethernet_filter_id = Some(EthernetFilterId::unmarshal(&ie.payload)?);
                }
                IeType::EthernetFilterProperties => {
                    ethernet_filter_properties =
                        Some(EthernetFilterProperties::unmarshal(&ie.payload)?);
                }
                IeType::MacAddress => {
                    // Collect all MAC Address IEs (up to 16 per spec)
                    mac_addresses.push(MacAddress::unmarshal(&ie.payload)?);
                }
                IeType::Ethertype => {
                    ethertype = Some(Ethertype::unmarshal(&ie.payload)?);
                }
                IeType::CTag => {
                    c_tag = Some(CTag::unmarshal(&ie.payload)?);
                }
                IeType::STag => {
                    s_tag = Some(STag::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(EthernetPacketFilter {
            ethernet_filter_id: ethernet_filter_id.ok_or(PfcpError::MissingMandatoryIe {
                ie_type: IeType::EthernetFilterId,
                message_type: None,
                parent_ie: Some(IeType::EthernetPacketFilter),
            })?,
            ethernet_filter_properties,
            mac_addresses,
            ethertype,
            c_tag,
            s_tag,
        })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetPacketFilter, self.marshal())
    }
}

/// Builder for Ethernet Packet Filter
///
/// Provides an ergonomic way to construct Ethernet Packet Filter IEs.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
/// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
/// use rs_pfcp::ie::ethernet_filter_properties::EthernetFilterProperties;
/// use rs_pfcp::ie::mac_address::MacAddress;
/// use rs_pfcp::ie::ethertype::Ethertype;
/// use rs_pfcp::ie::c_tag::CTag;
///
/// // Basic filter
/// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
///     .build()
///     .unwrap();
///
/// // IPv4 traffic filter with MAC address
/// let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(2))
///     .bidirectional()
///     .mac_address(mac)
///     .ethertype(Ethertype::ipv4())
///     .build()
///     .unwrap();
///
/// // VLAN-tagged traffic filter
/// let ctag = CTag::new(3, false, 100).unwrap();
/// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(3))
///     .c_tag(ctag)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct EthernetPacketFilterBuilder {
    ethernet_filter_id: EthernetFilterId,
    ethernet_filter_properties: Option<EthernetFilterProperties>,
    mac_addresses: Vec<MacAddress>,
    ethertype: Option<Ethertype>,
    c_tag: Option<CTag>,
    s_tag: Option<STag>,
}

impl EthernetPacketFilterBuilder {
    /// Create a new builder with mandatory Ethernet Filter ID
    ///
    /// # Arguments
    /// * `ethernet_filter_id` - Mandatory filter identifier
    pub fn new(ethernet_filter_id: EthernetFilterId) -> Self {
        EthernetPacketFilterBuilder {
            ethernet_filter_id,
            ethernet_filter_properties: None,
            mac_addresses: Vec::new(),
            ethertype: None,
            c_tag: None,
            s_tag: None,
        }
    }

    /// Set Ethernet Filter Properties
    pub fn ethernet_filter_properties(mut self, props: EthernetFilterProperties) -> Self {
        self.ethernet_filter_properties = Some(props);
        self
    }

    /// Set bidirectional filtering
    pub fn bidirectional(mut self) -> Self {
        self.ethernet_filter_properties = Some(EthernetFilterProperties::bidirectional());
        self
    }

    /// Set unidirectional filtering
    pub fn unidirectional(mut self) -> Self {
        self.ethernet_filter_properties = Some(EthernetFilterProperties::unidirectional());
        self
    }

    /// Add a MAC address filter
    ///
    /// Can be called multiple times to add up to 16 MAC addresses per 3GPP spec.
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let src_mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    /// let dst_mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    ///
    /// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
    ///     .mac_address(src_mac)
    ///     .mac_address(dst_mac)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn mac_address(mut self, mac: MacAddress) -> Self {
        self.mac_addresses.push(mac);
        self
    }

    /// Set multiple MAC addresses at once
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_packet_filter::EthernetPacketFilterBuilder;
    /// use rs_pfcp::ie::ethernet_filter_id::EthernetFilterId;
    /// use rs_pfcp::ie::mac_address::MacAddress;
    ///
    /// let macs = vec![
    ///     MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
    ///     MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
    /// ];
    ///
    /// let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
    ///     .mac_addresses(macs)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn mac_addresses(mut self, macs: Vec<MacAddress>) -> Self {
        self.mac_addresses = macs;
        self
    }

    /// Set Ethertype filter
    pub fn ethertype(mut self, ethertype: Ethertype) -> Self {
        self.ethertype = Some(ethertype);
        self
    }

    /// Set C-TAG (Customer VLAN) filter
    pub fn c_tag(mut self, c_tag: CTag) -> Self {
        self.c_tag = Some(c_tag);
        self
    }

    /// Set S-TAG (Service VLAN) filter
    pub fn s_tag(mut self, s_tag: STag) -> Self {
        self.s_tag = Some(s_tag);
        self
    }

    /// Build the Ethernet Packet Filter
    ///
    /// # Errors
    /// Returns error if more than 16 MAC addresses are specified (per 3GPP TS 29.244)
    pub fn build(self) -> Result<EthernetPacketFilter, PfcpError> {
        // Validate MAC address count per 3GPP spec
        if self.mac_addresses.len() > 16 {
            return Err(PfcpError::validation_error(
                "EthernetPacketFilterBuilder",
                "mac_addresses",
                format!(
                    "Ethernet Packet Filter can have at most 16 MAC addresses, got {}",
                    self.mac_addresses.len()
                ),
            ));
        }

        Ok(EthernetPacketFilter {
            ethernet_filter_id: self.ethernet_filter_id,
            ethernet_filter_properties: self.ethernet_filter_properties,
            mac_addresses: self.mac_addresses,
            ethertype: self.ethertype,
            c_tag: self.c_tag,
            s_tag: self.s_tag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_packet_filter_new() {
        let filter = EthernetPacketFilter::new(EthernetFilterId::new(1));
        assert_eq!(filter.ethernet_filter_id.value(), 1);
        assert!(filter.ethernet_filter_properties.is_none());
        assert!(filter.mac_addresses.is_empty());
        assert!(filter.ethertype.is_none());
        assert!(filter.c_tag.is_none());
        assert!(filter.s_tag.is_none());
    }

    #[test]
    fn test_ethernet_packet_filter_builder_basic() {
        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .build()
            .unwrap();

        assert_eq!(filter.ethernet_filter_id.value(), 1);
        assert!(filter.ethernet_filter_properties.is_none());
    }

    #[test]
    fn test_ethernet_packet_filter_builder_with_mac() {
        let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(2))
            .mac_address(mac)
            .build()
            .unwrap();

        assert_eq!(filter.ethernet_filter_id.value(), 2);
        assert_eq!(filter.mac_addresses.len(), 1);
        assert_eq!(filter.mac_addresses[0], mac);
    }

    #[test]
    fn test_ethernet_packet_filter_builder_with_ethertype() {
        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(3))
            .ethertype(Ethertype::ipv4())
            .build()
            .unwrap();

        assert_eq!(filter.ethertype, Some(Ethertype::ipv4()));
    }

    #[test]
    fn test_ethernet_packet_filter_builder_bidirectional() {
        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(4))
            .bidirectional()
            .build()
            .unwrap();

        assert!(filter.ethernet_filter_properties.is_some());
        assert!(filter
            .ethernet_filter_properties
            .unwrap()
            .is_bidirectional());
    }

    #[test]
    fn test_ethernet_packet_filter_builder_comprehensive() {
        let mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ctag = CTag::new(5, true, 100).unwrap();

        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(5))
            .bidirectional()
            .mac_address(mac)
            .ethertype(Ethertype::ipv4())
            .c_tag(ctag)
            .build()
            .unwrap();

        assert_eq!(filter.ethernet_filter_id.value(), 5);
        assert!(filter
            .ethernet_filter_properties
            .unwrap()
            .is_bidirectional());
        assert_eq!(filter.mac_addresses.len(), 1);
        assert_eq!(filter.mac_addresses[0], mac);
        assert_eq!(filter.ethertype, Some(Ethertype::ipv4()));
        assert_eq!(filter.c_tag, Some(ctag));
    }

    #[test]
    fn test_ethernet_packet_filter_round_trip_basic() {
        let original = EthernetPacketFilter::new(EthernetFilterId::new(1));
        let marshaled = original.marshal();
        let unmarshaled = EthernetPacketFilter::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_packet_filter_round_trip_with_mac() {
        let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let original = EthernetPacketFilterBuilder::new(EthernetFilterId::new(2))
            .mac_address(mac)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = EthernetPacketFilter::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_packet_filter_round_trip_comprehensive() {
        let mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ctag = CTag::new(3, false, 200).unwrap();
        let stag = STag::new(5, true, 500).unwrap();

        let original = EthernetPacketFilterBuilder::new(EthernetFilterId::new(10))
            .bidirectional()
            .mac_address(mac)
            .ethertype(Ethertype::ipv6())
            .c_tag(ctag)
            .s_tag(stag)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = EthernetPacketFilter::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_packet_filter_unmarshal_missing_filter_id() {
        // Create payload without Filter ID
        let mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let ie = mac.to_ie();
        let marshaled = ie.marshal();

        let result = EthernetPacketFilter::unmarshal(&marshaled);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PfcpError::MissingMandatoryIe { .. }
        ));
    }

    #[test]
    fn test_ethernet_packet_filter_to_ie() {
        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .build()
            .unwrap();

        let ie = filter.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetPacketFilter);

        // Verify IE can be unmarshaled
        let parsed = EthernetPacketFilter::unmarshal(&ie.payload).unwrap();
        assert_eq!(filter, parsed);
    }

    #[test]
    fn test_ethernet_packet_filter_scenarios() {
        // Scenario 1: Simple MAC-based filter
        let mac = MacAddress::source([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        let filter1 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .mac_address(mac)
            .build()
            .unwrap();
        assert_eq!(filter1.mac_addresses.len(), 1);
        assert_eq!(filter1.mac_addresses[0], mac);

        // Scenario 2: IPv4 traffic only
        let filter2 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(2))
            .ethertype(Ethertype::ipv4())
            .build()
            .unwrap();
        assert_eq!(filter2.ethertype, Some(Ethertype::ipv4()));

        // Scenario 3: VLAN-tagged traffic (C-TAG)
        let ctag = CTag::new(3, false, 100).unwrap();
        let filter3 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(3))
            .c_tag(ctag)
            .build()
            .unwrap();
        assert_eq!(filter3.c_tag, Some(ctag));

        // Scenario 4: QinQ double-tagged traffic
        let ctag = CTag::new(3, false, 100).unwrap();
        let stag = STag::new(5, false, 200).unwrap();
        let filter4 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(4))
            .c_tag(ctag)
            .s_tag(stag)
            .build()
            .unwrap();
        assert_eq!(filter4.c_tag, Some(ctag));
        assert_eq!(filter4.s_tag, Some(stag));

        // Scenario 5: Bidirectional MAC + IPv6 filter
        let mac = MacAddress::source([0x00, 0x50, 0x56, 0xC0, 0x00, 0x01]);
        let filter5 = EthernetPacketFilterBuilder::new(EthernetFilterId::new(5))
            .bidirectional()
            .mac_address(mac)
            .ethertype(Ethertype::ipv6())
            .build()
            .unwrap();
        assert!(filter5
            .ethernet_filter_properties
            .unwrap()
            .is_bidirectional());
        assert_eq!(filter5.mac_addresses.len(), 1);
        assert_eq!(filter5.mac_addresses[0], mac);
        assert_eq!(filter5.ethertype, Some(Ethertype::ipv6()));
    }

    #[test]
    fn test_ethernet_packet_filter_multiple_macs() {
        // Test with 2 MAC addresses (source and destination)
        let src_mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let dst_mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .mac_address(src_mac)
            .mac_address(dst_mac)
            .build()
            .unwrap();

        assert_eq!(filter.mac_addresses.len(), 2);
        assert_eq!(filter.mac_addresses[0], src_mac);
        assert_eq!(filter.mac_addresses[1], dst_mac);
    }

    #[test]
    fn test_ethernet_packet_filter_multiple_macs_with_vec() {
        let macs = vec![
            MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
            MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            MacAddress::source([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
        ];

        let filter = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .mac_addresses(macs.clone())
            .build()
            .unwrap();

        assert_eq!(filter.mac_addresses.len(), 3);
        assert_eq!(filter.mac_addresses, macs);
    }

    #[test]
    fn test_ethernet_packet_filter_max_macs() {
        // Test with maximum 16 MAC addresses per 3GPP spec
        let mut builder = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1));

        for i in 0..16 {
            let mac = MacAddress::source([i, i, i, i, i, i]);
            builder = builder.mac_address(mac);
        }

        let filter = builder.build().unwrap();
        assert_eq!(filter.mac_addresses.len(), 16);
    }

    #[test]
    fn test_ethernet_packet_filter_too_many_macs() {
        // Test validation: more than 16 MAC addresses should fail
        let mut macs = Vec::new();
        for i in 0..17 {
            macs.push(MacAddress::source([i, i, i, i, i, i]));
        }

        let result = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .mac_addresses(macs)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("at most 16 MAC addresses"));
    }

    #[test]
    fn test_ethernet_packet_filter_round_trip_multiple_macs() {
        let src_mac = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let dst_mac = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

        let original = EthernetPacketFilterBuilder::new(EthernetFilterId::new(1))
            .bidirectional()
            .mac_address(src_mac)
            .mac_address(dst_mac)
            .ethertype(Ethertype::ipv4())
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = EthernetPacketFilter::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
        assert_eq!(unmarshaled.mac_addresses.len(), 2);
    }
}
