//! PFCPSR Req-Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.119, contains flags for
//! PFCP Session Report Request.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpsrReqFlags: u8 {
        const PMSRI = 1 << 0; // Bit 1: PFCP Session Modification Request Indication
    }
}

impl PfcpsrReqFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPSR Req-Flags",
                IeType::PfcpsrReqFlags,
                1,
                0,
            ));
        }
        Ok(PfcpsrReqFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpsrReqFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpsrReqFlags::PMSRI;
        let parsed = PfcpsrReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = PfcpsrReqFlags::empty();
        let parsed = PfcpsrReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpsrReqFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            PfcpsrReqFlags::PMSRI.to_ie().ie_type,
            IeType::PfcpsrReqFlags
        );
    }
}
