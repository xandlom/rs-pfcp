//! Ethernet Context Information Element
//!
//! The Ethernet Context Information IE is a grouped IE used by SMF to provision MAC addresses
//! to the UPF for Ethernet PDU sessions. Per 3GPP TS 29.244 Section 7.5.4.21 Table 7.5.4.21-1
//! (IE type 254), this IE is used in PFCP Session Modification Request messages.
//!
//! NOTE: This IE contains only MAC Addresses Detected (for provisioning from SMF to UPF).
//! For reporting MAC address events from UPF to SMF, see Ethernet Traffic Information IE (143).

use crate::error::PfcpError;
use crate::ie::mac_addresses_detected::MacAddressesDetected;
use crate::ie::{Ie, IeType};
use std::io;

/// Ethernet Context Information (Grouped IE)
///
/// Used by SMF to provision MAC addresses to UPF for Ethernet PDU sessions.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 7.5.4.21 Table 7.5.4.21-1 (IE Type 254)
///
/// # Structure (Grouped IE containing):
/// - MAC Addresses Detected (mandatory when IE is present, may appear multiple times)
///
/// Per spec: "Several IEs with the same IE type may be present to provision multiple
/// lists of MAC addresses (e.g. with different V-LAN tags)."
///
/// # Direction
/// SMF → UPF (provisioning)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_context_information::{EthernetContextInformation, EthernetContextInformationBuilder};
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
///
/// // Provision single MAC address
/// let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
/// let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
///
/// let context = EthernetContextInformationBuilder::new()
///     .add_mac_addresses_detected(detected)
///     .build()
///     .unwrap();
///
/// // Provision multiple MAC address lists (e.g., different VLANs)
/// let vlan100_macs = MacAddressesDetected::new(vec![
///     [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
/// ]).unwrap();
/// let vlan200_macs = MacAddressesDetected::new(vec![
///     [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
/// ]).unwrap();
///
/// let context2 = EthernetContextInformationBuilder::new()
///     .add_mac_addresses_detected(vlan100_macs)
///     .add_mac_addresses_detected(vlan200_macs)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetContextInformation {
    /// MAC Addresses Detected (mandatory when IE is present, multiple instances allowed)
    /// Per 3GPP TS 29.244 Table 7.5.4.21-1
    pub mac_addresses_detected: Vec<MacAddressesDetected>,
}

impl EthernetContextInformation {
    /// Create a new Ethernet Context Information with MAC addresses
    ///
    /// # Arguments
    /// * `mac_addresses_detected` - List of MAC Addresses Detected IEs (at least one required)
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::ethernet_context_information::EthernetContextInformation;
    /// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
    ///
    /// let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    /// let detected = MacAddressesDetected::new(vec![mac]).unwrap();
    /// let context = EthernetContextInformation::new(vec![detected]);
    /// ```
    pub fn new(mac_addresses_detected: Vec<MacAddressesDetected>) -> Self {
        EthernetContextInformation {
            mac_addresses_detected,
        }
    }

    /// Marshal Ethernet Context Information to bytes (grouped IE format)
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Marshal all MAC Addresses Detected IEs
        for detected in &self.mac_addresses_detected {
            data.extend_from_slice(&detected.to_ie().marshal());
        }

        data
    }

    /// Unmarshal Ethernet Context Information from bytes
    ///
    /// # Arguments
    /// * `payload` - Grouped IE payload containing child IEs
    ///
    /// # Errors
    /// Returns error if no MAC Addresses Detected IE is present (mandatory per spec)
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut mac_addresses_detected = Vec::new();

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::MacAddressesDetected => {
                    mac_addresses_detected.push(MacAddressesDetected::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        // Validate that at least one MAC Addresses Detected IE is present (mandatory per spec)
        if mac_addresses_detected.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Ethernet Context Information requires at least one MAC Addresses Detected IE per 3GPP TS 29.244 Table 7.5.4.21-1",
            ));
        }

        Ok(EthernetContextInformation {
            mac_addresses_detected,
        })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetContextInformation, self.marshal())
    }
}

/// Builder for Ethernet Context Information
///
/// Provides an ergonomic way to construct Ethernet Context Information IEs.
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_context_information::EthernetContextInformationBuilder;
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
///
/// // Provision single MAC address list
/// let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
/// let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
///
/// let context = EthernetContextInformationBuilder::new()
///     .add_mac_addresses_detected(detected)
///     .build()
///     .unwrap();
///
/// // Provision multiple MAC address lists (e.g., different VLANs)
/// let vlan100 = MacAddressesDetected::new(vec![[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]]).unwrap();
/// let vlan200 = MacAddressesDetected::new(vec![[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]]).unwrap();
///
/// let context2 = EthernetContextInformationBuilder::new()
///     .add_mac_addresses_detected(vlan100)
///     .add_mac_addresses_detected(vlan200)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct EthernetContextInformationBuilder {
    mac_addresses_detected: Vec<MacAddressesDetected>,
}

impl EthernetContextInformationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        EthernetContextInformationBuilder {
            mac_addresses_detected: Vec::new(),
        }
    }

    /// Add a MAC Addresses Detected IE
    ///
    /// Can be called multiple times to add multiple MAC address lists
    /// (e.g., for different VLANs)
    pub fn add_mac_addresses_detected(mut self, detected: MacAddressesDetected) -> Self {
        self.mac_addresses_detected.push(detected);
        self
    }

    /// Set all MAC Addresses Detected IEs at once
    pub fn mac_addresses_detected(mut self, detected: Vec<MacAddressesDetected>) -> Self {
        self.mac_addresses_detected = detected;
        self
    }

    /// Build the Ethernet Context Information
    ///
    /// # Errors
    /// Returns error if no MAC Addresses Detected IE has been added (mandatory per spec)
    pub fn build(self) -> Result<EthernetContextInformation, PfcpError> {
        if self.mac_addresses_detected.is_empty() {
            return Err(PfcpError::validation_error(
                "EthernetContextInformationBuilder",
                "mac_addresses_detected",
                "Ethernet Context Information requires at least one MAC Addresses Detected IE per 3GPP TS 29.244 Table 7.5.4.21-1",
            ));
        }

        Ok(EthernetContextInformation {
            mac_addresses_detected: self.mac_addresses_detected,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_context_information_new() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let context = EthernetContextInformation::new(vec![detected.clone()]);

        assert_eq!(context.mac_addresses_detected.len(), 1);
        assert_eq!(context.mac_addresses_detected[0], detected);
    }

    #[test]
    fn test_ethernet_context_information_builder_empty() {
        // Builder should fail when no MAC addresses added (mandatory per spec)
        let result = EthernetContextInformationBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one"));
    }

    #[test]
    fn test_ethernet_context_information_builder_with_detected() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected.clone())
            .build()
            .unwrap();

        assert_eq!(context.mac_addresses_detected.len(), 1);
        assert_eq!(context.mac_addresses_detected[0], detected);
    }

    #[test]
    fn test_ethernet_context_information_builder_multiple_detected() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let detected1 = MacAddressesDetected::new(vec![mac1]).unwrap();
        let detected2 = MacAddressesDetected::new(vec![mac2]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected1.clone())
            .add_mac_addresses_detected(detected2.clone())
            .build()
            .unwrap();

        assert_eq!(context.mac_addresses_detected.len(), 2);
        assert_eq!(context.mac_addresses_detected[0], detected1);
        assert_eq!(context.mac_addresses_detected[1], detected2);
    }

    #[test]
    fn test_ethernet_context_information_unmarshal_empty() {
        // Empty payload should fail (mandatory MAC Addresses Detected per spec)
        let result = EthernetContextInformation::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one"));
    }

    #[test]
    fn test_ethernet_context_information_round_trip_with_detected() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();

        let original = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = EthernetContextInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_context_information_round_trip_comprehensive() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let mac3 = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        let detected1 = MacAddressesDetected::new(vec![mac1, mac2]).unwrap();
        let detected2 = MacAddressesDetected::new(vec![mac3]).unwrap();

        let original = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected1)
            .add_mac_addresses_detected(detected2)
            .build()
            .unwrap();

        let marshaled = original.marshal();
        let unmarshaled = EthernetContextInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_context_information_to_ie() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected)
            .build()
            .unwrap();

        let ie = context.to_ie();
        assert_eq!(ie.ie_type, IeType::EthernetContextInformation);

        // Verify IE can be unmarshaled
        let parsed = EthernetContextInformation::unmarshal(&ie.payload).unwrap();
        assert_eq!(context, parsed);
    }

    #[test]
    fn test_ethernet_context_information_scenarios() {
        // Scenario 1: Provision single MAC address
        let mac = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();
        let context1 = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected)
            .build()
            .unwrap();
        assert_eq!(context1.mac_addresses_detected.len(), 1);

        // Scenario 2: Provision multiple MAC address lists (different VLANs)
        let vlan100_macs = vec![
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            [0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC],
        ];
        let vlan200_macs = vec![[0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22]];

        let detected_vlan100 = MacAddressesDetected::new(vlan100_macs).unwrap();
        let detected_vlan200 = MacAddressesDetected::new(vlan200_macs).unwrap();

        let context2 = EthernetContextInformationBuilder::new()
            .add_mac_addresses_detected(detected_vlan100.clone())
            .add_mac_addresses_detected(detected_vlan200.clone())
            .build()
            .unwrap();

        assert_eq!(context2.mac_addresses_detected.len(), 2);
        assert_eq!(context2.mac_addresses_detected[0].count(), 2);
        assert_eq!(context2.mac_addresses_detected[1].count(), 1);
    }
}
