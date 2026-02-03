//! Ethernet Traffic Information Grouped IE
//!
//! The Ethernet Traffic Information IE is a grouped IE used for UPF → SMF reporting
//! of MAC learning events within Usage Reports. Per 3GPP TS 29.244 Table 7.5.8.3-3,
//! it contains MAC addresses detected and/or removed during an Ethernet PDU session.

use crate::error::PfcpError;
use crate::ie::{
    mac_addresses_detected::MacAddressesDetected, mac_addresses_removed::MacAddressesRemoved, Ie,
    IeType,
};

/// Ethernet Traffic Information
///
/// Grouped IE for reporting MAC address learning events (detected/removed) from UPF to SMF.
/// Used within Usage Report IE for Ethernet PDU sessions.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Table 7.5.8.3-3 (Ethernet Traffic Information IE within Usage Report IE)
///
/// # Structure
/// - MAC Addresses Detected (0+ instances) - Multiple lists with different VLAN tags
/// - MAC Addresses Removed (0+ instances) - Multiple lists with different VLAN tags
/// - At least one IE (Detected or Removed) must be present
///
/// # Flow Direction
/// UPF → SMF: Reports MAC learning events in Usage Reports
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_traffic_information::EthernetTrafficInformationBuilder;
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
/// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
///
/// // Report detected MACs only
/// let detected = MacAddressesDetected::new(vec![
///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
/// ]).unwrap();
///
/// let info = EthernetTrafficInformationBuilder::new()
///     .add_mac_addresses_detected(detected)
///     .build()
///     .unwrap();
///
/// // Report both detected and removed MACs
/// let detected2 = MacAddressesDetected::new(vec![
///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
/// ]).unwrap();
/// let removed = MacAddressesRemoved::new(vec![
///     [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]
/// ]).unwrap();
///
/// let info2 = EthernetTrafficInformationBuilder::new()
///     .add_mac_addresses_detected(detected2)
///     .add_mac_addresses_removed(removed)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetTrafficInformation {
    /// MAC Addresses Detected (multiple instances allowed for different VLAN tags)
    pub mac_addresses_detected: Vec<MacAddressesDetected>,
    /// MAC Addresses Removed (multiple instances allowed for different VLAN tags)
    pub mac_addresses_removed: Vec<MacAddressesRemoved>,
}

impl EthernetTrafficInformation {
    /// Create new Ethernet Traffic Information with detected and/or removed MAC addresses
    ///
    /// # Arguments
    /// * `mac_addresses_detected` - List of MAC Addresses Detected IEs (can be empty)
    /// * `mac_addresses_removed` - List of MAC Addresses Removed IEs (can be empty)
    ///
    /// # Errors
    /// Returns error if both lists are empty (at least one IE must be present)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_traffic_information::EthernetTrafficInformation;
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    ///
    /// let detected = MacAddressesDetected::new(vec![
    ///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
    /// ]).unwrap();
    ///
    /// let info = EthernetTrafficInformation::new(
    ///     vec![detected],
    ///     vec![]
    /// ).unwrap();
    /// ```
    pub fn new(
        mac_addresses_detected: Vec<MacAddressesDetected>,
        mac_addresses_removed: Vec<MacAddressesRemoved>,
    ) -> Result<Self, PfcpError> {
        if mac_addresses_detected.is_empty() && mac_addresses_removed.is_empty() {
            return Err(PfcpError::validation_error(
                "EthernetTrafficInformation",
                "mac_addresses",
                "Ethernet Traffic Information requires at least one MAC Addresses Detected or Removed IE (per 3GPP TS 29.244 Table 7.5.8.3-3)",
            ));
        }

        Ok(EthernetTrafficInformation {
            mac_addresses_detected,
            mac_addresses_removed,
        })
    }

    /// Marshal Ethernet Traffic Information to bytes (grouped IE format)
    ///
    /// # Returns
    /// Vector containing all child IEs marshaled with their TLV headers
    pub fn marshal(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Marshal all MAC Addresses Detected IEs
        for detected in &self.mac_addresses_detected {
            let ie = detected.to_ie();
            bytes.extend_from_slice(&ie.marshal());
        }

        // Marshal all MAC Addresses Removed IEs
        for removed in &self.mac_addresses_removed {
            let ie = removed.to_ie();
            bytes.extend_from_slice(&ie.marshal());
        }

        bytes
    }

    /// Unmarshal Ethernet Traffic Information from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing child IEs
    ///
    /// # Errors
    /// Returns error if:
    /// - Data is malformed
    /// - No MAC Addresses Detected or Removed IEs found
    /// - Child IEs cannot be parsed
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let mut mac_addresses_detected = Vec::new();
        let mut mac_addresses_removed = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            // Parse IE header (Type: u16, Length: u16)
            if offset + 4 > data.len() {
                return Err(PfcpError::invalid_length(
                    "Ethernet Traffic Information IE header",
                    IeType::EthernetTrafficInformation,
                    offset + 4,
                    data.len(),
                ));
            }

            let ie_type_raw = u16::from_be_bytes([data[offset], data[offset + 1]]);
            let ie_len = u16::from_be_bytes([data[offset + 2], data[offset + 3]]) as usize;
            offset += 4;

            if offset + ie_len > data.len() {
                return Err(PfcpError::invalid_length(
                    "Ethernet Traffic Information child IE",
                    IeType::EthernetTrafficInformation,
                    offset + ie_len,
                    data.len(),
                ));
            }

            let ie_data = &data[offset..offset + ie_len];

            match ie_type_raw {
                144 => {
                    // MAC Addresses Detected
                    let detected = MacAddressesDetected::unmarshal(ie_data)?;
                    mac_addresses_detected.push(detected);
                }
                145 => {
                    // MAC Addresses Removed
                    let removed = MacAddressesRemoved::unmarshal(ie_data)?;
                    mac_addresses_removed.push(removed);
                }
                _ => {
                    // Ignore unknown IEs (forward compatibility)
                }
            }

            offset += ie_len;
        }

        // Validate at least one IE present
        if mac_addresses_detected.is_empty() && mac_addresses_removed.is_empty() {
            return Err(PfcpError::validation_error(
                "EthernetTrafficInformation",
                "mac_addresses",
                "Ethernet Traffic Information requires at least one MAC Addresses Detected or Removed IE",
            ));
        }

        Ok(EthernetTrafficInformation {
            mac_addresses_detected,
            mac_addresses_removed,
        })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_traffic_information::EthernetTrafficInformation;
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let detected = MacAddressesDetected::new(vec![
    ///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
    /// ]).unwrap();
    ///
    /// let info = EthernetTrafficInformation::new(vec![detected], vec![]).unwrap();
    /// let ie = info.to_ie();
    /// assert_eq!(ie.ie_type, IeType::EthernetTrafficInformation);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetTrafficInformation, self.marshal())
    }
}

/// Builder for Ethernet Traffic Information
///
/// Provides ergonomic API for constructing Ethernet Traffic Information IEs.
///
/// # Example
/// ```
/// use rs_pfcp::ie::ethernet_traffic_information::EthernetTrafficInformationBuilder;
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
/// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
///
/// let detected = MacAddressesDetected::new(vec![
///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
/// ]).unwrap();
/// let removed = MacAddressesRemoved::new(vec![
///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
/// ]).unwrap();
///
/// let info = EthernetTrafficInformationBuilder::new()
///     .add_mac_addresses_detected(detected)
///     .add_mac_addresses_removed(removed)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct EthernetTrafficInformationBuilder {
    mac_addresses_detected: Vec<MacAddressesDetected>,
    mac_addresses_removed: Vec<MacAddressesRemoved>,
}

impl EthernetTrafficInformationBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a MAC Addresses Detected IE
    ///
    /// Can be called multiple times to add multiple lists (e.g., with different VLAN tags).
    pub fn add_mac_addresses_detected(mut self, detected: MacAddressesDetected) -> Self {
        self.mac_addresses_detected.push(detected);
        self
    }

    /// Add a MAC Addresses Removed IE
    ///
    /// Can be called multiple times to add multiple lists (e.g., with different VLAN tags).
    pub fn add_mac_addresses_removed(mut self, removed: MacAddressesRemoved) -> Self {
        self.mac_addresses_removed.push(removed);
        self
    }

    /// Build Ethernet Traffic Information
    ///
    /// # Errors
    /// Returns error if no MAC Addresses Detected or Removed IEs were added
    pub fn build(self) -> Result<EthernetTrafficInformation, PfcpError> {
        EthernetTrafficInformation::new(self.mac_addresses_detected, self.mac_addresses_removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_traffic_information_detected_only() {
        let detected =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();

        let info = EthernetTrafficInformation::new(vec![detected.clone()], vec![]).unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 1);
        assert_eq!(info.mac_addresses_removed.len(), 0);
        assert_eq!(info.mac_addresses_detected[0], detected);
    }

    #[test]
    fn test_ethernet_traffic_information_removed_only() {
        let removed = MacAddressesRemoved::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info = EthernetTrafficInformation::new(vec![], vec![removed.clone()]).unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 0);
        assert_eq!(info.mac_addresses_removed.len(), 1);
        assert_eq!(info.mac_addresses_removed[0], removed);
    }

    #[test]
    fn test_ethernet_traffic_information_both() {
        let detected =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
        let removed = MacAddressesRemoved::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info =
            EthernetTrafficInformation::new(vec![detected.clone()], vec![removed.clone()]).unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 1);
        assert_eq!(info.mac_addresses_removed.len(), 1);
        assert_eq!(info.mac_addresses_detected[0], detected);
        assert_eq!(info.mac_addresses_removed[0], removed);
    }

    #[test]
    fn test_ethernet_traffic_information_empty_invalid() {
        let result = EthernetTrafficInformation::new(vec![], vec![]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("at least one MAC Addresses"));
    }

    #[test]
    fn test_ethernet_traffic_information_multiple_detected() {
        let detected1 =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
        let detected2 =
            MacAddressesDetected::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info =
            EthernetTrafficInformation::new(vec![detected1.clone(), detected2.clone()], vec![])
                .unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 2);
        assert_eq!(info.mac_addresses_removed.len(), 0);
    }

    #[test]
    fn test_ethernet_traffic_information_round_trip() {
        let detected = MacAddressesDetected::new(vec![
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            [0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
        ])
        .unwrap();
        let removed = MacAddressesRemoved::new(vec![[0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11]]).unwrap();

        let original =
            EthernetTrafficInformation::new(vec![detected.clone()], vec![removed.clone()]).unwrap();
        let marshaled = original.marshal();
        let unmarshaled = EthernetTrafficInformation::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled, original);
    }

    #[test]
    fn test_ethernet_traffic_information_unmarshal_empty() {
        let result = EthernetTrafficInformation::unmarshal(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_traffic_information_to_ie() {
        let detected =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
        let info = EthernetTrafficInformation::new(vec![detected], vec![]).unwrap();
        let ie = info.to_ie();

        assert_eq!(ie.ie_type, IeType::EthernetTrafficInformation);

        // Verify IE can be unmarshaled
        let parsed = EthernetTrafficInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_builder_detected_only() {
        let detected =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();

        let info = EthernetTrafficInformationBuilder::new()
            .add_mac_addresses_detected(detected.clone())
            .build()
            .unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 1);
        assert_eq!(info.mac_addresses_removed.len(), 0);
    }

    #[test]
    fn test_builder_both() {
        let detected =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
        let removed = MacAddressesRemoved::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info = EthernetTrafficInformationBuilder::new()
            .add_mac_addresses_detected(detected.clone())
            .add_mac_addresses_removed(removed.clone())
            .build()
            .unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 1);
        assert_eq!(info.mac_addresses_removed.len(), 1);
    }

    #[test]
    fn test_builder_empty_invalid() {
        let result = EthernetTrafficInformationBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_multiple_detected() {
        let detected1 =
            MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
        let detected2 =
            MacAddressesDetected::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info = EthernetTrafficInformationBuilder::new()
            .add_mac_addresses_detected(detected1)
            .add_mac_addresses_detected(detected2)
            .build()
            .unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 2);
    }

    #[test]
    fn test_scenario_vlan_tagged_macs() {
        use crate::ie::c_tag::CTag;
        use crate::ie::s_tag::STag;

        // Scenario: Report MACs with different VLAN tags
        let c_tag = CTag::new(1, false, 100).unwrap();
        let s_tag = STag::new(2, false, 200).unwrap();

        let detected_vlan100 = MacAddressesDetected::new_with_vlan(
            vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]],
            Some(c_tag),
            Some(s_tag),
        )
        .unwrap();

        let detected_no_vlan =
            MacAddressesDetected::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();

        let info = EthernetTrafficInformationBuilder::new()
            .add_mac_addresses_detected(detected_vlan100)
            .add_mac_addresses_detected(detected_no_vlan)
            .build()
            .unwrap();

        assert_eq!(info.mac_addresses_detected.len(), 2);

        // Verify VLAN tags
        assert!(info.mac_addresses_detected[0].c_tag().is_some());
        assert!(info.mac_addresses_detected[0].s_tag().is_some());
        assert!(info.mac_addresses_detected[1].c_tag().is_none());
        assert!(info.mac_addresses_detected[1].s_tag().is_none());
    }
}
