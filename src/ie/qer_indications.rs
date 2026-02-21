//! QER Indications Information Element.
//!
//! Per 3GPP TS 29.244, contains QoS Enforcement Rule indication flags.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct QerIndications: u8 {
        const IQFCI = 1 << 0; // Bit 1: Insert QFI in encapsulation header
    }
}

impl QerIndications {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "QER Indications",
                IeType::QerIndications,
                1,
                0,
            ));
        }
        Ok(QerIndications::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::QerIndications, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let flags = QerIndications::IQFCI;
        let parsed = QerIndications::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            QerIndications::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            QerIndications::IQFCI.to_ie().ie_type,
            IeType::QerIndications
        );
    }
}
