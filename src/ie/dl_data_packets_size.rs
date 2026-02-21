//! DL Data Packets Size Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.178, contains the downlink data packets size
//! as a u16 value in bytes.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DlDataPacketsSize {
    pub value: u16,
}

impl DlDataPacketsSize {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "DL Data Packets Size",
                IeType::DlDataPacketsSize,
                2,
                data.len(),
            ));
        }
        Ok(Self {
            value: u16::from_be_bytes(data[0..2].try_into().unwrap()),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DlDataPacketsSize, self.marshal().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let ie = DlDataPacketsSize::new(1500);
        let parsed = DlDataPacketsSize::unmarshal(&ie.marshal()).unwrap();
        assert_eq!(parsed, ie);
    }

    #[test]
    fn test_round_trip_values() {
        for &v in &[0, 1, 64, 1500, 9000, u16::MAX] {
            let ie = DlDataPacketsSize::new(v);
            let parsed = DlDataPacketsSize::unmarshal(&ie.marshal()).unwrap();
            assert_eq!(parsed, ie);
        }
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            DlDataPacketsSize::unmarshal(&[0; 1]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            DlDataPacketsSize::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            DlDataPacketsSize::new(100).to_ie().ie_type,
            IeType::DlDataPacketsSize
        );
    }

    #[test]
    fn test_byte_order() {
        assert_eq!(DlDataPacketsSize::new(0x1234).marshal(), [0x12, 0x34]);
    }
}
