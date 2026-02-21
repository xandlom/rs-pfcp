//! Data Status Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.187, indicates the status of buffered/dropped data.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct DataStatus: u8 {
        const DROP = 1 << 0; // Bit 1: Data was dropped
        const BUFF = 1 << 1; // Bit 2: Data was buffered
    }
}

impl DataStatus {
    pub fn marshal(&self) -> [u8; 1] {
        [self.bits()]
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Data Status",
                IeType::DataStatus,
                1,
                0,
            ));
        }
        Ok(DataStatus::from_bits_truncate(data[0]))
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DataStatus, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal_both() {
        let flags = DataStatus::DROP | DataStatus::BUFF;
        let parsed = DataStatus::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_marshal_unmarshal_single() {
        let flags = DataStatus::DROP;
        let parsed = DataStatus::unmarshal(&flags.marshal()).unwrap();
        assert_eq!(parsed, flags);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            DataStatus::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(DataStatus::DROP.to_ie().ie_type, IeType::DataStatus);
    }
}
