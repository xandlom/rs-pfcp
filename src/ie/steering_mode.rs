//! Steering Mode Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.139, indicates the steering mode.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Steering mode values per 3GPP TS 29.244.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SteeringMode {
    ActiveStandby = 0,
    SmallestDelay = 1,
    LoadBalancing = 2,
    PriorityBased = 3,
    Redundant = 4,
}

impl SteeringMode {
    pub fn marshal(&self) -> [u8; 1] {
        [*self as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Steering Mode",
                IeType::SteeringMode,
                1,
                0,
            ));
        }
        match data[0] & 0x0F {
            0 => Ok(SteeringMode::ActiveStandby),
            1 => Ok(SteeringMode::SmallestDelay),
            2 => Ok(SteeringMode::LoadBalancing),
            3 => Ok(SteeringMode::PriorityBased),
            4 => Ok(SteeringMode::Redundant),
            v => Err(PfcpError::invalid_value(
                "Steering Mode",
                v.to_string(),
                "must be 0-4",
            )),
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::SteeringMode, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for sm in [
            SteeringMode::ActiveStandby,
            SteeringMode::SmallestDelay,
            SteeringMode::LoadBalancing,
            SteeringMode::PriorityBased,
            SteeringMode::Redundant,
        ] {
            let parsed = SteeringMode::unmarshal(&sm.marshal()).unwrap();
            assert_eq!(parsed, sm);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            SteeringMode::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid() {
        assert!(matches!(
            SteeringMode::unmarshal(&[8]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            SteeringMode::ActiveStandby.to_ie().ie_type,
            IeType::SteeringMode
        );
    }
}
