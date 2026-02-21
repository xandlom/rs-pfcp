//! Measurement Period Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.49, this IE contains the measurement period
//! in seconds for periodic usage reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementPeriod {
    pub value: u32,
}

impl MeasurementPeriod {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Measurement Period",
                IeType::MeasurementPeriod,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementPeriod, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = MeasurementPeriod::new(3600);
        let marshaled = ie.marshal();
        let unmarshaled = MeasurementPeriod::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 60, 3600, 86400, u32::MAX] {
            let ie = MeasurementPeriod::new(v);
            let data = ie.marshal();
            let parsed = MeasurementPeriod::unmarshal(&data).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            MeasurementPeriod::unmarshal(&[0; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MeasurementPeriod::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MeasurementPeriod::new(300);
        assert_eq!(ie.to_ie().ie_type, IeType::MeasurementPeriod);
    }

    #[test]
    fn test_byte_order() {
        let ie = MeasurementPeriod::new(0x12345678);
        assert_eq!(ie.marshal(), [0x12, 0x34, 0x56, 0x78]);
    }
}
