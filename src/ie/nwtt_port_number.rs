//! NW-TT Port Number Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.141, contains the NW-TT (Network-Side
//! TSN Translator) port number for TSN bridge management.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NwttPortNumber {
    pub value: u16,
}

impl NwttPortNumber {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "NW-TT Port Number",
                IeType::NwttPortNumber,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NwttPortNumber, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = NwttPortNumber::new(8805);
        let parsed = NwttPortNumber::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 80, 8805, u16::MAX] {
            let ie = NwttPortNumber::new(v);
            let parsed = NwttPortNumber::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            NwttPortNumber::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            NwttPortNumber::new(1).to_ie().ie_type,
            IeType::NwttPortNumber
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(NwttPortNumber::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
