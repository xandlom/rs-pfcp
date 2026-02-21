//! PFCP Association Release Request Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.66, contains flags for PFCP association release.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpAssociationReleaseRequest: u8 {
        const SARR = 1 << 0; // Bit 1: PFCP Association Release Request
        const URSS = 1 << 1; // Bit 2: User Plane Path Failure Report
    }
}

impl PfcpAssociationReleaseRequest {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCP Association Release Request",
                IeType::PfcpAssociationReleaseRequest,
                1,
                0,
            ));
        }
        Ok(PfcpAssociationReleaseRequest::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::PfcpAssociationReleaseRequest,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpAssociationReleaseRequest::SARR | PfcpAssociationReleaseRequest::URSS;
        let parsed = PfcpAssociationReleaseRequest::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_single() {
        let flags = PfcpAssociationReleaseRequest::SARR;
        let parsed = PfcpAssociationReleaseRequest::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpAssociationReleaseRequest::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PfcpAssociationReleaseRequest::SARR.to_ie().ie_type,
            IeType::PfcpAssociationReleaseRequest
        );
    }
}
