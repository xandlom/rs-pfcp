//! MPQUIC Control Information IE (IE Type 330).
//!
//! Per 3GPP TS 29.244 Section 8.2.225, provides details of required MPQUIC functionality.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// MPQUIC Control Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.225
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MpquicControlInformation: u8 {
        /// CUDP: CONNECT-UDP proxy type required
        const CUDP = 1 << 0;
    }
}

impl MpquicControlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MPQUIC Control Information",
                IeType::MpquicControlInformation,
                1,
                0,
            ));
        }
        Ok(MpquicControlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MpquicControlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_cudp() {
        let flags = MpquicControlInformation::CUDP;
        let parsed = MpquicControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let flags = MpquicControlInformation::empty();
        let parsed = MpquicControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            MpquicControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MpquicControlInformation::CUDP.to_ie();
        assert_eq!(ie.ie_type, IeType::MpquicControlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
