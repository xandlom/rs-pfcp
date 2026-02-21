//! Priority Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.141, indicates the priority for access forwarding.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Priority values per 3GPP TS 29.244.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    Active = 0,
    Standby = 1,
    NoPriority = 2,
    High = 3,
}

impl Priority {
    pub fn marshal(&self) -> [u8; 1] {
        [*self as u8]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Priority",
                IeType::Priority,
                1,
                0,
            ));
        }
        match data[0] & 0x0F {
            0 => Ok(Priority::Active),
            1 => Ok(Priority::Standby),
            2 => Ok(Priority::NoPriority),
            3 => Ok(Priority::High),
            v => Err(PfcpError::invalid_value(
                "Priority",
                v.to_string(),
                "must be 0-3",
            )),
        }
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Priority, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        for p in [
            Priority::Active,
            Priority::Standby,
            Priority::NoPriority,
            Priority::High,
        ] {
            let parsed = Priority::unmarshal(&p.marshal()).unwrap();
            assert_eq!(parsed, p);
        }
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            Priority::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid() {
        assert!(matches!(
            Priority::unmarshal(&[10]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(Priority::Active.to_ie().ie_type, IeType::Priority);
    }
}
