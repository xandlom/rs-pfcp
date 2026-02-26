//! Reporting Control Information Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.272, controls UE Level Measurement reporting.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    /// Reporting Control Information flags.
    ///
    /// # 3GPP Reference
    /// 3GPP TS 29.244 Section 8.2.272
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ReportingControlInformation: u8 {
        /// UELM: UE Level Measurement
        const UELM = 1 << 0;
    }
}

impl ReportingControlInformation {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Reporting Control Information",
                IeType::ReportingControlInformation,
                1,
                0,
            ));
        }
        Ok(ReportingControlInformation::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ReportingControlInformation, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = ReportingControlInformation::UELM;
        let parsed = ReportingControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_round_trip_empty() {
        let flags = ReportingControlInformation::empty();
        let parsed = ReportingControlInformation::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            ReportingControlInformation::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        let ie = ReportingControlInformation::UELM.to_ie();
        assert_eq!(ie.ie_type, IeType::ReportingControlInformation);
        assert_eq!(ie.payload, vec![0x01]);
    }
}
