//! Tunnel Password Information Element.
//!
//! Per 3GPP TS 29.244, contains tunnel password as raw bytes.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelPassword {
    pub value: Vec<u8>,
}

impl TunnelPassword {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        if data.is_empty() {
            return Err(PfcpError::invalid_length(
                "Tunnel Password",
                IeType::TunnelPassword,
                1,
                0,
            ));
        }
        Ok(Self {
            value: data.to_vec(),
        })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::TunnelPassword, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let pw = TunnelPassword::new(vec![0x01, 0x02, 0x03, 0x04]);
        let parsed = TunnelPassword::unmarshal(&pw.marshal()).unwrap();
        assert_eq!(parsed, pw);
    }

    #[test]
    fn test_unmarshal_empty() {
        assert!(matches!(
            TunnelPassword::unmarshal(&[]),
            Err(PfcpError::InvalidLength { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            TunnelPassword::new(vec![0x01]).to_ie().ie_type,
            IeType::TunnelPassword
        );
    }
}
