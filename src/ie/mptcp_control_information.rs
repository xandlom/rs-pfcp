//! MPTCP Control Information IE (IE Type 222).
//!
//! Per 3GPP TS 29.244 Section 8.2.154, provides details of required MPTCP functionality.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// MPTCP Control Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.154
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MptcpControlInformation: u8 {
        /// TCI: Transport Converter Indication
        const TCI = 1 << 0;
    }
}

impl MptcpControlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MPTCP Control Information",
                IeType::MptcpControlInformation,
                1,
                0,
            ));
        }
        Ok(MptcpControlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MptcpControlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_tci() {
        let flags = MptcpControlInformation::TCI;
        let parsed = MptcpControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_empty() {
        let flags = MptcpControlInformation::empty();
        let parsed = MptcpControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_short_buffer() {
        assert!(matches!(
            MptcpControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MptcpControlInformation::TCI.to_ie();
        assert_eq!(ie.ie_type, IeType::MptcpControlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
