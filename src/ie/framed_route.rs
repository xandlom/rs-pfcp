//! Framed-Route Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.120, contains a Framed-Route AVP value as a string.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedRoute {
    pub value: String,
}

impl FramedRoute {
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
            PfcpError::encoding_error("Framed-Route", IeType::FramedRoute, e.utf8_error())
        })?;
        Ok(Self { value })
    }

    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::FramedRoute, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_unmarshal() {
        let route = FramedRoute::new("10.0.0.0/8 192.168.1.1");
        let parsed = FramedRoute::unmarshal(&route.marshal()).unwrap();
        assert_eq!(parsed, route);
    }

    #[test]
    fn test_unmarshal_invalid_utf8() {
        assert!(matches!(
            FramedRoute::unmarshal(&[0xFF, 0xFE]),
            Err(PfcpError::EncodingError { .. })
        ));
    }

    #[test]
    fn test_to_ie() {
        assert_eq!(
            FramedRoute::new("test").to_ie().ie_type,
            IeType::FramedRoute
        );
    }
}
