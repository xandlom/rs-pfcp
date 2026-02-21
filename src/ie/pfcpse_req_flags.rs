//! PFCPSE Req-Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.134, contains flags for
//! PFCP Session Establishment Request.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpseReqFlags: u8 {
        const RESTI = 1 << 0; // Bit 1: Restoration Indication
    }
}

impl PfcpseReqFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPSE Req-Flags",
                IeType::PfcpseReqFlags,
                1,
                0,
            ));
        }
        Ok(PfcpseReqFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpseReqFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpseReqFlags::RESTI;
        let parsed = PfcpseReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = PfcpseReqFlags::empty();
        let parsed = PfcpseReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpseReqFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PfcpseReqFlags::RESTI.to_ie().ie_type,
            IeType::PfcpseReqFlags
        );
    }
}
