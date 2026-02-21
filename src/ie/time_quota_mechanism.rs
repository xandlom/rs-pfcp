//! Time Quota Mechanism Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.90, contains the time quota type and base time interval.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Base Time Interval Type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BtiType {
    Ctp = 0,
    Dtp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeQuotaMechanism {
    pub bti_type: BtiType,
    pub base_time_interval: u32,
}

impl TimeQuotaMechanism {
    pub fn new(bti_type: BtiType, base_time_interval: u32) -> Self {
        Self {
            bti_type,
            base_time_interval,
        }
    }

    pub fn marshal(&self) -> [u8; 5] {
        let mut data = [0u8; 5];
        data[0] = self.bti_type as u8;
        data[1..5].copy_from_slice(&self.base_time_interval.to_be_bytes());
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 5 {
            return Err(PfcpError::invalid_length(
                "Time Quota Mechanism",
                IeType::TimeQuotaMechanism,
                5,
                data.len(),
            ));
        }
        let bti_type = match data[0] & 0x03 {
            0 => BtiType::Ctp,
            1 => BtiType::Dtp,
            v => {
                return Err(PfcpError::invalid_value(
                    "Time Quota Mechanism BTI",
                    v.to_string(),
                    "must be 0 (CTP) or 1 (DTP)",
                ))
            }
        };
        let base_time_interval = u32::from_be_bytes(data[1..5].try_into().unwrap());
        Ok(Self {
            bti_type,
            base_time_interval,
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TimeQuotaMechanism, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let tqm = TimeQuotaMechanism::new(BtiType::Ctp, 3600);
        let parsed = TimeQuotaMechanism::unmarshal(&tqm.marshal()).unwrap();
        assert_eq!(parsed, tqm);
    }

    #[test]
    fn test_marshal_unmarshal_dtp() {
        let tqm = TimeQuotaMechanism::new(BtiType::Dtp, 1800);
        let parsed = TimeQuotaMechanism::unmarshal(&tqm.marshal()).unwrap();
        assert_eq!(parsed, tqm);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            TimeQuotaMechanism::unmarshal(&[0x00, 0x01]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_invalid_bti() {
        assert!(matches!(
            TimeQuotaMechanism::unmarshal(&[0x03, 0x00, 0x00, 0x00, 0x01]),
            Err(PfcpError::InvalidValue { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            TimeQuotaMechanism::new(BtiType::Ctp, 0).to_ie().ie_type,
            IeType::TimeQuotaMechanism
        );
    }
}
