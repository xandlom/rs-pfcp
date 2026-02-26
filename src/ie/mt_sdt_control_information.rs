//! MT-SDT Control Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.239, controls MT-SDT (Mobile Terminated
//! Small Data Transmission) behaviour.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// MT-SDT Control Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.239
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MtSdtControlInformation: u8 {
        /// RDSI: Release DL Data Indication
        const RDSI = 1 << 0;
    }
}

impl MtSdtControlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "MT-SDT Control Information",
                IeType::MtSdtControlInformation,
                1,
                0,
            ));
        }
        Ok(MtSdtControlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MtSdtControlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = MtSdtControlInformation::RDSI;
        let parsed = MtSdtControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = MtSdtControlInformation::empty();
        let parsed = MtSdtControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MtSdtControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = MtSdtControlInformation::RDSI.to_ie();
        assert_eq!(ie.ie_type, IeType::MtSdtControlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
