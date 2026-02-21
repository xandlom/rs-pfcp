//! Reporting Frequency Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.173, contains QoS monitoring reporting frequency flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ReportingFrequency: u8 {
        const EVETT = 1 << 0; // Bit 1: Event Triggered
        const PERIO = 1 << 1; // Bit 2: Periodic
        const SESRL = 1 << 2; // Bit 3: Session Released
    }
}

impl ReportingFrequency {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Reporting Frequency",
                IeType::ReportingFrequency,
                1,
                0,
            ));
        }
        Ok(ReportingFrequency::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingFrequency, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_all() {
        let flags =
            ReportingFrequency::EVETT | ReportingFrequency::PERIO | ReportingFrequency::SESRL;
        let parsed = ReportingFrequency::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ReportingFrequency::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            ReportingFrequency::EVETT.to_ie().ie_type,
            IeType::ReportingFrequency
        );
    }
}
