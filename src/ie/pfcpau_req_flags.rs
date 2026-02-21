//! PFCPAU Req-Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.120, contains flags for
//! PFCP Association Update Request.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpauReqFlags: u8 {
        const PARPS = 1 << 0; // Bit 1: PFCP Association Release Preparation Start
    }
}

impl PfcpauReqFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPAU Req-Flags",
                IeType::PfcpauReqFlags,
                1,
                0,
            ));
        }
        Ok(PfcpauReqFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpauReqFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpauReqFlags::PARPS;
        let parsed = PfcpauReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = PfcpauReqFlags::empty();
        let parsed = PfcpauReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpauReqFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PfcpauReqFlags::PARPS.to_ie().ie_type,
            IeType::PfcpauReqFlags
        );
    }
}
