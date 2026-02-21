//! DS-TT Port Number Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.140, contains the DS-TT (Device-Side
//! TSN Translator) port number for TSN bridge management.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DsttPortNumber {
    pub value: u16,
}

impl DsttPortNumber {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "DS-TT Port Number",
                IeType::DsttPortNumber,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DsttPortNumber, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = DsttPortNumber::new(8805);
        let parsed = DsttPortNumber::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 80, 8805, u16::MAX] {
            let ie = DsttPortNumber::new(v);
            let parsed = DsttPortNumber::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            DsttPortNumber::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            DsttPortNumber::new(1).to_ie().ie_type,
            IeType::DsttPortNumber
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(DsttPortNumber::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
