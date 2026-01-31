//! TSN Bridge ID Information Element
//!
//! The TSN Bridge ID IE is used to identify a TSN bridge in Time-Sensitive Networking scenarios.
//! This is critical for industrial IoT applications requiring deterministic networking.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// TSN Bridge ID Information Element
///
/// Used in TSN (Time-Sensitive Networking) scenarios to identify bridges
/// in deterministic networking topologies for industrial IoT applications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsnBridgeId {
    /// Bridge ID (6 bytes MAC address format)
    pub bridge_id: [u8; 6],
}

impl TsnBridgeId {
    /// Creates a new TSN Bridge ID
    pub fn new(bridge_id: [u8; 6]) -> Self {
        Self { bridge_id }
    }

    /// Creates TSN Bridge ID from MAC address bytes
    pub fn from_mac(mac: [u8; 6]) -> Self {
        Self::new(mac)
    }

    /// Marshal to bytes
    pub fn marshal(&self) -> Vec<u8> {
        self.bridge_id.to_vec()
    }

    /// Unmarshal from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() != 6 {
            return Err(PfcpError::invalid_length("TSN Bridge ID", IeType::TsnBridgeIdAdvanced, 6, data.len()));
        }

        let mut bridge_id = [0u8; 6];
        bridge_id.copy_from_slice(data);

        Ok(Self { bridge_id })
    }
}

impl From<TsnBridgeId> for Ie {
    fn from(ie: TsnBridgeId) -> Self {
        Ie::new(IeType::TsnBridgeIdAdvanced, ie.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsn_bridge_id_marshal_unmarshal() {
        let bridge_id = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let tsn_bridge = TsnBridgeId::new(bridge_id);
        
        let marshaled = tsn_bridge.marshal();
        assert_eq!(marshaled.len(), 6);
        assert_eq!(marshaled, bridge_id);
        
        let unmarshaled = TsnBridgeId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, tsn_bridge);
    }

    #[test]
    fn test_tsn_bridge_id_from_mac() {
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let tsn_bridge = TsnBridgeId::from_mac(mac);
        
        assert_eq!(tsn_bridge.bridge_id, mac);
    }

    #[test]
    fn test_tsn_bridge_id_invalid_length() {
        let invalid_data = vec![0x00, 0x11, 0x22]; // Too short
        assert!(TsnBridgeId::unmarshal(&invalid_data).is_err());
        
        let invalid_data = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66]; // Too long
        assert!(TsnBridgeId::unmarshal(&invalid_data).is_err());
    }

    #[test]
    fn test_tsn_bridge_id_into_ie() {
        let bridge_id = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
        let tsn_bridge = TsnBridgeId::new(bridge_id);
        let ie: Ie = tsn_bridge.into();
        
        assert_eq!(ie.ie_type, IeType::TsnBridgeIdAdvanced);
        assert_eq!(ie.payload, bridge_id);
    }
}
