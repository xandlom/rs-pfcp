//! PFCPSDRsp-Flags Information Element.
//!
//! Per 3GPP TS 29.244, contains flags for PFCP Session Deletion Response.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PfcpsdrspFlags: u8 {
        const PURU = 1 << 0; // Bit 1: Pending Usage Reports Update
    }
}

impl PfcpsdrspFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "PFCPSDRsp-Flags",
                IeType::PfcpsdrspFlags,
                1,
                0,
            ));
        }
        Ok(PfcpsdrspFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpsdrspFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = PfcpsdrspFlags::PURU;
        let parsed = PfcpsdrspFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            PfcpsdrspFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(PfcpsdrspFlags::PURU.to_ie().ie_type, IeType::PfcpsdrspFlags);
    }
}
