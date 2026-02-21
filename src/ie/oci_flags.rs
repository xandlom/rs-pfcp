//! OCI Flags Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.65, contains overload control information flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct OciFlags: u8 {
        const AOCI = 1 << 0; // Bit 1: Associate OCI with Node ID
    }
}

impl OciFlags {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "OCI Flags",
                IeType::OciFlags,
                1,
                0,
            ));
        }
        Ok(OciFlags::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::OciFlags, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = OciFlags::AOCI;
        let parsed = OciFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = OciFlags::empty();
        let parsed = OciFlags::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            OciFlags::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(OciFlags::AOCI.to_ie().ie_type, IeType::OciFlags);
    }
}
