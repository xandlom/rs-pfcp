//! Event Time Stamp Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.116, contains the event time stamp
//! as a 3GPP NTP timestamp (seconds since 1900-01-01).

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventTimeStamp {
    pub timestamp: u32,
}

impl EventTimeStamp {
    pub fn new(timestamp: u32) -> Self {
        Self { timestamp }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.timestamp.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 4 {
            return Err(PfcpError::invalid_length(
                "Event Time Stamp",
                IeType::EventTimeStamp,
                4,
                data.len(),
            ));
        }
        Ok(Self {
            timestamp: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::EventTimeStamp, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = EventTimeStamp::new(0xABCDEF01);
        let parsed = EventTimeStamp::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 0x12345678, 0xFFFFFFFF] {
            let ie = EventTimeStamp::new(v);
            let parsed = EventTimeStamp::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            EventTimeStamp::unmarshal(&[0; 3]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            EventTimeStamp::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            EventTimeStamp::new(42).to_ie().ie_type,
            IeType::EventTimeStamp
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(
            EventTimeStamp::new(0x12345678).marshal(),
            [0x12, 0x34, 0x56, 0x78]
        );
    }
}
