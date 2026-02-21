//! Requested Access Availability Information IE.
//!
//! Per 3GPP TS 29.244 Section 8.2.160, contains flags for access
//! availability information requests.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct RequestedAccessAvailabilityInformation: u8 {
        const RRCA = 1 << 0; // Bit 1: Request Reporting of Change in Access availability
    }
}

impl RequestedAccessAvailabilityInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Requested Access Availability Information",
                IeType::RequestedAccessAvailabilityInformation,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::RequestedAccessAvailabilityInformation,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = RequestedAccessAvailabilityInformation::RRCA;
        let parsed = RequestedAccessAvailabilityInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            RequestedAccessAvailabilityInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            RequestedAccessAvailabilityInformation::RRCA.to_ie().ie_type,
            IeType::RequestedAccessAvailabilityInformation
        );
    }
}
