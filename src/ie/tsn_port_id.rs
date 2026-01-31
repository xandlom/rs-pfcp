//! TSN Port ID Information Element
//!
//! The TSN Port ID IE identifies a specific port on a TSN bridge for
//! Time-Sensitive Networking applications requiring precise traffic control.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// TSN Port ID Information Element
///
/// Identifies a specific port on a TSN bridge for deterministic networking.
/// Critical for industrial IoT applications with strict timing requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsnPortId {
    /// Port ID (2 bytes)
    pub port_id: u16,
}

impl TsnPortId {
    /// Creates a new TSN Port ID
    pub fn new(port_id: u16) -> Self {
        Self { port_id }
    }

    /// Marshal to bytes
    pub fn marshal(&self) -> Vec<u8> {
        self.port_id.to_be_bytes().to_vec()
    }

    /// Unmarshal from bytes
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() != 2 {
            return Err(PfcpError::invalid_length("TSN Port ID", IeType::TsnPortIdAdvanced, 2, data.len()));
        }

        let port_id = u16::from_be_bytes([data[0], data[1]]);
        Ok(Self { port_id })
    }
}

impl From<TsnPortId> for Ie {
    fn from(ie: TsnPortId) -> Self {
        Ie::new(IeType::TsnPortIdAdvanced, ie.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsn_port_id_marshal_unmarshal() {
        let tsn_port = TsnPortId::new(1234);
        
        let marshaled = tsn_port.marshal();
        assert_eq!(marshaled.len(), 2);
        assert_eq!(marshaled, vec![0x04, 0xD2]); // 1234 in big-endian
        
        let unmarshaled = TsnPortId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, tsn_port);
        assert_eq!(unmarshaled.port_id, 1234);
    }

    #[test]
    fn test_tsn_port_id_edge_cases() {
        // Test minimum value
        let min_port = TsnPortId::new(0);
        let marshaled = min_port.marshal();
        let unmarshaled = TsnPortId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.port_id, 0);
        
        // Test maximum value
        let max_port = TsnPortId::new(65535);
        let marshaled = max_port.marshal();
        let unmarshaled = TsnPortId::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled.port_id, 65535);
    }

    #[test]
    fn test_tsn_port_id_invalid_length() {
        let invalid_data = vec![0x12]; // Too short
        assert!(TsnPortId::unmarshal(&invalid_data).is_err());
        
        let invalid_data = vec![0x12, 0x34, 0x56]; // Too long
        assert!(TsnPortId::unmarshal(&invalid_data).is_err());
    }

    #[test]
    fn test_tsn_port_id_into_ie() {
        let tsn_port = TsnPortId::new(8080);
        let ie: Ie = tsn_port.into();
        
        assert_eq!(ie.ie_type, IeType::TsnPortIdAdvanced);
        assert_eq!(ie.payload, vec![0x1F, 0x90]); // 8080 in big-endian
    }
}
