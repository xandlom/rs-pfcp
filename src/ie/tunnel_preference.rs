//! Tunnel Preference IE (IE Type 281).
//!
//! Per 3GPP TS 29.244 Section 8.2.189, a 3-octet big-endian value indicating
//! the preference for selecting an L2TP tunnel to a given LNS.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelPreference {
    pub preference: u32,
}

impl TunnelPreference {
    pub fn new(preference: u32) -> Self {
        Self { preference }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![
            (self.preference >> 16) as u8,
            (self.preference >> 8) as u8,
            self.preference as u8,
        ]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 3 {
            return Err(PfcpError::invalid_length(
                "Tunnel Preference",
                IeType::TunnelPreference,
                3,
                data.len(),
            ));
        }
        let preference =
            (u32::from(data[0]) << 16) | (u32::from(data[1]) << 8) | u32::from(data[2]);
        Ok(Self { preference })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TunnelPreference, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = TunnelPreference::new(0x00A1B2);
        let parsed = TunnelPreference::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_zero() {
        let original = TunnelPreference::new(0);
        let parsed = TunnelPreference::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_max() {
        let original = TunnelPreference::new(0x00FF_FFFF);
        let parsed = TunnelPreference::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            TunnelPreference::unmarshal(&[0x01, 0x02]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            TunnelPreference::new(1).to_ie().ie_type,
            IeType::TunnelPreference
        );
    }
}
