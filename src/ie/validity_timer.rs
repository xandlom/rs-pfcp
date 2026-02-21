//! Validity Timer Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.196, contains the validity timer
//! as a u16 value in seconds.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidityTimer {
    pub value: u16,
}

impl ValidityTimer {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Validity Timer",
                IeType::ValidityTimer,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ValidityTimer, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = ValidityTimer::new(300);
        let parsed = ValidityTimer::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 60, 3600, u16::MAX] {
            let ie = ValidityTimer::new(v);
            let parsed = ValidityTimer::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            ValidityTimer::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ValidityTimer::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            ValidityTimer::new(60).to_ie().ie_type,
            IeType::ValidityTimer
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(ValidityTimer::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
