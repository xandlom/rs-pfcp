//! PFCPASReq-Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.186, contains flags for
//! PFCP Association Setup Request.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpasReqFlags: u8 {
        const UUPSI = 1 << 0; // Bit 1: UPF User Plane Security Indication
    }
}

impl PfcpasReqFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPASReq-Flags",
                IeType::PfcpasReqFlags,
                1,
                0,
            ));
        }
        Ok(PfcpasReqFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpasReqFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpasReqFlags::UUPSI;
        let parsed = PfcpasReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = PfcpasReqFlags::empty();
        let parsed = PfcpasReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpasReqFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PfcpasReqFlags::UUPSI.to_ie().ie_type,
            IeType::PfcpasReqFlags
        );
    }
}
