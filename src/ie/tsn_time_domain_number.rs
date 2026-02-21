//! TSN Time Domain Number Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.150, contains the TSN time domain number
//! as a single u8 value for Time-Sensitive Networking.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsnTimeDomainNumber {
    pub value: u8,
}

impl TsnTimeDomainNumber {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 1] {
        [self.value]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "TSN Time Domain Number",
                IeType::TsnTimeDomainNumber,
                1,
                0,
            ));
        }
        Ok(Self { value: data[0] })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TsnTimeDomainNumber, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = TsnTimeDomainNumber::new(5);
        let parsed = TsnTimeDomainNumber::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for v in [0, 1, 127, 255] {
            let ie = TsnTimeDomainNumber::new(v);
            let parsed = TsnTimeDomainNumber::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TsnTimeDomainNumber::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            TsnTimeDomainNumber::new(0).to_ie().ie_type,
            IeType::TsnTimeDomainNumber
        );
    }

    #[test]
    fn test_marshal_byte() {
        assert_eq!(TsnTimeDomainNumber::new(42).marshal(), [42]);
    }
}
