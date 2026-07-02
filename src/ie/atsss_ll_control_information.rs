//! ATSSS-LL Control Information IE (IE Type 223).
//!
//! Per 3GPP TS 29.244 Section 8.2.155, provides details of required ATSSS-LL functionality.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// ATSSS-LL Control Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.155
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct AtsssLlControlInformation: u8 {
        /// LLI: ATSSS-LL steering functionality is required
        const LLI = 1 << 0;
    }
}

impl AtsssLlControlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "ATSSS-LL Control Information",
                IeType::AtsssLlControlInformation,
                1,
                0,
            ));
        }
        Ok(AtsssLlControlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::AtsssLlControlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_lli() {
        let flags = AtsssLlControlInformation::LLI;
        let parsed = AtsssLlControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let flags = AtsssLlControlInformation::empty();
        let parsed = AtsssLlControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            AtsssLlControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = AtsssLlControlInformation::LLI.to_ie();
        assert_eq!(ie.ie_type, IeType::AtsssLlControlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
