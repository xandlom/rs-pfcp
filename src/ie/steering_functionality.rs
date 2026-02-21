//! Steering Functionality Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.138, indicates the steering functionality.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Steering functionality values per 3GPP TS 29.244.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SteeringFunctionality {
    AtsdLL = 0,
    Mptcp = 1,
}

impl SteeringFunctionality {
    pub fn marshal(&self) -> [u8; 1] {
        [*self as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Steering Functionality",
                IeType::SteeringFunctionality,
                1,
                0,
            ));
        }
        match data[0] & 0x0F {
            0 => Ok(SteeringFunctionality::AtsdLL),
            1 => Ok(SteeringFunctionality::Mptcp),
            v => Err(PfcpError::invalid_value(
                "Steering Functionality",
                v.to_string(),
                "must be 0 (ATSSS-LL) or 1 (MPTCP)",
            )),
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SteeringFunctionality, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for sf in [SteeringFunctionality::AtsdLL, SteeringFunctionality::Mptcp] {
            let parsed = SteeringFunctionality::unmarshal(&sf.marshal()).unwrap();
            assert_eq!(parsed, sf);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            SteeringFunctionality::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid() {
        assert!(matches!(
            SteeringFunctionality::unmarshal(&[5]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            SteeringFunctionality::AtsdLL.to_ie().ie_type,
            IeType::SteeringFunctionality
        );
    }
}
