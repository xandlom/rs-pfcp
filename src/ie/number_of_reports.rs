//! Number of Reports Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.132, contains the number of reports
//! as a u16 count value.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberOfReports {
    pub value: u16,
}

impl NumberOfReports {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "Number of Reports",
                IeType::NumberOfReports,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::NumberOfReports, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = NumberOfReports::new(42);
        let parsed = NumberOfReports::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 100, 1000, u16::MAX] {
            let ie = NumberOfReports::new(v);
            let parsed = NumberOfReports::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            NumberOfReports::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            NumberOfReports::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            NumberOfReports::new(5).to_ie().ie_type,
            IeType::NumberOfReports
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(NumberOfReports::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
