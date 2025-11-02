//! Ethernet Context Information Element
//!
//! The Ethernet Context Information IE is a grouped IE that provides additional context
//! for Ethernet PDU sessions. Per 3GPP TS 29.244 Section 7.5.4.21 Table 7.5.4.21-1
//! (IE type 254), this is an R18 enhancement for advanced Ethernet session management.

use crate::ie::mac_addresses_detected::MacAddressesDetected;
use crate::ie::mac_addresses_removed::MacAddressesRemoved;
use crate::ie::{Ie, IeType};
use std::io;

/// Ethernet Context Information (Grouped IE)
///
/// Provides additional Ethernet session context including MAC address learning events.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 7.5.4.21 Table 7.5.4.21-1 (IE Type 254) - R18
///
/// # Structure (Grouped IE containing):
/// - MAC Addresses Detected (optional)
/// - MAC Addresses Removed (optional)
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::ethernet_context_information::{EthernetContextInformation, EthernetContextInformationBuilder};
/// use rs_pfcp::ie::mac_addresses_detected::MacAddressesDetected;
/// use rs_pfcp::ie::mac_address::MacAddress;
///
/// // Create with detected MAC addresses
/// let mac1 = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
///
/// let context = EthernetContextInformationBuilder::new()
///     .mac_addresses_detected(detected)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetContextInformation {
    /// MAC Addresses Detected (optional)
    pub mac_addresses_detected: Option<MacAddressesDetected>,
    /// MAC Addresses Removed (optional)
    pub mac_addresses_removed: Option<MacAddressesRemoved>,
}

impl EthernetContextInformation {
    /// Create a new empty Ethernet Context Information
    pub fn new() -> Self {
        EthernetContextInformation {
            mac_addresses_detected: None,
            mac_addresses_removed: None,
        }
    }

    /// Marshal Ethernet Context Information to bytes (grouped IE format)
    pub fn marshal(&self) -> Vec<u8> {
        let mut ies = Vec::new();

        if let Some(detected) = &self.mac_addresses_detected {
            ies.push(detected.to_ie());
        }
        if let Some(removed) = &self.mac_addresses_removed {
            ies.push(removed.to_ie());
        }

        let mut data = Vec::new();
        for ie in ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    /// Unmarshal Ethernet Context Information from bytes
    ///
    /// # Arguments
    /// * `payload` - Grouped IE payload containing child IEs
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut mac_addresses_detected = None;
        let mut mac_addresses_removed = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::MacAddressesDetected => {
                    mac_addresses_detected = Some(MacAddressesDetected::unmarshal(&ie.payload)?);
                }
                IeType::MacAddressesRemoved => {
                    mac_addresses_removed = Some(MacAddressesRemoved::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(EthernetContextInformation {
            mac_addresses_detected,
            mac_addresses_removed,
        })
    }

    /// Convert to generic IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EthernetContextInformation, self.marshal())
    }
}

impl Default for EthernetContextInformation {
    fn default() -> Self {
        Self::new()
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
/// use rs_pfcp::ie::mac_addresses_removed::MacAddressesRemoved;
/// use rs_pfcp::ie::mac_address::MacAddress;
///
/// // Report detected MAC addresses
/// let mac1 = MacAddress::source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
/// let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
///
/// let context = EthernetContextInformationBuilder::new()
///     .mac_addresses_detected(detected)
///     .build()
///     .unwrap();
///
/// // Report removed MAC addresses
/// let mac2 = MacAddress::source([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
/// let removed = MacAddressesRemoved::new(vec![mac2]).unwrap();
///
/// let context2 = EthernetContextInformationBuilder::new()
///     .mac_addresses_removed(removed)
///     .build()
///     .unwrap();
///
/// // Report both detected and removed
/// let mac3 = MacAddress::source([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
/// let mac4 = MacAddress::source([0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC]);
/// let detected2 = MacAddressesDetected::new(vec![mac3]).unwrap();
/// let removed2 = MacAddressesRemoved::new(vec![mac4]).unwrap();
///
/// let context3 = EthernetContextInformationBuilder::new()
///     .mac_addresses_detected(detected2)
///     .mac_addresses_removed(removed2)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct EthernetContextInformationBuilder {
    mac_addresses_detected: Option<MacAddressesDetected>,
    mac_addresses_removed: Option<MacAddressesRemoved>,
}

impl EthernetContextInformationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        EthernetContextInformationBuilder {
            mac_addresses_detected: None,
            mac_addresses_removed: None,
        }
    }

    /// Set MAC addresses detected
    pub fn mac_addresses_detected(mut self, detected: MacAddressesDetected) -> Self {
        self.mac_addresses_detected = Some(detected);
        self
    }

    /// Set MAC addresses removed
    pub fn mac_addresses_removed(mut self, removed: MacAddressesRemoved) -> Self {
        self.mac_addresses_removed = Some(removed);
        self
    }

    /// Build the Ethernet Context Information
    pub fn build(self) -> Result<EthernetContextInformation, io::Error> {
        Ok(EthernetContextInformation {
            mac_addresses_detected: self.mac_addresses_detected,
            mac_addresses_removed: self.mac_addresses_removed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_context_information_new() {
        let context = EthernetContextInformation::new();
        assert!(context.mac_addresses_detected.is_none());
        assert!(context.mac_addresses_removed.is_none());
    }

    #[test]
    fn test_ethernet_context_information_builder_empty() {
        let context = EthernetContextInformationBuilder::new().build().unwrap();
        assert!(context.mac_addresses_detected.is_none());
        assert!(context.mac_addresses_removed.is_none());
    }

    #[test]
    fn test_ethernet_context_information_builder_with_detected() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected.clone())
            .build()
            .unwrap();

        assert_eq!(context.mac_addresses_detected, Some(detected));
        assert!(context.mac_addresses_removed.is_none());
    }

    #[test]
    fn test_ethernet_context_information_builder_with_removed() {
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let removed = MacAddressesRemoved::new(vec![mac]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .mac_addresses_removed(removed.clone())
            .build()
            .unwrap();

        assert!(context.mac_addresses_detected.is_none());
        assert_eq!(context.mac_addresses_removed, Some(removed));
    }

    #[test]
    fn test_ethernet_context_information_builder_comprehensive() {
        let mac1 = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac2 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let detected = MacAddressesDetected::new(vec![mac1]).unwrap();
        let removed = MacAddressesRemoved::new(vec![mac2]).unwrap();

        let context = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected.clone())
            .mac_addresses_removed(removed.clone())
            .build()
            .unwrap();

        assert_eq!(context.mac_addresses_detected, Some(detected));
        assert_eq!(context.mac_addresses_removed, Some(removed));
    }

    #[test]
    fn test_ethernet_context_information_round_trip_empty() {
        let original = EthernetContextInformation::new();
        let marshaled = original.marshal();
        let unmarshaled = EthernetContextInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_ethernet_context_information_round_trip_with_detected() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let detected = MacAddressesDetected::new(vec![mac]).unwrap();

        let original = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected)
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

        let detected = MacAddressesDetected::new(vec![mac1, mac2]).unwrap();
        let removed = MacAddressesRemoved::new(vec![mac3]).unwrap();

        let original = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected)
            .mac_addresses_removed(removed)
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
            .mac_addresses_detected(detected)
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
        // Scenario 1: New MAC address learned
        let new_mac = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let detected = MacAddressesDetected::new(vec![new_mac]).unwrap();
        let context1 = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected)
            .build()
            .unwrap();
        assert!(context1.mac_addresses_detected.is_some());

        // Scenario 2: MAC address aged out
        let old_mac = [0x00, 0x50, 0x56, 0xC0, 0x00, 0x01];
        let removed = MacAddressesRemoved::new(vec![old_mac]).unwrap();
        let context2 = EthernetContextInformationBuilder::new()
            .mac_addresses_removed(removed)
            .build()
            .unwrap();
        assert!(context2.mac_addresses_removed.is_some());

        // Scenario 3: MAC address table update (both detected and removed)
        let new_devices = vec![
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            [0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC],
        ];
        let old_devices = vec![[0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22]];

        let detected = MacAddressesDetected::new(new_devices).unwrap();
        let removed = MacAddressesRemoved::new(old_devices).unwrap();

        let context3 = EthernetContextInformationBuilder::new()
            .mac_addresses_detected(detected)
            .mac_addresses_removed(removed)
            .build()
            .unwrap();

        assert!(context3.mac_addresses_detected.is_some());
        assert!(context3.mac_addresses_removed.is_some());
        assert_eq!(context3.mac_addresses_detected.unwrap().count(), 2);
        assert_eq!(context3.mac_addresses_removed.unwrap().count(), 1);
    }

    #[test]
    fn test_ethernet_context_information_default() {
        let context = EthernetContextInformation::default();
        assert!(context.mac_addresses_detected.is_none());
        assert!(context.mac_addresses_removed.is_none());
    }
}
