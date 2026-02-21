//! Requested Clock Drift Information IE.
//!
//! Per 3GPP TS 29.244 Section 8.2.148, contains flags for clock drift
//! information requests in TSN.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct RequestedClockDriftInformation: u8 {
        const RRTO = 1 << 0; // Bit 1: Request Reporting of Time Offset
        const RRCR = 1 << 1; // Bit 2: Request Reporting of Cumulative Rate Ratio
    }
}

impl RequestedClockDriftInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Requested Clock Drift Information",
                IeType::RequestedClockDriftInformation,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::RequestedClockDriftInformation,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = RequestedClockDriftInformation::RRTO | RequestedClockDriftInformation::RRCR;
        let parsed = RequestedClockDriftInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RequestedClockDriftInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            RequestedClockDriftInformation::RRTO.to_ie().ie_type,
            IeType::RequestedClockDriftInformation
        );
    }
}
