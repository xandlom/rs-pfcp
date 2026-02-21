//! MBSN4mb Req-Flags Information Element.
//!
//! Per 3GPP TS 29.244, contains flags for MBS N4mb Request.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Mbsn4mbReqFlags: u8 {
        const PLLSSM = 1 << 0; // Bit 1: Provide Linked Session Set Member
        const JMBSSM = 1 << 1; // Bit 2: Join MBS Session Set Member
        const LMBSSM = 1 << 2; // Bit 3: Leave MBS Session Set Member
    }
}

impl Mbsn4mbReqFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MBSN4mb Req-Flags",
                IeType::Mbsn4mbReqFlags,
                1,
                0,
            ));
        }
        Ok(Mbsn4mbReqFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Mbsn4mbReqFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = Mbsn4mbReqFlags::PLLSSM | Mbsn4mbReqFlags::JMBSSM;
        let parsed = Mbsn4mbReqFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            Mbsn4mbReqFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            Mbsn4mbReqFlags::PLLSSM.to_ie().ie_type,
            IeType::Mbsn4mbReqFlags
        );
    }
}
