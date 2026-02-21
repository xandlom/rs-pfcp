//! TSN Bridge ID Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.161, contains a TSN bridge identifier (MAC address).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsnBridgeId {
    pub mac: [u8; 6],
}

impl TsnBridgeId {
    pub fn new(mac: [u8; 6]) -> Self {
        Self { mac }
    }

    pub fn marshal(&self) -> Vec<u8> {
        // Flags byte (bit 1 = MAC address present) + MAC address
        let mut data = vec![0x01]; // MAC flag set
        data.extend_from_slice(&self.mac);
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "TSN Bridge ID",
                IeType::TsnBridgeId,
                1,
                0,
            ));
        }
        let flags = data[0];
        if (flags & 0x01) == 0 {
            return Err(PfcpError::invalid_value(
                "TSN Bridge ID flags",
                format!("0x{:02X}", flags),
                "MAC flag must be set",
            ));
        }
        if data.len() < 7 {
            return Err(PfcpError::invalid_length(
                "TSN Bridge ID",
                IeType::TsnBridgeId,
                7,
                data.len(),
            ));
        }
        let mut mac = [0u8; 6];
        mac.copy_from_slice(&data[1..7]);
        Ok(Self { mac })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TsnBridgeId, self.marshal())
    }
}

impl std::fmt::Display for TsnBridgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.mac[0], self.mac[1], self.mac[2], self.mac[3], self.mac[4], self.mac[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let bridge = TsnBridgeId::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let parsed = TsnBridgeId::unmarshal(&bridge.marshal()).unwrap();
        assert_eq!(parsed, bridge);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TsnBridgeId::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            TsnBridgeId::unmarshal(&[0x01, 0xAA, 0xBB]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_display() {
        let bridge = TsnBridgeId::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(format!("{}", bridge), "00:11:22:33:44:55");
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            TsnBridgeId::new([0; 6]).to_ie().ie_type,
            IeType::TsnBridgeId
        );
    }
}
