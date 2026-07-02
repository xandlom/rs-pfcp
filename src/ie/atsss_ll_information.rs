//! ATSSS-LL Information IE (IE Type 231).
//!
//! Per 3GPP TS 29.244 Section 8.2.160, indicates that resources have been
//! allocated for the ATSSS-LL steering functionality.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// ATSSS-LL Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.160
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct AtsssLlInformation: u8 {
        /// LLI: resources for ATSSS-LL steering functionality have been allocated
        const LLI = 1 << 0;
    }
}

impl AtsssLlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "ATSSS-LL Information",
                IeType::AtsssLlInformation,
                1,
                0,
            ));
        }
        Ok(AtsssLlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AtsssLlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_lli() {
        let flags = AtsssLlInformation::LLI;
        let parsed = AtsssLlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let flags = AtsssLlInformation::empty();
        let parsed = AtsssLlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            AtsssLlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AtsssLlInformation::LLI.to_ie();
        assert_eq!(ie.ie_type, IeType::AtsssLlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
