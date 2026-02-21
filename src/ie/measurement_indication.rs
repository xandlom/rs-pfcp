//! Measurement Indication Information Element.
//!
//! Per 3GPP TS 29.244, contains measurement indication flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MeasurementIndication: u8 {
        const RLCI = 1 << 0; // Bit 1: Report Loss Combination Information
        const DLQI = 1 << 1; // Bit 2: DL QoS Information
    }
}

impl MeasurementIndication {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Measurement Indication",
                IeType::MeasurementIndication,
                1,
                0,
            ));
        }
        Ok(MeasurementIndication::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::MeasurementIndication, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = MeasurementIndication::RLCI | MeasurementIndication::DLQI;
        let parsed = MeasurementIndication::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            MeasurementIndication::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            MeasurementIndication::RLCI.to_ie().ie_type,
            IeType::MeasurementIndication
        );
    }
}
