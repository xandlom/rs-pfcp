//! Traffic Parameter Measurement Indication Information Element.
//!
//! Per 3GPP TS 29.244, contains traffic parameter measurement indication flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct TrafficParameterMeasurementIndication: u8 {
        const TPMI = 1 << 0; // Bit 1: Traffic Parameter Measurement Indication
    }
}

impl TrafficParameterMeasurementIndication {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Traffic Parameter Measurement Indication",
                IeType::TrafficParameterMeasurementIndication,
                1,
                0,
            ));
        }
        Ok(Self::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(
            IeType::TrafficParameterMeasurementIndication,
            self.marshal().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = TrafficParameterMeasurementIndication::TPMI;
        let parsed = TrafficParameterMeasurementIndication::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TrafficParameterMeasurementIndication::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            TrafficParameterMeasurementIndication::TPMI.to_ie().ie_type,
            IeType::TrafficParameterMeasurementIndication
        );
    }
}
