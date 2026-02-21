//! UE IP Address Pool Identity Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.144, identifies a UE IP address pool.
//! Encoded as u16 length + UTF-8 string.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeIpAddressPoolIdentity {
    pub value: String,
}

impl UeIpAddressPoolIdentity {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        let bytes = self.value.as_bytes();
        let mut data = Vec::with_capacity(2 + bytes.len());
        data.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
        data.extend_from_slice(bytes);
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.len() < 2 {
            return Err(PfcpError::invalid_length(
                "UE IP Address Pool Identity",
                IeType::UeIpAddressPoolIdentity,
                2,
                data.len(),
            ));
        }
        let len = u16::from_be_bytes(data[0..2].try_into().unwrap()) as usize;
        if data.len() < 2 + len {
            return Err(PfcpError::invalid_length(
                "UE IP Address Pool Identity",
                IeType::UeIpAddressPoolIdentity,
                2 + len,
                data.len(),
            ));
        }
        let value = String::from_utf8(data[2..2 + len].to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "UE IP Address Pool Identity",
                IeType::UeIpAddressPoolIdentity,
                e.utf8_error(),
            )
        })?;
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UeIpAddressPoolIdentity, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let pool = UeIpAddressPoolIdentity::new("pool-1");
        let parsed = UeIpAddressPoolIdentity::unmarshal(&pool.marshal()).unwrap();
        assert_eq!(parsed, pool);
    }

    #[test]
    fn test_unmarshal_short() {
        assert!(matches!(
            UeIpAddressPoolIdentity::unmarshal(&[0x00]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_unmarshal_truncated_string() {
        // Length says 10, but only 3 bytes of string
        let data = [0x00, 0x0A, b'a', b'b', b'c'];
        assert!(matches!(
            UeIpAddressPoolIdentity::unmarshal(&data),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            UeIpAddressPoolIdentity::new("test").to_ie().ie_type,
            IeType::UeIpAddressPoolIdentity
        );
    }
}
