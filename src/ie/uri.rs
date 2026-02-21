//! URI Information Element.
//!
//! Per 3GPP TS 29.244, contains a URI as a variable-length string.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    pub value: String,
}

impl Uri {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let value = String::from_utf8(data.to_vec())
            .map_err(|e| PfcpError::encoding_error("URI", IeType::Uri, e.utf8_error()))?;
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::Uri, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let uri = Uri::new("https://example.com/callback");
        let parsed = Uri::unmarshal(&uri.marshal()).unwrap();
        assert_eq!(parsed, uri);
    }

    #[test]
    fn test_unmarshal_invalid_utf8() {
        assert!(matches!(
            Uri::unmarshal(&[0xFF, 0xFE]),
            Err(PfcpError::EncodingError { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(Uri::new("test").to_ie().ie_type, IeType::Uri);
    }
}
