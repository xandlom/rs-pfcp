//! Data Network Access Identifier Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.182, contains a data network access identifier string.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataNetworkAccessIdentifier {
    pub value: String,
}

impl DataNetworkAccessIdentifier {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let value = String::from_utf8(data.to_vec()).map_err(|e| {
            PfcpError::encoding_error(
                "Data Network Access Identifier",
                IeType::DataNetworkAccessIdentifier,
                e.utf8_error(),
            )
        })?;
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::DataNetworkAccessIdentifier, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let dnai = DataNetworkAccessIdentifier::new("edge-site-1");
        let parsed = DataNetworkAccessIdentifier::unmarshal(&dnai.marshal()).unwrap();
        assert_eq!(parsed, dnai);
    }

    #[test]
    fn test_unmarshal_invalid_utf8() {
        assert!(matches!(
            DataNetworkAccessIdentifier::unmarshal(&[0xFF, 0xFE]),
            Err(PfcpError::EncodingError { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            DataNetworkAccessIdentifier::new("test").to_ie().ie_type,
            IeType::DataNetworkAccessIdentifier
        );
    }
}
