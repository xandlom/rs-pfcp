//! Maximum Receive Unit IE (IE Type 287).
//!
//! Per 3GPP TS 29.244 Section 8.2.195, a 2-octet big-endian value specifying
//! the maximum receive unit size (in bytes) for an L2TP session.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumReceiveUnit {
    pub mru: u16,
}

impl MaximumReceiveUnit {
    pub fn new(mru: u16) -> Self {
        Self { mru }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.mru.to_be_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Maximum Receive Unit",
                IeType::MaximumReceiveUnit,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            mru: u16::from_be_bytes([data[0], data[1]]),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MaximumReceiveUnit, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = MaximumReceiveUnit::new(1500);
        let parsed = MaximumReceiveUnit::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_zero() {
        let original = MaximumReceiveUnit::new(0);
        let parsed = MaximumReceiveUnit::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_max() {
        let original = MaximumReceiveUnit::new(u16::MAX);
        let parsed = MaximumReceiveUnit::unmarshal(&original.marshal()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_short_buffer() {
        assert!(matches!(
            MaximumReceiveUnit::unmarshal(&[0x05]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie_type() {
        assert_eq!(
            MaximumReceiveUnit::new(1500).to_ie().ie_type,
            IeType::MaximumReceiveUnit
        );
    }
}
