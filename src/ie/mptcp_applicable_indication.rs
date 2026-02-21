//! MPTCP Applicable Indication Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.192, indicates MPTCP applicability.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MptcpApplicableIndication: u8 {
        const MAI = 1 << 0; // Bit 1: MPTCP Applicable Indication
    }
}

impl MptcpApplicableIndication {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MPTCP Applicable Indication",
                IeType::MptcpApplicableIndication,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MptcpApplicableIndication, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = MptcpApplicableIndication::MAI;
        let parsed = MptcpApplicableIndication::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MptcpApplicableIndication::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            MptcpApplicableIndication::MAI.to_ie().ie_type,
            IeType::MptcpApplicableIndication
        );
    }
}
