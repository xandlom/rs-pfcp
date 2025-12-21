// src/ie/gbr.rs

//! GBR Information Element.

use crate::error::PfcpError;
use crate::ie::IeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gbr {
    pub uplink: u64,
    pub downlink: u64,
}

impl Gbr {
    pub fn new(uplink: u64, downlink: u64) -> Self {
        Gbr { uplink, downlink }
    }

    pub fn marshal(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];
        bytes[0..5].copy_from_slice(&self.uplink.to_be_bytes()[3..]);
        bytes[5..10].copy_from_slice(&self.downlink.to_be_bytes()[3..]);
        bytes
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 10 {
            return Err(PfcpError::invalid_length(
                "GBR",
                IeType::Gbr,
                10,
                data.len(),
            ));
        }
        let mut ul_bytes = [0u8; 8];
        ul_bytes[3..].copy_from_slice(&data[0..5]);
        let mut dl_bytes = [0u8; 8];
        dl_bytes[3..].copy_from_slice(&data[5..10]);
        Ok(Gbr {
            uplink: u64::from_be_bytes(ul_bytes),
            downlink: u64::from_be_bytes(dl_bytes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gbr_marshal_unmarshal() {
        let gbr = Gbr::new(500_000, 750_000);
        let marshaled = gbr.marshal();
        let unmarshaled = Gbr::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, gbr);
    }

    #[test]
    fn test_gbr_unmarshal_invalid_data() {
        let data = [0; 9];
        let result = Gbr::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidLength { .. }));
        if let PfcpError::InvalidLength {
            ie_name,
            ie_type,
            expected,
            actual,
        } = err
        {
            assert_eq!(ie_name, "GBR");
            assert_eq!(ie_type, IeType::Gbr);
            assert_eq!(expected, 10);
            assert_eq!(actual, 9);
        }
    }
}
