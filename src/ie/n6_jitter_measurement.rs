//! N6 Jitter Measurement Information Element.
//!
//! Per 3GPP TS 29.244, contains jitter measurement in microseconds.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N6JitterMeasurement {
    pub value: u32,
}

impl N6JitterMeasurement {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "N6 Jitter Measurement",
                IeType::N6JitterMeasurement,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::N6JitterMeasurement, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let jitter = N6JitterMeasurement::new(1500);
        let parsed = N6JitterMeasurement::unmarshal(&jitter.marshal()).unwrap();
        assert_eq!(parsed, jitter);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            N6JitterMeasurement::unmarshal(&[0x00, 0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            N6JitterMeasurement::new(100).to_ie().ie_type,
            IeType::N6JitterMeasurement
        );
    }
}
